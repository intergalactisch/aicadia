use std::{borrow::Cow, net::SocketAddr, sync::Arc};

use axum::{
    Json as HttpJson, Router,
    extract::{Path, Query, State, rejection::JsonRejection, rejection::QueryRejection},
    http::{HeaderMap, StatusCode, request::Parts},
    response::{Html, IntoResponse, Response},
    routing::{get, post},
};
use rmcp::{
    ServerHandler,
    handler::server::{router::tool::ToolRouter, tool::Extension, wrapper::Json},
    model::{
        CallToolResult, Implementation, JsonObject, ProtocolVersion, ServerCapabilities, ServerInfo,
    },
    tool, tool_handler, tool_router,
    transport::streamable_http_server::{
        StreamableHttpServerConfig, StreamableHttpService, session::never::NeverSessionManager,
    },
};
use schemars::JsonSchema;
use serde::{Deserialize, de::DeserializeOwned};
use thiserror::Error;
use utoipa::{OpenApi, openapi::OpenApi as OpenApiDocument};

use crate::{
    World, agent_contract,
    wire::{
        AcceptedActionOutput, AcceptedInteractionOutput, ActivityPageOutput,
        CharacterEntityStatePageOutput, CharacterOutput, CreateCharacterInput, CreateEntityInput,
        CreateEntryPlaceInput, CurrentPlaceActivityPageOutput, CurrentPlaceEntityPageOutput,
        CurrentPlaceEntityStatePageOutput, EntityOutput, EntityPageOutput, ErrorCode, ErrorDetail,
        ErrorOutput, GetEntityAtCurrentPlaceInput, GetEntityCurrentStateInput, GetEntityInput,
        ListActivityAtCurrentPlaceInput, ListActivityInput, ListEntityAtCurrentPlaceInput,
        ListEntityInput, PlaceOutput, SubmitActionInput, SubmitInteractionInput,
        USER_CONTEXT_HEADER, UserOutput, WorldOutput, parse_user_context,
    },
};

const MCP_VERSION: &str = env!("CARGO_PKG_VERSION");
const LEDGER_HTML: &str = include_str!("../web/index.html");
fn mcp_input_schema<T: JsonSchema + 'static>() -> Arc<JsonObject> {
    rmcp::handler::server::common::schema_for_input::<T>()
        .unwrap_or_else(|error| panic!("invalid fixed MCP input schema: {error}"))
}

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
struct EmptyInput {}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ServerError {
    #[error("Aicadia must bind to a loopback address")]
    NonLoopback,
}

pub fn app(world: World, address: SocketAddr) -> Result<Router, ServerError> {
    if !address.ip().is_loopback() {
        return Err(ServerError::NonLoopback);
    }

    let allowed_origin = [format!("http://{address}")];
    let mcp_config = StreamableHttpServerConfig::default()
        .with_legacy_session_mode(false)
        .with_json_response(true)
        .with_sse_keep_alive(None)
        .with_sse_retry(None)
        .with_allowed_origins(allowed_origin)
        .with_stateless_protocol_metadata_required(true);
    let mcp_world = world.clone();
    let mcp: StreamableHttpService<AicadiaMcp, NeverSessionManager> = StreamableHttpService::new(
        move || Ok(AicadiaMcp::new(mcp_world.clone())),
        Arc::new(NeverSessionManager::default()),
        mcp_config,
    );

    Ok(Router::new()
        .route("/", get(ledger))
        .route("/api/world", get(get_world))
        .route("/api/user", get(get_user))
        .route("/api/character", get(get_character).post(create_character))
        .route("/api/place/entry", post(create_entry_place))
        .route("/api/world/entry", post(enter_world))
        .route("/api/activity", get(list_activity))
        .route("/api/entity", get(list_entity).post(create_entity))
        .route("/api/entity/{entity_id}", get(get_entity))
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
        .route("/api/action", post(submit_action))
        .route("/api/interaction", post(submit_interaction))
        .route("/api/openapi.json", get(openapi))
        .nest_service("/mcp", mcp)
        .with_state(world))
}

async fn ledger() -> Html<&'static str> {
    Html(LEDGER_HTML)
}

