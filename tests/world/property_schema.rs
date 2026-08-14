use super::*;

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
        include_str!("../../migration/0001_world.sql"),
        include_str!("../../migration/0002_rename_app_user.sql"),
        include_str!("../../migration/0003_character.sql"),
        include_str!("../../migration/0004_world_entry_activity.sql"),
        include_str!("../../migration/0005_agent_action.sql"),
        include_str!("../../migration/0006_entity_interaction.sql"),
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

    sqlx::raw_sql(include_str!("../../migration/0007_entity_property.sql"))
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
