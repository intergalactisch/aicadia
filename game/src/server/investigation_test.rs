use super::*;

use reqwest::{Client, StatusCode as ClientStatus};
use serde_json::{Value, json};
use sqlx::PgPool;
use tokio::{net::TcpListener, task::JoinHandle};
use uuid::Uuid;

use crate::{
    CreateCharacter, CreateEntity, CreateEntryPlace, PropertyInput, PropertyValue, TraitInput,
    UserId, wire::DiscoveryFindInput,
};

struct TestServer {
    base_url: String,
    client: Client,
    task: JoinHandle<()>,
}

impl TestServer {
    async fn start(world: World) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let router = app(world, address).unwrap();
        let task = tokio::spawn(async move { axum::serve(listener, router).await.unwrap() });
        Self {
            base_url: format!("http://{address}"),
            client: Client::new(),
            task,
        }
    }

    async fn tool(&self, name: &str, arguments: Value, user_id: UserId) -> Value {
        let response = self
            .client
            .post(format!("{}/mcp", self.base_url))
            .header("Content-Type", "application/json")
            .header("Accept", "application/json, text/event-stream")
            .header("MCP-Protocol-Version", "2026-07-28")
            .header("Mcp-Method", "tools/call")
            .header("Mcp-Name", name)
            .header(USER_CONTEXT_HEADER, user_id.0.to_string())
            .json(&json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": "tools/call",
                "params": {
                    "name": name,
                    "arguments": arguments,
                    "_meta": {
                        "io.modelcontextprotocol/protocolVersion": "2026-07-28",
                        "io.modelcontextprotocol/clientInfo": {
                            "name": "aicadia-investigation-test",
                            "version": "0.1.0"
                        },
                        "io.modelcontextprotocol/clientCapabilities": {}
                    }
                }
            }))
            .send()
            .await
            .unwrap();
        assert_eq!(response.status(), ClientStatus::OK);
        response.json().await.unwrap()
    }

    async fn post(&self, path: &str, body: &Value, user_id: UserId) -> (ClientStatus, Value) {
        let response = self
            .client
            .post(format!("{}{path}", self.base_url))
            .header(USER_CONTEXT_HEADER, user_id.0.to_string())
            .json(body)
            .send()
            .await
            .unwrap();
        (response.status(), response.json().await.unwrap())
    }
}

impl Drop for TestServer {
    fn drop(&mut self) {
        self.task.abort();
    }
}

async fn entered_world(pool: PgPool, draw: Vec<f64>) -> (World, UserId, UserId) {
    let world = World::with_scripted_chance(pool, draw);
    let actor = world.create_user().await.unwrap();
    let observer = world.create_user().await.unwrap();
    for (user_id, name) in [(actor.id, "Mara Venn"), (observer.id, "Tomas Reed")] {
        world
            .create_character(
                user_id,
                CreateCharacter {
                    name: name.to_owned(),
                    description: "A careful observer of the shared World.".to_owned(),
                    property: Vec::new(),
                    r#trait: Vec::new(),
                },
            )
            .await
            .unwrap();
    }
    world
        .create_entry_place(
            actor.id,
            CreateEntryPlace {
                name: "North Gate".to_owned(),
                description: "A wind-worn stone gate.".to_owned(),
                property: Vec::new(),
                r#trait: Vec::new(),
            },
        )
        .await
        .unwrap();
    world.enter_world(actor.id).await.unwrap();
    world.enter_world(observer.id).await.unwrap();
    (world, actor.id, observer.id)
}

fn discovery(request_id: Uuid, attempt_id: &Value, prose: &str) -> Value {
    json!({
        "request_id": request_id,
        "attempt_id": attempt_id,
        "prose": prose,
        "find": {
            "name": "Rainbell Cups",
            "description": "Chalk-pale cups whose thin rims ring in rain.",
            "property": [{
                "key": "colour",
                "value": {"type": "text", "text": "chalk-pale"}
            }],
            "trait": [{"statement": "Rings softly when collected rain shifts."}]
        }
    })
}

fn structured(response: &Value) -> &Value {
    &response["result"]["structuredContent"]
}

