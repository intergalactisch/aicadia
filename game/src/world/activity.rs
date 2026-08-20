use super::*;

pub(super) fn action_fingerprint(input: &SubmitAction) -> Vec<u8> {
    let mut hash = Sha256::new();
    for field in [
        b"aicadia-submit-action-fingerprint-v1".as_slice(),
        input.expected_place_revision.fingerprint_bytes().as_slice(),
        input.prose.as_bytes(),
    ] {
        fingerprint_field(&mut hash, field);
    }
    match &input.consequence {
        ActionConsequence::IntroduceEntity(consequence) => {
            for field in [
                b"introduce_entity".as_slice(),
                consequence.name.as_bytes(),
                consequence.description.as_bytes(),
            ] {
                fingerprint_field(&mut hash, field);
            }
            if let Some(description) = &consequence.position_description {
                fingerprint_field(&mut hash, b"position_description_v1");
                fingerprint_field(&mut hash, description.as_bytes());
            }
            fingerprint_property_input(&mut hash, &consequence.property);
            if !consequence.r#trait.is_empty() {
                fingerprint_field(&mut hash, b"initial_trait_v1");
                fingerprint_trait_input(&mut hash, &consequence.r#trait);
            }
        }
        ActionConsequence::ChangeEntityState(consequence) => {
            match (
                consequence.property_change.is_empty(),
                consequence.trait_change.is_empty(),
            ) {
                (false, true) => {
                    fingerprint_field(&mut hash, b"change_entity_property");
                    fingerprint_property_change(&mut hash, &consequence.property_change);
                }
                (true, false) => {
                    fingerprint_field(&mut hash, b"change_entity_trait");
                    fingerprint_trait_change(&mut hash, &consequence.trait_change);
                }
                (false, false) => {
                    fingerprint_field(&mut hash, b"change_entity_state");
                    fingerprint_property_change(&mut hash, &consequence.property_change);
                    fingerprint_trait_change(&mut hash, &consequence.trait_change);
                }
                (true, true) => unreachable!("empty state change is rejected during normalization"),
            }
        }
    }
    hash.finalize().to_vec()
}

#[cfg(test)]
mod fingerprint_test {
    use super::*;

    fn revision() -> PlaceRevision {
        PlaceRevision::from_parts(
            EntityId(Uuid::from_u128(1)),
            DateTime::<Utc>::from_timestamp(1_723_689_600, 123_000_000).unwrap(),
            ActivityId(Uuid::from_u128(2)),
        )
    }

    fn base_hash(prose: &str) -> Sha256 {
        let revision = revision();
        let mut hash = Sha256::new();
        for field in [
            b"aicadia-submit-action-fingerprint-v1".as_slice(),
            revision.fingerprint_bytes().as_slice(),
            prose.as_bytes(),
        ] {
            fingerprint_field(&mut hash, field);
        }
        hash
    }

    #[test]
    fn single_kind_state_fingerprints_remain_exactly_legacy_compatible() {
        let entity_id = EntityId(Uuid::from_u128(3));
        let property = SubmitAction {
            request_id: Uuid::from_u128(4),
            expected_place_revision: revision(),
            prose: "Mara records one fact.".to_owned(),
            consequence: ActionConsequence::ChangeEntityState(ChangeEntityState {
                property_change: vec![EntityPropertyChangeInput {
                    entity_id,
                    key: "leg_count".to_owned(),
                    value: PropertyValue::Integer(3),
                }],
                trait_change: Vec::new(),
            }),
        }
        .normalize()
        .unwrap();
        let mut expected_property = base_hash("Mara records one fact.");
        fingerprint_field(&mut expected_property, b"change_entity_property");
        fingerprint_property_change(
            &mut expected_property,
            &[EntityPropertyChangeInput {
                entity_id,
                key: "leg_count".to_owned(),
                value: PropertyValue::Integer(3),
            }],
        );
        assert_eq!(
            action_fingerprint(&property),
            expected_property.finalize().as_slice()
        );

        let r#trait = SubmitAction {
            request_id: Uuid::from_u128(5),
            expected_place_revision: revision(),
            prose: "Mara records one characterization.".to_owned(),
            consequence: ActionConsequence::ChangeEntityState(ChangeEntityState {
                property_change: Vec::new(),
                trait_change: vec![EntityTraitChangeInput::Establish {
                    entity_id,
                    statement: "Jumps unusually high.".to_owned(),
                }],
            }),
        }
        .normalize()
        .unwrap();
        let mut expected_trait = base_hash("Mara records one characterization.");
        fingerprint_field(&mut expected_trait, b"change_entity_trait");
        fingerprint_trait_change(
            &mut expected_trait,
            &[EntityTraitChangeInput::Establish {
                entity_id,
                statement: "Jumps unusually high.".to_owned(),
            }],
        );
        assert_eq!(
            action_fingerprint(&r#trait),
            expected_trait.finalize().as_slice()
        );
    }

