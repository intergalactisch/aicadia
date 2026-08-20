use super::*;

fn add_fingerprint_field(hash: &mut Sha256, field: &[u8]) {
    hash.update((field.len() as u64).to_be_bytes());
    hash.update(field);
}

fn legacy_action_fingerprint(
    revision: PlaceRevision,
    prose: &str,
    discriminator: &[u8],
    entity_id: EntityId,
    value: Option<(&str, &PropertyValue)>,
    statement: Option<&str>,
) -> Vec<u8> {
    let mut revision_bytes = Vec::with_capacity(41);
    revision_bytes.push(1);
    revision_bytes.extend_from_slice(revision.place_entity_id().0.as_bytes());
    revision_bytes.extend_from_slice(&revision.occurred_at().timestamp_micros().to_be_bytes());
    revision_bytes.extend_from_slice(revision.activity_id().0.as_bytes());
    let mut hash = Sha256::new();
    for field in [
        b"aicadia-submit-action-fingerprint-v1".as_slice(),
        revision_bytes.as_slice(),
        prose.as_bytes(),
        discriminator,
    ] {
        add_fingerprint_field(&mut hash, field);
    }
    if let Some((key, value)) = value {
        add_fingerprint_field(&mut hash, entity_id.0.as_bytes());
        add_fingerprint_field(&mut hash, key.as_bytes());
        match value {
            PropertyValue::Text(value) => {
                add_fingerprint_field(&mut hash, b"text");
                add_fingerprint_field(&mut hash, value.as_bytes());
            }
            PropertyValue::Integer(value) => {
                add_fingerprint_field(&mut hash, b"integer");
                add_fingerprint_field(&mut hash, &value.to_be_bytes());
            }
        }
    }
    if let Some(statement) = statement {
        add_fingerprint_field(&mut hash, b"establish");
        add_fingerprint_field(&mut hash, entity_id.0.as_bytes());
        add_fingerprint_field(&mut hash, statement.as_bytes());
    }
    hash.finalize().to_vec()
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
        AcceptedActionConsequence::ChangeEntityState { .. } => {
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
                    position_description: None,
                    property: Vec::new(),
                    r#trait: Vec::new(),
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
                    position_description: None,
                    property: Vec::new(),
                    r#trait: Vec::new(),
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
async fn current_single_kind_retries_decode_historical_action_rows_without_legacy_input(
    pool: PgPool,
) {
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

    let property_revision = world
        .list_entity_at_current_place(user_id, ListEntityAtCurrentPlace::default())
        .await
        .unwrap()
        .place_revision;
    let property_request_id = Uuid::new_v4();
    let property_activity_id = Uuid::new_v4();
    let property_value = PropertyValue::Integer(3);
    let property_fingerprint = legacy_action_fingerprint(
        property_revision,
        "Mara records three legs.",
        b"change_entity_property",
        character.entity.id,
        Some(("leg_count", &property_value)),
        None,
    );
    sqlx::query(
        r#"
        INSERT INTO activity (
            id, operation, requested_by_user_id,
            actor_character_entity_id, context_place_entity_id,
            prose, request_id, request_fingerprint, action_consequence
        )
        VALUES ($1, 'submit_action', $2, $3, $4, $5, $6, $7, 'change_entity_property')
        "#,
    )
    .bind(property_activity_id)
    .bind(user_id.0)
    .bind(character.entity.id.0)
    .bind(place.entity.id.0)
    .bind("Mara records three legs.")
    .bind(property_request_id)
    .bind(&property_fingerprint)
    .execute(&pool)
    .await
    .unwrap();
    for (entity_id, role) in [
        (character.entity.id, "subject"),
        (place.entity.id, "location"),
    ] {
        sqlx::query(
            "INSERT INTO activity_entity (activity_id, entity_id, role) VALUES ($1, $2, $3)",
        )
        .bind(property_activity_id)
        .bind(entity_id.0)
        .bind(role)
        .execute(&pool)
        .await
        .unwrap();
    }
    let property_key_id: i64 = sqlx::query_scalar(
        "INSERT INTO property_key (key, value_type, first_activity_id) VALUES ('leg_count', 'integer', $1) RETURNING id",
    )
    .bind(property_activity_id)
    .fetch_one(&pool)
    .await
    .unwrap();
    sqlx::query(
        r#"
        INSERT INTO entity_property_history (
            entity_id, property_key_id, activity_id, value_type, integer_value
        ) VALUES ($1, $2, $3, 'integer', 3)
        "#,
    )
    .bind(character.entity.id.0)
    .bind(property_key_id)
    .bind(property_activity_id)
    .execute(&pool)
    .await
    .unwrap();
    sqlx::query(
        "INSERT INTO entity_property (entity_id, property_key_id, current_activity_id) VALUES ($1, $2, $3)",
    )
    .bind(character.entity.id.0)
    .bind(property_key_id)
    .bind(property_activity_id)
    .execute(&pool)
    .await
    .unwrap();
    sqlx::query("UPDATE place SET latest_activity_id = $1 WHERE entity_id = $2")
        .bind(property_activity_id)
        .bind(place.entity.id.0)
        .execute(&pool)
        .await
        .unwrap();

    let property_retry = world
        .submit_action(
            user_id,
            property_action(
                property_request_id,
                property_revision,
                "Mara records three legs.",
                vec![property_change(
                    character.entity.id,
                    "leg_count",
                    property_value,
                )],
            ),
        )
        .await
        .unwrap();
    assert_eq!(property_retry.activity.id.0, property_activity_id);
    assert!(matches!(
        property_retry.consequence,
        AcceptedActionConsequence::ChangeEntityState {
            ref property_change,
            ref trait_change,
        } if property_change.len() == 1 && trait_change.is_empty()
    ));

    let trait_revision = world
        .list_entity_at_current_place(user_id, ListEntityAtCurrentPlace::default())
        .await
        .unwrap()
        .place_revision;
    let trait_request_id = Uuid::new_v4();
    let trait_activity_id = Uuid::new_v4();
    let trait_id = Uuid::new_v4();
    let statement = "Jumps unusually high.";
    let trait_fingerprint = legacy_action_fingerprint(
        trait_revision,
        "Mara records one lasting characterization.",
        b"change_entity_trait",
        character.entity.id,
        None,
        Some(statement),
    );
    sqlx::query(
        r#"
        INSERT INTO activity (
            id, operation, requested_by_user_id,
            actor_character_entity_id, context_place_entity_id,
            prose, request_id, request_fingerprint, action_consequence
        )
        VALUES ($1, 'submit_action', $2, $3, $4, $5, $6, $7, 'change_entity_trait')
        "#,
    )
    .bind(trait_activity_id)
    .bind(user_id.0)
    .bind(character.entity.id.0)
    .bind(place.entity.id.0)
    .bind("Mara records one lasting characterization.")
    .bind(trait_request_id)
    .bind(&trait_fingerprint)
    .execute(&pool)
    .await
    .unwrap();
    for (entity_id, role) in [
        (character.entity.id, "subject"),
        (place.entity.id, "location"),
    ] {
        sqlx::query(
            "INSERT INTO activity_entity (activity_id, entity_id, role) VALUES ($1, $2, $3)",
        )
        .bind(trait_activity_id)
        .bind(entity_id.0)
        .bind(role)
        .execute(&pool)
        .await
        .unwrap();
    }
    sqlx::raw_sql(
        "ALTER TABLE entity_trait_version DISABLE TRIGGER entity_trait_version_activity_check",
    )
    .execute(&pool)
    .await
    .unwrap();
    let mut historical_trait = pool.begin().await.unwrap();
    sqlx::query("INSERT INTO entity_trait (id, entity_id) VALUES ($1, $2)")
        .bind(trait_id)
        .bind(character.entity.id.0)
        .execute(&mut *historical_trait)
        .await
        .unwrap();
    sqlx::query(
        "INSERT INTO entity_trait_version (trait_id, entity_id, activity_id, statement) VALUES ($1, $2, $3, $4)",
    )
    .bind(trait_id)
    .bind(character.entity.id.0)
    .bind(trait_activity_id)
    .bind(statement)
    .execute(&mut *historical_trait)
    .await
    .unwrap();
    sqlx::query(
        "INSERT INTO entity_trait_current (trait_id, entity_id, current_activity_id) VALUES ($1, $2, $3)",
    )
    .bind(trait_id)
    .bind(character.entity.id.0)
    .bind(trait_activity_id)
    .execute(&mut *historical_trait)
    .await
    .unwrap();
    historical_trait.commit().await.unwrap();
    sqlx::raw_sql(
        "ALTER TABLE entity_trait_version ENABLE TRIGGER entity_trait_version_activity_check",
    )
    .execute(&pool)
    .await
    .unwrap();
    sqlx::query("UPDATE place SET latest_activity_id = $1 WHERE entity_id = $2")
        .bind(trait_activity_id)
        .bind(place.entity.id.0)
        .execute(&pool)
        .await
        .unwrap();

    let trait_retry = world
        .submit_action(
            user_id,
            trait_action(
                trait_request_id,
                trait_revision,
                "Mara records one lasting characterization.",
                vec![establish_trait(character.entity.id, statement)],
            ),
        )
        .await
        .unwrap();
    assert_eq!(trait_retry.activity.id.0, trait_activity_id);
    assert!(matches!(
        trait_retry.consequence,
        AcceptedActionConsequence::ChangeEntityState {
            ref property_change,
            ref trait_change,
        } if property_change.is_empty() && trait_change.len() == 1
    ));
    assert_eq!(
        sqlx::query_scalar::<_, i64>("SELECT count(*) FROM activity WHERE request_id IN ($1, $2)")
            .bind(property_request_id)
            .bind(trait_request_id)
            .fetch_one(&pool)
            .await
            .unwrap(),
        2
    );
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
                    position_description: None,
                    property: Vec::new(),
                    r#trait: Vec::new(),
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
                    position_description: None,
                    property: Vec::new(),
                    r#trait: Vec::new(),
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
    let mut second_place_setup = pool.begin().await.unwrap();
    sqlx::query(
        "INSERT INTO entity (id, name, description, introduced_by_user_id) VALUES ($1, 'Test South Place', 'Internal isolation fixture', $2)",
    )
    .bind(second_place_id)
    .bind(second_user.0)
    .execute(&mut *second_place_setup)
    .await
    .unwrap();
    sqlx::query(
        "INSERT INTO activity (id, operation, requested_by_user_id, actor_character_entity_id) VALUES ($1, 'create_entry_place', $2, $3)",
    )
    .bind(second_place_activity_id)
    .bind(second_user.0)
    .bind(second_character.entity.id.0)
    .execute(&mut *second_place_setup)
    .await
    .unwrap();
    sqlx::query("INSERT INTO position_version (entity_id, activity_id, x_cm, y_cm, z_cm) VALUES ($1, $2, 100, 0, 0)")
        .bind(second_place_id)
        .bind(second_place_activity_id)
        .execute(&mut *second_place_setup)
        .await
        .unwrap();
    sqlx::query("INSERT INTO position (entity_id, current_activity_id) VALUES ($1, $2)")
        .bind(second_place_id)
        .bind(second_place_activity_id)
        .execute(&mut *second_place_setup)
        .await
        .unwrap();
    sqlx::query("INSERT INTO activity_position (activity_id, role, position_entity_id, position_activity_id) VALUES ($1, 'result', $2, $1)")
        .bind(second_place_activity_id)
        .bind(second_place_id)
        .execute(&mut *second_place_setup)
        .await
        .unwrap();
    sqlx::query(
        "INSERT INTO place (entity_id, is_entry, latest_activity_id) VALUES ($1, false, $2)",
    )
    .bind(second_place_id)
    .bind(second_place_activity_id)
    .execute(&mut *second_place_setup)
    .await
    .unwrap();
    sqlx::query("INSERT INTO place_map_index (place_entity_id, position_activity_id, x_cm, y_cm, z_cm) VALUES ($1, $2, 100, 0, 0)")
        .bind(second_place_id)
        .bind(second_place_activity_id)
        .execute(&mut *second_place_setup)
        .await
        .unwrap();
    sqlx::query(
        "INSERT INTO activity_entity (activity_id, entity_id, role) VALUES ($1, $2, 'subject')",
    )
    .bind(second_place_activity_id)
    .bind(second_place_id)
    .execute(&mut *second_place_setup)
    .await
    .unwrap();
    sqlx::query("UPDATE character SET current_place_entity_id = $1 WHERE owner_user_id = $2")
        .bind(second_place_id)
        .bind(second_user.0)
        .execute(&mut *second_place_setup)
        .await
        .unwrap();
    second_place_setup.commit().await.unwrap();

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
        include_str!("../../migration/0001_world.sql"),
        include_str!("../../migration/0002_rename_app_user.sql"),
        include_str!("../../migration/0003_character.sql"),
        include_str!("../../migration/0004_world_entry_activity.sql"),
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

    sqlx::raw_sql(include_str!("../../migration/0005_agent_action.sql"))
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
