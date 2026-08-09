use std::{borrow::Cow, net::SocketAddr, sync::Arc};

use axum::{
    Json as HttpJson, Router,
    extract::{Path, Query, State, rejection::JsonRejection, rejection::QueryRejection},
    http::{HeaderMap, StatusCode, request::Parts},
    response::{IntoResponse, Response},
    routing::{get, post},
};
use rmcp::{
    ServerHandler,
    handler::server::{
        router::tool::ToolRouter,
        tool::Extension,
        wrapper::{Json, Parameters},
    },
    model::{CallToolResult, Implementation, ProtocolVersion, ServerCapabilities, ServerInfo},
    tool, tool_handler, tool_router,
    transport::streamable_http_server::{
        StreamableHttpServerConfig, StreamableHttpService, session::local::LocalSessionManager,
    },
};
use schemars::JsonSchema;
use serde::Deserialize;
use thiserror::Error;
use utoipa::{OpenApi, openapi::OpenApi as OpenApiDocument};

use crate::{
    World,
    wire::{
        ActivityPageOutput, CharacterOutput, CreateCharacterInput, CreateEntityInput,
        CreateEntryPlaceInput, EntityOutput, EntityPageOutput, ErrorCode, ErrorDetail, ErrorOutput,
        GetEntityInput, ListActivityInput, ListEntityInput, PlaceOutput, USER_CONTEXT_HEADER,
        UserOutput, WorldOutput, parse_user_context,
    },
};

const MCP_VERSION: &str = env!("CARGO_PKG_VERSION");

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
        .with_legacy_session_mode(true)
        .with_json_response(true)
        .with_sse_keep_alive(None)
        .with_sse_retry(None)
        .with_allowed_origins(allowed_origin)
        .with_stateless_protocol_metadata_required(true);
    let mcp_world = world.clone();
    let mcp: StreamableHttpService<AicadiaMcp, LocalSessionManager> = StreamableHttpService::new(
        move || Ok(AicadiaMcp::new(mcp_world.clone())),
        Arc::new(LocalSessionManager::default()),
        mcp_config,
    );

    Ok(Router::new()
        .route("/api/world", get(get_world))
        .route("/api/user", get(get_user))
        .route("/api/character", get(get_character).post(create_character))
        .route("/api/place/entry", post(create_entry_place))
        .route("/api/world/entry", post(enter_world))
        .route("/api/activity", get(list_activity))
        .route("/api/entity", get(list_entity).post(create_entity))
        .route("/api/entity/{entity_id}", get(get_entity))
        .route("/api/openapi.json", get(openapi))
        .nest_service("/mcp", mcp)
        .with_state(world))
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
        list_entity,
        get_entity,
        create_entity
    ),
    components(schemas(
        WorldOutput,
        UserOutput,
        CharacterOutput,
        CreateCharacterInput,
        PlaceOutput,
        CreateEntryPlaceInput,
        ActivityPageOutput,
        EntityOutput,
        EntityPageOutput,
        CreateEntityInput,
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
    params(("Aicadia-User-Id" = Uuid, Header, description = "Untrusted development User context")),
    responses(
        (status = 200, description = "Current Character", body = CharacterOutput),
        (status = 400, description = "Invalid User context", body = ErrorOutput),
        (status = 404, description = "User or Character not found", body = ErrorOutput),
        (status = 503, description = "World unavailable", body = ErrorOutput)
    )
)]
async fn get_character(
    State(world): State<World>,
    headers: HeaderMap,
) -> Result<HttpJson<CharacterOutput>, HttpError> {
    let user_id = user_context(&headers)?;
    world
        .get_character(user_id)
        .await
        .map(CharacterOutput::from)
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
            | ErrorCode::EntryPlaceAlreadyExists => StatusCode::CONFLICT,
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
        Self {
            world,
            tool_router: Self::tool_router(),
        }
    }

    fn error(error: ErrorOutput) -> CallToolResult {
        let value = serde_json::to_string(&error)
            .expect("the fixed wire error contract is always JSON serializable");
        CallToolResult::error(vec![rmcp::model::ContentBlock::text(value)])
    }
}