    #[test]
    fn combined_state_and_initial_trait_use_new_order_independent_components() {
        let entity_id = EntityId(Uuid::from_u128(6));
        let state = |reverse: bool| {
            let mut property_change = vec![
                EntityPropertyChangeInput {
                    entity_id,
                    key: "leg_count".to_owned(),
                    value: PropertyValue::Integer(3),
                },
                EntityPropertyChangeInput {
                    entity_id,
                    key: "temper".to_owned(),
                    value: PropertyValue::Text("hot".to_owned()),
                },
            ];
            let mut trait_change = vec![
                EntityTraitChangeInput::Establish {
                    entity_id,
                    statement: "Jumps unusually high.".to_owned(),
                },
                EntityTraitChangeInput::Establish {
                    entity_id,
                    statement: "Startles easily.".to_owned(),
                },
            ];
            if reverse {
                property_change.reverse();
                trait_change.reverse();
            }
            SubmitAction {
                request_id: Uuid::from_u128(7),
                expected_place_revision: revision(),
                prose: "Mara records the whole change.".to_owned(),
                consequence: ActionConsequence::ChangeEntityState(ChangeEntityState {
                    property_change,
                    trait_change,
                }),
            }
            .normalize()
            .unwrap()
        };
        let first = state(false);
        let reordered = state(true);
        assert_eq!(action_fingerprint(&first), action_fingerprint(&reordered));

        let mut expected = base_hash("Mara records the whole change.");
        fingerprint_field(&mut expected, b"change_entity_state");
        let ActionConsequence::ChangeEntityState(change) = &first.consequence else {
            unreachable!()
        };
        fingerprint_property_change(&mut expected, &change.property_change);
        fingerprint_trait_change(&mut expected, &change.trait_change);
        assert_eq!(action_fingerprint(&first), expected.finalize().as_slice());

        let introduction = |reverse: bool| {
            let mut r#trait = vec![
                TraitInput {
                    statement: "Jumps unusually high.".to_owned(),
                },
                TraitInput {
                    statement: "Startles easily.".to_owned(),
                },
            ];
            if reverse {
                r#trait.reverse();
            }
            SubmitAction {
                request_id: Uuid::from_u128(8),
                expected_place_revision: revision(),
                prose: "Mara introduces the frog.".to_owned(),
                consequence: ActionConsequence::IntroduceEntity(IntroduceEntity {
                    name: "Three-legged Frog".to_owned(),
                    description: "A heat-scorched frog.".to_owned(),
                    position_description: None,
                    property: Vec::new(),
                    r#trait,
                }),
            }
            .normalize()
            .unwrap()
        };
        assert_eq!(
            action_fingerprint(&introduction(false)),
            action_fingerprint(&introduction(true))
        );

        let without_trait = SubmitAction {
            request_id: Uuid::from_u128(8),
            expected_place_revision: revision(),
            prose: "Mara introduces the frog.".to_owned(),
            consequence: ActionConsequence::IntroduceEntity(IntroduceEntity {
                name: "Three-legged Frog".to_owned(),
                description: "A heat-scorched frog.".to_owned(),
                position_description: None,
                property: Vec::new(),
                r#trait: Vec::new(),
            }),
        }
        .normalize()
        .unwrap();
        let mut legacy = base_hash("Mara introduces the frog.");
        for field in [
            b"introduce_entity".as_slice(),
            b"Three-legged Frog".as_slice(),
            b"A heat-scorched frog.".as_slice(),
        ] {
            fingerprint_field(&mut legacy, field);
        }
        assert_eq!(
            action_fingerprint(&without_trait),
            legacy.finalize().as_slice()
        );
        assert_ne!(
            action_fingerprint(&without_trait),
            action_fingerprint(&introduction(false))
        );
    }
}

