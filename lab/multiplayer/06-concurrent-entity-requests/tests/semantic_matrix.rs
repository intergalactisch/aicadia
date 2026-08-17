use concurrent_entity_requests_lab::world::{
    Admission, Conflict, Dependency, ExpectedValue, FaultInjection, Outcome, PropertyWrite,
    Request, World, WorldError,
};
use pretty_assertions::assert_eq;
use serde_json::{Value, json};
use sqlx::{PgPool, postgres::PgPoolOptions, types::Json};
use uuid::Uuid;

const PLACE: Uuid = Uuid::from_u128(0x100);
const OTHER_PLACE: Uuid = Uuid::from_u128(0x101);
const ACTOR: Uuid = Uuid::from_u128(0x200);
const TREE: Uuid = Uuid::from_u128(0x300);
const OTHER_TREE: Uuid = Uuid::from_u128(0x301);

struct Fixture {
    baseline_activities: i64,
}

async fn fixture(pool: &PgPool) -> Fixture {
    sqlx::query("INSERT INTO place (id, name) VALUES ($1, 'Grove'), ($2, 'Hill')")
        .bind(PLACE)
        .bind(OTHER_PLACE)
        .execute(pool)
        .await
        .unwrap();
    sqlx::query("INSERT INTO character (id, place_id, placement_version) VALUES ($1, $2, 1)")
        .bind(ACTOR)
        .bind(PLACE)
        .execute(pool)
        .await
        .unwrap();
    sqlx::query(
        r#"
        INSERT INTO entity (id, place_id, name)
        VALUES ($1, $3, 'Old Tree'), ($2, $3, 'Young Tree')
        "#,
    )
    .bind(TREE)
    .bind(OTHER_TREE)
    .bind(PLACE)
    .execute(pool)
    .await
    .unwrap();
    seed(pool, TREE, "color", json!("gray"), Uuid::from_u128(0x400)).await;
    seed(
        pool,
        TREE,
        "condition",
        json!("standing"),
        Uuid::from_u128(0x401),
    )
    .await;
    seed(
        pool,
        OTHER_TREE,
        "color",
        json!("gray"),
        Uuid::from_u128(0x402),
    )
    .await;
    seed(
        pool,
        OTHER_TREE,
        "condition",
        json!("standing"),
        Uuid::from_u128(0x403),
    )
    .await;
    Fixture {
        baseline_activities: activity_count(pool).await,
    }
}

async fn seed(pool: &PgPool, entity_id: Uuid, key: &str, value: Value, activity_id: Uuid) {
    sqlx::query(
        r#"
        INSERT INTO activity (id, actor_character_id, place_id, operation)
        VALUES ($1, $2, $3, 'seed fixture')
        "#,
    )
    .bind(activity_id)
    .bind(ACTOR)
    .bind(PLACE)
    .execute(pool)
    .await
    .unwrap();
    sqlx::query(
        r#"
        INSERT INTO property_slot (
            entity_id, property_key, current_version, current_value, current_activity_id
        )
        VALUES ($1, $2, 1, $3, $4)
        "#,
    )
    .bind(entity_id)
    .bind(key)
    .bind(Json(value.clone()))
    .bind(activity_id)
    .execute(pool)
    .await
    .unwrap();
    sqlx::query(
        r#"
        INSERT INTO property_history (
            entity_id, property_key, version, activity_id, value
        )
        VALUES ($1, $2, 1, $3, $4)
        "#,
    )
    .bind(entity_id)
    .bind(key)
    .bind(activity_id)
    .bind(Json(value))
    .execute(pool)
    .await
    .unwrap();
}

async fn second_pool(pool: &PgPool) -> PgPool {
    PgPoolOptions::new()
        .max_connections(4)
        .connect_with(pool.connect_options().as_ref().clone())
        .await
        .unwrap()
}

async fn current(pool: &PgPool, entity_id: Uuid, key: &str) -> Dependency {
    let (version, value) = sqlx::query_as::<_, (i64, Json<Value>)>(
        r#"
        SELECT current_version, current_value
        FROM property_slot
        WHERE entity_id = $1 AND property_key = $2
        "#,
    )
    .bind(entity_id)
    .bind(key)
    .fetch_one(pool)
    .await
    .unwrap();
    Dependency {
        entity_id,
        property_key: key.to_owned(),
        expected: ExpectedValue::Current {
            version,
            value: value.0,
        },
    }
}

