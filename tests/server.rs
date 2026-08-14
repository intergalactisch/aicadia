use std::collections::BTreeSet;

use aicadia::{World, server, wire::USER_CONTEXT_HEADER};
use reqwest::{Client, StatusCode};
use serde_json::{Value, json};
use sqlx::PgPool;
use tokio::{net::TcpListener, task::JoinHandle};
use uuid::Uuid;

const PROTOCOL_VERSION: &str = "2026-07-28";
const CAPABILITY: [&str; 13] = [
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
];

const MCP_INSTRUCTIONS: &str = include_str!("../src/agent-play-contract.txt");
const MCP_TOOL_CATALOG: &str = include_str!("agent-tool-catalog.json");

struct TestServer {
    base_url: String,
    origin: String,
    client: Client,
    task: JoinHandle<()>,
}

impl TestServer {
    async fn start(world: World) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0")
            .await
            .expect("loopback listener should bind");
        let address = listener
            .local_addr()
            .expect("loopback listener should have an address");
        let router = server::app(world, address).expect("loopback app should build");
        let task = tokio::spawn(async move {
            axum::serve(listener, router)
                .await
                .expect("test server should run");
        });
        let origin = format!("http://{address}");

        Self {
            base_url: origin.clone(),
            origin,
            client: Client::new(),
            task,
        }
    }

    async fn mcp(
        &self,
        method: &str,
        tool_name: Option<&str>,
        params: Value,
        user_id: Option<Uuid>,
        origin: Option<&str>,
    ) -> (StatusCode, Value) {
        let user_context = user_id.map(|user_id| user_id.to_string());
        self.mcp_with_user_context(method, tool_name, params, user_context.as_deref(), origin)
            .await
    }

    async fn mcp_with_user_context(
        &self,
        method: &str,
        tool_name: Option<&str>,
        params: Value,
        user_context: Option<&str>,
        origin: Option<&str>,
    ) -> (StatusCode, Value) {
        let user_context = user_context.into_iter().collect::<Vec<_>>();
        self.mcp_with_user_contexts(method, tool_name, params, &user_context, origin)
            .await
    }

    async fn mcp_with_user_contexts(
        &self,
        method: &str,
        tool_name: Option<&str>,
        mut params: Value,
        user_context: &[&str],
        origin: Option<&str>,
    ) -> (StatusCode, Value) {
        params
            .as_object_mut()
            .expect("MCP params should be an object")
            .insert("_meta".to_owned(), request_meta());
        let mut request = self
            .client
            .post(format!("{}/mcp", self.base_url))
            .header("Content-Type", "application/json")
            .header("Accept", "application/json, text/event-stream")
            .header("MCP-Protocol-Version", PROTOCOL_VERSION)
            .header("Mcp-Method", method)
            .json(&json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": method,
                "params": params,
            }));
        if let Some(tool_name) = tool_name {
            request = request.header("Mcp-Name", tool_name);
        }
        for user_context in user_context {
            request = request.header(USER_CONTEXT_HEADER, *user_context);
        }
        if let Some(origin) = origin {
            request = request.header("Origin", origin);
        }

        let response = request.send().await.expect("MCP request should send");
        Self::response(response).await
    }

    async fn mcp_raw(&self, body: Value, headers: &[(&str, &str)]) -> (StatusCode, Value) {
        let response = self.mcp_raw_response(body, headers).await;
        Self::response(response).await
    }

    async fn mcp_raw_response(&self, body: Value, headers: &[(&str, &str)]) -> reqwest::Response {
        let mut request = self
            .client
            .post(format!("{}/mcp", self.base_url))
            .header("Content-Type", "application/json")
            .header("Accept", "application/json, text/event-stream")
            .json(&body);
        for (name, value) in headers {
            request = request.header(*name, *value);
        }

        request.send().await.expect("raw MCP request should send")
    }

    async fn response(response: reqwest::Response) -> (StatusCode, Value) {
        let status = response.status();
        let bytes = response
            .bytes()
            .await
            .expect("MCP response body should be readable");
        let body = if bytes.is_empty() {
            Value::Null
        } else {
            serde_json::from_slice(&bytes)
                .unwrap_or_else(|_| Value::String(String::from_utf8_lossy(&bytes).into_owned()))
        };
        (status, body)
    }

    async fn tool(&self, name: &str, arguments: Value, user_id: Option<Uuid>) -> Value {
        let (status, body) = self
            .mcp(
                "tools/call",
                Some(name),
                json!({"name": name, "arguments": arguments}),
                user_id,
                None,
            )
            .await;
        assert_eq!(status, StatusCode::OK, "unexpected MCP response: {body}");
        body
    }
}

impl Drop for TestServer {
    fn drop(&mut self) {
        self.task.abort();
    }
}

fn request_meta() -> Value {
    json!({
        "io.modelcontextprotocol/protocolVersion": PROTOCOL_VERSION,
        "io.modelcontextprotocol/clientInfo": {
            "name": "aicadia-test",
            "version": "0.1.0"
        },
        "io.modelcontextprotocol/clientCapabilities": {}
    })
}

fn structured(body: &Value) -> &Value {
    &body["result"]["structuredContent"]
}

fn error_code(body: &Value) -> &str {
    body["error"]["code"]
        .as_str()
        .expect("canonical error code should be a string")
}

fn mcp_error(body: &Value) -> Value {
    let text = body["result"]["content"][0]["text"]
        .as_str()
        .expect("canonical MCP error should be a text block");
    serde_json::from_str(text).expect("canonical MCP error text should be JSON")
}

fn mcp_error_code(body: &Value) -> &str {
    body["error"]["code"]
        .as_str()
        .expect("canonical MCP error code should be a string")
}

fn text_property(prefix: &str, count: usize) -> Vec<Value> {
    (0..count)
        .map(|index| {
            json!({
                "key": format!("{prefix}_{index:03}"),
                "value": {
                    "type": "text",
                    "text": format!("{prefix} value {index}")
                }
            })
        })
        .collect()
}

async fn collect_http_current_property(server: &TestServer, user_id: Uuid) -> (Value, Vec<Value>) {
    let character: Value = server
        .client
        .get(format!("{}/api/character", server.base_url))
        .header(USER_CONTEXT_HEADER, user_id.to_string())
        .send()
        .await
        .expect("Character fetch should send")
        .json()
        .await
        .expect("Character fetch should be JSON");
    let orientation: Value = server
        .client
        .get(format!(
            "{}/api/place/current/entity?limit=100",
            server.base_url
        ))
        .header(USER_CONTEXT_HEADER, user_id.to_string())
        .send()
        .await
        .expect("orientation should send")
        .json()
        .await
        .expect("orientation should be JSON");
    let mut entity = vec![
        json!({
            "id": character["character"]["entity"]["id"],
            "name": character["character"]["entity"]["name"]
        }),
        json!({
            "id": orientation["place"]["id"],
            "name": orientation["place"]["name"]
        }),
    ];
    entity.extend(orientation["entity"].as_array().unwrap().iter().cloned());
    let revision = orientation["place_revision"].clone();
    let mut property = Vec::new();
    for selected in entity {
        let mut cursor = None::<String>;
        loop {
            let suffix = cursor
                .as_deref()
                .map(|cursor| format!("&cursor={cursor}"))
                .unwrap_or_default();
            let response = server
                .client
                .get(format!(
                    "{}/api/place/current/entity/{}?limit=100{suffix}",
                    server.base_url,
                    selected["id"].as_str().unwrap()
                ))
                .header(USER_CONTEXT_HEADER, user_id.to_string())
                .send()
                .await
                .unwrap();
            assert_eq!(response.status(), StatusCode::OK);
            let page: Value = response.json().await.unwrap();
            assert_eq!(page["place_revision"], revision);
            for association in page["current_state"]["association"].as_array().unwrap() {
                if association["type"] == "property" {
                    property.push(json!({
                        "entity": {"id": selected["id"], "name": selected["name"]},
                        "key": association["property"]["key"],
                        "value": association["property"]["value"]
                    }));
                }
            }
            cursor = page["current_state"]["next"].as_str().map(str::to_owned);
            if cursor.is_none() {
                break;
            }
        }
    }
    (revision, property)
}

async fn collect_mcp_current_property(server: &TestServer, user_id: Uuid) -> (Value, Vec<Value>) {
    let character = server.tool("get_character", json!({}), Some(user_id)).await;
    let orientation = server
        .tool(
            "list_entity_at_current_place",
            json!({"limit":100}),
            Some(user_id),
        )
        .await;
    let character = structured(&character);
    let orientation = structured(&orientation);
    let mut entity = vec![
        json!({"id": character["character"]["entity"]["id"], "name": character["character"]["entity"]["name"]}),
        json!({"id": orientation["place"]["id"], "name": orientation["place"]["name"]}),
    ];
    entity.extend(orientation["entity"].as_array().unwrap().iter().cloned());
    let revision = orientation["place_revision"].clone();
    let mut property = Vec::new();
    for selected in entity {
        let mut cursor = None::<String>;
        loop {
            let mut input = json!({"entity_id": selected["id"], "limit": 100});
            if let Some(cursor) = &cursor {
                input["cursor"] = json!(cursor);
            }
            let response = server
                .tool("get_entity_at_current_place", input, Some(user_id))
                .await;
            let page = structured(&response);
            assert_eq!(page["place_revision"], revision);
            for association in page["current_state"]["association"].as_array().unwrap() {
                if association["type"] == "property" {
                    property.push(json!({"entity":{"id":selected["id"],"name":selected["name"]},"key":association["property"]["key"],"value":association["property"]["value"]}));
                }
            }
            cursor = page["current_state"]["next"].as_str().map(str::to_owned);
            if cursor.is_none() {
                break;
            }
        }
    }
    (revision, property)
}

fn assert_protocol_error(
    status: StatusCode,
    body: &Value,
    expected_status: StatusCode,
    expected_code: i64,
) {
    assert_eq!(
        status, expected_status,
        "unexpected protocol response: {body}"
    );
    assert_eq!(body["error"]["code"], expected_code);
    assert!(
        body.get("result").is_none(),
        "protocol errors must not be capability results: {body}"
    );
    assert!(
        body["error"]["code"].is_number(),
        "protocol errors use numeric JSON-RPC codes: {body}"
    );
    assert!(
        body["error"].get("error").is_none(),
        "protocol errors must not contain the game error envelope: {body}"
    );
}

async fn assert_cross_operation_cursor_rejected(
    server: &TestServer,
    http_path: &str,
    tool_name: &str,
    cursor: &str,
    user_id: Option<Uuid>,
) {
    let mut request = server.client.get(format!(
        "{}{}?cursor={cursor}&limit=1",
        server.base_url, http_path
    ));
    if let Some(user_id) = user_id {
        request = request.header(USER_CONTEXT_HEADER, user_id.to_string());
    }
    let response = request
        .send()
        .await
        .expect("cross-operation HTTP cursor request should send");
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let http_error: Value = response
        .json()
        .await
        .expect("cross-operation HTTP cursor error should be JSON");
    let mcp_response = server
        .tool(tool_name, json!({"cursor": cursor, "limit": 1}), user_id)
        .await;
    assert_eq!(error_code(&http_error), "invalid_request");
    assert_eq!(http_error["error"]["field"], "cursor");
    assert_eq!(http_error["error"]["reason"], "malformed");
    assert_eq!(mcp_error(&mcp_response), http_error);
}

#[sqlx::test(migrations = "./migration")]
async fn ledger_root_serves_the_self_contained_get_only_page(pool: PgPool) {
    let server = TestServer::start(World::new(pool)).await;

    let response = server
        .client
        .get(format!("{}/", server.base_url))
        .send()
        .await
        .expect("ledger request should send");
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|value| value.to_str().ok()),
        Some("text/html; charset=utf-8")
    );
    let html = response.text().await.expect("ledger should be text");
    assert!(html.contains("<main id=\"main\">"));
    assert!(html.contains("Shared Entity"));
    assert!(html.contains("Personal Activity and prose"));
    assert!(html.contains("^#user_id="));
    assert!(html.contains("sessionStorage.setItem"));
    assert!(html.contains("history.replaceState"));
    assert!(html.contains("fetch(path, { method: \"GET\", headers })"));
    assert!(html.contains("/api/world"));
    assert!(html.contains("/api/entity?"));
    assert!(html.contains("/api/entity/${"));
    assert!(html.contains("/api/activity?"));
    assert_eq!(
        html.matches("<button ").count(),
        html.matches("type=\"button\"").count()
    );

    for forbidden in [
        "<form",
        "<input",
        "<textarea",
        "<select",
        "contenteditable",
        "onclick=",
        "onsubmit=",
        "method: \"POST\"",
        "method: \"PUT\"",
        "method: \"PATCH\"",
        "method: \"DELETE\"",
        "/api/user",
        "/api/character",
        "/api/place",
        "/api/action",
        "/mcp",
    ] {
        assert!(
            !html.contains(forbidden),
            "ledger must not contain forbidden mutation or extra-read surface: {forbidden}"
        );
    }

    let response = server
        .client
        .post(format!("{}/", server.base_url))
        .send()
        .await
        .expect("root POST boundary request should send");
    assert_eq!(response.status(), StatusCode::METHOD_NOT_ALLOWED);
}

#[sqlx::test(migrations = "./migration")]
async fn ledger_reads_remain_truthful_before_character_onboarding(pool: PgPool) {
    let world = World::new(pool);
    let user = world.create_user().await.expect("setup User should exist");
    let server = TestServer::start(world).await;

    let world_response = server
        .client
        .get(format!("{}/api/world", server.base_url))
        .send()
        .await
        .expect("World request should send");
    assert_eq!(world_response.status(), StatusCode::OK);

    let entity_response = server
        .client
        .get(format!("{}/api/entity?limit=100", server.base_url))
        .send()
        .await
        .expect("Entity request should send");
    assert_eq!(entity_response.status(), StatusCode::OK);
    let entity_page: Value = entity_response
        .json()
        .await
        .expect("Entity page should be JSON");
    assert_eq!(entity_page, json!({"entity": [], "next": null}));

    let activity_response = server
        .client
        .get(format!("{}/api/activity?limit=100", server.base_url))
        .header(USER_CONTEXT_HEADER, user.id.0.to_string())
        .send()
        .await
        .expect("pre-Character Activity request should send");
    assert_eq!(activity_response.status(), StatusCode::NOT_FOUND);
    let activity_error: Value = activity_response
        .json()
        .await
        .expect("pre-Character Activity error should be JSON");
    assert_eq!(error_code(&activity_error), "character_not_found");
}

