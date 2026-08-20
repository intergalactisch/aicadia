use super::*;

use axum::{
    extract::{Path, Query, State, rejection::JsonRejection, rejection::QueryRejection},
    routing::{get, post},
};
use utoipa::{OpenApi, openapi::OpenApi as OpenApiDocument};
use uuid::Uuid;

pub(super) fn routes() -> Router<World> {
    Router::new()
        .route("/api/world", get(get_world))
        .route("/api/user", get(get_user))
        .route("/api/character", get(get_character).post(create_character))
        .route("/api/place/entry", post(create_entry_place))
        .route("/api/place", get(list_place))
        .route("/api/place/{place_id}/connection", get(list_connection))
        .route(
            "/api/place/{place_id}/connection/{connection_id}",
            get(get_connection),
        )
        .route("/api/world/entry", post(enter_world))
        .route("/api/activity", get(list_activity))
        .route("/api/entity", post(create_entity))
        .route(
            "/api/place/current/entity",
            get(list_entity_at_current_place),
        )
        .route(
            "/api/place/current/activity",
            get(list_activity_at_current_place),
        )
        .route(
            "/api/place/current/entity/{entity_id}",
            get(get_entity_at_current_place),
        )
        .route("/api/investigation", post(start_investigation))
        .route("/api/action", post(submit_action))
        .route("/api/interaction", post(submit_interaction))
        .route("/api/discovery", post(submit_discovery))
        .route("/api/character/movement", post(move_character))
        .route("/api/openapi.json", get(openapi))
}

#[derive(OpenApi)]
#[openapi(
    paths(
        get_world,
        get_user,
        get_character,
        create_character,
        create_entry_place,
        list_place,
        list_connection,
        get_connection,
        enter_world,
        list_activity,
        create_entity,
        list_entity_at_current_place,
        list_activity_at_current_place,
        get_entity_at_current_place,
        start_investigation,
        submit_action,
        submit_interaction,
        submit_discovery,
        move_character
    ),
    components(schemas(
        WorldOutput,
        UserOutput,
        CharacterOutput,
        CharacterEntityStatePageOutput,
        CreateCharacterInput,
        PlaceOutput,
        PlacePageOutput,
        ConnectionPageOutput,
        ConnectionOutput,
        ConnectionPointOutput,
        CreateEntryPlaceInput,
        ActivityPageOutput,
        EntityOutput,
        CreateEntityInput,
        CurrentPlaceEntityPageOutput,
        CurrentPlaceActivityPageOutput,
        CurrentPlaceEntityStatePageOutput,
        StartInvestigationInput,
        InvestigationResultOutput,
        SubmitActionInput,
        AcceptedActionOutput,
        SubmitInteractionInput,
        AcceptedInteractionOutput,
        SubmitDiscoveryInput,
        AcceptedDiscoveryOutput,
        MoveCharacterInput,
        AcceptedMovementOutput,
        ErrorDetail,
        ErrorOutput
    )),
    info(title = "Aicadia API", version = "0.1.0")
)]
struct ApiDocument;

/// The compiled OpenAPI document; Studio reads its routes instead of copying them.
pub fn openapi_document() -> OpenApiDocument {
    ApiDocument::openapi()
}

async fn openapi() -> HttpJson<OpenApiDocument> {
    HttpJson(openapi_document())
}

#[utoipa::path(
    get,
    path = "/api/world",
    responses((status = 200, description = "Shared World", body = WorldOutput))
)]
async fn get_world(State(world): State<World>) -> HttpJson<WorldOutput> {
    HttpJson(world.get_world().into())
}

#[utoipa::path(
    get,
    path = "/api/user",
    params(("Aicadia-User-Id" = Uuid, Header, description = "Untrusted development User context")),
    responses(
        (status = 200, description = "Current User", body = UserOutput),
        (status = 400, description = "Invalid User context", body = ErrorOutput),
        (status = 404, description = "User not found", body = ErrorOutput),
        (status = 503, description = "World unavailable", body = ErrorOutput)
    )
)]
async fn get_user(
    State(world): State<World>,
    headers: HeaderMap,
) -> Result<HttpJson<UserOutput>, HttpError> {
    let user_id = user_context(&headers)?;
    world
        .get_user(user_id)
        .await
        .map(UserOutput::from)
        .map(HttpJson)
        .map_err(ErrorOutput::from_world)
        .map_err(Into::into)
}

