//! Experimental subject-scoped conflict fixture.
//!
//! This is a deterministic in-memory candidate, not production code or a selected
//! production transaction contract. PostgreSQL, actual locks and actual concurrent
//! execution are deliberately absent.

use std::collections::BTreeMap;

pub type RequestId = u64;
pub type CharacterId = u64;
pub type EntityId = u64;
pub type PropertyKey = String;

pub const IVO: CharacterId = 1;
pub const NIA: CharacterId = 2;
pub const STONE: EntityId = 10;
pub const DOOR: EntityId = 11;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum PlaceId {
    OldQuarry,
    QuietGrove,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Character {
    pub place_id: PlaceId,
    pub placement_version: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Property {
    pub value: String,
    pub version: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Entity {
    pub place_id: PlaceId,
    pub properties: BTreeMap<PropertyKey, Property>,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum DependencyKey {
    CharacterPlacement(CharacterId),
    EntityExistence(EntityId),
    Property(EntityId, PropertyKey),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlacementExpectation {
    pub place_id: PlaceId,
    pub version: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PropertyExpectation {
    Absent,
    Present { value: String, version: u64 },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PropertyChange {
    pub key: PropertyKey,
    pub expected: PropertyExpectation,
    pub new_value: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Operation {
    CreateEntity {
        entity_id: EntityId,
        place_id: PlaceId,
    },
    SetProperties {
        entity_id: EntityId,
        changes: Vec<PropertyChange>,
    },
    AppendInteraction {
        place_id: PlaceId,
        summary: String,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Intent {
    pub request_id: RequestId,
    pub fingerprint: String,
    pub actor_character_id: CharacterId,
    pub expected_actor_placement: PlacementExpectation,
    pub operation: Operation,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ActivityKind {
    CreateEntity,
    SetProperties,
    Interaction,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Activity {
    /// The stable request identity is reused instead of allocating a global counter.
    pub id: RequestId,
    pub request_id: RequestId,
    pub actor_character_id: CharacterId,
    pub place_id: PlaceId,
    pub kind: ActivityKind,
    pub checked_dependencies: Vec<DependencyKey>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Conflict {
    RequestFingerprintChanged,
    ActorMissing,
    StaleActorPlacement,
    ActorNotAtOperationPlace,
    EntityAlreadyExists,
    EntityMissing,
    EntityNotAtActorPlace,
    PropertyChanged {
        entity_id: EntityId,
        key: PropertyKey,
    },
    DuplicatePropertyDependency {
        entity_id: EntityId,
        key: PropertyKey,
    },
    EmptyPropertyChange,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SubmissionResult {
    Accepted {
        activity_id: RequestId,
        checked_dependencies: Vec<DependencyKey>,
    },
    Conflict(Conflict),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AcceptedRequestRecord {
    pub fingerprint: String,
    pub activity_id: RequestId,
    pub checked_dependencies: Vec<DependencyKey>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SubjectConflictLab {
    pub character: BTreeMap<CharacterId, Character>,
    pub entity: BTreeMap<EntityId, Entity>,
    pub activity: Vec<Activity>,
    pub accepted_request: BTreeMap<RequestId, AcceptedRequestRecord>,
}

impl Default for SubjectConflictLab {
    fn default() -> Self {
        let character = BTreeMap::from([
            (
                IVO,
                Character {
                    place_id: PlaceId::OldQuarry,
                    placement_version: 1,
                },
            ),
            (
                NIA,
                Character {
                    place_id: PlaceId::OldQuarry,
                    placement_version: 1,
                },
            ),
        ]);
        let entity = BTreeMap::from([
            (
                STONE,
                Entity {
                    place_id: PlaceId::OldQuarry,
                    properties: BTreeMap::from([
                        (
                            "color".to_owned(),
                            Property {
                                value: "grey".to_owned(),
                                version: 1,
                            },
                        ),
                        (
                            "state".to_owned(),
                            Property {
                                value: "standing".to_owned(),
                                version: 1,
                            },
                        ),
                    ]),
                },
            ),
            (
                DOOR,
                Entity {
                    place_id: PlaceId::OldQuarry,
                    properties: BTreeMap::from([(
                        "state".to_owned(),
                        Property {
                            value: "closed".to_owned(),
                            version: 1,
                        },
                    )]),
                },
            ),
        ]);

        Self {
            character,
            entity,
            activity: Vec::new(),
            accepted_request: BTreeMap::new(),
        }
    }
}

impl SubjectConflictLab {
    pub fn placement_expectation(&self, character_id: CharacterId) -> PlacementExpectation {
        let character = self
            .character
            .get(&character_id)
            .expect("fixed fixture Character must exist");
        PlacementExpectation {
            place_id: character.place_id,
            version: character.placement_version,
        }
    }

    pub fn property_expectation(&self, entity_id: EntityId, key: &str) -> PropertyExpectation {
        match self
            .entity
            .get(&entity_id)
            .expect("fixed fixture Entity must exist")
            .properties
            .get(key)
        {
            Some(property) => PropertyExpectation::Present {
                value: property.value.clone(),
                version: property.version,
            },
            None => PropertyExpectation::Absent,
        }
    }

    /// Fixture setup only: movement itself is not an accepted candidate operation.
    pub fn move_character_fixture(&mut self, character_id: CharacterId, place_id: PlaceId) {
        let character = self
            .character
            .get_mut(&character_id)
            .expect("fixed fixture Character must exist");
        character.place_id = place_id;
        character.placement_version += 1;
    }

    pub fn submit(&mut self, intent: Intent) -> SubmissionResult {
        if let Some(record) = self.accepted_request.get(&intent.request_id) {
            return if record.fingerprint == intent.fingerprint {
                SubmissionResult::Accepted {
                    activity_id: record.activity_id,
                    checked_dependencies: record.checked_dependencies.clone(),
                }
            } else {
                SubmissionResult::Conflict(Conflict::RequestFingerprintChanged)
            };
        }

        let checked_dependencies = canonical_dependencies(&intent);
        match self.validate(&intent) {
            Ok(place_id) => {
                self.apply_operation(&intent.operation);
                let kind = match intent.operation {
                    Operation::CreateEntity { .. } => ActivityKind::CreateEntity,
                    Operation::SetProperties { .. } => ActivityKind::SetProperties,
                    Operation::AppendInteraction { .. } => ActivityKind::Interaction,
                };
                self.activity.push(Activity {
                    id: intent.request_id,
                    request_id: intent.request_id,
                    actor_character_id: intent.actor_character_id,
                    place_id,
                    kind,
                    checked_dependencies: checked_dependencies.clone(),
                });
                self.accepted_request.insert(
                    intent.request_id,
                    AcceptedRequestRecord {
                        fingerprint: intent.fingerprint,
                        activity_id: intent.request_id,
                        checked_dependencies: checked_dependencies.clone(),
                    },
                );
                SubmissionResult::Accepted {
                    activity_id: intent.request_id,
                    checked_dependencies,
                }
            }
            Err(conflict) => SubmissionResult::Conflict(conflict),
        }
    }

    fn validate(&self, intent: &Intent) -> Result<PlaceId, Conflict> {
        let actor = self
            .character
            .get(&intent.actor_character_id)
            .ok_or(Conflict::ActorMissing)?;
        if actor.place_id != intent.expected_actor_placement.place_id
            || actor.placement_version != intent.expected_actor_placement.version
        {
            return Err(Conflict::StaleActorPlacement);
        }

        match &intent.operation {
            Operation::CreateEntity {
                entity_id,
                place_id,
            } => {
                if actor.place_id != *place_id {
                    return Err(Conflict::ActorNotAtOperationPlace);
                }
                if self.entity.contains_key(entity_id) {
                    return Err(Conflict::EntityAlreadyExists);
                }
                Ok(*place_id)
            }
            Operation::SetProperties { entity_id, changes } => {
                if changes.is_empty() {
                    return Err(Conflict::EmptyPropertyChange);
                }
                let entity = self.entity.get(entity_id).ok_or(Conflict::EntityMissing)?;
                if entity.place_id != actor.place_id {
                    return Err(Conflict::EntityNotAtActorPlace);
                }

                let mut keys: Vec<_> = changes.iter().map(|change| &change.key).collect();
                keys.sort();
                if let Some(pair) = keys.windows(2).find(|pair| pair[0] == pair[1]) {
                    return Err(Conflict::DuplicatePropertyDependency {
                        entity_id: *entity_id,
                        key: pair[0].clone(),
                    });
                }

                for change in changes {
                    let current = entity.properties.get(&change.key);
                    let matches = match (&change.expected, current) {
                        (PropertyExpectation::Absent, None) => true,
                        (PropertyExpectation::Present { value, version }, Some(property)) => {
                            property.value == *value && property.version == *version
                        }
                        _ => false,
                    };
                    if !matches {
                        return Err(Conflict::PropertyChanged {
                            entity_id: *entity_id,
                            key: change.key.clone(),
                        });
                    }
                }
                Ok(entity.place_id)
            }
            Operation::AppendInteraction { place_id, .. } => {
                if actor.place_id != *place_id {
                    return Err(Conflict::ActorNotAtOperationPlace);
                }
                Ok(*place_id)
            }
        }
    }

    fn apply_operation(&mut self, operation: &Operation) {
        match operation {
            Operation::CreateEntity {
                entity_id,
                place_id,
            } => {
                self.entity.insert(
                    *entity_id,
                    Entity {
                        place_id: *place_id,
                        properties: BTreeMap::new(),
                    },
                );
            }
            Operation::SetProperties { entity_id, changes } => {
                let entity = self
                    .entity
                    .get_mut(entity_id)
                    .expect("validated fixture Entity must exist");
                for change in changes {
                    let next_version = entity
                        .properties
                        .get(&change.key)
                        .map_or(1, |property| property.version + 1);
                    entity.properties.insert(
                        change.key.clone(),
                        Property {
                            value: change.new_value.clone(),
                            version: next_version,
                        },
                    );
                }
            }
            Operation::AppendInteraction { .. } => {}
        }
    }
}

fn canonical_dependencies(intent: &Intent) -> Vec<DependencyKey> {
    let mut keys = vec![DependencyKey::CharacterPlacement(intent.actor_character_id)];
    match &intent.operation {
        Operation::CreateEntity { entity_id, .. } => {
            keys.push(DependencyKey::EntityExistence(*entity_id));
        }
        Operation::SetProperties { entity_id, changes } => {
            keys.extend(
                changes
                    .iter()
                    .map(|change| DependencyKey::Property(*entity_id, change.key.clone())),
            );
        }
        Operation::AppendInteraction { .. } => {}
    }
    keys.sort();
    keys
}

#[cfg(test)]
mod tests {
    use super::*;

    fn intent(
        lab: &SubjectConflictLab,
        request_id: RequestId,
        fingerprint: &str,
        actor_character_id: CharacterId,
        operation: Operation,
    ) -> Intent {
        Intent {
            request_id,
            fingerprint: fingerprint.to_owned(),
            actor_character_id,
            expected_actor_placement: lab.placement_expectation(actor_character_id),
            operation,
        }
    }

    fn change(key: &str, expected: PropertyExpectation, new_value: &str) -> PropertyChange {
        PropertyChange {
            key: key.to_owned(),
            expected,
            new_value: new_value.to_owned(),
        }
    }

    fn assert_accepted(result: &SubmissionResult) {
        assert!(
            matches!(result, SubmissionResult::Accepted { .. }),
            "unexpected result: {result:#?}"
        );
    }

    #[test]
    fn two_new_entities_at_one_place_are_independent() {
        let mut lab = SubjectConflictLab::default();
        let first = intent(
            &lab,
            101,
            "create-reed",
            IVO,
            Operation::CreateEntity {
                entity_id: 20,
                place_id: PlaceId::OldQuarry,
            },
        );
        let second = intent(
            &lab,
            102,
            "create-rope",
            IVO,
            Operation::CreateEntity {
                entity_id: 21,
                place_id: PlaceId::OldQuarry,
            },
        );

        assert_accepted(&lab.submit(first));
        assert_accepted(&lab.submit(second));

        assert!(lab.entity.contains_key(&20), "{lab:#?}");
        assert!(lab.entity.contains_key(&21), "{lab:#?}");
        assert_eq!(lab.activity.len(), 2, "{lab:#?}");
    }

    #[test]
    fn property_changes_on_different_entities_are_independent() {
        let mut lab = SubjectConflictLab::default();
        let stone = intent(
            &lab,
            201,
            "stone-falls",
            IVO,
            Operation::SetProperties {
                entity_id: STONE,
                changes: vec![change(
                    "state",
                    lab.property_expectation(STONE, "state"),
                    "fallen",
                )],
            },
        );
        let door = intent(
            &lab,
            202,
            "door-opens",
            IVO,
            Operation::SetProperties {
                entity_id: DOOR,
                changes: vec![change(
                    "state",
                    lab.property_expectation(DOOR, "state"),
                    "open",
                )],
            },
        );

        assert_accepted(&lab.submit(stone));
        assert_accepted(&lab.submit(door));

        assert_eq!(lab.entity[&STONE].properties["state"].version, 2);
        assert_eq!(lab.entity[&DOOR].properties["state"].version, 2);
        assert_eq!(lab.activity.len(), 2, "{lab:#?}");
    }

    #[test]
    fn different_properties_on_one_entity_are_independent() {
        let mut lab = SubjectConflictLab::default();
        let state = intent(
            &lab,
            301,
            "stone-falls",
            IVO,
            Operation::SetProperties {
                entity_id: STONE,
                changes: vec![change(
                    "state",
                    lab.property_expectation(STONE, "state"),
                    "fallen",
                )],
            },
        );
        let color = intent(
            &lab,
            302,
            "stone-darkens",
            IVO,
            Operation::SetProperties {
                entity_id: STONE,
                changes: vec![change(
                    "color",
                    lab.property_expectation(STONE, "color"),
                    "black",
                )],
            },
        );

        assert_accepted(&lab.submit(state));
        assert_accepted(&lab.submit(color));

        assert_eq!(lab.entity[&STONE].properties["state"].version, 2);
        assert_eq!(lab.entity[&STONE].properties["color"].version, 2);
        assert_eq!(lab.activity.len(), 2, "{lab:#?}");
    }

    #[test]
    fn same_property_snapshot_allows_one_winner_and_one_conflict() {
        let initial = SubjectConflictLab::default();
        let left = intent(
            &initial,
            401,
            "stone-falls",
            IVO,
            Operation::SetProperties {
                entity_id: STONE,
                changes: vec![change(
                    "state",
                    initial.property_expectation(STONE, "state"),
                    "fallen",
                )],
            },
        );
        let right = intent(
            &initial,
            402,
            "stone-rolls",
            NIA,
            Operation::SetProperties {
                entity_id: STONE,
                changes: vec![change(
                    "state",
                    initial.property_expectation(STONE, "state"),
                    "rolling",
                )],
            },
        );

        let mut left_first = initial.clone();
        assert_accepted(&left_first.submit(left.clone()));
        assert!(matches!(
            left_first.submit(right.clone()),
            SubmissionResult::Conflict(Conflict::PropertyChanged { .. })
        ));
        assert_eq!(left_first.activity.len(), 1, "{left_first:#?}");

        let mut right_first = initial;
        assert_accepted(&right_first.submit(right));
        assert!(matches!(
            right_first.submit(left),
            SubmissionResult::Conflict(Conflict::PropertyChanged { .. })
        ));
        assert_eq!(right_first.activity.len(), 1, "{right_first:#?}");
    }

    #[test]
    fn same_expected_absent_property_allows_one_winner_and_one_conflict() {
        let initial = SubjectConflictLab::default();
        let expectation = initial.property_expectation(STONE, "mark");
        assert_eq!(expectation, PropertyExpectation::Absent);
        let first = intent(
            &initial,
            501,
            "mark-circle",
            IVO,
            Operation::SetProperties {
                entity_id: STONE,
                changes: vec![change("mark", expectation.clone(), "circle")],
            },
        );
        let second = intent(
            &initial,
            502,
            "mark-cross",
            NIA,
            Operation::SetProperties {
                entity_id: STONE,
                changes: vec![change("mark", expectation, "cross")],
            },
        );

        let mut lab = initial;
        assert_accepted(&lab.submit(first));
        assert!(matches!(
            lab.submit(second),
            SubmissionResult::Conflict(Conflict::PropertyChanged { .. })
        ));

        assert_eq!(lab.entity[&STONE].properties["mark"].value, "circle");
        assert_eq!(lab.entity[&STONE].properties["mark"].version, 1);
        assert_eq!(lab.activity.len(), 1, "{lab:#?}");
    }

    #[test]
    fn activity_only_interactions_at_one_place_do_not_conflict() {
        let mut lab = SubjectConflictLab::default();
        let ivo = intent(
            &lab,
            601,
            "ivo-speaks",
            IVO,
            Operation::AppendInteraction {
                place_id: PlaceId::OldQuarry,
                summary: "Ivo calls toward the ridge.".to_owned(),
            },
        );
        let nia = intent(
            &lab,
            602,
            "nia-replies",
            NIA,
            Operation::AppendInteraction {
                place_id: PlaceId::OldQuarry,
                summary: "Nia answers from the path.".to_owned(),
            },
        );
        let entity_before = lab.entity.clone();

        assert_accepted(&lab.submit(ivo));
        assert_accepted(&lab.submit(nia));

        assert_eq!(lab.entity, entity_before, "{lab:#?}");
        assert_eq!(lab.activity.len(), 2, "{lab:#?}");
        assert!(
            lab.activity
                .iter()
                .all(|activity| activity.kind == ActivityKind::Interaction),
            "{lab:#?}"
        );
    }

    #[test]
    fn stale_actor_placement_rejects_the_prepared_intent() {
        let mut lab = SubjectConflictLab::default();
        let prepared = intent(
            &lab,
            701,
            "stone-falls",
            IVO,
            Operation::SetProperties {
                entity_id: STONE,
                changes: vec![change(
                    "state",
                    lab.property_expectation(STONE, "state"),
                    "fallen",
                )],
            },
        );

        lab.move_character_fixture(IVO, PlaceId::QuietGrove);
        assert_eq!(
            lab.submit(prepared),
            SubmissionResult::Conflict(Conflict::StaleActorPlacement)
        );
        assert!(lab.accepted_request.is_empty(), "{lab:#?}");
        assert!(lab.activity.is_empty(), "{lab:#?}");
        assert_eq!(lab.entity[&STONE].properties["state"].value, "standing");
    }

    #[test]
    fn equal_retry_replays_and_changed_fingerprint_conflicts() {
        let mut lab = SubjectConflictLab::default();
        let accepted = intent(
            &lab,
            801,
            "stone-falls",
            IVO,
            Operation::SetProperties {
                entity_id: STONE,
                changes: vec![change(
                    "state",
                    lab.property_expectation(STONE, "state"),
                    "fallen",
                )],
            },
        );
        let first_result = lab.submit(accepted.clone());
        assert_accepted(&first_result);

        assert_eq!(lab.submit(accepted.clone()), first_result, "{lab:#?}");
        let mut changed = accepted;
        changed.fingerprint = "stone-rolls".to_owned();
        assert_eq!(
            lab.submit(changed),
            SubmissionResult::Conflict(Conflict::RequestFingerprintChanged)
        );

        assert_eq!(lab.activity.len(), 1, "{lab:#?}");
        assert_eq!(lab.accepted_request.len(), 1, "{lab:#?}");
        assert_eq!(lab.entity[&STONE].properties["state"].version, 2);
    }

    #[test]
    fn rejected_request_leaves_no_record_and_can_be_corrected() {
        let mut lab = SubjectConflictLab::default();
        let initial_property = lab.property_expectation(STONE, "state");
        let first = intent(
            &lab,
            811,
            "stone-falls",
            IVO,
            Operation::SetProperties {
                entity_id: STONE,
                changes: vec![change("state", initial_property.clone(), "fallen")],
            },
        );
        assert_accepted(&lab.submit(first));

        let stale = intent(
            &lab,
            812,
            "stale-roll",
            IVO,
            Operation::SetProperties {
                entity_id: STONE,
                changes: vec![change("state", initial_property, "rolling")],
            },
        );
        assert!(matches!(
            lab.submit(stale),
            SubmissionResult::Conflict(Conflict::PropertyChanged { .. })
        ));
        assert!(!lab.accepted_request.contains_key(&812), "{lab:#?}");
        assert_eq!(lab.activity.len(), 1, "{lab:#?}");

        let corrected = intent(
            &lab,
            812,
            "corrected-roll",
            IVO,
            Operation::SetProperties {
                entity_id: STONE,
                changes: vec![change(
                    "state",
                    lab.property_expectation(STONE, "state"),
                    "rolling",
                )],
            },
        );
        assert_accepted(&lab.submit(corrected));

        assert_eq!(lab.accepted_request.len(), 2, "{lab:#?}");
        assert_eq!(lab.activity.len(), 2, "{lab:#?}");
        assert_eq!(lab.entity[&STONE].properties["state"].value, "rolling");
        assert_eq!(lab.entity[&STONE].properties["state"].version, 3);
    }

    #[test]
    fn dependencies_are_canonical_and_never_include_a_place_revision() {
        let mut lab = SubjectConflictLab::default();
        let prepared = intent(
            &lab,
            901,
            "stone-state-and-color",
            IVO,
            Operation::SetProperties {
                entity_id: STONE,
                changes: vec![
                    change("state", lab.property_expectation(STONE, "state"), "fallen"),
                    change("color", lab.property_expectation(STONE, "color"), "black"),
                ],
            },
        );

        let result = lab.submit(prepared);
        let SubmissionResult::Accepted {
            checked_dependencies,
            ..
        } = result
        else {
            panic!("expected accepted result, got {result:#?}");
        };
        assert_eq!(
            checked_dependencies,
            vec![
                DependencyKey::CharacterPlacement(IVO),
                DependencyKey::Property(STONE, "color".to_owned()),
                DependencyKey::Property(STONE, "state".to_owned()),
            ]
        );
        assert_eq!(
            lab.activity[0].checked_dependencies, checked_dependencies,
            "{lab:#?}"
        );
        assert_eq!(lab.activity.len(), 1, "{lab:#?}");
    }
}
