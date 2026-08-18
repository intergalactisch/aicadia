//! The connected PostgreSQL `public` schema as Studio reads it.
//!
//! Every structural read here is capped: crossing a cap fails the complete read
//! instead of presenting partial structure. The same introspection is the only
//! source that decides which table names, columns and primary keys the generic
//! row viewer in [`super::row`] is allowed to touch.

use chrono::{DateTime, Utc};
use serde::Serialize;
use sha2::{Digest, Sha256};
use sqlx::PgPool;

use crate::StudioError;

pub const MAX_SCHEMA_TABLE: usize = 256;
const MAX_SCHEMA_COLUMN: usize = 4096;
const MAX_SCHEMA_CONSTRAINT: usize = 4096;
const MAX_SCHEMA_RELATION: usize = 4096;
const MAX_SCHEMA_INDEX: usize = 4096;
pub const LATEST_MIGRATION_WINDOW: usize = 100;
pub const LATEST_MIGRATION_KNOWN: &str = "known";
pub const LATEST_MIGRATION_UNKNOWN_PARTIAL: &str = "unknown/partial";

/// The migration bookkeeping relation. It is never an application table and is
/// read only by [`super::migration`].
pub const MIGRATION_TABLE: &str = "_sqlx_migrations";

#[derive(Clone, Serialize, sqlx::FromRow)]
pub struct StorageColumn {
    table: String,
    name: String,
    position: i32,
    data_type: String,
    nullable: bool,
    default_value: Option<String>,
    identity: bool,
    generated: bool,
}

#[derive(Clone, Serialize, sqlx::FromRow)]
pub struct StorageConstraint {
    table: String,
    name: String,
    kind: String,
    columns: Vec<String>,
    definition: String,
}

#[derive(Clone, Serialize, sqlx::FromRow)]
pub struct StorageRelation {
    name: String,
    table: String,
    columns: Vec<String>,
    referenced_table: String,
    referenced_columns: Vec<String>,
    on_update: String,
    on_delete: String,
}

#[derive(Clone, Serialize, sqlx::FromRow)]
pub struct StorageIndex {
    table: String,
    name: String,
    definition: String,
    is_unique: bool,
    is_primary: bool,
}

#[derive(Clone, Serialize)]
pub struct StorageTable {
    name: String,
    column: Vec<StorageColumn>,
    constraint: Vec<StorageConstraint>,
    index: Vec<StorageIndex>,
}

#[derive(Serialize)]
pub struct StorageSnapshot {
    captured_at: DateTime<Utc>,
    latest_migration: LatestSuccessfulMigration,
    fingerprint: String,
    schema: &'static str,
    table: Vec<StorageTable>,
    relation: Vec<StorageRelation>,
}

#[derive(Debug, Serialize, PartialEq, Eq)]
pub struct LatestSuccessfulMigration {
    pub version: Option<i64>,
    pub state: &'static str,
    pub inspected_newest: usize,
}

#[derive(sqlx::FromRow)]
struct MigrationSuccessRow {
    version: i64,
    success: bool,
}

#[derive(Serialize)]
struct StorageFingerprint<'a> {
    schema: &'static str,
    table: &'a [StorageTable],
    relation: &'a [StorageRelation],
}

/// One introspected column with the exact type a cursor value must be cast to.
#[derive(Clone, Debug, Serialize, sqlx::FromRow)]
pub struct ColumnType {
    pub name: String,
    pub position: i32,
    pub data_type: String,
}

