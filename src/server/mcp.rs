use super::*;

use std::borrow::Cow;

use axum::http::request::Parts;
use rmcp::{
    ServerHandler,
    handler::server::{router::tool::ToolRouter, tool::Extension, wrapper::Json},
    model::{
        CallToolResult, Implementation, JsonObject, ProtocolVersion, ServerCapabilities, ServerInfo,
    },
    tool, tool_handler, tool_router,
};
use schemars::JsonSchema;
use serde::{Deserialize, de::DeserializeOwned};

use crate::agent_contract;

const MCP_VERSION: &str = env!("CARGO_PKG_VERSION");

fn mcp_input_schema<T: JsonSchema + 'static>() -> Arc<JsonObject> {
    rmcp::handler::server::common::schema_for_input::<T>()
        .unwrap_or_else(|error| panic!("invalid fixed MCP input schema: {error}"))
}

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(deny_unknown_fields)]
struct EmptyInput {}

#[derive(Clone)]
pub(super) struct AicadiaMcp {
    world: World,
    tool_router: ToolRouter<Self>,
}

impl AicadiaMcp {
    pub(super) fn new(world: World) -> Self {
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
        input_schema = mcp_input_schema::<StartInvestigationInput>(),
        annotations(
            title = "Start investigation",
            read_only_hint = false,
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = false
        )
    )]
    async fn start_investigation(
        &self,
        Extension(parts): Extension<Parts>,
        input: JsonObject,
    ) -> Result<Json<InvestigationResultOutput>, CallToolResult> {
        let user_id = user_context(&parts.headers).map_err(Self::error)?;
        let input: StartInvestigationInput = Self::decode(input, "Investigation body is invalid")?;
        self.world
            .start_investigation(user_id, input.into())
            .await
            .map(InvestigationResultOutput::from)
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

    #[tool(
        input_schema = mcp_input_schema::<SubmitDiscoveryInput>(),
        annotations(
            title = "Submit discovery",
            read_only_hint = false,
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = false
        )
    )]
    async fn submit_discovery(
        &self,
        Extension(parts): Extension<Parts>,
        input: JsonObject,
    ) -> Result<Json<AcceptedDiscoveryOutput>, CallToolResult> {
        let user_id = user_context(&parts.headers).map_err(Self::error)?;
        let input: SubmitDiscoveryInput = Self::decode(input, "Discovery body is invalid")?;
        self.world
            .submit_discovery(user_id, input.into())
            .await
            .map(AcceptedDiscoveryOutput::from)
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
            "start_investigation",
            "submit_action",
            "submit_interaction",
            "submit_discovery",
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
            .with_instructions(agent_contract::instructions())
    }

    fn supported_protocol_versions(&self) -> Cow<'static, [ProtocolVersion]> {
        Cow::Borrowed(&[ProtocolVersion::V_2026_07_28])
    }
}
