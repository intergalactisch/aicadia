use super::*;

fn character_input(name: &str) -> CreateCharacter {
    CreateCharacter {
        name: name.to_owned(),
        description: format!("{name} explores exact Connections."),
        property: Vec::new(),
        r#trait: Vec::new(),
    }
}

fn place_entity(name: &str) -> PlaceEntityInput {
    PlaceEntityInput {
        name: name.to_owned(),
        description: format!("{name} is an exact spatial destination."),
        property: Vec::new(),
        r#trait: Vec::new(),
    }
}

fn point(x_cm: i64, y_cm: i64, z_cm: i64) -> ConnectionPointInput {
    ConnectionPointInput { x_cm, y_cm, z_cm }
}

async fn setup(pool: PgPool) -> (World, User, User, Character, Character, Place) {
    let world = World::with_scripted_chance(pool, vec![0.0; 20]);
    let first_user = world.create_user().await.unwrap();
    world
        .create_character(first_user.id, character_input("First Traveller"))
        .await
        .unwrap();
    let entry = world
        .create_entry_place(
            first_user.id,
            CreateEntryPlace {
                name: "Origin A".to_owned(),
                description: "The shared travel origin.".to_owned(),
                property: Vec::new(),
                r#trait: Vec::new(),
            },
        )
        .await
        .unwrap();
    let first_character = world.enter_world(first_user.id).await.unwrap();
    let second_user = world.create_user().await.unwrap();
    world
        .create_character(second_user.id, character_input("Second Traveller"))
        .await
        .unwrap();
    let second_character = world.enter_world(second_user.id).await.unwrap();
    (
        world,
        first_user,
        second_user,
        first_character,
        second_character,
        entry,
    )
}

async fn establish_connection(
    world: &World,
    user_id: UserId,
    name: &str,
    destination: Point,
    allows_reverse: bool,
    course: Vec<ConnectionPointInput>,
) -> (Place, Connection) {
    let attempt = world
        .start_investigation(
            user_id,
            StartInvestigation {
                request_id: Uuid::new_v4(),
                kind: DiscoveryKind::ConnectedPlace,
            },
        )
        .await
        .unwrap();
    let accepted = world
        .submit_discovery(
            user_id,
            SubmitDiscovery {
                request_id: Uuid::new_v4(),
                attempt_id: attempt.attempt_id,
                prose: format!("The Agent confirms {name}."),
                result: DiscoveryResultInput::ConnectedPlace {
                    origin: DiscoveryOriginInput::AttemptPlace,
                    destination: DiscoveryDestinationInput::New {
                        entity: place_entity(name),
                        position: DirectPositionInput {
                            x_cm: destination.x,
                            y_cm: destination.y,
                            z_cm: destination.z,
                            description: None,
                        },
                    },
                    connection: ConnectionInput {
                        name: format!("Way to {name}"),
                        description: format!("One exact Connection to {name}."),
                        shape_description: None,
                        allows_reverse,
                        course,
                    },
                },
            },
        )
        .await
        .unwrap();
    match accepted {
        AcceptedDiscovery::ConnectedPlace {
            destination,
            connection,
            ..
        } => (destination, connection),
        AcceptedDiscovery::EntityAtPosition { .. } => unreachable!(),
    }
}

fn complete(
    request_id: Uuid,
    connection_id: ConnectionId,
    revision: PositionRevision,
    direction: MovementDirection,
) -> MoveCharacter {
    MoveCharacter {
        request_id,
        connection_id,
        expected_position_revision: revision,
        direction,
        target: MovementTarget::Complete,
    }
}

fn partial(
    request_id: Uuid,
    connection_id: ConnectionId,
    revision: PositionRevision,
    direction: MovementDirection,
    origin_segment_ordinal: u8,
    target_segment_ordinal: u8,
    target: Point,
) -> MoveCharacter {
    MoveCharacter {
        request_id,
        connection_id,
        expected_position_revision: revision,
        direction,
        target: MovementTarget::Partial {
            origin_segment_ordinal,
            target_segment_ordinal,
            x_cm: target.x,
            y_cm: target.y,
            z_cm: target.z,
        },
    }
}

