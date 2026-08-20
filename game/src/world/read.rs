use super::*;

pub(super) const CURRENT_ENTITY_STATE_SQL: &str = r#"
    WITH association AS (
        SELECT 0::smallint AS association_type,
               current.property_key_id, NULL::uuid AS trait_id,
               property_key.key, history.value_type,
               history.text_value, history.integer_value, NULL::text AS statement
        FROM entity_property AS current
        JOIN entity_property_history AS history
          ON history.entity_id = current.entity_id
         AND history.property_key_id = current.property_key_id
         AND history.activity_id = current.current_activity_id
        JOIN property_key ON property_key.id = current.property_key_id
        WHERE current.entity_id = $1
          AND (
                $2::smallint IS NULL
                OR ($2 = 0 AND current.property_key_id > $3::bigint)
              )

        UNION ALL

        SELECT 1::smallint, NULL::bigint, current.trait_id,
               NULL::text, NULL::text, NULL::text, NULL::bigint, version.statement
        FROM entity_trait_current AS current
        JOIN entity_trait_version AS version
          ON version.trait_id = current.trait_id
         AND version.entity_id = current.entity_id
         AND version.activity_id = current.current_activity_id
        WHERE current.entity_id = $1
          AND (
                $2::smallint IS NULL
                OR $2 = 0
                OR ($2 = 1 AND current.trait_id > $4::uuid)
              )
    )
    SELECT association_type, property_key_id, trait_id, key, value_type,
           text_value, integer_value, statement
    FROM association
    ORDER BY association_type, property_key_id, trait_id
    LIMIT $5
"#;

impl World {
    pub async fn get_user(&self, user_id: UserId) -> Result<User, WorldError> {
        sqlx::query_as::<_, User>(
            r#"
            SELECT id, created_at
            FROM "user"
            WHERE id = $1
            "#,
        )
        .bind(user_id.0)
        .fetch_optional(&self.pool)
        .await
        .map_err(|error| storage_error("get_user", error))?
        .ok_or(WorldError::UserNotFound)
    }

    pub async fn get_character(
        &self,
        user_id: UserId,
        request: GetEntityCurrentState,
    ) -> Result<CharacterEntityStatePage, WorldError> {
        validate_limit(request.limit, WorldError::InvalidEntityLimit)?;
        let mut transaction = self.begin_repeatable_read("get_character").await?;
        require_user(&mut transaction, user_id, "get_character").await?;
        let character = find_character(&mut transaction, user_id, false, "get_character")
            .await?
            .ok_or(WorldError::CharacterNotFound)?;
        let place_revision = match character.current_place.as_ref() {
            Some(place) => {
                Some(find_place_revision(&mut transaction, place.entity.id, "get_character").await?)
            }
            None => None,
        };
        let current_state = hydrate_entity_current_state(
            &mut transaction,
            character.entity.id,
            place_revision,
            request.cursor,
            request.limit,
            "get_character",
        )
        .await?;
        transaction
            .commit()
            .await
            .map_err(|error| storage_error("get_character", error))?;
        Ok(CharacterEntityStatePage {
            character,
            place_revision,
            current_state,
        })
    }

