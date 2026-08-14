use super::*;

pub(super) const INSERT_PROPERTY_KEY_SQL: &str = r#"
    INSERT INTO property_key (key, value_type, first_activity_id)
    SELECT submitted.key, submitted.value_type, $3
    FROM UNNEST($1::text[], $2::text[]) AS submitted(key, value_type)
    ORDER BY submitted.key
    ON CONFLICT (key) DO NOTHING
"#;

pub(super) const LOCK_CURRENT_ENTITY_PROPERTY_SQL: &str = r#"
    SELECT current.entity_id, current.property_key_id, current.current_activity_id
    FROM entity_property AS current
    WHERE (current.entity_id, current.property_key_id) IN (
        SELECT submitted.entity_id, submitted.property_key_id
        FROM UNNEST($1::uuid[], $2::bigint[])
            AS submitted(entity_id, property_key_id)
    )
    ORDER BY current.entity_id, current.property_key_id
    FOR UPDATE
"#;

pub(super) const INSERT_ENTITY_PROPERTY_HISTORY_SQL: &str = r#"
    INSERT INTO entity_property_history (
        entity_id, property_key_id, activity_id, previous_activity_id,
        value_type, text_value, integer_value
    )
    SELECT submitted.entity_id, submitted.property_key_id, $3,
           submitted.previous_activity_id, submitted.value_type,
           submitted.text_value, submitted.integer_value
    FROM UNNEST(
        $1::uuid[], $2::bigint[], $4::uuid[], $5::text[], $6::text[], $7::bigint[]
    ) AS submitted(
        entity_id, property_key_id, previous_activity_id,
        value_type, text_value, integer_value
    )
    ORDER BY submitted.entity_id, submitted.property_key_id
"#;

pub(super) const UPSERT_CURRENT_ENTITY_PROPERTY_SQL: &str = r#"
    INSERT INTO entity_property (entity_id, property_key_id, current_activity_id)
    SELECT submitted.entity_id, submitted.property_key_id, $3
    FROM UNNEST($1::uuid[], $2::bigint[]) AS submitted(entity_id, property_key_id)
    ORDER BY submitted.entity_id, submitted.property_key_id
    ON CONFLICT (entity_id, property_key_id) DO UPDATE
    SET current_activity_id = EXCLUDED.current_activity_id
"#;

pub(super) const HYDRATE_ENTITY_PROPERTY_CHANGE_SQL: &str = r#"
    SELECT history.activity_id, history.entity_id, entity.name AS entity_name,
           property_key.key, history.value_type,
           history.text_value, history.integer_value
    FROM entity_property_history AS history
    JOIN entity ON entity.id = history.entity_id
    JOIN property_key ON property_key.id = history.property_key_id
    WHERE history.activity_id = ANY($1::uuid[])
    ORDER BY history.activity_id, history.entity_id, property_key.key
"#;

const MAX_PROPERTY_COUNT: usize = 100;
const MAX_PROPERTY_KEY_LENGTH: usize = 64;
const MAX_PROPERTY_TEXT_LENGTH: usize = 4_000;
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PropertyValue {
    Text(String),
    Integer(i64),
}

