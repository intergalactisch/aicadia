//! Bounded operator-only Live reads over a disposable database.
//!
//! Every fixture below is seeded through the real `World` interface, so the rows
//! these reads page over are exactly the rows accepted game behavior writes. The
//! assertions are about the Studio reads only: their bounds, their keyset
//! stability, the joins they resolve and the labels they carry when PostgreSQL
//! cannot serve them from an index.

use aicadia::{
    ActionConsequence, ChangeEntityState, CreateCharacter, CreateEntity, CreateEntryPlace,
    EntityCurrentAssociation, EntityId, EntityPropertyChangeInput, EntityTraitChangeInput,
    EntityTraitId, GetEntityAtCurrentPlace, IntroduceEntity, ListEntityAtCurrentPlace,
    PlaceRevision, PropertyInput, PropertyValue, StartInvestigation, SubmitAction, TraitInput,
    World,
};
use aicadia_studio::StudioError;
use aicadia_studio::live::{
    character, chronicle, entity, estimate, investigation, migration, page::Bound, place, property,
    resolve, row, schema, r#trait, user,
};
use sqlx::PgPool;
use uuid::Uuid;

/// The exact number of application tables migrations `0001`–`0010` deliver.
const APPLICATION_TABLE_COUNT: usize = 14;

/// The Studio read refused the limit rather than clamping it.
fn refused_the_limit<T>(result: Result<T, StudioError>) -> bool {
    matches!(result, Err(StudioError::InvalidLimit))
}

/// The Studio read said the subject does not exist rather than inventing one.
fn not_found<T>(result: Result<T, StudioError>) -> bool {
    matches!(result, Err(StudioError::NotFound))
}

struct Seed {
    user_id: Uuid,
    character_entity_id: Uuid,
    place_entity_id: Uuid,
    /// Entities introduced by an Action, in creation order; all are at the Place.
    introduced_entity_id: Vec<Uuid>,
    /// Entities created outside a Place by `create_entity`.
    unplaced_entity_id: Vec<Uuid>,
    /// The Entity whose Property and Trait a later Action changed.
    changed_entity_id: Uuid,
    changed_trait_id: Uuid,
    change_activity_id: Uuid,
    change_request_id: Uuid,
    attempt_id: Uuid,
    attempt_request_id: Uuid,
}

async fn current_revision(world: &World, user_id: Uuid) -> PlaceRevision {
    world
        .list_entity_at_current_place(
            aicadia::UserId(user_id),
            ListEntityAtCurrentPlace::default(),
        )
        .await
        .expect("the entered Character can read its own Place")
        .place_revision
}

async fn trait_id_of(world: &World, user_id: Uuid, entity_id: Uuid) -> EntityTraitId {
    let page = world
        .get_entity_at_current_place(
            aicadia::UserId(user_id),
            GetEntityAtCurrentPlace {
                entity_id: EntityId(entity_id),
                cursor: None,
                limit: 100,
            },
        )
        .await
        .expect("a local Entity is readable at the current Place");
    page.current_state
        .association
        .into_iter()
        .find_map(|association| match association {
            EntityCurrentAssociation::Trait(entity_trait) => Some(entity_trait.id),
            EntityCurrentAssociation::Property { .. } => None,
        })
        .expect("the introduced Entity established one Trait")
}

