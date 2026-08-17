//! Character: the User-owned Entity role and its optional current Place.

use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::PgPool;
use uuid::Uuid;

use super::entity::EntityRef;
use super::investigation::{self, AttemptCursor, AttemptItem};
use super::page::{Bound, Page};
use crate::studio::StudioError;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct CharacterListItem {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub owner_user_id: Uuid,
    pub current_place_entity_id: Option<Uuid>,
    pub current_place_name: Option<String>,
    pub introduced_at: DateTime<Utc>,
}

/// Every Character, keyset by its stable `character.entity_id` primary key.
///
/// Character has no time index of its own, so the primary key is the ordering.
/// Both the Entity and the current-Place joins are primary-key lookups.
pub async fn list_character(
    pool: &PgPool,
    before: Option<Uuid>,
    bound: Bound,
) -> Result<Page<CharacterListItem, Uuid>, StudioError> {
    let item = sqlx::query_as::<_, CharacterListItem>(
        r#"
        SELECT character.entity_id AS id,
               entity.name,
               entity.description,
               character.owner_user_id,
               character.current_place_entity_id,
               current_place.name AS current_place_name,
               entity.introduced_at
        FROM character
        JOIN entity ON entity.id = character.entity_id
        LEFT JOIN entity AS current_place ON current_place.id = character.current_place_entity_id
        WHERE ($1::uuid IS NULL OR character.entity_id < $1::uuid)
        ORDER BY character.entity_id DESC
        LIMIT $2
        "#,
    )
    .bind(before)
    .bind(bound.fetch())
    .fetch_all(pool)
    .await?;
    Ok(Page::build(item, bound, |last| last.id))
}

#[derive(Debug, Serialize)]
pub struct CharacterDetail {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub introduced_by_user_id: Uuid,
    pub introduced_at: DateTime<Utc>,
    pub owner_user_id: Uuid,
    pub owner_created_at: DateTime<Utc>,
    pub current_place: Option<EntityRef>,
    pub current_place_is_entry: Option<bool>,
}

#[derive(sqlx::FromRow)]
struct CharacterRow {
    id: Uuid,
    name: String,
    description: String,
    introduced_by_user_id: Uuid,
    introduced_at: DateTime<Utc>,
    owner_user_id: Uuid,
    owner_created_at: DateTime<Utc>,
    current_place_entity_id: Option<Uuid>,
    current_place_name: Option<String>,
    current_place_is_entry: Option<bool>,
}

/// One Character with its Entity identity, owning User and current Place.
///
/// Every lookup here is a primary key: `character(entity_id)`, `entity(id)`,
/// `user(id)` and `place(entity_id)`. An absent `current_place` means the
/// Character exists but has not entered the World.
///
/// What this Character did is a separate bounded read:
/// [`super::chronicle::list_character_chronicle`] with this `id`.
pub async fn get_character(pool: &PgPool, entity_id: Uuid) -> Result<CharacterDetail, StudioError> {
    let row = sqlx::query_as::<_, CharacterRow>(
        r#"
        SELECT character.entity_id AS id,
               entity.name,
               entity.description,
               entity.introduced_by_user_id,
               entity.introduced_at,
               character.owner_user_id,
               owner.created_at AS owner_created_at,
               character.current_place_entity_id,
               current_place_entity.name AS current_place_name,
               current_place.is_entry AS current_place_is_entry
        FROM character
        JOIN entity ON entity.id = character.entity_id
        JOIN "user" AS owner ON owner.id = character.owner_user_id
        LEFT JOIN place AS current_place
          ON current_place.entity_id = character.current_place_entity_id
        LEFT JOIN entity AS current_place_entity
          ON current_place_entity.id = character.current_place_entity_id
        WHERE character.entity_id = $1
        "#,
    )
    .bind(entity_id)
    .fetch_optional(pool)
    .await?
    .ok_or(StudioError::NotFound)?;
    Ok(CharacterDetail {
        id: row.id,
        name: row.name,
        description: row.description,
        introduced_by_user_id: row.introduced_by_user_id,
        introduced_at: row.introduced_at,
        owner_user_id: row.owner_user_id,
        owner_created_at: row.owner_created_at,
        current_place: match (row.current_place_entity_id, row.current_place_name) {
            (Some(id), Some(name)) => Some(EntityRef { id, name }),
            _ => None,
        },
        current_place_is_entry: row.current_place_is_entry,
    })
}

/// One Character's investigation attempts, newest first.
///
/// `investigation_attempt.character_entity_id` carries no index, so this read
/// never filters on it. It resolves the Character's owning User through the
/// unique `character_owner_user_id_key` and then reads that User's attempts
/// through `investigation_attempt_user_created_at_index`. The two sets are the
/// same set: a User owns at most one Character, and every attempt stores the
/// Character World derived from its requesting User.
pub async fn list_character_attempt(
    pool: &PgPool,
    character_entity_id: Uuid,
    before: Option<AttemptCursor>,
    bound: Bound,
) -> Result<Page<AttemptItem, AttemptCursor>, StudioError> {
    let owner_user_id =
        sqlx::query_scalar::<_, Uuid>("SELECT owner_user_id FROM character WHERE entity_id = $1")
            .bind(character_entity_id)
            .fetch_optional(pool)
            .await?
            .ok_or(StudioError::NotFound)?;
    investigation::list_user_attempt(pool, owner_user_id, before, bound).await
}
