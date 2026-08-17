//! Investigation attempt: the durable internal record of one admitted
//! investigation, its outcome and its one-time consumption or voiding.

use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::PgPool;
use uuid::Uuid;

use super::entity::EntityRef;
use super::page::{self, Bound, Page};
use crate::studio::StudioError;

/// The order label the global attempt list carries.
pub const ATTEMPT_ID_ORDER: &str = "attempt id order";

/// The keyset of one User's attempt page: newest first by stored creation time.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct AttemptCursor {
    pub created_at: DateTime<Utc>,
    pub id: Uuid,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct AttemptItem {
    pub id: Uuid,
    pub requested_by_user_id: Uuid,
    pub outcome: String,
    pub character_entity_id: Uuid,
    pub character_name: String,
    pub place_entity_id: Uuid,
    pub place_name: String,
    pub created_at: DateTime<Utc>,
    pub consumed_by_activity_id: Option<Uuid>,
    pub voided_by_attempt_id: Option<Uuid>,
}

#[derive(Debug, Serialize)]
pub struct AttemptList {
    /// The exact order this page is in, so a reader never assumes it is time order.
    pub order: &'static str,
    pub page: Page<AttemptItem, Uuid>,
}

/// Every investigation attempt in the connected World.
///
/// `investigation_attempt` has no global time index — its only time index is
/// `investigation_attempt_user_created_at_index`, which leads with
/// `requested_by_user_id` and serves the rolling-hour admission window. This
/// global list is therefore keyset by the `id` primary key, which is a random
/// UUID and not time order; the page labels its order and each row shows its
/// stored `created_at`. For newest-first attempts, read one User's attempts with
/// [`list_user_attempt`], which is indexed.
pub async fn list_investigation(
    pool: &PgPool,
    after: Option<Uuid>,
    bound: Bound,
) -> Result<AttemptList, StudioError> {
    let item = sqlx::query_as::<_, AttemptItem>(
        r#"
        SELECT attempt.id,
               attempt.requested_by_user_id,
               attempt.outcome,
               attempt.character_entity_id,
               character_entity.name AS character_name,
               attempt.place_entity_id,
               place_entity.name AS place_name,
               attempt.created_at,
               attempt.consumed_by_activity_id,
               attempt.voided_by_attempt_id
        FROM investigation_attempt AS attempt
        JOIN entity AS character_entity ON character_entity.id = attempt.character_entity_id
        JOIN entity AS place_entity ON place_entity.id = attempt.place_entity_id
        WHERE ($1::uuid IS NULL OR attempt.id > $1::uuid)
        ORDER BY attempt.id
        LIMIT $2
        "#,
    )
    .bind(after)
    .bind(bound.fetch())
    .fetch_all(pool)
    .await?;
    Ok(AttemptList {
        order: ATTEMPT_ID_ORDER,
        page: Page::build(item, bound, |last| last.id),
    })
}

