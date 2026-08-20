use super::*;
use aicadia::{ActivityId, ConnectionId};

const MIGRATION_0011: &str = include_str!("../../migration/0011_spatial_exploration.sql");

async fn reset_to_0010(pool: &PgPool) {
    sqlx::raw_sql("DROP SCHEMA public CASCADE; CREATE SCHEMA public")
        .execute(pool)
        .await
        .unwrap();
    for migration in [
        include_str!("../../migration/0001_world.sql"),
        include_str!("../../migration/0002_rename_app_user.sql"),
        include_str!("../../migration/0003_character.sql"),
        include_str!("../../migration/0004_world_entry_activity.sql"),
        include_str!("../../migration/0005_agent_action.sql"),
        include_str!("../../migration/0006_entity_interaction.sql"),
        include_str!("../../migration/0007_entity_property.sql"),
        include_str!("../../migration/0008_entity_trait.sql"),
        include_str!("../../migration/0009_uniform_entity_state.sql"),
        include_str!("../../migration/0010_investigation.sql"),
    ] {
        sqlx::raw_sql(migration).execute(pool).await.unwrap();
    }
}

struct LegacyFixture {
    user_id: Uuid,
    character_id: Uuid,
    place_id: Uuid,
    entity_id: Uuid,
    place_activity_id: Uuid,
    entry_activity_id: Uuid,
    entity_activity_id: Uuid,
}

async fn legacy_fixture(pool: &PgPool) -> LegacyFixture {
    let fixture = LegacyFixture {
        user_id: Uuid::new_v4(),
        character_id: Uuid::new_v4(),
        place_id: Uuid::new_v4(),
        entity_id: Uuid::new_v4(),
        place_activity_id: Uuid::new_v4(),
        entry_activity_id: Uuid::new_v4(),
        entity_activity_id: Uuid::new_v4(),
    };
    sqlx::query("INSERT INTO \"user\" (id) VALUES ($1)")
        .bind(fixture.user_id)
        .execute(pool)
        .await
        .unwrap();
    for (id, name) in [
        (fixture.character_id, "Legacy Character"),
        (fixture.place_id, "Legacy Entry"),
        (fixture.entity_id, "Legacy Situated Entity"),
    ] {
        sqlx::query(
            "INSERT INTO entity (id, name, description, introduced_by_user_id) VALUES ($1, $2, 'Legacy exact state.', $3)",
        )
        .bind(id)
        .bind(name)
        .bind(fixture.user_id)
        .execute(pool)
        .await
        .unwrap();
    }
    sqlx::query("INSERT INTO character (entity_id, owner_user_id) VALUES ($1, $2)")
        .bind(fixture.character_id)
        .bind(fixture.user_id)
        .execute(pool)
        .await
        .unwrap();
    sqlx::query(
        "INSERT INTO activity (id, operation, requested_by_user_id, actor_character_entity_id) VALUES ($1, 'create_entry_place', $2, $3)",
    )
    .bind(fixture.place_activity_id)
    .bind(fixture.user_id)
    .bind(fixture.character_id)
    .execute(pool)
    .await
    .unwrap();
    sqlx::query(
        "INSERT INTO activity_entity (activity_id, entity_id, role) VALUES ($1, $2, 'subject')",
    )
    .bind(fixture.place_activity_id)
    .bind(fixture.place_id)
    .execute(pool)
    .await
    .unwrap();
    sqlx::query(
        "INSERT INTO place (entity_id, is_entry, latest_activity_id) VALUES ($1, true, $2)",
    )
    .bind(fixture.place_id)
    .bind(fixture.place_activity_id)
    .execute(pool)
    .await
    .unwrap();
    sqlx::query(
        "INSERT INTO activity (id, operation, requested_by_user_id, actor_character_entity_id, context_place_entity_id) VALUES ($1, 'enter_world', $2, $3, $4)",
    )
    .bind(fixture.entry_activity_id)
    .bind(fixture.user_id)
    .bind(fixture.character_id)
    .bind(fixture.place_id)
    .execute(pool)
    .await
    .unwrap();
    sqlx::query(
        "INSERT INTO activity_entity (activity_id, entity_id, role) VALUES ($1, $2, 'destination')",
    )
    .bind(fixture.entry_activity_id)
    .bind(fixture.place_id)
    .execute(pool)
    .await
    .unwrap();
    sqlx::query("UPDATE character SET current_place_entity_id = $2 WHERE entity_id = $1")
        .bind(fixture.character_id)
        .bind(fixture.place_id)
        .execute(pool)
        .await
        .unwrap();
    sqlx::query(
        r#"
        INSERT INTO activity (
            id, operation, requested_by_user_id, actor_character_entity_id,
            context_place_entity_id, prose, request_id, request_fingerprint,
            action_consequence
        ) VALUES ($1, 'submit_action', $2, $3, $4, 'Legacy introduction.', $5, $6, 'introduce_entity')
        "#,
    )
    .bind(fixture.entity_activity_id)
    .bind(fixture.user_id)
    .bind(fixture.character_id)
    .bind(fixture.place_id)
    .bind(Uuid::new_v4())
    .bind(vec![1_u8; 32])
    .execute(pool)
    .await
    .unwrap();
    sqlx::query(
        "INSERT INTO activity_entity (activity_id, entity_id, role) VALUES ($1, $2, 'subject'), ($1, $3, 'location')",
    )
    .bind(fixture.entity_activity_id)
    .bind(fixture.entity_id)
    .bind(fixture.place_id)
    .execute(pool)
    .await
    .unwrap();
    sqlx::query("INSERT INTO entity_location (entity_id, place_entity_id) VALUES ($1, $2)")
        .bind(fixture.entity_id)
        .bind(fixture.place_id)
        .execute(pool)
        .await
        .unwrap();
    fixture
}

