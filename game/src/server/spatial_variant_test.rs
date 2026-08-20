use super::investigation_test::{TestServer, entered_world, structured};

use crate::{
    ActivityId, ActivityOperation, ActivityPositionRole, ConnectionId, CreateCharacter, EntityId,
    GetConnection, GetEntityCurrentState, ListActivity, MoveCharacter, MovementDirection,
    MovementTarget, UserId, World,
};
use reqwest::StatusCode as ClientStatus;
use serde_json::{Value, json};
use sqlx::PgPool;
use uuid::Uuid;

pub(super) async fn add_actor(world: &World, name: &str) -> UserId {
    let user = world.create_user().await.unwrap();
    world
        .create_character(
            user.id,
            CreateCharacter {
                name: name.to_owned(),
                description: format!("{name} explores one exact spatial variant."),
                property: Vec::new(),
                r#trait: Vec::new(),
            },
        )
        .await
        .unwrap();
    world.enter_world(user.id).await.unwrap();
    user.id
}

pub(super) async fn start(server: &TestServer, actor: UserId, kind: &str, via_http: bool) -> Value {
    let input = json!({"request_id": Uuid::new_v4(), "kind": kind});
    let result = if via_http {
        let (status, result) = server.post("/api/investigation", &input, actor).await;
        assert_eq!(status, ClientStatus::OK, "investigation: {result}");
        result
    } else {
        structured(&server.tool("start_investigation", input, actor).await).clone()
    };
    assert_eq!(result["outcome"], "positive");
    result
}

pub(super) async fn submit_with_cross_retry(
    server: &TestServer,
    actor: UserId,
    input: Value,
    via_http: bool,
) -> Value {
    if via_http {
        let (status, accepted) = server.post("/api/discovery", &input, actor).await;
        assert_eq!(status, ClientStatus::CREATED, "discovery: {accepted}");
        assert_eq!(
            structured(&server.tool("submit_discovery", input, actor).await),
            &accepted
        );
        accepted
    } else {
        let accepted =
            structured(&server.tool("submit_discovery", input.clone(), actor).await).clone();
        let (status, retry) = server.post("/api/discovery", &input, actor).await;
        assert_eq!(status, ClientStatus::CREATED);
        assert_eq!(retry, accepted);
        accepted
    }
}

pub(super) fn place_input(name: &str) -> Value {
    json!({
        "name": name,
        "description": format!("The exact Place called {name}."),
        "property": [],
        "trait": []
    })
}

pub(super) async fn fixture_connection(
    server: &TestServer,
    actor: UserId,
    destination_name: &str,
    destination_x_cm: i64,
    course: Value,
) -> Value {
    let attempt = start(server, actor, "connected_place", false).await;
    let input = json!({
        "request_id": Uuid::new_v4(),
        "attempt_id": attempt["attempt_id"],
        "prose": format!("The fixture establishes {destination_name}."),
        "result": {
            "type": "connected_place",
            "origin": {"type": "attempt_place"},
            "destination": {
                "type": "new",
                "entity": place_input(destination_name),
                "position": {"x_cm": destination_x_cm, "y_cm": 0, "z_cm": 0}
            },
            "connection": {
                "name": format!("{destination_name} Path"),
                "description": format!("A fixture path to {destination_name}."),
                "allows_reverse": true,
                "course": course
            }
        }
    });
    structured(&server.tool("submit_discovery", input, actor).await).clone()
}