pub(super) fn interaction_fingerprint(input: &NormalizedSubmitInteraction) -> Vec<u8> {
    let mut hash = Sha256::new();
    for field in [
        b"aicadia-submit-interaction-fingerprint-v1".as_slice(),
        input.expected_place_revision.fingerprint_bytes().as_slice(),
        input.prose.as_bytes(),
    ] {
        fingerprint_field(&mut hash, field);
    }
    for target_entity_id in &input.target_entity_id {
        let field = target_entity_id.0.as_bytes();
        fingerprint_field(&mut hash, field);
    }
    let property_change = input
        .property_change
        .iter()
        .map(|write| EntityPropertyChangeInput {
            entity_id: write.entity_id,
            key: write.key.clone(),
            value: write.value.clone(),
        })
        .collect::<Vec<_>>();
    fingerprint_property_change(&mut hash, &property_change);
    fingerprint_trait_writes(&mut hash, &input.trait_change);
    hash.finalize().to_vec()
}

pub(super) async fn find_placed_entity(
    transaction: &mut Transaction<'_, Postgres>,
    entity_id: EntityId,
    place_entity_id: EntityId,
    operation: &'static str,
) -> Result<Option<Entity>, WorldError> {
    sqlx::query_as::<_, Entity>(
        r#"
        SELECT entity.id, entity.name, entity.description,
               entity.introduced_by_user_id, entity.introduced_at
        FROM entity_location
        JOIN entity ON entity.id = entity_location.entity_id
        WHERE entity_location.entity_id = $1
          AND entity_location.place_entity_id = $2
        "#,
    )
    .bind(entity_id.0)
    .bind(place_entity_id.0)
    .fetch_optional(&mut **transaction)
    .await
    .map_err(|error| storage_error(operation, error))
}

pub(super) fn fingerprint_field(hash: &mut Sha256, field: &[u8]) {
    hash.update((field.len() as u64).to_be_bytes());
    hash.update(field);
}

pub(super) fn fingerprint_property_input(hash: &mut Sha256, property: &[PropertyInput]) {
    for property in property {
        fingerprint_field(hash, property.key.as_bytes());
        fingerprint_property_value(hash, &property.value);
    }
}

pub(super) fn fingerprint_trait_input(hash: &mut Sha256, r#trait: &[TraitInput]) {
    for r#trait in r#trait {
        fingerprint_field(hash, r#trait.statement.as_bytes());
    }
}

fn fingerprint_property_change(hash: &mut Sha256, property: &[EntityPropertyChangeInput]) {
    for property in property {
        fingerprint_field(hash, property.entity_id.0.as_bytes());
        fingerprint_field(hash, property.key.as_bytes());
        fingerprint_property_value(hash, &property.value);
    }
}

fn fingerprint_property_value(hash: &mut Sha256, value: &PropertyValue) {
    match value {
        PropertyValue::Text(value) => {
            fingerprint_field(hash, b"text");
            fingerprint_field(hash, value.as_bytes());
        }
        PropertyValue::Integer(value) => {
            fingerprint_field(hash, b"integer");
            fingerprint_field(hash, &value.to_be_bytes());
        }
    }
}

fn fingerprint_trait_change(hash: &mut Sha256, change: &[EntityTraitChangeInput]) {
    for change in change {
        match change {
            EntityTraitChangeInput::Establish {
                entity_id,
                statement,
            } => {
                fingerprint_field(hash, b"establish");
                fingerprint_field(hash, entity_id.0.as_bytes());
                fingerprint_field(hash, statement.as_bytes());
            }
            EntityTraitChangeInput::Develop {
                trait_id,
                statement,
            } => {
                fingerprint_field(hash, b"develop");
                fingerprint_field(hash, trait_id.0.as_bytes());
                fingerprint_field(hash, statement.as_bytes());
            }
        }
    }
}