#[sqlx::test(migrations = "./migration")]
async fn spatial_migration_backfills_only_exact_legacy_position_history(pool: PgPool) {
    reset_to_0010(&pool).await;
    let fixture = legacy_fixture(&pool).await;
    sqlx::raw_sql(MIGRATION_0011).execute(&pool).await.unwrap();

    let position: Vec<(Uuid, Uuid, i64, i64, i64)> = sqlx::query_as(
        "SELECT entity_id, current_activity_id, x_cm, y_cm, z_cm FROM position JOIN position_version USING (entity_id) WHERE current_activity_id = activity_id ORDER BY entity_id",
    )
    .fetch_all(&pool)
    .await
    .unwrap();
    assert_eq!(position.len(), 3);
    assert!(
        position
            .iter()
            .all(|row| (row.2, row.3, row.4) == (0, 0, 0))
    );
    assert!(position.contains(&(fixture.place_id, fixture.place_activity_id, 0, 0, 0)));
    assert!(position.contains(&(fixture.character_id, fixture.entry_activity_id, 0, 0, 0)));
    assert!(position.contains(&(fixture.entity_id, fixture.entity_activity_id, 0, 0, 0)));
    let typed_count: i64 =
        sqlx::query_scalar("SELECT count(*) FROM activity_position WHERE role = 'result'")
            .fetch_one(&pool)
            .await
            .unwrap();
    let projection_count: i64 = sqlx::query_scalar("SELECT count(*) FROM place_map_index")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!((typed_count, projection_count), (3, 1));
    let investigation_kind_default: Option<String> = sqlx::query_scalar(
        "SELECT column_default FROM information_schema.columns WHERE table_schema = 'public' AND table_name = 'investigation_attempt' AND column_name = 'kind'",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(investigation_kind_default, None);
}

#[sqlx::test(migrations = "./migration")]
async fn spatial_migration_refuses_missing_or_ambiguous_legacy_facts(pool: PgPool) {
    for variant in ["missing_place", "ambiguous_character", "missing_entity"] {
        reset_to_0010(&pool).await;
        let fixture = legacy_fixture(&pool).await;
        match variant {
            "missing_place" => {
                sqlx::query(
                    "ALTER TABLE activity_entity DISABLE TRIGGER activity_entity_immutable",
                )
                .execute(&pool)
                .await
                .unwrap();
                sqlx::query(
                    "DELETE FROM activity_entity WHERE activity_id = $1 AND role = 'subject'",
                )
                .bind(fixture.place_activity_id)
                .execute(&pool)
                .await
                .unwrap();
            }
            "ambiguous_character" => {
                let activity_id = Uuid::new_v4();
                sqlx::query("INSERT INTO activity (id, operation, requested_by_user_id, actor_character_entity_id, context_place_entity_id) VALUES ($1, 'enter_world', $2, $3, $4)")
                    .bind(activity_id).bind(fixture.user_id).bind(fixture.character_id).bind(fixture.place_id)
                    .execute(&pool).await.unwrap();
                sqlx::query("INSERT INTO activity_entity (activity_id, entity_id, role) VALUES ($1, $2, 'destination')")
                    .bind(activity_id).bind(fixture.place_id).execute(&pool).await.unwrap();
            }
            "missing_entity" => {
                sqlx::query(
                    "ALTER TABLE activity_entity DISABLE TRIGGER activity_entity_immutable",
                )
                .execute(&pool)
                .await
                .unwrap();
                sqlx::query(
                    "DELETE FROM activity_entity WHERE activity_id = $1 AND role = 'subject'",
                )
                .bind(fixture.entity_activity_id)
                .execute(&pool)
                .await
                .unwrap();
            }
            _ => unreachable!(),
        }
        let error = sqlx::raw_sql(MIGRATION_0011)
            .execute(&pool)
            .await
            .unwrap_err();
        let constraint = error
            .as_database_error()
            .and_then(|error| error.constraint());
        assert!(
            matches!(
                constraint,
                Some("position_backfill_place_check")
                    | Some("position_backfill_character_check")
                    | Some("position_backfill_entity_check")
            ),
            "unexpected refusal for {variant}: {error:?}"
        );
        let spatial_table_count: i64 = sqlx::query_scalar(
            "SELECT count(*) FROM information_schema.tables WHERE table_schema = 'public' AND table_name = 'position_version'",
        )
        .fetch_one(&pool).await.unwrap();
        assert_eq!(
            spatial_table_count, 0,
            "{variant} must fail before accepting schema"
        );
    }
}

#[sqlx::test(migrations = "./migration")]
async fn world_spatial_writes_keep_position_activity_and_projection_atomic(pool: PgPool) {
    let world = World::new(pool.clone());
    let user_id = create_user(&world).await;
    let character = world
        .create_character(user_id, character("Spatial Actor"))
        .await
        .unwrap();
    assert!(character.position.is_none());
    let place = world
        .create_entry_place(user_id, place("Spatial Origin"))
        .await
        .unwrap();
    assert_eq!(
        (
            place.position.x_cm,
            place.position.y_cm,
            place.position.z_cm
        ),
        (0, 0, 0)
    );
    let entered = world.enter_world(user_id).await.unwrap();
    assert_eq!(
        entered.position.as_ref().map(|p| (p.x_cm, p.y_cm, p.z_cm)),
        Some((0, 0, 0))
    );
    let revision = world
        .get_character(user_id, GetEntityCurrentState::default())
        .await
        .unwrap()
        .place_revision
        .unwrap();
    let mut input = action(Uuid::new_v4(), revision, "Hovering Cup");
    let ActionConsequence::IntroduceEntity(ref mut introduced) = input.consequence else {
        unreachable!()
    };
    introduced.position_description =
        Some("This cup remains exactly two centimetres above the table.".to_owned());
    let accepted = world.submit_action(user_id, input).await.unwrap();
    let entity_id = introduced_entity(&accepted).id;
    let rows: (i64, i64, i64, Option<String>) = sqlx::query_as(
        r#"
        SELECT
          (SELECT count(*) FROM position WHERE entity_id = $1),
          (SELECT count(*) FROM activity_position WHERE activity_id = $2 AND role = 'origin'),
          (SELECT count(*) FROM activity_position WHERE activity_id = $2 AND role = 'result'),
          (SELECT description FROM position_version WHERE entity_id = $1)
        "#,
    )
    .bind(entity_id.0)
    .bind(accepted.activity.id.0)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!((rows.0, rows.1, rows.2), (1, 1, 1));
    assert_eq!(
        rows.3.as_deref(),
        Some("This cup remains exactly two centimetres above the table.")
    );

    sqlx::raw_sql(
        r#"
        CREATE FUNCTION reject_test_position() RETURNS trigger LANGUAGE plpgsql AS $$
        BEGIN
          IF EXISTS (SELECT 1 FROM entity WHERE id = NEW.entity_id AND name = 'Rollback Position') THEN
            RAISE EXCEPTION 'injected Position failure';
          END IF;
          RETURN NEW;
        END;
        $$;
        CREATE TRIGGER reject_test_position BEFORE INSERT ON position_version
        FOR EACH ROW EXECUTE FUNCTION reject_test_position();
        "#,
    ).execute(&pool).await.unwrap();
    let before: (i64, i64, i64) = sqlx::query_as(
        "SELECT (SELECT count(*) FROM entity), (SELECT count(*) FROM activity), (SELECT count(*) FROM position_version)",
    ).fetch_one(&pool).await.unwrap();
    let latest = world
        .get_character(user_id, GetEntityCurrentState::default())
        .await
        .unwrap()
        .place_revision
        .unwrap();
    assert!(
        world
            .submit_action(user_id, action(Uuid::new_v4(), latest, "Rollback Position"))
            .await
            .is_err()
    );
    let after: (i64, i64, i64) = sqlx::query_as(
        "SELECT (SELECT count(*) FROM entity), (SELECT count(*) FROM activity), (SELECT count(*) FROM position_version)",
    ).fetch_one(&pool).await.unwrap();
    assert_eq!(after, before);
}

#[sqlx::test(migrations = "./migration")]
async fn position_lineage_constraints_and_projection_rebuild_are_canonical(pool: PgPool) {
    let world = World::new(pool.clone());
    let user_id = create_user(&world).await;
    let lineage_character = world
        .create_character(user_id, character("Lineage Actor"))
        .await
        .unwrap();
    let place = world
        .create_entry_place(user_id, place("Lineage Place"))
        .await
        .unwrap();
    let entered_character = world.enter_world(user_id).await.unwrap();
    let root_activity = place.position.position_revision.activity_id().0;
    let character_root_activity = entered_character
        .position
        .as_ref()
        .unwrap()
        .position_revision
        .activity_id();

    let loose_entity = world
        .create_entity(user_id, entity("Unpositioned Constraint Subject"))
        .await
        .unwrap();
    let loose_activity: Uuid = sqlx::query_scalar(
        "SELECT activity_id FROM activity_entity WHERE entity_id = $1 AND role = 'subject'",
    )
    .bind(loose_entity.id.0)
    .fetch_one(&pool)
    .await
    .unwrap();
    let coordinate_error = sqlx::query(
        "INSERT INTO position_version (entity_id, activity_id, x_cm, y_cm, z_cm) VALUES ($1, $2, 1000000000000001, 0, 0)",
    )
    .bind(loose_entity.id.0)
    .bind(loose_activity)
    .execute(&pool)
    .await
    .unwrap_err();
    assert_eq!(
        coordinate_error
            .as_database_error()
            .and_then(|error| error.constraint()),
        Some("position_version_coordinate_check")
    );
    let orphan_error = sqlx::query(
        "INSERT INTO position_version (entity_id, activity_id, x_cm, y_cm, z_cm) VALUES ($1, $2, 1, 0, 0)",
    )
    .bind(loose_entity.id.0)
    .bind(loose_activity)
    .execute(&pool)
    .await
    .unwrap_err();
    assert_eq!(
        orphan_error
            .as_database_error()
            .and_then(|error| error.constraint()),
        Some("position_complete_check")
    );

    assert!(sqlx::query(
        "INSERT INTO position_version (entity_id, activity_id, x_cm, y_cm, z_cm) VALUES ($1, $2, 0, 0, 0)",
    ).bind(place.entity.id.0).bind(Uuid::new_v4()).execute(&pool).await.is_err());
    assert!(
        sqlx::query(
            "UPDATE position_version SET x_cm = 1 WHERE entity_id = $1 AND activity_id = $2"
        )
        .bind(place.entity.id.0)
        .bind(root_activity)
        .execute(&pool)
        .await
        .is_err()
    );
    assert!(
        sqlx::query("UPDATE position SET current_activity_id = $2 WHERE entity_id = $1")
            .bind(place.entity.id.0)
            .bind(Uuid::new_v4())
            .execute(&pool)
            .await
            .is_err()
    );

    sqlx::query("UPDATE place_map_index SET x_cm = 99 WHERE place_entity_id = $1")
        .bind(place.entity.id.0)
        .execute(&pool)
        .await
        .unwrap();
    sqlx::raw_sql(
        r#"
        TRUNCATE place_map_index;
        INSERT INTO place_map_index (place_entity_id, position_activity_id, x_cm, y_cm, z_cm)
        SELECT place.entity_id, position.current_activity_id, version.x_cm, version.y_cm, version.z_cm
        FROM place
        JOIN position ON position.entity_id = place.entity_id
        JOIN position_version version ON version.entity_id = position.entity_id AND version.activity_id = position.current_activity_id;
        "#,
    ).execute(&pool).await.unwrap();
    let rebuilt: (Uuid, i64, i64, i64) = sqlx::query_as(
        "SELECT position_activity_id, x_cm, y_cm, z_cm FROM place_map_index WHERE place_entity_id = $1",
    ).bind(place.entity.id.0).fetch_one(&pool).await.unwrap();
    assert_eq!(rebuilt, (root_activity, 0, 0, 0));
    let indexes: Vec<String> = sqlx::query_scalar(
        "SELECT indexname FROM pg_indexes WHERE schemaname = 'public' AND tablename = 'place_map_index' ORDER BY indexname",
    ).fetch_all(&pool).await.unwrap();
    assert!(indexes.contains(&"place_map_index_x_y_z_place_covering_index".to_owned()));
    assert!(indexes.contains(&"place_map_index_y_z_x_place_covering_index".to_owned()));
    assert!(indexes.contains(&"place_map_index_z_x_y_place_covering_index".to_owned()));

    let (destination_id, destination_activity_id) = raw_positioned_place(
        &pool,
        user_id,
        lineage_character.entity.id,
        place.entity.id,
        100,
        0,
        0,
    )
    .await;
    let mut connection_transaction = pool.begin().await.unwrap();
    let connection_activity = raw_connection_activity(
        &mut connection_transaction,
        user_id,
        lineage_character.entity.id,
        place.entity.id,
    )
    .await;
    let connection_id = insert_raw_connection(
        &mut connection_transaction,
        connection_activity,
        (
            place.entity.id,
            place.position.position_revision.activity_id(),
        ),
        (destination_id, destination_activity_id),
        &[],
    )
    .await;
    connection_transaction.commit().await.unwrap();

    let incomplete = commit_raw_position_successor(
        &pool,
        user_id,
        lineage_character.entity.id,
        place.entity.id,
        destination_id,
        connection_id,
        character_root_activity,
        100,
        false,
    )
    .await
    .unwrap_err();
    assert_eq!(
        incomplete
            .as_database_error()
            .and_then(|error| error.constraint()),
        Some("position_complete_check")
    );

    let successor = commit_raw_position_successor(
        &pool,
        user_id,
        lineage_character.entity.id,
        place.entity.id,
        destination_id,
        connection_id,
        character_root_activity,
        100,
        true,
    )
    .await
    .unwrap();
    let current: Uuid =
        sqlx::query_scalar("SELECT current_activity_id FROM position WHERE entity_id = $1")
            .bind(lineage_character.entity.id.0)
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(current, successor.0);
    let pointer_backtrack =
        sqlx::query("UPDATE position SET current_activity_id = $2 WHERE entity_id = $1")
            .bind(lineage_character.entity.id.0)
            .bind(character_root_activity.0)
            .execute(&pool)
            .await
            .unwrap_err();
    assert_eq!(
        pointer_backtrack
            .as_database_error()
            .and_then(|error| error.constraint()),
        Some("position_current_advance_check")
    );

    let backtrack = commit_raw_position_successor(
        &pool,
        user_id,
        lineage_character.entity.id,
        place.entity.id,
        destination_id,
        connection_id,
        character_root_activity,
        100,
        true,
    )
    .await
    .unwrap_err();
    assert_eq!(
        backtrack
            .as_database_error()
            .and_then(|error| error.constraint()),
        Some("position_predecessor_current_check")
    );

    let cycle_activity = Uuid::new_v4();
    let mut cycle_transaction = pool.begin().await.unwrap();
    sqlx::query(
        "INSERT INTO activity (id, operation, requested_by_user_id, actor_character_entity_id, request_id, request_fingerprint) VALUES ($1, 'move_character', $2, $3, $4, $5)",
    )
    .bind(cycle_activity)
    .bind(user_id.0)
    .bind(lineage_character.entity.id.0)
    .bind(Uuid::new_v4())
    .bind(vec![9_u8; 32])
    .execute(&mut *cycle_transaction)
    .await
    .unwrap();
    let cycle = sqlx::query(
        "INSERT INTO position_version (entity_id, activity_id, previous_activity_id, x_cm, y_cm, z_cm) VALUES ($1, $2, $2, 100, 0, 0)",
    )
    .bind(lineage_character.entity.id.0)
    .bind(cycle_activity)
    .execute(&mut *cycle_transaction)
    .await
    .unwrap_err();
    assert_eq!(
        cycle
            .as_database_error()
            .and_then(|error| error.constraint()),
        Some("position_cycle_check")
    );
    cycle_transaction.rollback().await.unwrap();

    let first_cycle_activity = Uuid::new_v4();
    let second_cycle_activity = Uuid::new_v4();
    let mut disconnected_cycle = pool.begin().await.unwrap();
    sqlx::query(
        r#"
        INSERT INTO activity (
            id, operation, requested_by_user_id, actor_character_entity_id,
            request_id, request_fingerprint
        ) VALUES
            ($1, 'move_character', $3, $4, $5, $7),
            ($2, 'move_character', $3, $4, $6, $7)
        "#,
    )
    .bind(first_cycle_activity)
    .bind(second_cycle_activity)
    .bind(user_id.0)
    .bind(lineage_character.entity.id.0)
    .bind(Uuid::new_v4())
    .bind(Uuid::new_v4())
    .bind(vec![12_u8; 32])
    .execute(&mut *disconnected_cycle)
    .await
    .unwrap();
    let disconnected_cycle_error = sqlx::query(
        r#"
        INSERT INTO position_version (
            entity_id, activity_id, previous_activity_id, x_cm, y_cm, z_cm
        ) VALUES
            ($1, $2, $3, 100, 0, 0),
            ($1, $3, $2, 100, 0, 0)
        "#,
    )
    .bind(lineage_character.entity.id.0)
    .bind(first_cycle_activity)
    .bind(second_cycle_activity)
    .execute(&mut *disconnected_cycle)
    .await
    .unwrap_err();
    assert_eq!(
        disconnected_cycle_error
            .as_database_error()
            .and_then(|error| error.constraint()),
        Some("position_predecessor_current_check")
    );
    disconnected_cycle.rollback().await.unwrap();

    let concurrent_user_id = create_user(&world).await;
    world
        .create_character(concurrent_user_id, character("Concurrent Lineage Actor"))
        .await
        .unwrap();
    let concurrent_character = world.enter_world(concurrent_user_id).await.unwrap();
    let concurrent_root = concurrent_character
        .position
        .as_ref()
        .unwrap()
        .position_revision
        .activity_id();
    let first_race = commit_raw_position_successor(
        &pool,
        concurrent_user_id,
        concurrent_character.entity.id,
        place.entity.id,
        destination_id,
        connection_id,
        concurrent_root,
        100,
        true,
    );
    let second_race = commit_raw_position_successor(
        &pool,
        concurrent_user_id,
        concurrent_character.entity.id,
        place.entity.id,
        destination_id,
        connection_id,
        concurrent_root,
        100,
        true,
    );
    let (first_race, second_race) = tokio::join!(first_race, second_race);
    assert_eq!(
        usize::from(first_race.is_ok()) + usize::from(second_race.is_ok()),
        1
    );
    let race_error = first_race.err().or_else(|| second_race.err()).unwrap();
    assert!(matches!(
        race_error
            .as_database_error()
            .and_then(|error| error.constraint()),
        Some("position_predecessor_current_check") | Some("position_version_one_successor_index")
    ));
    let successor_count: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM position_version WHERE entity_id = $1 AND previous_activity_id = $2",
    )
    .bind(concurrent_character.entity.id.0)
    .bind(concurrent_root.0)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(successor_count, 1);
}

