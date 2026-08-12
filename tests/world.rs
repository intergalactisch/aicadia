use aicadia::{
    ActionField, ActivityEntityRole, ActivityOperation, CreateCharacter, CreateEntity,
    CreateEntryPlace, EntityField, EntityId, IntroduceEntity, InvalidReason, ListActivity,
    ListActivityAtCurrentPlace, ListEntity, ListEntityAtCurrentPlace, PlaceRevision, SubmitAction,
    UserId, World, WorldError,
};
use chrono::{TimeZone, Utc};
use sha2::{Digest, Sha256};
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

fn place(name: &str) -> CreateEntryPlace {
    CreateEntryPlace {
        name: name.to_owned(),
        description: format!("Description of {name}"),
    }
}

fn action(request_id: Uuid, expected_place_revision: PlaceRevision, name: &str) -> SubmitAction {
    SubmitAction {
        request_id,
        expected_place_revision,
        prose: format!("Mara braces the {name} beside the crossing."),
        consequence: IntroduceEntity {
            name: name.to_owned(),
            description: format!("Description of {name}"),
        },
    }
}

async fn enter_at_entry(world: &World, user_id: UserId, name: &str) {
    world
        .create_character(user_id, character(name))
        .await
        .expect("character should be created");
    world
        .enter_world(user_id)
        .await
        .expect("character should enter the entry place");
}

async fn wait_for_database_lock_waiter(pool: &PgPool) {
    for _ in 0..1_000 {
        let waiting: bool = sqlx::query_scalar(
            r#"
            SELECT EXISTS (
                SELECT 1
                FROM pg_stat_activity
                WHERE datname = current_database()
                  AND pid <> pg_backend_pid()
                  AND wait_event_type = 'Lock'
            )
            "#,
        )
        .fetch_one(pool)
        .await
        .expect("lock waits should be observable");
        if waiting {
            return;
        }
        tokio::task::yield_now().await;
    }
    panic!("expected a database lock waiter");
}

#[derive(Debug, PartialEq, Eq, sqlx::FromRow)]
struct BackfilledActivity {
    operation: String,
    requested_by_user_id: Uuid,
    actor_character_entity_id: Option<Uuid>,
    context_place_entity_id: Option<Uuid>,
    occurred_at: chrono::DateTime<Utc>,
    entity_id: Uuid,
    role: String,
}

