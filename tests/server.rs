use std::collections::BTreeSet;

use aicadia::{World, server, wire::USER_CONTEXT_HEADER};
use reqwest::{Client, StatusCode};
use serde_json::{Value, json};
use sqlx::PgPool;
use tokio::{net::TcpListener, task::JoinHandle};
use uuid::Uuid;

const PROTOCOL_VERSION: &str = "2026-07-28";
const LEGACY_PROTOCOL_VERSION: &str = "2025-11-25";
const CAPABILITY: [&str; 5] = [
    "get_world",
    "get_user",
    "list_entity",
    "get_entity",
    "create_entity",
];

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

    async fn legacy_tool(
        &self,
        session_id: &str,
        request_id: u64,
        name: &str,
        arguments: Value,
        user_id: Option<Uuid>,
    ) -> Value {
        let mut request = self
            .client
            .post(format!("{}/mcp", self.base_url))
            .header("Content-Type", "application/json")
            .header("Accept", "application/json, text/event-stream")
            .header("Mcp-Session-Id", session_id)
            .header("MCP-Protocol-Version", LEGACY_PROTOCOL_VERSION)
            .json(&json!({
                "jsonrpc": "2.0",
                "id": request_id,
                "method": "tools/call",
                "params": {
                    "name": name,
                    "arguments": arguments
                }
            }));
        if let Some(user_id) = user_id {
            request = request.header(USER_CONTEXT_HEADER, user_id.to_string());
        }

        let response = request.send().await.expect("legacy tool call should send");
        assert_eq!(
            response.status(),
            StatusCode::OK,
            "unexpected legacy MCP response for {name}"
        );
        sse_json(response).await
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

async fn sse_json(response: reqwest::Response) -> Value {
    let body = response
        .text()
        .await
        .expect("MCP SSE response body should be readable");
    let data = body
        .lines()
        .filter_map(|line| line.strip_prefix("data: "))
        .find(|data| !data.is_empty())
        .unwrap_or_else(|| panic!("MCP SSE response should contain JSON data: {body}"));
    serde_json::from_str(data).expect("MCP SSE data should be JSON")
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

#[sqlx::test(migrations = "./migration")]
async fn catalogs_expose_exactly_the_five_player_capabilities(pool: PgPool) {
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
    assert_eq!(
        openapi["paths"]["/api/entity"]["post"]["responses"]["201"]["description"],
        "Created Entity"
    );
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
        json!([LEGACY_PROTOCOL_VERSION, PROTOCOL_VERSION])
    );
    assert_eq!(discover["result"]["capabilities"], json!({"tools": {}}));

    let (status, listed) = server
        .mcp("tools/list", None, json!({}), None, Some(&server.origin))
        .await;
    assert_eq!(status, StatusCode::OK, "unexpected tools/list: {listed}");
    assert_eq!(listed["result"]["cacheScope"], "public");
    assert_eq!(listed["result"]["ttlMs"], 0);
    let tools = listed["result"]["tools"]
        .as_array()
        .expect("tools/list should return an array");
    let expected_tools: Value = serde_json::from_str(include_str!("agent-tool-catalog.json"))
        .expect("catalog contract fixture should be valid JSON");
    assert_eq!(Value::Array(tools.clone()), expected_tools);

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
async fn legacy_mcp_session_supports_all_player_capabilities(pool: PgPool) {
    let world = World::new(pool);
    let user = world.create_user().await.expect("setup User should exist");
    let server = TestServer::start(world).await;

    let initialize = server
        .mcp_raw_response(
            json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": "initialize",
                "params": {
                    "protocolVersion": LEGACY_PROTOCOL_VERSION,
                    "capabilities": {},
                    "clientInfo": {
                        "name": "aicadia-legacy-test",
                        "version": "0.1.0"
                    }
                }
            }),
            &[],
        )
        .await;
    assert_eq!(initialize.status(), StatusCode::OK);
    let session_id = initialize
        .headers()
        .get("Mcp-Session-Id")
        .and_then(|value| value.to_str().ok())
        .expect("legacy initialize should create a transport session")
        .to_owned();
    let initialized = sse_json(initialize).await;
    assert_eq!(
        initialized["result"]["protocolVersion"],
        LEGACY_PROTOCOL_VERSION
    );

    let notification = server
        .mcp_raw_response(
            json!({
                "jsonrpc": "2.0",
                "method": "notifications/initialized"
            }),
            &[
                ("Mcp-Session-Id", &session_id),
                ("MCP-Protocol-Version", LEGACY_PROTOCOL_VERSION),
            ],
        )
        .await;
    assert_eq!(notification.status(), StatusCode::ACCEPTED);

    let listed = server
        .mcp_raw_response(
            json!({
                "jsonrpc": "2.0",
                "id": 2,
                "method": "tools/list",
                "params": {}
            }),
            &[
                ("Mcp-Session-Id", &session_id),
                ("MCP-Protocol-Version", LEGACY_PROTOCOL_VERSION),
            ],
        )
        .await;
    assert_eq!(listed.status(), StatusCode::OK);
    let listed = sse_json(listed).await;
    assert_eq!(
        listed["result"]["tools"]
            .as_array()
            .expect("legacy tools/list should return an array")
            .iter()
            .map(|tool| tool["name"].as_str().expect("tool should have a name"))
            .collect::<Vec<_>>(),
        CAPABILITY
    );
    assert!(listed["result"].get("cacheScope").is_none());
    assert!(listed["result"].get("ttlMs").is_none());

    let world = server
        .legacy_tool(&session_id, 3, "get_world", json!({}), None)
        .await;
    assert_eq!(structured(&world), &json!({"name": "Aicadia"}));

    let read_user = server
        .legacy_tool(&session_id, 4, "get_user", json!({}), Some(user.id.0))
        .await;
    assert_eq!(structured(&read_user)["id"], user.id.0.to_string());
    assert!(structured(&read_user)["created_at"].is_string());

    let before_create = server
        .legacy_tool(&session_id, 5, "list_entity", json!({}), None)
        .await;
    assert_eq!(structured(&before_create)["entity"], json!([]));
    assert_eq!(structured(&before_create)["next"], Value::Null);

    let created = server
        .legacy_tool(
            &session_id,
            6,
            "create_entity",
            json!({
                "name": "Legacy Waystone",
                "description": "A marker introduced through a legacy MCP transport session."
            }),
            Some(user.id.0),
        )
        .await;
    let created = structured(&created);
    assert_eq!(created["name"], "Legacy Waystone");
    assert_eq!(
        created["description"],
        "A marker introduced through a legacy MCP transport session."
    );
    assert_eq!(created["introduced_by_user_id"], user.id.0.to_string());
    let entity_id = created["id"]
        .as_str()
        .expect("created Entity should have an id")
        .to_owned();

    let read_entity = server
        .legacy_tool(
            &session_id,
            7,
            "get_entity",
            json!({"entity_id": entity_id}),
            None,
        )
        .await;
    assert_eq!(structured(&read_entity), created);

    let after_create = server
        .legacy_tool(&session_id, 8, "list_entity", json!({}), None)
        .await;
    assert_eq!(
        structured(&after_create)["entity"],
        json!([{
            "id": entity_id,
            "name": "Legacy Waystone"
        }])
    );
    assert_eq!(structured(&after_create)["next"], Value::Null);
}

