//! Entity: the durable World subject, its roles, current state and history.

use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::PgPool;
use uuid::Uuid;

use super::page::{self, Bound, Page};
use crate::StudioError;

/// The bounded current-state preview one Entity detail carries.
const STATE_PREVIEW_LIMIT: usize = 50;

/// One cross-link to an Entity: its stable id and its display name.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct EntityRef {
    pub id: Uuid,
    pub name: String,
}

/// The keyset of the shared newest-first Entity list.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct EntityCursor {
    pub introduced_at: DateTime<Utc>,
    pub id: Uuid,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct EntityListItem {
    pub id: Uuid,
    pub name: String,
    pub introduced_at: DateTime<Utc>,
    pub introduced_by_user_id: Uuid,
    pub is_character: bool,
    pub is_place: bool,
}

/// The Studio-owned newest-first Entity list.
///
/// Keyset over `entity_introduced_at_id_index` — `(introduced_at DESC, id DESC)`
/// — so the cost is the same whether the World holds ten Entities or ten million.
/// Studio owns this read; the game's own loopback Entity reads are a separate
/// surface and are not consulted here.
pub async fn list_entity(
    pool: &PgPool,
    before: Option<EntityCursor>,
    bound: Bound,
) -> Result<Page<EntityListItem, EntityCursor>, StudioError> {
    let item = sqlx::query_as::<_, EntityListItem>(
        r#"
        SELECT entity.id,
               entity.name,
               entity.introduced_at,
               entity.introduced_by_user_id,
               EXISTS (SELECT 1 FROM character WHERE character.entity_id = entity.id)
                   AS is_character,
               EXISTS (SELECT 1 FROM place WHERE place.entity_id = entity.id)
                   AS is_place
        FROM entity
        WHERE $1::timestamptz IS NULL
           OR (entity.introduced_at, entity.id) < ($1::timestamptz, $2::uuid)
        ORDER BY entity.introduced_at DESC, entity.id DESC
        LIMIT $3
        "#,
    )
    .bind(before.map(|cursor| cursor.introduced_at))
    .bind(before.map(|cursor| cursor.id))
    .bind(bound.fetch())
    .fetch_all(pool)
    .await?;
    Ok(Page::build(item, bound, |last| EntityCursor {
        introduced_at: last.introduced_at,
        id: last.id,
    }))
}

#[derive(Debug, Serialize)]
pub struct EntityIdentity {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub introduced_by_user_id: Uuid,
    pub introduced_at: DateTime<Utc>,
    pub is_character: bool,
    pub owner_user_id: Option<Uuid>,
    pub is_place: bool,
    pub is_entry_place: Option<bool>,
    pub current_place: Option<EntityRef>,
}