#[utoipa::path(
    get,
    path = "/api/character",
    params(
        ("Aicadia-User-Id" = Uuid, Header, description = "Untrusted development User context"),
        GetEntityCurrentStateInput
    ),
    responses(
        (status = 200, description = "Current Character with one bounded current-state page", body = CharacterEntityStatePageOutput),
        (status = 400, description = "Invalid User context, cursor or limit", body = ErrorOutput),
        (status = 404, description = "User or Character not found", body = ErrorOutput),
        (status = 503, description = "World unavailable", body = ErrorOutput)
    )
)]
async fn get_character(
    State(world): State<World>,
    headers: HeaderMap,
    query: Result<Query<GetEntityCurrentStateInput>, QueryRejection>,
) -> Result<HttpJson<CharacterEntityStatePageOutput>, HttpError> {
    let user_id = user_context(&headers)?;
    let input = query
        .map_err(|_| ErrorOutput::malformed_request("character query is invalid"))?
        .0
        .parse_character()?;
    world
        .get_character(user_id, input)
        .await
        .map(CharacterEntityStatePageOutput::from)
        .map(HttpJson)
        .map_err(ErrorOutput::from_world)
        .map_err(Into::into)
}

#[utoipa::path(
    post,
    path = "/api/character",
    params(("Aicadia-User-Id" = Uuid, Header, description = "Untrusted development User context")),
    request_body = CreateCharacterInput,
    responses(
        (status = 201, description = "Created Character", body = CharacterOutput),
        (status = 400, description = "Invalid Character or User context", body = ErrorOutput),
        (status = 404, description = "User not found", body = ErrorOutput),
        (status = 409, description = "Character already exists", body = ErrorOutput),
        (status = 503, description = "World unavailable", body = ErrorOutput)
    )
)]
async fn create_character(
    State(world): State<World>,
    headers: HeaderMap,
    body: Result<HttpJson<CreateCharacterInput>, JsonRejection>,
) -> Result<(StatusCode, HttpJson<CharacterOutput>), HttpError> {
    let user_id = user_context(&headers)?;
    let input = body
        .map_err(|_| ErrorOutput::malformed_request("character body is invalid"))?
        .0;
    world
        .create_character(user_id, input.into())
        .await
        .map(CharacterOutput::from)
        .map(|character| (StatusCode::CREATED, HttpJson(character)))
        .map_err(ErrorOutput::from_world)
        .map_err(Into::into)
}

#[utoipa::path(
    post,
    path = "/api/place/entry",
    params(("Aicadia-User-Id" = Uuid, Header, description = "Untrusted development User context")),
    request_body = CreateEntryPlaceInput,
    responses(
        (status = 201, description = "Created entry Place", body = PlaceOutput),
        (status = 400, description = "Invalid Place or User context", body = ErrorOutput),
        (status = 404, description = "User or Character not found", body = ErrorOutput),
        (status = 409, description = "Entry Place already exists or Character is placed", body = ErrorOutput),
        (status = 503, description = "World unavailable", body = ErrorOutput)
    )
)]
async fn create_entry_place(
    State(world): State<World>,
    headers: HeaderMap,
    body: Result<HttpJson<CreateEntryPlaceInput>, JsonRejection>,
) -> Result<(StatusCode, HttpJson<PlaceOutput>), HttpError> {
    let user_id = user_context(&headers)?;
    let input = body
        .map_err(|_| ErrorOutput::malformed_request("entry Place body is invalid"))?
        .0;
    world
        .create_entry_place(user_id, input.into())
        .await
        .map(PlaceOutput::from)
        .map(|place| (StatusCode::CREATED, HttpJson(place)))
        .map_err(ErrorOutput::from_world)
        .map_err(Into::into)
}

#[utoipa::path(
    post,
    path = "/api/world/entry",
    params(("Aicadia-User-Id" = Uuid, Header, description = "Untrusted development User context")),
    responses(
        (status = 200, description = "Current Character placed at the entry Place", body = CharacterOutput),
        (status = 400, description = "Invalid User context", body = ErrorOutput),
        (status = 404, description = "User, Character or entry Place not found", body = ErrorOutput),
        (status = 503, description = "World unavailable", body = ErrorOutput)
    )
)]
async fn enter_world(
    State(world): State<World>,
    headers: HeaderMap,
) -> Result<HttpJson<CharacterOutput>, HttpError> {
    let user_id = user_context(&headers)?;
    world
        .enter_world(user_id)
        .await
        .map(CharacterOutput::from)
        .map(HttpJson)
        .map_err(ErrorOutput::from_world)
        .map_err(Into::into)
}