async fn spatial_counts(pool: &PgPool, character_id: EntityId) -> (i64, i64, i64, i64) {
    sqlx::query_as(
        r#"
        SELECT (SELECT count(*) FROM activity),
               (SELECT count(*) FROM position_version WHERE entity_id = $1),
               (SELECT count(*) FROM activity_position),
               (SELECT count(*) FROM activity_connection)
        "#,
    )
    .bind(character_id.0)
    .fetch_one(pool)
    .await
    .unwrap()
}

#[test]
fn checked_geometry_uses_the_complete_coordinate_range() {
    let min = Point {
        x: -MAX_COORDINATE_CM,
        y: -MAX_COORDINATE_CM,
        z: -MAX_COORDINATE_CM,
    };
    let max = Point {
        x: MAX_COORDINATE_CM,
        y: MAX_COORDINATE_CM,
        z: MAX_COORDINATE_CM,
    };
    let middle = Point { x: 0, y: 0, z: 0 };
    assert_eq!(point_on_segment(middle, (min, max)), Ok(true));
    assert_eq!(
        point_on_segment(Point { x: 0, y: 1, z: 0 }, (min, max)),
        Ok(false)
    );
    assert!(segment_parameter(max, (min, max)).unwrap() > 0);
}

#[test]
fn movement_fingerprint_is_pinned_and_separates_every_semantic_field() {
    let base = partial(
        Uuid::from_u128(1),
        ConnectionId(Uuid::from_u128(2)),
        PositionRevision::from_parts(EntityId(Uuid::from_u128(3)), ActivityId(Uuid::from_u128(4))),
        MovementDirection::SourceToDestination,
        5,
        6,
        Point { x: 7, y: 8, z: 9 },
    );
    assert_eq!(
        movement_fingerprint(&base)
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect::<String>(),
        "33b32f1ff595070182ee93bcf4c9a068312029faa6acbf1c053f5d68aa1d9e3b"
    );
    let changed = [
        (
            "connection_id",
            partial(
                base.request_id,
                ConnectionId(Uuid::from_u128(10)),
                base.expected_position_revision,
                base.direction,
                5,
                6,
                Point { x: 7, y: 8, z: 9 },
            ),
        ),
        (
            "expected_position_revision.entity_id",
            partial(
                base.request_id,
                base.connection_id,
                PositionRevision::from_parts(
                    EntityId(Uuid::from_u128(11)),
                    base.expected_position_revision.activity_id(),
                ),
                base.direction,
                5,
                6,
                Point { x: 7, y: 8, z: 9 },
            ),
        ),
        (
            "expected_position_revision.activity_id",
            partial(
                base.request_id,
                base.connection_id,
                PositionRevision::from_parts(
                    base.expected_position_revision.entity_id(),
                    ActivityId(Uuid::from_u128(12)),
                ),
                base.direction,
                5,
                6,
                Point { x: 7, y: 8, z: 9 },
            ),
        ),
        (
            "direction",
            partial(
                base.request_id,
                base.connection_id,
                base.expected_position_revision,
                MovementDirection::DestinationToSource,
                5,
                6,
                Point { x: 7, y: 8, z: 9 },
            ),
        ),
        (
            "target tag",
            complete(
                base.request_id,
                base.connection_id,
                base.expected_position_revision,
                base.direction,
            ),
        ),
        (
            "origin_segment_ordinal",
            partial(
                base.request_id,
                base.connection_id,
                base.expected_position_revision,
                base.direction,
                13,
                6,
                Point { x: 7, y: 8, z: 9 },
            ),
        ),
        (
            "target_segment_ordinal",
            partial(
                base.request_id,
                base.connection_id,
                base.expected_position_revision,
                base.direction,
                5,
                14,
                Point { x: 7, y: 8, z: 9 },
            ),
        ),
        (
            "x_cm",
            partial(
                base.request_id,
                base.connection_id,
                base.expected_position_revision,
                base.direction,
                5,
                6,
                Point { x: 15, y: 8, z: 9 },
            ),
        ),
        (
            "y_cm",
            partial(
                base.request_id,
                base.connection_id,
                base.expected_position_revision,
                base.direction,
                5,
                6,
                Point { x: 7, y: 16, z: 9 },
            ),
        ),
        (
            "z_cm",
            partial(
                base.request_id,
                base.connection_id,
                base.expected_position_revision,
                base.direction,
                5,
                6,
                Point { x: 7, y: 8, z: 17 },
            ),
        ),
    ];
    let base_fingerprint = movement_fingerprint(&base);
    let mut observed = std::collections::HashSet::new();
    observed.insert(base_fingerprint.clone());
    for (field, changed) in changed {
        let changed_fingerprint = movement_fingerprint(&changed);
        assert_ne!(base_fingerprint, changed_fingerprint, "{field}");
        assert!(observed.insert(changed_fingerprint), "{field}");
    }
}

