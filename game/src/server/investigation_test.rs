use super::*;

use reqwest::{Client, StatusCode as ClientStatus};
use serde_json::{Value, json};
use sqlx::PgPool;
use tokio::{net::TcpListener, task::JoinHandle};
use uuid::Uuid;

use crate::{
    CreateCharacter, CreateEntity, CreateEntryPlace, PropertyInput, PropertyValue, TraitInput,
    UserId, wire::DiscoveryResultInputWire,
};

pub(super) struct TestServer {
    base_url: String,
    client: Client,
    task: JoinHandle<()>,
}

impl TestServer {
    pub(super) async fn start(world: World) -> Self {
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

    pub(super) async fn tool(&self, name: &str, arguments: Value, user_id: UserId) -> Value {
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

    pub(super) async fn post(
        &self,
        path: &str,
        body: &Value,
        user_id: UserId,
    ) -> (ClientStatus, Value) {
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

    pub(super) async fn get(&self, path: &str, user_id: UserId) -> (ClientStatus, Value) {
        let response = self
            .client
            .get(format!("{}{path}", self.base_url))
            .header(USER_CONTEXT_HEADER, user_id.0.to_string())
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

pub(super) async fn entered_world(pool: PgPool, draw: Vec<f64>) -> (World, UserId, UserId) {
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
        "result": {
            "type": "entity_at_position",
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

pub(super) fn structured(response: &Value) -> &Value {
    &response["result"]["structuredContent"]
}

pub(super) fn mcp_error(response: &Value) -> Value {
    let text = response["result"]["content"][0]["text"].as_str().unwrap();
    serde_json::from_str(text).unwrap()
}

fn mcp_error_code(response: &Value) -> String {
    mcp_error(response)["error"]["code"]
        .as_str()
        .unwrap()
        .to_owned()
}

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
async fn assert_get_rejected(
    server: &TestServer,
    actor: UserId,
    path: &str,
    tool: &str,
    input: Value,
    expected_status: ClientStatus,
    expected_code: &str,
) -> Value {
    let (status, http) = server.get(path, actor).await;
    assert_eq!(status, expected_status, "unexpected GET rejection: {http}");
    assert_eq!(http["error"]["code"], expected_code);
    let mcp = server.tool(tool, input, actor).await;
    assert_eq!(mcp_error(&mcp), http);
    http
}

async fn assert_post_rejected(
    server: &TestServer,
    actor: UserId,
    path: &str,
    tool: &str,
    input: Value,
    expected_status: ClientStatus,
    expected_code: &str,
) -> Value {
    let (status, http) = server.post(path, &input, actor).await;
    assert_eq!(
        status, expected_status,
        "expected {expected_code}, received: {http}"
    );
    assert_eq!(http["error"]["code"], expected_code);
    let mcp = server.tool(tool, input, actor).await;
    assert_eq!(mcp_error(&mcp), http);
    http
}

async fn start_positive(
    server: &TestServer,
    actor: UserId,
    kind: &str,
    through_http: bool,
) -> Value {
    let input = json!({"request_id": Uuid::new_v4(), "kind": kind});
    let result = if through_http {
        let (status, result) = server.post("/api/investigation", &input, actor).await;
        assert_eq!(
            status,
            ClientStatus::OK,
            "unexpected investigation: {result}"
        );
        result
    } else {
        structured(&server.tool("start_investigation", input, actor).await).clone()
    };
    assert_eq!(result["outcome"], "positive");
    result
}

fn connected_discovery(
    request_id: Uuid,
    attempt_id: &Value,
    origin: Value,
    destination_name: &str,
    destination_x_cm: i64,
    allows_reverse: bool,
    course: Value,
) -> Value {
    json!({
        "request_id": request_id,
        "attempt_id": attempt_id,
        "prose": format!("Mara charts {destination_name}."),
        "result": {
            "type": "connected_place",
            "origin": origin,
            "destination": {
                "type": "new",
                "entity": {
                    "name": destination_name,
                    "description": format!("The discovered place called {destination_name}."),
                    "property": [],
                    "trait": []
                },
                "position": {"x_cm": destination_x_cm, "y_cm": 0, "z_cm": 0}
            },
            "connection": {
                "name": format!("{destination_name} Path"),
                "description": format!("A direct path toward {destination_name}."),
                "allows_reverse": allows_reverse,
                "course": course
            }
        }
    })
}

pub(super) async fn activity_count(pool: &PgPool) -> i64 {
    sqlx::query_scalar("SELECT count(*) FROM activity")
        .fetch_one(pool)
        .await
        .unwrap()
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
        .json(&json!({"request_id": zero_id, "kind": "entity_at_position"}))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), ClientStatus::OK);
    let zero: Value = response.json().await.unwrap();
    assert_eq!(zero["outcome"], "zero");
    assert_eq!(
        zero["limit"],
        json!({"result_count": 1, "kind": "entity_at_position"})
    );
    let zero_retry = server
        .tool(
            "start_investigation",
            json!({"request_id": zero_id, "kind": "entity_at_position"}),
            actor,
        )
        .await;
    assert_eq!(structured(&zero_retry), &zero);

    let positive_id = Uuid::new_v4();
    let positive = server
        .tool(
            "start_investigation",
            json!({"request_id": positive_id, "kind": "entity_at_position"}),
            actor,
        )
        .await;
    let positive = structured(&positive).clone();
    assert_eq!(positive["outcome"], "positive");
    let response = server
        .client
        .post(format!("{}/api/investigation", server.base_url))
        .header(USER_CONTEXT_HEADER, actor.0.to_string())
        .json(&json!({"request_id": positive_id, "kind": "entity_at_position"}))
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
    assert_eq!(accepted["entity"]["name"], body["result"]["name"]);
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
            .json(&json!({"request_id": Uuid::new_v4(), "kind": "entity_at_position"}))
            .send()
            .await
            .unwrap();
        assert_eq!(response.status(), ClientStatus::OK);
    }
    let response = server
        .client
        .post(format!("{}/api/investigation", server.base_url))
        .header(USER_CONTEXT_HEADER, actor.0.to_string())
        .json(&json!({"request_id": Uuid::new_v4(), "kind": "entity_at_position"}))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), ClientStatus::TOO_MANY_REQUESTS);
    let body: Value = response.json().await.unwrap();
    assert_eq!(body["error"]["code"], "investigation_not_admitted");

    let other = server
        .tool(
            "start_investigation",
            json!({"request_id": Uuid::new_v4(), "kind": "entity_at_position"}),
            actor,
        )
        .await;
    assert_eq!(mcp_error(&other), body);

    let response = server
        .client
        .post(format!("{}/api/investigation", server.base_url))
        .header(USER_CONTEXT_HEADER, observer.0.to_string())
        .json(&json!({"request_id": Uuid::new_v4(), "kind": "entity_at_position"}))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), ClientStatus::SERVICE_UNAVAILABLE);
    let body: Value = response.json().await.unwrap();
    assert_eq!(body["error"]["code"], "unavailable");