#[utoipa::path(
    get,
    path = "/api/activity",
    params(
        ("Aicadia-User-Id" = Uuid, Header, description = "Untrusted development User context"),
        ListActivityInput
    ),
    responses(
        (status = 200, description = "Current Character activity page", body = ActivityPageOutput),
        (status = 400, description = "Invalid User context or list input", body = ErrorOutput),
        (status = 404, description = "User or Character not found", body = ErrorOutput),
        (status = 503, description = "World unavailable", body = ErrorOutput)
    )
)]
async fn list_activity(
    State(world): State<World>,
    headers: HeaderMap,
    query: Result<Query<ListActivityInput>, QueryRejection>,
) -> Result<HttpJson<ActivityPageOutput>, HttpError> {
    let user_id = user_context(&headers)?;
    let input = query
        .map_err(|_| ErrorOutput::malformed_request("activity query is invalid"))?
        .0
        .parse()?;
    world
        .list_activity(user_id, input)
        .await
        .map(ActivityPageOutput::from)
        .map(HttpJson)
        .map_err(ErrorOutput::from_world)
        .map_err(Into::into)
}

#[utoipa::path(
    post,
    path = "/api/entity",
    params(("Aicadia-User-Id" = Uuid, Header, description = "Untrusted development User context")),
    request_body = CreateEntityInput,
    responses(
        (status = 201, description = "Created Entity", body = EntityOutput),
        (status = 400, description = "Invalid Entity or User context", body = ErrorOutput),
        (status = 404, description = "User not found", body = ErrorOutput),
        (status = 503, description = "World unavailable", body = ErrorOutput)
    )
)]
async fn create_entity(
    State(world): State<World>,
    headers: HeaderMap,
    body: Result<HttpJson<CreateEntityInput>, JsonRejection>,
) -> Result<(StatusCode, HttpJson<EntityOutput>), HttpError> {
    let user_id = user_context(&headers)?;
    let input = body
        .map_err(|_| ErrorOutput::malformed_request("entity body is invalid"))?
        .0;
    world
        .create_entity(user_id, input.into())
        .await
        .map(EntityOutput::from)
        .map(|entity| (StatusCode::CREATED, HttpJson(entity)))
        .map_err(ErrorOutput::from_world)
        .map_err(Into::into)
}

#[utoipa::path(
    get,
    path = "/api/place/current/entity",
    params(
        ("Aicadia-User-Id" = Uuid, Header, description = "Untrusted development User context"),
        ListEntityAtCurrentPlaceInput
    ),
    responses(
        (status = 200, description = "Entities at the exact current Place", body = CurrentPlaceEntityPageOutput),
        (status = 400, description = "Invalid User context or list input", body = ErrorOutput),
        (status = 404, description = "User or Character not found", body = ErrorOutput),
        (status = 409, description = "Character has not entered the World", body = ErrorOutput),
        (status = 503, description = "World unavailable", body = ErrorOutput)
    )
)]
async fn list_entity_at_current_place(
    State(world): State<World>,
    headers: HeaderMap,
    query: Result<Query<ListEntityAtCurrentPlaceInput>, QueryRejection>,
) -> Result<HttpJson<CurrentPlaceEntityPageOutput>, HttpError> {
    let user_id = user_context(&headers)?;
    let input = query
        .map_err(|_| ErrorOutput::malformed_request("current Place Entity query is invalid"))?
        .0
        .parse()?;
    world
        .list_entity_at_current_place(user_id, input)
        .await
        .map(CurrentPlaceEntityPageOutput::from)
        .map(HttpJson)
        .map_err(ErrorOutput::from_world)
        .map_err(Into::into)
}

