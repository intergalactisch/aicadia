//! Bounded, operator-only reads over the one connected local World.
//!
//! Every read in this module tree is a pure function over a [`PgPool`]: it takes
//! its subject and a [`page::Bound`], returns a serializable result and never
//! writes. Reads are keyset-paged over an index or primary key that already
//! exists for current game behavior; no Studio read adds a game index. Where an
//! honest answer needs a path PostgreSQL cannot serve from an index, the read
//! says so in its own doc comment and carries a `scope` or `truncated` field the
//! page can show, so an operator never mistakes a local-development sort for a
//! game read.

pub mod character;
pub mod chronicle;
pub mod entity;
pub mod estimate;
pub mod investigation;
pub mod migration;
pub mod page;
pub mod place;
pub mod property;
pub mod resolve;
pub mod row;
pub mod schema;
pub mod r#trait;
pub mod user;

use axum::{
    Json,
    extract::{Path, Query, State},
    http::header,
    response::{IntoResponse, Response},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{StudioError, StudioState};
use page::Bound;
use schema::StorageSnapshot;

const ENTITY_STATE_PREVIEW_LIMIT: i64 = 50;
const ACTIVITY_ENTITY_PREVIEW_LIMIT: i64 = 256;

#[derive(Deserialize)]
pub(super) struct PageQuery {
    limit: Option<u16>,
    before: Option<Uuid>,
}

impl PageQuery {
    fn bound(&self) -> Result<Bound, StudioError> {
        Bound::new(self.limit)
    }
}

#[derive(Serialize)]
pub(super) struct CharacterPage {
    character: Vec<character::CharacterListItem>,
    next_cursor: Option<Uuid>,
}

pub(super) async fn list_character(
    State(state): State<StudioState>,
    Query(query): Query<PageQuery>,
) -> Result<Json<CharacterPage>, StudioError> {
    let page = character::list_character(&state.pool, query.before, query.bound()?).await?;
    Ok(Json(CharacterPage {
        character: page.item,
        next_cursor: page.next_cursor,
    }))
}

#[derive(Serialize)]
pub(super) struct PlacePage {
    place: Vec<place::PlaceListItem>,
    next_cursor: Option<Uuid>,
}

pub(super) async fn list_place(
    State(state): State<StudioState>,
    Query(query): Query<PageQuery>,
) -> Result<Json<PlacePage>, StudioError> {
    let page = place::list_place(&state.pool, query.before, query.bound()?).await?;
    Ok(Json(PlacePage {
        place: page.item,
        next_cursor: page.next_cursor,
    }))
}

#[derive(Serialize, sqlx::FromRow)]
pub(super) struct EntityView {
    id: Uuid,
    name: String,
    description: String,
    introduced_by_user_id: Uuid,
    introduced_at: DateTime<Utc>,
    is_character: bool,
    is_place: bool,
    place_entity_id: Option<Uuid>,
    place_name: Option<String>,
}

#[derive(Serialize, sqlx::FromRow)]
pub(super) struct PropertyView {
    key: String,
    value_type: String,
    text_value: Option<String>,
    integer_value: Option<i64>,
    current_activity_id: Uuid,
}

#[derive(Serialize, sqlx::FromRow)]
pub(super) struct TraitView {
    id: Uuid,
    statement: String,
    current_activity_id: Uuid,
}

#[derive(Serialize)]
pub(super) struct EntityDetail {
    entity: EntityView,
    property: Vec<PropertyView>,
    property_truncated: bool,
    r#trait: Vec<TraitView>,
    trait_truncated: bool,
}

pub(super) async fn get_entity(
    State(state): State<StudioState>,
    Path(entity_id): Path<Uuid>,
) -> Result<Json<EntityDetail>, StudioError> {
    let entity = sqlx::query_as::<_, EntityView>(
        r#"
        SELECT entity.id,
               entity.name,
               entity.description,
               entity.introduced_by_user_id,
               entity.introduced_at,
               EXISTS (SELECT 1 FROM character WHERE character.entity_id = entity.id) AS is_character,
               EXISTS (SELECT 1 FROM place WHERE place.entity_id = entity.id) AS is_place,
               entity_location.place_entity_id,
               location.name AS place_name
        FROM entity
        LEFT JOIN entity_location ON entity_location.entity_id = entity.id
        LEFT JOIN entity AS location ON location.id = entity_location.place_entity_id
        WHERE entity.id = $1
        "#,
    )
    .bind(entity_id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or(StudioError::NotFound)?;
    let mut property = sqlx::query_as::<_, PropertyView>(
        r#"
        SELECT property_key.key,
               history.value_type,
               history.text_value,
               history.integer_value,
               entity_property.current_activity_id
        FROM entity_property
        JOIN property_key ON property_key.id = entity_property.property_key_id
        JOIN entity_property_history AS history
          ON history.entity_id = entity_property.entity_id
         AND history.property_key_id = entity_property.property_key_id
         AND history.activity_id = entity_property.current_activity_id
        WHERE entity_property.entity_id = $1
        ORDER BY property_key.key
        LIMIT $2
        "#,
    )
    .bind(entity_id)
    .bind(ENTITY_STATE_PREVIEW_LIMIT + 1)
    .fetch_all(&state.pool)
    .await?;
    let property_truncated = page::truncate(&mut property, ENTITY_STATE_PREVIEW_LIMIT as usize);
    let mut r#trait = sqlx::query_as::<_, TraitView>(
        r#"
        SELECT entity_trait.id,
               version.statement,
               entity_trait_current.current_activity_id
        FROM entity_trait
        JOIN entity_trait_current ON entity_trait_current.trait_id = entity_trait.id
        JOIN entity_trait_version AS version
          ON version.trait_id = entity_trait_current.trait_id
         AND version.entity_id = entity_trait_current.entity_id
         AND version.activity_id = entity_trait_current.current_activity_id
        WHERE entity_trait.entity_id = $1
        ORDER BY entity_trait.id
        LIMIT $2
        "#,
    )
    .bind(entity_id)
    .bind(ENTITY_STATE_PREVIEW_LIMIT + 1)
    .fetch_all(&state.pool)
    .await?;
    let trait_truncated = page::truncate(&mut r#trait, ENTITY_STATE_PREVIEW_LIMIT as usize);
    Ok(Json(EntityDetail {
        entity,
        property,
        property_truncated,
        r#trait,
        trait_truncated,
    }))
}

#[derive(Serialize, sqlx::FromRow)]
pub(super) struct ActivityView {
    id: Uuid,
    operation: String,
    prose: Option<String>,
    action_consequence: Option<String>,
    actor_character_entity_id: Option<Uuid>,
    context_place_entity_id: Option<Uuid>,
    occurred_at: DateTime<Utc>,
}

#[derive(Serialize, sqlx::FromRow)]
pub(super) struct ActivityEntityView {
    entity_id: Uuid,
    entity_name: String,
    role: String,
}

#[derive(Serialize)]
pub(super) struct ActivityDetail {
    activity: ActivityView,
    involved_entity: Vec<ActivityEntityView>,
    involved_entity_truncated: bool,
}

pub(super) async fn get_activity(
    State(state): State<StudioState>,
    Path(activity_id): Path<Uuid>,
) -> Result<Json<ActivityDetail>, StudioError> {
    let activity = sqlx::query_as::<_, ActivityView>(
        r#"
        SELECT id,
               operation,
               prose,
               action_consequence,
               actor_character_entity_id,
               context_place_entity_id,
               occurred_at
        FROM activity
        WHERE id = $1
        "#,
    )
    .bind(activity_id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or(StudioError::NotFound)?;
    let mut involved_entity = sqlx::query_as::<_, ActivityEntityView>(
        r#"
        SELECT activity_entity.entity_id,
               entity.name AS entity_name,
               activity_entity.role
        FROM activity_entity
        JOIN entity ON entity.id = activity_entity.entity_id
        WHERE activity_entity.activity_id = $1
        ORDER BY activity_entity.role, activity_entity.entity_id
        LIMIT $2
        "#,
    )
    .bind(activity_id)
    .bind(ACTIVITY_ENTITY_PREVIEW_LIMIT + 1)
    .fetch_all(&state.pool)
    .await?;
    let involved_entity_truncated =
        page::truncate(&mut involved_entity, ACTIVITY_ENTITY_PREVIEW_LIMIT as usize);
    Ok(Json(ActivityDetail {
        activity,
        involved_entity,
        involved_entity_truncated,
    }))
}

pub(super) async fn get_storage(
    State(state): State<StudioState>,
) -> Result<Json<StorageSnapshot>, StudioError> {
    Ok(Json(schema::read_storage(&state.pool).await?))
}

pub(super) async fn download_storage(
    State(state): State<StudioState>,
) -> Result<Response, StudioError> {
    let snapshot = schema::read_storage(&state.pool).await?;
    let body = serde_json::to_string_pretty(&snapshot)
        .expect("the serializable Studio schema snapshot must encode as JSON");
    Ok((
        [
            (header::CONTENT_TYPE, "application/json; charset=utf-8"),
            (
                header::CONTENT_DISPOSITION,
                "attachment; filename=\"aicadia-schema-snapshot.json\"",
            ),
        ],
        body,
    )
        .into_response())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_studio_page_query_shares_the_one_hard_read_bound() {
        assert_eq!(
            PageQuery {
                limit: None,
                before: None
            }
            .bound()
            .unwrap()
            .limit(),
            24
        );
        assert!(matches!(
            PageQuery {
                limit: Some(0),
                before: None
            }
            .bound(),
            Err(StudioError::InvalidLimit)
        ));
        assert!(matches!(
            PageQuery {
                limit: Some(101),
                before: None
            }
            .bound(),
            Err(StudioError::InvalidLimit)
        ));
        assert_eq!(
            PageQuery {
                limit: Some(100),
                before: None
            }
            .bound()
            .unwrap()
            .limit(),
            100
        );
    }
}
