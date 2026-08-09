use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use chrono::{DateTime, SecondsFormat, Utc};
use schemars::{JsonSchema, Schema, SchemaGenerator};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::{
    Character, CreateCharacter, CreateEntity, Entity, EntityCursor, EntityField, EntityId,
    EntityPage, EntitySummary, InvalidReason, ListEntity, User, UserId, WorldError, WorldView,
};

pub const USER_CONTEXT_HEADER: &str = "Aicadia-User-Id";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub struct WorldOutput {
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
    pub id: Uuid,
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
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub introduced_by_user_id: Uuid,
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
    pub entity: EntityOutput,
    pub owner_user_id: Uuid,
}

impl From<Character> for CharacterOutput {
    fn from(value: Character) -> Self {
        Self {
            entity: value.entity.into(),
            owner_user_id: value.owner_user_id.0,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
pub struct EntitySummaryOutput {
    pub id: Uuid,
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
pub struct EntityPageOutput {
    pub entity: Vec<EntitySummaryOutput>,
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
    #[serde(default = "default_entity_limit")]
    #[schemars(default = "default_entity_limit", range(min = 1, max = 100))]
    #[param(default = 25, minimum = 1, maximum = 100)]
    pub limit: i64,
}

impl Default for ListEntityInput {
    fn default() -> Self {
        Self {
            cursor: None,
            limit: default_entity_limit(),
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

const fn default_entity_limit() -> i64 {
    25
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
    InvalidEntityLimit,
    UserNotFound,
    EntityNotFound,
    CharacterNotFound,
    CharacterAlreadyExists,
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
            WorldError::InvalidEntityLimit => Self::with_detail(
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

fn decode_cursor(value: &str) -> Result<EntityCursor, ErrorOutput> {
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
    if version != Some("v1") || part.next().is_some() {
        return Err(ErrorOutput::invalid_cursor());
    }

    let introduced_at = introduced_at
        .ok_or_else(ErrorOutput::invalid_cursor)?
        .parse::<DateTime<Utc>>()
        .map_err(|_| ErrorOutput::invalid_cursor())?;
    let entity_id = entity_id
        .ok_or_else(ErrorOutput::invalid_cursor)
        .and_then(|value| {
            Uuid::parse_str(value)
                .map(EntityId)
                .map_err(|_| ErrorOutput::invalid_cursor())
        })?;

    Ok(EntityCursor {
        introduced_at,
        entity_id,
    })
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
            (ErrorCode::InvalidEntityLimit, "invalid_entity_limit"),
            (ErrorCode::UserNotFound, "user_not_found"),
            (ErrorCode::EntityNotFound, "entity_not_found"),
            (ErrorCode::CharacterNotFound, "character_not_found"),
            (
                ErrorCode::CharacterAlreadyExists,
                "character_already_exists",
            ),
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
