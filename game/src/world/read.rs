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

pub(super) const LIST_PLACE_FIRST_PAGE_SQL: &str = r#"
    SELECT place_entity_id, position_activity_id, x_cm, y_cm, z_cm
    FROM place_map_index
    WHERE x_cm BETWEEN $1 AND $2
      AND y_cm BETWEEN $3 AND $4
      AND z_cm BETWEEN $5 AND $6
    ORDER BY x_cm, y_cm, z_cm, place_entity_id
    LIMIT $7
"#;

pub(super) const LIST_PLACE_CONTINUED_PAGE_SQL: &str = r#"
    SELECT place_entity_id, position_activity_id, x_cm, y_cm, z_cm
    FROM place_map_index
    WHERE x_cm BETWEEN $1 AND $2
      AND y_cm BETWEEN $3 AND $4
      AND z_cm BETWEEN $5 AND $6
      AND (x_cm, y_cm, z_cm, place_entity_id) > ($7, $8, $9, $10::uuid)
    ORDER BY x_cm, y_cm, z_cm, place_entity_id
    LIMIT $11
"#;

pub(super) const LIST_CONNECTION_FIRST_PAGE_SQL: &str = r#"
    (SELECT id
     FROM connection
     WHERE source_place_entity_id = $1
     ORDER BY id
     LIMIT $2)
    UNION ALL
    (SELECT id
     FROM connection
     WHERE destination_place_entity_id = $1
     ORDER BY id
     LIMIT $2)
    ORDER BY id
    LIMIT $2
"#;

pub(super) const LIST_CONNECTION_CONTINUED_PAGE_SQL: &str = r#"
    (SELECT id
     FROM connection
     WHERE source_place_entity_id = $1
       AND id > $2
     ORDER BY id
     LIMIT $3)
    UNION ALL
    (SELECT id
     FROM connection
     WHERE destination_place_entity_id = $1
       AND id > $2
     ORDER BY id
     LIMIT $3)
    ORDER BY id
    LIMIT $3
"#;

const CONNECTION_SUMMARY_SELECT: &str = r#"
    SELECT connection.id, connection.allows_reverse, connection.has_course,
           connection.name, connection.description,
           NULL::text AS shape_description,
           source_entity.id AS source_id, source_entity.name AS source_name,
           source_entity.description AS source_description,
           source_place.is_entry AS source_is_entry,
           connection.source_position_activity_id,
           source_version.x_cm AS source_x_cm,
           source_version.y_cm AS source_y_cm,
           source_version.z_cm AS source_z_cm,
           source_version.description AS source_position_description,
           destination_entity.id AS destination_id,
           destination_entity.name AS destination_name,
           destination_entity.description AS destination_description,
           destination_place.is_entry AS destination_is_entry,
           connection.destination_position_activity_id,
           destination_version.x_cm AS destination_x_cm,
           destination_version.y_cm AS destination_y_cm,
           destination_version.z_cm AS destination_z_cm,
           destination_version.description AS destination_position_description
    FROM connection
"#;

const CONNECTION_DETAIL_SELECT: &str = r#"
    SELECT connection.id, connection.allows_reverse, connection.has_course,
           connection.name, connection.description, connection.shape_description,
           source_entity.id AS source_id, source_entity.name AS source_name,
           source_entity.description AS source_description,
           source_place.is_entry AS source_is_entry,
           connection.source_position_activity_id,
           source_version.x_cm AS source_x_cm,
           source_version.y_cm AS source_y_cm,
           source_version.z_cm AS source_z_cm,
           source_version.description AS source_position_description,
           destination_entity.id AS destination_id,
           destination_entity.name AS destination_name,
           destination_entity.description AS destination_description,
           destination_place.is_entry AS destination_is_entry,
           connection.destination_position_activity_id,
           destination_version.x_cm AS destination_x_cm,
           destination_version.y_cm AS destination_y_cm,
           destination_version.z_cm AS destination_z_cm,
           destination_version.description AS destination_position_description
    FROM connection
"#;