#[derive(OpenApi)]
#[openapi(
    paths(
        get_world,
        get_user,
        get_character,
        create_character,
        create_entry_place,
        enter_world,
        list_activity,
        create_entity,
        list_entity_at_current_place,
        list_activity_at_current_place,
        get_entity_at_current_place,
        submit_action,
        submit_interaction
    ),
    components(schemas(
        WorldOutput,
        UserOutput,
        CharacterOutput,
        CharacterEntityStatePageOutput,
        CreateCharacterInput,
        PlaceOutput,
        CreateEntryPlaceInput,
        ActivityPageOutput,
        EntityOutput,
        EntityPageOutput,
        CreateEntityInput,
        CurrentPlaceEntityPageOutput,
        CurrentPlaceActivityPageOutput,
        CurrentPlaceEntityStatePageOutput,
        SubmitActionInput,
        AcceptedActionOutput,
        SubmitInteractionInput,
        AcceptedInteractionOutput,
        ErrorDetail,
        ErrorOutput
    )),
    info(title = "Aicadia API", version = "0.1.0")
)]
struct ApiDocument;

async fn openapi() -> HttpJson<OpenApiDocument> {
    HttpJson(ApiDocument::openapi())
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
    get,
    path = "/api/entity",
    params(ListEntityInput),
    responses(
        (status = 200, description = "Entity page", body = EntityPageOutput),
        (status = 400, description = "Invalid list input", body = ErrorOutput),
        (status = 503, description = "World unavailable", body = ErrorOutput)
    )
)]
async fn list_entity(
    State(world): State<World>,
    query: Result<Query<ListEntityInput>, QueryRejection>,
) -> Result<HttpJson<EntityPageOutput>, HttpError> {
    let input = query
        .map_err(|_| ErrorOutput::malformed_request("entity query is invalid"))?
        .0
        .parse()?;
    world
        .list_entity(input)
        .await
        .map(EntityPageOutput::from)
        .map(HttpJson)
        .map_err(ErrorOutput::from_world)
        .map_err(Into::into)
}

#[utoipa::path(
    get,
    path = "/api/entity/{entity_id}",
    params(("entity_id" = Uuid, Path, description = "Entity id")),
    responses(
        (status = 200, description = "Entity", body = EntityOutput),
        (status = 400, description = "Invalid Entity id", body = ErrorOutput),
        (status = 404, description = "Entity not found", body = ErrorOutput),
        (status = 503, description = "World unavailable", body = ErrorOutput)
    )
)]
async fn get_entity(
    State(world): State<World>,
    Path(entity_id): Path<String>,
) -> Result<HttpJson<EntityOutput>, HttpError> {
    let entity_id = GetEntityInput { entity_id }.parse()?;
    world
        .get_entity(entity_id)
        .await
        .map(EntityOutput::from)
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

#[derive(Debug)]
struct HttpError(ErrorOutput);

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
            | ErrorCode::InteractionTargetUnavailable
            | ErrorCode::PropertyEntityUnavailable
            | ErrorCode::EntityAtCurrentPlaceUnavailable
            | ErrorCode::TraitUnavailable
            | ErrorCode::PropertyKeyConflict => StatusCode::CONFLICT,
            ErrorCode::PlaceRevisionConflict => StatusCode::PRECONDITION_FAILED,
            ErrorCode::Unavailable => StatusCode::SERVICE_UNAVAILABLE,
            _ => StatusCode::BAD_REQUEST,
        };
        (status, HttpJson(self.0)).into_response()
    }
}