#[sqlx::test(migrations = "./migration")]
async fn unshaped_completion_retry_conflict_and_exact_history(pool: PgPool) {
    let (world, user, _, character, _, origin) = setup(pool.clone()).await;
    let (destination, connection) = establish_connection(
        &world,
        user.id,
        "Unshaped B",
        Point {
            x: 400,
            y: -20,
            z: 5,
        },
        true,
        Vec::new(),
    )
    .await;
    let request_id = Uuid::new_v4();
    let input = complete(
        request_id,
        connection.id,
        character.position.unwrap().position_revision,
        MovementDirection::SourceToDestination,
    );
    let before = spatial_counts(&pool, character.entity.id).await;
    let accepted = world.move_character(user.id, input).await.unwrap();
    assert_eq!(accepted.character.current_place, Some(destination.clone()));
    assert_eq!(
        accepted.character.position.as_ref().map(|position| (
            position.x_cm,
            position.y_cm,
            position.z_cm,
            position.description.clone()
        )),
        Some((400, -20, 5, None))
    );
    assert_eq!(
        accepted.activity.operation,
        ActivityOperation::MoveCharacter
    );
    assert_eq!(accepted.activity.prose, None);
    assert_eq!(
        accepted
            .activity
            .context_place
            .as_ref()
            .map(|place| place.entity.id),
        Some(destination.entity.id)
    );
    assert!(
        accepted
            .activity
            .involved_entity
            .iter()
            .any(|reference| reference.role == ActivityEntityRole::Location
                && reference.entity.id == origin.entity.id)
    );
    assert!(
        accepted
            .activity
            .involved_entity
            .iter()
            .any(
                |reference| reference.role == ActivityEntityRole::Destination
                    && reference.entity.id == destination.entity.id
            )
    );
    assert_eq!(
        accepted
            .activity
            .involved_position
            .iter()
            .filter(|reference| reference.role == ActivityPositionRole::Origin)
            .count(),
        1
    );
    assert_eq!(
        accepted
            .activity
            .involved_position
            .iter()
            .filter(|reference| reference.role == ActivityPositionRole::Result)
            .count(),
        1
    );
    assert_eq!(
        accepted.activity.involved_connection,
        vec![ActivityConnectionReference {
            connection_id: connection.id
        }]
    );
    let after = spatial_counts(&pool, character.entity.id).await;
    assert_eq!(
        after,
        (before.0 + 1, before.1 + 1, before.2 + 2, before.3 + 1)
    );
    assert_eq!(
        world.move_character(user.id, input).await.unwrap(),
        accepted
    );
    assert_eq!(spatial_counts(&pool, character.entity.id).await, after);
    assert_eq!(
        world
            .get_character(user.id, GetEntityCurrentState::default())
            .await
            .unwrap()
            .character,
        accepted.character
    );
    let history = world
        .list_activity(
            user.id,
            ListActivity {
                cursor: None,
                limit: 100,
            },
        )
        .await
        .unwrap();
    assert_eq!(
        history
            .activity
            .iter()
            .find(|activity| activity.id == accepted.activity.id),
        Some(&accepted.activity)
    );
    assert_eq!(
        world
            .move_character(
                user.id,
                complete(
                    Uuid::new_v4(),
                    connection.id,
                    accepted
                        .character
                        .position
                        .as_ref()
                        .unwrap()
                        .position_revision,
                    MovementDirection::SourceToDestination,
                ),
            )
            .await,
        Err(WorldError::MovementNoProgress)
    );
    assert_eq!(spatial_counts(&pool, character.entity.id).await, after);
    let mut changed = input;
    changed.direction = MovementDirection::DestinationToSource;
    assert_eq!(
        world.move_character(user.id, changed).await,
        Err(WorldError::MovementRequestConflict)
    );
    assert_eq!(spatial_counts(&pool, character.entity.id).await, after);
}

