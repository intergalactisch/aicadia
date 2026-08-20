use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EntityField {
    Name,
    Description,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActionField {
    Prose,
    Consequence,
    ConsequenceName,
    ConsequenceDescription,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InteractionField {
    Prose,
    TargetEntityId,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DiscoveryField {
    Prose,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PositionField {
    XCm,
    YCm,
    ZCm,
    Description,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConnectionField {
    Name,
    Description,
    ShapeDescription,
    Course,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MovementField {
    OriginSegmentOrdinal,
    TargetSegmentOrdinal,
    Target,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PropertyField {
    Property,
    PropertyChange,
    Key,
    Value,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InvalidReason {
    Empty,
    ContainsNul,
    TooLong,
    OutOfRange,
    InvalidFormat,
    Duplicate,
}

#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum WorldError {
    #[error("request input is invalid")]
    InvalidRequest,
    #[error("entity input is invalid")]
    InvalidEntity {
        field: EntityField,
        reason: InvalidReason,
    },
    #[error("character input is invalid")]
    InvalidCharacter {
        field: EntityField,
        reason: InvalidReason,
    },
    #[error("place input is invalid")]
    InvalidPlace {
        field: EntityField,
        reason: InvalidReason,
    },
    #[error("action input is invalid")]
    InvalidAction {
        field: ActionField,
        reason: InvalidReason,
    },
    #[error("interaction input is invalid")]
    InvalidInteraction {
        field: InteractionField,
        reason: InvalidReason,
    },
    #[error("discovery input is invalid")]
    InvalidDiscovery {
        field: DiscoveryField,
        reason: InvalidReason,
    },
    #[error("position input is invalid")]
    InvalidPosition {
        field: PositionField,
        reason: InvalidReason,
    },
    #[error("connection input is invalid")]
    InvalidConnection {
        field: ConnectionField,
        reason: InvalidReason,
    },
    #[error("movement input is invalid")]
    InvalidMovement {
        field: MovementField,
        reason: InvalidReason,
    },
    #[error("property input is invalid")]
    InvalidProperty {
        field: PropertyField,
        reason: InvalidReason,
    },
    #[error("trait input is invalid")]
    InvalidTrait,
    #[error("entity list limit must be between 1 and 100")]
    InvalidEntityLimit,
    #[error("activity list limit must be between 1 and 100")]
    InvalidActivityLimit,
    #[error("property list limit must be between 1 and 100")]
    InvalidPropertyLimit,
    #[error("place window is invalid")]
    InvalidPlaceWindow,
    #[error("place list limit must be between 1 and 100")]
    InvalidPlaceLimit,
    #[error("connection list limit must be between 1 and 100")]
    InvalidConnectionLimit,
    #[error("user was not found")]
    UserNotFound,
    #[error("character was not found")]
    CharacterNotFound,
    #[error("user already owns a character")]
    CharacterAlreadyExists,
    #[error("character is already placed")]
    CharacterAlreadyEntered,
    #[error("entry place already exists")]
    EntryPlaceAlreadyExists,
    #[error("entry place was not found")]
    EntryPlaceNotFound,
    #[error("place was not found")]
    PlaceNotFound,
    #[error("connection was not found")]
    ConnectionNotFound,
    #[error("character has not entered the world")]
    CharacterNotEntered,
    #[error("action request id has already been used with different content")]
    ActionRequestConflict,
    #[error("interaction request id has already been used with different content")]
    InteractionRequestConflict,
    #[error("discovery request id has already been used with different content")]
    DiscoveryRequestConflict,
    #[error("movement request id has already been used with different content")]
    MovementRequestConflict,
    #[error("investigation request id has already been used for another kind")]
    InvestigationRequestConflict,
    #[error("investigation attempt is unavailable")]
    DiscoveryAttemptUnavailable,
    #[error("investigation was not admitted")]
    InvestigationNotAdmitted,
    #[error("selected place is unavailable for discovery")]
    PlaceUnavailable,
    #[error("selected connection is unavailable for movement")]
    ConnectionUnavailable,
    #[error("selected connection does not allow that direction")]
    ConnectionDirectionDisallowed,
    #[error("character or target is off the submitted course segment")]
    MovementOffCourse,
    #[error("movement target makes no progress")]
    MovementNoProgress,
    #[error("one or more interaction targets are unavailable")]
    InteractionTargetUnavailable,
    #[error("one or more property entities are unavailable")]
    PropertyEntityUnavailable,
    #[error("one or more traits are unavailable")]
    TraitUnavailable,
    #[error("entity at current place is unavailable")]
    EntityAtCurrentPlaceUnavailable,
    #[error("property key already exists with a different value type")]
    PropertyKeyConflict,
    #[error("current place has changed since it was read")]
    PlaceRevisionConflict,
    #[error("current position has changed since it was read")]
    PositionRevisionConflict,
    #[error("world storage is unavailable")]
    Unavailable,
    #[error("world storage is temporarily unavailable")]
    TemporarilyUnavailable,
}

pub(super) fn invalid_stored_relation() -> WorldError {
    storage_error(
        "decode_world_state",
        sqlx::Error::ColumnNotFound("inconsistent relation".to_owned()),
    )
}

pub(super) fn invalid_connection(field: ConnectionField, reason: InvalidReason) -> WorldError {
    WorldError::InvalidConnection { field, reason }
}

pub(super) fn constraint(error: &sqlx::Error) -> Option<&str> {
    error
        .as_database_error()
        .and_then(|error| error.constraint())
}

pub(super) fn storage_error(operation: &'static str, error: sqlx::Error) -> WorldError {
    let database_code = error
        .as_database_error()
        .and_then(|error| error.code())
        .map(|code| code.into_owned());
    if matches!(
        operation,
        "start_investigation" | "submit_discovery" | "move_character"
    ) && matches!(database_code.as_deref(), Some("57014" | "55P03"))
    {
        return WorldError::TemporarilyUnavailable;
    }
    let category = match error {
        sqlx::Error::PoolTimedOut | sqlx::Error::PoolClosed | sqlx::Error::WorkerCrashed => "pool",
        sqlx::Error::Io(_) | sqlx::Error::Tls(_) => "connection",
        sqlx::Error::Database(_) => "database",
        _ => "other",
    };
    eprintln!(
        "{}",
        serde_json::json!({
            "owner": "world",
            "operation": operation,
            "status": "unavailable",
            "category": category,
            "recovery": "retry_later"
        })
    );
    WorldError::Unavailable
}

pub(super) fn spatial_read_error(operation: &'static str, error: sqlx::Error) -> WorldError {
    if error
        .as_database_error()
        .and_then(|error| error.code())
        .as_deref()
        == Some("57014")
    {
        return WorldError::TemporarilyUnavailable;
    }
    storage_error(operation, error)
}
