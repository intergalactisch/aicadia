use super::*;
use aicadia::{ActivityId, ConnectionId};

#[derive(Clone, Copy)]
struct RawPlace {
    entity_id: EntityId,
    activity_id: ActivityId,
}

async fn entered_reader(pool: &PgPool) -> (World, UserId, EntityId, RawPlace) {
    let world = World::new(pool.clone());
    let user_id = create_user(&world).await;
    let character = world
        .create_character(user_id, character("Mara Venn"))
        .await
        .unwrap();
    world
        .create_entry_place(user_id, place("First Marker"))
        .await
        .unwrap();
    let entered = world.enter_world(user_id).await.unwrap();
    let entry = entered.current_place.unwrap();
    (
        world,
        user_id,
        character.entity.id,
        RawPlace {
            entity_id: entry.entity.id,
            activity_id: entry.position.position_revision.activity_id(),
        },
    )
}

async fn insert_place(
    pool: &PgPool,
    user_id: UserId,
    actor_id: EntityId,
    context_place_id: EntityId,
    entity_id: EntityId,
    name: &str,
    point: (i64, i64, i64),
) -> RawPlace {
    let activity_id = ActivityId(Uuid::new_v4());
    let mut transaction = pool.begin().await.unwrap();
    sqlx::query(
        "INSERT INTO entity (id, name, description, introduced_by_user_id) VALUES ($1, $2, $3, $4)",
    )
    .bind(entity_id.0)
    .bind(name)
    .bind(format!("Exact description of {name}."))
    .bind(user_id.0)
    .execute(&mut *transaction)
    .await
    .unwrap();
    sqlx::query(
        r#"
        INSERT INTO activity (
            id, operation, requested_by_user_id, actor_character_entity_id,
            context_place_entity_id, prose, request_id, request_fingerprint
        ) VALUES ($1, 'submit_discovery', $2, $3, $4, $5, $6, $7)
        "#,
    )
    .bind(activity_id.0)
    .bind(user_id.0)
    .bind(actor_id.0)
    .bind(context_place_id.0)
    .bind(format!("{name} is established."))
    .bind(Uuid::new_v4())
    .bind(vec![3_u8; 32])
    .execute(&mut *transaction)
    .await
    .unwrap();
    sqlx::query(
        "INSERT INTO activity_entity (activity_id, entity_id, role) VALUES ($1, $2, 'subject'), ($1, $2, 'destination')",
    )
    .bind(activity_id.0)
    .bind(entity_id.0)
    .execute(&mut *transaction)
    .await
    .unwrap();
    sqlx::query(
        "INSERT INTO position_version (entity_id, activity_id, x_cm, y_cm, z_cm, description) VALUES ($1, $2, $3, $4, $5, $6)",
    )
    .bind(entity_id.0)
    .bind(activity_id.0)
    .bind(point.0)
    .bind(point.1)
    .bind(point.2)
    .bind(format!("{name} is fixed at its surveyed point."))
    .execute(&mut *transaction)
    .await
    .unwrap();
    sqlx::query("INSERT INTO position (entity_id, current_activity_id) VALUES ($1, $2)")
        .bind(entity_id.0)
        .bind(activity_id.0)
        .execute(&mut *transaction)
        .await
        .unwrap();
    sqlx::query(
        "INSERT INTO activity_position (activity_id, role, position_entity_id, position_activity_id) VALUES ($1, 'result', $2, $1)",
    )
    .bind(activity_id.0)
    .bind(entity_id.0)
    .execute(&mut *transaction)
    .await
    .unwrap();
    sqlx::query(
        "INSERT INTO place (entity_id, is_entry, latest_activity_id) VALUES ($1, false, $2)",
    )
    .bind(entity_id.0)
    .bind(activity_id.0)
    .execute(&mut *transaction)
    .await
    .unwrap();
    sqlx::query(
        "INSERT INTO place_map_index (place_entity_id, position_activity_id, x_cm, y_cm, z_cm) VALUES ($1, $2, $3, $4, $5)",
    )
    .bind(entity_id.0)
    .bind(activity_id.0)
    .bind(point.0)
    .bind(point.1)
    .bind(point.2)
    .execute(&mut *transaction)
    .await
    .unwrap();
    transaction.commit().await.unwrap();
    RawPlace {
        entity_id,
        activity_id,
    }
}