#[tool_router]
impl AicadiaMcp {
    #[tool(
        description = "Get the identity of the one persistent shared Aicadia World. No User context is required.",
        annotations(
            title = "Get world",
            read_only_hint = true,
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = false
        )
    )]
    async fn get_world(&self, Parameters(_input): Parameters<EmptyInput>) -> Json<WorldOutput> {
        Json(self.world.get_world().into())
    }

    #[tool(
        description = "Get the durable User represented by this request's Aicadia-User-Id context. This tool does not accept a User id and does not authenticate the caller.",
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
        Parameters(_input): Parameters<EmptyInput>,
    ) -> Result<Json<UserOutput>, CallToolResult> {
        let user_id = user_context(&parts.headers).map_err(Self::error)?;
        self.world
            .get_user(user_id)
            .await
            .map(UserOutput::from)
            .map(Json)
            .map_err(ErrorOutput::from_world)
            .map_err(Self::error)
    }

    #[tool(
        description = "Get the Character role owned by the current User. The embedded Entity id is also the Character id; there is no separate Character id. current_place is the complete current Place or null when the Character exists but has not entered the World. This tool derives the User from Aicadia-User-Id and accepts no ids.",
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
        Parameters(_input): Parameters<EmptyInput>,
    ) -> Result<Json<CharacterOutput>, CallToolResult> {
        let user_id = user_context(&parts.headers).map_err(Self::error)?;
        self.world
            .get_character(user_id)
            .await
            .map(CharacterOutput::from)
            .map(Json)
            .map_err(ErrorOutput::from_world)
            .map_err(Self::error)
    }

    #[tool(
        description = "Create the one Character role owned by the current User and its shared Entity. The embedded Entity id is also the Character id. Creation deliberately returns current_place null and does not enter the World; call enter_world separately when entry is wanted. This tool derives the User from Aicadia-User-Id and accepts no ids.",
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
        Parameters(input): Parameters<CreateCharacterInput>,
    ) -> Result<Json<CharacterOutput>, CallToolResult> {
        let user_id = user_context(&parts.headers).map_err(Self::error)?;
        self.world
            .create_character(user_id, input.into())
            .await
            .map(CharacterOutput::from)
            .map(Json)
            .map_err(ErrorOutput::from_world)
            .map_err(Self::error)
    }

    #[tool(
        description = "Establish World genesis by creating the one shared entry Place. Call this only for an existing unplaced Character after enter_world returns entry_place_not_found. Supply only semantic name and description; World derives the Place Entity id. Exactly one concurrent request wins. If entry_place_already_exists is returned, another Agent established genesis: call enter_world again. This never creates later Places.",
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
        Parameters(input): Parameters<CreateEntryPlaceInput>,
    ) -> Result<Json<PlaceOutput>, CallToolResult> {
        let user_id = user_context(&parts.headers).map_err(Self::error)?;
        self.world
            .create_entry_place(user_id, input.into())
            .await
            .map(PlaceOutput::from)
            .map(Json)
            .map_err(ErrorOutput::from_world)
            .map_err(Self::error)
    }

    #[tool(
        description = "Enter the shared World with the current unplaced Character at the one server-derived entry Place. Call this directly after obtaining or creating a Character. If entry_place_not_found is returned, establish genesis with create_entry_place and retry. This accepts no ids or Place selector. Retrying successful entry returns the same complete Character and Place without another activity.",
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
        Parameters(_input): Parameters<EmptyInput>,
    ) -> Result<Json<CharacterOutput>, CallToolResult> {
        let user_id = user_context(&parts.headers).map_err(Self::error)?;
        self.world
            .enter_world(user_id)
            .await
            .map(CharacterOutput::from)
            .map(Json)
            .map_err(ErrorOutput::from_world)
            .map_err(Self::error)
    }

    #[tool(
        description = "List immutable accepted World actions involving the current Character from newest to oldest. World derives the Character from User context. actor_character identifies who acted, context_place preserves where acceptance occurred, and involved_entity identifies each subject or destination. Null actor or Place means that context did not yet exist. limit defaults to 25 and must be 1 through 100; copy next unchanged into cursor for the following page.",
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
        Parameters(input): Parameters<ListActivityInput>,
    ) -> Result<Json<ActivityPageOutput>, CallToolResult> {
        let user_id = user_context(&parts.headers).map_err(Self::error)?;
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
        description = "List shared Entities from newest to oldest. limit defaults to 25 and must be 1 through 100. Copy next into cursor to read the following page; do not interpret the cursor.",
        annotations(
            title = "List entity",
            read_only_hint = true,
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = false
        )
    )]
    async fn list_entity(
        &self,
        Parameters(input): Parameters<ListEntityInput>,
    ) -> Result<Json<EntityPageOutput>, CallToolResult> {
        let input = input.parse().map_err(Self::error)?;
        self.world
            .list_entity(input)
            .await
            .map(EntityPageOutput::from)
            .map(Json)
            .map_err(ErrorOutput::from_world)
            .map_err(Self::error)
    }

    #[tool(
        description = "Get one shared Entity by its stable Entity id.",
        annotations(
            title = "Get entity",
            read_only_hint = true,
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = false
        )
    )]
    async fn get_entity(
        &self,
        Parameters(input): Parameters<GetEntityInput>,
    ) -> Result<Json<EntityOutput>, CallToolResult> {
        let entity_id = input.parse().map_err(Self::error)?;
        self.world
            .get_entity(entity_id)
            .await
            .map(EntityOutput::from)
            .map(Json)
            .map_err(ErrorOutput::from_world)
            .map_err(Self::error)
    }

    #[tool(
        description = "Create one shared Entity for a stable referent introduced by the current User. Use this only when later participants must refer to the same subject. This does not assert fictional creation, ownership or discovery, and repeating it creates another Entity. Acceptance also appends create_entity Activity; its actor and context Place are present when the current Character exists and is placed.",
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
        Parameters(input): Parameters<CreateEntityInput>,
    ) -> Result<Json<EntityOutput>, CallToolResult> {
        let user_id = user_context(&parts.headers).map_err(Self::error)?;
        self.world
            .create_entity(user_id, input.into())
            .await
            .map(EntityOutput::from)
            .map(Json)
            .map_err(ErrorOutput::from_world)
            .map_err(Self::error)
    }
}

