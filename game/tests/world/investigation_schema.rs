use super::*;

async fn raw_attempt(
    pool: &PgPool,
    user_id: UserId,
    character_entity_id: EntityId,
    place_entity_id: EntityId,
    outcome: &str,
) -> Uuid {
    let id = Uuid::new_v4();
    sqlx::query(
        r#"
        INSERT INTO investigation_attempt (
            id, requested_by_user_id, request_id, character_entity_id,
            kind, position_activity_id, place_entity_id, outcome, created_at
        ) VALUES ($1, $2, $3, $4, 'entity_at_position',
                  (SELECT current_activity_id FROM position WHERE entity_id = $4),
                  $5, $6, statement_timestamp())
        "#,
    )
    .bind(id)
    .bind(user_id.0)
    .bind(Uuid::new_v4())
    .bind(character_entity_id.0)
    .bind(place_entity_id.0)
    .bind(outcome)
    .execute(pool)
    .await
    .unwrap();
    id
}

#[sqlx::test(migrations = "./migration")]
async fn investigation_attempt_schema_enforces_immutable_identity_and_one_way_lifecycle(
    pool: PgPool,
) {
    let world = World::new(pool.clone());
    let user_id = create_user(&world).await;
    let character = world
        .create_character(user_id, character("Attempt schema actor"))
        .await
        .unwrap();
    let place = world
        .create_entry_place(user_id, place("Attempt schema place"))
        .await
        .unwrap();
    world.enter_world(user_id).await.unwrap();
    let activity_id: Uuid =
        sqlx::query_scalar("SELECT latest_activity_id FROM place WHERE entity_id = $1")
            .bind(place.entity.id.0)
            .fetch_one(&pool)
            .await
            .unwrap();
    let replacement_activity_id: Uuid =
        sqlx::query_scalar("SELECT id FROM activity WHERE id <> $1 ORDER BY id LIMIT 1")
            .bind(activity_id)
            .fetch_one(&pool)
            .await
            .unwrap();

    let consumed = raw_attempt(
        &pool,
        user_id,
        character.entity.id,
        place.entity.id,
        "positive",
    )
    .await;
    for mutation in [
        "id = gen_random_uuid()",
        "requested_by_user_id = gen_random_uuid()",
        "request_id = gen_random_uuid()",
        "character_entity_id = gen_random_uuid()",
        "kind = 'connected_place'",
        "position_activity_id = gen_random_uuid()",
        "place_entity_id = gen_random_uuid()",
        "outcome = 'zero'",
        "created_at = created_at - interval '1 second'",
    ] {
        let query = format!("UPDATE investigation_attempt SET {mutation} WHERE id = $1");
        let error = sqlx::query(&query)
            .bind(consumed)
            .execute(&pool)
            .await
            .unwrap_err();
        assert_eq!(
            error
                .as_database_error()
                .and_then(|database| database.constraint()),
            Some("investigation_attempt_immutable_check")
        );
    }
    sqlx::query("UPDATE investigation_attempt SET consumed_by_activity_id = $2 WHERE id = $1")
        .bind(consumed)
        .bind(activity_id)
        .execute(&pool)
        .await
        .unwrap();
    for replacement in [None, Some(replacement_activity_id)] {
        assert!(
            sqlx::query(
                "UPDATE investigation_attempt SET consumed_by_activity_id = $2 WHERE id = $1",
            )
            .bind(consumed)
            .bind(replacement)
            .execute(&pool)
            .await
            .is_err()
        );
    }
    assert!(
        sqlx::query("DELETE FROM investigation_attempt WHERE id = $1")
            .bind(consumed)
            .execute(&pool)
            .await
            .is_err()
    );

    let first = raw_attempt(
        &pool,
        user_id,
        character.entity.id,
        place.entity.id,
        "positive",
    )
    .await;
    let second = raw_attempt(
        &pool,
        user_id,
        character.entity.id,
        place.entity.id,
        "positive",
    )
    .await;
    let third = raw_attempt(
        &pool,
        user_id,
        character.entity.id,
        place.entity.id,
        "positive",
    )
    .await;
    sqlx::query("UPDATE investigation_attempt SET voided_by_attempt_id = $2 WHERE id = $1")
        .bind(first)
        .bind(second)
        .execute(&pool)
        .await
        .unwrap();
    for replacement in [None, Some(third)] {
        assert!(
            sqlx::query(
                "UPDATE investigation_attempt SET voided_by_attempt_id = $2 WHERE id = $1",
            )
            .bind(first)
            .bind(replacement)
            .execute(&pool)
            .await
            .is_err()
        );
    }
    assert!(
        sqlx::query("UPDATE investigation_attempt SET voided_by_attempt_id = id WHERE id = $1")
            .bind(third)
            .execute(&pool)
            .await
            .is_err()
    );
    assert!(
        sqlx::query(
            "UPDATE investigation_attempt SET consumed_by_activity_id = $2, voided_by_attempt_id = $3 WHERE id = $1",
        )
        .bind(third)
        .bind(activity_id)
        .bind(second)
        .execute(&pool)
        .await
        .is_err()
    );

    let zero = raw_attempt(&pool, user_id, character.entity.id, place.entity.id, "zero").await;
    assert!(
        sqlx::query("UPDATE investigation_attempt SET consumed_by_activity_id = $2 WHERE id = $1",)
            .bind(zero)
            .bind(activity_id)
            .execute(&pool)
            .await
            .is_err()
    );
    assert!(
        sqlx::query("UPDATE investigation_attempt SET voided_by_attempt_id = $2 WHERE id = $1")
            .bind(zero)
            .bind(second)
            .execute(&pool)
            .await
            .is_err()
    );
}

