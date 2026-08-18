//! Property key: the canonical key an Agent created at its first accepted use,
//! its immutable value type and the Entities currently holding a value for it.

use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::PgPool;
use uuid::Uuid;

use super::page::{Bound, Page};
use crate::StudioError;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct PropertyKeyItem {
    pub id: i64,
    pub key: String,
    pub value_type: String,
    pub first_activity_id: Uuid,
}

/// Every canonical Property key, keyset by the key itself.
///
/// Served by the unique `property_key_key_key` index, so the list is both
/// alphabetical and exactly pageable however many keys the World has created.
pub async fn list_property_key(
    pool: &PgPool,
    after: Option<&str>,
    bound: Bound,
) -> Result<Page<PropertyKeyItem, String>, StudioError> {
    let item = sqlx::query_as::<_, PropertyKeyItem>(
        r#"
        SELECT id, key, value_type, first_activity_id
        FROM property_key
        WHERE ($1::text IS NULL OR key > $1::text)
        ORDER BY key
        LIMIT $2
        "#,
    )
    .bind(after)
    .bind(bound.fetch())
    .fetch_all(pool)
    .await?;
    Ok(Page::build(item, bound, |last| last.key.clone()))
}

#[derive(Debug, Serialize)]
pub struct PropertyKeyDetail {
    pub id: i64,
    pub key: String,
    pub value_type: String,
    pub first_activity_id: Uuid,
    pub first_activity_operation: String,
    pub first_activity_occurred_at: DateTime<Utc>,
}

#[derive(sqlx::FromRow)]
struct PropertyKeyRow {
    id: i64,
    key: String,
    value_type: String,
    first_activity_id: Uuid,
    first_activity_operation: String,
    first_activity_occurred_at: DateTime<Utc>,
}

/// One Property key with its type and first use.
///
/// The key and its first Activity are unique-index and primary-key lookups.
pub async fn get_property_key(pool: &PgPool, key: &str) -> Result<PropertyKeyDetail, StudioError> {
    let row = sqlx::query_as::<_, PropertyKeyRow>(
        r#"
        SELECT property_key.id,
               property_key.key,
               property_key.value_type,
               property_key.first_activity_id,
               first_activity.operation AS first_activity_operation,
               first_activity.occurred_at AS first_activity_occurred_at
        FROM property_key
        JOIN activity AS first_activity ON first_activity.id = property_key.first_activity_id
        WHERE property_key.key = $1
        "#,
    )
    .bind(key)
    .fetch_optional(pool)
    .await?
    .ok_or(StudioError::NotFound)?;

    Ok(PropertyKeyDetail {
        id: row.id,
        key: row.key,
        value_type: row.value_type,
        first_activity_id: row.first_activity_id,
        first_activity_operation: row.first_activity_operation,
        first_activity_occurred_at: row.first_activity_occurred_at,
    })
}
