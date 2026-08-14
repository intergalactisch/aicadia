#[cfg(test)]
use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap};

use chrono::{DateTime, Utc};
use sha2::{Digest, Sha256};
use sqlx::{FromRow, PgPool, Postgres, Transaction};
use thiserror::Error;
use uuid::Uuid;

const MAX_ENTITY_NAME_LENGTH: usize = 120;
const MAX_ENTITY_DESCRIPTION_LENGTH: usize = 4_000;
const MAX_ACTION_PROSE_LENGTH: usize = 4_000;
const MAX_INTERACTION_PROSE_LENGTH: usize = 4_000;
const MAX_INTERACTION_TARGET_COUNT: usize = 100;
const DEFAULT_PAGE_SIZE: u16 = 25;
const MAX_PAGE_SIZE: u16 = 100;
const WORLD_NAME: &str = "Aicadia";

const INSERT_PROPERTY_KEY_SQL: &str = r#"
    INSERT INTO property_key (key, value_type, first_activity_id)
    SELECT submitted.key, submitted.value_type, $3
    FROM UNNEST($1::text[], $2::text[]) AS submitted(key, value_type)
    ORDER BY submitted.key
    ON CONFLICT (key) DO NOTHING
"#;

const LOCK_CURRENT_ENTITY_PROPERTY_SQL: &str = r#"
    SELECT current.entity_id, current.property_key_id, current.current_activity_id
    FROM entity_property AS current
    WHERE (current.entity_id, current.property_key_id) IN (
        SELECT submitted.entity_id, submitted.property_key_id
        FROM UNNEST($1::uuid[], $2::bigint[])
            AS submitted(entity_id, property_key_id)
    )
    ORDER BY current.entity_id, current.property_key_id
    FOR UPDATE
"#;

const INSERT_ENTITY_PROPERTY_HISTORY_SQL: &str = r#"
    INSERT INTO entity_property_history (
        entity_id, property_key_id, activity_id, previous_activity_id,
        value_type, text_value, integer_value
    )
    SELECT submitted.entity_id, submitted.property_key_id, $3,
           submitted.previous_activity_id, submitted.value_type,
           submitted.text_value, submitted.integer_value
    FROM UNNEST(
        $1::uuid[], $2::bigint[], $4::uuid[], $5::text[], $6::text[], $7::bigint[]
    ) AS submitted(
        entity_id, property_key_id, previous_activity_id,
        value_type, text_value, integer_value
    )
    ORDER BY submitted.entity_id, submitted.property_key_id
"#;

const UPSERT_CURRENT_ENTITY_PROPERTY_SQL: &str = r#"
    INSERT INTO entity_property (entity_id, property_key_id, current_activity_id)
    SELECT submitted.entity_id, submitted.property_key_id, $3
    FROM UNNEST($1::uuid[], $2::bigint[]) AS submitted(entity_id, property_key_id)
    ORDER BY submitted.entity_id, submitted.property_key_id
    ON CONFLICT (entity_id, property_key_id) DO UPDATE
    SET current_activity_id = EXCLUDED.current_activity_id
"#;

const HYDRATE_ENTITY_PROPERTY_CHANGE_SQL: &str = r#"
    SELECT history.activity_id, history.entity_id, entity.name AS entity_name,
           property_key.key, history.value_type,
           history.text_value, history.integer_value
    FROM entity_property_history AS history
    JOIN entity ON entity.id = history.entity_id
    JOIN property_key ON property_key.id = history.property_key_id
    WHERE history.activity_id = ANY($1::uuid[])
    ORDER BY history.activity_id, history.entity_id, property_key.key
"#;

const FIND_ENTITY_TRAIT_OWNER_SQL: &str = r#"
    SELECT id AS trait_id, entity_id
    FROM entity_trait
    WHERE id = ANY($1::uuid[])
    ORDER BY id
"#;

const LOCK_TRAIT_ENTITY_SQL: &str = r#"
    SELECT entity.id
    FROM UNNEST($1::uuid[]) AS submitted(entity_id)
    JOIN entity ON entity.id = submitted.entity_id
    ORDER BY entity.id
    FOR UPDATE OF entity
"#;

const LOCK_ACTIVE_ENTITY_TRAIT_STATEMENT_SQL: &str = r#"
    SELECT current.entity_id, current.trait_id, version.statement
    FROM UNNEST($1::uuid[], $2::text[]) AS submitted(entity_id, statement)
    JOIN entity_trait_current AS current
      ON current.entity_id = submitted.entity_id
    JOIN entity_trait_version AS version
      ON version.trait_id = current.trait_id
     AND version.entity_id = current.entity_id
     AND version.activity_id = current.current_activity_id
     AND version.statement = submitted.statement
    WHERE NOT (current.trait_id = ANY($3::uuid[]))
    ORDER BY current.entity_id, current.trait_id
    LIMIT 1
    FOR UPDATE OF current
"#;

const LOCK_CURRENT_ENTITY_TRAIT_SQL: &str = r#"
    SELECT current.trait_id, current.entity_id,
           current.current_activity_id, version.statement
    FROM entity_trait_current AS current
    JOIN entity_trait_version AS version
      ON version.trait_id = current.trait_id
     AND version.entity_id = current.entity_id
     AND version.activity_id = current.current_activity_id
    WHERE current.trait_id = ANY($1::uuid[])
    ORDER BY current.trait_id
    FOR UPDATE OF current
"#;

const INSERT_ENTITY_TRAIT_SQL: &str = r#"
    INSERT INTO entity_trait (id, entity_id)
    SELECT submitted.trait_id, submitted.entity_id
    FROM UNNEST($1::uuid[], $2::uuid[]) AS submitted(trait_id, entity_id)
    ORDER BY submitted.entity_id, submitted.trait_id
"#;

const INSERT_ENTITY_TRAIT_VERSION_SQL: &str = r#"
    INSERT INTO entity_trait_version (
        trait_id, entity_id, activity_id, previous_activity_id, statement
    )
    SELECT submitted.trait_id, submitted.entity_id, $3,
           submitted.previous_activity_id, submitted.statement
    FROM UNNEST($1::uuid[], $2::uuid[], $4::uuid[], $5::text[])
        AS submitted(trait_id, entity_id, previous_activity_id, statement)
    ORDER BY submitted.entity_id, submitted.trait_id
"#;

const UPSERT_CURRENT_ENTITY_TRAIT_SQL: &str = r#"
    INSERT INTO entity_trait_current (trait_id, entity_id, current_activity_id)
    SELECT submitted.trait_id, submitted.entity_id, $3
    FROM UNNEST($1::uuid[], $2::uuid[]) AS submitted(trait_id, entity_id)
    ORDER BY submitted.entity_id, submitted.trait_id
    ON CONFLICT (trait_id) DO UPDATE
    SET current_activity_id = EXCLUDED.current_activity_id
"#;

const HYDRATE_ENTITY_TRAIT_CHANGE_SQL: &str = r#"
    SELECT version.activity_id, trait.id AS trait_id,
           trait.entity_id, entity.name AS entity_name,
           version.previous_activity_id, previous.statement AS previous_statement,
           version.statement
    FROM entity_trait_version AS version
    JOIN entity_trait AS trait ON trait.id = version.trait_id
    JOIN entity ON entity.id = trait.entity_id
    LEFT JOIN entity_trait_version AS previous
      ON previous.trait_id = version.trait_id
     AND previous.entity_id = version.entity_id
     AND previous.activity_id = version.previous_activity_id
    WHERE version.activity_id = ANY($1::uuid[])
    ORDER BY version.activity_id, trait.entity_id, trait.id
"#;

const CURRENT_ENTITY_STATE_SQL: &str = r#"
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

#[derive(Clone, Copy)]
enum PropertyQueryKind {
    Write,
    CurrentRead,
    Hydration,
}

#[derive(Clone, Copy)]
enum TraitQueryKind {
    Write,
    CurrentRead,
    Hydration,
}

#[cfg(test)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct PropertyQueryCount {
    write: usize,
    current_read: usize,
    hydration: usize,
}

#[cfg(test)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct TraitQueryCount {
    write: usize,
    current_read: usize,
    hydration: usize,
}

#[cfg(test)]
tokio::task_local! {
    static PROPERTY_QUERY_COUNT: RefCell<PropertyQueryCount>;
}

#[cfg(test)]
tokio::task_local! {
    static TRAIT_QUERY_COUNT: RefCell<TraitQueryCount>;
}

#[inline]
fn record_property_query(kind: PropertyQueryKind) {
    #[cfg(test)]
    let _ = PROPERTY_QUERY_COUNT.try_with(|count| match kind {
        PropertyQueryKind::Write => count.borrow_mut().write += 1,
        PropertyQueryKind::CurrentRead => count.borrow_mut().current_read += 1,
        PropertyQueryKind::Hydration => count.borrow_mut().hydration += 1,
    });
    #[cfg(not(test))]
    let _ = kind;
}

#[inline]
fn record_trait_query(kind: TraitQueryKind) {
    #[cfg(test)]
    let _ = TRAIT_QUERY_COUNT.try_with(|count| match kind {
        TraitQueryKind::Write => count.borrow_mut().write += 1,
        TraitQueryKind::CurrentRead => count.borrow_mut().current_read += 1,
        TraitQueryKind::Hydration => count.borrow_mut().hydration += 1,
    });
    #[cfg(not(test))]
    let _ = kind;
}

#[derive(Clone)]
pub struct World {
    pool: PgPool,
}

impl World {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub fn get_world(&self) -> WorldView {
        WorldView {
            name: WORLD_NAME.to_owned(),
        }
    }

    pub async fn create_user(&self) -> Result<User, WorldError> {
        sqlx::query_as::<_, User>(
            r#"
            INSERT INTO "user" (id)
            VALUES ($1)
            RETURNING id, created_at
            "#,
        )
        .bind(Uuid::new_v4())
        .fetch_one(&self.pool)
        .await
        .map_err(|error| storage_error("create_user", error))
    }

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

    pub async fn list_entity(&self, request: ListEntity) -> Result<EntityPage, WorldError> {
        validate_limit(request.limit, WorldError::InvalidEntityLimit)?;

        let cursor_time = request.cursor.as_ref().map(|cursor| cursor.introduced_at);
        let cursor_id = request.cursor.as_ref().map(|cursor| cursor.entity_id.0);
        let fetch_limit = i64::from(request.limit) + 1;

        let mut row = sqlx::query_as::<_, EntityListRow>(
            r#"
            SELECT id, name, introduced_at
            FROM entity
            WHERE $1::timestamptz IS NULL
               OR (introduced_at, id) < ($1::timestamptz, $2::uuid)
            ORDER BY introduced_at DESC, id DESC
            LIMIT $3
            "#,
        )
        .bind(cursor_time)
        .bind(cursor_id)
        .bind(fetch_limit)
        .fetch_all(&self.pool)
        .await
        .map_err(|error| storage_error("list_entity", error))?;

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
            .map(|row| EntitySummary {
                id: row.id,
                name: row.name,
            })
            .collect();

        Ok(EntityPage { entity, next })
    }

    pub async fn get_entity(&self, entity_id: EntityId) -> Result<Entity, WorldError> {
        sqlx::query_as::<_, Entity>(
            r#"
            SELECT id, name, description, introduced_by_user_id, introduced_at
            FROM entity
            WHERE id = $1
            "#,
        )
        .bind(entity_id.0)
        .fetch_optional(&self.pool)
        .await
        .map_err(|error| storage_error("get_entity", error))?
        .ok_or(WorldError::EntityNotFound)
    }

