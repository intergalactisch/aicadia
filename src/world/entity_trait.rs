use super::*;

const FIND_ENTITY_TRAIT_OWNER_SQL: &str = r#"
    SELECT id AS trait_id, entity_id
    FROM entity_trait
    WHERE id = ANY($1::uuid[])
    ORDER BY id
"#;

const LOCK_TRAIT_ENTITY_SQL: &str = r#"
    SELECT entity.id
    FROM UNNEST($1::uuid[]) AS submitted(entity_id)
    JOIN entity ON entity.id = submitted.entity_id
    ORDER BY entity.id
    FOR UPDATE OF entity
"#;

pub(super) const LOCK_ACTIVE_ENTITY_TRAIT_STATEMENT_SQL: &str = r#"
    SELECT current.entity_id, current.trait_id, version.statement
    FROM UNNEST($1::uuid[], $2::text[]) AS submitted(entity_id, statement)
    JOIN entity_trait_current AS current
      ON current.entity_id = submitted.entity_id
    JOIN entity_trait_version AS version
      ON version.trait_id = current.trait_id
     AND version.entity_id = current.entity_id
     AND version.activity_id = current.current_activity_id
     AND version.statement = submitted.statement
    WHERE NOT (current.trait_id = ANY($3::uuid[]))
    ORDER BY current.entity_id, current.trait_id
    LIMIT 1
    FOR UPDATE OF current
"#;

const LOCK_CURRENT_ENTITY_TRAIT_SQL: &str = r#"
    SELECT current.trait_id, current.entity_id,
           current.current_activity_id, version.statement
    FROM entity_trait_current AS current
    JOIN entity_trait_version AS version
      ON version.trait_id = current.trait_id
     AND version.entity_id = current.entity_id
     AND version.activity_id = current.current_activity_id
    WHERE current.trait_id = ANY($1::uuid[])
    ORDER BY current.trait_id
    FOR UPDATE OF current
"#;

const INSERT_ENTITY_TRAIT_SQL: &str = r#"
    INSERT INTO entity_trait (id, entity_id)
    SELECT submitted.trait_id, submitted.entity_id
    FROM UNNEST($1::uuid[], $2::uuid[]) AS submitted(trait_id, entity_id)
    ORDER BY submitted.entity_id, submitted.trait_id
"#;

const INSERT_ENTITY_TRAIT_VERSION_SQL: &str = r#"
    INSERT INTO entity_trait_version (
        trait_id, entity_id, activity_id, previous_activity_id, statement
    )
    SELECT submitted.trait_id, submitted.entity_id, $3,
           submitted.previous_activity_id, submitted.statement
    FROM UNNEST($1::uuid[], $2::uuid[], $4::uuid[], $5::text[])
        AS submitted(trait_id, entity_id, previous_activity_id, statement)
    ORDER BY submitted.entity_id, submitted.trait_id
"#;

const UPSERT_CURRENT_ENTITY_TRAIT_SQL: &str = r#"
    INSERT INTO entity_trait_current (trait_id, entity_id, current_activity_id)
    SELECT submitted.trait_id, submitted.entity_id, $3
    FROM UNNEST($1::uuid[], $2::uuid[]) AS submitted(trait_id, entity_id)
    ORDER BY submitted.entity_id, submitted.trait_id
    ON CONFLICT (trait_id) DO UPDATE
    SET current_activity_id = EXCLUDED.current_activity_id
"#;

const HYDRATE_ENTITY_TRAIT_CHANGE_SQL: &str = r#"
    SELECT version.activity_id, trait.id AS trait_id,
           trait.entity_id, entity.name AS entity_name,
           version.previous_activity_id, previous.statement AS previous_statement,
           version.statement
    FROM entity_trait_version AS version
    JOIN entity_trait AS trait ON trait.id = version.trait_id
    JOIN entity ON entity.id = trait.entity_id
    LEFT JOIN entity_trait_version AS previous
      ON previous.trait_id = version.trait_id
     AND previous.entity_id = version.entity_id
     AND previous.activity_id = version.previous_activity_id
    WHERE version.activity_id = ANY($1::uuid[])
    ORDER BY version.activity_id, trait.entity_id, trait.id
"#;

pub(super) const MAX_TRAIT_COUNT: usize = 100;
pub(super) const MAX_TRAIT_STATEMENT_LENGTH: usize = 4_000;
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) enum TraitWrite {
    Establish {
        entity_id: EntityId,
        statement: String,
    },
    Develop {
        trait_id: Uuid,
        statement: String,
    },
}