async fn raw_positioned_place(
    pool: &PgPool,
    user_id: UserId,
    actor_entity_id: EntityId,
    context_place_id: EntityId,
    x_cm: i64,
    y_cm: i64,
    z_cm: i64,
) -> (EntityId, ActivityId) {
    let entity_id = EntityId(Uuid::new_v4());
    let activity_id = ActivityId(Uuid::new_v4());
    let mut transaction = pool.begin().await.unwrap();
    sqlx::query("INSERT INTO entity (id, name, description, introduced_by_user_id) VALUES ($1, 'Raw Destination', 'A test-only positioned Place.', $2)")
        .bind(entity_id.0).bind(user_id.0).execute(&mut *transaction).await.unwrap();
    sqlx::query(
        r#"
        INSERT INTO activity (
            id, operation, requested_by_user_id, actor_character_entity_id,
            context_place_entity_id, prose, request_id, request_fingerprint
        ) VALUES ($1, 'submit_discovery', $2, $3, $4, 'A test destination is established.', $5, $6)
        "#,
    )
    .bind(activity_id.0)
    .bind(user_id.0)
    .bind(actor_entity_id.0)
    .bind(context_place_id.0)
    .bind(Uuid::new_v4())
    .bind(vec![3_u8; 32])
    .execute(&mut *transaction)
    .await
    .unwrap();
    sqlx::query("INSERT INTO activity_entity (activity_id, entity_id, role) VALUES ($1, $2, 'subject'), ($1, $2, 'destination')")
        .bind(activity_id.0).bind(entity_id.0).execute(&mut *transaction).await.unwrap();
    sqlx::query("INSERT INTO position_version (entity_id, activity_id, x_cm, y_cm, z_cm) VALUES ($1, $2, $3, $4, $5)")
        .bind(entity_id.0).bind(activity_id.0).bind(x_cm).bind(y_cm).bind(z_cm)
        .execute(&mut *transaction).await.unwrap();
    sqlx::query("INSERT INTO position (entity_id, current_activity_id) VALUES ($1, $2)")
        .bind(entity_id.0)
        .bind(activity_id.0)
        .execute(&mut *transaction)
        .await
        .unwrap();
    sqlx::query("INSERT INTO activity_position (activity_id, role, position_entity_id, position_activity_id) VALUES ($1, 'result', $2, $1)")
        .bind(activity_id.0).bind(entity_id.0).execute(&mut *transaction).await.unwrap();
    sqlx::query(
        "INSERT INTO place (entity_id, is_entry, latest_activity_id) VALUES ($1, false, $2)",
    )
    .bind(entity_id.0)
    .bind(activity_id.0)
    .execute(&mut *transaction)
    .await
    .unwrap();
    sqlx::query("INSERT INTO place_map_index (place_entity_id, position_activity_id, x_cm, y_cm, z_cm) VALUES ($1, $2, $3, $4, $5)")
        .bind(entity_id.0).bind(activity_id.0).bind(x_cm).bind(y_cm).bind(z_cm)
        .execute(&mut *transaction).await.unwrap();
    transaction.commit().await.unwrap();
    (entity_id, activity_id)
}