    pub async fn create_entity(
        &self,
        user_id: UserId,
        input: CreateEntity,
    ) -> Result<Entity, WorldError> {
        let input = input.normalize()?;
        let mut transaction = self.begin("create_entity").await?;
        lock_user(&mut transaction, user_id, "create_entity").await?;
        let context = find_character(&mut transaction, user_id, false, "create_entity").await?;
        if let Some(place) = context
            .as_ref()
            .and_then(|character| character.current_place.as_ref())
        {
            lock_place(&mut transaction, place.entity.id, "create_entity").await?;
        }
        let entity = insert_entity(&mut transaction, user_id, input.name, input.description)
            .await
            .map_err(|error| storage_error("create_entity", error))?;

        let activity_id = append_activity(
            &mut transaction,
            ActivityDraft {
                operation: ActivityOperation::CreateEntity,
                requested_by_user_id: user_id,
                actor_character_entity_id: context.as_ref().map(|character| character.entity.id),
                context_place_entity_id: context
                    .as_ref()
                    .and_then(|character| character.current_place.as_ref())
                    .map(|place| place.entity.id),
                involved: &[(entity.id, ActivityEntityRole::Subject)],
                prose: None,
                request_id: None,
                request_fingerprint: None,
                action_consequence: None,
            },
            "create_entity",
        )
        .await?;
        let property = property_writes_for_entity(entity.id, input.property);
        write_property_changes(&mut transaction, activity_id, &property)
            .await
            .map_err(|error| map_property_error(error, "create_entity"))?;
        if let Some(place) = context
            .as_ref()
            .and_then(|character| character.current_place.as_ref())
        {
            advance_place_revision(
                &mut transaction,
                place.entity.id,
                activity_id,
                "create_entity",
            )
            .await?;
        }
        transaction
            .commit()
            .await
            .map_err(|error| storage_error("create_entity", error))?;
        Ok(entity)
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

    pub async fn create_character(
        &self,
        user_id: UserId,
        input: CreateCharacter,
    ) -> Result<Character, WorldError> {
        let input = input.normalize()?;
        let mut transaction = self.begin("create_character").await?;
        lock_user(&mut transaction, user_id, "create_character").await?;
        if find_character(&mut transaction, user_id, false, "create_character")
            .await?
            .is_some()
        {
            return Err(WorldError::CharacterAlreadyExists);
        }

        let entity = insert_entity(&mut transaction, user_id, input.name, input.description)
            .await
            .map_err(|error| storage_error("create_character", error))?;
        sqlx::query("INSERT INTO character (entity_id, owner_user_id) VALUES ($1, $2)")
            .bind(entity.id.0)
            .bind(user_id.0)
            .execute(&mut *transaction)
            .await
            .map_err(|error| storage_error("create_character", error))?;
        let activity_id = append_activity(
            &mut transaction,
            ActivityDraft {
                operation: ActivityOperation::CreateCharacter,
                requested_by_user_id: user_id,
                actor_character_entity_id: None,
                context_place_entity_id: None,
                involved: &[(entity.id, ActivityEntityRole::Subject)],
                prose: None,
                request_id: None,
                request_fingerprint: None,
                action_consequence: None,
            },
            "create_character",
        )
        .await?;
        let property = property_writes_for_entity(entity.id, input.property);
        write_property_changes(&mut transaction, activity_id, &property)
            .await
            .map_err(|error| map_property_error(error, "create_character"))?;
        transaction
            .commit()
            .await
            .map_err(|error| storage_error("create_character", error))?;

        Ok(Character {
            entity,
            owner_user_id: user_id,
            current_place: None,
        })
    }

    pub async fn create_entry_place(
        &self,
        user_id: UserId,
        input: CreateEntryPlace,
    ) -> Result<Place, WorldError> {
        let input = input.normalize()?;
        let mut transaction = self.begin("create_entry_place").await?;
        lock_user(&mut transaction, user_id, "create_entry_place").await?;
        let character = find_character(&mut transaction, user_id, true, "create_entry_place")
            .await?
            .ok_or(WorldError::CharacterNotFound)?;
        if character.current_place.is_some() {
            return Err(WorldError::CharacterAlreadyEntered);
        }
        if find_entry_place(&mut transaction, "create_entry_place")
            .await?
            .is_some()
        {
            return Err(WorldError::EntryPlaceAlreadyExists);
        }

        let entity = insert_entity(&mut transaction, user_id, input.name, input.description)
            .await
            .map_err(|error| storage_error("create_entry_place", error))?;
        let activity_id = append_activity(
            &mut transaction,
            ActivityDraft {
                operation: ActivityOperation::CreateEntryPlace,
                requested_by_user_id: user_id,
                actor_character_entity_id: Some(character.entity.id),
                context_place_entity_id: None,
                involved: &[(entity.id, ActivityEntityRole::Subject)],
                prose: None,
                request_id: None,
                request_fingerprint: None,
                action_consequence: None,
            },
            "create_entry_place",
        )
        .await?;
        let property = property_writes_for_entity(entity.id, input.property);
        write_property_changes(&mut transaction, activity_id, &property)
            .await
            .map_err(|error| map_property_error(error, "create_entry_place"))?;
        if let Err(error) = sqlx::query(
            "INSERT INTO place (entity_id, is_entry, latest_activity_id) VALUES ($1, true, $2)",
        )
        .bind(entity.id.0)
        .bind(activity_id.0)
        .execute(&mut *transaction)
        .await
        {
            if constraint(&error) == Some("place_one_entry_index") {
                return Err(WorldError::EntryPlaceAlreadyExists);
            }
            return Err(storage_error("create_entry_place", error));
        }
        transaction
            .commit()
            .await
            .map_err(|error| storage_error("create_entry_place", error))?;
        Ok(Place {
            entity,
            is_entry: true,
        })
    }

    pub async fn enter_world(&self, user_id: UserId) -> Result<Character, WorldError> {
        let mut transaction = self.begin("enter_world").await?;
        lock_user(&mut transaction, user_id, "enter_world").await?;
        let mut character = find_character(&mut transaction, user_id, true, "enter_world")
            .await?
            .ok_or(WorldError::CharacterNotFound)?;
        if character.current_place.is_some() {
            return Ok(character);
        }
        let entry_place = find_entry_place(&mut transaction, "enter_world")
            .await?
            .ok_or(WorldError::EntryPlaceNotFound)?;
        lock_place(&mut transaction, entry_place.entity.id, "enter_world").await?;

        sqlx::query(
            "UPDATE character SET current_place_entity_id = $1 WHERE entity_id = $2 AND current_place_entity_id IS NULL",
        )
        .bind(entry_place.entity.id.0)
        .bind(character.entity.id.0)
        .execute(&mut *transaction)
        .await
        .map_err(|error| storage_error("enter_world", error))?;
        let activity_id = append_activity(
            &mut transaction,
            ActivityDraft {
                operation: ActivityOperation::EnterWorld,
                requested_by_user_id: user_id,
                actor_character_entity_id: Some(character.entity.id),
                context_place_entity_id: Some(entry_place.entity.id),
                involved: &[(entry_place.entity.id, ActivityEntityRole::Destination)],
                prose: None,
                request_id: None,
                request_fingerprint: None,
                action_consequence: None,
            },
            "enter_world",
        )
        .await?;
        advance_place_revision(
            &mut transaction,
            entry_place.entity.id,
            activity_id,
            "enter_world",
        )
        .await?;
        transaction
            .commit()
            .await
            .map_err(|error| storage_error("enter_world", error))?;
        character.current_place = Some(entry_place);
        Ok(character)
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
                SELECT entity.id, entity.name, entity.description, entity.introduced_at
                FROM character
                JOIN entity ON entity.id = character.entity_id
                WHERE character.current_place_entity_id = $1
                  AND character.entity_id <> $2

                UNION ALL

                SELECT entity.id, entity.name, entity.description, entity.introduced_at
                FROM entity_location
                JOIN entity ON entity.id = entity_location.entity_id
                WHERE entity_location.place_entity_id = $1
                  AND NOT EXISTS (
                      SELECT 1 FROM character WHERE character.entity_id = entity.id
                  )
                  AND NOT EXISTS (
                      SELECT 1 FROM place WHERE place.entity_id = entity.id
                  )
            )
            SELECT id, name, description, introduced_at
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
            SELECT entity.id, entity.name, entity.description, entity.introduced_at
            FROM entity
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
            },
            current_state,
        })
    }

    pub async fn submit_action(
        &self,
        user_id: UserId,
        input: SubmitAction,
    ) -> Result<AcceptedAction, WorldError> {
        let input = input.normalize()?;
        let request_fingerprint = action_fingerprint(&input);
        let mut transaction = self.begin("submit_action").await?;
        lock_user(&mut transaction, user_id, "submit_action").await?;

        if let Some(existing) =
            find_accepted_action(&mut transaction, user_id, input.request_id, "submit_action")
                .await?
        {
            if existing.request_fingerprint != request_fingerprint {
                return Err(WorldError::ActionRequestConflict);
            }
            return Ok(existing.accepted_action);
        }

        let character = find_character(&mut transaction, user_id, true, "submit_action")
            .await?
            .ok_or(WorldError::CharacterNotFound)?;
        let place = character
            .current_place
            .ok_or(WorldError::CharacterNotEntered)?;
        lock_place(&mut transaction, place.entity.id, "submit_action").await?;
        let current_revision =
            find_place_revision(&mut transaction, place.entity.id, "submit_action").await?;
        if input.expected_place_revision != current_revision {
            return Err(WorldError::PlaceRevisionConflict);
        }

        let (involved, property, trait_change, eligible_trait_entity, action_consequence) =
            match input.consequence {
                ActionConsequence::IntroduceEntity(consequence) => {
                    let entity = insert_entity(
                        &mut transaction,
                        user_id,
                        consequence.name,
                        consequence.description,
                    )
                    .await
                    .map_err(|error| storage_error("submit_action", error))?;
                    sqlx::query(
                        "INSERT INTO entity_location (entity_id, place_entity_id) VALUES ($1, $2)",
                    )
                    .bind(entity.id.0)
                    .bind(place.entity.id.0)
                    .execute(&mut *transaction)
                    .await
                    .map_err(|error| storage_error("submit_action", error))?;
                    (
                        vec![
                            (entity.id, ActivityEntityRole::Subject),
                            (place.entity.id, ActivityEntityRole::Location),
                        ],
                        property_writes_for_entity(entity.id, consequence.property),
                        Vec::new(),
                        Vec::new(),
                        "introduce_entity",
                    )
                }
                ActionConsequence::ChangeEntityProperty(consequence) => {
                    let property = consequence
                        .property_change
                        .into_iter()
                        .map(|change| PropertyWrite {
                            entity_id: change.entity_id,
                            key: change.key,
                            value: change.value,
                        })
                        .collect::<Vec<_>>();
                    require_local_property_entity(
                        &mut transaction,
                        character.entity.id,
                        place.entity.id,
                        &property,
                        "submit_action",
                    )
                    .await?;
                    let mut subject = property
                        .iter()
                        .map(|write| write.entity_id)
                        .collect::<Vec<_>>();
                    subject
                        .sort_unstable_by(|left, right| left.0.as_bytes().cmp(right.0.as_bytes()));
                    subject.dedup();
                    let mut involved = subject
                        .into_iter()
                        .map(|entity_id| (entity_id, ActivityEntityRole::Subject))
                        .collect::<Vec<_>>();
                    involved.push((place.entity.id, ActivityEntityRole::Location));
                    (
                        involved,
                        property,
                        Vec::new(),
                        Vec::new(),
                        "change_entity_property",
                    )
                }
                ActionConsequence::ChangeEntityTrait(consequence) => {
                    let trait_change = consequence
                        .trait_change
                        .into_iter()
                        .map(|change| match change {
                            EntityTraitChangeInput::Establish {
                                entity_id,
                                statement,
                            } => TraitWrite::Establish {
                                entity_id,
                                statement,
                            },
                            EntityTraitChangeInput::Develop {
                                trait_id,
                                statement,
                            } => TraitWrite::Develop {
                                trait_id: trait_id.0,
                                statement,
                            },
                        })
                        .collect();
                    let eligible_trait_entity = find_local_entity_ids(
                        &mut transaction,
                        character.entity.id,
                        place.entity.id,
                        "submit_action",
                    )
                    .await?;
                    (
                        vec![(place.entity.id, ActivityEntityRole::Location)],
                        Vec::new(),
                        trait_change,
                        eligible_trait_entity,
                        "change_entity_trait",
                    )
                }
            };
        let activity_id = append_activity(
            &mut transaction,
            ActivityDraft {
                operation: ActivityOperation::SubmitAction,
                requested_by_user_id: user_id,
                actor_character_entity_id: Some(character.entity.id),
                context_place_entity_id: Some(place.entity.id),
                involved: &involved,
                prose: Some(&input.prose),
                request_id: Some(input.request_id),
                request_fingerprint: Some(&request_fingerprint),
                action_consequence: Some(action_consequence),
            },
            "submit_action",
        )
        .await?;
        write_property_changes(&mut transaction, activity_id, &property)
            .await
            .map_err(|error| map_property_error(error, "submit_action"))?;
        let stored_trait_change = write_trait_changes(
            &mut transaction,
            activity_id,
            &trait_change,
            &eligible_trait_entity,
        )
        .await
        .map_err(|error| map_trait_error(error, "submit_action"))?;
        if !stored_trait_change.is_empty() {
            let mut subject = stored_trait_change
                .iter()
                .map(|change| change.entity_id)
                .collect::<Vec<_>>();
            subject.sort_unstable_by(|left, right| left.0.as_bytes().cmp(right.0.as_bytes()));
            subject.dedup();
            append_activity_entity_roles(
                &mut transaction,
                activity_id,
                &subject
                    .into_iter()
                    .map(|entity_id| (entity_id, ActivityEntityRole::Subject))
                    .collect::<Vec<_>>(),
                "submit_action",
            )
            .await?;
        }
        advance_place_revision(
            &mut transaction,
            place.entity.id,
            activity_id,
            "submit_action",
        )
        .await?;
        let accepted =
            find_accepted_action(&mut transaction, user_id, input.request_id, "submit_action")
                .await?
                .ok_or_else(invalid_stored_relation)?
                .accepted_action;
        transaction
            .commit()
            .await
            .map_err(|error| storage_error("submit_action", error))?;
        Ok(accepted)
    }

    pub async fn submit_interaction(
        &self,
        user_id: UserId,
        input: SubmitInteraction,
    ) -> Result<AcceptedInteraction, WorldError> {
        let input = input.normalize()?;
        let request_fingerprint = interaction_fingerprint(&input);
        let mut transaction = self.begin("submit_interaction").await?;
        lock_user(&mut transaction, user_id, "submit_interaction").await?;

        if let Some(existing) = find_request_activity(
            &mut transaction,
            user_id,
            input.request_id,
            "submit_interaction",
        )
        .await?
        {
            if existing.operation != ActivityOperation::SubmitInteraction
                || existing.request_fingerprint != request_fingerprint
            {
                return Err(WorldError::InteractionRequestConflict);
            }
            return existing
                .into_accepted_interaction(&mut transaction, "submit_interaction")
                .await;
        }

        if input.has_duplicate_target {
            return Err(WorldError::InteractionTargetUnavailable);
        }

        let character = find_character(&mut transaction, user_id, true, "submit_interaction")
            .await?
            .ok_or(WorldError::CharacterNotFound)?;
        let place = character
            .current_place
            .ok_or(WorldError::CharacterNotEntered)?;
        lock_place(&mut transaction, place.entity.id, "submit_interaction").await?;
        let current_revision =
            find_place_revision(&mut transaction, place.entity.id, "submit_interaction").await?;
        if input.expected_place_revision != current_revision {
            return Err(WorldError::PlaceRevisionConflict);
        }

        let target_uuid = input
            .target_entity_id
            .iter()
            .map(|entity_id| entity_id.0)
            .collect::<Vec<_>>();
        let eligible_count: i64 = sqlx::query_scalar(
            r#"
            SELECT count(*)
            FROM UNNEST($1::uuid[]) AS submitted(entity_id)
            JOIN (
                SELECT character.entity_id
                FROM character
                WHERE character.current_place_entity_id = $2
                  AND character.entity_id <> $3

                UNION

                SELECT entity_location.entity_id
                FROM entity_location
                WHERE entity_location.place_entity_id = $2
                  AND NOT EXISTS (
                      SELECT 1
                      FROM character
                      WHERE character.entity_id = entity_location.entity_id
                  )
                  AND NOT EXISTS (
                      SELECT 1
                      FROM place
                      WHERE place.entity_id = entity_location.entity_id
                  )

                UNION

                SELECT $2::uuid
            ) eligible ON eligible.entity_id = submitted.entity_id
            "#,
        )
        .bind(&target_uuid)
        .bind(place.entity.id.0)
        .bind(character.entity.id.0)
        .fetch_one(&mut *transaction)
        .await
        .map_err(|error| storage_error("submit_interaction", error))?;
        if eligible_count != i64::try_from(input.target_entity_id.len()).unwrap_or(i64::MAX) {
            return Err(WorldError::InteractionTargetUnavailable);
        }
        if input.property_change.iter().any(|write| {
            write.entity_id != character.entity.id
                && input
                    .target_entity_id
                    .binary_search_by(|target| {
                        target.0.as_bytes().cmp(write.entity_id.0.as_bytes())
                    })
                    .is_err()
        }) {
            return Err(WorldError::PropertyEntityUnavailable);
        }

        let mut eligible_trait_entity = input.target_entity_id.clone();
        eligible_trait_entity.push(character.entity.id);
        eligible_trait_entity
            .sort_unstable_by(|left, right| left.0.as_bytes().cmp(right.0.as_bytes()));
        eligible_trait_entity.dedup();

        let mut involved = input
            .target_entity_id
            .iter()
            .copied()
            .map(|entity_id| (entity_id, ActivityEntityRole::Target))
            .collect::<Vec<_>>();
        involved.push((place.entity.id, ActivityEntityRole::Location));
        let activity_id = append_activity(
            &mut transaction,
            ActivityDraft {
                operation: ActivityOperation::SubmitInteraction,
                requested_by_user_id: user_id,
                actor_character_entity_id: Some(character.entity.id),
                context_place_entity_id: Some(place.entity.id),
                involved: &involved,
                prose: Some(&input.prose),
                request_id: Some(input.request_id),
                request_fingerprint: Some(&request_fingerprint),
                action_consequence: None,
            },
            "submit_interaction",
        )
        .await?;
        write_property_changes(&mut transaction, activity_id, &input.property_change)
            .await
            .map_err(|error| map_property_error(error, "submit_interaction"))?;
        write_trait_changes(
            &mut transaction,
            activity_id,
            &input.trait_change,
            &eligible_trait_entity,
        )
        .await
        .map_err(|error| map_trait_error(error, "submit_interaction"))?;
        advance_place_revision(
            &mut transaction,
            place.entity.id,
            activity_id,
            "submit_interaction",
        )
        .await?;
        let accepted = find_request_activity(
            &mut transaction,
            user_id,
            input.request_id,
            "submit_interaction",
        )
        .await?
        .ok_or_else(invalid_stored_relation)?
        .into_accepted_interaction(&mut transaction, "submit_interaction")
        .await?;
        transaction
            .commit()
            .await
            .map_err(|error| storage_error("submit_interaction", error))?;
        Ok(accepted)
    }

    async fn begin(
        &self,
        operation: &'static str,
    ) -> Result<Transaction<'_, Postgres>, WorldError> {
        self.pool
            .begin()
            .await
            .map_err(|error| storage_error(operation, error))
    }

    async fn begin_repeatable_read(
        &self,
        operation: &'static str,
    ) -> Result<Transaction<'_, Postgres>, WorldError> {
        let mut transaction = self.begin(operation).await?;
        sqlx::query("SET TRANSACTION ISOLATION LEVEL REPEATABLE READ READ ONLY")
            .execute(&mut *transaction)
            .await
            .map_err(|error| storage_error(operation, error))?;
        Ok(transaction)
    }
}

async fn lock_user(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: UserId,
    operation: &'static str,
) -> Result<(), WorldError> {
    let found: Option<Uuid> =
        sqlx::query_scalar("SELECT id FROM \"user\" WHERE id = $1 FOR UPDATE")
            .bind(user_id.0)
            .fetch_optional(&mut **transaction)
            .await
            .map_err(|error| storage_error(operation, error))?;
    found.map(|_| ()).ok_or(WorldError::UserNotFound)
}

async fn require_user(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: UserId,
    operation: &'static str,
) -> Result<(), WorldError> {
    let found: Option<Uuid> = sqlx::query_scalar("SELECT id FROM \"user\" WHERE id = $1")
        .bind(user_id.0)
        .fetch_optional(&mut **transaction)
        .await
        .map_err(|error| storage_error(operation, error))?;
    found.map(|_| ()).ok_or(WorldError::UserNotFound)
}

async fn lock_place(
    transaction: &mut Transaction<'_, Postgres>,
    place_entity_id: EntityId,
    operation: &'static str,
) -> Result<(), WorldError> {
    let found: Option<Uuid> =
        sqlx::query_scalar("SELECT entity_id FROM place WHERE entity_id = $1 FOR UPDATE")
            .bind(place_entity_id.0)
            .fetch_optional(&mut **transaction)
            .await
            .map_err(|error| storage_error(operation, error))?;
    found.map(|_| ()).ok_or_else(invalid_stored_relation)
}

async fn require_local_property_entity(
    transaction: &mut Transaction<'_, Postgres>,
    actor_entity_id: EntityId,
    place_entity_id: EntityId,
    property: &[PropertyWrite],
    operation: &'static str,
) -> Result<(), WorldError> {
    let mut submitted = property
        .iter()
        .map(|write| write.entity_id.0)
        .collect::<Vec<_>>();
    submitted.sort_unstable_by(|left, right| left.as_bytes().cmp(right.as_bytes()));
    submitted.dedup();
    let eligible_count: i64 = sqlx::query_scalar(
        r#"
        SELECT count(*)
        FROM UNNEST($1::uuid[]) AS submitted(entity_id)
        JOIN (
            SELECT character.entity_id
            FROM character
            WHERE character.current_place_entity_id = $2

            UNION

            SELECT entity_location.entity_id
            FROM entity_location
            WHERE entity_location.place_entity_id = $2
              AND NOT EXISTS (
                  SELECT 1 FROM character
                  WHERE character.entity_id = entity_location.entity_id
              )
              AND NOT EXISTS (
                  SELECT 1 FROM place
                  WHERE place.entity_id = entity_location.entity_id
              )

            UNION

            SELECT $2::uuid

            UNION

            SELECT $3::uuid
        ) eligible ON eligible.entity_id = submitted.entity_id
        "#,
    )
    .bind(&submitted)
    .bind(place_entity_id.0)
    .bind(actor_entity_id.0)
    .fetch_one(&mut **transaction)
    .await
    .map_err(|error| storage_error(operation, error))?;
    if eligible_count != i64::try_from(submitted.len()).unwrap_or(i64::MAX) {
        return Err(WorldError::PropertyEntityUnavailable);
    }
    Ok(())
}

async fn find_local_entity_ids(
    transaction: &mut Transaction<'_, Postgres>,
    actor_entity_id: EntityId,
    place_entity_id: EntityId,
    operation: &'static str,
) -> Result<Vec<EntityId>, WorldError> {
    sqlx::query_scalar::<_, EntityId>(
        r#"
        SELECT character.entity_id
        FROM character
        WHERE character.current_place_entity_id = $1

        UNION

        SELECT entity_location.entity_id
        FROM entity_location
        WHERE entity_location.place_entity_id = $1
          AND NOT EXISTS (
              SELECT 1 FROM character
              WHERE character.entity_id = entity_location.entity_id
          )
          AND NOT EXISTS (
              SELECT 1 FROM place
              WHERE place.entity_id = entity_location.entity_id
          )

        UNION

        SELECT $1::uuid

        UNION

        SELECT $2::uuid

        ORDER BY 1
        "#,
    )
    .bind(place_entity_id.0)
    .bind(actor_entity_id.0)
    .fetch_all(&mut **transaction)
    .await
    .map_err(|error| storage_error(operation, error))
}

async fn find_place_revision(
    transaction: &mut Transaction<'_, Postgres>,
    place_entity_id: EntityId,
    operation: &'static str,
) -> Result<PlaceRevision, WorldError> {
    let row: Option<(DateTime<Utc>, ActivityId)> = sqlx::query_as(
        r#"
        SELECT activity.occurred_at, activity.id
        FROM place
        JOIN activity ON activity.id = place.latest_activity_id
        WHERE place.entity_id = $1
        "#,
    )
    .bind(place_entity_id.0)
    .fetch_optional(&mut **transaction)
    .await
    .map_err(|error| storage_error(operation, error))?;
    let (occurred_at, activity_id) = row.ok_or_else(invalid_stored_relation)?;
    Ok(PlaceRevision::from_parts(
        place_entity_id,
        occurred_at,
        activity_id,
    ))
}

async fn advance_place_revision(
    transaction: &mut Transaction<'_, Postgres>,
    place_entity_id: EntityId,
    activity_id: ActivityId,
    operation: &'static str,
) -> Result<(), WorldError> {
    let result = sqlx::query("UPDATE place SET latest_activity_id = $1 WHERE entity_id = $2")
        .bind(activity_id.0)
        .bind(place_entity_id.0)
        .execute(&mut **transaction)
        .await
        .map_err(|error| storage_error(operation, error))?;
    if result.rows_affected() != 1 {
        return Err(invalid_stored_relation());
    }
    Ok(())
}