#[sqlx::test(migrations = "./migration")]
async fn activity_request_id_from_action_conflicts_without_movement_writes(pool: PgPool) {
    let (world, user, _, character, _, _) = setup(pool.clone()).await;
    let (_, connection) = establish_connection(
        &world,
        user.id,
        "Shared Namespace B",
        Point { x: 250, y: 0, z: 0 },
        true,
        Vec::new(),
    )
    .await;
    let grounded = world
        .get_character(user.id, GetEntityCurrentState::default())
        .await
        .unwrap();
    let shared_request_id = Uuid::new_v4();
    world
        .submit_action(
            user.id,
            SubmitAction {
                request_id: shared_request_id,
                expected_place_revision: grounded.place_revision.unwrap(),
                prose: "The traveller records one exact preparation.".to_owned(),
                consequence: ActionConsequence::ChangeEntityState(ChangeEntityState {
                    property_change: vec![EntityPropertyChangeInput {
                        entity_id: character.entity.id,
                        key: "travel_prepared".to_owned(),
                        value: PropertyValue::Integer(1),
                    }],
                    trait_change: Vec::new(),
                }),
            },
        )
        .await
        .unwrap();
    let before = spatial_counts(&pool, character.entity.id).await;
    let current = world
        .get_character(user.id, GetEntityCurrentState::default())
        .await
        .unwrap()
        .character;
    assert_eq!(
        world
            .move_character(
                user.id,
                complete(
                    shared_request_id,
                    connection.id,
                    current.position.unwrap().position_revision,
                    MovementDirection::SourceToDestination,
                ),
            )
            .await,
        Err(WorldError::MovementRequestConflict)
    );
    assert_eq!(spatial_counts(&pool, character.entity.id).await, before);
}

#[sqlx::test(migrations = "./migration")]
async fn equal_coordinate_places_remain_distinct_topological_endpoints(pool: PgPool) {
    let (world, user, _, character, _, origin) = setup(pool).await;
    let (destination, connection) = establish_connection(
        &world,
        user.id,
        "Co-located B",
        Point { x: 0, y: 0, z: 0 },
        true,
        Vec::new(),
    )
    .await;
    let arrived = world
        .move_character(
            user.id,
            complete(
                Uuid::new_v4(),
                connection.id,
                character.position.unwrap().position_revision,
                MovementDirection::SourceToDestination,
            ),
        )
        .await
        .unwrap();
    assert_eq!(arrived.character.current_place, Some(destination));
    let returned = world
        .move_character(
            user.id,
            complete(
                Uuid::new_v4(),
                connection.id,
                arrived.character.position.unwrap().position_revision,
                MovementDirection::DestinationToSource,
            ),
        )
        .await
        .unwrap();
    assert_eq!(returned.character.current_place, Some(origin));
    assert_eq!(
        returned
            .character
            .position
            .map(|position| (position.x_cm, position.y_cm, position.z_cm)),
        Some((0, 0, 0))
    );
}

