//! Activity: the immutable chronicle, read per Place, per Character and — for
//! local development only — across the whole table.

use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::PgPool;
use uuid::Uuid;

use super::entity::EntityRef;
use super::page::{self, Bound, Page};
use crate::StudioError;

/// How much prose one chronicle row carries before the page must open the
/// Activity itself.
const PROSE_PREVIEW_LENGTH: i32 = 240;

/// The label the World chronicle carries so no reader mistakes it for a game read.
pub const WORLD_CHRONICLE_SCOPE: &str = "local development sort";

/// The keyset every chronicle shares: newest first by stored time, then by id.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct ChronicleCursor {
    pub occurred_at: DateTime<Utc>,
    pub id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct ChronicleItem {
    pub id: Uuid,
    pub operation: String,
    pub action_consequence: Option<String>,
    pub actor_character: Option<EntityRef>,
    pub context_place: Option<EntityRef>,
    pub occurred_at: DateTime<Utc>,
    pub prose: Option<String>,
    pub prose_truncated: bool,
    pub involved_entity_count: i64,
    pub involved_entity_count_truncated: bool,
}

#[derive(sqlx::FromRow)]
struct ChronicleRow {
    id: Uuid,
    operation: String,
    action_consequence: Option<String>,
    actor_character_entity_id: Option<Uuid>,
    actor_character_name: Option<String>,
    context_place_entity_id: Option<Uuid>,
    context_place_name: Option<String>,
    occurred_at: DateTime<Utc>,
    prose: Option<String>,
    prose_truncated: bool,
    counted_involved_entity: i64,
}

impl From<ChronicleRow> for ChronicleItem {
    fn from(row: ChronicleRow) -> Self {
        let (involved_entity_count, involved_entity_count_truncated) =
            page::preview_count(row.counted_involved_entity);
        Self {
            id: row.id,
            operation: row.operation,
            action_consequence: row.action_consequence,
            actor_character: reference(row.actor_character_entity_id, row.actor_character_name),
            context_place: reference(row.context_place_entity_id, row.context_place_name),
            occurred_at: row.occurred_at,
            prose: row.prose,
            prose_truncated: row.prose_truncated,
            involved_entity_count,
            involved_entity_count_truncated,
        }
    }
}

fn reference(id: Option<Uuid>, name: Option<String>) -> Option<EntityRef> {
    match (id, name) {
        (Some(id), Some(name)) => Some(EntityRef { id, name }),
        _ => None,
    }
}

/// The one chronicle projection. `$1` is the optional scope id, `$2`/`$3` the
/// keyset, `$4` the row bound, `$5` the prose preview length and `$6` the bound
/// on the involved-Entity count.
///
/// The scope predicate is written so exactly one of the two partial indexes —
/// `activity_place_occurred_at_id_index` or `activity_actor_occurred_at_id_index`
/// — can serve the ordering, and so the unscoped World read stays a plain
/// newest-first sort of `activity`.
fn chronicle_sql(scope: &'static str) -> String {
    format!(
        r#"
        SELECT activity.id,
               activity.operation,
               activity.action_consequence,
               activity.actor_character_entity_id,
               actor.name AS actor_character_name,
               activity.context_place_entity_id,
               context.name AS context_place_name,
               activity.occurred_at,
               left(activity.prose, $5::integer) AS prose,
               COALESCE(char_length(activity.prose) > $5::integer, false) AS prose_truncated,
               (
                   SELECT count(*)
                   FROM (
                       SELECT 1
                       FROM activity_entity
                       WHERE activity_entity.activity_id = activity.id
                       LIMIT $6
                   ) AS bounded
               ) AS counted_involved_entity
        FROM activity
        LEFT JOIN entity AS actor ON actor.id = activity.actor_character_entity_id
        LEFT JOIN entity AS context ON context.id = activity.context_place_entity_id
        WHERE {scope}
          AND (
                $2::timestamptz IS NULL
                OR (activity.occurred_at, activity.id) < ($2::timestamptz, $3::uuid)
              )
        ORDER BY activity.occurred_at DESC, activity.id DESC
        LIMIT $4
        "#
    )
}

async fn read_chronicle(
    pool: &PgPool,
    sql: String,
    scope_id: Option<Uuid>,
    before: Option<ChronicleCursor>,
    bound: Bound,
) -> Result<Page<ChronicleItem, ChronicleCursor>, StudioError> {
    let row = sqlx::query_as::<_, ChronicleRow>(&sql)
        .bind(scope_id)
        .bind(before.map(|cursor| cursor.occurred_at))
        .bind(before.map(|cursor| cursor.id))
        .bind(bound.fetch())
        .bind(PROSE_PREVIEW_LENGTH)
        .bind(page::PREVIEW_COUNT_LIMIT)
        .fetch_all(pool)
        .await?;
    let item = row.into_iter().map(ChronicleItem::from).collect::<Vec<_>>();
    Ok(Page::build(item, bound, |last| ChronicleCursor {
        occurred_at: last.occurred_at,
        id: last.id,
    }))
}

