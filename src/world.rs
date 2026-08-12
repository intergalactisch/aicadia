use std::collections::HashMap;

use chrono::{DateTime, Utc};
use sha2::{Digest, Sha256};
use sqlx::{FromRow, PgPool, Postgres, Transaction};
use thiserror::Error;
use uuid::Uuid;

const MAX_ENTITY_NAME_LENGTH: usize = 120;
const MAX_ENTITY_DESCRIPTION_LENGTH: usize = 4_000;
const MAX_ACTION_PROSE_LENGTH: usize = 4_000;
const DEFAULT_PAGE_SIZE: u16 = 25;
const MAX_PAGE_SIZE: u16 = 100;
const WORLD_NAME: &str = "Aicadia";

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
            },
            "create_entity",
        )
        .await?;
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

    pub async fn get_character(&self, user_id: UserId) -> Result<Character, WorldError> {
        self.get_user(user_id).await?;
        find_character_pool(&self.pool, user_id, "get_character")
            .await?
            .ok_or(WorldError::CharacterNotFound)
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
        append_activity(
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
            },
            "create_character",
        )
        .await?;
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
            },
            "create_entry_place",
        )
        .await?;
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
        let character = self.get_character(user_id).await?;
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
                    activity.actor_character_entity_id = $1
                    OR EXISTS (
                        SELECT 1
                        FROM activity_entity
                        WHERE activity_entity.activity_id = activity.id
                          AND activity_entity.entity_id = $1
                    )
                  )
              AND (
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
        .fetch_all(&self.pool)
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
        let activity_ids = row.iter().map(|row| row.id.0).collect::<Vec<_>>();
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
            .fetch_all(&self.pool)
            .await
            .map_err(|error| storage_error("list_activity", error))?
        };
        let mut involved_by_activity: HashMap<ActivityId, Vec<ActivityEntityReference>> =
            HashMap::new();
        for related in related {
            involved_by_activity
                .entry(related.activity_id)
                .or_default()
                .push(related.try_into()?);
        }
        let activity = row
            .into_iter()
            .map(|row| {
                let id = row.id;
                row.into_activity(involved_by_activity.remove(&id).unwrap_or_default())
            })
            .collect::<Result<Vec<_>, _>>()?;

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
        let mut row = sqlx::query_as::<_, EntityListRow>(
            r#"
            SELECT entity.id, entity.name, entity.introduced_at
            FROM entity_location
            JOIN entity ON entity.id = entity_location.entity_id
            WHERE entity_location.place_entity_id = $1
              AND (
                    $2::timestamptz IS NULL
                    OR (entity.introduced_at, entity.id) < ($2::timestamptz, $3::uuid)
                  )
            ORDER BY entity.introduced_at DESC, entity.id DESC
            LIMIT $4
            "#,
        )
        .bind(place.entity.id.0)
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
            .map(|row| EntitySummary {
                id: row.id,
                name: row.name,
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
                    $2::timestamptz IS NULL
                    OR (activity.occurred_at, activity.id) < ($2::timestamptz, $3::uuid)
                  )
            ORDER BY activity.occurred_at DESC, activity.id DESC
            LIMIT $4
            "#,
        )
        .bind(place.entity.id.0)
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

        let entity = insert_entity(
            &mut transaction,
            user_id,
            input.consequence.name,
            input.consequence.description,
        )
        .await
        .map_err(|error| storage_error("submit_action", error))?;
        sqlx::query("INSERT INTO entity_location (entity_id, place_entity_id) VALUES ($1, $2)")
            .bind(entity.id.0)
            .bind(place.entity.id.0)
            .execute(&mut *transaction)
            .await
            .map_err(|error| storage_error("submit_action", error))?;
        let activity_id = append_activity(
            &mut transaction,
            ActivityDraft {
                operation: ActivityOperation::SubmitAction,
                requested_by_user_id: user_id,
                actor_character_entity_id: Some(character.entity.id),
                context_place_entity_id: Some(place.entity.id),
                involved: &[
                    (entity.id, ActivityEntityRole::Subject),
                    (place.entity.id, ActivityEntityRole::Location),
                ],
                prose: Some(&input.prose),
                request_id: Some(input.request_id),
                request_fingerprint: Some(&request_fingerprint),
            },
            "submit_action",
        )
        .await?;
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

fn action_fingerprint(input: &SubmitAction) -> Vec<u8> {
    let mut hash = Sha256::new();
    for field in [
        b"aicadia-submit-action-fingerprint-v1".as_slice(),
        input.expected_place_revision.fingerprint_bytes().as_slice(),
        input.prose.as_bytes(),
        b"introduce_entity".as_slice(),
        input.consequence.name.as_bytes(),
        input.consequence.description.as_bytes(),
    ] {
        hash.update((field.len() as u64).to_be_bytes());
        hash.update(field);
    }
    hash.finalize().to_vec()
}

async fn activities_from_rows(
    transaction: &mut Transaction<'_, Postgres>,
    row: Vec<ActivityRow>,
    operation: &'static str,
) -> Result<Vec<Activity>, WorldError> {
    let activity_ids = row.iter().map(|row| row.id.0).collect::<Vec<_>>();
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
    row.into_iter()
        .map(|row| {
            let id = row.id;
            row.into_activity(involved_by_activity.remove(&id).unwrap_or_default())
        })
        .collect()
}

