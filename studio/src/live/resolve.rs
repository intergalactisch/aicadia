//! One id in, every record that holds it out.
//!
//! The resolver is an exact-id lookup, never a search: it answers "what is this
//! UUID" for an operator holding an id from a log line, an error or another
//! page. It never matches names, prose or partial ids.

use serde::Serialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::StudioError;

/// How the resolver found one hit, so the page can show what the answer cost.
pub const PRIMARY_KEY: &str = "primary key";

#[derive(Debug, Serialize)]
pub struct ResolveHit {
    /// The World subject or provenance column that holds the id.
    pub subject: &'static str,
    /// The stable primary key of the record that holds it.
    pub id: Uuid,
    pub name: Option<String>,
    pub detail: Option<String>,
    pub lookup: &'static str,
}

#[derive(Debug, Serialize)]
pub struct ResolveResult {
    pub id: Uuid,
    pub hit: Vec<ResolveHit>,
}

#[derive(sqlx::FromRow)]
struct NamedRow {
    id: Uuid,
    name: Option<String>,
    detail: Option<String>,
}

/// Every record holding one exact UUID.
///
/// All five probes are primary keys: `entity.id`, `activity.id`,
/// `user.id`, `entity_trait.id` and `investigation_attempt.id`.
pub async fn resolve(pool: &PgPool, id: Uuid) -> Result<ResolveResult, StudioError> {
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

    Ok(ResolveResult { id, hit })
}