async fn raw_connection_activity(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    user_id: UserId,
    actor_entity_id: EntityId,
    context_place_id: EntityId,
) -> ActivityId {
    let activity_id = ActivityId(Uuid::new_v4());
    sqlx::query(
        r#"
        INSERT INTO activity (
            id, operation, requested_by_user_id, actor_character_entity_id,
            context_place_entity_id, prose, request_id, request_fingerprint
        ) VALUES ($1, 'submit_discovery', $2, $3, $4, 'A direct Connection is established.', $5, $6)
        "#,
    )
    .bind(activity_id.0)
    .bind(user_id.0)
    .bind(actor_entity_id.0)
    .bind(context_place_id.0)
    .bind(Uuid::new_v4())
    .bind(vec![4_u8; 32])
    .execute(&mut **transaction)
    .await
    .unwrap();
    activity_id
}

async fn insert_raw_connection(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    activity_id: ActivityId,
    source: (EntityId, ActivityId),
    destination: (EntityId, ActivityId),
    point: &[(i16, i64, i64, i64)],
) -> ConnectionId {
    let connection_id = ConnectionId(Uuid::new_v4());
    sqlx::query(
        r#"
        INSERT INTO connection (
            id, source_place_entity_id, destination_place_entity_id,
            source_position_activity_id, destination_position_activity_id,
            allows_reverse, has_course, name, description, shape_description,
            created_by_activity_id
        ) VALUES ($1, $2, $3, $4, $5, true, $6, 'Test Crossing',
                  'A test-only direct alternative.', NULL, $7)
        "#,
    )
    .bind(connection_id.0)
    .bind(source.0.0)
    .bind(destination.0.0)
    .bind(source.1.0)
    .bind(destination.1.0)
    .bind(!point.is_empty())
    .bind(activity_id.0)
    .execute(&mut **transaction)
    .await
    .unwrap();
    for (ordinal, x_cm, y_cm, z_cm) in point {
        sqlx::query("INSERT INTO connection_point (connection_id, ordinal, x_cm, y_cm, z_cm) VALUES ($1, $2, $3, $4, $5)")
            .bind(connection_id.0).bind(*ordinal).bind(*x_cm).bind(*y_cm).bind(*z_cm)
            .execute(&mut **transaction).await.unwrap();
    }
    sqlx::query("INSERT INTO activity_connection (activity_id, connection_id) VALUES ($1, $2)")
        .bind(activity_id.0)
        .bind(connection_id.0)
        .execute(&mut **transaction)
        .await
        .unwrap();
    connection_id
}

