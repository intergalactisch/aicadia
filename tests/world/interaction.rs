use super::*;

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