async fn insert_connection(
    pool: &PgPool,
    user_id: UserId,
    actor_id: EntityId,
    connection_id: ConnectionId,
    source: RawPlace,
    destination: RawPlace,
    allows_reverse: bool,
    point: &[(i16, i64, i64, i64)],
    marker: u8,
) {
    let activity_id = ActivityId(Uuid::new_v4());
    let mut transaction = pool.begin().await.unwrap();
    sqlx::query(
        r#"
        INSERT INTO activity (
            id, operation, requested_by_user_id, actor_character_entity_id,
            context_place_entity_id, prose, request_id, request_fingerprint
        ) VALUES ($1, 'submit_discovery', $2, $3, $4, $5, $6, $7)
        "#,
    )
    .bind(activity_id.0)
    .bind(user_id.0)
    .bind(actor_id.0)
    .bind(source.entity_id.0)
    .bind(format!("Connection {marker} is established."))
    .bind(Uuid::new_v4())
    .bind(vec![marker; 32])
    .execute(&mut *transaction)
    .await
    .unwrap();
    sqlx::query(
        r#"
        INSERT INTO connection (
            id, source_place_entity_id, destination_place_entity_id,
            source_position_activity_id, destination_position_activity_id,
            allows_reverse, has_course, name, description, shape_description,
            created_by_activity_id
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        "#,
    )
    .bind(connection_id.0)
    .bind(source.entity_id.0)
    .bind(destination.entity_id.0)
    .bind(source.activity_id.0)
    .bind(destination.activity_id.0)
    .bind(allows_reverse)
    .bind(!point.is_empty())
    .bind(format!("Connection {marker}"))
    .bind(format!("Exact description of Connection {marker}."))
    .bind((!point.is_empty()).then(|| format!("Shape {marker}.")))
    .bind(activity_id.0)
    .execute(&mut *transaction)
    .await
    .unwrap();
    if !point.is_empty() {
        let ordinal: Vec<_> = point.iter().map(|point| point.0).collect();
        let x_cm: Vec<_> = point.iter().map(|point| point.1).collect();
        let y_cm: Vec<_> = point.iter().map(|point| point.2).collect();
        let z_cm: Vec<_> = point.iter().map(|point| point.3).collect();
        sqlx::query(
            r#"
            INSERT INTO connection_point (connection_id, ordinal, x_cm, y_cm, z_cm)
            SELECT $1, point.ordinal, point.x_cm, point.y_cm, point.z_cm
            FROM unnest($2::smallint[], $3::bigint[], $4::bigint[], $5::bigint[])
                 AS point(ordinal, x_cm, y_cm, z_cm)
            "#,
        )
        .bind(connection_id.0)
        .bind(&ordinal)
        .bind(&x_cm)
        .bind(&y_cm)
        .bind(&z_cm)
        .execute(&mut *transaction)
        .await
        .unwrap();
    }
    sqlx::query("INSERT INTO activity_connection (activity_id, connection_id) VALUES ($1, $2)")
        .bind(activity_id.0)
        .bind(connection_id.0)
        .execute(&mut *transaction)
        .await
        .unwrap();
    transaction.commit().await.unwrap();
}

fn place_window(x: i64, cursor: Option<String>, limit: u16) -> ListPlace {
    ListPlace {
        min_x_cm: x,
        max_x_cm: x,
        min_y_cm: 0,
        max_y_cm: 0,
        min_z_cm: 0,
        max_z_cm: 0,
        cursor,
        limit,
    }
}

#[sqlx::test(migrations = "./migration")]
async fn place_read_pages_exact_positions_and_rejects_stale_projection(pool: PgPool) {
    let (world, user_id, actor_id, entry) = entered_reader(&pool).await;
    let ids = [
        "10000000-0000-0000-0000-000000000001",
        "10000000-0000-0000-0000-000000000002",
        "10000000-0000-0000-0000-000000000003",
        "10000000-0000-0000-0000-000000000004",
    ];
    let mut place = Vec::new();
    for (index, id) in ids.into_iter().enumerate() {
        let x_cm = if index == 1 { 11 } else { 10 };
        place.push(
            insert_place(
                &pool,
                user_id,
                actor_id,
                entry.entity_id,
                EntityId(Uuid::parse_str(id).unwrap()),
                &format!("Survey {index}"),
                (x_cm, 0, 0),
            )
            .await,
        );
    }
    // This fixed-id projection sorts between the first and third valid candidates,
    // but its canonical Position remains at x=11 and must fail exact hydration.
    sqlx::query("UPDATE place_map_index SET x_cm = 10 WHERE place_entity_id = $1")
        .bind(place[1].entity_id.0)
        .execute(&pool)
        .await
        .unwrap();
    let before: i64 = sqlx::query_scalar("SELECT count(*) FROM activity")
        .fetch_one(&pool)
        .await
        .unwrap();

    let first = world
        .list_place(user_id, place_window(10, None, 2))
        .await
        .unwrap();
    assert_eq!(
        first.place.iter().map(|item| item.id).collect::<Vec<_>>(),
        vec![place[0].entity_id]
    );
    assert!(
        first.next.is_some(),
        "the stale candidate must advance the cursor"
    );
    assert!(first.place.iter().all(|item| {
        item.position.x_cm == 10
            && item.position.description.as_deref()
                == Some(&format!("{} is fixed at its surveyed point.", item.name))
    }));
    let second = world
        .list_place(user_id, place_window(10, first.next.clone(), 2))
        .await
        .unwrap();
    assert_eq!(
        second.place.iter().map(|item| item.id).collect::<Vec<_>>(),
        vec![place[2].entity_id, place[3].entity_id]
    );
    assert_eq!(second.next, None);
    assert!(!first.place.iter().any(|item| item.id == entry.entity_id));
    assert_eq!(
        world
            .list_place(user_id, place_window(11, first.next, 2))
            .await,
        Err(WorldError::InvalidRequest)
    );
    let after: i64 = sqlx::query_scalar("SELECT count(*) FROM activity")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(after, before);
}

#[sqlx::test(migrations = "./migration")]
async fn place_read_validates_window_limit_and_positioned_character(pool: PgPool) {
    let (world, user_id, _, _) = entered_reader(&pool).await;
    assert_eq!(
        world
            .list_place(UserId(Uuid::new_v4()), place_window(0, None, 1))
            .await,
        Err(WorldError::UserNotFound)
    );
    assert_eq!(
        world.list_place(user_id, place_window(0, None, 0)).await,
        Err(WorldError::InvalidPlaceLimit)
    );
    assert_eq!(
        world
            .list_place(user_id, place_window(0, Some("A".repeat(513)), 1))
            .await,
        Err(WorldError::InvalidRequest)
    );
    let mut invalid = place_window(0, None, 1);
    invalid.max_x_cm = 100_000_001;
    assert_eq!(
        world.list_place(user_id, invalid).await,
        Err(WorldError::InvalidPlaceWindow)
    );
    assert_eq!(
        world
            .list_place(user_id, place_window(0, Some("foreign".to_owned()), 1))
            .await,
        Err(WorldError::InvalidRequest)
    );
    let no_character = create_user(&world).await;
    assert_eq!(
        world
            .list_place(no_character, place_window(0, None, 1))
            .await,
        Err(WorldError::CharacterNotFound)
    );
    let not_entered = create_user(&world).await;
    world
        .create_character(not_entered, character("Waiting Character"))
        .await
        .unwrap();
    assert_eq!(
        world
            .list_place(not_entered, place_window(0, None, 1))
            .await,
        Err(WorldError::CharacterNotEntered)
    );
}

#[sqlx::test(migrations = "./migration")]
async fn spatial_read_statement_budget_is_retryable_and_writes_nothing(pool: PgPool) {
    let (world, user_id, _, _) = entered_reader(&pool).await;
    let before: i64 = sqlx::query_scalar("SELECT count(*) FROM activity")
        .fetch_one(&pool)
        .await
        .unwrap();
    let mut blocker = pool.begin().await.unwrap();
    sqlx::query("LOCK TABLE place_map_index IN ACCESS EXCLUSIVE MODE")
        .execute(&mut *blocker)
        .await
        .unwrap();
    let result = tokio::time::timeout(
        std::time::Duration::from_secs(5),
        world.list_place(user_id, place_window(0, None, 1)),
    )
    .await
    .expect("the three-second statement budget must end the blocked read");
    assert_eq!(result, Err(WorldError::TemporarilyUnavailable));
    blocker.rollback().await.unwrap();

    let mut auth_blocker = pool.begin().await.unwrap();
    sqlx::query(r#"LOCK TABLE "user" IN ACCESS EXCLUSIVE MODE"#)
        .execute(&mut *auth_blocker)
        .await
        .unwrap();
    let auth_result = tokio::time::timeout(
        std::time::Duration::from_secs(5),
        world.list_place(user_id, place_window(0, None, 1)),
    )
    .await
    .expect("the three-second statement budget must include reader authorization");
    assert_eq!(auth_result, Err(WorldError::TemporarilyUnavailable));
    auth_blocker.rollback().await.unwrap();

    let after: i64 = sqlx::query_scalar("SELECT count(*) FROM activity")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(after, before);
}

#[sqlx::test(migrations = "./migration")]
async fn place_cursor_has_stable_progress_across_concurrent_inserts(pool: PgPool) {
    let (world, user_id, actor_id, entry) = entered_reader(&pool).await;
    for suffix in [2_u128, 4, 6] {
        insert_place(
            &pool,
            user_id,
            actor_id,
            entry.entity_id,
            EntityId(Uuid::from_u128(suffix)),
            &format!("Initial {suffix}"),
            (20, 0, 0),
        )
        .await;
    }
    let first = world
        .list_place(user_id, place_window(20, None, 2))
        .await
        .unwrap();
    assert_eq!(
        first
            .place
            .iter()
            .map(|place| place.id.0)
            .collect::<Vec<_>>(),
        vec![Uuid::from_u128(2), Uuid::from_u128(4)]
    );
    for suffix in [3_u128, 5] {
        insert_place(
            &pool,
            user_id,
            actor_id,
            entry.entity_id,
            EntityId(Uuid::from_u128(suffix)),
            &format!("Concurrent {suffix}"),
            (20, 0, 0),
        )
        .await;
    }
    let continued = world
        .list_place(user_id, place_window(20, first.next, 100))
        .await
        .unwrap();
    assert_eq!(
        continued
            .place
            .iter()
            .map(|place| place.id.0)
            .collect::<Vec<_>>(),
        vec![Uuid::from_u128(5), Uuid::from_u128(6)]
    );
    assert_eq!(
        world
            .list_place(user_id, place_window(20, None, 100))
            .await
            .unwrap()
            .place
            .len(),
        5
    );
}

#[sqlx::test(migrations = "./migration")]
async fn dense_hot_point_hydrates_at_most_one_hundred_places(pool: PgPool) {
    let (world, user_id, actor_id, entry) = entered_reader(&pool).await;
    for suffix in 1_u128..=150 {
        insert_place(
            &pool,
            user_id,
            actor_id,
            entry.entity_id,
            EntityId(Uuid::from_u128(1_000 + suffix)),
            &format!("Dense {suffix}"),
            (30, 0, 0),
        )
        .await;
    }
    let first = world
        .list_place(user_id, place_window(30, None, 100))
        .await
        .unwrap();
    assert_eq!(first.place.len(), 100);
    assert!(first.next.is_some());
    let second = world
        .list_place(user_id, place_window(30, first.next, 100))
        .await
        .unwrap();
    assert_eq!(second.place.len(), 50);
    assert_eq!(second.next, None);
}

#[sqlx::test(migrations = "./migration")]
async fn connection_list_is_incident_paged_complete_and_course_free(pool: PgPool) {
    let (world, user_id, actor_id, entry) = entered_reader(&pool).await;
    let b = insert_place(
        &pool,
        user_id,
        actor_id,
        entry.entity_id,
        EntityId(Uuid::from_u128(100)),
        "Dune B",
        (100, 0, 0),
    )
    .await;
    let c = insert_place(
        &pool,
        user_id,
        actor_id,
        entry.entity_id,
        EntityId(Uuid::from_u128(101)),
        "Dune C",
        (200, 0, 0),
    )
    .await;
    for (id, source, destination, marker) in
        [(10, entry, b, 10), (20, b, entry, 20), (30, entry, c, 30)]
    {
        let start = if source.entity_id == entry.entity_id {
            0
        } else {
            100
        };
        let end = if destination.entity_id == entry.entity_id {
            0
        } else if destination.entity_id == b.entity_id {
            100
        } else {
            200
        };
        insert_connection(
            &pool,
            user_id,
            actor_id,
            ConnectionId(Uuid::from_u128(id)),
            source,
            destination,
            true,
            &[(0, start, 0, 0), (1, end, 0, 0)],
            marker,
        )
        .await;
    }
    let before: i64 = sqlx::query_scalar("SELECT count(*) FROM activity")
        .fetch_one(&pool)
        .await
        .unwrap();
    let first = world
        .list_connection(
            user_id,
            ListConnection {
                place_id: entry.entity_id,
                cursor: None,
                limit: 2,
            },
        )
        .await
        .unwrap();
    assert_eq!(first.place.id, entry.entity_id);
    assert_eq!(
        first
            .connection
            .iter()
            .map(|connection| connection.id.0)
            .collect::<Vec<_>>(),
        vec![Uuid::from_u128(10), Uuid::from_u128(20)]
    );
    assert!(first.connection.iter().all(|connection| {
        connection.has_course
            && connection
                .source
                .place
                .position
                .position_revision
                .entity_id()
                == connection.source.place.id
            && connection
                .destination
                .place
                .position
                .position_revision
                .entity_id()
                == connection.destination.place.id
    }));
    assert_eq!(
        world
            .list_connection(
                user_id,
                ListConnection {
                    place_id: b.entity_id,
                    cursor: first.next.clone(),
                    limit: 2,
                },
            )
            .await,
        Err(WorldError::InvalidRequest)
    );
    assert_eq!(
        world
            .list_connection(
                user_id,
                ListConnection {
                    place_id: entry.entity_id,
                    cursor: Some("A".repeat(513)),
                    limit: 1
                }
            )
            .await,
        Err(WorldError::InvalidRequest)
    );
    let second = world
        .list_connection(
            user_id,
            ListConnection {
                place_id: entry.entity_id,
                cursor: first.next,
                limit: 2,
            },
        )
        .await
        .unwrap();
    assert_eq!(second.connection.len(), 1);
    assert_eq!(second.connection[0].id.0, Uuid::from_u128(30));
    assert_eq!(second.next, None);
    let after: i64 = sqlx::query_scalar("SELECT count(*) FROM activity")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(after, before);
}

#[sqlx::test(migrations = "./migration")]
async fn connection_get_is_neutral_and_hydrates_only_selected_course(pool: PgPool) {
    let (world, user_id, actor_id, entry) = entered_reader(&pool).await;
    let b = insert_place(
        &pool,
        user_id,
        actor_id,
        entry.entity_id,
        EntityId(Uuid::from_u128(200)),
        "Bridge B",
        (100, 0, 0),
    )
    .await;
    let c = insert_place(
        &pool,
        user_id,
        actor_id,
        entry.entity_id,
        EntityId(Uuid::from_u128(201)),
        "Bridge C",
        (200, 0, 0),
    )
    .await;
    let selected = ConnectionId(Uuid::from_u128(300));
    let other = ConnectionId(Uuid::from_u128(301));
    insert_connection(
        &pool,
        user_id,
        actor_id,
        selected,
        entry,
        b,
        false,
        &[(0, 0, 0, 0), (1, 50, 0, 0), (2, 100, 0, 0)],
        40,
    )
    .await;
    insert_connection(
        &pool,
        user_id,
        actor_id,
        other,
        entry,
        c,
        true,
        &[(0, 0, 0, 0), (1, 200, 0, 0)],
        41,
    )
    .await;
    let connection = world
        .get_connection(
            user_id,
            GetConnection {
                place_id: entry.entity_id,
                connection_id: selected,
            },
        )
        .await
        .unwrap();
    assert_eq!(connection.id, selected);
    assert_eq!(connection.shape_description.as_deref(), Some("Shape 40."));
    assert_eq!(
        connection
            .course
            .iter()
            .map(|point| (point.ordinal, point.x_cm))
            .collect::<Vec<_>>(),
        vec![(0, 0), (1, 50), (2, 100)]
    );
    for request in [
        GetConnection {
            place_id: b.entity_id,
            connection_id: other,
        },
        GetConnection {
            place_id: EntityId(Uuid::new_v4()),
            connection_id: selected,
        },
        GetConnection {
            place_id: entry.entity_id,
            connection_id: ConnectionId(Uuid::new_v4()),
        },
    ] {
        assert_eq!(
            world.get_connection(user_id, request).await,
            Err(WorldError::ConnectionNotFound)
        );
    }
}

#[sqlx::test(migrations = "./migration")]
async fn connection_list_does_not_expand_courses_on_hundred_summary_page(pool: PgPool) {
    let (world, user_id, actor_id, entry) = entered_reader(&pool).await;
    let destination = insert_place(
        &pool,
        user_id,
        actor_id,
        entry.entity_id,
        EntityId(Uuid::from_u128(9_000)),
        "Far Course End",
        (127, 0, 0),
    )
    .await;
    let point: Vec<_> = (0_i16..128)
        .map(|ordinal| (ordinal, i64::from(ordinal), 0, 0))
        .collect();
    for suffix in 1_u128..=100 {
        let selected_point = if suffix <= 3 { point.as_slice() } else { &[] };
        insert_connection(
            &pool,
            user_id,
            actor_id,
            ConnectionId(Uuid::from_u128(10_000 + suffix)),
            entry,
            destination,
            true,
            selected_point,
            u8::try_from(suffix).unwrap(),
        )
        .await;
    }
    let stored_point_count: i64 = sqlx::query_scalar("SELECT count(*) FROM connection_point")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(stored_point_count, 384);
    let page = world
        .list_connection(
            user_id,
            ListConnection {
                place_id: entry.entity_id,
                cursor: None,
                limit: 100,
            },
        )
        .await
        .unwrap();
    assert_eq!(page.connection.len(), 100);
    assert_eq!(
        page.connection
            .iter()
            .filter(|connection| connection.has_course)
            .count(),
        3
    );
    assert_eq!(page.next, None);
}

#[sqlx::test(migrations = "./migration")]
async fn connection_read_validates_anchor_cursor_limit_and_reader(pool: PgPool) {
    let (world, user_id, _, entry) = entered_reader(&pool).await;
    assert_eq!(
        world
            .list_connection(
                user_id,
                ListConnection {
                    place_id: entry.entity_id,
                    cursor: None,
                    limit: 101
                }
            )
            .await,
        Err(WorldError::InvalidConnectionLimit)
    );
    assert_eq!(
        world
            .list_connection(
                user_id,
                ListConnection {
                    place_id: entry.entity_id,
                    cursor: Some("not-a-cursor".to_owned()),
                    limit: 1
                }
            )
            .await,
        Err(WorldError::InvalidRequest)
    );
    assert_eq!(
        world
            .list_connection(
                user_id,
                ListConnection {
                    place_id: EntityId(Uuid::new_v4()),
                    cursor: None,
                    limit: 1
                }
            )
            .await,
        Err(WorldError::PlaceNotFound)
    );
    let unentered = create_user(&world).await;
    world
        .create_character(unentered, character("Unentered Reader"))
        .await
        .unwrap();
    assert_eq!(
        world
            .get_connection(
                unentered,
                GetConnection {
                    place_id: entry.entity_id,
                    connection_id: ConnectionId(Uuid::new_v4())
                }
            )
            .await,
        Err(WorldError::CharacterNotEntered)
    );
    sqlx::query("UPDATE character SET current_place_entity_id = NULL WHERE owner_user_id = $1")
        .bind(user_id.0)
        .execute(&pool)
        .await
        .unwrap();
    assert_eq!(
        world
            .list_place(user_id, place_window(0, None, 1))
            .await
            .unwrap()
            .place
            .len(),
        1
    );
    assert!(
        world
            .list_connection(
                user_id,
                ListConnection {
                    place_id: entry.entity_id,
                    cursor: None,
                    limit: 1,
                },
            )
            .await
            .is_ok()
    );
}