    pub async fn list_activity(
        &self,
        user_id: UserId,
        request: ListActivity,
    ) -> Result<ActivityPage, WorldError> {
        validate_limit(request.limit, WorldError::InvalidActivityLimit)?;
        let mut transaction = self.begin_repeatable_read("list_activity").await?;
        require_user(&mut transaction, user_id, "list_activity").await?;
        let character = find_character(&mut transaction, user_id, false, "list_activity")
            .await?
            .ok_or(WorldError::CharacterNotFound)?;
        let cursor_time = request.cursor.as_ref().map(|cursor| cursor.occurred_at);
        let cursor_id = request.cursor.as_ref().map(|cursor| cursor.activity_id.0);
        let fetch_limit = i64::from(request.limit) + 1;
        let mut row = sqlx::query_as::<_, ActivityRow>(
            r#"
            SELECT activity.id, activity.operation, activity.prose, activity.occurred_at,
                   actor.id AS actor_entity_id, actor.name AS actor_name,
                   context.id AS context_entity_id, context.name AS context_name,
                   context_place.is_entry AS context_is_entry
            FROM (
                SELECT id
                FROM activity
                WHERE actor_character_entity_id = $1
                UNION
                SELECT activity_id
                FROM activity_entity
                WHERE entity_id = $1
            ) relevant_activity
            JOIN activity ON activity.id = relevant_activity.id
            LEFT JOIN entity actor ON actor.id = activity.actor_character_entity_id
            LEFT JOIN place context_place ON context_place.entity_id = activity.context_place_entity_id
            LEFT JOIN entity context ON context.id = context_place.entity_id
            WHERE (
                    $2::timestamptz IS NULL
                    OR (activity.occurred_at, activity.id) < ($2::timestamptz, $3::uuid)
                  )
            ORDER BY activity.occurred_at DESC, activity.id DESC
            LIMIT $4
            "#,
        )
        .bind(character.entity.id.0)
        .bind(cursor_time)
        .bind(cursor_id)
        .bind(fetch_limit)
        .fetch_all(&mut *transaction)
        .await
        .map_err(|error| storage_error("list_activity", error))?;

        let has_more = row.len() > usize::from(request.limit);
        if has_more {
            row.pop();
        }
        let next = has_more.then(|| {
            let last = row
                .last()
                .expect("a page with another row always has a returned row");
            ActivityCursor {
                occurred_at: last.occurred_at,
                activity_id: last.id,
            }
        });
        let activity = activities_from_rows(&mut transaction, row, "list_activity").await?;
        transaction
            .commit()
            .await
            .map_err(|error| storage_error("list_activity", error))?;
        Ok(ActivityPage { activity, next })
    }

    pub async fn list_entity_at_current_place(
        &self,
        user_id: UserId,
        request: ListEntityAtCurrentPlace,
    ) -> Result<CurrentPlaceEntityPage, WorldError> {
        validate_limit(request.limit, WorldError::InvalidEntityLimit)?;
        let mut transaction = self
            .begin_repeatable_read("list_entity_at_current_place")
            .await?;
        require_user(&mut transaction, user_id, "list_entity_at_current_place").await?;
        let character = find_character(
            &mut transaction,
            user_id,
            false,
            "list_entity_at_current_place",
        )
        .await?
        .ok_or(WorldError::CharacterNotFound)?;
        let place = character
            .current_place
            .ok_or(WorldError::CharacterNotEntered)?;
        let place_revision = find_place_revision(
            &mut transaction,
            place.entity.id,
            "list_entity_at_current_place",
        )
        .await?;
        let cursor_time = request.cursor.as_ref().map(|cursor| cursor.introduced_at);
        let cursor_id = request.cursor.as_ref().map(|cursor| cursor.entity_id.0);
        let fetch_limit = i64::from(request.limit) + 1;
        let mut row = sqlx::query_as::<_, CurrentPlaceEntityRow>(
            r#"
            WITH eligible_entity AS (
                SELECT entity.id, entity.name, entity.description, entity.introduced_at,
                       position.current_activity_id AS position_activity_id,
                       version.x_cm AS position_x_cm,
                       version.y_cm AS position_y_cm,
                       version.z_cm AS position_z_cm,
                       version.description AS position_description
                FROM character
                JOIN entity ON entity.id = character.entity_id
                JOIN position ON position.entity_id = entity.id
                JOIN position_version version
                  ON version.entity_id = position.entity_id
                 AND version.activity_id = position.current_activity_id
                WHERE character.current_place_entity_id = $1
                  AND character.entity_id <> $2

                UNION ALL

                SELECT entity.id, entity.name, entity.description, entity.introduced_at,
                       position.current_activity_id AS position_activity_id,
                       version.x_cm AS position_x_cm,
                       version.y_cm AS position_y_cm,
                       version.z_cm AS position_z_cm,
                       version.description AS position_description
                FROM entity_location
                JOIN entity ON entity.id = entity_location.entity_id
                JOIN position ON position.entity_id = entity.id
                JOIN position_version version
                  ON version.entity_id = position.entity_id
                 AND version.activity_id = position.current_activity_id
                WHERE entity_location.place_entity_id = $1
                  AND NOT EXISTS (
                      SELECT 1 FROM character WHERE character.entity_id = entity.id
                  )
                  AND NOT EXISTS (
                      SELECT 1 FROM place WHERE place.entity_id = entity.id
                  )
            )
            SELECT id, name, description, introduced_at,
                   position_activity_id, position_x_cm, position_y_cm,
                   position_z_cm, position_description
            FROM eligible_entity
            WHERE (
                    $3::timestamptz IS NULL
                    OR (introduced_at, id) < ($3::timestamptz, $4::uuid)
                  )
            ORDER BY introduced_at DESC, id DESC
            LIMIT $5
            "#,
        )
        .bind(place.entity.id.0)
        .bind(character.entity.id.0)
        .bind(cursor_time)
        .bind(cursor_id)
        .bind(fetch_limit)
        .fetch_all(&mut *transaction)
        .await
        .map_err(|error| storage_error("list_entity_at_current_place", error))?;
        let has_more = row.len() > usize::from(request.limit);
        if has_more {
            row.pop();
        }
        let next = has_more.then(|| {
            let last = row
                .last()
                .expect("a page with another row always has a returned row");
            EntityCursor {
                introduced_at: last.introduced_at,
                entity_id: last.id,
            }
        });
        let entity = row
            .into_iter()
            .map(|row| CurrentPlaceEntity {
                id: row.id,
                name: row.name,
                description: row.description,
                position: Position {
                    x_cm: row.position_x_cm,
                    y_cm: row.position_y_cm,
                    z_cm: row.position_z_cm,
                    description: row.position_description,
                    position_revision: PositionRevision::from_parts(
                        row.id,
                        row.position_activity_id,
                    ),
                },
            })
            .collect();
        transaction
            .commit()
            .await
            .map_err(|error| storage_error("list_entity_at_current_place", error))?;
        Ok(CurrentPlaceEntityPage {
            place,
            place_revision,
            entity,
            next,
        })
    }

