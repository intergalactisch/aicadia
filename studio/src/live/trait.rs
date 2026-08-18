//! Trait: one Entity-owned statement and the append-only lineage of every
//! version it has held.

use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::PgPool;
use uuid::Uuid;

use super::entity::EntityRef;
use super::page::{self, Bound};
use crate::StudioError;

/// The order label a Trait lineage carries.
pub const TRAIT_VERSION_ORDER: &str = "trait version activity id order";

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct TraitVersion {
    pub activity_id: Uuid,
    pub previous_activity_id: Option<Uuid>,
    pub statement: String,
    pub operation: String,
    pub action_consequence: Option<String>,
    pub occurred_at: DateTime<Utc>,
    pub is_root: bool,
    pub is_current: bool,
}

#[derive(Debug, Serialize)]
pub struct TraitDetail {
    pub id: Uuid,
    pub entity: EntityRef,
    pub current_statement: String,
    pub current_activity_id: Uuid,
    /// The exact order the lineage is in, so a reader never assumes it is time order.
    pub order: &'static str,
    pub version: Vec<TraitVersion>,
    pub version_truncated: bool,
}

#[derive(sqlx::FromRow)]
struct TraitRow {
    id: Uuid,
    entity_id: Uuid,
    entity_name: String,
    current_statement: String,
    current_activity_id: Uuid,
}

/// One Trait with its owning Entity, current statement and full version lineage.
///
/// Identity, the owning Entity and the current version are primary-key and
/// unique-lineage lookups. The lineage itself is an index range over the
/// `entity_trait_version` primary key `(trait_id, activity_id)`, so reading one
/// Trait's history never touches another Trait's.
///
/// That primary key orders by an Activity **id**, which is a random UUID and not
/// time order, so the returned lineage carries [`TRAIT_VERSION_ORDER`] rather
/// than claiming to be chronological. The exact chain is in the data: exactly one
/// version has `previous_activity_id IS NULL` and is flagged `is_root` — the
/// partial unique `entity_trait_version_one_root_index` guarantees it — and every
/// later version names its single predecessor. Each version also carries its
/// Activity's stored `occurred_at`.
///
/// The lineage stops at 100 versions and reports `version_truncated`.
pub async fn get_trait(pool: &PgPool, trait_id: Uuid) -> Result<TraitDetail, StudioError> {
    let bound = Bound::new(Some(page::MAX_LIMIT))?;
    let row = sqlx::query_as::<_, TraitRow>(
        r#"
        SELECT entity_trait.id,
               entity_trait.entity_id,
               entity.name AS entity_name,
               current_version.statement AS current_statement,
               current.current_activity_id
        FROM entity_trait
        JOIN entity ON entity.id = entity_trait.entity_id
        JOIN entity_trait_current AS current ON current.trait_id = entity_trait.id
        JOIN entity_trait_version AS current_version
          ON current_version.trait_id = current.trait_id
         AND current_version.entity_id = current.entity_id
         AND current_version.activity_id = current.current_activity_id
        WHERE entity_trait.id = $1
        "#,
    )
    .bind(trait_id)
    .fetch_optional(pool)
    .await?
    .ok_or(StudioError::NotFound)?;

    let mut version = sqlx::query_as::<_, TraitVersion>(
        r#"
        SELECT version.activity_id,
               version.previous_activity_id,
               version.statement,
               activity.operation,
               activity.action_consequence,
               activity.occurred_at,
               version.previous_activity_id IS NULL AS is_root,
               version.activity_id = $2 AS is_current
        FROM entity_trait_version AS version
        JOIN activity ON activity.id = version.activity_id
        WHERE version.trait_id = $1
        ORDER BY version.activity_id
        LIMIT $3
        "#,
    )
    .bind(trait_id)
    .bind(row.current_activity_id)
    .bind(bound.fetch())
    .fetch_all(pool)
    .await?;
    let version_truncated = page::truncate(&mut version, bound.limit());

    Ok(TraitDetail {
        id: row.id,
        entity: EntityRef {
            id: row.entity_id,
            name: row.entity_name,
        },
        current_statement: row.current_statement,
        current_activity_id: row.current_activity_id,
        order: TRAIT_VERSION_ORDER,
        version,
        version_truncated,
    })
}
