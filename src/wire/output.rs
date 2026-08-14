use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub struct WorldOutput {
    /// Stable name of the one persistent shared World.
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
    /// Stable id of the User represented by the request context.
    pub id: Uuid,
    /// Time at which this durable User was provisioned.
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
    /// Stable Entity id used by every concrete role of this referent.
    pub id: Uuid,
    /// Current semantic name of this shared referent.
    pub name: String,
    /// Current semantic description of this shared referent.
    pub description: String,
    /// User whose accepted request first introduced this Entity.
    pub introduced_by_user_id: Uuid,
    /// Time at which this Entity was introduced into the shared World.
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
    /// Complete shared Entity that also has the Character role. Its Entity id is
    /// the Character's only id.
    pub entity: EntityOutput,
    /// User that exclusively owns this Character role.
    pub owner_user_id: Uuid,
    /// Complete current Place. Null means the Character exists but has not entered
    /// the World; create_character deliberately returns null.
    #[schemars(schema_with = "nullable_place_schema", required)]
    #[schema(required = true, nullable = true)]
    pub current_place: Option<PlaceOutput>,
}

impl From<Character> for CharacterOutput {
    fn from(value: Character) -> Self {
        Self {
            entity: value.entity.into(),
            owner_user_id: value.owner_user_id.0,
            current_place: value.current_place.map(Into::into),
        }
    }
}

