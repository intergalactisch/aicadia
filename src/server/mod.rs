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
        AcceptedActionOutput, AcceptedDiscoveryOutput, AcceptedInteractionOutput,
        ActivityPageOutput, CharacterEntityStatePageOutput, CharacterOutput, CreateCharacterInput,
        CreateEntityInput, CreateEntryPlaceInput, CurrentPlaceActivityPageOutput,
        CurrentPlaceEntityPageOutput, CurrentPlaceEntityStatePageOutput, EntityOutput,
        EntityPageOutput, ErrorCode, ErrorDetail, ErrorOutput, GetEntityAtCurrentPlaceInput,
        GetEntityCurrentStateInput, GetEntityInput, InvestigationResultOutput,
        ListActivityAtCurrentPlaceInput, ListActivityInput, ListEntityAtCurrentPlaceInput,
        ListEntityInput, PlaceOutput, StartInvestigationInput, SubmitActionInput,
        SubmitDiscoveryInput, SubmitInteractionInput, USER_CONTEXT_HEADER, UserOutput, WorldOutput,
        parse_user_context,
    },
};

mod error;
mod http;
mod mcp;

use error::{HttpError, user_context};
use mcp::AicadiaMcp;

const MCP_VERSION: &str = env!("CARGO_PKG_VERSION");
const LEDGER_HTML: &str = include_str!("../../web/index.html");
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

    Ok(http::routes().nest_service("/mcp", mcp).with_state(world))
}

#[cfg(test)]
mod investigation_test;

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
        let response = HttpError::from(ErrorOutput::from_world(crate::WorldError::Unavailable))
            .into_response();

        assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
    }
}