fn user_context(headers: &HeaderMap) -> Result<crate::UserId, ErrorOutput> {
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

#[derive(Clone)]
struct AicadiaMcp {
    world: World,
    tool_router: ToolRouter<Self>,
}

impl AicadiaMcp {
    fn new(world: World) -> Self {
        let mut tool_router = Self::tool_router();
        agent_contract::apply(&mut tool_router);
        Self { world, tool_router }
    }

    fn error(error: ErrorOutput) -> CallToolResult {
        let value = serde_json::to_string(&error)
            .expect("the fixed wire error contract is always JSON serializable");
        CallToolResult::error(vec![rmcp::model::ContentBlock::text(value)])
    }

    fn decode<T: DeserializeOwned>(
        input: JsonObject,
        message: &'static str,
    ) -> Result<T, CallToolResult> {
        serde_json::from_value(serde_json::Value::Object(input))
            .map_err(|_| ErrorOutput::malformed_request(message))
            .map_err(Self::error)
    }
}

#[tool_router]
impl AicadiaMcp {
    #[tool(
        input_schema = mcp_input_schema::<EmptyInput>(),
        annotations(
            title = "Get world",
            read_only_hint = true,
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = false
        )
    )]
    async fn get_world(&self, input: JsonObject) -> Result<Json<WorldOutput>, CallToolResult> {
        let _input: EmptyInput = Self::decode(input, "world input is invalid")?;
        Ok(Json(self.world.get_world().into()))
    }

    #[tool(
        input_schema = mcp_input_schema::<EmptyInput>(),
        annotations(
            title = "Get user",
            read_only_hint = true,
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = false
        )
    )]
    async fn get_user(
        &self,
        Extension(parts): Extension<Parts>,
        input: JsonObject,
    ) -> Result<Json<UserOutput>, CallToolResult> {
        let user_id = user_context(&parts.headers).map_err(Self::error)?;
        let _input: EmptyInput = Self::decode(input, "user input is invalid")?;
        self.world
            .get_user(user_id)
            .await
            .map(UserOutput::from)
            .map(Json)
            .map_err(ErrorOutput::from_world)
            .map_err(Self::error)
    }

    #[tool(
        input_schema = mcp_input_schema::<GetEntityCurrentStateInput>(),
        annotations(
            title = "Get character",
            read_only_hint = true,
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = false
        )
    )]
    async fn get_character(
        &self,
        Extension(parts): Extension<Parts>,
        input: JsonObject,
    ) -> Result<Json<CharacterEntityStatePageOutput>, CallToolResult> {
        let user_id = user_context(&parts.headers).map_err(Self::error)?;
        let input: GetEntityCurrentStateInput = Self::decode(input, "character input is invalid")?;
        let input = input.parse_character().map_err(Self::error)?;
        self.world
            .get_character(user_id, input)
            .await
            .map(CharacterEntityStatePageOutput::from)
            .map(Json)
            .map_err(ErrorOutput::from_world)
            .map_err(Self::error)
    }

    #[tool(
        input_schema = mcp_input_schema::<CreateCharacterInput>(),
        annotations(
            title = "Create character",
            read_only_hint = false,
            destructive_hint = false,
            idempotent_hint = false,
            open_world_hint = false
        )
    )]
    async fn create_character(
        &self,
        Extension(parts): Extension<Parts>,
        input: JsonObject,
    ) -> Result<Json<CharacterOutput>, CallToolResult> {
        let user_id = user_context(&parts.headers).map_err(Self::error)?;
        let input: CreateCharacterInput = Self::decode(input, "character body is invalid")?;
        self.world
            .create_character(user_id, input.into())
            .await
            .map(CharacterOutput::from)
            .map(Json)
            .map_err(ErrorOutput::from_world)
            .map_err(Self::error)
    }

    #[tool(
        input_schema = mcp_input_schema::<CreateEntryPlaceInput>(),
        annotations(
            title = "Create entry place",
            read_only_hint = false,
            destructive_hint = false,
            idempotent_hint = false,
            open_world_hint = false
        )
    )]
    async fn create_entry_place(
        &self,
        Extension(parts): Extension<Parts>,
        input: JsonObject,
    ) -> Result<Json<PlaceOutput>, CallToolResult> {
        let user_id = user_context(&parts.headers).map_err(Self::error)?;
        let input: CreateEntryPlaceInput = Self::decode(input, "entry Place body is invalid")?;
        self.world
            .create_entry_place(user_id, input.into())
            .await
            .map(PlaceOutput::from)
            .map(Json)
            .map_err(ErrorOutput::from_world)
            .map_err(Self::error)
    }

    #[tool(
        input_schema = mcp_input_schema::<EmptyInput>(),
        annotations(
            title = "Enter world",
            read_only_hint = false,
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = false
        )
    )]
    async fn enter_world(
        &self,
        Extension(parts): Extension<Parts>,
        input: JsonObject,
    ) -> Result<Json<CharacterOutput>, CallToolResult> {
        let user_id = user_context(&parts.headers).map_err(Self::error)?;
        let _input: EmptyInput = Self::decode(input, "World entry input is invalid")?;
        self.world
            .enter_world(user_id)
            .await
            .map(CharacterOutput::from)
            .map(Json)
            .map_err(ErrorOutput::from_world)
            .map_err(Self::error)
    }

    #[tool(
        input_schema = mcp_input_schema::<ListActivityInput>(),
        annotations(
            title = "List activity",
            read_only_hint = true,
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = false
        )
    )]
    async fn list_activity(
        &self,
        Extension(parts): Extension<Parts>,
        input: JsonObject,
    ) -> Result<Json<ActivityPageOutput>, CallToolResult> {
        let user_id = user_context(&parts.headers).map_err(Self::error)?;
        let input: ListActivityInput = Self::decode(input, "activity query is invalid")?;
        let input = input.parse().map_err(Self::error)?;
        self.world
            .list_activity(user_id, input)
            .await
            .map(ActivityPageOutput::from)
            .map(Json)
            .map_err(ErrorOutput::from_world)
            .map_err(Self::error)
    }

    #[tool(
        input_schema = mcp_input_schema::<CreateEntityInput>(),
        annotations(
            title = "Create entity",
            read_only_hint = false,
            destructive_hint = false,
            idempotent_hint = false,
            open_world_hint = false
        )
    )]
    async fn create_entity(
        &self,
        Extension(parts): Extension<Parts>,
        input: JsonObject,
    ) -> Result<Json<EntityOutput>, CallToolResult> {
        let user_id = user_context(&parts.headers).map_err(Self::error)?;
        let input: CreateEntityInput = Self::decode(input, "entity body is invalid")?;
        self.world
            .create_entity(user_id, input.into())
            .await
            .map(EntityOutput::from)
            .map(Json)
            .map_err(ErrorOutput::from_world)
            .map_err(Self::error)
    }

    #[tool(
        input_schema = mcp_input_schema::<ListEntityAtCurrentPlaceInput>(),
        annotations(
            title = "List entity at current place",
            read_only_hint = true,
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = false
        )
    )]
    async fn list_entity_at_current_place(
        &self,
        Extension(parts): Extension<Parts>,
        input: JsonObject,
    ) -> Result<Json<CurrentPlaceEntityPageOutput>, CallToolResult> {
        let user_id = user_context(&parts.headers).map_err(Self::error)?;
        let input: ListEntityAtCurrentPlaceInput =
            Self::decode(input, "current Place Entity query is invalid")?;
        let input = input.parse().map_err(Self::error)?;
        self.world
            .list_entity_at_current_place(user_id, input)
            .await
            .map(CurrentPlaceEntityPageOutput::from)
            .map(Json)
            .map_err(ErrorOutput::from_world)
            .map_err(Self::error)
    }

    #[tool(
        input_schema = mcp_input_schema::<ListActivityAtCurrentPlaceInput>(),
        annotations(
            title = "List activity at current place",
            read_only_hint = true,
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = false
        )
    )]
    async fn list_activity_at_current_place(
        &self,
        Extension(parts): Extension<Parts>,
        input: JsonObject,
    ) -> Result<Json<CurrentPlaceActivityPageOutput>, CallToolResult> {
        let user_id = user_context(&parts.headers).map_err(Self::error)?;
        let input: ListActivityAtCurrentPlaceInput =
            Self::decode(input, "current Place Activity query is invalid")?;
        let input = input.parse().map_err(Self::error)?;
        self.world
            .list_activity_at_current_place(user_id, input)
            .await
            .map(CurrentPlaceActivityPageOutput::from)
            .map(Json)
            .map_err(ErrorOutput::from_world)
            .map_err(Self::error)
    }

    #[tool(
        input_schema = mcp_input_schema::<GetEntityAtCurrentPlaceInput>(),
        annotations(
            title = "Get entity at current place",
            read_only_hint = true,
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = false
        )
    )]
    async fn get_entity_at_current_place(
        &self,
        Extension(parts): Extension<Parts>,
        input: JsonObject,
    ) -> Result<Json<CurrentPlaceEntityStatePageOutput>, CallToolResult> {
        let user_id = user_context(&parts.headers).map_err(Self::error)?;
        let input: GetEntityAtCurrentPlaceInput =
            Self::decode(input, "current Place Entity-state query is invalid")?;
        let input = input.parse().map_err(Self::error)?;
        self.world
            .get_entity_at_current_place(user_id, input)
            .await
            .map(CurrentPlaceEntityStatePageOutput::from)
            .map(Json)
            .map_err(ErrorOutput::from_world)
            .map_err(Self::error)
    }

    #[tool(
        input_schema = mcp_input_schema::<SubmitActionInput>(),
        annotations(
            title = "Submit action",
            read_only_hint = false,
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = false
        )
    )]
    async fn submit_action(
        &self,
        Extension(parts): Extension<Parts>,
        input: JsonObject,
    ) -> Result<Json<AcceptedActionOutput>, CallToolResult> {
        let user_id = user_context(&parts.headers).map_err(Self::error)?;
        let input: SubmitActionInput = Self::decode(input, "action body is invalid")?;
        let input = input.parse().map_err(Self::error)?;
        self.world
            .submit_action(user_id, input)
            .await
            .map(AcceptedActionOutput::from)
            .map(Json)
            .map_err(ErrorOutput::from_world)
            .map_err(Self::error)
    }

    #[tool(
        input_schema = mcp_input_schema::<SubmitInteractionInput>(),
        annotations(
            title = "Submit interaction",
            read_only_hint = false,
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = false
        )
    )]
    async fn submit_interaction(
        &self,
        Extension(parts): Extension<Parts>,
        input: JsonObject,
    ) -> Result<Json<AcceptedInteractionOutput>, CallToolResult> {
        let user_id = user_context(&parts.headers).map_err(Self::error)?;
        let input: SubmitInteractionInput = Self::decode(input, "Interaction body is invalid")?;
        let input = input.parse().map_err(Self::error)?;
        self.world
            .submit_interaction(user_id, input)
            .await
            .map(AcceptedInteractionOutput::from)
            .map(Json)
            .map_err(ErrorOutput::from_world)
            .map_err(Self::error)
    }
}

