use super::super::*;
use super::chance::PLACE_ACTIVITY_WINDOW;
use super::model::*;

pub(super) const MAX_ATTEMPTS_PER_HOUR: i64 = 12;
pub(super) const MAX_LIVE_POSITIVES: i64 = 3;

pub(super) const ADMISSION_SQL: &str = r#"
    SELECT statement_timestamp() AS database_now,
           (
               SELECT count(*)
               FROM investigation_attempt
               WHERE requested_by_user_id = $1
                 AND created_at >= statement_timestamp() - interval '1 hour'
                 AND created_at <= statement_timestamp()
           ) AS admitted_count
"#;

pub(super) const PLACE_WINDOW_DISCOVERY_COUNT_SQL: &str = r#"
    SELECT count(*)
    FROM (
        SELECT operation
        FROM activity
        WHERE context_place_entity_id = $1
        ORDER BY occurred_at DESC, id DESC
        LIMIT $2
    ) AS recent
    WHERE operation = 'submit_discovery'
"#;

pub(super) const VOID_OLDEST_PRIOR_POSITIVE_SQL: &str = r#"
    WITH candidate AS (
        SELECT id
        FROM investigation_attempt
        WHERE requested_by_user_id = $1
          AND outcome = 'positive'
          AND consumed_by_activity_id IS NULL
          AND voided_by_attempt_id IS NULL
          AND id <> $2
          AND (
              SELECT count(*)
              FROM investigation_attempt
              WHERE requested_by_user_id = $1
                AND outcome = 'positive'
                AND consumed_by_activity_id IS NULL
                AND voided_by_attempt_id IS NULL
          ) > $3
        ORDER BY created_at ASC, id ASC
        LIMIT 1
    )
    UPDATE investigation_attempt
    SET voided_by_attempt_id = $2
    FROM candidate
    WHERE investigation_attempt.id = candidate.id
"#;

#[derive(FromRow)]
struct AttemptRow {
    id: InvestigationAttemptId,
    outcome: String,
}

#[derive(FromRow)]
struct AdmissionRow {
    database_now: DateTime<Utc>,
    admitted_count: i64,
}

pub(super) async fn find_result(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: UserId,
    request_id: Uuid,
    operation: &'static str,
) -> Result<Option<InvestigationResult>, WorldError> {
    sqlx::query_as::<_, AttemptRow>(
        r#"
        SELECT id, outcome
        FROM investigation_attempt
        WHERE requested_by_user_id = $1 AND request_id = $2
        "#,
    )
    .bind(user_id.0)
    .bind(request_id)
    .fetch_optional(&mut **transaction)
    .await
    .map_err(|error| storage_error(operation, error))?
    .map(|row| {
        Ok(InvestigationResult {
            attempt_id: row.id,
            outcome: InvestigationOutcome::parse(&row.outcome)?,
            limit: InvestigationLimit::CURRENT,
        })
    })
    .transpose()
}

pub(super) async fn admission_time_for_user(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: UserId,
    operation: &'static str,
) -> Result<DateTime<Utc>, WorldError> {
    let row = sqlx::query_as::<_, AdmissionRow>(ADMISSION_SQL)
        .bind(user_id.0)
        .fetch_one(&mut **transaction)
        .await
        .map_err(|error| storage_error(operation, error))?;
    if row.admitted_count >= MAX_ATTEMPTS_PER_HOUR {
        return Err(WorldError::InvestigationNotAdmitted);
    }
    Ok(row.database_now)
}

pub(super) async fn recent_discovery_count(
    transaction: &mut Transaction<'_, Postgres>,
    place_entity_id: EntityId,
    operation: &'static str,
) -> Result<u32, WorldError> {
    let count: i64 = sqlx::query_scalar(PLACE_WINDOW_DISCOVERY_COUNT_SQL)
        .bind(place_entity_id.0)
        .bind(PLACE_ACTIVITY_WINDOW)
        .fetch_one(&mut **transaction)
        .await
        .map_err(|error| storage_error(operation, error))?;
    u32::try_from(count).map_err(|_| invalid_stored_relation())
}

pub(super) async fn insert_attempt(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: UserId,
    request_id: Uuid,
    character_entity_id: EntityId,
    place_entity_id: EntityId,
    outcome: InvestigationOutcome,
    created_at: DateTime<Utc>,
) -> Result<InvestigationResult, WorldError> {
    let attempt_id = InvestigationAttemptId(Uuid::new_v4());
    sqlx::query(
        r#"
        INSERT INTO investigation_attempt (
            id, requested_by_user_id, request_id, character_entity_id,
            place_entity_id, outcome, created_at
        ) VALUES ($1, $2, $3, $4, $5, $6, $7)
        "#,
    )
    .bind(attempt_id.0)
    .bind(user_id.0)
    .bind(request_id)
    .bind(character_entity_id.0)
    .bind(place_entity_id.0)
    .bind(outcome.as_str())
    .bind(created_at)
    .execute(&mut **transaction)
    .await
    .map_err(|error| storage_error("start_investigation", error))?;
    if outcome == InvestigationOutcome::Positive {
        void_oldest_prior_positive(transaction, user_id, attempt_id, "start_investigation").await?;
    }
    Ok(InvestigationResult {
        attempt_id,
        outcome,
        limit: InvestigationLimit::CURRENT,
    })
}

async fn void_oldest_prior_positive(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: UserId,
    new_attempt_id: InvestigationAttemptId,
    operation: &'static str,
) -> Result<(), WorldError> {
    sqlx::query(VOID_OLDEST_PRIOR_POSITIVE_SQL)
        .bind(user_id.0)
        .bind(new_attempt_id.0)
        .bind(MAX_LIVE_POSITIVES)
        .execute(&mut **transaction)
        .await
        .map_err(|error| storage_error(operation, error))?;
    Ok(())
}

pub(super) async fn available_place(
    transaction: &mut Transaction<'_, Postgres>,
    attempt_id: InvestigationAttemptId,
    user_id: UserId,
    character_entity_id: EntityId,
    current_place_entity_id: EntityId,
    operation: &'static str,
) -> Result<Option<EntityId>, WorldError> {
    sqlx::query_scalar::<_, EntityId>(
        r#"
        SELECT place_entity_id
        FROM investigation_attempt
        WHERE id = $1
          AND requested_by_user_id = $2
          AND character_entity_id = $3
          AND place_entity_id = $4
          AND outcome = 'positive'
          AND consumed_by_activity_id IS NULL
          AND voided_by_attempt_id IS NULL
        "#,
    )
    .bind(attempt_id.0)
    .bind(user_id.0)
    .bind(character_entity_id.0)
    .bind(current_place_entity_id.0)
    .fetch_optional(&mut **transaction)
    .await
    .map_err(|error| storage_error(operation, error))
}

pub(super) async fn consume(
    transaction: &mut Transaction<'_, Postgres>,
    attempt_id: InvestigationAttemptId,
    activity_id: ActivityId,
    operation: &'static str,
) -> Result<(), WorldError> {
    let result = sqlx::query(
        r#"
        UPDATE investigation_attempt
        SET consumed_by_activity_id = $2
        WHERE id = $1
          AND outcome = 'positive'
          AND consumed_by_activity_id IS NULL
          AND voided_by_attempt_id IS NULL
        "#,
    )
    .bind(attempt_id.0)
    .bind(activity_id.0)
    .execute(&mut **transaction)
    .await
    .map_err(|error| storage_error(operation, error))?;
    if result.rows_affected() != 1 {
        return Err(WorldError::DiscoveryAttemptUnavailable);
    }
    Ok(())
}