#[sqlx::test(migrations = "./migration")]
async fn catalog_exposes_exactly_the_thirteen_player_capabilities(pool: PgPool) {
    let server = TestServer::start(World::new(pool)).await;

    let openapi: Value = server
        .client
        .get(format!("{}/api/openapi.json", server.base_url))
        .send()
        .await
        .expect("OpenAPI request should send")
        .json()
        .await
        .expect("OpenAPI should be JSON");
    let operation_id = openapi["paths"]
        .as_object()
        .expect("OpenAPI should have paths")
        .values()
        .flat_map(|path| {
            path.as_object()
                .expect("OpenAPI path should be an object")
                .values()
        })
        .map(|operation| {
            operation["operationId"]
                .as_str()
                .expect("operation should have an id")
        })
        .collect::<BTreeSet<_>>();
    assert_eq!(operation_id, CAPABILITY.into_iter().collect());
    for unavailable_player_operation in [
        "list_entity",
        "get_entity",
        "count_entity",
        "count_character",
        "count_user",
        "count_building",
    ] {
        assert!(
            !operation_id.contains(unavailable_player_operation),
            "player OpenAPI must not publish global probe {unavailable_player_operation}"
        );
    }
    assert!(
        openapi["paths"]["/api/entity"].get("get").is_none(),
        "loopback operator Entity list must be absent from the player OpenAPI catalog"
    );
    assert!(
        openapi["paths"].get("/api/entity/{entity_id}").is_none(),
        "loopback operator Entity lookup must be absent from the player OpenAPI catalog"
    );
    assert_eq!(
        openapi["paths"]["/api/entity"]["post"]["responses"]["201"]["description"],
        "Created Entity"
    );
    assert_eq!(
        openapi["paths"]["/api/action"]["post"]["responses"]["201"]["description"],
        "Accepted action"
    );
    assert_eq!(
        openapi["paths"]["/api/action"]["post"]["responses"]["412"]["description"],
        "Exact current Place changed after it was read"
    );
    assert_eq!(
        openapi["paths"]["/api/interaction"]["post"]["responses"]["201"]["description"],
        "Accepted Interaction"
    );
    assert_eq!(
        openapi["paths"]["/api/place/current/entity/{entity_id}"]["get"]["responses"]["200"]["description"],
        "One exact-local Entity with one bounded current-state page"
    );
    for schema in [
        "SubmitActionInput",
        "AcceptedActionOutput",
        "SubmitInteractionInput",
        "AcceptedInteractionOutput",
        "CurrentPlaceEntityPageOutput",
        "CurrentPlaceActivityPageOutput",
        "CharacterEntityStatePageOutput",
        "CurrentPlaceEntityStatePageOutput",
    ] {
        assert!(
            openapi["components"]["schemas"].get(schema).is_some(),
            "OpenAPI should publish shared schema {schema}"
        );
    }
    assert!(openapi.to_string().contains("ErrorDetail"));
    assert!(!openapi.to_string().contains("create_user"));

    let (status, discover) = server
        .mcp(
            "server/discover",
            None,
            json!({}),
            None,
            Some(&server.origin),
        )
        .await;
    assert_eq!(status, StatusCode::OK, "unexpected discover: {discover}");
    assert_eq!(
        discover["result"]["supportedVersions"],
        json!([PROTOCOL_VERSION])
    );
    assert_eq!(discover["result"]["capabilities"], json!({"tools": {}}));
    assert_eq!(discover["result"]["instructions"], MCP_INSTRUCTIONS);

    let (status, listed) = server
        .mcp("tools/list", None, json!({}), None, Some(&server.origin))
        .await;
    assert_eq!(status, StatusCode::OK, "unexpected tools/list: {listed}");
    assert_eq!(listed["result"]["cacheScope"], "public");
    assert_eq!(listed["result"]["ttlMs"], 0);
    let tools = listed["result"]["tools"]
        .as_array()
        .expect("tools/list should return an array");
    let expected_tools: Value =
        serde_json::from_str(MCP_TOOL_CATALOG).expect("tool fixture should be valid JSON");
    assert_eq!(
        listed["result"]["tools"], expected_tools,
        "the checked-in catalog must equal the runtime catalog after central Agent descriptions are applied"
    );
    assert_eq!(tools.len(), 13);
    let tool_name = tools
        .iter()
        .map(|tool| tool["name"].as_str().expect("tool name should be text"))
        .collect::<BTreeSet<_>>();
    assert_eq!(tool_name, CAPABILITY.into_iter().collect());
    for removed in ["list_entity", "get_entity"] {
        assert!(!tool_name.contains(removed));
    }
    let interaction = tools
        .iter()
        .find(|tool| tool["name"] == "submit_interaction")
        .expect("submit_interaction should be in the fixed catalog");
    assert_eq!(interaction["annotations"]["readOnlyHint"], false);
    assert_eq!(interaction["annotations"]["idempotentHint"], true);
    assert_eq!(interaction["annotations"]["openWorldHint"], false);
    let property = tools
        .iter()
        .find(|tool| tool["name"] == "get_entity_at_current_place")
        .expect("get_entity_at_current_place should be in the fixed catalog");
    assert_eq!(property["annotations"]["readOnlyHint"], true);
    assert_eq!(property["annotations"]["destructiveHint"], false);
    assert_eq!(property["annotations"]["idempotentHint"], true);
    assert_eq!(property["annotations"]["openWorldHint"], false);
    for required in [
        "Repository files, source, HTTP, databases, shell, browser, logs and remembered conversation are never live-state fallbacks",
        "Absolute numbers of Users, Characters, buildings or Entities remain honestly unknown",
        "A distant occurrence is unknown until a later accepted local carrier",
        "Use Activity-backed history for truthful recognition and callbacks; model memory or plausible prose is not evidence",
        "Recap selectively: prefer one relevant recent change, one grounded older callback when useful and one present affordance",
        "Three proposals are invitations, never an exhaustive menu",
        "Every explicit call stands alone; never continue play, trigger an Agent, notify a User or spend tokens in the background",
    ] {
        assert!(
            MCP_INSTRUCTIONS.contains(required),
            "global Agent contract must preserve: {required}"
        );
    }
    for forbidden_fragment in ["count_", "notify", "notification", "background_agent"] {
        assert!(
            tool_name
                .iter()
                .all(|name| !name.contains(forbidden_fragment)),
            "the player catalog must not expose {forbidden_fragment} capability"
        );
    }
    let (status, _) = server.mcp("tools/list", None, json!({}), None, None).await;
    assert_eq!(status, StatusCode::OK, "an absent Origin must be accepted");
    let foreign_origin = format!(
        "http://localhost:{}",
        server.origin.rsplit_once(':').unwrap().1
    );
    let (status, _) = server
        .mcp("tools/list", None, json!({}), None, Some(&foreign_origin))
        .await;
    assert_eq!(status, StatusCode::FORBIDDEN);
}

#[sqlx::test(migrations = "./migration")]
async fn current_mcp_remains_stateless_and_requires_per_request_metadata(pool: PgPool) {
    let server = TestServer::start(World::new(pool)).await;

    for (id, version) in [(1, PROTOCOL_VERSION), (2, "2025-11-25")] {
        let response = server
            .mcp_raw_response(
                json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "method": "initialize",
                    "params": {
                        "protocolVersion": version,
                        "capabilities": {},
                        "clientInfo": {
                            "name": "unsupported-initialize-test",
                            "version": "0.1.0"
                        }
                    }
                }),
                &[],
            )
            .await;
        assert!(
            response.headers().get("Mcp-Session-Id").is_none(),
            "initialize must not create a transport session"
        );
        let (status, body) = TestServer::response(response).await;
        assert_protocol_error(status, &body, StatusCode::OK, -32601);
    }

    let mut params = json!({});
    params
        .as_object_mut()
        .expect("MCP params should be an object")
        .insert("_meta".to_owned(), request_meta());

    let response = server
        .mcp_raw_response(
            json!({
                "jsonrpc": "2.0",
                "id": 3,
                "method": "tools/list",
                "params": params
            }),
            &[
                ("MCP-Protocol-Version", PROTOCOL_VERSION),
                ("Mcp-Method", "tools/list"),
            ],
        )
        .await;
    assert_eq!(response.status(), StatusCode::OK);
    assert!(
        response.headers().get("Mcp-Session-Id").is_none(),
        "MCP 2026 must not create a transport session"
    );
    assert_eq!(
        response
            .headers()
            .get("Content-Type")
            .and_then(|value| value.to_str().ok()),
        Some("application/json")
    );
    let listed: Value = response
        .json()
        .await
        .expect("stateless MCP response should be JSON");
    assert!(listed["result"]["tools"].is_array());

    let (status, missing_meta) = server
        .mcp_raw(
            json!({
                "jsonrpc": "2.0",
                "id": 4,
                "method": "tools/list",
                "params": {}
            }),
            &[
                ("MCP-Protocol-Version", PROTOCOL_VERSION),
                ("Mcp-Method", "tools/list"),
            ],
        )
        .await;
    assert_protocol_error(status, &missing_meta, StatusCode::BAD_REQUEST, -32602);

    let unsupported_version = "2025-11-25";
    let (status, unsupported) = server
        .mcp_raw(
            json!({
                "jsonrpc": "2.0",
                "id": 5,
                "method": "tools/list",
                "params": {
                    "_meta": {
                        "io.modelcontextprotocol/protocolVersion": unsupported_version,
                        "io.modelcontextprotocol/clientInfo": {
                            "name": "unsupported-version-test",
                            "version": "0.1.0"
                        },
                        "io.modelcontextprotocol/clientCapabilities": {}
                    }
                }
            }),
            &[
                ("MCP-Protocol-Version", unsupported_version),
                ("Mcp-Method", "tools/list"),
            ],
        )
        .await;
    assert_protocol_error(status, &unsupported, StatusCode::BAD_REQUEST, -32022);
}

#[sqlx::test(migrations = "./migration")]
async fn mcp_arguments_fail_closed_with_canonical_invalid_request_for_all_capabilities(
    pool: PgPool,
) {
    let world = World::new(pool);
    let user = world.create_user().await.expect("setup User should exist");
    let server = TestServer::start(world).await;

    for (name, contextual) in [
        ("get_world", false),
        ("get_user", true),
        ("get_character", true),
        ("create_character", true),
        ("create_entry_place", true),
        ("enter_world", true),
        ("list_activity", true),
        ("create_entity", true),
        ("list_entity_at_current_place", true),
        ("list_activity_at_current_place", true),
        ("get_entity_at_current_place", true),
        ("submit_action", true),
        ("submit_interaction", true),
    ] {
        let response = server
            .tool(
                name,
                json!({"unexpected": true}),
                contextual.then_some(user.id.0),
            )
            .await;
        assert_eq!(
            response["result"]["isError"], true,
            "{name} should return a game error"
        );
        let error = mcp_error(&response);
        assert_eq!(
            error_code(&error),
            "invalid_request",
            "{name} should canonically reject malformed capability arguments"
        );
        assert!(
            response.get("error").is_none(),
            "{name} argument decoding must not escape as a JSON-RPC protocol error"
        );
    }

    let unknown_entity_body = json!({
        "name": "Must not exist",
        "description": "Unknown fields reject this body.",
        "unexpected": true
    });
    let http_response = server
        .client
        .post(format!("{}/api/entity", server.base_url))
        .header(USER_CONTEXT_HEADER, user.id.0.to_string())
        .json(&unknown_entity_body)
        .send()
        .await
        .expect("unknown HTTP Entity field should send");
    assert_eq!(http_response.status(), StatusCode::BAD_REQUEST);
    let http_error: Value = http_response
        .json()
        .await
        .expect("unknown HTTP Entity field should be JSON");
    let mcp_response = server
        .tool("create_entity", unknown_entity_body, Some(user.id.0))
        .await;
    assert_eq!(mcp_error(&mcp_response), http_error);

    let http_response = server
        .client
        .get(format!("{}/api/entity?unexpected=true", server.base_url))
        .send()
        .await
        .expect("unknown HTTP Entity query should send");
    assert_eq!(http_response.status(), StatusCode::BAD_REQUEST);
    let http_error: Value = http_response
        .json()
        .await
        .expect("unknown HTTP Entity query should be JSON");
    assert_eq!(error_code(&http_error), "invalid_request");

    let http_response = server
        .client
        .get(format!(
            "{}/api/place/current/entity/{}?unexpected=true",
            server.base_url,
            Uuid::new_v4()
        ))
        .header(USER_CONTEXT_HEADER, user.id.0.to_string())
        .send()
        .await
        .expect("unknown HTTP Property query field should send");
    assert_eq!(http_response.status(), StatusCode::BAD_REQUEST);
    let http_error: Value = http_response
        .json()
        .await
        .expect("unknown HTTP Property query field should be JSON");
    let mcp_response = server
        .tool(
            "get_entity_at_current_place",
            json!({"unexpected": true}),
            Some(user.id.0),
        )
        .await;
    assert_eq!(error_code(&http_error), "invalid_request");
    assert_eq!(mcp_error(&mcp_response), http_error);

    let malformed_action = json!({
        "request_id": Uuid::new_v4(),
        "expected_place_revision": "not-a-revision",
        "prose": "This action must not be decoded.",
        "consequence": {
            "type": "introduce_entity",
            "name": "Must not exist",
            "description": "An unknown nested field rejects the body.",
            "unexpected": true
        }
    });
    let http_response = server
        .client
        .post(format!("{}/api/action", server.base_url))
        .header(USER_CONTEXT_HEADER, user.id.0.to_string())
        .json(&malformed_action)
        .send()
        .await
        .expect("unknown nested HTTP action field should send");
    assert_eq!(http_response.status(), StatusCode::BAD_REQUEST);
    let http_error: Value = http_response
        .json()
        .await
        .expect("unknown nested HTTP action field should be JSON");
    let mcp_response = server
        .tool("submit_action", malformed_action, Some(user.id.0))
        .await;
    assert_eq!(mcp_error(&mcp_response), http_error);

    let http_response = server
        .client
        .get(format!("{}/api/entity/not-a-uuid", server.base_url))
        .send()
        .await
        .expect("malformed HTTP Entity id should send");
    assert_eq!(http_response.status(), StatusCode::BAD_REQUEST);
    let http_error: Value = http_response
        .json()
        .await
        .expect("malformed HTTP Entity id should be JSON");
    assert_eq!(http_error["error"]["field"], "entity_id");
    assert_eq!(http_error["error"]["reason"], "invalid_uuid");
}

