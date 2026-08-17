//! Property key: the canonical key an Agent created at its first accepted use,
//! its immutable value type and the Entities currently holding a value for it.

use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::PgPool;
use uuid::Uuid;

use super::page::{self, Bound, Page};
use crate::studio::StudioError;

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

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct PropertyHolder {
    pub entity_id: Uuid,
    pub entity_name: String,
    pub value_type: String,
    pub text_value: Option<String>,
    pub integer_value: Option<i64>,
    pub current_activity_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct PropertyKeyDetail {
    pub id: i64,
    pub key: String,
    pub value_type: String,
    pub first_activity_id: Uuid,
    pub first_activity_operation: String,
    pub first_activity_occurred_at: DateTime<Utc>,
    /// Entities with a current value for this key. Not indexed by key; see the
    /// read's doc comment.
    pub holder: Vec<PropertyHolder>,
    pub holder_truncated: bool,
    pub holder_scope: &'static str,
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

/// One Property key with its type, first use and current holders.
///
/// The key and its first Activity are unique-index and primary-key lookups.
/// The holder list is not: `entity_property` has primary key
/// `(entity_id, property_key_id)` and no index leading with the key, because
/// every current game read asks "what does this Entity hold", never "who holds
/// this key". Studio does not add that index. It scans with a hard stop at 100
/// rows, labels the read `local development scan` and reports
/// `holder_truncated`, so an operator sees both the answer and its cost.
pub async fn get_property_key(pool: &PgPool, key: &str) -> Result<PropertyKeyDetail, StudioError> {
    let bound = Bound::new(Some(page::MAX_LIMIT))?;
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

    let mut holder = sqlx::query_as::<_, PropertyHolder>(
        r#"
        SELECT entity_property.entity_id,
               entity.name AS entity_name,
               history.value_type,
               history.text_value,
               history.integer_value,
               entity_property.current_activity_id
        FROM entity_property
        JOIN entity ON entity.id = entity_property.entity_id
        JOIN entity_property_history AS history
          ON history.entity_id = entity_property.entity_id
         AND history.property_key_id = entity_property.property_key_id
         AND history.activity_id = entity_property.current_activity_id
        WHERE entity_property.property_key_id = $1
        ORDER BY entity_property.entity_id
        LIMIT $2
        "#,
    )
    .bind(row.id)
    .bind(bound.fetch())
    .fetch_all(pool)
    .await?;
    let holder_truncated = page::truncate(&mut holder, bound.limit());

    Ok(PropertyKeyDetail {
        id: row.id,
        key: row.key,
        value_type: row.value_type,
        first_activity_id: row.first_activity_id,
        first_activity_operation: row.first_activity_operation,
        first_activity_occurred_at: row.first_activity_occurred_at,
        holder,
        holder_truncated,
        holder_scope: page::LOCAL_DEVELOPMENT_SCAN,
    })
}