#[sqlx::test(migrations = "./migration")]
async fn investigation_attempt_schema_enforces_retry_and_provenance_foreign_keys(pool: PgPool) {
    let world = World::new(pool.clone());
    let user_id = create_user(&world).await;
    let character = world
        .create_character(user_id, character("Attempt FK actor"))
        .await
        .unwrap();
    let place = world
        .create_entry_place(user_id, place("Attempt FK place"))
        .await
        .unwrap();
    world.enter_world(user_id).await.unwrap();
    let first = raw_attempt(
        &pool,
        user_id,
        character.entity.id,
        place.entity.id,
        "positive",
    )
    .await;
    let request_id: Uuid =
        sqlx::query_scalar("SELECT request_id FROM investigation_attempt WHERE id = $1")
            .bind(first)
            .fetch_one(&pool)
            .await
            .unwrap();
    assert!(
        sqlx::query(
            r#"
            INSERT INTO investigation_attempt (
                id, requested_by_user_id, request_id, character_entity_id,
                kind, position_activity_id, place_entity_id, outcome, created_at
            ) VALUES ($1, $2, $3, $4, 'entity_at_position',
                      (SELECT current_activity_id FROM position WHERE entity_id = $4),
                      $5, 'positive', statement_timestamp())
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(user_id.0)
        .bind(request_id)
        .bind(character.entity.id.0)
        .bind(place.entity.id.0)
        .execute(&pool)
        .await
        .is_err()
    );
    for (invalid_user, invalid_character, invalid_place) in [
        (Some(Uuid::new_v4()), None, None),
        (None, Some(Uuid::new_v4()), None),
        (None, None, Some(Uuid::new_v4())),
    ] {
        assert!(
            sqlx::query(
                r#"
                INSERT INTO investigation_attempt (
                    id, requested_by_user_id, request_id, character_entity_id,
                    kind, position_activity_id, place_entity_id, outcome, created_at
                ) VALUES ($1, $2, $3, $4, 'entity_at_position',
                          (SELECT current_activity_id FROM position WHERE entity_id = $4),
                          $5, 'positive', statement_timestamp())
                "#,
            )
            .bind(Uuid::new_v4())
            .bind(invalid_user.unwrap_or(user_id.0))
            .bind(Uuid::new_v4())
            .bind(invalid_character.unwrap_or(character.entity.id.0))
            .bind(invalid_place.unwrap_or(place.entity.id.0))
            .execute(&pool)
            .await
            .is_err()
        );
    }
    let second = raw_attempt(
        &pool,
        user_id,
        character.entity.id,
        place.entity.id,
        "positive",
    )
    .await;
    assert!(
        sqlx::query("UPDATE investigation_attempt SET voided_by_attempt_id = $2 WHERE id = $1")
            .bind(second)
            .bind(Uuid::new_v4())
            .execute(&pool)
            .await
            .is_err()
    );
    assert!(
        sqlx::query("UPDATE investigation_attempt SET consumed_by_activity_id = $2 WHERE id = $1",)
            .bind(second)
            .bind(Uuid::new_v4())
            .execute(&pool)
            .await
            .is_err()
    );
}