fn mcp_error(response: &Value) -> Value {
    let text = response["result"]["content"][0]["text"].as_str().unwrap();
    serde_json::from_str(text).unwrap()
}

fn mcp_error_code(response: &Value) -> String {
    mcp_error(response)["error"]["code"]
        .as_str()
        .unwrap()
        .to_owned()
}

/// Rejects one discovery body on both adapters and proves they publish the same
/// status class and the byte-equal error envelope.
async fn assert_discovery_rejected(
    server: &TestServer,
    actor: UserId,
    body: Value,
    expected_status: ClientStatus,
    expected_code: &str,
) {
    let (status, http) = server.post("/api/discovery", &body, actor).await;
    assert_eq!(
        status, expected_status,
        "unexpected discovery status: {http}"
    );
    assert_eq!(http["error"]["code"], expected_code);
    let mcp = server.tool("submit_discovery", body, actor).await;
    assert_eq!(mcp_error(&mcp), http);
}

#[sqlx::test(migrations = "./migration")]
async fn deterministic_http_and_mcp_share_start_submit_retry_and_observer_results(pool: PgPool) {
    let (world, actor, observer) = entered_world(pool, vec![0.99, 0.0]).await;
    let server = TestServer::start(world).await;

    let zero_id = Uuid::new_v4();
    let response = server
        .client
        .post(format!("{}/api/investigation", server.base_url))
        .header(USER_CONTEXT_HEADER, actor.0.to_string())
        .json(&json!({"request_id": zero_id}))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), ClientStatus::OK);
    let zero: Value = response.json().await.unwrap();
    assert_eq!(zero["outcome"], "zero");
    assert_eq!(
        zero["limit"],
        json!({"result_count": 1, "kind": "entity_at_current_place"})
    );
    let zero_retry = server
        .tool("start_investigation", json!({"request_id": zero_id}), actor)
        .await;
    assert_eq!(structured(&zero_retry), &zero);

    let positive_id = Uuid::new_v4();
    let positive = server
        .tool(
            "start_investigation",
            json!({"request_id": positive_id}),
            actor,
        )
        .await;
    let positive = structured(&positive).clone();
    assert_eq!(positive["outcome"], "positive");
    let response = server
        .client
        .post(format!("{}/api/investigation", server.base_url))
        .header(USER_CONTEXT_HEADER, actor.0.to_string())
        .json(&json!({"request_id": positive_id}))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), ClientStatus::OK);
    assert_eq!(response.json::<Value>().await.unwrap(), positive);

    let character = server.tool("get_character", json!({}), actor).await;
    let character = structured(&character);
    let orientation = server
        .tool("list_entity_at_current_place", json!({"limit": 100}), actor)
        .await;
    let orientation = structured(&orientation);
    let activity = server
        .tool(
            "list_activity_at_current_place",
            json!({"limit": 100}),
            actor,
        )
        .await;
    let activity = structured(&activity);
    assert_eq!(character["place_revision"], orientation["place_revision"]);
    assert_eq!(activity["place_revision"], orientation["place_revision"]);
    let actor_state = server
        .tool(
            "get_entity_at_current_place",
            json!({"entity_id": character["character"]["entity"]["id"]}),
            actor,
        )
        .await;
    assert_eq!(
        structured(&actor_state)["place_revision"],
        orientation["place_revision"]
    );

    let request_id = Uuid::new_v4();
    let body = discovery(
        request_id,
        &positive["attempt_id"],
        "Mara parts the reeds and finds rainbell cups.",
    );
    let response = server
        .client
        .post(format!("{}/api/discovery", server.base_url))
        .header(USER_CONTEXT_HEADER, actor.0.to_string())
        .json(&body)
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), ClientStatus::CREATED);
    let accepted: Value = response.json().await.unwrap();
    assert_eq!(accepted["activity"]["operation"], "submit_discovery");
    assert_eq!(accepted["activity"]["prose"], body["prose"]);
    assert_eq!(accepted["entity"]["name"], body["find"]["name"]);
    let retry = server.tool("submit_discovery", body.clone(), actor).await;
    assert_eq!(structured(&retry), &accepted);

    let entity_id = accepted["entity"]["id"].clone();
    let observed = server
        .tool(
            "get_entity_at_current_place",
            json!({"entity_id": entity_id}),
            observer,
        )
        .await;
    let observed = structured(&observed);
    assert_eq!(observed["entity"]["name"], "Rainbell Cups");
    assert_eq!(
        observed["current_state"]["association"]
            .as_array()
            .unwrap()
            .len(),
        2
    );
    let history = server
        .tool(
            "list_activity_at_current_place",
            json!({"limit": 100}),
            observer,
        )
        .await;
    assert!(
        structured(&history)["activity"]
            .as_array()
            .unwrap()
            .iter()
            .any(|activity| activity["id"] == accepted["activity"]["id"])
    );

    let conflict = server
        .client
        .post(format!("{}/api/discovery", server.base_url))
        .header(USER_CONTEXT_HEADER, actor.0.to_string())
        .json(&discovery(
            request_id,
            &positive["attempt_id"],
            "Changed prose.",
        ))
        .send()
        .await
        .unwrap();
    assert_eq!(conflict.status(), ClientStatus::CONFLICT);
    let conflict: Value = conflict.json().await.unwrap();
    assert_eq!(conflict["error"]["code"], "discovery_request_conflict");

    let unavailable = server
        .tool(
            "submit_discovery",
            discovery(Uuid::new_v4(), &positive["attempt_id"], "Another request."),
            actor,
        )
        .await;
    assert_eq!(
        mcp_error_code(&unavailable),
        "discovery_attempt_unavailable"
    );
}

