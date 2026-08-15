use super::*;

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
                    ActionField::Consequence => ("consequence", "Action consequence"),
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

pub(super) fn encode_cursor(cursor: EntityCursor) -> String {
    let value = format!(
        "v1|{}|{}",
        cursor
            .introduced_at
            .to_rfc3339_opts(SecondsFormat::Nanos, true),
        cursor.entity_id.0
    );
    URL_SAFE_NO_PAD.encode(value)
}

pub(super) fn encode_activity_cursor(cursor: ActivityCursor) -> String {
    encode_cursor_parts("a1", cursor.occurred_at, cursor.activity_id.0)
}

pub(super) fn encode_place_entity_cursor(cursor: EntityCursor) -> String {
    encode_cursor_parts("pe1", cursor.introduced_at, cursor.entity_id.0)
}

pub(super) fn encode_place_activity_cursor(cursor: ActivityCursor) -> String {
    encode_cursor_parts("pa1", cursor.occurred_at, cursor.activity_id.0)
}

pub(super) fn encode_character_state_cursor(cursor: EntityCurrentStateCursor) -> String {
    encode_entity_current_state_cursor(cursor, "gc1")
}

pub(super) fn encode_current_place_entity_state_cursor(cursor: EntityCurrentStateCursor) -> String {
    encode_entity_current_state_cursor(cursor, "ge1")
}

pub(super) fn encode_entity_current_state_cursor(
    cursor: EntityCurrentStateCursor,
    version: &str,
) -> String {
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

pub(super) fn encode_place_revision(revision: PlaceRevision) -> String {
    URL_SAFE_NO_PAD.encode(format!(
        "p1|{}|{}|{}",
        revision.place_entity_id().0,
        revision
            .occurred_at()
            .to_rfc3339_opts(SecondsFormat::Nanos, true),
        revision.activity_id().0
    ))
}

pub(super) fn encode_cursor_parts(version: &str, timestamp: DateTime<Utc>, id: Uuid) -> String {
    URL_SAFE_NO_PAD.encode(format!(
        "{version}|{}|{id}",
        timestamp.to_rfc3339_opts(SecondsFormat::Nanos, true)
    ))
}

pub(super) fn decode_cursor(value: &str) -> Result<EntityCursor, ErrorOutput> {
    let (introduced_at, entity_id) = decode_cursor_parts(value, "v1")?;
    Ok(EntityCursor {
        introduced_at,
        entity_id: EntityId(entity_id),
    })
}

pub(super) fn decode_activity_cursor(value: &str) -> Result<ActivityCursor, ErrorOutput> {
    let (occurred_at, activity_id) = decode_cursor_parts(value, "a1")?;
    Ok(ActivityCursor {
        occurred_at,
        activity_id: ActivityId(activity_id),
    })
}

pub(super) fn decode_place_entity_cursor(value: &str) -> Result<EntityCursor, ErrorOutput> {
    let (introduced_at, entity_id) = decode_cursor_parts(value, "pe1")?;
    Ok(EntityCursor {
        introduced_at,
        entity_id: EntityId(entity_id),
    })
}

pub(super) fn decode_place_activity_cursor(value: &str) -> Result<ActivityCursor, ErrorOutput> {
    let (occurred_at, activity_id) = decode_cursor_parts(value, "pa1")?;
    Ok(ActivityCursor {
        occurred_at,
        activity_id: ActivityId(activity_id),
    })
}

pub(super) fn decode_character_state_cursor(
    value: &str,
) -> Result<EntityCurrentStateCursor, ErrorOutput> {
    decode_entity_current_state_cursor(value, "gc1")
}

pub(super) fn decode_current_place_entity_state_cursor(
    value: &str,
) -> Result<EntityCurrentStateCursor, ErrorOutput> {
    decode_entity_current_state_cursor(value, "ge1")
}

pub(super) fn decode_entity_current_state_cursor(
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

pub(super) fn decode_place_revision(value: &str) -> Result<PlaceRevision, ErrorOutput> {
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

pub(super) fn decode_cursor_parts(
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