#[tool_handler(router = self.tool_router)]
impl ServerHandler for AicadiaMcp {
    async fn initialize(
        &self,
        _request: rmcp::model::InitializeRequestParams,
        _context: rmcp::service::RequestContext<rmcp::RoleServer>,
    ) -> Result<rmcp::model::InitializeResult, rmcp::ErrorData> {
        Err(rmcp::ErrorData::method_not_found::<
            rmcp::model::InitializeResultMethod,
        >())
    }

    async fn list_tools(
        &self,
        _request: Option<rmcp::model::PaginatedRequestParams>,
        _context: rmcp::service::RequestContext<rmcp::RoleServer>,
    ) -> Result<rmcp::model::ListToolsResult, rmcp::ErrorData> {
        let tools = [
            "get_world",
            "get_user",
            "get_character",
            "create_character",
            "create_entry_place",
            "enter_world",
            "list_activity",
            "create_entity",
            "list_entity_at_current_place",
            "list_activity_at_current_place",
            "get_entity_at_current_place",
            "submit_action",
            "submit_interaction",
        ]
        .into_iter()
        .map(|name| {
            self.tool_router
                .get(name)
                .cloned()
                .expect("the fixed player capability catalog must be registered")
        })
        .collect();
        Ok(rmcp::model::ListToolsResult {
            result_type: Some(rmcp::model::ResultType::COMPLETE),
            tools,
            meta: None,
            next_cursor: None,
            ttl_ms: Some(0),
            cache_scope: Some(rmcp::model::CacheScope::Public),
        })
    }

    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(ServerCapabilities::builder().enable_tools().build())
            .with_protocol_version(ProtocolVersion::V_2026_07_28)
            .with_server_info(Implementation::new("aicadia", MCP_VERSION))
            .with_instructions(agent_contract::INSTRUCTIONS)
    }

    fn supported_protocol_versions(&self) -> Cow<'static, [ProtocolVersion]> {
        Cow::Borrowed(&[ProtocolVersion::V_2026_07_28])
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use sqlx::PgPool;

    #[tokio::test]
    async fn non_loopback_address_is_rejected_before_serving() {
        let pool =
            PgPool::connect_lazy("postgresql:///unused").expect("lazy pool should not connect");
        let address = "192.0.2.1:3000".parse().expect("address should parse");

        assert!(matches!(
            app(World::new(pool), address),
            Err(ServerError::NonLoopback)
        ));
    }

    #[test]
    fn unavailable_is_an_http_service_unavailable_error() {
        let response =
            HttpError(ErrorOutput::from_world(crate::WorldError::Unavailable)).into_response();

        assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
    }
}
