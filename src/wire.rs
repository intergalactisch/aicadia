use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use chrono::{DateTime, SecondsFormat, Utc};
use schemars::{JsonSchema, Schema, SchemaGenerator};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::{
    Activity, ActivityCursor, ActivityEntityReference, ActivityEntityRole, ActivityId,
    ActivityOperation, ActivityPage, Character, CreateCharacter, CreateEntity, CreateEntryPlace,
    Entity, EntityCursor, EntityField, EntityId, EntityPage, EntitySummary, InvalidReason,
    ListActivity, ListEntity, Place, PlaceSummary, User, UserId, WorldError, WorldView,
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

impl From<EntitySummary> for EntitySummaryOutput {
    fn from(value: EntitySummary) -> Self {
        Self {
            id: value.id.0,
            name: value.name,
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
}

impl From<ActivityOperation> for ActivityOperationOutput {
    fn from(value: ActivityOperation) -> Self {
        match value {
            ActivityOperation::CreateCharacter => Self::CreateCharacter,
            ActivityOperation::CreateEntity => Self::CreateEntity,
            ActivityOperation::CreateEntryPlace => Self::CreateEntryPlace,
            ActivityOperation::EnterWorld => Self::EnterWorld,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum ActivityEntityRoleOutput {
    Subject,
    Destination,
}

impl From<ActivityEntityRole> for ActivityEntityRoleOutput {
    fn from(value: ActivityEntityRole) -> Self {
        match value {
            ActivityEntityRole::Subject => Self::Subject,
            ActivityEntityRole::Destination => Self::Destination,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub struct ActivityEntityReferenceOutput {
    /// Shared Entity involved in the accepted action.
    pub entity: EntitySummaryOutput,
    /// Server-owned meaning of this Entity in the action: subject is what the
    /// action introduced or acted on; destination is where entry placed the
    /// Character.
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
    /// Stable id of this immutable accepted-action record.
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
}

impl From<CreateEntityInput> for CreateEntity {
    fn from(value: CreateEntityInput) -> Self {
        Self {
            name: value.name,
            description: value.description,
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
}

impl From<CreateCharacterInput> for CreateCharacter {
    fn from(value: CreateCharacterInput) -> Self {
        Self {
            name: value.name,
            description: value.description,
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
}

impl From<CreateEntryPlaceInput> for CreateEntryPlace {
    fn from(value: CreateEntryPlaceInput) -> Self {
        Self {
            name: value.name,
            description: value.description,
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
    InvalidEntityLimit,
    InvalidActivityLimit,
    UserNotFound,
    EntityNotFound,
    CharacterNotFound,
    CharacterAlreadyExists,
    CharacterAlreadyEntered,
    EntryPlaceAlreadyExists,
    EntryPlaceNotFound,
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

    pub fn malformed_request(message: impl Into<String>) -> Self {
        Self::new(ErrorCode::InvalidRequest, message)
    }

    pub fn from_world(error: WorldError) -> Self {
        match error {
            WorldError::InvalidEntity { field, reason } => {
                let field = match field {
                    EntityField::Name => "name",
                    EntityField::Description => "description",
                };
                let (reason, explanation) = match reason {
                    InvalidReason::Empty => ("empty", "is empty"),
                    InvalidReason::ContainsNul => ("contains_nul", "contains U+0000"),
                    InvalidReason::TooLong => ("too_long", "is too long"),
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
                };
                Self::with_detail(
                    ErrorCode::InvalidPlace,
                    format!("Place {field} {explanation}."),
                    field,
                    reason,
                )
            }
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
            WorldError::EntryPlaceAlreadyExists => Self::new(
                ErrorCode::EntryPlaceAlreadyExists,
                "The World already has an entry Place.",
            ),
            WorldError::EntryPlaceNotFound => Self::new(
                ErrorCode::EntryPlaceNotFound,
                "The World does not have an entry Place yet.",
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
            (
                ErrorCode::EntryPlaceAlreadyExists,
                "entry_place_already_exists",
            ),
            (ErrorCode::EntryPlaceNotFound, "entry_place_not_found"),
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
