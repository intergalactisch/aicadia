use super::attempt::{
    ADMISSION_SQL, MAX_ATTEMPTS_PER_HOUR, MAX_LIVE_POSITIVES, PLACE_WINDOW_DISCOVERY_COUNT_SQL,
    VOID_OLDEST_PRIOR_POSITIVE_SQL,
};
use super::chance::PLACE_ACTIVITY_WINDOW;
use super::*;
use chrono::Duration;
use sqlx::PgPool;

async fn entered_world(pool: PgPool, draw: Vec<f64>) -> (World, UserId, Character, Place) {
    let world = World::with_scripted_chance(pool, draw);
    let user = world.create_user().await.unwrap();
    let character = world
        .create_character(
            user.id,
            CreateCharacter {
                name: "Mara Venn".to_owned(),
                description: "A careful surveyor.".to_owned(),
                property: Vec::new(),
                r#trait: Vec::new(),
            },
        )
        .await
        .unwrap();
    let place = world
        .create_entry_place(
            user.id,
            CreateEntryPlace {
                name: "North Gate".to_owned(),
                description: "A wind-worn stone gate.".to_owned(),
                property: Vec::new(),
                r#trait: Vec::new(),
            },
        )
        .await
        .unwrap();
    world.enter_world(user.id).await.unwrap();
    (world, user.id, character, place)
}

fn discovery(request_id: Uuid, attempt_id: InvestigationAttemptId, prose: &str) -> SubmitDiscovery {
    SubmitDiscovery {
        request_id,
        attempt_id,
        prose: prose.to_owned(),
        find: DiscoveryFind {
            name: "Rainbell Cups".to_owned(),
            description: "Chalk-pale cups whose thin rims ring in rain.".to_owned(),
            property: vec![PropertyInput {
                key: "colour".to_owned(),
                value: PropertyValue::Text(" chalk-pale ".to_owned()),
            }],
            r#trait: vec![TraitInput {
                statement: " Rings softly when collected rain shifts. ".to_owned(),
            }],
        },
    }
}

#[test]
fn discovery_fingerprint_uses_normalized_semantically_unordered_find_state() {
    let attempt_id = InvestigationAttemptId(Uuid::new_v4());
    let request_id = Uuid::new_v4();
    let mut first = discovery(request_id, attempt_id, " A complete find. ");
    first.find.property.push(PropertyInput {
        key: "height".to_owned(),
        value: PropertyValue::Integer(2),
    });
    first.find.r#trait.push(TraitInput {
        statement: "Holds a bead of water.".to_owned(),
    });
    let mut reordered = first.clone();
    reordered.find.property.reverse();
    reordered.find.r#trait.reverse();
    let first = first.normalize().unwrap();
    let reordered = reordered.normalize().unwrap();
    assert_eq!(
        discovery_fingerprint(&first),
        discovery_fingerprint(&reordered)
    );
}

async fn positive(world: &World, user_id: UserId) -> InvestigationResult {
    world
        .start_investigation(
            user_id,
            StartInvestigation {
                request_id: Uuid::new_v4(),
            },
        )
        .await
        .unwrap()
}

#[sqlx::test(migrations = "./migration")]
async fn start_is_retry_stable_across_restart_and_draws_exactly_once(pool: PgPool) {
    let (world, user_id, _, _) = entered_world(pool.clone(), vec![0.0]).await;
    let request_id = Uuid::new_v4();
    let first = world
        .start_investigation(user_id, StartInvestigation { request_id })
        .await
        .unwrap();
    assert_eq!(first.outcome, InvestigationOutcome::Positive);
    assert_eq!(first.limit, InvestigationLimit::CURRENT);

    let restarted = World::with_scripted_chance(pool.clone(), Vec::new());
    let retry = restarted
        .start_investigation(user_id, StartInvestigation { request_id })
        .await
        .unwrap();
    assert_eq!(retry, first);
    let count: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM investigation_attempt WHERE requested_by_user_id = $1",
    )
    .bind(user_id.0)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(count, 1);
}

#[sqlx::test(migrations = "./migration")]
async fn exhausted_scripted_chance_is_unavailable_without_any_write(pool: PgPool) {
    let (world, user_id, _, place) = entered_world(pool.clone(), Vec::new()).await;
    let before: (i64, i64, Uuid) = sqlx::query_as(
        r#"
        SELECT (SELECT count(*) FROM investigation_attempt),
               (SELECT count(*) FROM activity),
               latest_activity_id
        FROM place WHERE entity_id = $1
        "#,
    )
    .bind(place.entity.id.0)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(
        world
            .start_investigation(
                user_id,
                StartInvestigation {
                    request_id: Uuid::new_v4(),
                },
            )
            .await,
        Err(WorldError::Unavailable)
    );
    let after: (i64, i64, Uuid) = sqlx::query_as(
        r#"
        SELECT (SELECT count(*) FROM investigation_attempt),
               (SELECT count(*) FROM activity),
               latest_activity_id
        FROM place WHERE entity_id = $1
        "#,
    )
    .bind(place.entity.id.0)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(after, before);
}