fn absent(entity_id: Uuid, key: &str) -> Dependency {
    Dependency {
        entity_id,
        property_key: key.to_owned(),
        expected: ExpectedValue::Absent,
    }
}

fn write(entity_id: Uuid, key: &str, value: Value) -> PropertyWrite {
    PropertyWrite {
        entity_id,
        property_key: key.to_owned(),
        value,
    }
}

fn request(
    id: u128,
    operation: &str,
    dependencies: Vec<Dependency>,
    writes: Vec<PropertyWrite>,
) -> Request {
    Request {
        id: Uuid::from_u128(id),
        actor_character_id: ACTOR,
        expected_place_id: PLACE,
        expected_placement_version: 1,
        operation: operation.to_owned(),
        dependencies,
        writes,
    }
}

async fn activity_count(pool: &PgPool) -> i64 {
    sqlx::query_scalar("SELECT count(*) FROM activity")
        .fetch_one(pool)
        .await
        .unwrap()
}

async fn accepted_count(pool: &PgPool) -> i64 {
    sqlx::query_scalar("SELECT count(*) FROM accepted_request")
        .fetch_one(pool)
        .await
        .unwrap()
}

async fn value(pool: &PgPool, entity_id: Uuid, key: &str) -> Value {
    sqlx::query_scalar::<_, Json<Value>>(
        "SELECT current_value FROM property_slot WHERE entity_id = $1 AND property_key = $2",
    )
    .bind(entity_id)
    .bind(key)
    .fetch_one(pool)
    .await
    .unwrap()
    .0
}

fn accepted(outcome: &Outcome) -> bool {
    matches!(
        outcome,
        Outcome::Accepted {
            replayed: false,
            ..
        }
    )
}

#[sqlx::test(migrations = "./migration")]
async fn same_fact_has_one_accept_and_one_dependency_conflict(pool: PgPool) {
    let fixture = fixture(&pool).await;
    let observed = current(&pool, TREE, "color").await;
    let second = second_pool(&pool).await;
    let first_world = World::new(pool.clone());
    let second_world = World::new(second.clone());
    let blue = request(
        0x501,
        "paint the old Tree blue",
        vec![observed.clone()],
        vec![write(TREE, "color", json!("blue"))],
    );
    let red = request(
        0x502,
        "paint the old Tree red",
        vec![observed],
        vec![write(TREE, "color", json!("red"))],
    );

    let (blue, red) = tokio::join!(
        first_world.apply(blue, Admission::Wait, FaultInjection::None),
        second_world.apply(red, Admission::Wait, FaultInjection::None),
    );
    let outcomes = [blue.unwrap(), red.unwrap()];
    assert_eq!(
        outcomes.iter().filter(|outcome| accepted(outcome)).count(),
        1
    );
    assert_eq!(
        outcomes
            .iter()
            .filter(|outcome| matches!(
                outcome,
                Outcome::Conflict(Conflict::DependencyChanged { .. })
            ))
            .count(),
        1
    );
    assert_eq!(activity_count(&pool).await - fixture.baseline_activities, 1);
    assert_eq!(accepted_count(&pool).await, 1);
    assert!(matches!(
        value(&pool, TREE, "color").await.as_str(),
        Some("blue" | "red")
    ));
    second.close().await;
}

#[sqlx::test(migrations = "./migration")]
async fn independent_facts_on_one_tree_compose_without_entity_lock(pool: PgPool) {
    let fixture = fixture(&pool).await;
    let color = current(&pool, TREE, "color").await;
    let condition = current(&pool, TREE, "condition").await;
    let second = second_pool(&pool).await;
    let first_world = World::new(pool.clone());
    let second_world = World::new(second.clone());
    let (paint, fell) = tokio::join!(
        first_world.apply(
            request(
                0x510,
                "paint the Tree blue",
                vec![color],
                vec![write(TREE, "color", json!("blue"))],
            ),
            Admission::Wait,
            FaultInjection::None,
        ),
        second_world.apply(
            request(
                0x511,
                "fell the Tree",
                vec![condition],
                vec![write(TREE, "condition", json!("felled"))],
            ),
            Admission::Wait,
            FaultInjection::None,
        ),
    );

    assert!(accepted(&paint.unwrap()));
    assert!(accepted(&fell.unwrap()));
    assert_eq!(value(&pool, TREE, "color").await, json!("blue"));
    assert_eq!(value(&pool, TREE, "condition").await, json!("felled"));
    assert_eq!(activity_count(&pool).await - fixture.baseline_activities, 2);
    second.close().await;
}