#[sqlx::test(migrations = "./migration")]
async fn investigation_http_maps_admission_and_entropy_unavailable(pool: PgPool) {
    let (world, actor, observer) = entered_world(pool, vec![0.99; 12]).await;
    let server = TestServer::start(world).await;
    for _ in 0..12 {
        let response = server
            .client
            .post(format!("{}/api/investigation", server.base_url))
            .header(USER_CONTEXT_HEADER, actor.0.to_string())
            .json(&json!({"request_id": Uuid::new_v4()}))
            .send()
            .await
            .unwrap();
        assert_eq!(response.status(), ClientStatus::OK);
    }
    let response = server
        .client
        .post(format!("{}/api/investigation", server.base_url))
        .header(USER_CONTEXT_HEADER, actor.0.to_string())
        .json(&json!({"request_id": Uuid::new_v4()}))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), ClientStatus::TOO_MANY_REQUESTS);
    let body: Value = response.json().await.unwrap();
    assert_eq!(body["error"]["code"], "investigation_not_admitted");

    let other = server
        .tool(
            "start_investigation",
            json!({"request_id": Uuid::new_v4()}),
            actor,
        )
        .await;
    assert_eq!(mcp_error_code(&other), "investigation_not_admitted");

    let response = server
        .client
        .post(format!("{}/api/investigation", server.base_url))
        .header(USER_CONTEXT_HEADER, observer.0.to_string())
        .json(&json!({"request_id": Uuid::new_v4()}))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), ClientStatus::SERVICE_UNAVAILABLE);
    let body: Value = response.json().await.unwrap();
    assert_eq!(body["error"]["code"], "unavailable");

    let unavailable = server
        .tool(
            "start_investigation",
            json!({"request_id": Uuid::new_v4()}),
            observer,
        )
        .await;
    assert_eq!(mcp_error_code(&unavailable), "unavailable");
}

