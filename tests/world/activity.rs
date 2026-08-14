use super::*;

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
        include_str!("../../migration/0001_world.sql"),
        include_str!("../../migration/0002_rename_app_user.sql"),
        include_str!("../../migration/0003_character.sql"),
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

    sqlx::raw_sql(include_str!(
        "../../migration/0004_world_entry_activity.sql"
    ))
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