#[sqlx::test(migrations = "./migration")]
async fn agent_declared_cross_fact_dependency_controls_sequential_validity(pool: PgPool) {
    let fixture = fixture(&pool).await;
    let world = World::new(pool.clone());

    let paint_first = request(
        0x520,
        "paint only while the old Tree is standing",
        vec![
            current(&pool, TREE, "color").await,
            current(&pool, TREE, "condition").await,
        ],
        vec![write(TREE, "color", json!("blue"))],
    );
    let fell_after = request(
        0x521,
        "fell the old Tree",
        vec![current(&pool, TREE, "condition").await],
        vec![write(TREE, "condition", json!("felled"))],
    );
    assert!(accepted(
        &world
            .apply(paint_first, Admission::Wait, FaultInjection::None)
            .await
            .unwrap()
    ));
    assert!(accepted(
        &world
            .apply(fell_after, Admission::Wait, FaultInjection::None)
            .await
            .unwrap()
    ));

    let paint_stale = request(
        0x522,
        "paint only while the young Tree is standing",
        vec![
            current(&pool, OTHER_TREE, "color").await,
            current(&pool, OTHER_TREE, "condition").await,
        ],
        vec![write(OTHER_TREE, "color", json!("blue"))],
    );
    let fell_first = request(
        0x523,
        "fell the young Tree",
        vec![current(&pool, OTHER_TREE, "condition").await],
        vec![write(OTHER_TREE, "condition", json!("felled"))],
    );
    assert!(accepted(
        &world
            .apply(fell_first, Admission::Wait, FaultInjection::None)
            .await
            .unwrap()
    ));
    assert!(matches!(
        world
            .apply(paint_stale, Admission::Wait, FaultInjection::None)
            .await
            .unwrap(),
        Outcome::Conflict(Conflict::DependencyChanged {
            entity_id: OTHER_TREE,
            property_key
        }) if property_key == "condition"
    ));
    assert_eq!(value(&pool, TREE, "color").await, json!("blue"));
    assert_eq!(value(&pool, TREE, "condition").await, json!("felled"));
    assert_eq!(value(&pool, OTHER_TREE, "color").await, json!("gray"));
    assert_eq!(value(&pool, OTHER_TREE, "condition").await, json!("felled"));
    assert_eq!(activity_count(&pool).await - fixture.baseline_activities, 3);
}

#[sqlx::test(migrations = "./migration")]
async fn canonical_multi_fact_locks_avoid_deadlock_and_revalidate_all_facts(pool: PgPool) {
    let fixture = fixture(&pool).await;
    let color = current(&pool, TREE, "color").await;
    let condition = current(&pool, TREE, "condition").await;
    let second = second_pool(&pool).await;
    let first_world = World::new(pool.clone());
    let second_world = World::new(second.clone());
    let (paint, fell) = tokio::join!(
        first_world.apply(
            request(
                0x528,
                "paint only while standing",
                vec![color.clone(), condition.clone()],
                vec![write(TREE, "color", json!("blue"))],
            ),
            Admission::Wait,
            FaultInjection::None,
        ),
        second_world.apply(
            request(
                0x529,
                "fell only while gray",
                vec![condition, color],
                vec![write(TREE, "condition", json!("felled"))],
            ),
            Admission::Wait,
            FaultInjection::None,
        ),
    );
    let outcomes = [paint.unwrap(), fell.unwrap()];
    assert_eq!(
        outcomes.iter().filter(|outcome| accepted(outcome)).count(),
        1
    );
    assert_eq!(
        outcomes
            .iter()
            .filter(|outcome| matches!(
                outcome,
                Outcome::Conflict(Conflict::DependencyChanged { .. })
            ))
            .count(),
        1
    );
    assert_eq!(activity_count(&pool).await - fixture.baseline_activities, 1);
    second.close().await;
}