const CONNECTION_JOINS: &str = r#"
    JOIN place source_place
      ON source_place.entity_id = connection.source_place_entity_id
    JOIN entity source_entity ON source_entity.id = source_place.entity_id
    JOIN position source_current
      ON source_current.entity_id = source_place.entity_id
     AND source_current.current_activity_id = connection.source_position_activity_id
    JOIN position_version source_version
      ON source_version.entity_id = source_current.entity_id
     AND source_version.activity_id = source_current.current_activity_id
    JOIN place destination_place
      ON destination_place.entity_id = connection.destination_place_entity_id
    JOIN entity destination_entity ON destination_entity.id = destination_place.entity_id
    JOIN position destination_current
      ON destination_current.entity_id = destination_place.entity_id
     AND destination_current.current_activity_id = connection.destination_position_activity_id
    JOIN position_version destination_version
      ON destination_version.entity_id = destination_current.entity_id
     AND destination_version.activity_id = destination_current.current_activity_id
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

    pub async fn list_place(
        &self,
        user_id: UserId,
        request: ListPlace,
    ) -> Result<PlacePage, WorldError> {
        validate_place_window(&request)?;
        validate_limit(request.limit, WorldError::InvalidPlaceLimit)?;
        let cursor = request
            .cursor
            .as_deref()
            .map(|cursor| PlaceCursor::decode(cursor, &request))
            .transpose()?;
        let mut transaction = self.begin_spatial_read("list_place").await?;
        require_spatial_reader(&mut transaction, user_id, "list_place").await?;

        let fetch_limit = i64::from(request.limit) + 1;
        record_spatial_read_query(SpatialReadQueryKind::PlaceCandidate);
        let mut candidate = match cursor {
            None => {
                sqlx::query_as::<_, PlaceMapCandidate>(LIST_PLACE_FIRST_PAGE_SQL)
                    .bind(request.min_x_cm)
                    .bind(request.max_x_cm)
                    .bind(request.min_y_cm)
                    .bind(request.max_y_cm)
                    .bind(request.min_z_cm)
                    .bind(request.max_z_cm)
                    .bind(fetch_limit)
                    .fetch_all(&mut *transaction)
                    .await
            }
            Some(cursor) => {
                let (cursor_x, cursor_y, cursor_z, cursor_id) = cursor.last_tuple();
                sqlx::query_as::<_, PlaceMapCandidate>(LIST_PLACE_CONTINUED_PAGE_SQL)
                    .bind(request.min_x_cm)
                    .bind(request.max_x_cm)
                    .bind(request.min_y_cm)
                    .bind(request.max_y_cm)
                    .bind(request.min_z_cm)
                    .bind(request.max_z_cm)
                    .bind(cursor_x)
                    .bind(cursor_y)
                    .bind(cursor_z)
                    .bind(cursor_id)
                    .bind(fetch_limit)
                    .fetch_all(&mut *transaction)
                    .await
            }
        }
        .map_err(|error| spatial_read_error("list_place", error))?;

        let has_more = candidate.len() > usize::from(request.limit);
        if has_more {
            candidate.pop();
        }
        let next = has_more.then(|| {
            PlaceCursor::from_candidate(
                &request,
                *candidate
                    .last()
                    .expect("a continued Place page processes at least one candidate"),
            )
            .encode()
        });
        let place = hydrate_place_candidates(&mut transaction, &candidate, "list_place").await?;
        transaction
            .commit()
            .await
            .map_err(|error| spatial_read_error("list_place", error))?;
        Ok(PlacePage { place, next })
    }

    pub async fn list_connection(
        &self,
        user_id: UserId,
        request: ListConnection,
    ) -> Result<ConnectionPage, WorldError> {
        validate_limit(request.limit, WorldError::InvalidConnectionLimit)?;
        let cursor = request
            .cursor
            .as_deref()
            .map(|cursor| ConnectionCursor::decode(cursor, request.place_id))
            .transpose()?;
        let mut transaction = self.begin_spatial_read("list_connection").await?;
        require_spatial_reader(&mut transaction, user_id, "list_connection").await?;
        let place = find_positioned_place(&mut transaction, request.place_id, "list_connection")
            .await?
            .ok_or(WorldError::PlaceNotFound)?;
        let fetch_limit = i64::from(request.limit) + 1;
        record_spatial_read_query(SpatialReadQueryKind::ConnectionCandidate);
        let mut connection_id = match cursor {
            None => {
                sqlx::query_scalar::<_, Uuid>(LIST_CONNECTION_FIRST_PAGE_SQL)
                    .bind(request.place_id.0)
                    .bind(fetch_limit)
                    .fetch_all(&mut *transaction)
                    .await
            }
            Some(cursor) => {
                sqlx::query_scalar::<_, Uuid>(LIST_CONNECTION_CONTINUED_PAGE_SQL)
                    .bind(request.place_id.0)
                    .bind(cursor.last_connection_id().0)
                    .bind(fetch_limit)
                    .fetch_all(&mut *transaction)
                    .await
            }
        }
        .map_err(|error| spatial_read_error("list_connection", error))?;
        let has_more = connection_id.len() > usize::from(request.limit);
        if has_more {
            connection_id.pop();
        }
        let next = has_more.then(|| {
            ConnectionCursor::new(
                request.place_id,
                ConnectionId(
                    *connection_id
                        .last()
                        .expect("a continued Connection page processes at least one candidate"),
                ),
            )
            .encode()
        });
        let row = hydrate_connection_summaries(&mut transaction, &connection_id, "list_connection")
            .await?;
        let connection = row.into_iter().map(ConnectionRow::into_summary).collect();
        transaction
            .commit()
            .await
            .map_err(|error| spatial_read_error("list_connection", error))?;
        Ok(ConnectionPage {
            place,
            connection,
            next,
        })
    }

    pub async fn get_connection(
        &self,
        user_id: UserId,
        request: GetConnection,
    ) -> Result<Connection, WorldError> {
        let mut transaction = self.begin_spatial_read("get_connection").await?;
        require_spatial_reader(&mut transaction, user_id, "get_connection").await?;
        record_spatial_read_query(SpatialReadQueryKind::ConnectionGet);
        let row = sqlx::query_as::<_, ConnectionRow>(&format!(
            r#"
            {CONNECTION_DETAIL_SELECT}
            {CONNECTION_JOINS}
            WHERE connection.id = $1
              AND (
                    connection.source_place_entity_id = $2
                    OR connection.destination_place_entity_id = $2
                  )
            "#,
        ))
        .bind(request.connection_id.0)
        .bind(request.place_id.0)
        .fetch_optional(&mut *transaction)
        .await
        .map_err(|error| spatial_read_error("get_connection", error))?
        .ok_or(WorldError::ConnectionNotFound)?;
        record_spatial_read_query(SpatialReadQueryKind::ConnectionCourse);
        let course = sqlx::query_as::<_, ConnectionPointRow>(
            r#"
            SELECT ordinal, x_cm, y_cm, z_cm
            FROM connection_point
            WHERE connection_id = $1
            ORDER BY ordinal
            LIMIT 129
            "#,
        )
        .bind(request.connection_id.0)
        .fetch_all(&mut *transaction)
        .await
        .map_err(|error| spatial_read_error("get_connection", error))?
        .into_iter()
        .map(ConnectionPoint::try_from)
        .collect::<Result<Vec<_>, _>>()?;
        if course.len() > 128 {
            return Err(invalid_stored_relation());
        }
        let connection = row.into_connection(course);
        transaction
            .commit()
            .await
            .map_err(|error| spatial_read_error("get_connection", error))?;
        Ok(connection)
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

fn validate_place_window(request: &ListPlace) -> Result<(), WorldError> {
    for coordinate in [
        request.min_x_cm,
        request.max_x_cm,
        request.min_y_cm,
        request.max_y_cm,
        request.min_z_cm,
        request.max_z_cm,
    ] {
        if validate_coordinate(coordinate).is_err() {
            return Err(WorldError::InvalidPlaceWindow);
        }
    }
    for (minimum, maximum) in [
        (request.min_x_cm, request.max_x_cm),
        (request.min_y_cm, request.max_y_cm),
        (request.min_z_cm, request.max_z_cm),
    ] {
        if minimum > maximum
            || maximum
                .checked_sub(minimum)
                .is_none_or(|span| span > MAX_PLACE_WINDOW_SPAN_CM)
        {
            return Err(WorldError::InvalidPlaceWindow);
        }
    }
    Ok(())
}

async fn require_spatial_reader(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: UserId,
    operation: &'static str,
) -> Result<(), WorldError> {
    let reader: Option<(Option<Uuid>, Option<Uuid>)> = sqlx::query_as(
        r#"
        SELECT character.entity_id, position.current_activity_id
        FROM "user"
        LEFT JOIN character ON character.owner_user_id = "user".id
        LEFT JOIN position ON position.entity_id = character.entity_id
        WHERE "user".id = $1
        "#,
    )
    .bind(user_id.0)
    .fetch_optional(&mut **transaction)
    .await
    .map_err(|error| spatial_read_error(operation, error))?;
    match reader {
        None => Err(WorldError::UserNotFound),
        Some((None, _)) => Err(WorldError::CharacterNotFound),
        Some((Some(_), None)) => Err(WorldError::CharacterNotEntered),
        Some((Some(_), Some(_))) => Ok(()),
    }
}

async fn hydrate_place_candidates(
    transaction: &mut Transaction<'_, Postgres>,
    candidate: &[PlaceMapCandidate],
    operation: &'static str,
) -> Result<Vec<PlacePosition>, WorldError> {
    if candidate.is_empty() {
        return Ok(Vec::new());
    }
    record_spatial_read_query(SpatialReadQueryKind::PlaceHydration);
    let place_id: Vec<_> = candidate.iter().map(|row| row.place_entity_id.0).collect();
    let activity_id: Vec<_> = candidate
        .iter()
        .map(|row| row.position_activity_id.0)
        .collect();
    let x_cm: Vec<_> = candidate.iter().map(|row| row.x_cm).collect();
    let y_cm: Vec<_> = candidate.iter().map(|row| row.y_cm).collect();
    let z_cm: Vec<_> = candidate.iter().map(|row| row.z_cm).collect();
    sqlx::query_as::<_, PlacePositionRow>(
        r#"
        WITH candidate AS (
            SELECT *
            FROM unnest(
                $1::uuid[], $2::uuid[], $3::bigint[], $4::bigint[], $5::bigint[]
            ) WITH ORDINALITY AS item(
                place_entity_id, position_activity_id,
                x_cm, y_cm, z_cm, sort_ordinal
            )
        )
        SELECT entity.id, entity.name, entity.description, place.is_entry,
               version.activity_id AS position_activity_id,
               version.x_cm, version.y_cm, version.z_cm,
               version.description AS position_description
        FROM candidate
        JOIN position current
          ON current.entity_id = candidate.place_entity_id
         AND current.current_activity_id = candidate.position_activity_id
        JOIN position_version version
          ON version.entity_id = current.entity_id
         AND version.activity_id = current.current_activity_id
         AND version.x_cm = candidate.x_cm
         AND version.y_cm = candidate.y_cm
         AND version.z_cm = candidate.z_cm
        JOIN place ON place.entity_id = current.entity_id
        JOIN entity ON entity.id = place.entity_id
        ORDER BY candidate.sort_ordinal
        "#,
    )
    .bind(&place_id)
    .bind(&activity_id)
    .bind(&x_cm)
    .bind(&y_cm)
    .bind(&z_cm)
    .fetch_all(&mut **transaction)
    .await
    .map_err(|error| spatial_read_error(operation, error))
    .map(|row| row.into_iter().map(PlacePosition::from).collect())
}

async fn hydrate_connection_summaries(
    transaction: &mut Transaction<'_, Postgres>,
    connection_id: &[Uuid],
    operation: &'static str,
) -> Result<Vec<ConnectionRow>, WorldError> {
    if connection_id.is_empty() {
        return Ok(Vec::new());
    }
    record_spatial_read_query(SpatialReadQueryKind::ConnectionHydration);
    sqlx::query_as::<_, ConnectionRow>(&format!(
        r#"
        WITH selected_connection AS (
            SELECT id, sort_ordinal
            FROM unnest($1::uuid[]) WITH ORDINALITY AS item(id, sort_ordinal)
        )
        {CONNECTION_SUMMARY_SELECT}
        JOIN selected_connection selected ON selected.id = connection.id
        {CONNECTION_JOINS}
        ORDER BY selected.sort_ordinal
        "#,
    ))
    .bind(connection_id)
    .fetch_all(&mut **transaction)
    .await
    .map_err(|error| spatial_read_error(operation, error))
}

async fn find_positioned_place(
    transaction: &mut Transaction<'_, Postgres>,
    place_id: EntityId,
    operation: &'static str,
) -> Result<Option<PlacePosition>, WorldError> {
    record_spatial_read_query(SpatialReadQueryKind::ConnectionAnchor);
    sqlx::query_as::<_, PlacePositionRow>(
        r#"
        SELECT entity.id, entity.name, entity.description, place.is_entry,
               current.current_activity_id AS position_activity_id,
               version.x_cm, version.y_cm, version.z_cm,
               version.description AS position_description
        FROM place
        JOIN entity ON entity.id = place.entity_id
        JOIN position current ON current.entity_id = place.entity_id
        JOIN position_version version
          ON version.entity_id = current.entity_id
         AND version.activity_id = current.current_activity_id
        WHERE place.entity_id = $1
        "#,
    )
    .bind(place_id.0)
    .fetch_optional(&mut **transaction)
    .await
    .map_err(|error| spatial_read_error(operation, error))
    .map(|row| row.map(PlacePosition::from))
}
