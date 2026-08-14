use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use chrono::{DateTime, SecondsFormat, Utc};
use schemars::{JsonSchema, Schema, SchemaGenerator};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::{
    AcceptedAction, AcceptedActionConsequence, AcceptedInteraction, ActionConsequence, ActionField,
    Activity, ActivityCursor, ActivityEntityReference, ActivityEntityRole, ActivityId,
    ActivityOperation, ActivityPage, ActivityTraitChange, ChangeEntityProperty, ChangeEntityTrait,
    Character, CharacterEntityStatePage, CreateCharacter, CreateEntity, CreateEntryPlace,
    CurrentPlaceActivityPage, CurrentPlaceEntity, CurrentPlaceEntityPage,
    CurrentPlaceEntityStatePage, Entity, EntityCurrentAssociation, EntityCurrentStateCursor,
    EntityCurrentStatePage, EntityCursor, EntityField, EntityId, EntityPage, EntityPropertyChange,
    EntityPropertyChangeInput as WorldPropertyChange, EntitySummary, EntityTrait,
    EntityTraitChangeInput as WorldTraitChange, EntityTraitId, GetEntityAtCurrentPlace,
    GetEntityCurrentState, InteractionField, IntroduceEntity, InvalidReason, ListActivity,
    ListActivityAtCurrentPlace, ListEntity, ListEntityAtCurrentPlace, Place, PlaceRevision,
    PlaceSummary, PropertyField, PropertyInput as WorldPropertyInput, PropertyValue, SubmitAction,
    SubmitInteraction, User, UserId, WorldError, WorldView,
};

pub const USER_CONTEXT_HEADER: &str = "Aicadia-User-Id";

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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub struct GetEntityInput {
    /// Stable Entity id.
    #[schemars(with = "Uuid")]
    #[schema(value_type = Uuid)]
    pub entity_id: String,
}

impl GetEntityInput {
    pub fn parse(self) -> Result<EntityId, ErrorOutput> {
        Uuid::parse_str(&self.entity_id)
            .map(EntityId)
            .map_err(|_| ErrorOutput::invalid_entity_id())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema, IntoParams)]
#[serde(default, deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
#[into_params(parameter_in = Query)]
pub struct ListEntityInput {
    /// Opaque cursor returned as `next` by a previous list response.
    pub cursor: Option<String>,
    /// Page size. Defaults to 25. The World accepts values from 1 through 100.
    #[serde(default = "default_page_limit")]
    #[schemars(default = "default_page_limit", range(min = 1, max = 100))]
    #[param(default = 25, minimum = 1, maximum = 100)]
    pub limit: i64,
}

impl Default for ListEntityInput {
    fn default() -> Self {
        Self {
            cursor: None,
            limit: default_page_limit(),
        }
    }
}

impl ListEntityInput {
    pub fn parse(self) -> Result<ListEntity, ErrorOutput> {
        let limit = u16::try_from(self.limit)
            .map_err(|_| ErrorOutput::from_world(WorldError::InvalidEntityLimit))?;
        Ok(ListEntity {
            cursor: self.cursor.as_deref().map(decode_cursor).transpose()?,
            limit,
        })
    }
}

const fn default_page_limit() -> i64 {
    25
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema, IntoParams)]
#[serde(default, deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
#[into_params(parameter_in = Query)]
pub struct ListActivityInput {
    /// Opaque cursor returned as `next` by a previous activity page.
    pub cursor: Option<String>,
    /// Page size. Defaults to 25. The World accepts values from 1 through 100.
    #[serde(default = "default_page_limit")]
    #[schemars(default = "default_page_limit", range(min = 1, max = 100))]
    #[param(default = 25, minimum = 1, maximum = 100)]
    pub limit: i64,
}

impl Default for ListActivityInput {
    fn default() -> Self {
        Self {
            cursor: None,
            limit: default_page_limit(),
        }
    }
}