/// One World with a User, a Character, an entry Place, Entities inside and
/// outside that Place, one accepted state change and two investigation attempts.
async fn seed(pool: &PgPool) -> Seed {
    let world = World::new(pool.clone());
    let user = world.create_user().await.expect("a User is created");
    let user_id = user.id;

    let character = world
        .create_character(
            user_id,
            CreateCharacter {
                name: "Mara Venn".to_owned(),
                description: "A careful surveyor at the edge of the known World.".to_owned(),
                property: vec![PropertyInput {
                    key: "role".to_owned(),
                    value: PropertyValue::Text("surveyor".to_owned()),
                }],
                r#trait: vec![TraitInput {
                    statement: "Keeps a field journal of every measurement.".to_owned(),
                }],
            },
        )
        .await
        .expect("the User creates one Character");

    let place = world
        .create_entry_place(
            user_id,
            CreateEntryPlace {
                name: "North Gate".to_owned(),
                description: "The one established entry into the shared World.".to_owned(),
                property: vec![PropertyInput {
                    key: "surface".to_owned(),
                    value: PropertyValue::Text("packed gravel".to_owned()),
                }],
                r#trait: Vec::new(),
            },
        )
        .await
        .expect("the World gains its entry Place");
    world
        .enter_world(user_id)
        .await
        .expect("the Character enters the World");

    let mut unplaced_entity_id = Vec::new();
    for index in 0..2 {
        let entity = world
            .create_entity(
                user_id,
                CreateEntity {
                    name: format!("Survey note {index}"),
                    description: format!("A loose note from the {index}th survey run."),
                    property: vec![PropertyInput {
                        key: "page_count".to_owned(),
                        value: PropertyValue::Integer(index + 1),
                    }],
                    r#trait: Vec::new(),
                },
            )
            .await
            .expect("the User creates an Entity");
        unplaced_entity_id.push(entity.id.0);
    }

    let mut introduced_entity_id = Vec::new();
    for index in 0..3 {
        let revision = current_revision(&world, user_id.0).await;
        let accepted = world
            .submit_action(
                user_id,
                SubmitAction {
                    request_id: Uuid::new_v4(),
                    expected_place_revision: revision,
                    prose: format!(
                        "Mara sets a marker stone {} paces along the path and records it.",
                        index + 1
                    ),
                    consequence: ActionConsequence::IntroduceEntity(IntroduceEntity {
                        name: format!("Marker stone {index}"),
                        description: format!(
                            "A weathered stone marking length {index} of the path."
                        ),
                        property: vec![PropertyInput {
                            key: "height_cm".to_owned(),
                            value: PropertyValue::Integer(60 + index),
                        }],
                        r#trait: vec![TraitInput {
                            statement: format!("Stands {} paces from the gate.", index + 1),
                        }],
                        position_description: None,
                    }),
                },
            )
            .await
            .expect("the entered Character introduces an Entity");
        match accepted.consequence {
            aicadia::AcceptedActionConsequence::IntroduceEntity(entity) => {
                introduced_entity_id.push(entity.id.0)
            }
            aicadia::AcceptedActionConsequence::ChangeEntityState { .. } => {
                panic!("the introduce Action must return the introduced Entity")
            }
        }
    }

    let changed_entity_id = introduced_entity_id[0];
    let changed_trait_id = trait_id_of(&world, user_id.0, changed_entity_id).await;
    let revision = current_revision(&world, user_id.0).await;
    let change_request_id = Uuid::new_v4();
    let accepted = world
        .submit_action(
            user_id,
            SubmitAction {
                request_id: change_request_id,
                expected_place_revision: revision,
                prose: "Mara re-measures the first marker stone and revises her note.".to_owned(),
                consequence: ActionConsequence::ChangeEntityState(ChangeEntityState {
                    property_change: vec![EntityPropertyChangeInput {
                        entity_id: EntityId(changed_entity_id),
                        key: "height_cm".to_owned(),
                        value: PropertyValue::Integer(64),
                    }],
                    trait_change: vec![EntityTraitChangeInput::Develop {
                        trait_id: changed_trait_id,
                        statement: "Stands one pace from the gate, leaning north.".to_owned(),
                    }],
                }),
            },
        )
        .await
        .expect("the entered Character changes local Entity state");
    let change_activity_id = accepted.activity.id.0;

    let attempt_request_id = Uuid::new_v4();
    let attempt = world
        .start_investigation(
            user_id,
            StartInvestigation {
                request_id: attempt_request_id,
            },
        )
        .await
        .expect("the entered Character may start one investigation");
    world
        .start_investigation(
            user_id,
            StartInvestigation {
                request_id: Uuid::new_v4(),
            },
        )
        .await
        .expect("a second investigation stays inside the hourly admission window");

    Seed {
        user_id: user_id.0,
        character_entity_id: character.entity.id.0,
        place_entity_id: place.entity.id.0,
        introduced_entity_id,
        unplaced_entity_id,
        changed_entity_id,
        changed_trait_id: changed_trait_id.0,
        change_activity_id,
        change_request_id,
        attempt_id: attempt.attempt_id.0,
        attempt_request_id,
    }
}

#[sqlx::test(migrations = "../game/migration")]
async fn every_live_list_refuses_a_limit_outside_its_one_hard_bound(pool: PgPool) {
    seed(&pool).await;

    assert!(refused_the_limit(Bound::new(Some(0))));
    assert!(refused_the_limit(Bound::new(Some(101))));
    assert!(refused_the_limit(Bound::new(Some(u16::MAX))));
    assert_eq!(Bound::new(None).expect("the default bound").limit(), 24);
    assert_eq!(
        Bound::new(Some(100)).expect("the maximum bound").limit(),
        100
    );
}

#[sqlx::test(migrations = "../game/migration")]
async fn every_live_list_stops_at_its_bound_and_names_the_row_after_it(pool: PgPool) {
    let seed = seed(&pool).await;
    let bound = Bound::new(Some(2)).expect("a two-row bound");

    let entity = entity::list_entity(&pool, None, bound)
        .await
        .expect("the Entity list reads");
    assert_eq!(entity.item.len(), 2);
    assert!(entity.truncated);
    assert!(entity.next_cursor.is_some());

    let world_chronicle = chronicle::list_world_chronicle(&pool, None, bound)
        .await
        .expect("the World chronicle reads");
    assert_eq!(world_chronicle.page.item.len(), 2);
    assert!(world_chronicle.page.truncated);

    let place_entity = place::list_place_entity(&pool, seed.place_entity_id, None, bound)
        .await
        .expect("the Place membership reads");
    assert_eq!(place_entity.item.len(), 2);
    assert!(place_entity.truncated);

    let attempt = investigation::list_investigation(&pool, None, Bound::new(Some(1)).unwrap())
        .await
        .expect("the attempt list reads");
    assert_eq!(attempt.page.item.len(), 1);
    assert!(attempt.page.truncated);
    assert!(attempt.page.next_cursor.is_some());

    // A list shorter than its bound carries no cursor and claims no truncation.
    let user_page = user::list_user(&pool, None, Bound::new(Some(100)).unwrap())
        .await
        .expect("the User list reads");
    assert_eq!(user_page.item.len(), 1);
    assert!(!user_page.truncated);
    assert!(user_page.next_cursor.is_none());
}

