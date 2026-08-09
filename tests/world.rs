use aicadia::{
    CreateCharacter, CreateEntity, EntityField, EntityId, InvalidReason, ListEntity, UserId, World,
    WorldError,
};
use chrono::{TimeZone, Utc};
use sqlx::PgPool;
use uuid::Uuid;

async fn create_user(world: &World) -> UserId {
    world
        .create_user()
        .await
        .expect("user should be created")
        .id
}

fn entity(name: &str) -> CreateEntity {
    CreateEntity {
        name: name.to_owned(),
        description: format!("Description of {name}"),
    }
}

fn character(name: &str) -> CreateCharacter {
    CreateCharacter {
        name: name.to_owned(),
        description: format!("Description of {name}"),
    }
}

#[sqlx::test(migrations = "./migration")]
async fn world_has_one_infallible_view(pool: PgPool) {
    let world = World::new(pool.clone());

    assert_eq!(world.get_world().name, "Aicadia");

    let user_count: i64 = sqlx::query_scalar("SELECT count(*) FROM \"user\"")
        .fetch_one(&pool)
        .await
        .expect("user count should be readable");
    assert_eq!(user_count, 0);
}

#[sqlx::test(migrations = "./migration")]
async fn user_is_explicit_and_persists_across_world_restart(pool: PgPool) {
    let world = World::new(pool.clone());
    let created = world.create_user().await.expect("user should be created");

    assert_eq!(world.get_user(created.id).await, Ok(created.clone()));

    drop(world);
    let restarted_world = World::new(pool);
    assert_eq!(restarted_world.get_user(created.id).await, Ok(created));
}

#[sqlx::test(migrations = "./migration")]
async fn get_user_rejects_an_unknown_id(pool: PgPool) {
    let world = World::new(pool);

    let error = world
        .get_user(UserId(Uuid::new_v4()))
        .await
        .expect_err("unknown user should not exist");

    assert_eq!(error, WorldError::UserNotFound);
}

#[sqlx::test(migrations = "./migration")]
async fn entities_are_shared_and_persist_across_world_restart(pool: PgPool) {
    let world = World::new(pool.clone());
    let first_user_id = create_user(&world).await;
    let second_user_id = create_user(&world).await;

    let first = world
        .create_entity(first_user_id, entity("Old Willow"))
        .await
        .expect("first user should introduce an entity");
    let second = world
        .create_entity(second_user_id, entity("Glassmere Lake"))
        .await
        .expect("second user should introduce an entity");

    assert_eq!(first.introduced_by_user_id, first_user_id);
    assert_eq!(second.introduced_by_user_id, second_user_id);

    drop(world);
    let restarted_world = World::new(pool);
    assert_eq!(
        restarted_world.get_entity(first.id).await,
        Ok(first.clone())
    );
    assert_eq!(
        restarted_world.get_entity(second.id).await,
        Ok(second.clone())
    );

    let page = restarted_world
        .list_entity(ListEntity {
            cursor: None,
            limit: 10,
        })
        .await
        .expect("shared entity list should be readable without a user");
    let listed_id = page
        .entity
        .iter()
        .map(|summary| summary.id)
        .collect::<Vec<_>>();
    assert!(listed_id.contains(&first.id));
    assert!(listed_id.contains(&second.id));
}

#[sqlx::test(migrations = "./migration")]
async fn unknown_introducer_does_not_insert_an_entity(pool: PgPool) {
    let world = World::new(pool.clone());

    let error = world
        .create_entity(UserId(Uuid::new_v4()), entity("Old Willow"))
        .await
        .expect_err("unknown user should not introduce an entity");
    assert_eq!(error, WorldError::UserNotFound);

    let entity_count: i64 = sqlx::query_scalar("SELECT count(*) FROM entity")
        .fetch_one(&pool)
        .await
        .expect("entity count should be readable");
    assert_eq!(entity_count, 0);
}

