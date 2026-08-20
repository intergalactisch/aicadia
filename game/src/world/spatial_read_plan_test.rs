use super::read::{
    LIST_CONNECTION_CONTINUED_PAGE_SQL, LIST_CONNECTION_FIRST_PAGE_SQL,
    LIST_PLACE_CONTINUED_PAGE_SQL, LIST_PLACE_FIRST_PAGE_SQL,
};
use super::*;
use serde_json::Value;

#[derive(Clone, Copy)]
enum ProjectionFixture {
    Dense,
    CrossAxis,
    Rotated,
}

async fn load_projection_fixture(pool: &PgPool, fixture: ProjectionFixture) {
    sqlx::query("TRUNCATE place_map_index")
        .execute(pool)
        .await
        .unwrap();
    sqlx::query("ALTER TABLE place_map_index DISABLE TRIGGER ALL")
        .execute(pool)
        .await
        .unwrap();
    let select = match fixture {
        ProjectionFixture::Dense => {
            "SELECT md5(series::text)::uuid, md5(('activity-' || series)::text)::uuid, 0::bigint, 0::bigint, 0::bigint FROM generate_series(1, 1000000) series"
        }
        ProjectionFixture::CrossAxis => {
            "SELECT md5(series::text)::uuid, md5(('activity-' || series)::text)::uuid, (series - 500001)::bigint, CASE WHEN series > 999800 THEN 0 ELSE 1000000 END::bigint, 0::bigint FROM generate_series(1, 1000000) series"
        }
        ProjectionFixture::Rotated => {
            "SELECT md5(series::text)::uuid, md5(('activity-' || series)::text)::uuid, (series - 500001)::bigint, (series - 500001)::bigint, CASE WHEN series > 999800 THEN 0 ELSE 1000000 END::bigint FROM generate_series(1, 1000000) series"
        }
    };
    sqlx::query(&format!(
        "INSERT INTO place_map_index (place_entity_id, position_activity_id, x_cm, y_cm, z_cm) {select}"
    ))
    .execute(pool)
    .await
    .unwrap();
    sqlx::query("ALTER TABLE place_map_index ENABLE TRIGGER ALL")
        .execute(pool)
        .await
        .unwrap();
    sqlx::query("VACUUM (ANALYZE) place_map_index")
        .execute(pool)
        .await
        .unwrap();
}

fn plan_has_node(plan: &Value, node_type: &str) -> bool {
    match plan {
        Value::Object(map) => {
            map.get("Node Type").and_then(Value::as_str) == Some(node_type)
                || map.values().any(|value| plan_has_node(value, node_type))
        }
        Value::Array(value) => value.iter().any(|value| plan_has_node(value, node_type)),
        _ => false,
    }
}

fn root_shared_blocks(plan: &Value) -> u64 {
    let root = &plan[0]["Plan"];
    root["Shared Hit Blocks"].as_u64().unwrap_or(0)
        + root["Shared Read Blocks"].as_u64().unwrap_or(0)
}

async fn explain_first_page(pool: &PgPool, bounds: (i64, i64, i64, i64, i64, i64)) -> Value {
    sqlx::query_scalar(&format!(
        "EXPLAIN (ANALYZE, BUFFERS, FORMAT JSON) {LIST_PLACE_FIRST_PAGE_SQL}"
    ))
    .bind(bounds.0)
    .bind(bounds.1)
    .bind(bounds.2)
    .bind(bounds.3)
    .bind(bounds.4)
    .bind(bounds.5)
    .bind(101_i64)
    .fetch_one(pool)
    .await
    .unwrap()
}

async fn first_page_cursor(
    pool: &PgPool,
    bounds: (i64, i64, i64, i64, i64, i64),
) -> PlaceMapCandidate {
    sqlx::query_as::<_, PlaceMapCandidate>(LIST_PLACE_FIRST_PAGE_SQL)
        .bind(bounds.0)
        .bind(bounds.1)
        .bind(bounds.2)
        .bind(bounds.3)
        .bind(bounds.4)
        .bind(bounds.5)
        .bind(100_i64)
        .fetch_all(pool)
        .await
        .unwrap()
        .pop()
        .expect("every falsifier exposes at least one hundred candidates")
}