#[utoipa::path(
    get,
    path = "/api/place/current/activity",
    params(
        ("Aicadia-User-Id" = Uuid, Header, description = "Untrusted development User context"),
        ListActivityAtCurrentPlaceInput
    ),
    responses(
        (status = 200, description = "Activity at the exact current Place", body = CurrentPlaceActivityPageOutput),
        (status = 400, description = "Invalid User context or list input", body = ErrorOutput),
        (status = 404, description = "User or Character not found", body = ErrorOutput),
        (status = 409, description = "Character has not entered the World", body = ErrorOutput),
        (status = 503, description = "World unavailable", body = ErrorOutput)
    )
)]
async fn list_activity_at_current_place(
    State(world): State<World>,
    headers: HeaderMap,
    query: Result<Query<ListActivityAtCurrentPlaceInput>, QueryRejection>,
) -> Result<HttpJson<CurrentPlaceActivityPageOutput>, HttpError> {
    let user_id = user_context(&headers)?;
    let input = query
        .map_err(|_| ErrorOutput::malformed_request("current Place Activity query is invalid"))?
        .0
        .parse()?;
    world
        .list_activity_at_current_place(user_id, input)
        .await
        .map(CurrentPlaceActivityPageOutput::from)
        .map(HttpJson)
        .map_err(ErrorOutput::from_world)
        .map_err(Into::into)
}

#[utoipa::path(
    get,
    path = "/api/place/current/entity/{entity_id}",
    params(
        ("Aicadia-User-Id" = Uuid, Header, description = "Untrusted development User context"),
        ("entity_id" = Uuid, Path, description = "Exact-local Entity id"),
        GetEntityCurrentStateInput
    ),
    responses(
        (status = 200, description = "One exact-local Entity with one bounded current-state page", body = CurrentPlaceEntityStatePageOutput),
        (status = 400, description = "Invalid User context, Entity id, cursor or limit", body = ErrorOutput),
        (status = 404, description = "User or Character not found", body = ErrorOutput),
        (status = 409, description = "Character not entered or selected Entity unavailable", body = ErrorOutput),
        (status = 412, description = "Exact current Place changed after it was read", body = ErrorOutput),
        (status = 503, description = "World unavailable", body = ErrorOutput)
    )
)]
async fn get_entity_at_current_place(
    State(world): State<World>,
    headers: HeaderMap,
    Path(entity_id): Path<String>,
    query: Result<Query<GetEntityCurrentStateInput>, QueryRejection>,
) -> Result<HttpJson<CurrentPlaceEntityStatePageOutput>, HttpError> {
    let user_id = user_context(&headers)?;
    let entity_id = GetEntityInput { entity_id }.parse()?;
    let input = query
        .map_err(|_| ErrorOutput::malformed_request("current Place Entity-state query is invalid"))?
        .0
        .parse_current_place_entity(entity_id)?;
    world
        .get_entity_at_current_place(user_id, input)
        .await
        .map(CurrentPlaceEntityStatePageOutput::from)
        .map(HttpJson)
        .map_err(ErrorOutput::from_world)
        .map_err(Into::into)
}

#[utoipa::path(
    get,
    path = "/api/place",
    params(("Aicadia-User-Id" = Uuid, Header, description = "Untrusted development User context"), ListPlaceInput),
    responses(
        (status = 200, description = "Bounded shared Place page", body = PlacePageOutput),
        (status = 400, description = "Invalid Place window, cursor or limit", body = ErrorOutput),
        (status = 404, description = "User or Character not found", body = ErrorOutput),
        (status = 409, description = "Character has not entered", body = ErrorOutput),
        (status = 503, description = "World unavailable", body = ErrorOutput)
    )
)]
async fn list_place(
    State(world): State<World>,
    headers: HeaderMap,
    query: Result<Query<ListPlaceInput>, QueryRejection>,
) -> Result<HttpJson<PlacePageOutput>, HttpError> {
    let user_id = user_context(&headers)?;
    let input = query
        .map_err(|_| ErrorOutput::malformed_request("Place query is invalid"))?
        .0
        .parse()?;
    world
        .list_place(user_id, input)
        .await
        .map(Into::into)
        .map(HttpJson)
        .map_err(ErrorOutput::from_world)
        .map_err(Into::into)
}

#[utoipa::path(
    get,
    path = "/api/place/{place_id}/connection",
    params(("Aicadia-User-Id" = Uuid, Header, description = "Untrusted development User context"), ("place_id" = Uuid, Path, description = "Anchor Place id"), ListConnectionPageInput),
    responses(
        (status = 200, description = "Bounded incident Connection page", body = ConnectionPageOutput),
        (status = 400, description = "Invalid id, cursor or limit", body = ErrorOutput),
        (status = 404, description = "User, Character or Place not found", body = ErrorOutput),
        (status = 409, description = "Character has not entered", body = ErrorOutput),
        (status = 503, description = "World unavailable", body = ErrorOutput)
    )
)]
async fn list_connection(
    State(world): State<World>,
    headers: HeaderMap,
    Path(place_id): Path<String>,
    query: Result<Query<ListConnectionPageInput>, QueryRejection>,
) -> Result<HttpJson<ConnectionPageOutput>, HttpError> {
    let user_id = user_context(&headers)?;
    let place_id = Uuid::parse_str(&place_id).map_err(|_| ErrorOutput::invalid_place_id())?;
    let input = query
        .map_err(|_| ErrorOutput::malformed_request("Connection query is invalid"))?
        .0
        .parse(place_id)?;
    world
        .list_connection(user_id, input)
        .await
        .map(Into::into)
        .map(HttpJson)
        .map_err(ErrorOutput::from_world)
        .map_err(Into::into)
}