#[derive(sqlx::FromRow)]
struct EntityIdentityRow {
    id: Uuid,
    name: String,
    description: String,
    introduced_by_user_id: Uuid,
    introduced_at: DateTime<Utc>,
    is_character: bool,
    owner_user_id: Option<Uuid>,
    is_place: bool,
    is_entry_place: Option<bool>,
    current_place_entity_id: Option<Uuid>,
    current_place_name: Option<String>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct EntityProperty {
    pub property_key_id: i64,
    pub key: String,
    pub value_type: String,
    pub text_value: Option<String>,
    pub integer_value: Option<i64>,
    pub current_activity_id: Uuid,
    #[sqlx(rename = "counted_version")]
    pub version_count: i64,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct EntityTraitCurrent {
    pub id: Uuid,
    pub statement: String,
    pub current_activity_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct EntityDetail {
    pub entity: EntityIdentity,
    pub property: Vec<EntityProperty>,
    pub property_truncated: bool,
    pub property_version_count_truncated: bool,
    pub r#trait: Vec<EntityTraitCurrent>,
    pub trait_truncated: bool,
}

/// One Entity with its roles, current Place and a bounded preview of its current
/// Properties and Traits.
///
/// Identity, roles and placement are primary-key lookups. The current-state
/// previews stop at fifty rows each and are ordered by their own primary keys —
/// `entity_property(entity_id, property_key_id)` and
/// `entity_trait_current(entity_id, trait_id)` — so an Entity carrying thousands
/// of Properties still costs one bounded index range, not a sort of everything
/// it owns. Each Property carries a version count that stops at 101, so the page
/// can honestly render "100+" without an unbounded aggregate.
///
pub async fn get_entity(pool: &PgPool, entity_id: Uuid) -> Result<EntityDetail, StudioError> {
    let row = sqlx::query_as::<_, EntityIdentityRow>(
        r#"
        SELECT entity.id,
               entity.name,
               entity.description,
               entity.introduced_by_user_id,
               entity.introduced_at,
               character.entity_id IS NOT NULL AS is_character,
               character.owner_user_id,
               place.entity_id IS NOT NULL AS is_place,
               place.is_entry AS is_entry_place,
               entity_location.place_entity_id AS current_place_entity_id,
               location.name AS current_place_name
        FROM entity
        LEFT JOIN character ON character.entity_id = entity.id
        LEFT JOIN place ON place.entity_id = entity.id
        LEFT JOIN entity_location ON entity_location.entity_id = entity.id
        LEFT JOIN entity AS location ON location.id = entity_location.place_entity_id
        WHERE entity.id = $1
        "#,
    )
    .bind(entity_id)
    .fetch_optional(pool)
    .await?
    .ok_or(StudioError::NotFound)?;

    let mut property = sqlx::query_as::<_, EntityProperty>(
        r#"
        SELECT entity_property.property_key_id,
               property_key.key,
               history.value_type,
               history.text_value,
               history.integer_value,
               entity_property.current_activity_id,
               (
                   SELECT count(*)
                   FROM (
                       SELECT 1
                       FROM entity_property_history AS version
                       WHERE version.entity_id = entity_property.entity_id
                         AND version.property_key_id = entity_property.property_key_id
                       LIMIT $2
                   ) AS bounded
               ) AS counted_version
        FROM entity_property
        JOIN property_key ON property_key.id = entity_property.property_key_id
        JOIN entity_property_history AS history
          ON history.entity_id = entity_property.entity_id
         AND history.property_key_id = entity_property.property_key_id
         AND history.activity_id = entity_property.current_activity_id
        WHERE entity_property.entity_id = $1
        ORDER BY entity_property.property_key_id
        LIMIT $3
        "#,
    )
    .bind(entity_id)
    .bind(page::PREVIEW_COUNT_LIMIT)
    .bind(i64::try_from(STATE_PREVIEW_LIMIT + 1).expect("the state preview bound fits i64"))
    .fetch_all(pool)
    .await?;
    let property_truncated = page::truncate(&mut property, STATE_PREVIEW_LIMIT);
    let mut property_version_count_truncated = false;
    for property in &mut property {
        let (count, truncated) = page::preview_count(property.version_count);
        property.version_count = count;
        property_version_count_truncated |= truncated;
    }

    let mut r#trait = sqlx::query_as::<_, EntityTraitCurrent>(
        r#"
        SELECT current.trait_id AS id,
               version.statement,
               current.current_activity_id
        FROM entity_trait_current AS current
        JOIN entity_trait_version AS version
          ON version.trait_id = current.trait_id
         AND version.entity_id = current.entity_id
         AND version.activity_id = current.current_activity_id
        WHERE current.entity_id = $1
        ORDER BY current.trait_id
        LIMIT $2
        "#,
    )
    .bind(entity_id)
    .bind(i64::try_from(STATE_PREVIEW_LIMIT + 1).expect("the state preview bound fits i64"))
    .fetch_all(pool)
    .await?;
    let trait_truncated = page::truncate(&mut r#trait, STATE_PREVIEW_LIMIT);

    Ok(EntityDetail {
        entity: EntityIdentity {
            id: row.id,
            name: row.name,
            description: row.description,
            introduced_by_user_id: row.introduced_by_user_id,
            introduced_at: row.introduced_at,
            is_character: row.is_character,
            owner_user_id: row.owner_user_id,
            is_place: row.is_place,
            is_entry_place: row.is_entry_place,
            current_place: match (row.current_place_entity_id, row.current_place_name) {
                (Some(id), Some(name)) => Some(EntityRef { id, name }),
                _ => None,
            },
        },
        property,
        property_truncated,
        property_version_count_truncated,
        r#trait,
        trait_truncated,
    })
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct PropertyVersion {
    pub activity_id: Uuid,
    pub previous_activity_id: Option<Uuid>,
    pub value_type: String,
    pub text_value: Option<String>,
    pub integer_value: Option<i64>,
    pub operation: String,
    pub occurred_at: DateTime<Utc>,
    pub is_current: bool,
}

#[derive(Debug, Serialize)]
pub struct PropertyHistory {
    /// The exact order this page is in, so a reader never assumes it is time order.
    pub order: &'static str,
    pub entity_id: Uuid,
    pub property_key_id: i64,
    pub key: String,
    pub page: Page<PropertyVersion, Uuid>,
}

/// The order label every Property history and Entity participation page carries.
pub const ACTIVITY_ID_ORDER: &str = "activity id order";

/// Every stored version of one Entity Property key.
///
/// `entity_property_history` has primary key
/// `(entity_id, property_key_id, activity_id)`, so this walks exactly one key's
/// lineage as an index range and keysets on `activity_id`. That is primary-key
/// order over an Activity **id**, not chronological order: an Activity id is a
/// random UUID. Each version therefore carries its own stored `occurred_at` and
/// its `previous_activity_id`, which is the exact predecessor link, and the page
/// labels its order rather than claiming to be newest-first.
pub async fn list_property_history(
    pool: &PgPool,
    entity_id: Uuid,
    property_key_id: i64,
    before_activity_id: Option<Uuid>,
    bound: Bound,
) -> Result<PropertyHistory, StudioError> {
    let key = sqlx::query_scalar::<_, String>("SELECT key FROM property_key WHERE id = $1")
        .bind(property_key_id)
        .fetch_optional(pool)
        .await?
        .ok_or(StudioError::NotFound)?;
    let item = sqlx::query_as::<_, PropertyVersion>(
        r#"
        SELECT version.activity_id,
               version.previous_activity_id,
               version.value_type,
               version.text_value,
               version.integer_value,
               activity.operation,
               activity.occurred_at,
               COALESCE(version.activity_id = current.current_activity_id, false) AS is_current
        FROM entity_property_history AS version
        JOIN activity ON activity.id = version.activity_id
        LEFT JOIN entity_property AS current
          ON current.entity_id = version.entity_id
         AND current.property_key_id = version.property_key_id
        WHERE version.entity_id = $1
          AND version.property_key_id = $2
          AND ($3::uuid IS NULL OR version.activity_id < $3::uuid)
        ORDER BY version.activity_id DESC
        LIMIT $4
        "#,
    )
    .bind(entity_id)
    .bind(property_key_id)
    .bind(before_activity_id)
    .bind(bound.fetch())
    .fetch_all(pool)
    .await?;
    Ok(PropertyHistory {
        order: ACTIVITY_ID_ORDER,
        entity_id,
        property_key_id,
        key,
        page: Page::build(item, bound, |last| last.activity_id),
    })
}

/// The keyset of one Entity's participation page: an Activity may involve the
/// same Entity under more than one role, so the cursor names both.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ParticipationCursor {
    pub activity_id: Uuid,
    pub role: String,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct ParticipationItem {
    pub activity_id: Uuid,
    pub role: String,
    pub operation: String,
    pub occurred_at: DateTime<Utc>,
    pub actor_character_entity_id: Option<Uuid>,
    pub context_place_entity_id: Option<Uuid>,
}

#[derive(Debug, Serialize)]
pub struct Participation {
    /// The exact order this page is in, so a reader never assumes it is time order.
    pub order: &'static str,
    pub entity_id: Uuid,
    pub page: Page<ParticipationItem, ParticipationCursor>,
}

/// Every Activity that explicitly involves one Entity, with the role it held.
///
/// The index for this join is
/// `activity_entity_entity_id_activity_id_index` — `(entity_id, activity_id)` —
/// which fixes the Entity and hands back `activity_id` order for free, so the
/// page is ordered by `activity_id` descending and keysets on
/// `(activity_id, role)`. The `role` tiebreak is needed because one Activity may
/// involve the same Entity twice, and it costs only an incremental sort inside
/// each `activity_id` group. Index order over a random UUID is not time order, so
/// each row shows the stored `occurred_at` and the page labels its order instead
/// of claiming to be newest-first. Sorting one hot Entity's complete
/// participation by time would need an index that current game behavior does not
/// earn, so Studio does not take one.
pub async fn list_participation(
    pool: &PgPool,
    entity_id: Uuid,
    before: Option<ParticipationCursor>,
    bound: Bound,
) -> Result<Participation, StudioError> {
    let (before_activity_id, before_role) = match before {
        Some(cursor) => (Some(cursor.activity_id), Some(cursor.role)),
        None => (None, None),
    };
    let item = sqlx::query_as::<_, ParticipationItem>(
        r#"
        SELECT activity_entity.activity_id,
               activity_entity.role,
               activity.operation,
               activity.occurred_at,
               activity.actor_character_entity_id,
               activity.context_place_entity_id
        FROM activity_entity
        JOIN activity ON activity.id = activity_entity.activity_id
        WHERE activity_entity.entity_id = $1
          AND (
                $2::uuid IS NULL
                OR (activity_entity.activity_id, activity_entity.role) < ($2::uuid, $3::text)
              )
        ORDER BY activity_entity.activity_id DESC, activity_entity.role DESC
        LIMIT $4
        "#,
    )
    .bind(entity_id)
    .bind(before_activity_id)
    .bind(before_role)
    .bind(bound.fetch())
    .fetch_all(pool)
    .await?;
    Ok(Participation {
        order: ACTIVITY_ID_ORDER,
        entity_id,
        page: Page::build(item, bound, |last| ParticipationCursor {
            activity_id: last.activity_id,
            role: last.role.clone(),
        }),
    })
}