async fn hydrate_entity_current_state(
    transaction: &mut Transaction<'_, Postgres>,
    entity_id: EntityId,
    place_revision: Option<PlaceRevision>,
    cursor: Option<EntityCurrentStateCursor>,
    limit: u16,
    operation: &'static str,
) -> Result<EntityCurrentStatePage, WorldError> {
    if let Some(cursor) = cursor {
        if cursor.entity_id != entity_id {
            return Err(WorldError::InvalidRequest);
        }
        if cursor.place_revision != place_revision {
            return Err(WorldError::PlaceRevisionConflict);
        }
    }
    let cursor_type = cursor.map(|cursor| match cursor.key {
        EntityCurrentStateCursorKey::Property(_) => 0_i16,
        EntityCurrentStateCursorKey::Trait(_) => 1_i16,
    });
    let cursor_property_key_id = cursor.and_then(EntityCurrentStateCursor::property_key_id);
    let cursor_trait_id = cursor
        .and_then(EntityCurrentStateCursor::trait_id)
        .map(|id| id.0);
    let fetch_limit = i64::from(limit) + 1;
    record_property_query(PropertyQueryKind::CurrentRead);
    record_trait_query(TraitQueryKind::CurrentRead);
    let mut row = sqlx::query_as::<_, EntityCurrentAssociationRow>(CURRENT_ENTITY_STATE_SQL)
        .bind(entity_id.0)
        .bind(cursor_type)
        .bind(cursor_property_key_id)
        .bind(cursor_trait_id)
        .bind(fetch_limit)
        .fetch_all(&mut **transaction)
        .await
        .map_err(|error| storage_error(operation, error))?;
    let has_more = row.len() > usize::from(limit);
    if has_more {
        row.pop();
    }
    let association = row
        .into_iter()
        .map(EntityCurrentAssociationRow::into_association)
        .collect::<Result<Vec<_>, _>>()?;
    let next = has_more.then(|| {
        let key = association
            .last()
            .expect("a current-state continuation always follows one returned association")
            .0;
        EntityCurrentStateCursor {
            entity_id,
            place_revision,
            key,
        }
    });
    Ok(EntityCurrentStatePage {
        association: association
            .into_iter()
            .map(|(_, association)| association)
            .collect(),
        next,
    })
}

fn action_fingerprint(input: &SubmitAction) -> Vec<u8> {
    let mut hash = Sha256::new();
    for field in [
        b"aicadia-submit-action-fingerprint-v1".as_slice(),
        input.expected_place_revision.fingerprint_bytes().as_slice(),
        input.prose.as_bytes(),
    ] {
        fingerprint_field(&mut hash, field);
    }
    match &input.consequence {
        ActionConsequence::IntroduceEntity(consequence) => {
            for field in [
                b"introduce_entity".as_slice(),
                consequence.name.as_bytes(),
                consequence.description.as_bytes(),
            ] {
                fingerprint_field(&mut hash, field);
            }
            fingerprint_property_input(&mut hash, &consequence.property);
        }
        ActionConsequence::ChangeEntityProperty(consequence) => {
            fingerprint_field(&mut hash, b"change_entity_property");
            fingerprint_property_change(&mut hash, &consequence.property_change);
        }
        ActionConsequence::ChangeEntityTrait(consequence) => {
            fingerprint_field(&mut hash, b"change_entity_trait");
            fingerprint_trait_change(&mut hash, &consequence.trait_change);
        }
    }
    hash.finalize().to_vec()
}

fn interaction_fingerprint(input: &NormalizedSubmitInteraction) -> Vec<u8> {
    let mut hash = Sha256::new();
    for field in [
        b"aicadia-submit-interaction-fingerprint-v1".as_slice(),
        input.expected_place_revision.fingerprint_bytes().as_slice(),
        input.prose.as_bytes(),
    ] {
        fingerprint_field(&mut hash, field);
    }
    for target_entity_id in &input.target_entity_id {
        let field = target_entity_id.0.as_bytes();
        fingerprint_field(&mut hash, field);
    }
    let property_change = input
        .property_change
        .iter()
        .map(|write| EntityPropertyChangeInput {
            entity_id: write.entity_id,
            key: write.key.clone(),
            value: write.value.clone(),
        })
        .collect::<Vec<_>>();
    fingerprint_property_change(&mut hash, &property_change);
    fingerprint_trait_writes(&mut hash, &input.trait_change);
    hash.finalize().to_vec()
}

fn fingerprint_field(hash: &mut Sha256, field: &[u8]) {
    hash.update((field.len() as u64).to_be_bytes());
    hash.update(field);
}

fn fingerprint_property_input(hash: &mut Sha256, property: &[PropertyInput]) {
    for property in property {
        fingerprint_field(hash, property.key.as_bytes());
        fingerprint_property_value(hash, &property.value);
    }
}

fn fingerprint_property_change(hash: &mut Sha256, property: &[EntityPropertyChangeInput]) {
    for property in property {
        fingerprint_field(hash, property.entity_id.0.as_bytes());
        fingerprint_field(hash, property.key.as_bytes());
        fingerprint_property_value(hash, &property.value);
    }
}

fn fingerprint_property_value(hash: &mut Sha256, value: &PropertyValue) {
    match value {
        PropertyValue::Text(value) => {
            fingerprint_field(hash, b"text");
            fingerprint_field(hash, value.as_bytes());
        }
        PropertyValue::Integer(value) => {
            fingerprint_field(hash, b"integer");
            fingerprint_field(hash, &value.to_be_bytes());
        }
    }
}

fn fingerprint_trait_change(hash: &mut Sha256, change: &[EntityTraitChangeInput]) {
    for change in change {
        match change {
            EntityTraitChangeInput::Establish {
                entity_id,
                statement,
            } => {
                fingerprint_field(hash, b"establish");
                fingerprint_field(hash, entity_id.0.as_bytes());
                fingerprint_field(hash, statement.as_bytes());
            }
            EntityTraitChangeInput::Develop {
                trait_id,
                statement,
            } => {
                fingerprint_field(hash, b"develop");
                fingerprint_field(hash, trait_id.0.as_bytes());
                fingerprint_field(hash, statement.as_bytes());
            }
        }
    }
}

fn fingerprint_trait_writes(hash: &mut Sha256, change: &[TraitWrite]) {
    for change in change {
        let statement = change.statement();
        match change {
            TraitWrite::Establish {
                entity_id,
                statement: _,
            } => {
                fingerprint_field(hash, b"establish");
                fingerprint_field(hash, entity_id.0.as_bytes());
                fingerprint_field(hash, statement.as_bytes());
            }
            TraitWrite::Develop {
                trait_id,
                statement: _,
            } => {
                fingerprint_field(hash, b"develop");
                fingerprint_field(hash, trait_id.as_bytes());
                fingerprint_field(hash, statement.as_bytes());
            }
        }
    }
}

async fn activities_from_rows(
    transaction: &mut Transaction<'_, Postgres>,
    row: Vec<ActivityRow>,
    operation: &'static str,
) -> Result<Vec<Activity>, WorldError> {
    let typed_activity_ids = row.iter().map(|row| row.id).collect::<Vec<_>>();
    let activity_ids = typed_activity_ids.iter().map(|id| id.0).collect::<Vec<_>>();
    let related = if activity_ids.is_empty() {
        Vec::new()
    } else {
        sqlx::query_as::<_, ActivityEntityRow>(
            r#"
            SELECT activity_entity.activity_id, activity_entity.entity_id,
                   entity.name, activity_entity.role
            FROM activity_entity
            JOIN entity ON entity.id = activity_entity.entity_id
            WHERE activity_entity.activity_id = ANY($1)
            ORDER BY activity_entity.activity_id, activity_entity.role, activity_entity.entity_id
            "#,
        )
        .bind(&activity_ids)
        .fetch_all(&mut **transaction)
        .await
        .map_err(|error| storage_error(operation, error))?
    };
    let mut involved_by_activity: HashMap<ActivityId, Vec<ActivityEntityReference>> =
        HashMap::new();
    for related in related {
        involved_by_activity
            .entry(related.activity_id)
            .or_default()
            .push(related.try_into()?);
    }
    let mut property_by_activity = hydrate_property_changes(transaction, &typed_activity_ids)
        .await
        .map_err(|error| map_property_error(error, operation))?;
    let mut trait_by_activity = hydrate_trait_changes(transaction, &typed_activity_ids)
        .await
        .map_err(|error| map_trait_error(error, operation))?;
    row.into_iter()
        .map(|row| {
            let id = row.id;
            let property_change = property_by_activity
                .remove(&id)
                .unwrap_or_default()
                .into_iter()
                .map(EntityPropertyChange::from)
                .collect();
            let trait_change = trait_by_activity
                .remove(&id)
                .unwrap_or_default()
                .into_iter()
                .map(ActivityTraitChange::from)
                .collect();
            row.into_activity(
                involved_by_activity.remove(&id).unwrap_or_default(),
                property_change,
                trait_change,
            )
        })
        .collect()
}

struct StoredAcceptedAction {
    request_fingerprint: Vec<u8>,
    accepted_action: AcceptedAction,
}

struct StoredRequestActivity {
    operation: ActivityOperation,
    request_fingerprint: Vec<u8>,
    activity: Activity,
}

impl StoredRequestActivity {
    async fn into_accepted_interaction(
        self,
        transaction: &mut Transaction<'_, Postgres>,
        operation: &'static str,
    ) -> Result<AcceptedInteraction, WorldError> {
        if self.operation != ActivityOperation::SubmitInteraction {
            return Err(invalid_stored_relation());
        }
        let location_id = self
            .activity
            .involved_entity
            .iter()
            .find(|reference| reference.role == ActivityEntityRole::Location)
            .map(|reference| reference.entity.id)
            .ok_or_else(invalid_stored_relation)?;
        let target_count = self
            .activity
            .involved_entity
            .iter()
            .filter(|reference| reference.role == ActivityEntityRole::Target)
            .count();
        if !(1..=MAX_INTERACTION_TARGET_COUNT).contains(&target_count) {
            return Err(invalid_stored_relation());
        }
        let place = find_place_by_id(transaction, location_id, operation)
            .await?
            .ok_or_else(invalid_stored_relation)?;
        Ok(AcceptedInteraction {
            activity: self.activity,
            place,
        })
    }
}

async fn find_request_activity(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: UserId,
    request_id: Uuid,
    operation: &'static str,
) -> Result<Option<StoredRequestActivity>, WorldError> {
    let row = sqlx::query_as::<_, AcceptedActionActivityRow>(
        r#"
        SELECT activity.id, activity.operation, activity.prose, activity.occurred_at,
               activity.request_fingerprint, activity.action_consequence,
               actor.id AS actor_entity_id, actor.name AS actor_name,
               context.id AS context_entity_id, context.name AS context_name,
               context_place.is_entry AS context_is_entry
        FROM activity
        LEFT JOIN entity actor ON actor.id = activity.actor_character_entity_id
        LEFT JOIN place context_place ON context_place.entity_id = activity.context_place_entity_id
        LEFT JOIN entity context ON context.id = context_place.entity_id
        WHERE activity.requested_by_user_id = $1
          AND activity.request_id = $2
        "#,
    )
    .bind(user_id.0)
    .bind(request_id)
    .fetch_optional(&mut **transaction)
    .await
    .map_err(|error| storage_error(operation, error))?;
    let Some(row) = row else {
        return Ok(None);
    };
    let request_fingerprint = row.request_fingerprint.clone();
    let activity = activities_from_rows(transaction, vec![row.into()], operation)
        .await?
        .pop()
        .ok_or_else(invalid_stored_relation)?;
    Ok(Some(StoredRequestActivity {
        operation: activity.operation,
        request_fingerprint,
        activity,
    }))
}

async fn find_accepted_action(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: UserId,
    request_id: Uuid,
    operation: &'static str,
) -> Result<Option<StoredAcceptedAction>, WorldError> {
    let row = sqlx::query_as::<_, AcceptedActionActivityRow>(
        r#"
        SELECT activity.id, activity.operation, activity.prose, activity.occurred_at,
               activity.request_fingerprint, activity.action_consequence,
               actor.id AS actor_entity_id, actor.name AS actor_name,
               context.id AS context_entity_id, context.name AS context_name,
               context_place.is_entry AS context_is_entry
        FROM activity
        LEFT JOIN entity actor ON actor.id = activity.actor_character_entity_id
        LEFT JOIN place context_place ON context_place.entity_id = activity.context_place_entity_id
        LEFT JOIN entity context ON context.id = context_place.entity_id
        WHERE activity.requested_by_user_id = $1
          AND activity.request_id = $2
        "#,
    )
    .bind(user_id.0)
    .bind(request_id)
    .fetch_optional(&mut **transaction)
    .await
    .map_err(|error| storage_error(operation, error))?;
    let Some(row) = row else {
        return Ok(None);
    };
    let activity_id = row.id;
    let action_consequence = row.action_consequence.clone();
    let request_fingerprint = row.request_fingerprint.clone();
    let activity = activities_from_rows(transaction, vec![row.into()], operation)
        .await?
        .pop()
        .ok_or_else(invalid_stored_relation)?;
    if activity.operation != ActivityOperation::SubmitAction {
        return Err(WorldError::ActionRequestConflict);
    }
    let location_id = activity
        .involved_entity
        .iter()
        .find(|reference| reference.role == ActivityEntityRole::Location)
        .map(|reference| reference.entity.id)
        .ok_or_else(invalid_stored_relation)?;
    let consequence = match action_consequence.as_deref() {
        Some("introduce_entity") => {
            let subject_id = activity
                .involved_entity
                .iter()
                .find(|reference| reference.role == ActivityEntityRole::Subject)
                .map(|reference| reference.entity.id)
                .ok_or_else(invalid_stored_relation)?;
            let entity = sqlx::query_as::<_, Entity>(
                r#"
                SELECT entity.id, entity.name, entity.description,
                       entity.introduced_by_user_id, entity.introduced_at
                FROM entity_location
                JOIN entity ON entity.id = entity_location.entity_id
                WHERE entity_location.entity_id = $1
                  AND entity_location.place_entity_id = $2
                "#,
            )
            .bind(subject_id.0)
            .bind(location_id.0)
            .fetch_optional(&mut **transaction)
            .await
            .map_err(|error| storage_error(operation, error))?
            .ok_or_else(invalid_stored_relation)?;
            AcceptedActionConsequence::IntroduceEntity(entity)
        }
        Some("change_entity_property") if !activity.property_change.is_empty() => {
            AcceptedActionConsequence::ChangeEntityProperty(activity.property_change.clone())
        }
        Some("change_entity_trait") if !activity.trait_change.is_empty() => {
            AcceptedActionConsequence::ChangeEntityTrait(activity.trait_change.clone())
        }
        _ => return Err(invalid_stored_relation()),
    };
    let place = find_place_by_id(transaction, location_id, operation)
        .await?
        .ok_or_else(invalid_stored_relation)?;
    debug_assert_eq!(activity.id, activity_id);
    Ok(Some(StoredAcceptedAction {
        request_fingerprint,
        accepted_action: AcceptedAction {
            activity,
            consequence,
            place,
        },
    }))
}

