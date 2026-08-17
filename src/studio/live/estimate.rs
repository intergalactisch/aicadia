//! Planner row estimates per application table.
//!
//! Studio never counts rows. An exact `count(*)` is an unbounded read of the
//! whole table, and a page that shows one would cost more the more the World
//! holds. `pg_class.reltuples` is what the query planner itself believes, is
//! read from one catalog row per table and is labeled as an estimate everywhere
//! it appears.

use serde::Serialize;
use sqlx::PgPool;

use super::schema::{self, MAX_SCHEMA_TABLE};
use crate::studio::StudioError;

/// The label every row estimate carries.
pub const PLANNER_ESTIMATE: &str = "planner estimate";

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct TableEstimate {
    pub table: String,
    /// The planner's believed row count, or absent when the table has never been
    /// analyzed (`reltuples = -1`).
    pub row_estimate: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct EstimateReport {
    pub scope: &'static str,
    pub table: Vec<TableEstimate>,
}

/// The planner's row estimate for every application table in `public`.
///
/// One bounded read of `pg_class`; `_sqlx_migrations` is excluded because it is
/// bookkeeping, not World data. A table PostgreSQL has never analyzed stores
/// `reltuples = -1`, which is reported as an absent estimate rather than as
/// "minus one rows" or as zero.
pub async fn estimate(pool: &PgPool) -> Result<EstimateReport, StudioError> {
    let table = sqlx::query_as::<_, TableEstimate>(
        r#"
        SELECT relation.relname AS table,
               CASE
                   WHEN relation.reltuples < 0 THEN NULL
                   ELSE relation.reltuples::bigint
               END AS row_estimate
        FROM pg_class AS relation
        JOIN pg_namespace AS namespace ON namespace.oid = relation.relnamespace
        WHERE namespace.nspname = 'public'
          AND relation.relkind IN ('r', 'p')
          AND relation.relname <> '_sqlx_migrations'
        ORDER BY relation.relname
        LIMIT $1
        "#,
    )
    .bind(i64::try_from(MAX_SCHEMA_TABLE + 1).expect("schema table bound fits i64"))
    .fetch_all(pool)
    .await?;
    schema::ensure_schema_bound(table.len(), MAX_SCHEMA_TABLE, "tables")?;
    Ok(EstimateReport {
        scope: PLANNER_ESTIMATE,
        table,
    })
}