#[allow(clippy::too_many_arguments)]
async fn commit_raw_position_successor(
    pool: &PgPool,
    user_id: UserId,
    character_id: EntityId,
    source_place_id: EntityId,
    destination_place_id: EntityId,
    connection_id: ConnectionId,
    previous_activity_id: ActivityId,
    x_cm: i64,
    include_result: bool,
) -> Result<ActivityId, sqlx::Error> {
    let activity_id = ActivityId(Uuid::new_v4());
    let mut transaction = pool.begin().await?;
    sqlx::query(
        r#"
        INSERT INTO activity (
            id, operation, requested_by_user_id, actor_character_entity_id,
            context_place_entity_id, request_id, request_fingerprint
        ) VALUES ($1, 'move_character', $2, $3, $4, $5, $6)
        "#,
    )
    .bind(activity_id.0)
    .bind(user_id.0)
    .bind(character_id.0)
    .bind(destination_place_id.0)
    .bind(Uuid::new_v4())
    .bind(vec![8_u8; 32])
    .execute(&mut *transaction)
    .await?;
    sqlx::query(
        "INSERT INTO activity_entity (activity_id, entity_id, role) VALUES ($1, $2, 'location'), ($1, $3, 'destination')",
    )
    .bind(activity_id.0)
    .bind(source_place_id.0)
    .bind(destination_place_id.0)
    .execute(&mut *transaction)
    .await?;
    sqlx::query("INSERT INTO activity_connection (activity_id, connection_id) VALUES ($1, $2)")
        .bind(activity_id.0)
        .bind(connection_id.0)
        .execute(&mut *transaction)
        .await?;
    sqlx::query(
        "INSERT INTO activity_position (activity_id, role, position_entity_id, position_activity_id) VALUES ($1, 'origin', $2, $3)",
    )
    .bind(activity_id.0)
    .bind(character_id.0)
    .bind(previous_activity_id.0)
    .execute(&mut *transaction)
    .await?;
    sqlx::query(
        "INSERT INTO position_version (entity_id, activity_id, previous_activity_id, x_cm, y_cm, z_cm) VALUES ($1, $2, $3, $4, 0, 0)",
    )
    .bind(character_id.0)
    .bind(activity_id.0)
    .bind(previous_activity_id.0)
    .bind(x_cm)
    .execute(&mut *transaction)
    .await?;
    sqlx::query("UPDATE position SET current_activity_id = $2 WHERE entity_id = $1")
        .bind(character_id.0)
        .bind(activity_id.0)
        .execute(&mut *transaction)
        .await?;
    sqlx::query("UPDATE character SET current_place_entity_id = $2 WHERE entity_id = $1")
        .bind(character_id.0)
        .bind(destination_place_id.0)
        .execute(&mut *transaction)
        .await?;
    if include_result {
        sqlx::query(
            "INSERT INTO activity_position (activity_id, role, position_entity_id, position_activity_id) VALUES ($1, 'result', $2, $1)",
        )
        .bind(activity_id.0)
        .bind(character_id.0)
        .execute(&mut *transaction)
        .await?;
    }
    transaction.commit().await?;
    Ok(activity_id)
}