#[sqlx::test(migrations = "./migration")]
async fn invalid_mcp_framing_stays_outside_the_game_error_contract(pool: PgPool) {
    let server = TestServer::start(World::new(pool)).await;
    let call = |name: &str, with_meta: bool| {
        let mut params = json!({"name": name, "arguments": {}});
        if with_meta {
            params
                .as_object_mut()
                .expect("tool params should be an object")
                .insert("_meta".to_owned(), request_meta());
        }
        json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "tools/call",
            "params": params
        })
    };

    let (status, body) = server
        .mcp_raw(
            call("not_a_tool", true),
            &[
                ("MCP-Protocol-Version", PROTOCOL_VERSION),
                ("Mcp-Method", "tools/call"),
                ("Mcp-Name", "not_a_tool"),
            ],
        )
        .await;
    assert_protocol_error(status, &body, StatusCode::BAD_REQUEST, -32602);

    for unavailable_player_tool in [
        "list_entity",
        "get_entity",
        "count_entity",
        "count_character",
        "count_user",
        "count_building",
    ] {
        let (status, body) = server
            .mcp_raw(
                call(unavailable_player_tool, true),
                &[
                    ("MCP-Protocol-Version", PROTOCOL_VERSION),
                    ("Mcp-Method", "tools/call"),
                    ("Mcp-Name", unavailable_player_tool),
                ],
            )
            .await;
        assert_protocol_error(status, &body, StatusCode::BAD_REQUEST, -32602);
    }

    for (headers, body) in [
        (
            vec![
                ("MCP-Protocol-Version", PROTOCOL_VERSION),
                ("Mcp-Name", "get_world"),
            ],
            call("get_world", true),
        ),
        (
            vec![
                ("MCP-Protocol-Version", PROTOCOL_VERSION),
                ("Mcp-Method", "tools/list"),
                ("Mcp-Name", "get_world"),
            ],
            call("get_world", true),
        ),
        (
            vec![
                ("MCP-Protocol-Version", PROTOCOL_VERSION),
                ("Mcp-Method", "tools/call"),
            ],
            call("get_world", true),
        ),
        (
            vec![
                ("MCP-Protocol-Version", PROTOCOL_VERSION),
                ("Mcp-Method", "tools/call"),
                ("Mcp-Name", "get_user"),
            ],
            call("get_world", true),
        ),
    ] {
        let (status, body) = server.mcp_raw(body, &headers).await;
        assert_protocol_error(status, &body, StatusCode::BAD_REQUEST, -32020);
    }

    let (status, body) = server
        .mcp_raw(
            call("get_world", false),
            &[
                ("MCP-Protocol-Version", PROTOCOL_VERSION),
                ("Mcp-Method", "tools/call"),
                ("Mcp-Name", "get_world"),
            ],
        )
        .await;
    assert_protocol_error(status, &body, StatusCode::BAD_REQUEST, -32602);

    let (status, body) = server
        .mcp_raw(
            call("get_world", true),
            &[("Mcp-Method", "tools/call"), ("Mcp-Name", "get_world")],
        )
        .await;
    assert_protocol_error(status, &body, StatusCode::BAD_REQUEST, -32020);
}

#[sqlx::test(migrations = "./migration")]
async fn http_and_mcp_share_successful_world_state(pool: PgPool) {
    let world = World::new(pool);
    let user = world.create_user().await.expect("setup User should exist");
    let second_user = world.create_user().await.expect("second User should exist");
    let server = TestServer::start(world).await;

    let http_world: Value = server
        .client
        .get(format!("{}/api/world", server.base_url))
        .send()
        .await
        .expect("World request should send")
        .json()
        .await
        .expect("World should be JSON");
    let mcp_world = server.tool("get_world", json!({}), None).await;
    assert_eq!(http_world, *structured(&mcp_world));

    let http_user: Value = server
        .client
        .get(format!("{}/api/user", server.base_url))
        .header(USER_CONTEXT_HEADER, user.id.0.to_string())
        .send()
        .await
        .expect("User request should send")
        .json()
        .await
        .expect("User should be JSON");
    let mcp_user = server.tool("get_user", json!({}), Some(user.id.0)).await;
    assert_eq!(http_user, *structured(&mcp_user));

    let response = server
        .client
        .post(format!("{}/api/character", server.base_url))
        .header(USER_CONTEXT_HEADER, user.id.0.to_string())
        .json(&json!({
            "name": "Mara Venn",
            "description": "A careful surveyor at the edge of the known World."
        }))
        .send()
        .await
        .expect("Character create should send");
    assert_eq!(response.status(), StatusCode::CREATED);
    let http_character: Value = response.json().await.expect("Character should be JSON");
    assert_eq!(http_character["owner_user_id"], user.id.0.to_string());
    let mcp_character = server
        .tool("get_character", json!({}), Some(user.id.0))
        .await;
    assert_eq!(http_character, structured(&mcp_character)["character"]);
    let character_entity: Value = server
        .client
        .get(format!(
            "{}/api/entity/{}",
            server.base_url,
            http_character["entity"]["id"]
                .as_str()
                .expect("Character Entity id should be text")
        ))
        .send()
        .await
        .expect("operator Entity lookup should send")
        .json()
        .await
        .expect("operator Entity lookup should be JSON");
    assert_eq!(character_entity, http_character["entity"]);

    let mcp_second_character = server
        .tool(
            "create_character",
            json!({
                "name": "Tomas Reed",
                "description": "A patient observer of changes in the shared World."
            }),
            Some(second_user.id.0),
        )
        .await;
    let http_second_character: Value = server
        .client
        .get(format!("{}/api/character", server.base_url))
        .header(USER_CONTEXT_HEADER, second_user.id.0.to_string())
        .send()
        .await
        .expect("Character read should send")
        .json()
        .await
        .expect("Character should be JSON");
    assert_eq!(
        http_second_character["character"],
        *structured(&mcp_second_character)
    );

    let response = server
        .client
        .post(format!("{}/api/place/entry", server.base_url))
        .header(USER_CONTEXT_HEADER, user.id.0.to_string())
        .json(&json!({
            "name": "North Gate",
            "description": "The one established entry into the shared World."
        }))
        .send()
        .await
        .expect("entry Place create should send");
    assert_eq!(response.status(), StatusCode::CREATED);
    let entry_place: Value = response.json().await.expect("entry Place should be JSON");
    assert_eq!(entry_place["is_entry"], true);
    let entered = server.tool("enter_world", json!({}), Some(user.id.0)).await;
    assert_eq!(
        structured(&entered)["current_place"],
        entry_place,
        "MCP entry should use the HTTP-created server-derived Place"
    );
    let response = server
        .client
        .post(format!("{}/api/world/entry", server.base_url))
        .header(USER_CONTEXT_HEADER, second_user.id.0.to_string())
        .send()
        .await
        .expect("second Character entry should send");
    assert_eq!(response.status(), StatusCode::OK);
    let http_entered: Value = response
        .json()
        .await
        .expect("entered Character should be JSON");
    assert_eq!(http_entered["current_place"], entry_place);

    let first_activity: Value = server
        .client
        .get(format!("{}/api/activity?limit=1", server.base_url))
        .header(USER_CONTEXT_HEADER, user.id.0.to_string())
        .send()
        .await
        .expect("activity page should send")
        .json()
        .await
        .expect("activity page should be JSON");
    assert_eq!(first_activity["activity"][0]["operation"], "enter_world");
    let activity_cursor = first_activity["next"]
        .as_str()
        .expect("earlier personal activity should produce a cursor");
    let next_activity = server
        .tool(
            "list_activity",
            json!({"cursor": activity_cursor, "limit": 1}),
            Some(user.id.0),
        )
        .await;
    assert_eq!(
        structured(&next_activity)["activity"][0]["operation"],
        "create_entry_place"
    );

    let response = server
        .client
        .post(format!("{}/api/entity", server.base_url))
        .header(USER_CONTEXT_HEADER, user.id.0.to_string())
        .json(&json!({
            "name": "Old Willow",
            "description": "A mature willow beside Glassmere Lake."
        }))
        .send()
        .await
        .expect("Entity create should send");
    assert_eq!(response.status(), StatusCode::CREATED);
    let http_created: Value = response.json().await.expect("Entity should be JSON");
    let operator_read: Value = server
        .client
        .get(format!(
            "{}/api/entity/{}",
            server.base_url,
            http_created["id"]
                .as_str()
                .expect("Entity id should be text")
        ))
        .send()
        .await
        .expect("operator Entity lookup should send")
        .json()
        .await
        .expect("operator Entity lookup should be JSON");
    assert_eq!(http_created, operator_read);

    let mcp_created = server
        .tool(
            "create_entity",
            json!({
                "name": "Glassmere Lake",
                "description": "A lake shared by every participant."
            }),
            Some(user.id.0),
        )
        .await;
    let mcp_created = structured(&mcp_created);
    let http_list: Value = server
        .client
        .get(format!("{}/api/entity", server.base_url))
        .send()
        .await
        .expect("Entity list should send")
        .json()
        .await
        .expect("Entity list should be JSON");
    assert!(
        http_list["entity"]
            .as_array()
            .expect("Entity page should contain an array")
            .iter()
            .any(|entity| entity["id"] == mcp_created["id"])
    );

    let first_page: Value = server
        .client
        .get(format!("{}/api/entity?limit=1", server.base_url))
        .send()
        .await
        .expect("first page should send")
        .json()
        .await
        .expect("first page should be JSON");
    let cursor = first_page["next"]
        .as_str()
        .expect("two Entities should produce a cursor");
    let second_page: Value = server
        .client
        .get(format!(
            "{}/api/entity?cursor={cursor}&limit=1",
            server.base_url
        ))
        .send()
        .await
        .expect("second operator Entity page should send")
        .json()
        .await
        .expect("second operator Entity page should be JSON");
    assert_ne!(
        first_page["entity"][0]["id"],
        second_page["entity"][0]["id"]
    );
}