fn fingerprint_trait_writes(hash: &mut Sha256, change: &[TraitWrite]) {
    for change in change {
        let statement = change.statement();
        match change {
            TraitWrite::Establish {
                entity_id,
                statement: _,
            } => {
                fingerprint_field(hash, b"establish");
                fingerprint_field(hash, entity_id.0.as_bytes());
                fingerprint_field(hash, statement.as_bytes());
            }
            TraitWrite::Develop {
                trait_id,
                statement: _,
            } => {
                fingerprint_field(hash, b"develop");
                fingerprint_field(hash, trait_id.as_bytes());
                fingerprint_field(hash, statement.as_bytes());
            }
        }
    }
}

pub(super) async fn activities_from_rows(
    transaction: &mut Transaction<'_, Postgres>,
    row: Vec<ActivityRow>,
    operation: &'static str,
) -> Result<Vec<Activity>, WorldError> {
    let typed_activity_ids = row.iter().map(|row| row.id).collect::<Vec<_>>();
    let activity_ids = typed_activity_ids.iter().map(|id| id.0).collect::<Vec<_>>();
    let related = if activity_ids.is_empty() {
        Vec::new()
    } else {
        sqlx::query_as::<_, ActivityEntityRow>(
            r#"
            SELECT activity_entity.activity_id, activity_entity.entity_id,
                   entity.name, activity_entity.role
            FROM activity_entity
            JOIN entity ON entity.id = activity_entity.entity_id
            WHERE activity_entity.activity_id = ANY($1)
            ORDER BY activity_entity.activity_id, activity_entity.role, activity_entity.entity_id
            "#,
        )
        .bind(&activity_ids)
        .fetch_all(&mut **transaction)
        .await
        .map_err(|error| storage_error(operation, error))?
    };
    let mut involved_by_activity: HashMap<ActivityId, Vec<ActivityEntityReference>> =
        HashMap::new();
    for related in related {
        involved_by_activity
            .entry(related.activity_id)
            .or_default()
            .push(related.try_into()?);
    }
    let position = if activity_ids.is_empty() {
        Vec::new()
    } else {
        sqlx::query_as::<_, ActivityPositionRow>(
            r#"
            SELECT activity_position.activity_id, activity_position.role,
                   version.entity_id, version.activity_id AS position_activity_id,
                   version.x_cm, version.y_cm, version.z_cm, version.description
            FROM activity_position
            JOIN position_version version
              ON version.entity_id = activity_position.position_entity_id
             AND version.activity_id = activity_position.position_activity_id
            WHERE activity_position.activity_id = ANY($1)
            ORDER BY activity_position.activity_id, activity_position.role,
                     activity_position.position_entity_id,
                     activity_position.position_activity_id
            "#,
        )
        .bind(&activity_ids)
        .fetch_all(&mut **transaction)
        .await
        .map_err(|error| storage_error(operation, error))?
    };
    let mut position_by_activity: HashMap<ActivityId, Vec<ActivityPositionReference>> =
        HashMap::new();
    for position in position {
        position_by_activity
            .entry(position.activity_id)
            .or_default()
            .push(position.try_into()?);
    }
    let connection = if activity_ids.is_empty() {
        Vec::new()
    } else {
        sqlx::query_as::<_, ActivityConnectionRow>(
            r#"
            SELECT activity_id, connection_id
            FROM activity_connection
            WHERE activity_id = ANY($1)
            ORDER BY activity_id, connection_id
            "#,
        )
        .bind(&activity_ids)
        .fetch_all(&mut **transaction)
        .await
        .map_err(|error| storage_error(operation, error))?
    };
    let mut connection_by_activity: HashMap<ActivityId, Vec<ActivityConnectionReference>> =
        HashMap::new();
    for connection in connection {
        connection_by_activity
            .entry(connection.activity_id)
            .or_default()
            .push(ActivityConnectionReference {
                connection_id: connection.connection_id,
            });
    }
    let mut property_by_activity = hydrate_property_changes(transaction, &typed_activity_ids)
        .await
        .map_err(|error| map_property_error(error, operation))?;
    let mut trait_by_activity = hydrate_trait_changes(transaction, &typed_activity_ids)
        .await
        .map_err(|error| map_trait_error(error, operation))?;
    row.into_iter()
        .map(|row| {
            let id = row.id;
            let property_change = property_by_activity
                .remove(&id)
                .unwrap_or_default()
                .into_iter()
                .map(EntityPropertyChange::from)
                .collect();
            let trait_change = trait_by_activity
                .remove(&id)
                .unwrap_or_default()
                .into_iter()
                .map(ActivityTraitChange::from)
                .collect();
            row.into_activity(
                involved_by_activity.remove(&id).unwrap_or_default(),
                position_by_activity.remove(&id).unwrap_or_default(),
                connection_by_activity.remove(&id).unwrap_or_default(),
                property_change,
                trait_change,
            )
        })
        .collect()
}