    pub async fn list_activity_at_current_place(
        &self,
        user_id: UserId,
        request: ListActivityAtCurrentPlace,
    ) -> Result<CurrentPlaceActivityPage, WorldError> {
        validate_limit(request.limit, WorldError::InvalidActivityLimit)?;
        let mut transaction = self
            .begin_repeatable_read("list_activity_at_current_place")
            .await?;
        require_user(&mut transaction, user_id, "list_activity_at_current_place").await?;
        let character = find_character(
            &mut transaction,
            user_id,
            false,
            "list_activity_at_current_place",
        )
        .await?
        .ok_or(WorldError::CharacterNotFound)?;
        let place = character
            .current_place
            .ok_or(WorldError::CharacterNotEntered)?;
        let place_revision = find_place_revision(
            &mut transaction,
            place.entity.id,
            "list_activity_at_current_place",
        )
        .await?;
        let cursor_time = request.cursor.as_ref().map(|cursor| cursor.occurred_at);
        let cursor_id = request.cursor.as_ref().map(|cursor| cursor.activity_id.0);
        let fetch_limit = i64::from(request.limit) + 1;
        let mut row = sqlx::query_as::<_, ActivityRow>(
            r#"
            SELECT activity.id, activity.operation, activity.prose, activity.occurred_at,
                   actor.id AS actor_entity_id, actor.name AS actor_name,
                   context.id AS context_entity_id, context.name AS context_name,
                   context_place.is_entry AS context_is_entry
            FROM activity
            LEFT JOIN entity actor ON actor.id = activity.actor_character_entity_id
            LEFT JOIN place context_place ON context_place.entity_id = activity.context_place_entity_id
            LEFT JOIN entity context ON context.id = context_place.entity_id
            WHERE (
                    activity.context_place_entity_id = $1
                    OR EXISTS (
                        SELECT 1
                        FROM activity_entity
                        WHERE activity_entity.activity_id = activity.id
                          AND activity_entity.entity_id = $1
                    )
                  )
              AND (
                    activity.operation <> 'submit_interaction'
                    OR activity.actor_character_entity_id = $2
                    OR EXISTS (
                        SELECT 1
                        FROM activity_entity target_participation
                        WHERE target_participation.activity_id = activity.id
                          AND target_participation.entity_id = $2
                          AND target_participation.role = 'target'
                    )
                  )
              AND (
                    $3::timestamptz IS NULL
                    OR (activity.occurred_at, activity.id) < ($3::timestamptz, $4::uuid)
                  )
            ORDER BY activity.occurred_at DESC, activity.id DESC
            LIMIT $5
            "#,
        )
        .bind(place.entity.id.0)
        .bind(character.entity.id.0)
        .bind(cursor_time)
        .bind(cursor_id)
        .bind(fetch_limit)
        .fetch_all(&mut *transaction)
        .await
        .map_err(|error| storage_error("list_activity_at_current_place", error))?;
        let has_more = row.len() > usize::from(request.limit);
        if has_more {
            row.pop();
        }
        let next = has_more.then(|| {
            let last = row
                .last()
                .expect("a page with another row always has a returned row");
            ActivityCursor {
                occurred_at: last.occurred_at,
                activity_id: last.id,
            }
        });
        let activity =
            activities_from_rows(&mut transaction, row, "list_activity_at_current_place").await?;
        transaction
            .commit()
            .await
            .map_err(|error| storage_error("list_activity_at_current_place", error))?;
        Ok(CurrentPlaceActivityPage {
            place,
            place_revision,
            activity,
            next,
        })
    }