async fn explain_continued_page(
    pool: &PgPool,
    bounds: (i64, i64, i64, i64, i64, i64),
    cursor: PlaceMapCandidate,
) -> Value {
    sqlx::query_scalar(&format!(
        "EXPLAIN (ANALYZE, BUFFERS, FORMAT JSON) {LIST_PLACE_CONTINUED_PAGE_SQL}"
    ))
    .bind(bounds.0)
    .bind(bounds.1)
    .bind(bounds.2)
    .bind(bounds.3)
    .bind(bounds.4)
    .bind(bounds.5)
    .bind(cursor.x_cm)
    .bind(cursor.y_cm)
    .bind(cursor.z_cm)
    .bind(cursor.place_entity_id.0)
    .bind(101_i64)
    .fetch_one(pool)
    .await
    .unwrap()
}

#[sqlx::test(migrations = "./migration")]
async fn exact_production_page_forms_survive_million_row_falsifiers(pool: PgPool) {
    let world = World::new(pool.clone());
    let user = world.create_user().await.unwrap();
    world
        .create_character(
            user.id,
            CreateCharacter {
                name: "Plan Reader".to_owned(),
                description: "Exercises exact production Place queries.".to_owned(),
                property: Vec::new(),
                r#trait: Vec::new(),
            },
        )
        .await
        .unwrap();
    world
        .create_entry_place(
            user.id,
            CreateEntryPlace {
                name: "Plan Origin".to_owned(),
                description: "Origin for exact production Place-query evidence.".to_owned(),
                property: Vec::new(),
                r#trait: Vec::new(),
            },
        )
        .await
        .unwrap();
    world.enter_world(user.id).await.unwrap();

    for (fixture, bounds) in [
        (ProjectionFixture::Dense, (0, 0, 0, 0, 0, 0)),
        (
            ProjectionFixture::CrossAxis,
            (-500_000, 499_999, 0, 0, 0, 0),
        ),
        (
            ProjectionFixture::Rotated,
            (-500_000, 499_999, -500_000, 499_999, 0, 0),
        ),
    ] {
        load_projection_fixture(&pool, fixture).await;
        let first_plan = explain_first_page(&pool, bounds).await;
        let cursor = first_page_cursor(&pool, bounds).await;
        let continued_plan = explain_continued_page(&pool, bounds, cursor).await;
        for plan in [&first_plan, &continued_plan] {
            assert!(!plan_has_node(plan, "Seq Scan"), "unexpected plan: {plan}");
            assert!(
                root_shared_blocks(plan) <= 100,
                "candidate page touched too many shared blocks: {plan}"
            );
        }

        if matches!(fixture, ProjectionFixture::Dense) {
            let mut connection = pool.acquire().await.unwrap();
            let deep_cursor = sqlx::query_as::<_, PlaceMapCandidate>(
                r#"
                SELECT place_entity_id, position_activity_id, x_cm, y_cm, z_cm
                FROM place_map_index
                ORDER BY x_cm, y_cm, z_cm, place_entity_id
                OFFSET 899999
                LIMIT 1
                "#,
            )
            .fetch_one(&mut *connection)
            .await
            .unwrap();
            sqlx::query("SET plan_cache_mode = force_generic_plan")
                .execute(&mut *connection)
                .await
                .unwrap();
            let deep_plan: Value = sqlx::query_scalar(&format!(
                "EXPLAIN (ANALYZE, BUFFERS, FORMAT JSON) {LIST_PLACE_CONTINUED_PAGE_SQL}"
            ))
            .bind(bounds.0)
            .bind(bounds.1)
            .bind(bounds.2)
            .bind(bounds.3)
            .bind(bounds.4)
            .bind(bounds.5)
            .bind(deep_cursor.x_cm)
            .bind(deep_cursor.y_cm)
            .bind(deep_cursor.z_cm)
            .bind(deep_cursor.place_entity_id.0)
            .bind(101_i64)
            .fetch_one(&mut *connection)
            .await
            .unwrap();
            assert!(
                !plan_has_node(&deep_plan, "Seq Scan"),
                "unexpected deep generic plan: {deep_plan}"
            );
            assert_eq!(deep_plan[0]["Plan"]["Actual Rows"].as_u64(), Some(101));
            assert!(
                root_shared_blocks(&deep_plan) <= 100,
                "deep generic Place page touched too many shared blocks: {deep_plan}"
            );
        }

        let first = world
            .list_place(
                user.id,
                ListPlace {
                    min_x_cm: bounds.0,
                    max_x_cm: bounds.1,
                    min_y_cm: bounds.2,
                    max_y_cm: bounds.3,
                    min_z_cm: bounds.4,
                    max_z_cm: bounds.5,
                    cursor: None,
                    limit: 100,
                },
            )
            .await
            .unwrap();
        assert!(first.place.is_empty());
        let continued = world
            .list_place(
                user.id,
                ListPlace {
                    min_x_cm: bounds.0,
                    max_x_cm: bounds.1,
                    min_y_cm: bounds.2,
                    max_y_cm: bounds.3,
                    min_z_cm: bounds.4,
                    max_z_cm: bounds.5,
                    cursor: first.next,
                    limit: 100,
                },
            )
            .await
            .unwrap();
        assert!(continued.place.is_empty());
    }
}

