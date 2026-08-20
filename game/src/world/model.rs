use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, sqlx::Type)]
#[sqlx(transparent)]
pub struct UserId(pub Uuid);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, sqlx::Type)]
#[sqlx(transparent)]
pub struct EntityId(pub Uuid);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, sqlx::Type)]
#[sqlx(transparent)]
pub struct EntityTraitId(pub Uuid);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, sqlx::Type)]
#[sqlx(transparent)]
pub struct ActivityId(pub Uuid);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, sqlx::Type)]
#[sqlx(transparent)]
pub struct ConnectionId(pub Uuid);

#[derive(Clone, Debug, PartialEq, Eq, FromRow)]
pub struct User {
    pub id: UserId,
    pub created_at: DateTime<Utc>,
}

#[derive(Clone, Debug, PartialEq, Eq, FromRow)]
pub struct Entity {
    pub id: EntityId,
    pub name: String,
    pub description: String,
    pub introduced_by_user_id: UserId,
    pub introduced_at: DateTime<Utc>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Character {
    pub entity: Entity,
    pub owner_user_id: UserId,
    pub position: Option<Position>,
    pub current_place: Option<Place>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Place {
    pub entity: Entity,
    pub position: Position,
    pub is_entry: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PositionRevision {
    entity_id: EntityId,
    activity_id: ActivityId,
}

impl PositionRevision {
    pub fn from_parts(entity_id: EntityId, activity_id: ActivityId) -> Self {
        Self {
            entity_id,
            activity_id,
        }
    }

    pub fn entity_id(self) -> EntityId {
        self.entity_id
    }

    pub fn activity_id(self) -> ActivityId {
        self.activity_id
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Position {
    pub x_cm: i64,
    pub y_cm: i64,
    pub z_cm: i64,
    pub description: Option<String>,
    pub position_revision: PositionRevision,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConnectionPoint {
    pub ordinal: u16,
    pub x_cm: i64,
    pub y_cm: i64,
    pub z_cm: i64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConnectionEndpoint {
    pub place: PlacePosition,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PlacePosition {
    pub id: EntityId,
    pub name: String,
    pub description: String,
    pub is_entry: bool,
    pub position: Position,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConnectionSummary {
    pub id: ConnectionId,
    pub source: ConnectionEndpoint,
    pub destination: ConnectionEndpoint,
    pub allows_reverse: bool,
    pub name: String,
    pub description: String,
    pub has_course: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Connection {
    pub id: ConnectionId,
    pub source: ConnectionEndpoint,
    pub destination: ConnectionEndpoint,
    pub allows_reverse: bool,
    pub name: String,
    pub description: String,
    pub shape_description: Option<String>,
    pub course: Vec<ConnectionPoint>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ListPlace {
    pub min_x_cm: i64,
    pub max_x_cm: i64,
    pub min_y_cm: i64,
    pub max_y_cm: i64,
    pub min_z_cm: i64,
    pub max_z_cm: i64,
    pub cursor: Option<String>,
    pub limit: u16,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PlacePage {
    pub place: Vec<PlacePosition>,
    pub next: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ListConnection {
    pub place_id: EntityId,
    pub cursor: Option<String>,
    pub limit: u16,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConnectionPage {
    pub place: PlacePosition,
    pub connection: Vec<ConnectionSummary>,
    pub next: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetConnection {
    pub place_id: EntityId,
    pub connection_id: ConnectionId,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EntitySummary {
    pub id: EntityId,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PlaceSummary {
    pub entity: EntitySummary,
    pub is_entry: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorldView {
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PropertyInput {
    pub key: String,
    pub value: PropertyValue,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TraitInput {
    pub statement: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EntityPropertyChangeInput {
    pub entity_id: EntityId,
    pub key: String,
    pub value: PropertyValue,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EntityPropertyChange {
    pub entity: EntitySummary,
    pub key: String,
    pub value: PropertyValue,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EntityTrait {
    pub id: EntityTraitId,
    pub statement: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EntityTraitChangeInput {
    Establish {
        entity_id: EntityId,
        statement: String,
    },
    Develop {
        trait_id: EntityTraitId,
        statement: String,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ActivityTraitChange {
    Establish {
        entity: EntitySummary,
        r#trait: EntityTrait,
    },
    Develop {
        entity: EntitySummary,
        r#trait: EntityTrait,
        previous_statement: String,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CreateEntity {
    pub name: String,
    pub description: String,
    pub property: Vec<PropertyInput>,
    pub r#trait: Vec<TraitInput>,
}

impl CreateEntity {
    pub(super) fn normalize(self) -> Result<Self, WorldError> {
        let (name, description) =
            normalize_entity_text(self.name, self.description, |field, reason| {
                WorldError::InvalidEntity { field, reason }
            })?;
        let property = normalize_property_input(self.property, PropertyField::Property)?;
        let r#trait = normalize_trait_input(self.r#trait)?;
        Ok(Self {
            name,
            description,
            property,
            r#trait,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CreateCharacter {
    pub name: String,
    pub description: String,
    pub property: Vec<PropertyInput>,
    pub r#trait: Vec<TraitInput>,
}

impl CreateCharacter {
    pub(super) fn normalize(self) -> Result<Self, WorldError> {
        let (name, description) =
            normalize_entity_text(self.name, self.description, |field, reason| {
                WorldError::InvalidCharacter { field, reason }
            })?;
        let property = normalize_property_input(self.property, PropertyField::Property)?;
        let r#trait = normalize_trait_input(self.r#trait)?;
        Ok(Self {
            name,
            description,
            property,
            r#trait,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CreateEntryPlace {
    pub name: String,
    pub description: String,
    pub property: Vec<PropertyInput>,
    pub r#trait: Vec<TraitInput>,
}

impl CreateEntryPlace {
    pub(super) fn normalize(self) -> Result<Self, WorldError> {
        let (name, description) =
            normalize_entity_text(self.name, self.description, |field, reason| {
                WorldError::InvalidPlace { field, reason }
            })?;
        let property = normalize_property_input(self.property, PropertyField::Property)?;
        let r#trait = normalize_trait_input(self.r#trait)?;
        Ok(Self {
            name,
            description,
            property,
            r#trait,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SubmitAction {
    pub request_id: Uuid,
    pub expected_place_revision: PlaceRevision,
    pub prose: String,
    pub consequence: ActionConsequence,
}

impl SubmitAction {
    pub(super) fn normalize(self) -> Result<Self, WorldError> {
        let prose = self.prose.trim().to_owned();
        let prose_reason = if prose.is_empty() {
            Some(InvalidReason::Empty)
        } else if prose.contains('\0') {
            Some(InvalidReason::ContainsNul)
        } else if prose.chars().count() > MAX_ACTION_PROSE_LENGTH {
            Some(InvalidReason::TooLong)
        } else {
            None
        };
        if let Some(reason) = prose_reason {
            return Err(WorldError::InvalidAction {
                field: ActionField::Prose,
                reason,
            });
        }
        let consequence = self.consequence.normalize()?;
        Ok(Self {
            request_id: self.request_id,
            expected_place_revision: self.expected_place_revision,
            prose,
            consequence,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IntroduceEntity {
    pub name: String,
    pub description: String,
    pub position_description: Option<String>,
    pub property: Vec<PropertyInput>,
    pub r#trait: Vec<TraitInput>,
}

impl IntroduceEntity {
    fn normalize(self) -> Result<Self, WorldError> {
        let (name, description) =
            normalize_entity_text(self.name, self.description, |field, reason| {
                WorldError::InvalidAction {
                    field: match field {
                        EntityField::Name => ActionField::ConsequenceName,
                        EntityField::Description => ActionField::ConsequenceDescription,
                    },
                    reason,
                }
            })?;
        let property = normalize_property_input(self.property, PropertyField::Property)?;
        let r#trait = normalize_trait_input(self.r#trait)?;
        let position_description = normalize_position_description(self.position_description)?;
        Ok(Self {
            name,
            description,
            position_description,
            property,
            r#trait,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChangeEntityState {
    pub property_change: Vec<EntityPropertyChangeInput>,
    pub trait_change: Vec<EntityTraitChangeInput>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ActionConsequence {
    IntroduceEntity(IntroduceEntity),
    ChangeEntityState(ChangeEntityState),
}

impl ActionConsequence {
    fn normalize(self) -> Result<Self, WorldError> {
        match self {
            Self::IntroduceEntity(input) => input.normalize().map(Self::IntroduceEntity),
            Self::ChangeEntityState(input) => {
                let writes = input
                    .property_change
                    .into_iter()
                    .map(|change| PropertyWrite {
                        entity_id: change.entity_id,
                        key: change.key,
                        value: change.value,
                    })
                    .collect();
                let writes = normalize_property_writes(writes, true).map_err(|error| {
                    map_property_normalization_error(error, PropertyField::PropertyChange)
                })?;
                let trait_change = normalize_trait_change_input(input.trait_change, true)?;
                if writes.is_empty() && trait_change.is_empty() {
                    return Err(WorldError::InvalidAction {
                        field: ActionField::Consequence,
                        reason: InvalidReason::Empty,
                    });
                }
                Ok(Self::ChangeEntityState(ChangeEntityState {
                    property_change: writes
                        .into_iter()
                        .map(|write| EntityPropertyChangeInput {
                            entity_id: write.entity_id,
                            key: write.key,
                            value: write.value,
                        })
                        .collect(),
                    trait_change: trait_input_from_writes(trait_change),
                }))
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PlaceRevision {
    place_entity_id: EntityId,
    occurred_at: DateTime<Utc>,
    activity_id: ActivityId,
}

impl PlaceRevision {
    pub fn from_parts(
        place_entity_id: EntityId,
        occurred_at: DateTime<Utc>,
        activity_id: ActivityId,
    ) -> Self {
        Self {
            place_entity_id,
            occurred_at,
            activity_id,
        }
    }

    pub fn place_entity_id(self) -> EntityId {
        self.place_entity_id
    }

    pub fn occurred_at(self) -> DateTime<Utc> {
        self.occurred_at
    }

    pub fn activity_id(self) -> ActivityId {
        self.activity_id
    }

    pub(super) fn fingerprint_bytes(self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(41);
        bytes.push(1);
        bytes.extend_from_slice(self.place_entity_id.0.as_bytes());
        bytes.extend_from_slice(&self.occurred_at.timestamp_micros().to_be_bytes());
        bytes.extend_from_slice(self.activity_id.0.as_bytes());
        bytes
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AcceptedAction {
    pub activity: Activity,
    pub consequence: AcceptedActionConsequence,
    pub place: Place,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AcceptedActionConsequence {
    IntroduceEntity(Entity),
    ChangeEntityState {
        property_change: Vec<EntityPropertyChange>,
        trait_change: Vec<ActivityTraitChange>,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SubmitInteraction {
    pub request_id: Uuid,
    pub expected_place_revision: PlaceRevision,
    pub prose: String,
    pub target_entity_id: Vec<EntityId>,
    pub property_change: Vec<EntityPropertyChangeInput>,
    pub trait_change: Vec<EntityTraitChangeInput>,
}

pub(super) struct NormalizedSubmitInteraction {
    pub(super) request_id: Uuid,
    pub(super) expected_place_revision: PlaceRevision,
    pub(super) prose: String,
    pub(super) target_entity_id: Vec<EntityId>,
    pub(super) has_duplicate_target: bool,
    pub(super) property_change: Vec<PropertyWrite>,
    pub(super) trait_change: Vec<TraitWrite>,
}

impl SubmitInteraction {
    pub(super) fn normalize(self) -> Result<NormalizedSubmitInteraction, WorldError> {
        let prose = self.prose.trim().to_owned();
        let prose_reason = if prose.is_empty() {
            Some(InvalidReason::Empty)
        } else if prose.contains('\0') {
            Some(InvalidReason::ContainsNul)
        } else if prose.chars().count() > MAX_INTERACTION_PROSE_LENGTH {
            Some(InvalidReason::TooLong)
        } else {
            None
        };
        if let Some(reason) = prose_reason {
            return Err(WorldError::InvalidInteraction {
                field: InteractionField::Prose,
                reason,
            });
        }
        if !(1..=MAX_INTERACTION_TARGET_COUNT).contains(&self.target_entity_id.len()) {
            return Err(WorldError::InvalidInteraction {
                field: InteractionField::TargetEntityId,
                reason: InvalidReason::OutOfRange,
            });
        }
        let mut target_entity_id = self.target_entity_id;
        target_entity_id.sort_unstable_by(|left, right| left.0.as_bytes().cmp(right.0.as_bytes()));
        let has_duplicate_target = target_entity_id.windows(2).any(|pair| pair[0] == pair[1]);
        let property_change = self
            .property_change
            .into_iter()
            .map(|change| PropertyWrite {
                entity_id: change.entity_id,
                key: change.key,
                value: change.value,
            })
            .collect();
        let property_change =
            normalize_property_writes(property_change, true).map_err(|error| {
                map_property_normalization_error(error, PropertyField::PropertyChange)
            })?;
        let trait_change = normalize_trait_change_input(self.trait_change, true)?;
        Ok(NormalizedSubmitInteraction {
            request_id: self.request_id,
            expected_place_revision: self.expected_place_revision,
            prose,
            target_entity_id,
            has_duplicate_target,
            property_change,
            trait_change,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AcceptedInteraction {
    pub activity: Activity,
    pub place: Place,
}

pub(super) fn normalize_entity_text(
    name: String,
    description: String,
    invalid: impl Fn(EntityField, InvalidReason) -> WorldError,
) -> Result<(String, String), WorldError> {
    let name = name.trim().to_owned();
    let description = description.trim().to_owned();
    for (field, value, maximum) in [
        (EntityField::Name, name.as_str(), MAX_ENTITY_NAME_LENGTH),
        (
            EntityField::Description,
            description.as_str(),
            MAX_ENTITY_DESCRIPTION_LENGTH,
        ),
    ] {
        let reason = if value.is_empty() {
            Some(InvalidReason::Empty)
        } else if value.contains('\0') {
            Some(InvalidReason::ContainsNul)
        } else if value.chars().count() > maximum {
            Some(InvalidReason::TooLong)
        } else {
            None
        };
        if let Some(reason) = reason {
            return Err(invalid(field, reason));
        }
    }
    Ok((name, description))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EntityCursor {
    pub introduced_at: DateTime<Utc>,
    pub entity_id: EntityId,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ListEntityAtCurrentPlace {
    pub cursor: Option<EntityCursor>,
    pub limit: u16,
}

impl Default for ListEntityAtCurrentPlace {
    fn default() -> Self {
        Self {
            cursor: None,
            limit: DEFAULT_PAGE_SIZE,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CurrentPlaceEntityPage {
    pub place: Place,
    pub place_revision: PlaceRevision,
    pub entity: Vec<CurrentPlaceEntity>,
    pub next: Option<EntityCursor>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CurrentPlaceEntity {
    pub id: EntityId,
    pub name: String,
    pub description: String,
    pub position: Position,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ListActivity {
    pub cursor: Option<ActivityCursor>,
    pub limit: u16,
}

impl Default for ListActivity {
    fn default() -> Self {
        Self {
            cursor: None,
            limit: DEFAULT_PAGE_SIZE,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ActivityCursor {
    pub occurred_at: DateTime<Utc>,
    pub activity_id: ActivityId,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActivityPage {
    pub activity: Vec<Activity>,
    pub next: Option<ActivityCursor>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ListActivityAtCurrentPlace {
    pub cursor: Option<ActivityCursor>,
    pub limit: u16,
}

impl Default for ListActivityAtCurrentPlace {
    fn default() -> Self {
        Self {
            cursor: None,
            limit: DEFAULT_PAGE_SIZE,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CurrentPlaceActivityPage {
    pub place: Place,
    pub place_revision: PlaceRevision,
    pub activity: Vec<Activity>,
    pub next: Option<ActivityCursor>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GetEntityCurrentState {
    pub cursor: Option<EntityCurrentStateCursor>,
    pub limit: u16,
}

impl Default for GetEntityCurrentState {
    fn default() -> Self {
        Self {
            cursor: None,
            limit: DEFAULT_PAGE_SIZE,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GetEntityAtCurrentPlace {
    pub entity_id: EntityId,
    pub cursor: Option<EntityCurrentStateCursor>,
    pub limit: u16,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum EntityCurrentStateCursorKey {
    Property(i64),
    Trait(EntityTraitId),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EntityCurrentStateCursor {
    pub(super) entity_id: EntityId,
    pub(super) place_revision: Option<PlaceRevision>,
    pub(super) key: EntityCurrentStateCursorKey,
}

impl EntityCurrentStateCursor {
    pub(crate) fn from_property(
        entity_id: EntityId,
        place_revision: Option<PlaceRevision>,
        property_key_id: i64,
    ) -> Self {
        Self {
            entity_id,
            place_revision,
            key: EntityCurrentStateCursorKey::Property(property_key_id),
        }
    }

    pub(crate) fn from_trait(
        entity_id: EntityId,
        place_revision: Option<PlaceRevision>,
        trait_id: EntityTraitId,
    ) -> Self {
        Self {
            entity_id,
            place_revision,
            key: EntityCurrentStateCursorKey::Trait(trait_id),
        }
    }

    pub(crate) fn entity_id(self) -> EntityId {
        self.entity_id
    }

    pub(crate) fn place_revision(self) -> Option<PlaceRevision> {
        self.place_revision
    }

    pub(crate) fn property_key_id(self) -> Option<i64> {
        match self.key {
            EntityCurrentStateCursorKey::Property(id) => Some(id),
            EntityCurrentStateCursorKey::Trait(_) => None,
        }
    }

    pub(crate) fn trait_id(self) -> Option<EntityTraitId> {
        match self.key {
            EntityCurrentStateCursorKey::Property(_) => None,
            EntityCurrentStateCursorKey::Trait(id) => Some(id),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EntityCurrentAssociation {
    Property { key: String, value: PropertyValue },
    Trait(EntityTrait),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EntityCurrentStatePage {
    pub association: Vec<EntityCurrentAssociation>,
    pub next: Option<EntityCurrentStateCursor>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CharacterEntityStatePage {
    pub character: Character,
    pub place_revision: Option<PlaceRevision>,
    pub current_state: EntityCurrentStatePage,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CurrentPlaceEntityStatePage {
    pub place: Place,
    pub place_revision: PlaceRevision,
    pub entity: CurrentPlaceEntity,
    pub current_state: EntityCurrentStatePage,
}
