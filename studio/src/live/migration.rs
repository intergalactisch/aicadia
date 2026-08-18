//! Applied migrations joined to the repository files that declare them.

use std::path::Path;

use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::PgPool;

use crate::StudioError;

/// The largest number of migrations Studio will read or list.
const MAX_MIGRATION: usize = 1000;

#[derive(Debug, Serialize)]
pub struct AppliedMigration {
    pub version: i64,
    pub description: String,
    pub installed_on: DateTime<Utc>,
    pub success: bool,
    pub execution_time: i64,
    /// The applied checksum as lowercase hexadecimal.
    pub checksum: String,
    /// The repository file declaring this version, when one exists.
    pub file: Option<String>,
}

#[derive(sqlx::FromRow)]
struct MigrationRow {
    version: i64,
    description: String,
    installed_on: DateTime<Utc>,
    success: bool,
    execution_time: i64,
    checksum: Vec<u8>,
}

#[derive(Debug, Serialize)]
pub struct MigrationReport {
    pub applied: Vec<AppliedMigration>,
    /// Repository migration files with no applied row, newest last. A non-empty
    /// list means the connected database is behind the repository.
    pub unapplied_file: Vec<String>,
    pub truncated: bool,
}

/// Every applied migration, joined by version to the files under `game/migration/`.
///
/// `_sqlx_migrations` is read in primary-key order with a hard bound. The
/// repository side is a directory listing only: Studio reads file *names*, never
/// migration content, so nothing here can drift from what the file says because
/// nothing here reads what the file says. A file whose version has no applied row
/// is reported explicitly instead of being silently omitted.
pub async fn list_migration(
    pool: &PgPool,
    repository_root: &Path,
) -> Result<MigrationReport, StudioError> {
    let row = sqlx::query_as::<_, MigrationRow>(
        r#"
        SELECT version, description, installed_on, success, execution_time, checksum
        FROM _sqlx_migrations
        ORDER BY version
        LIMIT $1
        "#,
    )
    .bind(i64::try_from(MAX_MIGRATION + 1).expect("the migration bound fits i64"))
    .fetch_all(pool)
    .await?;
    let truncated = row.len() > MAX_MIGRATION;

    let mut file = repository_file(&repository_root.join("game/migration"))?;
    file.sort();
    let applied = row
        .into_iter()
        .take(MAX_MIGRATION)
        .map(|row| AppliedMigration {
            file: file
                .iter()
                .find(|name| file_version(name) == Some(row.version))
                .cloned(),
            version: row.version,
            description: row.description,
            installed_on: row.installed_on,
            success: row.success,
            execution_time: row.execution_time,
            checksum: row
                .checksum
                .iter()
                .map(|byte| format!("{byte:02x}"))
                .collect::<String>(),
        })
        .collect::<Vec<_>>();
    let unapplied_file = file
        .into_iter()
        .filter(|name| {
            file_version(name)
                .is_none_or(|version| !applied.iter().any(|applied| applied.version == version))
        })
        .collect::<Vec<_>>();

    Ok(MigrationReport {
        applied,
        unapplied_file,
        truncated,
    })
}

/// The `*.sql` file names directly under one directory.
fn repository_file(directory: &Path) -> Result<Vec<String>, StudioError> {
    let mut name = Vec::new();
    for entry in std::fs::read_dir(directory)? {
        let entry = entry?;
        if !entry.file_type()?.is_file() {
            continue;
        }
        let file_name = entry.file_name().to_string_lossy().into_owned();
        if file_name.ends_with(".sql") {
            name.push(file_name);
        }
        if name.len() > MAX_MIGRATION {
            return Err(StudioError::SourceTooLarge("migration"));
        }
    }
    Ok(name)
}

/// The version `sqlx` derives from a migration file name: the leading digits
/// before the first underscore.
fn file_version(file_name: &str) -> Option<i64> {
    file_name.split('_').next()?.parse::<i64>().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_migration_file_name_yields_the_version_sqlx_applied() {
        assert_eq!(file_version("0001_world.sql"), Some(1));
        assert_eq!(file_version("0010_investigation.sql"), Some(10));
        assert_eq!(file_version("world.sql"), None);
        assert_eq!(file_version("0002"), Some(2));
    }

    #[test]
    fn the_repository_migration_directory_lists_only_sql_files() {
        let file = repository_file(&crate::workspace_root().join("game/migration"))
            .expect("the repository migration directory is readable");

        assert!(
            file.len() >= 10,
            "only {} migration files found",
            file.len()
        );
        assert!(file.iter().all(|name| name.ends_with(".sql")));
        assert!(file.iter().all(|name| file_version(name).is_some()));
    }
}