#[sqlx::test(migrations = "./migration")]
async fn corrupted_retry_origin_cannot_disagree_with_result_predecessor(pool: PgPool) {
    let (world, user, _, character, _, _) = setup(pool.clone()).await;
    let root_revision = character.position.unwrap().position_revision;
    let (_, connection) = establish_connection(
        &world,
        user.id,
        "Corruption B",
        Point { x: 100, y: 0, z: 0 },
        true,
        Vec::new(),
    )
    .await;
    let forward = world
        .move_character(
            user.id,
            complete(
                Uuid::new_v4(),
                connection.id,
                root_revision,
                MovementDirection::SourceToDestination,
            ),
        )
        .await
        .unwrap();
    let retry_input = complete(
        Uuid::new_v4(),
        connection.id,
        forward.character.position.unwrap().position_revision,
        MovementDirection::DestinationToSource,
    );
    let accepted = world.move_character(user.id, retry_input).await.unwrap();
    let before = spatial_counts(&pool, character.entity.id).await;

    // Exact storage-corruption falsificator: accepted spatial history is immutable
    // in production. This disposable database briefly disables only that guard to
    // prove retry reconstruction does not trust a mismatched typed origin.
    sqlx::raw_sql("ALTER TABLE activity_position DISABLE TRIGGER activity_position_immutable")
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query(
        r#"
        UPDATE activity_position
        SET position_activity_id = $1
        WHERE activity_id = $2
          AND role = 'origin'
          AND position_entity_id = $3
        "#,
    )
    .bind(root_revision.activity_id().0)
    .bind(accepted.activity.id.0)
    .bind(character.entity.id.0)
    .execute(&pool)
    .await
    .unwrap();
    sqlx::raw_sql("ALTER TABLE activity_position ENABLE TRIGGER activity_position_immutable")
        .execute(&pool)
        .await
        .unwrap();

    assert_eq!(
        world.move_character(user.id, retry_input).await,
        Err(WorldError::Unavailable)
    );
    assert_eq!(spatial_counts(&pool, character.entity.id).await, before);
}

#[sqlx::test(migrations = "./migration")]
async fn corrupted_movement_result_description_fails_retry_closed(pool: PgPool) {
    let (world, user, _, character, _, _) = setup(pool.clone()).await;
    let (_, connection) = establish_connection(
        &world,
        user.id,
        "Description Corruption B",
        Point { x: 100, y: 0, z: 0 },
        true,
        Vec::new(),
    )
    .await;
    let retry_input = complete(
        Uuid::new_v4(),
        connection.id,
        character.position.unwrap().position_revision,
        MovementDirection::SourceToDestination,
    );
    let accepted = world.move_character(user.id, retry_input).await.unwrap();
    let before = spatial_counts(&pool, character.entity.id).await;

    // Exact storage-corruption falsificator: bypass the immutable-version guard
    // only in this disposable database and prove Movement retry rejects prose that
    // the accepted Movement writer can never create.
    sqlx::raw_sql("ALTER TABLE position_version DISABLE TRIGGER position_version_immutable")
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query(
        "UPDATE position_version SET description = 'corrupted' WHERE entity_id = $1 AND activity_id = $2",
    )
    .bind(character.entity.id.0)
    .bind(accepted.activity.id.0)
    .execute(&pool)
    .await
    .unwrap();
    sqlx::raw_sql("ALTER TABLE position_version ENABLE TRIGGER position_version_immutable")
        .execute(&pool)
        .await
        .unwrap();

    assert_eq!(
        world.move_character(user.id, retry_input).await,
        Err(WorldError::Unavailable)
    );
    assert_eq!(spatial_counts(&pool, character.entity.id).await, before);
}

