use super::*;

fn fingerprint_hex(fingerprint: &[u8]) -> String {
    fingerprint
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

#[test]
fn entity_at_position_fingerprint_remains_exact_pre_t4_v1() {
    let input = SubmitDiscovery {
        request_id: Uuid::from_u128(1),
        attempt_id: InvestigationAttemptId(
            Uuid::parse_str("00112233-4455-6677-8899-aabbccddeeff").unwrap(),
        ),
        prose: "Pinned prose.".to_owned(),
        result: DiscoveryResultInput::EntityAtPosition {
            name: "Pinned Find".to_owned(),
            description: "An old accepted result.".to_owned(),
            position_description: Some("Two centimetres above stone.".to_owned()),
            property: vec![
                PropertyInput {
                    key: "height".to_owned(),
                    value: PropertyValue::Integer(2),
                },
                PropertyInput {
                    key: "colour".to_owned(),
                    value: PropertyValue::Text("blue".to_owned()),
                },
            ],
            r#trait: vec![TraitInput {
                statement: "Rings in rain.".to_owned(),
            }],
        },
    };
    assert_eq!(
        fingerprint_hex(&discovery_fingerprint(&input.clone().normalize().unwrap())),
        "c0d6267cbd6e9df6f983984b4a4591502153d34c4e906fcac643860159099a4e"
    );
    let mut without_description = input;
    let DiscoveryResultInput::EntityAtPosition {
        position_description,
        ..
    } = &mut without_description.result
    else {
        unreachable!()
    };
    *position_description = None;
    assert_eq!(
        fingerprint_hex(&discovery_fingerprint(
            &without_description.normalize().unwrap()
        )),
        "36965ca9283e5fa030b5b3a91e382aa0450562c29a45676fbd1710b3cd160a09"
    );
}

pub(super) async fn positive(world: &World, user_id: UserId) -> InvestigationResult {
    world
        .start_investigation(
            user_id,
            StartInvestigation {
                request_id: Uuid::new_v4(),
                kind: DiscoveryKind::EntityAtPosition,
            },
        )
        .await
        .unwrap()
}