#[sqlx::test(migrations = "./migration")]
async fn discovery_normalization_keeps_prose_and_find_errors_typed_before_writes(pool: PgPool) {
    let world = World::new(pool.clone());
    let user_id = UserId(Uuid::new_v4());
    let attempt_id = InvestigationAttemptId(Uuid::new_v4());

    for (prose, reason) in [
        ("   ".to_owned(), InvalidReason::Empty),
        ("contains\0nul".to_owned(), InvalidReason::ContainsNul),
        ("x".repeat(4_001), InvalidReason::TooLong),
    ] {
        let mut input = discovery(Uuid::new_v4(), attempt_id, &prose);
        input.prose = prose;
        assert_eq!(
            world.submit_discovery(user_id, input).await,
            Err(WorldError::InvalidDiscovery {
                field: DiscoveryField::Prose,
                reason,
            })
        );
    }

    let mut invalid_entity = discovery(Uuid::new_v4(), attempt_id, "Valid discovery prose.");
    invalid_entity.find.name = "   ".to_owned();
    assert_eq!(
        world.submit_discovery(user_id, invalid_entity).await,
        Err(WorldError::InvalidEntity {
            field: EntityField::Name,
            reason: InvalidReason::Empty,
        })
    );

    let mut invalid_property = discovery(Uuid::new_v4(), attempt_id, "Valid discovery prose.");
    invalid_property.find.property[0].key = "Not_Canonical".to_owned();
    assert_eq!(
        world.submit_discovery(user_id, invalid_property).await,
        Err(WorldError::InvalidProperty {
            field: PropertyField::Key,
            reason: InvalidReason::InvalidFormat,
        })
    );

    let mut invalid_trait = discovery(Uuid::new_v4(), attempt_id, "Valid discovery prose.");
    invalid_trait.find.r#trait[0].statement = "   ".to_owned();
    assert_eq!(
        world.submit_discovery(user_id, invalid_trait).await,
        Err(WorldError::InvalidTrait)
    );

    let counts: (i64, i64, i64) = sqlx::query_as(
        r#"
        SELECT (SELECT count(*) FROM investigation_attempt),
               (SELECT count(*) FROM activity),
               (SELECT count(*) FROM entity)
        "#,
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(counts, (0, 0, 0));
}

#[sqlx::test(migrations = "./migration")]
async fn zero_retry_changes_no_activity_or_place_pointer(pool: PgPool) {
    let (world, user_id, _, place) = entered_world(pool.clone(), vec![0.99]).await;
    let before_activity: i64 = sqlx::query_scalar("SELECT count(*) FROM activity")
        .fetch_one(&pool)
        .await
        .unwrap();
    let before_pointer: Uuid =
        sqlx::query_scalar("SELECT latest_activity_id FROM place WHERE entity_id = $1")
            .bind(place.entity.id.0)
            .fetch_one(&pool)
            .await
            .unwrap();
    let request_id = Uuid::new_v4();
    let first = world
        .start_investigation(user_id, StartInvestigation { request_id })
        .await
        .unwrap();
    assert_eq!(first.outcome, InvestigationOutcome::Zero);
    sqlx::query("UPDATE character SET current_place_entity_id = NULL WHERE owner_user_id = $1")
        .bind(user_id.0)
        .execute(&pool)
        .await
        .unwrap();
    assert_eq!(
        world
            .start_investigation(user_id, StartInvestigation { request_id })
            .await
            .unwrap(),
        first
    );
    let after: (i64, Uuid) = sqlx::query_as(
        r#"
        SELECT (SELECT count(*) FROM activity), latest_activity_id
        FROM place WHERE entity_id = $1
        "#,
    )
    .bind(place.entity.id.0)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(after, (before_activity, before_pointer));
}

#[sqlx::test(migrations = "./migration")]
async fn start_admission_is_bounded_and_rejection_does_not_draw_or_insert(pool: PgPool) {
    let (world, user_id, _, _) =
        entered_world(pool.clone(), vec![0.99; MAX_ATTEMPTS_PER_HOUR as usize]).await;
    for _ in 0..MAX_ATTEMPTS_PER_HOUR {
        let result = positive_or_zero(&world, user_id).await.unwrap();
        assert_eq!(result.outcome, InvestigationOutcome::Zero);
    }
    assert_eq!(
        positive_or_zero(&world, user_id).await,
        Err(WorldError::InvestigationNotAdmitted)
    );
    let count: i64 = sqlx::query_scalar("SELECT count(*) FROM investigation_attempt")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(count, MAX_ATTEMPTS_PER_HOUR);
}

#[sqlx::test(migrations = "./migration")]
async fn admission_counts_exactly_the_rolling_hour_before_the_database_clock(pool: PgPool) {
    let (world, user_id, character, place) = entered_world(pool.clone(), vec![0.99]).await;
    let reference: DateTime<Utc> = sqlx::query_scalar("SELECT statement_timestamp()")
        .fetch_one(&pool)
        .await
        .unwrap();
    let boundary = reference - Duration::hours(1);
    let inside = boundary + Duration::seconds(5);
    for _ in 0..MAX_ATTEMPTS_PER_HOUR {
        insert_raw_timed_attempt(&pool, user_id, character.entity.id, place.entity.id, inside)
            .await;
    }
    assert_eq!(
        positive_or_zero(&world, user_id).await,
        Err(WorldError::InvestigationNotAdmitted)
    );

    let boundary_user = world.create_user().await.unwrap();
    let boundary_character = world
        .create_character(
            boundary_user.id,
            CreateCharacter {
                name: "Boundary Iria".to_owned(),
                description: "Tests the far side of admission.".to_owned(),
                property: Vec::new(),
                r#trait: Vec::new(),
            },
        )
        .await
        .unwrap();
    world.enter_world(boundary_user.id).await.unwrap();
    for index in 0..MAX_ATTEMPTS_PER_HOUR {
        let created_at = if index == 0 { boundary } else { inside };
        insert_raw_timed_attempt(
            &pool,
            boundary_user.id,
            boundary_character.entity.id,
            place.entity.id,
            created_at,
        )
        .await;
    }
    assert_eq!(
        positive_or_zero(&world, boundary_user.id)
            .await
            .unwrap()
            .outcome,
        InvestigationOutcome::Zero
    );
}

async fn insert_raw_timed_attempt(
    pool: &PgPool,
    user_id: UserId,
    character_entity_id: EntityId,
    place_entity_id: EntityId,
    created_at: DateTime<Utc>,
) {
    sqlx::query(
        r#"
        INSERT INTO investigation_attempt (
            id, requested_by_user_id, request_id, character_entity_id,
            place_entity_id, outcome, created_at
        ) VALUES ($1, $2, $3, $4, $5, 'zero', $6)
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(user_id.0)
    .bind(Uuid::new_v4())
    .bind(character_entity_id.0)
    .bind(place_entity_id.0)
    .bind(created_at)
    .execute(pool)
    .await
    .unwrap();
}

async fn positive_or_zero(
    world: &World,
    user_id: UserId,
) -> Result<InvestigationResult, WorldError> {
    world
        .start_investigation(
            user_id,
            StartInvestigation {
                request_id: Uuid::new_v4(),
            },
        )
        .await
}

#[sqlx::test(migrations = "./migration")]
async fn fourth_positive_voids_oldest_prior_with_new_provenance(pool: PgPool) {
    let (world, user_id, _, _) = entered_world(pool.clone(), vec![0.0; 4]).await;
    let mut attempt = Vec::new();
    for _ in 0..4 {
        attempt.push(positive(&world, user_id).await.attempt_id);
    }
    let rows: Vec<(Uuid, Option<Uuid>)> = sqlx::query_as(
        r#"
        SELECT id, voided_by_attempt_id
        FROM investigation_attempt
        WHERE requested_by_user_id = $1
        ORDER BY created_at ASC, id ASC
        "#,
    )
    .bind(user_id.0)
    .fetch_all(&pool)
    .await
    .unwrap();
    assert_eq!(rows.len(), 4);
    assert_eq!(rows[0].1, Some(attempt[3].0));
    assert_ne!(rows[0].0, attempt[3].0);
    assert_eq!(rows.iter().filter(|row| row.1.is_none()).count(), 3);
    assert!(rows.iter().all(|(id, voided_by)| *voided_by != Some(*id)));
}

#[sqlx::test(migrations = "./migration")]
async fn fifo_breaks_equal_created_at_ties_by_attempt_id(pool: PgPool) {
    let (world, user_id, character, place) = entered_world(pool.clone(), vec![0.0]).await;
    let created_at: DateTime<Utc> = sqlx::query_scalar("SELECT statement_timestamp()")
        .fetch_one(&pool)
        .await
        .unwrap();
    for id in [Uuid::from_u128(3), Uuid::from_u128(1), Uuid::from_u128(2)] {
        sqlx::query(
            r#"
            INSERT INTO investigation_attempt (
                id, requested_by_user_id, request_id, character_entity_id,
                place_entity_id, outcome, created_at
            ) VALUES ($1, $2, $3, $4, $5, 'positive', $6)
            "#,
        )
        .bind(id)
        .bind(user_id.0)
        .bind(Uuid::new_v4())
        .bind(character.entity.id.0)
        .bind(place.entity.id.0)
        .bind(created_at)
        .execute(&pool)
        .await
        .unwrap();
    }
    let newest = positive(&world, user_id).await;
    let voided_by: Option<Uuid> =
        sqlx::query_scalar("SELECT voided_by_attempt_id FROM investigation_attempt WHERE id = $1")
            .bind(Uuid::from_u128(1))
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(voided_by, Some(newest.attempt_id.0));
}

#[sqlx::test(migrations = "./migration")]
async fn discovery_commits_exact_state_history_consumption_and_retry_precedence(pool: PgPool) {
    let (world, user_id, character, place) = entered_world(pool.clone(), vec![0.0]).await;
    let attempt = positive(&world, user_id).await;
    let request_id = Uuid::new_v4();
    let mut input = discovery(request_id, attempt.attempt_id, " Mara parts the reeds. ");
    input.find.property.push(PropertyInput {
        key: "height".to_owned(),
        value: PropertyValue::Integer(2),
    });
    input.find.r#trait.push(TraitInput {
        statement: "Holds a bead of water.".to_owned(),
    });
    let accepted = world
        .submit_discovery(user_id, input.clone())
        .await
        .unwrap();
    assert_eq!(
        accepted.activity.operation,
        ActivityOperation::SubmitDiscovery
    );
    assert_eq!(
        accepted.activity.actor_character.as_ref().unwrap().id,
        character.entity.id
    );
    assert_eq!(
        accepted.activity.context_place.as_ref().unwrap().entity.id,
        place.entity.id
    );
    assert_eq!(
        accepted.activity.prose.as_deref(),
        Some("Mara parts the reeds.")
    );
    assert_eq!(accepted.activity.property_change.len(), 2);
    assert_eq!(accepted.activity.trait_change.len(), 2);
    assert!(accepted.activity.involved_entity.iter().any(|reference| {
        reference.entity.id == accepted.entity.id && reference.role == ActivityEntityRole::Subject
    }));
    assert!(accepted.activity.involved_entity.iter().any(|reference| {
        reference.entity.id == place.entity.id && reference.role == ActivityEntityRole::Location
    }));
    let consumed: Option<Uuid> = sqlx::query_scalar(
        "SELECT consumed_by_activity_id FROM investigation_attempt WHERE id = $1",
    )
    .bind(attempt.attempt_id.0)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(consumed, Some(accepted.activity.id.0));
    let latest: Uuid =
        sqlx::query_scalar("SELECT latest_activity_id FROM place WHERE entity_id = $1")
            .bind(place.entity.id.0)
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(latest, accepted.activity.id.0);

    let observer = world.create_user().await.unwrap();
    world
        .create_character(
            observer.id,
            CreateCharacter {
                name: "Iria".to_owned(),
                description: "A co-present observer.".to_owned(),
                property: Vec::new(),
                r#trait: Vec::new(),
            },
        )
        .await
        .unwrap();
    world.enter_world(observer.id).await.unwrap();
    let observed = world
        .list_activity_at_current_place(observer.id, ListActivityAtCurrentPlace::default())
        .await
        .unwrap();
    assert!(
        observed
            .activity
            .iter()
            .any(|activity| activity.id == accepted.activity.id)
    );
    let observed_entity = world
        .get_entity_at_current_place(
            observer.id,
            GetEntityAtCurrentPlace {
                entity_id: accepted.entity.id,
                cursor: None,
                limit: 100,
            },
        )
        .await
        .unwrap();
    assert_eq!(observed_entity.entity.id, accepted.entity.id);
    assert_eq!(observed_entity.current_state.association.len(), 4);
    assert_eq!(
        observed_entity
            .current_state
            .association
            .iter()
            .filter(|association| {
                matches!(association, EntityCurrentAssociation::Property { .. })
            })
            .count(),
        2
    );
    assert_eq!(
        observed_entity
            .current_state
            .association
            .iter()
            .filter(|association| matches!(association, EntityCurrentAssociation::Trait(_)))
            .count(),
        2
    );

    sqlx::query("UPDATE character SET current_place_entity_id = NULL WHERE owner_user_id = $1")
        .bind(user_id.0)
        .execute(&pool)
        .await
        .unwrap();
    input.find.property.reverse();
    input.find.r#trait.reverse();
    let retry = world.submit_discovery(user_id, input).await.unwrap();
    assert_eq!(retry, accepted);
    let mut changed = discovery(request_id, attempt.attempt_id, "Different prose.");
    changed.find.name = "Different find".to_owned();
    assert_eq!(
        world.submit_discovery(user_id, changed).await,
        Err(WorldError::DiscoveryRequestConflict)
    );
}