/// Everything that happened at one Place, newest first.
///
/// Served by `activity_place_occurred_at_id_index` —
/// `(context_place_entity_id, occurred_at DESC, id DESC)` — so a Place with a
/// million Activities pages at the same cost as a quiet one.
pub async fn list_place_chronicle(
    pool: &PgPool,
    place_entity_id: Uuid,
    before: Option<ChronicleCursor>,
    bound: Bound,
) -> Result<Page<ChronicleItem, ChronicleCursor>, StudioError> {
    read_chronicle(
        pool,
        chronicle_sql("activity.context_place_entity_id = $1::uuid"),
        Some(place_entity_id),
        before,
        bound,
    )
    .await
}

/// Everything one Character did, newest first.
///
/// Served by `activity_actor_occurred_at_id_index` —
/// `(actor_character_entity_id, occurred_at DESC, id DESC)`.
pub async fn list_character_chronicle(
    pool: &PgPool,
    character_entity_id: Uuid,
    before: Option<ChronicleCursor>,
    bound: Bound,
) -> Result<Page<ChronicleItem, ChronicleCursor>, StudioError> {
    read_chronicle(
        pool,
        chronicle_sql("activity.actor_character_entity_id = $1::uuid"),
        Some(character_entity_id),
        before,
        bound,
    )
    .await
}

/// The whole `activity` table newest first, carrying its own warning label.
#[derive(Debug, Serialize)]
pub struct WorldChronicle {
    pub scope: &'static str,
    pub page: Page<ChronicleItem, ChronicleCursor>,
}