#[sqlx::test(migrations = "./migration")]
async fn equal_entity_input_creates_distinct_entities(pool: PgPool) {
    let world = World::new(pool);
    let user_id = create_user(&world).await;
    let input = entity("Old Willow");

    let first = world
        .create_entity(user_id, input.clone())
        .await
        .expect("first entity should be created");
    let second = world
        .create_entity(user_id, input)
        .await
        .expect("second entity should be created");

    assert_ne!(first.id, second.id);
    assert_eq!(first.name, second.name);
    assert_eq!(first.description, second.description);
}

#[sqlx::test(migrations = "./migration")]
async fn entity_input_is_trimmed_and_enforces_character_bounds(pool: PgPool) {
    let world = World::new(pool.clone());
    let user_id = create_user(&world).await;

    let trimmed = world
        .create_entity(
            user_id,
            CreateEntity {
                name: "  Old Willow  ".to_owned(),
                description: "  A mature willow beside Glassmere Lake.  ".to_owned(),
            },
        )
        .await
        .expect("trimmed input should be valid");
    assert_eq!(trimmed.name, "Old Willow");
    assert_eq!(
        trimmed.description,
        "A mature willow beside Glassmere Lake."
    );

    world
        .create_entity(
            user_id,
            CreateEntity {
                name: "n".repeat(120),
                description: "d".repeat(4_000),
            },
        )
        .await
        .expect("maximum lengths should be valid");

    let invalid_input = [
        (
            CreateEntity {
                name: "   ".to_owned(),
                description: "Valid".to_owned(),
            },
            EntityField::Name,
            InvalidReason::Empty,
        ),
        (
            CreateEntity {
                name: "n".repeat(121),
                description: "Valid".to_owned(),
            },
            EntityField::Name,
            InvalidReason::TooLong,
        ),
        (
            CreateEntity {
                name: "Valid".to_owned(),
                description: "   ".to_owned(),
            },
            EntityField::Description,
            InvalidReason::Empty,
        ),
        (
            CreateEntity {
                name: "Valid".to_owned(),
                description: "d".repeat(4_001),
            },
            EntityField::Description,
            InvalidReason::TooLong,
        ),
    ];

    for (input, field, reason) in invalid_input {
        let error = world
            .create_entity(user_id, input)
            .await
            .expect_err("out-of-bounds input should be rejected");
        assert_eq!(error, WorldError::InvalidEntity { field, reason });
    }

    let entity_count: i64 = sqlx::query_scalar("SELECT count(*) FROM entity")
        .fetch_one(&pool)
        .await
        .expect("entity count should be readable");
    assert_eq!(entity_count, 2);
}

#[sqlx::test(migrations = "./migration")]
async fn entity_input_rejects_nul_without_inserting(pool: PgPool) {
    let world = World::new(pool.clone());
    let user_id = create_user(&world).await;

    for (input, field) in [
        (
            CreateEntity {
                name: "Reed\0cap".to_owned(),
                description: "Valid".to_owned(),
            },
            EntityField::Name,
        ),
        (
            CreateEntity {
                name: "Valid".to_owned(),
                description: "A pale\0fungus".to_owned(),
            },
            EntityField::Description,
        ),
    ] {
        let error = world
            .create_entity(user_id, input)
            .await
            .expect_err("NUL should be rejected before storage");
        assert_eq!(
            error,
            WorldError::InvalidEntity {
                field,
                reason: InvalidReason::ContainsNul,
            }
        );
    }

    let entity_count: i64 = sqlx::query_scalar("SELECT count(*) FROM entity")
        .fetch_one(&pool)
        .await
        .expect("entity count should be readable");
    assert_eq!(entity_count, 0);
}

