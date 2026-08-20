use super::investigation_test::{TestServer, activity_count, entered_world, mcp_error, structured};

use crate::{
    ActivityId, ActivityOperation, ActivityPositionRole, ConnectionId, EntityId, GetConnection,
    ListActivity,
};

use reqwest::StatusCode as ClientStatus;
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;

#[sqlx::test(migrations = "./migration")]
async fn spatial_adapters_share_exact_window_cursor_and_place_unavailable_errors(pool: PgPool) {
    let readback = pool.clone();
    let (world, actor, _observer) = entered_world(pool, vec![0.0]).await;
    let origin_id = world
        .get_character(actor, crate::GetEntityCurrentState::default())
        .await
        .unwrap()
        .character
        .current_place
        .unwrap()
        .entity
        .id;
    let server = TestServer::start(world).await;
    let before = activity_count(&readback).await;

    for (min_x_cm, max_x_cm, field, reason) in [
        (
            1_000_000_000_000_001_i64,
            1_000_000_000_000_001_i64,
            "min_x_cm",
            "out_of_range",
        ),
        (1, 0, "max_x_cm", "before_minimum"),
        (0, 100_000_001, "max_x_cm", "span_too_wide"),
    ] {
        let path = format!(
            "/api/place?min_x_cm={min_x_cm}&max_x_cm={max_x_cm}&min_y_cm=0&max_y_cm=0&min_z_cm=0&max_z_cm=0"
        );
        let (status, http) = server.get(&path, actor).await;
        assert_eq!(status, ClientStatus::BAD_REQUEST);
        assert_eq!(http["error"]["code"], "invalid_place_window");
        assert_eq!(http["error"]["field"], field);
        assert_eq!(http["error"]["reason"], reason);
        let mcp = server
            .tool(
                "list_place",
                json!({
                    "min_x_cm": min_x_cm, "max_x_cm": max_x_cm,
                    "min_y_cm": 0, "max_y_cm": 0, "min_z_cm": 0, "max_z_cm": 0
                }),
                actor,
            )
            .await;
        assert_eq!(mcp_error(&mcp), http);
    }

    let malformed_path = format!("/api/place/{}/connection?cursor=foreign", origin_id.0);
    let (status, cursor_http) = server.get(&malformed_path, actor).await;
    assert_eq!(status, ClientStatus::BAD_REQUEST);
    assert_eq!(cursor_http["error"]["code"], "invalid_request");
    let cursor_mcp = server
        .tool(
            "list_connection",
            json!({"place_id": origin_id.0, "cursor": "foreign"}),
            actor,
        )
        .await;
    assert_eq!(mcp_error(&cursor_mcp), cursor_http);

    let started = server
        .tool(
            "start_investigation",
            json!({"request_id": Uuid::new_v4(), "kind": "connected_place"}),
            actor,
        )
        .await;
    let discovery = json!({
        "request_id": Uuid::new_v4(),
        "attempt_id": structured(&started)["attempt_id"],
        "prose": "Mara tests an unavailable destination without changing the World.",
        "result": {
            "type": "connected_place",
            "origin": {"type": "attempt_place"},
            "destination": {"type": "existing", "place_id": Uuid::new_v4()},
            "connection": {
                "name": "Unknown Path", "description": "A path with no accepted destination.",
                "allows_reverse": true, "course": []
            }
        }
    });
    let (status, unavailable_http) = server.post("/api/discovery", &discovery, actor).await;
    assert_eq!(status, ClientStatus::CONFLICT);
    assert_eq!(unavailable_http["error"]["code"], "place_unavailable");
    let unavailable_mcp = server
        .tool("submit_discovery", discovery.clone(), actor)
        .await;
    assert_eq!(mcp_error(&unavailable_mcp), unavailable_http);

    let mut unavailable_attempt = discovery;
    unavailable_attempt["request_id"] = json!(Uuid::new_v4());
    unavailable_attempt["attempt_id"] = json!(Uuid::new_v4());
    let (status, attempt_http) = server
        .post("/api/discovery", &unavailable_attempt, actor)
        .await;
    assert_eq!(status, ClientStatus::CONFLICT);
    assert_eq!(
        attempt_http["error"]["code"],
        "discovery_attempt_unavailable"
    );
    let attempt_mcp = server
        .tool("submit_discovery", unavailable_attempt, actor)
        .await;
    assert_eq!(mcp_error(&attempt_mcp), attempt_http);
    assert_eq!(activity_count(&readback).await, before);
}

