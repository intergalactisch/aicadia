use super::investigation_test::{TestServer, entered_world, structured};
use super::spatial_variant_test::{add_actor, fixture_connection};

use crate::{
    ActivityId, ActivityOperation, ConnectionId, GetEntityCurrentState, ListActivity,
    MoveCharacter, MovementDirection, MovementTarget, UserId, World,
};
use reqwest::StatusCode as ClientStatus;
use serde_json::{Value, json};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum MovementCase {
    UnshapedComplete,
    ShapedForward,
    ShapedReverse,
    Partial,
}

impl MovementCase {
    fn name(self) -> &'static str {
        match self {
            Self::UnshapedComplete => "unshaped complete",
            Self::ShapedForward => "shaped forward",
            Self::ShapedReverse => "shaped reverse",
            Self::Partial => "partial",
        }
    }
}

async fn move_with_cross_retry(
    server: &TestServer,
    actor: UserId,
    input: Value,
    via_http: bool,
) -> Value {
    if via_http {
        let (status, accepted) = server.post("/api/character/movement", &input, actor).await;
        assert_eq!(status, ClientStatus::OK, "movement: {accepted}");
        assert_eq!(
            structured(&server.tool("move_character", input, actor).await),
            &accepted
        );
        accepted
    } else {
        let accepted =
            structured(&server.tool("move_character", input.clone(), actor).await).clone();
        let (status, retry) = server.post("/api/character/movement", &input, actor).await;
        assert_eq!(status, ClientStatus::OK);
        assert_eq!(retry, accepted);
        accepted
    }
}

fn connection_id(discovery: &Value) -> ConnectionId {
    ConnectionId(Uuid::parse_str(discovery["connection"]["id"].as_str().unwrap()).unwrap())
}

async fn move_to_shaped_destination(world: &World, actor: UserId, connection_id: ConnectionId) {
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
                connection_id,
                expected_position_revision: character.position.unwrap().position_revision,
                direction: MovementDirection::SourceToDestination,
                target: MovementTarget::Complete,
            },
        )
        .await
        .unwrap();
}