#[sqlx::test(migrations = "./migration")]
async fn connection_storage_keeps_identity_course_and_history_strict(pool: PgPool) {
    let world = World::new(pool.clone());
    let user_id = create_user(&world).await;
    let character = world
        .create_character(user_id, character("Connection Actor"))
        .await
        .unwrap();
    let source = world
        .create_entry_place(user_id, place("Connection Source"))
        .await
        .unwrap();
    world.enter_world(user_id).await.unwrap();
    let (destination_id, destination_activity_id) = raw_positioned_place(
        &pool,
        user_id,
        character.entity.id,
        source.entity.id,
        100,
        0,
        0,
    )
    .await;
    let source_key = (
        source.entity.id,
        source.position.position_revision.activity_id(),
    );
    let destination_key = (destination_id, destination_activity_id);

    let mut first_transaction = pool.begin().await.unwrap();
    let first_activity = raw_connection_activity(
        &mut first_transaction,
        user_id,
        character.entity.id,
        source.entity.id,
    )
    .await;
    let first = insert_raw_connection(
        &mut first_transaction,
        first_activity,
        source_key,
        destination_key,
        &[(0, 0, 0, 0), (1, 100, 0, 0)],
    )
    .await;
    first_transaction.commit().await.unwrap();

    let mut second_transaction = pool.begin().await.unwrap();
    let second_activity = raw_connection_activity(
        &mut second_transaction,
        user_id,
        character.entity.id,
        source.entity.id,
    )
    .await;
    let second = insert_raw_connection(
        &mut second_transaction,
        second_activity,
        source_key,
        destination_key,
        &[(0, 0, 0, 0), (1, 100, 0, 0)],
    )
    .await;
    second_transaction.commit().await.unwrap();
    assert_ne!(first, second);

    let mut unshaped_transaction = pool.begin().await.unwrap();
    let unshaped_activity = raw_connection_activity(
        &mut unshaped_transaction,
        user_id,
        character.entity.id,
        source.entity.id,
    )
    .await;
    let unshaped = insert_raw_connection(
        &mut unshaped_transaction,
        unshaped_activity,
        source_key,
        destination_key,
        &[],
    )
    .await;
    unshaped_transaction.commit().await.unwrap();
    assert_ne!(first, unshaped);
    let equal_count: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM connection WHERE source_place_entity_id = $1 AND destination_place_entity_id = $2 AND name = 'Test Crossing'",
    ).bind(source.entity.id.0).bind(destination_id.0).fetch_one(&pool).await.unwrap();
    assert_eq!(equal_count, 3);
    let shaped_row: (
        Uuid,
        Uuid,
        Uuid,
        Uuid,
        Uuid,
        bool,
        bool,
        String,
        String,
        Option<String>,
        Uuid,
    ) = sqlx::query_as(
        r#"
        SELECT id, source_place_entity_id, destination_place_entity_id,
               source_position_activity_id, destination_position_activity_id,
               allows_reverse, has_course, name, description, shape_description,
               created_by_activity_id
        FROM connection WHERE id = $1
        "#,
    )
    .bind(first.0)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(shaped_row.0, first.0);
    assert_eq!(shaped_row.1, source.entity.id.0);
    assert_eq!(shaped_row.2, destination_id.0);
    assert_eq!(
        (shaped_row.3, shaped_row.4),
        (
            source.position.position_revision.activity_id().0,
            destination_activity_id.0
        )
    );
    assert_eq!((shaped_row.5, shaped_row.6), (true, true));
    assert_eq!(shaped_row.7, "Test Crossing");
    assert_eq!(shaped_row.8, "A test-only direct alternative.");
    assert_eq!(shaped_row.9, None);
    assert_eq!(shaped_row.10, first_activity.0);
    let shaped_course: Vec<(i16, i64, i64, i64)> = sqlx::query_as(
        "SELECT ordinal, x_cm, y_cm, z_cm FROM connection_point WHERE connection_id = $1 ORDER BY ordinal",
    )
    .bind(first.0)
    .fetch_all(&pool)
    .await
    .unwrap();
    assert_eq!(shaped_course, vec![(0, 0, 0, 0), (1, 100, 0, 0)]);
    let unshaped_readback: (bool, i64) = sqlx::query_as(
        "SELECT has_course, (SELECT count(*) FROM connection_point WHERE connection_id = connection.id) FROM connection WHERE id = $1",
    )
    .bind(unshaped.0)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(unshaped_readback, (false, 0));

    let mut late_course = pool.begin().await.unwrap();
    sqlx::query(
        "INSERT INTO connection_point (connection_id, ordinal, x_cm, y_cm, z_cm) VALUES ($1, 0, 0, 0, 0), ($1, 1, 100, 0, 0)",
    )
    .bind(unshaped.0)
    .execute(&mut *late_course)
    .await
    .unwrap();
    let late_course = late_course.commit().await.unwrap_err();
    assert_eq!(
        late_course
            .as_database_error()
            .and_then(|error| error.constraint()),
        Some("connection_complete_check")
    );
    assert!(
        sqlx::query("UPDATE connection SET name = 'Changed' WHERE id = $1")
            .bind(first.0)
            .execute(&pool)
            .await
            .is_err()
    );
    assert!(
        sqlx::query("DELETE FROM connection_point WHERE connection_id = $1 AND ordinal = 1")
            .bind(first.0)
            .execute(&pool)
            .await
            .is_err()
    );
    assert!(
        sqlx::query("DELETE FROM activity_connection WHERE connection_id = $1")
            .bind(first.0)
            .execute(&pool)
            .await
            .is_err()
    );

    for (label, points) in [
        ("one point", vec![(0, 0, 0, 0)]),
        ("ordinal gap", vec![(0, 0, 0, 0), (2, 100, 0, 0)]),
        ("wrong endpoint", vec![(0, 1, 0, 0), (1, 100, 0, 0)]),
        ("duplicate consecutive", vec![(0, 0, 0, 0), (1, 0, 0, 0)]),
        (
            "self intersection",
            vec![
                (0, 0, 0, 0),
                (1, 100, 100, 0),
                (2, 0, 100, 0),
                (3, 100, 0, 0),
            ],
        ),
    ] {
        let mut transaction = pool.begin().await.unwrap();
        let activity = raw_connection_activity(
            &mut transaction,
            user_id,
            character.entity.id,
            source.entity.id,
        )
        .await;
        insert_raw_connection(
            &mut transaction,
            activity,
            source_key,
            destination_key,
            &points,
        )
        .await;
        let error = transaction.commit().await.expect_err(label);
        assert_eq!(
            error
                .as_database_error()
                .and_then(|error| error.constraint()),
            Some("connection_complete_check"),
            "unexpected constraint for {label}: {error:?}"
        );
    }

    let (overlap_destination_id, overlap_destination_activity_id) = raw_positioned_place(
        &pool,
        user_id,
        character.entity.id,
        source.entity.id,
        50,
        0,
        0,
    )
    .await;
    let mut overlap_transaction = pool.begin().await.unwrap();
    let overlap_activity = raw_connection_activity(
        &mut overlap_transaction,
        user_id,
        character.entity.id,
        source.entity.id,
    )
    .await;
    insert_raw_connection(
        &mut overlap_transaction,
        overlap_activity,
        source_key,
        (overlap_destination_id, overlap_destination_activity_id),
        &[(0, 0, 0, 0), (1, 100, 0, 0), (2, 50, 0, 0)],
    )
    .await;
    let overlap_error = overlap_transaction.commit().await.unwrap_err();
    assert_eq!(
        overlap_error
            .as_database_error()
            .and_then(|error| error.constraint()),
        Some("connection_complete_check")
    );

    let mut invalid_endpoint = pool.begin().await.unwrap();
    let activity = raw_connection_activity(
        &mut invalid_endpoint,
        user_id,
        character.entity.id,
        source.entity.id,
    )
    .await;
    let error = sqlx::query(
        r#"
        INSERT INTO connection (
            id, source_place_entity_id, destination_place_entity_id,
            source_position_activity_id, destination_position_activity_id,
            allows_reverse, has_course, name, description, created_by_activity_id
        ) VALUES ($1, $2, $2, $3, $3, false, false, 'Loop', 'Invalid same endpoint.', $4)
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(source.entity.id.0)
    .bind(source.position.position_revision.activity_id().0)
    .bind(activity.0)
    .execute(&mut *invalid_endpoint)
    .await
    .unwrap_err();
    assert_eq!(
        error
            .as_database_error()
            .and_then(|error| error.constraint()),
        Some("connection_distinct_endpoint_check")
    );
    invalid_endpoint.rollback().await.unwrap();

    let mut wrong_creator = pool.begin().await.unwrap();
    let wrong_creator_activity = ActivityId(Uuid::new_v4());
    sqlx::query(
        "INSERT INTO activity (id, operation, requested_by_user_id, actor_character_entity_id, request_id, request_fingerprint) VALUES ($1, 'move_character', $2, $3, $4, $5)",
    )
    .bind(wrong_creator_activity.0)
    .bind(user_id.0)
    .bind(character.entity.id.0)
    .bind(Uuid::new_v4())
    .bind(vec![10_u8; 32])
    .execute(&mut *wrong_creator)
    .await
    .unwrap();
    let wrong_creator_error = sqlx::query(
        r#"
        INSERT INTO connection (
            id, source_place_entity_id, destination_place_entity_id,
            source_position_activity_id, destination_position_activity_id,
            allows_reverse, has_course, name, description, created_by_activity_id
        ) VALUES ($1, $2, $3, $4, $5, false, false, 'Wrong creator',
                  'A Movement cannot create a Connection.', $6)
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(source.entity.id.0)
    .bind(destination_id.0)
    .bind(source.position.position_revision.activity_id().0)
    .bind(destination_activity_id.0)
    .bind(wrong_creator_activity.0)
    .execute(&mut *wrong_creator)
    .await
    .unwrap_err();
    assert_eq!(
        wrong_creator_error
            .as_database_error()
            .and_then(|error| error.constraint()),
        Some("connection_creator_operation_check")
    );
    wrong_creator.rollback().await.unwrap();

    let mut wrong_discovery_link = pool.begin().await.unwrap();
    let unrelated_discovery = raw_connection_activity(
        &mut wrong_discovery_link,
        user_id,
        character.entity.id,
        source.entity.id,
    )
    .await;
    let wrong_discovery_link_error =
        sqlx::query("INSERT INTO activity_connection (activity_id, connection_id) VALUES ($1, $2)")
            .bind(unrelated_discovery.0)
            .bind(first.0)
            .execute(&mut *wrong_discovery_link)
            .await
            .unwrap_err();
    assert_eq!(
        wrong_discovery_link_error
            .as_database_error()
            .and_then(|error| error.constraint()),
        Some("activity_connection_discovery_owner_check")
    );
    wrong_discovery_link.rollback().await.unwrap();

    let mut wrong_link_operation = pool.begin().await.unwrap();
    let create_entity_activity = Uuid::new_v4();
    sqlx::query(
        "INSERT INTO activity (id, operation, requested_by_user_id) VALUES ($1, 'create_entity', $2)",
    )
    .bind(create_entity_activity)
    .bind(user_id.0)
    .execute(&mut *wrong_link_operation)
    .await
    .unwrap();
    let wrong_link_operation_error =
        sqlx::query("INSERT INTO activity_connection (activity_id, connection_id) VALUES ($1, $2)")
            .bind(create_entity_activity)
            .bind(first.0)
            .execute(&mut *wrong_link_operation)
            .await
            .unwrap_err();
    assert_eq!(
        wrong_link_operation_error
            .as_database_error()
            .and_then(|error| error.constraint()),
        Some("activity_connection_operation_check")
    );
    wrong_link_operation.rollback().await.unwrap();

    let mut missing_move_link = pool.begin().await.unwrap();
    sqlx::query(
        "INSERT INTO activity (id, operation, requested_by_user_id, actor_character_entity_id, request_id, request_fingerprint) VALUES ($1, 'move_character', $2, $3, $4, $5)",
    )
    .bind(Uuid::new_v4())
    .bind(user_id.0)
    .bind(character.entity.id.0)
    .bind(Uuid::new_v4())
    .bind(vec![13_u8; 32])
    .execute(&mut *missing_move_link)
    .await
    .unwrap();
    let missing_move_link_error = missing_move_link.commit().await.unwrap_err();
    assert_eq!(
        missing_move_link_error
            .as_database_error()
            .and_then(|error| error.constraint()),
        Some("activity_connection_move_required_check")
    );

    let mut second_link = pool.begin().await.unwrap();
    let movement_activity = ActivityId(Uuid::new_v4());
    sqlx::query(
        "INSERT INTO activity (id, operation, requested_by_user_id, actor_character_entity_id, request_id, request_fingerprint) VALUES ($1, 'move_character', $2, $3, $4, $5)",
    )
    .bind(movement_activity.0)
    .bind(user_id.0)
    .bind(character.entity.id.0)
    .bind(Uuid::new_v4())
    .bind(vec![11_u8; 32])
    .execute(&mut *second_link)
    .await
    .unwrap();
    sqlx::query("INSERT INTO activity_connection (activity_id, connection_id) VALUES ($1, $2)")
        .bind(movement_activity.0)
        .bind(first.0)
        .execute(&mut *second_link)
        .await
        .unwrap();
    let second_link_error =
        sqlx::query("INSERT INTO activity_connection (activity_id, connection_id) VALUES ($1, $2)")
            .bind(movement_activity.0)
            .bind(second.0)
            .execute(&mut *second_link)
            .await
            .unwrap_err();
    assert_eq!(
        second_link_error
            .as_database_error()
            .and_then(|error| error.constraint()),
        Some("activity_connection_pkey")
    );
    second_link.rollback().await.unwrap();

    let activity_connection_indexes: Vec<String> = sqlx::query_scalar(
        "SELECT indexname FROM pg_indexes WHERE schemaname = 'public' AND tablename = 'activity_connection' ORDER BY indexname",
    )
    .fetch_all(&pool)
    .await
    .unwrap();
    assert_eq!(
        activity_connection_indexes,
        vec![
            "activity_connection_connection_activity_index".to_owned(),
            "activity_connection_pkey".to_owned(),
        ]
    );
}