#[sqlx::test(migrations = "./migration")]
async fn two_user_spatial_flow_has_exact_shared_state_and_private_movement_history(pool: PgPool) {
    let readback_pool = pool.clone();
    let (world, actor, observer) = entered_world(pool, vec![0.0]).await;
    let readback = world.clone();
    let origin_state = readback
        .get_character(actor, crate::GetEntityCurrentState::default())
        .await
        .unwrap();
    let actor_origin_position = origin_state.character.position.unwrap();
    let origin = origin_state.character.current_place.unwrap();
    let origin_id = origin.entity.id;
    let origin_position = origin.position;
    let server = TestServer::start(world).await;

    let before_map_read = activity_count(&readback_pool).await;
    let actor_places = server
        .tool(
            "list_place",
            json!({
                "min_x_cm": origin_position.x_cm, "max_x_cm": origin_position.x_cm,
                "min_y_cm": origin_position.y_cm, "max_y_cm": origin_position.y_cm,
                "min_z_cm": origin_position.z_cm, "max_z_cm": origin_position.z_cm
            }),
            actor,
        )
        .await;
    assert!(
        structured(&actor_places)["place"]
            .as_array()
            .unwrap()
            .iter()
            .any(|place| place["id"] == origin_id.0.to_string())
    );
    assert_eq!(activity_count(&readback_pool).await, before_map_read);

    let investigation = server
        .tool(
            "start_investigation",
            json!({"request_id": Uuid::new_v4(), "kind": "connected_place"}),
            actor,
        )
        .await;
    let attempt_id = structured(&investigation)["attempt_id"].clone();
    let discovery_input = json!({
        "request_id": Uuid::new_v4(),
        "attempt_id": attempt_id,
        "prose": "Mara charts Bell Meadow without leaving North Gate.",
        "result": {
            "type": "connected_place",
            "origin": {"type": "attempt_place"},
            "destination": {
                "type": "new",
                "entity": {
                    "name": "Bell Meadow",
                    "description": "A meadow whose seed heads ring in the wind.",
                    "property": [],
                    "trait": []
                },
                "position": {"x_cm": 200, "y_cm": 0, "z_cm": 0}
            },
            "connection": {
                "name": "Bell Path",
                "description": "A narrow path through tall silver grass.",
                "allows_reverse": true,
                "course": [
                    {"x_cm": 0, "y_cm": 0, "z_cm": 0},
                    {"x_cm": 100, "y_cm": 0, "z_cm": 0},
                    {"x_cm": 200, "y_cm": 0, "z_cm": 0}
                ]
            }
        }
    });
    let (status, discovered) = server.post("/api/discovery", &discovery_input, actor).await;
    assert_eq!(status, ClientStatus::CREATED, "discovery: {discovered}");
    let retry = server
        .tool("submit_discovery", discovery_input, actor)
        .await;
    assert_eq!(structured(&retry), &discovered);
    let destination_id = EntityId(
        Uuid::parse_str(discovered["destination"]["entity"]["id"].as_str().unwrap()).unwrap(),
    );
    let connection_id =
        ConnectionId(Uuid::parse_str(discovered["connection"]["id"].as_str().unwrap()).unwrap());
    let discovery_activity_id =
        ActivityId(Uuid::parse_str(discovered["activity"]["id"].as_str().unwrap()).unwrap());
    let after_discovery = activity_count(&readback_pool).await;

    let actor_connections = server
        .tool("list_connection", json!({"place_id": origin_id.0}), actor)
        .await;
    assert_eq!(
        structured(&actor_connections)["connection"][0]["id"],
        connection_id.0.to_string()
    );
    assert!(
        structured(&actor_connections)["connection"][0]
            .get("course")
            .is_none()
    );
    assert_eq!(activity_count(&readback_pool).await, after_discovery);
    let actor_connection = server
        .tool(
            "get_connection",
            json!({"place_id": origin_id.0, "connection_id": connection_id.0}),
            actor,
        )
        .await;
    assert_eq!(
        structured(&actor_connection)["course"]
            .as_array()
            .unwrap()
            .len(),
        3
    );
    assert_eq!(activity_count(&readback_pool).await, after_discovery);

    let partial_input = json!({
        "request_id": Uuid::new_v4(),
        "connection_id": connection_id.0,
        "expected_position_revision": discovered["character"]["position"]["position_revision"],
        "direction": "source_to_destination",
        "target": {
            "type": "partial", "origin_segment_ordinal": 0, "target_segment_ordinal": 0,
            "x_cm": 50, "y_cm": 0, "z_cm": 0
        }
    });
    let partial_mcp = server
        .tool("move_character", partial_input.clone(), actor)
        .await;
    let partial = structured(&partial_mcp).clone();
    let partial_activity_id =
        ActivityId(Uuid::parse_str(partial["activity"]["id"].as_str().unwrap()).unwrap());
    let (status, partial_retry) = server
        .post("/api/character/movement", &partial_input, actor)
        .await;
    assert_eq!(status, ClientStatus::OK);
    assert_eq!(partial_retry, partial);
    assert!(partial["character"]["current_place"].is_null());

    let complete_input = json!({
        "request_id": Uuid::new_v4(),
        "connection_id": connection_id.0,
        "expected_position_revision": partial["character"]["position"]["position_revision"],
        "direction": "source_to_destination",
        "target": {"type": "complete"}
    });
    let (status, complete) = server
        .post("/api/character/movement", &complete_input, actor)
        .await;
    assert_eq!(status, ClientStatus::OK, "complete movement: {complete}");
    let complete_activity_id =
        ActivityId(Uuid::parse_str(complete["activity"]["id"].as_str().unwrap()).unwrap());
    let complete_retry = server.tool("move_character", complete_input, actor).await;
    assert_eq!(structured(&complete_retry), &complete);
    let after_movement = activity_count(&readback_pool).await;

    let observer_places = server
        .tool(
            "list_place",
            json!({
                "min_x_cm": 200, "max_x_cm": 200,
                "min_y_cm": 0, "max_y_cm": 0,
                "min_z_cm": 0, "max_z_cm": 0
            }),
            observer,
        )
        .await;
    assert_eq!(
        structured(&observer_places)["place"][0]["id"],
        destination_id.0.to_string()
    );
    assert_eq!(activity_count(&readback_pool).await, after_movement);
    let observer_connections = server
        .tool(
            "list_connection",
            json!({"place_id": destination_id.0}),
            observer,
        )
        .await;
    assert_eq!(
        structured(&observer_connections)["connection"][0]["id"],
        connection_id.0.to_string()
    );
    assert_eq!(activity_count(&readback_pool).await, after_movement);
    let (status, observer_connection) = server
        .get(
            &format!(
                "/api/place/{}/connection/{}",
                destination_id.0, connection_id.0
            ),
            observer,
        )
        .await;
    assert_eq!(status, ClientStatus::OK);
    assert_eq!(observer_connection["course"].as_array().unwrap().len(), 3);
    assert_eq!(activity_count(&readback_pool).await, after_movement);
    let observer_history = server
        .tool(
            "list_activity_at_current_place",
            json!({"limit": 100}),
            observer,
        )
        .await;
    assert!(
        structured(&observer_history)["activity"]
            .as_array()
            .unwrap()
            .iter()
            .any(|activity| activity["id"] == discovery_activity_id.0.to_string())
    );
    assert_eq!(activity_count(&readback_pool).await, after_movement);
    let actor_history = server
        .tool("list_activity", json!({"limit": 100}), actor)
        .await;
    for activity_id in [partial_activity_id, complete_activity_id] {
        let activity = structured(&actor_history)["activity"]
            .as_array()
            .unwrap()
            .iter()
            .find(|activity| activity["id"] == activity_id.0.to_string())
            .unwrap();
        assert_eq!(activity["involved_position"].as_array().unwrap().len(), 2);
        assert_eq!(
            activity["involved_connection"][0]["id"],
            connection_id.0.to_string()
        );
    }
    assert_eq!(activity_count(&readback_pool).await, after_movement);

    let actor_state = readback
        .get_character(actor, crate::GetEntityCurrentState::default())
        .await
        .unwrap()
        .character;
    let actor_position = actor_state.position.unwrap();
    assert_eq!(
        (
            actor_position.x_cm,
            actor_position.y_cm,
            actor_position.z_cm
        ),
        (200, 0, 0)
    );
    assert_eq!(actor_state.current_place.unwrap().entity.id, destination_id);
    let observer_state = readback
        .get_character(observer, crate::GetEntityCurrentState::default())
        .await
        .unwrap()
        .character;
    let observer_position = observer_state.position.unwrap();
    assert_eq!(
        (
            observer_position.x_cm,
            observer_position.y_cm,
            observer_position.z_cm
        ),
        (
            origin_position.x_cm,
            origin_position.y_cm,
            origin_position.z_cm
        )
    );
    assert_eq!(observer_state.current_place.unwrap().entity.id, origin_id);
    let connection = readback
        .get_connection(
            observer,
            GetConnection {
                place_id: destination_id,
                connection_id,
            },
        )
        .await
        .unwrap();
    assert_eq!(
        (connection.source.place.id, connection.destination.place.id),
        (origin_id, destination_id)
    );
    assert!(connection.allows_reverse);
    assert_eq!(
        connection
            .course
            .iter()
            .map(|point| (point.ordinal, point.x_cm, point.y_cm, point.z_cm))
            .collect::<Vec<_>>(),
        vec![(0, 0, 0, 0), (1, 100, 0, 0), (2, 200, 0, 0)]
    );

    let history = readback
        .list_activity(
            actor,
            ListActivity {
                cursor: None,
                limit: 100,
            },
        )
        .await
        .unwrap();
    let discovery = history
        .activity
        .iter()
        .find(|activity| activity.id == discovery_activity_id)
        .unwrap();
    assert_eq!(discovery.operation, ActivityOperation::SubmitDiscovery);
    assert_eq!(
        discovery.involved_connection[0].connection_id,
        connection_id
    );
    assert_eq!(discovery.involved_position.len(), 2);
    let discovery_position = |role| {
        discovery
            .involved_position
            .iter()
            .find(|item| item.role == role)
            .unwrap()
    };
    assert_eq!(
        discovery_position(ActivityPositionRole::Origin)
            .position
            .position_revision,
        actor_origin_position.position_revision
    );
    assert_eq!(
        discovery_position(ActivityPositionRole::Result)
            .position
            .x_cm,
        200
    );
    for (activity_id, origin_x_cm, result_x_cm) in [
        (partial_activity_id, 0, 50),
        (complete_activity_id, 50, 200),
    ] {
        let movement = history
            .activity
            .iter()
            .find(|activity| activity.id == activity_id)
            .unwrap();
        assert_eq!(movement.operation, ActivityOperation::MoveCharacter);
        assert_eq!(movement.involved_connection[0].connection_id, connection_id);
        let position = |role| {
            movement
                .involved_position
                .iter()
                .find(|item| item.role == role)
                .unwrap()
        };
        assert_eq!(
            position(ActivityPositionRole::Origin).position.x_cm,
            origin_x_cm
        );
        assert_eq!(
            position(ActivityPositionRole::Result).position.x_cm,
            result_x_cm
        );
    }

    let connection_rows: i64 = sqlx::query_scalar("SELECT count(*) FROM connection WHERE id = $1")
        .bind(connection_id.0)
        .fetch_one(&readback_pool)
        .await
        .unwrap();
    let movement_rows: Vec<Uuid> = sqlx::query_scalar(
        "SELECT id FROM activity WHERE operation = 'move_character' ORDER BY id",
    )
    .fetch_all(&readback_pool)
    .await
    .unwrap();
    let movement_connections: Vec<(Uuid, Uuid)> = sqlx::query_as(
        "SELECT activity_id, connection_id FROM activity_connection WHERE activity_id = ANY($1) ORDER BY activity_id",
    ).bind(&movement_rows).fetch_all(&readback_pool).await.unwrap();
    let movement_positions: Vec<(Uuid, String, i64, i64, i64)> = sqlx::query_as(
        "SELECT ap.activity_id, ap.role, pv.x_cm, pv.y_cm, pv.z_cm FROM activity_position ap JOIN position_version pv ON pv.entity_id = ap.position_entity_id AND pv.activity_id = ap.position_activity_id WHERE ap.activity_id = ANY($1) ORDER BY ap.activity_id, ap.role",
    ).bind(&movement_rows).fetch_all(&readback_pool).await.unwrap();
    assert_eq!(connection_rows, 1);
    let mut expected_movement_ids = vec![partial_activity_id.0, complete_activity_id.0];
    expected_movement_ids.sort();
    assert_eq!(movement_rows, expected_movement_ids);
    assert_eq!(
        movement_connections,
        movement_rows
            .iter()
            .map(|id| (*id, connection_id.0))
            .collect::<Vec<_>>()
    );
    assert_eq!(movement_positions.len(), 4);
    for activity_id in movement_rows {
        let expected = if activity_id == partial_activity_id.0 {
            [0, 50]
        } else {
            [50, 200]
        };
        let position = movement_positions
            .iter()
            .filter(|row| row.0 == activity_id)
            .collect::<Vec<_>>();
        assert_eq!(position.len(), 2);
        assert_eq!(
            (position[0].1.as_str(), position[0].2),
            ("origin", expected[0])
        );
        assert_eq!(
            (position[1].1.as_str(), position[1].2),
            ("result", expected[1])
        );
        assert!(position.iter().all(|row| (row.3, row.4) == (0, 0)));
    }
    assert_eq!(activity_count(&readback_pool).await, after_movement);
}