/// The ordinary and partitioned `public` tables, application tables first and
/// `_sqlx_migrations` excluded. This discovered set is the row viewer's complete
/// admissible table boundary.
pub async fn application_table(pool: &PgPool) -> Result<Vec<String>, StudioError> {
    let table_name = sqlx::query_scalar::<_, String>(
        r#"
        SELECT relation.relname
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
    ensure_schema_bound(table_name.len(), MAX_SCHEMA_TABLE, "tables")?;
    Ok(table_name)
}

/// The columns of one introspected table in storage order, with each column's
/// `format_type` so a text cursor value can be cast back to its own type.
pub async fn table_column(pool: &PgPool, table: &str) -> Result<Vec<ColumnType>, StudioError> {
    let column = sqlx::query_as::<_, ColumnType>(
        r#"
        SELECT attribute.attname AS name,
               attribute.attnum::integer AS position,
               format_type(attribute.atttypid, attribute.atttypmod) AS data_type
        FROM pg_class AS relation
        JOIN pg_namespace AS namespace ON namespace.oid = relation.relnamespace
        JOIN pg_attribute AS attribute ON attribute.attrelid = relation.oid
        WHERE namespace.nspname = 'public'
          AND relation.relname = $1
          AND attribute.attnum > 0
          AND NOT attribute.attisdropped
        ORDER BY attribute.attnum
        LIMIT $2
        "#,
    )
    .bind(table)
    .bind(i64::try_from(MAX_SCHEMA_COLUMN + 1).expect("schema column bound fits i64"))
    .fetch_all(pool)
    .await?;
    ensure_schema_bound(column.len(), MAX_SCHEMA_COLUMN, "columns")?;
    Ok(column)
}

/// The primary-key columns of one introspected table in key order. An empty
/// answer means the table has no primary key and can only be read in the
/// unstable physical order the row viewer labels as such.
pub async fn primary_key_column(pool: &PgPool, table: &str) -> Result<Vec<String>, StudioError> {
    Ok(sqlx::query_scalar::<_, String>(
        r#"
        SELECT attribute.attname
        FROM pg_constraint AS table_constraint
        JOIN pg_class AS relation ON relation.oid = table_constraint.conrelid
        JOIN pg_namespace AS namespace ON namespace.oid = relation.relnamespace
        JOIN unnest(table_constraint.conkey) WITH ORDINALITY AS key(number, position) ON true
        JOIN pg_attribute AS attribute
          ON attribute.attrelid = relation.oid
         AND attribute.attnum = key.number
        WHERE namespace.nspname = 'public'
          AND relation.relname = $1
          AND table_constraint.contype = 'p'
        ORDER BY key.position
        LIMIT $2
        "#,
    )
    .bind(table)
    .bind(i64::try_from(MAX_SCHEMA_COLUMN + 1).expect("schema column bound fits i64"))
    .fetch_all(pool)
    .await?)
}

/// The latest successful migration found inside one fixed newest-version window.
///
/// `_sqlx_migrations(version)` is its primary key, so the query reads one bounded
/// index range newest first. If that window contains no successful row, Studio
/// reports `unknown/partial`: it never scans older rows to manufacture a definite
/// answer.
pub async fn latest_successful_migration(
    pool: &PgPool,
) -> Result<LatestSuccessfulMigration, StudioError> {
    let row = sqlx::query_as::<_, MigrationSuccessRow>(
        r#"
        SELECT version, success
        FROM _sqlx_migrations
        ORDER BY version DESC
        LIMIT $1
        "#,
    )
    .bind(i64::try_from(LATEST_MIGRATION_WINDOW).expect("migration window fits i64"))
    .fetch_all(pool)
    .await?;
    let inspected_newest = row.len();
    let version = row
        .into_iter()
        .find(|migration| migration.success)
        .map(|migration| migration.version);
    Ok(LatestSuccessfulMigration {
        state: if version.is_some() {
            LATEST_MIGRATION_KNOWN
        } else {
            LATEST_MIGRATION_UNKNOWN_PARTIAL
        },
        version,
        inspected_newest,
    })
}

pub async fn read_storage(pool: &PgPool) -> Result<StorageSnapshot, StudioError> {
    let table_name = application_table(pool).await?;

    let column = sqlx::query_as::<_, StorageColumn>(
        r#"
        SELECT relation.relname AS table,
               attribute.attname AS name,
               attribute.attnum::integer AS position,
               format_type(attribute.atttypid, attribute.atttypmod) AS data_type,
               NOT attribute.attnotnull AS nullable,
               pg_get_expr(default_value.adbin, default_value.adrelid) AS default_value,
               attribute.attidentity <> '' AS identity,
               attribute.attgenerated <> '' AS generated
        FROM pg_class AS relation
        JOIN pg_namespace AS namespace ON namespace.oid = relation.relnamespace
        JOIN pg_attribute AS attribute ON attribute.attrelid = relation.oid
        LEFT JOIN pg_attrdef AS default_value
          ON default_value.adrelid = relation.oid
         AND default_value.adnum = attribute.attnum
        WHERE namespace.nspname = 'public'
          AND relation.relname = ANY($1)
          AND attribute.attnum > 0
          AND NOT attribute.attisdropped
        ORDER BY relation.relname, attribute.attnum
        LIMIT $2
        "#,
    )
    .bind(&table_name)
    .bind(i64::try_from(MAX_SCHEMA_COLUMN + 1).expect("schema column bound fits i64"))
    .fetch_all(pool)
    .await?;
    ensure_schema_bound(column.len(), MAX_SCHEMA_COLUMN, "columns")?;

    let constraint = sqlx::query_as::<_, StorageConstraint>(
        r#"
        SELECT relation.relname AS table,
               table_constraint.conname AS name,
               CASE table_constraint.contype
                   WHEN 'p' THEN 'primary key'
                   WHEN 'u' THEN 'unique'
                   WHEN 'c' THEN 'check'
                   WHEN 'x' THEN 'exclusion'
               END AS kind,
               COALESCE(
                   ARRAY(
                       SELECT attribute.attname
                       FROM unnest(table_constraint.conkey) WITH ORDINALITY AS key(number, position)
                       JOIN pg_attribute AS attribute
                         ON attribute.attrelid = relation.oid
                        AND attribute.attnum = key.number
                       ORDER BY key.position
                   ),
                   ARRAY[]::text[]
               ) AS columns,
               pg_get_constraintdef(table_constraint.oid, true) AS definition
        FROM pg_constraint AS table_constraint
        JOIN pg_class AS relation ON relation.oid = table_constraint.conrelid
        JOIN pg_namespace AS namespace ON namespace.oid = relation.relnamespace
        WHERE namespace.nspname = 'public'
          AND relation.relname = ANY($1)
          AND table_constraint.contype IN ('p', 'u', 'c', 'x')
        ORDER BY relation.relname, table_constraint.conname
        LIMIT $2
        "#,
    )
    .bind(&table_name)
    .bind(i64::try_from(MAX_SCHEMA_CONSTRAINT + 1).expect("schema constraint bound fits i64"))
    .fetch_all(pool)
    .await?;
    ensure_schema_bound(constraint.len(), MAX_SCHEMA_CONSTRAINT, "constraints")?;

    let relation = sqlx::query_as::<_, StorageRelation>(
        r#"
        SELECT relation_constraint.conname AS name,
               source.relname AS table,
               ARRAY(
                   SELECT source_attribute.attname
                   FROM unnest(relation_constraint.conkey) WITH ORDINALITY AS key(number, position)
                   JOIN pg_attribute AS source_attribute
                     ON source_attribute.attrelid = source.oid
                    AND source_attribute.attnum = key.number
                   ORDER BY key.position
               ) AS columns,
               target.relname AS referenced_table,
               ARRAY(
                   SELECT target_attribute.attname
                   FROM unnest(relation_constraint.confkey) WITH ORDINALITY AS key(number, position)
                   JOIN pg_attribute AS target_attribute
                     ON target_attribute.attrelid = target.oid
                    AND target_attribute.attnum = key.number
                   ORDER BY key.position
               ) AS referenced_columns,
               CASE relation_constraint.confupdtype
                   WHEN 'a' THEN 'no action'
                   WHEN 'r' THEN 'restrict'
                   WHEN 'c' THEN 'cascade'
                   WHEN 'n' THEN 'set null'
                   WHEN 'd' THEN 'set default'
               END AS on_update,
               CASE relation_constraint.confdeltype
                   WHEN 'a' THEN 'no action'
                   WHEN 'r' THEN 'restrict'
                   WHEN 'c' THEN 'cascade'
                   WHEN 'n' THEN 'set null'
                   WHEN 'd' THEN 'set default'
               END AS on_delete
        FROM pg_constraint AS relation_constraint
        JOIN pg_class AS source ON source.oid = relation_constraint.conrelid
        JOIN pg_namespace AS namespace ON namespace.oid = source.relnamespace
        JOIN pg_class AS target ON target.oid = relation_constraint.confrelid
        WHERE relation_constraint.contype = 'f'
          AND namespace.nspname = 'public'
          AND source.relname = ANY($1)
        ORDER BY source.relname, relation_constraint.conname
        LIMIT $2
        "#,
    )
    .bind(&table_name)
    .bind(i64::try_from(MAX_SCHEMA_RELATION + 1).expect("schema relation bound fits i64"))
    .fetch_all(pool)
    .await?;
    ensure_schema_bound(relation.len(), MAX_SCHEMA_RELATION, "relations")?;

    let index = sqlx::query_as::<_, StorageIndex>(
        r#"
        SELECT source.relname AS table,
               index_relation.relname AS name,
               pg_get_indexdef(index_relation.oid) AS definition,
               index_metadata.indisunique AS is_unique,
               index_metadata.indisprimary AS is_primary
        FROM pg_index AS index_metadata
        JOIN pg_class AS source ON source.oid = index_metadata.indrelid
        JOIN pg_namespace AS namespace ON namespace.oid = source.relnamespace
        JOIN pg_class AS index_relation ON index_relation.oid = index_metadata.indexrelid
        WHERE namespace.nspname = 'public'
          AND source.relname = ANY($1)
        ORDER BY source.relname, index_relation.relname
        LIMIT $2
        "#,
    )
    .bind(&table_name)
    .bind(i64::try_from(MAX_SCHEMA_INDEX + 1).expect("schema index bound fits i64"))
    .fetch_all(pool)
    .await?;
    ensure_schema_bound(index.len(), MAX_SCHEMA_INDEX, "indexes")?;

    let latest_migration = latest_successful_migration(pool).await?;
    let table = table_name
        .into_iter()
        .map(|name| StorageTable {
            column: column
                .iter()
                .filter(|item| item.table == name)
                .cloned()
                .collect(),
            constraint: constraint
                .iter()
                .filter(|item| item.table == name)
                .cloned()
                .collect(),
            index: index
                .iter()
                .filter(|item| item.table == name)
                .cloned()
                .collect(),
            name,
        })
        .collect::<Vec<_>>();
    let fingerprint = storage_fingerprint(&table, &relation);
    Ok(StorageSnapshot {
        captured_at: Utc::now(),
        latest_migration,
        fingerprint,
        schema: "public",
        table,
        relation,
    })
}

pub fn ensure_schema_bound(
    actual: usize,
    maximum: usize,
    subject: &'static str,
) -> Result<(), StudioError> {
    if actual > maximum {
        return Err(StudioError::SchemaTooLarge(subject));
    }
    Ok(())
}

fn storage_fingerprint(table: &[StorageTable], relation: &[StorageRelation]) -> String {
    let payload = serde_json::to_vec(&StorageFingerprint {
        schema: "public",
        table,
        relation,
    })
    .expect("the serializable Studio schema structure must encode as JSON");
    let digest = Sha256::digest(payload);
    digest
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schema_fingerprint_is_structural_and_deterministic() {
        let table = vec![StorageTable {
            name: "entity".to_owned(),
            column: vec![StorageColumn {
                table: "entity".to_owned(),
                name: "id".to_owned(),
                position: 1,
                data_type: "uuid".to_owned(),
                nullable: false,
                default_value: None,
                identity: false,
                generated: false,
            }],
            constraint: vec![StorageConstraint {
                table: "entity".to_owned(),
                name: "entity_pkey".to_owned(),
                kind: "primary key".to_owned(),
                columns: vec!["id".to_owned()],
                definition: "PRIMARY KEY (id)".to_owned(),
            }],
            index: Vec::new(),
        }];

        let first = storage_fingerprint(&table, &[]);
        let second = storage_fingerprint(&table, &[]);
        let mut changed = table.clone();
        changed[0].column[0].nullable = true;

        assert_eq!(first, second);
        assert_eq!(first.len(), 64);
        assert_ne!(first, storage_fingerprint(&changed, &[]));
    }

    #[test]
    fn schema_bounds_fail_closed() {
        assert!(ensure_schema_bound(256, 256, "tables").is_ok());
        assert!(matches!(
            ensure_schema_bound(257, 256, "tables"),
            Err(StudioError::SchemaTooLarge("tables"))
        ));
    }
}
