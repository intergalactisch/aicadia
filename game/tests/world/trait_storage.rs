use super::*;

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

    sqlx::raw_sql(include_str!("../../migration/0008_entity_trait.sql"))
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

    let action_activity_id = insert_historical_trait_action_activity(&pool, user_id).await;
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
            $3, $4, 'change_entity_state'
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