/// One User's investigation attempts, newest first.
///
/// Served by `investigation_attempt_user_created_at_index` —
/// `(requested_by_user_id, created_at DESC)` — the same index the rolling-hour
/// admission window uses. The keyset adds `id` as the tiebreak so two attempts
/// stamped in the same microsecond still page exactly.
pub async fn list_user_attempt(
    pool: &PgPool,
    user_id: Uuid,
    before: Option<AttemptCursor>,
    bound: Bound,
) -> Result<Page<AttemptItem, AttemptCursor>, StudioError> {
    let item = sqlx::query_as::<_, AttemptItem>(
        r#"
        SELECT attempt.id,
               attempt.requested_by_user_id,
               attempt.outcome,
               attempt.character_entity_id,
               character_entity.name AS character_name,
               attempt.place_entity_id,
               place_entity.name AS place_name,
               attempt.created_at,
               attempt.consumed_by_activity_id,
               attempt.voided_by_attempt_id
        FROM investigation_attempt AS attempt
        JOIN entity AS character_entity ON character_entity.id = attempt.character_entity_id
        JOIN entity AS place_entity ON place_entity.id = attempt.place_entity_id
        WHERE attempt.requested_by_user_id = $1
          AND (
                $2::timestamptz IS NULL
                OR (attempt.created_at, attempt.id) < ($2::timestamptz, $3::uuid)
              )
        ORDER BY attempt.created_at DESC, attempt.id DESC
        LIMIT $4
        "#,
    )
    .bind(user_id)
    .bind(before.map(|cursor| cursor.created_at))
    .bind(before.map(|cursor| cursor.id))
    .bind(bound.fetch())
    .fetch_all(pool)
    .await?;
    Ok(Page::build(item, bound, |last| AttemptCursor {
        created_at: last.created_at,
        id: last.id,
    }))
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct VoidedAttempt {
    pub id: Uuid,
    pub outcome: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct InvestigationDetail {
    pub id: Uuid,
    pub requested_by_user_id: Uuid,
    pub request_id: Uuid,
    pub outcome: String,
    pub character: EntityRef,
    pub place: EntityRef,
    pub created_at: DateTime<Utc>,
    pub consumed_by_activity_id: Option<Uuid>,
    pub consumed_by_activity_occurred_at: Option<DateTime<Utc>>,
    pub consumed_by_activity_operation: Option<String>,
    pub voided_by_attempt_id: Option<Uuid>,
    /// Attempts this attempt voided. Not indexed; see the read's doc comment.
    pub voided_attempt: Vec<VoidedAttempt>,
    pub voided_attempt_truncated: bool,
    pub voided_attempt_scope: &'static str,
}

#[derive(sqlx::FromRow)]
struct InvestigationRow {
    id: Uuid,
    requested_by_user_id: Uuid,
    request_id: Uuid,
    outcome: String,
    character_entity_id: Uuid,
    character_name: String,
    place_entity_id: Uuid,
    place_name: String,
    created_at: DateTime<Utc>,
    consumed_by_activity_id: Option<Uuid>,
    consumed_by_activity_occurred_at: Option<DateTime<Utc>>,
    consumed_by_activity_operation: Option<String>,
    voided_by_attempt_id: Option<Uuid>,
}

/// One investigation attempt with its complete stored lifecycle.
///
/// Identity, the Character and Place Entities and the consuming Activity are
/// primary-key lookups. The reverse direction — which attempts *this* attempt
/// voided — has no index: `voided_by_attempt_id` is a plain nullable foreign key
/// that current game behavior only ever writes and reads forward, one attempt at
/// a time, under the User lock. Studio therefore reads it as a bounded scan
/// stopping at 100 rows, labels it `local development scan` and reports
/// `voided_attempt_truncated`. In practice deterministic FIFO voiding names one
/// attempt at a time, so the list is short; the label exists so an operator never
/// mistakes this for a read a game capability could take.
pub async fn get_investigation(
    pool: &PgPool,
    attempt_id: Uuid,
) -> Result<InvestigationDetail, StudioError> {
    let bound = Bound::new(Some(page::MAX_LIMIT))?;
    let row = sqlx::query_as::<_, InvestigationRow>(
        r#"
        SELECT attempt.id,
               attempt.requested_by_user_id,
               attempt.request_id,
               attempt.outcome,
               attempt.character_entity_id,
               character_entity.name AS character_name,
               attempt.place_entity_id,
               place_entity.name AS place_name,
               attempt.created_at,
               attempt.consumed_by_activity_id,
               consumed.occurred_at AS consumed_by_activity_occurred_at,
               consumed.operation AS consumed_by_activity_operation,
               attempt.voided_by_attempt_id
        FROM investigation_attempt AS attempt
        JOIN entity AS character_entity ON character_entity.id = attempt.character_entity_id
        JOIN entity AS place_entity ON place_entity.id = attempt.place_entity_id
        LEFT JOIN activity AS consumed ON consumed.id = attempt.consumed_by_activity_id
        WHERE attempt.id = $1
        "#,
    )
    .bind(attempt_id)
    .fetch_optional(pool)
    .await?
    .ok_or(StudioError::NotFound)?;

    let mut voided_attempt = sqlx::query_as::<_, VoidedAttempt>(
        r#"
        SELECT id, outcome, created_at
        FROM investigation_attempt
        WHERE voided_by_attempt_id = $1
        ORDER BY created_at, id
        LIMIT $2
        "#,
    )
    .bind(attempt_id)
    .bind(bound.fetch())
    .fetch_all(pool)
    .await?;
    let voided_attempt_truncated = page::truncate(&mut voided_attempt, bound.limit());

    Ok(InvestigationDetail {
        id: row.id,
        requested_by_user_id: row.requested_by_user_id,
        request_id: row.request_id,
        outcome: row.outcome,
        character: EntityRef {
            id: row.character_entity_id,
            name: row.character_name,
        },
        place: EntityRef {
            id: row.place_entity_id,
            name: row.place_name,
        },
        created_at: row.created_at,
        consumed_by_activity_id: row.consumed_by_activity_id,
        consumed_by_activity_occurred_at: row.consumed_by_activity_occurred_at,
        consumed_by_activity_operation: row.consumed_by_activity_operation,
        voided_by_attempt_id: row.voided_by_attempt_id,
        voided_attempt,
        voided_attempt_truncated,
        voided_attempt_scope: page::LOCAL_DEVELOPMENT_SCAN,
    })
}
