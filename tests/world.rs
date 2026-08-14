use aicadia::{
    AcceptedActionConsequence, ActionConsequence, ActionField, ActivityEntityRole,
    ActivityOperation, ActivityTraitChange, ChangeEntityProperty, ChangeEntityTrait,
    CreateCharacter, CreateEntity, CreateEntryPlace, EntityCurrentAssociation, EntityField,
    EntityId, EntityPropertyChangeInput, EntityTraitChangeInput, EntityTraitId,
    GetEntityAtCurrentPlace, GetEntityCurrentState, InteractionField, IntroduceEntity,
    InvalidReason, ListActivity, ListActivityAtCurrentPlace, ListEntity, ListEntityAtCurrentPlace,
    PlaceRevision, PropertyField, PropertyInput, PropertyValue, SubmitAction, SubmitInteraction,
    UserId, World, WorldError,
};
use chrono::{TimeZone, Utc};
use sha2::{Digest, Sha256};
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
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
        property: Vec::new(),
    }
}

fn character(name: &str) -> CreateCharacter {
    CreateCharacter {
        name: name.to_owned(),
        description: format!("Description of {name}"),
        property: Vec::new(),
    }
}

fn place(name: &str) -> CreateEntryPlace {
    CreateEntryPlace {
        name: name.to_owned(),
        description: format!("Description of {name}"),
        property: Vec::new(),
    }
}

fn action(request_id: Uuid, expected_place_revision: PlaceRevision, name: &str) -> SubmitAction {
    SubmitAction {
        request_id,
        expected_place_revision,
        prose: format!("Mara braces the {name} beside the crossing."),
        consequence: ActionConsequence::IntroduceEntity(IntroduceEntity {
            name: name.to_owned(),
            description: format!("Description of {name}"),
            property: Vec::new(),
        }),
    }
}

fn interaction(
    request_id: Uuid,
    expected_place_revision: PlaceRevision,
    prose: &str,
    target_entity_id: Vec<EntityId>,
) -> SubmitInteraction {
    SubmitInteraction {
        request_id,
        expected_place_revision,
        prose: prose.to_owned(),
        target_entity_id,
        property_change: Vec::new(),
        trait_change: Vec::new(),
    }
}

fn text_property(key: impl Into<String>, value: impl Into<String>) -> PropertyInput {
    PropertyInput {
        key: key.into(),
        value: PropertyValue::Text(value.into()),
    }
}

fn integer_property(key: impl Into<String>, value: i64) -> PropertyInput {
    PropertyInput {
        key: key.into(),
        value: PropertyValue::Integer(value),
    }
}

fn property_change(
    entity_id: EntityId,
    key: impl Into<String>,
    value: PropertyValue,
) -> EntityPropertyChangeInput {
    EntityPropertyChangeInput {
        entity_id,
        key: key.into(),
        value,
    }
}

fn property_action(
    request_id: Uuid,
    expected_place_revision: PlaceRevision,
    prose: impl Into<String>,
    property_change: Vec<EntityPropertyChangeInput>,
) -> SubmitAction {
    SubmitAction {
        request_id,
        expected_place_revision,
        prose: prose.into(),
        consequence: ActionConsequence::ChangeEntityProperty(ChangeEntityProperty {
            property_change,
        }),
    }
}

fn establish_trait(entity_id: EntityId, statement: impl Into<String>) -> EntityTraitChangeInput {
    EntityTraitChangeInput::Establish {
        entity_id,
        statement: statement.into(),
    }
}

fn develop_trait(trait_id: EntityTraitId, statement: impl Into<String>) -> EntityTraitChangeInput {
    EntityTraitChangeInput::Develop {
        trait_id,
        statement: statement.into(),
    }
}

fn trait_action(
    request_id: Uuid,
    expected_place_revision: PlaceRevision,
    prose: impl Into<String>,
    trait_change: Vec<EntityTraitChangeInput>,
) -> SubmitAction {
    SubmitAction {
        request_id,
        expected_place_revision,
        prose: prose.into(),
        consequence: ActionConsequence::ChangeEntityTrait(ChangeEntityTrait { trait_change }),
    }
}

fn accepted_trait_change(accepted: &aicadia::AcceptedAction) -> &[ActivityTraitChange] {
    match &accepted.consequence {
        AcceptedActionConsequence::ChangeEntityTrait(change) => change,
        AcceptedActionConsequence::IntroduceEntity(_)
        | AcceptedActionConsequence::ChangeEntityProperty(_) => {
            panic!("expected Trait changes")
        }
    }
}

