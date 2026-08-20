use super::*;

#[derive(Debug)]
pub(super) struct HttpError(ErrorOutput);

impl From<ErrorOutput> for HttpError {
    fn from(value: ErrorOutput) -> Self {
        Self(value)
    }
}

impl IntoResponse for HttpError {
    fn into_response(self) -> Response {
        // Exhaustive on purpose: a new ErrorCode without a published status stops
        // compiling here instead of silently defaulting to 400.
        let status = match self.0.error.code {
            ErrorCode::UserContextRequired
            | ErrorCode::InvalidRequest
            | ErrorCode::InvalidEntity
            | ErrorCode::InvalidCharacter
            | ErrorCode::InvalidPlace
            | ErrorCode::InvalidPosition
            | ErrorCode::InvalidPlaceWindow
            | ErrorCode::InvalidConnection
            | ErrorCode::InvalidMovement
            | ErrorCode::InvalidAction
            | ErrorCode::InvalidInteraction
            | ErrorCode::InvalidDiscovery
            | ErrorCode::InvalidProperty
            | ErrorCode::InvalidTrait
            | ErrorCode::InvalidEntityLimit
            | ErrorCode::InvalidActivityLimit
            | ErrorCode::InvalidPlaceLimit
            | ErrorCode::InvalidConnectionLimit => StatusCode::BAD_REQUEST,
            ErrorCode::UserNotFound
            | ErrorCode::CharacterNotFound
            | ErrorCode::PlaceNotFound
            | ErrorCode::ConnectionNotFound
            | ErrorCode::EntryPlaceNotFound => StatusCode::NOT_FOUND,
            ErrorCode::CharacterAlreadyExists
            | ErrorCode::CharacterAlreadyEntered
            | ErrorCode::CharacterNotEntered
            | ErrorCode::CharacterNotAtPlace
            | ErrorCode::EntryPlaceAlreadyExists
            | ErrorCode::ActionRequestConflict
            | ErrorCode::InteractionRequestConflict
            | ErrorCode::DiscoveryRequestConflict
            | ErrorCode::MovementRequestConflict
            | ErrorCode::InvestigationRequestConflict
            | ErrorCode::DiscoveryAttemptUnavailable
            | ErrorCode::PlaceUnavailable
            | ErrorCode::ConnectionUnavailable
            | ErrorCode::ConnectionDirectionDisallowed
            | ErrorCode::MovementOffCourse
            | ErrorCode::MovementNoProgress
            | ErrorCode::InteractionTargetUnavailable
            | ErrorCode::PropertyEntityUnavailable
            | ErrorCode::EntityAtCurrentPlaceUnavailable
            | ErrorCode::TraitUnavailable
            | ErrorCode::PropertyKeyConflict => StatusCode::CONFLICT,
            ErrorCode::PlaceRevisionConflict => StatusCode::PRECONDITION_FAILED,
            ErrorCode::PositionRevisionConflict => StatusCode::PRECONDITION_FAILED,
            ErrorCode::InvestigationNotAdmitted => StatusCode::TOO_MANY_REQUESTS,
            ErrorCode::Unavailable | ErrorCode::TemporarilyUnavailable => {
                StatusCode::SERVICE_UNAVAILABLE
            }
        };
        (status, HttpJson(self.0)).into_response()
    }
}

pub(super) fn user_context(headers: &HeaderMap) -> Result<crate::UserId, ErrorOutput> {
    let mut values = headers.get_all(USER_CONTEXT_HEADER).iter();
    let value = values.next();
    if values.next().is_some() {
        return Err(ErrorOutput::multiple_user_context());
    }
    let value = value
        .map(|value| {
            value
                .to_str()
                .map_err(|_| ErrorOutput::invalid_user_context())
        })
        .transpose()?;
    if value.is_some_and(|value| value.contains(',')) {
        return Err(ErrorOutput::multiple_user_context());
    }
    parse_user_context(value)
}