impl ListActivityInput {
    pub fn parse(self) -> Result<ListActivity, ErrorOutput> {
        let limit = u16::try_from(self.limit)
            .map_err(|_| ErrorOutput::from_world(WorldError::InvalidActivityLimit))?;
        Ok(ListActivity {
            cursor: self
                .cursor
                .as_deref()
                .map(decode_activity_cursor)
                .transpose()?,
            limit,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema, IntoParams)]
#[serde(default, deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
#[into_params(parameter_in = Query)]
pub struct ListEntityAtCurrentPlaceInput {
    /// Opaque cursor returned as `next` by a previous exact-Place Entity page.
    pub cursor: Option<String>,
    /// Page size. Defaults to 25. The World accepts values from 1 through 100.
    #[serde(default = "default_page_limit")]
    #[schemars(default = "default_page_limit", range(min = 1, max = 100))]
    #[param(default = 25, minimum = 1, maximum = 100)]
    pub limit: i64,
}

impl Default for ListEntityAtCurrentPlaceInput {
    fn default() -> Self {
        Self {
            cursor: None,
            limit: default_page_limit(),
        }
    }
}

impl ListEntityAtCurrentPlaceInput {
    pub fn parse(self) -> Result<ListEntityAtCurrentPlace, ErrorOutput> {
        let limit = u16::try_from(self.limit)
            .map_err(|_| ErrorOutput::from_world(WorldError::InvalidEntityLimit))?;
        Ok(ListEntityAtCurrentPlace {
            cursor: self
                .cursor
                .as_deref()
                .map(decode_place_entity_cursor)
                .transpose()?,
            limit,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema, IntoParams)]
#[serde(default, deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
#[into_params(parameter_in = Query)]
pub struct ListActivityAtCurrentPlaceInput {
    /// Opaque cursor returned as `next` by a previous exact-Place Activity page.
    pub cursor: Option<String>,
    /// Page size. Defaults to 25. The World accepts values from 1 through 100.
    #[serde(default = "default_page_limit")]
    #[schemars(default = "default_page_limit", range(min = 1, max = 100))]
    #[param(default = 25, minimum = 1, maximum = 100)]
    pub limit: i64,
}

impl Default for ListActivityAtCurrentPlaceInput {
    fn default() -> Self {
        Self {
            cursor: None,
            limit: default_page_limit(),
        }
    }
}

impl ListActivityAtCurrentPlaceInput {
    pub fn parse(self) -> Result<ListActivityAtCurrentPlace, ErrorOutput> {
        let limit = u16::try_from(self.limit)
            .map_err(|_| ErrorOutput::from_world(WorldError::InvalidActivityLimit))?;
        Ok(ListActivityAtCurrentPlace {
            cursor: self
                .cursor
                .as_deref()
                .map(decode_place_activity_cursor)
                .transpose()?,
            limit,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema, IntoParams)]
#[serde(default, deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
#[into_params(parameter_in = Query)]
pub struct GetEntityCurrentStateInput {
    /// Opaque cursor returned by the same full-Entity fetch.
    pub cursor: Option<String>,
    /// Combined Property/Trait page size. Defaults to 25; World accepts 1 through 100.
    #[serde(default = "default_page_limit")]
    #[schemars(default = "default_page_limit", range(min = 1, max = 100))]
    #[param(default = 25, minimum = 1, maximum = 100)]
    pub limit: i64,
}

impl Default for GetEntityCurrentStateInput {
    fn default() -> Self {
        Self {
            cursor: None,
            limit: default_page_limit(),
        }
    }
}

impl GetEntityCurrentStateInput {
    pub fn parse_character(self) -> Result<GetEntityCurrentState, ErrorOutput> {
        let limit = u16::try_from(self.limit)
            .map_err(|_| ErrorOutput::from_world(WorldError::InvalidEntityLimit))?;
        Ok(GetEntityCurrentState {
            cursor: self
                .cursor
                .as_deref()
                .map(decode_character_state_cursor)
                .transpose()?,
            limit,
        })
    }

    pub fn parse_current_place_entity(
        self,
        entity_id: EntityId,
    ) -> Result<GetEntityAtCurrentPlace, ErrorOutput> {
        let limit = u16::try_from(self.limit)
            .map_err(|_| ErrorOutput::from_world(WorldError::InvalidEntityLimit))?;
        Ok(GetEntityAtCurrentPlace {
            entity_id,
            cursor: self
                .cursor
                .as_deref()
                .map(decode_current_place_entity_state_cursor)
                .transpose()?,
            limit,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub struct GetEntityAtCurrentPlaceInput {
    /// Stable id selected from compact exact-current-Place orientation.
    pub entity_id: Uuid,
    /// Opaque cursor returned by this same scoped Entity fetch.
    #[serde(default)]
    pub cursor: Option<String>,
    /// Combined Property/Trait page size. Defaults to 25; World accepts 1 through 100.
    #[serde(default = "default_page_limit")]
    #[schemars(default = "default_page_limit", range(min = 1, max = 100))]
    #[schema(default = 25, minimum = 1, maximum = 100)]
    pub limit: i64,
}

impl GetEntityAtCurrentPlaceInput {
    pub fn parse(self) -> Result<GetEntityAtCurrentPlace, ErrorOutput> {
        GetEntityCurrentStateInput {
            cursor: self.cursor,
            limit: self.limit,
        }
        .parse_current_place_entity(EntityId(self.entity_id))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub struct CreateEntityInput {
    /// Display name. The World trims it and accepts 1 through 120 Unicode characters.
    #[schemars(length(min = 1, max = 120))]
    #[schema(min_length = 1, max_length = 120)]
    pub name: String,
    /// Description. The World trims it and accepts 1 through 4,000 Unicode characters.
    #[schemars(length(min = 1, max = 4000))]
    #[schema(min_length = 1, max_length = 4000)]
    pub description: String,
    #[serde(default)]
    #[schemars(length(max = 100))]
    #[schema(max_items = 100)]
    pub property: Vec<PropertyInput>,
}

impl From<CreateEntityInput> for CreateEntity {
    fn from(value: CreateEntityInput) -> Self {
        Self {
            name: value.name,
            description: value.description,
            property: value.property.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub struct CreateCharacterInput {
    /// Display name. The World trims it and accepts 1 through 120 Unicode characters.
    #[schemars(length(min = 1, max = 120))]
    #[schema(min_length = 1, max_length = 120)]
    pub name: String,
    /// Description. The World trims it and accepts 1 through 4,000 Unicode characters.
    #[schemars(length(min = 1, max = 4000))]
    #[schema(min_length = 1, max_length = 4000)]
    pub description: String,
    #[serde(default)]
    #[schemars(length(max = 100))]
    #[schema(max_items = 100)]
    pub property: Vec<PropertyInput>,
}

impl From<CreateCharacterInput> for CreateCharacter {
    fn from(value: CreateCharacterInput) -> Self {
        Self {
            name: value.name,
            description: value.description,
            property: value.property.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub struct CreateEntryPlaceInput {
    /// Display name. The World trims it and accepts 1 through 120 Unicode characters.
    #[schemars(length(min = 1, max = 120))]
    #[schema(min_length = 1, max_length = 120)]
    pub name: String,
    /// Description. The World trims it and accepts 1 through 4,000 Unicode characters.
    #[schemars(length(min = 1, max = 4000))]
    #[schema(min_length = 1, max_length = 4000)]
    pub description: String,
    #[serde(default)]
    #[schemars(length(max = 100))]
    #[schema(max_items = 100)]
    pub property: Vec<PropertyInput>,
}

impl From<CreateEntryPlaceInput> for CreateEntryPlace {
    fn from(value: CreateEntryPlaceInput) -> Self {
        Self {
            name: value.name,
            description: value.description,
            property: value.property.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub enum ActionConsequenceInput {
    /// Introduce one new Entity and establish it at the derived current Place.
    IntroduceEntity {
        /// Display name. World trims it and accepts 1 through 120 Unicode characters.
        #[schemars(length(min = 1, max = 120))]
        #[schema(min_length = 1, max_length = 120)]
        name: String,
        /// Description. World trims it and accepts 1 through 4,000 Unicode characters.
        #[schemars(length(min = 1, max = 4000))]
        #[schema(min_length = 1, max_length = 4000)]
        description: String,
        #[serde(default)]
        #[schemars(length(max = 100))]
        #[schema(max_items = 100)]
        property: Vec<PropertyInput>,
    },
    /// Change one or more typed Properties of exact local Entities atomically.
    ChangeEntityProperty {
        #[schemars(length(min = 1, max = 100))]
        #[schema(min_items = 1, max_items = 100)]
        property_change: Vec<EntityPropertyChangeInput>,
    },
    /// Establish and/or develop one through 100 exact-local Entity-owned Traits.
    ChangeEntityTrait {
        #[schemars(length(min = 1, max = 100))]
        #[schema(min_items = 1, max_items = 100)]
        trait_change: Vec<EntityTraitChangeInput>,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub struct SubmitActionInput {
    /// Agent-generated UUID for this one intended action. Reuse only for an
    /// uncertain delivery retry of byte-equivalent semantic input.
    pub request_id: Uuid,
    /// Opaque exact-Place revision copied unchanged from a grounded Place read.
    pub expected_place_revision: String,
    /// Exact canonical English prose previewed and explicitly confirmed by the User.
    #[schemars(length(min = 1, max = 4000))]
    #[schema(min_length = 1, max_length = 4000)]
    pub prose: String,
    /// The one closed first-slice consequence.
    pub consequence: ActionConsequenceInput,
}

impl SubmitActionInput {
    pub fn parse(self) -> Result<SubmitAction, ErrorOutput> {
        let consequence = match self.consequence {
            ActionConsequenceInput::IntroduceEntity {
                name,
                description,
                property,
            } => ActionConsequence::IntroduceEntity(IntroduceEntity {
                name,
                description,
                property: property.into_iter().map(Into::into).collect(),
            }),
            ActionConsequenceInput::ChangeEntityProperty { property_change } => {
                ActionConsequence::ChangeEntityProperty(ChangeEntityProperty {
                    property_change: property_change.into_iter().map(Into::into).collect(),
                })
            }
            ActionConsequenceInput::ChangeEntityTrait { trait_change } => {
                ActionConsequence::ChangeEntityTrait(ChangeEntityTrait {
                    trait_change: trait_change.into_iter().map(Into::into).collect(),
                })
            }
        };
        Ok(SubmitAction {
            request_id: self.request_id,
            expected_place_revision: decode_place_revision(&self.expected_place_revision)?,
            prose: self.prose,
            consequence,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub struct SubmitInteractionInput {
    /// Agent-generated UUID for this one intended Interaction. Reuse only for an
    /// uncertain delivery retry of semantically identical input.
    pub request_id: Uuid,
    /// Opaque exact-Place revision copied unchanged from a grounded Place read.
    pub expected_place_revision: String,
    /// Exact canonical English outward behavior previewed and explicitly confirmed
    /// by the User. It never authors a target's response, thought or private intent.
    #[schemars(length(min = 1, max = 4000))]
    #[schema(min_length = 1, max_length = 4000)]
    pub prose: String,
    /// Unordered set of 1 through 100 distinct target Entity ids selected from the
    /// current exact-Place Entity read.
    #[schemars(length(min = 1, max = 100))]
    #[schema(min_items = 1, max_items = 100)]
    pub target_entity_id: Vec<Uuid>,
    /// Optional typed changes to the actor or explicit targets.
    #[serde(default)]
    #[schemars(length(max = 100))]
    #[schema(max_items = 100)]
    pub property_change: Vec<EntityPropertyChangeInput>,
    /// Optional mixed establishment/development of Traits owned by the actor or
    /// explicit targets. This may coexist atomically with Property changes.
    #[serde(default)]
    #[schemars(length(max = 100))]
    #[schema(max_items = 100)]
    pub trait_change: Vec<EntityTraitChangeInput>,
}

impl SubmitInteractionInput {
    pub fn parse(self) -> Result<SubmitInteraction, ErrorOutput> {
        Ok(SubmitInteraction {
            request_id: self.request_id,
            expected_place_revision: decode_place_revision(&self.expected_place_revision)?,
            prose: self.prose,
            target_entity_id: self.target_entity_id.into_iter().map(EntityId).collect(),
            property_change: self.property_change.into_iter().map(Into::into).collect(),
            trait_change: self.trait_change.into_iter().map(Into::into).collect(),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub struct AcceptedInteractionOutput {
    /// Immutable accepted Interaction with actor, location, targets and canonical
    /// outward behavior.
    pub activity: ActivityOutput,
    /// Flat safe id, name and description of the exact Place at which the
    /// Interaction was accepted; complete Entity provenance and entry status are omitted.
    pub place: CurrentPlaceOutput,
}

impl From<AcceptedInteraction> for AcceptedInteractionOutput {
    fn from(value: AcceptedInteraction) -> Self {
        Self {
            activity: value.activity.into(),
            place: value.place.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub struct ErrorOutput {
    pub error: ErrorDetail,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub struct ErrorDetail {
    pub code: ErrorCode,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum ErrorCode {
    UserContextRequired,
    InvalidRequest,
    InvalidEntity,
    InvalidCharacter,
    InvalidPlace,
    InvalidAction,
    InvalidInteraction,
    InvalidProperty,
    InvalidTrait,
    InvalidEntityLimit,
    InvalidActivityLimit,
    UserNotFound,
    EntityNotFound,
    CharacterNotFound,
    CharacterAlreadyExists,
    CharacterAlreadyEntered,
    CharacterNotEntered,
    EntryPlaceAlreadyExists,
    EntryPlaceNotFound,
    ActionRequestConflict,
    InteractionRequestConflict,
    InteractionTargetUnavailable,
    PropertyEntityUnavailable,
    EntityAtCurrentPlaceUnavailable,
    TraitUnavailable,
    PropertyKeyConflict,
    PlaceRevisionConflict,
    Unavailable,
}

impl ErrorOutput {
    pub fn missing_user_context() -> Self {
        Self::new(
            ErrorCode::UserContextRequired,
            format!("{USER_CONTEXT_HEADER} is required."),
        )
    }

    pub fn invalid_user_context() -> Self {
        Self::with_detail(
            ErrorCode::InvalidRequest,
            format!("{USER_CONTEXT_HEADER} must be a UUID."),
            USER_CONTEXT_HEADER,
            "invalid_uuid",
        )
    }

    pub fn multiple_user_context() -> Self {
        Self::with_detail(
            ErrorCode::InvalidRequest,
            format!("{USER_CONTEXT_HEADER} must contain exactly one value."),
            USER_CONTEXT_HEADER,
            "multiple_values",
        )
    }

    pub fn invalid_entity_id() -> Self {
        Self::with_detail(
            ErrorCode::InvalidRequest,
            "entity_id must be a UUID.",
            "entity_id",
            "invalid_uuid",
        )
    }

    pub fn invalid_cursor() -> Self {
        Self::with_detail(
            ErrorCode::InvalidRequest,
            "cursor is malformed.",
            "cursor",
            "malformed",
        )
    }

    pub fn invalid_place_revision() -> Self {
        Self::with_detail(
            ErrorCode::InvalidRequest,
            "expected_place_revision is malformed.",
            "expected_place_revision",
            "malformed",
        )
    }

    pub fn malformed_request(message: impl Into<String>) -> Self {
        Self::new(ErrorCode::InvalidRequest, message)
    }

    pub fn from_world(error: WorldError) -> Self {
        match error {
            WorldError::InvalidRequest => Self::new(
                ErrorCode::InvalidRequest,
                "The request does not match the selected World operation.",
            ),
            WorldError::InvalidEntity { field, reason } => {
                let field = match field {
                    EntityField::Name => "name",
                    EntityField::Description => "description",
                };
                let (reason, explanation) = match reason {
                    InvalidReason::Empty => ("empty", "is empty"),
                    InvalidReason::ContainsNul => ("contains_nul", "contains U+0000"),
                    InvalidReason::TooLong => ("too_long", "is too long"),
                    InvalidReason::OutOfRange => ("out_of_range", "is outside the accepted range"),
                    InvalidReason::InvalidFormat => ("invalid_format", "has an invalid format"),
                    InvalidReason::Duplicate => ("duplicate", "contains a duplicate"),
                };
                Self::with_detail(
                    ErrorCode::InvalidEntity,
                    format!("Entity {field} {explanation}."),
                    field,
                    reason,
                )
            }
            WorldError::InvalidCharacter { field, reason } => {
                let field = match field {
                    EntityField::Name => "name",
                    EntityField::Description => "description",
                };
                let (reason, explanation) = match reason {
                    InvalidReason::Empty => ("empty", "is empty"),
                    InvalidReason::ContainsNul => ("contains_nul", "contains U+0000"),
                    InvalidReason::TooLong => ("too_long", "is too long"),
                    InvalidReason::OutOfRange => ("out_of_range", "is outside the accepted range"),
                    InvalidReason::InvalidFormat => ("invalid_format", "has an invalid format"),
                    InvalidReason::Duplicate => ("duplicate", "contains a duplicate"),
                };
                Self::with_detail(
                    ErrorCode::InvalidCharacter,
                    format!("Character {field} {explanation}."),
                    field,
                    reason,
                )
            }
            WorldError::InvalidPlace { field, reason } => {
                let field = match field {
                    EntityField::Name => "name",
                    EntityField::Description => "description",
                };
                let (reason, explanation) = match reason {
                    InvalidReason::Empty => ("empty", "is empty"),
                    InvalidReason::ContainsNul => ("contains_nul", "contains U+0000"),
                    InvalidReason::TooLong => ("too_long", "is too long"),
                    InvalidReason::OutOfRange => ("out_of_range", "is outside the accepted range"),
                    InvalidReason::InvalidFormat => ("invalid_format", "has an invalid format"),
                    InvalidReason::Duplicate => ("duplicate", "contains a duplicate"),
                };
                Self::with_detail(
                    ErrorCode::InvalidPlace,
                    format!("Place {field} {explanation}."),
                    field,
                    reason,
                )
            }
            WorldError::InvalidAction { field, reason } => {
                let (field, subject) = match field {
                    ActionField::Prose => ("prose", "Action prose"),
                    ActionField::ConsequenceName => ("consequence.name", "Action consequence name"),
                    ActionField::ConsequenceDescription => {
                        ("consequence.description", "Action consequence description")
                    }
                };
                let (reason, explanation) = match reason {
                    InvalidReason::Empty => ("empty", "is empty"),
                    InvalidReason::ContainsNul => ("contains_nul", "contains U+0000"),
                    InvalidReason::TooLong => ("too_long", "is too long"),
                    InvalidReason::OutOfRange => ("out_of_range", "is outside the accepted range"),
                    InvalidReason::InvalidFormat => ("invalid_format", "has an invalid format"),
                    InvalidReason::Duplicate => ("duplicate", "contains a duplicate"),
                };
                Self::with_detail(
                    ErrorCode::InvalidAction,
                    format!("{subject} {explanation}."),
                    field,
                    reason,
                )
            }
            WorldError::InvalidInteraction { field, reason } => {
                let (field, subject) = match field {
                    InteractionField::Prose => ("prose", "Interaction prose"),
                    InteractionField::TargetEntityId => {
                        ("target_entity_id", "Interaction target list")
                    }
                };
                let (reason, explanation) = match reason {
                    InvalidReason::Empty => ("empty", "is empty"),
                    InvalidReason::ContainsNul => ("contains_nul", "contains U+0000"),
                    InvalidReason::TooLong => ("too_long", "is too long"),
                    InvalidReason::OutOfRange => {
                        ("out_of_range", "must contain 1 through 100 targets")
                    }
                    InvalidReason::InvalidFormat => ("invalid_format", "has an invalid format"),
                    InvalidReason::Duplicate => ("duplicate", "contains a duplicate"),
                };
                Self::with_detail(
                    ErrorCode::InvalidInteraction,
                    format!("{subject} {explanation}."),
                    field,
                    reason,
                )
            }
            WorldError::InvalidProperty { field, reason } => {
                let field = match field {
                    PropertyField::Property => "property",
                    PropertyField::PropertyChange => "property_change",
                    PropertyField::Key => "key",
                    PropertyField::Value => "value",
                };
                let (reason, explanation) = match reason {
                    InvalidReason::Empty => ("empty", "is empty"),
                    InvalidReason::ContainsNul => ("contains_nul", "contains U+0000"),
                    InvalidReason::TooLong => ("too_long", "is too long"),
                    InvalidReason::OutOfRange => ("out_of_range", "is outside the accepted range"),
                    InvalidReason::InvalidFormat => ("invalid_format", "has an invalid format"),
                    InvalidReason::Duplicate => ("duplicate", "contains a duplicate"),
                };
                Self::with_detail(
                    ErrorCode::InvalidProperty,
                    format!("Property {field} {explanation}."),
                    field,
                    reason,
                )
            }
            WorldError::InvalidTrait => Self::new(
                ErrorCode::InvalidTrait,
                "Trait input is invalid, duplicated or unchanged.",
            ),
            WorldError::InvalidEntityLimit => Self::with_detail(
                ErrorCode::InvalidEntityLimit,
                "limit must be from 1 through 100.",
                "limit",
                "out_of_range",
            ),
            WorldError::InvalidActivityLimit => Self::with_detail(
                ErrorCode::InvalidActivityLimit,
                "limit must be from 1 through 100.",
                "limit",
                "out_of_range",
            ),
            WorldError::InvalidPropertyLimit => Self::with_detail(
                ErrorCode::InvalidEntityLimit,
                "limit must be from 1 through 100.",
                "limit",
                "out_of_range",
            ),
            WorldError::UserNotFound => Self::new(
                ErrorCode::UserNotFound,
                format!("{USER_CONTEXT_HEADER} does not identify an existing User."),
            ),
            WorldError::EntityNotFound => Self::new(
                ErrorCode::EntityNotFound,
                "entity_id does not identify an existing Entity.",
            ),
            WorldError::CharacterNotFound => Self::new(
                ErrorCode::CharacterNotFound,
                "The current User does not own a Character.",
            ),
            WorldError::CharacterAlreadyExists => Self::new(
                ErrorCode::CharacterAlreadyExists,
                "The current User already owns a Character.",
            ),
            WorldError::CharacterAlreadyEntered => Self::new(
                ErrorCode::CharacterAlreadyEntered,
                "The current Character is already placed in the World.",
            ),
            WorldError::CharacterNotEntered => Self::new(
                ErrorCode::CharacterNotEntered,
                "The current Character has not entered the World.",
            ),
            WorldError::EntryPlaceAlreadyExists => Self::new(
                ErrorCode::EntryPlaceAlreadyExists,
                "The World already has an entry Place.",
            ),
            WorldError::EntryPlaceNotFound => Self::new(
                ErrorCode::EntryPlaceNotFound,
                "The World does not have an entry Place yet.",
            ),
            WorldError::ActionRequestConflict => Self::new(
                ErrorCode::ActionRequestConflict,
                "request_id was already accepted with different action content.",
            ),
            WorldError::InteractionRequestConflict => Self::new(
                ErrorCode::InteractionRequestConflict,
                "request_id was already accepted with different interaction content.",
            ),
            WorldError::InteractionTargetUnavailable => Self::new(
                ErrorCode::InteractionTargetUnavailable,
                "One or more interaction targets are unavailable.",
            ),
            WorldError::PropertyEntityUnavailable => Self::new(
                ErrorCode::PropertyEntityUnavailable,
                "One or more Property subjects are unavailable.",
            ),
            WorldError::TraitUnavailable => Self::new(
                ErrorCode::TraitUnavailable,
                "One or more selected Traits are unavailable.",
            ),
            WorldError::EntityAtCurrentPlaceUnavailable => Self::new(
                ErrorCode::EntityAtCurrentPlaceUnavailable,
                "The selected Entity is unavailable at the current Place.",
            ),
            WorldError::PropertyKeyConflict => Self::new(
                ErrorCode::PropertyKeyConflict,
                "A Property key already exists with another value type.",
            ),
            WorldError::PlaceRevisionConflict => Self::new(
                ErrorCode::PlaceRevisionConflict,
                "The current Place changed after it was read.",
            ),
            WorldError::Unavailable => Self::new(
                ErrorCode::Unavailable,
                "The World could not complete the request; retry later.",
            ),
        }
    }

    fn new(code: ErrorCode, message: impl Into<String>) -> Self {
        Self {
            error: ErrorDetail {
                code,
                message: message.into(),
                field: None,
                reason: None,
            },
        }
    }

    fn with_detail(
        code: ErrorCode,
        message: impl Into<String>,
        field: impl Into<String>,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            error: ErrorDetail {
                code,
                message: message.into(),
                field: Some(field.into()),
                reason: Some(reason.into()),
            },
        }
    }
}

pub fn parse_user_context(value: Option<&str>) -> Result<UserId, ErrorOutput> {
    let value = value.ok_or_else(ErrorOutput::missing_user_context)?;
    Uuid::parse_str(value)
        .map(UserId)
        .map_err(|_| ErrorOutput::invalid_user_context())
}

fn encode_cursor(cursor: EntityCursor) -> String {
    let value = format!(
        "v1|{}|{}",
        cursor
            .introduced_at
            .to_rfc3339_opts(SecondsFormat::Nanos, true),
        cursor.entity_id.0
    );
    URL_SAFE_NO_PAD.encode(value)
}

fn encode_activity_cursor(cursor: ActivityCursor) -> String {
    encode_cursor_parts("a1", cursor.occurred_at, cursor.activity_id.0)
}

fn encode_place_entity_cursor(cursor: EntityCursor) -> String {
    encode_cursor_parts("pe1", cursor.introduced_at, cursor.entity_id.0)
}

fn encode_place_activity_cursor(cursor: ActivityCursor) -> String {
    encode_cursor_parts("pa1", cursor.occurred_at, cursor.activity_id.0)
}

fn encode_character_state_cursor(cursor: EntityCurrentStateCursor) -> String {
    encode_entity_current_state_cursor(cursor, "gc1")
}

fn encode_current_place_entity_state_cursor(cursor: EntityCurrentStateCursor) -> String {
    encode_entity_current_state_cursor(cursor, "ge1")
}

fn encode_entity_current_state_cursor(cursor: EntityCurrentStateCursor, version: &str) -> String {
    let (place_entity_id, occurred_at, activity_id) = match cursor.place_revision() {
        Some(revision) => (
            revision.place_entity_id().0.to_string(),
            revision
                .occurred_at()
                .to_rfc3339_opts(SecondsFormat::Nanos, true),
            revision.activity_id().0.to_string(),
        ),
        None => ("-".to_owned(), "-".to_owned(), "-".to_owned()),
    };
    let (kind, key) = match (cursor.property_key_id(), cursor.trait_id()) {
        (Some(property_key_id), None) => ("p", property_key_id.to_string()),
        (None, Some(trait_id)) => ("t", trait_id.0.to_string()),
        _ => unreachable!("World current-state cursor has exactly one typed key"),
    };
    URL_SAFE_NO_PAD.encode(format!(
        "{version}|{}|{place_entity_id}|{occurred_at}|{activity_id}|{kind}|{key}",
        cursor.entity_id().0
    ))
}

fn encode_place_revision(revision: PlaceRevision) -> String {
    URL_SAFE_NO_PAD.encode(format!(
        "p1|{}|{}|{}",
        revision.place_entity_id().0,
        revision
            .occurred_at()
            .to_rfc3339_opts(SecondsFormat::Nanos, true),
        revision.activity_id().0
    ))
}

fn encode_cursor_parts(version: &str, timestamp: DateTime<Utc>, id: Uuid) -> String {
    URL_SAFE_NO_PAD.encode(format!(
        "{version}|{}|{id}",
        timestamp.to_rfc3339_opts(SecondsFormat::Nanos, true)
    ))
}

fn decode_cursor(value: &str) -> Result<EntityCursor, ErrorOutput> {
    let (introduced_at, entity_id) = decode_cursor_parts(value, "v1")?;
    Ok(EntityCursor {
        introduced_at,
        entity_id: EntityId(entity_id),
    })
}

fn decode_activity_cursor(value: &str) -> Result<ActivityCursor, ErrorOutput> {
    let (occurred_at, activity_id) = decode_cursor_parts(value, "a1")?;
    Ok(ActivityCursor {
        occurred_at,
        activity_id: ActivityId(activity_id),
    })
}

fn decode_place_entity_cursor(value: &str) -> Result<EntityCursor, ErrorOutput> {
    let (introduced_at, entity_id) = decode_cursor_parts(value, "pe1")?;
    Ok(EntityCursor {
        introduced_at,
        entity_id: EntityId(entity_id),
    })
}

fn decode_place_activity_cursor(value: &str) -> Result<ActivityCursor, ErrorOutput> {
    let (occurred_at, activity_id) = decode_cursor_parts(value, "pa1")?;
    Ok(ActivityCursor {
        occurred_at,
        activity_id: ActivityId(activity_id),
    })
}

fn decode_character_state_cursor(value: &str) -> Result<EntityCurrentStateCursor, ErrorOutput> {
    decode_entity_current_state_cursor(value, "gc1")
}

fn decode_current_place_entity_state_cursor(
    value: &str,
) -> Result<EntityCurrentStateCursor, ErrorOutput> {
    decode_entity_current_state_cursor(value, "ge1")
}

fn decode_entity_current_state_cursor(
    value: &str,
    expected_version: &str,
) -> Result<EntityCurrentStateCursor, ErrorOutput> {
    if value.len() > 768 {
        return Err(ErrorOutput::invalid_cursor());
    }
    let decoded = URL_SAFE_NO_PAD
        .decode(value)
        .map_err(|_| ErrorOutput::invalid_cursor())?;
    let decoded = std::str::from_utf8(&decoded).map_err(|_| ErrorOutput::invalid_cursor())?;
    let mut part = decoded.split('|');
    let version = part.next();
    let entity_id = part.next();
    let place_entity_id = part.next();
    let occurred_at = part.next();
    let activity_id = part.next();
    let kind = part.next();
    let key = part.next();
    if version != Some(expected_version) || part.next().is_some() {
        return Err(ErrorOutput::invalid_cursor());
    }
    let entity_id = entity_id
        .ok_or_else(ErrorOutput::invalid_cursor)
        .and_then(|value| Uuid::parse_str(value).map_err(|_| ErrorOutput::invalid_cursor()))?;
    let place_revision = match (place_entity_id, occurred_at, activity_id) {
        (Some("-"), Some("-"), Some("-")) => None,
        (Some(place_entity_id), Some(occurred_at), Some(activity_id)) => {
            Some(PlaceRevision::from_parts(
                EntityId(
                    Uuid::parse_str(place_entity_id).map_err(|_| ErrorOutput::invalid_cursor())?,
                ),
                occurred_at
                    .parse::<DateTime<Utc>>()
                    .map_err(|_| ErrorOutput::invalid_cursor())?,
                ActivityId(
                    Uuid::parse_str(activity_id).map_err(|_| ErrorOutput::invalid_cursor())?,
                ),
            ))
        }
        _ => return Err(ErrorOutput::invalid_cursor()),
    };
    let key = key.ok_or_else(ErrorOutput::invalid_cursor)?;
    match kind {
        Some("p") => {
            let property_key_id = key
                .parse::<i64>()
                .map_err(|_| ErrorOutput::invalid_cursor())?;
            if property_key_id <= 0 {
                return Err(ErrorOutput::invalid_cursor());
            }
            Ok(EntityCurrentStateCursor::from_property(
                EntityId(entity_id),
                place_revision,
                property_key_id,
            ))
        }
        Some("t") => Ok(EntityCurrentStateCursor::from_trait(
            EntityId(entity_id),
            place_revision,
            EntityTraitId(Uuid::parse_str(key).map_err(|_| ErrorOutput::invalid_cursor())?),
        )),
        _ => Err(ErrorOutput::invalid_cursor()),
    }
}

fn decode_place_revision(value: &str) -> Result<PlaceRevision, ErrorOutput> {
    if value.len() > 384 {
        return Err(ErrorOutput::invalid_place_revision());
    }
    let decoded = URL_SAFE_NO_PAD
        .decode(value)
        .map_err(|_| ErrorOutput::invalid_place_revision())?;
    let decoded =
        std::str::from_utf8(&decoded).map_err(|_| ErrorOutput::invalid_place_revision())?;
    let mut part = decoded.split('|');
    let version = part.next();
    let place_entity_id = part.next();
    let occurred_at = part.next();
    let activity_id = part.next();
    if version != Some("p1") || part.next().is_some() {
        return Err(ErrorOutput::invalid_place_revision());
    }

    let place_entity_id = place_entity_id
        .ok_or_else(ErrorOutput::invalid_place_revision)
        .and_then(|value| {
            Uuid::parse_str(value).map_err(|_| ErrorOutput::invalid_place_revision())
        })?;
    let occurred_at = occurred_at
        .ok_or_else(ErrorOutput::invalid_place_revision)?
        .parse::<DateTime<Utc>>()
        .map_err(|_| ErrorOutput::invalid_place_revision())?;
    let activity_id = activity_id
        .ok_or_else(ErrorOutput::invalid_place_revision)
        .and_then(|value| {
            Uuid::parse_str(value).map_err(|_| ErrorOutput::invalid_place_revision())
        })?;

    Ok(PlaceRevision::from_parts(
        EntityId(place_entity_id),
        occurred_at,
        ActivityId(activity_id),
    ))
}

fn decode_cursor_parts(
    value: &str,
    expected_version: &str,
) -> Result<(DateTime<Utc>, Uuid), ErrorOutput> {
    if value.len() > 256 {
        return Err(ErrorOutput::invalid_cursor());
    }

    let decoded = URL_SAFE_NO_PAD
        .decode(value)
        .map_err(|_| ErrorOutput::invalid_cursor())?;
    let decoded = std::str::from_utf8(&decoded).map_err(|_| ErrorOutput::invalid_cursor())?;
    let mut part = decoded.split('|');
    let version = part.next();
    let introduced_at = part.next();
    let entity_id = part.next();
    if version != Some(expected_version) || part.next().is_some() {
        return Err(ErrorOutput::invalid_cursor());
    }

    let introduced_at = introduced_at
        .ok_or_else(ErrorOutput::invalid_cursor)?
        .parse::<DateTime<Utc>>()
        .map_err(|_| ErrorOutput::invalid_cursor())?;
    let id = entity_id
        .ok_or_else(ErrorOutput::invalid_cursor)
        .and_then(|value| Uuid::parse_str(value).map_err(|_| ErrorOutput::invalid_cursor()))?;

    Ok((introduced_at, id))
}

#[cfg(test)]
mod test {
    use chrono::TimeZone;

    use super::*;

    #[test]
    fn cursor_round_trips_and_rejects_invalid_input() {
        let cursor = EntityCursor {
            introduced_at: Utc
                .with_ymd_and_hms(2026, 8, 7, 12, 30, 0)
                .single()
                .expect("timestamp should be valid"),
            entity_id: EntityId(Uuid::new_v4()),
        };

        assert_eq!(decode_cursor(&encode_cursor(cursor)), Ok(cursor));
        assert_eq!(
            decode_cursor("not-a-cursor"),
            Err(ErrorOutput::invalid_cursor())
        );

        let activity_cursor = ActivityCursor {
            occurred_at: cursor.introduced_at,
            activity_id: ActivityId(Uuid::new_v4()),
        };
        assert_eq!(
            decode_activity_cursor(&encode_activity_cursor(activity_cursor)),
            Ok(activity_cursor)
        );
        assert_eq!(
            decode_activity_cursor(&encode_cursor(cursor)),
            Err(ErrorOutput::invalid_cursor())
        );
        assert_eq!(
            decode_place_entity_cursor(&encode_place_entity_cursor(cursor)),
            Ok(cursor)
        );
        assert_eq!(
            decode_place_activity_cursor(&encode_place_activity_cursor(activity_cursor)),
            Ok(activity_cursor)
        );
        let property_cursor = EntityCurrentStateCursor::from_property(
            cursor.entity_id,
            Some(PlaceRevision::from_parts(
                cursor.entity_id,
                cursor.introduced_at,
                activity_cursor.activity_id,
            )),
            17,
        );
        assert_eq!(
            decode_character_state_cursor(&encode_character_state_cursor(property_cursor)),
            Ok(property_cursor)
        );
        assert_eq!(
            decode_current_place_entity_state_cursor(&encode_current_place_entity_state_cursor(
                property_cursor,
            )),
            Ok(property_cursor)
        );
        assert_eq!(
            decode_current_place_entity_state_cursor(&encode_character_state_cursor(
                property_cursor,
            )),
            Err(ErrorOutput::invalid_cursor())
        );
        let trait_cursor = EntityCurrentStateCursor::from_trait(
            cursor.entity_id,
            None,
            EntityTraitId(Uuid::new_v4()),
        );
        assert_eq!(
            decode_character_state_cursor(&encode_character_state_cursor(trait_cursor)),
            Ok(trait_cursor)
        );

        for encoded in [
            encode_activity_cursor(activity_cursor),
            encode_place_entity_cursor(cursor),
            encode_place_activity_cursor(activity_cursor),
        ] {
            assert_eq!(decode_cursor(&encoded), Err(ErrorOutput::invalid_cursor()));
        }
        for encoded in [
            encode_cursor(cursor),
            encode_place_entity_cursor(cursor),
            encode_place_activity_cursor(activity_cursor),
        ] {
            assert_eq!(
                decode_activity_cursor(&encoded),
                Err(ErrorOutput::invalid_cursor())
            );
        }
        for encoded in [
            encode_cursor(cursor),
            encode_activity_cursor(activity_cursor),
            encode_place_activity_cursor(activity_cursor),
        ] {
            assert_eq!(
                decode_place_entity_cursor(&encoded),
                Err(ErrorOutput::invalid_cursor())
            );
        }
        for encoded in [
            encode_cursor(cursor),
            encode_activity_cursor(activity_cursor),
            encode_place_entity_cursor(cursor),
        ] {
            assert_eq!(
                decode_place_activity_cursor(&encoded),
                Err(ErrorOutput::invalid_cursor())
            );
        }

        let revision = PlaceRevision::from_parts(
            cursor.entity_id,
            cursor.introduced_at,
            activity_cursor.activity_id,
        );
        assert_eq!(
            decode_place_revision(&encode_place_revision(revision)),
            Ok(revision)
        );
        assert_eq!(
            decode_place_revision(&encode_activity_cursor(activity_cursor)),
            Err(ErrorOutput::invalid_place_revision())
        );
    }

    #[test]
    fn list_limit_representation_is_parsed_before_world_validation() {
        for limit in [0, 101] {
            let parsed = ListEntityInput {
                cursor: None,
                limit,
            }
            .parse()
            .expect("u16 values should reach World validation");
            assert_eq!(parsed.limit, limit as u16);
        }

        for limit in [-1, 65_536] {
            assert_eq!(
                ListEntityInput {
                    cursor: None,
                    limit,
                }
                .parse(),
                Err(ErrorOutput::from_world(WorldError::InvalidEntityLimit))
            );
        }
    }

    #[test]
    fn error_codes_have_one_compiler_checked_wire_spelling() {
        let code = [
            (ErrorCode::UserContextRequired, "user_context_required"),
            (ErrorCode::InvalidRequest, "invalid_request"),
            (ErrorCode::InvalidEntity, "invalid_entity"),
            (ErrorCode::InvalidCharacter, "invalid_character"),
            (ErrorCode::InvalidPlace, "invalid_place"),
            (ErrorCode::InvalidAction, "invalid_action"),
            (ErrorCode::InvalidInteraction, "invalid_interaction"),
            (ErrorCode::InvalidProperty, "invalid_property"),
            (ErrorCode::InvalidTrait, "invalid_trait"),
            (ErrorCode::InvalidEntityLimit, "invalid_entity_limit"),
            (ErrorCode::InvalidActivityLimit, "invalid_activity_limit"),
            (ErrorCode::UserNotFound, "user_not_found"),
            (ErrorCode::EntityNotFound, "entity_not_found"),
            (ErrorCode::CharacterNotFound, "character_not_found"),
            (
                ErrorCode::CharacterAlreadyExists,
                "character_already_exists",
            ),
            (
                ErrorCode::CharacterAlreadyEntered,
                "character_already_entered",
            ),
            (ErrorCode::CharacterNotEntered, "character_not_entered"),
            (
                ErrorCode::EntryPlaceAlreadyExists,
                "entry_place_already_exists",
            ),
            (ErrorCode::EntryPlaceNotFound, "entry_place_not_found"),
            (ErrorCode::ActionRequestConflict, "action_request_conflict"),
            (
                ErrorCode::InteractionRequestConflict,
                "interaction_request_conflict",
            ),
            (
                ErrorCode::InteractionTargetUnavailable,
                "interaction_target_unavailable",
            ),
            (
                ErrorCode::PropertyEntityUnavailable,
                "property_entity_unavailable",
            ),
            (
                ErrorCode::EntityAtCurrentPlaceUnavailable,
                "entity_at_current_place_unavailable",
            ),
            (ErrorCode::TraitUnavailable, "trait_unavailable"),
            (ErrorCode::PropertyKeyConflict, "property_key_conflict"),
            (ErrorCode::PlaceRevisionConflict, "place_revision_conflict"),
            (ErrorCode::Unavailable, "unavailable"),
        ];

        for (code, expected) in code {
            assert_eq!(
                serde_json::to_value(code).expect("error code should serialize"),
                serde_json::json!(expected)
            );
        }
    }
}