/// Every Activity in the connected local World, newest first.
///
/// **This is a local-development sort of the `activity` table and is never a
/// game read.** No index orders `activity` by `(occurred_at, id)` across the
/// whole World, and current game behavior does not earn one: every player-facing
/// history is scoped to a Place or a Character. The keyset and the hard `LIMIT`
/// keep one page bounded, but the sort itself is only affordable because a local
/// development World is small. The returned value carries
/// `scope = "local development sort"` so the page always says so, and no game
/// capability may be built on this read.
pub async fn list_world_chronicle(
    pool: &PgPool,
    before: Option<ChronicleCursor>,
    bound: Bound,
) -> Result<WorldChronicle, StudioError> {
    let page = read_chronicle(pool, chronicle_sql("$1::uuid IS NULL"), None, before, bound).await?;
    Ok(WorldChronicle {
        scope: WORLD_CHRONICLE_SCOPE,
        page,
    })
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct InvolvedEntity {
    pub entity_id: Uuid,
    pub name: String,
    pub role: String,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct ActivityPropertyChange {
    pub entity_id: Uuid,
    pub entity_name: String,
    pub property_key_id: i64,
    pub key: String,
    pub value_type: String,
    pub text_value: Option<String>,
    pub integer_value: Option<i64>,
    pub previous_activity_id: Option<Uuid>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct ActivityTraitChange {
    pub trait_id: Uuid,
    pub entity_id: Uuid,
    pub entity_name: String,
    pub statement: String,
    pub previous_activity_id: Option<Uuid>,
}

#[derive(Debug, Serialize)]
pub struct ActivityDetail {
    pub id: Uuid,
    pub operation: String,
    pub action_consequence: Option<String>,
    pub requested_by_user_id: Uuid,
    pub request_id: Option<Uuid>,
    pub actor_character: Option<EntityRef>,
    pub context_place: Option<EntityRef>,
    pub occurred_at: DateTime<Utc>,
    pub prose: Option<String>,
    pub consumed_investigation_attempt_id: Option<Uuid>,
    pub involved_entity: Vec<InvolvedEntity>,
    pub involved_entity_truncated: bool,
    pub property_change: Vec<ActivityPropertyChange>,
    pub property_change_truncated: bool,
    pub trait_change: Vec<ActivityTraitChange>,
    pub trait_change_truncated: bool,
}

#[derive(sqlx::FromRow)]
struct ActivityRow {
    id: Uuid,
    operation: String,
    action_consequence: Option<String>,
    requested_by_user_id: Uuid,
    request_id: Option<Uuid>,
    actor_character_entity_id: Option<Uuid>,
    actor_character_name: Option<String>,
    context_place_entity_id: Option<Uuid>,
    context_place_name: Option<String>,
    occurred_at: DateTime<Utc>,
    prose: Option<String>,
    consumed_investigation_attempt_id: Option<Uuid>,
}

/// One Activity with everything it explicitly recorded.
///
/// Identity is a primary-key lookup. The involved-Entity list follows the
/// `activity_entity` primary key; the Property and Trait changes follow
/// `entity_property_history_activity_index` and
/// `entity_trait_version_activity_entity_trait_index`; the consumed
/// investigation attempt follows the unique
/// `investigation_attempt_consumed_by_activity_id_key`. Every list stops at 100
/// rows and says so, because one accepted mutation may carry up to 100 changes.
pub async fn get_activity(pool: &PgPool, activity_id: Uuid) -> Result<ActivityDetail, StudioError> {
    let bound = Bound::new(Some(page::MAX_LIMIT))?;
    let row = sqlx::query_as::<_, ActivityRow>(
        r#"
        SELECT activity.id,
               activity.operation,
               activity.action_consequence,
               activity.requested_by_user_id,
               activity.request_id,
               activity.actor_character_entity_id,
               actor.name AS actor_character_name,
               activity.context_place_entity_id,
               context.name AS context_place_name,
               activity.occurred_at,
               activity.prose,
               attempt.id AS consumed_investigation_attempt_id
        FROM activity
        LEFT JOIN entity AS actor ON actor.id = activity.actor_character_entity_id
        LEFT JOIN entity AS context ON context.id = activity.context_place_entity_id
        LEFT JOIN investigation_attempt AS attempt
          ON attempt.consumed_by_activity_id = activity.id
        WHERE activity.id = $1
        "#,
    )
    .bind(activity_id)
    .fetch_optional(pool)
    .await?
    .ok_or(StudioError::NotFound)?;

    let mut involved_entity = sqlx::query_as::<_, InvolvedEntity>(
        r#"
        SELECT activity_entity.entity_id,
               entity.name,
               activity_entity.role
        FROM activity_entity
        JOIN entity ON entity.id = activity_entity.entity_id
        WHERE activity_entity.activity_id = $1
        ORDER BY activity_entity.entity_id, activity_entity.role
        LIMIT $2
        "#,
    )
    .bind(activity_id)
    .bind(bound.fetch())
    .fetch_all(pool)
    .await?;
    let involved_entity_truncated = page::truncate(&mut involved_entity, bound.limit());

    let mut property_change = sqlx::query_as::<_, ActivityPropertyChange>(
        r#"
        SELECT version.entity_id,
               entity.name AS entity_name,
               version.property_key_id,
               property_key.key,
               version.value_type,
               version.text_value,
               version.integer_value,
               version.previous_activity_id
        FROM entity_property_history AS version
        JOIN entity ON entity.id = version.entity_id
        JOIN property_key ON property_key.id = version.property_key_id
        WHERE version.activity_id = $1
        ORDER BY version.entity_id, version.property_key_id
        LIMIT $2
        "#,
    )
    .bind(activity_id)
    .bind(bound.fetch())
    .fetch_all(pool)
    .await?;
    let property_change_truncated = page::truncate(&mut property_change, bound.limit());

    let mut trait_change = sqlx::query_as::<_, ActivityTraitChange>(
        r#"
        SELECT version.trait_id,
               version.entity_id,
               entity.name AS entity_name,
               version.statement,
               version.previous_activity_id
        FROM entity_trait_version AS version
        JOIN entity ON entity.id = version.entity_id
        WHERE version.activity_id = $1
        ORDER BY version.entity_id, version.trait_id
        LIMIT $2
        "#,
    )
    .bind(activity_id)
    .bind(bound.fetch())
    .fetch_all(pool)
    .await?;
    let trait_change_truncated = page::truncate(&mut trait_change, bound.limit());

    Ok(ActivityDetail {
        id: row.id,
        operation: row.operation,
        action_consequence: row.action_consequence,
        requested_by_user_id: row.requested_by_user_id,
        request_id: row.request_id,
        actor_character: reference(row.actor_character_entity_id, row.actor_character_name),
        context_place: reference(row.context_place_entity_id, row.context_place_name),
        occurred_at: row.occurred_at,
        prose: row.prose,
        consumed_investigation_attempt_id: row.consumed_investigation_attempt_id,
        involved_entity,
        involved_entity_truncated,
        property_change,
        property_change_truncated,
        trait_change,
        trait_change_truncated,
    })
}