#[sqlx::test(migrations = "./migration")]
async fn http_and_mcp_share_every_published_discovery_rejection(pool: PgPool) {
    let (world, actor, _observer) = entered_world(pool, vec![0.0]).await;
    world
        .create_entity(
            actor,
            CreateEntity {
                name: "Survey Marker".to_owned(),
                description: "A notched post left by an earlier survey.".to_owned(),
                property: vec![PropertyInput {
                    key: "measure".to_owned(),
                    value: PropertyValue::Integer(2),
                }],
                r#trait: Vec::new(),
            },
        )
        .await
        .unwrap();
    let server = TestServer::start(world).await;

    let started = server
        .tool(
            "start_investigation",
            json!({"request_id": Uuid::new_v4()}),
            actor,
        )
        .await;
    let started = structured(&started).clone();
    assert_eq!(started["outcome"], "positive");
    let attempt_id = started["attempt_id"].clone();

    let empty_prose = discovery(Uuid::new_v4(), &attempt_id, "   ");
    assert_discovery_rejected(
        &server,
        actor,
        empty_prose,
        ClientStatus::BAD_REQUEST,
        "invalid_discovery",
    )
    .await;

    let mut empty_name = discovery(Uuid::new_v4(), &attempt_id, "Mara finds nothing nameable.");
    empty_name["find"]["name"] = json!("   ");
    assert_discovery_rejected(
        &server,
        actor,
        empty_name,
        ClientStatus::BAD_REQUEST,
        "invalid_entity",
    )
    .await;

    let mut bad_key = discovery(
        Uuid::new_v4(),
        &attempt_id,
        "Mara finds an unnameable quality.",
    );
    bad_key["find"]["property"][0]["key"] = json!("Colour");
    assert_discovery_rejected(
        &server,
        actor,
        bad_key,
        ClientStatus::BAD_REQUEST,
        "invalid_property",
    )
    .await;

    let mut empty_statement = discovery(Uuid::new_v4(), &attempt_id, "Mara finds a silent cup.");
    empty_statement["find"]["trait"][0]["statement"] = json!("   ");
    assert_discovery_rejected(
        &server,
        actor,
        empty_statement,
        ClientStatus::BAD_REQUEST,
        "invalid_trait",
    )
    .await;

    let mut key_conflict = discovery(Uuid::new_v4(), &attempt_id, "Mara finds a contradiction.");
    key_conflict["find"]["property"][0] =
        json!({"key": "measure", "value": {"type": "text", "text": "two"}});
    assert_discovery_rejected(
        &server,
        actor,
        key_conflict,
        ClientStatus::CONFLICT,
        "property_key_conflict",
    )
    .await;

    let request_id = Uuid::new_v4();
    let accepted_body = discovery(
        request_id,
        &attempt_id,
        "Mara parts the reeds and finds rainbell cups.",
    );
    let (status, accepted) = server.post("/api/discovery", &accepted_body, actor).await;
    assert_eq!(
        status,
        ClientStatus::CREATED,
        "unexpected acceptance: {accepted}"
    );
    let (status, retried) = server.post("/api/discovery", &accepted_body, actor).await;
    assert_eq!(status, ClientStatus::CREATED, "unexpected retry: {retried}");
    assert_eq!(retried, accepted);

    let changed = discovery(request_id, &attempt_id, "Changed prose.");
    assert_discovery_rejected(
        &server,
        actor,
        changed,
        ClientStatus::CONFLICT,
        "discovery_request_conflict",
    )
    .await;

    let consumed = discovery(Uuid::new_v4(), &attempt_id, "Mara reuses a spent attempt.");
    assert_discovery_rejected(
        &server,
        actor,
        consumed,
        ClientStatus::CONFLICT,
        "discovery_attempt_unavailable",
    )
    .await;
}

#[test]
fn investigation_wire_conversions_preserve_typed_initial_state() {
    let input = SubmitDiscoveryInput {
        request_id: Uuid::new_v4(),
        attempt_id: Uuid::new_v4(),
        prose: "A discovery.".to_owned(),
        find: DiscoveryFindInput {
            name: "Rainbell Cups".to_owned(),
            description: "Pale cups.".to_owned(),
            property: vec![crate::wire::PropertyInput {
                key: "height".to_owned(),
                value: crate::wire::PropertyValueInput::Integer { integer: 2 },
            }],
            r#trait: vec![crate::wire::TraitInput {
                statement: "Rings in rain.".to_owned(),
            }],
        },
    };
    let parsed: crate::SubmitDiscovery = input.into();
    let crate::DiscoveryResultInput::EntityAtPosition {
        property, r#trait, ..
    } = parsed.result
    else {
        unreachable!("the legacy adapter creates only entity_at_position")
    };
    assert_eq!(
        property,
        vec![PropertyInput {
            key: "height".to_owned(),
            value: PropertyValue::Integer(2),
        }]
    );
    assert_eq!(
        r#trait,
        vec![TraitInput {
            statement: "Rings in rain.".to_owned(),
        }]
    );
}
