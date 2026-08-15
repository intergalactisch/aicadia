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
    #[error("user was not found")]
    UserNotFound,
    #[error("entity was not found")]
    EntityNotFound,
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
    #[error("character has not entered the world")]
    CharacterNotEntered,
    #[error("action request id has already been used with different content")]
    ActionRequestConflict,
    #[error("interaction request id has already been used with different content")]
    InteractionRequestConflict,
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
    #[error("world storage is unavailable")]
    Unavailable,
}

pub(super) fn invalid_stored_relation() -> WorldError {
    storage_error(
        "decode_world_state",
        sqlx::Error::ColumnNotFound("inconsistent relation".to_owned()),
    )
}

pub(super) fn constraint(error: &sqlx::Error) -> Option<&str> {
    error
        .as_database_error()
        .and_then(|error| error.constraint())
}

pub(super) fn storage_error(operation: &'static str, error: sqlx::Error) -> WorldError {
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