#[sqlx::test(migrations = "./migration")]
async fn shaped_partial_complete_reverse_and_rejections_are_exact(pool: PgPool) {
    let (world, first_user, second_user, first_character, second_character, origin) =
        setup(pool.clone()).await;
    let (destination, connection) = establish_connection(
        &world,
        first_user.id,
        "Shaped B",
        Point {
            x: 200,
            y: 100,
            z: 0,
        },
        true,
        vec![point(0, 0, 0), point(100, 100, 0), point(200, 100, 0)],
    )
    .await;
    let initial_revision = first_character.position.unwrap().position_revision;
    let direct_forward = world
        .move_character(
            second_user.id,
            complete(
                Uuid::new_v4(),
                connection.id,
                second_character.position.unwrap().position_revision,
                MovementDirection::SourceToDestination,
            ),
        )
        .await
        .unwrap();
    assert_eq!(
        direct_forward.character.current_place,
        Some(destination.clone())
    );
    let direct_reverse = world
        .move_character(
            second_user.id,
            complete(
                Uuid::new_v4(),
                connection.id,
                direct_forward.character.position.unwrap().position_revision,
                MovementDirection::DestinationToSource,
            ),
        )
        .await
        .unwrap();
    assert_eq!(direct_reverse.character.current_place, Some(origin.clone()));
    let baseline = spatial_counts(&pool, first_character.entity.id).await;
    let off_course = partial(
        Uuid::new_v4(),
        connection.id,
        initial_revision,
        MovementDirection::SourceToDestination,
        0,
        0,
        Point { x: 50, y: 49, z: 0 },
    );
    assert_eq!(
        world.move_character(first_user.id, off_course).await,
        Err(WorldError::MovementOffCourse)
    );
    let backwards = partial(
        Uuid::new_v4(),
        connection.id,
        initial_revision,
        MovementDirection::SourceToDestination,
        0,
        0,
        Point { x: 0, y: 0, z: 0 },
    );
    assert_eq!(
        world.move_character(first_user.id, backwards).await,
        Err(WorldError::InvalidMovement {
            field: MovementField::Target,
            reason: InvalidReason::InvalidFormat
        })
    );
    assert_eq!(
        spatial_counts(&pool, first_character.entity.id).await,
        baseline
    );

    let partial_input = partial(
        Uuid::new_v4(),
        connection.id,
        initial_revision,
        MovementDirection::SourceToDestination,
        0,
        1,
        Point {
            x: 150,
            y: 100,
            z: 0,
        },
    );
    let stopped = world
        .move_character(first_user.id, partial_input)
        .await
        .unwrap();
    assert_eq!(stopped.character.current_place, None);
    let stopped_position = stopped.character.position.clone().unwrap();
    assert_eq!(
        (
            stopped_position.x_cm,
            stopped_position.y_cm,
            stopped_position.z_cm
        ),
        (150, 100, 0)
    );
    let no_progress = partial(
        Uuid::new_v4(),
        connection.id,
        stopped_position.position_revision,
        MovementDirection::SourceToDestination,
        1,
        1,
        Point {
            x: 125,
            y: 100,
            z: 0,
        },
    );
    let before_no_progress = spatial_counts(&pool, first_character.entity.id).await;
    assert_eq!(
        world.move_character(first_user.id, no_progress).await,
        Err(WorldError::MovementNoProgress)
    );
    assert_eq!(
        spatial_counts(&pool, first_character.entity.id).await,
        before_no_progress
    );

    let arrived = world
        .move_character(
            first_user.id,
            complete(
                Uuid::new_v4(),
                connection.id,
                stopped_position.position_revision,
                MovementDirection::SourceToDestination,
            ),
        )
        .await
        .unwrap();
    assert_eq!(arrived.character.current_place, Some(destination.clone()));
    assert_eq!(
        world
            .move_character(first_user.id, partial_input)
            .await
            .unwrap(),
        stopped
    );

    let reverse_stop = world
        .move_character(
            first_user.id,
            partial(
                Uuid::new_v4(),
                connection.id,
                arrived.character.position.unwrap().position_revision,
                MovementDirection::DestinationToSource,
                1,
                0,
                Point {
                    x: 100,
                    y: 100,
                    z: 0,
                },
            ),
        )
        .await
        .unwrap();
    assert_eq!(reverse_stop.character.current_place, None);
    let returned = world
        .move_character(
            first_user.id,
            complete(
                Uuid::new_v4(),
                connection.id,
                reverse_stop.character.position.unwrap().position_revision,
                MovementDirection::DestinationToSource,
            ),
        )
        .await
        .unwrap();
    assert_eq!(returned.character.current_place, Some(origin));
    assert_eq!(
        returned.character.position.as_ref().map(|position| (
            position.x_cm,
            position.y_cm,
            position.z_cm
        )),
        Some((0, 0, 0))
    );
}