#[sqlx::test(migrations = "./migration")]
async fn one_multi_fact_request_commits_one_activity_and_two_histories(pool: PgPool) {
    let fixture = fixture(&pool).await;
    let outcome = World::new(pool.clone())
        .apply(
            request(
                0x52a,
                "paint and fell in one bounded package",
                vec![
                    current(&pool, TREE, "condition").await,
                    current(&pool, TREE, "color").await,
                ],
                vec![
                    write(TREE, "condition", json!("felled")),
                    write(TREE, "color", json!("blue")),
                ],
            ),
            Admission::Wait,
            FaultInjection::None,
        )
        .await
        .unwrap();
    let Outcome::Accepted {
        replayed: false,
        result,
    } = outcome
    else {
        panic!("multi-fact package must be accepted");
    };
    assert_eq!(result.facts.len(), 2);
    let history_rows: i64 =
        sqlx::query_scalar("SELECT count(*) FROM property_history WHERE activity_id = $1")
            .bind(result.activity_id)
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(history_rows, 2);
    assert_eq!(activity_count(&pool).await - fixture.baseline_activities, 1);
    assert_eq!(value(&pool, TREE, "color").await, json!("blue"));
    assert_eq!(value(&pool, TREE, "condition").await, json!("felled"));
}

#[sqlx::test(migrations = "./migration")]
async fn expected_absence_is_one_exact_coordinator(pool: PgPool) {
    let fixture = fixture(&pool).await;
    let second = second_pool(&pool).await;
    let carve_a = request(
        0x530,
        "carve an owl",
        vec![absent(TREE, "carving")],
        vec![write(TREE, "carving", json!("owl"))],
    );
    let carve_b = request(
        0x531,
        "carve a fox",
        vec![absent(TREE, "carving")],
        vec![write(TREE, "carving", json!("fox"))],
    );
    let first_world = World::new(pool.clone());
    let second_world = World::new(second.clone());
    let (first, second_outcome) = tokio::join!(
        first_world.apply(carve_a, Admission::Wait, FaultInjection::None),
        second_world.apply(carve_b, Admission::Wait, FaultInjection::None),
    );
    let outcomes = [first.unwrap(), second_outcome.unwrap()];
    assert_eq!(
        outcomes.iter().filter(|outcome| accepted(outcome)).count(),
        1
    );
    assert_eq!(
        outcomes
            .iter()
            .filter(|outcome| matches!(
                outcome,
                Outcome::Conflict(Conflict::DependencyChanged { .. })
            ))
            .count(),
        1
    );
    assert_eq!(activity_count(&pool).await - fixture.baseline_activities, 1);
    assert_eq!(accepted_count(&pool).await, 1);
    assert!(matches!(
        value(&pool, TREE, "carving").await.as_str(),
        Some("owl" | "fox")
    ));
    second.close().await;
}

#[sqlx::test(migrations = "./migration")]
async fn quiet_entity_progresses_independently(pool: PgPool) {
    let fixture = fixture(&pool).await;
    let second = second_pool(&pool).await;
    let first_world = World::new(pool.clone());
    let second_world = World::new(second.clone());
    let (old_tree, young_tree) = tokio::join!(
        first_world.apply(
            request(
                0x540,
                "paint the old Tree",
                vec![current(&pool, TREE, "color").await],
                vec![write(TREE, "color", json!("blue"))],
            ),
            Admission::Wait,
            FaultInjection::None,
        ),
        second_world.apply(
            request(
                0x541,
                "paint the young Tree",
                vec![current(&pool, OTHER_TREE, "color").await],
                vec![write(OTHER_TREE, "color", json!("green"))],
            ),
            Admission::Wait,
            FaultInjection::None,
        ),
    );
    assert!(accepted(&old_tree.unwrap()));
    assert!(accepted(&young_tree.unwrap()));
    assert_eq!(activity_count(&pool).await - fixture.baseline_activities, 2);
    second.close().await;
}

