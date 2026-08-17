//! One id in, every record that holds it out.
//!
//! The resolver is an exact-id lookup, never a search: it answers "what is this
//! UUID" for an operator holding an id from a log line, an error or another
//! page. It never matches names, prose or partial ids.

use serde::Serialize;
use sqlx::PgPool;
use uuid::Uuid;

use super::page::{self, Bound};
use crate::studio::StudioError;

/// How the resolver found one hit, so the page can show what the answer cost.
pub const PRIMARY_KEY: &str = "primary key";
pub const LOCAL_DEVELOPMENT_SCAN: &str = page::LOCAL_DEVELOPMENT_SCAN;

#[derive(Debug, Serialize)]
pub struct ResolveHit {
    /// The World subject or provenance column that holds the id.
    pub subject: &'static str,
    /// The stable id of the record that holds it: for a `request_id` hit, the id
    /// of the owning Activity or attempt.
    pub id: Uuid,
    pub name: Option<String>,
    pub detail: Option<String>,
    pub lookup: &'static str,
}

#[derive(Debug, Serialize)]
pub struct ResolveResult {
    pub id: Uuid,
    pub hit: Vec<ResolveHit>,
    /// Labels the two `request_id` probes, which no index serves.
    pub request_id_scope: &'static str,
}

#[derive(sqlx::FromRow)]
struct NamedRow {
    id: Uuid,
    name: Option<String>,
    detail: Option<String>,
}

/// Every record holding one exact UUID.
///
/// Five of the seven probes are primary keys: `entity.id`, `activity.id`,
/// `user.id`, `entity_trait.id` and `investigation_attempt.id`.
///
/// The two `request_id` probes are not. Both `activity` and
/// `investigation_attempt` make a request id unique only *within its User* —
/// `activity_requested_by_user_id_request_id_index` and
/// `investigation_attempt_requested_by_user_id_request_id_key` both lead with
/// `requested_by_user_id` — because retry lookup always knows the User. A probe
/// by request id alone cannot use either index and is a scan. Studio keeps it
/// because resolving an id from a log line is exactly what an operator needs,
/// bounds each probe at 100 rows and labels the result
/// `request_id_scope = "local development scan"`. No game capability may be
/// built on it.
pub async fn resolve(pool: &PgPool, id: Uuid) -> Result<ResolveResult, StudioError> {
    let bound = Bound::new(Some(page::MAX_LIMIT))?;
    let mut hit = Vec::new();

    for (subject, sql) in [
        (
            "entity",
            r#"SELECT id, name, NULL::text AS detail FROM entity WHERE id = $1"#,
        ),
        (
            "activity",
            r#"SELECT id, NULL::text AS name, operation AS detail FROM activity WHERE id = $1"#,
        ),
        (
            "user",
            r#"SELECT id, NULL::text AS name, NULL::text AS detail FROM "user" WHERE id = $1"#,
        ),
        (
            "trait",
            r#"
            SELECT entity_trait.id,
                   entity.name,
                   NULL::text AS detail
            FROM entity_trait
            JOIN entity ON entity.id = entity_trait.entity_id
            WHERE entity_trait.id = $1
            "#,
        ),
        (
            "investigation attempt",
            r#"
            SELECT id, NULL::text AS name, outcome AS detail
            FROM investigation_attempt
            WHERE id = $1
            "#,
        ),
    ] {
        if let Some(row) = sqlx::query_as::<_, NamedRow>(sql)
            .bind(id)
            .fetch_optional(pool)
            .await?
        {
            hit.push(ResolveHit {
                subject,
                id: row.id,
                name: row.name,
                detail: row.detail,
                lookup: PRIMARY_KEY,
            });
        }
    }

    for (subject, sql) in [
        (
            "activity request",
            r#"
            SELECT id, NULL::text AS name, operation AS detail
            FROM activity
            WHERE request_id = $1
            ORDER BY id
            LIMIT $2
            "#,
        ),
        (
            "investigation attempt request",
            r#"
            SELECT id, NULL::text AS name, outcome AS detail
            FROM investigation_attempt
            WHERE request_id = $1
            ORDER BY id
            LIMIT $2
            "#,
        ),
    ] {
        let mut row = sqlx::query_as::<_, NamedRow>(sql)
            .bind(id)
            .bind(bound.fetch())
            .fetch_all(pool)
            .await?;
        page::truncate(&mut row, bound.limit());
        hit.extend(row.into_iter().map(|row| ResolveHit {
            subject,
            id: row.id,
            name: row.name,
            detail: row.detail,
            lookup: LOCAL_DEVELOPMENT_SCAN,
        }));
    }

    Ok(ResolveResult {
        id,
        hit,
        request_id_scope: LOCAL_DEVELOPMENT_SCAN,
    })
}