fn introduced_entity(accepted: &aicadia::AcceptedAction) -> &aicadia::Entity {
    match &accepted.consequence {
        AcceptedActionConsequence::IntroduceEntity(entity) => entity,
        AcceptedActionConsequence::ChangeEntityProperty(_) => {
            panic!("expected an introduced Entity")
        }
        AcceptedActionConsequence::ChangeEntityTrait(_) => panic!("expected an introduced Entity"),
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

async fn insert_trait_action_activity(pool: &PgPool, user_id: UserId) -> Uuid {
    let activity_id = Uuid::new_v4();
    sqlx::query(
        r#"
        INSERT INTO activity (
            id, operation, requested_by_user_id, prose,
            request_id, request_fingerprint, action_consequence
        )
        VALUES (
            $1, 'submit_action', $2, 'A Trait changes.',
            $3, $4, 'change_entity_trait'
        )
        "#,
    )
    .bind(activity_id)
    .bind(user_id.0)
    .bind(Uuid::new_v4())
    .bind(vec![8_u8; 32])
    .execute(pool)
    .await
    .expect("a raw Trait Action Activity should be valid");
    activity_id
}

async fn insert_trait_interaction_activity(
    pool: &PgPool,
    user_id: UserId,
    actor_entity_id: EntityId,
    place_entity_id: EntityId,
) -> Uuid {
    let activity_id = Uuid::new_v4();
    sqlx::query(
        r#"
        INSERT INTO activity (
            id, operation, requested_by_user_id,
            actor_character_entity_id, context_place_entity_id,
            prose, request_id, request_fingerprint, action_consequence
        )
        VALUES (
            $1, 'submit_interaction', $2, $3, $4,
            'An encounter establishes a Trait.', $5, $6, NULL
        )
        "#,
    )
    .bind(activity_id)
    .bind(user_id.0)
    .bind(actor_entity_id.0)
    .bind(place_entity_id.0)
    .bind(Uuid::new_v4())
    .bind(vec![9_u8; 32])
    .execute(pool)
    .await
    .expect("a raw Trait Interaction Activity should be valid");
    activity_id
}

async fn insert_complete_trait_root(
    pool: &PgPool,
    trait_id: Uuid,
    entity_id: EntityId,
    activity_id: Uuid,
    statement: &str,
) {
    let mut transaction = pool.begin().await.unwrap();
    sqlx::query("INSERT INTO entity_trait (id, entity_id) VALUES ($1, $2)")
        .bind(trait_id)
        .bind(entity_id.0)
        .execute(&mut *transaction)
        .await
        .unwrap();
    sqlx::query(
        r#"
        INSERT INTO entity_trait_version (
            trait_id, entity_id, activity_id, statement
        ) VALUES ($1, $2, $3, $4)
        "#,
    )
    .bind(trait_id)
    .bind(entity_id.0)
    .bind(activity_id)
    .bind(statement)
    .execute(&mut *transaction)
    .await
    .unwrap();
    sqlx::query(
        "INSERT INTO entity_trait_current (trait_id, entity_id, current_activity_id) VALUES ($1, $2, $3)",
    )
    .bind(trait_id)
    .bind(entity_id.0)
    .bind(activity_id)
    .execute(&mut *transaction)
    .await
    .unwrap();
    transaction.commit().await.unwrap();
}

async fn entered_characters(
    world: &World,
    name: &[&str],
) -> (aicadia::Place, Vec<(UserId, EntityId)>) {
    assert!(!name.is_empty());
    let first_user_id = create_user(world).await;
    let first_character = world
        .create_character(first_user_id, character(name[0]))
        .await
        .unwrap();
    let entry = world
        .create_entry_place(first_user_id, place("North Gate"))
        .await
        .unwrap();
    world.enter_world(first_user_id).await.unwrap();
    let mut result = vec![(first_user_id, first_character.entity.id)];
    for character_name in &name[1..] {
        let user_id = create_user(world).await;
        let character = world
            .create_character(user_id, character(character_name))
            .await
            .unwrap();
        world.enter_world(user_id).await.unwrap();
        result.push((user_id, character.entity.id));
    }
    (entry, result)
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
                property: Vec::new(),
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
                property: Vec::new(),
            },
        )
        .await
        .expect("maximum lengths should be valid");

    let invalid_input = [
        (
            CreateEntity {
                name: "   ".to_owned(),
                description: "Valid".to_owned(),
                property: Vec::new(),
            },
            EntityField::Name,
            InvalidReason::Empty,
        ),
        (
            CreateEntity {
                name: "n".repeat(121),
                description: "Valid".to_owned(),
                property: Vec::new(),
            },
            EntityField::Name,
            InvalidReason::TooLong,
        ),
        (
            CreateEntity {
                name: "Valid".to_owned(),
                description: "   ".to_owned(),
                property: Vec::new(),
            },
            EntityField::Description,
            InvalidReason::Empty,
        ),
        (
            CreateEntity {
                name: "Valid".to_owned(),
                description: "d".repeat(4_001),
                property: Vec::new(),
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
                property: Vec::new(),
            },
            EntityField::Name,
        ),
        (
            CreateEntity {
                name: "Valid".to_owned(),
                description: "A pale\0fungus".to_owned(),
                property: Vec::new(),
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
    assert_eq!(
        world
            .get_character(user_id, GetEntityCurrentState::default())
            .await
            .map(|page| page.character),
        Ok(created.clone())
    );
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
    assert_eq!(
        World::new(pool)
            .get_character(user_id, GetEntityCurrentState::default())
            .await
            .map(|page| page.character),
        Ok(created)
    );
}

#[sqlx::test(migrations = "./migration")]
async fn character_operations_distinguish_unknown_user_missing_and_existing_character(
    pool: PgPool,
) {
    let world = World::new(pool.clone());
    let unknown_user_id = UserId(Uuid::new_v4());
    assert_eq!(
        world
            .get_character(unknown_user_id, GetEntityCurrentState::default())
            .await,
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
        world
            .get_character(user_id, GetEntityCurrentState::default())
            .await,
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
                property: Vec::new(),
            },
            EntityField::Name,
            InvalidReason::Empty,
        ),
        (
            CreateCharacter {
                name: "Valid".to_owned(),
                description: "d".repeat(4_001),
                property: Vec::new(),
            },
            EntityField::Description,
            InvalidReason::TooLong,
        ),
        (
            CreateCharacter {
                name: "Invalid\0name".to_owned(),
                description: "Valid".to_owned(),
                property: Vec::new(),
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
        world
            .get_character(user_id, GetEntityCurrentState::default())
            .await
            .unwrap()
            .character
            .current_place,
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
                    description: "Valid".to_owned(),
                    property: Vec::new(),
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
    assert_eq!(
        world
            .get_character(user_id, GetEntityCurrentState::default())
            .await
            .map(|page| page.character),
        Ok(entered)
    );
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
        world
            .get_character(user_id, GetEntityCurrentState::default())
            .await
            .unwrap()
            .character
            .current_place,
        None
    );
    let enter_count: i64 =
        sqlx::query_scalar("SELECT count(*) FROM activity WHERE operation = 'enter_world'")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(enter_count, 0);
    assert_eq!(
        world
            .get_character(user_id, GetEntityCurrentState::default())
            .await
            .unwrap()
            .character
            .entity
            .id,
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
        DROP TABLE entity_property, entity_property_history, property_key,
                   activity_entity, activity, place, character, entity, "user" CASCADE;
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
    assert_eq!(before.entity.len(), 1);
    assert_eq!(before.entity[0].name, "Tomas Reed");
    let request_id = Uuid::new_v4();
    let accepted = world
        .submit_action(
            first_user,
            action(request_id, before.place_revision, "Cedar Crossing Marker"),
        )
        .await
        .expect("grounded action should be accepted");

    assert_eq!(accepted.place, entry);
    let accepted_entity = match &accepted.consequence {
        AcceptedActionConsequence::IntroduceEntity(entity) => entity,
        AcceptedActionConsequence::ChangeEntityProperty(_) => {
            panic!("the helper submits an introduction")
        }
        AcceptedActionConsequence::ChangeEntityTrait(_) => {
            panic!("the helper submits an introduction")
        }
    };
    assert_eq!(accepted_entity.name, "Cedar Crossing Marker");
    assert_eq!(accepted.activity.operation, ActivityOperation::SubmitAction);
    assert_eq!(
        accepted.activity.prose.as_deref(),
        Some("Mara braces the Cedar Crossing Marker beside the crossing.")
    );
    assert_eq!(
        accepted.activity.actor_character.as_ref().unwrap().id,
        world
            .get_character(first_user, GetEntityCurrentState::default())
            .await
            .unwrap()
            .character
            .entity
            .id
    );
    assert_eq!(
        accepted.activity.context_place.as_ref().unwrap().entity.id,
        entry.entity.id
    );
    assert!(accepted.activity.involved_entity.iter().any(|reference| {
        reference.entity.id == accepted_entity.id && reference.role == ActivityEntityRole::Subject
    }));
    assert!(accepted.activity.involved_entity.iter().any(|reference| {
        reference.entity.id == entry.entity.id && reference.role == ActivityEntityRole::Location
    }));

    let visible_entity = world
        .list_entity_at_current_place(second_user, ListEntityAtCurrentPlace::default())
        .await
        .unwrap();
    assert_eq!(visible_entity.entity.len(), 2);
    assert!(
        visible_entity
            .entity
            .iter()
            .any(|entity| entity.id == accepted_entity.id)
    );
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
    .bind(accepted_entity.id.0)
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
                consequence: ActionConsequence::IntroduceEntity(IntroduceEntity {
                    name: "  Cedar Marker  ".to_owned(),
                    description: "  Three lines cross its face.  ".to_owned(),
                    property: Vec::new(),
                }),
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
                consequence: ActionConsequence::IntroduceEntity(IntroduceEntity {
                    name: "Cedar Marker".to_owned(),
                    description: "Three lines cross its face.".to_owned(),
                    property: Vec::new(),
                }),
            },
        )
        .await
        .unwrap();
    assert_eq!(retry, first);
    assert_eq!(first.activity.prose.as_deref(), Some("Mara sets a marker."));
    assert!(matches!(
        &first.consequence,
        AcceptedActionConsequence::IntroduceEntity(entity) if entity.name == "Cedar Marker"
    ));

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
                consequence: ActionConsequence::IntroduceEntity(IntroduceEntity {
                    name: "Valid".to_owned(),
                    description: "Valid".to_owned(),
                    property: Vec::new(),
                }),
            },
            ActionField::Prose,
            InvalidReason::Empty,
        ),
        (
            SubmitAction {
                request_id: Uuid::new_v4(),
                expected_place_revision: revision,
                prose: "Valid".to_owned(),
                consequence: ActionConsequence::IntroduceEntity(IntroduceEntity {
                    name: "Bad\0name".to_owned(),
                    description: "Valid".to_owned(),
                    property: Vec::new(),
                }),
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
        DROP TABLE entity_property, entity_property_history, property_key,
                   entity_location, activity_entity, activity, place, character,
                   entity, "user" CASCADE;
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

#[sqlx::test(migrations = "./migration")]
async fn interaction_builds_directional_many_to_many_history_and_scopes_each_view(pool: PgPool) {
    let world = World::new(pool.clone());
    let (entry, character_id) =
        entered_characters(&world, &["Pip the Grey Rat", "Mara Venn", "Eno Vale"]).await;
    let (pip_user, pip) = character_id[0];
    let (mara_user, mara) = character_id[1];
    let (eno_user, eno) = character_id[2];
    let distant_user = create_user(&world).await;
    world
        .create_character(distant_user, character("Lysa Beyond the Gate"))
        .await
        .unwrap();

    let mara_before = world
        .get_character(mara_user, GetEntityCurrentState::default())
        .await
        .unwrap()
        .character;
    let mara_context = world
        .list_entity_at_current_place(mara_user, ListEntityAtCurrentPlace::default())
        .await
        .unwrap();
    assert_eq!(
        mara_context
            .entity
            .iter()
            .find(|entity| entity.id == pip)
            .map(|entity| (entity.name.as_str(), entity.description.as_str())),
        Some(("Pip the Grey Rat", "Description of Pip the Grey Rat")),
        "the rat must be an ordinary safe contextual Entity, not a control category"
    );

    let marker_revision = world
        .list_entity_at_current_place(pip_user, ListEntityAtCurrentPlace::default())
        .await
        .unwrap()
        .place_revision;
    let bowl_action = world
        .submit_action(
            pip_user,
            action(Uuid::new_v4(), marker_revision, "Small Copper Bowl"),
        )
        .await
        .unwrap();
    let bowl = match bowl_action.consequence {
        AcceptedActionConsequence::IntroduceEntity(entity) => entity.id,
        AcceptedActionConsequence::ChangeEntityProperty(_) => {
            panic!("the helper submits an introduction")
        }
        AcceptedActionConsequence::ChangeEntityTrait(_) => {
            panic!("the helper submits an introduction")
        }
    };

    let first_page = world
        .list_entity_at_current_place(
            pip_user,
            ListEntityAtCurrentPlace {
                cursor: None,
                limit: 2,
            },
        )
        .await
        .unwrap();
    let second_page = world
        .list_entity_at_current_place(
            pip_user,
            ListEntityAtCurrentPlace {
                cursor: first_page.next,
                limit: 2,
            },
        )
        .await
        .unwrap();
    assert_eq!(first_page.place, entry);
    assert_eq!(first_page.place_revision, second_page.place_revision);
    assert_eq!(first_page.entity.len() + second_page.entity.len(), 3);
    let contextual_entity = first_page
        .entity
        .iter()
        .chain(&second_page.entity)
        .collect::<Vec<_>>();
    for expected in [mara, eno, bowl] {
        let actual = contextual_entity
            .iter()
            .find(|entity| entity.id == expected)
            .expect("every other co-present Entity should be a safe target fact");
        assert!(!actual.name.is_empty());
        assert!(!actual.description.is_empty());
    }
    assert!(!contextual_entity.iter().any(|entity| entity.id == pip));
    assert!(
        !contextual_entity
            .iter()
            .any(|entity| entity.id == entry.entity.id)
    );

    let pip_revision = world
        .list_activity_at_current_place(pip_user, ListActivityAtCurrentPlace::default())
        .await
        .unwrap()
        .place_revision;
    let pip_interaction = world
        .submit_interaction(
            pip_user,
            interaction(
                Uuid::new_v4(),
                pip_revision,
                "Pip darts in three quick circles around Mara's feet and noses the bowl.",
                vec![entry.entity.id, bowl, mara],
            ),
        )
        .await
        .unwrap();
    assert_eq!(pip_interaction.place, entry);
    assert_eq!(
        pip_interaction.activity.operation,
        ActivityOperation::SubmitInteraction
    );
    assert_eq!(
        pip_interaction
            .activity
            .actor_character
            .as_ref()
            .unwrap()
            .id,
        pip
    );
    let target = pip_interaction
        .activity
        .involved_entity
        .iter()
        .filter(|reference| reference.role == ActivityEntityRole::Target)
        .map(|reference| reference.entity.id)
        .collect::<std::collections::HashSet<_>>();
    assert_eq!(
        target,
        std::collections::HashSet::from([entry.entity.id, bowl, mara])
    );
    assert!(
        pip_interaction
            .activity
            .involved_entity
            .iter()
            .any(|reference| {
                reference.entity.id == entry.entity.id
                    && reference.role == ActivityEntityRole::Location
            })
    );

    let mara_personal = world
        .list_activity(mara_user, ListActivity::default())
        .await
        .unwrap();
    assert!(
        mara_personal
            .activity
            .iter()
            .any(|activity| activity.id == pip_interaction.activity.id)
    );
    let eno_personal = world
        .list_activity(eno_user, ListActivity::default())
        .await
        .unwrap();
    assert!(
        !eno_personal
            .activity
            .iter()
            .any(|activity| activity.id == pip_interaction.activity.id)
    );
    let eno_place = world
        .list_activity_at_current_place(eno_user, ListActivityAtCurrentPlace::default())
        .await
        .unwrap();
    assert!(
        !eno_place
            .activity
            .iter()
            .any(|activity| activity.id == pip_interaction.activity.id)
    );
    assert!(
        eno_place
            .activity
            .iter()
            .any(|activity| activity.id == bowl_action.activity.id)
    );
    let distant_personal = world
        .list_activity(distant_user, ListActivity::default())
        .await
        .unwrap();
    assert!(
        !distant_personal
            .activity
            .iter()
            .any(|activity| activity.id == pip_interaction.activity.id),
        "an unplaced distant Character must not receive the local Interaction"
    );
    assert_eq!(
        world
            .get_character(mara_user, GetEntityCurrentState::default())
            .await
            .unwrap()
            .character,
        mara_before,
        "target participation must not mutate the target Character"
    );

    let repeated_revision = world
        .list_activity_at_current_place(pip_user, ListActivityAtCurrentPlace::default())
        .await
        .unwrap()
        .place_revision;
    let repeated = world
        .submit_interaction(
            pip_user,
            interaction(
                Uuid::new_v4(),
                repeated_revision,
                "Pip traces another small circle around Mara's feet.",
                vec![mara],
            ),
        )
        .await
        .expect("repeated confirmed targeting remains accepted in this slice");
    assert_ne!(repeated.activity.id, pip_interaction.activity.id);
    assert_eq!(
        world
            .get_character(mara_user, GetEntityCurrentState::default())
            .await
            .unwrap()
            .character,
        mara_before,
        "repeated targeting still must not author or mutate Mara's response"
    );
    let repeated_count: i64 = sqlx::query_scalar(
        r#"
        SELECT count(*)
        FROM activity_entity
        WHERE entity_id = $1
          AND role = 'target'
          AND activity_id = ANY($2)
        "#,
    )
    .bind(mara.0)
    .bind(vec![pip_interaction.activity.id.0, repeated.activity.id.0])
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(
        repeated_count, 2,
        "this proves the documented deferred attention-control boundary"
    );

    let reply_revision = world
        .list_activity_at_current_place(mara_user, ListActivityAtCurrentPlace::default())
        .await
        .unwrap()
        .place_revision;
    let reply = world
        .submit_interaction(
            mara_user,
            interaction(
                Uuid::new_v4(),
                reply_revision,
                "Mara crouches and offers Pip an open palm.",
                vec![pip],
            ),
        )
        .await
        .unwrap();
    let convergence_revision = world
        .list_entity_at_current_place(eno_user, ListEntityAtCurrentPlace::default())
        .await
        .unwrap()
        .place_revision;
    let convergence = world
        .submit_interaction(
            eno_user,
            interaction(
                Uuid::new_v4(),
                convergence_revision,
                "Eno sets a folded scrap of cloth beside Pip.",
                vec![pip],
            ),
        )
        .await
        .unwrap();
    let pip_history = world
        .list_activity(pip_user, ListActivity::default())
        .await
        .unwrap()
        .activity;
    assert!(
        pip_history
            .iter()
            .any(|activity| activity.id == pip_interaction.activity.id)
    );
    assert!(
        pip_history
            .iter()
            .any(|activity| activity.id == repeated.activity.id)
    );
    assert!(
        pip_history
            .iter()
            .any(|activity| activity.id == reply.activity.id)
    );
    assert!(
        pip_history
            .iter()
            .any(|activity| activity.id == convergence.activity.id)
    );
    assert!(
        pip_history.windows(2).all(|pair| {
            (pair[0].occurred_at, pair[0].id.0) > (pair[1].occurred_at, pair[1].id.0)
        })
    );
}

#[sqlx::test(migrations = "./migration")]
async fn interaction_uses_one_neutral_error_for_every_well_formed_unavailable_target(pool: PgPool) {
    let world = World::new(pool.clone());
    let (_entry, character) = entered_characters(&world, &["Pip", "Mara"]).await;
    let (pip_user, pip) = character[0];
    let (mara_user, mara) = character[1];
    let remote = world
        .create_entity(pip_user, entity("Distant Unplaced Bell"))
        .await
        .unwrap();
    let revision = world
        .list_entity_at_current_place(pip_user, ListEntityAtCurrentPlace::default())
        .await
        .unwrap()
        .place_revision;
    for target_entity_id in [
        vec![pip],
        vec![mara, mara],
        vec![EntityId(Uuid::new_v4())],
        vec![remote.id],
    ] {
        assert_eq!(
            world
                .submit_interaction(
                    pip_user,
                    interaction(
                        Uuid::new_v4(),
                        revision,
                        "Pip makes one grounded attempt.",
                        target_entity_id,
                    ),
                )
                .await,
            Err(WorldError::InteractionTargetUnavailable)
        );
    }

    sqlx::query("UPDATE character SET current_place_entity_id = NULL WHERE owner_user_id = $1")
        .bind(mara_user.0)
        .execute(&pool)
        .await
        .unwrap();
    assert_eq!(
        world
            .submit_interaction(
                pip_user,
                interaction(
                    Uuid::new_v4(),
                    revision,
                    "Pip looks toward where Mara had been.",
                    vec![mara],
                ),
            )
            .await,
        Err(WorldError::InteractionTargetUnavailable)
    );
    let count: i64 =
        sqlx::query_scalar("SELECT count(*) FROM activity WHERE operation = 'submit_interaction'")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(count, 0);
}

#[sqlx::test(migrations = "./migration")]
async fn interaction_validates_bounds_freshness_and_canonical_delivery_identity(pool: PgPool) {
    let world = World::new(pool.clone());
    let (_entry, character) = entered_characters(&world, &["Pip", "Mara"]).await;
    let (pip_user, _pip) = character[0];
    let (_mara_user, mara) = character[1];
    let original_revision = world
        .list_entity_at_current_place(pip_user, ListEntityAtCurrentPlace::default())
        .await
        .unwrap()
        .place_revision;

    for (prose, target_entity_id, field, reason) in [
        (
            "  ".to_owned(),
            vec![mara],
            InteractionField::Prose,
            InvalidReason::Empty,
        ),
        (
            "Pip waits.".to_owned(),
            Vec::new(),
            InteractionField::TargetEntityId,
            InvalidReason::OutOfRange,
        ),
        (
            "Pip waits.".to_owned(),
            (0..101)
                .map(|_| EntityId(Uuid::new_v4()))
                .collect::<Vec<_>>(),
            InteractionField::TargetEntityId,
            InvalidReason::OutOfRange,
        ),
    ] {
        assert_eq!(
            world
                .submit_interaction(
                    pip_user,
                    SubmitInteraction {
                        request_id: Uuid::new_v4(),
                        expected_place_revision: original_revision,
                        prose,
                        target_entity_id,
                        property_change: Vec::new(),
                        trait_change: Vec::new(),
                    },
                )
                .await,
            Err(WorldError::InvalidInteraction { field, reason })
        );
    }
    let rejected_bound_count: i64 =
        sqlx::query_scalar("SELECT count(*) FROM activity WHERE operation = 'submit_interaction'")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(
        rejected_bound_count, 0,
        "zero and 101 target requests must leave no Interaction history"
    );

    world
        .create_entity(pip_user, entity("Unplaced revision marker"))
        .await
        .unwrap();
    assert_eq!(
        world
            .submit_interaction(
                pip_user,
                interaction(
                    Uuid::new_v4(),
                    original_revision,
                    "Pip approaches Mara.",
                    vec![mara],
                ),
            )
            .await,
        Err(WorldError::PlaceRevisionConflict)
    );
    let stale_count: i64 =
        sqlx::query_scalar("SELECT count(*) FROM activity WHERE operation = 'submit_interaction'")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(stale_count, 0, "a stale Interaction must write nothing");

    let revision = world
        .list_activity_at_current_place(pip_user, ListActivityAtCurrentPlace::default())
        .await
        .unwrap()
        .place_revision;
    let request_id = Uuid::new_v4();
    let accepted = world
        .submit_interaction(
            pip_user,
            interaction(
                request_id,
                revision,
                "  Pip circles Mara once.  ",
                vec![mara, revision.place_entity_id()],
            ),
        )
        .await
        .unwrap();
    let retry = world
        .submit_interaction(
            pip_user,
            interaction(
                request_id,
                revision,
                "Pip circles Mara once.",
                vec![revision.place_entity_id(), mara],
            ),
        )
        .await
        .unwrap();
    assert_eq!(retry, accepted);
    assert_eq!(
        accepted.activity.prose.as_deref(),
        Some("Pip circles Mara once.")
    );
    assert_eq!(
        world
            .submit_interaction(
                pip_user,
                interaction(
                    request_id,
                    revision,
                    "Pip circles Mara twice.",
                    vec![mara, revision.place_entity_id()],
                ),
            )
            .await,
        Err(WorldError::InteractionRequestConflict)
    );

    sqlx::query("UPDATE character SET current_place_entity_id = NULL WHERE owner_user_id = $1")
        .bind(pip_user.0)
        .execute(&pool)
        .await
        .unwrap();
    assert_eq!(
        world
            .submit_interaction(
                pip_user,
                interaction(
                    request_id,
                    revision,
                    "Pip circles Mara once.",
                    vec![mara, revision.place_entity_id()],
                ),
            )
            .await,
        Ok(accepted.clone())
    );
    assert_eq!(
        world
            .submit_action(
                pip_user,
                action(request_id, revision, "Cross-operation collision")
            )
            .await,
        Err(WorldError::ActionRequestConflict)
    );
    let count: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM activity WHERE requested_by_user_id = $1 AND request_id = $2",
    )
    .bind(pip_user.0)
    .bind(request_id)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(count, 1);
}

#[sqlx::test(migrations = "./migration")]
async fn interaction_cross_operation_conflict_concurrency_and_rollback_are_atomic(pool: PgPool) {
    let world = World::new(pool.clone());
    let (_entry, character) = entered_characters(&world, &["Pip", "Mara"]).await;
    let (pip_user, pip) = character[0];
    let (mara_user, mara) = character[1];

    let action_revision = world
        .list_entity_at_current_place(pip_user, ListEntityAtCurrentPlace::default())
        .await
        .unwrap()
        .place_revision;
    let action_request_id = Uuid::new_v4();
    world
        .submit_action(
            pip_user,
            action(action_request_id, action_revision, "Existing Action"),
        )
        .await
        .unwrap();
    let current_revision = world
        .list_activity_at_current_place(pip_user, ListActivityAtCurrentPlace::default())
        .await
        .unwrap()
        .place_revision;
    assert_eq!(
        world
            .submit_interaction(
                pip_user,
                interaction(
                    action_request_id,
                    current_revision,
                    "Pip greets Mara.",
                    vec![mara],
                ),
            )
            .await,
        Err(WorldError::InteractionRequestConflict)
    );

    let equal_request_id = Uuid::new_v4();
    let equal_request = interaction(
        equal_request_id,
        current_revision,
        "Pip greets Mara.",
        vec![mara],
    );
    let (first, second) = tokio::join!(
        world.submit_interaction(pip_user, equal_request.clone()),
        world.submit_interaction(pip_user, equal_request)
    );
    assert!(first.is_ok());
    assert_eq!(first, second);
    let equal_count: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM activity WHERE requested_by_user_id = $1 AND request_id = $2",
    )
    .bind(pip_user.0)
    .bind(equal_request_id)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(equal_count, 1);

    let shared_revision = world
        .list_entity_at_current_place(pip_user, ListEntityAtCurrentPlace::default())
        .await
        .unwrap()
        .place_revision;
    let (pip_attempt, mara_attempt) = tokio::join!(
        world.submit_interaction(
            pip_user,
            interaction(
                Uuid::new_v4(),
                shared_revision,
                "Pip runs toward Mara.",
                vec![mara],
            )
        ),
        world.submit_interaction(
            mara_user,
            interaction(
                Uuid::new_v4(),
                shared_revision,
                "Mara reaches toward Pip.",
                vec![pip],
            )
        )
    );
    assert_eq!(
        usize::from(pip_attempt.is_ok()) + usize::from(mara_attempt.is_ok()),
        1
    );
    assert_eq!(
        if pip_attempt.is_err() {
            pip_attempt
        } else {
            mara_attempt
        },
        Err(WorldError::PlaceRevisionConflict)
    );

    let before_rollback: i64 =
        sqlx::query_scalar("SELECT count(*) FROM activity WHERE operation = 'submit_interaction'")
            .fetch_one(&pool)
            .await
            .unwrap();
    let rollback_revision = world
        .list_activity_at_current_place(pip_user, ListActivityAtCurrentPlace::default())
        .await
        .unwrap()
        .place_revision;
    sqlx::raw_sql(
        r#"
        CREATE FUNCTION reject_interaction_pointer_update() RETURNS trigger LANGUAGE plpgsql AS $$
        BEGIN
            IF NEW.latest_activity_id <> OLD.latest_activity_id THEN
                RAISE EXCEPTION 'forced interaction pointer failure';
            END IF;
            RETURN NEW;
        END;
        $$;
        CREATE TRIGGER reject_interaction_pointer_update BEFORE UPDATE ON place
            FOR EACH ROW EXECUTE FUNCTION reject_interaction_pointer_update();
        "#,
    )
    .execute(&pool)
    .await
    .unwrap();
    assert_eq!(
        world
            .submit_interaction(
                pip_user,
                interaction(
                    Uuid::new_v4(),
                    rollback_revision,
                    "This interaction must roll back.",
                    vec![mara],
                ),
            )
            .await,
        Err(WorldError::Unavailable)
    );
    let after_rollback: i64 =
        sqlx::query_scalar("SELECT count(*) FROM activity WHERE operation = 'submit_interaction'")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(after_rollback, before_rollback);
}

#[sqlx::test(migrations = "./migration")]
async fn interaction_migration_stores_and_decodes_one_to_many_target_history(pool: PgPool) {
    let world = World::new(pool.clone());
    let actor_user_id = create_user(&world).await;
    let target_user_id = create_user(&world).await;
    let actor = world
        .create_character(actor_user_id, character("Pip"))
        .await
        .unwrap();
    let place = world
        .create_entry_place(actor_user_id, place("North Gate"))
        .await
        .unwrap();
    world.enter_world(actor_user_id).await.unwrap();
    let target = world
        .create_character(target_user_id, character("Mara Venn"))
        .await
        .unwrap();
    world.enter_world(target_user_id).await.unwrap();
    let bowl = world
        .create_entity(actor_user_id, entity("Food Bowl"))
        .await
        .unwrap();
    let activity_id = Uuid::new_v4();

    let mut transaction = pool.begin().await.unwrap();
    sqlx::query(
        r#"
        INSERT INTO activity (
            id, operation, requested_by_user_id,
            actor_character_entity_id, context_place_entity_id,
            prose, request_id, request_fingerprint
        )
        VALUES ($1, 'submit_interaction', $2, $3, $4, $5, $6, $7)
        "#,
    )
    .bind(activity_id)
    .bind(actor_user_id.0)
    .bind(actor.entity.id.0)
    .bind(place.entity.id.0)
    .bind("Pip darts around Mara and the bowl.")
    .bind(Uuid::new_v4())
    .bind(vec![7_u8; 32])
    .execute(&mut *transaction)
    .await
    .expect("valid Interaction Activity should be accepted");
    sqlx::query(
        r#"
        INSERT INTO activity_entity (activity_id, entity_id, role)
        SELECT $1, involved.entity_id, involved.role
        FROM UNNEST($2::uuid[], $3::text[]) AS involved(entity_id, role)
        "#,
    )
    .bind(activity_id)
    .bind(vec![target.entity.id.0, bowl.id.0, place.entity.id.0])
    .bind(vec!["target", "target", "location"])
    .execute(&mut *transaction)
    .await
    .expect("target and location roles should be accepted together");
    transaction.commit().await.unwrap();

    let activity = world
        .list_activity(target_user_id, ListActivity::default())
        .await
        .unwrap()
        .activity
        .into_iter()
        .find(|activity| activity.id.0 == activity_id)
        .expect("a target Character should decode the stored Interaction");
    assert_eq!(activity.operation, ActivityOperation::SubmitInteraction);
    assert_eq!(activity.actor_character.unwrap().id, actor.entity.id);
    assert_eq!(activity.context_place.unwrap().entity.id, place.entity.id);
    assert_eq!(
        activity
            .involved_entity
            .iter()
            .filter(|reference| reference.role == ActivityEntityRole::Target)
            .map(|reference| reference.entity.id)
            .collect::<std::collections::HashSet<_>>(),
        [target.entity.id, bowl.id].into_iter().collect()
    );

    assert!(
        sqlx::query(
            "INSERT INTO activity_entity (activity_id, entity_id, role) VALUES ($1, $2, 'target')",
        )
        .bind(activity_id)
        .bind(target.entity.id.0)
        .execute(&pool)
        .await
        .is_err(),
        "the existing composite primary key must reject a duplicate role"
    );
    assert!(
        sqlx::query("UPDATE activity SET prose = 'changed' WHERE id = $1")
            .bind(activity_id)
            .execute(&pool)
            .await
            .is_err(),
        "Interaction Activity must retain the existing immutable-history rule"
    );
    assert!(
        sqlx::query(
            "DELETE FROM activity_entity WHERE activity_id = $1 AND entity_id = $2 AND role = 'target'",
        )
        .bind(activity_id)
        .bind(target.entity.id.0)
        .execute(&pool)
        .await
        .is_err(),
        "Interaction participation must retain the existing immutable-history rule"
    );
}

#[sqlx::test(migrations = "./migration")]
async fn interaction_migration_rejects_invalid_operation_provenance_context_and_role(pool: PgPool) {
    let world = World::new(pool.clone());
    let user_id = create_user(&world).await;
    let actor = world
        .create_character(user_id, character("Pip"))
        .await
        .unwrap();
    let place = world
        .create_entry_place(user_id, place("North Gate"))
        .await
        .unwrap();
    world.enter_world(user_id).await.unwrap();

    assert!(
        sqlx::query("INSERT INTO activity (id, operation, requested_by_user_id) VALUES ($1, 'unknown_operation', $2)")
            .bind(Uuid::new_v4())
            .bind(user_id.0)
            .execute(&pool)
            .await
            .is_err()
    );
    assert!(
        sqlx::query(
            r#"
            INSERT INTO activity (
                id, operation, requested_by_user_id,
                actor_character_entity_id, context_place_entity_id
            )
            VALUES ($1, 'submit_interaction', $2, $3, $4)
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(user_id.0)
        .bind(actor.entity.id.0)
        .bind(place.entity.id.0)
        .execute(&pool)
        .await
        .is_err(),
        "Interaction requires confirmed prose and request provenance"
    );
    assert!(
        sqlx::query(
            r#"
            INSERT INTO activity (
                id, operation, requested_by_user_id,
                actor_character_entity_id, context_place_entity_id,
                prose, request_id, request_fingerprint
            )
            VALUES ($1, 'create_entity', $2, $3, $4, 'not allowed', $5, $6)
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(user_id.0)
        .bind(actor.entity.id.0)
        .bind(place.entity.id.0)
        .bind(Uuid::new_v4())
        .bind(vec![1_u8; 32])
        .execute(&pool)
        .await
        .is_err(),
        "non-confirmed operations must retain null prose and request provenance"
    );
    for (actor_id, place_id) in [
        (None, Some(place.entity.id.0)),
        (Some(actor.entity.id.0), None),
    ] {
        assert!(
            sqlx::query(
                r#"
                INSERT INTO activity (
                    id, operation, requested_by_user_id,
                    actor_character_entity_id, context_place_entity_id,
                    prose, request_id, request_fingerprint
                )
                VALUES ($1, 'submit_interaction', $2, $3, $4, 'Pip circles.', $5, $6)
                "#,
            )
            .bind(Uuid::new_v4())
            .bind(user_id.0)
            .bind(actor_id)
            .bind(place_id)
            .bind(Uuid::new_v4())
            .bind(vec![2_u8; 32])
            .execute(&pool)
            .await
            .is_err(),
            "Interaction must have both an actor Character and context Place"
        );
    }

    let activity_id: Uuid =
        sqlx::query_scalar("SELECT id FROM activity WHERE operation = 'create_character' LIMIT 1")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert!(
        sqlx::query(
            "INSERT INTO activity_entity (activity_id, entity_id, role) VALUES ($1, $2, 'invalid_role')",
        )
        .bind(activity_id)
        .bind(place.entity.id.0)
        .execute(&pool)
        .await
        .is_err()
    );
}

#[sqlx::test(migrations = "./migration")]
async fn interaction_relation_failure_rolls_back_activity_and_partial_targets_and_index_exists(
    pool: PgPool,
) {
    let world = World::new(pool.clone());
    let user_id = create_user(&world).await;
    let actor = world
        .create_character(user_id, character("Pip"))
        .await
        .unwrap();
    let place = world
        .create_entry_place(user_id, place("North Gate"))
        .await
        .unwrap();
    world.enter_world(user_id).await.unwrap();
    let activity_id = Uuid::new_v4();

    let mut transaction = pool.begin().await.unwrap();
    sqlx::query(
        r#"
        INSERT INTO activity (
            id, operation, requested_by_user_id,
            actor_character_entity_id, context_place_entity_id,
            prose, request_id, request_fingerprint
        )
        VALUES ($1, 'submit_interaction', $2, $3, $4, 'Pip circles.', $5, $6)
        "#,
    )
    .bind(activity_id)
    .bind(user_id.0)
    .bind(actor.entity.id.0)
    .bind(place.entity.id.0)
    .bind(Uuid::new_v4())
    .bind(vec![3_u8; 32])
    .execute(&mut *transaction)
    .await
    .unwrap();
    assert!(
        sqlx::query(
            r#"
            INSERT INTO activity_entity (activity_id, entity_id, role)
            SELECT $1, involved.entity_id, 'target'
            FROM UNNEST($2::uuid[]) AS involved(entity_id)
            "#,
        )
        .bind(activity_id)
        .bind(vec![place.entity.id.0, Uuid::new_v4()])
        .execute(&mut *transaction)
        .await
        .is_err(),
        "one invalid target relation must fail the complete bulk statement"
    );
    transaction.rollback().await.unwrap();
    let persisted: (i64, i64) = sqlx::query_as(
        r#"
        SELECT
            (SELECT count(*) FROM activity WHERE id = $1),
            (SELECT count(*) FROM activity_entity WHERE activity_id = $1)
        "#,
    )
    .bind(activity_id)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(persisted, (0, 0));

    let index_definition: String = sqlx::query_scalar(
        r#"
        SELECT indexdef
        FROM pg_indexes
        WHERE schemaname = current_schema()
          AND tablename = 'character'
          AND indexname = 'character_current_place_entity_id_entity_id_index'
        "#,
    )
    .fetch_one(&pool)
    .await
    .expect("the exact-Place Character index should exist");
    assert!(index_definition.contains("(current_place_entity_id, entity_id)"));
    assert!(index_definition.contains("WHERE (current_place_entity_id IS NOT NULL)"));
}

#[sqlx::test(migrations = "./migration")]
async fn property_migration_backfills_actions_and_allows_many_changes_per_activity(pool: PgPool) {
    sqlx::raw_sql(
        r#"
        DROP TABLE entity_property, entity_property_history, property_key,
                   entity_location, activity_entity, activity, place, character,
                   entity, "user" CASCADE;
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
        include_str!("../migration/0005_agent_action.sql"),
        include_str!("../migration/0006_entity_interaction.sql"),
    ] {
        sqlx::raw_sql(migration).execute(&pool).await.unwrap();
    }

    let user_id = Uuid::new_v4();
    let action_id = Uuid::new_v4();
    sqlx::query("INSERT INTO \"user\" (id) VALUES ($1)")
        .bind(user_id)
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query(
        r#"
        INSERT INTO activity (
            id, operation, requested_by_user_id, prose, request_id, request_fingerprint
        )
        VALUES ($1, 'submit_action', $2, 'Historic introduction.', $3, $4)
        "#,
    )
    .bind(action_id)
    .bind(user_id)
    .bind(Uuid::new_v4())
    .bind(vec![1_u8; 32])
    .execute(&pool)
    .await
    .unwrap();

    sqlx::raw_sql(include_str!("../migration/0007_entity_property.sql"))
        .execute(&pool)
        .await
        .unwrap();
    let discriminator: Option<String> =
        sqlx::query_scalar("SELECT action_consequence FROM activity WHERE id = $1")
            .bind(action_id)
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(discriminator.as_deref(), Some("introduce_entity"));

    let entity_id = [Uuid::new_v4(), Uuid::new_v4()];
    for (index, id) in entity_id.iter().enumerate() {
        sqlx::query(
            "INSERT INTO entity (id, name, description, introduced_by_user_id) VALUES ($1, $2, 'Historic entity', $3)",
        )
        .bind(id)
        .bind(format!("Historic Property Entity {index}"))
        .bind(user_id)
        .execute(&pool)
        .await
        .unwrap();
    }
    let key_id: i64 = sqlx::query_scalar(
        "INSERT INTO property_key (key, value_type, first_activity_id) VALUES ('colour', 'text', $1) RETURNING id",
    )
    .bind(action_id)
    .fetch_one(&pool)
    .await
    .unwrap();
    sqlx::query(
        r#"
        INSERT INTO entity_property_history (
            entity_id, property_key_id, activity_id, value_type, text_value
        )
        SELECT submitted.entity_id, $2, $3, 'text', submitted.value
        FROM UNNEST($1::uuid[], $4::text[]) AS submitted(entity_id, value)
        "#,
    )
    .bind(entity_id.to_vec())
    .bind(key_id)
    .bind(action_id)
    .bind(vec!["grey", "green"])
    .execute(&pool)
    .await
    .expect("one Activity must own multiple Property history rows");
    let count: i64 =
        sqlx::query_scalar("SELECT count(*) FROM entity_property_history WHERE activity_id = $1")
            .bind(action_id)
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(count, 2);
}

#[sqlx::test(migrations = "./migration")]
async fn property_schema_rejects_invalid_keys_values_lineage_and_history_changes(pool: PgPool) {
    let world = World::new(pool.clone());
    let user_id = create_user(&world).await;
    let first = world
        .create_entity(user_id, entity("First Property Subject"))
        .await
        .unwrap();
    let second = world
        .create_entity(user_id, entity("Second Property Subject"))
        .await
        .unwrap();
    let first_activity_id: Uuid = sqlx::query_scalar(
        "SELECT activity_id FROM activity_entity WHERE entity_id = $1 AND role = 'subject'",
    )
    .bind(first.id.0)
    .fetch_one(&pool)
    .await
    .unwrap();
    let second_activity_id: Uuid = sqlx::query_scalar(
        "SELECT activity_id FROM activity_entity WHERE entity_id = $1 AND role = 'subject'",
    )
    .bind(second.id.0)
    .fetch_one(&pool)
    .await
    .unwrap();

    for invalid_key in [
        "",
        "1st_colour",
        "HairColour",
        "_hair_colour",
        "hair__colour",
        "hair_colour_",
        "hair-colour",
        "éclair",
        &"a".repeat(65),
    ] {
        assert!(
            sqlx::query(
                "INSERT INTO property_key (key, value_type, first_activity_id) VALUES ($1, 'text', $2)",
            )
            .bind(invalid_key)
            .bind(first_activity_id)
            .execute(&pool)
            .await
            .is_err(),
            "invalid canonical key must fail: {invalid_key}"
        );
    }
    assert!(
        sqlx::query(
            "INSERT INTO property_key (key, value_type, first_activity_id) VALUES ('size', 'decimal', $1)",
        )
        .bind(first_activity_id)
        .execute(&pool)
        .await
        .is_err()
    );

    let text_key_id: i64 = sqlx::query_scalar(
        "INSERT INTO property_key (key, value_type, first_activity_id) VALUES ('hair_colour', 'text', $1) RETURNING id",
    )
    .bind(first_activity_id)
    .fetch_one(&pool)
    .await
    .unwrap();
    let integer_key_id: i64 = sqlx::query_scalar(
        "INSERT INTO property_key (key, value_type, first_activity_id) VALUES ('leg_count', 'integer', $1) RETURNING id",
    )
    .bind(first_activity_id)
    .fetch_one(&pool)
    .await
    .unwrap();

    for (value_type, text_value, integer_value) in [
        ("text", None, None),
        ("text", Some(""), None),
        ("text", Some(" red "), None),
        ("text", Some("red"), Some(3_i64)),
        ("integer", None, None),
        ("integer", Some("three"), Some(3_i64)),
    ] {
        assert!(
            sqlx::query(
                r#"
                INSERT INTO entity_property_history (
                    entity_id, property_key_id, activity_id,
                    value_type, text_value, integer_value
                )
                VALUES ($1, $2, $3, $4, $5, $6)
                "#,
            )
            .bind(first.id.0)
            .bind(if value_type == "text" {
                text_key_id
            } else {
                integer_key_id
            })
            .bind(first_activity_id)
            .bind(value_type)
            .bind(text_value)
            .bind(integer_value)
            .execute(&pool)
            .await
            .is_err(),
            "invalid typed value must fail"
        );
    }
    assert!(
        sqlx::query(
            r#"
            INSERT INTO entity_property_history (
                entity_id, property_key_id, activity_id,
                value_type, integer_value
            )
            VALUES ($1, $2, $3, 'integer', 3)
            "#,
        )
        .bind(first.id.0)
        .bind(text_key_id)
        .bind(first_activity_id)
        .execute(&pool)
        .await
        .is_err(),
        "history type must match its canonical key type"
    );

    sqlx::query(
        r#"
        INSERT INTO entity_property_history (
            entity_id, property_key_id, activity_id, value_type, text_value
        )
        VALUES ($1, $2, $3, 'text', 'blond')
        "#,
    )
    .bind(first.id.0)
    .bind(text_key_id)
    .bind(first_activity_id)
    .execute(&pool)
    .await
    .unwrap();
    assert!(
        sqlx::query(
            r#"
            INSERT INTO entity_property_history (
                entity_id, property_key_id, activity_id, previous_activity_id,
                value_type, text_value
            )
            VALUES ($1, $2, $3, $4, 'text', 'red')
            "#,
        )
        .bind(second.id.0)
        .bind(text_key_id)
        .bind(second_activity_id)
        .bind(first_activity_id)
        .execute(&pool)
        .await
        .is_err(),
        "a predecessor must belong to the same Entity/key lineage"
    );
    assert!(
        sqlx::query(
            "INSERT INTO entity_property (entity_id, property_key_id, current_activity_id) VALUES ($1, $2, $3)",
        )
        .bind(second.id.0)
        .bind(text_key_id)
        .bind(first_activity_id)
        .execute(&pool)
        .await
        .is_err(),
        "a current pointer must reference the same Entity/key lineage"
    );
    assert!(
        sqlx::query("UPDATE entity_property_history SET text_value = 'red'")
            .execute(&pool)
            .await
            .is_err()
    );
    assert!(
        sqlx::query("DELETE FROM entity_property_history")
            .execute(&pool)
            .await
            .is_err()
    );
    assert!(
        sqlx::query("UPDATE property_key SET value_type = 'integer' WHERE id = $1")
            .bind(text_key_id)
            .execute(&pool)
            .await
            .is_err(),
        "canonical key meaning and first provenance are immutable"
    );

    assert!(
        sqlx::query(
            r#"
            INSERT INTO activity (
                id, operation, requested_by_user_id, prose,
                request_id, request_fingerprint, action_consequence
            )
            VALUES ($1, 'submit_action', $2, 'Missing discriminator.', $3, $4, NULL)
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(user_id.0)
        .bind(Uuid::new_v4())
        .bind(vec![2_u8; 32])
        .execute(&pool)
        .await
        .is_err()
    );
    assert!(
        sqlx::query(
            "INSERT INTO activity (id, operation, requested_by_user_id, action_consequence) VALUES ($1, 'create_entity', $2, 'introduce_entity')",
        )
        .bind(Uuid::new_v4())
        .bind(user_id.0)
        .execute(&pool)
        .await
        .is_err()
    );
    let property_action_id = Uuid::new_v4();
    sqlx::query(
        r#"
        INSERT INTO activity (
            id, operation, requested_by_user_id, prose,
            request_id, request_fingerprint, action_consequence
        )
        VALUES (
            $1, 'submit_action', $2, 'A Property changes.',
            $3, $4, 'change_entity_property'
        )
        "#,
    )
    .bind(property_action_id)
    .bind(user_id.0)
    .bind(Uuid::new_v4())
    .bind(vec![3_u8; 32])
    .execute(&pool)
    .await
    .expect("the second closed Action discriminator must be accepted");
    assert!(
        sqlx::query(
            r#"
            INSERT INTO activity (
                id, operation, requested_by_user_id, prose,
                request_id, request_fingerprint, action_consequence
            )
            VALUES ($1, 'submit_action', $2, 'Unknown consequence.', $3, $4, 'unknown')
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(user_id.0)
        .bind(Uuid::new_v4())
        .bind(vec![4_u8; 32])
        .execute(&pool)
        .await
        .is_err()
    );
}

#[sqlx::test(migrations = "./migration")]
async fn property_key_races_reuse_one_type_and_roll_back_a_conflicting_bundle(pool: PgPool) {
    let world = World::new(pool.clone());
    let user_id = create_user(&world).await;
    let first = world
        .create_entity(user_id, entity("First Key Racer"))
        .await
        .unwrap();
    let second = world
        .create_entity(user_id, entity("Second Key Racer"))
        .await
        .unwrap();
    let first_activity_id: Uuid = sqlx::query_scalar(
        "SELECT activity_id FROM activity_entity WHERE entity_id = $1 AND role = 'subject'",
    )
    .bind(first.id.0)
    .fetch_one(&pool)
    .await
    .unwrap();
    let second_activity_id: Uuid = sqlx::query_scalar(
        "SELECT activity_id FROM activity_entity WHERE entity_id = $1 AND role = 'subject'",
    )
    .bind(second.id.0)
    .fetch_one(&pool)
    .await
    .unwrap();

    let mut first_transaction = pool.begin().await.unwrap();
    sqlx::query(
        "INSERT INTO property_key (key, value_type, first_activity_id) VALUES ('shared_size', 'text', $1) ON CONFLICT (key) DO NOTHING",
    )
    .bind(first_activity_id)
    .execute(&mut *first_transaction)
    .await
    .unwrap();
    let same_type = {
        let pool = pool.clone();
        tokio::spawn(async move {
            let mut transaction = pool.begin().await.unwrap();
            sqlx::query(
                "INSERT INTO property_key (key, value_type, first_activity_id) VALUES ('shared_size', 'text', $1) ON CONFLICT (key) DO NOTHING",
            )
            .bind(second_activity_id)
            .execute(&mut *transaction)
            .await
            .unwrap();
            let found: (String, Uuid) = sqlx::query_as(
                "SELECT value_type, first_activity_id FROM property_key WHERE key = 'shared_size' FOR KEY SHARE",
            )
            .fetch_one(&mut *transaction)
            .await
            .unwrap();
            transaction.commit().await.unwrap();
            found
        })
    };
    wait_for_database_lock_waiter(&pool).await;
    first_transaction.commit().await.unwrap();
    assert_eq!(
        same_type.await.unwrap(),
        ("text".to_owned(), first_activity_id)
    );
    let same_count: i64 =
        sqlx::query_scalar("SELECT count(*) FROM property_key WHERE key = 'shared_size'")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(same_count, 1);

    let mut winning_transaction = pool.begin().await.unwrap();
    sqlx::query(
        "INSERT INTO property_key (key, value_type, first_activity_id) VALUES ('shared_weight', 'text', $1) ON CONFLICT (key) DO NOTHING",
    )
    .bind(first_activity_id)
    .execute(&mut *winning_transaction)
    .await
    .unwrap();
    let different_type = {
        let pool = pool.clone();
        tokio::spawn(async move {
            let mut transaction = pool.begin().await.unwrap();
            sqlx::query(
                "INSERT INTO property_key (key, value_type, first_activity_id) VALUES ('conflict_bundle_key', 'integer', $1)",
            )
            .bind(second_activity_id)
            .execute(&mut *transaction)
            .await
            .unwrap();
            sqlx::query(
                "INSERT INTO property_key (key, value_type, first_activity_id) VALUES ('shared_weight', 'integer', $1) ON CONFLICT (key) DO NOTHING",
            )
            .bind(second_activity_id)
            .execute(&mut *transaction)
            .await
            .unwrap();
            let found: String = sqlx::query_scalar(
                "SELECT value_type FROM property_key WHERE key = 'shared_weight' FOR KEY SHARE",
            )
            .fetch_one(&mut *transaction)
            .await
            .unwrap();
            if found == "integer" {
                transaction.commit().await.unwrap();
                true
            } else {
                transaction.rollback().await.unwrap();
                false
            }
        })
    };
    wait_for_database_lock_waiter(&pool).await;
    winning_transaction.commit().await.unwrap();
    assert!(!different_type.await.unwrap());
    let key_state: (String, i64) = sqlx::query_as(
        r#"
        SELECT
            (SELECT value_type FROM property_key WHERE key = 'shared_weight'),
            (SELECT count(*) FROM property_key WHERE key = 'conflict_bundle_key')
        "#,
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(key_state, ("text".to_owned(), 0));
}

#[sqlx::test(migrations = "./migration")]
async fn property_bulk_failure_rolls_back_keys_history_and_current_pointers(pool: PgPool) {
    let world = World::new(pool.clone());
    let user_id = create_user(&world).await;
    let entity = world
        .create_entity(user_id, entity("Bulk Property Subject"))
        .await
        .unwrap();
    let activity_id: Uuid = sqlx::query_scalar(
        "SELECT activity_id FROM activity_entity WHERE entity_id = $1 AND role = 'subject'",
    )
    .bind(entity.id.0)
    .fetch_one(&pool)
    .await
    .unwrap();

    let mut transaction = pool.begin().await.unwrap();
    let key_id: i64 = sqlx::query_scalar(
        "INSERT INTO property_key (key, value_type, first_activity_id) VALUES ('bulk_colour', 'text', $1) RETURNING id",
    )
    .bind(activity_id)
    .fetch_one(&mut *transaction)
    .await
    .unwrap();
    assert!(
        sqlx::query(
            r#"
            INSERT INTO entity_property_history (
                entity_id, property_key_id, activity_id, value_type, text_value
            )
            SELECT submitted.entity_id, $2, $3, 'text', submitted.value
            FROM UNNEST($1::uuid[], $4::text[]) AS submitted(entity_id, value)
            "#,
        )
        .bind(vec![entity.id.0, Uuid::new_v4()])
        .bind(key_id)
        .bind(activity_id)
        .bind(vec!["green", "red"])
        .execute(&mut *transaction)
        .await
        .is_err(),
        "one invalid Entity must reject the set-based history write"
    );
    transaction.rollback().await.unwrap();

    let counts: (i64, i64, i64) = sqlx::query_as(
        r#"
        SELECT
            (SELECT count(*) FROM property_key WHERE key = 'bulk_colour'),
            (SELECT count(*) FROM entity_property_history),
            (SELECT count(*) FROM entity_property)
        "#,
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(counts, (0, 0, 0));
}

#[sqlx::test(migrations = "./migration")]
async fn property_indexes_are_minimal_and_match_declared_access_paths(pool: PgPool) {
    let index: Vec<(String, bool, String)> = sqlx::query_as(
        r#"
        SELECT indexes.indexname, pg_index.indisunique, indexes.indexdef
        FROM pg_indexes AS indexes
        JOIN pg_class ON pg_class.relname = indexes.indexname
        JOIN pg_index ON pg_index.indexrelid = pg_class.oid
        WHERE indexes.schemaname = current_schema()
          AND indexes.tablename IN (
              'property_key', 'entity_property_history', 'entity_property'
          )
        ORDER BY indexes.indexname
        "#,
    )
    .fetch_all(&pool)
    .await
    .unwrap();
    let index_name = index
        .iter()
        .map(|(name, _, _)| name.as_str())
        .collect::<Vec<_>>();
    assert_eq!(
        index_name,
        vec![
            "entity_property_history_activity_index",
            "entity_property_history_pkey",
            "entity_property_pkey",
            "property_key_id_value_type_key",
            "property_key_key_key",
            "property_key_pkey",
        ]
    );
    let activity_index = index
        .iter()
        .find(|(name, _, _)| name == "entity_property_history_activity_index")
        .unwrap();
    assert!(
        !activity_index.1,
        "Activity history cardinality must not be unique"
    );
    assert!(
        activity_index
            .2
            .contains("(activity_id, entity_id, property_key_id)")
    );
}

#[sqlx::test(migrations = "./migration")]
async fn every_entity_creation_route_atomically_establishes_one_hundred_properties(pool: PgPool) {
    let world = World::new(pool.clone());
    let user_id = create_user(&world).await;
    let hundred = || {
        (0..100)
            .map(|index| integer_property(format!("measure_{index}"), index))
            .collect::<Vec<_>>()
    };

    let ordinary = world
        .create_entity(
            user_id,
            CreateEntity {
                name: "Unplaced Herbarium".to_owned(),
                description: "One hundred measured specimens.".to_owned(),
                property: hundred(),
            },
        )
        .await
        .unwrap();
    let character = world
        .create_character(
            user_id,
            CreateCharacter {
                name: "Mara Venn".to_owned(),
                description: "A patient surveyor.".to_owned(),
                property: hundred(),
            },
        )
        .await
        .unwrap();
    let place = world
        .create_entry_place(
            user_id,
            CreateEntryPlace {
                name: "North Gate".to_owned(),
                description: "The shared threshold.".to_owned(),
                property: hundred(),
            },
        )
        .await
        .unwrap();
    world.enter_world(user_id).await.unwrap();
    let revision = world
        .list_entity_at_current_place(user_id, ListEntityAtCurrentPlace::default())
        .await
        .unwrap()
        .place_revision;
    let introduced = world
        .submit_action(
            user_id,
            SubmitAction {
                request_id: Uuid::new_v4(),
                expected_place_revision: revision,
                prose: "Mara establishes a measured cairn.".to_owned(),
                consequence: ActionConsequence::IntroduceEntity(IntroduceEntity {
                    name: "Measured Cairn".to_owned(),
                    description: "A cairn with one hundred recorded measures.".to_owned(),
                    property: hundred(),
                }),
            },
        )
        .await
        .unwrap();
    assert_eq!(introduced.activity.property_change.len(), 100);
    assert_eq!(introduced_entity(&introduced).name, "Measured Cairn");

    let counts: (i64, i64, i64, i64) = sqlx::query_as(
        r#"
        SELECT
            (SELECT count(*) FROM property_key),
            (SELECT count(*) FROM entity_property_history),
            (SELECT count(*) FROM entity_property),
            (SELECT count(DISTINCT activity_id) FROM entity_property_history)
        "#,
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(counts, (100, 400, 400, 4));
    for entity_id in [
        ordinary.id,
        character.entity.id,
        place.entity.id,
        introduced_entity(&introduced).id,
    ] {
        let count: i64 =
            sqlx::query_scalar("SELECT count(*) FROM entity_property WHERE entity_id = $1")
                .bind(entity_id.0)
                .fetch_one(&pool)
                .await
                .unwrap();
        assert_eq!(count, 100);
    }
}

#[sqlx::test(migrations = "./migration")]
async fn invalid_initial_properties_roll_back_each_creation_route_without_orphans(pool: PgPool) {
    let world = World::new(pool.clone());
    let user_id = create_user(&world).await;
    let over_bound = || {
        (0..101)
            .map(|index| integer_property(format!("measure_{index}"), index))
            .collect::<Vec<_>>()
    };
    assert_eq!(
        world
            .create_entity(
                user_id,
                CreateEntity {
                    name: "Rejected Entity".to_owned(),
                    description: "Must not persist.".to_owned(),
                    property: over_bound(),
                },
            )
            .await,
        Err(WorldError::InvalidProperty {
            field: PropertyField::Property,
            reason: InvalidReason::OutOfRange,
        })
    );
    assert_eq!(
        world
            .create_character(
                user_id,
                CreateCharacter {
                    name: "Rejected Character".to_owned(),
                    description: "Must not persist.".to_owned(),
                    property: over_bound(),
                },
            )
            .await,
        Err(WorldError::InvalidProperty {
            field: PropertyField::Property,
            reason: InvalidReason::OutOfRange,
        })
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
                    name: "Rejected Gate".to_owned(),
                    description: "Must not persist.".to_owned(),
                    property: vec![
                        text_property("colour", "grey"),
                        text_property("colour", "red")
                    ],
                },
            )
            .await,
        Err(WorldError::InvalidProperty {
            field: PropertyField::Property,
            reason: InvalidReason::Duplicate,
        })
    );
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
    assert_eq!(
        world
            .submit_action(
                user_id,
                SubmitAction {
                    request_id: Uuid::new_v4(),
                    expected_place_revision: revision,
                    prose: "This introduction is invalid.".to_owned(),
                    consequence: ActionConsequence::IntroduceEntity(IntroduceEntity {
                        name: "Rejected Marker".to_owned(),
                        description: "Must not persist.".to_owned(),
                        property: over_bound(),
                    }),
                },
            )
            .await,
        Err(WorldError::InvalidProperty {
            field: PropertyField::Property,
            reason: InvalidReason::OutOfRange,
        })
    );
    let counts: (i64, i64, i64, i64) = sqlx::query_as(
        r#"
        SELECT
            (SELECT count(*) FROM entity WHERE name LIKE 'Rejected%'),
            (SELECT count(*) FROM activity WHERE prose = 'This introduction is invalid.'),
            (SELECT count(*) FROM property_key),
            (SELECT count(*) FROM entity_property_history)
        "#,
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(counts, (0, 0, 0, 0));
}

#[sqlx::test(migrations = "./migration")]
async fn property_action_changes_actor_place_ordinary_and_other_character_uniformly(pool: PgPool) {
    let world = World::new(pool.clone());
    let (place, participant) = entered_characters(&world, &["Mara Venn", "Pip"]).await;
    let (mara_user, mara) = participant[0];
    let (_, pip) = participant[1];
    let revision = world
        .list_entity_at_current_place(mara_user, ListEntityAtCurrentPlace::default())
        .await
        .unwrap()
        .place_revision;
    let bowl_action = world
        .submit_action(mara_user, action(Uuid::new_v4(), revision, "Copper Bowl"))
        .await
        .unwrap();
    let bowl = introduced_entity(&bowl_action).id;
    let revision = world
        .list_entity_at_current_place(mara_user, ListEntityAtCurrentPlace::default())
        .await
        .unwrap()
        .place_revision;
    let accepted = world
        .submit_action(
            mara_user,
            property_action(
                Uuid::new_v4(),
                revision,
                "The blast stains every nearby surface crimson.",
                vec![
                    property_change(
                        place.entity.id,
                        "colour",
                        PropertyValue::Text("crimson".into()),
                    ),
                    property_change(pip, "colour", PropertyValue::Text("crimson".into())),
                    property_change(bowl, "colour", PropertyValue::Text("crimson".into())),
                    property_change(mara, "colour", PropertyValue::Text("crimson".into())),
                    property_change(mara, "hair_colour", PropertyValue::Text("red".into())),
                ],
            ),
        )
        .await
        .unwrap();
    let change = match &accepted.consequence {
        AcceptedActionConsequence::ChangeEntityProperty(change) => change,
        AcceptedActionConsequence::IntroduceEntity(_) => panic!("expected Property change"),
        AcceptedActionConsequence::ChangeEntityTrait(_) => panic!("expected Property change"),
    };
    assert_eq!(change, &accepted.activity.property_change);
    assert_eq!(change.len(), 5);
    assert!(change.windows(2).all(|pair| {
        (pair[0].entity.id.0.as_bytes(), pair[0].key.as_str())
            <= (pair[1].entity.id.0.as_bytes(), pair[1].key.as_str())
    }));
    let subject = accepted
        .activity
        .involved_entity
        .iter()
        .filter(|reference| reference.role == ActivityEntityRole::Subject)
        .map(|reference| reference.entity.id)
        .collect::<std::collections::HashSet<_>>();
    assert_eq!(
        subject,
        [mara, pip, bowl, place.entity.id].into_iter().collect()
    );
    assert!(accepted.activity.involved_entity.iter().any(|reference| {
        reference.entity.id == place.entity.id && reference.role == ActivityEntityRole::Location
    }));

    let mut current_property = Vec::new();
    for entity_id in [mara, pip, bowl, place.entity.id] {
        let current = world
            .get_entity_at_current_place(
                mara_user,
                GetEntityAtCurrentPlace {
                    entity_id,
                    cursor: None,
                    limit: 100,
                },
            )
            .await
            .unwrap();
        assert_eq!(current.place.entity.id, place.entity.id);
        current_property.extend(current.current_state.association.into_iter().filter_map(
            |association| match association {
                EntityCurrentAssociation::Property { key, value } => Some((key, value)),
                EntityCurrentAssociation::Trait(_) => None,
            },
        ));
    }
    assert_eq!(current_property.len(), 5);
    assert!(current_property.iter().all(|(key, value)| {
        key == "hair_colour"
            || value == &PropertyValue::Text("crimson".to_owned())
            || value == &PropertyValue::Text("red".to_owned())
    }));
    let colour_key_count: i64 =
        sqlx::query_scalar("SELECT count(*) FROM property_key WHERE key = 'colour'")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(colour_key_count, 1);
    let one_activity: i64 = sqlx::query_scalar(
        "SELECT count(DISTINCT activity_id) FROM entity_property_history WHERE activity_id = $1",
    )
    .bind(accepted.activity.id.0)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(one_activity, 1);
}

#[sqlx::test(migrations = "./migration")]
async fn property_action_rejects_unavailable_mixed_subjects_and_retries_sorted_history(
    pool: PgPool,
) {
    let world = World::new(pool.clone());
    let user_id = create_user(&world).await;
    let actor = world
        .create_character(user_id, character("Mara Venn"))
        .await
        .unwrap();
    world
        .create_entry_place(user_id, place("North Gate"))
        .await
        .unwrap();
    world.enter_world(user_id).await.unwrap();
    let unavailable = world
        .create_entity(
            user_id,
            CreateEntity {
                name: "Unplaced Remote Stone".to_owned(),
                description: "It has no current Place.".to_owned(),
                property: vec![text_property("colour", "grey")],
            },
        )
        .await
        .unwrap();
    let revision = world
        .list_entity_at_current_place(user_id, ListEntityAtCurrentPlace::default())
        .await
        .unwrap()
        .place_revision;
    let before_activity: i64 = sqlx::query_scalar("SELECT count(*) FROM activity")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(
        world
            .submit_action(
                user_id,
                property_action(
                    Uuid::new_v4(),
                    revision,
                    "A change cannot reach the absent stone.",
                    vec![
                        property_change(
                            actor.entity.id,
                            "colour",
                            PropertyValue::Text("red".into()),
                        ),
                        property_change(
                            unavailable.id,
                            "colour",
                            PropertyValue::Text("red".into()),
                        ),
                    ],
                ),
            )
            .await,
        Err(WorldError::PropertyEntityUnavailable)
    );
    let after_activity: i64 = sqlx::query_scalar("SELECT count(*) FROM activity")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(after_activity, before_activity);
    let actor_colour: i64 = sqlx::query_scalar(
        r#"
        SELECT count(*)
        FROM entity_property
        JOIN property_key ON property_key.id = entity_property.property_key_id
        WHERE entity_property.entity_id = $1 AND property_key.key = 'colour'
        "#,
    )
    .bind(actor.entity.id.0)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(actor_colour, 0);

    let request_id = Uuid::new_v4();
    let original = world
        .submit_action(
            user_id,
            property_action(
                request_id,
                revision,
                "Mara changes two facts.",
                vec![
                    property_change(actor.entity.id, "size", PropertyValue::Text("small".into())),
                    property_change(actor.entity.id, "leg_count", PropertyValue::Integer(3)),
                ],
            ),
        )
        .await
        .unwrap();
    let later_revision = world
        .list_entity_at_current_place(user_id, ListEntityAtCurrentPlace::default())
        .await
        .unwrap()
        .place_revision;
    world
        .submit_action(
            user_id,
            property_action(
                Uuid::new_v4(),
                later_revision,
                "Mara grows again.",
                vec![property_change(
                    actor.entity.id,
                    "size",
                    PropertyValue::Text("tall".into()),
                )],
            ),
        )
        .await
        .unwrap();
    let retry = world
        .submit_action(
            user_id,
            property_action(
                request_id,
                revision,
                "Mara changes two facts.",
                vec![
                    property_change(actor.entity.id, "leg_count", PropertyValue::Integer(3)),
                    property_change(actor.entity.id, "size", PropertyValue::Text("small".into())),
                ],
            ),
        )
        .await
        .unwrap();
    assert_eq!(retry, original);
    assert_eq!(
        world
            .submit_action(
                user_id,
                property_action(
                    request_id,
                    revision,
                    "Mara changes two facts.",
                    vec![
                        property_change(actor.entity.id, "leg_count", PropertyValue::Integer(4)),
                        property_change(
                            actor.entity.id,
                            "size",
                            PropertyValue::Text("small".into())
                        ),
                    ],
                ),
            )
            .await,
        Err(WorldError::ActionRequestConflict)
    );
}

#[sqlx::test(migrations = "./migration")]
async fn property_world_validation_rejects_keys_values_duplicates_and_change_bounds(pool: PgPool) {
    let world = World::new(pool.clone());
    let user_id = create_user(&world).await;
    let actor = world
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

    let invalid = vec![
        (
            Vec::new(),
            PropertyField::PropertyChange,
            InvalidReason::OutOfRange,
        ),
        (
            vec![property_change(
                actor.entity.id,
                "HairColour",
                PropertyValue::Text("red".into()),
            )],
            PropertyField::Key,
            InvalidReason::InvalidFormat,
        ),
        (
            vec![property_change(
                actor.entity.id,
                "hair_colour",
                PropertyValue::Text(" \0 ".into()),
            )],
            PropertyField::Value,
            InvalidReason::ContainsNul,
        ),
        (
            vec![
                property_change(
                    actor.entity.id,
                    "hair_colour",
                    PropertyValue::Text("red".into()),
                ),
                property_change(
                    actor.entity.id,
                    "hair_colour",
                    PropertyValue::Text("blue".into()),
                ),
            ],
            PropertyField::PropertyChange,
            InvalidReason::Duplicate,
        ),
        (
            (0..101)
                .map(|index| {
                    property_change(
                        actor.entity.id,
                        format!("measure_{index}"),
                        PropertyValue::Integer(index),
                    )
                })
                .collect(),
            PropertyField::PropertyChange,
            InvalidReason::OutOfRange,
        ),
    ];
    for (property_change, field, reason) in invalid {
        assert_eq!(
            world
                .submit_action(
                    user_id,
                    property_action(
                        Uuid::new_v4(),
                        revision,
                        "This invalid change must not be accepted.",
                        property_change,
                    ),
                )
                .await,
            Err(WorldError::InvalidProperty { field, reason })
        );
    }
    let count: i64 =
        sqlx::query_scalar("SELECT count(*) FROM activity WHERE operation = 'submit_action'")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(count, 0);
}

#[sqlx::test(migrations = "./migration")]
async fn interaction_property_changes_actor_and_target_without_authoring_a_response(pool: PgPool) {
    let world = World::new(pool.clone());
    let (_, participant) = entered_characters(&world, &["Pip", "Mara", "Tomas"]).await;
    let (pip_user, pip) = participant[0];
    let (mara_user, mara) = participant[1];
    let (_, tomas) = participant[2];
    let revision = world
        .list_entity_at_current_place(pip_user, ListEntityAtCurrentPlace::default())
        .await
        .unwrap()
        .place_revision;
    let request_id = Uuid::new_v4();
    let accepted = world
        .submit_interaction(
            pip_user,
            SubmitInteraction {
                request_id,
                expected_place_revision: revision,
                prose: "Pip splashes Mara with blue dye.".to_owned(),
                target_entity_id: vec![mara],
                property_change: vec![
                    property_change(mara, "colour", PropertyValue::Text("blue".into())),
                    property_change(pip, "colour", PropertyValue::Text("blue".into())),
                ],
                trait_change: Vec::new(),
            },
        )
        .await
        .unwrap();
    assert_eq!(accepted.activity.property_change.len(), 2);
    assert_eq!(
        accepted
            .activity
            .involved_entity
            .iter()
            .filter(|reference| reference.role == ActivityEntityRole::Target)
            .map(|reference| reference.entity.id)
            .collect::<Vec<_>>(),
        vec![mara]
    );
    let mara_history = world
        .list_activity(mara_user, ListActivity::default())
        .await
        .unwrap();
    let observed = mara_history
        .activity
        .iter()
        .find(|activity| activity.id == accepted.activity.id)
        .unwrap();
    assert_eq!(observed, &accepted.activity);
    assert_ne!(observed.actor_character.as_ref().unwrap().id, mara);

    let later_revision = world
        .list_entity_at_current_place(pip_user, ListEntityAtCurrentPlace::default())
        .await
        .unwrap()
        .place_revision;
    world
        .submit_action(
            pip_user,
            property_action(
                Uuid::new_v4(),
                later_revision,
                "The blue dye dries on Pip.",
                vec![property_change(
                    pip,
                    "surface",
                    PropertyValue::Text("dry".into()),
                )],
            ),
        )
        .await
        .unwrap();

    let retry = world
        .submit_interaction(
            pip_user,
            SubmitInteraction {
                request_id,
                expected_place_revision: revision,
                prose: "Pip splashes Mara with blue dye.".to_owned(),
                target_entity_id: vec![mara],
                property_change: vec![
                    property_change(pip, "colour", PropertyValue::Text("blue".into())),
                    property_change(mara, "colour", PropertyValue::Text("blue".into())),
                ],
                trait_change: Vec::new(),
            },
        )
        .await
        .unwrap();
    assert_eq!(retry, accepted);
    let current_revision = world
        .list_entity_at_current_place(pip_user, ListEntityAtCurrentPlace::default())
        .await
        .unwrap()
        .place_revision;
    let before: i64 =
        sqlx::query_scalar("SELECT count(*) FROM activity WHERE operation = 'submit_interaction'")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(
        world
            .submit_interaction(
                pip_user,
                SubmitInteraction {
                    request_id: Uuid::new_v4(),
                    expected_place_revision: current_revision,
                    prose: "Pip cannot affect a bystander without targeting them.".to_owned(),
                    target_entity_id: vec![mara],
                    property_change: vec![property_change(
                        tomas,
                        "colour",
                        PropertyValue::Text("blue".into()),
                    )],
                    trait_change: Vec::new(),
                },
            )
            .await,
        Err(WorldError::PropertyEntityUnavailable)
    );
    let after: i64 =
        sqlx::query_scalar("SELECT count(*) FROM activity WHERE operation = 'submit_interaction'")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(after, before);
}

#[sqlx::test(migrations = "./migration")]
async fn current_property_read_paginates_local_facts_and_excludes_unplaced_entities(pool: PgPool) {
    let world = World::new(pool.clone());
    let user_id = create_user(&world).await;
    let actor = world
        .create_character(
            user_id,
            CreateCharacter {
                name: "Mara Venn".to_owned(),
                description: "Blond hair is introductory history.".to_owned(),
                property: Vec::new(),
            },
        )
        .await
        .unwrap();
    let place = world
        .create_entry_place(
            user_id,
            CreateEntryPlace {
                name: "North Gate".to_owned(),
                description: "The threshold.".to_owned(),
                property: vec![text_property("weather", "clear")],
            },
        )
        .await
        .unwrap();
    world.enter_world(user_id).await.unwrap();
    let revision = world
        .list_entity_at_current_place(user_id, ListEntityAtCurrentPlace::default())
        .await
        .unwrap()
        .place_revision;
    let mut change = (0..99)
        .map(|index| {
            property_change(
                actor.entity.id,
                format!("measure_{index}"),
                PropertyValue::Integer(index),
            )
        })
        .collect::<Vec<_>>();
    change.push(property_change(
        actor.entity.id,
        "hair_colour",
        PropertyValue::Text("red".into()),
    ));
    let accepted = world
        .submit_action(
            user_id,
            property_action(
                Uuid::new_v4(),
                revision,
                "Mara's hair turns red while ninety-nine measures settle.",
                change,
            ),
        )
        .await
        .unwrap();
    assert_eq!(accepted.activity.property_change.len(), 100);
    let remote = world
        .create_entity(
            user_id,
            CreateEntity {
                name: "Unplaced Almanac".to_owned(),
                description: "Not locally observable.".to_owned(),
                property: vec![text_property("secret_mark", "remote")],
            },
        )
        .await
        .unwrap();
    let first = world
        .get_entity_at_current_place(
            user_id,
            GetEntityAtCurrentPlace {
                entity_id: actor.entity.id,
                cursor: None,
                limit: 50,
            },
        )
        .await
        .unwrap();
    assert_eq!(first.current_state.association.len(), 50);
    assert!(first.current_state.next.is_some());
    let second = world
        .get_entity_at_current_place(
            user_id,
            GetEntityAtCurrentPlace {
                entity_id: actor.entity.id,
                cursor: first.current_state.next,
                limit: 50,
            },
        )
        .await
        .unwrap();
    assert_eq!(second.current_state.association.len(), 50);
    assert!(second.current_state.next.is_none());
    let all = first
        .current_state
        .association
        .into_iter()
        .chain(second.current_state.association)
        .collect::<Vec<_>>();
    assert!(all.iter().any(|association| matches!(
        association,
        EntityCurrentAssociation::Property { key, value }
            if key == "hair_colour" && value == &PropertyValue::Text("red".to_owned())
    )));
    let place_state = world
        .get_entity_at_current_place(
            user_id,
            GetEntityAtCurrentPlace {
                entity_id: place.entity.id,
                cursor: None,
                limit: 100,
            },
        )
        .await
        .unwrap();
    assert!(
        place_state
            .current_state
            .association
            .iter()
            .any(|association| matches!(
                association,
                EntityCurrentAssociation::Property { key, value }
                    if key == "weather" && value == &PropertyValue::Text("clear".to_owned())
            ))
    );
    assert_eq!(
        world
            .get_entity_at_current_place(
                user_id,
                GetEntityAtCurrentPlace {
                    entity_id: remote.id,
                    cursor: None,
                    limit: 100,
                },
            )
            .await,
        Err(WorldError::EntityAtCurrentPlaceUnavailable)
    );
    assert_eq!(
        world
            .get_entity_at_current_place(
                user_id,
                GetEntityAtCurrentPlace {
                    entity_id: actor.entity.id,
                    cursor: None,
                    limit: 0,
                },
            )
            .await,
        Err(WorldError::InvalidEntityLimit)
    );
}

#[sqlx::test(migrations = "./migration")]
async fn concurrent_world_first_key_use_reuses_type_and_rolls_back_type_conflict(pool: PgPool) {
    let world = World::new(pool.clone());
    let first_user = create_user(&world).await;
    let second_user = create_user(&world).await;
    let same_first = {
        let world = world.clone();
        tokio::spawn(async move {
            world
                .create_entity(
                    first_user,
                    CreateEntity {
                        name: "First Grey Stone".to_owned(),
                        description: "A concurrent first use.".to_owned(),
                        property: vec![text_property("surface", "rough")],
                    },
                )
                .await
        })
    };
    let same_second = {
        let world = world.clone();
        tokio::spawn(async move {
            world
                .create_entity(
                    second_user,
                    CreateEntity {
                        name: "Second Grey Stone".to_owned(),
                        description: "Another concurrent first use.".to_owned(),
                        property: vec![text_property("surface", "smooth")],
                    },
                )
                .await
        })
    };
    assert!(same_first.await.unwrap().is_ok());
    assert!(same_second.await.unwrap().is_ok());
    let same_state: (i64, i64) = sqlx::query_as(
        r#"
        SELECT
            (SELECT count(*) FROM property_key WHERE key = 'surface'),
            (SELECT count(*) FROM entity_property_history
             JOIN property_key ON property_key.id = entity_property_history.property_key_id
             WHERE property_key.key = 'surface')
        "#,
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(same_state, (1, 2));

    let third_user = create_user(&world).await;
    let fourth_user = create_user(&world).await;
    let text = {
        let world = world.clone();
        tokio::spawn(async move {
            world
                .create_entity(
                    third_user,
                    CreateEntity {
                        name: "Text Weight".to_owned(),
                        description: "One type must win.".to_owned(),
                        property: vec![text_property("weight", "heavy")],
                    },
                )
                .await
        })
    };
    let integer = {
        let world = world.clone();
        tokio::spawn(async move {
            world
                .create_entity(
                    fourth_user,
                    CreateEntity {
                        name: "Integer Weight".to_owned(),
                        description: "The other type must roll back.".to_owned(),
                        property: vec![integer_property("weight", 12)],
                    },
                )
                .await
        })
    };
    let text = text.await.unwrap();
    let integer = integer.await.unwrap();
    assert_eq!(usize::from(text.is_ok()) + usize::from(integer.is_ok()), 1);
    assert_eq!(
        if text.is_err() {
            text.err()
        } else {
            integer.err()
        },
        Some(WorldError::PropertyKeyConflict)
    );
    let conflict_state: (i64, i64, i64) = sqlx::query_as(
        r#"
        SELECT
            (SELECT count(*) FROM property_key WHERE key = 'weight'),
            (SELECT count(*) FROM entity WHERE name IN ('Text Weight', 'Integer Weight')),
            (SELECT count(*) FROM activity
             JOIN activity_entity ON activity_entity.activity_id = activity.id
             JOIN entity ON entity.id = activity_entity.entity_id
             WHERE entity.name IN ('Text Weight', 'Integer Weight'))
        "#,
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(conflict_state, (1, 1, 1));
}

#[sqlx::test(migrations = "./migration")]
async fn reversed_multi_key_actions_at_distinct_places_complete_without_deadlock(pool: PgPool) {
    let world = World::new(pool.clone());
    let first_user = create_user(&world).await;
    let first_character = world
        .create_character(first_user, character("Northern Surveyor"))
        .await
        .unwrap();
    let first_place = world
        .create_entry_place(first_user, place("North Gate"))
        .await
        .unwrap();
    world.enter_world(first_user).await.unwrap();

    let second_user = create_user(&world).await;
    let second_character = world
        .create_character(second_user, character("Southern Surveyor"))
        .await
        .unwrap();
    let second_place_entity = world
        .create_entity(second_user, entity("South Gate"))
        .await
        .unwrap();
    let second_place_genesis: Uuid = sqlx::query_scalar(
        "SELECT activity_id FROM activity_entity WHERE entity_id = $1 AND role = 'subject'",
    )
    .bind(second_place_entity.id.0)
    .fetch_one(&pool)
    .await
    .unwrap();
    let second_enter_activity = Uuid::new_v4();
    let mut setup = pool.begin().await.unwrap();
    sqlx::query(
        "INSERT INTO place (entity_id, is_entry, latest_activity_id) VALUES ($1, false, $2)",
    )
    .bind(second_place_entity.id.0)
    .bind(second_place_genesis)
    .execute(&mut *setup)
    .await
    .unwrap();
    sqlx::query(
        r#"
        INSERT INTO activity (
            id, operation, requested_by_user_id,
            actor_character_entity_id, context_place_entity_id
        )
        VALUES ($1, 'enter_world', $2, $3, $4)
        "#,
    )
    .bind(second_enter_activity)
    .bind(second_user.0)
    .bind(second_character.entity.id.0)
    .bind(second_place_entity.id.0)
    .execute(&mut *setup)
    .await
    .unwrap();
    sqlx::query(
        "INSERT INTO activity_entity (activity_id, entity_id, role) VALUES ($1, $2, 'destination')",
    )
    .bind(second_enter_activity)
    .bind(second_place_entity.id.0)
    .execute(&mut *setup)
    .await
    .unwrap();
    sqlx::query("UPDATE character SET current_place_entity_id = $1 WHERE entity_id = $2")
        .bind(second_place_entity.id.0)
        .bind(second_character.entity.id.0)
        .execute(&mut *setup)
        .await
        .unwrap();
    sqlx::query("UPDATE place SET latest_activity_id = $1 WHERE entity_id = $2")
        .bind(second_enter_activity)
        .bind(second_place_entity.id.0)
        .execute(&mut *setup)
        .await
        .unwrap();
    setup.commit().await.unwrap();

    let first_revision = world
        .list_entity_at_current_place(first_user, ListEntityAtCurrentPlace::default())
        .await
        .unwrap()
        .place_revision;
    let second_revision = world
        .list_entity_at_current_place(second_user, ListEntityAtCurrentPlace::default())
        .await
        .unwrap()
        .place_revision;
    assert_ne!(first_place.entity.id, second_place_entity.id);

    let timed_options = pool
        .connect_options()
        .as_ref()
        .clone()
        .options([("statement_timeout", "5s"), ("lock_timeout", "5s")]);
    let timed_pool = PgPoolOptions::new()
        .max_connections(4)
        .connect_with(timed_options)
        .await
        .unwrap();
    let barrier = std::sync::Arc::new(tokio::sync::Barrier::new(3));
    let first = {
        let world = World::new(timed_pool.clone());
        let barrier = barrier.clone();
        tokio::spawn(async move {
            barrier.wait().await;
            world
                .submit_action(
                    first_user,
                    property_action(
                        Uuid::new_v4(),
                        first_revision,
                        "The northern surveyor fixes both calibration marks.",
                        vec![
                            property_change(
                                first_character.entity.id,
                                "lock_alpha",
                                PropertyValue::Integer(1),
                            ),
                            property_change(
                                first_character.entity.id,
                                "lock_beta",
                                PropertyValue::Integer(2),
                            ),
                        ],
                    ),
                )
                .await
        })
    };
    let second = {
        let world = World::new(timed_pool.clone());
        let barrier = barrier.clone();
        tokio::spawn(async move {
            barrier.wait().await;
            world
                .submit_action(
                    second_user,
                    property_action(
                        Uuid::new_v4(),
                        second_revision,
                        "The southern surveyor fixes both calibration marks.",
                        vec![
                            property_change(
                                second_character.entity.id,
                                "lock_beta",
                                PropertyValue::Integer(20),
                            ),
                            property_change(
                                second_character.entity.id,
                                "lock_alpha",
                                PropertyValue::Integer(10),
                            ),
                        ],
                    ),
                )
                .await
        })
    };
    barrier.wait().await;
    let first = first.await.unwrap().unwrap();
    let second = second.await.unwrap().unwrap();
    assert_eq!(first.activity.property_change.len(), 2);
    assert_eq!(second.activity.property_change.len(), 2);
    assert_eq!(
        first
            .activity
            .property_change
            .iter()
            .map(|change| change.key.as_str())
            .collect::<Vec<_>>(),
        vec!["lock_alpha", "lock_beta"]
    );
    assert_eq!(
        second
            .activity
            .property_change
            .iter()
            .map(|change| change.key.as_str())
            .collect::<Vec<_>>(),
        vec!["lock_alpha", "lock_beta"]
    );
    let counts: (i64, i64, i64, i64) = sqlx::query_as(
        r#"
        SELECT
            (SELECT count(*) FROM property_key
             WHERE key IN ('lock_alpha', 'lock_beta')),
            (SELECT count(*) FROM entity_property_history
             JOIN property_key ON property_key.id = entity_property_history.property_key_id
             WHERE property_key.key IN ('lock_alpha', 'lock_beta')),
            (SELECT count(*) FROM entity_property
             JOIN property_key ON property_key.id = entity_property.property_key_id
             WHERE property_key.key IN ('lock_alpha', 'lock_beta')),
            (SELECT count(*) FROM activity
             WHERE operation = 'submit_action'
               AND action_consequence = 'change_entity_property')
        "#,
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(counts, (2, 4, 4, 2));
    timed_pool.close().await;
}

#[sqlx::test(migrations = "./migration")]
async fn property_storage_failure_rolls_back_entity_activity_key_and_place_revision(pool: PgPool) {
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
        .list_entity_at_current_place(user_id, ListEntityAtCurrentPlace::default())
        .await
        .unwrap()
        .place_revision;
    sqlx::raw_sql(
        r#"
        CREATE FUNCTION reject_property_history_insert() RETURNS trigger LANGUAGE plpgsql AS $$
        BEGIN
            RAISE EXCEPTION 'forced Property history failure';
        END;
        $$;
        CREATE TRIGGER reject_property_history_insert BEFORE INSERT ON entity_property_history
            FOR EACH ROW EXECUTE FUNCTION reject_property_history_insert();
        "#,
    )
    .execute(&pool)
    .await
    .unwrap();
    assert_eq!(
        world
            .create_entity(
                user_id,
                CreateEntity {
                    name: "Rolled Back Property Entity".to_owned(),
                    description: "No partial bundle may survive.".to_owned(),
                    property: vec![text_property("colour", "red")],
                },
            )
            .await,
        Err(WorldError::Unavailable)
    );
    let after = world
        .list_entity_at_current_place(user_id, ListEntityAtCurrentPlace::default())
        .await
        .unwrap()
        .place_revision;
    assert_eq!(after, before);
    let counts: (i64, i64, i64, i64) = sqlx::query_as(
        r#"
        SELECT
            (SELECT count(*) FROM entity WHERE name = 'Rolled Back Property Entity'),
            (SELECT count(*) FROM activity
             WHERE operation = 'create_entity' AND context_place_entity_id IS NOT NULL),
            (SELECT count(*) FROM property_key WHERE key = 'colour'),
            (SELECT count(*) FROM entity_property_history)
        "#,
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(counts, (0, 0, 0, 0));
}

#[sqlx::test(migrations = "./migration")]
async fn trait_migration_preserves_actions_and_admits_only_action_or_interaction_causes(
    pool: PgPool,
) {
    let world = World::new(pool.clone());
    let user_id = create_user(&world).await;
    let character = world
        .create_character(user_id, character("Trait Migration Actor"))
        .await
        .unwrap();
    let place = world
        .create_entry_place(user_id, place("Trait Migration Place"))
        .await
        .unwrap();
    world.enter_world(user_id).await.unwrap();
    let entity = world
        .create_entity(user_id, entity("Trait Migration Subject"))
        .await
        .unwrap();
    let creation_activity_id: Uuid = sqlx::query_scalar(
        "SELECT activity_id FROM activity_entity WHERE entity_id = $1 AND role = 'subject'",
    )
    .bind(entity.id.0)
    .fetch_one(&pool)
    .await
    .unwrap();

    sqlx::raw_sql(
        r#"
        DROP TABLE entity_trait_current, entity_trait_version, entity_trait;
        DROP FUNCTION validate_entity_trait_version_activity();
        DROP FUNCTION validate_entity_trait_complete();
        DROP FUNCTION reject_entity_trait_current_identity_change();
        ALTER TABLE activity
            DROP CONSTRAINT activity_action_consequence_check,
            ADD CONSTRAINT activity_action_consequence_check CHECK (
                (
                    operation = 'submit_action'
                    AND action_consequence IS NOT NULL
                    AND action_consequence IN ('introduce_entity', 'change_entity_property')
                ) OR (
                    operation <> 'submit_action'
                    AND action_consequence IS NULL
                )
            );
        "#,
    )
    .execute(&pool)
    .await
    .unwrap();

    let introduction_activity_id = Uuid::new_v4();
    let property_activity_id = Uuid::new_v4();
    for (activity_id, consequence) in [
        (introduction_activity_id, "introduce_entity"),
        (property_activity_id, "change_entity_property"),
    ] {
        sqlx::query(
            r#"
            INSERT INTO activity (
                id, operation, requested_by_user_id, prose,
                request_id, request_fingerprint, action_consequence
            )
            VALUES ($1, 'submit_action', $2, 'Historic Action.', $3, $4, $5)
            "#,
        )
        .bind(activity_id)
        .bind(user_id.0)
        .bind(Uuid::new_v4())
        .bind(vec![7_u8; 32])
        .bind(consequence)
        .execute(&pool)
        .await
        .unwrap();
    }

    sqlx::raw_sql(include_str!("../migration/0008_entity_trait.sql"))
        .execute(&pool)
        .await
        .unwrap();

    let historic: Vec<(Uuid, String)> = sqlx::query_as(
        r#"
        SELECT id, action_consequence
        FROM activity
        WHERE id = ANY($1::uuid[])
        ORDER BY action_consequence
        "#,
    )
    .bind(vec![introduction_activity_id, property_activity_id])
    .fetch_all(&pool)
    .await
    .unwrap();
    assert_eq!(historic.len(), 2);
    assert!(historic.iter().any(|(_, tag)| tag == "introduce_entity"));
    assert!(
        historic
            .iter()
            .any(|(_, tag)| tag == "change_entity_property")
    );

    let action_activity_id = insert_trait_action_activity(&pool, user_id).await;
    let action_trait_id = Uuid::new_v4();
    insert_complete_trait_root(
        &pool,
        action_trait_id,
        entity.id,
        action_activity_id,
        "Waits for the second echo.",
    )
    .await;

    let interaction_activity_id =
        insert_trait_interaction_activity(&pool, user_id, character.entity.id, place.entity.id)
            .await;
    let interaction_trait_id = Uuid::new_v4();
    insert_complete_trait_root(
        &pool,
        interaction_trait_id,
        entity.id,
        interaction_activity_id,
        "Leans toward familiar footsteps.",
    )
    .await;

    let invalid_trait_id = Uuid::new_v4();
    let mut invalid = pool.begin().await.unwrap();
    sqlx::query("INSERT INTO entity_trait (id, entity_id) VALUES ($1, $2)")
        .bind(invalid_trait_id)
        .bind(entity.id.0)
        .execute(&mut *invalid)
        .await
        .unwrap();
    assert!(
        sqlx::query(
            r#"
            INSERT INTO entity_trait_version (
                trait_id, entity_id, activity_id, statement
            ) VALUES ($1, $2, $3, 'Creation cannot establish this Trait.')
            "#,
        )
        .bind(invalid_trait_id)
        .bind(entity.id.0)
        .bind(creation_activity_id)
        .execute(&mut *invalid)
        .await
        .is_err(),
        "a non-Action/Interaction Activity must not own a Trait version"
    );
    invalid.rollback().await.unwrap();

    let invalid_property_trait_id = Uuid::new_v4();
    let mut invalid = pool.begin().await.unwrap();
    sqlx::query("INSERT INTO entity_trait (id, entity_id) VALUES ($1, $2)")
        .bind(invalid_property_trait_id)
        .bind(entity.id.0)
        .execute(&mut *invalid)
        .await
        .unwrap();
    assert!(
        sqlx::query(
            r#"
            INSERT INTO entity_trait_version (
                trait_id, entity_id, activity_id, statement
            ) VALUES ($1, $2, $3, 'A Property Action cannot establish this Trait.')
            "#,
        )
        .bind(invalid_property_trait_id)
        .bind(entity.id.0)
        .bind(property_activity_id)
        .execute(&mut *invalid)
        .await
        .is_err(),
        "the Action discriminator must identify a Trait consequence"
    );
    invalid.rollback().await.unwrap();

    assert!(
        sqlx::query(
            r#"
            INSERT INTO activity (
                id, operation, requested_by_user_id, prose,
                request_id, request_fingerprint, action_consequence
            ) VALUES ($1, 'submit_action', $2, 'Unknown.', $3, $4, 'trait_change')
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(user_id.0)
        .bind(Uuid::new_v4())
        .bind(vec![6_u8; 32])
        .execute(&pool)
        .await
        .is_err(),
        "only the accepted change_entity_trait discriminator spelling is valid"
    );
}

#[sqlx::test(migrations = "./migration")]
async fn trait_schema_enforces_statement_lineage_current_pointer_and_immutability(pool: PgPool) {
    let world = World::new(pool.clone());
    let user_id = create_user(&world).await;
    let first = world
        .create_entity(user_id, entity("First Trait Subject"))
        .await
        .unwrap();
    let second = world
        .create_entity(user_id, entity("Second Trait Subject"))
        .await
        .unwrap();

    for invalid_statement in ["", " padded statement "] {
        let activity_id = insert_trait_action_activity(&pool, user_id).await;
        let trait_id = Uuid::new_v4();
        let mut transaction = pool.begin().await.unwrap();
        sqlx::query("INSERT INTO entity_trait (id, entity_id) VALUES ($1, $2)")
            .bind(trait_id)
            .bind(first.id.0)
            .execute(&mut *transaction)
            .await
            .unwrap();
        assert!(
            sqlx::query(
                r#"
                INSERT INTO entity_trait_version (
                    trait_id, entity_id, activity_id, statement
                ) VALUES ($1, $2, $3, $4)
                "#,
            )
            .bind(trait_id)
            .bind(first.id.0)
            .bind(activity_id)
            .bind(invalid_statement)
            .execute(&mut *transaction)
            .await
            .is_err(),
            "invalid stored statement must fail: {invalid_statement:?}"
        );
        transaction.rollback().await.unwrap();
    }

    let too_long = "x".repeat(4_001);
    let activity_id = insert_trait_action_activity(&pool, user_id).await;
    let trait_id = Uuid::new_v4();
    let mut transaction = pool.begin().await.unwrap();
    sqlx::query("INSERT INTO entity_trait (id, entity_id) VALUES ($1, $2)")
        .bind(trait_id)
        .bind(first.id.0)
        .execute(&mut *transaction)
        .await
        .unwrap();
    assert!(
        sqlx::query(
            r#"
            INSERT INTO entity_trait_version (
                trait_id, entity_id, activity_id, statement
            ) VALUES ($1, $2, $3, $4)
            "#,
        )
        .bind(trait_id)
        .bind(first.id.0)
        .bind(activity_id)
        .bind(too_long)
        .execute(&mut *transaction)
        .await
        .is_err()
    );
    transaction.rollback().await.unwrap();

    let nul_activity_id = insert_trait_action_activity(&pool, user_id).await;
    let nul_trait_id = Uuid::new_v4();
    let mut transaction = pool.begin().await.unwrap();
    sqlx::query("INSERT INTO entity_trait (id, entity_id) VALUES ($1, $2)")
        .bind(nul_trait_id)
        .bind(first.id.0)
        .execute(&mut *transaction)
        .await
        .unwrap();
    assert!(
        sqlx::query(
            r#"
            INSERT INTO entity_trait_version (
                trait_id, entity_id, activity_id, statement
            ) VALUES ($1, $2, $3, $4)
            "#,
        )
        .bind(nul_trait_id)
        .bind(first.id.0)
        .bind(nul_activity_id)
        .bind("contains\0nul")
        .execute(&mut *transaction)
        .await
        .is_err(),
        "PostgreSQL text must reject U+0000"
    );
    transaction.rollback().await.unwrap();

    let establish_activity_id = insert_trait_action_activity(&pool, user_id).await;
    let first_trait_id = Uuid::new_v4();
    let second_trait_id = Uuid::new_v4();
    let mut transaction = pool.begin().await.unwrap();
    sqlx::query(
        r#"
        INSERT INTO entity_trait (id, entity_id)
        VALUES ($1, $2), ($3, $4)
        "#,
    )
    .bind(first_trait_id)
    .bind(first.id.0)
    .bind(second_trait_id)
    .bind(second.id.0)
    .execute(&mut *transaction)
    .await
    .unwrap();
    sqlx::query(
        r#"
        INSERT INTO entity_trait_version (
            trait_id, entity_id, activity_id, statement
        ) VALUES
            ($1, $2, $5, 'x'),
            ($3, $4, $5, $6)
        "#,
    )
    .bind(first_trait_id)
    .bind(first.id.0)
    .bind(second_trait_id)
    .bind(second.id.0)
    .bind(establish_activity_id)
    .bind("z".repeat(4_000))
    .execute(&mut *transaction)
    .await
    .expect("one Activity may establish multiple bounded Trait roots");
    sqlx::query(
        r#"
        INSERT INTO entity_trait_current (trait_id, entity_id, current_activity_id)
        VALUES ($1, $2, $5), ($3, $4, $5)
        "#,
    )
    .bind(first_trait_id)
    .bind(first.id.0)
    .bind(second_trait_id)
    .bind(second.id.0)
    .bind(establish_activity_id)
    .execute(&mut *transaction)
    .await
    .unwrap();
    transaction.commit().await.unwrap();
    let same_activity_count: i64 =
        sqlx::query_scalar("SELECT count(*) FROM entity_trait_version WHERE activity_id = $1")
            .bind(establish_activity_id)
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(same_activity_count, 2);

    let develop_activity_id = insert_trait_action_activity(&pool, user_id).await;
    let mut transaction = pool.begin().await.unwrap();
    sqlx::query(
        r#"
        INSERT INTO entity_trait_version (
            trait_id, entity_id, activity_id, previous_activity_id, statement
        ) VALUES ($1, $2, $3, $4, 'Waits for the second echo.')
        "#,
    )
    .bind(first_trait_id)
    .bind(first.id.0)
    .bind(develop_activity_id)
    .bind(establish_activity_id)
    .execute(&mut *transaction)
    .await
    .unwrap();
    sqlx::query("UPDATE entity_trait_current SET current_activity_id = $1 WHERE trait_id = $2")
        .bind(develop_activity_id)
        .bind(first_trait_id)
        .execute(&mut *transaction)
        .await
        .expect("the current pointer is the only mutable Trait relation");
    transaction.commit().await.unwrap();

    let rejected_activity_id = insert_trait_action_activity(&pool, user_id).await;
    assert!(
        sqlx::query(
            r#"
            INSERT INTO entity_trait_version (
                trait_id, entity_id, activity_id, statement
            ) VALUES ($1, $2, $3, 'A second root is forbidden.')
            "#,
        )
        .bind(first_trait_id)
        .bind(first.id.0)
        .bind(rejected_activity_id)
        .execute(&pool)
        .await
        .is_err(),
        "one Trait may have only one root version"
    );
    assert!(
        sqlx::query(
            r#"
            INSERT INTO entity_trait_version (
                trait_id, entity_id, activity_id, previous_activity_id, statement
            ) VALUES ($1, $2, $3, $4, 'A branch is forbidden.')
            "#,
        )
        .bind(first_trait_id)
        .bind(first.id.0)
        .bind(rejected_activity_id)
        .bind(establish_activity_id)
        .execute(&pool)
        .await
        .is_err(),
        "one predecessor may have only one successor"
    );
    assert!(
        sqlx::query(
            r#"
            INSERT INTO entity_trait_version (
                trait_id, entity_id, activity_id, previous_activity_id, statement
            ) VALUES ($1, $2, $3, $4, 'Cross-lineage predecessor.')
            "#,
        )
        .bind(second_trait_id)
        .bind(second.id.0)
        .bind(rejected_activity_id)
        .bind(develop_activity_id)
        .execute(&pool)
        .await
        .is_err(),
        "a predecessor must belong to the same Trait and Entity lineage"
    );
    assert!(
        sqlx::query(
            r#"
            INSERT INTO entity_trait_version (
                trait_id, entity_id, activity_id, previous_activity_id, statement
            ) VALUES ($1, $2, $3, $3, 'Self predecessor.')
            "#,
        )
        .bind(second_trait_id)
        .bind(second.id.0)
        .bind(rejected_activity_id)
        .execute(&pool)
        .await
        .is_err(),
        "a version cannot name itself as predecessor"
    );
    assert!(
        sqlx::query(
            r#"
            UPDATE entity_trait_current
            SET current_activity_id = $1
            WHERE trait_id = $2
            "#,
        )
        .bind(develop_activity_id)
        .bind(second_trait_id)
        .execute(&pool)
        .await
        .is_err(),
        "a current pointer must remain in the same Trait and Entity lineage"
    );
    assert!(
        sqlx::query("UPDATE entity_trait SET entity_id = $1 WHERE id = $2")
            .bind(second.id.0)
            .bind(first_trait_id)
            .execute(&pool)
            .await
            .is_err(),
        "stable Trait identity and Entity ownership are immutable"
    );
    assert!(
        sqlx::query("DELETE FROM entity_trait WHERE id = $1")
            .bind(first_trait_id)
            .execute(&pool)
            .await
            .is_err()
    );
    assert!(
        sqlx::query("UPDATE entity_trait_version SET statement = 'Changed' WHERE trait_id = $1",)
            .bind(first_trait_id)
            .execute(&pool)
            .await
            .is_err(),
        "accepted statement versions are append-only"
    );
    assert!(
        sqlx::query("DELETE FROM entity_trait_version WHERE trait_id = $1")
            .bind(first_trait_id)
            .execute(&pool)
            .await
            .is_err()
    );

    let incomplete_trait_id = Uuid::new_v4();
    let mut incomplete = pool.begin().await.unwrap();
    sqlx::query("INSERT INTO entity_trait (id, entity_id) VALUES ($1, $2)")
        .bind(incomplete_trait_id)
        .bind(first.id.0)
        .execute(&mut *incomplete)
        .await
        .unwrap();
    assert!(
        incomplete.commit().await.is_err(),
        "a stable Trait identity without a root and current pointer must not commit"
    );

    let root_without_current_activity_id = insert_trait_action_activity(&pool, user_id).await;
    let root_without_current_trait_id = Uuid::new_v4();
    let mut incomplete = pool.begin().await.unwrap();
    sqlx::query("INSERT INTO entity_trait (id, entity_id) VALUES ($1, $2)")
        .bind(root_without_current_trait_id)
        .bind(first.id.0)
        .execute(&mut *incomplete)
        .await
        .unwrap();
    sqlx::query(
        r#"
        INSERT INTO entity_trait_version (
            trait_id, entity_id, activity_id, statement
        ) VALUES ($1, $2, $3, 'A root cannot remain without a current pointer.')
        "#,
    )
    .bind(root_without_current_trait_id)
    .bind(first.id.0)
    .bind(root_without_current_activity_id)
    .execute(&mut *incomplete)
    .await
    .unwrap();
    assert!(
        incomplete.commit().await.is_err(),
        "a Trait root without a current pointer must not commit"
    );

    let successor_without_pointer_activity_id = insert_trait_action_activity(&pool, user_id).await;
    let mut incomplete = pool.begin().await.unwrap();
    sqlx::query(
        r#"
        INSERT INTO entity_trait_version (
            trait_id, entity_id, activity_id, previous_activity_id, statement
        ) VALUES ($1, $2, $3, $4, 'A successor cannot outrun its current pointer.')
        "#,
    )
    .bind(first_trait_id)
    .bind(first.id.0)
    .bind(successor_without_pointer_activity_id)
    .bind(develop_activity_id)
    .execute(&mut *incomplete)
    .await
    .unwrap();
    assert!(
        incomplete.commit().await.is_err(),
        "a new Trait tip without the matching current-pointer advance must not commit"
    );

    let mut incomplete = pool.begin().await.unwrap();
    sqlx::query("DELETE FROM entity_trait_current WHERE trait_id = $1")
        .bind(first_trait_id)
        .execute(&mut *incomplete)
        .await
        .unwrap();
    assert!(
        incomplete.commit().await.is_err(),
        "retirement by deleting the current pointer must not commit"
    );

    let mut incomplete = pool.begin().await.unwrap();
    sqlx::query("UPDATE entity_trait_current SET current_activity_id = $1 WHERE trait_id = $2")
        .bind(establish_activity_id)
        .bind(first_trait_id)
        .execute(&mut *incomplete)
        .await
        .unwrap();
    assert!(
        incomplete.commit().await.is_err(),
        "a current pointer moved backward from the lineage tip must not commit"
    );
}

#[sqlx::test(migrations = "./migration")]
async fn concurrent_trait_successors_have_one_winner_without_a_branch(pool: PgPool) {
    let world = World::new(pool.clone());
    let user_id = create_user(&world).await;
    let entity = world
        .create_entity(user_id, entity("Concurrent Trait Subject"))
        .await
        .unwrap();
    let root_activity_id = insert_trait_action_activity(&pool, user_id).await;
    let first_successor_activity_id = insert_trait_action_activity(&pool, user_id).await;
    let second_successor_activity_id = insert_trait_action_activity(&pool, user_id).await;
    let trait_id = Uuid::new_v4();
    insert_complete_trait_root(
        &pool,
        trait_id,
        entity.id,
        root_activity_id,
        "Startles at every hard sound.",
    )
    .await;

    let mut winner = pool.begin().await.unwrap();
    sqlx::query(
        r#"
        INSERT INTO entity_trait_version (
            trait_id, entity_id, activity_id, previous_activity_id, statement
        ) VALUES ($1, $2, $3, $4, 'Waits for the second echo.')
        "#,
    )
    .bind(trait_id)
    .bind(entity.id.0)
    .bind(first_successor_activity_id)
    .bind(root_activity_id)
    .execute(&mut *winner)
    .await
    .unwrap();
    sqlx::query("UPDATE entity_trait_current SET current_activity_id = $1 WHERE trait_id = $2")
        .bind(first_successor_activity_id)
        .bind(trait_id)
        .execute(&mut *winner)
        .await
        .unwrap();

    let competing_pool = pool.clone();
    let competing = tokio::spawn(async move {
        sqlx::query(
            r#"
            INSERT INTO entity_trait_version (
                trait_id, entity_id, activity_id, previous_activity_id, statement
            ) VALUES ($1, $2, $3, $4, 'Springs at the first echo.')
            "#,
        )
        .bind(trait_id)
        .bind(entity.id.0)
        .bind(second_successor_activity_id)
        .bind(root_activity_id)
        .execute(&competing_pool)
        .await
    });
    wait_for_database_lock_waiter(&pool).await;
    winner.commit().await.unwrap();
    assert!(
        competing.await.unwrap().is_err(),
        "the partial unique successor index must reject a concurrent branch"
    );

    let state: (i64, Uuid) = sqlx::query_as(
        r#"
        SELECT
            (SELECT count(*) FROM entity_trait_version WHERE trait_id = $1),
            current_activity_id
        FROM entity_trait_current
        WHERE trait_id = $1
        "#,
    )
    .bind(trait_id)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(state, (2, first_successor_activity_id));
}

#[sqlx::test(migrations = "./migration")]
async fn trait_bulk_failure_rolls_back_activity_root_version_and_current(pool: PgPool) {
    let world = World::new(pool.clone());
    let user_id = create_user(&world).await;
    let entity = world
        .create_entity(user_id, entity("Trait Rollback Subject"))
        .await
        .unwrap();
    let activity_id = Uuid::new_v4();
    let valid_trait_id = Uuid::new_v4();
    let invalid_trait_id = Uuid::new_v4();
    let mut transaction = pool.begin().await.unwrap();
    sqlx::query(
        r#"
        INSERT INTO activity (
            id, operation, requested_by_user_id, prose,
            request_id, request_fingerprint, action_consequence
        ) VALUES (
            $1, 'submit_action', $2, 'A whole Trait bundle changes.',
            $3, $4, 'change_entity_trait'
        )
        "#,
    )
    .bind(activity_id)
    .bind(user_id.0)
    .bind(Uuid::new_v4())
    .bind(vec![5_u8; 32])
    .execute(&mut *transaction)
    .await
    .unwrap();
    assert!(
        sqlx::query(
            r#"
            INSERT INTO entity_trait (id, entity_id)
            VALUES ($1, $2), ($3, $4)
            "#,
        )
        .bind(valid_trait_id)
        .bind(entity.id.0)
        .bind(invalid_trait_id)
        .bind(Uuid::new_v4())
        .execute(&mut *transaction)
        .await
        .is_err(),
        "one invalid Entity must reject the complete set-based root insert"
    );
    transaction.rollback().await.unwrap();

    let counts: (i64, i64, i64, i64) = sqlx::query_as(
        r#"
        SELECT
            (SELECT count(*) FROM activity WHERE id = $1),
            (SELECT count(*) FROM entity_trait WHERE id = ANY($2::uuid[])),
            (SELECT count(*) FROM entity_trait_version WHERE activity_id = $1),
            (SELECT count(*) FROM entity_trait_current WHERE trait_id = ANY($2::uuid[]))
        "#,
    )
    .bind(activity_id)
    .bind(vec![valid_trait_id, invalid_trait_id])
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(counts, (0, 0, 0, 0));
}

#[sqlx::test(migrations = "./migration")]
async fn trait_indexes_are_exact_and_support_current_predecessor_and_activity_paths(pool: PgPool) {
    let world = World::new(pool.clone());
    let user_id = create_user(&world).await;
    let entity = world
        .create_entity(user_id, entity("Indexed Trait Subject"))
        .await
        .unwrap();
    let activity_id = insert_trait_action_activity(&pool, user_id).await;
    let trait_id = (0..100).map(|_| Uuid::new_v4()).collect::<Vec<_>>();
    let entity_id = vec![entity.id.0; 100];
    let statement = (0..100)
        .map(|index| format!("Indexed statement {index}."))
        .collect::<Vec<_>>();
    let mut transaction = pool.begin().await.unwrap();
    sqlx::query(
        r#"
        INSERT INTO entity_trait (id, entity_id)
        SELECT submitted.trait_id, submitted.entity_id
        FROM UNNEST($1::uuid[], $2::uuid[]) AS submitted(trait_id, entity_id)
        "#,
    )
    .bind(&trait_id)
    .bind(&entity_id)
    .execute(&mut *transaction)
    .await
    .unwrap();
    sqlx::query(
        r#"
        INSERT INTO entity_trait_version (
            trait_id, entity_id, activity_id, statement
        )
        SELECT submitted.trait_id, submitted.entity_id, $3, submitted.statement
        FROM UNNEST($1::uuid[], $2::uuid[], $4::text[])
            AS submitted(trait_id, entity_id, statement)
        "#,
    )
    .bind(&trait_id)
    .bind(&entity_id)
    .bind(activity_id)
    .bind(&statement)
    .execute(&mut *transaction)
    .await
    .unwrap();
    sqlx::query(
        r#"
        INSERT INTO entity_trait_current (trait_id, entity_id, current_activity_id)
        SELECT submitted.trait_id, submitted.entity_id, $3
        FROM UNNEST($1::uuid[], $2::uuid[]) AS submitted(trait_id, entity_id)
        "#,
    )
    .bind(&trait_id)
    .bind(&entity_id)
    .bind(activity_id)
    .execute(&mut *transaction)
    .await
    .unwrap();
    transaction.commit().await.unwrap();

    let sparse_activity_id = insert_trait_action_activity(&pool, user_id).await;
    let sparse_trait_id = Uuid::new_v4();
    insert_complete_trait_root(
        &pool,
        sparse_trait_id,
        entity.id,
        sparse_activity_id,
        "A sparsely selected indexed statement.",
    )
    .await;

    let index: Vec<(String, bool, String)> = sqlx::query_as(
        r#"
        SELECT indexes.indexname, pg_index.indisunique, indexes.indexdef
        FROM pg_indexes AS indexes
        JOIN pg_class ON pg_class.relname = indexes.indexname
        JOIN pg_index ON pg_index.indexrelid = pg_class.oid
        WHERE indexes.schemaname = current_schema()
          AND indexes.tablename IN (
              'entity_trait', 'entity_trait_version', 'entity_trait_current'
          )
        ORDER BY indexes.indexname
        "#,
    )
    .fetch_all(&pool)
    .await
    .unwrap();
    let index_name = index
        .iter()
        .map(|(name, _, _)| name.as_str())
        .collect::<Vec<_>>();
    assert_eq!(
        index_name,
        vec![
            "entity_trait_current_entity_id_trait_id_index",
            "entity_trait_current_pkey",
            "entity_trait_entity_id_id_index",
            "entity_trait_id_entity_id_key",
            "entity_trait_pkey",
            "entity_trait_version_activity_entity_trait_index",
            "entity_trait_version_one_root_index",
            "entity_trait_version_one_successor_index",
            "entity_trait_version_pkey",
            "entity_trait_version_trait_id_entity_id_activity_id_key",
        ]
    );
    let activity_index = index
        .iter()
        .find(|(name, _, _)| name == "entity_trait_version_activity_entity_trait_index")
        .unwrap();
    assert!(!activity_index.1, "one Activity may change many Traits");
    assert!(
        activity_index
            .2
            .contains("(activity_id, entity_id, trait_id)")
    );

    sqlx::query("ANALYZE entity_trait, entity_trait_version, entity_trait_current")
        .execute(&pool)
        .await
        .unwrap();
    let mut transaction = pool.begin().await.unwrap();
    sqlx::query("SET LOCAL enable_seqscan = off")
        .execute(&mut *transaction)
        .await
        .unwrap();
    let entity_plan = sqlx::query_scalar::<_, String>(
        "EXPLAIN (COSTS OFF) SELECT id FROM entity_trait WHERE entity_id = $1 ORDER BY id",
    )
    .bind(entity.id.0)
    .fetch_all(&mut *transaction)
    .await
    .unwrap()
    .join("\n");
    assert!(
        entity_plan.contains("entity_trait_entity_id_id_index"),
        "Entity-owned Trait access must use its declared index: {entity_plan}"
    );
    let current_plan = sqlx::query_scalar::<_, String>(
        "EXPLAIN (COSTS OFF) SELECT trait_id FROM entity_trait_current WHERE entity_id = $1 ORDER BY trait_id",
    )
    .bind(entity.id.0)
    .fetch_all(&mut *transaction)
    .await
    .unwrap()
    .join("\n");
    assert!(
        current_plan.contains("entity_trait_current_entity_id_trait_id_index"),
        "current Entity Trait access must use its declared index: {current_plan}"
    );
    let exact_plan = sqlx::query_scalar::<_, String>(
        "EXPLAIN (COSTS OFF) SELECT entity_id FROM entity_trait WHERE id = $1",
    )
    .bind(trait_id[0])
    .fetch_all(&mut *transaction)
    .await
    .unwrap()
    .join("\n");
    assert!(
        exact_plan.contains("entity_trait_pkey")
            || exact_plan.contains("entity_trait_id_entity_id_key")
            || exact_plan.contains("entity_trait_entity_id_id_index"),
        "stable Trait lookup must use an identity index: {exact_plan}"
    );
    let predecessor_plan = sqlx::query_scalar::<_, String>(
        "EXPLAIN (COSTS OFF) SELECT statement FROM entity_trait_version WHERE trait_id = $1 AND activity_id = $2",
    )
    .bind(trait_id[0])
    .bind(activity_id)
    .fetch_all(&mut *transaction)
    .await
    .unwrap()
    .join("\n");
    assert!(
        predecessor_plan.contains("entity_trait_version_pkey")
            || predecessor_plan.contains("entity_trait_version_trait_id_entity_id_activity_id_key")
            || predecessor_plan.contains("entity_trait_version_activity_entity_trait_index"),
        "predecessor validation must use a lineage identity index: {predecessor_plan}"
    );
    let hydration_plan = sqlx::query_scalar::<_, String>(
        "EXPLAIN (COSTS OFF) SELECT trait_id FROM entity_trait_version WHERE activity_id = ANY($1::uuid[]) ORDER BY activity_id, entity_id, trait_id",
    )
    .bind(vec![sparse_activity_id])
    .fetch_all(&mut *transaction)
    .await
    .unwrap()
    .join("\n");
    assert!(
        hydration_plan.contains("entity_trait_version_activity_entity_trait_index"),
        "Activity hydration must use its declared nonunique index: {hydration_plan}"
    );
    transaction.rollback().await.unwrap();
}

#[sqlx::test(migrations = "./migration")]
async fn trait_action_uniformly_establishes_develops_reads_and_reconstructs_retry(pool: PgPool) {
    let world = World::new(pool.clone());
    let (place, participant) = entered_characters(&world, &["Mara", "Pip"]).await;
    let (mara_user, mara) = participant[0];
    let (_, pip) = participant[1];
    let revision = world
        .list_entity_at_current_place(mara_user, ListEntityAtCurrentPlace::default())
        .await
        .unwrap()
        .place_revision;
    let bowl = world
        .submit_action(mara_user, action(Uuid::new_v4(), revision, "Copper Bowl"))
        .await
        .unwrap();
    let bowl = introduced_entity(&bowl).id;
    let revision = world
        .list_entity_at_current_place(mara_user, ListEntityAtCurrentPlace::default())
        .await
        .unwrap()
        .place_revision;
    let establishment_revision = revision;
    let request_id = Uuid::new_v4();
    let establishment = vec![
        establish_trait(pip, "Waits for a second echo."),
        establish_trait(
            place.entity.id,
            "Holds every departing footstep for a breath.",
        ),
        establish_trait(mara, "Jumps unusually high."),
        establish_trait(bowl, "Rings only after the hand withdraws."),
    ];
    let accepted = world
        .submit_action(
            mara_user,
            trait_action(
                request_id,
                establishment_revision,
                "Mara notices four lasting characterizations.",
                establishment.clone(),
            ),
        )
        .await
        .unwrap();
    let established = accepted_trait_change(&accepted);
    assert_eq!(established, accepted.activity.trait_change);
    assert_eq!(established.len(), 4);
    assert!(
        established
            .iter()
            .all(|change| matches!(change, ActivityTraitChange::Establish { .. }))
    );
    let subject = accepted
        .activity
        .involved_entity
        .iter()
        .filter(|reference| reference.role == ActivityEntityRole::Subject)
        .map(|reference| reference.entity.id)
        .collect::<std::collections::HashSet<_>>();
    assert_eq!(
        subject,
        [mara, pip, bowl, place.entity.id].into_iter().collect()
    );

    let trait_by_entity = established
        .iter()
        .map(|change| match change {
            ActivityTraitChange::Establish { entity, r#trait } => (entity.id, r#trait.id),
            ActivityTraitChange::Develop { .. } => unreachable!(),
        })
        .collect::<std::collections::HashMap<_, _>>();
    let revision = world
        .list_entity_at_current_place(mara_user, ListEntityAtCurrentPlace::default())
        .await
        .unwrap()
        .place_revision;
    let developed = world
        .submit_action(
            mara_user,
            trait_action(
                Uuid::new_v4(),
                revision,
                "Each earlier characterization develops without changing identity.",
                vec![
                    develop_trait(trait_by_entity[&bowl], "Rings before the hand withdraws."),
                    develop_trait(
                        trait_by_entity[&mara],
                        "Lands quietly after impossible jumps.",
                    ),
                    establish_trait(mara, "Refuses every invitation to jump."),
                    develop_trait(
                        trait_by_entity[&place.entity.id],
                        "Releases departing footsteps into the dawn.",
                    ),
                    develop_trait(trait_by_entity[&pip], "Moves on the second echo."),
                ],
            ),
        )
        .await
        .unwrap();
    assert_eq!(accepted_trait_change(&developed).len(), 5);
    for change in accepted_trait_change(&developed) {
        if let ActivityTraitChange::Develop {
            r#trait,
            previous_statement,
            ..
        } = change
        {
            assert_eq!(
                trait_by_entity
                    .values()
                    .filter(|id| **id == r#trait.id)
                    .count(),
                1
            );
            assert!(!previous_statement.is_empty());
        }
    }

    for entity_id in [mara, pip, bowl, place.entity.id] {
        let page = world
            .get_entity_at_current_place(
                mara_user,
                GetEntityAtCurrentPlace {
                    entity_id,
                    cursor: None,
                    limit: 100,
                },
            )
            .await
            .unwrap();
        assert!(
            page.current_state
                .association
                .iter()
                .any(|association| matches!(
                    association,
                    EntityCurrentAssociation::Trait(r#trait)
                        if r#trait.id == trait_by_entity[&entity_id]
                ))
        );
    }

    let mut reordered = establishment;
    reordered.reverse();
    let retry = world
        .submit_action(
            mara_user,
            trait_action(
                request_id,
                establishment_revision,
                "Mara notices four lasting characterizations.",
                reordered,
            ),
        )
        .await
        .unwrap();
    assert_eq!(retry, accepted);
    assert_eq!(
        world
            .submit_action(
                mara_user,
                trait_action(
                    request_id,
                    establishment_revision,
                    "Mara notices four lasting characterizations.",
                    vec![establish_trait(mara, "Changed retry content.")],
                ),
            )
            .await,
        Err(WorldError::ActionRequestConflict)
    );
    let lineage_count: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM entity_trait_version WHERE trait_id = ANY($1::uuid[])",
    )
    .bind(
        trait_by_entity
            .values()
            .map(|trait_id| trait_id.0)
            .collect::<Vec<_>>(),
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(lineage_count, 8);
}

#[sqlx::test(migrations = "./migration")]
async fn interaction_trait_and_property_changes_are_atomic_target_scoped_and_retryable(
    pool: PgPool,
) {
    let world = World::new(pool.clone());
    let (_, participant) = entered_characters(&world, &["Pip", "Mara", "Tomas"]).await;
    let (pip_user, pip) = participant[0];
    let (mara_user, mara) = participant[1];
    let (tomas_user, tomas) = participant[2];
    let original_revision = world
        .list_entity_at_current_place(pip_user, ListEntityAtCurrentPlace::default())
        .await
        .unwrap()
        .place_revision;
    let request_id = Uuid::new_v4();
    let accepted = world
        .submit_interaction(
            pip_user,
            SubmitInteraction {
                request_id,
                expected_place_revision: original_revision,
                prose: "Pip and Mara leave matching marks after the exchange.".to_owned(),
                target_entity_id: vec![mara],
                property_change: vec![
                    property_change(pip, "mark", PropertyValue::Text("silver".into())),
                    property_change(mara, "mark", PropertyValue::Text("silver".into())),
                ],
                trait_change: vec![
                    establish_trait(mara, "Answers only after Pip lowers a hand."),
                    establish_trait(pip, "Lowers a hand before asking twice."),
                ],
            },
        )
        .await
        .unwrap();
    assert_eq!(accepted.activity.property_change.len(), 2);
    assert_eq!(accepted.activity.trait_change.len(), 2);
    assert_eq!(accepted.activity.actor_character.as_ref().unwrap().id, pip);
    assert!(accepted.activity.involved_entity.iter().any(|reference| {
        reference.entity.id == mara && reference.role == ActivityEntityRole::Target
    }));
    assert!(
        !accepted
            .activity
            .involved_entity
            .iter()
            .any(|reference| reference.role == ActivityEntityRole::Subject)
    );
    let mara_history = world
        .list_activity(
            mara_user,
            ListActivity {
                cursor: None,
                limit: 100,
            },
        )
        .await
        .unwrap();
    let historical = mara_history
        .activity
        .iter()
        .find(|activity| activity.id == accepted.activity.id)
        .expect("an explicit target may read the Interaction Activity");
    assert_eq!(historical.trait_change, accepted.activity.trait_change);
    assert!(
        !world
            .list_activity(
                tomas_user,
                ListActivity {
                    cursor: None,
                    limit: 100,
                },
            )
            .await
            .unwrap()
            .activity
            .iter()
            .any(|activity| activity.id == accepted.activity.id)
    );

    let trait_by_entity = accepted
        .activity
        .trait_change
        .iter()
        .map(|change| match change {
            ActivityTraitChange::Establish { entity, r#trait } => (entity.id, r#trait.id),
            ActivityTraitChange::Develop { .. } => unreachable!(),
        })
        .collect::<std::collections::HashMap<_, _>>();
    let revision = world
        .list_entity_at_current_place(pip_user, ListEntityAtCurrentPlace::default())
        .await
        .unwrap()
        .place_revision;
    let developed = world
        .submit_interaction(
            pip_user,
            SubmitInteraction {
                request_id: Uuid::new_v4(),
                expected_place_revision: revision,
                prose: "The exchange changes both familiar characterizations.".to_owned(),
                target_entity_id: vec![mara],
                property_change: Vec::new(),
                trait_change: vec![
                    develop_trait(trait_by_entity[&mara], "Answers before Pip lowers a hand."),
                    develop_trait(trait_by_entity[&pip], "Asks once before lowering a hand."),
                ],
            },
        )
        .await
        .unwrap();
    assert!(
        developed
            .activity
            .trait_change
            .iter()
            .all(|change| matches!(change, ActivityTraitChange::Develop { .. }))
    );

    let retry = world
        .submit_interaction(
            pip_user,
            SubmitInteraction {
                request_id,
                expected_place_revision: original_revision,
                prose: "Pip and Mara leave matching marks after the exchange.".to_owned(),
                target_entity_id: vec![mara],
                property_change: vec![
                    property_change(mara, "mark", PropertyValue::Text("silver".into())),
                    property_change(pip, "mark", PropertyValue::Text("silver".into())),
                ],
                trait_change: vec![
                    establish_trait(pip, "Lowers a hand before asking twice."),
                    establish_trait(mara, "Answers only after Pip lowers a hand."),
                ],
            },
        )
        .await
        .unwrap();
    assert_eq!(retry, accepted);

    let current_revision = world
        .list_entity_at_current_place(pip_user, ListEntityAtCurrentPlace::default())
        .await
        .unwrap()
        .place_revision;
    let before_activity: i64 =
        sqlx::query_scalar("SELECT count(*) FROM activity WHERE operation = 'submit_interaction'")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(
        world
            .submit_interaction(
                pip_user,
                SubmitInteraction {
                    request_id: Uuid::new_v4(),
                    expected_place_revision: current_revision,
                    prose: "Tomas was not part of this exchange.".to_owned(),
                    target_entity_id: vec![mara],
                    property_change: vec![property_change(
                        pip,
                        "rollback_marker",
                        PropertyValue::Text("must not persist".into()),
                    )],
                    trait_change: vec![establish_trait(
                        tomas,
                        "Responds despite not being a target.",
                    )],
                },
            )
            .await,
        Err(WorldError::TraitUnavailable)
    );
    let after: (i64, i64) = sqlx::query_as(
        r#"
        SELECT
            (SELECT count(*) FROM activity WHERE operation = 'submit_interaction'),
            (SELECT count(*)
             FROM entity_property
             JOIN property_key ON property_key.id = entity_property.property_key_id
             WHERE entity_property.entity_id = $1 AND property_key.key = 'rollback_marker')
        "#,
    )
    .bind(pip.0)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(after, (before_activity, 0));
}

#[sqlx::test(migrations = "./migration")]
async fn action_and_interaction_reject_every_duplicate_intended_active_trait_shape_atomically(
    pool: PgPool,
) {
    let world = World::new(pool.clone());
    let (_, participant) = entered_characters(&world, &["Pip", "Mara"]).await;
    let (pip_user, pip) = participant[0];
    let (_, mara) = participant[1];
    let revision = world
        .list_entity_at_current_place(pip_user, ListEntityAtCurrentPlace::default())
        .await
        .unwrap()
        .place_revision;
    let established = world
        .submit_action(
            pip_user,
            trait_action(
                Uuid::new_v4(),
                revision,
                "Pip establishes two distinct active characterizations.",
                vec![
                    establish_trait(pip, "First distinct active statement."),
                    establish_trait(pip, "Second distinct active statement."),
                ],
            ),
        )
        .await
        .unwrap();
    let trait_by_statement = accepted_trait_change(&established)
        .iter()
        .map(|change| match change {
            ActivityTraitChange::Establish { r#trait, .. } => {
                (r#trait.statement.as_str(), r#trait.id)
            }
            ActivityTraitChange::Develop { .. } => unreachable!(),
        })
        .collect::<std::collections::HashMap<_, _>>();
    let first_trait_id = trait_by_statement["First distinct active statement."];
    let second_trait_id = trait_by_statement["Second distinct active statement."];
    let revision = world
        .list_entity_at_current_place(pip_user, ListEntityAtCurrentPlace::default())
        .await
        .unwrap()
        .place_revision;
    let before: (i64, i64, i64, i64, i64, i64) = sqlx::query_as(
        r#"
        SELECT
            (SELECT count(*) FROM activity),
            (SELECT count(*) FROM entity_trait),
            (SELECT count(*) FROM entity_trait_version),
            (SELECT count(*) FROM entity_trait_current),
            (SELECT count(*) FROM property_key WHERE key = 'rollback_marker'),
            (SELECT count(*) FROM entity_property_history)
        "#,
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    let duplicate_shapes = || {
        vec![
            vec![develop_trait(
                first_trait_id,
                "Second distinct active statement.",
            )],
            vec![
                develop_trait(first_trait_id, "Shared intended active statement."),
                develop_trait(second_trait_id, "Shared intended active statement."),
            ],
            vec![
                establish_trait(pip, "Establish/develop shared intended statement."),
                develop_trait(
                    first_trait_id,
                    "Establish/develop shared intended statement.",
                ),
            ],
        ]
    };

    for trait_change in duplicate_shapes() {
        assert_eq!(
            world
                .submit_action(
                    pip_user,
                    trait_action(
                        Uuid::new_v4(),
                        revision,
                        "This duplicate intended active state must roll back.",
                        trait_change,
                    ),
                )
                .await,
            Err(WorldError::InvalidTrait)
        );
    }

    for trait_change in duplicate_shapes() {
        assert_eq!(
            world
                .submit_interaction(
                    pip_user,
                    SubmitInteraction {
                        request_id: Uuid::new_v4(),
                        expected_place_revision: revision,
                        prose: "This duplicate Interaction package must roll back.".to_owned(),
                        target_entity_id: vec![mara],
                        property_change: vec![property_change(
                            pip,
                            "rollback_marker",
                            PropertyValue::Text("must not persist".to_owned()),
                        )],
                        trait_change,
                    },
                )
                .await,
            Err(WorldError::InvalidTrait)
        );
    }

    let after: (i64, i64, i64, i64, i64, i64) = sqlx::query_as(
        r#"
        SELECT
            (SELECT count(*) FROM activity),
            (SELECT count(*) FROM entity_trait),
            (SELECT count(*) FROM entity_trait_version),
            (SELECT count(*) FROM entity_trait_current),
            (SELECT count(*) FROM property_key WHERE key = 'rollback_marker'),
            (SELECT count(*) FROM entity_property_history)
        "#,
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(after, before);
    let current: Vec<(Uuid, String)> = sqlx::query_as(
        r#"
        SELECT current.trait_id, version.statement
        FROM entity_trait_current AS current
        JOIN entity_trait_version AS version
          ON version.trait_id = current.trait_id
         AND version.entity_id = current.entity_id
         AND version.activity_id = current.current_activity_id
        WHERE current.trait_id = ANY($1::uuid[])
        ORDER BY current.trait_id
        "#,
    )
    .bind(vec![first_trait_id.0, second_trait_id.0])
    .fetch_all(&pool)
    .await
    .unwrap();
    assert_eq!(current.len(), 2);
    assert_eq!(
        current
            .iter()
            .map(|(_, statement)| statement.as_str())
            .collect::<std::collections::HashSet<_>>(),
        [
            "First distinct active statement.",
            "Second distinct active statement."
        ]
        .into_iter()
        .collect()
    );
}

#[sqlx::test(migrations = "./migration")]
async fn action_trait_package_reuses_a_statement_vacated_by_another_lineage(pool: PgPool) {
    let world = World::new(pool.clone());
    let (_, participant) = entered_characters(&world, &["Pip"]).await;
    let (pip_user, pip) = participant[0];
    let revision = world
        .list_entity_at_current_place(pip_user, ListEntityAtCurrentPlace::default())
        .await
        .unwrap()
        .place_revision;
    let established = world
        .submit_action(
            pip_user,
            trait_action(
                Uuid::new_v4(),
                revision,
                "Pip reveals two distinct characterizations.",
                vec![
                    establish_trait(pip, "Waits until the third knock."),
                    establish_trait(pip, "Answers before the first echo."),
                ],
            ),
        )
        .await
        .unwrap();
    let trait_by_statement = accepted_trait_change(&established)
        .iter()
        .map(|change| match change {
            ActivityTraitChange::Establish { r#trait, .. } => {
                (r#trait.statement.as_str(), r#trait.id)
            }
            ActivityTraitChange::Develop { .. } => unreachable!(),
        })
        .collect::<std::collections::HashMap<_, _>>();
    let vacating_trait_id = trait_by_statement["Waits until the third knock."];
    let reusing_trait_id = trait_by_statement["Answers before the first echo."];
    let revision = world
        .list_entity_at_current_place(pip_user, ListEntityAtCurrentPlace::default())
        .await
        .unwrap()
        .place_revision;
    let request_id = Uuid::new_v4();
    let obstructive_order = vec![
        develop_trait(reusing_trait_id, "Waits until the third knock."),
        develop_trait(vacating_trait_id, "Leaves before the third knock."),
    ];
    let accepted = world
        .submit_action(
            pip_user,
            trait_action(
                request_id,
                revision,
                "One characterization moves on as another inherits its exact wording.",
                obstructive_order.clone(),
            ),
        )
        .await
        .expect("post-package uniqueness must allow reuse after a same-package vacancy");
    assert_eq!(
        accepted_trait_change(&accepted),
        accepted.activity.trait_change
    );
    let developed_by_id = accepted_trait_change(&accepted)
        .iter()
        .map(|change| match change {
            ActivityTraitChange::Develop {
                entity,
                r#trait,
                previous_statement,
            } => {
                assert_eq!(entity.id, pip);
                (
                    r#trait.id,
                    (previous_statement.as_str(), r#trait.statement.as_str()),
                )
            }
            ActivityTraitChange::Establish { .. } => unreachable!(),
        })
        .collect::<std::collections::HashMap<_, _>>();
    assert_eq!(developed_by_id.len(), 2);
    assert_eq!(
        developed_by_id[&vacating_trait_id],
        (
            "Waits until the third knock.",
            "Leaves before the third knock."
        )
    );
    assert_eq!(
        developed_by_id[&reusing_trait_id],
        (
            "Answers before the first echo.",
            "Waits until the third knock."
        )
    );

    let current = world
        .get_character(
            pip_user,
            GetEntityCurrentState {
                cursor: None,
                limit: 100,
            },
        )
        .await
        .unwrap();
    let current_by_id = current
        .current_state
        .association
        .iter()
        .filter_map(|association| match association {
            EntityCurrentAssociation::Trait(r#trait) => {
                Some((r#trait.id, r#trait.statement.as_str()))
            }
            EntityCurrentAssociation::Property { .. } => None,
        })
        .collect::<std::collections::HashMap<_, _>>();
    assert_eq!(
        current_by_id[&vacating_trait_id],
        "Leaves before the third knock."
    );
    assert_eq!(
        current_by_id[&reusing_trait_id],
        "Waits until the third knock."
    );

    let predecessor_by_id = sqlx::query_as::<_, (Uuid, Uuid, String)>(
        r#"
        SELECT trait_id, previous_activity_id, statement
        FROM entity_trait_version
        WHERE activity_id = $1
        ORDER BY trait_id
        "#,
    )
    .bind(accepted.activity.id.0)
    .fetch_all(&pool)
    .await
    .unwrap()
    .into_iter()
    .map(|(trait_id, previous_activity_id, statement)| {
        (trait_id, (previous_activity_id, statement))
    })
    .collect::<std::collections::HashMap<_, _>>();
    assert_eq!(predecessor_by_id.len(), 2);
    for trait_id in [vacating_trait_id, reusing_trait_id] {
        assert_eq!(
            predecessor_by_id[&trait_id.0].0, established.activity.id.0,
            "each developed version must point to its own established predecessor Activity"
        );
    }

    let historical = world
        .list_activity(
            pip_user,
            ListActivity {
                cursor: None,
                limit: 100,
            },
        )
        .await
        .unwrap()
        .activity
        .into_iter()
        .find(|activity| activity.id == accepted.activity.id)
        .expect("the accepted Trait development must remain readable as Activity history");
    assert_eq!(historical.trait_change, accepted.activity.trait_change);

    let mut retry_order = obstructive_order;
    retry_order.reverse();
    let retry = world
        .submit_action(
            pip_user,
            trait_action(
                request_id,
                revision,
                "One characterization moves on as another inherits its exact wording.",
                retry_order,
            ),
        )
        .await
        .unwrap();
    assert_eq!(retry, accepted);
}

#[sqlx::test(migrations = "./migration")]
async fn trait_validation_combined_cursor_unavailability_and_concurrency_are_closed(pool: PgPool) {
    let world = World::new(pool.clone());
    let user_id = create_user(&world).await;
    let character = world
        .create_character(
            user_id,
            CreateCharacter {
                name: "Mara".to_owned(),
                description: "A bounded Trait subject.".to_owned(),
                property: vec![
                    text_property("colour", "amber"),
                    integer_property("leg_count", 2),
                ],
            },
        )
        .await
        .unwrap();
    let unplaced = world
        .get_character(
            user_id,
            GetEntityCurrentState {
                cursor: None,
                limit: 1,
            },
        )
        .await
        .unwrap();
    assert_eq!(unplaced.place_revision, None);
    assert_eq!(unplaced.current_state.association.len(), 1);
    let unplaced_cursor = unplaced.current_state.next;
    world
        .create_entry_place(user_id, place("North Gate"))
        .await
        .unwrap();
    world.enter_world(user_id).await.unwrap();
    assert_eq!(
        world
            .get_character(
                user_id,
                GetEntityCurrentState {
                    cursor: unplaced_cursor,
                    limit: 1,
                },
            )
            .await,
        Err(WorldError::PlaceRevisionConflict)
    );

    let revision = world
        .list_entity_at_current_place(user_id, ListEntityAtCurrentPlace::default())
        .await
        .unwrap()
        .place_revision;
    let established = world
        .submit_action(
            user_id,
            trait_action(
                Uuid::new_v4(),
                revision,
                "Mara's established characterization becomes World state.",
                vec![establish_trait(
                    character.entity.id,
                    "Waits for a second echo.",
                )],
            ),
        )
        .await
        .unwrap();
    let trait_id = match &accepted_trait_change(&established)[0] {
        ActivityTraitChange::Establish { r#trait, .. } => r#trait.id,
        ActivityTraitChange::Develop { .. } => unreachable!(),
    };
    let first = world
        .get_character(
            user_id,
            GetEntityCurrentState {
                cursor: None,
                limit: 2,
            },
        )
        .await
        .unwrap();
    assert!(
        first
            .current_state
            .association
            .iter()
            .all(|association| matches!(association, EntityCurrentAssociation::Property { .. }))
    );
    let stale_cursor = first.current_state.next;
    let revision = first.place_revision.unwrap();
    world
        .submit_action(
            user_id,
            trait_action(
                Uuid::new_v4(),
                revision,
                "A second valid but contradictory characterization is accepted.",
                vec![establish_trait(
                    character.entity.id,
                    "Never waits for an echo.",
                )],
            ),
        )
        .await
        .unwrap();
    assert_eq!(
        world
            .get_character(
                user_id,
                GetEntityCurrentState {
                    cursor: stale_cursor,
                    limit: 2,
                },
            )
            .await,
        Err(WorldError::PlaceRevisionConflict)
    );
    let fresh = world
        .get_character(
            user_id,
            GetEntityCurrentState {
                cursor: None,
                limit: 100,
            },
        )
        .await
        .unwrap();
    assert_eq!(fresh.current_state.association.len(), 4);
    assert!(matches!(
        fresh.current_state.association[0],
        EntityCurrentAssociation::Property { .. }
    ));
    assert!(matches!(
        fresh.current_state.association[1],
        EntityCurrentAssociation::Property { .. }
    ));
    assert!(
        fresh.current_state.association[2..]
            .iter()
            .all(|association| matches!(association, EntityCurrentAssociation::Trait(_)))
    );

    let revision = fresh.place_revision.unwrap();
    for invalid in [
        Vec::new(),
        vec![
            establish_trait(character.entity.id, " Duplicate statement. "),
            establish_trait(character.entity.id, "Duplicate statement."),
        ],
        vec![develop_trait(trait_id, "Waits for a second echo.")],
        vec![
            develop_trait(trait_id, "First proposed successor."),
            develop_trait(trait_id, "Second proposed successor."),
        ],
        vec![establish_trait(
            character.entity.id,
            "Waits for a second echo.",
        )],
        (0..101)
            .map(|index| establish_trait(character.entity.id, format!("Bound {index}.")))
            .collect(),
    ] {
        assert_eq!(
            world
                .submit_action(
                    user_id,
                    trait_action(
                        Uuid::new_v4(),
                        revision,
                        "This invalid Trait package writes nothing.",
                        invalid,
                    ),
                )
                .await,
            Err(WorldError::InvalidTrait)
        );
    }
    assert_eq!(
        world
            .submit_action(
                user_id,
                trait_action(
                    Uuid::new_v4(),
                    revision,
                    "A missing Trait remains neutrally unavailable.",
                    vec![develop_trait(
                        EntityTraitId(Uuid::new_v4()),
                        "Cannot develop an unavailable lineage.",
                    )],
                ),
            )
            .await,
        Err(WorldError::TraitUnavailable)
    );

    let remote = world
        .create_entity(user_id, entity("Unplaced Remote Trait Subject"))
        .await
        .unwrap();
    let revision = world
        .list_entity_at_current_place(user_id, ListEntityAtCurrentPlace::default())
        .await
        .unwrap()
        .place_revision;
    assert_eq!(
        world
            .submit_action(
                user_id,
                trait_action(
                    Uuid::new_v4(),
                    revision,
                    "A remote Entity cannot receive a contextual Trait.",
                    vec![establish_trait(remote.id, "Cannot be reached here.")],
                ),
            )
            .await,
        Err(WorldError::TraitUnavailable)
    );

    let barrier = std::sync::Arc::new(tokio::sync::Barrier::new(3));
    let first = {
        let world = world.clone();
        let barrier = barrier.clone();
        tokio::spawn(async move {
            barrier.wait().await;
            world
                .submit_action(
                    user_id,
                    trait_action(
                        Uuid::new_v4(),
                        revision,
                        "The first concurrent successor is proposed.",
                        vec![develop_trait(trait_id, "Moves on the second echo.")],
                    ),
                )
                .await
        })
    };
    let second = {
        let world = world.clone();
        let barrier = barrier.clone();
        tokio::spawn(async move {
            barrier.wait().await;
            world
                .submit_action(
                    user_id,
                    trait_action(
                        Uuid::new_v4(),
                        revision,
                        "The second concurrent successor is proposed.",
                        vec![develop_trait(trait_id, "Moves before the second echo.")],
                    ),
                )
                .await
        })
    };
    barrier.wait().await;
    let result = [first.await.unwrap(), second.await.unwrap()];
    assert_eq!(result.iter().filter(|result| result.is_ok()).count(), 1);
    assert_eq!(
        result
            .iter()
            .filter(|result| **result == Err(WorldError::PlaceRevisionConflict))
            .count(),
        1
    );
    let version_count: i64 =
        sqlx::query_scalar("SELECT count(*) FROM entity_trait_version WHERE trait_id = $1")
            .bind(trait_id.0)
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(version_count, 2);
}

#[sqlx::test(migrations = "./migration")]
async fn every_creation_route_remains_trait_free(pool: PgPool) {
    let world = World::new(pool.clone());
    let user_id = create_user(&world).await;
    world
        .create_character(user_id, character("Trait-free Character"))
        .await
        .unwrap();
    world
        .create_entry_place(user_id, place("Trait-free Place"))
        .await
        .unwrap();
    world
        .create_entity(user_id, entity("Trait-free ordinary Entity"))
        .await
        .unwrap();
    world.enter_world(user_id).await.unwrap();
    let revision = world
        .list_entity_at_current_place(user_id, ListEntityAtCurrentPlace::default())
        .await
        .unwrap()
        .place_revision;
    world
        .submit_action(
            user_id,
            action(Uuid::new_v4(), revision, "Trait-free introduced Entity"),
        )
        .await
        .unwrap();
    let count: (i64, i64, i64) = sqlx::query_as(
        "SELECT (SELECT count(*) FROM entity_trait), (SELECT count(*) FROM entity_trait_version), (SELECT count(*) FROM entity_trait_current)",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(count, (0, 0, 0));
}

#[sqlx::test(migrations = "./migration")]
async fn trait_world_storage_failure_rolls_back_activity_lineage_pointer_and_revision(
    pool: PgPool,
) {
    let world = World::new(pool.clone());
    let (place, participant) = entered_characters(&world, &["Mara"]).await;
    let (user_id, character_id) = participant[0];
    let revision = world
        .list_entity_at_current_place(user_id, ListEntityAtCurrentPlace::default())
        .await
        .unwrap()
        .place_revision;
    sqlx::raw_sql(
        r#"
        CREATE FUNCTION fail_trait_current_write() RETURNS trigger
        LANGUAGE plpgsql AS $$
        BEGIN
            RAISE EXCEPTION 'forced Trait pointer failure';
        END;
        $$;
        CREATE TRIGGER fail_trait_current_write
            BEFORE INSERT OR UPDATE ON entity_trait_current
            FOR EACH STATEMENT EXECUTE FUNCTION fail_trait_current_write();
        "#,
    )
    .execute(&pool)
    .await
    .unwrap();
    assert_eq!(
        world
            .submit_action(
                user_id,
                trait_action(
                    Uuid::new_v4(),
                    revision,
                    "This accepted package must roll back on storage failure.",
                    vec![establish_trait(
                        character_id,
                        "Must leave no partial lineage."
                    )],
                ),
            )
            .await,
        Err(WorldError::Unavailable)
    );
    let count: (i64, i64, i64, i64) = sqlx::query_as(
        r#"
        SELECT
            (SELECT count(*) FROM activity WHERE action_consequence = 'change_entity_trait'),
            (SELECT count(*) FROM entity_trait),
            (SELECT count(*) FROM entity_trait_version),
            (SELECT count(*) FROM entity_trait_current)
        "#,
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(count, (0, 0, 0, 0));
    let after = world
        .list_entity_at_current_place(user_id, ListEntityAtCurrentPlace::default())
        .await
        .unwrap();
    assert_eq!(after.place.entity.id, place.entity.id);
    assert_eq!(after.place_revision, revision);
}