#[sqlx::test(migrations = "./migration")]
async fn list_entity_uses_uuid_tiebreak_across_cursor_page(pool: PgPool) {
    let world = World::new(pool.clone());
    let user_id = create_user(&world).await;
    let mut expected_id = Vec::new();

    for number in 0..4 {
        let created = world
            .create_entity(user_id, entity(&format!("Boundary Marker {number}")))
            .await
            .expect("entity should be created");
        expected_id.push(created.id);
    }

    let shared_introduced_at = Utc
        .with_ymd_and_hms(2026, 1, 1, 0, 0, 0)
        .single()
        .expect("test timestamp should be valid");
    for entity_id in &expected_id {
        sqlx::query("UPDATE entity SET introduced_at = $1 WHERE id = $2")
            .bind(shared_introduced_at)
            .bind(entity_id.0)
            .execute(&pool)
            .await
            .expect("test timestamp should be set");
    }
    expected_id.sort_by(|left, right| right.0.cmp(&left.0));

    let first = world
        .list_entity(ListEntity {
            cursor: None,
            limit: 2,
        })
        .await
        .expect("first page should be returned");
    assert_eq!(
        first
            .entity
            .iter()
            .map(|summary| summary.id)
            .collect::<Vec<_>>(),
        expected_id[..2]
    );
    assert!(first.next.is_some());

    let second = world
        .list_entity(ListEntity {
            cursor: first.next,
            limit: 2,
        })
        .await
        .expect("second page should be returned");
    assert_eq!(
        second
            .entity
            .iter()
            .map(|summary| summary.id)
            .collect::<Vec<_>>(),
        expected_id[2..]
    );
    assert_eq!(second.next, None);
}

#[sqlx::test(migrations = "./migration")]
async fn list_entity_defaults_to_25_accepts_100_and_rejects_out_of_range_limits(pool: PgPool) {
    let world = World::new(pool);
    let user_id = create_user(&world).await;

    let request = ListEntity::default();
    assert_eq!(request.cursor, None);
    assert_eq!(request.limit, 25);

    for number in 0..101 {
        world
            .create_entity(user_id, entity(&format!("Boundary Marker {number}")))
            .await
            .expect("entity should be created");
    }

    let page = world
        .list_entity(request)
        .await
        .expect("default page should be returned");
    assert_eq!(page.entity.len(), 25);
    assert!(page.next.is_some());

    let maximum_page = world
        .list_entity(ListEntity {
            cursor: None,
            limit: 100,
        })
        .await
        .expect("maximum page size should be accepted");
    assert_eq!(maximum_page.entity.len(), 100);
    assert!(maximum_page.next.is_some());

    for limit in [0, 101] {
        let error = world
            .list_entity(ListEntity {
                cursor: None,
                limit,
            })
            .await
            .expect_err("out-of-range limit should be rejected");
        assert_eq!(error, WorldError::InvalidEntityLimit);
    }
}

#[sqlx::test(migrations = "./migration")]
async fn get_entity_rejects_an_unknown_id(pool: PgPool) {
    let world = World::new(pool);

    let error = world
        .get_entity(EntityId(Uuid::new_v4()))
        .await
        .expect_err("unknown entity should not exist");

    assert_eq!(error, WorldError::EntityNotFound);
}

#[sqlx::test(migrations = "./migration")]
async fn character_composes_its_owned_persistent_shared_entity(pool: PgPool) {
    let world = World::new(pool.clone());
    let user_id = create_user(&world).await;
    let created = world
        .create_character(user_id, character("Mara Venn"))
        .await
        .expect("character should be created");

    assert_eq!(created.owner_user_id, user_id);
    assert_eq!(world.get_character(user_id).await, Ok(created.clone()));
    let entity = world
        .get_entity(created.entity.id)
        .await
        .expect("character entity should be shared");
    assert_eq!(entity, created.entity);
    assert_eq!(entity.introduced_by_user_id, user_id);
    assert!(
        world
            .list_entity(ListEntity::default())
            .await
            .expect("entity catalog should be readable")
            .entity
            .iter()
            .any(|summary| summary.id == created.entity.id)
    );

    drop(world);
    assert_eq!(World::new(pool).get_character(user_id).await, Ok(created));
}