#[sqlx::test(migrations = "./migration")]
async fn both_adapters_accept_every_movement_shape_and_independent_travellers(pool: PgPool) {
    let readback_pool = pool.clone();
    let (world, builder, _observer) = entered_world(pool, vec![0.0; 4]).await;
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
        "Shaped End",
        100,
        json!([
            {"x_cm": 0, "y_cm": 0, "z_cm": 0},
            {"x_cm": 100, "y_cm": 0, "z_cm": 0}
        ]),
    )
    .await;
    let shaped_id = connection_id(&shaped);
    let shaped_destination_id =
        Uuid::parse_str(shaped["destination"]["entity"]["id"].as_str().unwrap()).unwrap();
    let unshaped = fixture_connection(&server, builder, "Unshaped End", 200, json!([])).await;
    let unshaped_id = connection_id(&unshaped);
    let unshaped_destination_id =
        Uuid::parse_str(unshaped["destination"]["entity"]["id"].as_str().unwrap()).unwrap();
    let point_before: Vec<(i16, i64, i64, i64)> = sqlx::query_as(
        "SELECT ordinal, x_cm, y_cm, z_cm FROM connection_point WHERE connection_id = $1 ORDER BY ordinal",
    )
    .bind(shaped_id.0)
    .fetch_all(&readback_pool)
    .await
    .unwrap();
    let mut independent = Vec::new();

    for case in [
        MovementCase::UnshapedComplete,
        MovementCase::ShapedForward,
        MovementCase::ShapedReverse,
        MovementCase::Partial,
    ] {
        for via_http in [true, false] {
            let actor = add_actor(
                &world,
                &format!(
                    "{} {} traveller",
                    if via_http { "HTTP" } else { "MCP" },
                    case.name()
                ),
            )
            .await;
            if case == MovementCase::ShapedReverse {
                move_to_shaped_destination(&world, actor, shaped_id).await;
            }
            let character =
                structured(&server.tool("get_character", json!({}), actor).await)["character"]
                    .clone();
            let selected_connection = if case == MovementCase::UnshapedComplete {
                unshaped_id
            } else {
                shaped_id
            };
            let direction = if case == MovementCase::ShapedReverse {
                "destination_to_source"
            } else {
                "source_to_destination"
            };
            let target = if case == MovementCase::Partial {
                json!({
                    "type": "partial", "origin_segment_ordinal": 0,
                    "target_segment_ordinal": 0, "x_cm": 50, "y_cm": 0, "z_cm": 0
                })
            } else {
                json!({"type": "complete"})
            };
            let input = json!({
                "request_id": Uuid::new_v4(),
                "connection_id": selected_connection.0,
                "expected_position_revision": character["position"]["position_revision"],
                "direction": direction,
                "target": target
            });
            let accepted = move_with_cross_retry(&server, actor, input, via_http).await;
            let activity_id =
                ActivityId(Uuid::parse_str(accepted["activity"]["id"].as_str().unwrap()).unwrap());
            let state = world
                .get_character(actor, GetEntityCurrentState::default())
                .await
                .unwrap()
                .character;
            let position = state.position.unwrap();
            let (expected_x_cm, expected_place_id) = match case {
                MovementCase::UnshapedComplete => (200, Some(unshaped_destination_id)),
                MovementCase::ShapedForward => (100, Some(shaped_destination_id)),
                MovementCase::ShapedReverse => (0, Some(entry_id.0)),
                MovementCase::Partial => (50, None),
            };
            assert_eq!(
                (position.x_cm, position.y_cm, position.z_cm),
                (expected_x_cm, 0, 0)
            );
            assert_eq!(
                state.current_place.map(|place| place.entity.id.0),
                expected_place_id
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
            assert_eq!(activity.operation, ActivityOperation::MoveCharacter);
            assert_eq!(activity.involved_position.len(), 2);
            assert_eq!(activity.involved_connection.len(), 1);
            assert_eq!(
                activity.involved_connection[0].connection_id,
                selected_connection
            );
            let movement_count: i64 = sqlx::query_scalar(
                "SELECT count(*) FROM activity WHERE requested_by_user_id = $1 AND operation = 'move_character'",
            )
            .bind(actor.0)
            .fetch_one(&readback_pool)
            .await
            .unwrap();
            assert_eq!(
                movement_count,
                if case == MovementCase::ShapedReverse {
                    2
                } else {
                    1
                }
            );
            if case == MovementCase::ShapedForward {
                independent.push((actor, activity_id));
            }
        }
    }

    assert_eq!(independent.len(), 2);
    assert_ne!(independent[0].1, independent[1].1);
    for (actor, own_activity_id) in &independent {
        let history = world
            .list_activity(
                *actor,
                ListActivity {
                    cursor: None,
                    limit: 100,
                },
            )
            .await
            .unwrap();
        assert!(
            history
                .activity
                .iter()
                .any(|activity| activity.id == *own_activity_id)
        );
        assert!(
            history.activity.iter().all(|activity| activity.id
                != independent.iter().find(|item| item.0 != *actor).unwrap().1)
        );
    }
    let point_after: Vec<(i16, i64, i64, i64)> = sqlx::query_as(
        "SELECT ordinal, x_cm, y_cm, z_cm FROM connection_point WHERE connection_id = $1 ORDER BY ordinal",
    )
    .bind(shaped_id.0)
    .fetch_all(&readback_pool)
    .await
    .unwrap();
    assert_eq!(point_after, point_before);
    let connection_count: i64 = sqlx::query_scalar("SELECT count(*) FROM connection")
        .fetch_one(&readback_pool)
        .await
        .unwrap();
    assert_eq!(connection_count, 2);
}