#[sqlx::test(migrations = "./migration")]
async fn current_mcp_remains_stateless_and_requires_per_request_metadata(pool: PgPool) {
    let server = TestServer::start(World::new(pool)).await;
    let mut params = json!({});
    params
        .as_object_mut()
        .expect("MCP params should be an object")
        .insert("_meta".to_owned(), request_meta());

    let response = server
        .mcp_raw_response(
            json!({
                "jsonrpc": "2.0",
                "id": 1,
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
                "id": 2,
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
    let mcp_read = server
        .tool("get_entity", json!({"entity_id": http_created["id"]}), None)
        .await;
    assert_eq!(http_created, *structured(&mcp_read));

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
    let second_page = server
        .tool("list_entity", json!({"cursor": cursor, "limit": 1}), None)
        .await;
    assert_ne!(
        first_page["entity"][0]["id"],
        structured(&second_page)["entity"][0]["id"]
    );
}

#[sqlx::test(migrations = "./migration")]
async fn http_and_mcp_share_canonical_capability_errors(pool: PgPool) {
    let world = World::new(pool);
    let user = world.create_user().await.expect("setup User should exist");
    let server = TestServer::start(world).await;

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
    let unknown_entity_mcp = server
        .tool("get_entity", json!({"entity_id": unknown_entity}), None)
        .await;
    assert_eq!(error_code(&unknown_entity_http), "entity_not_found");
    assert_eq!(
        mcp_error_code(&mcp_error(&unknown_entity_mcp)),
        error_code(&unknown_entity_http)
    );

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
    let bad_cursor_mcp = server
        .tool(
            "list_entity",
            json!({"cursor": "not-a-cursor", "limit": 25}),
            None,
        )
        .await;
    assert_eq!(error_code(&bad_cursor_http), "invalid_request");
    assert_eq!(mcp_error(&bad_cursor_mcp), bad_cursor_http);

    for limit in [-1, 0, 101, 65_536] {
        let response = server
            .client
            .get(format!("{}/api/entity?limit={limit}", server.base_url))
            .send()
            .await
            .expect("out-of-range limit request should send");
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        let limit_http: Value = response.json().await.expect("error should be JSON");
        let limit_mcp = server
            .tool("list_entity", json!({"limit": limit}), None)
            .await;
        assert_eq!(error_code(&limit_http), "invalid_entity_limit");
        assert_eq!(limit_http["error"]["field"], "limit");
        assert_eq!(limit_http["error"]["reason"], "out_of_range");
        assert_eq!(mcp_error(&limit_mcp), limit_http);
    }
}