struct StoredAcceptedAction {
    request_fingerprint: Vec<u8>,
    accepted_action: AcceptedAction,
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
               activity.request_fingerprint,
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
    let request_fingerprint = row.request_fingerprint.clone();
    let activity = activities_from_rows(transaction, vec![row.into()], operation)
        .await?
        .pop()
        .ok_or_else(invalid_stored_relation)?;
    if activity.operation != ActivityOperation::SubmitAction {
        return Err(invalid_stored_relation());
    }
    let subject_id = activity
        .involved_entity
        .iter()
        .find(|reference| reference.role == ActivityEntityRole::Subject)
        .map(|reference| reference.entity.id)
        .ok_or_else(invalid_stored_relation)?;
    let location_id = activity
        .involved_entity
        .iter()
        .find(|reference| reference.role == ActivityEntityRole::Location)
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
    let place = find_place_by_id(transaction, location_id, operation)
        .await?
        .ok_or_else(invalid_stored_relation)?;
    debug_assert_eq!(activity.id, activity_id);
    Ok(Some(StoredAcceptedAction {
        request_fingerprint,
        accepted_action: AcceptedAction {
            activity,
            entity,
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

async fn find_character_pool(
    pool: &PgPool,
    user_id: UserId,
    operation: &'static str,
) -> Result<Option<Character>, WorldError> {
    sqlx::query_as::<_, CharacterRow>(&character_query(false))
        .bind(user_id.0)
        .fetch_optional(pool)
        .await
        .map_err(|error| storage_error(operation, error))?
        .map(CharacterRow::try_into)
        .transpose()
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

struct ActivityDraft<'a> {
    operation: ActivityOperation,
    requested_by_user_id: UserId,
    actor_character_entity_id: Option<EntityId>,
    context_place_entity_id: Option<EntityId>,
    involved: &'a [(EntityId, ActivityEntityRole)],
    prose: Option<&'a str>,
    request_id: Option<Uuid>,
    request_fingerprint: Option<&'a [u8]>,
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
            prose, request_id, request_fingerprint
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
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
    .execute(&mut **transaction)
    .await
    .map_err(|error| storage_error(storage_operation, error))?;
    for (entity_id, role) in draft.involved {
        sqlx::query(
            "INSERT INTO activity_entity (activity_id, entity_id, role) VALUES ($1, $2, $3)",
        )
        .bind(activity_id.0)
        .bind(entity_id.0)
        .bind(role.as_str())
        .execute(&mut **transaction)
        .await
        .map_err(|error| storage_error(storage_operation, error))?;
    }
    Ok(activity_id)
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
pub struct CreateEntity {
    pub name: String,
    pub description: String,
}

impl CreateEntity {
    fn normalize(self) -> Result<Self, WorldError> {
        let (name, description) =
            normalize_entity_text(self.name, self.description, |field, reason| {
                WorldError::InvalidEntity { field, reason }
            })?;
        Ok(Self { name, description })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CreateCharacter {
    pub name: String,
    pub description: String,
}

impl CreateCharacter {
    fn normalize(self) -> Result<Self, WorldError> {
        let (name, description) =
            normalize_entity_text(self.name, self.description, |field, reason| {
                WorldError::InvalidCharacter { field, reason }
            })?;
        Ok(Self { name, description })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CreateEntryPlace {
    pub name: String,
    pub description: String,
}

impl CreateEntryPlace {
    fn normalize(self) -> Result<Self, WorldError> {
        let (name, description) =
            normalize_entity_text(self.name, self.description, |field, reason| {
                WorldError::InvalidPlace { field, reason }
            })?;
        Ok(Self { name, description })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SubmitAction {
    pub request_id: Uuid,
    pub expected_place_revision: PlaceRevision,
    pub prose: String,
    pub consequence: IntroduceEntity,
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
        Ok(Self { name, description })
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
    pub entity: Entity,
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
    pub entity: Vec<EntitySummary>,
    pub next: Option<EntityCursor>,
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
pub struct Activity {
    pub id: ActivityId,
    pub operation: ActivityOperation,
    pub actor_character: Option<EntitySummary>,
    pub context_place: Option<PlaceSummary>,
    pub involved_entity: Vec<ActivityEntityReference>,
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
}

impl ActivityOperation {
    fn as_str(self) -> &'static str {
        match self {
            Self::CreateCharacter => "create_character",
            Self::CreateEntity => "create_entity",
            Self::CreateEntryPlace => "create_entry_place",
            Self::EnterWorld => "enter_world",
            Self::SubmitAction => "submit_action",
        }
    }

    fn parse(value: &str) -> Result<Self, WorldError> {
        match value {
            "create_character" => Ok(Self::CreateCharacter),
            "create_entity" => Ok(Self::CreateEntity),
            "create_entry_place" => Ok(Self::CreateEntryPlace),
            "enter_world" => Ok(Self::EnterWorld),
            "submit_action" => Ok(Self::SubmitAction),
            _ => Err(invalid_stored_relation()),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActivityEntityRole {
    Subject,
    Destination,
    Location,
}

impl ActivityEntityRole {
    fn as_str(self) -> &'static str {
        match self {
            Self::Subject => "subject",
            Self::Destination => "destination",
            Self::Location => "location",
        }
    }

    fn parse(value: &str) -> Result<Self, WorldError> {
        match value {
            "subject" => Ok(Self::Subject),
            "destination" => Ok(Self::Destination),
            "location" => Ok(Self::Location),
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
pub enum InvalidReason {
    Empty,
    ContainsNul,
    TooLong,
}

#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum WorldError {
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
    #[error("entity list limit must be between 1 and 100")]
    InvalidEntityLimit,
    #[error("activity list limit must be between 1 and 100")]
    InvalidActivityLimit,
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