pub(super) struct StoredAcceptedAction {
    pub(super) request_fingerprint: Vec<u8>,
    pub(super) accepted_action: AcceptedAction,
}

pub(super) struct StoredRequestActivity {
    pub(super) operation: ActivityOperation,
    pub(super) request_fingerprint: Vec<u8>,
    pub(super) activity: Activity,
}

impl StoredRequestActivity {
    pub(super) async fn into_accepted_interaction(
        self,
        transaction: &mut Transaction<'_, Postgres>,
        operation: &'static str,
    ) -> Result<AcceptedInteraction, WorldError> {
        if self.operation != ActivityOperation::SubmitInteraction {
            return Err(invalid_stored_relation());
        }
        let location_id = self
            .activity
            .involved_entity
            .iter()
            .find(|reference| reference.role == ActivityEntityRole::Location)
            .map(|reference| reference.entity.id)
            .ok_or_else(invalid_stored_relation)?;
        let target_count = self
            .activity
            .involved_entity
            .iter()
            .filter(|reference| reference.role == ActivityEntityRole::Target)
            .count();
        if !(1..=MAX_INTERACTION_TARGET_COUNT).contains(&target_count) {
            return Err(invalid_stored_relation());
        }
        let place = find_place_by_id(transaction, location_id, operation)
            .await?
            .ok_or_else(invalid_stored_relation)?;
        Ok(AcceptedInteraction {
            activity: self.activity,
            place,
        })
    }
}

pub(super) async fn find_request_activity(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: UserId,
    request_id: Uuid,
    operation: &'static str,
) -> Result<Option<StoredRequestActivity>, WorldError> {
    let row = sqlx::query_as::<_, AcceptedActionActivityRow>(
        r#"
        SELECT activity.id, activity.operation, activity.prose, activity.occurred_at,
               activity.request_fingerprint, activity.action_consequence,
               actor.id AS actor_entity_id, actor.name AS actor_name,
               context.id AS context_entity_id, context.name AS context_name,
               context_place.is_entry AS context_is_entry
        FROM activity
        LEFT JOIN entity actor ON actor.id = activity.actor_character_entity_id
        LEFT JOIN place context_place ON context_place.entity_id = activity.context_place_entity_id
        LEFT JOIN entity context ON context.id = context_place.entity_id
        WHERE activity.requested_by_user_id = $1
          AND activity.request_id = $2
        "#,
    )
    .bind(user_id.0)
    .bind(request_id)
    .fetch_optional(&mut **transaction)
    .await
    .map_err(|error| storage_error(operation, error))?;
    let Some(row) = row else {
        return Ok(None);
    };
    let request_fingerprint = row.request_fingerprint.clone();
    let activity = activities_from_rows(transaction, vec![row.into()], operation)
        .await?
        .pop()
        .ok_or_else(invalid_stored_relation)?;
    Ok(Some(StoredRequestActivity {
        operation: activity.operation,
        request_fingerprint,
        activity,
    }))
}

