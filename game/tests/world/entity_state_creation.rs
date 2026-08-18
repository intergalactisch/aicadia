use super::*;

#[sqlx::test(migrations = "./migration")]
async fn creation_accepts_one_hundred_properties_and_traits_in_one_activity(pool: PgPool) {
    let world = World::new(pool.clone());
    let user_id = create_user(&world).await;
    let entity = world
        .create_entity(
            user_id,
            CreateEntity {
                name: "Hundred-state Frog".to_owned(),
                description: "A heat-scorched frog with three legs.".to_owned(),
                property: (0..100)
                    .map(|index| integer_property(format!("measure_{index}"), index))
                    .collect(),
                r#trait: (0..100)
                    .map(|index| TraitInput {
                        statement: format!("Distinct bounded characterization {index}."),
                    })
                    .collect(),
            },
        )
        .await
        .unwrap();

    let counts: (i64, i64, i64, i64, i64, i64) = sqlx::query_as(
        r#"
        SELECT
            (SELECT count(*) FROM entity_property_history WHERE entity_id = $1),
            (SELECT count(*) FROM entity_property WHERE entity_id = $1),
            (SELECT count(*) FROM entity_trait WHERE entity_id = $1),
            (SELECT count(*) FROM entity_trait_version WHERE entity_id = $1),
            (SELECT count(*) FROM entity_trait_current WHERE entity_id = $1),
            (
                SELECT count(DISTINCT version.activity_id)
                FROM entity_trait_version AS version
                JOIN activity_entity AS involved
                  ON involved.activity_id = version.activity_id
                 AND involved.entity_id = version.entity_id
                 AND involved.role = 'subject'
                WHERE version.entity_id = $1
            )
        "#,
    )
    .bind(entity.id.0)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(counts, (100, 100, 100, 100, 100, 1));
}

#[sqlx::test(migrations = "./migration")]
async fn duplicate_initial_traits_reject_every_creation_route_without_orphans(pool: PgPool) {
    let world = World::new(pool.clone());
    let user_id = create_user(&world).await;
    let duplicate = || {
        vec![
            TraitInput {
                statement: "Jumps unusually high.".to_owned(),
            },
            TraitInput {
                statement: "  Jumps unusually high.  ".to_owned(),
            },
        ]
    };

    assert_eq!(
        world
            .create_character(
                user_id,
                CreateCharacter {
                    name: "Rejected Character".to_owned(),
                    description: "Must leave no orphan.".to_owned(),
                    property: vec![integer_property("leg_count", 3)],
                    r#trait: duplicate(),
                },
            )
            .await,
        Err(WorldError::InvalidTrait)
    );
    world
        .create_character(user_id, character("Mara"))
        .await
        .unwrap();
    assert_eq!(
        world
            .create_entry_place(
                user_id,
                CreateEntryPlace {
                    name: "Rejected Place".to_owned(),
                    description: "Must leave no orphan.".to_owned(),
                    property: vec![text_property("surface", "scorched")],
                    r#trait: duplicate(),
                },
            )
            .await,
        Err(WorldError::InvalidTrait)
    );
    world
        .create_entry_place(user_id, place("Frog Court"))
        .await
        .unwrap();
    assert_eq!(
        world
            .create_entity(
                user_id,
                CreateEntity {
                    name: "Rejected Frog".to_owned(),
                    description: "Must leave no orphan.".to_owned(),
                    property: vec![integer_property("leg_count", 3)],
                    r#trait: duplicate(),
                },
            )
            .await,
        Err(WorldError::InvalidTrait)
    );
    world.enter_world(user_id).await.unwrap();
    let revision = world
        .list_entity_at_current_place(user_id, ListEntityAtCurrentPlace::default())
        .await
        .unwrap()
        .place_revision;
    assert_eq!(
        world
            .submit_action(
                user_id,
                SubmitAction {
                    request_id: Uuid::new_v4(),
                    expected_place_revision: revision,
                    prose: "Mara introduces a frog, but its duplicated state is rejected."
                        .to_owned(),
                    consequence: ActionConsequence::IntroduceEntity(IntroduceEntity {
                        name: "Rejected introduced Frog".to_owned(),
                        description: "Must leave no orphan.".to_owned(),
                        property: vec![integer_property("leg_count", 3)],
                        r#trait: duplicate(),
                    }),
                },
            )
            .await,
        Err(WorldError::InvalidTrait)
    );

    let rejected: (i64, i64, i64, i64) = sqlx::query_as(
        r#"
        SELECT
            (SELECT count(*) FROM entity WHERE name LIKE 'Rejected%'),
            (SELECT count(*) FROM activity WHERE prose LIKE '%duplicated state%'),
            (SELECT count(*) FROM entity_property_history
             JOIN entity ON entity.id = entity_property_history.entity_id
             WHERE entity.name LIKE 'Rejected%'),
            (SELECT count(*) FROM entity_trait_version
             JOIN entity ON entity.id = entity_trait_version.entity_id
             WHERE entity.name LIKE 'Rejected%')
        "#,
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(rejected, (0, 0, 0, 0));
}

#[sqlx::test(migrations = "./migration")]
async fn initial_trait_storage_failure_rolls_back_entity_property_activity_and_trait(pool: PgPool) {
    let world = World::new(pool.clone());
    let user_id = create_user(&world).await;
    sqlx::raw_sql(
        r#"
        CREATE FUNCTION fail_initial_trait_current_write() RETURNS trigger
        LANGUAGE plpgsql AS $$
        BEGIN
            RAISE EXCEPTION 'forced initial Trait pointer failure';
        END;
        $$;
        CREATE TRIGGER fail_initial_trait_current_write
            BEFORE INSERT OR UPDATE ON entity_trait_current
            FOR EACH STATEMENT EXECUTE FUNCTION fail_initial_trait_current_write();
        "#,
    )
    .execute(&pool)
    .await
    .unwrap();

    assert_eq!(
        world
            .create_entity(
                user_id,
                CreateEntity {
                    name: "Rolled-back Frog".to_owned(),
                    description: "Must leave no partial state.".to_owned(),
                    property: vec![integer_property("leg_count", 3)],
                    r#trait: vec![TraitInput {
                        statement: "Jumps unusually high.".to_owned(),
                    }],
                },
            )
            .await,
        Err(WorldError::Unavailable)
    );
    let counts: (i64, i64, i64, i64) = sqlx::query_as(
        r#"
        SELECT
            (SELECT count(*) FROM entity WHERE name = 'Rolled-back Frog'),
            (SELECT count(*) FROM activity_entity
             JOIN entity ON entity.id = activity_entity.entity_id
             WHERE entity.name = 'Rolled-back Frog'),
            (SELECT count(*) FROM entity_property_history
             JOIN entity ON entity.id = entity_property_history.entity_id
             WHERE entity.name = 'Rolled-back Frog'),
            (SELECT count(*) FROM entity_trait_version
             JOIN entity ON entity.id = entity_trait_version.entity_id
             WHERE entity.name = 'Rolled-back Frog')
        "#,
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(counts, (0, 0, 0, 0));
}
