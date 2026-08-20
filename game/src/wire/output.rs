use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub struct WorldOutput {
    /// Name of the one shared World.
    pub name: String,
}

impl From<WorldView> for WorldOutput {
    fn from(value: WorldView) -> Self {
        Self { name: value.name }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub struct UserOutput {
    /// User id.
    pub id: Uuid,
    /// When the User was provisioned.
    pub created_at: DateTime<Utc>,
}

impl From<User> for UserOutput {
    fn from(value: User) -> Self {
        Self {
            id: value.id.0,
            created_at: value.created_at,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub struct EntityOutput {
    /// Entity id; also the id of its Character or Place role.
    pub id: Uuid,
    /// Current name.
    pub name: String,
    /// Current description.
    pub description: String,
    /// User who introduced this Entity.
    pub introduced_by_user_id: Uuid,
    /// When the Entity was introduced.
    pub introduced_at: DateTime<Utc>,
}

impl From<Entity> for EntityOutput {
    fn from(value: Entity) -> Self {
        Self {
            id: value.id.0,
            name: value.name,
            description: value.description,
            introduced_by_user_id: value.introduced_by_user_id.0,
            introduced_at: value.introduced_at,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub struct CharacterOutput {
    /// The Character's Entity; its id is the Character id.
    pub entity: EntityOutput,
    /// User who owns this Character.
    pub owner_user_id: Uuid,
    /// Exact current Position, or null before World entry.
    #[schemars(schema_with = "nullable_position_schema", required)]
    #[schema(required = true, nullable = true)]
    pub position: Option<PositionOutput>,
    /// Current Place, or null before entry and between Places.
    #[schemars(schema_with = "nullable_place_schema", required)]
    #[schema(required = true, nullable = true)]
    pub current_place: Option<PlaceOutput>,
}

impl From<Character> for CharacterOutput {
    fn from(value: Character) -> Self {
        Self {
            entity: value.entity.into(),
            owner_user_id: value.owner_user_id.0,
            position: value.position.map(Into::into),
            current_place: value.current_place.map(Into::into),
        }
    }
}

fn nullable_position_schema(generator: &mut SchemaGenerator) -> Schema {
    let position = generator.subschema_for::<PositionOutput>();
    schemars::json_schema!({"oneOf": [position, {"type": "null"}]})
}

fn nullable_place_schema(generator: &mut SchemaGenerator) -> Schema {
    let place = generator.subschema_for::<PlaceOutput>();
    schemars::json_schema!({"oneOf": [place, {"type": "null"}]})
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub struct PlaceOutput {
    /// The Place's Entity; its id is the Place id.
    pub entity: EntityOutput,
    /// Exact current Position.
    pub position: PositionOutput,
    /// True only for the one entry Place.
    pub is_entry: bool,
}

impl From<Place> for PlaceOutput {
    fn from(value: Place) -> Self {
        Self {
            entity: value.entity.into(),
            position: value.position.into(),
            is_entry: value.is_entry,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub struct EntitySummaryOutput {
    /// Entity id.
    pub id: Uuid,
    /// Current name.
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub struct CurrentPlaceEntityOutput {
    /// Entity id.
    pub id: Uuid,
    /// Current name.
    pub name: String,
    /// Current description.
    pub description: String,
    /// Exact current Position.
    pub position: PositionOutput,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub struct CurrentPlaceOutput {
    /// Place id.
    pub id: Uuid,
    /// Current name.
    pub name: String,
    /// Current description.
    pub description: String,
    /// Exact current Position.
    pub position: PositionOutput,
}

impl From<Place> for CurrentPlaceOutput {
    fn from(value: Place) -> Self {
        Self {
            id: value.entity.id.0,
            name: value.entity.name,
            description: value.entity.description,
            position: value.position.into(),
        }
    }
}

impl From<CurrentPlaceEntity> for CurrentPlaceEntityOutput {
    fn from(value: CurrentPlaceEntity) -> Self {
        Self {
            id: value.id.0,
            name: value.name,
            description: value.description,
            position: value.position.into(),
        }
    }
}

impl From<EntitySummary> for EntitySummaryOutput {
    fn from(value: EntitySummary) -> Self {
        Self {
            id: value.id.0,
            name: value.name,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub enum PropertyValueInput {
    /// Text value.
    Text {
        /// Text.
        #[schemars(length(min = 1, max = 4000))]
        #[schema(min_length = 1, max_length = 4000)]
        text: String,
    },
    /// Integer value.
    Integer { integer: i64 },
}

impl From<PropertyValueInput> for PropertyValue {
    fn from(value: PropertyValueInput) -> Self {
        match value {
            PropertyValueInput::Text { text } => Self::Text(text),
            PropertyValueInput::Integer { integer } => Self::Integer(integer),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub enum PropertyValueOutput {
    /// Text value.
    Text { text: String },
    /// Integer value.
    Integer { integer: i64 },
}

impl From<PropertyValue> for PropertyValueOutput {
    fn from(value: PropertyValue) -> Self {
        match value {
            PropertyValue::Text(text) => Self::Text { text },
            PropertyValue::Integer(integer) => Self::Integer { integer },
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub struct PropertyInput {
    /// English lower_snake_case key starting with a letter.
    #[schemars(length(min = 1, max = 64))]
    #[schema(min_length = 1, max_length = 64)]
    pub key: String,
    /// Text or integer value.
    pub value: PropertyValueInput,
}

impl From<PropertyInput> for WorldPropertyInput {
    fn from(value: PropertyInput) -> Self {
        Self {
            key: value.key,
            value: value.value.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub struct TraitInput {
    /// English statement of one new Trait.
    #[schemars(length(min = 1, max = 4000))]
    #[schema(min_length = 1, max_length = 4000)]
    pub statement: String,
}

impl From<TraitInput> for WorldTraitInput {
    fn from(value: TraitInput) -> Self {
        Self {
            statement: value.statement,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub struct EntityPropertyChangeInput {
    /// Local Entity id from your grounding reads.
    pub entity_id: Uuid,
    /// English lower_snake_case key starting with a letter.
    #[schemars(length(min = 1, max = 64))]
    #[schema(min_length = 1, max_length = 64)]
    pub key: String,
    /// Text or integer value.
    pub value: PropertyValueInput,
}

impl From<EntityPropertyChangeInput> for WorldPropertyChange {
    fn from(value: EntityPropertyChangeInput) -> Self {
        Self {
            entity_id: EntityId(value.entity_id),
            key: value.key,
            value: value.value.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub enum EntityTraitChangeInput {
    /// Establish a new Trait with its first statement.
    Establish {
        entity_id: Uuid,
        #[schemars(length(min = 1, max = 4000))]
        #[schema(min_length = 1, max_length = 4000)]
        statement: String,
    },
    /// Develop an existing Trait to a new current statement.
    Develop {
        trait_id: Uuid,
        #[schemars(length(min = 1, max = 4000))]
        #[schema(min_length = 1, max_length = 4000)]
        statement: String,
    },
}

impl From<EntityTraitChangeInput> for WorldTraitChange {
    fn from(value: EntityTraitChangeInput) -> Self {
        match value {
            EntityTraitChangeInput::Establish {
                entity_id,
                statement,
            } => Self::Establish {
                entity_id: EntityId(entity_id),
                statement,
            },
            EntityTraitChangeInput::Develop {
                trait_id,
                statement,
            } => Self::Develop {
                trait_id: EntityTraitId(trait_id),
                statement,
            },
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub struct EntityPropertyOutput {
    /// The Entity this Property belongs to.
    pub entity: EntitySummaryOutput,
    /// Property key.
    pub key: String,
    /// Value.
    pub value: PropertyValueOutput,
}

impl From<EntityPropertyChange> for EntityPropertyOutput {
    fn from(value: EntityPropertyChange) -> Self {
        Self {
            entity: value.entity.into(),
            key: value.key,
            value: value.value.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub struct EntityCurrentPropertyOutput {
    /// Property key.
    pub key: String,
    /// Current value.
    pub value: PropertyValueOutput,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub struct EntityTraitOutput {
    /// Trait id.
    pub id: Uuid,
    /// Current statement.
    pub statement: String,
}

impl From<EntityTrait> for EntityTraitOutput {
    fn from(value: EntityTrait) -> Self {
        Self {
            id: value.id.0,
            statement: value.statement,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub enum ActivityTraitChangeOutput {
    /// This Activity established a new Trait.
    Establish {
        entity: EntitySummaryOutput,
        r#trait: EntityTraitOutput,
    },
    /// This Activity developed an existing Trait.
    Develop {
        entity: EntitySummaryOutput,
        r#trait: EntityTraitOutput,
        previous_statement: String,
    },
}

impl From<ActivityTraitChange> for ActivityTraitChangeOutput {
    fn from(value: ActivityTraitChange) -> Self {
        match value {
            ActivityTraitChange::Establish { entity, r#trait } => Self::Establish {
                entity: entity.into(),
                r#trait: r#trait.into(),
            },
            ActivityTraitChange::Develop {
                entity,
                r#trait,
                previous_statement,
            } => Self::Develop {
                entity: entity.into(),
                r#trait: r#trait.into(),
                previous_statement,
            },
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub enum EntityCurrentAssociationOutput {
    Property {
        property: EntityCurrentPropertyOutput,
    },
    Trait {
        r#trait: EntityTraitOutput,
    },
}

impl From<EntityCurrentAssociation> for EntityCurrentAssociationOutput {
    fn from(value: EntityCurrentAssociation) -> Self {
        match value {
            EntityCurrentAssociation::Property { key, value } => Self::Property {
                property: EntityCurrentPropertyOutput {
                    key,
                    value: value.into(),
                },
            },
            EntityCurrentAssociation::Trait(r#trait) => Self::Trait {
                r#trait: r#trait.into(),
            },
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub struct EntityCurrentStatePageOutput {
    /// Current Properties, then Traits.
    pub association: Vec<EntityCurrentAssociationOutput>,
    /// Cursor for the next page, or null.
    #[schemars(schema_with = "nullable_string_schema", required)]
    #[schema(required = true, nullable = true)]
    pub next: Option<String>,
}

impl EntityCurrentStatePageOutput {
    fn from_character(value: EntityCurrentStatePage) -> Self {
        Self {
            association: value.association.into_iter().map(Into::into).collect(),
            next: value.next.map(encode_character_state_cursor),
        }
    }

    fn from_current_place_entity(value: EntityCurrentStatePage) -> Self {
        Self {
            association: value.association.into_iter().map(Into::into).collect(),
            next: value.next.map(encode_current_place_entity_state_cursor),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub struct CharacterEntityStatePageOutput {
    pub character: CharacterOutput,
    #[schemars(schema_with = "nullable_string_schema", required)]
    #[schema(required = true, nullable = true)]
    pub place_revision: Option<String>,
    pub current_state: EntityCurrentStatePageOutput,
}

impl From<CharacterEntityStatePage> for CharacterEntityStatePageOutput {
    fn from(value: CharacterEntityStatePage) -> Self {
        Self {
            character: value.character.into(),
            place_revision: value.place_revision.map(encode_place_revision),
            current_state: EntityCurrentStatePageOutput::from_character(value.current_state),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub struct PlaceSummaryOutput {
    /// The Place's Entity; its id is the Place id.
    pub entity: EntitySummaryOutput,
    /// True only for the one entry Place.
    pub is_entry: bool,
}

impl From<PlaceSummary> for PlaceSummaryOutput {
    fn from(value: PlaceSummary) -> Self {
        Self {
            entity: value.entity.into(),
            is_entry: value.is_entry,
        }
    }
}

fn nullable_string_schema(_: &mut SchemaGenerator) -> Schema {
    schemars::json_schema!({"type": ["string", "null"]})
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum ActivityOperationOutput {
    CreateCharacter,
    CreateEntity,
    CreateEntryPlace,
    EnterWorld,
    SubmitAction,
    SubmitInteraction,
    SubmitDiscovery,
    MoveCharacter,
}

impl From<ActivityOperation> for ActivityOperationOutput {
    fn from(value: ActivityOperation) -> Self {
        match value {
            ActivityOperation::CreateCharacter => Self::CreateCharacter,
            ActivityOperation::CreateEntity => Self::CreateEntity,
            ActivityOperation::CreateEntryPlace => Self::CreateEntryPlace,
            ActivityOperation::EnterWorld => Self::EnterWorld,
            ActivityOperation::SubmitAction => Self::SubmitAction,
            ActivityOperation::SubmitInteraction => Self::SubmitInteraction,
            ActivityOperation::SubmitDiscovery => Self::SubmitDiscovery,
            ActivityOperation::MoveCharacter => Self::MoveCharacter,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum ActivityEntityRoleOutput {
    Subject,
    Destination,
    Location,
    Target,
}

impl From<ActivityEntityRole> for ActivityEntityRoleOutput {
    fn from(value: ActivityEntityRole) -> Self {
        match value {
            ActivityEntityRole::Subject => Self::Subject,
            ActivityEntityRole::Destination => Self::Destination,
            ActivityEntityRole::Location => Self::Location,
            ActivityEntityRole::Target => Self::Target,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub struct ActivityEntityReferenceOutput {
    /// Involved Entity.
    pub entity: EntitySummaryOutput,
    /// Role of this Entity in the Activity: subject, destination, location or target.
    pub role: ActivityEntityRoleOutput,
}

impl From<ActivityEntityReference> for ActivityEntityReferenceOutput {
    fn from(value: ActivityEntityReference) -> Self {
        Self {
            entity: value.entity.into(),
            role: value.role.into(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum ActivityPositionRoleOutput {
    /// Position state before or grounding this Activity.
    Origin,
    /// Position state established by this Activity.
    Result,
}

impl From<crate::ActivityPositionRole> for ActivityPositionRoleOutput {
    fn from(value: crate::ActivityPositionRole) -> Self {
        match value {
            crate::ActivityPositionRole::Origin => Self::Origin,
            crate::ActivityPositionRole::Result => Self::Result,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub struct ActivityPositionReferenceOutput {
    /// Entity owning this exact Position version.
    pub entity: EntitySummaryOutput,
    /// Whether this Position grounds or results from the Activity.
    pub role: ActivityPositionRoleOutput,
    /// Exact immutable Position version involved.
    pub position: PositionOutput,
}

impl From<crate::ActivityPositionReference> for ActivityPositionReferenceOutput {
    fn from(value: crate::ActivityPositionReference) -> Self {
        Self {
            entity: value.entity.into(),
            role: value.role.into(),
            position: value.position.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub struct ActivityConnectionReferenceOutput {
    /// Stable involved Connection id.
    pub id: Uuid,
    /// Accepted Connection name.
    #[schemars(length(min = 1, max = 120))]
    #[schema(min_length = 1, max_length = 120)]
    pub name: String,
    /// Source Place id in stored Connection direction.
    pub source_place_id: Uuid,
    /// Destination Place id in stored Connection direction.
    pub destination_place_id: Uuid,
}

impl From<crate::ActivityConnectionReference> for ActivityConnectionReferenceOutput {
    fn from(value: crate::ActivityConnectionReference) -> Self {
        Self {
            id: value.connection_id.0,
            name: value.name,
            source_place_id: value.source_place_id.0,
            destination_place_id: value.destination_place_id.0,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub struct ActivityOutput {
    /// Activity id.
    pub id: Uuid,
    /// Name of the accepted operation.
    pub operation: ActivityOperationOutput,
    /// Acting Character, or null when none existed yet.
    #[schemars(schema_with = "nullable_entity_summary_schema", required)]
    #[schema(required = true, nullable = true)]
    pub actor_character: Option<EntitySummaryOutput>,
    /// Place where the action was accepted, or null; it never changes later.
    #[schemars(schema_with = "nullable_place_summary_schema", required)]
    #[schema(required = true, nullable = true)]
    pub context_place: Option<PlaceSummaryOutput>,
    /// Entities involved, each with its role.
    pub involved_entity: Vec<ActivityEntityReferenceOutput>,
    /// Exact Position versions involved in this Activity.
    #[schemars(length(max = 3))]
    #[schema(max_items = 3)]
    pub involved_position: Vec<ActivityPositionReferenceOutput>,
    /// Compact immutable Connections involved in this Activity.
    #[schemars(length(max = 1))]
    #[schema(max_items = 1)]
    pub involved_connection: Vec<ActivityConnectionReferenceOutput>,
    /// Property changes made by this Activity.
    pub property_change: Vec<EntityPropertyOutput>,
    /// Trait establishments and developments made by this Activity.
    pub trait_change: Vec<ActivityTraitChangeOutput>,
    /// Prose accepted with the action, or null.
    #[schemars(schema_with = "nullable_string_schema", required)]
    #[schema(required = true, nullable = true)]
    pub prose: Option<String>,
    /// When the World accepted this action.
    pub occurred_at: DateTime<Utc>,
}

fn nullable_entity_summary_schema(generator: &mut SchemaGenerator) -> Schema {
    let entity = generator.subschema_for::<EntitySummaryOutput>();
    schemars::json_schema!({"oneOf": [entity, {"type": "null"}]})
}

fn nullable_place_summary_schema(generator: &mut SchemaGenerator) -> Schema {
    let place = generator.subschema_for::<PlaceSummaryOutput>();
    schemars::json_schema!({"oneOf": [place, {"type": "null"}]})
}

impl From<Activity> for ActivityOutput {
    fn from(value: Activity) -> Self {
        Self {
            id: value.id.0,
            operation: value.operation.into(),
            actor_character: value.actor_character.map(Into::into),
            context_place: value.context_place.map(Into::into),
            involved_entity: value.involved_entity.into_iter().map(Into::into).collect(),
            involved_position: value
                .involved_position
                .into_iter()
                .map(Into::into)
                .collect(),
            involved_connection: value
                .involved_connection
                .into_iter()
                .map(Into::into)
                .collect(),
            property_change: value.property_change.into_iter().map(Into::into).collect(),
            trait_change: value.trait_change.into_iter().map(Into::into).collect(),
            prose: value.prose,
            occurred_at: value.occurred_at,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub struct ActivityPageOutput {
    /// Activities involving the current Character, newest first.
    pub activity: Vec<ActivityOutput>,
    /// Cursor for the next page, or null.
    #[schemars(schema_with = "nullable_string_schema", required)]
    #[schema(required = true, nullable = true)]
    pub next: Option<String>,
}

impl From<ActivityPage> for ActivityPageOutput {
    fn from(value: ActivityPage) -> Self {
        Self {
            activity: value.activity.into_iter().map(Into::into).collect(),
            next: value.next.map(encode_activity_cursor),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub struct CurrentPlaceEntityPageOutput {
    /// The current Place: id, name and description.
    pub place: CurrentPlaceOutput,
    /// Revision of the current Place; copy it unchanged into expected_place_revision.
    pub place_revision: String,
    /// Other Entities present at the current Place.
    pub entity: Vec<CurrentPlaceEntityOutput>,
    /// Cursor for the next page, or null.
    #[schemars(schema_with = "nullable_string_schema", required)]
    #[schema(required = true, nullable = true)]
    pub next: Option<String>,
}

impl From<CurrentPlaceEntityPage> for CurrentPlaceEntityPageOutput {
    fn from(value: CurrentPlaceEntityPage) -> Self {
        Self {
            place: value.place.into(),
            place_revision: encode_place_revision(value.place_revision),
            entity: value.entity.into_iter().map(Into::into).collect(),
            next: value.next.map(encode_place_entity_cursor),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub struct CurrentPlaceActivityPageOutput {
    /// The current Place: id, name and description.
    pub place: CurrentPlaceOutput,
    /// Revision of the current Place; every page used for one proposal must match.
    pub place_revision: String,
    /// Activity at the current Place, newest first.
    pub activity: Vec<ActivityOutput>,
    /// Cursor for the next page, or null.
    #[schemars(schema_with = "nullable_string_schema", required)]
    #[schema(required = true, nullable = true)]
    pub next: Option<String>,
}

impl From<CurrentPlaceActivityPage> for CurrentPlaceActivityPageOutput {
    fn from(value: CurrentPlaceActivityPage) -> Self {
        Self {
            place: value.place.into(),
            place_revision: encode_place_revision(value.place_revision),
            activity: value.activity.into_iter().map(Into::into).collect(),
            next: value.next.map(encode_place_activity_cursor),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub struct CurrentPlaceEntityStatePageOutput {
    /// The current Place.
    pub place: CurrentPlaceOutput,
    /// Revision of the current Place.
    pub place_revision: String,
    /// The selected Entity.
    pub entity: CurrentPlaceEntityOutput,
    /// One page of current Properties and Traits.
    pub current_state: EntityCurrentStatePageOutput,
}

impl From<CurrentPlaceEntityStatePage> for CurrentPlaceEntityStatePageOutput {
    fn from(value: CurrentPlaceEntityStatePage) -> Self {
        Self {
            place: value.place.into(),
            place_revision: encode_place_revision(value.place_revision),
            entity: value.entity.into(),
            current_state: EntityCurrentStatePageOutput::from_current_place_entity(
                value.current_state,
            ),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub struct AcceptedActionOutput {
    /// The accepted Activity with its prose.
    pub activity: ActivityOutput,
    /// The accepted consequence.
    pub consequence: AcceptedActionConsequenceOutput,
    /// The Place where the Action was accepted.
    pub place: PlaceOutput,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub enum AcceptedActionConsequenceOutput {
    IntroduceEntity {
        entity: EntityOutput,
    },
    ChangeEntityState {
        property_change: Vec<EntityPropertyOutput>,
        trait_change: Vec<ActivityTraitChangeOutput>,
    },
}

impl From<AcceptedActionConsequence> for AcceptedActionConsequenceOutput {
    fn from(value: AcceptedActionConsequence) -> Self {
        match value {
            AcceptedActionConsequence::IntroduceEntity(entity) => Self::IntroduceEntity {
                entity: entity.into(),
            },
            AcceptedActionConsequence::ChangeEntityState {
                property_change,
                trait_change,
            } => Self::ChangeEntityState {
                property_change: property_change.into_iter().map(Into::into).collect(),
                trait_change: trait_change.into_iter().map(Into::into).collect(),
            },
        }
    }
}

impl From<AcceptedAction> for AcceptedActionOutput {
    fn from(value: AcceptedAction) -> Self {
        Self {
            activity: value.activity.into(),
            consequence: value.consequence.into(),
            place: value.place.into(),
        }
    }
}