    pub async fn get_entity_at_current_place(
        &self,
        user_id: UserId,
        request: GetEntityAtCurrentPlace,
    ) -> Result<CurrentPlaceEntityStatePage, WorldError> {
        validate_limit(request.limit, WorldError::InvalidEntityLimit)?;
        let mut transaction = self
            .begin_repeatable_read("get_entity_at_current_place")
            .await?;
        require_user(&mut transaction, user_id, "get_entity_at_current_place").await?;
        let character = find_character(
            &mut transaction,
            user_id,
            false,
            "get_entity_at_current_place",
        )
        .await?
        .ok_or(WorldError::CharacterNotFound)?;
        let place = character
            .current_place
            .ok_or(WorldError::CharacterNotEntered)?;
        let place_revision = find_place_revision(
            &mut transaction,
            place.entity.id,
            "get_entity_at_current_place",
        )
        .await?;

        let entity = sqlx::query_as::<_, CurrentPlaceEntityRow>(
            r#"
            SELECT entity.id, entity.name, entity.description, entity.introduced_at,
                   position.current_activity_id AS position_activity_id,
                   version.x_cm AS position_x_cm,
                   version.y_cm AS position_y_cm,
                   version.z_cm AS position_z_cm,
                   version.description AS position_description
            FROM entity
            JOIN position ON position.entity_id = entity.id
            JOIN position_version version
              ON version.entity_id = position.entity_id
             AND version.activity_id = position.current_activity_id
            WHERE entity.id = $1
              AND (
                    entity.id = $2
                    OR entity.id = $3
                    OR EXISTS (
                        SELECT 1
                        FROM character
                        WHERE character.entity_id = entity.id
                          AND character.current_place_entity_id = $3
                    )
                    OR EXISTS (
                        SELECT 1
                        FROM entity_location
                        WHERE entity_location.entity_id = entity.id
                          AND entity_location.place_entity_id = $3
                          AND NOT EXISTS (
                              SELECT 1 FROM character
                              WHERE character.entity_id = entity.id
                          )
                          AND NOT EXISTS (
                              SELECT 1 FROM place
                              WHERE place.entity_id = entity.id
                          )
                    )
                  )
            "#,
        )
        .bind(request.entity_id.0)
        .bind(character.entity.id.0)
        .bind(place.entity.id.0)
        .fetch_optional(&mut *transaction)
        .await
        .map_err(|error| storage_error("get_entity_at_current_place", error))?
        .ok_or(WorldError::EntityAtCurrentPlaceUnavailable)?;
        let current_state = hydrate_entity_current_state(
            &mut transaction,
            request.entity_id,
            Some(place_revision),
            request.cursor,
            request.limit,
            "get_entity_at_current_place",
        )
        .await?;
        transaction
            .commit()
            .await
            .map_err(|error| storage_error("get_entity_at_current_place", error))?;
        Ok(CurrentPlaceEntityStatePage {
            place,
            place_revision,
            entity: CurrentPlaceEntity {
                id: entity.id,
                name: entity.name,
                description: entity.description,
                position: Position {
                    x_cm: entity.position_x_cm,
                    y_cm: entity.position_y_cm,
                    z_cm: entity.position_z_cm,
                    description: entity.position_description,
                    position_revision: PositionRevision::from_parts(
                        entity.id,
                        entity.position_activity_id,
                    ),
                },
            },
            current_state,
        })
    }
}