impl PropertyValue {
    fn value_type(&self) -> &'static str {
        match self {
            Self::Text(_) => "text",
            Self::Integer(_) => "integer",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct PropertyWrite {
    pub(super) entity_id: EntityId,
    pub(super) key: String,
    pub(super) value: PropertyValue,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct StoredPropertyChange {
    entity_id: EntityId,
    entity_name: String,
    key: String,
    value: PropertyValue,
}

impl From<StoredPropertyChange> for EntityPropertyChange {
    fn from(value: StoredPropertyChange) -> Self {
        Self {
            entity: EntitySummary {
                id: value.entity_id,
                name: value.entity_name,
            },
            key: value.key,
            value: value.value,
        }
    }
}

#[derive(Debug)]
pub(super) enum PropertyPersistenceError {
    KeyConflict,
    InvalidStoredRelation,
    Storage(Box<sqlx::Error>),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum PropertyNormalizationError {
    InvalidCount,
    InvalidKey,
    InvalidText(InvalidReason),
    DuplicateEntityKey,
}

trait NormalizableProperty {
    fn entity_id(&self) -> Option<EntityId>;
    fn key(&self) -> &str;
    fn value_mut(&mut self) -> &mut PropertyValue;
}

impl NormalizableProperty for PropertyInput {
    fn entity_id(&self) -> Option<EntityId> {
        None
    }

    fn key(&self) -> &str {
        &self.key
    }

    fn value_mut(&mut self) -> &mut PropertyValue {
        &mut self.value
    }
}

impl NormalizableProperty for PropertyWrite {
    fn entity_id(&self) -> Option<EntityId> {
        Some(self.entity_id)
    }

    fn key(&self) -> &str {
        &self.key
    }

    fn value_mut(&mut self) -> &mut PropertyValue {
        &mut self.value
    }
}

fn property_storage_error(error: sqlx::Error) -> PropertyPersistenceError {
    PropertyPersistenceError::Storage(Box::new(error))
}

pub(super) fn property_writes_for_entity(
    entity_id: EntityId,
    property: Vec<PropertyInput>,
) -> Vec<PropertyWrite> {
    property
        .into_iter()
        .map(|property| PropertyWrite {
            entity_id,
            key: property.key,
            value: property.value,
        })
        .collect()
}

pub(super) fn map_property_error(
    error: PropertyPersistenceError,
    operation: &'static str,
) -> WorldError {
    match error {
        PropertyPersistenceError::KeyConflict => WorldError::PropertyKeyConflict,
        PropertyPersistenceError::InvalidStoredRelation => invalid_stored_relation(),
        PropertyPersistenceError::Storage(error) => storage_error(operation, *error),
    }
}

pub(super) fn map_property_normalization_error(
    error: PropertyNormalizationError,
    field: PropertyField,
) -> WorldError {
    match error {
        PropertyNormalizationError::InvalidCount => WorldError::InvalidProperty {
            field,
            reason: InvalidReason::OutOfRange,
        },
        PropertyNormalizationError::InvalidKey => WorldError::InvalidProperty {
            field: PropertyField::Key,
            reason: InvalidReason::InvalidFormat,
        },
        PropertyNormalizationError::InvalidText(reason) => WorldError::InvalidProperty {
            field: PropertyField::Value,
            reason,
        },
        PropertyNormalizationError::DuplicateEntityKey => WorldError::InvalidProperty {
            field,
            reason: InvalidReason::Duplicate,
        },
    }
}

pub(super) fn normalize_property_input(
    property: Vec<PropertyInput>,
    field: PropertyField,
) -> Result<Vec<PropertyInput>, WorldError> {
    normalize_property(property, true)
        .map_err(|error| map_property_normalization_error(error, field))
}

pub(super) fn normalize_property_writes(
    writes: Vec<PropertyWrite>,
    allow_empty: bool,
) -> Result<Vec<PropertyWrite>, PropertyNormalizationError> {
    normalize_property(writes, allow_empty)
}

fn normalize_property<T: NormalizableProperty>(
    mut property: Vec<T>,
    allow_empty: bool,
) -> Result<Vec<T>, PropertyNormalizationError> {
    if property.len() > MAX_PROPERTY_COUNT || (!allow_empty && property.is_empty()) {
        return Err(PropertyNormalizationError::InvalidCount);
    }
    for item in &mut property {
        if !is_canonical_property_key(item.key()) {
            return Err(PropertyNormalizationError::InvalidKey);
        }
        if let PropertyValue::Text(value) = item.value_mut() {
            *value = value.trim().to_owned();
            let reason = if value.is_empty() {
                Some(InvalidReason::Empty)
            } else if value.contains('\0') {
                Some(InvalidReason::ContainsNul)
            } else if value.chars().count() > MAX_PROPERTY_TEXT_LENGTH {
                Some(InvalidReason::TooLong)
            } else {
                None
            };
            if let Some(reason) = reason {
                return Err(PropertyNormalizationError::InvalidText(reason));
            }
        }
    }
    property.sort_unstable_by(|left, right| {
        match (left.entity_id(), right.entity_id()) {
            (Some(left), Some(right)) => left.0.as_bytes().cmp(right.0.as_bytes()),
            (None, None) => std::cmp::Ordering::Equal,
            (None, Some(_)) => std::cmp::Ordering::Less,
            (Some(_), None) => std::cmp::Ordering::Greater,
        }
        .then_with(|| left.key().cmp(right.key()))
    });
    if property
        .windows(2)
        .any(|pair| pair[0].entity_id() == pair[1].entity_id() && pair[0].key() == pair[1].key())
    {
        return Err(PropertyNormalizationError::DuplicateEntityKey);
    }
    Ok(property)
}

fn is_canonical_property_key(key: &str) -> bool {
    let bytes = key.as_bytes();
    if bytes.is_empty() || bytes.len() > MAX_PROPERTY_KEY_LENGTH || !bytes[0].is_ascii_lowercase() {
        return false;
    }
    let mut previous_underscore = false;
    for byte in bytes {
        if *byte == b'_' {
            if previous_underscore {
                return false;
            }
            previous_underscore = true;
        } else if byte.is_ascii_lowercase() || byte.is_ascii_digit() {
            previous_underscore = false;
        } else {
            return false;
        }
    }
    !previous_underscore
}

#[derive(FromRow)]
struct PropertyKeyRow {
    id: i64,
    key: String,
    value_type: String,
}

async fn resolve_property_keys(
    transaction: &mut Transaction<'_, Postgres>,
    first_activity_id: ActivityId,
    writes: &[PropertyWrite],
) -> Result<HashMap<String, i64>, PropertyPersistenceError> {
    let mut requested_type = BTreeMap::<String, &'static str>::new();
    for write in writes {
        match requested_type.get(&write.key) {
            Some(value_type) if *value_type != write.value.value_type() => {
                return Err(PropertyPersistenceError::KeyConflict);
            }
            Some(_) => {}
            None => {
                requested_type.insert(write.key.clone(), write.value.value_type());
            }
        }
    }
    if requested_type.is_empty() {
        return Ok(HashMap::new());
    }
    let key = requested_type.keys().cloned().collect::<Vec<_>>();
    let value_type = requested_type.values().copied().collect::<Vec<_>>();
    record_property_query(PropertyQueryKind::Write);
    sqlx::query(INSERT_PROPERTY_KEY_SQL)
        .bind(&key)
        .bind(&value_type)
        .bind(first_activity_id.0)
        .execute(&mut **transaction)
        .await
        .map_err(property_storage_error)?;

    record_property_query(PropertyQueryKind::Write);
    let row = sqlx::query_as::<_, PropertyKeyRow>(
        r#"
        SELECT id, key, value_type
        FROM property_key
        WHERE key = ANY($1::text[])
        ORDER BY key
        FOR KEY SHARE
        "#,
    )
    .bind(&key)
    .fetch_all(&mut **transaction)
    .await
    .map_err(property_storage_error)?;
    if row.len() != requested_type.len() {
        return Err(PropertyPersistenceError::InvalidStoredRelation);
    }
    let mut resolved = HashMap::with_capacity(row.len());
    for row in row {
        if requested_type.get(&row.key).copied() != Some(row.value_type.as_str()) {
            return Err(PropertyPersistenceError::KeyConflict);
        }
        resolved.insert(row.key, row.id);
    }
    Ok(resolved)
}

pub(super) async fn write_property_changes(
    transaction: &mut Transaction<'_, Postgres>,
    activity_id: ActivityId,
    writes: &[PropertyWrite],
) -> Result<Vec<StoredPropertyChange>, PropertyPersistenceError> {
    if writes.is_empty() {
        return Ok(Vec::new());
    }
    let key_id = resolve_property_keys(transaction, activity_id, writes).await?;
    let mut resolved = writes
        .iter()
        .map(|write| {
            let property_key_id = key_id
                .get(&write.key)
                .copied()
                .ok_or(PropertyPersistenceError::InvalidStoredRelation)?;
            Ok((write, property_key_id))
        })
        .collect::<Result<Vec<_>, PropertyPersistenceError>>()?;
    resolved.sort_unstable_by(|(left, left_key_id), (right, right_key_id)| {
        left.entity_id
            .0
            .as_bytes()
            .cmp(right.entity_id.0.as_bytes())
            .then_with(|| left_key_id.cmp(right_key_id))
    });

    let entity_id = resolved
        .iter()
        .map(|(write, _)| write.entity_id.0)
        .collect::<Vec<_>>();
    let property_key_id = resolved
        .iter()
        .map(|(_, property_key_id)| *property_key_id)
        .collect::<Vec<_>>();
    record_property_query(PropertyQueryKind::Write);
    let existing = sqlx::query_as::<_, (Uuid, i64, Uuid)>(LOCK_CURRENT_ENTITY_PROPERTY_SQL)
        .bind(&entity_id)
        .bind(&property_key_id)
        .fetch_all(&mut **transaction)
        .await
        .map_err(property_storage_error)?;
    let previous = existing
        .into_iter()
        .map(|(entity_id, property_key_id, activity_id)| {
            ((entity_id, property_key_id), activity_id)
        })
        .collect::<HashMap<_, _>>();

    let previous_activity_id = resolved
        .iter()
        .map(|(write, property_key_id)| {
            previous
                .get(&(write.entity_id.0, *property_key_id))
                .copied()
        })
        .collect::<Vec<_>>();
    let value_type = resolved
        .iter()
        .map(|(write, _)| write.value.value_type())
        .collect::<Vec<_>>();
    let text_value = resolved
        .iter()
        .map(|(write, _)| match &write.value {
            PropertyValue::Text(value) => Some(value.as_str()),
            PropertyValue::Integer(_) => None,
        })
        .collect::<Vec<_>>();
    let integer_value = resolved
        .iter()
        .map(|(write, _)| match &write.value {
            PropertyValue::Text(_) => None,
            PropertyValue::Integer(value) => Some(*value),
        })
        .collect::<Vec<_>>();
    record_property_query(PropertyQueryKind::Write);
    sqlx::query(INSERT_ENTITY_PROPERTY_HISTORY_SQL)
        .bind(&entity_id)
        .bind(&property_key_id)
        .bind(activity_id.0)
        .bind(&previous_activity_id)
        .bind(&value_type)
        .bind(&text_value)
        .bind(&integer_value)
        .execute(&mut **transaction)
        .await
        .map_err(property_storage_error)?;
    record_property_query(PropertyQueryKind::Write);
    sqlx::query(UPSERT_CURRENT_ENTITY_PROPERTY_SQL)
        .bind(&entity_id)
        .bind(&property_key_id)
        .bind(activity_id.0)
        .execute(&mut **transaction)
        .await
        .map_err(property_storage_error)?;
    hydrate_property_changes(transaction, &[activity_id])
        .await
        .map(|mut change| change.remove(&activity_id).unwrap_or_default())
}

#[derive(FromRow)]
struct StoredPropertyChangeRow {
    activity_id: ActivityId,
    entity_id: EntityId,
    entity_name: String,
    key: String,
    value_type: String,
    text_value: Option<String>,
    integer_value: Option<i64>,
}

pub(super) async fn hydrate_property_changes(
    transaction: &mut Transaction<'_, Postgres>,
    activity_id: &[ActivityId],
) -> Result<HashMap<ActivityId, Vec<StoredPropertyChange>>, PropertyPersistenceError> {
    if activity_id.is_empty() {
        return Ok(HashMap::new());
    }
    let activity_uuid = activity_id.iter().map(|id| id.0).collect::<Vec<_>>();
    record_property_query(PropertyQueryKind::Hydration);
    let row = sqlx::query_as::<_, StoredPropertyChangeRow>(HYDRATE_ENTITY_PROPERTY_CHANGE_SQL)
        .bind(&activity_uuid)
        .fetch_all(&mut **transaction)
        .await
        .map_err(property_storage_error)?;
    let mut change = HashMap::<ActivityId, Vec<StoredPropertyChange>>::new();
    for row in row {
        let value = match (row.value_type.as_str(), row.text_value, row.integer_value) {
            ("text", Some(value), None) => PropertyValue::Text(value),
            ("integer", None, Some(value)) => PropertyValue::Integer(value),
            _ => return Err(PropertyPersistenceError::InvalidStoredRelation),
        };
        change
            .entry(row.activity_id)
            .or_default()
            .push(StoredPropertyChange {
                entity_id: row.entity_id,
                entity_name: row.entity_name,
                key: row.key,
                value,
            });
    }
    Ok(change)
}
