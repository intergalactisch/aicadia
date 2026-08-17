use axum::{
    Json,
    extract::{Path, Query, State},
    http::header,
    response::{IntoResponse, Response},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use uuid::Uuid;

use super::{StudioError, StudioState};

const DEFAULT_LIMIT: u16 = 24;
const MAX_LIMIT: u16 = 100;
const ENTITY_STATE_PREVIEW_LIMIT: i64 = 50;
const ACTIVITY_ENTITY_PREVIEW_LIMIT: i64 = 256;
const MAX_SCHEMA_TABLE: usize = 256;
const MAX_SCHEMA_COLUMN: usize = 4096;
const MAX_SCHEMA_CONSTRAINT: usize = 4096;
const MAX_SCHEMA_RELATION: usize = 4096;
const MAX_SCHEMA_INDEX: usize = 4096;

#[derive(Deserialize)]
pub(super) struct PageQuery {
    limit: Option<u16>,
    before: Option<Uuid>,
}

impl PageQuery {
    fn limit(&self) -> Result<usize, StudioError> {
        let limit = self.limit.unwrap_or(DEFAULT_LIMIT);
        if !(1..=MAX_LIMIT).contains(&limit) {
            return Err(StudioError::InvalidLimit);
        }
        Ok(usize::from(limit))
    }
}

#[derive(Serialize, sqlx::FromRow)]
pub(super) struct CharacterView {
    id: Uuid,
    name: String,
    description: String,
    owner_user_id: Uuid,
    current_place_entity_id: Option<Uuid>,
    current_place_name: Option<String>,
    introduced_at: DateTime<Utc>,
}

#[derive(Serialize)]
pub(super) struct CharacterPage {
    character: Vec<CharacterView>,
    next_cursor: Option<Uuid>,
}

pub(super) async fn list_character(
    State(state): State<StudioState>,
    Query(query): Query<PageQuery>,
) -> Result<Json<CharacterPage>, StudioError> {
    let limit = query.limit()?;
    let mut character = sqlx::query_as::<_, CharacterView>(
        r#"
        SELECT character.entity_id AS id,
               entity.name,
               entity.description,
               character.owner_user_id,
               character.current_place_entity_id,
               current_place.name AS current_place_name,
               entity.introduced_at
        FROM character
        JOIN entity ON entity.id = character.entity_id
        LEFT JOIN entity AS current_place ON current_place.id = character.current_place_entity_id
        WHERE ($1::uuid IS NULL OR character.entity_id < $1)
        ORDER BY character.entity_id DESC
        LIMIT $2
        "#,
    )
    .bind(query.before)
    .bind(i64::try_from(limit + 1).expect("Studio page limit fits i64"))
    .fetch_all(&state.pool)
    .await?;
    let next_cursor = page_cursor(&mut character, limit, |item| item.id);
    Ok(Json(CharacterPage {
        character,
        next_cursor,
    }))
}

#[derive(Serialize, sqlx::FromRow)]
pub(super) struct EntityView {
    id: Uuid,
    name: String,
    description: String,
    introduced_by_user_id: Uuid,
    introduced_at: DateTime<Utc>,
    is_character: bool,
    is_place: bool,
    place_entity_id: Option<Uuid>,
    place_name: Option<String>,
}

#[derive(Serialize, sqlx::FromRow)]
pub(super) struct PropertyView {
    key: String,
    value_type: String,
    text_value: Option<String>,
    integer_value: Option<i64>,
    current_activity_id: Uuid,
}

#[derive(Serialize, sqlx::FromRow)]
pub(super) struct TraitView {
    id: Uuid,
    statement: String,
    current_activity_id: Uuid,
}

#[derive(Serialize)]
pub(super) struct EntityDetail {
    entity: EntityView,
    property: Vec<PropertyView>,
    property_truncated: bool,
    r#trait: Vec<TraitView>,
    trait_truncated: bool,
}

pub(super) async fn get_entity(
    State(state): State<StudioState>,
    Path(entity_id): Path<Uuid>,
) -> Result<Json<EntityDetail>, StudioError> {
    let entity = sqlx::query_as::<_, EntityView>(
        r#"
        SELECT entity.id,
               entity.name,
               entity.description,
               entity.introduced_by_user_id,
               entity.introduced_at,
               EXISTS (SELECT 1 FROM character WHERE character.entity_id = entity.id) AS is_character,
               EXISTS (SELECT 1 FROM place WHERE place.entity_id = entity.id) AS is_place,
               entity_location.place_entity_id,
               location.name AS place_name
        FROM entity
        LEFT JOIN entity_location ON entity_location.entity_id = entity.id
        LEFT JOIN entity AS location ON location.id = entity_location.place_entity_id
        WHERE entity.id = $1
        "#,
    )
    .bind(entity_id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or(StudioError::NotFound)?;
    let mut property = sqlx::query_as::<_, PropertyView>(
        r#"
        SELECT property_key.key,
               history.value_type,
               history.text_value,
               history.integer_value,
               entity_property.current_activity_id
        FROM entity_property
        JOIN property_key ON property_key.id = entity_property.property_key_id
        JOIN entity_property_history AS history
          ON history.entity_id = entity_property.entity_id
         AND history.property_key_id = entity_property.property_key_id
         AND history.activity_id = entity_property.current_activity_id
        WHERE entity_property.entity_id = $1
        ORDER BY property_key.key
        LIMIT $2
        "#,
    )
    .bind(entity_id)
    .bind(ENTITY_STATE_PREVIEW_LIMIT + 1)
    .fetch_all(&state.pool)
    .await?;
    let property_truncated = property.len() > ENTITY_STATE_PREVIEW_LIMIT as usize;
    property.truncate(ENTITY_STATE_PREVIEW_LIMIT as usize);
    let mut r#trait = sqlx::query_as::<_, TraitView>(
        r#"
        SELECT entity_trait.id,
               version.statement,
               entity_trait_current.current_activity_id
        FROM entity_trait
        JOIN entity_trait_current ON entity_trait_current.trait_id = entity_trait.id
        JOIN entity_trait_version AS version
          ON version.trait_id = entity_trait_current.trait_id
         AND version.entity_id = entity_trait_current.entity_id
         AND version.activity_id = entity_trait_current.current_activity_id
        WHERE entity_trait.entity_id = $1
        ORDER BY entity_trait.id
        LIMIT $2
        "#,
    )
    .bind(entity_id)
    .bind(ENTITY_STATE_PREVIEW_LIMIT + 1)
    .fetch_all(&state.pool)
    .await?;
    let trait_truncated = r#trait.len() > ENTITY_STATE_PREVIEW_LIMIT as usize;
    r#trait.truncate(ENTITY_STATE_PREVIEW_LIMIT as usize);
    Ok(Json(EntityDetail {
        entity,
        property,
        property_truncated,
        r#trait,
        trait_truncated,
    }))
}

#[derive(Serialize, sqlx::FromRow)]
pub(super) struct ActivityView {
    id: Uuid,
    operation: String,
    prose: Option<String>,
    action_consequence: Option<String>,
    actor_character_entity_id: Option<Uuid>,
    context_place_entity_id: Option<Uuid>,
    occurred_at: DateTime<Utc>,
}

#[derive(Serialize, sqlx::FromRow)]
pub(super) struct ActivityEntityView {
    entity_id: Uuid,
    entity_name: String,
    role: String,
}

#[derive(Serialize)]
pub(super) struct ActivityDetail {
    activity: ActivityView,
    involved_entity: Vec<ActivityEntityView>,
    involved_entity_truncated: bool,
}

pub(super) async fn get_activity(
    State(state): State<StudioState>,
    Path(activity_id): Path<Uuid>,
) -> Result<Json<ActivityDetail>, StudioError> {
    let activity = sqlx::query_as::<_, ActivityView>(
        r#"
        SELECT id,
               operation,
               prose,
               action_consequence,
               actor_character_entity_id,
               context_place_entity_id,
               occurred_at
        FROM activity
        WHERE id = $1
        "#,
    )
    .bind(activity_id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or(StudioError::NotFound)?;
    let mut involved_entity = sqlx::query_as::<_, ActivityEntityView>(
        r#"
        SELECT activity_entity.entity_id,
               entity.name AS entity_name,
               activity_entity.role
        FROM activity_entity
        JOIN entity ON entity.id = activity_entity.entity_id
        WHERE activity_entity.activity_id = $1
        ORDER BY activity_entity.role, activity_entity.entity_id
        LIMIT $2
        "#,
    )
    .bind(activity_id)
    .bind(ACTIVITY_ENTITY_PREVIEW_LIMIT + 1)
    .fetch_all(&state.pool)
    .await?;
    let involved_entity_truncated = involved_entity.len() > ACTIVITY_ENTITY_PREVIEW_LIMIT as usize;
    involved_entity.truncate(ACTIVITY_ENTITY_PREVIEW_LIMIT as usize);
    Ok(Json(ActivityDetail {
        activity,
        involved_entity,
        involved_entity_truncated,
    }))
}

#[derive(Serialize, sqlx::FromRow)]
pub(super) struct PlaceView {
    id: Uuid,
    name: String,
    description: String,
    is_entry: bool,
    latest_activity_id: Uuid,
    introduced_at: DateTime<Utc>,
}

#[derive(Serialize)]
pub(super) struct PlacePage {
    place: Vec<PlaceView>,
    next_cursor: Option<Uuid>,
}

pub(super) async fn list_place(
    State(state): State<StudioState>,
    Query(query): Query<PageQuery>,
) -> Result<Json<PlacePage>, StudioError> {
    let limit = query.limit()?;
    let mut place = sqlx::query_as::<_, PlaceView>(
        r#"
        SELECT place.entity_id AS id,
               entity.name,
               entity.description,
               place.is_entry,
               place.latest_activity_id,
               entity.introduced_at
        FROM place
        JOIN entity ON entity.id = place.entity_id
        WHERE ($1::uuid IS NULL OR place.entity_id < $1)
        ORDER BY place.entity_id DESC
        LIMIT $2
        "#,
    )
    .bind(query.before)
    .bind(i64::try_from(limit + 1).expect("Studio page limit fits i64"))
    .fetch_all(&state.pool)
    .await?;
    let next_cursor = page_cursor(&mut place, limit, |item| item.id);
    Ok(Json(PlacePage { place, next_cursor }))
}

fn page_cursor<T>(item: &mut Vec<T>, limit: usize, id: impl Fn(&T) -> Uuid) -> Option<Uuid> {
    if item.len() <= limit {
        return None;
    }
    item.truncate(limit);
    item.last().map(id)
}

#[derive(Clone, Serialize, sqlx::FromRow)]
pub(super) struct StorageColumn {
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
pub(super) struct StorageConstraint {
    table: String,
    name: String,
    kind: String,
    columns: Vec<String>,
    definition: String,
}

#[derive(Clone, Serialize, sqlx::FromRow)]
pub(super) struct StorageRelation {
    name: String,
    table: String,
    columns: Vec<String>,
    referenced_table: String,
    referenced_columns: Vec<String>,
    on_update: String,
    on_delete: String,
}

#[derive(Clone, Serialize, sqlx::FromRow)]
pub(super) struct StorageIndex {
    table: String,
    name: String,
    definition: String,
    is_unique: bool,
    is_primary: bool,
}

#[derive(Clone, Serialize)]
pub(super) struct StorageTable {
    name: String,
    column: Vec<StorageColumn>,
    constraint: Vec<StorageConstraint>,
    index: Vec<StorageIndex>,
}

#[derive(Serialize)]
pub(super) struct StorageSnapshot {
    captured_at: DateTime<Utc>,
    latest_migration: Option<i64>,
    fingerprint: String,
    schema: &'static str,
    table: Vec<StorageTable>,
    relation: Vec<StorageRelation>,
}

#[derive(Serialize)]
struct StorageFingerprint<'a> {
    schema: &'static str,
    table: &'a [StorageTable],
    relation: &'a [StorageRelation],
}

pub(super) async fn get_storage(
    State(state): State<StudioState>,
) -> Result<Json<StorageSnapshot>, StudioError> {
    Ok(Json(read_storage(&state.pool).await?))
}

pub(super) async fn download_storage(
    State(state): State<StudioState>,
) -> Result<Response, StudioError> {
    let snapshot = read_storage(&state.pool).await?;
    let body = serde_json::to_string_pretty(&snapshot)
        .expect("the serializable Studio schema snapshot must encode as JSON");
    Ok((
        [
            (header::CONTENT_TYPE, "application/json; charset=utf-8"),
            (
                header::CONTENT_DISPOSITION,
                "attachment; filename=\"aicadia-schema-snapshot.json\"",
            ),
        ],
        body,
    )
        .into_response())
}

async fn read_storage(pool: &sqlx::PgPool) -> Result<StorageSnapshot, StudioError> {
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

    let latest_migration = sqlx::query_scalar::<_, Option<i64>>(
        "SELECT MAX(version) FROM _sqlx_migrations WHERE success",
    )
    .fetch_one(pool)
    .await?;
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

fn ensure_schema_bound(
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
    fn live_page_limits_are_hard_bounded() {
        assert_eq!(
            PageQuery {
                limit: None,
                before: None
            }
            .limit()
            .unwrap(),
            24
        );
        assert!(matches!(
            PageQuery {
                limit: Some(0),
                before: None
            }
            .limit(),
            Err(StudioError::InvalidLimit)
        ));
        assert!(matches!(
            PageQuery {
                limit: Some(101),
                before: None
            }
            .limit(),
            Err(StudioError::InvalidLimit)
        ));
        assert_eq!(
            PageQuery {
                limit: Some(100),
                before: None
            }
            .limit()
            .unwrap(),
            100
        );
    }

    #[test]
    fn keyset_cursor_is_the_last_visible_item_only_when_more_exist() {
        let mut id = (0..3).map(|_| Uuid::new_v4()).collect::<Vec<_>>();
        let expected = id[1];
        assert_eq!(page_cursor(&mut id, 2, |value| *value), Some(expected));
        assert_eq!(id.len(), 2);
    }

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
