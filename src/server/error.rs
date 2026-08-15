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
        let status = match self.0.error.code {
            ErrorCode::UserNotFound
            | ErrorCode::EntityNotFound
            | ErrorCode::CharacterNotFound
            | ErrorCode::EntryPlaceNotFound => StatusCode::NOT_FOUND,
            ErrorCode::CharacterAlreadyExists
            | ErrorCode::CharacterAlreadyEntered
            | ErrorCode::CharacterNotEntered
            | ErrorCode::EntryPlaceAlreadyExists
            | ErrorCode::ActionRequestConflict
            | ErrorCode::InteractionRequestConflict
            | ErrorCode::DiscoveryRequestConflict
            | ErrorCode::DiscoveryAttemptUnavailable
            | ErrorCode::InteractionTargetUnavailable
            | ErrorCode::PropertyEntityUnavailable
            | ErrorCode::EntityAtCurrentPlaceUnavailable
            | ErrorCode::TraitUnavailable
            | ErrorCode::PropertyKeyConflict => StatusCode::CONFLICT,
            ErrorCode::PlaceRevisionConflict => StatusCode::PRECONDITION_FAILED,
            ErrorCode::InvestigationNotAdmitted => StatusCode::TOO_MANY_REQUESTS,
            ErrorCode::Unavailable => StatusCode::SERVICE_UNAVAILABLE,
            _ => StatusCode::BAD_REQUEST,
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