#[sqlx::test(migrations = "./migration")]
async fn action_http_and_mcp_share_commit_retry_visibility_and_errors(pool: PgPool) {
    let world = World::new(pool.clone());
    let actor = world.create_user().await.expect("actor User should exist");
    let observer = world
        .create_user()
        .await
        .expect("observer User should exist");
    let server = TestServer::start(world).await;

    server
        .tool(
            "create_character",
            json!({"name": "Mara Venn", "description": "A careful surveyor."}),
            Some(actor.id.0),
        )
        .await;
    server
        .tool(
            "create_character",
            json!({"name": "Tomas Reed", "description": "A patient observer."}),
            Some(observer.id.0),
        )
        .await;

    let unplaced_response = server
        .client
        .get(format!("{}/api/place/current/entity", server.base_url))
        .header(USER_CONTEXT_HEADER, observer.id.0.to_string())
        .send()
        .await
        .expect("unplaced exact-Place read should send");
    assert_eq!(unplaced_response.status(), StatusCode::CONFLICT);
    let unplaced_http: Value = unplaced_response
        .json()
        .await
        .expect("unplaced error should be JSON");
    let unplaced_mcp = server
        .tool(
            "list_entity_at_current_place",
            json!({}),
            Some(observer.id.0),
        )
        .await;
    assert_eq!(error_code(&unplaced_http), "character_not_entered");
    assert_eq!(mcp_error(&unplaced_mcp), unplaced_http);

    let created_place = server
        .tool(
            "create_entry_place",
            json!({
                "name": "North Gate",
                "description": "The one established entry into the shared World."
            }),
            Some(actor.id.0),
        )
        .await;
    let created_place = structured(&created_place).clone();
    let scoped_place = json!({
        "id": created_place["entity"]["id"],
        "name": created_place["entity"]["name"],
        "description": created_place["entity"]["description"]
    });
    for user_id in [actor.id.0, observer.id.0] {
        let response = server
            .client
            .post(format!("{}/api/world/entry", server.base_url))
            .header(USER_CONTEXT_HEADER, user_id.to_string())
            .send()
            .await
            .expect("World entry should send");
        assert_eq!(response.status(), StatusCode::OK);
        let entered: Value = response.json().await.expect("entry should be JSON");
        assert_eq!(entered["current_place"], created_place);
    }

    let entity_context: Value = server
        .client
        .get(format!("{}/api/place/current/entity", server.base_url))
        .header(USER_CONTEXT_HEADER, actor.id.0.to_string())
        .send()
        .await
        .expect("exact-Place Entity read should send")
        .json()
        .await
        .expect("exact-Place Entity page should be JSON");
    let activity_context = server
        .tool(
            "list_activity_at_current_place",
            json!({}),
            Some(actor.id.0),
        )
        .await;
    let activity_context = structured(&activity_context);
    assert_eq!(
        entity_context["place_revision"], activity_context["place_revision"],
        "independent exact-Place reads should expose one shared revision"
    );
    assert_eq!(entity_context["place"], scoped_place);
    assert_eq!(activity_context["place"], scoped_place);
    assert_eq!(entity_context["entity"].as_array().map(Vec::len), Some(1));
    assert_eq!(entity_context["entity"][0]["name"], "Tomas Reed");
    assert_eq!(
        entity_context["entity"][0]
            .as_object()
            .expect("current-Place Entity should be an object")
            .keys()
            .cloned()
            .collect::<BTreeSet<_>>(),
        BTreeSet::from(["description".to_owned(), "id".to_owned(), "name".to_owned()])
    );

    let bad_limit_response = server
        .client
        .get(format!(
            "{}/api/place/current/entity?limit=0",
            server.base_url
        ))
        .header(USER_CONTEXT_HEADER, actor.id.0.to_string())
        .send()
        .await
        .expect("invalid exact-Place Entity limit should send");
    assert_eq!(bad_limit_response.status(), StatusCode::BAD_REQUEST);
    let bad_limit_http: Value = bad_limit_response
        .json()
        .await
        .expect("invalid Entity limit should be JSON");
    let bad_limit_mcp = server
        .tool(
            "list_entity_at_current_place",
            json!({"limit": 0}),
            Some(actor.id.0),
        )
        .await;
    assert_eq!(error_code(&bad_limit_http), "invalid_entity_limit");
    assert_eq!(mcp_error(&bad_limit_mcp), bad_limit_http);

    let bad_cursor_response = server
        .client
        .get(format!(
            "{}/api/place/current/activity?cursor=not-a-cursor",
            server.base_url
        ))
        .header(USER_CONTEXT_HEADER, actor.id.0.to_string())
        .send()
        .await
        .expect("invalid exact-Place Activity cursor should send");
    assert_eq!(bad_cursor_response.status(), StatusCode::BAD_REQUEST);
    let bad_cursor_http: Value = bad_cursor_response
        .json()
        .await
        .expect("invalid Activity cursor should be JSON");
    let bad_cursor_mcp = server
        .tool(
            "list_activity_at_current_place",
            json!({"cursor": "not-a-cursor"}),
            Some(actor.id.0),
        )
        .await;
    assert_eq!(error_code(&bad_cursor_http), "invalid_request");
    assert_eq!(mcp_error(&bad_cursor_mcp), bad_cursor_http);

    let invalid_action = json!({
        "request_id": Uuid::new_v4(),
        "expected_place_revision": entity_context["place_revision"],
        "prose": "   ",
        "consequence": {
            "type": "introduce_entity",
            "name": "Rejected Marker",
            "description": "This marker must not exist."
        }
    });
    let invalid_response = server
        .client
        .post(format!("{}/api/action", server.base_url))
        .header(USER_CONTEXT_HEADER, actor.id.0.to_string())
        .json(&invalid_action)
        .send()
        .await
        .expect("invalid action should send");
    assert_eq!(invalid_response.status(), StatusCode::BAD_REQUEST);
    let invalid_http: Value = invalid_response
        .json()
        .await
        .expect("invalid action error should be JSON");
    let mut invalid_mcp_input = invalid_action;
    invalid_mcp_input["request_id"] = json!(Uuid::new_v4());
    let invalid_mcp = server
        .tool("submit_action", invalid_mcp_input, Some(actor.id.0))
        .await;
    assert_eq!(
        invalid_http,
        json!({
            "error": {
                "code": "invalid_action",
                "message": "Action prose is empty.",
                "field": "prose",
                "reason": "empty"
            }
        })
    );
    assert_eq!(mcp_error(&invalid_mcp), invalid_http);

    let malformed_action = json!({
        "request_id": Uuid::new_v4(),
        "expected_place_revision": "not-a-revision",
        "prose": "This package must not be accepted.",
        "consequence": {
            "type": "introduce_entity",
            "name": "Rejected Marker",
            "description": "This marker must not exist."
        }
    });
    let malformed_response = server
        .client
        .post(format!("{}/api/action", server.base_url))
        .header(USER_CONTEXT_HEADER, actor.id.0.to_string())
        .json(&malformed_action)
        .send()
        .await
        .expect("malformed action should send");
    assert_eq!(malformed_response.status(), StatusCode::BAD_REQUEST);
    let malformed_http: Value = malformed_response
        .json()
        .await
        .expect("malformed error should be JSON");
    let malformed_mcp = server
        .tool("submit_action", malformed_action, Some(actor.id.0))
        .await;
    assert_eq!(error_code(&malformed_http), "invalid_request");
    assert_eq!(malformed_http["error"]["field"], "expected_place_revision");
    assert_eq!(mcp_error(&malformed_mcp), malformed_http);

    let unsupported_consequence = json!({
        "request_id": Uuid::new_v4(),
        "expected_place_revision": entity_context["place_revision"],
        "prose": "This unsupported consequence must not change the World.",
        "consequence": {
            "type": "move_character",
            "name": "Impossible Passage",
            "description": "This consequence is outside the current action surface."
        }
    });
    let unsupported_response = server
        .client
        .post(format!("{}/api/action", server.base_url))
        .header(USER_CONTEXT_HEADER, actor.id.0.to_string())
        .json(&unsupported_consequence)
        .send()
        .await
        .expect("unsupported HTTP consequence should send");
    assert_eq!(unsupported_response.status(), StatusCode::BAD_REQUEST);
    let unsupported_http: Value = unsupported_response
        .json()
        .await
        .expect("unsupported consequence error should be JSON");
    let unsupported_mcp = server
        .tool("submit_action", unsupported_consequence, Some(actor.id.0))
        .await;
    assert_eq!(error_code(&unsupported_http), "invalid_request");
    assert_eq!(mcp_error(&unsupported_mcp), unsupported_http);
    let unsupported_writes: (i64, i64) = sqlx::query_as(
        r#"
        SELECT
            (SELECT count(*) FROM entity WHERE name = 'Impossible Passage'),
            (SELECT count(*) FROM activity WHERE operation = 'submit_action')
        "#,
    )
    .fetch_one(&pool)
    .await
    .expect("unsupported consequence write counts should load");
    assert_eq!(unsupported_writes, (0, 0));

    let request_id = Uuid::new_v4();
    let action = json!({
        "request_id": request_id,
        "expected_place_revision": entity_context["place_revision"],
        "prose": "Mara braces a carved cedar marker beside the crossing.",
        "consequence": {
            "type": "introduce_entity",
            "name": "Cedar Crossing Marker",
            "description": "A waist-high cedar marker carved with three crossing lines."
        }
    });
    let accepted_response = server
        .client
        .post(format!("{}/api/action", server.base_url))
        .header(USER_CONTEXT_HEADER, actor.id.0.to_string())
        .json(&action)
        .send()
        .await
        .expect("action should send");
    assert_eq!(accepted_response.status(), StatusCode::CREATED);
    let accepted: Value = accepted_response
        .json()
        .await
        .expect("accepted action should be JSON");
    assert_eq!(accepted["activity"]["operation"], "submit_action");
    assert_eq!(
        accepted["activity"]["prose"],
        "Mara braces a carved cedar marker beside the crossing."
    );
    assert_eq!(accepted["consequence"]["type"], "introduce_entity");
    assert_eq!(
        accepted["consequence"]["entity"]["name"],
        "Cedar Crossing Marker"
    );
    assert_eq!(accepted["place"], created_place);
    let role = accepted["activity"]["involved_entity"]
        .as_array()
        .expect("Activity roles should be an array")
        .iter()
        .map(|reference| reference["role"].as_str().expect("role should be a string"))
        .collect::<BTreeSet<_>>();
    assert_eq!(role, BTreeSet::from(["location", "subject"]));

    let http_retry = server
        .client
        .post(format!("{}/api/action", server.base_url))
        .header(USER_CONTEXT_HEADER, actor.id.0.to_string())
        .json(&action)
        .send()
        .await
        .expect("HTTP delivery retry should send");
    assert_eq!(http_retry.status(), StatusCode::CREATED);
    assert_eq!(
        http_retry
            .json::<Value>()
            .await
            .expect("HTTP delivery retry should be JSON"),
        accepted
    );
    let retry = server
        .tool("submit_action", action.clone(), Some(actor.id.0))
        .await;
    assert_eq!(structured(&retry), &accepted);

    let observer_entities = server
        .tool(
            "list_entity_at_current_place",
            json!({}),
            Some(observer.id.0),
        )
        .await;
    let observer_entities = structured(&observer_entities)["entity"]
        .as_array()
        .expect("current-Place Entities should be an array");
    assert!(observer_entities.iter().any(|entity| {
        entity["id"] == accepted["consequence"]["entity"]["id"]
            && entity["name"] == "Cedar Crossing Marker"
            && entity["description"]
                == "A waist-high cedar marker carved with three crossing lines."
    }));
    for entity in observer_entities {
        assert_eq!(
            entity
                .as_object()
                .expect("current-Place Entity should be an object")
                .keys()
                .cloned()
                .collect::<BTreeSet<_>>(),
            BTreeSet::from(["description".to_owned(), "id".to_owned(), "name".to_owned()])
        );
    }
    let observer_activity: Value = server
        .client
        .get(format!(
            "{}/api/place/current/activity?limit=1",
            server.base_url
        ))
        .header(USER_CONTEXT_HEADER, observer.id.0.to_string())
        .send()
        .await
        .expect("observer Activity read should send")
        .json()
        .await
        .expect("observer Activity page should be JSON");
    assert_eq!(observer_activity["activity"][0], accepted["activity"]);
    assert_ne!(
        observer_activity["place_revision"],
        entity_context["place_revision"]
    );
    let activity_cursor = observer_activity["next"]
        .as_str()
        .expect("Place Activity should have another page");
    let next_observer_activity = server
        .tool(
            "list_activity_at_current_place",
            json!({"cursor": activity_cursor, "limit": 1}),
            Some(observer.id.0),
        )
        .await;
    assert_eq!(
        structured(&next_observer_activity)["place_revision"],
        observer_activity["place_revision"]
    );
    assert_ne!(
        structured(&next_observer_activity)["activity"][0]["id"],
        observer_activity["activity"][0]["id"]
    );

    let changed = json!({
        "request_id": request_id,
        "expected_place_revision": entity_context["place_revision"],
        "prose": "Different content under an accepted request id.",
        "consequence": {
            "type": "introduce_entity",
            "name": "Conflicting Marker",
            "description": "This marker must not be created."
        }
    });
    let conflict_response = server
        .client
        .post(format!("{}/api/action", server.base_url))
        .header(USER_CONTEXT_HEADER, actor.id.0.to_string())
        .json(&changed)
        .send()
        .await
        .expect("conflicting retry should send");
    assert_eq!(conflict_response.status(), StatusCode::CONFLICT);
    let conflict_http: Value = conflict_response
        .json()
        .await
        .expect("conflict should be JSON");
    let conflict_mcp = server
        .tool("submit_action", changed, Some(actor.id.0))
        .await;
    assert_eq!(error_code(&conflict_http), "action_request_conflict");
    assert_eq!(mcp_error(&conflict_mcp), conflict_http);

    let stale = json!({
        "request_id": Uuid::new_v4(),
        "expected_place_revision": entity_context["place_revision"],
        "prose": "A stale action must not change the World.",
        "consequence": {
            "type": "introduce_entity",
            "name": "Stale Marker",
            "description": "This marker must not be created."
        }
    });
    let stale_response = server
        .client
        .post(format!("{}/api/action", server.base_url))
        .header(USER_CONTEXT_HEADER, actor.id.0.to_string())
        .json(&stale)
        .send()
        .await
        .expect("stale action should send");
    assert_eq!(stale_response.status(), StatusCode::PRECONDITION_FAILED);
    let stale_http: Value = stale_response
        .json()
        .await
        .expect("freshness error should be JSON");
    let mut stale_mcp_input = stale;
    stale_mcp_input["request_id"] = json!(Uuid::new_v4());
    let stale_mcp = server
        .tool("submit_action", stale_mcp_input, Some(actor.id.0))
        .await;
    assert_eq!(error_code(&stale_http), "place_revision_conflict");
    assert_eq!(mcp_error(&stale_mcp), stale_http);

    let latest_revision = observer_activity["place_revision"].clone();
    let second_action = json!({
        "request_id": Uuid::new_v4(),
        "expected_place_revision": latest_revision,
        "prose": "Tomas sets a second marker where travelers can compare the routes.",
        "consequence": {
            "type": "introduce_entity",
            "name": "Route Comparison Marker",
            "description": "A second cedar marker with two route notches."
        }
    });
    let second_accepted = server
        .tool("submit_action", second_action, Some(observer.id.0))
        .await;
    assert_eq!(
        structured(&second_accepted)["activity"]["operation"],
        "submit_action"
    );

    let first_entity_page: Value = server
        .client
        .get(format!(
            "{}/api/place/current/entity?limit=1",
            server.base_url
        ))
        .header(USER_CONTEXT_HEADER, actor.id.0.to_string())
        .send()
        .await
        .expect("first exact-Place Entity page should send")
        .json()
        .await
        .expect("first exact-Place Entity page should be JSON");
    let entity_cursor = first_entity_page["next"]
        .as_str()
        .expect("two placed Entities should produce a cursor");
    let second_entity_page = server
        .tool(
            "list_entity_at_current_place",
            json!({"cursor": entity_cursor, "limit": 1}),
            Some(actor.id.0),
        )
        .await;
    assert_eq!(
        structured(&second_entity_page)["place_revision"],
        first_entity_page["place_revision"]
    );
    assert_ne!(
        structured(&second_entity_page)["entity"][0]["id"],
        first_entity_page["entity"][0]["id"]
    );

    let global_entity_page: Value = server
        .client
        .get(format!("{}/api/entity?limit=1", server.base_url))
        .send()
        .await
        .expect("global Entity cursor source should send")
        .json()
        .await
        .expect("global Entity cursor source should be JSON");
    let global_entity_cursor = global_entity_page["next"]
        .as_str()
        .expect("global Entity state should have another page")
        .to_owned();
    let personal_activity_page: Value = server
        .client
        .get(format!("{}/api/activity?limit=1", server.base_url))
        .header(USER_CONTEXT_HEADER, actor.id.0.to_string())
        .send()
        .await
        .expect("personal Activity cursor source should send")
        .json()
        .await
        .expect("personal Activity cursor source should be JSON");
    let personal_activity_cursor = personal_activity_page["next"]
        .as_str()
        .expect("personal Activity state should have another page")
        .to_owned();
    let operation = [
        (
            "/api/activity",
            "list_activity",
            personal_activity_cursor,
            Some(actor.id.0),
        ),
        (
            "/api/place/current/entity",
            "list_entity_at_current_place",
            entity_cursor.to_owned(),
            Some(actor.id.0),
        ),
        (
            "/api/place/current/activity",
            "list_activity_at_current_place",
            activity_cursor.to_owned(),
            Some(actor.id.0),
        ),
    ];
    for (http_path, tool_name, _, user_id) in &operation {
        assert_cross_operation_cursor_rejected(
            &server,
            http_path,
            tool_name,
            &global_entity_cursor,
            *user_id,
        )
        .await;
    }
    for (source_index, (_, _, source_cursor, _)) in operation.iter().enumerate() {
        for (target_index, (http_path, tool_name, _, user_id)) in operation.iter().enumerate() {
            if source_index != target_index {
                assert_cross_operation_cursor_rejected(
                    &server,
                    http_path,
                    tool_name,
                    source_cursor,
                    *user_id,
                )
                .await;
            }
        }
    }
}

