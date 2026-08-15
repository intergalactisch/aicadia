mod action;
mod concurrency;
mod entity_state_creation;
mod entity_trait;
mod error;
mod interaction;
mod property;
mod protocol;

use std::collections::BTreeSet;

use aicadia::{World, server, wire::USER_CONTEXT_HEADER};
use reqwest::{Client, StatusCode};
use serde_json::{Value, json};
use sqlx::PgPool;
use tokio::{net::TcpListener, task::JoinHandle};
use uuid::Uuid;

const PROTOCOL_VERSION: &str = "2026-07-28";
const CAPABILITY: [&str; 15] = [
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
];

static MCP_INSTRUCTIONS: std::sync::LazyLock<&'static str> =
    std::sync::LazyLock::new(aicadia::agent_contract::instructions);
const MCP_TOOL_CATALOG: &str = include_str!("../agent-tool-catalog.json");

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
