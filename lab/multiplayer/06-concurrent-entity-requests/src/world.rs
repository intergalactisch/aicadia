use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use sqlx::{FromRow, PgPool, Postgres, Transaction, types::Json};
use thiserror::Error;
use uuid::Uuid;

pub const MAX_DEPENDENCIES: usize = 16;
pub const MAX_WRITES: usize = 16;
pub const MAX_KEY_BYTES: usize = 64;
pub const MAX_VALUE_BYTES: usize = 4_096;
pub const MAX_OPERATION_BYTES: usize = 128;
pub const MAX_REQUEST_BYTES: usize = 65_536;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Request {
    pub id: Uuid,
    pub actor_character_id: Uuid,
    pub expected_place_id: Uuid,
    pub expected_placement_version: i64,
    pub operation: String,
    pub dependencies: Vec<Dependency>,
    pub writes: Vec<PropertyWrite>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Dependency {
    pub entity_id: Uuid,
    pub property_key: String,
    pub expected: ExpectedValue,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "state", rename_all = "snake_case")]
pub enum ExpectedValue {
    Absent,
    Current { version: i64, value: Value },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PropertyWrite {
    pub entity_id: Uuid,
    pub property_key: String,
    pub value: Value,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AcceptedResult {
    pub activity_id: Uuid,
    pub facts: Vec<FactVersion>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FactVersion {
    pub entity_id: Uuid,
    pub property_key: String,
    pub version: i64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Outcome {
    Accepted {
        replayed: bool,
        result: AcceptedResult,
    },
    Conflict(Conflict),
    Busy,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Conflict {
    PlacementChanged,
    SubjectUnavailable,
    DependencyChanged {
        entity_id: Uuid,
        property_key: String,
    },
    RequestChanged,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Admission {
    Wait,
    Nowait,
    LockTimeoutMillis(u64),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FaultInjection {
    None,
    AfterActivityInsert,
}

#[derive(Debug, Error)]
pub enum WorldError {
    #[error("invalid request: {0}")]
    Invalid(String),
    #[error("injected failure after Activity insertion")]
    InjectedFailure,
    #[error(transparent)]
    Database(#[from] sqlx::Error),
}

#[derive(Clone)]
pub struct World {
    pool: PgPool,
}

#[derive(Debug, FromRow)]
struct SlotRow {
    entity_id: Uuid,
    property_key: String,
    current_version: Option<i64>,
    current_value: Option<Json<Value>>,
}

type FactKey = (Uuid, String);

impl World {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn apply(
        &self,
        request: Request,
        admission: Admission,
        fault: FaultInjection,
    ) -> Result<Outcome, WorldError> {
        let request = normalize_and_validate(request, admission)?;
        let encoded = serde_json::to_vec(&request).map_err(|error| {
            WorldError::Invalid(format!("request is not serializable: {error}"))
        })?;
        if encoded.len() > MAX_REQUEST_BYTES {
            return Err(WorldError::Invalid(format!(
                "serialized request is {} bytes; maximum is {MAX_REQUEST_BYTES}",
                encoded.len()
            )));
        }
        let fingerprint = Sha256::digest(&encoded).to_vec();
        let mut transaction = self.pool.begin().await?;

        configure_transaction(&mut transaction, admission).await?;
        let request_lock = request_advisory_key(request.actor_character_id, request.id);
        if let Err(error) = sqlx::query("SELECT pg_advisory_xact_lock($1)")
            .bind(request_lock)
            .execute(&mut *transaction)
            .await
        {
            return finish_database_error(transaction, error).await;
        }

        if let Some((stored_fingerprint, stored_result)) =
            sqlx::query_as::<_, (Vec<u8>, Json<AcceptedResult>)>(
                r#"
                SELECT fingerprint, result
                FROM accepted_request
                WHERE actor_character_id = $1 AND request_id = $2
                "#,
            )
            .bind(request.actor_character_id)
            .bind(request.id)
            .fetch_optional(&mut *transaction)
            .await?
        {
            transaction.rollback().await?;
            if stored_fingerprint == fingerprint {
                return Ok(Outcome::Accepted {
                    replayed: true,
                    result: stored_result.0,
                });
            }
            return Ok(Outcome::Conflict(Conflict::RequestChanged));
        }

        let placement = sqlx::query_as::<_, (Uuid, i64)>(
            r#"
            SELECT place_id, placement_version
            FROM character
            WHERE id = $1
            FOR KEY SHARE
            "#,
        )
        .bind(request.actor_character_id)
        .fetch_optional(&mut *transaction)
        .await?;
        let Some((place_id, placement_version)) = placement else {
            transaction.rollback().await?;
            return Ok(Outcome::Conflict(Conflict::SubjectUnavailable));
        };
        if place_id != request.expected_place_id
            || placement_version != request.expected_placement_version
        {
            transaction.rollback().await?;
            return Ok(Outcome::Conflict(Conflict::PlacementChanged));
        }

        let entity_ids = request
            .dependencies
            .iter()
            .map(|dependency| dependency.entity_id)
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        let entity_places = sqlx::query_as::<_, (Uuid, Uuid)>(
            "SELECT id, place_id FROM entity WHERE id = ANY($1::uuid[]) ORDER BY id",
        )
        .bind(&entity_ids)
        .fetch_all(&mut *transaction)
        .await?;
        if entity_places.len() != entity_ids.len()
            || entity_places
                .iter()
                .any(|(_, entity_place_id)| *entity_place_id != place_id)
        {
            transaction.rollback().await?;
            return Ok(Outcome::Conflict(Conflict::SubjectUnavailable));
        }

        let dependency_entity_ids = request
            .dependencies
            .iter()
            .map(|dependency| dependency.entity_id)
            .collect::<Vec<_>>();
        let dependency_keys = request
            .dependencies
            .iter()
            .map(|dependency| dependency.property_key.as_str())
            .collect::<Vec<_>>();
        let materialized = sqlx::query(
            r#"
            INSERT INTO property_slot (entity_id, property_key)
            SELECT submitted.entity_id, submitted.property_key
            FROM UNNEST($1::uuid[], $2::text[])
                AS submitted(entity_id, property_key)
            ORDER BY submitted.entity_id, submitted.property_key
            ON CONFLICT (entity_id, property_key) DO NOTHING
            "#,
        )
        .bind(&dependency_entity_ids)
        .bind(&dependency_keys)
        .execute(&mut *transaction)
        .await;
        if let Err(error) = materialized {
            return finish_database_error(transaction, error).await;
        }

        let lock_sql = match admission {
            Admission::Nowait => {
                r#"
                SELECT entity_id, property_key, current_version, current_value
                FROM property_slot
                WHERE (entity_id, property_key) IN (
                    SELECT submitted.entity_id, submitted.property_key
                    FROM UNNEST($1::uuid[], $2::text[])
                        AS submitted(entity_id, property_key)
                )
                ORDER BY entity_id, property_key
                FOR UPDATE NOWAIT
                "#
            }
            Admission::Wait | Admission::LockTimeoutMillis(_) => {
                r#"
                SELECT entity_id, property_key, current_version, current_value
                FROM property_slot
                WHERE (entity_id, property_key) IN (
                    SELECT submitted.entity_id, submitted.property_key
                    FROM UNNEST($1::uuid[], $2::text[])
                        AS submitted(entity_id, property_key)
                )
                ORDER BY entity_id, property_key
                FOR UPDATE
                "#
            }
        };
        let locked = sqlx::query_as::<_, SlotRow>(lock_sql)
            .bind(&dependency_entity_ids)
            .bind(&dependency_keys)
            .fetch_all(&mut *transaction)
            .await;
        let locked = match locked {
            Ok(locked) => locked,
            Err(error) => return finish_database_error(transaction, error).await,
        };
        if locked.len() != request.dependencies.len() {
            transaction.rollback().await?;
            return Ok(Outcome::Conflict(Conflict::SubjectUnavailable));
        }
        let current = locked
            .into_iter()
            .map(|row| {
                (
                    (row.entity_id, row.property_key),
                    (row.current_version, row.current_value.map(|value| value.0)),
                )
            })
            .collect::<BTreeMap<_, _>>();

        for dependency in &request.dependencies {
            let key = (dependency.entity_id, dependency.property_key.clone());
            let (actual_version, actual_value) = current
                .get(&key)
                .expect("all canonical dependency slots were locked");
            let matches = match &dependency.expected {
                ExpectedValue::Absent => actual_version.is_none() && actual_value.is_none(),
                ExpectedValue::Current { version, value } => {
                    actual_version == &Some(*version) && actual_value.as_ref() == Some(value)
                }
            };
            if !matches {
                transaction.rollback().await?;
                return Ok(Outcome::Conflict(Conflict::DependencyChanged {
                    entity_id: dependency.entity_id,
                    property_key: dependency.property_key.clone(),
                }));
            }
        }

        let activity_id = Uuid::new_v4();
        sqlx::query(
            r#"
            INSERT INTO activity (id, actor_character_id, place_id, operation)
            VALUES ($1, $2, $3, $4)
            "#,
        )
        .bind(activity_id)
        .bind(request.actor_character_id)
        .bind(place_id)
        .bind(&request.operation)
        .execute(&mut *transaction)
        .await?;

        if fault == FaultInjection::AfterActivityInsert {
            transaction.rollback().await?;
            return Err(WorldError::InjectedFailure);
        }

        for (ordinal, dependency) in request.dependencies.iter().enumerate() {
            let (expected_version, expected_value) = match &dependency.expected {
                ExpectedValue::Absent => (None, None),
                ExpectedValue::Current { version, value } => {
                    (Some(*version), Some(Json(value.clone())))
                }
            };
            sqlx::query(
                r#"
                INSERT INTO activity_dependency (
                    activity_id,
                    ordinal,
                    entity_id,
                    property_key,
                    expected_version,
                    expected_value
                )
                VALUES ($1, $2, $3, $4, $5, $6)
                "#,
            )
            .bind(activity_id)
            .bind(i16::try_from(ordinal).expect("dependency bound fits smallint"))
            .bind(dependency.entity_id)
            .bind(&dependency.property_key)
            .bind(expected_version)
            .bind(expected_value)
            .execute(&mut *transaction)
            .await?;
        }

        let mut facts = Vec::with_capacity(request.writes.len());
        for write in &request.writes {
            let key: FactKey = (write.entity_id, write.property_key.clone());
            let previous_version = current
                .get(&key)
                .expect("every write has a dependency")
                .0
                .unwrap_or(0);
            let version = previous_version + 1;
            sqlx::query(
                r#"
                INSERT INTO property_history (
                    entity_id, property_key, version, activity_id, value
                )
                VALUES ($1, $2, $3, $4, $5)
                "#,
            )
            .bind(write.entity_id)
            .bind(&write.property_key)
            .bind(version)
            .bind(activity_id)
            .bind(Json(write.value.clone()))
            .execute(&mut *transaction)
            .await?;
            sqlx::query(
                r#"
                UPDATE property_slot
                SET current_version = $3,
                    current_value = $4,
                    current_activity_id = $5
                WHERE entity_id = $1 AND property_key = $2
                "#,
            )
            .bind(write.entity_id)
            .bind(&write.property_key)
            .bind(version)
            .bind(Json(write.value.clone()))
            .bind(activity_id)
            .execute(&mut *transaction)
            .await?;
            facts.push(FactVersion {
                entity_id: write.entity_id,
                property_key: write.property_key.clone(),
                version,
            });
        }
        let result = AcceptedResult { activity_id, facts };
        sqlx::query(
            r#"
            INSERT INTO accepted_request (
                actor_character_id, request_id, fingerprint, activity_id, result
            )
            VALUES ($1, $2, $3, $4, $5)
            "#,
        )
        .bind(request.actor_character_id)
        .bind(request.id)
        .bind(&fingerprint)
        .bind(activity_id)
        .bind(Json(result.clone()))
        .execute(&mut *transaction)
        .await?;
        transaction.commit().await?;

        Ok(Outcome::Accepted {
            replayed: false,
            result,
        })
    }
}

fn normalize_and_validate(
    mut request: Request,
    admission: Admission,
) -> Result<Request, WorldError> {
    if request.expected_placement_version <= 0 {
        return Err(WorldError::Invalid(
            "expected placement version must be positive".to_owned(),
        ));
    }
    if request.operation.is_empty() || request.operation.len() > MAX_OPERATION_BYTES {
        return Err(WorldError::Invalid(format!(
            "operation must contain 1..={MAX_OPERATION_BYTES} bytes"
        )));
    }
    if request.dependencies.is_empty() || request.dependencies.len() > MAX_DEPENDENCIES {
        return Err(WorldError::Invalid(format!(
            "dependencies must contain 1..={MAX_DEPENDENCIES} facts"
        )));
    }
    if request.writes.is_empty() || request.writes.len() > MAX_WRITES {
        return Err(WorldError::Invalid(format!(
            "writes must contain 1..={MAX_WRITES} facts"
        )));
    }
    if let Admission::LockTimeoutMillis(milliseconds) = admission
        && !(1..=1_000).contains(&milliseconds)
    {
        return Err(WorldError::Invalid(
            "lock timeout must be between 1 and 1000 milliseconds".to_owned(),
        ));
    }

    request.dependencies.sort_unstable_by(compare_dependency);
    request.writes.sort_unstable_by(compare_write);
    let mut dependency_keys = BTreeSet::new();
    for dependency in &request.dependencies {
        validate_key(&dependency.property_key)?;
        if let ExpectedValue::Current { version, value } = &dependency.expected {
            if *version <= 0 {
                return Err(WorldError::Invalid(
                    "current dependency version must be positive".to_owned(),
                ));
            }
            validate_value(value)?;
        }
        if !dependency_keys.insert((dependency.entity_id, dependency.property_key.clone())) {
            return Err(WorldError::Invalid(
                "dependency facts must be unique".to_owned(),
            ));
        }
    }
    let mut write_keys = BTreeSet::new();
    for write in &request.writes {
        validate_key(&write.property_key)?;
        validate_value(&write.value)?;
        let key = (write.entity_id, write.property_key.clone());
        if !write_keys.insert(key.clone()) {
            return Err(WorldError::Invalid("write facts must be unique".to_owned()));
        }
        if !dependency_keys.contains(&key) {
            return Err(WorldError::Invalid(format!(
                "write {}:{} has no exact dependency",
                write.entity_id, write.property_key
            )));
        }
    }
    Ok(request)
}

fn compare_dependency(left: &Dependency, right: &Dependency) -> std::cmp::Ordering {
    left.entity_id
        .as_bytes()
        .cmp(right.entity_id.as_bytes())
        .then_with(|| left.property_key.cmp(&right.property_key))
}

fn compare_write(left: &PropertyWrite, right: &PropertyWrite) -> std::cmp::Ordering {
    left.entity_id
        .as_bytes()
        .cmp(right.entity_id.as_bytes())
        .then_with(|| left.property_key.cmp(&right.property_key))
}

fn validate_key(key: &str) -> Result<(), WorldError> {
    let bytes = key.as_bytes();
    let valid = !bytes.is_empty()
        && bytes.len() <= MAX_KEY_BYTES
        && bytes[0].is_ascii_lowercase()
        && bytes
            .iter()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || *byte == b'_')
        && !key.ends_with('_')
        && !key.contains("__");
    if !valid {
        return Err(WorldError::Invalid(format!(
            "Property key must be 1..={MAX_KEY_BYTES} bytes of lower_snake_case"
        )));
    }
    Ok(())
}

fn validate_value(value: &Value) -> Result<(), WorldError> {
    let encoded = serde_json::to_vec(value)
        .map_err(|error| WorldError::Invalid(format!("value is not serializable: {error}")))?;
    if encoded.len() > MAX_VALUE_BYTES {
        return Err(WorldError::Invalid(format!(
            "Property value is {} bytes; maximum is {MAX_VALUE_BYTES}",
            encoded.len()
        )));
    }
    Ok(())
}

async fn configure_transaction(
    transaction: &mut Transaction<'_, Postgres>,
    admission: Admission,
) -> Result<(), sqlx::Error> {
    sqlx::query("SELECT set_config('statement_timeout', '5s', true)")
        .execute(&mut **transaction)
        .await?;
    let lock_timeout = match admission {
        Admission::Wait => "5s".to_owned(),
        Admission::Nowait => "1ms".to_owned(),
        Admission::LockTimeoutMillis(milliseconds) => format!("{milliseconds}ms"),
    };
    sqlx::query("SELECT set_config('lock_timeout', $1, true)")
        .bind(lock_timeout)
        .execute(&mut **transaction)
        .await?;
    Ok(())
}

fn request_advisory_key(actor_character_id: Uuid, request_id: Uuid) -> i64 {
    let mut hasher = Sha256::new();
    hasher.update(actor_character_id.as_bytes());
    hasher.update(request_id.as_bytes());
    let digest = hasher.finalize();
    i64::from_be_bytes(
        digest[..8]
            .try_into()
            .expect("SHA-256 digest always contains eight bytes"),
    )
}

async fn finish_database_error(
    transaction: Transaction<'_, Postgres>,
    error: sqlx::Error,
) -> Result<Outcome, WorldError> {
    let busy = is_busy_error(&error);
    transaction.rollback().await?;
    if busy {
        Ok(Outcome::Busy)
    } else {
        Err(WorldError::Database(error))
    }
}

fn is_busy_error(error: &sqlx::Error) -> bool {
    error
        .as_database_error()
        .and_then(|database_error| database_error.code())
        .is_some_and(|code| code == "55P03" || code == "57014")
}