#[sqlx::test(migrations = "./migration")]
async fn interaction_http_and_mcp_share_strict_commit_errors_and_scoped_results(pool: PgPool) {
    let world = World::new(pool);
    let actor = world.create_user().await.expect("actor User should exist");
    let target = world.create_user().await.expect("target User should exist");
    let server = TestServer::start(world).await;

    let actor_character = server
        .tool(
            "create_character",
            json!({"name": "Pip", "description": "A tiny grey wanderer."}),
            Some(actor.id.0),
        )
        .await;
    let actor_entity_id = structured(&actor_character)["entity"]["id"].clone();
    server
        .tool(
            "create_character",
            json!({"name": "Mara", "description": "A patient keeper of the square."}),
            Some(target.id.0),
        )
        .await;
    server
        .tool(
            "create_entry_place",
            json!({
                "name": "First Landing",
                "description": "A quiet square where paths begin."
            }),
            Some(actor.id.0),
        )
        .await;
    for user_id in [actor.id.0, target.id.0] {
        server.tool("enter_world", json!({}), Some(user_id)).await;
    }

    let actor_context: Value = server
        .client
        .get(format!("{}/api/place/current/entity", server.base_url))
        .header(USER_CONTEXT_HEADER, actor.id.0.to_string())
        .send()
        .await
        .expect("actor context should send")
        .json()
        .await
        .expect("actor context should be JSON");
    assert_eq!(actor_context["entity"].as_array().map(Vec::len), Some(1));
    let target_entity = &actor_context["entity"][0];
    assert_eq!(target_entity["name"], "Mara");
    for value in [&actor_context["place"], target_entity] {
        assert_eq!(
            value
                .as_object()
                .expect("scoped result should be an object")
                .keys()
                .cloned()
                .collect::<BTreeSet<_>>(),
            BTreeSet::from(["description".to_owned(), "id".to_owned(), "name".to_owned()])
        );
    }
    let serialized_context = actor_context.to_string();
    for forbidden in [
        "introduced_by_user_id",
        "owner_user_id",
        "is_entry",
        "user_controlled",
        "npc",
    ] {
        assert!(!serialized_context.contains(forbidden));
    }

    let target_context = server
        .tool("list_entity_at_current_place", json!({}), Some(target.id.0))
        .await;
    let rat = structured(&target_context)["entity"]
        .as_array()
        .expect("target context should contain safe Entities")
        .iter()
        .find(|entity| entity["name"] == "Pip")
        .expect("Mara should receive Pip as an ordinary local subject");
    assert_eq!(rat["description"], "A tiny grey wanderer.");
    assert_eq!(
        rat.as_object()
            .expect("safe contextual Entity should be an object")
            .keys()
            .cloned()
            .collect::<BTreeSet<_>>(),
        BTreeSet::from(["description".to_owned(), "id".to_owned(), "name".to_owned()])
    );
    for forbidden in [
        "introduced_by_user_id",
        "owner_user_id",
        "character",
        "user_controlled",
        "control",
        "npc",
    ] {
        assert!(
            !rat.to_string().to_lowercase().contains(forbidden),
            "the rat's typed player result must not disclose {forbidden}"
        );
    }

    let request_id = Uuid::new_v4();
    let interaction = json!({
        "request_id": request_id,
        "expected_place_revision": actor_context["place_revision"],
        "prose": "Pip darts in a small circle around Mara's boots, then sits very still.",
        "target_entity_id": [target_entity["id"]]
    });
    let response = server
        .client
        .post(format!("{}/api/interaction", server.base_url))
        .header(USER_CONTEXT_HEADER, actor.id.0.to_string())
        .json(&interaction)
        .send()
        .await
        .expect("HTTP Interaction should send");
    assert_eq!(response.status(), StatusCode::CREATED);
    let accepted: Value = response
        .json()
        .await
        .expect("accepted Interaction should be JSON");
    assert_eq!(accepted["activity"]["operation"], "submit_interaction");
    assert_eq!(accepted["activity"]["actor_character"]["name"], "Pip");
    assert_eq!(accepted["activity"]["prose"], interaction["prose"]);
    assert_eq!(accepted["activity"]["property_change"], json!([]));
    assert_eq!(
        accepted["activity"]["involved_entity"]
            .as_array()
            .expect("Interaction involvement should be an array")
            .iter()
            .map(|reference| reference["role"].as_str().expect("role should be text"))
            .collect::<BTreeSet<_>>(),
        BTreeSet::from(["location", "target"])
    );
    assert_eq!(
        accepted["place"]
            .as_object()
            .expect("accepted Place should be safe")
            .keys()
            .cloned()
            .collect::<BTreeSet<_>>(),
        BTreeSet::from(["description".to_owned(), "id".to_owned(), "name".to_owned()])
    );
    for forbidden in ["introduced_by_user_id", "owner_user_id", "response"] {
        assert!(!accepted.to_string().contains(forbidden));
    }

    let retry = server
        .tool("submit_interaction", interaction.clone(), Some(actor.id.0))
        .await;
    assert_eq!(structured(&retry), &accepted);

    let target_history = server
        .tool("list_activity", json!({"limit": 100}), Some(target.id.0))
        .await;
    let observed = structured(&target_history)["activity"]
        .as_array()
        .expect("target history should be an array")
        .iter()
        .find(|activity| activity["id"] == accepted["activity"]["id"])
        .expect("target history should contain the outward Interaction");
    assert_eq!(observed, &accepted["activity"]);
    assert_eq!(observed["prose"], interaction["prose"]);

    let changed = json!({
        "request_id": request_id,
        "expected_place_revision": actor_context["place_revision"],
        "prose": "Different outward behavior under an accepted request id.",
        "target_entity_id": [target_entity["id"]]
    });
    let response = server
        .client
        .post(format!("{}/api/interaction", server.base_url))
        .header(USER_CONTEXT_HEADER, actor.id.0.to_string())
        .json(&changed)
        .send()
        .await
        .expect("conflicting HTTP Interaction should send");
    assert_eq!(response.status(), StatusCode::CONFLICT);
    let conflict_http: Value = response.json().await.expect("conflict should be JSON");
    let conflict_mcp = server
        .tool("submit_interaction", changed, Some(actor.id.0))
        .await;
    assert_eq!(error_code(&conflict_http), "interaction_request_conflict");
    assert_eq!(mcp_error(&conflict_mcp), conflict_http);

    let fresh_context = server
        .tool("list_entity_at_current_place", json!({}), Some(actor.id.0))
        .await;
    let fresh_revision = structured(&fresh_context)["place_revision"].clone();
    let empty_target = json!({
        "request_id": Uuid::new_v4(),
        "expected_place_revision": fresh_revision,
        "prose": "This has no directed target.",
        "target_entity_id": []
    });
    let response = server
        .client
        .post(format!("{}/api/interaction", server.base_url))
        .header(USER_CONTEXT_HEADER, actor.id.0.to_string())
        .json(&empty_target)
        .send()
        .await
        .expect("invalid HTTP Interaction should send");
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let invalid_http: Value = response.json().await.expect("invalid input should be JSON");
    let mut empty_target_mcp = empty_target;
    empty_target_mcp["request_id"] = json!(Uuid::new_v4());
    let invalid_mcp = server
        .tool("submit_interaction", empty_target_mcp, Some(actor.id.0))
        .await;
    assert_eq!(error_code(&invalid_http), "invalid_interaction");
    assert_eq!(invalid_http["error"]["field"], "target_entity_id");
    assert_eq!(mcp_error(&invalid_mcp), invalid_http);

    let unavailable = json!({
        "request_id": Uuid::new_v4(),
        "expected_place_revision": fresh_revision,
        "prose": "Pip cannot direct an Interaction at Pip.",
        "target_entity_id": [actor_entity_id]
    });
    let response = server
        .client
        .post(format!("{}/api/interaction", server.base_url))
        .header(USER_CONTEXT_HEADER, actor.id.0.to_string())
        .json(&unavailable)
        .send()
        .await
        .expect("unavailable HTTP target should send");
    assert_eq!(response.status(), StatusCode::CONFLICT);
    let unavailable_http: Value = response
        .json()
        .await
        .expect("unavailable target should be JSON");
    let mut unavailable_mcp_input = unavailable;
    unavailable_mcp_input["request_id"] = json!(Uuid::new_v4());
    let unavailable_mcp = server
        .tool(
            "submit_interaction",
            unavailable_mcp_input,
            Some(actor.id.0),
        )
        .await;
    assert_eq!(
        error_code(&unavailable_http),
        "interaction_target_unavailable"
    );
    assert_eq!(mcp_error(&unavailable_mcp), unavailable_http);
}