#[utoipa::path(
    get,
    path = "/api/place/{place_id}/connection/{connection_id}",
    params(("Aicadia-User-Id" = Uuid, Header, description = "Untrusted development User context"), ("place_id" = Uuid, Path), ("connection_id" = Uuid, Path)),
    responses(
        (status = 200, description = "Selected complete Connection", body = ConnectionOutput),
        (status = 400, description = "Invalid id", body = ErrorOutput),
        (status = 404, description = "Connection not found", body = ErrorOutput),
        (status = 409, description = "Character has not entered", body = ErrorOutput),
        (status = 503, description = "World unavailable", body = ErrorOutput)
    )
)]
async fn get_connection(
    State(world): State<World>,
    headers: HeaderMap,
    Path((place_id, connection_id)): Path<(String, String)>,
) -> Result<HttpJson<ConnectionOutput>, HttpError> {
    let user_id = user_context(&headers)?;
    let place_id = Uuid::parse_str(&place_id).map_err(|_| ErrorOutput::invalid_place_id())?;
    let connection_id =
        Uuid::parse_str(&connection_id).map_err(|_| ErrorOutput::invalid_connection_id())?;
    world
        .get_connection(
            user_id,
            GetConnectionInput {
                place_id,
                connection_id,
            }
            .into(),
        )
        .await
        .map(Into::into)
        .map(HttpJson)
        .map_err(ErrorOutput::from_world)
        .map_err(Into::into)
}

#[utoipa::path(
    post,
    path = "/api/investigation",
    request_body = StartInvestigationInput,
    params(("Aicadia-User-Id" = Uuid, Header, description = "Untrusted development User context")),
    responses(
        (status = 200, description = "Stored investigation result", body = InvestigationResultOutput),
        (status = 400, description = "Invalid User context or request", body = ErrorOutput),
        (status = 404, description = "User or Character not found", body = ErrorOutput),
        (status = 409, description = "Character has not entered", body = ErrorOutput),
        (status = 429, description = "Investigation not admitted", body = ErrorOutput),
        (status = 503, description = "World unavailable", body = ErrorOutput)
    )
)]
async fn start_investigation(
    State(world): State<World>,
    headers: HeaderMap,
    body: Result<HttpJson<StartInvestigationInput>, JsonRejection>,
) -> Result<HttpJson<InvestigationResultOutput>, HttpError> {
    let user_id = user_context(&headers)?;
    let input = body
        .map_err(|_| ErrorOutput::malformed_request("Investigation body is invalid"))?
        .0;
    world
        .start_investigation(user_id, input.into())
        .await
        .map(InvestigationResultOutput::from)
        .map(HttpJson)
        .map_err(ErrorOutput::from_world)
        .map_err(Into::into)
}

#[utoipa::path(
    post,
    path = "/api/action",
    params(("Aicadia-User-Id" = Uuid, Header, description = "Untrusted development User context")),
    request_body = SubmitActionInput,
    responses(
        (status = 201, description = "Accepted action", body = AcceptedActionOutput),
        (status = 400, description = "Invalid action or User context", body = ErrorOutput),
        (status = 404, description = "User or Character not found", body = ErrorOutput),
        (status = 409, description = "Character is unplaced or request id conflicts", body = ErrorOutput),
        (status = 412, description = "Exact current Place changed after it was read", body = ErrorOutput),
        (status = 503, description = "World unavailable", body = ErrorOutput)
    )
)]
async fn submit_action(
    State(world): State<World>,
    headers: HeaderMap,
    body: Result<HttpJson<SubmitActionInput>, JsonRejection>,
) -> Result<(StatusCode, HttpJson<AcceptedActionOutput>), HttpError> {
    let user_id = user_context(&headers)?;
    let input = body
        .map_err(|_| ErrorOutput::malformed_request("action body is invalid"))?
        .0
        .parse()?;
    world
        .submit_action(user_id, input)
        .await
        .map(AcceptedActionOutput::from)
        .map(|action| (StatusCode::CREATED, HttpJson(action)))
        .map_err(ErrorOutput::from_world)
        .map_err(Into::into)
}

