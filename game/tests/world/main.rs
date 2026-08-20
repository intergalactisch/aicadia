mod action;
mod activity;
mod entity;
mod entity_state_creation;
mod interaction;
mod investigation_schema;
mod property_behavior;
mod property_schema;
mod spatial_storage;
mod trait_behavior;
mod trait_storage;

use aicadia::{
    AcceptedActionConsequence, ActionConsequence, ActionField, ActivityEntityRole,
    ActivityOperation, ActivityTraitChange, ChangeEntityState, CreateCharacter, CreateEntity,
    CreateEntryPlace, EntityCurrentAssociation, EntityField, EntityId, EntityPropertyChangeInput,
    EntityTraitChangeInput, EntityTraitId, GetEntityAtCurrentPlace, GetEntityCurrentState,
    InteractionField, IntroduceEntity, InvalidReason, ListActivity, ListActivityAtCurrentPlace,
    ListEntityAtCurrentPlace, PlaceRevision, PropertyField, PropertyInput, PropertyValue,
    SubmitAction, SubmitInteraction, TraitInput, UserId, World, WorldError,
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
        r#trait: Vec::new(),
    }
}

fn character(name: &str) -> CreateCharacter {
    CreateCharacter {
        name: name.to_owned(),
        description: format!("Description of {name}"),
        property: Vec::new(),
        r#trait: Vec::new(),
    }
}

fn place(name: &str) -> CreateEntryPlace {
    CreateEntryPlace {
        name: name.to_owned(),
        description: format!("Description of {name}"),
        property: Vec::new(),
        r#trait: Vec::new(),
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
            position_description: None,
            property: Vec::new(),
            r#trait: Vec::new(),
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
        consequence: ActionConsequence::ChangeEntityState(ChangeEntityState {
            property_change,
            trait_change: Vec::new(),
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
        consequence: ActionConsequence::ChangeEntityState(ChangeEntityState {
            property_change: Vec::new(),
            trait_change,
        }),
    }
}

fn accepted_trait_change(accepted: &aicadia::AcceptedAction) -> &[ActivityTraitChange] {
    match &accepted.consequence {
        AcceptedActionConsequence::ChangeEntityState { trait_change, .. } => trait_change,
        AcceptedActionConsequence::IntroduceEntity(_) => panic!("expected Trait changes"),
    }
}

fn introduced_entity(accepted: &aicadia::AcceptedAction) -> &aicadia::Entity {
    match &accepted.consequence {
        AcceptedActionConsequence::IntroduceEntity(entity) => entity,
        AcceptedActionConsequence::ChangeEntityState { .. } => {
            panic!("expected an introduced Entity")
        }
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
            $3, $4, 'change_entity_state'
        )
        "#,
    )
    .bind(activity_id)
    .bind(user_id.0)
    .bind(Uuid::new_v4())
    .bind(vec![8_u8; 32])
    .execute(pool)
    .await
    .expect("a raw state Action Activity should be valid");
    activity_id
}

async fn insert_historical_trait_action_activity(pool: &PgPool, user_id: UserId) -> Uuid {
    let activity_id = Uuid::new_v4();
    sqlx::query(
        r#"
        INSERT INTO activity (
            id, operation, requested_by_user_id, prose,
            request_id, request_fingerprint, action_consequence
        )
        VALUES (
            $1, 'submit_action', $2, 'A historical Trait changes.',
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
    .expect("a historical Trait Action Activity should be valid before migration 0009");
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