#[sqlx::test(migrations = "./migration")]
async fn property_http_and_mcp_share_creation_bulk_change_history_and_strict_errors(pool: PgPool) {
    let world = World::new(pool.clone());
    let actor = world.create_user().await.expect("actor User should exist");
    let target = world.create_user().await.expect("target User should exist");
    let server = TestServer::start(world).await;

    let actor_property = text_property("actor", 100);
    let actor_response = server
        .client
        .post(format!("{}/api/character", server.base_url))
        .header(USER_CONTEXT_HEADER, actor.id.0.to_string())
        .json(&json!({
            "name": "Mara",
            "description": "A careful surveyor.",
            "property": actor_property
        }))
        .send()
        .await
        .expect("Property-bearing Character request should send");
    assert_eq!(actor_response.status(), StatusCode::CREATED);
    let actor_character: Value = actor_response
        .json()
        .await
        .expect("Property-bearing Character should be JSON");
    let actor_entity_id = actor_character["entity"]["id"].clone();

    let target_character = server
        .tool(
            "create_character",
            json!({
                "name": "Pip",
                "description": "A tiny grey wanderer.",
                "property": []
            }),
            Some(target.id.0),
        )
        .await;
    let target_entity_id = structured(&target_character)["entity"]["id"].clone();

    let remote_property = text_property("remote", 100);
    let remote_response = server
        .client
        .post(format!("{}/api/entity", server.base_url))
        .header(USER_CONTEXT_HEADER, actor.id.0.to_string())
        .json(&json!({
            "name": "Remote Herbarium",
            "description": "An unplaced collection outside local play.",
            "property": remote_property
        }))
        .send()
        .await
        .expect("Property-bearing Entity request should send");
    assert_eq!(remote_response.status(), StatusCode::CREATED);
    let remote_entity: Value = remote_response
        .json()
        .await
        .expect("Property-bearing Entity should be JSON");
    let remote_entity_id = remote_entity["id"].clone();

    let place_property = text_property("place", 100);
    let place = server
        .tool(
            "create_entry_place",
            json!({
                "name": "First Landing",
                "description": "A quiet square where paths begin.",
                "property": place_property
            }),
            Some(actor.id.0),
        )
        .await;
    let place = structured(&place).clone();
    let place_entity_id = place["entity"]["id"].clone();

    for user_id in [actor.id.0, target.id.0] {
        server.tool("enter_world", json!({}), Some(user_id)).await;
    }

    let context = server
        .tool(
            "list_entity_at_current_place",
            json!({"limit": 100}),
            Some(actor.id.0),
        )
        .await;
    let introduced_property = text_property("marker", 100);
    let introduce = json!({
        "request_id": Uuid::new_v4(),
        "expected_place_revision": structured(&context)["place_revision"],
        "prose": "Mara sets a hundred-marked cedar post beside the landing.",
        "consequence": {
            "type": "introduce_entity",
            "name": "Cedar Post",
            "description": "A cedar post covered in small deliberate marks.",
            "property": introduced_property
        }
    });
    let introduced = server
        .tool("submit_action", introduce, Some(actor.id.0))
        .await;
    let introduced = structured(&introduced).clone();
    assert_eq!(introduced["consequence"]["type"], "introduce_entity");
    assert_eq!(
        introduced["activity"]["property_change"]
            .as_array()
            .map(Vec::len),
        Some(100)
    );
    let ordinary_entity_id = introduced["consequence"]["entity"]["id"].clone();

    let (property_revision, local_property) =
        collect_http_current_property(&server, actor.id.0).await;
    assert_eq!(local_property.len(), 300);
    assert!(
        local_property
            .iter()
            .all(|property| property["entity"]["id"] != remote_entity_id),
        "an unplaced Entity's Properties must not leak into the local page"
    );
    for property in &local_property {
        assert_eq!(
            property
                .as_object()
                .expect("Property row should be an object")
                .keys()
                .cloned()
                .collect::<BTreeSet<_>>(),
            BTreeSet::from(["entity".to_owned(), "key".to_owned(), "value".to_owned()])
        );
        assert_eq!(
            property["entity"]
                .as_object()
                .expect("Property Entity should be a summary")
                .keys()
                .cloned()
                .collect::<BTreeSet<_>>(),
            BTreeSet::from(["id".to_owned(), "name".to_owned()])
        );
    }
    for forbidden in [
        "property_key_id",
        "owner_user_id",
        "introduced_by_user_id",
        "user_controlled",
        "npc",
    ] {
        assert!(
            !Value::Array(local_property.clone())
                .to_string()
                .contains(forbidden)
        );
    }

    let first_property_page: Value = server
        .client
        .get(format!(
            "{}/api/place/current/entity/{}?limit=1",
            server.base_url,
            actor_entity_id.as_str().expect("actor id should be text")
        ))
        .header(USER_CONTEXT_HEADER, actor.id.0.to_string())
        .send()
        .await
        .expect("first paged Property request should send")
        .json()
        .await
        .expect("first paged Property response should be JSON");
    let property_cursor = first_property_page["current_state"]["next"]
        .as_str()
        .expect("three hundred local Properties should produce a cursor");
    let second_property_page = server
        .tool(
            "get_entity_at_current_place",
            json!({"entity_id": actor_entity_id, "cursor": property_cursor, "limit": 1}),
            Some(actor.id.0),
        )
        .await;
    assert_eq!(
        structured(&second_property_page)["place_revision"],
        first_property_page["place_revision"]
    );
    assert_ne!(
        structured(&second_property_page)["current_state"]["association"][0],
        first_property_page["current_state"]["association"][0]
    );
    assert_cross_operation_cursor_rejected(
        &server,
        "/api/place/current/activity",
        "list_activity_at_current_place",
        property_cursor,
        Some(actor.id.0),
    )
    .await;
    for (query, mcp_input, expected_code) in [
        (
            "cursor=not-a-property-cursor",
            json!({"entity_id": actor_entity_id, "cursor": "not-a-property-cursor"}),
            "invalid_request",
        ),
        (
            "limit=0",
            json!({"entity_id": actor_entity_id, "limit": 0}),
            "invalid_entity_limit",
        ),
        (
            "limit=101",
            json!({"entity_id": actor_entity_id, "limit": 101}),
            "invalid_entity_limit",
        ),
    ] {
        let response = server
            .client
            .get(format!(
                "{}/api/place/current/entity/{}?{query}",
                server.base_url,
                actor_entity_id.as_str().expect("actor id should be text")
            ))
            .header(USER_CONTEXT_HEADER, actor.id.0.to_string())
            .send()
            .await
            .expect("invalid Property page request should send");
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        let http_error: Value = response
            .json()
            .await
            .expect("invalid Property page error should be JSON");
        let mcp = server
            .tool("get_entity_at_current_place", mcp_input, Some(actor.id.0))
            .await;
        assert_eq!(error_code(&http_error), expected_code);
        assert_eq!(mcp_error(&mcp), http_error);
    }

    let mut explosion_change = vec![
        json!({"entity_id": place_entity_id, "key": "surface", "value": {"type": "text", "text": "blackened"}}),
        json!({"entity_id": ordinary_entity_id, "key": "surface", "value": {"type": "text", "text": "blackened"}}),
        json!({"entity_id": target_entity_id, "key": "surface", "value": {"type": "text", "text": "blackened"}}),
        json!({"entity_id": actor_entity_id, "key": "surface", "value": {"type": "text", "text": "blackened"}}),
    ];
    explosion_change.extend((0..96).map(|index| {
        json!({
            "entity_id": actor_entity_id,
            "key": format!("blast_{index:03}"),
            "value": {"type": "integer", "integer": index}
        })
    }));
    let explosion = json!({
        "request_id": Uuid::new_v4(),
        "expected_place_revision": property_revision,
        "prose": "A sudden blast blackens everyone and everything around the landing.",
        "consequence": {
            "type": "change_entity_property",
            "property_change": explosion_change
        }
    });
    let explosion_response = server
        .client
        .post(format!("{}/api/action", server.base_url))
        .header(USER_CONTEXT_HEADER, actor.id.0.to_string())
        .json(&explosion)
        .send()
        .await
        .expect("Property explosion should send");
    assert_eq!(explosion_response.status(), StatusCode::CREATED);
    let accepted_explosion: Value = explosion_response
        .json()
        .await
        .expect("accepted Property explosion should be JSON");
    assert_eq!(
        accepted_explosion["consequence"]["type"],
        "change_entity_property"
    );
    assert_eq!(
        accepted_explosion["consequence"]["property_change"]
            .as_array()
            .map(Vec::len),
        Some(100)
    );
    assert_eq!(
        accepted_explosion["activity"]["property_change"],
        accepted_explosion["consequence"]["property_change"]
    );
    let explosion_retry = server
        .tool("submit_action", explosion.clone(), Some(actor.id.0))
        .await;
    assert_eq!(structured(&explosion_retry), &accepted_explosion);

    let after_explosion = server
        .tool(
            "get_entity_at_current_place",
            json!({"entity_id": actor_entity_id, "limit": 100}),
            Some(actor.id.0),
        )
        .await;
    let interaction = json!({
        "request_id": Uuid::new_v4(),
        "expected_place_revision": structured(&after_explosion)["place_revision"],
        "prose": "Mara brushes ash from Pip and from her own coat without deciding Pip's response.",
        "target_entity_id": [target_entity_id],
        "property_change": [
            {"entity_id": target_entity_id, "key": "ash", "value": {"type": "text", "text": "brushed away"}},
            {"entity_id": actor_entity_id, "key": "ash", "value": {"type": "text", "text": "brushed away"}}
        ]
    });
    let accepted_interaction = server
        .tool("submit_interaction", interaction.clone(), Some(actor.id.0))
        .await;
    let accepted_interaction = structured(&accepted_interaction).clone();
    assert_eq!(
        accepted_interaction["activity"]["property_change"]
            .as_array()
            .map(Vec::len),
        Some(2)
    );
    assert!(accepted_interaction.get("response").is_none());
    assert!(!accepted_interaction.to_string().contains("owner_user_id"));
    let interaction_retry = server
        .client
        .post(format!("{}/api/interaction", server.base_url))
        .header(USER_CONTEXT_HEADER, actor.id.0.to_string())
        .json(&interaction)
        .send()
        .await
        .expect("Interaction retry should send");
    assert_eq!(interaction_retry.status(), StatusCode::CREATED);
    assert_eq!(
        interaction_retry
            .json::<Value>()
            .await
            .expect("Interaction retry should be JSON"),
        accepted_interaction
    );

    let (special_revision, _) = collect_mcp_current_property(&server, actor.id.0).await;
    let provenance_like_change = json!([
        {
            "entity_id": ordinary_entity_id,
            "key": "user_controlled",
            "value": {"type": "text", "text": "a phrase painted for a guessing game"}
        },
        {
            "entity_id": ordinary_entity_id,
            "key": "npc",
            "value": {"type": "integer", "integer": 7}
        },
        {
            "entity_id": ordinary_entity_id,
            "key": "owner_user_id",
            "value": {"type": "text", "text": "a fictional catalogue label"}
        }
    ]);
    let expected_provenance_like_change = json!([
        {
            "entity": {"id": ordinary_entity_id, "name": "Cedar Post"},
            "key": "npc",
            "value": {"type": "integer", "integer": 7}
        },
        {
            "entity": {"id": ordinary_entity_id, "name": "Cedar Post"},
            "key": "owner_user_id",
            "value": {"type": "text", "text": "a fictional catalogue label"}
        },
        {
            "entity": {"id": ordinary_entity_id, "name": "Cedar Post"},
            "key": "user_controlled",
            "value": {"type": "text", "text": "a phrase painted for a guessing game"}
        }
    ]);
    let provenance_like_action = json!({
        "request_id": Uuid::new_v4(),
        "expected_place_revision": special_revision,
        "prose": "Mara paints three deliberately misleading catalogue labels on the cedar post; they classify no person or controller.",
        "consequence": {
            "type": "change_entity_property",
            "property_change": provenance_like_change
        }
    });
    let provenance_like_response = server
        .client
        .post(format!("{}/api/action", server.base_url))
        .header(USER_CONTEXT_HEADER, actor.id.0.to_string())
        .json(&provenance_like_action)
        .send()
        .await
        .expect("provenance-like Property Action should send");
    assert_eq!(provenance_like_response.status(), StatusCode::CREATED);
    let accepted_provenance_like: Value = provenance_like_response
        .json()
        .await
        .expect("provenance-like Property Action should be JSON");
    assert_eq!(
        accepted_provenance_like["consequence"]["property_change"], expected_provenance_like_change,
        "canonical provenance-like keys must be accepted as ordinary fictional Property data"
    );
    assert_eq!(
        accepted_provenance_like["activity"]["property_change"],
        expected_provenance_like_change
    );

    let (http_special_revision, http_current_property) =
        collect_http_current_property(&server, actor.id.0).await;
    let (mcp_special_revision, mcp_current_property) =
        collect_mcp_current_property(&server, actor.id.0).await;
    assert_eq!(http_special_revision, mcp_special_revision);
    assert_eq!(http_current_property, mcp_current_property);
    for (key, value) in [
        (
            "user_controlled",
            json!({"type": "text", "text": "a phrase painted for a guessing game"}),
        ),
        ("npc", json!({"type": "integer", "integer": 7})),
        (
            "owner_user_id",
            json!({"type": "text", "text": "a fictional catalogue label"}),
        ),
    ] {
        let property = http_current_property
            .iter()
            .find(|property| {
                property["entity"]["id"] == ordinary_entity_id && property["key"] == key
            })
            .unwrap_or_else(|| panic!("current Property output should retain ordinary key {key}"));
        assert_eq!(property["value"], value);
        assert_eq!(
            property
                .as_object()
                .expect("current Property should be an object")
                .keys()
                .cloned()
                .collect::<BTreeSet<_>>(),
            BTreeSet::from(["entity".to_owned(), "key".to_owned(), "value".to_owned()])
        );
        assert_eq!(
            property["entity"],
            json!({"id": ordinary_entity_id, "name": "Cedar Post"}),
            "a provenance-like fictional key must not classify or expose control of its Entity"
        );
    }

    let ordinary_after_labels = server
        .tool(
            "list_entity_at_current_place",
            json!({"limit": 100}),
            Some(actor.id.0),
        )
        .await;
    let ordinary_after_labels = structured(&ordinary_after_labels)["entity"]
        .as_array()
        .expect("current Entity page should contain local subjects")
        .iter()
        .find(|entity| entity["id"] == ordinary_entity_id)
        .expect("the labelled cedar post should remain locally selectable");
    assert_eq!(
        ordinary_after_labels
            .as_object()
            .expect("current Entity should be an object")
            .keys()
            .cloned()
            .collect::<BTreeSet<_>>(),
        BTreeSet::from(["description".to_owned(), "id".to_owned(), "name".to_owned()]),
        "fictional keys must not add Character, NPC, owner, User or control classification fields"
    );

    let history: Value = server
        .client
        .get(format!("{}/api/activity?limit=100", server.base_url))
        .header(USER_CONTEXT_HEADER, actor.id.0.to_string())
        .send()
        .await
        .expect("Property history should send")
        .json()
        .await
        .expect("Property history should be JSON");
    let history = history["activity"]
        .as_array()
        .expect("personal history should contain Activities");
    let mcp_history = server
        .tool("list_activity", json!({"limit": 100}), Some(actor.id.0))
        .await;
    let mcp_history = structured(&mcp_history)["activity"]
        .as_array()
        .expect("MCP personal history should contain Activities");
    let provenance_like_activity_id = accepted_provenance_like["activity"]["id"].clone();
    for authorized_history in [history, mcp_history] {
        let activity = authorized_history
            .iter()
            .find(|activity| activity["id"] == provenance_like_activity_id)
            .expect("authorized Activity should retain provenance-like fictional keys");
        assert_eq!(activity["property_change"], expected_provenance_like_change);
        for property in activity["property_change"]
            .as_array()
            .expect("Activity Property changes should be an array")
        {
            assert_eq!(
                property
                    .as_object()
                    .expect("Activity Property should be an object")
                    .keys()
                    .cloned()
                    .collect::<BTreeSet<_>>(),
                BTreeSet::from(["entity".to_owned(), "key".to_owned(), "value".to_owned()])
            );
            assert_eq!(
                property["entity"],
                json!({"id": ordinary_entity_id, "name": "Cedar Post"})
            );
        }
    }
    for (activity_id, expected_change) in [
        (
            accepted_explosion["activity"]["id"].clone(),
            accepted_explosion["activity"]["property_change"].clone(),
        ),
        (
            accepted_interaction["activity"]["id"].clone(),
            accepted_interaction["activity"]["property_change"].clone(),
        ),
        (
            introduced["activity"]["id"].clone(),
            introduced["activity"]["property_change"].clone(),
        ),
        (
            accepted_provenance_like["activity"]["id"].clone(),
            accepted_provenance_like["activity"]["property_change"].clone(),
        ),
    ] {
        let activity = history
            .iter()
            .find(|activity| activity["id"] == activity_id)
            .expect("exact Property-changing Activity should be visible");
        assert_eq!(activity["property_change"], expected_change);
    }
    for operation in ["create_character", "create_entity", "create_entry_place"] {
        let activity = history
            .iter()
            .find(|activity| activity["operation"] == operation)
            .expect("each creation Activity should be visible");
        assert_eq!(
            activity["property_change"].as_array().map(Vec::len),
            Some(100),
            "{operation} should hydrate all initial Properties"
        );
    }

    let fresh = server
        .tool(
            "get_entity_at_current_place",
            json!({"entity_id": actor_entity_id, "limit": 1}),
            Some(actor.id.0),
        )
        .await;
    let fresh_revision = structured(&fresh)["place_revision"].clone();

    for malformed in [
        json!({
            "request_id": Uuid::new_v4(),
            "expected_place_revision": fresh_revision,
            "prose": "A malformed Entity id must fail at the wire boundary.",
            "consequence": {
                "type": "change_entity_property",
                "property_change": [
                    {"entity_id": "not-a-uuid", "key": "wire", "value": {"type": "text", "text": "no"}}
                ]
            }
        }),
        json!({
            "request_id": Uuid::new_v4(),
            "expected_place_revision": fresh_revision,
            "prose": "An unsupported Property value tag must fail at the wire boundary.",
            "consequence": {
                "type": "change_entity_property",
                "property_change": [
                    {"entity_id": actor_entity_id, "key": "wire", "value": {"type": "boolean", "boolean": true}}
                ]
            }
        }),
        json!({
            "request_id": Uuid::new_v4(),
            "expected_place_revision": fresh_revision,
            "prose": "An unknown nested Property field must fail at the wire boundary.",
            "consequence": {
                "type": "change_entity_property",
                "property_change": [
                    {"entity_id": actor_entity_id, "key": "wire", "value": {"type": "text", "text": "no"}, "unexpected": true}
                ]
            }
        }),
    ] {
        let response = server
            .client
            .post(format!("{}/api/action", server.base_url))
            .header(USER_CONTEXT_HEADER, actor.id.0.to_string())
            .json(&malformed)
            .send()
            .await
            .expect("malformed Property Action should send");
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        let http_error: Value = response
            .json()
            .await
            .expect("malformed Property error should be JSON");
        let mcp = server
            .tool("submit_action", malformed, Some(actor.id.0))
            .await;
        assert_eq!(error_code(&http_error), "invalid_request");
        assert_eq!(mcp_error(&mcp), http_error);
    }

    let remote_change = json!({
        "request_id": Uuid::new_v4(),
        "expected_place_revision": fresh_revision,
        "prose": "This cannot reach the remote herbarium.",
        "consequence": {
            "type": "change_entity_property",
            "property_change": [
                {"entity_id": remote_entity_id, "key": "leaked", "value": {"type": "text", "text": "no"}}
            ]
        }
    });
    let remote_response = server
        .client
        .post(format!("{}/api/action", server.base_url))
        .header(USER_CONTEXT_HEADER, actor.id.0.to_string())
        .json(&remote_change)
        .send()
        .await
        .expect("remote Property Action should send");
    assert_eq!(remote_response.status(), StatusCode::CONFLICT);
    let remote_http: Value = remote_response
        .json()
        .await
        .expect("remote Property error should be JSON");
    let mut remote_mcp_input = remote_change;
    remote_mcp_input["request_id"] = json!(Uuid::new_v4());
    let remote_mcp = server
        .tool("submit_action", remote_mcp_input, Some(actor.id.0))
        .await;
    assert_eq!(error_code(&remote_http), "property_entity_unavailable");
    assert_eq!(mcp_error(&remote_mcp), remote_http);

    let non_target_change = json!({
        "request_id": Uuid::new_v4(),
        "expected_place_revision": fresh_revision,
        "prose": "This names a local Entity that is not an Interaction target.",
        "target_entity_id": [target_entity_id],
        "property_change": [
            {"entity_id": ordinary_entity_id, "key": "leaked", "value": {"type": "text", "text": "no"}}
        ]
    });
    let non_target_response = server
        .client
        .post(format!("{}/api/interaction", server.base_url))
        .header(USER_CONTEXT_HEADER, actor.id.0.to_string())
        .json(&non_target_change)
        .send()
        .await
        .expect("non-target Property Interaction should send");
    assert_eq!(non_target_response.status(), StatusCode::CONFLICT);
    let non_target_http: Value = non_target_response
        .json()
        .await
        .expect("non-target Property error should be JSON");
    let mut non_target_mcp_input = non_target_change;
    non_target_mcp_input["request_id"] = json!(Uuid::new_v4());
    let non_target_mcp = server
        .tool("submit_interaction", non_target_mcp_input, Some(actor.id.0))
        .await;
    assert_eq!(error_code(&non_target_http), "property_entity_unavailable");
    assert_eq!(mcp_error(&non_target_mcp), non_target_http);

    let invalid_action = |property_change: Vec<Value>, prose: &str| {
        json!({
            "request_id": Uuid::new_v4(),
            "expected_place_revision": fresh_revision,
            "prose": prose,
            "consequence": {
                "type": "change_entity_property",
                "property_change": property_change
            }
        })
    };
    let invalid = [
        (
            invalid_action(Vec::new(), "An empty Property consequence must fail."),
            "out_of_range",
        ),
        (
            invalid_action(
                vec![
                    json!({"entity_id": actor_entity_id, "key": "duplicate", "value": {"type": "text", "text": "first"}}),
                    json!({"entity_id": actor_entity_id, "key": "duplicate", "value": {"type": "text", "text": "second"}}),
                ],
                "A duplicate Entity and key pair must fail.",
            ),
            "duplicate",
        ),
        (
            invalid_action(
                vec![
                    json!({"entity_id": actor_entity_id, "key": "Bad Key", "value": {"type": "text", "text": "no"}}),
                ],
                "An invalid canonical key must fail.",
            ),
            "invalid_format",
        ),
    ];
    for (input, reason) in invalid {
        let response = server
            .client
            .post(format!("{}/api/action", server.base_url))
            .header(USER_CONTEXT_HEADER, actor.id.0.to_string())
            .json(&input)
            .send()
            .await
            .expect("invalid Property Action should send");
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        let http_error: Value = response
            .json()
            .await
            .expect("invalid Property error should be JSON");
        let mut mcp_input = input;
        mcp_input["request_id"] = json!(Uuid::new_v4());
        let mcp = server
            .tool("submit_action", mcp_input, Some(actor.id.0))
            .await;
        assert_eq!(error_code(&http_error), "invalid_property");
        assert_eq!(http_error["error"]["reason"], reason);
        assert_eq!(mcp_error(&mcp), http_error);
    }

    let overflow_change = (0..101)
        .map(|index| {
            json!({
                "entity_id": actor_entity_id,
                "key": format!("overflow_change_{index:03}"),
                "value": {"type": "integer", "integer": index}
            })
        })
        .collect();
    let overflow_action = invalid_action(
        overflow_change,
        "One hundred and one Property changes must fail atomically.",
    );
    let overflow_response = server
        .client
        .post(format!("{}/api/action", server.base_url))
        .header(USER_CONTEXT_HEADER, actor.id.0.to_string())
        .json(&overflow_action)
        .send()
        .await
        .expect("overflow Property Action should send");
    assert_eq!(overflow_response.status(), StatusCode::BAD_REQUEST);
    let overflow_http: Value = overflow_response
        .json()
        .await
        .expect("overflow Property error should be JSON");
    let overflow_mcp = server
        .tool("submit_action", overflow_action, Some(actor.id.0))
        .await;
    assert_eq!(error_code(&overflow_http), "invalid_property");
    assert_eq!(overflow_http["error"]["reason"], "out_of_range");
    assert_eq!(mcp_error(&overflow_mcp), overflow_http);

    let conflict = invalid_action(
        vec![json!({
            "entity_id": actor_entity_id,
            "key": "actor_000",
            "value": {"type": "integer", "integer": 1}
        })],
        "A canonical key cannot change its value type.",
    );
    let conflict_response = server
        .client
        .post(format!("{}/api/action", server.base_url))
        .header(USER_CONTEXT_HEADER, actor.id.0.to_string())
        .json(&conflict)
        .send()
        .await
        .expect("Property type conflict should send");
    assert_eq!(conflict_response.status(), StatusCode::CONFLICT);
    let conflict_http: Value = conflict_response
        .json()
        .await
        .expect("Property type conflict should be JSON");
    let mut conflict_mcp_input = conflict;
    conflict_mcp_input["request_id"] = json!(Uuid::new_v4());
    let conflict_mcp = server
        .tool("submit_action", conflict_mcp_input, Some(actor.id.0))
        .await;
    assert_eq!(error_code(&conflict_http), "property_key_conflict");
    assert_eq!(mcp_error(&conflict_mcp), conflict_http);

    let stale = json!({
        "request_id": Uuid::new_v4(),
        "expected_place_revision": first_property_page["place_revision"],
        "prose": "A stale Property consequence must not be accepted.",
        "consequence": {
            "type": "change_entity_property",
            "property_change": [
                {"entity_id": actor_entity_id, "key": "stale", "value": {"type": "text", "text": "no"}}
            ]
        }
    });
    let stale_response = server
        .client
        .post(format!("{}/api/action", server.base_url))
        .header(USER_CONTEXT_HEADER, actor.id.0.to_string())
        .json(&stale)
        .send()
        .await
        .expect("stale Property Action should send");
    assert_eq!(stale_response.status(), StatusCode::PRECONDITION_FAILED);

    let overflow_property = text_property("overflow", 101);
    let overflow_entity = json!({
        "name": "Overflow Entity",
        "description": "This Entity must roll back completely.",
        "property": overflow_property
    });
    let overflow_entity_response = server
        .client
        .post(format!("{}/api/entity", server.base_url))
        .header(USER_CONTEXT_HEADER, actor.id.0.to_string())
        .json(&overflow_entity)
        .send()
        .await
        .expect("overflow Entity request should send");
    assert_eq!(overflow_entity_response.status(), StatusCode::BAD_REQUEST);
    let overflow_entity_http: Value = overflow_entity_response
        .json()
        .await
        .expect("overflow Entity error should be JSON");
    let overflow_entity_mcp = server
        .tool("create_entity", overflow_entity, Some(actor.id.0))
        .await;
    assert_eq!(error_code(&overflow_entity_http), "invalid_property");
    assert_eq!(overflow_entity_http["error"]["field"], "property");
    assert_eq!(overflow_entity_http["error"]["reason"], "out_of_range");
    assert_eq!(mcp_error(&overflow_entity_mcp), overflow_entity_http);

    let rejected_state: (i64, i64, i64) = sqlx::query_as(
        r#"
        SELECT
          (SELECT count(*) FROM entity WHERE name = 'Overflow Entity'),
          (SELECT count(*) FROM property_key WHERE key = 'leaked'),
          (SELECT count(*) FROM property_key WHERE key LIKE 'overflow_change_%')
        "#,
    )
    .fetch_one(&pool)
    .await
    .expect("rejected Property state should be queryable");
    assert_eq!(rejected_state, (0, 0, 0));
}