#[sqlx::test(migrations = "./migration")]
async fn character_operations_distinguish_unknown_user_missing_and_existing_character(
    pool: PgPool,
) {
    let world = World::new(pool.clone());
    let unknown_user_id = UserId(Uuid::new_v4());
    assert_eq!(
        world.get_character(unknown_user_id).await,
        Err(WorldError::UserNotFound)
    );
    assert_eq!(
        world
            .create_character(unknown_user_id, character("Nobody"))
            .await,
        Err(WorldError::UserNotFound)
    );
    let entity_count: i64 = sqlx::query_scalar("SELECT count(*) FROM entity")
        .fetch_one(&pool)
        .await
        .expect("entity count should be readable");
    assert_eq!(entity_count, 0);

    let user_id = create_user(&world).await;
    assert_eq!(
        world.get_character(user_id).await,
        Err(WorldError::CharacterNotFound)
    );
    world
        .create_character(user_id, character("Mara Venn"))
        .await
        .expect("first character should be created");
    assert_eq!(
        world
            .create_character(user_id, character("Second Character"))
            .await,
        Err(WorldError::CharacterAlreadyExists)
    );
}

#[sqlx::test(migrations = "./migration")]
async fn character_reuses_entity_validation_without_inserting(pool: PgPool) {
    let world = World::new(pool.clone());
    let user_id = create_user(&world).await;

    for (input, field, reason) in [
        (
            CreateCharacter {
                name: "  ".to_owned(),
                description: "Valid".to_owned(),
            },
            EntityField::Name,
            InvalidReason::Empty,
        ),
        (
            CreateCharacter {
                name: "Valid".to_owned(),
                description: "d".repeat(4_001),
            },
            EntityField::Description,
            InvalidReason::TooLong,
        ),
        (
            CreateCharacter {
                name: "Invalid\0name".to_owned(),
                description: "Valid".to_owned(),
            },
            EntityField::Name,
            InvalidReason::ContainsNul,
        ),
    ] {
        assert_eq!(
            world.create_character(user_id, input).await,
            Err(WorldError::InvalidCharacter { field, reason })
        );
    }

    let entity_count: i64 = sqlx::query_scalar("SELECT count(*) FROM entity")
        .fetch_one(&pool)
        .await
        .expect("entity count should be readable");
    assert_eq!(entity_count, 0);
}

#[sqlx::test(migrations = "./migration")]
async fn concurrent_character_creation_has_one_winner_and_no_orphan_entity(pool: PgPool) {
    let world = World::new(pool.clone());
    let user_id = create_user(&world).await;
    let barrier = std::sync::Arc::new(tokio::sync::Barrier::new(3));
    let mut task = Vec::new();

    for name in ["Mara Venn", "Tomas Reed"] {
        let world = world.clone();
        let barrier = barrier.clone();
        task.push(tokio::spawn(async move {
            barrier.wait().await;
            world.create_character(user_id, character(name)).await
        }));
    }
    barrier.wait().await;
    let first = task.remove(0).await.expect("first task should finish");
    let second = task.remove(0).await.expect("second task should finish");

    assert_eq!(usize::from(first.is_ok()) + usize::from(second.is_ok()), 1);
    let loser = if first.is_err() { first } else { second };
    assert_eq!(loser, Err(WorldError::CharacterAlreadyExists));
    let character_count: i64 = sqlx::query_scalar("SELECT count(*) FROM character")
        .fetch_one(&pool)
        .await
        .expect("character count should be readable");
    let entity_count: i64 = sqlx::query_scalar("SELECT count(*) FROM entity")
        .fetch_one(&pool)
        .await
        .expect("entity count should be readable");
    assert_eq!(character_count, 1);
    assert_eq!(entity_count, 1);
}