impl TraitWrite {
    pub(super) fn statement(&self) -> &str {
        match self {
            Self::Establish { statement, .. } | Self::Develop { statement, .. } => statement,
        }
    }

    fn statement_mut(&mut self) -> &mut String {
        match self {
            Self::Establish { statement, .. } | Self::Develop { statement, .. } => statement,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum StoredTraitLifecycle {
    Establish,
    Develop,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct StoredTraitChange {
    pub(super) lifecycle: StoredTraitLifecycle,
    pub(super) trait_id: Uuid,
    pub(super) entity_id: EntityId,
    pub(super) entity_name: String,
    pub(super) previous_statement: Option<String>,
    pub(super) statement: String,
}

impl From<StoredTraitChange> for ActivityTraitChange {
    fn from(value: StoredTraitChange) -> Self {
        let entity = EntitySummary {
            id: value.entity_id,
            name: value.entity_name,
        };
        let r#trait = EntityTrait {
            id: EntityTraitId(value.trait_id),
            statement: value.statement,
        };
        match (value.lifecycle, value.previous_statement) {
            (StoredTraitLifecycle::Establish, None) => Self::Establish { entity, r#trait },
            (StoredTraitLifecycle::Develop, Some(previous_statement)) => Self::Develop {
                entity,
                r#trait,
                previous_statement,
            },
            _ => unreachable!("stored Trait lifecycle is validated during hydration"),
        }
    }
}

#[derive(Debug)]
pub(super) enum TraitPersistenceError {
    InvalidInput,
    Unavailable,
    InvalidStoredRelation,
    Storage(Box<sqlx::Error>),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum TraitNormalizationError {
    InvalidCount,
    InvalidStatement(InvalidReason),
    DuplicateEstablishment,
    DuplicateDevelopment,
}

pub(super) fn map_trait_error(error: TraitPersistenceError, operation: &'static str) -> WorldError {
    match error {
        TraitPersistenceError::InvalidInput => WorldError::InvalidTrait,
        TraitPersistenceError::Unavailable => WorldError::TraitUnavailable,
        TraitPersistenceError::InvalidStoredRelation => invalid_stored_relation(),
        TraitPersistenceError::Storage(error) => storage_error(operation, *error),
    }
}

fn map_trait_normalization_error(_error: TraitNormalizationError) -> WorldError {
    WorldError::InvalidTrait
}

pub(super) fn normalize_trait_change_input(
    input: Vec<EntityTraitChangeInput>,
    allow_empty: bool,
) -> Result<Vec<TraitWrite>, WorldError> {
    let writes = input
        .into_iter()
        .map(|change| match change {
            EntityTraitChangeInput::Establish {
                entity_id,
                statement,
            } => TraitWrite::Establish {
                entity_id,
                statement,
            },
            EntityTraitChangeInput::Develop {
                trait_id,
                statement,
            } => TraitWrite::Develop {
                trait_id: trait_id.0,
                statement,
            },
        })
        .collect();
    normalize_trait_writes(writes, allow_empty).map_err(map_trait_normalization_error)
}

pub(super) fn trait_input_from_writes(writes: Vec<TraitWrite>) -> Vec<EntityTraitChangeInput> {
    writes
        .into_iter()
        .map(|write| match write {
            TraitWrite::Establish {
                entity_id,
                statement,
            } => EntityTraitChangeInput::Establish {
                entity_id,
                statement,
            },
            TraitWrite::Develop {
                trait_id,
                statement,
            } => EntityTraitChangeInput::Develop {
                trait_id: EntityTraitId(trait_id),
                statement,
            },
        })
        .collect()
}
fn trait_storage_error(error: sqlx::Error) -> TraitPersistenceError {
    TraitPersistenceError::Storage(Box::new(error))
}

pub(super) fn normalize_trait_writes(
    mut writes: Vec<TraitWrite>,
    allow_empty: bool,
) -> Result<Vec<TraitWrite>, TraitNormalizationError> {
    if writes.len() > MAX_TRAIT_COUNT || (!allow_empty && writes.is_empty()) {
        return Err(TraitNormalizationError::InvalidCount);
    }
    for write in &mut writes {
        let statement = write.statement_mut();
        *statement = statement.trim().to_owned();
        let reason = if statement.is_empty() {
            Some(InvalidReason::Empty)
        } else if statement.contains('\0') {
            Some(InvalidReason::ContainsNul)
        } else if statement.chars().count() > MAX_TRAIT_STATEMENT_LENGTH {
            Some(InvalidReason::TooLong)
        } else {
            None
        };
        if let Some(reason) = reason {
            return Err(TraitNormalizationError::InvalidStatement(reason));
        }
    }
    writes.sort_unstable_by(|left, right| match (left, right) {
        (
            TraitWrite::Establish {
                entity_id: left_entity_id,
                statement: left_statement,
            },
            TraitWrite::Establish {
                entity_id: right_entity_id,
                statement: right_statement,
            },
        ) => left_entity_id
            .0
            .as_bytes()
            .cmp(right_entity_id.0.as_bytes())
            .then_with(|| left_statement.cmp(right_statement)),
        (TraitWrite::Establish { .. }, TraitWrite::Develop { .. }) => std::cmp::Ordering::Less,
        (TraitWrite::Develop { .. }, TraitWrite::Establish { .. }) => std::cmp::Ordering::Greater,
        (
            TraitWrite::Develop {
                trait_id: left_trait_id,
                statement: left_statement,
            },
            TraitWrite::Develop {
                trait_id: right_trait_id,
                statement: right_statement,
            },
        ) => left_trait_id
            .as_bytes()
            .cmp(right_trait_id.as_bytes())
            .then_with(|| left_statement.cmp(right_statement)),
    });
    for pair in writes.windows(2) {
        match (&pair[0], &pair[1]) {
            (
                TraitWrite::Establish {
                    entity_id: left_entity_id,
                    statement: left_statement,
                },
                TraitWrite::Establish {
                    entity_id: right_entity_id,
                    statement: right_statement,
                },
            ) if left_entity_id == right_entity_id && left_statement == right_statement => {
                return Err(TraitNormalizationError::DuplicateEstablishment);
            }
            (
                TraitWrite::Develop {
                    trait_id: left_trait_id,
                    ..
                },
                TraitWrite::Develop {
                    trait_id: right_trait_id,
                    ..
                },
            ) if left_trait_id == right_trait_id => {
                return Err(TraitNormalizationError::DuplicateDevelopment);
            }
            _ => {}
        }
    }
    Ok(writes)
}

#[derive(FromRow)]
struct TraitOwnerRow {
    trait_id: Uuid,
    entity_id: EntityId,
}

#[derive(FromRow)]
struct CurrentTraitRow {
    trait_id: Uuid,
    entity_id: EntityId,
    current_activity_id: ActivityId,
    statement: String,
}

struct ResolvedTraitWrite<'a> {
    lifecycle: StoredTraitLifecycle,
    trait_id: Uuid,
    entity_id: EntityId,
    previous_activity_id: Option<ActivityId>,
    statement: &'a str,
}

pub(super) async fn write_trait_changes(
    transaction: &mut Transaction<'_, Postgres>,
    activity_id: ActivityId,
    writes: &[TraitWrite],
    eligible_entity_id: &[EntityId],
) -> Result<Vec<StoredTraitChange>, TraitPersistenceError> {
    if writes.is_empty() {
        return Ok(Vec::new());
    }

    let establish = writes
        .iter()
        .filter_map(|write| match write {
            TraitWrite::Establish {
                entity_id,
                statement,
            } => Some((*entity_id, statement.as_str())),
            TraitWrite::Develop { .. } => None,
        })
        .collect::<Vec<_>>();
    let develop_trait_id = writes
        .iter()
        .filter_map(|write| match write {
            TraitWrite::Establish { .. } => None,
            TraitWrite::Develop { trait_id, .. } => Some(*trait_id),
        })
        .collect::<Vec<_>>();

    let owner_row = if develop_trait_id.is_empty() {
        Vec::new()
    } else {
        record_trait_query(TraitQueryKind::Write);
        sqlx::query_as::<_, TraitOwnerRow>(FIND_ENTITY_TRAIT_OWNER_SQL)
            .bind(&develop_trait_id)
            .fetch_all(&mut **transaction)
            .await
            .map_err(trait_storage_error)?
    };
    if owner_row.len() != develop_trait_id.len() {
        return Err(TraitPersistenceError::Unavailable);
    }
    let owner_by_trait = owner_row
        .into_iter()
        .map(|row| (row.trait_id, row.entity_id))
        .collect::<HashMap<_, _>>();

    let mut affected_entity_id = establish
        .iter()
        .map(|(entity_id, _)| *entity_id)
        .chain(owner_by_trait.values().copied())
        .collect::<Vec<_>>();
    affected_entity_id.sort_unstable_by(|left, right| left.0.as_bytes().cmp(right.0.as_bytes()));
    affected_entity_id.dedup();

    let mut eligible_entity_id = eligible_entity_id.to_vec();
    eligible_entity_id.sort_unstable_by(|left, right| left.0.as_bytes().cmp(right.0.as_bytes()));
    eligible_entity_id.dedup();
    if affected_entity_id.iter().any(|entity_id| {
        eligible_entity_id
            .binary_search_by(|eligible| eligible.0.as_bytes().cmp(entity_id.0.as_bytes()))
            .is_err()
    }) {
        return Err(TraitPersistenceError::Unavailable);
    }

    let affected_uuid = affected_entity_id
        .iter()
        .map(|entity_id| entity_id.0)
        .collect::<Vec<_>>();
    record_trait_query(TraitQueryKind::Write);
    let locked_entity_id = sqlx::query_scalar::<_, Uuid>(LOCK_TRAIT_ENTITY_SQL)
        .bind(&affected_uuid)
        .fetch_all(&mut **transaction)
        .await
        .map_err(trait_storage_error)?;
    if locked_entity_id.len() != affected_entity_id.len() {
        return Err(TraitPersistenceError::Unavailable);
    }

    let current_row = if develop_trait_id.is_empty() {
        Vec::new()
    } else {
        record_trait_query(TraitQueryKind::Write);
        sqlx::query_as::<_, CurrentTraitRow>(LOCK_CURRENT_ENTITY_TRAIT_SQL)
            .bind(&develop_trait_id)
            .fetch_all(&mut **transaction)
            .await
            .map_err(trait_storage_error)?
    };
    if current_row.len() != develop_trait_id.len() {
        return Err(TraitPersistenceError::Unavailable);
    }
    let current_by_trait = current_row
        .into_iter()
        .map(|row| (row.trait_id, row))
        .collect::<HashMap<_, _>>();

    let mut resolved = Vec::with_capacity(writes.len());
    for write in writes {
        match write {
            TraitWrite::Establish {
                entity_id,
                statement,
            } => resolved.push(ResolvedTraitWrite {
                lifecycle: StoredTraitLifecycle::Establish,
                trait_id: Uuid::new_v4(),
                entity_id: *entity_id,
                previous_activity_id: None,
                statement,
            }),
            TraitWrite::Develop {
                trait_id,
                statement,
            } => {
                let current = current_by_trait
                    .get(trait_id)
                    .ok_or(TraitPersistenceError::InvalidStoredRelation)?;
                if owner_by_trait.get(trait_id).copied() != Some(current.entity_id) {
                    return Err(TraitPersistenceError::InvalidStoredRelation);
                }
                if current.statement == *statement {
                    return Err(TraitPersistenceError::InvalidInput);
                }
                resolved.push(ResolvedTraitWrite {
                    lifecycle: StoredTraitLifecycle::Develop,
                    trait_id: *trait_id,
                    entity_id: current.entity_id,
                    previous_activity_id: Some(current.current_activity_id),
                    statement,
                });
            }
        }
    }
    resolved.sort_unstable_by(|left, right| {
        left.entity_id
            .0
            .as_bytes()
            .cmp(right.entity_id.0.as_bytes())
            .then_with(|| left.trait_id.as_bytes().cmp(right.trait_id.as_bytes()))
    });

    let developed_trait_id = resolved
        .iter()
        .filter(|write| write.lifecycle == StoredTraitLifecycle::Develop)
        .map(|write| write.trait_id)
        .collect::<Vec<_>>();
    let mut intended_active = resolved
        .iter()
        .map(|write| (write.entity_id, write.statement.to_owned()))
        .collect::<Vec<_>>();
    intended_active.sort_unstable_by(|left, right| {
        left.0
            .0
            .as_bytes()
            .cmp(right.0.0.as_bytes())
            .then_with(|| left.1.cmp(&right.1))
    });
    if intended_active.windows(2).any(|pair| pair[0] == pair[1]) {
        return Err(TraitPersistenceError::InvalidInput);
    }
    let proposed_entity_id = intended_active
        .iter()
        .map(|(entity_id, _)| entity_id.0)
        .collect::<Vec<_>>();
    let proposed_statement = intended_active
        .iter()
        .map(|(_, statement)| statement.as_str())
        .collect::<Vec<_>>();
    let active_duplicate = {
        record_trait_query(TraitQueryKind::Write);
        sqlx::query_as::<_, (Uuid, Uuid, String)>(LOCK_ACTIVE_ENTITY_TRAIT_STATEMENT_SQL)
            .bind(&proposed_entity_id)
            .bind(&proposed_statement)
            .bind(&developed_trait_id)
            .fetch_all(&mut **transaction)
            .await
            .map_err(trait_storage_error)?
    };
    if !active_duplicate.is_empty() {
        return Err(TraitPersistenceError::InvalidInput);
    }

    let root = resolved
        .iter()
        .filter(|write| write.lifecycle == StoredTraitLifecycle::Establish)
        .collect::<Vec<_>>();
    if !root.is_empty() {
        let trait_id = root.iter().map(|write| write.trait_id).collect::<Vec<_>>();
        let entity_id = root
            .iter()
            .map(|write| write.entity_id.0)
            .collect::<Vec<_>>();
        record_trait_query(TraitQueryKind::Write);
        sqlx::query(INSERT_ENTITY_TRAIT_SQL)
            .bind(&trait_id)
            .bind(&entity_id)
            .execute(&mut **transaction)
            .await
            .map_err(trait_storage_error)?;
    }

    let trait_id = resolved
        .iter()
        .map(|write| write.trait_id)
        .collect::<Vec<_>>();
    let entity_id = resolved
        .iter()
        .map(|write| write.entity_id.0)
        .collect::<Vec<_>>();
    let previous_activity_id = resolved
        .iter()
        .map(|write| write.previous_activity_id.map(|id| id.0))
        .collect::<Vec<_>>();
    let statement = resolved
        .iter()
        .map(|write| write.statement)
        .collect::<Vec<_>>();
    record_trait_query(TraitQueryKind::Write);
    sqlx::query(INSERT_ENTITY_TRAIT_VERSION_SQL)
        .bind(&trait_id)
        .bind(&entity_id)
        .bind(activity_id.0)
        .bind(&previous_activity_id)
        .bind(&statement)
        .execute(&mut **transaction)
        .await
        .map_err(trait_storage_error)?;
    record_trait_query(TraitQueryKind::Write);
    sqlx::query(UPSERT_CURRENT_ENTITY_TRAIT_SQL)
        .bind(&trait_id)
        .bind(&entity_id)
        .bind(activity_id.0)
        .execute(&mut **transaction)
        .await
        .map_err(trait_storage_error)?;

    hydrate_trait_changes(transaction, &[activity_id])
        .await
        .map(|mut change| change.remove(&activity_id).unwrap_or_default())
}

#[derive(FromRow)]
struct StoredTraitChangeRow {
    activity_id: ActivityId,
    trait_id: Uuid,
    entity_id: EntityId,
    entity_name: String,
    previous_activity_id: Option<ActivityId>,
    previous_statement: Option<String>,
    statement: String,
}

pub(super) async fn hydrate_trait_changes(
    transaction: &mut Transaction<'_, Postgres>,
    activity_id: &[ActivityId],
) -> Result<HashMap<ActivityId, Vec<StoredTraitChange>>, TraitPersistenceError> {
    if activity_id.is_empty() {
        return Ok(HashMap::new());
    }
    let activity_uuid = activity_id.iter().map(|id| id.0).collect::<Vec<_>>();
    record_trait_query(TraitQueryKind::Hydration);
    let row = sqlx::query_as::<_, StoredTraitChangeRow>(HYDRATE_ENTITY_TRAIT_CHANGE_SQL)
        .bind(&activity_uuid)
        .fetch_all(&mut **transaction)
        .await
        .map_err(trait_storage_error)?;
    let mut change = HashMap::<ActivityId, Vec<StoredTraitChange>>::new();
    for row in row {
        let lifecycle = match (row.previous_activity_id, row.previous_statement.as_ref()) {
            (None, None) => StoredTraitLifecycle::Establish,
            (Some(_), Some(_)) => StoredTraitLifecycle::Develop,
            _ => return Err(TraitPersistenceError::InvalidStoredRelation),
        };
        change
            .entry(row.activity_id)
            .or_default()
            .push(StoredTraitChange {
                lifecycle,
                trait_id: row.trait_id,
                entity_id: row.entity_id,
                entity_name: row.entity_name,
                previous_statement: row.previous_statement,
                statement: row.statement,
            });
    }
    Ok(change)
}