#[sqlx::test(migrations = "./migration")]
async fn trait_http_and_mcp_share_contextual_change_entity_fetch_and_history(pool: PgPool) {
    let world = World::new(pool);
    let actor = world.create_user().await.unwrap();
    let target = world.create_user().await.unwrap();
    let server = TestServer::start(world).await;

    let actor_character = server
        .tool(
            "create_character",
            json!({"name":"Mara","description":"A careful surveyor.","property":[{"key":"pace","value":{"type":"integer","integer":2}}]}),
            Some(actor.id.0),
        )
        .await;
    let actor_entity_id = structured(&actor_character)["entity"]["id"].clone();
    let target_character = server
        .tool(
            "create_character",
            json!({"name":"Pip","description":"A tiny grey wanderer."}),
            Some(target.id.0),
        )
        .await;
    let target_entity_id = structured(&target_character)["entity"]["id"].clone();
    server
        .tool(
            "create_entry_place",
            json!({"name":"First Landing","description":"A quiet square."}),
            Some(actor.id.0),
        )
        .await;
    for user_id in [actor.id.0, target.id.0] {
        server.tool("enter_world", json!({}), Some(user_id)).await;
    }

    let context = server
        .tool("list_entity_at_current_place", json!({}), Some(actor.id.0))
        .await;
    let place_entity_id = structured(&context)["place"]["id"].clone();
    let introduced = server
        .tool(
            "submit_action",
            json!({
                "request_id":Uuid::new_v4(),
                "expected_place_revision":structured(&context)["place_revision"],
                "prose":"Mara sets a small brass bell beside the landing.",
                "consequence":{"type":"introduce_entity","name":"Brass Bell","description":"A small bell with a clear tone."}
            }),
            Some(actor.id.0),
        )
        .await;
    let ordinary_entity_id = structured(&introduced)["consequence"]["entity"]["id"].clone();
    let context = server
        .tool("list_entity_at_current_place", json!({}), Some(actor.id.0))
        .await;
    let establish = json!({
        "request_id": Uuid::new_v4(),
        "expected_place_revision": structured(&context)["place_revision"],
        "prose": "Mara watches Pip wait for the returning echo.",
        "consequence": {
            "type": "change_entity_trait",
            "trait_change": [
                {"type":"establish","entity_id":actor_entity_id,"statement":"Waits for the second echo before moving."},
                {"type":"establish","entity_id":target_entity_id,"statement":"Jumps unusually high when startled."},
                {"type":"establish","entity_id":place_entity_id,"statement":"Carries a returning echo beneath its arches."},
                {"type":"establish","entity_id":ordinary_entity_id,"statement":"Rings with an unusually clear second tone."}
            ]
        }
    });
    let response = server
        .client
        .post(format!("{}/api/action", server.base_url))
        .header(USER_CONTEXT_HEADER, actor.id.0.to_string())
        .json(&establish)
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    let accepted: Value = response.json().await.unwrap();
    assert_eq!(accepted["consequence"]["type"], "change_entity_trait");
    assert_eq!(
        accepted["activity"]["trait_change"]
            .as_array()
            .map(Vec::len),
        Some(4)
    );
    assert_eq!(accepted["activity"]["property_change"], json!([]));
    let actor_trait_id = accepted["consequence"]["trait_change"]
        .as_array()
        .unwrap()
        .iter()
        .find(|change| change["entity"]["id"] == actor_entity_id)
        .unwrap()["trait"]["id"]
        .clone();

    let actor_state = server
        .tool(
            "get_entity_at_current_place",
            json!({"entity_id":actor_entity_id,"limit":1}),
            Some(actor.id.0),
        )
        .await;
    assert_eq!(
        structured(&actor_state)["current_state"]["association"][0]["type"],
        "property"
    );
    let stale_cursor = structured(&actor_state)["current_state"]["next"].clone();
    let interaction = json!({
        "request_id": Uuid::new_v4(),
        "expected_place_revision": structured(&actor_state)["place_revision"],
        "prose": "Mara taps twice; Pip springs closer.",
        "target_entity_id": [target_entity_id],
        "property_change": [{"entity_id":target_entity_id,"key":"stance","value":{"type":"text","text":"close"}}],
        "trait_change": [{"type":"develop","trait_id":actor_trait_id,"statement":"Waits for Pip's second footfall before moving."}]
    });
    let developed = server
        .tool("submit_interaction", interaction.clone(), Some(actor.id.0))
        .await;
    let developed = structured(&developed);
    assert_eq!(developed["activity"]["trait_change"][0]["type"], "develop");
    assert_eq!(
        developed["activity"]["trait_change"][0]["previous_statement"],
        "Waits for the second echo before moving."
    );
    assert_eq!(
        developed["activity"]["property_change"]
            .as_array()
            .map(Vec::len),
        Some(1)
    );
    assert!(developed.get("response").is_none());

    let stale = server
        .tool(
            "get_entity_at_current_place",
            json!({"entity_id":actor_entity_id,"cursor":stale_cursor,"limit":1}),
            Some(actor.id.0),
        )
        .await;
    assert_eq!(error_code(&mcp_error(&stale)), "place_revision_conflict");

    let page: Value = server
        .client
        .get(format!("{}/api/character?limit=100", server.base_url))
        .header(USER_CONTEXT_HEADER, actor.id.0.to_string())
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(page["character"]["entity"]["id"], actor_entity_id);
    let current_trait = page["current_state"]["association"]
        .as_array()
        .unwrap()
        .iter()
        .find(|association| association["type"] == "trait")
        .unwrap();
    assert_eq!(current_trait["trait"]["id"], actor_trait_id);
    assert_eq!(
        current_trait["trait"]["statement"],
        "Waits for Pip's second footfall before moving."
    );
    assert!(!page.to_string().contains("owner_user_id\":null"));

    let no_op = json!({
        "request_id":Uuid::new_v4(),
        "expected_place_revision":page["place_revision"],
        "prose":"Nothing changes.",
        "consequence":{"type":"change_entity_trait","trait_change":[{"type":"develop","trait_id":actor_trait_id,"statement":"Waits for Pip's second footfall before moving."}]}
    });
    let no_op_response = server.tool("submit_action", no_op, Some(actor.id.0)).await;
    assert_eq!(error_code(&mcp_error(&no_op_response)), "invalid_trait");

    let target_first: Value = server
        .client
        .get(format!(
            "{}/api/place/current/entity/{}?limit=1",
            server.base_url,
            target_entity_id.as_str().unwrap()
        ))
        .header(USER_CONTEXT_HEADER, actor.id.0.to_string())
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(
        target_first["current_state"]["association"][0]["type"],
        "property"
    );
    let target_second = server.tool(
        "get_entity_at_current_place",
        json!({"entity_id":target_entity_id,"cursor":target_first["current_state"]["next"],"limit":1}),
        Some(actor.id.0)
    ).await;
    assert_eq!(
        structured(&target_second)["place_revision"],
        target_first["place_revision"]
    );
    assert_eq!(
        structured(&target_second)["current_state"]["association"][0]["type"],
        "trait"
    );

    let (removed_status, removed) = server
        .mcp(
            "tools/call",
            Some("list_entity_property_at_current_place"),
            json!({"name":"list_entity_property_at_current_place","arguments":{}}),
            Some(actor.id.0),
            None,
        )
        .await;
    assert_eq!(removed_status, StatusCode::BAD_REQUEST);
    assert_eq!(removed["error"]["code"], -32602);
}