fn nullable_place_schema(generator: &mut SchemaGenerator) -> Schema {
    let place = generator.subschema_for::<PlaceOutput>();
    schemars::json_schema!({"oneOf": [place, {"type": "null"}]})
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub struct PlaceOutput {
    /// Complete shared Entity that also has the Place role. Its Entity id is the
    /// Place's only id.
    pub entity: EntityOutput,
    /// True only for the one server-recognized World entry Place.
    pub is_entry: bool,
}

impl From<Place> for PlaceOutput {
    fn from(value: Place) -> Self {
        Self {
            entity: value.entity.into(),
            is_entry: value.is_entry,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub struct EntitySummaryOutput {
    /// Stable Entity id, including when the Entity also has a Character or Place
    /// role.
    pub id: Uuid,
    /// Current semantic name of the Entity.
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub struct CurrentPlaceEntityOutput {
    /// Stable id of this selectable local Entity.
    pub id: Uuid,
    /// Safe current name of this selectable local Entity.
    pub name: String,
    /// Safe current description of this selectable local Entity.
    pub description: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub struct CurrentPlaceOutput {
    /// Stable id of the exact current Place Entity.
    pub id: Uuid,
    /// Safe current name of the exact current Place.
    pub name: String,
    /// Safe current description of the exact current Place.
    pub description: String,
}

impl From<Place> for CurrentPlaceOutput {
    fn from(value: Place) -> Self {
        Self {
            id: value.entity.id.0,
            name: value.entity.name,
            description: value.entity.description,
        }
    }
}

impl From<CurrentPlaceEntity> for CurrentPlaceEntityOutput {
    fn from(value: CurrentPlaceEntity) -> Self {
        Self {
            id: value.id.0,
            name: value.name,
            description: value.description,
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
    /// Bounded current text established for the canonical key.
    Text {
        /// World trims this value and accepts 1 through 4,000 non-NUL characters.
        #[schemars(length(min = 1, max = 4000))]
        #[schema(min_length = 1, max_length = 4000)]
        text: String,
    },
    /// Signed whole-number current value established for the canonical key.
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
    /// Canonical current text value.
    Text { text: String },
    /// Canonical current signed whole-number value.
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
    /// Canonical English lower-snake-case key. World accepts 1 through 64 ASCII
    /// characters, starting with a letter.
    #[schemars(length(min = 1, max = 64))]
    #[schema(min_length = 1, max_length = 64)]
    pub key: String,
    /// One strict tagged text or integer value.
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
pub struct EntityPropertyChangeInput {
    /// Exact local Entity selected from current grounded context.
    pub entity_id: Uuid,
    /// Canonical English lower-snake-case key.
    #[schemars(length(min = 1, max = 64))]
    #[schema(min_length = 1, max_length = 64)]
    pub key: String,
    /// One strict tagged text or integer value.
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
    /// Establish the first immutable statement of a World-assigned Trait lineage.
    Establish {
        entity_id: Uuid,
        #[schemars(length(min = 1, max = 4000))]
        #[schema(min_length = 1, max_length = 4000)]
        statement: String,
    },
    /// Advance one stable Trait lineage to a new immutable current statement.
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
    /// Safe summary of the Entity whose current state changed or is being read.
    pub entity: EntitySummaryOutput,
    /// Canonical Property key; no internal key id is exposed.
    pub key: String,
    /// Exact typed value established by the Activity or held currently.
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
    /// Canonical Property key; the internal Property-key identity is never exposed.
    pub key: String,
    /// Exact current typed value.
    pub value: PropertyValueOutput,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub struct EntityTraitOutput {
    /// World-assigned stable identity of one Trait lineage.
    pub id: Uuid,
    /// Current non-executable statement for this Trait lineage.
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
    /// This Activity established the first immutable version of a new Trait.
    Establish {
        entity: EntitySummaryOutput,
        r#trait: EntityTraitOutput,
    },
    /// This Activity advanced an existing Trait lineage to a new immutable version.
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
    /// Combined bounded current state, Properties before Traits.
    pub association: Vec<EntityCurrentAssociationOutput>,
    /// Opaque operation-specific continuation, or null when complete.
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
    /// Shared Entity summary for this Place; the Entity id is also the Place id.
    pub entity: EntitySummaryOutput,
    /// True only for the one server-recognized World entry Place.
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub struct EntityPageOutput {
    /// Shared Entity summaries in newest-to-oldest order.
    pub entity: Vec<EntitySummaryOutput>,
    /// Opaque cursor for the following page, or null when no following page exists.
    /// Copy it unchanged into list_entity.cursor.
    #[schemars(schema_with = "nullable_string_schema", required)]
    #[schema(required = true, nullable = true)]
    pub next: Option<String>,
}

fn nullable_string_schema(_: &mut SchemaGenerator) -> Schema {
    schemars::json_schema!({"type": ["string", "null"]})
}

impl From<EntityPage> for EntityPageOutput {
    fn from(value: EntityPage) -> Self {
        Self {
            entity: value.entity.into_iter().map(Into::into).collect(),
            next: value.next.map(encode_cursor),
        }
    }
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
    /// Shared Entity involved in this immutable accepted Activity.
    pub entity: EntitySummaryOutput,
    /// Server-owned meaning of this Entity in the Activity: subject is what an
    /// action introduced or acted on; destination is where entry placed the
    /// Character; location is where the Activity happened; target is where the
    /// actor directed Interaction behavior and never establishes perception,
    /// consent, agreement, thought or response.
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub struct ActivityOutput {
    /// Stable id of this immutable accepted World Activity.
    pub id: Uuid,
    /// Server-owned name of the accepted World operation.
    pub operation: ActivityOperationOutput,
    /// Character that performed the action in the World, or null when no Character
    /// actor existed yet. This is an Entity summary because Character is an Entity
    /// role.
    #[schemars(schema_with = "nullable_entity_summary_schema", required)]
    #[schema(required = true, nullable = true)]
    pub actor_character: Option<EntitySummaryOutput>,
    /// Place at which World accepted the action, or null when the Character was
    /// unplaced or no Character actor existed. This historical context does not
    /// change when the Character later moves.
    #[schemars(schema_with = "nullable_place_summary_schema", required)]
    #[schema(required = true, nullable = true)]
    pub context_place: Option<PlaceSummaryOutput>,
    /// Shared Entities linked to the action with explicit server-owned roles.
    pub involved_entity: Vec<ActivityEntityReferenceOutput>,
    /// Exact typed Property changes established by this Activity, sorted by Entity
    /// id and canonical key. Empty when this Activity changed no Property.
    pub property_change: Vec<EntityPropertyOutput>,
    /// Exact Trait establishments/developments caused by this Activity. Entity
    /// references remain compact and current state is not recursively hydrated.
    pub trait_change: Vec<ActivityTraitChangeOutput>,
    /// Canonical readable text accepted with submit_action or submit_interaction,
    /// or null for every other operation.
    #[schemars(schema_with = "nullable_string_schema", required)]
    #[schema(required = true, nullable = true)]
    pub prose: Option<String>,
    /// Time at which World accepted this action.
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
    /// Activities involving the current Character, newest to oldest. Involvement
    /// includes acting or being linked by an explicit role.
    pub activity: Vec<ActivityOutput>,
    /// Opaque cursor for the following page, or null when no following page exists.
    /// Copy it unchanged into list_activity.cursor.
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
    /// Flat safe id, name and description of the exact current Place derived from
    /// the current Character; complete Entity provenance and entry status are omitted.
    pub place: CurrentPlaceOutput,
    /// Opaque strong revision for this exact Place representation. Copy it
    /// unchanged into submit_action.expected_place_revision or
    /// submit_interaction.expected_place_revision.
    pub place_revision: String,
    /// Safe target facts for other Characters and ordinary Entities at this exact Place.
    pub entity: Vec<CurrentPlaceEntityOutput>,
    /// Opaque cursor for the following page, or null when no following page exists.
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
    /// Flat safe id, name and description of the exact current Place derived from
    /// the current Character; complete Entity provenance and entry status are omitted.
    pub place: CurrentPlaceOutput,
    /// Opaque strong revision for this exact Place representation. Pages used to
    /// ground one action must agree on this value.
    pub place_revision: String,
    /// Canonical Activity at or involving this exact Place, newest first.
    pub activity: Vec<ActivityOutput>,
    /// Opaque cursor for the following page, or null when no following page exists.
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
    /// Safe exact current Place derived from the current Character.
    pub place: CurrentPlaceOutput,
    /// Opaque strong revision shared by exact-current-Place reads.
    pub place_revision: String,
    /// One selected exact-local Entity with compact safe fields.
    pub entity: CurrentPlaceEntityOutput,
    /// One combined bounded current Property/Trait association page.
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
    /// One immutable Activity containing the canonical accepted prose.
    pub activity: ActivityOutput,
    /// Exact tagged consequence accepted by World.
    pub consequence: AcceptedActionConsequenceOutput,
    /// Exact Place at which World accepted the Action.
    pub place: PlaceOutput,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub enum AcceptedActionConsequenceOutput {
    IntroduceEntity {
        entity: EntityOutput,
    },
    ChangeEntityProperty {
        property_change: Vec<EntityPropertyOutput>,
    },
    ChangeEntityTrait {
        trait_change: Vec<ActivityTraitChangeOutput>,
    },
}

impl From<AcceptedActionConsequence> for AcceptedActionConsequenceOutput {
    fn from(value: AcceptedActionConsequence) -> Self {
        match value {
            AcceptedActionConsequence::IntroduceEntity(entity) => Self::IntroduceEntity {
                entity: entity.into(),
            },
            AcceptedActionConsequence::ChangeEntityProperty(property_change) => {
                Self::ChangeEntityProperty {
                    property_change: property_change.into_iter().map(Into::into).collect(),
                }
            }
            AcceptedActionConsequence::ChangeEntityTrait(trait_change) => Self::ChangeEntityTrait {
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