#[sqlx::test(migrations = "./migration")]
async fn stale_placement_rejects_without_request_or_history(pool: PgPool) {
    let fixture = fixture(&pool).await;
    let prepared = request(
        0x550,
        "paint from a stale Place",
        vec![current(&pool, TREE, "color").await],
        vec![write(TREE, "color", json!("blue"))],
    );
    sqlx::query("UPDATE character SET place_id = $2, placement_version = 2 WHERE id = $1")
        .bind(ACTOR)
        .bind(OTHER_PLACE)
        .execute(&pool)
        .await
        .unwrap();

    assert_eq!(
        World::new(pool.clone())
            .apply(prepared, Admission::Wait, FaultInjection::None)
            .await
            .unwrap(),
        Outcome::Conflict(Conflict::PlacementChanged)
    );
    assert_eq!(activity_count(&pool).await, fixture.baseline_activities);
    assert_eq!(accepted_count(&pool).await, 0);
    assert_eq!(value(&pool, TREE, "color").await, json!("gray"));
}

#[sqlx::test(migrations = "./migration")]
async fn accepted_retry_replays_and_changed_fingerprint_conflicts(pool: PgPool) {
    let fixture = fixture(&pool).await;
    let world = World::new(pool.clone());
    let original = request(
        0x560,
        "paint idempotently",
        vec![current(&pool, TREE, "color").await],
        vec![write(TREE, "color", json!("blue"))],
    );
    let accepted_outcome = world
        .apply(original.clone(), Admission::Wait, FaultInjection::None)
        .await
        .unwrap();
    let Outcome::Accepted {
        replayed: false,
        result,
    } = accepted_outcome
    else {
        panic!("first attempt must be accepted");
    };
    assert_eq!(
        world
            .apply(original.clone(), Admission::Wait, FaultInjection::None)
            .await
            .unwrap(),
        Outcome::Accepted {
            replayed: true,
            result: result.clone()
        }
    );
    let mut changed = original;
    changed.operation = "reuse id for a different request".to_owned();
    assert_eq!(
        world
            .apply(changed, Admission::Wait, FaultInjection::None)
            .await
            .unwrap(),
        Outcome::Conflict(Conflict::RequestChanged)
    );
    assert_eq!(activity_count(&pool).await - fixture.baseline_activities, 1);
    assert_eq!(accepted_count(&pool).await, 1);
}

#[sqlx::test(migrations = "./migration")]
async fn injected_failure_rolls_back_activity_slot_history_and_request(pool: PgPool) {
    let fixture = fixture(&pool).await;
    let failed = request(
        0x570,
        "fail after Activity",
        vec![absent(TREE, "ribbon")],
        vec![write(TREE, "ribbon", json!("blue"))],
    );
    assert!(matches!(
        World::new(pool.clone())
            .apply(failed, Admission::Wait, FaultInjection::AfterActivityInsert)
            .await,
        Err(WorldError::InjectedFailure)
    ));
    assert_eq!(activity_count(&pool).await, fixture.baseline_activities);
    assert_eq!(accepted_count(&pool).await, 0);
    let slot_count: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM property_slot WHERE entity_id = $1 AND property_key = 'ribbon'",
    )
    .bind(TREE)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(slot_count, 0);
    let history_count: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM property_history WHERE entity_id = $1 AND property_key = 'ribbon'",
    )
    .bind(TREE)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(history_count, 0);
}