#[sqlx::test(migrations = "./migration")]
async fn stale_wrong_direction_place_and_deferred_failure_write_nothing(pool: PgPool) {
    let (world, user, _, character, _, _) = setup(pool.clone()).await;
    let (_, one_way) = establish_connection(
        &world,
        user.id,
        "One-way B",
        Point { x: 100, y: 0, z: 0 },
        false,
        Vec::new(),
    )
    .await;
    let revision = character.position.unwrap().position_revision;
    let origin_place_id = one_way.source.place.id;
    let baseline = spatial_counts(&pool, character.entity.id).await;
    assert_eq!(
        world
            .move_character(
                user.id,
                complete(
                    Uuid::new_v4(),
                    one_way.id,
                    revision,
                    MovementDirection::DestinationToSource
                )
            )
            .await,
        Err(WorldError::ConnectionDirectionDisallowed)
    );
    assert_eq!(spatial_counts(&pool, character.entity.id).await, baseline);
    sqlx::query("UPDATE character SET current_place_entity_id = NULL WHERE entity_id = $1")
        .bind(character.entity.id.0)
        .execute(&pool)
        .await
        .unwrap();
    assert_eq!(
        world
            .move_character(
                user.id,
                complete(
                    Uuid::new_v4(),
                    one_way.id,
                    revision,
                    MovementDirection::SourceToDestination
                )
            )
            .await,
        Err(WorldError::ConnectionUnavailable)
    );
    sqlx::query("UPDATE character SET current_place_entity_id = $1 WHERE entity_id = $2")
        .bind(one_way.source.place.id.0)
        .bind(character.entity.id.0)
        .execute(&pool)
        .await
        .unwrap();
    assert_eq!(spatial_counts(&pool, character.entity.id).await, baseline);

    let stale = complete(
        Uuid::new_v4(),
        one_way.id,
        PositionRevision::from_parts(character.entity.id, ActivityId(Uuid::new_v4())),
        MovementDirection::SourceToDestination,
    );
    assert_eq!(
        world.move_character(user.id, stale).await,
        Err(WorldError::PositionRevisionConflict)
    );
    assert_eq!(spatial_counts(&pool, character.entity.id).await, baseline);

    sqlx::raw_sql(
        r#"
        CREATE FUNCTION fail_movement_commit() RETURNS trigger LANGUAGE plpgsql AS $$
        BEGIN
            RAISE EXCEPTION 'injected deferred Movement failure';
        END;
        $$;
        CREATE CONSTRAINT TRIGGER fail_movement_commit
            AFTER INSERT ON position_version
            DEFERRABLE INITIALLY DEFERRED
            FOR EACH ROW
            WHEN (NEW.previous_activity_id IS NOT NULL)
            EXECUTE FUNCTION fail_movement_commit();
        "#,
    )
    .execute(&pool)
    .await
    .unwrap();
    assert_eq!(
        world
            .move_character(
                user.id,
                complete(
                    Uuid::new_v4(),
                    one_way.id,
                    revision,
                    MovementDirection::SourceToDestination
                )
            )
            .await,
        Err(WorldError::Unavailable)
    );
    assert_eq!(spatial_counts(&pool, character.entity.id).await, baseline);
    let current = world
        .get_character(user.id, GetEntityCurrentState::default())
        .await
        .unwrap()
        .character;
    assert_eq!(current.position.unwrap().position_revision, revision);
    assert_eq!(
        current.current_place.map(|place| place.entity.id),
        Some(origin_place_id)
    );
}