pub(super) async fn find_accepted_action(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: UserId,
    request_id: Uuid,
    operation: &'static str,
) -> Result<Option<StoredAcceptedAction>, WorldError> {
    let row = sqlx::query_as::<_, AcceptedActionActivityRow>(
        r#"
        SELECT activity.id, activity.operation, activity.prose, activity.occurred_at,
               activity.request_fingerprint, activity.action_consequence,
               actor.id AS actor_entity_id, actor.name AS actor_name,
               context.id AS context_entity_id, context.name AS context_name,
               context_place.is_entry AS context_is_entry
        FROM activity
        LEFT JOIN entity actor ON actor.id = activity.actor_character_entity_id
        LEFT JOIN place context_place ON context_place.entity_id = activity.context_place_entity_id
        LEFT JOIN entity context ON context.id = context_place.entity_id
        WHERE activity.requested_by_user_id = $1
          AND activity.request_id = $2
        "#,
    )
    .bind(user_id.0)
    .bind(request_id)
    .fetch_optional(&mut **transaction)
    .await
    .map_err(|error| storage_error(operation, error))?;
    let Some(row) = row else {
        return Ok(None);
    };
    let activity_id = row.id;
    let action_consequence = row.action_consequence.clone();
    let request_fingerprint = row.request_fingerprint.clone();
    let activity = activities_from_rows(transaction, vec![row.into()], operation)
        .await?
        .pop()
        .ok_or_else(invalid_stored_relation)?;
    if activity.operation != ActivityOperation::SubmitAction {
        return Err(WorldError::ActionRequestConflict);
    }
    let location_id = activity
        .involved_entity
        .iter()
        .find(|reference| reference.role == ActivityEntityRole::Location)
        .map(|reference| reference.entity.id)
        .ok_or_else(invalid_stored_relation)?;
    let consequence = match action_consequence.as_deref() {
        Some("introduce_entity") => {
            let subject_id = activity
                .involved_entity
                .iter()
                .find(|reference| reference.role == ActivityEntityRole::Subject)
                .map(|reference| reference.entity.id)
                .ok_or_else(invalid_stored_relation)?;
            let entity = find_placed_entity(transaction, subject_id, location_id, operation)
                .await?
                .ok_or_else(invalid_stored_relation)?;
            AcceptedActionConsequence::IntroduceEntity(entity)
        }
        Some("change_entity_property") if !activity.property_change.is_empty() => {
            AcceptedActionConsequence::ChangeEntityState {
                property_change: activity.property_change.clone(),
                trait_change: Vec::new(),
            }
        }
        Some("change_entity_trait") if !activity.trait_change.is_empty() => {
            AcceptedActionConsequence::ChangeEntityState {
                property_change: Vec::new(),
                trait_change: activity.trait_change.clone(),
            }
        }
        Some("change_entity_state")
            if !activity.property_change.is_empty() || !activity.trait_change.is_empty() =>
        {
            AcceptedActionConsequence::ChangeEntityState {
                property_change: activity.property_change.clone(),
                trait_change: activity.trait_change.clone(),
            }
        }
        _ => return Err(invalid_stored_relation()),
    };
    let place = find_place_by_id(transaction, location_id, operation)
        .await?
        .ok_or_else(invalid_stored_relation)?;
    debug_assert_eq!(activity.id, activity_id);
    Ok(Some(StoredAcceptedAction {
        request_fingerprint,
        accepted_action: AcceptedAction {
            activity,
            consequence,
            place,
        },
    }))
}

pub(super) struct ActivityDraft<'a> {
    pub(super) operation: ActivityOperation,
    pub(super) requested_by_user_id: UserId,
    pub(super) actor_character_entity_id: Option<EntityId>,
    pub(super) context_place_entity_id: Option<EntityId>,
    pub(super) involved: &'a [(EntityId, ActivityEntityRole)],
    pub(super) prose: Option<&'a str>,
    pub(super) request_id: Option<Uuid>,
    pub(super) request_fingerprint: Option<&'a [u8]>,
    pub(super) action_consequence: Option<&'static str>,
}