#[sqlx::test(migrations = "./migration")]
async fn activity_request_id_reuse_conflicts_before_current_preconditions(pool: PgPool) {
    let (world, user_id, character, place) = entered_world(pool.clone(), vec![0.0]).await;
    let attempt = positive(&world, user_id).await;
    let revision = world
        .get_character(user_id, GetEntityCurrentState::default())
        .await
        .unwrap()
        .place_revision
        .unwrap();
    let request_id = Uuid::new_v4();
    world
        .submit_action(
            user_id,
            SubmitAction {
                request_id,
                expected_place_revision: revision,
                prose: "Mara places a marker.".to_owned(),
                consequence: ActionConsequence::IntroduceEntity(IntroduceEntity {
                    name: "Conflict marker".to_owned(),
                    description: "A request namespace marker.".to_owned(),
                    property: Vec::new(),
                    r#trait: Vec::new(),
                }),
            },
        )
        .await
        .unwrap();
    sqlx::query("UPDATE character SET current_place_entity_id = NULL WHERE owner_user_id = $1")
        .bind(user_id.0)
        .execute(&pool)
        .await
        .unwrap();
    assert_eq!(
        world
            .submit_discovery(
                user_id,
                discovery(
                    request_id,
                    attempt.attempt_id,
                    "This conflicts before current state.",
                ),
            )
            .await,
        Err(WorldError::DiscoveryRequestConflict)
    );
    let interaction_request_id = Uuid::new_v4();
    sqlx::query(
        r#"
        INSERT INTO activity (
            id, operation, requested_by_user_id, actor_character_entity_id,
            context_place_entity_id, prose, request_id, request_fingerprint
        ) VALUES ($1, 'submit_interaction', $2, $3, $4, 'Existing interaction.', $5, $6)
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(user_id.0)
    .bind(character.entity.id.0)
    .bind(place.entity.id.0)
    .bind(interaction_request_id)
    .bind(vec![4_u8; 32])
    .execute(&pool)
    .await
    .unwrap();
    assert_eq!(
        world
            .submit_discovery(
                user_id,
                discovery(
                    interaction_request_id,
                    attempt.attempt_id,
                    "This interaction id also conflicts first.",
                ),
            )
            .await,
        Err(WorldError::DiscoveryRequestConflict)
    );
}

#[sqlx::test(migrations = "./migration")]
async fn unavailable_attempt_reasons_share_one_neutral_error(pool: PgPool) {
    let draw = vec![0.0, 0.99, 0.0, 0.0, 0.0, 0.0, 0.0];
    let (world, user_id, character, _) = entered_world(pool.clone(), draw).await;
    let other_user = world.create_user().await.unwrap();
    world
        .create_character(
            other_user.id,
            CreateCharacter {
                name: "Iria".to_owned(),
                description: "A second surveyor.".to_owned(),
                property: Vec::new(),
                r#trait: Vec::new(),
            },
        )
        .await
        .unwrap();
    world.enter_world(other_user.id).await.unwrap();
    let alternate_place_entity = world
        .create_entity(
            user_id,
            CreateEntity {
                name: "Far Bank".to_owned(),
                description: "A second valid Place used only to exercise movement neutrality."
                    .to_owned(),
                property: Vec::new(),
                r#trait: Vec::new(),
            },
        )
        .await
        .unwrap();
    let alternate_activity_id: Uuid = sqlx::query_scalar(
        "SELECT activity_id FROM activity_entity WHERE entity_id = $1 AND role = 'subject'",
    )
    .bind(alternate_place_entity.id.0)
    .fetch_one(&pool)
    .await
    .unwrap();
    sqlx::query(
        "INSERT INTO place (entity_id, is_entry, latest_activity_id) VALUES ($1, false, $2)",
    )
    .bind(alternate_place_entity.id.0)
    .bind(alternate_activity_id)
    .execute(&pool)
    .await
    .unwrap();

    let foreign_attempt = positive(&world, user_id).await;
    let foreign = world
        .submit_discovery(
            other_user.id,
            discovery(
                Uuid::new_v4(),
                foreign_attempt.attempt_id,
                "Iria cannot use Mara's attempt.",
            ),
        )
        .await;
    let zero_attempt = positive_or_zero(&world, user_id).await.unwrap();
    assert_eq!(zero_attempt.outcome, InvestigationOutcome::Zero);
    let zero = world
        .submit_discovery(
            user_id,
            discovery(
                Uuid::new_v4(),
                zero_attempt.attempt_id,
                "A zero attempt cannot be used.",
            ),
        )
        .await;

    let consumed_attempt = positive(&world, user_id).await;
    world
        .submit_discovery(
            user_id,
            discovery(
                Uuid::new_v4(),
                consumed_attempt.attempt_id,
                "Mara consumes this attempt.",
            ),
        )
        .await
        .unwrap();
    let consumed = world
        .submit_discovery(
            user_id,
            discovery(
                Uuid::new_v4(),
                consumed_attempt.attempt_id,
                "It is already consumed.",
            ),
        )
        .await;

    let mut live = Vec::new();
    for _ in 0..4 {
        live.push(positive(&world, user_id).await);
    }
    let voided_id: Uuid = sqlx::query_scalar(
        r#"
        SELECT id FROM investigation_attempt
        WHERE requested_by_user_id = $1 AND voided_by_attempt_id IS NOT NULL
        "#,
    )
    .bind(user_id.0)
    .fetch_one(&pool)
    .await
    .unwrap();
    let voided = world
        .submit_discovery(
            user_id,
            discovery(
                Uuid::new_v4(),
                InvestigationAttemptId(voided_id),
                "A voided attempt cannot be used.",
            ),
        )
        .await;

    sqlx::query("UPDATE character SET current_place_entity_id = $2 WHERE entity_id = $1")
        .bind(character.entity.id.0)
        .bind(alternate_place_entity.id.0)
        .execute(&pool)
        .await
        .unwrap();
    let moved = world
        .submit_discovery(
            user_id,
            discovery(
                Uuid::new_v4(),
                live.last().unwrap().attempt_id,
                "A moved Character cannot use it.",
            ),
        )
        .await;
    sqlx::query("UPDATE character SET current_place_entity_id = NULL WHERE entity_id = $1")
        .bind(character.entity.id.0)
        .execute(&pool)
        .await
        .unwrap();
    let unplaced = world
        .submit_discovery(
            user_id,
            discovery(
                Uuid::new_v4(),
                live[live.len() - 2].attempt_id,
                "An unplaced Character cannot use it.",
            ),
        )
        .await;
    for result in [foreign, zero, consumed, voided, moved, unplaced] {
        assert_eq!(result, Err(WorldError::DiscoveryAttemptUnavailable));
    }
}

#[sqlx::test(migrations = "./migration")]
async fn unrelated_place_action_does_not_stale_positive_attempt(pool: PgPool) {
    let (world, user_id, _, _) = entered_world(pool, vec![0.0]).await;
    let attempt = positive(&world, user_id).await;
    let revision = world
        .get_character(user_id, GetEntityCurrentState::default())
        .await
        .unwrap()
        .place_revision
        .unwrap();
    world
        .submit_action(
            user_id,
            SubmitAction {
                request_id: Uuid::new_v4(),
                expected_place_revision: revision,
                prose: "Mara braces a survey marker.".to_owned(),
                consequence: ActionConsequence::IntroduceEntity(IntroduceEntity {
                    name: "Survey marker".to_owned(),
                    description: "A plain marker.".to_owned(),
                    property: Vec::new(),
                    r#trait: Vec::new(),
                }),
            },
        )
        .await
        .unwrap();
    world
        .submit_discovery(
            user_id,
            discovery(
                Uuid::new_v4(),
                attempt.attempt_id,
                "Mara finds rainbell cups.",
            ),
        )
        .await
        .unwrap();
}

#[sqlx::test(migrations = "./migration")]
async fn discovery_database_failure_rolls_back_every_write_and_leaves_attempt_live(pool: PgPool) {
    let (world, user_id, _, place) = entered_world(pool.clone(), vec![0.0]).await;
    world
        .create_entity(
            user_id,
            CreateEntity {
                name: "Known measure".to_owned(),
                description: "Establishes an integer key.".to_owned(),
                property: vec![PropertyInput {
                    key: "measure".to_owned(),
                    value: PropertyValue::Integer(1),
                }],
                r#trait: Vec::new(),
            },
        )
        .await
        .unwrap();
    let attempt = positive(&world, user_id).await;
    let before: (i64, i64, i64, Uuid) = sqlx::query_as(
        r#"
        SELECT (SELECT count(*) FROM entity),
               (SELECT count(*) FROM entity_location),
               (SELECT count(*) FROM activity),
               (SELECT latest_activity_id FROM place WHERE entity_id = $1)
        "#,
    )
    .bind(place.entity.id.0)
    .fetch_one(&pool)
    .await
    .unwrap();
    let mut input = discovery(
        Uuid::new_v4(),
        attempt.attempt_id,
        "Mara finds a contradiction.",
    );
    input.find.property = vec![PropertyInput {
        key: "measure".to_owned(),
        value: PropertyValue::Text("one".to_owned()),
    }];
    assert_eq!(
        world.submit_discovery(user_id, input).await,
        Err(WorldError::PropertyKeyConflict)
    );
    let after: (i64, i64, i64, Uuid) = sqlx::query_as(
        r#"
        SELECT (SELECT count(*) FROM entity),
               (SELECT count(*) FROM entity_location),
               (SELECT count(*) FROM activity),
               (SELECT latest_activity_id FROM place WHERE entity_id = $1)
        "#,
    )
    .bind(place.entity.id.0)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(after, before);
    let lifecycle: (Option<Uuid>, Option<Uuid>) = sqlx::query_as(
        "SELECT consumed_by_activity_id, voided_by_attempt_id FROM investigation_attempt WHERE id = $1",
    )
    .bind(attempt.attempt_id.0)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(lifecycle, (None, None));
}

#[sqlx::test(migrations = "./migration")]
async fn deferred_commit_failure_rolls_back_state_consumption_and_place_pointer(pool: PgPool) {
    let (world, user_id, _, place) = entered_world(pool.clone(), vec![0.0]).await;
    let attempt = positive(&world, user_id).await;
    let before: (i64, i64, i64, Uuid) = sqlx::query_as(
        r#"
        SELECT (SELECT count(*) FROM entity),
               (SELECT count(*) FROM entity_location),
               (SELECT count(*) FROM activity),
               latest_activity_id
        FROM place WHERE entity_id = $1
        "#,
    )
    .bind(place.entity.id.0)
    .fetch_one(&pool)
    .await
    .unwrap();
    sqlx::raw_sql(
        r#"
        CREATE FUNCTION fail_discovery_commit() RETURNS trigger
        LANGUAGE plpgsql AS $$
        BEGIN
            RAISE EXCEPTION 'injected deferred commit failure';
        END;
        $$;
        CREATE CONSTRAINT TRIGGER fail_discovery_commit
            AFTER UPDATE ON place
            DEFERRABLE INITIALLY DEFERRED
            FOR EACH ROW EXECUTE FUNCTION fail_discovery_commit();
        "#,
    )
    .execute(&pool)
    .await
    .unwrap();
    assert_eq!(
        world
            .submit_discovery(
                user_id,
                discovery(
                    Uuid::new_v4(),
                    attempt.attempt_id,
                    "Mara finds something that must roll back.",
                ),
            )
            .await,
        Err(WorldError::Unavailable)
    );
    let after: (i64, i64, i64, Uuid) = sqlx::query_as(
        r#"
        SELECT (SELECT count(*) FROM entity),
               (SELECT count(*) FROM entity_location),
               (SELECT count(*) FROM activity),
               latest_activity_id
        FROM place WHERE entity_id = $1
        "#,
    )
    .bind(place.entity.id.0)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(after, before);
    let lifecycle: (Option<Uuid>, Option<Uuid>) = sqlx::query_as(
        "SELECT consumed_by_activity_id, voided_by_attempt_id FROM investigation_attempt WHERE id = $1",
    )
    .bind(attempt.attempt_id.0)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(lifecycle, (None, None));
}

#[sqlx::test(migrations = "./migration")]
async fn equal_concurrent_starts_share_one_attempt(pool: PgPool) {
    let (world, user_id, _, _) = entered_world(pool.clone(), vec![0.0]).await;
    let request_id = Uuid::new_v4();
    let left = world.clone();
    let right = world.clone();
    let (left, right) = tokio::join!(
        left.start_investigation(user_id, StartInvestigation { request_id }),
        right.start_investigation(user_id, StartInvestigation { request_id })
    );
    assert_eq!(left.unwrap(), right.unwrap());
    let count: i64 = sqlx::query_scalar("SELECT count(*) FROM investigation_attempt")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(count, 1);
}

#[sqlx::test(migrations = "./migration")]
async fn distinct_concurrent_starts_preserve_admission_and_live_positive_caps(pool: PgPool) {
    let (world, user_id, _, _) =
        entered_world(pool.clone(), vec![0.0; MAX_ATTEMPTS_PER_HOUR as usize]).await;
    let mut tasks = tokio::task::JoinSet::new();
    for _ in 0..=MAX_ATTEMPTS_PER_HOUR {
        let world = world.clone();
        tasks.spawn(async move {
            world
                .start_investigation(
                    user_id,
                    StartInvestigation {
                        request_id: Uuid::new_v4(),
                    },
                )
                .await
        });
    }
    let mut accepted = 0;
    let mut rejected = 0;
    while let Some(result) = tasks.join_next().await {
        match result.unwrap() {
            Ok(result) => {
                assert_eq!(result.outcome, InvestigationOutcome::Positive);
                accepted += 1;
            }
            Err(WorldError::InvestigationNotAdmitted) => rejected += 1,
            other => panic!("unexpected concurrent start result: {other:?}"),
        }
    }
    assert_eq!(accepted, MAX_ATTEMPTS_PER_HOUR);
    assert_eq!(rejected, 1);
    let counts: (i64, i64) = sqlx::query_as(
        r#"
        SELECT count(*), count(*) FILTER (
            WHERE outcome = 'positive'
              AND consumed_by_activity_id IS NULL
              AND voided_by_attempt_id IS NULL
        )
        FROM investigation_attempt
        "#,
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(counts, (MAX_ATTEMPTS_PER_HOUR, MAX_LIVE_POSITIVES));
}

#[sqlx::test(migrations = "./migration")]
async fn concurrent_commits_consume_one_attempt_once(pool: PgPool) {
    let (world, user_id, _, _) = entered_world(pool.clone(), vec![0.0]).await;
    let attempt = positive(&world, user_id).await;
    let left = world.clone();
    let right = world.clone();
    let (left, right) = tokio::join!(
        left.submit_discovery(
            user_id,
            discovery(
                Uuid::new_v4(),
                attempt.attempt_id,
                "Mara finds the first cups."
            ),
        ),
        right.submit_discovery(
            user_id,
            discovery(
                Uuid::new_v4(),
                attempt.attempt_id,
                "Mara finds the same cups."
            ),
        )
    );
    assert!(matches!(
        (&left, &right),
        (Ok(_), Err(WorldError::DiscoveryAttemptUnavailable))
            | (Err(WorldError::DiscoveryAttemptUnavailable), Ok(_))
    ));
    let discovery_count: i64 =
        sqlx::query_scalar("SELECT count(*) FROM activity WHERE operation = 'submit_discovery'")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(discovery_count, 1);
}

#[sqlx::test(migrations = "./migration")]
async fn chance_counts_discovery_only_after_bounding_last_forty_eight_activities(pool: PgPool) {
    let (world, user_id, character, place) = entered_world(pool.clone(), vec![0.49]).await;
    insert_raw_activity(
        &pool,
        user_id,
        character.entity.id,
        place.entity.id,
        "submit_discovery",
    )
    .await;
    for _ in 0..48 {
        insert_raw_activity(
            &pool,
            user_id,
            character.entity.id,
            place.entity.id,
            "create_entity",
        )
        .await;
    }
    let result = positive(&world, user_id).await;
    assert_eq!(result.outcome, InvestigationOutcome::Positive);
}

async fn insert_raw_activity(
    pool: &PgPool,
    user_id: UserId,
    character_entity_id: EntityId,
    place_entity_id: EntityId,
    operation: &str,
) {
    let confirmed = operation == "submit_discovery";
    sqlx::query(
        r#"
        INSERT INTO activity (
            id, operation, requested_by_user_id, actor_character_entity_id,
            context_place_entity_id, prose, request_id, request_fingerprint
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(operation)
    .bind(user_id.0)
    .bind(character_entity_id.0)
    .bind(place_entity_id.0)
    .bind(confirmed.then_some("A find enters shared history."))
    .bind(confirmed.then(Uuid::new_v4))
    .bind(confirmed.then(|| vec![1_u8; 32]))
    .execute(pool)
    .await
    .unwrap();
}

fn colour_as_property_find() -> DiscoveryFind {
    DiscoveryFind {
        name: "Rainbell Cups".to_owned(),
        description: "Chalk-pale cups whose thin rims ring in rain.".to_owned(),
        property: vec![PropertyInput {
            key: "colour".to_owned(),
            value: PropertyValue::Text("warm".to_owned()),
        }],
        r#trait: Vec::new(),
    }
}

fn colour_as_trait_find() -> DiscoveryFind {
    DiscoveryFind {
        name: "Rainbell Cups".to_owned(),
        description: "Chalk-pale cups whose thin rims ring in rain.".to_owned(),
        property: Vec::new(),
        r#trait: ["colour", "text", "warm"]
            .into_iter()
            .map(|statement| TraitInput {
                statement: statement.to_owned(),
            })
            .collect(),
    }
}

#[test]
fn discovery_fingerprint_separates_property_content_from_trait_content() {
    let attempt_id = InvestigationAttemptId(Uuid::new_v4());
    let request_id = Uuid::new_v4();
    let mut as_property = discovery(request_id, attempt_id, "Mara notes one warm colour.");
    as_property.find = colour_as_property_find();
    let mut as_trait = as_property.clone();
    as_trait.find = colour_as_trait_find();
    let as_property = as_property.normalize().unwrap();
    let as_trait = as_trait.normalize().unwrap();
    assert!(as_property.find.r#trait.is_empty());
    assert!(as_trait.find.property.is_empty());
    assert_ne!(
        discovery_fingerprint(&as_property),
        discovery_fingerprint(&as_trait)
    );
}

#[sqlx::test(migrations = "./migration")]
async fn discovery_retry_moving_find_content_between_property_and_trait_conflicts(pool: PgPool) {
    let (world, user_id, _, _) = entered_world(pool.clone(), vec![0.0]).await;
    let attempt = positive(&world, user_id).await;
    let prose = "Mara notes one warm colour.";
    let mut first = discovery(Uuid::new_v4(), attempt.attempt_id, prose);
    first.find = colour_as_property_find();
    let mut second = first.clone();
    second.find = colour_as_trait_find();
    world.submit_discovery(user_id, first).await.unwrap();
    assert_eq!(
        world.submit_discovery(user_id, second).await,
        Err(WorldError::DiscoveryRequestConflict)
    );
}

fn tag_lookalike_as_property_find() -> DiscoveryFind {
    DiscoveryFind {
        name: "Rainbell Cups".to_owned(),
        description: "Chalk-pale cups whose thin rims ring in rain.".to_owned(),
        property: vec![
            PropertyInput {
                key: "a".to_owned(),
                value: PropertyValue::Text("v1".to_owned()),
            },
            PropertyInput {
                key: "trait".to_owned(),
                value: PropertyValue::Text("tf".to_owned()),
            },
        ],
        r#trait: vec![TraitInput {
            statement: "z".to_owned(),
        }],
    }
}

fn tag_lookalike_as_trait_find() -> DiscoveryFind {
    DiscoveryFind {
        name: "Rainbell Cups".to_owned(),
        description: "Chalk-pale cups whose thin rims ring in rain.".to_owned(),
        property: vec![PropertyInput {
            key: "a".to_owned(),
            value: PropertyValue::Text("v1".to_owned()),
        }],
        r#trait: ["text", "tf", "trait", "z"]
            .into_iter()
            .map(|statement| TraitInput {
                statement: statement.to_owned(),
            })
            .collect(),
    }
}

#[sqlx::test(migrations = "./migration")]
async fn discovery_retry_realigning_a_find_across_tag_lookalike_content_conflicts(pool: PgPool) {
    let (world, user_id, _, _) = entered_world(pool.clone(), vec![0.0]).await;
    let attempt = positive(&world, user_id).await;
    let prose = "Mara records a key and a statement that read like list tags.".to_owned();
    let mut first = discovery(Uuid::new_v4(), attempt.attempt_id, &prose);
    first.find = tag_lookalike_as_property_find();
    let mut second = first.clone();
    second.find = tag_lookalike_as_trait_find();
    world.submit_discovery(user_id, first).await.unwrap();
    assert_eq!(
        world.submit_discovery(user_id, second).await,
        Err(WorldError::DiscoveryRequestConflict)
    );
}

#[sqlx::test(migrations = "./migration")]
async fn one_discovery_inside_the_place_window_lowers_the_outcome(pool: PgPool) {
    let (world, user_id, character, place) = entered_world(pool.clone(), vec![0.49]).await;
    insert_raw_activity(
        &pool,
        user_id,
        character.entity.id,
        place.entity.id,
        "submit_discovery",
    )
    .await;
    assert_eq!(
        positive_or_zero(&world, user_id).await.unwrap().outcome,
        InvestigationOutcome::Zero
    );
}

#[sqlx::test(migrations = "./migration")]
async fn investigation_indexes_support_bounded_hot_subject_queries(pool: PgPool) {
    let (_world, user_id, character, place) = entered_world(pool.clone(), Vec::new()).await;
    for age_days in 0..128_i32 {
        sqlx::query(
            r#"
            INSERT INTO investigation_attempt (
                id, requested_by_user_id, request_id, character_entity_id,
                place_entity_id, outcome, created_at
            ) VALUES (
                $1, $2, $3, $4, $5, 'positive',
                statement_timestamp() - make_interval(days => $6)
            )
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(user_id.0)
        .bind(Uuid::new_v4())
        .bind(character.entity.id.0)
        .bind(place.entity.id.0)
        .bind(age_days)
        .execute(&pool)
        .await
        .unwrap();
    }
    sqlx::query("ANALYZE activity, investigation_attempt")
        .execute(&pool)
        .await
        .unwrap();

    let mut transaction = pool.begin().await.unwrap();
    sqlx::query("SET LOCAL enable_seqscan = off")
        .execute(&mut *transaction)
        .await
        .unwrap();

    let window_plan = sqlx::query_scalar::<_, String>(&format!(
        "EXPLAIN (COSTS OFF) {PLACE_WINDOW_DISCOVERY_COUNT_SQL}"
    ))
    .bind(place.entity.id.0)
    .bind(PLACE_ACTIVITY_WINDOW)
    .fetch_all(&mut *transaction)
    .await
    .unwrap()
    .join("\n");
    assert!(
        window_plan.contains("activity_place_occurred_at_id_index"),
        "the live bounded Place window must use its declared index: {window_plan}"
    );

    let admission_plan =
        sqlx::query_scalar::<_, String>(&format!("EXPLAIN (COSTS OFF) {ADMISSION_SQL}"))
            .bind(user_id.0)
            .fetch_all(&mut *transaction)
            .await
            .unwrap()
            .join("\n");
    assert!(
        admission_plan.contains("investigation_attempt_user_created_at_index"),
        "the live admission count must use its declared index: {admission_plan}"
    );

    let void_plan = sqlx::query_scalar::<_, String>(&format!(
        "EXPLAIN (COSTS OFF) {VOID_OLDEST_PRIOR_POSITIVE_SQL}"
    ))
    .bind(user_id.0)
    .bind(Uuid::new_v4())
    .bind(MAX_LIVE_POSITIVES)
    .fetch_all(&mut *transaction)
    .await
    .unwrap()
    .join("\n");
    assert!(
        void_plan.contains("investigation_attempt_live_positive_index"),
        "the live hoarding void must use its declared index: {void_plan}"
    );
}