#[sqlx::test(migrations = "./migration")]
async fn rejected_and_busy_absence_leave_no_slot_but_accepted_absence_does(pool: PgPool) {
    let fixture = fixture(&pool).await;
    let world = World::new(pool.clone());
    let stale = request(
        0x580,
        "stale request with an absent dependency",
        vec![
            absent(TREE, "future_marker"),
            Dependency {
                entity_id: TREE,
                property_key: "color".to_owned(),
                expected: ExpectedValue::Current {
                    version: 99,
                    value: json!("gray"),
                },
            },
        ],
        vec![write(TREE, "color", json!("blue"))],
    );
    assert!(matches!(
        world
            .apply(stale, Admission::Wait, FaultInjection::None)
            .await
            .unwrap(),
        Outcome::Conflict(Conflict::DependencyChanged { .. })
    ));
    let future_slots: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM property_slot WHERE entity_id = $1 AND property_key = 'future_marker'",
    )
    .bind(TREE)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(future_slots, 0);

    let mut held = pool.begin().await.unwrap();
    sqlx::query(
        "SELECT entity_id FROM property_slot WHERE entity_id = $1 AND property_key = 'color' FOR UPDATE",
    )
    .bind(TREE)
    .execute(&mut *held)
    .await
    .unwrap();
    let busy_request = request(
        0x581,
        "busy request with an absent dependency",
        vec![
            current(&pool, TREE, "color").await,
            absent(TREE, "temporary_marker"),
        ],
        vec![write(TREE, "color", json!("blue"))],
    );
    assert_eq!(
        world
            .apply(busy_request, Admission::Nowait, FaultInjection::None)
            .await
            .unwrap(),
        Outcome::Busy
    );
    held.rollback().await.unwrap();
    let temporary_slots: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM property_slot WHERE entity_id = $1 AND property_key = 'temporary_marker'",
    )
    .bind(TREE)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(temporary_slots, 0);

    let accepted_absence = request(
        0x582,
        "paint while noting one absent fact",
        vec![
            current(&pool, TREE, "color").await,
            absent(TREE, "future_marker"),
        ],
        vec![write(TREE, "color", json!("blue"))],
    );
    assert!(accepted(
        &world
            .apply(accepted_absence, Admission::Wait, FaultInjection::None)
            .await
            .unwrap()
    ));
    let (empty, stored_bytes) = sqlx::query_as::<_, (bool, i32)>(
        r#"
        SELECT current_version IS NULL, pg_column_size(slot)
        FROM property_slot AS slot
        WHERE entity_id = $1 AND property_key = 'future_marker'
        "#,
    )
    .bind(TREE)
    .fetch_one(&pool)
    .await
    .unwrap();
    println!("persistent_absence_slot_bytes={stored_bytes}");
    assert!(empty);
    assert!((1..512).contains(&stored_bytes));
    assert_eq!(activity_count(&pool).await - fixture.baseline_activities, 1);
    assert_eq!(accepted_count(&pool).await, 1);
}

#[sqlx::test(migrations = "./migration")]
async fn structurally_invalid_write_is_rejected_before_world_state(pool: PgPool) {
    let fixture = fixture(&pool).await;
    let invalid = request(
        0x590,
        "omit the written fact from dependencies",
        vec![current(&pool, TREE, "condition").await],
        vec![write(TREE, "color", json!("blue"))],
    );
    assert!(matches!(
        World::new(pool.clone())
            .apply(invalid, Admission::Wait, FaultInjection::None)
            .await,
        Err(WorldError::Invalid(message)) if message.contains("has no exact dependency")
    ));
    assert_eq!(activity_count(&pool).await, fixture.baseline_activities);
    assert_eq!(accepted_count(&pool).await, 0);
}

#[sqlx::test(migrations = "./migration")]
async fn exact_fact_read_stays_on_primary_key_with_100k_unrelated_slots(pool: PgPool) {
    fixture(&pool).await;
    sqlx::query(
        r#"
        INSERT INTO property_slot (entity_id, property_key)
        SELECT $1, 'fact_' || lpad(number::text, 6, '0')
        FROM generate_series(1, 100000) AS number
        "#,
    )
    .bind(TREE)
    .execute(&pool)
    .await
    .unwrap();
    let plan = sqlx::query_scalar::<_, String>(
        r#"
        EXPLAIN (ANALYZE, BUFFERS, FORMAT TEXT)
        SELECT current_version, current_value
        FROM property_slot
        WHERE entity_id = $1 AND property_key = 'color'
        "#,
    )
    .bind(TREE)
    .fetch_all(&pool)
    .await
    .unwrap()
    .join("\n");
    println!("{plan}");
    assert!(
        plan.contains("Index Scan using property_slot_pkey")
            || plan.contains("Index Only Scan using property_slot_pkey"),
        "exact fact read must use the composite primary-key index:\n{plan}"
    );
    assert!(
        !plan.contains("Rows Removed by Filter"),
        "exact fact read must not filter unrelated rows:\n{plan}"
    );
}