#[sqlx::test(migrations = "./migration")]
async fn http_and_mcp_share_canonical_capability_errors(pool: PgPool) {
    let world = World::new(pool);
    let user = world.create_user().await.expect("setup User should exist");
    let server = TestServer::start(world).await;

    let response = server
        .client
        .get(format!("{}/api/character", server.base_url))
        .header(USER_CONTEXT_HEADER, user.id.0.to_string())
        .send()
        .await
        .expect("missing Character request should send");
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    let missing_character_http: Value = response.json().await.expect("error should be JSON");
    let missing_character_mcp = server
        .tool("get_character", json!({}), Some(user.id.0))
        .await;
    assert_eq!(error_code(&missing_character_http), "character_not_found");
    assert_eq!(mcp_error(&missing_character_mcp), missing_character_http);

    let invalid_character = json!({"name": "   ", "description": "Valid"});
    let response = server
        .client
        .post(format!("{}/api/character", server.base_url))
        .header(USER_CONTEXT_HEADER, user.id.0.to_string())
        .json(&invalid_character)
        .send()
        .await
        .expect("invalid Character request should send");
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let invalid_character_http: Value = response.json().await.expect("error should be JSON");
    let invalid_character_mcp = server
        .tool("create_character", invalid_character, Some(user.id.0))
        .await;
    assert_eq!(error_code(&invalid_character_http), "invalid_character");
    assert_eq!(mcp_error(&invalid_character_mcp), invalid_character_http);

    let valid_character = json!({"name": "Mara Venn", "description": "A surveyor."});
    server
        .tool("create_character", valid_character.clone(), Some(user.id.0))
        .await;
    let response = server
        .client
        .post(format!("{}/api/character", server.base_url))
        .header(USER_CONTEXT_HEADER, user.id.0.to_string())
        .json(&valid_character)
        .send()
        .await
        .expect("duplicate Character request should send");
    assert_eq!(response.status(), StatusCode::CONFLICT);
    let duplicate_character_http: Value = response.json().await.expect("error should be JSON");
    let duplicate_character_mcp = server
        .tool("create_character", valid_character, Some(user.id.0))
        .await;
    assert_eq!(
        error_code(&duplicate_character_http),
        "character_already_exists"
    );
    assert_eq!(
        mcp_error(&duplicate_character_mcp),
        duplicate_character_http
    );

    let response = server
        .client
        .post(format!("{}/api/world/entry", server.base_url))
        .header(USER_CONTEXT_HEADER, user.id.0.to_string())
        .send()
        .await
        .expect("entry without a Place should send");
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    let no_entry_http: Value = response.json().await.expect("error should be JSON");
    let no_entry_mcp = server.tool("enter_world", json!({}), Some(user.id.0)).await;
    assert_eq!(error_code(&no_entry_http), "entry_place_not_found");
    assert_eq!(mcp_error(&no_entry_mcp), no_entry_http);

    let invalid_place = json!({"name": "   ", "description": "Valid"});
    let response = server
        .client
        .post(format!("{}/api/place/entry", server.base_url))
        .header(USER_CONTEXT_HEADER, user.id.0.to_string())
        .json(&invalid_place)
        .send()
        .await
        .expect("invalid entry Place should send");
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let invalid_place_http: Value = response.json().await.expect("error should be JSON");
    let invalid_place_mcp = server
        .tool("create_entry_place", invalid_place, Some(user.id.0))
        .await;
    assert_eq!(error_code(&invalid_place_http), "invalid_place");
    assert_eq!(mcp_error(&invalid_place_mcp), invalid_place_http);

    server
        .tool(
            "create_entry_place",
            json!({"name": "North Gate", "description": "The shared entry."}),
            Some(user.id.0),
        )
        .await;
    let duplicate_place = json!({"name": "Other Gate", "description": "Must not exist."});
    let response = server
        .client
        .post(format!("{}/api/place/entry", server.base_url))
        .header(USER_CONTEXT_HEADER, user.id.0.to_string())
        .json(&duplicate_place)
        .send()
        .await
        .expect("duplicate entry Place should send");
    assert_eq!(response.status(), StatusCode::CONFLICT);
    let duplicate_place_http: Value = response.json().await.expect("error should be JSON");
    let duplicate_place_mcp = server
        .tool("create_entry_place", duplicate_place, Some(user.id.0))
        .await;
    assert_eq!(
        error_code(&duplicate_place_http),
        "entry_place_already_exists"
    );
    assert_eq!(mcp_error(&duplicate_place_mcp), duplicate_place_http);

    let response = server
        .client
        .get(format!("{}/api/user", server.base_url))
        .send()
        .await
        .expect("missing-context request should send");
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let missing_http: Value = response.json().await.expect("error should be JSON");
    let missing_mcp = server.tool("get_user", json!({}), None).await;
    assert_eq!(
        missing_http,
        json!({
            "error": {
                "code": "user_context_required",
                "message": "Aicadia-User-Id is required."
            }
        })
    );
    assert_eq!(error_code(&missing_http), "user_context_required");
    let missing_mcp_error = mcp_error(&missing_mcp);
    assert_eq!(
        mcp_error_code(&missing_mcp_error),
        error_code(&missing_http)
    );
    assert_eq!(missing_mcp["result"]["isError"], true);
    assert!(missing_mcp["result"].get("structuredContent").is_none());
    assert_eq!(
        missing_mcp["result"]["content"].as_array().unwrap().len(),
        1
    );

    let response = server
        .client
        .get(format!("{}/api/user", server.base_url))
        .header(USER_CONTEXT_HEADER, "not-a-uuid")
        .send()
        .await
        .expect("malformed-context request should send");
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let malformed_http: Value = response.json().await.expect("error should be JSON");
    let (_, malformed_mcp) = server
        .mcp_with_user_context(
            "tools/call",
            Some("get_user"),
            json!({"name": "get_user", "arguments": {}}),
            Some("not-a-uuid"),
            None,
        )
        .await;
    assert_eq!(error_code(&malformed_http), "invalid_request");
    assert_eq!(malformed_http["error"]["field"], USER_CONTEXT_HEADER);
    assert_eq!(malformed_http["error"]["reason"], "invalid_uuid");
    assert_eq!(
        mcp_error_code(&mcp_error(&malformed_mcp)),
        "invalid_request"
    );

    let user_context = user.id.0.to_string();
    let response = server
        .client
        .get(format!("{}/api/user", server.base_url))
        .header(USER_CONTEXT_HEADER, &user_context)
        .header(USER_CONTEXT_HEADER, &user_context)
        .send()
        .await
        .expect("duplicate-context request should send");
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let duplicate_http: Value = response.json().await.expect("error should be JSON");
    let (status, duplicate_mcp) = server
        .mcp_with_user_contexts(
            "tools/call",
            Some("get_user"),
            json!({"name": "get_user", "arguments": {}}),
            &[&user_context, &user_context],
            None,
        )
        .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(error_code(&duplicate_http), "invalid_request");
    assert_eq!(duplicate_http["error"]["field"], USER_CONTEXT_HEADER);
    assert_eq!(duplicate_http["error"]["reason"], "multiple_values");
    assert_eq!(mcp_error(&duplicate_mcp), duplicate_http);

    let comma_separated = format!("{user_context}, {user_context}");
    let response = server
        .client
        .get(format!("{}/api/user", server.base_url))
        .header(USER_CONTEXT_HEADER, comma_separated)
        .send()
        .await
        .expect("comma-separated context request should send");
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let comma_http: Value = response.json().await.expect("error should be JSON");
    assert_eq!(comma_http["error"]["reason"], "multiple_values");

    let unknown_user = Uuid::new_v4();
    let response = server
        .client
        .get(format!("{}/api/user", server.base_url))
        .header(USER_CONTEXT_HEADER, unknown_user.to_string())
        .send()
        .await
        .expect("unknown-context request should send");
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    let unknown_http: Value = response.json().await.expect("error should be JSON");
    let unknown_mcp = server.tool("get_user", json!({}), Some(unknown_user)).await;
    assert_eq!(error_code(&unknown_http), "user_not_found");
    assert_eq!(
        mcp_error_code(&mcp_error(&unknown_mcp)),
        error_code(&unknown_http)
    );

    let response = server
        .client
        .get(format!("{}/api/character", server.base_url))
        .header(USER_CONTEXT_HEADER, unknown_user.to_string())
        .send()
        .await
        .expect("unknown Character owner request should send");
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    let unknown_character_http: Value = response.json().await.expect("error should be JSON");
    let unknown_character_mcp = server
        .tool("get_character", json!({}), Some(unknown_user))
        .await;
    assert_eq!(error_code(&unknown_character_http), "user_not_found");
    assert_eq!(mcp_error(&unknown_character_mcp), unknown_character_http);

    let valid_entity = json!({"name": "Context Matrix", "description": "Not stored."});
    let response = server
        .client
        .post(format!("{}/api/entity", server.base_url))
        .json(&valid_entity)
        .send()
        .await
        .expect("missing create context should send");
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let missing_create_http: Value = response.json().await.expect("error should be JSON");
    let missing_create_mcp = server
        .tool("create_entity", valid_entity.clone(), None)
        .await;
    assert_eq!(mcp_error(&missing_create_mcp), missing_create_http);

    let response = server
        .client
        .post(format!("{}/api/entity", server.base_url))
        .header(USER_CONTEXT_HEADER, "not-a-uuid")
        .json(&valid_entity)
        .send()
        .await
        .expect("malformed create context should send");
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let malformed_create_http: Value = response.json().await.expect("error should be JSON");
    let (_, malformed_create_mcp) = server
        .mcp_with_user_context(
            "tools/call",
            Some("create_entity"),
            json!({"name": "create_entity", "arguments": valid_entity.clone()}),
            Some("not-a-uuid"),
            None,
        )
        .await;
    assert_eq!(mcp_error(&malformed_create_mcp), malformed_create_http);

    let response = server
        .client
        .post(format!("{}/api/entity", server.base_url))
        .header(USER_CONTEXT_HEADER, &user_context)
        .header(USER_CONTEXT_HEADER, &user_context)
        .json(&valid_entity)
        .send()
        .await
        .expect("duplicate create context should send");
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let duplicate_create_http: Value = response.json().await.expect("error should be JSON");
    let (_, duplicate_create_mcp) = server
        .mcp_with_user_contexts(
            "tools/call",
            Some("create_entity"),
            json!({"name": "create_entity", "arguments": valid_entity.clone()}),
            &[&user_context, &user_context],
            None,
        )
        .await;
    assert_eq!(mcp_error(&duplicate_create_mcp), duplicate_create_http);

    let response = server
        .client
        .post(format!("{}/api/entity", server.base_url))
        .header(USER_CONTEXT_HEADER, unknown_user.to_string())
        .json(&valid_entity)
        .send()
        .await
        .expect("unknown create context should send");
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    let unknown_create_http: Value = response.json().await.expect("error should be JSON");
    let unknown_create_mcp = server
        .tool("create_entity", valid_entity, Some(unknown_user))
        .await;
    assert_eq!(mcp_error(&unknown_create_mcp), unknown_create_http);

    let response = server
        .client
        .post(format!("{}/api/entity", server.base_url))
        .header(USER_CONTEXT_HEADER, user.id.0.to_string())
        .json(&json!({"name": "   ", "description": "Valid"}))
        .send()
        .await
        .expect("invalid Entity request should send");
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let invalid_http: Value = response.json().await.expect("error should be JSON");
    let invalid_mcp = server
        .tool(
            "create_entity",
            json!({"name": "   ", "description": "Valid"}),
            Some(user.id.0),
        )
        .await;
    assert_eq!(error_code(&invalid_http), "invalid_entity");
    assert_eq!(invalid_http["error"]["field"], "name");
    assert_eq!(invalid_http["error"]["reason"], "empty");
    assert_eq!(mcp_error(&invalid_mcp), invalid_http);

    let unknown_entity = Uuid::new_v4();
    let response = server
        .client
        .get(format!("{}/api/entity/{unknown_entity}", server.base_url))
        .send()
        .await
        .expect("unknown Entity request should send");
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    let unknown_entity_http: Value = response.json().await.expect("error should be JSON");
    assert_eq!(error_code(&unknown_entity_http), "entity_not_found");

    let response = server
        .client
        .get(format!(
            "{}/api/entity?cursor=not-a-cursor",
            server.base_url
        ))
        .send()
        .await
        .expect("bad cursor request should send");
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let bad_cursor_http: Value = response.json().await.expect("error should be JSON");
    assert_eq!(error_code(&bad_cursor_http), "invalid_request");

    for limit in [-1, 0, 101, 65_536] {
        let response = server
            .client
            .get(format!("{}/api/entity?limit={limit}", server.base_url))
            .send()
            .await
            .expect("out-of-range limit request should send");
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        let limit_http: Value = response.json().await.expect("error should be JSON");
        assert_eq!(error_code(&limit_http), "invalid_entity_limit");
        assert_eq!(limit_http["error"]["field"], "limit");
        assert_eq!(limit_http["error"]["reason"], "out_of_range");
    }

    for limit in [-1, 0, 101, 65_536] {
        let response = server
            .client
            .get(format!("{}/api/activity?limit={limit}", server.base_url))
            .header(USER_CONTEXT_HEADER, user.id.0.to_string())
            .send()
            .await
            .expect("out-of-range activity limit request should send");
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        let limit_http: Value = response.json().await.expect("error should be JSON");
        let limit_mcp = server
            .tool("list_activity", json!({"limit": limit}), Some(user.id.0))
            .await;
        assert_eq!(error_code(&limit_http), "invalid_activity_limit");
        assert_eq!(mcp_error(&limit_mcp), limit_http);
    }
}

#[sqlx::test(migrations = "./migration")]
async fn http_concurrency_preserves_single_genesis_and_retry_safe_entry(pool: PgPool) {
    let world = World::new(pool.clone());
    let first_user = world.create_user().await.unwrap();
    let second_user = world.create_user().await.unwrap();
    world
        .create_character(
            first_user.id,
            aicadia::CreateCharacter {
                name: "Mara Venn".to_owned(),
                description: "A careful surveyor.".to_owned(),
                property: Vec::new(),
            },
        )
        .await
        .unwrap();
    world
        .create_character(
            second_user.id,
            aicadia::CreateCharacter {
                name: "Tomas Reed".to_owned(),
                description: "A patient observer.".to_owned(),
                property: Vec::new(),
            },
        )
        .await
        .unwrap();
    let server = TestServer::start(world).await;
    let first_genesis = server
        .client
        .post(format!("{}/api/place/entry", server.base_url))
        .header(USER_CONTEXT_HEADER, first_user.id.0.to_string())
        .json(&json!({"name": "North Gate", "description": "First candidate."}))
        .send();
    let second_genesis = server
        .client
        .post(format!("{}/api/place/entry", server.base_url))
        .header(USER_CONTEXT_HEADER, second_user.id.0.to_string())
        .json(&json!({"name": "South Gate", "description": "Second candidate."}))
        .send();
    let (first_genesis, second_genesis) = tokio::join!(first_genesis, second_genesis);
    let status = [
        first_genesis.unwrap().status(),
        second_genesis.unwrap().status(),
    ];
    assert_eq!(
        status
            .iter()
            .filter(|status| **status == StatusCode::CREATED)
            .count(),
        1
    );
    assert_eq!(
        status
            .iter()
            .filter(|status| **status == StatusCode::CONFLICT)
            .count(),
        1
    );
    let place_count: i64 = sqlx::query_scalar("SELECT count(*) FROM place")
        .fetch_one(&pool)
        .await
        .unwrap();
    let entity_count: i64 = sqlx::query_scalar("SELECT count(*) FROM entity")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(place_count, 1);
    assert_eq!(entity_count, 3);

    let enter_url = format!("{}/api/world/entry", server.base_url);
    let first_entry = server
        .client
        .post(&enter_url)
        .header(USER_CONTEXT_HEADER, first_user.id.0.to_string())
        .send();
    let second_entry = server
        .client
        .post(&enter_url)
        .header(USER_CONTEXT_HEADER, first_user.id.0.to_string())
        .send();
    let (first_entry, second_entry) = tokio::join!(first_entry, second_entry);
    let first_entry = first_entry.unwrap();
    let second_entry = second_entry.unwrap();
    assert_eq!(first_entry.status(), StatusCode::OK);
    assert_eq!(second_entry.status(), StatusCode::OK);
    assert_eq!(
        first_entry.json::<Value>().await.unwrap(),
        second_entry.json::<Value>().await.unwrap()
    );
    let enter_count: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM activity WHERE operation = 'enter_world' AND requested_by_user_id = $1",
    )
    .bind(first_user.id.0)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(enter_count, 1);
}