#[sqlx::test(migrations = "../game/migration")]
async fn keyset_paging_is_stable_across_two_pages(pool: PgPool) {
    let seed = seed(&pool).await;
    let bound = Bound::new(Some(2)).expect("a two-row bound");

    let first = entity::list_entity(&pool, None, bound)
        .await
        .expect("the first Entity page reads");
    let second = entity::list_entity(&pool, first.next_cursor, bound)
        .await
        .expect("the second Entity page reads");
    let mut seen = first.item.iter().map(|item| item.id).collect::<Vec<_>>();
    seen.extend(second.item.iter().map(|item| item.id));
    let mut unique = seen.clone();
    unique.sort();
    unique.dedup();
    assert_eq!(seen.len(), 4);
    assert_eq!(unique.len(), 4, "two Entity pages must not repeat a row");
    let time = first
        .item
        .iter()
        .chain(second.item.iter())
        .map(|item| item.introduced_at)
        .collect::<Vec<_>>();
    assert!(
        time.windows(2).all(|pair| pair[0] >= pair[1]),
        "the Entity list stays newest-first across pages: {time:?}"
    );

    // The Place membership keyset ascends by entity_id and must not repeat either.
    let first = place::list_place_entity(&pool, seed.place_entity_id, None, bound)
        .await
        .expect("the first membership page reads");
    let second = place::list_place_entity(&pool, seed.place_entity_id, first.next_cursor, bound)
        .await
        .expect("the second membership page reads");
    let id = first
        .item
        .iter()
        .chain(second.item.iter())
        .map(|item| item.id)
        .collect::<Vec<_>>();
    let mut unique = id.clone();
    unique.sort();
    unique.dedup();
    assert_eq!(id.len(), unique.len(), "membership pages must not repeat");
    assert!(id.windows(2).all(|pair| pair[0] < pair[1]));
}

#[sqlx::test(migrations = "../game/migration")]
async fn every_chronicle_is_newest_first_and_the_world_chronicle_says_what_it_is(pool: PgPool) {
    let seed = seed(&pool).await;
    let bound = Bound::new(Some(100)).expect("the maximum bound");

    let place_chronicle = chronicle::list_place_chronicle(&pool, seed.place_entity_id, None, bound)
        .await
        .expect("the Place chronicle reads");
    assert!(!place_chronicle.item.is_empty());
    assert!(
        place_chronicle
            .item
            .windows(2)
            .all(|pair| (pair[0].occurred_at, pair[0].id) > (pair[1].occurred_at, pair[1].id))
    );
    assert!(place_chronicle.item.iter().all(|item| {
        item.context_place
            .as_ref()
            .is_some_and(|place| place.id == seed.place_entity_id)
    }));

    let character_chronicle =
        chronicle::list_character_chronicle(&pool, seed.character_entity_id, None, bound)
            .await
            .expect("the Character chronicle reads");
    assert!(!character_chronicle.item.is_empty());
    assert!(
        character_chronicle
            .item
            .windows(2)
            .all(|pair| (pair[0].occurred_at, pair[0].id) > (pair[1].occurred_at, pair[1].id))
    );
    assert!(character_chronicle.item.iter().all(|item| {
        item.actor_character
            .as_ref()
            .is_some_and(|actor| actor.id == seed.character_entity_id)
    }));

    let world_chronicle = chronicle::list_world_chronicle(&pool, None, bound)
        .await
        .expect("the World chronicle reads");
    assert_eq!(world_chronicle.scope, "local development sort");
    assert_eq!(world_chronicle.scope, chronicle::WORLD_CHRONICLE_SCOPE);
    assert!(world_chronicle.page.item.len() > place_chronicle.item.len());
    assert!(
        world_chronicle
            .page
            .item
            .windows(2)
            .all(|pair| (pair[0].occurred_at, pair[0].id) > (pair[1].occurred_at, pair[1].id))
    );
    assert!(
        ["create_entity", "submit_action"]
            .into_iter()
            .all(|operation| {
                world_chronicle
                    .page
                    .item
                    .iter()
                    .any(|item| item.operation == operation)
            }),
        "the server loads an unfiltered Activity window for browser-only filtering"
    );

    // Prose is previewed, and every row carries a bounded involved-Entity count.
    let action = world_chronicle
        .page
        .item
        .iter()
        .find(|item| item.id == seed.change_activity_id)
        .expect("the state-changing Action is in the World chronicle");
    assert!(action.prose.is_some());
    assert!(!action.prose_truncated);
    assert_eq!(action.involved_entity_count, 2);
    assert!(!action.involved_entity_count_truncated);
}