    let unavailable = server
        .tool(
            "start_investigation",
            json!({"request_id": Uuid::new_v4(), "kind": "entity_at_position"}),
            observer,
        )
        .await;
    assert_eq!(mcp_error(&unavailable), body);
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
            json!({"request_id": Uuid::new_v4(), "kind": "entity_at_position"}),
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
    empty_name["result"]["name"] = json!("   ");
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
    bad_key["result"]["property"][0]["key"] = json!("Colour");
    assert_discovery_rejected(
        &server,
        actor,
        bad_key,
        ClientStatus::BAD_REQUEST,
        "invalid_property",
    )
    .await;

    let mut empty_statement = discovery(Uuid::new_v4(), &attempt_id, "Mara finds a silent cup.");
    empty_statement["result"]["trait"][0]["statement"] = json!("   ");
    assert_discovery_rejected(
        &server,
        actor,
        empty_statement,
        ClientStatus::BAD_REQUEST,
        "invalid_trait",
    )
    .await;

    let mut key_conflict = discovery(Uuid::new_v4(), &attempt_id, "Mara finds a contradiction.");
    key_conflict["result"]["property"][0] =
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
        result: DiscoveryResultInputWire::EntityAtPosition {
            name: "Rainbell Cups".to_owned(),
            description: "Pale cups.".to_owned(),
            position_description: None,
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
        panic!("the tagged adapter returned connected_place")
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

#[sqlx::test(migrations = "./migration")]
async fn spatial_read_adapters_share_success_bounds_cursors_selection_and_no_history(pool: PgPool) {
    let readback = pool.clone();
    let (world, actor, _observer) = entered_world(pool, vec![0.0, 0.0]).await;
    let server = TestServer::start(world).await;

    let first = start_positive(&server, actor, "connected_place", true).await;
    let first_input = connected_discovery(
        Uuid::new_v4(),
        &first["attempt_id"],
        json!({"type": "attempt_place"}),
        "Bell Meadow",
        200,
        true,
        json!([
            {"x_cm": 0, "y_cm": 0, "z_cm": 0},
            {"x_cm": 200, "y_cm": 0, "z_cm": 0}
        ]),
    );
    let (status, first_discovery) = server.post("/api/discovery", &first_input, actor).await;
    assert_eq!(status, ClientStatus::CREATED);
    assert_eq!(
        structured(
            &server
                .tool("submit_discovery", first_input.clone(), actor)
                .await
        ),
        &first_discovery
    );

    let second = start_positive(&server, actor, "connected_place", false).await;
    let second_input = connected_discovery(
        Uuid::new_v4(),
        &second["attempt_id"],
        json!({"type": "attempt_place"}),
        "Cedar Rise",
        400,
        true,
        json!([
            {"x_cm": 0, "y_cm": 0, "z_cm": 0},
            {"x_cm": 400, "y_cm": 0, "z_cm": 0}
        ]),
    );
    let second_discovery = structured(
        &server
            .tool("submit_discovery", second_input.clone(), actor)
            .await,
    )
    .clone();
    let (status, second_retry) = server.post("/api/discovery", &second_input, actor).await;
    assert_eq!(status, ClientStatus::CREATED);
    assert_eq!(second_retry, second_discovery);

    let origin_id = first_discovery["origin"]["entity"]["id"].as_str().unwrap();
    let first_destination_id = first_discovery["destination"]["entity"]["id"]
        .as_str()
        .unwrap();
    let second_destination_id = second_discovery["destination"]["entity"]["id"]
        .as_str()
        .unwrap();
    let first_connection_id = first_discovery["connection"]["id"].as_str().unwrap();
    let before_reads = activity_count(&readback).await;

    let place_input = json!({
        "min_x_cm": 0, "max_x_cm": 500, "min_y_cm": 0, "max_y_cm": 0,
        "min_z_cm": 0, "max_z_cm": 0, "limit": 1
    });
    let (status, first_place_page) = server
        .get("/api/place?min_x_cm=0&max_x_cm=500&min_y_cm=0&max_y_cm=0&min_z_cm=0&max_z_cm=0&limit=1", actor)
        .await;
    assert_eq!(status, ClientStatus::OK);
    assert_eq!(
        structured(&server.tool("list_place", place_input.clone(), actor).await),
        &first_place_page
    );
    let place_cursor = first_place_page["next"].as_str().unwrap();
    assert_get_rejected(
        &server,
        actor,
        &format!("/api/place?min_x_cm=0&max_x_cm=400&min_y_cm=0&max_y_cm=0&min_z_cm=0&max_z_cm=0&limit=1&cursor={place_cursor}"),
        "list_place",
        json!({
            "min_x_cm": 0, "max_x_cm": 400, "min_y_cm": 0, "max_y_cm": 0,
            "min_z_cm": 0, "max_z_cm": 0, "limit": 1, "cursor": place_cursor
        }),
        ClientStatus::BAD_REQUEST,
        "invalid_request",
    )
    .await;
    assert_get_rejected(
        &server,
        actor,
        "/api/place?min_x_cm=0&max_x_cm=500&min_y_cm=0&max_y_cm=0&min_z_cm=0&max_z_cm=0&cursor=foreign",
        "list_place",
        json!({
            "min_x_cm": 0, "max_x_cm": 500, "min_y_cm": 0, "max_y_cm": 0,
            "min_z_cm": 0, "max_z_cm": 0, "cursor": "foreign"
        }),
        ClientStatus::BAD_REQUEST,
        "invalid_request",
    )
    .await;
    for limit in [0, 101] {
        assert_get_rejected(
            &server,
            actor,
            &format!("/api/place?min_x_cm=0&max_x_cm=500&min_y_cm=0&max_y_cm=0&min_z_cm=0&max_z_cm=0&limit={limit}"),
            "list_place",
            json!({
                "min_x_cm": 0, "max_x_cm": 500, "min_y_cm": 0, "max_y_cm": 0,
                "min_z_cm": 0, "max_z_cm": 0, "limit": limit
            }),
            ClientStatus::BAD_REQUEST,
            "invalid_place_limit",
        )
        .await;
    }

    let (status, first_connection_page) = server
        .get(&format!("/api/place/{origin_id}/connection?limit=1"), actor)
        .await;
    assert_eq!(status, ClientStatus::OK);
    assert_eq!(
        structured(
            &server
                .tool(
                    "list_connection",
                    json!({"place_id": origin_id, "limit": 1}),
                    actor,
                )
                .await
        ),
        &first_connection_page
    );
    let connection_cursor = first_connection_page["next"].as_str().unwrap();
    assert_get_rejected(
        &server,
        actor,
        &format!("/api/place/{first_destination_id}/connection?limit=1&cursor={connection_cursor}"),
        "list_connection",
        json!({
            "place_id": first_destination_id, "limit": 1, "cursor": connection_cursor
        }),
        ClientStatus::BAD_REQUEST,
        "invalid_request",
    )
    .await;
    for limit in [0, 101] {
        assert_get_rejected(
            &server,
            actor,
            &format!("/api/place/{origin_id}/connection?limit={limit}"),
            "list_connection",
            json!({"place_id": origin_id, "limit": limit}),
            ClientStatus::BAD_REQUEST,
            "invalid_connection_limit",
        )
        .await;
    }
    let unknown_place = Uuid::new_v4();
    assert_get_rejected(
        &server,
        actor,
        &format!("/api/place/{unknown_place}/connection"),
        "list_connection",
        json!({"place_id": unknown_place}),
        ClientStatus::NOT_FOUND,
        "place_not_found",
    )
    .await;
    assert_get_rejected(
        &server,
        actor,
        &format!("/api/place/{second_destination_id}/connection/{first_connection_id}"),
        "get_connection",
        json!({
            "place_id": second_destination_id, "connection_id": first_connection_id
        }),
        ClientStatus::NOT_FOUND,
        "connection_not_found",
    )
    .await;
    let absent_connection = Uuid::new_v4();
    assert_get_rejected(
        &server,
        actor,
        &format!("/api/place/{origin_id}/connection/{absent_connection}"),
        "get_connection",
        json!({"place_id": origin_id, "connection_id": absent_connection}),
        ClientStatus::NOT_FOUND,
        "connection_not_found",
    )
    .await;
    let (status, selected_http) = server
        .get(
            &format!("/api/place/{origin_id}/connection/{first_connection_id}"),
            actor,
        )
        .await;
    assert_eq!(status, ClientStatus::OK);
    assert_eq!(
        structured(
            &server
                .tool(
                    "get_connection",
                    json!({
                        "place_id": origin_id,
                        "connection_id": first_connection_id
                    }),
                    actor,
                )
                .await
        ),
        &selected_http
    );
    assert_eq!(activity_count(&readback).await, before_reads);
}

#[sqlx::test(migrations = "./migration")]
async fn spatial_investigation_adapters_share_kind_position_and_connection_rejections(
    pool: PgPool,
) {
    let readback = pool.clone();
    let (world, actor, _observer) = entered_world(pool, vec![0.0, 0.0, 0.0]).await;
    let server = TestServer::start(world).await;
    let before = activity_count(&readback).await;

    let request_id = Uuid::new_v4();
    let first_kind = json!({"request_id": request_id, "kind": "entity_at_position"});
    let (status, started) = server.post("/api/investigation", &first_kind, actor).await;
    assert_eq!(status, ClientStatus::OK);
    assert_eq!(started["outcome"], "positive");
    let conflicting_kind = json!({"request_id": request_id, "kind": "connected_place"});
    assert_post_rejected(
        &server,
        actor,
        "/api/investigation",
        "start_investigation",
        conflicting_kind,
        ClientStatus::CONFLICT,
        "investigation_request_conflict",
    )
    .await;

    let invalid_entity_position = json!({
        "request_id": Uuid::new_v4(),
        "attempt_id": started["attempt_id"],
        "prose": "Mara finds an oddly placed marker.",
        "result": {
            "type": "entity_at_position",
            "name": "Odd Marker",
            "description": "A marker with invalid Position guidance.",
            "position_description": "",
            "property": [],
            "trait": []
        }
    });
    assert_discovery_rejected(
        &server,
        actor,
        invalid_entity_position,
        ClientStatus::BAD_REQUEST,
        "invalid_position",
    )
    .await;

    let connected = start_positive(&server, actor, "connected_place", false).await;
    let invalid_position = connected_discovery(
        Uuid::new_v4(),
        &connected["attempt_id"],
        json!({"type": "attempt_place"}),
        "Beyond Bounds",
        1_000_000_000_000_001,
        true,
        json!([]),
    );
    assert_discovery_rejected(
        &server,
        actor,
        invalid_position,
        ClientStatus::BAD_REQUEST,
        "invalid_position",
    )
    .await;

    let invalid_connection = connected_discovery(
        Uuid::new_v4(),
        &connected["attempt_id"],
        json!({"type": "attempt_place"}),
        "One Point",
        200,
        true,
        json!([{"x_cm": 0, "y_cm": 0, "z_cm": 0}]),
    );
    assert_discovery_rejected(
        &server,
        actor,
        invalid_connection,
        ClientStatus::BAD_REQUEST,
        "invalid_connection",
    )
    .await;

    assert_eq!(activity_count(&readback).await, before);
}

#[sqlx::test(migrations = "./migration")]
async fn movement_adapters_share_validation_conflicts_partial_complete_and_exact_retries(
    pool: PgPool,
) {
    let readback = pool.clone();
    let (world, actor, _observer) = entered_world(pool, vec![0.0]).await;
    let server = TestServer::start(world).await;
    let started = start_positive(&server, actor, "connected_place", false).await;
    let discovery_input = connected_discovery(
        Uuid::new_v4(),
        &started["attempt_id"],
        json!({"type": "attempt_place"}),
        "Movement Reach",
        100,
        false,
        json!([
            {"x_cm": 0, "y_cm": 0, "z_cm": 0},
            {"x_cm": 100, "y_cm": 0, "z_cm": 0}
        ]),
    );
    let (status, discovered) = server.post("/api/discovery", &discovery_input, actor).await;
    assert_eq!(status, ClientStatus::CREATED);
    let connection_id = discovered["connection"]["id"].clone();
    let character = structured(&server.tool("get_character", json!({}), actor).await).clone();
    let revision = character["character"]["position"]["position_revision"].clone();
    let before_movement = activity_count(&readback).await;

    let unavailable = json!({
        "request_id": Uuid::new_v4(), "connection_id": Uuid::new_v4(),
        "expected_position_revision": revision,
        "direction": "source_to_destination", "target": {"type": "complete"}
    });
    assert_post_rejected(
        &server,
        actor,
        "/api/character/movement",
        "move_character",
        unavailable,
        ClientStatus::CONFLICT,
        "connection_unavailable",
    )
    .await;
    let reverse = json!({
        "request_id": Uuid::new_v4(), "connection_id": connection_id,
        "expected_position_revision": revision,
        "direction": "destination_to_source", "target": {"type": "complete"}
    });
    assert_post_rejected(
        &server,
        actor,
        "/api/character/movement",
        "move_character",
        reverse,
        ClientStatus::CONFLICT,
        "connection_direction_disallowed",
    )
    .await;
    let invalid = json!({
        "request_id": Uuid::new_v4(), "connection_id": connection_id,
        "expected_position_revision": revision,
        "direction": "source_to_destination",
        "target": {"type": "partial", "origin_segment_ordinal": 1, "target_segment_ordinal": 1, "x_cm": 50, "y_cm": 0, "z_cm": 0}
    });
    assert_post_rejected(
        &server,
        actor,
        "/api/character/movement",
        "move_character",
        invalid,
        ClientStatus::BAD_REQUEST,
        "invalid_movement",
    )
    .await;
    let off_course = json!({
        "request_id": Uuid::new_v4(), "connection_id": connection_id,
        "expected_position_revision": revision,
        "direction": "source_to_destination",
        "target": {"type": "partial", "origin_segment_ordinal": 0, "target_segment_ordinal": 0, "x_cm": 50, "y_cm": 1, "z_cm": 0}
    });
    assert_post_rejected(
        &server,
        actor,
        "/api/character/movement",
        "move_character",
        off_course,
        ClientStatus::CONFLICT,
        "movement_off_course",
    )
    .await;
    assert_eq!(activity_count(&readback).await, before_movement);

    let partial_input = json!({
        "request_id": Uuid::new_v4(), "connection_id": connection_id,
        "expected_position_revision": revision,
        "direction": "source_to_destination",
        "target": {"type": "partial", "origin_segment_ordinal": 0, "target_segment_ordinal": 0, "x_cm": 50, "y_cm": 0, "z_cm": 0}
    });
    let (status, partial) = server
        .post("/api/character/movement", &partial_input, actor)
        .await;
    assert_eq!(status, ClientStatus::OK);
    assert_eq!(
        structured(
            &server
                .tool("move_character", partial_input.clone(), actor)
                .await
        ),
        &partial
    );
    assert!(partial["character"]["current_place"].is_null());
    assert_get_rejected(
        &server,
        actor,
        "/api/place/current/entity",
        "list_entity_at_current_place",
        json!({}),
        ClientStatus::CONFLICT,
        "character_not_at_place",
    )
    .await;
    let stale_revision = json!({
        "request_id": Uuid::new_v4(), "connection_id": connection_id,
        "expected_position_revision": revision,
        "direction": "source_to_destination", "target": {"type": "complete"}
    });
    assert_post_rejected(
        &server,
        actor,
        "/api/character/movement",
        "move_character",
        stale_revision,
        ClientStatus::PRECONDITION_FAILED,
        "position_revision_conflict",
    )
    .await;
    assert_eq!(activity_count(&readback).await, before_movement + 1);

    let complete_request_id = Uuid::new_v4();
    let complete_input = json!({
        "request_id": complete_request_id, "connection_id": connection_id,
        "expected_position_revision": partial["character"]["position"]["position_revision"],
        "direction": "source_to_destination", "target": {"type": "complete"}
    });
    let complete = structured(
        &server
            .tool("move_character", complete_input.clone(), actor)
            .await,
    )
    .clone();
    let (status, complete_retry) = server
        .post("/api/character/movement", &complete_input, actor)
        .await;
    assert_eq!(status, ClientStatus::OK);
    assert_eq!(complete_retry, complete);
    assert!(complete["character"]["current_place"].is_object());

    let no_progress = json!({
        "request_id": Uuid::new_v4(), "connection_id": connection_id,
        "expected_position_revision": complete["character"]["position"]["position_revision"],
        "direction": "source_to_destination", "target": {"type": "complete"}
    });
    assert_post_rejected(
        &server,
        actor,
        "/api/character/movement",
        "move_character",
        no_progress,
        ClientStatus::CONFLICT,
        "movement_no_progress",
    )
    .await;

    let conflicting = json!({
        "request_id": complete_request_id, "connection_id": connection_id,
        "expected_position_revision": partial["character"]["position"]["position_revision"],
        "direction": "destination_to_source", "target": {"type": "complete"}
    });
    assert_post_rejected(
        &server,
        actor,
        "/api/character/movement",
        "move_character",
        conflicting,
        ClientStatus::CONFLICT,
        "movement_request_conflict",
    )
    .await;
    assert_eq!(activity_count(&readback).await, before_movement + 2);
}

#[sqlx::test(migrations = "./migration")]
async fn movement_adapters_share_temporary_unavailable_without_history(pool: PgPool) {
    let readback = pool.clone();
    let (world, actor, _observer) = entered_world(pool, vec![0.0]).await;
    let server = TestServer::start(world).await;
    let started = start_positive(&server, actor, "connected_place", false).await;
    let discovery_input = connected_discovery(
        Uuid::new_v4(),
        &started["attempt_id"],
        json!({"type": "attempt_place"}),
        "Locked Reach",
        100,
        true,
        json!([]),
    );
    let (status, discovered) = server.post("/api/discovery", &discovery_input, actor).await;
    assert_eq!(status, ClientStatus::CREATED);
    let character = structured(&server.tool("get_character", json!({}), actor).await).clone();
    let input = json!({
        "request_id": Uuid::new_v4(),
        "connection_id": discovered["connection"]["id"],
        "expected_position_revision": character["character"]["position"]["position_revision"],
        "direction": "source_to_destination",
        "target": {"type": "complete"}
    });
    let before = activity_count(&readback).await;
    let character_id = character["character"]["entity"]["id"].as_str().unwrap();
    let mut blocker = readback.begin().await.unwrap();
    sqlx::query("SELECT entity_id FROM character WHERE entity_id = $1 FOR UPDATE")
        .bind(Uuid::parse_str(character_id).unwrap())
        .fetch_one(&mut *blocker)
        .await
        .unwrap();

    let (status, http) = server.post("/api/character/movement", &input, actor).await;
    assert_eq!(status, ClientStatus::SERVICE_UNAVAILABLE);
    assert_eq!(http["error"]["code"], "temporarily_unavailable");
    let mcp = server.tool("move_character", input, actor).await;
    assert_eq!(mcp_error(&mcp), http);
    blocker.rollback().await.unwrap();
    assert_eq!(activity_count(&readback).await, before);
}

#[sqlx::test(migrations = "./migration")]
async fn connected_discovery_adapters_share_loose_origin_without_moving_character(pool: PgPool) {
    let (world, actor, _observer) = entered_world(pool, vec![0.0, 0.0]).await;
    let server = TestServer::start(world).await;
    let first = start_positive(&server, actor, "connected_place", false).await;
    let first_input = connected_discovery(
        Uuid::new_v4(),
        &first["attempt_id"],
        json!({"type": "attempt_place"}),
        "Far Bank",
        100,
        true,
        json!([
            {"x_cm": 0, "y_cm": 0, "z_cm": 0},
            {"x_cm": 100, "y_cm": 0, "z_cm": 0}
        ]),
    );
    let first_discovery =
        structured(&server.tool("submit_discovery", first_input, actor).await).clone();
    let character = structured(&server.tool("get_character", json!({}), actor).await).clone();
    let partial = json!({
        "request_id": Uuid::new_v4(),
        "connection_id": first_discovery["connection"]["id"],
        "expected_position_revision": character["character"]["position"]["position_revision"],
        "direction": "source_to_destination",
        "target": {"type": "partial", "origin_segment_ordinal": 0, "target_segment_ordinal": 0, "x_cm": 50, "y_cm": 0, "z_cm": 0}
    });
    let (status, stopped) = server
        .post("/api/character/movement", &partial, actor)
        .await;
    assert_eq!(status, ClientStatus::OK);
    assert!(stopped["character"]["current_place"].is_null());

    let loose = start_positive(&server, actor, "connected_place", true).await;
    let loose_input = connected_discovery(
        Uuid::new_v4(),
        &loose["attempt_id"],
        json!({
            "type": "new",
            "entity": {
                "name": "Midway Cairn",
                "description": "A cairn established at the loose Position.",
                "property": [],
                "trait": []
            },
            "position_description": "Exactly where the Character stopped."
        }),
        "Distant Orchard",
        200,
        true,
        json!([
            {"x_cm": 50, "y_cm": 0, "z_cm": 0},
            {"x_cm": 200, "y_cm": 0, "z_cm": 0}
        ]),
    );
    let (status, accepted) = server.post("/api/discovery", &loose_input, actor).await;
    assert_eq!(
        status,
        ClientStatus::CREATED,
        "unexpected discovery: {accepted}"
    );
    assert_eq!(
        structured(&server.tool("submit_discovery", loose_input, actor).await),
        &accepted
    );
    assert_eq!(accepted["origin"]["position"]["x_cm"], 50);
    assert_eq!(accepted["character"]["position"]["x_cm"], 50);
    assert_eq!(
        accepted["character"]["current_place"]["entity"]["id"],
        accepted["origin"]["entity"]["id"]
    );
}