pub(super) async fn append_activity(
    transaction: &mut Transaction<'_, Postgres>,
    draft: ActivityDraft<'_>,
    storage_operation: &'static str,
) -> Result<ActivityId, WorldError> {
    let activity_id = ActivityId(Uuid::new_v4());
    sqlx::query(
        r#"
        INSERT INTO activity (
            id, operation, requested_by_user_id,
            actor_character_entity_id, context_place_entity_id,
            prose, request_id, request_fingerprint, action_consequence
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        "#,
    )
    .bind(activity_id.0)
    .bind(draft.operation.as_str())
    .bind(draft.requested_by_user_id.0)
    .bind(draft.actor_character_entity_id.map(|id| id.0))
    .bind(draft.context_place_entity_id.map(|id| id.0))
    .bind(draft.prose)
    .bind(draft.request_id)
    .bind(draft.request_fingerprint)
    .bind(draft.action_consequence)
    .execute(&mut **transaction)
    .await
    .map_err(|error| storage_error(storage_operation, error))?;
    append_activity_entity_roles(transaction, activity_id, draft.involved, storage_operation)
        .await?;
    Ok(activity_id)
}

pub(super) async fn append_activity_entity_roles(
    transaction: &mut Transaction<'_, Postgres>,
    activity_id: ActivityId,
    involved: &[(EntityId, ActivityEntityRole)],
    storage_operation: &'static str,
) -> Result<(), WorldError> {
    let involved_entity_id = involved
        .iter()
        .map(|(entity_id, _)| entity_id.0)
        .collect::<Vec<_>>();
    let involved_role = involved
        .iter()
        .map(|(_, role)| role.as_str())
        .collect::<Vec<_>>();
    if !involved.is_empty() {
        sqlx::query(
            r#"
            INSERT INTO activity_entity (activity_id, entity_id, role)
            SELECT $1, involved.entity_id, involved.role
            FROM UNNEST($2::uuid[], $3::text[]) AS involved(entity_id, role)
            "#,
        )
        .bind(activity_id.0)
        .bind(&involved_entity_id)
        .bind(&involved_role)
        .execute(&mut **transaction)
        .await
        .map_err(|error| storage_error(storage_operation, error))?;
    }
    Ok(())
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Activity {
    pub id: ActivityId,
    pub operation: ActivityOperation,
    pub actor_character: Option<EntitySummary>,
    pub context_place: Option<PlaceSummary>,
    pub involved_entity: Vec<ActivityEntityReference>,
    pub involved_position: Vec<ActivityPositionReference>,
    pub involved_connection: Vec<ActivityConnectionReference>,
    pub property_change: Vec<EntityPropertyChange>,
    pub trait_change: Vec<ActivityTraitChange>,
    pub prose: Option<String>,
    pub occurred_at: DateTime<Utc>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActivityOperation {
    CreateCharacter,
    CreateEntity,
    CreateEntryPlace,
    EnterWorld,
    SubmitAction,
    SubmitInteraction,
    SubmitDiscovery,
    MoveCharacter,
}

impl ActivityOperation {
    fn as_str(self) -> &'static str {
        match self {
            Self::CreateCharacter => "create_character",
            Self::CreateEntity => "create_entity",
            Self::CreateEntryPlace => "create_entry_place",
            Self::EnterWorld => "enter_world",
            Self::SubmitAction => "submit_action",
            Self::SubmitInteraction => "submit_interaction",
            Self::SubmitDiscovery => "submit_discovery",
            Self::MoveCharacter => "move_character",
        }
    }

    fn parse(value: &str) -> Result<Self, WorldError> {
        match value {
            "create_character" => Ok(Self::CreateCharacter),
            "create_entity" => Ok(Self::CreateEntity),
            "create_entry_place" => Ok(Self::CreateEntryPlace),
            "enter_world" => Ok(Self::EnterWorld),
            "submit_action" => Ok(Self::SubmitAction),
            "submit_interaction" => Ok(Self::SubmitInteraction),
            "submit_discovery" => Ok(Self::SubmitDiscovery),
            "move_character" => Ok(Self::MoveCharacter),
            _ => Err(invalid_stored_relation()),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActivityEntityRole {
    Subject,
    Destination,
    Location,
    Target,
}

impl ActivityEntityRole {
    fn as_str(self) -> &'static str {
        match self {
            Self::Subject => "subject",
            Self::Destination => "destination",
            Self::Location => "location",
            Self::Target => "target",
        }
    }

    fn parse(value: &str) -> Result<Self, WorldError> {
        match value {
            "subject" => Ok(Self::Subject),
            "destination" => Ok(Self::Destination),
            "location" => Ok(Self::Location),
            "target" => Ok(Self::Target),
            _ => Err(invalid_stored_relation()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActivityEntityReference {
    pub entity: EntitySummary,
    pub role: ActivityEntityRole,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActivityPositionReference {
    pub role: ActivityPositionRole,
    pub position: Position,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActivityConnectionReference {
    pub connection_id: ConnectionId,
}

#[derive(FromRow)]
pub(super) struct ActivityRow {
    pub(super) id: ActivityId,
    operation: String,
    prose: Option<String>,
    pub(super) occurred_at: DateTime<Utc>,
    actor_entity_id: Option<EntityId>,
    actor_name: Option<String>,
    context_entity_id: Option<EntityId>,
    context_name: Option<String>,
    context_is_entry: Option<bool>,
}

#[derive(FromRow)]
struct AcceptedActionActivityRow {
    id: ActivityId,
    operation: String,
    prose: Option<String>,
    occurred_at: DateTime<Utc>,
    request_fingerprint: Vec<u8>,
    action_consequence: Option<String>,
    actor_entity_id: Option<EntityId>,
    actor_name: Option<String>,
    context_entity_id: Option<EntityId>,
    context_name: Option<String>,
    context_is_entry: Option<bool>,
}

impl From<AcceptedActionActivityRow> for ActivityRow {
    fn from(value: AcceptedActionActivityRow) -> Self {
        Self {
            id: value.id,
            operation: value.operation,
            prose: value.prose,
            occurred_at: value.occurred_at,
            actor_entity_id: value.actor_entity_id,
            actor_name: value.actor_name,
            context_entity_id: value.context_entity_id,
            context_name: value.context_name,
            context_is_entry: value.context_is_entry,
        }
    }
}

impl ActivityRow {
    fn into_activity(
        self,
        involved_entity: Vec<ActivityEntityReference>,
        involved_position: Vec<ActivityPositionReference>,
        involved_connection: Vec<ActivityConnectionReference>,
        property_change: Vec<EntityPropertyChange>,
        trait_change: Vec<ActivityTraitChange>,
    ) -> Result<Activity, WorldError> {
        let actor_character = optional_summary(self.actor_entity_id, self.actor_name)?;
        let context_place = match optional_summary(self.context_entity_id, self.context_name)? {
            None => None,
            Some(entity) => Some(PlaceSummary {
                entity,
                is_entry: self.context_is_entry.ok_or_else(invalid_stored_relation)?,
            }),
        };
        Ok(Activity {
            id: self.id,
            operation: ActivityOperation::parse(&self.operation)?,
            actor_character,
            context_place,
            involved_entity,
            involved_position,
            involved_connection,
            property_change,
            trait_change,
            prose: self.prose,
            occurred_at: self.occurred_at,
        })
    }
}

#[derive(FromRow)]
struct ActivityEntityRow {
    activity_id: ActivityId,
    entity_id: EntityId,
    name: String,
    role: String,
}

#[derive(FromRow)]
struct ActivityPositionRow {
    activity_id: ActivityId,
    role: String,
    entity_id: EntityId,
    position_activity_id: ActivityId,
    x_cm: i64,
    y_cm: i64,
    z_cm: i64,
    description: Option<String>,
}

impl TryFrom<ActivityPositionRow> for ActivityPositionReference {
    type Error = WorldError;

    fn try_from(value: ActivityPositionRow) -> Result<Self, Self::Error> {
        Ok(Self {
            role: ActivityPositionRole::parse(&value.role)?,
            position: Position {
                x_cm: value.x_cm,
                y_cm: value.y_cm,
                z_cm: value.z_cm,
                description: value.description,
                position_revision: PositionRevision::from_parts(
                    value.entity_id,
                    value.position_activity_id,
                ),
            },
        })
    }
}

#[derive(FromRow)]
struct ActivityConnectionRow {
    activity_id: ActivityId,
    connection_id: ConnectionId,
}

impl TryFrom<ActivityEntityRow> for ActivityEntityReference {
    type Error = WorldError;

    fn try_from(value: ActivityEntityRow) -> Result<Self, Self::Error> {
        Ok(Self {
            entity: EntitySummary {
                id: value.entity_id,
                name: value.name,
            },
            role: ActivityEntityRole::parse(&value.role)?,
        })
    }
}

fn optional_summary(
    id: Option<EntityId>,
    name: Option<String>,
) -> Result<Option<EntitySummary>, WorldError> {
    match (id, name) {
        (None, None) => Ok(None),
        (Some(id), Some(name)) => Ok(Some(EntitySummary { id, name })),
        _ => Err(invalid_stored_relation()),
    }
}