#[utoipa::path(
    post,
    path = "/api/interaction",
    params(("Aicadia-User-Id" = Uuid, Header, description = "Untrusted development User context")),
    request_body = SubmitInteractionInput,
    responses(
        (status = 201, description = "Accepted Interaction", body = AcceptedInteractionOutput),
        (status = 400, description = "Invalid Interaction or User context", body = ErrorOutput),
        (status = 404, description = "User or Character not found", body = ErrorOutput),
        (status = 409, description = "Character is unplaced, target unavailable or request id conflicts", body = ErrorOutput),
        (status = 412, description = "Exact current Place changed after it was read", body = ErrorOutput),
        (status = 503, description = "World unavailable", body = ErrorOutput)
    )
)]
async fn submit_interaction(
    State(world): State<World>,
    headers: HeaderMap,
    body: Result<HttpJson<SubmitInteractionInput>, JsonRejection>,
) -> Result<(StatusCode, HttpJson<AcceptedInteractionOutput>), HttpError> {
    let user_id = user_context(&headers)?;
    let input = body
        .map_err(|_| ErrorOutput::malformed_request("Interaction body is invalid"))?
        .0
        .parse()?;
    world
        .submit_interaction(user_id, input)
        .await
        .map(AcceptedInteractionOutput::from)
        .map(|interaction| (StatusCode::CREATED, HttpJson(interaction)))
        .map_err(ErrorOutput::from_world)
        .map_err(Into::into)
}

#[utoipa::path(
    post,
    path = "/api/discovery",
    request_body = SubmitDiscoveryInput,
    params(("Aicadia-User-Id" = Uuid, Header, description = "Untrusted development User context")),
    responses(
        (status = 201, description = "Accepted discovery", body = AcceptedDiscoveryOutput),
        (status = 400, description = "Invalid User context, prose or find", body = ErrorOutput),
        (status = 404, description = "User or Character not found", body = ErrorOutput),
        (status = 409, description = "Request conflict or attempt unavailable", body = ErrorOutput),
        (status = 503, description = "World unavailable", body = ErrorOutput)
    )
)]
async fn submit_discovery(
    State(world): State<World>,
    headers: HeaderMap,
    body: Result<HttpJson<SubmitDiscoveryInput>, JsonRejection>,
) -> Result<(StatusCode, HttpJson<AcceptedDiscoveryOutput>), HttpError> {
    let user_id = user_context(&headers)?;
    let input = body
        .map_err(|_| ErrorOutput::malformed_request("Discovery body is invalid"))?
        .0;
    world
        .submit_discovery(user_id, input.into())
        .await
        .map(AcceptedDiscoveryOutput::from)
        .map(|discovery| (StatusCode::CREATED, HttpJson(discovery)))
        .map_err(ErrorOutput::from_world)
        .map_err(Into::into)
}

#[utoipa::path(
    post,
    path = "/api/character/movement",
    params(("Aicadia-User-Id" = Uuid, Header, description = "Untrusted development User context")),
    request_body = MoveCharacterInput,
    responses(
        (status = 200, description = "Accepted Character Movement", body = AcceptedMovementOutput),
        (status = 400, description = "Invalid Movement or User context", body = ErrorOutput),
        (status = 404, description = "User or Character not found", body = ErrorOutput),
        (status = 409, description = "Movement conflict or unavailable Connection", body = ErrorOutput),
        (status = 412, description = "Character Position changed after it was read", body = ErrorOutput),
        (status = 503, description = "World unavailable", body = ErrorOutput)
    )
)]
async fn move_character(
    State(world): State<World>,
    headers: HeaderMap,
    body: Result<HttpJson<MoveCharacterInput>, JsonRejection>,
) -> Result<HttpJson<AcceptedMovementOutput>, HttpError> {
    let user_id = user_context(&headers)?;
    let input = body
        .map_err(|_| ErrorOutput::malformed_request("Movement body is invalid"))?
        .0
        .parse()?;
    world
        .move_character(user_id, input)
        .await
        .map(Into::into)
        .map(HttpJson)
        .map_err(ErrorOutput::from_world)
        .map_err(Into::into)
}