#[derive(Debug, PartialEq, Eq, sqlx::FromRow)]
struct HistoricActionColumns {
    prose: Option<String>,
    request_id: Option<Uuid>,
    request_fingerprint: Option<Vec<u8>>,
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
    expected_id.sort_by_key(|entity_id| std::cmp::Reverse(entity_id.0));

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

#[sqlx::test(migrations = "./migration")]
async fn character_starts_unplaced_and_its_creation_has_exact_history(pool: PgPool) {
    let world = World::new(pool);
    let user_id = create_user(&world).await;
    let character = world
        .create_character(user_id, character("Mara Venn"))
        .await
        .expect("character should be created");

    assert_eq!(character.current_place, None);
    assert_eq!(
        world.get_character(user_id).await.unwrap().current_place,
        None
    );
    let page = world
        .list_activity(user_id, ListActivity::default())
        .await
        .expect("personal activity should be readable");
    assert_eq!(page.activity.len(), 1);
    let activity = &page.activity[0];
    assert_eq!(activity.operation, ActivityOperation::CreateCharacter);
    assert_eq!(activity.actor_character, None);
    assert_eq!(activity.context_place, None);
    assert_eq!(activity.involved_entity.len(), 1);
    assert_eq!(activity.involved_entity[0].entity.id, character.entity.id);
    assert_eq!(
        activity.involved_entity[0].role,
        ActivityEntityRole::Subject
    );
}

#[sqlx::test(migrations = "./migration")]
async fn concurrent_entry_place_creation_has_one_winner_and_no_orphan(pool: PgPool) {
    let world = World::new(pool.clone());
    let first_user = create_user(&world).await;
    let second_user = create_user(&world).await;
    world
        .create_character(first_user, character("Mara Venn"))
        .await
        .unwrap();
    world
        .create_character(second_user, character("Tomas Reed"))
        .await
        .unwrap();
    let barrier = std::sync::Arc::new(tokio::sync::Barrier::new(3));
    let mut task = Vec::new();

    for (user_id, name) in [(first_user, "North Gate"), (second_user, "South Gate")] {
        let world = world.clone();
        let barrier = barrier.clone();
        task.push(tokio::spawn(async move {
            barrier.wait().await;
            world.create_entry_place(user_id, place(name)).await
        }));
    }
    barrier.wait().await;
    let first = task.remove(0).await.unwrap();
    let second = task.remove(0).await.unwrap();

    assert_eq!(usize::from(first.is_ok()) + usize::from(second.is_ok()), 1);
    let loser = if first.is_err() { first } else { second };
    assert_eq!(loser, Err(WorldError::EntryPlaceAlreadyExists));
    let place_count: i64 = sqlx::query_scalar("SELECT count(*) FROM place")
        .fetch_one(&pool)
        .await
        .unwrap();
    let entity_count: i64 = sqlx::query_scalar("SELECT count(*) FROM entity")
        .fetch_one(&pool)
        .await
        .unwrap();
    let genesis_count: i64 =
        sqlx::query_scalar("SELECT count(*) FROM activity WHERE operation = 'create_entry_place'")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(place_count, 1);
    assert_eq!(entity_count, 3);
    assert_eq!(genesis_count, 1);
}

#[sqlx::test(migrations = "./migration")]
async fn entry_place_requires_an_existing_unplaced_character_and_valid_content(pool: PgPool) {
    let world = World::new(pool.clone());
    let user_id = create_user(&world).await;
    assert_eq!(
        world.create_entry_place(user_id, place("North Gate")).await,
        Err(WorldError::CharacterNotFound)
    );
    world
        .create_character(user_id, character("Mara Venn"))
        .await
        .unwrap();
    assert_eq!(
        world
            .create_entry_place(
                user_id,
                CreateEntryPlace {
                    name: "   ".to_owned(),
                    description: "Valid".to_owned()
                },
            )
            .await,
        Err(WorldError::InvalidPlace {
            field: EntityField::Name,
            reason: InvalidReason::Empty
        })
    );
    let count: i64 = sqlx::query_scalar("SELECT count(*) FROM place")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(count, 0);
}

#[sqlx::test(migrations = "./migration")]
async fn enter_world_derives_entry_place_and_retry_does_not_duplicate_history(pool: PgPool) {
    let world = World::new(pool.clone());
    let user_id = create_user(&world).await;
    let character = world
        .create_character(user_id, character("Mara Venn"))
        .await
        .unwrap();
    assert_eq!(
        world.enter_world(user_id).await,
        Err(WorldError::EntryPlaceNotFound)
    );
    let entry = world
        .create_entry_place(user_id, place("North Gate"))
        .await
        .unwrap();

    let entered = world
        .enter_world(user_id)
        .await
        .expect("entry should succeed");
    assert_eq!(entered.entity.id, character.entity.id);
    assert_eq!(entered.current_place, Some(entry.clone()));
    assert_eq!(world.enter_world(user_id).await, Ok(entered.clone()));
    assert_eq!(world.get_character(user_id).await, Ok(entered));
    let count: i64 =
        sqlx::query_scalar("SELECT count(*) FROM activity WHERE operation = 'enter_world'")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(count, 1);
}

#[sqlx::test(migrations = "./migration")]
async fn concurrent_world_entry_writes_one_placement_activity(pool: PgPool) {
    let world = World::new(pool.clone());
    let user_id = create_user(&world).await;
    world
        .create_character(user_id, character("Mara Venn"))
        .await
        .unwrap();
    world
        .create_entry_place(user_id, place("North Gate"))
        .await
        .unwrap();
    let (first, second) = tokio::join!(world.enter_world(user_id), world.enter_world(user_id));
    assert!(first.is_ok());
    assert_eq!(first, second);
    let count: i64 =
        sqlx::query_scalar("SELECT count(*) FROM activity WHERE operation = 'enter_world'")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(count, 1);
}

#[sqlx::test(migrations = "./migration")]
async fn personal_activity_is_authorized_scoped_and_captures_place_context(pool: PgPool) {
    let world = World::new(pool.clone());
    let first_user = create_user(&world).await;
    let second_user = create_user(&world).await;
    let empty_user = create_user(&world).await;
    assert_eq!(
        world
            .list_activity(UserId(Uuid::new_v4()), ListActivity::default())
            .await,
        Err(WorldError::UserNotFound)
    );
    assert_eq!(
        world
            .list_activity(empty_user, ListActivity::default())
            .await,
        Err(WorldError::CharacterNotFound)
    );
    let first_character = world
        .create_character(first_user, character("Mara Venn"))
        .await
        .unwrap();
    world
        .create_character(second_user, character("Tomas Reed"))
        .await
        .unwrap();
    let entry = world
        .create_entry_place(first_user, place("North Gate"))
        .await
        .unwrap();
    world.enter_world(first_user).await.unwrap();
    world.enter_world(second_user).await.unwrap();
    let first_subject = world
        .create_entity(first_user, entity("Old Willow"))
        .await
        .unwrap();
    let second_subject = world
        .create_entity(second_user, entity("Glassmere Lake"))
        .await
        .unwrap();
    sqlx::query("UPDATE character SET current_place_entity_id = NULL WHERE owner_user_id = $1")
        .bind(first_user.0)
        .execute(&pool)
        .await
        .unwrap();

    let activity = world
        .list_activity(first_user, ListActivity::default())
        .await
        .unwrap()
        .activity;
    assert_eq!(activity.len(), 4);
    assert!(
        activity
            .windows(2)
            .all(|pair| (pair[0].occurred_at, pair[0].id.0) > (pair[1].occurred_at, pair[1].id.0))
    );
    assert!(
        activity
            .iter()
            .any(|item| item.operation == ActivityOperation::CreateCharacter)
    );
    assert!(
        activity
            .iter()
            .any(|item| item.operation == ActivityOperation::CreateEntryPlace)
    );
    assert!(
        activity
            .iter()
            .any(|item| item.operation == ActivityOperation::EnterWorld)
    );
    let created = activity
        .iter()
        .find(|item| {
            item.involved_entity
                .iter()
                .any(|related| related.entity.id == first_subject.id)
        })
        .expect("own Entity creation should be included");
    assert_eq!(
        created.actor_character.as_ref().unwrap().id,
        first_character.entity.id
    );
    assert_eq!(
        created.context_place.as_ref().unwrap().entity.id,
        entry.entity.id
    );
    assert!(!activity.iter().any(|item| {
        item.involved_entity
            .iter()
            .any(|related| related.entity.id == second_subject.id)
    }));
}

#[sqlx::test(migrations = "./migration")]
async fn activity_cursor_uses_uuid_tiebreak_without_duplicates(pool: PgPool) {
    let world = World::new(pool.clone());
    let user_id = create_user(&world).await;
    let character = world
        .create_character(user_id, character("Mara Venn"))
        .await
        .unwrap();
    let occurred_at = Utc.with_ymd_and_hms(2099, 1, 1, 0, 0, 0).single().unwrap();
    let mut expected = Vec::new();
    for _ in 0..4 {
        let id = Uuid::new_v4();
        expected.push(id);
        sqlx::query(
            "INSERT INTO activity (id, operation, requested_by_user_id, actor_character_entity_id, occurred_at) VALUES ($1, 'create_entity', $2, $3, $4)",
        )
        .bind(id).bind(user_id.0).bind(character.entity.id.0).bind(occurred_at)
        .execute(&pool).await.unwrap();
    }
    expected.sort_by(|left, right| right.cmp(left));

    let first = world
        .list_activity(
            user_id,
            ListActivity {
                cursor: None,
                limit: 2,
            },
        )
        .await
        .unwrap();
    assert_eq!(
        first
            .activity
            .iter()
            .map(|item| item.id.0)
            .collect::<Vec<_>>(),
        expected[..2]
    );
    let second = world
        .list_activity(
            user_id,
            ListActivity {
                cursor: first.next,
                limit: 2,
            },
        )
        .await
        .unwrap();
    assert_eq!(
        second
            .activity
            .iter()
            .map(|item| item.id.0)
            .collect::<Vec<_>>(),
        expected[2..]
    );
}

#[sqlx::test(migrations = "./migration")]
async fn current_state_and_activity_roll_back_together_when_history_insert_fails(pool: PgPool) {
    let world = World::new(pool.clone());
    let user_id = create_user(&world).await;
    let character = world
        .create_character(user_id, character("Mara Venn"))
        .await
        .unwrap();
    world
        .create_entry_place(user_id, place("North Gate"))
        .await
        .unwrap();
    sqlx::raw_sql(
        r#"
        CREATE FUNCTION reject_test_activity() RETURNS trigger LANGUAGE plpgsql AS $$
        BEGIN
            IF NEW.operation IN ('create_entity', 'enter_world') THEN
                RAISE EXCEPTION 'forced activity failure';
            END IF;
            RETURN NEW;
        END;
        $$;
        CREATE TRIGGER reject_test_activity BEFORE INSERT ON activity
            FOR EACH ROW EXECUTE FUNCTION reject_test_activity();
        "#,
    )
    .execute(&pool)
    .await
    .unwrap();

    assert_eq!(
        world.create_entity(user_id, entity("Rolled Back")).await,
        Err(WorldError::Unavailable)
    );
    assert_eq!(
        world.enter_world(user_id).await,
        Err(WorldError::Unavailable)
    );
    let rolled_back_entity: i64 =
        sqlx::query_scalar("SELECT count(*) FROM entity WHERE name = 'Rolled Back'")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(rolled_back_entity, 0);
    assert_eq!(
        world.get_character(user_id).await.unwrap().current_place,
        None
    );
    let enter_count: i64 =
        sqlx::query_scalar("SELECT count(*) FROM activity WHERE operation = 'enter_world'")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(enter_count, 0);
    assert_eq!(
        world.get_character(user_id).await.unwrap().entity.id,
        character.entity.id
    );
}

#[sqlx::test(migrations = "./migration")]
async fn accepted_activity_and_relations_are_immutable(pool: PgPool) {
    let world = World::new(pool.clone());
    let user_id = create_user(&world).await;
    world
        .create_character(user_id, character("Mara Venn"))
        .await
        .unwrap();
    assert!(
        sqlx::query("UPDATE activity SET occurred_at = now()")
            .execute(&pool)
            .await
            .is_err()
    );
    assert!(
        sqlx::query("DELETE FROM activity_entity")
            .execute(&pool)
            .await
            .is_err()
    );
}

#[sqlx::test(migrations = "./migration")]
async fn activity_migration_backfills_only_exact_pre_history_facts(pool: PgPool) {
    sqlx::raw_sql(
        r#"
        DROP TABLE activity_entity, activity, place, character, entity, "user" CASCADE;
        DROP FUNCTION reject_activity_change();
        "#,
    )
    .execute(&pool)
    .await
    .unwrap();
    for migration in [
        include_str!("../migration/0001_world.sql"),
        include_str!("../migration/0002_rename_app_user.sql"),
        include_str!("../migration/0003_character.sql"),
    ] {
        sqlx::raw_sql(migration).execute(&pool).await.unwrap();
    }
    let user_id = Uuid::new_v4();
    let character_id = Uuid::new_v4();
    let entity_id = Uuid::new_v4();
    let character_time = Utc.with_ymd_and_hms(2025, 1, 2, 3, 4, 5).single().unwrap();
    let entity_time = Utc.with_ymd_and_hms(2025, 2, 3, 4, 5, 6).single().unwrap();
    sqlx::query("INSERT INTO \"user\" (id) VALUES ($1)")
        .bind(user_id)
        .execute(&pool)
        .await
        .unwrap();
    for (id, name, introduced_at) in [
        (character_id, "Historic Character", character_time),
        (entity_id, "Historic Entity", entity_time),
    ] {
        sqlx::query(
            "INSERT INTO entity (id, name, description, introduced_by_user_id, introduced_at) VALUES ($1, $2, 'Historic description', $3, $4)",
        )
        .bind(id).bind(name).bind(user_id).bind(introduced_at)
        .execute(&pool).await.unwrap();
    }
    sqlx::query("INSERT INTO character (entity_id, owner_user_id) VALUES ($1, $2)")
        .bind(character_id)
        .bind(user_id)
        .execute(&pool)
        .await
        .unwrap();

    sqlx::raw_sql(include_str!("../migration/0004_world_entry_activity.sql"))
        .execute(&pool)
        .await
        .unwrap();
    let row: Vec<BackfilledActivity> = sqlx::query_as(
        r#"
            SELECT activity.operation, activity.requested_by_user_id,
                   activity.actor_character_entity_id, activity.context_place_entity_id,
                   activity.occurred_at, activity_entity.entity_id, activity_entity.role
            FROM activity
            JOIN activity_entity ON activity_entity.activity_id = activity.id
            ORDER BY activity.occurred_at
            "#,
    )
    .fetch_all(&pool)
    .await
    .unwrap();
    assert_eq!(
        row,
        vec![
            BackfilledActivity {
                operation: "create_character".to_owned(),
                requested_by_user_id: user_id,
                actor_character_entity_id: None,
                context_place_entity_id: None,
                occurred_at: character_time,
                entity_id: character_id,
                role: "subject".to_owned(),
            },
            BackfilledActivity {
                operation: "create_entity".to_owned(),
                requested_by_user_id: user_id,
                actor_character_entity_id: None,
                context_place_entity_id: None,
                occurred_at: entity_time,
                entity_id,
                role: "subject".to_owned(),
            },
        ]
    );
}

#[sqlx::test(migrations = "./migration")]
async fn action_atomically_places_one_entity_and_exposes_canonical_prose_to_two_users(
    pool: PgPool,
) {
    let world = World::new(pool.clone());
    let first_user = create_user(&world).await;
    let second_user = create_user(&world).await;
    world
        .create_character(first_user, character("Mara Venn"))
        .await
        .unwrap();
    let entry = world
        .create_entry_place(first_user, place("North Gate"))
        .await
        .unwrap();
    world.enter_world(first_user).await.unwrap();
    enter_at_entry(&world, second_user, "Tomas Reed").await;

    let before = world
        .list_entity_at_current_place(first_user, ListEntityAtCurrentPlace::default())
        .await
        .unwrap();
    assert!(before.entity.is_empty());
    let request_id = Uuid::new_v4();
    let accepted = world
        .submit_action(
            first_user,
            action(request_id, before.place_revision, "Cedar Crossing Marker"),
        )
        .await
        .expect("grounded action should be accepted");

    assert_eq!(accepted.place, entry);
    assert_eq!(accepted.entity.name, "Cedar Crossing Marker");
    assert_eq!(accepted.activity.operation, ActivityOperation::SubmitAction);
    assert_eq!(
        accepted.activity.prose.as_deref(),
        Some("Mara braces the Cedar Crossing Marker beside the crossing.")
    );
    assert_eq!(
        accepted.activity.actor_character.as_ref().unwrap().id,
        world.get_character(first_user).await.unwrap().entity.id
    );
    assert_eq!(
        accepted.activity.context_place.as_ref().unwrap().entity.id,
        entry.entity.id
    );
    assert!(accepted.activity.involved_entity.iter().any(|reference| {
        reference.entity.id == accepted.entity.id && reference.role == ActivityEntityRole::Subject
    }));
    assert!(accepted.activity.involved_entity.iter().any(|reference| {
        reference.entity.id == entry.entity.id && reference.role == ActivityEntityRole::Location
    }));

    let visible_entity = world
        .list_entity_at_current_place(second_user, ListEntityAtCurrentPlace::default())
        .await
        .unwrap();
    assert_eq!(visible_entity.entity.len(), 1);
    assert_eq!(visible_entity.entity[0].id, accepted.entity.id);
    let visible_activity = world
        .list_activity_at_current_place(second_user, ListActivityAtCurrentPlace::default())
        .await
        .unwrap();
    assert_eq!(
        visible_entity.place_revision,
        visible_activity.place_revision
    );
    assert_eq!(visible_activity.activity[0], accepted.activity);

    let stored: (i64, i64, i64, i64) = sqlx::query_as(
        r#"
        SELECT
            (SELECT count(*) FROM entity_location WHERE entity_id = $1),
            (SELECT count(*) FROM activity WHERE request_id = $2 AND prose IS NOT NULL),
            (SELECT count(*) FROM activity_entity WHERE activity_id = $3 AND role = 'subject'),
            (SELECT count(*) FROM activity_entity WHERE activity_id = $3 AND role = 'location')
        "#,
    )
    .bind(accepted.entity.id.0)
    .bind(request_id)
    .bind(accepted.activity.id.0)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(stored, (1, 1, 1, 1));
}

#[sqlx::test(migrations = "./migration")]
async fn action_normalizes_before_fingerprinting_and_equal_retry_returns_canonical_result(
    pool: PgPool,
) {
    let world = World::new(pool.clone());
    let user_id = create_user(&world).await;
    world
        .create_character(user_id, character("Mara Venn"))
        .await
        .unwrap();
    world
        .create_entry_place(user_id, place("North Gate"))
        .await
        .unwrap();
    world.enter_world(user_id).await.unwrap();
    let revision = world
        .list_entity_at_current_place(user_id, ListEntityAtCurrentPlace::default())
        .await
        .unwrap()
        .place_revision;
    let request_id = Uuid::new_v4();
    let first = world
        .submit_action(
            user_id,
            SubmitAction {
                request_id,
                expected_place_revision: revision,
                prose: "  Mara sets a marker.  ".to_owned(),
                consequence: IntroduceEntity {
                    name: "  Cedar Marker  ".to_owned(),
                    description: "  Three lines cross its face.  ".to_owned(),
                },
            },
        )
        .await
        .unwrap();
    let retry = world
        .submit_action(
            user_id,
            SubmitAction {
                request_id,
                expected_place_revision: revision,
                prose: "Mara sets a marker.".to_owned(),
                consequence: IntroduceEntity {
                    name: "Cedar Marker".to_owned(),
                    description: "Three lines cross its face.".to_owned(),
                },
            },
        )
        .await
        .unwrap();
    assert_eq!(retry, first);
    assert_eq!(first.activity.prose.as_deref(), Some("Mara sets a marker."));
    assert_eq!(first.entity.name, "Cedar Marker");

    let fingerprint: Vec<u8> =
        sqlx::query_scalar("SELECT request_fingerprint FROM activity WHERE request_id = $1")
            .bind(request_id)
            .fetch_one(&pool)
            .await
            .unwrap();
    let mut revision_bytes = Vec::with_capacity(41);
    revision_bytes.push(1);
    revision_bytes.extend_from_slice(revision.place_entity_id().0.as_bytes());
    revision_bytes.extend_from_slice(&revision.occurred_at().timestamp_micros().to_be_bytes());
    revision_bytes.extend_from_slice(revision.activity_id().0.as_bytes());
    let mut expected = Sha256::new();
    for field in [
        b"aicadia-submit-action-fingerprint-v1".as_slice(),
        revision_bytes.as_slice(),
        b"Mara sets a marker.".as_slice(),
        b"introduce_entity".as_slice(),
        b"Cedar Marker".as_slice(),
        b"Three lines cross its face.".as_slice(),
    ] {
        expected.update((field.len() as u64).to_be_bytes());
        expected.update(field);
    }
    assert_eq!(fingerprint, expected.finalize().as_slice());
    assert_eq!(fingerprint.len(), 32);
    let counts: (i64, i64, i64) = sqlx::query_as(
        r#"
        SELECT
            (SELECT count(*) FROM entity WHERE name = 'Cedar Marker'),
            (SELECT count(*) FROM entity_location),
            (SELECT count(*) FROM activity WHERE request_id = $1)
        "#,
    )
    .bind(request_id)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(counts, (1, 1, 1));
}

#[sqlx::test(migrations = "./migration")]
async fn reused_action_request_id_with_changed_content_conflicts_without_writes(pool: PgPool) {
    let world = World::new(pool.clone());
    let user_id = create_user(&world).await;
    world
        .create_character(user_id, character("Mara Venn"))
        .await
        .unwrap();
    world
        .create_entry_place(user_id, place("North Gate"))
        .await
        .unwrap();
    world.enter_world(user_id).await.unwrap();
    let revision = world
        .list_entity_at_current_place(user_id, ListEntityAtCurrentPlace::default())
        .await
        .unwrap()
        .place_revision;
    let request_id = Uuid::new_v4();
    world
        .submit_action(user_id, action(request_id, revision, "First Marker"))
        .await
        .unwrap();
    assert_eq!(
        world
            .submit_action(user_id, action(request_id, revision, "Changed Marker"))
            .await,
        Err(WorldError::ActionRequestConflict)
    );
    let changed_count: i64 =
        sqlx::query_scalar("SELECT count(*) FROM entity WHERE name = 'Changed Marker'")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(changed_count, 0);
}

#[sqlx::test(migrations = "./migration")]
async fn accepted_action_retry_resolves_before_later_place_preconditions(pool: PgPool) {
    let world = World::new(pool.clone());
    let user_id = create_user(&world).await;
    world
        .create_character(user_id, character("Mara Venn"))
        .await
        .unwrap();
    world
        .create_entry_place(user_id, place("North Gate"))
        .await
        .unwrap();
    world.enter_world(user_id).await.unwrap();
    let revision = world
        .list_entity_at_current_place(user_id, ListEntityAtCurrentPlace::default())
        .await
        .unwrap()
        .place_revision;
    let request_id = Uuid::new_v4();
    let request = action(request_id, revision, "Cedar Marker");
    let accepted = world.submit_action(user_id, request.clone()).await.unwrap();
    world
        .create_entity(user_id, entity("Later Unplaced Referent"))
        .await
        .unwrap();
    sqlx::query("UPDATE character SET current_place_entity_id = NULL WHERE owner_user_id = $1")
        .bind(user_id.0)
        .execute(&pool)
        .await
        .unwrap();

    assert_eq!(world.submit_action(user_id, request).await, Ok(accepted));
}

#[sqlx::test(migrations = "./migration")]
async fn stale_place_revision_rejects_action_after_each_existing_place_writer(pool: PgPool) {
    let world = World::new(pool.clone());
    let first_user = create_user(&world).await;
    let second_user = create_user(&world).await;
    world
        .create_character(first_user, character("Mara Venn"))
        .await
        .unwrap();
    world
        .create_entry_place(first_user, place("North Gate"))
        .await
        .unwrap();
    world.enter_world(first_user).await.unwrap();
    world
        .create_character(second_user, character("Tomas Reed"))
        .await
        .unwrap();

    let before_entry = world
        .list_activity_at_current_place(first_user, ListActivityAtCurrentPlace::default())
        .await
        .unwrap()
        .place_revision;
    world.enter_world(second_user).await.unwrap();
    let after_entry = world
        .list_activity_at_current_place(first_user, ListActivityAtCurrentPlace::default())
        .await
        .unwrap()
        .place_revision;
    assert_ne!(before_entry, after_entry);
    assert_eq!(
        world
            .submit_action(
                first_user,
                action(Uuid::new_v4(), before_entry, "Stale Entry Marker")
            )
            .await,
        Err(WorldError::PlaceRevisionConflict)
    );

    world
        .create_entity(second_user, entity("Unplaced Bell"))
        .await
        .unwrap();
    let after_entity = world
        .list_entity_at_current_place(first_user, ListEntityAtCurrentPlace::default())
        .await
        .unwrap()
        .place_revision;
    assert_ne!(after_entry, after_entity);
    assert_eq!(
        world
            .submit_action(
                first_user,
                action(Uuid::new_v4(), after_entry, "Stale Entity Marker")
            )
            .await,
        Err(WorldError::PlaceRevisionConflict)
    );
    let stale_count: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM entity WHERE name IN ('Stale Entry Marker', 'Stale Entity Marker')",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(stale_count, 0);
}

#[sqlx::test(migrations = "./migration")]
async fn concurrent_equal_action_delivery_writes_once_and_returns_one_canonical_result(
    pool: PgPool,
) {
    let world = World::new(pool.clone());
    let user_id = create_user(&world).await;
    world
        .create_character(user_id, character("Mara Venn"))
        .await
        .unwrap();
    world
        .create_entry_place(user_id, place("North Gate"))
        .await
        .unwrap();
    world.enter_world(user_id).await.unwrap();
    let revision = world
        .list_entity_at_current_place(user_id, ListEntityAtCurrentPlace::default())
        .await
        .unwrap()
        .place_revision;
    let request = action(Uuid::new_v4(), revision, "Concurrent Marker");
    let (first, second) = tokio::join!(
        world.submit_action(user_id, request.clone()),
        world.submit_action(user_id, request)
    );
    assert_eq!(first, second);
    assert!(first.is_ok());
    let counts: (i64, i64, i64) = sqlx::query_as(
        r#"
        SELECT
            (SELECT count(*) FROM entity WHERE name = 'Concurrent Marker'),
            (SELECT count(*) FROM entity_location),
            (SELECT count(*) FROM activity WHERE operation = 'submit_action')
        "#,
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(counts, (1, 1, 1));
}

#[sqlx::test(migrations = "./migration")]
async fn concurrent_distinct_actions_from_one_place_revision_have_one_winner(pool: PgPool) {
    let world = World::new(pool.clone());
    let first_user = create_user(&world).await;
    let second_user = create_user(&world).await;
    world
        .create_character(first_user, character("Mara Venn"))
        .await
        .unwrap();
    world
        .create_entry_place(first_user, place("North Gate"))
        .await
        .unwrap();
    world.enter_world(first_user).await.unwrap();
    enter_at_entry(&world, second_user, "Tomas Reed").await;
    let revision = world
        .list_entity_at_current_place(first_user, ListEntityAtCurrentPlace::default())
        .await
        .unwrap()
        .place_revision;
    let (first, second) = tokio::join!(
        world.submit_action(
            first_user,
            action(Uuid::new_v4(), revision, "First Concurrent Marker")
        ),
        world.submit_action(
            second_user,
            action(Uuid::new_v4(), revision, "Second Concurrent Marker")
        )
    );
    assert_eq!(usize::from(first.is_ok()) + usize::from(second.is_ok()), 1);
    let loser = if first.is_err() { first } else { second };
    assert_eq!(loser, Err(WorldError::PlaceRevisionConflict));
    let counts: (i64, i64, i64) = sqlx::query_as(
        r#"
        SELECT
            (SELECT count(*) FROM entity_location),
            (SELECT count(*) FROM activity WHERE operation = 'submit_action'),
            (SELECT count(*) FROM activity WHERE request_id IS NOT NULL)
        "#,
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(counts, (1, 1, 1));
}

#[sqlx::test(migrations = "./migration")]
async fn invalid_unplaced_stale_and_storage_failed_actions_leave_no_partial_rows(pool: PgPool) {
    let world = World::new(pool.clone());
    let user_id = create_user(&world).await;
    world
        .create_character(user_id, character("Mara Venn"))
        .await
        .unwrap();
    assert_eq!(
        world
            .submit_action(
                user_id,
                action(
                    Uuid::new_v4(),
                    PlaceRevision::from_parts(
                        EntityId(Uuid::new_v4()),
                        Utc::now(),
                        aicadia::ActivityId(Uuid::new_v4())
                    ),
                    "Unplaced Marker"
                )
            )
            .await,
        Err(WorldError::CharacterNotEntered)
    );
    let entry = world
        .create_entry_place(user_id, place("North Gate"))
        .await
        .unwrap();
    world.enter_world(user_id).await.unwrap();
    let revision = world
        .list_entity_at_current_place(user_id, ListEntityAtCurrentPlace::default())
        .await
        .unwrap()
        .place_revision;
    for (input, field, reason) in [
        (
            SubmitAction {
                request_id: Uuid::new_v4(),
                expected_place_revision: revision,
                prose: "  ".to_owned(),
                consequence: IntroduceEntity {
                    name: "Valid".to_owned(),
                    description: "Valid".to_owned(),
                },
            },
            ActionField::Prose,
            InvalidReason::Empty,
        ),
        (
            SubmitAction {
                request_id: Uuid::new_v4(),
                expected_place_revision: revision,
                prose: "Valid".to_owned(),
                consequence: IntroduceEntity {
                    name: "Bad\0name".to_owned(),
                    description: "Valid".to_owned(),
                },
            },
            ActionField::ConsequenceName,
            InvalidReason::ContainsNul,
        ),
    ] {
        assert_eq!(
            world.submit_action(user_id, input).await,
            Err(WorldError::InvalidAction { field, reason })
        );
    }
    sqlx::raw_sql(
        r#"
        CREATE FUNCTION reject_submit_action() RETURNS trigger LANGUAGE plpgsql AS $$
        BEGIN
            IF NEW.operation = 'submit_action' THEN
                RAISE EXCEPTION 'forced submit failure';
            END IF;
            RETURN NEW;
        END;
        $$;
        CREATE TRIGGER reject_submit_action BEFORE INSERT ON activity
            FOR EACH ROW EXECUTE FUNCTION reject_submit_action();
        "#,
    )
    .execute(&pool)
    .await
    .unwrap();
    assert_eq!(
        world
            .submit_action(
                user_id,
                action(Uuid::new_v4(), revision, "Rolled Back Marker")
            )
            .await,
        Err(WorldError::Unavailable)
    );
    let partial: (i64, i64, i64) = sqlx::query_as(
        r#"
        SELECT
            (SELECT count(*) FROM entity WHERE name IN ('Unplaced Marker', 'Rolled Back Marker')),
            (SELECT count(*) FROM entity_location),
            (SELECT count(*) FROM activity WHERE operation = 'submit_action')
        "#,
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(partial, (0, 0, 0));
    assert_eq!(revision.place_entity_id(), entry.entity.id);
}

#[sqlx::test(migrations = "./migration")]
async fn current_place_reads_reject_missing_or_unplaced_character_and_paginate(pool: PgPool) {
    let world = World::new(pool.clone());
    let missing_character_user = create_user(&world).await;
    assert_eq!(
        world
            .list_entity_at_current_place(
                missing_character_user,
                ListEntityAtCurrentPlace::default()
            )
            .await,
        Err(WorldError::CharacterNotFound)
    );
    world
        .create_character(missing_character_user, character("Mara Venn"))
        .await
        .unwrap();
    assert_eq!(
        world
            .list_activity_at_current_place(
                missing_character_user,
                ListActivityAtCurrentPlace::default()
            )
            .await,
        Err(WorldError::CharacterNotEntered)
    );
    world
        .create_entry_place(missing_character_user, place("North Gate"))
        .await
        .unwrap();
    world.enter_world(missing_character_user).await.unwrap();
    for number in 0..4 {
        let revision = world
            .list_entity_at_current_place(
                missing_character_user,
                ListEntityAtCurrentPlace::default(),
            )
            .await
            .unwrap()
            .place_revision;
        world
            .submit_action(
                missing_character_user,
                action(Uuid::new_v4(), revision, &format!("Marker {number}")),
            )
            .await
            .unwrap();
    }
    let first = world
        .list_entity_at_current_place(
            missing_character_user,
            ListEntityAtCurrentPlace {
                cursor: None,
                limit: 2,
            },
        )
        .await
        .unwrap();
    assert_eq!(first.entity.len(), 2);
    assert!(first.next.is_some());
    let second = world
        .list_entity_at_current_place(
            missing_character_user,
            ListEntityAtCurrentPlace {
                cursor: first.next,
                limit: 2,
            },
        )
        .await
        .unwrap();
    assert_eq!(second.entity.len(), 2);
    assert_eq!(second.next, None);
    assert_eq!(first.place_revision, second.place_revision);
    let mut ids = first
        .entity
        .into_iter()
        .chain(second.entity)
        .map(|entity| entity.id)
        .collect::<Vec<_>>();
    ids.sort_by_key(|id| id.0);
    ids.dedup();
    assert_eq!(ids.len(), 4);

    let activity_first = world
        .list_activity_at_current_place(
            missing_character_user,
            ListActivityAtCurrentPlace {
                cursor: None,
                limit: 3,
            },
        )
        .await
        .unwrap();
    let activity_second = world
        .list_activity_at_current_place(
            missing_character_user,
            ListActivityAtCurrentPlace {
                cursor: activity_first.next,
                limit: 100,
            },
        )
        .await
        .unwrap();
    assert_eq!(
        activity_first.place_revision,
        activity_second.place_revision
    );
    assert!(
        activity_first
            .activity
            .iter()
            .chain(&activity_second.activity)
            .any(|activity| activity.operation == ActivityOperation::CreateEntryPlace)
    );
}

#[sqlx::test(migrations = "./migration")]
async fn activity_at_an_unrelated_place_does_not_invalidate_revision(pool: PgPool) {
    let world = World::new(pool.clone());
    let first_user = create_user(&world).await;
    let second_user = create_user(&world).await;
    world
        .create_character(first_user, character("Mara Venn"))
        .await
        .unwrap();
    world
        .create_entry_place(first_user, place("North Gate"))
        .await
        .unwrap();
    world.enter_world(first_user).await.unwrap();
    let second_character = world
        .create_character(second_user, character("Tomas Reed"))
        .await
        .unwrap();
    let second_place_id = Uuid::new_v4();
    let second_place_activity_id = Uuid::new_v4();
    sqlx::query(
        "INSERT INTO entity (id, name, description, introduced_by_user_id) VALUES ($1, 'Test South Place', 'Internal isolation fixture', $2)",
    )
    .bind(second_place_id)
    .bind(second_user.0)
    .execute(&pool)
    .await
    .unwrap();
    sqlx::query(
        "INSERT INTO activity (id, operation, requested_by_user_id, actor_character_entity_id) VALUES ($1, 'create_entry_place', $2, $3)",
    )
    .bind(second_place_activity_id)
    .bind(second_user.0)
    .bind(second_character.entity.id.0)
    .execute(&pool)
    .await
    .unwrap();
    sqlx::query(
        "INSERT INTO place (entity_id, is_entry, latest_activity_id) VALUES ($1, false, $2)",
    )
    .bind(second_place_id)
    .bind(second_place_activity_id)
    .execute(&pool)
    .await
    .unwrap();
    sqlx::query(
        "INSERT INTO activity_entity (activity_id, entity_id, role) VALUES ($1, $2, 'subject')",
    )
    .bind(second_place_activity_id)
    .bind(second_place_id)
    .execute(&pool)
    .await
    .unwrap();
    sqlx::query("UPDATE character SET current_place_entity_id = $1 WHERE owner_user_id = $2")
        .bind(second_place_id)
        .bind(second_user.0)
        .execute(&pool)
        .await
        .unwrap();

    let first_revision = world
        .list_entity_at_current_place(first_user, ListEntityAtCurrentPlace::default())
        .await
        .unwrap()
        .place_revision;
    world
        .create_entity(second_user, entity("Southern Bell"))
        .await
        .unwrap();
    let unchanged = world
        .list_activity_at_current_place(first_user, ListActivityAtCurrentPlace::default())
        .await
        .unwrap()
        .place_revision;
    assert_eq!(unchanged, first_revision);
    world
        .submit_action(
            first_user,
            action(Uuid::new_v4(), first_revision, "Northern Marker"),
        )
        .await
        .expect("unrelated Place activity must not stale the action");
}

#[sqlx::test(migrations = "./migration")]
async fn action_columns_and_relations_are_immutable_and_historic_rows_remain_null(pool: PgPool) {
    let world = World::new(pool.clone());
    let user_id = create_user(&world).await;
    world
        .create_character(user_id, character("Mara Venn"))
        .await
        .unwrap();
    let historic_nulls: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM activity WHERE prose IS NULL AND request_id IS NULL AND request_fingerprint IS NULL",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(historic_nulls, 1);
    assert!(
        sqlx::query("UPDATE activity SET prose = 'changed'")
            .execute(&pool)
            .await
            .is_err()
    );
    assert!(
        sqlx::query("UPDATE activity_entity SET role = 'location'")
            .execute(&pool)
            .await
            .is_err()
    );
}

#[sqlx::test(migrations = "./migration")]
async fn place_revision_pointer_advances_when_timestamp_and_uuid_order_move_backward(pool: PgPool) {
    let world = World::new(pool.clone());
    let user_id = create_user(&world).await;
    let character = world
        .create_character(user_id, character("Mara Venn"))
        .await
        .unwrap();
    let place = world
        .create_entry_place(user_id, place("North Gate"))
        .await
        .unwrap();
    world.enter_world(user_id).await.unwrap();

    let fixed_time = Utc.with_ymd_and_hms(2099, 1, 1, 0, 0, 0).single().unwrap();
    let maximum_id = Uuid::from_u128(u128::MAX);
    sqlx::query(
        r#"
        INSERT INTO activity (
            id, operation, requested_by_user_id,
            actor_character_entity_id, context_place_entity_id, occurred_at
        )
        VALUES ($1, 'create_entity', $2, $3, $4, $5)
        "#,
    )
    .bind(maximum_id)
    .bind(user_id.0)
    .bind(character.entity.id.0)
    .bind(place.entity.id.0)
    .bind(fixed_time)
    .execute(&pool)
    .await
    .unwrap();
    sqlx::query("UPDATE place SET latest_activity_id = $1 WHERE entity_id = $2")
        .bind(maximum_id)
        .bind(place.entity.id.0)
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query(
        "ALTER TABLE activity ALTER COLUMN occurred_at SET DEFAULT '2099-01-01 00:00:00+00'",
    )
    .execute(&pool)
    .await
    .unwrap();

    let before = world
        .list_activity_at_current_place(user_id, ListActivityAtCurrentPlace::default())
        .await
        .unwrap()
        .place_revision;
    assert_eq!(before.activity_id().0, maximum_id);
    world
        .create_entity(user_id, entity("Equal-time Referent"))
        .await
        .unwrap();
    let after_equal_time = world
        .list_activity_at_current_place(user_id, ListActivityAtCurrentPlace::default())
        .await
        .unwrap()
        .place_revision;
    assert_eq!(after_equal_time.occurred_at(), fixed_time);
    assert_ne!(after_equal_time.activity_id().0, maximum_id);
    assert!(after_equal_time.activity_id().0 < maximum_id);

    let historic_max: Uuid = sqlx::query_scalar(
        r#"
        SELECT id
        FROM activity
        WHERE context_place_entity_id = $1
        ORDER BY occurred_at DESC, id DESC
        LIMIT 1
        "#,
    )
    .bind(place.entity.id.0)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(historic_max, maximum_id, "the former MAX query stays stale");

    let earlier_time = Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).single().unwrap();
    sqlx::query(
        "ALTER TABLE activity ALTER COLUMN occurred_at SET DEFAULT '2020-01-01 00:00:00+00'",
    )
    .execute(&pool)
    .await
    .unwrap();
    world
        .create_entity(user_id, entity("Clock-rollback Referent"))
        .await
        .unwrap();
    let after_clock_rollback = world
        .list_entity_at_current_place(user_id, ListEntityAtCurrentPlace::default())
        .await
        .unwrap()
        .place_revision;
    assert_eq!(after_clock_rollback.occurred_at(), earlier_time);
    assert_ne!(after_clock_rollback, after_equal_time);
    assert_eq!(
        world
            .submit_action(
                user_id,
                action(Uuid::new_v4(), after_equal_time, "Stale Clock Marker")
            )
            .await,
        Err(WorldError::PlaceRevisionConflict)
    );
}

#[sqlx::test(migrations = "./migration")]
async fn action_migration_backfills_each_existing_place_pointer_without_fabricating_history(
    pool: PgPool,
) {
    sqlx::raw_sql(
        r#"
        DROP TABLE entity_location, activity_entity, activity, place, character, entity, "user" CASCADE;
        DROP FUNCTION reject_activity_change();
        "#,
    )
    .execute(&pool)
    .await
    .unwrap();
    for migration in [
        include_str!("../migration/0001_world.sql"),
        include_str!("../migration/0002_rename_app_user.sql"),
        include_str!("../migration/0003_character.sql"),
        include_str!("../migration/0004_world_entry_activity.sql"),
    ] {
        sqlx::raw_sql(migration).execute(&pool).await.unwrap();
    }
    let user_id = Uuid::new_v4();
    let character_id = Uuid::new_v4();
    let place_id = Uuid::new_v4();
    let first_activity_id = Uuid::from_u128(1);
    let latest_historic_id = Uuid::from_u128(u128::MAX);
    let occurred_at = Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).single().unwrap();
    sqlx::query("INSERT INTO \"user\" (id) VALUES ($1)")
        .bind(user_id)
        .execute(&pool)
        .await
        .unwrap();
    for (id, name) in [
        (character_id, "Historic Character"),
        (place_id, "Historic Place"),
    ] {
        sqlx::query(
            "INSERT INTO entity (id, name, description, introduced_by_user_id) VALUES ($1, $2, 'Historic description', $3)",
        )
        .bind(id)
        .bind(name)
        .bind(user_id)
        .execute(&pool)
        .await
        .unwrap();
    }
    sqlx::query("INSERT INTO character (entity_id, owner_user_id) VALUES ($1, $2)")
        .bind(character_id)
        .bind(user_id)
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query("INSERT INTO place (entity_id, is_entry) VALUES ($1, true)")
        .bind(place_id)
        .execute(&pool)
        .await
        .unwrap();
    for activity_id in [first_activity_id, latest_historic_id] {
        sqlx::query(
            r#"
            INSERT INTO activity (
                id, operation, requested_by_user_id,
                actor_character_entity_id, context_place_entity_id, occurred_at
            )
            VALUES ($1, 'enter_world', $2, $3, $4, $5)
            "#,
        )
        .bind(activity_id)
        .bind(user_id)
        .bind(character_id)
        .bind(place_id)
        .bind(occurred_at)
        .execute(&pool)
        .await
        .unwrap();
    }

    sqlx::raw_sql(include_str!("../migration/0005_agent_action.sql"))
        .execute(&pool)
        .await
        .unwrap();
    let pointer: Uuid =
        sqlx::query_scalar("SELECT latest_activity_id FROM place WHERE entity_id = $1")
            .bind(place_id)
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(pointer, latest_historic_id);
    let historic_rows: Vec<HistoricActionColumns> =
        sqlx::query_as("SELECT prose, request_id, request_fingerprint FROM activity ORDER BY id")
            .fetch_all(&pool)
            .await
            .unwrap();
    assert_eq!(
        historic_rows,
        vec![
            HistoricActionColumns {
                prose: None,
                request_id: None,
                request_fingerprint: None,
            },
            HistoricActionColumns {
                prose: None,
                request_id: None,
                request_fingerprint: None,
            },
        ]
    );
    let locations: i64 = sqlx::query_scalar("SELECT count(*) FROM entity_location")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(locations, 0);
}

#[sqlx::test(migrations = "./migration")]
async fn place_pointer_failure_rolls_back_entity_activity_and_location(pool: PgPool) {
    let world = World::new(pool.clone());
    let user_id = create_user(&world).await;
    world
        .create_character(user_id, character("Mara Venn"))
        .await
        .unwrap();
    world
        .create_entry_place(user_id, place("North Gate"))
        .await
        .unwrap();
    world.enter_world(user_id).await.unwrap();
    let before = world
        .list_activity_at_current_place(user_id, ListActivityAtCurrentPlace::default())
        .await
        .unwrap()
        .place_revision;
    sqlx::raw_sql(
        r#"
        CREATE FUNCTION reject_place_pointer_update() RETURNS trigger LANGUAGE plpgsql AS $$
        BEGIN
            IF NEW.latest_activity_id <> OLD.latest_activity_id THEN
                RAISE EXCEPTION 'forced pointer failure';
            END IF;
            RETURN NEW;
        END;
        $$;
        CREATE TRIGGER reject_place_pointer_update BEFORE UPDATE ON place
            FOR EACH ROW EXECUTE FUNCTION reject_place_pointer_update();
        "#,
    )
    .execute(&pool)
    .await
    .unwrap();
    assert_eq!(
        world
            .create_entity(user_id, entity("Rolled Back Pointer Entity"))
            .await,
        Err(WorldError::Unavailable)
    );
    let after = world
        .list_activity_at_current_place(user_id, ListActivityAtCurrentPlace::default())
        .await
        .unwrap()
        .place_revision;
    assert_eq!(after, before);
    let counts: (i64, i64, i64) = sqlx::query_as(
        r#"
        SELECT
            (SELECT count(*) FROM entity WHERE name = 'Rolled Back Pointer Entity'),
            (SELECT count(*) FROM activity WHERE operation = 'create_entity'
                AND context_place_entity_id IS NOT NULL),
            (SELECT count(*) FROM entity_location)
        "#,
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(counts, (0, 0, 0));
}

#[sqlx::test(migrations = "./migration")]
async fn every_place_relevant_writer_waits_for_the_same_place_lock(pool: PgPool) {
    let world = World::new(pool.clone());
    let first_user = create_user(&world).await;
    let second_user = create_user(&world).await;
    world
        .create_character(first_user, character("Mara Venn"))
        .await
        .unwrap();
    let place = world
        .create_entry_place(first_user, place("North Gate"))
        .await
        .unwrap();
    world.enter_world(first_user).await.unwrap();
    world
        .create_character(second_user, character("Tomas Reed"))
        .await
        .unwrap();

    let mut blocker = pool.begin().await.unwrap();
    sqlx::query("SELECT entity_id FROM place WHERE entity_id = $1 FOR UPDATE")
        .bind(place.entity.id.0)
        .fetch_one(&mut *blocker)
        .await
        .unwrap();
    let enter_world = {
        let world = world.clone();
        tokio::spawn(async move { world.enter_world(second_user).await })
    };
    wait_for_database_lock_waiter(&pool).await;
    assert!(!enter_world.is_finished());
    blocker.rollback().await.unwrap();
    enter_world.await.unwrap().unwrap();

    let before_entity = world
        .list_entity_at_current_place(first_user, ListEntityAtCurrentPlace::default())
        .await
        .unwrap()
        .place_revision;
    let mut blocker = pool.begin().await.unwrap();
    sqlx::query("SELECT entity_id FROM place WHERE entity_id = $1 FOR UPDATE")
        .bind(place.entity.id.0)
        .fetch_one(&mut *blocker)
        .await
        .unwrap();
    let create_entity = {
        let world = world.clone();
        tokio::spawn(async move {
            world
                .create_entity(first_user, entity("Lock-disciplined Referent"))
                .await
        })
    };
    wait_for_database_lock_waiter(&pool).await;
    assert!(!create_entity.is_finished());
    blocker.rollback().await.unwrap();
    create_entity.await.unwrap().unwrap();
    let after_entity = world
        .list_entity_at_current_place(first_user, ListEntityAtCurrentPlace::default())
        .await
        .unwrap()
        .place_revision;
    assert_ne!(after_entity, before_entity);

    let mut blocker = pool.begin().await.unwrap();
    sqlx::query("SELECT entity_id FROM place WHERE entity_id = $1 FOR UPDATE")
        .bind(place.entity.id.0)
        .fetch_one(&mut *blocker)
        .await
        .unwrap();
    let submit_action = {
        let world = world.clone();
        tokio::spawn(async move {
            world
                .submit_action(
                    first_user,
                    action(Uuid::new_v4(), after_entity, "Lock-disciplined Marker"),
                )
                .await
        })
    };
    wait_for_database_lock_waiter(&pool).await;
    assert!(!submit_action.is_finished());
    blocker.rollback().await.unwrap();
    submit_action.await.unwrap().unwrap();
}

#[sqlx::test(migrations = "./migration")]
async fn exact_place_page_and_revision_share_one_snapshot_during_concurrent_commit(pool: PgPool) {
    let world = World::new(pool.clone());
    let first_user = create_user(&world).await;
    let character = world
        .create_character(first_user, character("Mara Venn"))
        .await
        .unwrap();
    let place = world
        .create_entry_place(first_user, place("North Gate"))
        .await
        .unwrap();
    world.enter_world(first_user).await.unwrap();
    let before = world
        .list_activity_at_current_place(first_user, ListActivityAtCurrentPlace::default())
        .await
        .unwrap();

    let mut blocker = pool.begin().await.unwrap();
    sqlx::query("LOCK TABLE activity_entity IN ACCESS EXCLUSIVE MODE")
        .execute(&mut *blocker)
        .await
        .unwrap();
    let page_during_commit = {
        let world = world.clone();
        tokio::spawn(async move {
            world
                .list_activity_at_current_place(first_user, ListActivityAtCurrentPlace::default())
                .await
        })
    };
    wait_for_database_lock_waiter(&pool).await;
    assert!(!page_during_commit.is_finished());

    let concurrent_activity_id = Uuid::new_v4();
    let mut writer = pool.begin().await.unwrap();
    sqlx::query(
        r#"
        INSERT INTO activity (
            id, operation, requested_by_user_id,
            actor_character_entity_id, context_place_entity_id
        )
        VALUES ($1, 'create_entity', $2, $3, $4)
        "#,
    )
    .bind(concurrent_activity_id)
    .bind(first_user.0)
    .bind(character.entity.id.0)
    .bind(place.entity.id.0)
    .execute(&mut *writer)
    .await
    .unwrap();
    sqlx::query("UPDATE place SET latest_activity_id = $1 WHERE entity_id = $2")
        .bind(concurrent_activity_id)
        .bind(place.entity.id.0)
        .execute(&mut *writer)
        .await
        .unwrap();
    writer
        .commit()
        .await
        .expect("concurrent Place state should commit while the page query waits");
    let after_commit = PlaceRevision::from_parts(
        place.entity.id,
        sqlx::query_scalar("SELECT occurred_at FROM activity WHERE id = $1")
            .bind(concurrent_activity_id)
            .fetch_one(&pool)
            .await
            .unwrap(),
        aicadia::ActivityId(concurrent_activity_id),
    );
    assert_ne!(after_commit, before.place_revision);
    blocker.rollback().await.unwrap();

    let page_during_commit = page_during_commit.await.unwrap().unwrap();
    assert_eq!(page_during_commit.place_revision, before.place_revision);
    assert_eq!(page_during_commit.activity, before.activity);
    assert!(
        !page_during_commit
            .activity
            .iter()
            .any(|activity| activity.id.0 == concurrent_activity_id)
    );
    let next_snapshot = world
        .list_activity_at_current_place(first_user, ListActivityAtCurrentPlace::default())
        .await
        .unwrap();
    assert_eq!(next_snapshot.place_revision, after_commit);
    assert!(
        next_snapshot
            .activity
            .iter()
            .any(|activity| activity.id.0 == concurrent_activity_id)
    );
}