#[sqlx::test(migrations = "./migration")]
async fn both_adapters_accept_entity_and_every_connected_place_variant(pool: PgPool) {
    let readback_pool = pool.clone();
    let (world, builder, _observer) = entered_world(pool, vec![0.0; 24]).await;
    let server = TestServer::start(world.clone()).await;
    let entry_id = world
        .get_character(builder, GetEntityCurrentState::default())
        .await
        .unwrap()
        .character
        .current_place
        .unwrap()
        .entity
        .id;
    let shaped = fixture_connection(
        &server,
        builder,
        "Fixture End",
        100,
        json!([
            {"x_cm": 0, "y_cm": 0, "z_cm": 0},
            {"x_cm": 100, "y_cm": 0, "z_cm": 0}
        ]),
    )
    .await;
    let shaped_id =
        ConnectionId(Uuid::parse_str(shaped["connection"]["id"].as_str().unwrap()).unwrap());
    let existing_destination_id =
        EntityId(Uuid::parse_str(shaped["destination"]["entity"]["id"].as_str().unwrap()).unwrap());

    let builder_state = world
        .get_character(builder, GetEntityCurrentState::default())
        .await
        .unwrap()
        .character;
    world
        .move_character(
            builder,
            MoveCharacter {
                request_id: Uuid::new_v4(),
                connection_id: shaped_id,
                expected_position_revision: builder_state.position.unwrap().position_revision,
                direction: MovementDirection::SourceToDestination,
                target: MovementTarget::Partial {
                    origin_segment_ordinal: 0,
                    target_segment_ordinal: 0,
                    x_cm: 50,
                    y_cm: 0,
                    z_cm: 0,
                },
            },
        )
        .await
        .unwrap();
    let midpoint_attempt = start(&server, builder, "connected_place", false).await;
    let midpoint_input = json!({
        "request_id": Uuid::new_v4(),
        "attempt_id": midpoint_attempt["attempt_id"],
        "prose": "The fixture establishes an exact midpoint Place.",
        "result": {
            "type": "connected_place",
            "origin": {"type": "new", "entity": place_input("Fixture Midpoint")},
            "destination": {
                "type": "new", "entity": place_input("Fixture Beyond"),
                "position": {"x_cm": 300, "y_cm": 0, "z_cm": 0}
            },
            "connection": {
                "name": "Beyond Path", "description": "An unshaped fixture path.",
                "allows_reverse": true, "course": []
            }
        }
    });
    let midpoint = structured(
        &server
            .tool("submit_discovery", midpoint_input, builder)
            .await,
    )
    .clone();
    let existing_origin_id =
        EntityId(Uuid::parse_str(midpoint["origin"]["entity"]["id"].as_str().unwrap()).unwrap());

    for via_http in [true, false] {
        let actor = add_actor(
            &world,
            if via_http {
                "HTTP Finder"
            } else {
                "MCP Finder"
            },
        )
        .await;
        let attempt = start(&server, actor, "entity_at_position", via_http).await;
        let input = json!({
            "request_id": Uuid::new_v4(), "attempt_id": attempt["attempt_id"],
            "prose": "A finder establishes one exact adapter-grounded Entity.",
            "result": {
                "type": "entity_at_position",
                "name": if via_http { "HTTP Fern" } else { "MCP Fern" },
                "description": "A fern found at the Character's exact Position.",
                "position_description": "Rooted beside the entry stones.",
                "property": [], "trait": []
            }
        });
        let accepted = submit_with_cross_retry(&server, actor, input, via_http).await;
        assert_eq!(accepted["position"]["x_cm"], 0);
        assert_eq!(accepted["place"]["entity"]["id"], entry_id.0.to_string());
        let activity_id =
            ActivityId(Uuid::parse_str(accepted["activity"]["id"].as_str().unwrap()).unwrap());
        let history = world
            .list_activity(
                actor,
                ListActivity {
                    cursor: None,
                    limit: 100,
                },
            )
            .await
            .unwrap();
        let activity = history
            .activity
            .iter()
            .find(|activity| activity.id == activity_id)
            .unwrap();
        assert_eq!(activity.operation, ActivityOperation::SubmitDiscovery);
        assert_eq!(activity.involved_position.len(), 2);
    }

    for origin_kind in ["current", "new", "existing"] {
        for destination_kind in ["new", "existing"] {
            for via_http in [true, false] {
                let label = format!(
                    "{} {} {}",
                    if via_http { "HTTP" } else { "MCP" },
                    origin_kind,
                    destination_kind
                );
                let actor = add_actor(&world, &label).await;
                if origin_kind != "current" {
                    let character = world
                        .get_character(actor, GetEntityCurrentState::default())
                        .await
                        .unwrap()
                        .character;
                    world
                        .move_character(
                            actor,
                            MoveCharacter {
                                request_id: Uuid::new_v4(),
                                connection_id: shaped_id,
                                expected_position_revision: character
                                    .position
                                    .unwrap()
                                    .position_revision,
                                direction: MovementDirection::SourceToDestination,
                                target: MovementTarget::Partial {
                                    origin_segment_ordinal: 0,
                                    target_segment_ordinal: 0,
                                    x_cm: 50,
                                    y_cm: 0,
                                    z_cm: 0,
                                },
                            },
                        )
                        .await
                        .unwrap();
                }
                let before = world
                    .get_character(actor, GetEntityCurrentState::default())
                    .await
                    .unwrap()
                    .character
                    .position
                    .unwrap();
                let attempt = start(&server, actor, "connected_place", via_http).await;
                let origin = match origin_kind {
                    "current" => json!({"type": "attempt_place"}),
                    "new" => json!({
                        "type": "new",
                        "entity": place_input(&format!("{label} Origin")),
                        "position_description": "Established at the exact loose point."
                    }),
                    "existing" => json!({
                        "type": "existing", "place_id": existing_origin_id.0
                    }),
                    _ => unreachable!(),
                };
                let destination = if destination_kind == "new" {
                    json!({
                        "type": "new", "entity": place_input(&format!("{label} Destination")),
                        "position": {
                            "x_cm": 1_000 + i64::from(via_http),
                            "y_cm": if origin_kind == "current" { 10 } else { 20 },
                            "z_cm": if destination_kind == "new" { 30 } else { 40 }
                        }
                    })
                } else {
                    json!({"type": "existing", "place_id": existing_destination_id.0})
                };
                let input = json!({
                    "request_id": Uuid::new_v4(), "attempt_id": attempt["attempt_id"],
                    "prose": format!("The {label} variant establishes one Connection."),
                    "result": {
                        "type": "connected_place", "origin": origin, "destination": destination,
                        "connection": {
                            "name": format!("{label} Path"),
                            "description": format!("The exact {label} Connection."),
                            "shape_description": "No course points are required here.",
                            "allows_reverse": true, "course": []
                        }
                    }
                });
                let accepted = submit_with_cross_retry(&server, actor, input, via_http).await;
                let origin_id = EntityId(
                    Uuid::parse_str(accepted["origin"]["entity"]["id"].as_str().unwrap()).unwrap(),
                );
                let destination_id = EntityId(
                    Uuid::parse_str(accepted["destination"]["entity"]["id"].as_str().unwrap())
                        .unwrap(),
                );
                let connection_id = ConnectionId(
                    Uuid::parse_str(accepted["connection"]["id"].as_str().unwrap()).unwrap(),
                );
                let selected = world
                    .get_connection(
                        actor,
                        GetConnection {
                            place_id: origin_id,
                            connection_id,
                        },
                    )
                    .await
                    .unwrap();
                assert_eq!(selected.source.place.id, origin_id);
                assert_eq!(selected.destination.place.id, destination_id);
                assert!(selected.course.is_empty());
                let activity_id = ActivityId(
                    Uuid::parse_str(accepted["activity"]["id"].as_str().unwrap()).unwrap(),
                );
                let history = world
                    .list_activity(
                        actor,
                        ListActivity {
                            cursor: None,
                            limit: 100,
                        },
                    )
                    .await
                    .unwrap();
                let activity = history
                    .activity
                    .iter()
                    .find(|activity| activity.id == activity_id)
                    .unwrap();
                assert_eq!(activity.operation, ActivityOperation::SubmitDiscovery);
                assert_eq!(activity.involved_connection.len(), 1);
                assert_eq!(activity.involved_connection[0].connection_id, connection_id);
                assert_eq!(
                    activity
                        .involved_position
                        .iter()
                        .filter(|reference| reference.role == ActivityPositionRole::Origin)
                        .count(),
                    1
                );
                assert_eq!(
                    activity
                        .involved_position
                        .iter()
                        .filter(|reference| reference.role == ActivityPositionRole::Result)
                        .count(),
                    usize::from(origin_kind == "new") + usize::from(destination_kind == "new")
                );
                let after = world
                    .get_character(actor, GetEntityCurrentState::default())
                    .await
                    .unwrap()
                    .character;
                let after_position = after.position.unwrap();
                assert_eq!(
                    (
                        after_position.x_cm,
                        after_position.y_cm,
                        after_position.z_cm
                    ),
                    (before.x_cm, before.y_cm, before.z_cm)
                );
                let expected_current = match origin_kind {
                    "current" => entry_id,
                    "new" => origin_id,
                    "existing" => existing_origin_id,
                    _ => unreachable!(),
                };
                assert_eq!(after.current_place.unwrap().entity.id, expected_current);
                let connection_rows: i64 =
                    sqlx::query_scalar("SELECT count(*) FROM connection WHERE id = $1")
                        .bind(connection_id.0)
                        .fetch_one(&readback_pool)
                        .await
                        .unwrap();
                assert_eq!(connection_rows, 1);
            }
        }
    }
}