async fn load_hot_connection_fixture(pool: &PgPool, anchor_id: Uuid) {
    sqlx::query("TRUNCATE connection CASCADE")
        .execute(pool)
        .await
        .unwrap();
    sqlx::query("ALTER TABLE connection DISABLE TRIGGER ALL")
        .execute(pool)
        .await
        .unwrap();
    sqlx::query(
        r#"
        INSERT INTO connection (
            id, source_place_entity_id, destination_place_entity_id,
            source_position_activity_id, destination_position_activity_id,
            allows_reverse, has_course, name, description, created_by_activity_id
        )
        SELECT lpad(to_hex(series * 2 - 1), 32, '0')::uuid,
               $1, 'ffffffff-ffff-ffff-ffff-fffffffffff1'::uuid,
               'ffffffff-ffff-ffff-ffff-fffffffffff2'::uuid,
               'ffffffff-ffff-ffff-ffff-fffffffffff3'::uuid,
               true, false, 'Hot source Connection',
               'Planner-only source-side incident fixture.',
               lpad(to_hex(series * 2 - 1), 32, '0')::uuid
        FROM generate_series(1, 500000) AS series
        "#,
    )
    .bind(anchor_id)
    .execute(pool)
    .await
    .unwrap();
    sqlx::query(
        r#"
        INSERT INTO connection (
            id, source_place_entity_id, destination_place_entity_id,
            source_position_activity_id, destination_position_activity_id,
            allows_reverse, has_course, name, description, created_by_activity_id
        )
        SELECT lpad(to_hex(series * 2), 32, '0')::uuid,
               'ffffffff-ffff-ffff-ffff-fffffffffff4'::uuid, $1,
               'ffffffff-ffff-ffff-ffff-fffffffffff5'::uuid,
               'ffffffff-ffff-ffff-ffff-fffffffffff6'::uuid,
               true, false, 'Hot destination Connection',
               'Planner-only destination-side incident fixture.',
               lpad(to_hex(series * 2), 32, '0')::uuid
        FROM generate_series(1, 500000) AS series
        "#,
    )
    .bind(anchor_id)
    .execute(pool)
    .await
    .unwrap();
    sqlx::query("ALTER TABLE connection ENABLE TRIGGER ALL")
        .execute(pool)
        .await
        .unwrap();
    sqlx::query("VACUUM (ANALYZE) connection")
        .execute(pool)
        .await
        .unwrap();
}

#[sqlx::test(migrations = "./migration")]
async fn exact_connection_page_forms_keep_hot_deep_generic_plans_bounded(pool: PgPool) {
    let anchor_id = Uuid::from_u128(u128::MAX - 100);
    load_hot_connection_fixture(&pool, anchor_id).await;
    let mut connection = pool.acquire().await.unwrap();
    sqlx::query("SET plan_cache_mode = force_generic_plan")
        .execute(&mut *connection)
        .await
        .unwrap();

    let first: Value = sqlx::query_scalar(&format!(
        "EXPLAIN (ANALYZE, BUFFERS, FORMAT JSON) {LIST_CONNECTION_FIRST_PAGE_SQL}"
    ))
    .bind(anchor_id)
    .bind(101_i64)
    .fetch_one(&mut *connection)
    .await
    .unwrap();
    let continued: Value = sqlx::query_scalar(&format!(
        "EXPLAIN (ANALYZE, BUFFERS, FORMAT JSON) {LIST_CONNECTION_CONTINUED_PAGE_SQL}"
    ))
    .bind(anchor_id)
    .bind(Uuid::from_u128(900_000))
    .bind(101_i64)
    .fetch_one(&mut *connection)
    .await
    .unwrap();

    for plan in [&first, &continued] {
        assert!(!plan_has_node(plan, "Seq Scan"), "unexpected plan: {plan}");
        assert_eq!(plan[0]["Plan"]["Actual Rows"].as_u64(), Some(101));
        assert!(
            root_shared_blocks(plan) <= 100,
            "incident page touched too many shared blocks: {plan}"
        );
    }
}