async fn insert_entity(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: UserId,
    name: String,
    description: String,
) -> Result<Entity, sqlx::Error> {
    sqlx::query_as::<_, Entity>(
        r#"
        INSERT INTO entity (id, name, description, introduced_by_user_id)
        VALUES ($1, $2, $3, $4)
        RETURNING id, name, description, introduced_by_user_id, introduced_at
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(name)
    .bind(description)
    .bind(user_id.0)
    .fetch_one(&mut **transaction)
    .await
}

async fn find_character(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: UserId,
    for_update: bool,
    operation: &'static str,
) -> Result<Option<Character>, WorldError> {
    sqlx::query_as::<_, CharacterRow>(&character_query(for_update))
        .bind(user_id.0)
        .fetch_optional(&mut **transaction)
        .await
        .map_err(|error| storage_error(operation, error))?
        .map(CharacterRow::try_into)
        .transpose()
}

fn character_query(for_update: bool) -> String {
    format!(
        r#"
        SELECT entity.id AS entity_id, entity.name, entity.description,
               entity.introduced_by_user_id, entity.introduced_at,
               character.owner_user_id,
               place_entity.id AS place_entity_id, place_entity.name AS place_name,
               place_entity.description AS place_description,
               place_entity.introduced_by_user_id AS place_introduced_by_user_id,
               place_entity.introduced_at AS place_introduced_at,
               place.is_entry AS place_is_entry
        FROM character
        JOIN entity ON entity.id = character.entity_id
        LEFT JOIN place ON place.entity_id = character.current_place_entity_id
        LEFT JOIN entity place_entity ON place_entity.id = place.entity_id
        WHERE character.owner_user_id = $1
        {}
        "#,
        if for_update {
            "FOR UPDATE OF character"
        } else {
            ""
        }
    )
}

async fn find_entry_place(
    transaction: &mut Transaction<'_, Postgres>,
    operation: &'static str,
) -> Result<Option<Place>, WorldError> {
    sqlx::query_as::<_, PlaceRow>(
        r#"
        SELECT entity.id AS entity_id, entity.name, entity.description,
               entity.introduced_by_user_id, entity.introduced_at, place.is_entry
        FROM place
        JOIN entity ON entity.id = place.entity_id
        WHERE place.is_entry
        "#,
    )
    .fetch_optional(&mut **transaction)
    .await
    .map_err(|error| storage_error(operation, error))
    .map(|row| row.map(Into::into))
}

async fn find_place_by_id(
    transaction: &mut Transaction<'_, Postgres>,
    place_entity_id: EntityId,
    operation: &'static str,
) -> Result<Option<Place>, WorldError> {
    sqlx::query_as::<_, PlaceRow>(
        r#"
        SELECT entity.id AS entity_id, entity.name, entity.description,
               entity.introduced_by_user_id, entity.introduced_at, place.is_entry
        FROM place
        JOIN entity ON entity.id = place.entity_id
        WHERE place.entity_id = $1
        "#,
    )
    .bind(place_entity_id.0)
    .fetch_optional(&mut **transaction)
    .await
    .map_err(|error| storage_error(operation, error))
    .map(|row| row.map(Into::into))
}

const MAX_PROPERTY_COUNT: usize = 100;
const MAX_PROPERTY_KEY_LENGTH: usize = 64;
const MAX_PROPERTY_TEXT_LENGTH: usize = 4_000;
const MAX_TRAIT_COUNT: usize = 100;
const MAX_TRAIT_STATEMENT_LENGTH: usize = 4_000;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PropertyValue {
    Text(String),
    Integer(i64),
}

impl PropertyValue {
    fn value_type(&self) -> &'static str {
        match self {
            Self::Text(_) => "text",
            Self::Integer(_) => "integer",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct PropertyWrite {
    entity_id: EntityId,
    key: String,
    value: PropertyValue,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct StoredPropertyChange {
    entity_id: EntityId,
    entity_name: String,
    key: String,
    value: PropertyValue,
}

impl From<StoredPropertyChange> for EntityPropertyChange {
    fn from(value: StoredPropertyChange) -> Self {
        Self {
            entity: EntitySummary {
                id: value.entity_id,
                name: value.entity_name,
            },
            key: value.key,
            value: value.value,
        }
    }
}

#[derive(Debug)]
enum PropertyPersistenceError {
    KeyConflict,
    InvalidStoredRelation,
    Storage(Box<sqlx::Error>),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PropertyNormalizationError {
    InvalidCount,
    InvalidKey,
    InvalidText(InvalidReason),
    DuplicateEntityKey,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum TraitWrite {
    Establish {
        entity_id: EntityId,
        statement: String,
    },
    Develop {
        trait_id: Uuid,
        statement: String,
    },
}

impl TraitWrite {
    fn statement(&self) -> &str {
        match self {
            Self::Establish { statement, .. } | Self::Develop { statement, .. } => statement,
        }
    }

    fn statement_mut(&mut self) -> &mut String {
        match self {
            Self::Establish { statement, .. } | Self::Develop { statement, .. } => statement,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum StoredTraitLifecycle {
    Establish,
    Develop,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct StoredTraitChange {
    lifecycle: StoredTraitLifecycle,
    trait_id: Uuid,
    entity_id: EntityId,
    entity_name: String,
    previous_statement: Option<String>,
    statement: String,
}

impl From<StoredTraitChange> for ActivityTraitChange {
    fn from(value: StoredTraitChange) -> Self {
        let entity = EntitySummary {
            id: value.entity_id,
            name: value.entity_name,
        };
        let r#trait = EntityTrait {
            id: EntityTraitId(value.trait_id),
            statement: value.statement,
        };
        match (value.lifecycle, value.previous_statement) {
            (StoredTraitLifecycle::Establish, None) => Self::Establish { entity, r#trait },
            (StoredTraitLifecycle::Develop, Some(previous_statement)) => Self::Develop {
                entity,
                r#trait,
                previous_statement,
            },
            _ => unreachable!("stored Trait lifecycle is validated during hydration"),
        }
    }
}

#[derive(Debug)]
enum TraitPersistenceError {
    InvalidInput,
    Unavailable,
    InvalidStoredRelation,
    Storage(Box<sqlx::Error>),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TraitNormalizationError {
    InvalidCount,
    InvalidStatement(InvalidReason),
    DuplicateEstablishment,
    DuplicateDevelopment,
}

fn map_trait_error(error: TraitPersistenceError, operation: &'static str) -> WorldError {
    match error {
        TraitPersistenceError::InvalidInput => WorldError::InvalidTrait,
        TraitPersistenceError::Unavailable => WorldError::TraitUnavailable,
        TraitPersistenceError::InvalidStoredRelation => invalid_stored_relation(),
        TraitPersistenceError::Storage(error) => storage_error(operation, *error),
    }
}

fn map_trait_normalization_error(_error: TraitNormalizationError) -> WorldError {
    WorldError::InvalidTrait
}

fn normalize_trait_change_input(
    input: Vec<EntityTraitChangeInput>,
    allow_empty: bool,
) -> Result<Vec<TraitWrite>, WorldError> {
    let writes = input
        .into_iter()
        .map(|change| match change {
            EntityTraitChangeInput::Establish {
                entity_id,
                statement,
            } => TraitWrite::Establish {
                entity_id,
                statement,
            },
            EntityTraitChangeInput::Develop {
                trait_id,
                statement,
            } => TraitWrite::Develop {
                trait_id: trait_id.0,
                statement,
            },
        })
        .collect();
    normalize_trait_writes(writes, allow_empty).map_err(map_trait_normalization_error)
}

fn trait_input_from_writes(writes: Vec<TraitWrite>) -> Vec<EntityTraitChangeInput> {
    writes
        .into_iter()
        .map(|write| match write {
            TraitWrite::Establish {
                entity_id,
                statement,
            } => EntityTraitChangeInput::Establish {
                entity_id,
                statement,
            },
            TraitWrite::Develop {
                trait_id,
                statement,
            } => EntityTraitChangeInput::Develop {
                trait_id: EntityTraitId(trait_id),
                statement,
            },
        })
        .collect()
}

trait NormalizableProperty {
    fn entity_id(&self) -> Option<EntityId>;
    fn key(&self) -> &str;
    fn value_mut(&mut self) -> &mut PropertyValue;
}

impl NormalizableProperty for PropertyInput {
    fn entity_id(&self) -> Option<EntityId> {
        None
    }

    fn key(&self) -> &str {
        &self.key
    }

    fn value_mut(&mut self) -> &mut PropertyValue {
        &mut self.value
    }
}

impl NormalizableProperty for PropertyWrite {
    fn entity_id(&self) -> Option<EntityId> {
        Some(self.entity_id)
    }

    fn key(&self) -> &str {
        &self.key
    }

    fn value_mut(&mut self) -> &mut PropertyValue {
        &mut self.value
    }
}

fn property_storage_error(error: sqlx::Error) -> PropertyPersistenceError {
    PropertyPersistenceError::Storage(Box::new(error))
}

fn property_writes_for_entity(
    entity_id: EntityId,
    property: Vec<PropertyInput>,
) -> Vec<PropertyWrite> {
    property
        .into_iter()
        .map(|property| PropertyWrite {
            entity_id,
            key: property.key,
            value: property.value,
        })
        .collect()
}

fn map_property_error(error: PropertyPersistenceError, operation: &'static str) -> WorldError {
    match error {
        PropertyPersistenceError::KeyConflict => WorldError::PropertyKeyConflict,
        PropertyPersistenceError::InvalidStoredRelation => invalid_stored_relation(),
        PropertyPersistenceError::Storage(error) => storage_error(operation, *error),
    }
}

fn map_property_normalization_error(
    error: PropertyNormalizationError,
    field: PropertyField,
) -> WorldError {
    match error {
        PropertyNormalizationError::InvalidCount => WorldError::InvalidProperty {
            field,
            reason: InvalidReason::OutOfRange,
        },
        PropertyNormalizationError::InvalidKey => WorldError::InvalidProperty {
            field: PropertyField::Key,
            reason: InvalidReason::InvalidFormat,
        },
        PropertyNormalizationError::InvalidText(reason) => WorldError::InvalidProperty {
            field: PropertyField::Value,
            reason,
        },
        PropertyNormalizationError::DuplicateEntityKey => WorldError::InvalidProperty {
            field,
            reason: InvalidReason::Duplicate,
        },
    }
}

fn normalize_property_input(
    property: Vec<PropertyInput>,
    field: PropertyField,
) -> Result<Vec<PropertyInput>, WorldError> {
    normalize_property(property, true)
        .map_err(|error| map_property_normalization_error(error, field))
}

fn normalize_property_writes(
    writes: Vec<PropertyWrite>,
    allow_empty: bool,
) -> Result<Vec<PropertyWrite>, PropertyNormalizationError> {
    normalize_property(writes, allow_empty)
}

fn normalize_property<T: NormalizableProperty>(
    mut property: Vec<T>,
    allow_empty: bool,
) -> Result<Vec<T>, PropertyNormalizationError> {
    if property.len() > MAX_PROPERTY_COUNT || (!allow_empty && property.is_empty()) {
        return Err(PropertyNormalizationError::InvalidCount);
    }
    for item in &mut property {
        if !is_canonical_property_key(item.key()) {
            return Err(PropertyNormalizationError::InvalidKey);
        }
        if let PropertyValue::Text(value) = item.value_mut() {
            *value = value.trim().to_owned();
            let reason = if value.is_empty() {
                Some(InvalidReason::Empty)
            } else if value.contains('\0') {
                Some(InvalidReason::ContainsNul)
            } else if value.chars().count() > MAX_PROPERTY_TEXT_LENGTH {
                Some(InvalidReason::TooLong)
            } else {
                None
            };
            if let Some(reason) = reason {
                return Err(PropertyNormalizationError::InvalidText(reason));
            }
        }
    }
    property.sort_unstable_by(|left, right| {
        match (left.entity_id(), right.entity_id()) {
            (Some(left), Some(right)) => left.0.as_bytes().cmp(right.0.as_bytes()),
            (None, None) => std::cmp::Ordering::Equal,
            (None, Some(_)) => std::cmp::Ordering::Less,
            (Some(_), None) => std::cmp::Ordering::Greater,
        }
        .then_with(|| left.key().cmp(right.key()))
    });
    if property
        .windows(2)
        .any(|pair| pair[0].entity_id() == pair[1].entity_id() && pair[0].key() == pair[1].key())
    {
        return Err(PropertyNormalizationError::DuplicateEntityKey);
    }
    Ok(property)
}

fn is_canonical_property_key(key: &str) -> bool {
    let bytes = key.as_bytes();
    if bytes.is_empty() || bytes.len() > MAX_PROPERTY_KEY_LENGTH || !bytes[0].is_ascii_lowercase() {
        return false;
    }
    let mut previous_underscore = false;
    for byte in bytes {
        if *byte == b'_' {
            if previous_underscore {
                return false;
            }
            previous_underscore = true;
        } else if byte.is_ascii_lowercase() || byte.is_ascii_digit() {
            previous_underscore = false;
        } else {
            return false;
        }
    }
    !previous_underscore
}

#[derive(FromRow)]
struct PropertyKeyRow {
    id: i64,
    key: String,
    value_type: String,
}

async fn resolve_property_keys(
    transaction: &mut Transaction<'_, Postgres>,
    first_activity_id: ActivityId,
    writes: &[PropertyWrite],
) -> Result<HashMap<String, i64>, PropertyPersistenceError> {
    let mut requested_type = BTreeMap::<String, &'static str>::new();
    for write in writes {
        match requested_type.get(&write.key) {
            Some(value_type) if *value_type != write.value.value_type() => {
                return Err(PropertyPersistenceError::KeyConflict);
            }
            Some(_) => {}
            None => {
                requested_type.insert(write.key.clone(), write.value.value_type());
            }
        }
    }
    if requested_type.is_empty() {
        return Ok(HashMap::new());
    }
    let key = requested_type.keys().cloned().collect::<Vec<_>>();
    let value_type = requested_type.values().copied().collect::<Vec<_>>();
    record_property_query(PropertyQueryKind::Write);
    sqlx::query(INSERT_PROPERTY_KEY_SQL)
        .bind(&key)
        .bind(&value_type)
        .bind(first_activity_id.0)
        .execute(&mut **transaction)
        .await
        .map_err(property_storage_error)?;

    record_property_query(PropertyQueryKind::Write);
    let row = sqlx::query_as::<_, PropertyKeyRow>(
        r#"
        SELECT id, key, value_type
        FROM property_key
        WHERE key = ANY($1::text[])
        ORDER BY key
        FOR KEY SHARE
        "#,
    )
    .bind(&key)
    .fetch_all(&mut **transaction)
    .await
    .map_err(property_storage_error)?;
    if row.len() != requested_type.len() {
        return Err(PropertyPersistenceError::InvalidStoredRelation);
    }
    let mut resolved = HashMap::with_capacity(row.len());
    for row in row {
        if requested_type.get(&row.key).copied() != Some(row.value_type.as_str()) {
            return Err(PropertyPersistenceError::KeyConflict);
        }
        resolved.insert(row.key, row.id);
    }
    Ok(resolved)
}

async fn write_property_changes(
    transaction: &mut Transaction<'_, Postgres>,
    activity_id: ActivityId,
    writes: &[PropertyWrite],
) -> Result<Vec<StoredPropertyChange>, PropertyPersistenceError> {
    if writes.is_empty() {
        return Ok(Vec::new());
    }
    let key_id = resolve_property_keys(transaction, activity_id, writes).await?;
    let mut resolved = writes
        .iter()
        .map(|write| {
            let property_key_id = key_id
                .get(&write.key)
                .copied()
                .ok_or(PropertyPersistenceError::InvalidStoredRelation)?;
            Ok((write, property_key_id))
        })
        .collect::<Result<Vec<_>, PropertyPersistenceError>>()?;
    resolved.sort_unstable_by(|(left, left_key_id), (right, right_key_id)| {
        left.entity_id
            .0
            .as_bytes()
            .cmp(right.entity_id.0.as_bytes())
            .then_with(|| left_key_id.cmp(right_key_id))
    });

    let entity_id = resolved
        .iter()
        .map(|(write, _)| write.entity_id.0)
        .collect::<Vec<_>>();
    let property_key_id = resolved
        .iter()
        .map(|(_, property_key_id)| *property_key_id)
        .collect::<Vec<_>>();
    record_property_query(PropertyQueryKind::Write);
    let existing = sqlx::query_as::<_, (Uuid, i64, Uuid)>(LOCK_CURRENT_ENTITY_PROPERTY_SQL)
        .bind(&entity_id)
        .bind(&property_key_id)
        .fetch_all(&mut **transaction)
        .await
        .map_err(property_storage_error)?;
    let previous = existing
        .into_iter()
        .map(|(entity_id, property_key_id, activity_id)| {
            ((entity_id, property_key_id), activity_id)
        })
        .collect::<HashMap<_, _>>();

    let previous_activity_id = resolved
        .iter()
        .map(|(write, property_key_id)| {
            previous
                .get(&(write.entity_id.0, *property_key_id))
                .copied()
        })
        .collect::<Vec<_>>();
    let value_type = resolved
        .iter()
        .map(|(write, _)| write.value.value_type())
        .collect::<Vec<_>>();
    let text_value = resolved
        .iter()
        .map(|(write, _)| match &write.value {
            PropertyValue::Text(value) => Some(value.as_str()),
            PropertyValue::Integer(_) => None,
        })
        .collect::<Vec<_>>();
    let integer_value = resolved
        .iter()
        .map(|(write, _)| match &write.value {
            PropertyValue::Text(_) => None,
            PropertyValue::Integer(value) => Some(*value),
        })
        .collect::<Vec<_>>();
    record_property_query(PropertyQueryKind::Write);
    sqlx::query(INSERT_ENTITY_PROPERTY_HISTORY_SQL)
        .bind(&entity_id)
        .bind(&property_key_id)
        .bind(activity_id.0)
        .bind(&previous_activity_id)
        .bind(&value_type)
        .bind(&text_value)
        .bind(&integer_value)
        .execute(&mut **transaction)
        .await
        .map_err(property_storage_error)?;
    record_property_query(PropertyQueryKind::Write);
    sqlx::query(UPSERT_CURRENT_ENTITY_PROPERTY_SQL)
        .bind(&entity_id)
        .bind(&property_key_id)
        .bind(activity_id.0)
        .execute(&mut **transaction)
        .await
        .map_err(property_storage_error)?;
    hydrate_property_changes(transaction, &[activity_id])
        .await
        .map(|mut change| change.remove(&activity_id).unwrap_or_default())
}

#[derive(FromRow)]
struct StoredPropertyChangeRow {
    activity_id: ActivityId,
    entity_id: EntityId,
    entity_name: String,
    key: String,
    value_type: String,
    text_value: Option<String>,
    integer_value: Option<i64>,
}

async fn hydrate_property_changes(
    transaction: &mut Transaction<'_, Postgres>,
    activity_id: &[ActivityId],
) -> Result<HashMap<ActivityId, Vec<StoredPropertyChange>>, PropertyPersistenceError> {
    if activity_id.is_empty() {
        return Ok(HashMap::new());
    }
    let activity_uuid = activity_id.iter().map(|id| id.0).collect::<Vec<_>>();
    record_property_query(PropertyQueryKind::Hydration);
    let row = sqlx::query_as::<_, StoredPropertyChangeRow>(HYDRATE_ENTITY_PROPERTY_CHANGE_SQL)
        .bind(&activity_uuid)
        .fetch_all(&mut **transaction)
        .await
        .map_err(property_storage_error)?;
    let mut change = HashMap::<ActivityId, Vec<StoredPropertyChange>>::new();
    for row in row {
        let value = match (row.value_type.as_str(), row.text_value, row.integer_value) {
            ("text", Some(value), None) => PropertyValue::Text(value),
            ("integer", None, Some(value)) => PropertyValue::Integer(value),
            _ => return Err(PropertyPersistenceError::InvalidStoredRelation),
        };
        change
            .entry(row.activity_id)
            .or_default()
            .push(StoredPropertyChange {
                entity_id: row.entity_id,
                entity_name: row.entity_name,
                key: row.key,
                value,
            });
    }
    Ok(change)
}

fn trait_storage_error(error: sqlx::Error) -> TraitPersistenceError {
    TraitPersistenceError::Storage(Box::new(error))
}

fn normalize_trait_writes(
    mut writes: Vec<TraitWrite>,
    allow_empty: bool,
) -> Result<Vec<TraitWrite>, TraitNormalizationError> {
    if writes.len() > MAX_TRAIT_COUNT || (!allow_empty && writes.is_empty()) {
        return Err(TraitNormalizationError::InvalidCount);
    }
    for write in &mut writes {
        let statement = write.statement_mut();
        *statement = statement.trim().to_owned();
        let reason = if statement.is_empty() {
            Some(InvalidReason::Empty)
        } else if statement.contains('\0') {
            Some(InvalidReason::ContainsNul)
        } else if statement.chars().count() > MAX_TRAIT_STATEMENT_LENGTH {
            Some(InvalidReason::TooLong)
        } else {
            None
        };
        if let Some(reason) = reason {
            return Err(TraitNormalizationError::InvalidStatement(reason));
        }
    }
    writes.sort_unstable_by(|left, right| match (left, right) {
        (
            TraitWrite::Establish {
                entity_id: left_entity_id,
                statement: left_statement,
            },
            TraitWrite::Establish {
                entity_id: right_entity_id,
                statement: right_statement,
            },
        ) => left_entity_id
            .0
            .as_bytes()
            .cmp(right_entity_id.0.as_bytes())
            .then_with(|| left_statement.cmp(right_statement)),
        (TraitWrite::Establish { .. }, TraitWrite::Develop { .. }) => std::cmp::Ordering::Less,
        (TraitWrite::Develop { .. }, TraitWrite::Establish { .. }) => std::cmp::Ordering::Greater,
        (
            TraitWrite::Develop {
                trait_id: left_trait_id,
                statement: left_statement,
            },
            TraitWrite::Develop {
                trait_id: right_trait_id,
                statement: right_statement,
            },
        ) => left_trait_id
            .as_bytes()
            .cmp(right_trait_id.as_bytes())
            .then_with(|| left_statement.cmp(right_statement)),
    });
    for pair in writes.windows(2) {
        match (&pair[0], &pair[1]) {
            (
                TraitWrite::Establish {
                    entity_id: left_entity_id,
                    statement: left_statement,
                },
                TraitWrite::Establish {
                    entity_id: right_entity_id,
                    statement: right_statement,
                },
            ) if left_entity_id == right_entity_id && left_statement == right_statement => {
                return Err(TraitNormalizationError::DuplicateEstablishment);
            }
            (
                TraitWrite::Develop {
                    trait_id: left_trait_id,
                    ..
                },
                TraitWrite::Develop {
                    trait_id: right_trait_id,
                    ..
                },
            ) if left_trait_id == right_trait_id => {
                return Err(TraitNormalizationError::DuplicateDevelopment);
            }
            _ => {}
        }
    }
    Ok(writes)
}

#[derive(FromRow)]
struct TraitOwnerRow {
    trait_id: Uuid,
    entity_id: EntityId,
}

#[derive(FromRow)]
struct CurrentTraitRow {
    trait_id: Uuid,
    entity_id: EntityId,
    current_activity_id: ActivityId,
    statement: String,
}

struct ResolvedTraitWrite<'a> {
    lifecycle: StoredTraitLifecycle,
    trait_id: Uuid,
    entity_id: EntityId,
    previous_activity_id: Option<ActivityId>,
    statement: &'a str,
}

async fn write_trait_changes(
    transaction: &mut Transaction<'_, Postgres>,
    activity_id: ActivityId,
    writes: &[TraitWrite],
    eligible_entity_id: &[EntityId],
) -> Result<Vec<StoredTraitChange>, TraitPersistenceError> {
    if writes.is_empty() {
        return Ok(Vec::new());
    }

    let establish = writes
        .iter()
        .filter_map(|write| match write {
            TraitWrite::Establish {
                entity_id,
                statement,
            } => Some((*entity_id, statement.as_str())),
            TraitWrite::Develop { .. } => None,
        })
        .collect::<Vec<_>>();
    let develop_trait_id = writes
        .iter()
        .filter_map(|write| match write {
            TraitWrite::Establish { .. } => None,
            TraitWrite::Develop { trait_id, .. } => Some(*trait_id),
        })
        .collect::<Vec<_>>();

    let owner_row = if develop_trait_id.is_empty() {
        Vec::new()
    } else {
        record_trait_query(TraitQueryKind::Write);
        sqlx::query_as::<_, TraitOwnerRow>(FIND_ENTITY_TRAIT_OWNER_SQL)
            .bind(&develop_trait_id)
            .fetch_all(&mut **transaction)
            .await
            .map_err(trait_storage_error)?
    };
    if owner_row.len() != develop_trait_id.len() {
        return Err(TraitPersistenceError::Unavailable);
    }
    let owner_by_trait = owner_row
        .into_iter()
        .map(|row| (row.trait_id, row.entity_id))
        .collect::<HashMap<_, _>>();

    let mut affected_entity_id = establish
        .iter()
        .map(|(entity_id, _)| *entity_id)
        .chain(owner_by_trait.values().copied())
        .collect::<Vec<_>>();
    affected_entity_id.sort_unstable_by(|left, right| left.0.as_bytes().cmp(right.0.as_bytes()));
    affected_entity_id.dedup();

    let mut eligible_entity_id = eligible_entity_id.to_vec();
    eligible_entity_id.sort_unstable_by(|left, right| left.0.as_bytes().cmp(right.0.as_bytes()));
    eligible_entity_id.dedup();
    if affected_entity_id.iter().any(|entity_id| {
        eligible_entity_id
            .binary_search_by(|eligible| eligible.0.as_bytes().cmp(entity_id.0.as_bytes()))
            .is_err()
    }) {
        return Err(TraitPersistenceError::Unavailable);
    }

    let affected_uuid = affected_entity_id
        .iter()
        .map(|entity_id| entity_id.0)
        .collect::<Vec<_>>();
    record_trait_query(TraitQueryKind::Write);
    let locked_entity_id = sqlx::query_scalar::<_, Uuid>(LOCK_TRAIT_ENTITY_SQL)
        .bind(&affected_uuid)
        .fetch_all(&mut **transaction)
        .await
        .map_err(trait_storage_error)?;
    if locked_entity_id.len() != affected_entity_id.len() {
        return Err(TraitPersistenceError::Unavailable);
    }

    let current_row = if develop_trait_id.is_empty() {
        Vec::new()
    } else {
        record_trait_query(TraitQueryKind::Write);
        sqlx::query_as::<_, CurrentTraitRow>(LOCK_CURRENT_ENTITY_TRAIT_SQL)
            .bind(&develop_trait_id)
            .fetch_all(&mut **transaction)
            .await
            .map_err(trait_storage_error)?
    };
    if current_row.len() != develop_trait_id.len() {
        return Err(TraitPersistenceError::Unavailable);
    }
    let current_by_trait = current_row
        .into_iter()
        .map(|row| (row.trait_id, row))
        .collect::<HashMap<_, _>>();

    let mut resolved = Vec::with_capacity(writes.len());
    for write in writes {
        match write {
            TraitWrite::Establish {
                entity_id,
                statement,
            } => resolved.push(ResolvedTraitWrite {
                lifecycle: StoredTraitLifecycle::Establish,
                trait_id: Uuid::new_v4(),
                entity_id: *entity_id,
                previous_activity_id: None,
                statement,
            }),
            TraitWrite::Develop {
                trait_id,
                statement,
            } => {
                let current = current_by_trait
                    .get(trait_id)
                    .ok_or(TraitPersistenceError::InvalidStoredRelation)?;
                if owner_by_trait.get(trait_id).copied() != Some(current.entity_id) {
                    return Err(TraitPersistenceError::InvalidStoredRelation);
                }
                if current.statement == *statement {
                    return Err(TraitPersistenceError::InvalidInput);
                }
                resolved.push(ResolvedTraitWrite {
                    lifecycle: StoredTraitLifecycle::Develop,
                    trait_id: *trait_id,
                    entity_id: current.entity_id,
                    previous_activity_id: Some(current.current_activity_id),
                    statement,
                });
            }
        }
    }
    resolved.sort_unstable_by(|left, right| {
        left.entity_id
            .0
            .as_bytes()
            .cmp(right.entity_id.0.as_bytes())
            .then_with(|| left.trait_id.as_bytes().cmp(right.trait_id.as_bytes()))
    });

    let developed_trait_id = resolved
        .iter()
        .filter(|write| write.lifecycle == StoredTraitLifecycle::Develop)
        .map(|write| write.trait_id)
        .collect::<Vec<_>>();
    let mut intended_active = resolved
        .iter()
        .map(|write| (write.entity_id, write.statement.to_owned()))
        .collect::<Vec<_>>();
    intended_active.sort_unstable_by(|left, right| {
        left.0
            .0
            .as_bytes()
            .cmp(right.0.0.as_bytes())
            .then_with(|| left.1.cmp(&right.1))
    });
    if intended_active.windows(2).any(|pair| pair[0] == pair[1]) {
        return Err(TraitPersistenceError::InvalidInput);
    }
    let proposed_entity_id = intended_active
        .iter()
        .map(|(entity_id, _)| entity_id.0)
        .collect::<Vec<_>>();
    let proposed_statement = intended_active
        .iter()
        .map(|(_, statement)| statement.as_str())
        .collect::<Vec<_>>();
    let active_duplicate = {
        record_trait_query(TraitQueryKind::Write);
        sqlx::query_as::<_, (Uuid, Uuid, String)>(LOCK_ACTIVE_ENTITY_TRAIT_STATEMENT_SQL)
            .bind(&proposed_entity_id)
            .bind(&proposed_statement)
            .bind(&developed_trait_id)
            .fetch_all(&mut **transaction)
            .await
            .map_err(trait_storage_error)?
    };
    if !active_duplicate.is_empty() {
        return Err(TraitPersistenceError::InvalidInput);
    }

    let root = resolved
        .iter()
        .filter(|write| write.lifecycle == StoredTraitLifecycle::Establish)
        .collect::<Vec<_>>();
    if !root.is_empty() {
        let trait_id = root.iter().map(|write| write.trait_id).collect::<Vec<_>>();
        let entity_id = root
            .iter()
            .map(|write| write.entity_id.0)
            .collect::<Vec<_>>();
        record_trait_query(TraitQueryKind::Write);
        sqlx::query(INSERT_ENTITY_TRAIT_SQL)
            .bind(&trait_id)
            .bind(&entity_id)
            .execute(&mut **transaction)
            .await
            .map_err(trait_storage_error)?;
    }

    let trait_id = resolved
        .iter()
        .map(|write| write.trait_id)
        .collect::<Vec<_>>();
    let entity_id = resolved
        .iter()
        .map(|write| write.entity_id.0)
        .collect::<Vec<_>>();
    let previous_activity_id = resolved
        .iter()
        .map(|write| write.previous_activity_id.map(|id| id.0))
        .collect::<Vec<_>>();
    let statement = resolved
        .iter()
        .map(|write| write.statement)
        .collect::<Vec<_>>();
    record_trait_query(TraitQueryKind::Write);
    sqlx::query(INSERT_ENTITY_TRAIT_VERSION_SQL)
        .bind(&trait_id)
        .bind(&entity_id)
        .bind(activity_id.0)
        .bind(&previous_activity_id)
        .bind(&statement)
        .execute(&mut **transaction)
        .await
        .map_err(trait_storage_error)?;
    record_trait_query(TraitQueryKind::Write);
    sqlx::query(UPSERT_CURRENT_ENTITY_TRAIT_SQL)
        .bind(&trait_id)
        .bind(&entity_id)
        .bind(activity_id.0)
        .execute(&mut **transaction)
        .await
        .map_err(trait_storage_error)?;

    hydrate_trait_changes(transaction, &[activity_id])
        .await
        .map(|mut change| change.remove(&activity_id).unwrap_or_default())
}

#[derive(FromRow)]
struct StoredTraitChangeRow {
    activity_id: ActivityId,
    trait_id: Uuid,
    entity_id: EntityId,
    entity_name: String,
    previous_activity_id: Option<ActivityId>,
    previous_statement: Option<String>,
    statement: String,
}

async fn hydrate_trait_changes(
    transaction: &mut Transaction<'_, Postgres>,
    activity_id: &[ActivityId],
) -> Result<HashMap<ActivityId, Vec<StoredTraitChange>>, TraitPersistenceError> {
    if activity_id.is_empty() {
        return Ok(HashMap::new());
    }
    let activity_uuid = activity_id.iter().map(|id| id.0).collect::<Vec<_>>();
    record_trait_query(TraitQueryKind::Hydration);
    let row = sqlx::query_as::<_, StoredTraitChangeRow>(HYDRATE_ENTITY_TRAIT_CHANGE_SQL)
        .bind(&activity_uuid)
        .fetch_all(&mut **transaction)
        .await
        .map_err(trait_storage_error)?;
    let mut change = HashMap::<ActivityId, Vec<StoredTraitChange>>::new();
    for row in row {
        let lifecycle = match (row.previous_activity_id, row.previous_statement.as_ref()) {
            (None, None) => StoredTraitLifecycle::Establish,
            (Some(_), Some(_)) => StoredTraitLifecycle::Develop,
            _ => return Err(TraitPersistenceError::InvalidStoredRelation),
        };
        change
            .entry(row.activity_id)
            .or_default()
            .push(StoredTraitChange {
                lifecycle,
                trait_id: row.trait_id,
                entity_id: row.entity_id,
                entity_name: row.entity_name,
                previous_statement: row.previous_statement,
                statement: row.statement,
            });
    }
    Ok(change)
}

struct ActivityDraft<'a> {
    operation: ActivityOperation,
    requested_by_user_id: UserId,
    actor_character_entity_id: Option<EntityId>,
    context_place_entity_id: Option<EntityId>,
    involved: &'a [(EntityId, ActivityEntityRole)],
    prose: Option<&'a str>,
    request_id: Option<Uuid>,
    request_fingerprint: Option<&'a [u8]>,
    action_consequence: Option<&'static str>,
}

async fn append_activity(
    transaction: &mut Transaction<'_, Postgres>,
    draft: ActivityDraft<'_>,
    storage_operation: &'static str,
) -> Result<ActivityId, WorldError> {
    let activity_id = ActivityId(Uuid::new_v4());
    sqlx::query(
        r#"
        INSERT INTO activity (
            id, operation, requested_by_user_id,
            actor_character_entity_id, context_place_entity_id,
            prose, request_id, request_fingerprint, action_consequence
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        "#,
    )
    .bind(activity_id.0)
    .bind(draft.operation.as_str())
    .bind(draft.requested_by_user_id.0)
    .bind(draft.actor_character_entity_id.map(|id| id.0))
    .bind(draft.context_place_entity_id.map(|id| id.0))
    .bind(draft.prose)
    .bind(draft.request_id)
    .bind(draft.request_fingerprint)
    .bind(draft.action_consequence)
    .execute(&mut **transaction)
    .await
    .map_err(|error| storage_error(storage_operation, error))?;
    append_activity_entity_roles(transaction, activity_id, draft.involved, storage_operation)
        .await?;
    Ok(activity_id)
}

async fn append_activity_entity_roles(
    transaction: &mut Transaction<'_, Postgres>,
    activity_id: ActivityId,
    involved: &[(EntityId, ActivityEntityRole)],
    storage_operation: &'static str,
) -> Result<(), WorldError> {
    let involved_entity_id = involved
        .iter()
        .map(|(entity_id, _)| entity_id.0)
        .collect::<Vec<_>>();
    let involved_role = involved
        .iter()
        .map(|(_, role)| role.as_str())
        .collect::<Vec<_>>();
    if !involved.is_empty() {
        sqlx::query(
            r#"
            INSERT INTO activity_entity (activity_id, entity_id, role)
            SELECT $1, involved.entity_id, involved.role
            FROM UNNEST($2::uuid[], $3::text[]) AS involved(entity_id, role)
            "#,
        )
        .bind(activity_id.0)
        .bind(&involved_entity_id)
        .bind(&involved_role)
        .execute(&mut **transaction)
        .await
        .map_err(|error| storage_error(storage_operation, error))?;
    }
    Ok(())
}

fn validate_limit(limit: u16, error: WorldError) -> Result<(), WorldError> {
    if limit == 0 || limit > MAX_PAGE_SIZE {
        Err(error)
    } else {
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, sqlx::Type)]
#[sqlx(transparent)]
pub struct UserId(pub Uuid);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, sqlx::Type)]
#[sqlx(transparent)]
pub struct EntityId(pub Uuid);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, sqlx::Type)]
#[sqlx(transparent)]
pub struct EntityTraitId(pub Uuid);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, sqlx::Type)]
#[sqlx(transparent)]
pub struct ActivityId(pub Uuid);

#[derive(Clone, Debug, PartialEq, Eq, FromRow)]
pub struct User {
    pub id: UserId,
    pub created_at: DateTime<Utc>,
}

#[derive(Clone, Debug, PartialEq, Eq, FromRow)]
pub struct Entity {
    pub id: EntityId,
    pub name: String,
    pub description: String,
    pub introduced_by_user_id: UserId,
    pub introduced_at: DateTime<Utc>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Character {
    pub entity: Entity,
    pub owner_user_id: UserId,
    pub current_place: Option<Place>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Place {
    pub entity: Entity,
    pub is_entry: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EntitySummary {
    pub id: EntityId,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PlaceSummary {
    pub entity: EntitySummary,
    pub is_entry: bool,
}

#[derive(FromRow)]
struct EntityListRow {
    id: EntityId,
    name: String,
    introduced_at: DateTime<Utc>,
}

#[derive(FromRow)]
struct CurrentPlaceEntityRow {
    id: EntityId,
    name: String,
    description: String,
    introduced_at: DateTime<Utc>,
}

#[derive(FromRow)]
struct EntityCurrentAssociationRow {
    association_type: i16,
    property_key_id: Option<i64>,
    trait_id: Option<Uuid>,
    key: Option<String>,
    value_type: Option<String>,
    text_value: Option<String>,
    integer_value: Option<i64>,
    statement: Option<String>,
}

impl EntityCurrentAssociationRow {
    fn into_association(
        self,
    ) -> Result<(EntityCurrentStateCursorKey, EntityCurrentAssociation), WorldError> {
        match (
            self.association_type,
            self.property_key_id,
            self.trait_id,
            self.key,
            self.value_type.as_deref(),
            self.text_value,
            self.integer_value,
            self.statement,
        ) {
            (0, Some(property_key_id), None, Some(key), Some("text"), Some(value), None, None) => {
                Ok((
                    EntityCurrentStateCursorKey::Property(property_key_id),
                    EntityCurrentAssociation::Property {
                        key,
                        value: PropertyValue::Text(value),
                    },
                ))
            }
            (
                0,
                Some(property_key_id),
                None,
                Some(key),
                Some("integer"),
                None,
                Some(value),
                None,
            ) => Ok((
                EntityCurrentStateCursorKey::Property(property_key_id),
                EntityCurrentAssociation::Property {
                    key,
                    value: PropertyValue::Integer(value),
                },
            )),
            (1, None, Some(trait_id), None, None, None, None, Some(statement)) => Ok((
                EntityCurrentStateCursorKey::Trait(EntityTraitId(trait_id)),
                EntityCurrentAssociation::Trait(EntityTrait {
                    id: EntityTraitId(trait_id),
                    statement,
                }),
            )),
            _ => Err(invalid_stored_relation()),
        }
    }
}

#[derive(FromRow)]
struct CharacterRow {
    entity_id: EntityId,
    name: String,
    description: String,
    introduced_by_user_id: UserId,
    introduced_at: DateTime<Utc>,
    owner_user_id: UserId,
    place_entity_id: Option<EntityId>,
    place_name: Option<String>,
    place_description: Option<String>,
    place_introduced_by_user_id: Option<UserId>,
    place_introduced_at: Option<DateTime<Utc>>,
    place_is_entry: Option<bool>,
}

impl TryFrom<CharacterRow> for Character {
    type Error = WorldError;

    fn try_from(value: CharacterRow) -> Result<Self, Self::Error> {
        let current_place = match value.place_entity_id {
            None => None,
            Some(id) => Some(Place {
                entity: Entity {
                    id,
                    name: value.place_name.ok_or_else(invalid_stored_relation)?,
                    description: value
                        .place_description
                        .ok_or_else(invalid_stored_relation)?,
                    introduced_by_user_id: value
                        .place_introduced_by_user_id
                        .ok_or_else(invalid_stored_relation)?,
                    introduced_at: value
                        .place_introduced_at
                        .ok_or_else(invalid_stored_relation)?,
                },
                is_entry: value.place_is_entry.ok_or_else(invalid_stored_relation)?,
            }),
        };
        Ok(Self {
            entity: Entity {
                id: value.entity_id,
                name: value.name,
                description: value.description,
                introduced_by_user_id: value.introduced_by_user_id,
                introduced_at: value.introduced_at,
            },
            owner_user_id: value.owner_user_id,
            current_place,
        })
    }
}

#[derive(FromRow)]
struct PlaceRow {
    entity_id: EntityId,
    name: String,
    description: String,
    introduced_by_user_id: UserId,
    introduced_at: DateTime<Utc>,
    is_entry: bool,
}

impl From<PlaceRow> for Place {
    fn from(value: PlaceRow) -> Self {
        Self {
            entity: Entity {
                id: value.entity_id,
                name: value.name,
                description: value.description,
                introduced_by_user_id: value.introduced_by_user_id,
                introduced_at: value.introduced_at,
            },
            is_entry: value.is_entry,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorldView {
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PropertyInput {
    pub key: String,
    pub value: PropertyValue,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EntityPropertyChangeInput {
    pub entity_id: EntityId,
    pub key: String,
    pub value: PropertyValue,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EntityPropertyChange {
    pub entity: EntitySummary,
    pub key: String,
    pub value: PropertyValue,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EntityTrait {
    pub id: EntityTraitId,
    pub statement: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EntityTraitChangeInput {
    Establish {
        entity_id: EntityId,
        statement: String,
    },
    Develop {
        trait_id: EntityTraitId,
        statement: String,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ActivityTraitChange {
    Establish {
        entity: EntitySummary,
        r#trait: EntityTrait,
    },
    Develop {
        entity: EntitySummary,
        r#trait: EntityTrait,
        previous_statement: String,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CreateEntity {
    pub name: String,
    pub description: String,
    pub property: Vec<PropertyInput>,
}

impl CreateEntity {
    fn normalize(self) -> Result<Self, WorldError> {
        let (name, description) =
            normalize_entity_text(self.name, self.description, |field, reason| {
                WorldError::InvalidEntity { field, reason }
            })?;
        let property = normalize_property_input(self.property, PropertyField::Property)?;
        Ok(Self {
            name,
            description,
            property,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CreateCharacter {
    pub name: String,
    pub description: String,
    pub property: Vec<PropertyInput>,
}

impl CreateCharacter {
    fn normalize(self) -> Result<Self, WorldError> {
        let (name, description) =
            normalize_entity_text(self.name, self.description, |field, reason| {
                WorldError::InvalidCharacter { field, reason }
            })?;
        let property = normalize_property_input(self.property, PropertyField::Property)?;
        Ok(Self {
            name,
            description,
            property,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CreateEntryPlace {
    pub name: String,
    pub description: String,
    pub property: Vec<PropertyInput>,
}

impl CreateEntryPlace {
    fn normalize(self) -> Result<Self, WorldError> {
        let (name, description) =
            normalize_entity_text(self.name, self.description, |field, reason| {
                WorldError::InvalidPlace { field, reason }
            })?;
        let property = normalize_property_input(self.property, PropertyField::Property)?;
        Ok(Self {
            name,
            description,
            property,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SubmitAction {
    pub request_id: Uuid,
    pub expected_place_revision: PlaceRevision,
    pub prose: String,
    pub consequence: ActionConsequence,
}

impl SubmitAction {
    fn normalize(self) -> Result<Self, WorldError> {
        let prose = self.prose.trim().to_owned();
        let prose_reason = if prose.is_empty() {
            Some(InvalidReason::Empty)
        } else if prose.contains('\0') {
            Some(InvalidReason::ContainsNul)
        } else if prose.chars().count() > MAX_ACTION_PROSE_LENGTH {
            Some(InvalidReason::TooLong)
        } else {
            None
        };
        if let Some(reason) = prose_reason {
            return Err(WorldError::InvalidAction {
                field: ActionField::Prose,
                reason,
            });
        }
        let consequence = self.consequence.normalize()?;
        Ok(Self {
            request_id: self.request_id,
            expected_place_revision: self.expected_place_revision,
            prose,
            consequence,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IntroduceEntity {
    pub name: String,
    pub description: String,
    pub property: Vec<PropertyInput>,
}

impl IntroduceEntity {
    fn normalize(self) -> Result<Self, WorldError> {
        let (name, description) =
            normalize_entity_text(self.name, self.description, |field, reason| {
                WorldError::InvalidAction {
                    field: match field {
                        EntityField::Name => ActionField::ConsequenceName,
                        EntityField::Description => ActionField::ConsequenceDescription,
                    },
                    reason,
                }
            })?;
        let property = normalize_property_input(self.property, PropertyField::Property)?;
        Ok(Self {
            name,
            description,
            property,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChangeEntityProperty {
    pub property_change: Vec<EntityPropertyChangeInput>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChangeEntityTrait {
    pub trait_change: Vec<EntityTraitChangeInput>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ActionConsequence {
    IntroduceEntity(IntroduceEntity),
    ChangeEntityProperty(ChangeEntityProperty),
    ChangeEntityTrait(ChangeEntityTrait),
}

impl ActionConsequence {
    fn normalize(self) -> Result<Self, WorldError> {
        match self {
            Self::IntroduceEntity(input) => input.normalize().map(Self::IntroduceEntity),
            Self::ChangeEntityProperty(input) => {
                let writes = input
                    .property_change
                    .into_iter()
                    .map(|change| PropertyWrite {
                        entity_id: change.entity_id,
                        key: change.key,
                        value: change.value,
                    })
                    .collect();
                let writes = normalize_property_writes(writes, false).map_err(|error| {
                    map_property_normalization_error(error, PropertyField::PropertyChange)
                })?;
                Ok(Self::ChangeEntityProperty(ChangeEntityProperty {
                    property_change: writes
                        .into_iter()
                        .map(|write| EntityPropertyChangeInput {
                            entity_id: write.entity_id,
                            key: write.key,
                            value: write.value,
                        })
                        .collect(),
                }))
            }
            Self::ChangeEntityTrait(input) => {
                let writes = normalize_trait_change_input(input.trait_change, false)?;
                Ok(Self::ChangeEntityTrait(ChangeEntityTrait {
                    trait_change: trait_input_from_writes(writes),
                }))
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PlaceRevision {
    place_entity_id: EntityId,
    occurred_at: DateTime<Utc>,
    activity_id: ActivityId,
}

impl PlaceRevision {
    pub fn from_parts(
        place_entity_id: EntityId,
        occurred_at: DateTime<Utc>,
        activity_id: ActivityId,
    ) -> Self {
        Self {
            place_entity_id,
            occurred_at,
            activity_id,
        }
    }

    pub fn place_entity_id(self) -> EntityId {
        self.place_entity_id
    }

    pub fn occurred_at(self) -> DateTime<Utc> {
        self.occurred_at
    }

    pub fn activity_id(self) -> ActivityId {
        self.activity_id
    }

    fn fingerprint_bytes(self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(41);
        bytes.push(1);
        bytes.extend_from_slice(self.place_entity_id.0.as_bytes());
        bytes.extend_from_slice(&self.occurred_at.timestamp_micros().to_be_bytes());
        bytes.extend_from_slice(self.activity_id.0.as_bytes());
        bytes
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AcceptedAction {
    pub activity: Activity,
    pub consequence: AcceptedActionConsequence,
    pub place: Place,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AcceptedActionConsequence {
    IntroduceEntity(Entity),
    ChangeEntityProperty(Vec<EntityPropertyChange>),
    ChangeEntityTrait(Vec<ActivityTraitChange>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SubmitInteraction {
    pub request_id: Uuid,
    pub expected_place_revision: PlaceRevision,
    pub prose: String,
    pub target_entity_id: Vec<EntityId>,
    pub property_change: Vec<EntityPropertyChangeInput>,
    pub trait_change: Vec<EntityTraitChangeInput>,
}

struct NormalizedSubmitInteraction {
    request_id: Uuid,
    expected_place_revision: PlaceRevision,
    prose: String,
    target_entity_id: Vec<EntityId>,
    has_duplicate_target: bool,
    property_change: Vec<PropertyWrite>,
    trait_change: Vec<TraitWrite>,
}

impl SubmitInteraction {
    fn normalize(self) -> Result<NormalizedSubmitInteraction, WorldError> {
        let prose = self.prose.trim().to_owned();
        let prose_reason = if prose.is_empty() {
            Some(InvalidReason::Empty)
        } else if prose.contains('\0') {
            Some(InvalidReason::ContainsNul)
        } else if prose.chars().count() > MAX_INTERACTION_PROSE_LENGTH {
            Some(InvalidReason::TooLong)
        } else {
            None
        };
        if let Some(reason) = prose_reason {
            return Err(WorldError::InvalidInteraction {
                field: InteractionField::Prose,
                reason,
            });
        }
        if !(1..=MAX_INTERACTION_TARGET_COUNT).contains(&self.target_entity_id.len()) {
            return Err(WorldError::InvalidInteraction {
                field: InteractionField::TargetEntityId,
                reason: InvalidReason::OutOfRange,
            });
        }
        let mut target_entity_id = self.target_entity_id;
        target_entity_id.sort_unstable_by(|left, right| left.0.as_bytes().cmp(right.0.as_bytes()));
        let has_duplicate_target = target_entity_id.windows(2).any(|pair| pair[0] == pair[1]);
        let property_change = self
            .property_change
            .into_iter()
            .map(|change| PropertyWrite {
                entity_id: change.entity_id,
                key: change.key,
                value: change.value,
            })
            .collect();
        let property_change =
            normalize_property_writes(property_change, true).map_err(|error| {
                map_property_normalization_error(error, PropertyField::PropertyChange)
            })?;
        let trait_change = normalize_trait_change_input(self.trait_change, true)?;
        Ok(NormalizedSubmitInteraction {
            request_id: self.request_id,
            expected_place_revision: self.expected_place_revision,
            prose,
            target_entity_id,
            has_duplicate_target,
            property_change,
            trait_change,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AcceptedInteraction {
    pub activity: Activity,
    pub place: Place,
}

fn normalize_entity_text(
    name: String,
    description: String,
    invalid: impl Fn(EntityField, InvalidReason) -> WorldError,
) -> Result<(String, String), WorldError> {
    let name = name.trim().to_owned();
    let description = description.trim().to_owned();
    for (field, value, maximum) in [
        (EntityField::Name, name.as_str(), MAX_ENTITY_NAME_LENGTH),
        (
            EntityField::Description,
            description.as_str(),
            MAX_ENTITY_DESCRIPTION_LENGTH,
        ),
    ] {
        let reason = if value.is_empty() {
            Some(InvalidReason::Empty)
        } else if value.contains('\0') {
            Some(InvalidReason::ContainsNul)
        } else if value.chars().count() > maximum {
            Some(InvalidReason::TooLong)
        } else {
            None
        };
        if let Some(reason) = reason {
            return Err(invalid(field, reason));
        }
    }
    Ok((name, description))
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ListEntity {
    pub cursor: Option<EntityCursor>,
    pub limit: u16,
}

impl Default for ListEntity {
    fn default() -> Self {
        Self {
            cursor: None,
            limit: DEFAULT_PAGE_SIZE,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EntityCursor {
    pub introduced_at: DateTime<Utc>,
    pub entity_id: EntityId,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EntityPage {
    pub entity: Vec<EntitySummary>,
    pub next: Option<EntityCursor>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ListEntityAtCurrentPlace {
    pub cursor: Option<EntityCursor>,
    pub limit: u16,
}

impl Default for ListEntityAtCurrentPlace {
    fn default() -> Self {
        Self {
            cursor: None,
            limit: DEFAULT_PAGE_SIZE,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CurrentPlaceEntityPage {
    pub place: Place,
    pub place_revision: PlaceRevision,
    pub entity: Vec<CurrentPlaceEntity>,
    pub next: Option<EntityCursor>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CurrentPlaceEntity {
    pub id: EntityId,
    pub name: String,
    pub description: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ListActivity {
    pub cursor: Option<ActivityCursor>,
    pub limit: u16,
}

impl Default for ListActivity {
    fn default() -> Self {
        Self {
            cursor: None,
            limit: DEFAULT_PAGE_SIZE,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ActivityCursor {
    pub occurred_at: DateTime<Utc>,
    pub activity_id: ActivityId,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActivityPage {
    pub activity: Vec<Activity>,
    pub next: Option<ActivityCursor>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ListActivityAtCurrentPlace {
    pub cursor: Option<ActivityCursor>,
    pub limit: u16,
}

impl Default for ListActivityAtCurrentPlace {
    fn default() -> Self {
        Self {
            cursor: None,
            limit: DEFAULT_PAGE_SIZE,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CurrentPlaceActivityPage {
    pub place: Place,
    pub place_revision: PlaceRevision,
    pub activity: Vec<Activity>,
    pub next: Option<ActivityCursor>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GetEntityCurrentState {
    pub cursor: Option<EntityCurrentStateCursor>,
    pub limit: u16,
}

impl Default for GetEntityCurrentState {
    fn default() -> Self {
        Self {
            cursor: None,
            limit: DEFAULT_PAGE_SIZE,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GetEntityAtCurrentPlace {
    pub entity_id: EntityId,
    pub cursor: Option<EntityCurrentStateCursor>,
    pub limit: u16,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum EntityCurrentStateCursorKey {
    Property(i64),
    Trait(EntityTraitId),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EntityCurrentStateCursor {
    entity_id: EntityId,
    place_revision: Option<PlaceRevision>,
    key: EntityCurrentStateCursorKey,
}

impl EntityCurrentStateCursor {
    pub(crate) fn from_property(
        entity_id: EntityId,
        place_revision: Option<PlaceRevision>,
        property_key_id: i64,
    ) -> Self {
        Self {
            entity_id,
            place_revision,
            key: EntityCurrentStateCursorKey::Property(property_key_id),
        }
    }

    pub(crate) fn from_trait(
        entity_id: EntityId,
        place_revision: Option<PlaceRevision>,
        trait_id: EntityTraitId,
    ) -> Self {
        Self {
            entity_id,
            place_revision,
            key: EntityCurrentStateCursorKey::Trait(trait_id),
        }
    }

    pub(crate) fn entity_id(self) -> EntityId {
        self.entity_id
    }

    pub(crate) fn place_revision(self) -> Option<PlaceRevision> {
        self.place_revision
    }

    pub(crate) fn property_key_id(self) -> Option<i64> {
        match self.key {
            EntityCurrentStateCursorKey::Property(id) => Some(id),
            EntityCurrentStateCursorKey::Trait(_) => None,
        }
    }

    pub(crate) fn trait_id(self) -> Option<EntityTraitId> {
        match self.key {
            EntityCurrentStateCursorKey::Property(_) => None,
            EntityCurrentStateCursorKey::Trait(id) => Some(id),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EntityCurrentAssociation {
    Property { key: String, value: PropertyValue },
    Trait(EntityTrait),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EntityCurrentStatePage {
    pub association: Vec<EntityCurrentAssociation>,
    pub next: Option<EntityCurrentStateCursor>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CharacterEntityStatePage {
    pub character: Character,
    pub place_revision: Option<PlaceRevision>,
    pub current_state: EntityCurrentStatePage,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CurrentPlaceEntityStatePage {
    pub place: Place,
    pub place_revision: PlaceRevision,
    pub entity: CurrentPlaceEntity,
    pub current_state: EntityCurrentStatePage,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Activity {
    pub id: ActivityId,
    pub operation: ActivityOperation,
    pub actor_character: Option<EntitySummary>,
    pub context_place: Option<PlaceSummary>,
    pub involved_entity: Vec<ActivityEntityReference>,
    pub property_change: Vec<EntityPropertyChange>,
    pub trait_change: Vec<ActivityTraitChange>,
    pub prose: Option<String>,
    pub occurred_at: DateTime<Utc>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActivityOperation {
    CreateCharacter,
    CreateEntity,
    CreateEntryPlace,
    EnterWorld,
    SubmitAction,
    SubmitInteraction,
}

impl ActivityOperation {
    fn as_str(self) -> &'static str {
        match self {
            Self::CreateCharacter => "create_character",
            Self::CreateEntity => "create_entity",
            Self::CreateEntryPlace => "create_entry_place",
            Self::EnterWorld => "enter_world",
            Self::SubmitAction => "submit_action",
            Self::SubmitInteraction => "submit_interaction",
        }
    }

    fn parse(value: &str) -> Result<Self, WorldError> {
        match value {
            "create_character" => Ok(Self::CreateCharacter),
            "create_entity" => Ok(Self::CreateEntity),
            "create_entry_place" => Ok(Self::CreateEntryPlace),
            "enter_world" => Ok(Self::EnterWorld),
            "submit_action" => Ok(Self::SubmitAction),
            "submit_interaction" => Ok(Self::SubmitInteraction),
            _ => Err(invalid_stored_relation()),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActivityEntityRole {
    Subject,
    Destination,
    Location,
    Target,
}

impl ActivityEntityRole {
    fn as_str(self) -> &'static str {
        match self {
            Self::Subject => "subject",
            Self::Destination => "destination",
            Self::Location => "location",
            Self::Target => "target",
        }
    }

    fn parse(value: &str) -> Result<Self, WorldError> {
        match value {
            "subject" => Ok(Self::Subject),
            "destination" => Ok(Self::Destination),
            "location" => Ok(Self::Location),
            "target" => Ok(Self::Target),
            _ => Err(invalid_stored_relation()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActivityEntityReference {
    pub entity: EntitySummary,
    pub role: ActivityEntityRole,
}

#[derive(FromRow)]
struct ActivityRow {
    id: ActivityId,
    operation: String,
    prose: Option<String>,
    occurred_at: DateTime<Utc>,
    actor_entity_id: Option<EntityId>,
    actor_name: Option<String>,
    context_entity_id: Option<EntityId>,
    context_name: Option<String>,
    context_is_entry: Option<bool>,
}

#[derive(FromRow)]
struct AcceptedActionActivityRow {
    id: ActivityId,
    operation: String,
    prose: Option<String>,
    occurred_at: DateTime<Utc>,
    request_fingerprint: Vec<u8>,
    action_consequence: Option<String>,
    actor_entity_id: Option<EntityId>,
    actor_name: Option<String>,
    context_entity_id: Option<EntityId>,
    context_name: Option<String>,
    context_is_entry: Option<bool>,
}

impl From<AcceptedActionActivityRow> for ActivityRow {
    fn from(value: AcceptedActionActivityRow) -> Self {
        Self {
            id: value.id,
            operation: value.operation,
            prose: value.prose,
            occurred_at: value.occurred_at,
            actor_entity_id: value.actor_entity_id,
            actor_name: value.actor_name,
            context_entity_id: value.context_entity_id,
            context_name: value.context_name,
            context_is_entry: value.context_is_entry,
        }
    }
}

impl ActivityRow {
    fn into_activity(
        self,
        involved_entity: Vec<ActivityEntityReference>,
        property_change: Vec<EntityPropertyChange>,
        trait_change: Vec<ActivityTraitChange>,
    ) -> Result<Activity, WorldError> {
        let actor_character = optional_summary(self.actor_entity_id, self.actor_name)?;
        let context_place = match optional_summary(self.context_entity_id, self.context_name)? {
            None => None,
            Some(entity) => Some(PlaceSummary {
                entity,
                is_entry: self.context_is_entry.ok_or_else(invalid_stored_relation)?,
            }),
        };
        Ok(Activity {
            id: self.id,
            operation: ActivityOperation::parse(&self.operation)?,
            actor_character,
            context_place,
            involved_entity,
            property_change,
            trait_change,
            prose: self.prose,
            occurred_at: self.occurred_at,
        })
    }
}

#[derive(FromRow)]
struct ActivityEntityRow {
    activity_id: ActivityId,
    entity_id: EntityId,
    name: String,
    role: String,
}

impl TryFrom<ActivityEntityRow> for ActivityEntityReference {
    type Error = WorldError;

    fn try_from(value: ActivityEntityRow) -> Result<Self, Self::Error> {
        Ok(Self {
            entity: EntitySummary {
                id: value.entity_id,
                name: value.name,
            },
            role: ActivityEntityRole::parse(&value.role)?,
        })
    }
}

fn optional_summary(
    id: Option<EntityId>,
    name: Option<String>,
) -> Result<Option<EntitySummary>, WorldError> {
    match (id, name) {
        (None, None) => Ok(None),
        (Some(id), Some(name)) => Ok(Some(EntitySummary { id, name })),
        _ => Err(invalid_stored_relation()),
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EntityField {
    Name,
    Description,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActionField {
    Prose,
    ConsequenceName,
    ConsequenceDescription,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InteractionField {
    Prose,
    TargetEntityId,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PropertyField {
    Property,
    PropertyChange,
    Key,
    Value,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InvalidReason {
    Empty,
    ContainsNul,
    TooLong,
    OutOfRange,
    InvalidFormat,
    Duplicate,
}

#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum WorldError {
    #[error("request input is invalid")]
    InvalidRequest,
    #[error("entity input is invalid")]
    InvalidEntity {
        field: EntityField,
        reason: InvalidReason,
    },
    #[error("character input is invalid")]
    InvalidCharacter {
        field: EntityField,
        reason: InvalidReason,
    },
    #[error("place input is invalid")]
    InvalidPlace {
        field: EntityField,
        reason: InvalidReason,
    },
    #[error("action input is invalid")]
    InvalidAction {
        field: ActionField,
        reason: InvalidReason,
    },
    #[error("interaction input is invalid")]
    InvalidInteraction {
        field: InteractionField,
        reason: InvalidReason,
    },
    #[error("property input is invalid")]
    InvalidProperty {
        field: PropertyField,
        reason: InvalidReason,
    },
    #[error("trait input is invalid")]
    InvalidTrait,
    #[error("entity list limit must be between 1 and 100")]
    InvalidEntityLimit,
    #[error("activity list limit must be between 1 and 100")]
    InvalidActivityLimit,
    #[error("property list limit must be between 1 and 100")]
    InvalidPropertyLimit,
    #[error("user was not found")]
    UserNotFound,
    #[error("entity was not found")]
    EntityNotFound,
    #[error("character was not found")]
    CharacterNotFound,
    #[error("user already owns a character")]
    CharacterAlreadyExists,
    #[error("character is already placed")]
    CharacterAlreadyEntered,
    #[error("entry place already exists")]
    EntryPlaceAlreadyExists,
    #[error("entry place was not found")]
    EntryPlaceNotFound,
    #[error("character has not entered the world")]
    CharacterNotEntered,
    #[error("action request id has already been used with different content")]
    ActionRequestConflict,
    #[error("interaction request id has already been used with different content")]
    InteractionRequestConflict,
    #[error("one or more interaction targets are unavailable")]
    InteractionTargetUnavailable,
    #[error("one or more property entities are unavailable")]
    PropertyEntityUnavailable,
    #[error("one or more traits are unavailable")]
    TraitUnavailable,
    #[error("entity at current place is unavailable")]
    EntityAtCurrentPlaceUnavailable,
    #[error("property key already exists with a different value type")]
    PropertyKeyConflict,
    #[error("current place has changed since it was read")]
    PlaceRevisionConflict,
    #[error("world storage is unavailable")]
    Unavailable,
}

fn invalid_stored_relation() -> WorldError {
    storage_error(
        "decode_world_state",
        sqlx::Error::ColumnNotFound("inconsistent relation".to_owned()),
    )
}

fn constraint(error: &sqlx::Error) -> Option<&str> {
    error
        .as_database_error()
        .and_then(|error| error.constraint())
}

fn storage_error(operation: &'static str, error: sqlx::Error) -> WorldError {
    let category = match error {
        sqlx::Error::PoolTimedOut | sqlx::Error::PoolClosed | sqlx::Error::WorkerCrashed => "pool",
        sqlx::Error::Io(_) | sqlx::Error::Tls(_) => "connection",
        sqlx::Error::Database(_) => "database",
        _ => "other",
    };
    eprintln!(
        "{}",
        serde_json::json!({
            "owner": "world",
            "operation": operation,
            "status": "unavailable",
            "category": category,
            "recovery": "retry_later"
        })
    );
    WorldError::Unavailable
}

#[cfg(test)]
mod property_query_count_test {
    use super::*;
    use sqlx::PgPool;

    fn reset_property_query_count() {
        PROPERTY_QUERY_COUNT.with(|count| *count.borrow_mut() = PropertyQueryCount::default());
    }

    fn property_query_count() -> PropertyQueryCount {
        PROPERTY_QUERY_COUNT.with(|count| *count.borrow())
    }

    fn changes(entity_id: EntityId, count: usize) -> Vec<EntityPropertyChangeInput> {
        (0..count)
            .map(|index| EntityPropertyChangeInput {
                entity_id,
                key: format!("measure_{index}"),
                value: PropertyValue::Integer(index as i64),
            })
            .collect()
    }

    async fn submit_change(
        world: &World,
        user_id: UserId,
        entity_id: EntityId,
        revision: PlaceRevision,
        count: usize,
    ) -> AcceptedAction {
        world
            .submit_action(
                user_id,
                SubmitAction {
                    request_id: Uuid::new_v4(),
                    expected_place_revision: revision,
                    prose: format!("Mara records {count} exact physical measures."),
                    consequence: ActionConsequence::ChangeEntityProperty(ChangeEntityProperty {
                        property_change: changes(entity_id, count),
                    }),
                },
            )
            .await
            .expect("bounded Property Action should be accepted")
    }

    #[sqlx::test(migrations = "./migration")]
    async fn one_and_one_hundred_properties_use_constant_query_counts(pool: PgPool) {
        PROPERTY_QUERY_COUNT
            .scope(
                RefCell::new(PropertyQueryCount::default()),
                assert_one_and_one_hundred_properties_use_constant_query_counts(pool),
            )
            .await;
    }

    async fn assert_one_and_one_hundred_properties_use_constant_query_counts(pool: PgPool) {
        let world = World::new(pool);
        let user = world.create_user().await.unwrap();
        let character = world
            .create_character(
                user.id,
                CreateCharacter {
                    name: "Mara Venn".to_owned(),
                    description: "A careful surveyor.".to_owned(),
                    property: Vec::new(),
                },
            )
            .await
            .unwrap();
        world
            .create_entry_place(
                user.id,
                CreateEntryPlace {
                    name: "North Gate".to_owned(),
                    description: "A wind-worn stone gate.".to_owned(),
                    property: Vec::new(),
                },
            )
            .await
            .unwrap();
        world.enter_world(user.id).await.unwrap();
        let initial = world
            .get_character(user.id, GetEntityCurrentState::default())
            .await
            .unwrap();

        reset_property_query_count();
        submit_change(
            &world,
            user.id,
            character.entity.id,
            initial.place_revision.unwrap(),
            1,
        )
        .await;
        let one_write = property_query_count();

        reset_property_query_count();
        let one_current = world
            .get_character(
                user.id,
                GetEntityCurrentState {
                    cursor: None,
                    limit: 100,
                },
            )
            .await
            .unwrap();
        assert_eq!(one_current.current_state.association.len(), 1);
        let one_current_count = property_query_count();

        reset_property_query_count();
        let one_history = world
            .list_activity(
                user.id,
                ListActivity {
                    cursor: None,
                    limit: 1,
                },
            )
            .await
            .unwrap();
        assert_eq!(one_history.activity[0].property_change.len(), 1);
        let one_hydration = property_query_count();

        reset_property_query_count();
        submit_change(
            &world,
            user.id,
            character.entity.id,
            one_current.place_revision.unwrap(),
            100,
        )
        .await;
        let hundred_write = property_query_count();

        reset_property_query_count();
        let hundred_current = world
            .get_character(
                user.id,
                GetEntityCurrentState {
                    cursor: None,
                    limit: 100,
                },
            )
            .await
            .unwrap();
        assert_eq!(hundred_current.current_state.association.len(), 100);
        let hundred_current_count = property_query_count();

        reset_property_query_count();
        let hundred_history = world
            .list_activity(
                user.id,
                ListActivity {
                    cursor: None,
                    limit: 1,
                },
            )
            .await
            .unwrap();
        assert_eq!(hundred_history.activity[0].property_change.len(), 100);
        let hundred_hydration = property_query_count();

        assert_eq!(one_write, hundred_write);
        assert_eq!(
            one_write,
            PropertyQueryCount {
                write: 5,
                current_read: 0,
                hydration: 2,
            }
        );
        assert_eq!(one_current_count, hundred_current_count);
        assert_eq!(
            one_current_count,
            PropertyQueryCount {
                write: 0,
                current_read: 1,
                hydration: 0,
            }
        );
        assert_eq!(one_hydration, hundred_hydration);
        assert_eq!(
            one_hydration,
            PropertyQueryCount {
                write: 0,
                current_read: 0,
                hydration: 1,
            }
        );
    }

    #[sqlx::test(migrations = "./migration")]
    async fn live_property_queries_have_bound_set_based_plans(pool: PgPool) {
        let world = World::new(pool.clone());
        let user = world.create_user().await.unwrap();
        let character = world
            .create_character(
                user.id,
                CreateCharacter {
                    name: "Index Seer".to_owned(),
                    description: "A careful surveyor.".to_owned(),
                    property: Vec::new(),
                },
            )
            .await
            .unwrap();
        let _place = world
            .create_entry_place(
                user.id,
                CreateEntryPlace {
                    name: "North Gate".to_owned(),
                    description: "A wind-worn stone gate.".to_owned(),
                    property: Vec::new(),
                },
            )
            .await
            .unwrap();
        world.enter_world(user.id).await.unwrap();
        let revision = world
            .get_character(user.id, GetEntityCurrentState::default())
            .await
            .unwrap()
            .place_revision
            .unwrap();
        let accepted = submit_change(&world, user.id, character.entity.id, revision, 100).await;
        let activity_id = accepted.activity.id.0;
        let key = (0..100)
            .map(|index| format!("measure_{index}"))
            .collect::<Vec<_>>();
        let property_key_id: Vec<i64> = sqlx::query_scalar(
            "SELECT id FROM property_key WHERE key = ANY($1::text[]) ORDER BY key",
        )
        .bind(&key)
        .fetch_all(&pool)
        .await
        .unwrap();
        assert_eq!(property_key_id.len(), 100);
        sqlx::query("ANALYZE property_key, entity_property_history, entity_property")
            .execute(&pool)
            .await
            .unwrap();

        let mut transaction = pool.begin().await.unwrap();
        sqlx::query("SET LOCAL enable_seqscan = off")
            .execute(&mut *transaction)
            .await
            .unwrap();

        let hydration_explain = format!("EXPLAIN (COSTS OFF) {HYDRATE_ENTITY_PROPERTY_CHANGE_SQL}");
        let hydration_plan = sqlx::query_scalar::<_, String>(&hydration_explain)
            .bind(vec![activity_id])
            .fetch_all(&mut *transaction)
            .await
            .unwrap()
            .join("\n");
        assert!(
            hydration_plan.contains("entity_property_history_activity_index"),
            "the live bounded Activity hydration query must use its declared index: {hydration_plan}"
        );

        let current_explain = format!("EXPLAIN (COSTS OFF) {CURRENT_ENTITY_STATE_SQL}");
        let current_plan = sqlx::query_scalar::<_, String>(&current_explain)
            .bind(character.entity.id.0)
            .bind(Option::<i16>::None)
            .bind(Option::<i64>::None)
            .bind(Option::<Uuid>::None)
            .bind(101_i64)
            .fetch_all(&mut *transaction)
            .await
            .unwrap()
            .join("\n");
        assert!(
            current_plan.contains("entity_property_pkey")
                && current_plan.contains("entity_property_history_pkey")
                && current_plan.contains("entity_trait_current_entity_id_trait_id_index"),
            "the live combined current-read query must use Property and Trait current indexes: {current_plan}"
        );

        let value_type = vec!["integer"; 100];
        let key_insert_explain = format!("EXPLAIN (COSTS OFF) {INSERT_PROPERTY_KEY_SQL}");
        let key_insert_plan = sqlx::query_scalar::<_, String>(&key_insert_explain)
            .bind(&key)
            .bind(&value_type)
            .bind(activity_id)
            .fetch_all(&mut *transaction)
            .await
            .unwrap()
            .join("\n");
        assert!(
            key_insert_plan.contains("Conflict Arbiter Indexes: property_key_key_key")
                && key_insert_plan.contains("Function Scan on submitted"),
            "the live bulk key-arbitration write must use one UNNEST and its unique index: {key_insert_plan}"
        );

        let entity_id = vec![character.entity.id.0; 100];
        let pointer_lock_explain =
            format!("EXPLAIN (COSTS OFF) {LOCK_CURRENT_ENTITY_PROPERTY_SQL}");
        let pointer_lock_plan = sqlx::query_scalar::<_, String>(&pointer_lock_explain)
            .bind(&entity_id)
            .bind(&property_key_id)
            .fetch_all(&mut *transaction)
            .await
            .unwrap()
            .join("\n");
        assert!(
            pointer_lock_plan.contains("entity_property_pkey")
                && pointer_lock_plan.contains("Function Scan on submitted"),
            "the live pointer-lock query must use one submitted set and the current-state key: {pointer_lock_plan}"
        );

        let previous_activity_id = vec![Some(activity_id); 100];
        let text_value = vec![None::<String>; 100];
        let integer_value = (0..100).map(Some).collect::<Vec<_>>();
        let history_insert_explain =
            format!("EXPLAIN (COSTS OFF) {INSERT_ENTITY_PROPERTY_HISTORY_SQL}");
        let history_insert_plan = sqlx::query_scalar::<_, String>(&history_insert_explain)
            .bind(&entity_id)
            .bind(&property_key_id)
            .bind(Uuid::new_v4())
            .bind(&previous_activity_id)
            .bind(&value_type)
            .bind(&text_value)
            .bind(&integer_value)
            .fetch_all(&mut *transaction)
            .await
            .unwrap()
            .join("\n");
        assert!(
            history_insert_plan.contains("Function Scan on submitted")
                && history_insert_plan
                    .contains("Sort Key: submitted.entity_id, submitted.property_key_id"),
            "the live history write must remain one sorted set-based INSERT: {history_insert_plan}"
        );

        let pointer_upsert_explain =
            format!("EXPLAIN (COSTS OFF) {UPSERT_CURRENT_ENTITY_PROPERTY_SQL}");
        let pointer_upsert_plan = sqlx::query_scalar::<_, String>(&pointer_upsert_explain)
            .bind(&entity_id)
            .bind(&property_key_id)
            .bind(Uuid::new_v4())
            .fetch_all(&mut *transaction)
            .await
            .unwrap()
            .join("\n");
        assert!(
            pointer_upsert_plan.contains("Conflict Arbiter Indexes: entity_property_pkey")
                && pointer_upsert_plan.contains("Function Scan on submitted"),
            "the live current-pointer write must use one UNNEST and the pointer key: {pointer_upsert_plan}"
        );
        transaction.rollback().await.unwrap();
    }
}

#[cfg(test)]
mod trait_persistence_test {
    use super::*;
    use sqlx::PgPool;

    async fn append_trait_action(
        transaction: &mut Transaction<'_, Postgres>,
        user_id: UserId,
        involved: &[(EntityId, ActivityEntityRole)],
    ) -> ActivityId {
        let fingerprint = vec![8_u8; 32];
        append_activity(
            transaction,
            ActivityDraft {
                operation: ActivityOperation::SubmitAction,
                requested_by_user_id: user_id,
                actor_character_entity_id: None,
                context_place_entity_id: None,
                involved,
                prose: Some("A Trait changes."),
                request_id: Some(Uuid::new_v4()),
                request_fingerprint: Some(&fingerprint),
                action_consequence: Some("change_entity_trait"),
            },
            "trait_test",
        )
        .await
        .unwrap()
    }

    fn establish(entity_id: EntityId, statement: impl Into<String>) -> TraitWrite {
        TraitWrite::Establish {
            entity_id,
            statement: statement.into(),
        }
    }

    fn develop(trait_id: Uuid, statement: impl Into<String>) -> TraitWrite {
        TraitWrite::Develop {
            trait_id,
            statement: statement.into(),
        }
    }

    #[test]
    fn trait_normalization_enforces_bounds_text_and_exact_input_duplicates() {
        let entity_id = EntityId(Uuid::new_v4());
        let trait_id = Uuid::new_v4();
        assert_eq!(
            normalize_trait_writes(Vec::new(), false),
            Err(TraitNormalizationError::InvalidCount)
        );
        assert_eq!(normalize_trait_writes(Vec::new(), true), Ok(Vec::new()));
        assert_eq!(
            normalize_trait_writes(vec![establish(entity_id, " \t ")], false),
            Err(TraitNormalizationError::InvalidStatement(
                InvalidReason::Empty
            ))
        );
        assert_eq!(
            normalize_trait_writes(vec![develop(trait_id, "has\0nul")], false),
            Err(TraitNormalizationError::InvalidStatement(
                InvalidReason::ContainsNul
            ))
        );
        assert_eq!(
            normalize_trait_writes(
                vec![establish(
                    entity_id,
                    "x".repeat(MAX_TRAIT_STATEMENT_LENGTH + 1)
                )],
                false,
            ),
            Err(TraitNormalizationError::InvalidStatement(
                InvalidReason::TooLong
            ))
        );
        assert_eq!(
            normalize_trait_writes(
                vec![
                    establish(entity_id, "Same statement."),
                    establish(entity_id, " Same statement. "),
                ],
                false,
            ),
            Err(TraitNormalizationError::DuplicateEstablishment)
        );
        assert_eq!(
            normalize_trait_writes(
                vec![develop(trait_id, "First."), develop(trait_id, "Second.")],
                false,
            ),
            Err(TraitNormalizationError::DuplicateDevelopment)
        );
        assert_eq!(
            normalize_trait_writes(
                (0..=MAX_TRAIT_COUNT)
                    .map(|index| establish(entity_id, format!("Statement {index}.")))
                    .collect(),
                false,
            ),
            Err(TraitNormalizationError::InvalidCount)
        );
        let normalized = normalize_trait_writes(
            vec![
                develop(trait_id, " Developed statement. "),
                establish(entity_id, " Established statement. "),
            ],
            false,
        )
        .unwrap();
        assert_eq!(normalized[0].statement(), "Established statement.");
        assert_eq!(normalized[1].statement(), "Developed statement.");
    }

    #[sqlx::test(migrations = "./migration")]
    async fn mixed_trait_writer_establishes_develops_hydrates_and_rejects_exact_noops(
        pool: PgPool,
    ) {
        let world = World::new(pool.clone());
        let user = world.create_user().await.unwrap();
        let first = world
            .create_entity(
                user.id,
                CreateEntity {
                    name: "First Trait Writer Subject".to_owned(),
                    description: "A first subject.".to_owned(),
                    property: Vec::new(),
                },
            )
            .await
            .unwrap();
        let second = world
            .create_entity(
                user.id,
                CreateEntity {
                    name: "Second Trait Writer Subject".to_owned(),
                    description: "A second subject.".to_owned(),
                    property: Vec::new(),
                },
            )
            .await
            .unwrap();
        let eligible = [first.id, second.id];

        let mut transaction = pool.begin().await.unwrap();
        let establish_activity_id = append_trait_action(
            &mut transaction,
            user.id,
            &[
                (first.id, ActivityEntityRole::Subject),
                (second.id, ActivityEntityRole::Subject),
            ],
        )
        .await;
        let initial = normalize_trait_writes(
            vec![
                establish(first.id, "Startles at every hard sound."),
                establish(second.id, "Keeps watch beside the gate."),
            ],
            false,
        )
        .unwrap();
        let established =
            write_trait_changes(&mut transaction, establish_activity_id, &initial, &eligible)
                .await
                .unwrap();
        transaction.commit().await.unwrap();
        assert_eq!(established.len(), 2);
        assert!(
            established
                .iter()
                .all(|change| change.lifecycle == StoredTraitLifecycle::Establish)
        );
        let first_trait_id = established
            .iter()
            .find(|change| change.entity_id == first.id)
            .unwrap()
            .trait_id;

        let mut transaction = pool.begin().await.unwrap();
        let mixed_activity_id = append_trait_action(
            &mut transaction,
            user.id,
            &[
                (first.id, ActivityEntityRole::Subject),
                (second.id, ActivityEntityRole::Subject),
            ],
        )
        .await;
        let mixed = normalize_trait_writes(
            vec![
                establish(second.id, "Listens for footsteps beyond the arch."),
                develop(
                    first_trait_id,
                    "Waits for the second echo before springing.",
                ),
            ],
            false,
        )
        .unwrap();
        let mixed_change =
            write_trait_changes(&mut transaction, mixed_activity_id, &mixed, &eligible)
                .await
                .unwrap();
        transaction.commit().await.unwrap();
        assert_eq!(mixed_change.len(), 2);
        let developed = mixed_change
            .iter()
            .find(|change| change.lifecycle == StoredTraitLifecycle::Develop)
            .unwrap();
        assert_eq!(developed.trait_id, first_trait_id);
        assert_eq!(
            developed.previous_statement.as_deref(),
            Some("Startles at every hard sound.")
        );
        assert_eq!(
            developed.statement,
            "Waits for the second echo before springing."
        );

        for invalid in [
            establish(first.id, "Waits for the second echo before springing."),
            develop(
                first_trait_id,
                "Waits for the second echo before springing.",
            ),
        ] {
            let mut transaction = pool.begin().await.unwrap();
            let invalid_activity_id = append_trait_action(
                &mut transaction,
                user.id,
                &[(first.id, ActivityEntityRole::Subject)],
            )
            .await;
            let invalid = normalize_trait_writes(vec![invalid], false).unwrap();
            assert!(matches!(
                write_trait_changes(&mut transaction, invalid_activity_id, &invalid, &eligible,)
                    .await,
                Err(TraitPersistenceError::InvalidInput)
            ));
            transaction.rollback().await.unwrap();
        }
        let activity_count: i64 = sqlx::query_scalar(
            "SELECT count(*) FROM activity WHERE action_consequence = 'change_entity_trait'",
        )
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_eq!(activity_count, 2);
        let lineage: Vec<(Uuid, Option<Uuid>, String)> = sqlx::query_as(
            r#"
            SELECT activity_id, previous_activity_id, statement
            FROM entity_trait_version
            WHERE trait_id = $1
            ORDER BY activity_id
            "#,
        )
        .bind(first_trait_id)
        .fetch_all(&pool)
        .await
        .unwrap();
        assert_eq!(lineage.len(), 2);
        assert!(lineage.iter().any(|(_, previous, statement)| {
            *previous == Some(establish_activity_id.0)
                && statement == "Waits for the second echo before springing."
        }));
    }

    #[sqlx::test(migrations = "./migration")]
    async fn one_and_one_hundred_trait_establishments_use_constant_query_counts(pool: PgPool) {
        TRAIT_QUERY_COUNT
            .scope(
                RefCell::new(TraitQueryCount::default()),
                assert_constant_trait_query_counts(pool),
            )
            .await;
    }

    async fn assert_constant_trait_query_counts(pool: PgPool) {
        let world = World::new(pool.clone());
        let user = world.create_user().await.unwrap();
        let entity = world
            .create_entity(
                user.id,
                CreateEntity {
                    name: "Bounded Trait Subject".to_owned(),
                    description: "A subject with many bounded Traits.".to_owned(),
                    property: Vec::new(),
                },
            )
            .await
            .unwrap();

        async fn write_counted(
            pool: &PgPool,
            user_id: UserId,
            entity_id: EntityId,
            count: usize,
        ) -> (TraitQueryCount, Vec<StoredTraitChange>) {
            let mut transaction = pool.begin().await.unwrap();
            let activity_id = append_trait_action(
                &mut transaction,
                user_id,
                &[(entity_id, ActivityEntityRole::Subject)],
            )
            .await;
            let writes = normalize_trait_writes(
                (0..count)
                    .map(|index| establish(entity_id, format!("Batch {count} statement {index}.")))
                    .collect(),
                false,
            )
            .unwrap();
            TRAIT_QUERY_COUNT.with(|queries| *queries.borrow_mut() = TraitQueryCount::default());
            let stored = write_trait_changes(&mut transaction, activity_id, &writes, &[entity_id])
                .await
                .unwrap();
            let count = TRAIT_QUERY_COUNT.with(|queries| *queries.borrow());
            transaction.commit().await.unwrap();
            (count, stored)
        }

        let (one_count, one) = write_counted(&pool, user.id, entity.id, 1).await;
        let (hundred_count, hundred) = write_counted(&pool, user.id, entity.id, 100).await;
        assert_eq!(one.len(), 1);
        assert_eq!(hundred.len(), 100);
        assert_eq!(one_count, hundred_count);
        assert_eq!(
            one_count,
            TraitQueryCount {
                write: 5,
                current_read: 0,
                hydration: 1,
            }
        );
    }

    #[sqlx::test(migrations = "./migration")]
    async fn live_trait_duplicate_query_is_set_bounded_and_indexed(pool: PgPool) {
        let world = World::new(pool.clone());
        let user = world.create_user().await.unwrap();
        let entity = world
            .create_entity(
                user.id,
                CreateEntity {
                    name: "Indexed Active Trait Subject".to_owned(),
                    description: "Carries one hundred active indexed Traits.".to_owned(),
                    property: Vec::new(),
                },
            )
            .await
            .unwrap();
        let mut transaction = pool.begin().await.unwrap();
        let activity_id = append_trait_action(
            &mut transaction,
            user.id,
            &[(entity.id, ActivityEntityRole::Subject)],
        )
        .await;
        let writes = normalize_trait_writes(
            (0..100)
                .map(|index| establish(entity.id, format!("Indexed active statement {index}.")))
                .collect(),
            false,
        )
        .unwrap();
        write_trait_changes(&mut transaction, activity_id, &writes, &[entity.id])
            .await
            .unwrap();
        transaction.commit().await.unwrap();

        sqlx::query("ANALYZE entity_trait_version, entity_trait_current")
            .execute(&pool)
            .await
            .unwrap();
        let proposed_entity_id = vec![entity.id.0; 100];
        let proposed_statement = (0..100)
            .map(|index| format!("Indexed active statement {index}."))
            .collect::<Vec<_>>();
        let mut transaction = pool.begin().await.unwrap();
        sqlx::query("SET LOCAL enable_seqscan = off")
            .execute(&mut *transaction)
            .await
            .unwrap();
        let explain = format!("EXPLAIN (COSTS OFF) {LOCK_ACTIVE_ENTITY_TRAIT_STATEMENT_SQL}");
        let plan = sqlx::query_scalar::<_, String>(&explain)
            .bind(&proposed_entity_id)
            .bind(&proposed_statement)
            .bind(Vec::<Uuid>::new())
            .fetch_all(&mut *transaction)
            .await
            .unwrap()
            .join("\n");
        assert!(
            plan.contains("Function Scan on submitted")
                && plan.contains("entity_trait_current_entity_id_trait_id_index")
                && plan.contains("entity_trait_version_activity_entity_trait_index")
                && plan.contains("Limit"),
            "the live active-duplicate query must stay one bounded submitted set on declared indexes: {plan}"
        );
        transaction.rollback().await.unwrap();
    }

    #[sqlx::test(migrations = "./migration")]
    async fn public_trait_world_paths_are_set_bounded_for_one_and_one_hundred(pool: PgPool) {
        TRAIT_QUERY_COUNT
            .scope(
                RefCell::new(TraitQueryCount::default()),
                assert_public_trait_world_paths_are_set_bounded(pool),
            )
            .await;
    }

    async fn assert_public_trait_world_paths_are_set_bounded(pool: PgPool) {
        let world = World::new(pool);
        let user = world.create_user().await.unwrap();
        let character = world
            .create_character(
                user.id,
                CreateCharacter {
                    name: "Bounded Trait Character".to_owned(),
                    description: "Carries one or one hundred Traits.".to_owned(),
                    property: Vec::new(),
                },
            )
            .await
            .unwrap();
        world
            .create_entry_place(
                user.id,
                CreateEntryPlace {
                    name: "Bounded Trait Place".to_owned(),
                    description: "One exact current Place.".to_owned(),
                    property: Vec::new(),
                },
            )
            .await
            .unwrap();
        world.enter_world(user.id).await.unwrap();

        async fn submit_counted(
            world: &World,
            user_id: UserId,
            entity_id: EntityId,
            count: usize,
        ) -> TraitQueryCount {
            let revision = world
                .list_entity_at_current_place(user_id, ListEntityAtCurrentPlace::default())
                .await
                .unwrap()
                .place_revision;
            TRAIT_QUERY_COUNT.with(|queries| *queries.borrow_mut() = TraitQueryCount::default());
            world
                .submit_action(
                    user_id,
                    SubmitAction {
                        request_id: Uuid::new_v4(),
                        expected_place_revision: revision,
                        prose: format!("The World accepts {count} bounded Traits."),
                        consequence: ActionConsequence::ChangeEntityTrait(ChangeEntityTrait {
                            trait_change: (0..count)
                                .map(|index| EntityTraitChangeInput::Establish {
                                    entity_id,
                                    statement: format!("Public batch {count} statement {index}."),
                                })
                                .collect(),
                        }),
                    },
                )
                .await
                .unwrap();
            TRAIT_QUERY_COUNT.with(|queries| *queries.borrow())
        }

        let one_write = submit_counted(&world, user.id, character.entity.id, 1).await;
        TRAIT_QUERY_COUNT.with(|queries| *queries.borrow_mut() = TraitQueryCount::default());
        let one_current = world
            .get_character(
                user.id,
                GetEntityCurrentState {
                    cursor: None,
                    limit: 100,
                },
            )
            .await
            .unwrap();
        assert_eq!(one_current.current_state.association.len(), 1);
        let one_current_count = TRAIT_QUERY_COUNT.with(|queries| *queries.borrow());
        TRAIT_QUERY_COUNT.with(|queries| *queries.borrow_mut() = TraitQueryCount::default());
        let one_history = world
            .list_activity(
                user.id,
                ListActivity {
                    cursor: None,
                    limit: 1,
                },
            )
            .await
            .unwrap();
        assert_eq!(one_history.activity[0].trait_change.len(), 1);
        let one_history_count = TRAIT_QUERY_COUNT.with(|queries| *queries.borrow());

        let hundred_write = submit_counted(&world, user.id, character.entity.id, 100).await;
        TRAIT_QUERY_COUNT.with(|queries| *queries.borrow_mut() = TraitQueryCount::default());
        let hundred_current = world
            .get_character(
                user.id,
                GetEntityCurrentState {
                    cursor: None,
                    limit: 100,
                },
            )
            .await
            .unwrap();
        assert_eq!(hundred_current.current_state.association.len(), 100);
        let hundred_current_count = TRAIT_QUERY_COUNT.with(|queries| *queries.borrow());
        TRAIT_QUERY_COUNT.with(|queries| *queries.borrow_mut() = TraitQueryCount::default());
        let hundred_history = world
            .list_activity(
                user.id,
                ListActivity {
                    cursor: None,
                    limit: 1,
                },
            )
            .await
            .unwrap();
        assert_eq!(hundred_history.activity[0].trait_change.len(), 100);
        let hundred_history_count = TRAIT_QUERY_COUNT.with(|queries| *queries.borrow());

        assert_eq!(one_write, hundred_write);
        assert_eq!(
            one_write,
            TraitQueryCount {
                write: 5,
                current_read: 0,
                hydration: 2,
            }
        );
        assert_eq!(one_current_count, hundred_current_count);
        assert_eq!(
            one_current_count,
            TraitQueryCount {
                write: 0,
                current_read: 1,
                hydration: 0,
            }
        );
        assert_eq!(one_history_count, hundred_history_count);
        assert_eq!(
            one_history_count,
            TraitQueryCount {
                write: 0,
                current_read: 0,
                hydration: 1,
            }
        );
    }

    #[sqlx::test(migrations = "./migration")]
    async fn reversed_multi_trait_writes_complete_without_deadlock(pool: PgPool) {
        let world = World::new(pool.clone());
        let user = world.create_user().await.unwrap();
        let first = world
            .create_entity(
                user.id,
                CreateEntity {
                    name: "First Lock Subject".to_owned(),
                    description: "First lock-order subject.".to_owned(),
                    property: Vec::new(),
                },
            )
            .await
            .unwrap();
        let second = world
            .create_entity(
                user.id,
                CreateEntity {
                    name: "Second Lock Subject".to_owned(),
                    description: "Second lock-order subject.".to_owned(),
                    property: Vec::new(),
                },
            )
            .await
            .unwrap();
        let eligible = vec![first.id, second.id];
        let mut transaction = pool.begin().await.unwrap();
        let activity_id = append_trait_action(
            &mut transaction,
            user.id,
            &[
                (first.id, ActivityEntityRole::Subject),
                (second.id, ActivityEntityRole::Subject),
            ],
        )
        .await;
        let roots = normalize_trait_writes(
            vec![
                establish(first.id, "First root statement."),
                establish(second.id, "Second root statement."),
            ],
            false,
        )
        .unwrap();
        let established = write_trait_changes(&mut transaction, activity_id, &roots, &eligible)
            .await
            .unwrap();
        transaction.commit().await.unwrap();
        let first_trait_id = established
            .iter()
            .find(|change| change.entity_id == first.id)
            .unwrap()
            .trait_id;
        let second_trait_id = established
            .iter()
            .find(|change| change.entity_id == second.id)
            .unwrap()
            .trait_id;

        let run = |pool: PgPool,
                   user_id: UserId,
                   first_statement: &'static str,
                   second_statement: &'static str,
                   reverse: bool| {
            let eligible = eligible.clone();
            async move {
                let mut transaction = pool.begin().await.unwrap();
                let activity_id = append_trait_action(
                    &mut transaction,
                    user_id,
                    &[
                        (first.id, ActivityEntityRole::Subject),
                        (second.id, ActivityEntityRole::Subject),
                    ],
                )
                .await;
                let mut writes = vec![
                    develop(first_trait_id, first_statement),
                    develop(second_trait_id, second_statement),
                ];
                if reverse {
                    writes.reverse();
                }
                let writes = normalize_trait_writes(writes, false).unwrap();
                write_trait_changes(&mut transaction, activity_id, &writes, &eligible)
                    .await
                    .unwrap();
                transaction.commit().await.unwrap();
            }
        };
        let first_write = tokio::spawn(run(
            pool.clone(),
            user.id,
            "First concurrent statement A.",
            "Second concurrent statement A.",
            false,
        ));
        let second_write = tokio::spawn(run(
            pool.clone(),
            user.id,
            "First concurrent statement B.",
            "Second concurrent statement B.",
            true,
        ));
        tokio::time::timeout(std::time::Duration::from_secs(10), async {
            first_write.await.unwrap();
            second_write.await.unwrap();
        })
        .await
        .expect("stable Entity/Trait lock order must not deadlock");

        let versions: Vec<(Uuid, i64)> = sqlx::query_as(
            r#"
            SELECT trait_id, count(*)
            FROM entity_trait_version
            WHERE trait_id = ANY($1::uuid[])
            GROUP BY trait_id
            ORDER BY trait_id
            "#,
        )
        .bind(vec![first_trait_id, second_trait_id])
        .fetch_all(&pool)
        .await
        .unwrap();
        assert_eq!(versions.len(), 2);
        assert!(versions.iter().all(|(_, count)| *count == 3));
    }
}