#[sqlx::test(migrations = "./migration")]
async fn attempt_kind_conflicts_and_mismatched_submission_write_nothing(pool: PgPool) {
    let (world, user_id, _, _) = entered_world(pool.clone(), vec![0.0]).await;
    let request_id = Uuid::new_v4();
    let attempt = world
        .start_investigation(
            user_id,
            StartInvestigation {
                request_id,
                kind: DiscoveryKind::EntityAtPosition,
            },
        )
        .await
        .unwrap();
    assert_eq!(
        world
            .start_investigation(
                user_id,
                StartInvestigation {
                    request_id,
                    kind: DiscoveryKind::ConnectedPlace,
                },
            )
            .await,
        Err(WorldError::InvestigationRequestConflict)
    );

    let before: (i64, i64, i64, i64) = sqlx::query_as(
        r#"
        SELECT (SELECT count(*) FROM entity),
               (SELECT count(*) FROM place),
               (SELECT count(*) FROM connection),
               (SELECT count(*) FROM activity)
        "#,
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(
        world
            .submit_discovery(
                user_id,
                connected_discovery(
                    attempt.attempt_id,
                    DiscoveryOriginInput::AttemptPlace,
                    DiscoveryDestinationInput::New {
                        entity: place_entity("Wrong-kind destination"),
                        position: direct_position(100, 0, 0),
                    },
                    connection(Vec::new()),
                ),
            )
            .await,
        Err(WorldError::DiscoveryAttemptUnavailable)
    );
    let after: (i64, i64, i64, i64) = sqlx::query_as(
        r#"
        SELECT (SELECT count(*) FROM entity),
               (SELECT count(*) FROM place),
               (SELECT count(*) FROM connection),
               (SELECT count(*) FROM activity)
        "#,
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(after, before);
    let consumed_by: Option<Uuid> = sqlx::query_scalar(
        "SELECT consumed_by_activity_id FROM investigation_attempt WHERE id = $1",
    )
    .bind(attempt.attempt_id.0)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert!(consumed_by.is_none());
}

#[sqlx::test(migrations = "./migration")]
async fn pre_t4_v1_activity_without_origin_retries_after_restart(pool: PgPool) {
    let (world, user_id, character, place) = entered_world(pool.clone(), vec![0.0]).await;
    let attempt = positive(&world, user_id).await;
    let input = SubmitDiscovery {
        request_id: Uuid::new_v4(),
        attempt_id: attempt.attempt_id,
        prose: "Mara remembers the original find.".to_owned(),
        result: DiscoveryResultInput::EntityAtPosition {
            name: "Old Rain Cup".to_owned(),
            description: "A find accepted before spatial grounding.".to_owned(),
            position_description: Some("Beside the north gate.".to_owned()),
            property: Vec::new(),
            r#trait: Vec::new(),
        },
    };
    let normalized = input.clone().normalize().unwrap();
    let request_fingerprint = discovery_fingerprint(&normalized);
    let actor_position = &place.position;
    let mut transaction = pool.begin().await.unwrap();
    let entity = insert_entity(
        &mut transaction,
        user_id,
        "Old Rain Cup".to_owned(),
        "A find accepted before spatial grounding.".to_owned(),
    )
    .await
    .unwrap();
    let involved = [
        (entity.id, ActivityEntityRole::Subject),
        (place.entity.id, ActivityEntityRole::Location),
    ];
    let activity_id = append_activity(
        &mut transaction,
        ActivityDraft {
            operation: ActivityOperation::SubmitDiscovery,
            requested_by_user_id: user_id,
            actor_character_entity_id: Some(character.entity.id),
            context_place_entity_id: Some(place.entity.id),
            involved: &involved,
            prose: Some("Mara remembers the original find."),
            request_id: Some(input.request_id),
            request_fingerprint: Some(&request_fingerprint),
            action_consequence: None,
        },
        "pre_t4_retry_test",
    )
    .await
    .unwrap();
    insert_root_position(
        &mut transaction,
        entity.id,
        activity_id,
        [
            actor_position.x_cm,
            actor_position.y_cm,
            actor_position.z_cm,
        ],
        Some("Beside the north gate."),
        "pre_t4_retry_test",
    )
    .await
    .unwrap();
    sqlx::query("INSERT INTO entity_location (entity_id, place_entity_id) VALUES ($1, $2)")
        .bind(entity.id.0)
        .bind(place.entity.id.0)
        .execute(&mut *transaction)
        .await
        .unwrap();
    attempt::consume(
        &mut transaction,
        attempt.attempt_id,
        activity_id,
        "pre_t4_retry_test",
    )
    .await
    .unwrap();
    advance_place_revision(
        &mut transaction,
        place.entity.id,
        activity_id,
        "pre_t4_retry_test",
    )
    .await
    .unwrap();
    transaction.commit().await.unwrap();

    let before: (i64, i64, i64, i64) = sqlx::query_as(
        r#"
        SELECT (SELECT count(*) FROM entity),
               (SELECT count(*) FROM activity),
               (SELECT count(*) FROM position),
               (SELECT count(*) FROM entity_location)
        "#,
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    let restarted = World::new(pool.clone());
    let accepted = restarted.submit_discovery(user_id, input).await.unwrap();
    let AcceptedDiscovery::EntityAtPosition {
        activity,
        entity: accepted_entity,
        position,
        place: accepted_place,
    } = accepted
    else {
        unreachable!()
    };
    assert_eq!(activity.id, activity_id);
    assert_eq!(accepted_entity, entity);
    assert_eq!(position.position_revision.activity_id(), activity_id);
    assert_eq!(accepted_place.unwrap().entity.id, place.entity.id);
    let after: (i64, i64, i64, i64) = sqlx::query_as(
        r#"
        SELECT (SELECT count(*) FROM entity),
               (SELECT count(*) FROM activity),
               (SELECT count(*) FROM position),
               (SELECT count(*) FROM entity_location)
        "#,
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(after, before);
}

fn place_entity(name: &str) -> PlaceEntityInput {
    PlaceEntityInput {
        name: name.to_owned(),
        description: format!("{name} is a deliberately established Place."),
        property: vec![PropertyInput {
            key: "surface".to_owned(),
            value: PropertyValue::Text("weathered stone".to_owned()),
        }],
        r#trait: vec![TraitInput {
            statement: format!("{name} catches the evening light."),
        }],
    }
}

fn maximal_place_entity(name: &str, key_prefix: &str) -> PlaceEntityInput {
    PlaceEntityInput {
        name: name.to_owned(),
        description: format!("{name} carries the maximum accepted initial state."),
        property: (0..100)
            .map(|index| PropertyInput {
                key: format!("{key_prefix}_{index:03}"),
                value: PropertyValue::Integer(index),
            })
            .collect(),
        r#trait: (0..100)
            .map(|index| TraitInput {
                statement: format!("{name} has exact initial trait {index:03}."),
            })
            .collect(),
    }
}

fn direct_position(x_cm: i64, y_cm: i64, z_cm: i64) -> DirectPositionInput {
    DirectPositionInput {
        x_cm,
        y_cm,
        z_cm,
        description: Some("The point is marked by a narrow cairn.".to_owned()),
    }
}

fn connection(course: Vec<ConnectionPointInput>) -> ConnectionInput {
    ConnectionInput {
        name: "Cairn Walk".to_owned(),
        description: "A deliberate path between two known points.".to_owned(),
        shape_description: (!course.is_empty())
            .then(|| "The path bends sharply through open air.".to_owned()),
        allows_reverse: true,
        course,
    }
}

async fn positive_kind(world: &World, user_id: UserId, kind: DiscoveryKind) -> InvestigationResult {
    let result = world
        .start_investigation(
            user_id,
            StartInvestigation {
                request_id: Uuid::new_v4(),
                kind,
            },
        )
        .await
        .unwrap();
    assert_eq!(result.outcome, InvestigationOutcome::Positive);
    assert_eq!(result.limit, InvestigationLimit::for_kind(kind));
    result
}

fn connected_discovery(
    attempt_id: InvestigationAttemptId,
    origin: DiscoveryOriginInput,
    destination: DiscoveryDestinationInput,
    connection: ConnectionInput,
) -> SubmitDiscovery {
    SubmitDiscovery {
        request_id: Uuid::new_v4(),
        attempt_id,
        prose: "Mara establishes one exact spatial alternative.".to_owned(),
        result: DiscoveryResultInput::ConnectedPlace {
            origin,
            destination,
            connection,
        },
    }
}

#[sqlx::test(migrations = "./migration")]
async fn entity_at_loose_position_has_no_invented_place_and_does_not_move(pool: PgPool) {
    let (world, user_id, _, _) = entered_world(pool.clone(), vec![0.0]).await;
    let before = world
        .get_character(user_id, GetEntityCurrentState::default())
        .await
        .unwrap()
        .character;
    sqlx::query("UPDATE character SET current_place_entity_id = NULL WHERE owner_user_id = $1")
        .bind(user_id.0)
        .execute(&pool)
        .await
        .unwrap();
    let attempt = positive_kind(&world, user_id, DiscoveryKind::EntityAtPosition).await;
    let accepted = world
        .submit_discovery(
            user_id,
            discovery(
                Uuid::new_v4(),
                attempt.attempt_id,
                "Mara finds cups between named Places.",
            ),
        )
        .await
        .unwrap();
    let AcceptedDiscovery::EntityAtPosition {
        activity,
        entity,
        position,
        place,
    } = accepted
    else {
        unreachable!()
    };
    assert!(place.is_none());
    assert!(activity.context_place.is_none());
    assert!(
        activity
            .involved_entity
            .iter()
            .all(|reference| reference.role != ActivityEntityRole::Location)
    );
    assert_eq!(position.position_revision.entity_id(), entity.id);
    assert_eq!(
        (position.x_cm, position.y_cm, position.z_cm),
        (
            before.position.as_ref().unwrap().x_cm,
            before.position.as_ref().unwrap().y_cm,
            before.position.as_ref().unwrap().z_cm,
        )
    );
    let location_count: i64 =
        sqlx::query_scalar("SELECT count(*) FROM entity_location WHERE entity_id = $1")
            .bind(entity.id.0)
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(location_count, 0);
    let after = world
        .get_character(user_id, GetEntityCurrentState::default())
        .await
        .unwrap()
        .character;
    assert_eq!(after.position, before.position);
    assert!(after.current_place.is_none());
}

#[sqlx::test(migrations = "./migration")]
async fn connected_place_covers_new_existing_destination_and_loose_origin_variants(pool: PgPool) {
    let (world, user_id, _, entry) = entered_world(pool.clone(), vec![0.0; 8]).await;
    let before = world
        .get_character(user_id, GetEntityCurrentState::default())
        .await
        .unwrap()
        .character;

    let first_attempt = positive_kind(&world, user_id, DiscoveryKind::ConnectedPlace).await;
    let first_input = connected_discovery(
        first_attempt.attempt_id,
        DiscoveryOriginInput::AttemptPlace,
        DiscoveryDestinationInput::New {
            entity: place_entity("Bell Meadow"),
            position: direct_position(12_000, 40, -20),
        },
        connection(Vec::new()),
    );
    let first = world
        .submit_discovery(user_id, first_input.clone())
        .await
        .unwrap();
    let AcceptedDiscovery::ConnectedPlace {
        activity,
        origin,
        destination,
        connection: first_connection,
        character,
    } = &first
    else {
        unreachable!()
    };
    assert_eq!(origin.entity.id, entry.entity.id);
    assert_eq!(character.position, before.position);
    assert_eq!(
        character.current_place.as_ref().unwrap().entity.id,
        entry.entity.id
    );
    assert_eq!(activity.involved_connection.len(), 1);
    assert_eq!(
        activity.involved_connection[0].connection_id,
        first_connection.id
    );
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
        1
    );
    let destination_id = destination.entity.id;
    sqlx::query("UPDATE character SET current_place_entity_id = NULL WHERE owner_user_id = $1")
        .bind(user_id.0)
        .execute(&pool)
        .await
        .unwrap();
    let first_retry = world
        .submit_discovery(user_id, first_input.clone())
        .await
        .unwrap();
    assert_eq!(first_retry, first);
    sqlx::query("UPDATE character SET current_place_entity_id = $1 WHERE owner_user_id = $2")
        .bind(entry.entity.id.0)
        .bind(user_id.0)
        .execute(&pool)
        .await
        .unwrap();
    let mut changed = first_input;
    let DiscoveryResultInput::ConnectedPlace {
        connection: changed_connection,
        ..
    } = &mut changed.result
    else {
        unreachable!()
    };
    changed_connection.description.push_str(" Changed.");
    assert_eq!(
        world.submit_discovery(user_id, changed).await,
        Err(WorldError::DiscoveryRequestConflict)
    );

    let place_count: i64 = sqlx::query_scalar("SELECT count(*) FROM place")
        .fetch_one(&pool)
        .await
        .unwrap();
    let existing_attempt = positive_kind(&world, user_id, DiscoveryKind::ConnectedPlace).await;
    let existing = world
        .submit_discovery(
            user_id,
            connected_discovery(
                existing_attempt.attempt_id,
                DiscoveryOriginInput::AttemptPlace,
                DiscoveryDestinationInput::Existing {
                    place_id: destination_id,
                },
                connection(Vec::new()),
            ),
        )
        .await
        .unwrap();
    let AcceptedDiscovery::ConnectedPlace {
        connection: existing_connection,
        ..
    } = existing
    else {
        unreachable!()
    };
    assert_ne!(existing_connection.id, first_connection.id);
    assert_eq!(
        sqlx::query_scalar::<_, i64>("SELECT count(*) FROM place")
            .fetch_one(&pool)
            .await
            .unwrap(),
        place_count
    );

    sqlx::query("UPDATE character SET current_place_entity_id = NULL WHERE owner_user_id = $1")
        .bind(user_id.0)
        .execute(&pool)
        .await
        .unwrap();
    let loose_existing_attempt =
        positive_kind(&world, user_id, DiscoveryKind::ConnectedPlace).await;
    let loose_existing = world
        .submit_discovery(
            user_id,
            connected_discovery(
                loose_existing_attempt.attempt_id,
                DiscoveryOriginInput::Existing {
                    place_id: entry.entity.id,
                },
                DiscoveryDestinationInput::Existing {
                    place_id: destination_id,
                },
                connection(Vec::new()),
            ),
        )
        .await
        .unwrap();
    let AcceptedDiscovery::ConnectedPlace { character, .. } = loose_existing else {
        unreachable!()
    };
    assert_eq!(character.position, before.position);
    assert_eq!(character.current_place.unwrap().entity.id, entry.entity.id);

    sqlx::query("UPDATE character SET current_place_entity_id = NULL WHERE owner_user_id = $1")
        .bind(user_id.0)
        .execute(&pool)
        .await
        .unwrap();
    let loose_new_attempt = positive_kind(&world, user_id, DiscoveryKind::ConnectedPlace).await;
    let loose_new = world
        .submit_discovery(
            user_id,
            connected_discovery(
                loose_new_attempt.attempt_id,
                DiscoveryOriginInput::New {
                    entity: place_entity("Unnamed Cairn"),
                    position_description: Some(
                        "The new origin shares the Character's exact point.".to_owned(),
                    ),
                },
                DiscoveryDestinationInput::Existing {
                    place_id: destination_id,
                },
                connection(Vec::new()),
            ),
        )
        .await
        .unwrap();
    let AcceptedDiscovery::ConnectedPlace {
        activity,
        origin,
        character,
        ..
    } = loose_new
    else {
        unreachable!()
    };
    assert!(activity.context_place.is_none());
    assert_eq!(origin.position.x_cm, before.position.as_ref().unwrap().x_cm);
    assert_eq!(character.position, before.position);
    assert_eq!(character.current_place.unwrap().entity.id, origin.entity.id);
}

#[sqlx::test(migrations = "./migration")]
async fn loose_new_origin_and_destination_keep_independent_hundred_state_bounds(pool: PgPool) {
    let (world, first_user_id, _, _) = entered_world(pool.clone(), vec![0.0; 2]).await;
    let second_user = world.create_user().await.unwrap();
    world
        .create_character(
            second_user.id,
            CreateCharacter {
                name: "Reverse Surveyor".to_owned(),
                description: "Establishes the same keys in reverse package order.".to_owned(),
                property: Vec::new(),
                r#trait: Vec::new(),
            },
        )
        .await
        .unwrap();
    world.enter_world(second_user.id).await.unwrap();
    let first_before = world
        .get_character(first_user_id, GetEntityCurrentState::default())
        .await
        .unwrap()
        .character;
    let second_before = world
        .get_character(second_user.id, GetEntityCurrentState::default())
        .await
        .unwrap()
        .character;
    sqlx::query(
        "UPDATE character SET current_place_entity_id = NULL WHERE owner_user_id = ANY($1)",
    )
    .bind([first_user_id.0, second_user.id.0])
    .execute(&pool)
    .await
    .unwrap();
    let first_attempt = positive_kind(&world, first_user_id, DiscoveryKind::ConnectedPlace).await;
    let second_attempt = positive_kind(&world, second_user.id, DiscoveryKind::ConnectedPlace).await;
    let first_input = connected_discovery(
        first_attempt.attempt_id,
        DiscoveryOriginInput::New {
            entity: maximal_place_entity("First Hundredfold Origin", "alpha_property"),
            position_description: Some("The Character's unchanged exact point.".to_owned()),
        },
        DiscoveryDestinationInput::New {
            entity: maximal_place_entity("First Hundredfold Destination", "zeta_property"),
            position: direct_position(20_000, 0, 0),
        },
        connection(Vec::new()),
    );
    let second_input = connected_discovery(
        second_attempt.attempt_id,
        DiscoveryOriginInput::New {
            entity: maximal_place_entity("Second Hundredfold Origin", "zeta_property"),
            position_description: Some("The Character's unchanged exact point.".to_owned()),
        },
        DiscoveryDestinationInput::New {
            entity: maximal_place_entity("Second Hundredfold Destination", "alpha_property"),
            position: direct_position(-20_000, 0, 0),
        },
        connection(Vec::new()),
    );
    let (first, second) = tokio::join!(
        world.submit_discovery(first_user_id, first_input.clone()),
        world.submit_discovery(second_user.id, second_input.clone()),
    );
    let first = first.unwrap();
    let second = second.unwrap();
    let mut new_entity_id = Vec::new();
    let mut activity_id = Vec::new();
    for (accepted, before) in [(&first, &first_before), (&second, &second_before)] {
        let AcceptedDiscovery::ConnectedPlace {
            activity,
            origin,
            destination,
            character,
            ..
        } = accepted
        else {
            unreachable!()
        };
        assert_eq!(activity.property_change.len(), 200);
        assert_eq!(activity.trait_change.len(), 200);
        assert_eq!(character.position, before.position);
        assert_eq!(
            character.current_place.as_ref().unwrap().entity.id,
            origin.entity.id
        );
        new_entity_id.extend([origin.entity.id.0, destination.entity.id.0]);
        activity_id.push(activity.id.0);
    }
    let counts: (i64, i64) = sqlx::query_as(
        r#"
        SELECT (SELECT count(*) FROM entity_property WHERE entity_id = ANY($1)),
               (SELECT count(*) FROM entity_trait_current WHERE entity_id = ANY($1))
        "#,
    )
    .bind(&new_entity_id)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(counts, (400, 400));
    for activity_id in activity_id {
        let history: (i64, i64) = sqlx::query_as(
            r#"
            SELECT (SELECT count(*) FROM entity_property_history WHERE activity_id = $1),
                   (SELECT count(*) FROM entity_trait_version WHERE activity_id = $1)
            "#,
        )
        .bind(activity_id)
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_eq!(history, (200, 200));
    }

    let restarted = World::new(pool);
    assert_eq!(
        restarted
            .submit_discovery(first_user_id, first_input)
            .await
            .unwrap(),
        first
    );
    assert_eq!(
        restarted
            .submit_discovery(second_user.id, second_input)
            .await
            .unwrap(),
        second
    );
}

#[sqlx::test(migrations = "./migration")]
async fn extreme_nonintersecting_course_uses_full_coordinate_contract(pool: PgPool) {
    let (world, user_id, _, _) = entered_world(pool, vec![0.0]).await;
    let attempt = positive_kind(&world, user_id, DiscoveryKind::ConnectedPlace).await;
    let maximum = MAX_COORDINATE_CM;
    let course = vec![
        ConnectionPointInput {
            x_cm: 0,
            y_cm: 0,
            z_cm: 0,
        },
        ConnectionPointInput {
            x_cm: -maximum,
            y_cm: maximum,
            z_cm: maximum,
        },
        ConnectionPointInput {
            x_cm: maximum,
            y_cm: maximum,
            z_cm: maximum,
        },
        ConnectionPointInput {
            x_cm: maximum,
            y_cm: -maximum,
            z_cm: -maximum,
        },
    ];
    let accepted = world
        .submit_discovery(
            user_id,
            connected_discovery(
                attempt.attempt_id,
                DiscoveryOriginInput::AttemptPlace,
                DiscoveryDestinationInput::New {
                    entity: place_entity("Extreme Reach"),
                    position: direct_position(maximum, -maximum, -maximum),
                },
                connection(course.clone()),
            ),
        )
        .await
        .unwrap();
    let AcceptedDiscovery::ConnectedPlace { connection, .. } = accepted else {
        unreachable!()
    };
    assert_eq!(connection.course.len(), course.len());
    assert_eq!(connection.course.last().unwrap().x_cm, maximum);
}

#[sqlx::test(migrations = "./migration")]
async fn invalid_deferred_course_rolls_back_the_complete_connected_package(pool: PgPool) {
    let (world, user_id, _, _) = entered_world(pool.clone(), vec![0.0]).await;
    sqlx::query("UPDATE character SET current_place_entity_id = NULL WHERE owner_user_id = $1")
        .bind(user_id.0)
        .execute(&pool)
        .await
        .unwrap();
    let attempt = positive_kind(&world, user_id, DiscoveryKind::ConnectedPlace).await;
    let before: (i64, i64, i64, i64, i64) = sqlx::query_as(
        r#"
        SELECT (SELECT count(*) FROM entity),
               (SELECT count(*) FROM place),
               (SELECT count(*) FROM position),
               (SELECT count(*) FROM activity),
               (SELECT count(*) FROM connection)
        "#,
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    let crossing = vec![
        ConnectionPointInput {
            x_cm: 0,
            y_cm: 0,
            z_cm: 0,
        },
        ConnectionPointInput {
            x_cm: 100,
            y_cm: 100,
            z_cm: 0,
        },
        ConnectionPointInput {
            x_cm: 0,
            y_cm: 100,
            z_cm: 0,
        },
        ConnectionPointInput {
            x_cm: 100,
            y_cm: 0,
            z_cm: 0,
        },
    ];
    assert_eq!(
        world
            .submit_discovery(
                user_id,
                connected_discovery(
                    attempt.attempt_id,
                    DiscoveryOriginInput::New {
                        entity: place_entity("Crossed Origin"),
                        position_description: None,
                    },
                    DiscoveryDestinationInput::New {
                        entity: place_entity("Crossed Reach"),
                        position: direct_position(100, 0, 0),
                    },
                    connection(crossing),
                ),
            )
            .await,
        Err(WorldError::InvalidConnection {
            field: ConnectionField::Course,
            reason: InvalidReason::InvalidFormat,
        })
    );
    let after: (i64, i64, i64, i64, i64) = sqlx::query_as(
        r#"
        SELECT (SELECT count(*) FROM entity),
               (SELECT count(*) FROM place),
               (SELECT count(*) FROM position),
               (SELECT count(*) FROM activity),
               (SELECT count(*) FROM connection)
        "#,
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(after, before);
    let consumed: Option<Uuid> = sqlx::query_scalar(
        "SELECT consumed_by_activity_id FROM investigation_attempt WHERE id = $1",
    )
    .bind(attempt.attempt_id.0)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert!(consumed.is_none());
    let current_place: Option<Uuid> = sqlx::query_scalar(
        "SELECT current_place_entity_id FROM character WHERE owner_user_id = $1",
    )
    .bind(user_id.0)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert!(current_place.is_none());
}

#[sqlx::test(migrations = "./migration")]
async fn two_users_concurrently_establish_equal_looking_connections_without_deduplication(
    pool: PgPool,
) {
    let (world, first_user, _, entry) = entered_world(pool.clone(), vec![0.0; 5]).await;
    let seed_attempt = positive_kind(&world, first_user, DiscoveryKind::ConnectedPlace).await;
    let seeded = world
        .submit_discovery(
            first_user,
            connected_discovery(
                seed_attempt.attempt_id,
                DiscoveryOriginInput::AttemptPlace,
                DiscoveryDestinationInput::New {
                    entity: place_entity("Shared Destination"),
                    position: direct_position(8_000, 0, 0),
                },
                connection(Vec::new()),
            ),
        )
        .await
        .unwrap();
    let AcceptedDiscovery::ConnectedPlace { destination, .. } = seeded else {
        unreachable!()
    };

    let second_user = world.create_user().await.unwrap();
    world
        .create_character(
            second_user.id,
            CreateCharacter {
                name: "Second Surveyor".to_owned(),
                description: "Independently confirms geography.".to_owned(),
                property: Vec::new(),
                r#trait: Vec::new(),
            },
        )
        .await
        .unwrap();
    world.enter_world(second_user.id).await.unwrap();
    let first_attempt = positive_kind(&world, first_user, DiscoveryKind::ConnectedPlace).await;
    let second_attempt = positive_kind(&world, second_user.id, DiscoveryKind::ConnectedPlace).await;
    let input = |attempt_id| {
        connected_discovery(
            attempt_id,
            DiscoveryOriginInput::AttemptPlace,
            DiscoveryDestinationInput::Existing {
                place_id: destination.entity.id,
            },
            connection(Vec::new()),
        )
    };
    let (first, second) = tokio::join!(
        world.submit_discovery(first_user, input(first_attempt.attempt_id)),
        world.submit_discovery(second_user.id, input(second_attempt.attempt_id)),
    );
    let AcceptedDiscovery::ConnectedPlace {
        connection: first, ..
    } = first.unwrap()
    else {
        unreachable!()
    };
    let AcceptedDiscovery::ConnectedPlace {
        connection: second, ..
    } = second.unwrap()
    else {
        unreachable!()
    };
    assert_ne!(first.id, second.id);
    let count: i64 = sqlx::query_scalar(
        r#"
        SELECT count(*)
        FROM connection
        WHERE source_place_entity_id = $1
          AND destination_place_entity_id = $2
          AND name = 'Cairn Walk'
        "#,
    )
    .bind(entry.entity.id.0)
    .bind(destination.entity.id.0)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(count, 3);

    let key_share_attempt =
        positive_kind(&world, second_user.id, DiscoveryKind::ConnectedPlace).await;
    let mut blocker = pool.begin().await.unwrap();
    sqlx::query(
        r#"
        SELECT id
        FROM entity
        WHERE id = ANY($1)
        ORDER BY id
        FOR KEY SHARE
        "#,
    )
    .bind([entry.entity.id.0, destination.entity.id.0])
    .fetch_all(&mut *blocker)
    .await
    .unwrap();
    let started_at = std::time::Instant::now();
    let under_key_share = world
        .submit_discovery(second_user.id, input(key_share_attempt.attempt_id))
        .await
        .unwrap();
    assert!(started_at.elapsed() < std::time::Duration::from_secs(2));
    blocker.rollback().await.unwrap();
    let AcceptedDiscovery::ConnectedPlace {
        connection: under_key_share,
        ..
    } = under_key_share
    else {
        unreachable!()
    };
    assert_ne!(under_key_share.id, first.id);
    assert_ne!(under_key_share.id, second.id);
}

#[sqlx::test(migrations = "./migration")]
async fn changed_character_position_revision_neutrally_stales_discovery(pool: PgPool) {
    let (world, user_id, _, entry) = entered_world(pool.clone(), vec![0.0; 2]).await;
    let connection_attempt = positive_kind(&world, user_id, DiscoveryKind::ConnectedPlace).await;
    let connected_input = connected_discovery(
        connection_attempt.attempt_id,
        DiscoveryOriginInput::AttemptPlace,
        DiscoveryDestinationInput::New {
            entity: place_entity("Position Successor"),
            position: direct_position(500, 0, 0),
        },
        connection(Vec::new()),
    );
    let connected = world
        .submit_discovery(user_id, connected_input.clone())
        .await
        .unwrap();
    let (destination, connection, character) = match &connected {
        AcceptedDiscovery::ConnectedPlace {
            destination,
            connection,
            character,
            ..
        } => (destination.clone(), connection.clone(), character.clone()),
        _ => unreachable!(),
    };
    let stale_attempt = positive_kind(&world, user_id, DiscoveryKind::EntityAtPosition).await;
    let old_revision = character.position.unwrap().position_revision;
    let movement_activity_id = ActivityId(Uuid::new_v4());
    let mut transaction = pool.begin().await.unwrap();
    sqlx::query(
        r#"
        INSERT INTO activity (
            id, operation, requested_by_user_id, actor_character_entity_id,
            context_place_entity_id, prose, request_id, request_fingerprint
        ) VALUES ($1, 'move_character', $2, $3, $4, NULL, $5, $6)
        "#,
    )
    .bind(movement_activity_id.0)
    .bind(user_id.0)
    .bind(character.entity.id.0)
    .bind(destination.entity.id.0)
    .bind(Uuid::new_v4())
    .bind(vec![42_u8; 32])
    .execute(&mut *transaction)
    .await
    .unwrap();
    sqlx::query(
        "INSERT INTO activity_entity (activity_id, entity_id, role) VALUES ($1, $2, 'location'), ($1, $3, 'destination')",
    )
    .bind(movement_activity_id.0)
    .bind(entry.entity.id.0)
    .bind(destination.entity.id.0)
    .execute(&mut *transaction)
    .await
    .unwrap();
    sqlx::query("INSERT INTO activity_connection (activity_id, connection_id) VALUES ($1, $2)")
        .bind(movement_activity_id.0)
        .bind(connection.id.0)
        .execute(&mut *transaction)
        .await
        .unwrap();
    append_activity_position(
        &mut transaction,
        movement_activity_id,
        ActivityPositionRole::Origin,
        old_revision,
        "stale_position_test",
    )
    .await
    .unwrap();
    sqlx::query(
        r#"
        INSERT INTO position_version (
            entity_id, activity_id, previous_activity_id,
            x_cm, y_cm, z_cm, description
        ) VALUES ($1, $2, $3, $4, $5, $6, NULL)
        "#,
    )
    .bind(character.entity.id.0)
    .bind(movement_activity_id.0)
    .bind(old_revision.activity_id().0)
    .bind(destination.position.x_cm)
    .bind(destination.position.y_cm)
    .bind(destination.position.z_cm)
    .execute(&mut *transaction)
    .await
    .unwrap();
    sqlx::query("UPDATE position SET current_activity_id = $1 WHERE entity_id = $2")
        .bind(movement_activity_id.0)
        .bind(character.entity.id.0)
        .execute(&mut *transaction)
        .await
        .unwrap();
    append_activity_position(
        &mut transaction,
        movement_activity_id,
        ActivityPositionRole::Result,
        PositionRevision::from_parts(character.entity.id, movement_activity_id),
        "stale_position_test",
    )
    .await
    .unwrap();
    sqlx::query("UPDATE character SET current_place_entity_id = $1 WHERE entity_id = $2")
        .bind(destination.entity.id.0)
        .bind(character.entity.id.0)
        .execute(&mut *transaction)
        .await
        .unwrap();
    transaction.commit().await.unwrap();

    let activity_after_movement: i64 = sqlx::query_scalar("SELECT count(*) FROM activity")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(
        world
            .submit_discovery(user_id, connected_input)
            .await
            .unwrap(),
        connected
    );
    assert_eq!(
        sqlx::query_scalar::<_, i64>("SELECT count(*) FROM activity")
            .fetch_one(&pool)
            .await
            .unwrap(),
        activity_after_movement
    );

    let before_activity: i64 = sqlx::query_scalar("SELECT count(*) FROM activity")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(
        world
            .submit_discovery(
                user_id,
                discovery(
                    Uuid::new_v4(),
                    stale_attempt.attempt_id,
                    "This stale find cannot settle.",
                ),
            )
            .await,
        Err(WorldError::DiscoveryAttemptUnavailable)
    );
    assert_eq!(
        sqlx::query_scalar::<_, i64>("SELECT count(*) FROM activity")
            .fetch_one(&pool)
            .await
            .unwrap(),
        before_activity
    );
}

#[sqlx::test(migrations = "./migration")]
async fn spatial_mutation_lock_and_statement_budgets_fail_retryably_without_writes(pool: PgPool) {
    let (world, user_id, character, _) = entered_world(pool.clone(), vec![0.0; 2]).await;
    let locked_request_id = Uuid::new_v4();
    let mut blocker = pool.begin().await.unwrap();
    sqlx::query("SELECT id FROM \"user\" WHERE id = $1 FOR UPDATE")
        .bind(user_id.0)
        .execute(&mut *blocker)
        .await
        .unwrap();
    let started_at = std::time::Instant::now();
    assert_eq!(
        world
            .start_investigation(
                user_id,
                StartInvestigation {
                    request_id: locked_request_id,
                    kind: DiscoveryKind::EntityAtPosition,
                },
            )
            .await,
        Err(WorldError::TemporarilyUnavailable)
    );
    assert!(started_at.elapsed() < std::time::Duration::from_secs(2));
    blocker.rollback().await.unwrap();
    assert_eq!(
        sqlx::query_scalar::<_, i64>(
            "SELECT count(*) FROM investigation_attempt WHERE requested_by_user_id = $1 AND request_id = $2",
        )
        .bind(user_id.0)
        .bind(locked_request_id)
        .fetch_one(&pool)
        .await
        .unwrap(),
        0
    );

    let key_share_request_id = Uuid::new_v4();
    let mut blocker = pool.begin().await.unwrap();
    sqlx::query("SELECT entity_id FROM character WHERE entity_id = $1 FOR KEY SHARE")
        .bind(character.entity.id.0)
        .execute(&mut *blocker)
        .await
        .unwrap();
    let started_at = std::time::Instant::now();
    assert_eq!(
        world
            .start_investigation(
                user_id,
                StartInvestigation {
                    request_id: key_share_request_id,
                    kind: DiscoveryKind::EntityAtPosition,
                },
            )
            .await
            .unwrap()
            .outcome,
        InvestigationOutcome::Positive
    );
    assert!(started_at.elapsed() < std::time::Duration::from_secs(2));
    blocker.rollback().await.unwrap();

    sqlx::query(
        r#"
        CREATE FUNCTION delay_investigation_insert() RETURNS trigger
        LANGUAGE plpgsql AS $$
        BEGIN
            PERFORM pg_sleep(4);
            RETURN NEW;
        END;
        $$
        "#,
    )
    .execute(&pool)
    .await
    .unwrap();
    sqlx::query(
        r#"
        CREATE TRIGGER delay_investigation_insert
        BEFORE INSERT ON investigation_attempt
        FOR EACH ROW EXECUTE FUNCTION delay_investigation_insert()
        "#,
    )
    .execute(&pool)
    .await
    .unwrap();
    let delayed_request_id = Uuid::new_v4();
    let started_at = std::time::Instant::now();
    assert_eq!(
        world
            .start_investigation(
                user_id,
                StartInvestigation {
                    request_id: delayed_request_id,
                    kind: DiscoveryKind::ConnectedPlace,
                },
            )
            .await,
        Err(WorldError::TemporarilyUnavailable)
    );
    assert!(started_at.elapsed() >= std::time::Duration::from_millis(2_500));
    assert!(started_at.elapsed() < std::time::Duration::from_secs(4));
    assert_eq!(
        sqlx::query_scalar::<_, i64>(
            "SELECT count(*) FROM investigation_attempt WHERE requested_by_user_id = $1 AND request_id = $2",
        )
        .bind(user_id.0)
        .bind(delayed_request_id)
        .fetch_one(&pool)
        .await
        .unwrap(),
        0
    );
}
