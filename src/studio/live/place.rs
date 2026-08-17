//! Place: the Entity role that holds World entry, membership and the revision
//! every accepted mutation at that Place advances.

use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::PgPool;
use uuid::Uuid;

use super::page::{Bound, Page};
use crate::studio::StudioError;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct PlaceListItem {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub is_entry: bool,
    pub latest_activity_id: Uuid,
    pub introduced_at: DateTime<Utc>,
}

/// Every Place, keyset by its stable `place.entity_id` primary key.
///
/// Place has no time index of its own, so the primary key is the ordering: it is
/// stable, unique and already indexed. The Entity join is a primary-key lookup.
pub async fn list_place(
    pool: &PgPool,
    before: Option<Uuid>,
    bound: Bound,
) -> Result<Page<PlaceListItem, Uuid>, StudioError> {
    let item = sqlx::query_as::<_, PlaceListItem>(
        r#"
        SELECT place.entity_id AS id,
               entity.name,
               entity.description,
               place.is_entry,
               place.latest_activity_id,
               entity.introduced_at
        FROM place
        JOIN entity ON entity.id = place.entity_id
        WHERE ($1::uuid IS NULL OR place.entity_id < $1::uuid)
        ORDER BY place.entity_id DESC
        LIMIT $2
        "#,
    )
    .bind(before)
    .bind(bound.fetch())
    .fetch_all(pool)
    .await?;
    Ok(Page::build(item, bound, |last| last.id))
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct PlaceDetail {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub introduced_by_user_id: Uuid,
    pub introduced_at: DateTime<Utc>,
    pub is_entry: bool,
    pub latest_activity_id: Uuid,
    pub latest_activity_occurred_at: DateTime<Utc>,
    pub latest_activity_operation: String,
}

/// One Place with its Entity identity, entry flag and current revision.
///
/// `place.latest_activity_id` is the Place revision every accepted mutation
/// advances while holding the Place lock; joining that Activity gives the exact
/// time and operation of the last accepted change. Everything here is a
/// primary-key lookup.
///
/// The Place's own history is a separate bounded read:
/// [`super::chronicle::list_place_chronicle`] with this `id`.
pub async fn get_place(pool: &PgPool, entity_id: Uuid) -> Result<PlaceDetail, StudioError> {
    sqlx::query_as::<_, PlaceDetail>(
        r#"
        SELECT place.entity_id AS id,
               entity.name,
               entity.description,
               entity.introduced_by_user_id,
               entity.introduced_at,
               place.is_entry,
               place.latest_activity_id,
               latest_activity.occurred_at AS latest_activity_occurred_at,
               latest_activity.operation AS latest_activity_operation
        FROM place
        JOIN entity ON entity.id = place.entity_id
        JOIN activity AS latest_activity ON latest_activity.id = place.latest_activity_id
        WHERE place.entity_id = $1
        "#,
    )
    .bind(entity_id)
    .fetch_optional(pool)
    .await?
    .ok_or(StudioError::NotFound)
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct PlaceEntityItem {
    pub id: Uuid,
    pub name: String,
    pub introduced_at: DateTime<Utc>,
    pub is_character: bool,
    pub is_place: bool,
}

/// Every Entity explicitly located at one Place.
///
/// Served by `entity_location_place_entity_id_entity_id_index` —
/// `(place_entity_id, entity_id)` — and keyset by `entity_id` ascending, so a
/// Place holding a million Entities pages at constant cost.
pub async fn list_place_entity(
    pool: &PgPool,
    place_entity_id: Uuid,
    after: Option<Uuid>,
    bound: Bound,
) -> Result<Page<PlaceEntityItem, Uuid>, StudioError> {
    let item = sqlx::query_as::<_, PlaceEntityItem>(
        r#"
        SELECT entity.id,
               entity.name,
               entity.introduced_at,
               EXISTS (SELECT 1 FROM character WHERE character.entity_id = entity.id)
                   AS is_character,
               EXISTS (SELECT 1 FROM place WHERE place.entity_id = entity.id)
                   AS is_place
        FROM entity_location
        JOIN entity ON entity.id = entity_location.entity_id
        WHERE entity_location.place_entity_id = $1
          AND ($2::uuid IS NULL OR entity_location.entity_id > $2::uuid)
        ORDER BY entity_location.entity_id
        LIMIT $3
        "#,
    )
    .bind(place_entity_id)
    .bind(after)
    .bind(bound.fetch())
    .fetch_all(pool)
    .await?;
    Ok(Page::build(item, bound, |last| last.id))
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct PlaceCharacterItem {
    pub id: Uuid,
    pub name: String,
    pub owner_user_id: Uuid,
}

/// Every Character whose current Place is this Place.
///
/// Served by the partial index
/// `character_current_place_entity_id_entity_id_index` —
/// `(current_place_entity_id, entity_id) WHERE current_place_entity_id IS NOT
/// NULL` — and keyset by `entity_id` ascending. A Character that has not entered
/// the World has no current Place and never appears here.
pub async fn list_place_character(
    pool: &PgPool,
    place_entity_id: Uuid,
    after: Option<Uuid>,
    bound: Bound,
) -> Result<Page<PlaceCharacterItem, Uuid>, StudioError> {
    let item = sqlx::query_as::<_, PlaceCharacterItem>(
        r#"
        SELECT character.entity_id AS id,
               entity.name,
               character.owner_user_id
        FROM character
        JOIN entity ON entity.id = character.entity_id
        WHERE character.current_place_entity_id = $1
          AND ($2::uuid IS NULL OR character.entity_id > $2::uuid)
        ORDER BY character.entity_id
        LIMIT $3
        "#,
    )
    .bind(place_entity_id)
    .bind(after)
    .bind(bound.fetch())
    .fetch_all(pool)
    .await?;
    Ok(Page::build(item, bound, |last| last.id))
}