#[sqlx::test(migrations = "./migration")]
async fn same_character_serializes_while_other_traveller_remains_independent(pool: PgPool) {
    let (world, first_user, second_user, first_character, second_character, _) =
        setup(pool.clone()).await;
    let (_, connection) = establish_connection(
        &world,
        first_user.id,
        "Concurrent B",
        Point { x: 300, y: 0, z: 0 },
        true,
        Vec::new(),
    )
    .await;
    let first_revision = first_character.position.unwrap().position_revision;
    let mut blocker = pool.begin().await.unwrap();
    sqlx::query("SELECT entity_id FROM character WHERE entity_id = $1 FOR UPDATE")
        .bind(first_character.entity.id.0)
        .fetch_one(&mut *blocker)
        .await
        .unwrap();
    let blocked_world = world.clone();
    let blocked = tokio::spawn(async move {
        let started = std::time::Instant::now();
        let result = blocked_world
            .move_character(
                first_user.id,
                complete(
                    Uuid::new_v4(),
                    connection.id,
                    first_revision,
                    MovementDirection::SourceToDestination,
                ),
            )
            .await;
        (result, started.elapsed())
    });
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    let started = std::time::Instant::now();
    let independent = world
        .move_character(
            second_user.id,
            complete(
                Uuid::new_v4(),
                connection.id,
                second_character.position.unwrap().position_revision,
                MovementDirection::SourceToDestination,
            ),
        )
        .await
        .unwrap();
    assert!(started.elapsed() < std::time::Duration::from_secs(2));
    let (blocked_result, blocked_elapsed) = blocked.await.unwrap();
    assert_eq!(blocked_result, Err(WorldError::TemporarilyUnavailable));
    assert!(blocked_elapsed >= std::time::Duration::from_millis(400));
    assert!(blocked_elapsed < std::time::Duration::from_secs(2));
    blocker.rollback().await.unwrap();
    assert_eq!(independent.connection.id, connection.id);

    let current = world
        .get_character(first_user.id, GetEntityCurrentState::default())
        .await
        .unwrap()
        .character;
    let revision = current.position.unwrap().position_revision;
    let left = world.clone();
    let right = world.clone();
    let left_input = complete(
        Uuid::new_v4(),
        connection.id,
        revision,
        MovementDirection::SourceToDestination,
    );
    let right_input = complete(
        Uuid::new_v4(),
        connection.id,
        revision,
        MovementDirection::SourceToDestination,
    );
    let (left_result, right_result) = tokio::join!(
        left.move_character(first_user.id, left_input),
        right.move_character(first_user.id, right_input),
    );
    assert!(matches!(
        (&left_result, &right_result),
        (Ok(_), Err(WorldError::PositionRevisionConflict))
            | (Err(WorldError::PositionRevisionConflict), Ok(_))
    ));
}

#[sqlx::test(migrations = "./migration")]
async fn extreme_shaped_partial_uses_no_float_or_narrowing(pool: PgPool) {
    let (world, user, _, character, _, _) = setup(pool).await;
    let (_, connection) = establish_connection(
        &world,
        user.id,
        "Extreme B",
        Point {
            x: MAX_COORDINATE_CM,
            y: MAX_COORDINATE_CM,
            z: MAX_COORDINATE_CM,
        },
        true,
        vec![
            point(0, 0, 0),
            point(MAX_COORDINATE_CM, MAX_COORDINATE_CM, MAX_COORDINATE_CM),
        ],
    )
    .await;
    let accepted = world
        .move_character(
            user.id,
            partial(
                Uuid::new_v4(),
                connection.id,
                character.position.unwrap().position_revision,
                MovementDirection::SourceToDestination,
                0,
                0,
                Point {
                    x: 500_000_000_000_000,
                    y: 500_000_000_000_000,
                    z: 500_000_000_000_000,
                },
            ),
        )
        .await
        .unwrap();
    assert_eq!(
        accepted.character.position.as_ref().map(|position| (
            position.x_cm,
            position.y_cm,
            position.z_cm
        )),
        Some((
            500_000_000_000_000,
            500_000_000_000_000,
            500_000_000_000_000
        ))
    );
}