#[tool_handler(router = self.tool_router)]
impl ServerHandler for AicadiaMcp {
    async fn list_tools(
        &self,
        _request: Option<rmcp::model::PaginatedRequestParams>,
        context: rmcp::service::RequestContext<rmcp::RoleServer>,
    ) -> Result<rmcp::model::ListToolsResult, rmcp::ErrorData> {
        let tools = [
            "get_world",
            "get_user",
            "get_character",
            "create_character",
            "create_entry_place",
            "enter_world",
            "list_activity",
            "list_entity",
            "get_entity",
            "create_entity",
        ]
        .into_iter()
        .map(|name| {
            self.tool_router
                .get(name)
                .cloned()
                .expect("the fixed player capability catalog must be registered")
        })
        .collect();
        let supports_cache_hints = context
            .protocol_version()
            .is_some_and(|version| version >= ProtocolVersion::V_2026_07_28);

        Ok(rmcp::model::ListToolsResult {
            result_type: Some(rmcp::model::ResultType::COMPLETE),
            tools,
            meta: None,
            next_cursor: None,
            ttl_ms: supports_cache_hints.then_some(0),
            cache_scope: supports_cache_hints.then_some(rmcp::model::CacheScope::Public),
        })
    }

    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(ServerCapabilities::builder().enable_tools().build())
            .with_server_info(Implementation::new("aicadia", MCP_VERSION))
            .with_instructions(
                "Inspect and extend the shared Aicadia World through the ten listed tools. For entry: get_character; create_character only when absent; if current_place is null call enter_world; only after entry_place_not_found call create_entry_place and then retry enter_world. Never ask the User for Character or Place ids.",
            )
    }

    fn supported_protocol_versions(&self) -> Cow<'static, [ProtocolVersion]> {
        Cow::Borrowed(&[ProtocolVersion::V_2025_11_25, ProtocolVersion::V_2026_07_28])
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
