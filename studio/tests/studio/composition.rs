//! The root binary composition keeps the player server and read-only Studio on one
//! loopback process without mixing their surfaces.

use aicadia::{World, server};
use aicadia_studio as studio;
use reqwest::{Client, StatusCode};
use serde_json::Value;
use sqlx::PgPool;
use tokio::net::TcpListener;

#[sqlx::test(migrations = "../game/migration")]
async fn game_mcp_and_studio_share_one_router_without_sharing_capabilities(pool: PgPool) {
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("loopback listener should bind");
    let address = listener
        .local_addr()
        .expect("listener should have an address");
    let world = World::new(pool.clone());
    let router = server::app(world.clone(), address)
        .expect("game router should build")
        .merge(studio::app(world, pool));
    let task = tokio::spawn(async move {
        axum::serve(listener, router)
            .await
            .expect("combined application should serve");
    });
    let client = Client::new();
    let origin = format!("http://{address}");

    for path in ["/", "/game", "/dev", "/live", "/brief"] {
        let response = client
            .get(format!("{origin}{path}"))
            .send()
            .await
            .unwrap_or_else(|error| panic!("{path} should send: {error}"));
        assert_eq!(response.status(), StatusCode::OK, "{path} should respond");
    }

    let html = client
        .get(format!("{origin}/"))
        .send()
        .await
        .expect("Studio root should send")
        .text()
        .await
        .expect("Studio root should be text");
    assert!(html.contains("Aicadia Studio"));
    assert!(!html.contains("href=\"/mcp"));

    let response = client
        .post(format!("{origin}/mcp"))
        .header("Content-Type", "application/json")
        .header("Accept", "application/json, text/event-stream")
        .header("MCP-Protocol-Version", "2026-07-28")
        .header("Mcp-Method", "tools/list")
        .header("Origin", &origin)
        .json(&serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "tools/list",
            "params": {
                "_meta": {
                    "io.modelcontextprotocol/protocolVersion": "2026-07-28",
                    "io.modelcontextprotocol/clientInfo": {
                        "name": "aicadia-composition-test",
                        "version": "1"
                    },
                    "io.modelcontextprotocol/clientCapabilities": {}
                }
            }
        }))
        .send()
        .await
        .expect("MCP tools/list should send");
    assert_eq!(response.status(), StatusCode::OK);
    let body: Value = response.json().await.expect("MCP response should be JSON");
    let tools = body["result"]["tools"]
        .as_array()
        .expect("tools/list should return tools");
    assert_eq!(tools.len(), 19);
    assert!(tools.iter().all(|tool| {
        tool["name"]
            .as_str()
            .is_some_and(|name| !name.contains("studio") && !name.contains("dev"))
    }));

    task.abort();
}