#[sqlx::test(migrations = "../game/migration")]
async fn entity_detail_joins_roles_and_bounded_current_state(pool: PgPool) {
    let seed = seed(&pool).await;

    let detail = entity::get_entity(&pool, seed.changed_entity_id)
        .await
        .expect("the changed Entity reads");
    assert_eq!(detail.entity.id, seed.changed_entity_id);
    assert!(!detail.entity.is_character);
    assert!(!detail.entity.is_place);
    assert_eq!(
        detail.entity.current_place.as_ref().map(|place| place.id),
        Some(seed.place_entity_id)
    );
    let document = serde_json::to_value(&detail).expect("the Entity detail serializes");
    assert!(
        document.get("establishing_activity").is_none(),
        "the index-free detail has no establishing-Activity reversal"
    );

    assert_eq!(detail.property.len(), 1);
    assert_eq!(detail.property[0].key, "height_cm");
    assert_eq!(detail.property[0].integer_value, Some(64));
    assert_eq!(detail.property[0].version_count, 2);
    assert!(!detail.property_version_count_truncated);
    assert!(!detail.property_truncated);

    assert_eq!(detail.r#trait.len(), 1);
    assert_eq!(detail.r#trait[0].id, seed.changed_trait_id);
    assert_eq!(
        detail.r#trait[0].statement,
        "Stands one pace from the gate, leaning north."
    );
    assert!(!detail.trait_truncated);

    let participation = entity::list_participation(
        &pool,
        seed.changed_entity_id,
        None,
        Bound::new(Some(100)).unwrap(),
    )
    .await
    .expect("the Entity participation reads");
    assert_eq!(participation.order, entity::ACTIVITY_ID_ORDER);
    assert_eq!(participation.page.item.len(), 2);
    assert!(
        participation
            .page
            .item
            .iter()
            .all(|item| item.role == "subject")
    );
    assert!(
        participation
            .page
            .item
            .iter()
            .any(|item| item.activity_id == seed.change_activity_id)
    );
    assert!(
        participation
            .page
            .item
            .windows(2)
            .all(|pair| pair[0].activity_id > pair[1].activity_id),
        "participation is ordered by the index it uses"
    );

    // The Character role and its owner resolve from the same Entity read.
    let character = entity::get_entity(&pool, seed.character_entity_id)
        .await
        .expect("the Character Entity reads");
    assert!(character.entity.is_character);
    assert_eq!(character.entity.owner_user_id, Some(seed.user_id));

    let place_entity = entity::get_entity(&pool, seed.place_entity_id)
        .await
        .expect("the Place Entity reads");
    assert!(place_entity.entity.is_place);
    assert_eq!(place_entity.entity.is_entry_place, Some(true));

    assert!(not_found(entity::get_entity(&pool, Uuid::new_v4()).await));
}

#[sqlx::test(migrations = "../game/migration")]
async fn property_history_holds_every_stored_version_of_one_key(pool: PgPool) {
    let seed = seed(&pool).await;

    let detail = entity::get_entity(&pool, seed.changed_entity_id)
        .await
        .expect("the changed Entity reads");
    let property_key_id = detail.property[0].property_key_id;

    let history = entity::list_property_history(
        &pool,
        seed.changed_entity_id,
        property_key_id,
        None,
        Bound::new(Some(100)).unwrap(),
    )
    .await
    .expect("the Property history reads");
    assert_eq!(history.key, "height_cm");
    assert_eq!(history.order, entity::ACTIVITY_ID_ORDER);
    assert_eq!(history.page.item.len(), 2);
    assert!(!history.page.truncated);
    assert_eq!(
        history
            .page
            .item
            .iter()
            .filter(|version| version.previous_activity_id.is_none())
            .count(),
        1,
        "exactly one Property version starts the lineage"
    );
    assert_eq!(
        history
            .page
            .item
            .iter()
            .filter(|version| version.is_current)
            .count(),
        1
    );
    let mut value = history
        .page
        .item
        .iter()
        .filter_map(|version| version.integer_value)
        .collect::<Vec<_>>();
    value.sort();
    assert_eq!(value, vec![60, 64]);
    assert!(
        history
            .page
            .item
            .iter()
            .all(|version| version.operation == "submit_action")
    );

    // A key the World never created is not found.
    assert!(not_found(
        entity::list_property_history(
            &pool,
            seed.changed_entity_id,
            i64::MAX,
            None,
            Bound::new(None).unwrap(),
        )
        .await
    ));
}

#[sqlx::test(migrations = "../game/migration")]
async fn a_trait_lineage_has_exactly_one_root_and_one_current_tip(pool: PgPool) {
    let seed = seed(&pool).await;

    let detail = r#trait::get_trait(&pool, seed.changed_trait_id)
        .await
        .expect("the developed Trait reads");
    assert_eq!(detail.id, seed.changed_trait_id);
    assert_eq!(detail.entity.id, seed.changed_entity_id);
    assert_eq!(detail.order, r#trait::TRAIT_VERSION_ORDER);
    assert_eq!(detail.version.len(), 2);
    assert!(!detail.version_truncated);
    assert_eq!(
        detail
            .version
            .iter()
            .filter(|version| version.is_root)
            .count(),
        1
    );
    assert_eq!(
        detail
            .version
            .iter()
            .filter(|version| version.is_current)
            .count(),
        1
    );
    let current = detail
        .version
        .iter()
        .find(|version| version.is_current)
        .expect("one current version");
    assert_eq!(current.statement, detail.current_statement);
    assert_eq!(current.activity_id, detail.current_activity_id);
    assert_eq!(current.previous_activity_id, {
        let root = detail
            .version
            .iter()
            .find(|version| version.is_root)
            .expect("one root version");
        Some(root.activity_id)
    });

    assert!(not_found(r#trait::get_trait(&pool, Uuid::new_v4()).await));
}

#[sqlx::test(migrations = "../game/migration")]
async fn the_resolver_answers_every_kind_of_exact_id_and_stays_silent_otherwise(pool: PgPool) {
    let seed = seed(&pool).await;

    for (id, subject) in [
        (seed.changed_entity_id, "entity"),
        (seed.change_activity_id, "activity"),
        (seed.user_id, "user"),
        (seed.changed_trait_id, "trait"),
        (seed.attempt_id, "investigation attempt"),
    ] {
        let resolved = resolve::resolve(&pool, id)
            .await
            .expect("the resolver reads");
        assert_eq!(resolved.id, id);
        assert!(
            resolved.hit.iter().any(|hit| hit.subject == subject),
            "{id} should resolve to {subject}, got {:?}",
            resolved
                .hit
                .iter()
                .map(|hit| hit.subject)
                .collect::<Vec<_>>()
        );
    }

    let resolved = resolve::resolve(&pool, seed.changed_entity_id)
        .await
        .expect("the resolver reads");
    let entity_hit = resolved
        .hit
        .iter()
        .find(|hit| hit.subject == "entity")
        .expect("the Entity hit");
    assert_eq!(entity_hit.name.as_deref(), Some("Marker stone 0"));
    assert_eq!(entity_hit.lookup, resolve::PRIMARY_KEY);

    for request_id in [seed.change_request_id, seed.attempt_request_id] {
        let resolved = resolve::resolve(&pool, request_id)
            .await
            .expect("the resolver reads a request id");
        assert!(
            resolved.hit.is_empty(),
            "request ids are not globally probed without their owning User"
        );
    }

    let missing = resolve::resolve(&pool, Uuid::new_v4())
        .await
        .expect("the resolver reads an unknown id");
    assert!(
        missing.hit.is_empty(),
        "an unknown id resolves to nothing: {:?}",
        missing
            .hit
            .iter()
            .map(|hit| hit.subject)
            .collect::<Vec<_>>()
    );
}

#[sqlx::test(migrations = "../game/migration")]
async fn row_estimates_cover_every_application_table_and_say_they_are_estimates(pool: PgPool) {
    seed(&pool).await;

    let table = schema::application_table(&pool)
        .await
        .expect("the application tables read");
    assert_eq!(table.len(), APPLICATION_TABLE_COUNT);
    assert!(!table.iter().any(|name| name == "_sqlx_migrations"));

    let report = estimate::estimate(&pool).await.expect("the estimates read");
    assert_eq!(report.scope, estimate::PLANNER_ESTIMATE);
    assert_eq!(report.scope, "planner estimate");
    assert_eq!(report.table.len(), APPLICATION_TABLE_COUNT);
    let estimated = report
        .table
        .iter()
        .map(|table| table.table.clone())
        .collect::<Vec<_>>();
    assert_eq!(estimated, table);
}

#[sqlx::test(migrations = "../game/migration")]
async fn applied_migrations_match_the_repository_migration_files(pool: PgPool) {
    let root = aicadia_studio::workspace_root();
    let file_count = std::fs::read_dir(root.join("game/migration"))
        .expect("the repository migration directory is readable")
        .filter_map(Result::ok)
        .filter(|entry| entry.file_name().to_string_lossy().ends_with(".sql"))
        .count();

    let report = migration::list_migration(&pool, &root)
        .await
        .expect("the migration report reads");
    assert_eq!(report.applied.len(), file_count);
    assert!(!report.truncated);
    assert!(
        report.unapplied_file.is_empty(),
        "a disposable test database is fully migrated: {:?}",
        report.unapplied_file
    );
    assert!(report.applied.iter().all(|applied| applied.success));
    assert!(report.applied.iter().all(|applied| applied.file.is_some()));
    // The digest width is sqlx's choice; Studio only renders what it stored.
    assert!(report.applied.iter().all(|applied| {
        !applied.checksum.is_empty()
            && applied.checksum.len().is_multiple_of(2)
            && applied
                .checksum
                .chars()
                .all(|character| character.is_ascii_hexdigit() && !character.is_uppercase())
    }));
    assert!(
        report
            .applied
            .windows(2)
            .all(|pair| pair[0].version < pair[1].version)
    );
    assert_eq!(
        report.applied.first().map(|applied| applied.file.clone()),
        Some(Some("0001_world.sql".to_owned()))
    );
}

#[sqlx::test(migrations = "../game/migration")]
async fn latest_successful_migration_is_one_fixed_newest_primary_key_window(pool: PgPool) {
    let latest = schema::latest_successful_migration(&pool)
        .await
        .expect("the latest migration reads");
    assert_eq!(latest.version, Some(10));
    assert_eq!(latest.state, schema::LATEST_MIGRATION_KNOWN);
    assert_eq!(latest.inspected_newest, 10);

    sqlx::query(
        r#"
        INSERT INTO _sqlx_migrations
            (version, description, installed_on, success, checksum, execution_time)
        SELECT version,
               'failed fixture',
               clock_timestamp(),
               false,
               decode('00', 'hex'),
               0
        FROM generate_series(11, 110) AS version
        "#,
    )
    .execute(&pool)
    .await
    .expect("the newest failed migration window is inserted");

    let partial = schema::latest_successful_migration(&pool)
        .await
        .expect("the bounded latest migration reads");
    assert_eq!(partial.version, None);
    assert_eq!(partial.state, schema::LATEST_MIGRATION_UNKNOWN_PARTIAL);
    assert_eq!(partial.inspected_newest, schema::LATEST_MIGRATION_WINDOW);
}

#[sqlx::test(migrations = "../game/migration")]
async fn the_row_viewer_pages_every_current_table_by_its_primary_key(pool: PgPool) {
    seed(&pool).await;
    let bound = Bound::new(Some(1)).expect("a one-row bound");

    let table = schema::application_table(&pool)
        .await
        .expect("the application tables read");
    assert_eq!(table.len(), APPLICATION_TABLE_COUNT);

    let mut paged = 0;
    for name in &table {
        let first = row::list_row(&pool, name, None, bound)
            .await
            .unwrap_or_else(|error| panic!("{name} should page: {error:?}"));
        assert_eq!(&first.table, name);
        assert!(!first.column.is_empty(), "{name} has columns");
        assert!(
            !first.primary_key.is_empty(),
            "{name} has a primary key, so its order is stable"
        );
        assert!(first.row.len() <= 1);
        assert!(first.row.iter().all(|row| row.is_object()));

        if !first.truncated {
            assert!(first.next_cursor.is_none());
            continue;
        }
        paged += 1;
        let cursor = first.next_cursor.clone().expect("a truncated page keys on");
        assert_eq!(cursor.len(), first.primary_key.len());
        let second = row::list_row(&pool, name, Some(&cursor), bound)
            .await
            .unwrap_or_else(|error| panic!("{name} should page again: {error:?}"));
        assert_eq!(second.row.len(), 1);
        assert_ne!(
            first.row[0], second.row[0],
            "{name} must not repeat a row across pages"
        );
        // The same cursor read twice returns the same row.
        let repeat = row::list_row(&pool, name, Some(&cursor), bound)
            .await
            .expect("a cursor is stable");
        assert_eq!(second.row[0], repeat.row[0]);
    }
    assert!(
        paged >= 5,
        "the seeded World should give several tables more than one row, got {paged}"
    );

    // Every row object carries exactly the introspected columns.
    let entity_row = row::list_row(&pool, "entity", None, Bound::new(Some(100)).unwrap())
        .await
        .expect("the entity table pages");
    let first = entity_row.row.first().expect("the World holds Entities");
    let mut key = first
        .as_object()
        .expect("a row is a JSON object")
        .keys()
        .cloned()
        .collect::<Vec<_>>();
    key.sort();
    let mut column = entity_row.column.clone();
    column.sort();
    assert_eq!(key, column);
}

#[sqlx::test(migrations = "../game/migration")]
async fn the_row_viewer_only_reads_a_table_live_introspection_names(pool: PgPool) {
    seed(&pool).await;
    let bound = Bound::new(None).expect("the default bound");

    for rejected in [
        "no_such_table",
        "entity\"",
        "\"entity\"",
        "entity; DROP TABLE entity",
        "entity--",
        "pg_class",
        "_sqlx_migrations",
        "",
    ] {
        assert!(
            not_found(row::list_row(&pool, rejected, None, bound).await),
            "the row viewer must refuse {rejected:?}"
        );
    }

    // A cursor that does not match the table's primary key is refused, not guessed.
    assert!(not_found(
        row::list_row(
            &pool,
            "entity_property",
            Some(&[Uuid::new_v4().to_string()]),
            bound
        )
        .await
    ));
    assert!(
        matches!(
            row::list_row(&pool, "entity", Some(&["not-a-uuid".to_owned()]), bound).await,
            Err(StudioError::Database(_))
        ),
        "a cursor value of the wrong type is refused by the column's own cast"
    );
}

#[sqlx::test(migrations = "../game/migration")]
async fn the_row_viewer_refuses_a_live_table_without_a_primary_key(pool: PgPool) {
    sqlx::query("CREATE TABLE studio_no_primary_key (value text NOT NULL)")
        .execute(&pool)
        .await
        .expect("the fixture table is created");
    sqlx::query("INSERT INTO studio_no_primary_key (value) VALUES ('one'), ('two')")
        .execute(&pool)
        .await
        .expect("the fixture rows are inserted");

    assert!(
        matches!(
            row::list_row(
                &pool,
                "studio_no_primary_key",
                None,
                Bound::new(Some(1)).unwrap()
            )
            .await,
            Err(StudioError::UnpageableTable)
        ),
        "a LIMIT cannot make physical no-PK paging bounded and stable"
    );
}

#[sqlx::test(migrations = "../game/migration")]
async fn place_character_user_and_investigation_reads_cross_link_their_subjects(pool: PgPool) {
    let seed = seed(&pool).await;
    let bound = Bound::new(Some(100)).expect("the maximum bound");

    let place_list = place::list_place(&pool, None, bound)
        .await
        .expect("the Place list reads");
    assert_eq!(place_list.item.len(), 1);
    assert_eq!(place_list.item[0].id, seed.place_entity_id);
    assert!(place_list.item[0].is_entry);

    let place_detail = place::get_place(&pool, seed.place_entity_id)
        .await
        .expect("the Place detail reads");
    assert_eq!(place_detail.name, "North Gate");
    assert!(place_detail.is_entry);
    assert_eq!(place_detail.latest_activity_id, seed.change_activity_id);
    assert_eq!(place_detail.latest_activity_operation, "submit_action");

    let member = place::list_place_entity(&pool, seed.place_entity_id, None, bound)
        .await
        .expect("the Place membership reads");
    let member_id = member.item.iter().map(|item| item.id).collect::<Vec<_>>();
    assert!(
        seed.introduced_entity_id
            .iter()
            .all(|id| member_id.contains(id))
    );
    assert!(
        seed.unplaced_entity_id
            .iter()
            .all(|id| !member_id.contains(id)),
        "an Entity created outside a Place is not located at one"
    );

    let present = place::list_place_character(&pool, seed.place_entity_id, None, bound)
        .await
        .expect("the present Characters read");
    assert_eq!(present.item.len(), 1);
    assert_eq!(present.item[0].id, seed.character_entity_id);
    assert_eq!(present.item[0].owner_user_id, seed.user_id);

    let character_list = character::list_character(&pool, None, bound)
        .await
        .expect("the Character list reads");
    assert_eq!(character_list.item.len(), 1);
    assert_eq!(character_list.item[0].id, seed.character_entity_id);
    assert_eq!(
        character_list.item[0].current_place_name.as_deref(),
        Some("North Gate")
    );

    let character_detail = character::get_character(&pool, seed.character_entity_id)
        .await
        .expect("the Character detail reads");
    assert_eq!(character_detail.owner_user_id, seed.user_id);
    assert_eq!(
        character_detail
            .current_place
            .as_ref()
            .map(|place| place.id),
        Some(seed.place_entity_id)
    );
    assert_eq!(character_detail.current_place_is_entry, Some(true));

    let user_detail = user::get_user(&pool, seed.user_id)
        .await
        .expect("the User detail reads");
    assert_eq!(
        user_detail.character.as_ref().map(|character| character.id),
        Some(seed.character_entity_id)
    );
    assert_eq!(
        user_detail.introduced_entity_note,
        user::INTRODUCED_ENTITY_NOTE
    );
    assert!(user_detail.introduced_entity_note.contains("no index"));

    let user_attempt = investigation::list_user_attempt(&pool, seed.user_id, None, bound)
        .await
        .expect("the User attempts read");
    assert_eq!(user_attempt.item.len(), 2);
    assert!(
        user_attempt
            .item
            .windows(2)
            .all(|pair| pair[0].request_id < pair[1].request_id),
        "one User's attempts follow the existing User/request-id index"
    );
    let first_attempt =
        investigation::list_user_attempt(&pool, seed.user_id, None, Bound::new(Some(1)).unwrap())
            .await
            .expect("the first User-attempt page reads");
    assert!(first_attempt.truncated);
    let second_attempt = investigation::list_user_attempt(
        &pool,
        seed.user_id,
        first_attempt.next_cursor,
        Bound::new(Some(1)).unwrap(),
    )
    .await
    .expect("the second User-attempt page reads");
    assert_ne!(first_attempt.item[0].id, second_attempt.item[0].id);
    assert!(first_attempt.item[0].request_id < second_attempt.item[0].request_id);
    let character_attempt =
        character::list_character_attempt(&pool, seed.character_entity_id, None, bound)
            .await
            .expect("the Character attempts read");
    assert_eq!(
        character_attempt
            .item
            .iter()
            .map(|item| item.id)
            .collect::<Vec<_>>(),
        user_attempt
            .item
            .iter()
            .map(|item| item.id)
            .collect::<Vec<_>>(),
        "a Character's attempts are exactly its owning User's attempts"
    );

    let attempt_detail = investigation::get_investigation(&pool, seed.attempt_id)
        .await
        .expect("the attempt detail reads");
    assert_eq!(attempt_detail.character.id, seed.character_entity_id);
    assert_eq!(attempt_detail.place.id, seed.place_entity_id);
    assert_eq!(attempt_detail.request_id, seed.attempt_request_id);
    assert!(["zero", "positive"].contains(&attempt_detail.outcome.as_str()));
    let attempt_document =
        serde_json::to_value(&attempt_detail).expect("the attempt detail serializes");
    assert!(
        attempt_document.get("voided_attempt").is_none(),
        "only the forward voided-by link belongs to the bounded detail"
    );

    let attempt_list = investigation::list_investigation(&pool, None, bound)
        .await
        .expect("the attempt list reads");
    assert_eq!(attempt_list.order, investigation::ATTEMPT_ID_ORDER);
    assert_eq!(attempt_list.page.item.len(), 2);
    assert!(
        attempt_list
            .page
            .item
            .windows(2)
            .all(|pair| pair[0].id < pair[1].id)
    );

    assert!(not_found(place::get_place(&pool, Uuid::new_v4()).await));
    assert!(not_found(
        character::get_character(&pool, Uuid::new_v4()).await
    ));
    assert!(not_found(user::get_user(&pool, Uuid::new_v4()).await));
    assert!(not_found(
        investigation::get_investigation(&pool, Uuid::new_v4()).await
    ));
}

#[sqlx::test(migrations = "../game/migration")]
async fn property_key_reads_name_type_and_first_use_without_holder_reversal(pool: PgPool) {
    seed(&pool).await;
    let bound = Bound::new(Some(100)).expect("the maximum bound");

    let key = property::list_property_key(&pool, None, bound)
        .await
        .expect("the Property key list reads");
    let listed = key
        .item
        .iter()
        .map(|item| item.key.clone())
        .collect::<Vec<_>>();
    assert_eq!(
        listed,
        vec!["height_cm", "page_count", "role", "surface"],
        "Property keys page in canonical key order"
    );
    assert!(!key.truncated);

    let first_page = property::list_property_key(&pool, None, Bound::new(Some(2)).unwrap())
        .await
        .expect("the first Property key page reads");
    assert_eq!(first_page.next_cursor.as_deref(), Some("page_count"));
    let second_page = property::list_property_key(
        &pool,
        first_page.next_cursor.as_deref(),
        Bound::new(Some(2)).unwrap(),
    )
    .await
    .expect("the second Property key page reads");
    assert_eq!(
        second_page
            .item
            .iter()
            .map(|item| item.key.clone())
            .collect::<Vec<_>>(),
        vec!["role", "surface"]
    );

    let detail = property::get_property_key(&pool, "height_cm")
        .await
        .expect("the Property key detail reads");
    assert_eq!(detail.value_type, "integer");
    assert_eq!(detail.first_activity_operation, "submit_action");
    let document = serde_json::to_value(&detail).expect("the Property-key detail serializes");
    assert!(
        document.get("holder").is_none(),
        "the detail has no reverse current-holder scan"
    );

    assert!(not_found(
        property::get_property_key(&pool, "no_such_key").await
    ));
}

#[sqlx::test(migrations = "../game/migration")]
async fn activity_detail_names_everything_one_accepted_mutation_recorded(pool: PgPool) {
    let seed = seed(&pool).await;

    let detail = chronicle::get_activity(&pool, seed.change_activity_id)
        .await
        .expect("the Action detail reads");
    assert_eq!(detail.operation, "submit_action");
    assert_eq!(
        detail.action_consequence.as_deref(),
        Some("change_entity_state")
    );
    assert_eq!(detail.requested_by_user_id, seed.user_id);
    assert_eq!(detail.request_id, Some(seed.change_request_id));
    assert_eq!(
        detail.actor_character.as_ref().map(|actor| actor.id),
        Some(seed.character_entity_id)
    );
    assert_eq!(
        detail.context_place.as_ref().map(|place| place.id),
        Some(seed.place_entity_id)
    );
    assert_eq!(detail.involved_entity.len(), 2);
    assert!(!detail.involved_entity_truncated);
    assert_eq!(detail.property_change.len(), 1);
    assert_eq!(detail.property_change[0].key, "height_cm");
    assert_eq!(detail.property_change[0].integer_value, Some(64));
    assert!(detail.property_change[0].previous_activity_id.is_some());
    assert_eq!(detail.trait_change.len(), 1);
    assert_eq!(detail.trait_change[0].trait_id, seed.changed_trait_id);
    assert!(detail.trait_change[0].previous_activity_id.is_some());
    assert!(detail.consumed_investigation_attempt_id.is_none());

    assert!(not_found(
        chronicle::get_activity(&pool, Uuid::new_v4()).await
    ));
}

#[sqlx::test(migrations = "../game/migration")]
async fn a_hundred_and_one_rows_page_as_one_hundred_rows_and_a_cursor(pool: PgPool) {
    let world = World::new(pool.clone());
    let user = world.create_user().await.expect("a User is created");
    for index in 0..101 {
        world
            .create_entity(
                user.id,
                CreateEntity {
                    name: format!("Survey note {index:03}"),
                    description: format!("Note {index:03} from the long survey run."),
                    property: Vec::new(),
                    r#trait: Vec::new(),
                },
            )
            .await
            .expect("the User creates an Entity");
    }
    let bound = Bound::new(Some(100)).expect("the maximum bound");

    let first = entity::list_entity(&pool, None, bound)
        .await
        .expect("the Entity list reads");
    assert_eq!(first.item.len(), 100);
    assert!(first.truncated);
    let last = entity::list_entity(&pool, first.next_cursor, bound)
        .await
        .expect("the last Entity page reads");
    assert_eq!(last.item.len(), 1);
    assert!(!last.truncated);
    assert!(last.next_cursor.is_none());

    // Each Entity created one Activity, so the World chronicle stops at 100 too.
    let chronicle = chronicle::list_world_chronicle(&pool, None, bound)
        .await
        .expect("the World chronicle reads");
    assert_eq!(chronicle.page.item.len(), 100);
    assert!(chronicle.page.truncated);
    let last = chronicle::list_world_chronicle(&pool, chronicle.page.next_cursor, bound)
        .await
        .expect("the last World chronicle page reads");
    assert_eq!(last.page.item.len(), 1);
    assert!(!last.page.truncated);

    // The generic row viewer obeys the same one bound over the raw table.
    let first = row::list_row(&pool, "entity", None, bound)
        .await
        .expect("the entity table pages");
    assert_eq!(first.row.len(), 100);
    assert!(first.truncated);
    let cursor = first.next_cursor.expect("a truncated page keys on");
    let last = row::list_row(&pool, "entity", Some(&cursor), bound)
        .await
        .expect("the last entity row page reads");
    assert_eq!(last.row.len(), 1);
    assert!(!last.truncated);
    assert!(last.next_cursor.is_none());
}
