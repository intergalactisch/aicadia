use super::*;

#[sqlx::test(migrations = "./migration")]
async fn every_entity_creation_route_atomically_establishes_one_hundred_properties(pool: PgPool) {
    let world = World::new(pool.clone());
    let user_id = create_user(&world).await;
    let hundred = || {
        (0..100)
            .map(|index| integer_property(format!("measure_{index}"), index))
            .collect::<Vec<_>>()
    };

    let ordinary = world
        .create_entity(
            user_id,
            CreateEntity {
                name: "Unplaced Herbarium".to_owned(),
                description: "One hundred measured specimens.".to_owned(),
                property: hundred(),
                r#trait: Vec::new(),
            },
        )
        .await
        .unwrap();
    let character = world
        .create_character(
            user_id,
            CreateCharacter {
                name: "Mara Venn".to_owned(),
                description: "A patient surveyor.".to_owned(),
                property: hundred(),
                r#trait: Vec::new(),
            },
        )
        .await
        .unwrap();
    let place = world
        .create_entry_place(
            user_id,
            CreateEntryPlace {
                name: "North Gate".to_owned(),
                description: "The shared threshold.".to_owned(),
                property: hundred(),
                r#trait: Vec::new(),
            },
        )
        .await
        .unwrap();
    world.enter_world(user_id).await.unwrap();
    let revision = world
        .list_entity_at_current_place(user_id, ListEntityAtCurrentPlace::default())
        .await
        .unwrap()
        .place_revision;
    let introduced = world
        .submit_action(
            user_id,
            SubmitAction {
                request_id: Uuid::new_v4(),
                expected_place_revision: revision,
                prose: "Mara establishes a measured cairn.".to_owned(),
                consequence: ActionConsequence::IntroduceEntity(IntroduceEntity {
                    name: "Measured Cairn".to_owned(),
                    description: "A cairn with one hundred recorded measures.".to_owned(),
                    position_description: None,
                    property: hundred(),
                    r#trait: Vec::new(),
                }),
            },
        )
        .await
        .unwrap();
    assert_eq!(introduced.activity.property_change.len(), 100);
    assert_eq!(introduced_entity(&introduced).name, "Measured Cairn");

    let counts: (i64, i64, i64, i64) = sqlx::query_as(
        r#"
        SELECT
            (SELECT count(*) FROM property_key),
            (SELECT count(*) FROM entity_property_history),
            (SELECT count(*) FROM entity_property),
            (SELECT count(DISTINCT activity_id) FROM entity_property_history)
        "#,
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(counts, (100, 400, 400, 4));
    for entity_id in [
        ordinary.id,
        character.entity.id,
        place.entity.id,
        introduced_entity(&introduced).id,
    ] {
        let count: i64 =
            sqlx::query_scalar("SELECT count(*) FROM entity_property WHERE entity_id = $1")
                .bind(entity_id.0)
                .fetch_one(&pool)
                .await
                .unwrap();
        assert_eq!(count, 100);
    }
}

#[sqlx::test(migrations = "./migration")]
async fn invalid_initial_properties_roll_back_each_creation_route_without_orphans(pool: PgPool) {
    let world = World::new(pool.clone());
    let user_id = create_user(&world).await;
    let over_bound = || {
        (0..101)
            .map(|index| integer_property(format!("measure_{index}"), index))
            .collect::<Vec<_>>()
    };
    assert_eq!(
        world
            .create_entity(
                user_id,
                CreateEntity {
                    name: "Rejected Entity".to_owned(),
                    description: "Must not persist.".to_owned(),
                    property: over_bound(),
                    r#trait: Vec::new(),
                },
            )
            .await,
        Err(WorldError::InvalidProperty {
            field: PropertyField::Property,
            reason: InvalidReason::OutOfRange,
        })
    );
    assert_eq!(
        world
            .create_character(
                user_id,
                CreateCharacter {
                    name: "Rejected Character".to_owned(),
                    description: "Must not persist.".to_owned(),
                    property: over_bound(),
                    r#trait: Vec::new(),
                },
            )
            .await,
        Err(WorldError::InvalidProperty {
            field: PropertyField::Property,
            reason: InvalidReason::OutOfRange,
        })
    );
    world
        .create_character(user_id, character("Mara Venn"))
        .await
        .unwrap();
    assert_eq!(
        world
            .create_entry_place(
                user_id,
                CreateEntryPlace {
                    name: "Rejected Gate".to_owned(),
                    description: "Must not persist.".to_owned(),
                    property: vec![
                        text_property("colour", "grey"),
                        text_property("colour", "red")
                    ],
                    r#trait: Vec::new(),
                },
            )
            .await,
        Err(WorldError::InvalidProperty {
            field: PropertyField::Property,
            reason: InvalidReason::Duplicate,
        })
    );
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
    assert_eq!(
        world
            .submit_action(
                user_id,
                SubmitAction {
                    request_id: Uuid::new_v4(),
                    expected_place_revision: revision,
                    prose: "This introduction is invalid.".to_owned(),
                    consequence: ActionConsequence::IntroduceEntity(IntroduceEntity {
                        name: "Rejected Marker".to_owned(),
                        description: "Must not persist.".to_owned(),
                        position_description: None,
                        property: over_bound(),
                        r#trait: Vec::new(),
                    }),
                },
            )
            .await,
        Err(WorldError::InvalidProperty {
            field: PropertyField::Property,
            reason: InvalidReason::OutOfRange,
        })
    );
    let counts: (i64, i64, i64, i64) = sqlx::query_as(
        r#"
        SELECT
            (SELECT count(*) FROM entity WHERE name LIKE 'Rejected%'),
            (SELECT count(*) FROM activity WHERE prose = 'This introduction is invalid.'),
            (SELECT count(*) FROM property_key),
            (SELECT count(*) FROM entity_property_history)
        "#,
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(counts, (0, 0, 0, 0));
}

#[sqlx::test(migrations = "./migration")]
async fn property_action_changes_actor_place_ordinary_and_other_character_uniformly(pool: PgPool) {
    let world = World::new(pool.clone());
    let (place, participant) = entered_characters(&world, &["Mara Venn", "Pip"]).await;
    let (mara_user, mara) = participant[0];
    let (_, pip) = participant[1];
    let revision = world
        .list_entity_at_current_place(mara_user, ListEntityAtCurrentPlace::default())
        .await
        .unwrap()
        .place_revision;
    let bowl_action = world
        .submit_action(mara_user, action(Uuid::new_v4(), revision, "Copper Bowl"))
        .await
        .unwrap();
    let bowl = introduced_entity(&bowl_action).id;
    let revision = world
        .list_entity_at_current_place(mara_user, ListEntityAtCurrentPlace::default())
        .await
        .unwrap()
        .place_revision;
    let accepted = world
        .submit_action(
            mara_user,
            property_action(
                Uuid::new_v4(),
                revision,
                "The blast stains every nearby surface crimson.",
                vec![
                    property_change(
                        place.entity.id,
                        "colour",
                        PropertyValue::Text("crimson".into()),
                    ),
                    property_change(pip, "colour", PropertyValue::Text("crimson".into())),
                    property_change(bowl, "colour", PropertyValue::Text("crimson".into())),
                    property_change(mara, "colour", PropertyValue::Text("crimson".into())),
                    property_change(mara, "hair_colour", PropertyValue::Text("red".into())),
                ],
            ),
        )
        .await
        .unwrap();
    let change = match &accepted.consequence {
        AcceptedActionConsequence::ChangeEntityState {
            property_change, ..
        } => property_change,
        AcceptedActionConsequence::IntroduceEntity(_) => panic!("expected Property change"),
    };
    assert_eq!(change, &accepted.activity.property_change);
    assert_eq!(change.len(), 5);
    assert!(change.windows(2).all(|pair| {
        (pair[0].entity.id.0.as_bytes(), pair[0].key.as_str())
            <= (pair[1].entity.id.0.as_bytes(), pair[1].key.as_str())
    }));
    let subject = accepted
        .activity
        .involved_entity
        .iter()
        .filter(|reference| reference.role == ActivityEntityRole::Subject)
        .map(|reference| reference.entity.id)
        .collect::<std::collections::HashSet<_>>();
    assert_eq!(
        subject,
        [mara, pip, bowl, place.entity.id].into_iter().collect()
    );
    assert!(accepted.activity.involved_entity.iter().any(|reference| {
        reference.entity.id == place.entity.id && reference.role == ActivityEntityRole::Location
    }));

    let mut current_property = Vec::new();
    for entity_id in [mara, pip, bowl, place.entity.id] {
        let current = world
            .get_entity_at_current_place(
                mara_user,
                GetEntityAtCurrentPlace {
                    entity_id,
                    cursor: None,
                    limit: 100,
                },
            )
            .await
            .unwrap();
        assert_eq!(current.place.entity.id, place.entity.id);
        current_property.extend(current.current_state.association.into_iter().filter_map(
            |association| match association {
                EntityCurrentAssociation::Property { key, value } => Some((key, value)),
                EntityCurrentAssociation::Trait(_) => None,
            },
        ));
    }
    assert_eq!(current_property.len(), 5);
    assert!(current_property.iter().all(|(key, value)| {
        key == "hair_colour"
            || value == &PropertyValue::Text("crimson".to_owned())
            || value == &PropertyValue::Text("red".to_owned())
    }));
    let colour_key_count: i64 =
        sqlx::query_scalar("SELECT count(*) FROM property_key WHERE key = 'colour'")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(colour_key_count, 1);
    let one_activity: i64 = sqlx::query_scalar(
        "SELECT count(DISTINCT activity_id) FROM entity_property_history WHERE activity_id = $1",
    )
    .bind(accepted.activity.id.0)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(one_activity, 1);
}

#[sqlx::test(migrations = "./migration")]
async fn property_action_rejects_unavailable_mixed_subjects_and_retries_sorted_history(
    pool: PgPool,
) {
    let world = World::new(pool.clone());
    let user_id = create_user(&world).await;
    let actor = world
        .create_character(user_id, character("Mara Venn"))
        .await
        .unwrap();
    world
        .create_entry_place(user_id, place("North Gate"))
        .await
        .unwrap();
    world.enter_world(user_id).await.unwrap();
    let unavailable = world
        .create_entity(
            user_id,
            CreateEntity {
                name: "Unplaced Remote Stone".to_owned(),
                description: "It has no current Place.".to_owned(),
                property: vec![text_property("colour", "grey")],
                r#trait: Vec::new(),
            },
        )
        .await
        .unwrap();
    let revision = world
        .list_entity_at_current_place(user_id, ListEntityAtCurrentPlace::default())
        .await
        .unwrap()
        .place_revision;
    let before_activity: i64 = sqlx::query_scalar("SELECT count(*) FROM activity")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(
        world
            .submit_action(
                user_id,
                property_action(
                    Uuid::new_v4(),
                    revision,
                    "A change cannot reach the absent stone.",
                    vec![
                        property_change(
                            actor.entity.id,
                            "colour",
                            PropertyValue::Text("red".into()),
                        ),
                        property_change(
                            unavailable.id,
                            "colour",
                            PropertyValue::Text("red".into()),
                        ),
                    ],
                ),
            )
            .await,
        Err(WorldError::PropertyEntityUnavailable)
    );
    let after_activity: i64 = sqlx::query_scalar("SELECT count(*) FROM activity")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(after_activity, before_activity);
    let actor_colour: i64 = sqlx::query_scalar(
        r#"
        SELECT count(*)
        FROM entity_property
        JOIN property_key ON property_key.id = entity_property.property_key_id
        WHERE entity_property.entity_id = $1 AND property_key.key = 'colour'
        "#,
    )
    .bind(actor.entity.id.0)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(actor_colour, 0);

    let request_id = Uuid::new_v4();
    let original = world
        .submit_action(
            user_id,
            property_action(
                request_id,
                revision,
                "Mara changes two facts.",
                vec![
                    property_change(actor.entity.id, "size", PropertyValue::Text("small".into())),
                    property_change(actor.entity.id, "leg_count", PropertyValue::Integer(3)),
                ],
            ),
        )
        .await
        .unwrap();
    let later_revision = world
        .list_entity_at_current_place(user_id, ListEntityAtCurrentPlace::default())
        .await
        .unwrap()
        .place_revision;
    world
        .submit_action(
            user_id,
            property_action(
                Uuid::new_v4(),
                later_revision,
                "Mara grows again.",
                vec![property_change(
                    actor.entity.id,
                    "size",
                    PropertyValue::Text("tall".into()),
                )],
            ),
        )
        .await
        .unwrap();
    let retry = world
        .submit_action(
            user_id,
            property_action(
                request_id,
                revision,
                "Mara changes two facts.",
                vec![
                    property_change(actor.entity.id, "leg_count", PropertyValue::Integer(3)),
                    property_change(actor.entity.id, "size", PropertyValue::Text("small".into())),
                ],
            ),
        )
        .await
        .unwrap();
    assert_eq!(retry, original);
    assert_eq!(
        world
            .submit_action(
                user_id,
                property_action(
                    request_id,
                    revision,
                    "Mara changes two facts.",
                    vec![
                        property_change(actor.entity.id, "leg_count", PropertyValue::Integer(4)),
                        property_change(
                            actor.entity.id,
                            "size",
                            PropertyValue::Text("small".into())
                        ),
                    ],
                ),
            )
            .await,
        Err(WorldError::ActionRequestConflict)
    );
}

#[sqlx::test(migrations = "./migration")]
async fn property_world_validation_rejects_keys_values_duplicates_and_change_bounds(pool: PgPool) {
    let world = World::new(pool.clone());
    let user_id = create_user(&world).await;
    let actor = world
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

    assert_eq!(
        world
            .submit_action(
                user_id,
                property_action(
                    Uuid::new_v4(),
                    revision,
                    "This empty state change must not be accepted.",
                    Vec::new(),
                ),
            )
            .await,
        Err(WorldError::InvalidAction {
            field: ActionField::Consequence,
            reason: InvalidReason::Empty,
        })
    );
    let invalid = vec![
        (
            vec![property_change(
                actor.entity.id,
                "HairColour",
                PropertyValue::Text("red".into()),
            )],
            PropertyField::Key,
            InvalidReason::InvalidFormat,
        ),
        (
            vec![property_change(
                actor.entity.id,
                "hair_colour",
                PropertyValue::Text(" \0 ".into()),
            )],
            PropertyField::Value,
            InvalidReason::ContainsNul,
        ),
        (
            vec![
                property_change(
                    actor.entity.id,
                    "hair_colour",
                    PropertyValue::Text("red".into()),
                ),
                property_change(
                    actor.entity.id,
                    "hair_colour",
                    PropertyValue::Text("blue".into()),
                ),
            ],
            PropertyField::PropertyChange,
            InvalidReason::Duplicate,
        ),
        (
            (0..101)
                .map(|index| {
                    property_change(
                        actor.entity.id,
                        format!("measure_{index}"),
                        PropertyValue::Integer(index),
                    )
                })
                .collect(),
            PropertyField::PropertyChange,
            InvalidReason::OutOfRange,
        ),
    ];
    for (property_change, field, reason) in invalid {
        assert_eq!(
            world
                .submit_action(
                    user_id,
                    property_action(
                        Uuid::new_v4(),
                        revision,
                        "This invalid change must not be accepted.",
                        property_change,
                    ),
                )
                .await,
            Err(WorldError::InvalidProperty { field, reason })
        );
    }
    let count: i64 =
        sqlx::query_scalar("SELECT count(*) FROM activity WHERE operation = 'submit_action'")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(count, 0);
}

#[sqlx::test(migrations = "./migration")]
async fn interaction_property_changes_actor_and_target_without_authoring_a_response(pool: PgPool) {
    let world = World::new(pool.clone());
    let (_, participant) = entered_characters(&world, &["Pip", "Mara", "Tomas"]).await;
    let (pip_user, pip) = participant[0];
    let (mara_user, mara) = participant[1];
    let (_, tomas) = participant[2];
    let revision = world
        .list_entity_at_current_place(pip_user, ListEntityAtCurrentPlace::default())
        .await
        .unwrap()
        .place_revision;
    let request_id = Uuid::new_v4();
    let accepted = world
        .submit_interaction(
            pip_user,
            SubmitInteraction {
                request_id,
                expected_place_revision: revision,
                prose: "Pip splashes Mara with blue dye.".to_owned(),
                target_entity_id: vec![mara],
                property_change: vec![
                    property_change(mara, "colour", PropertyValue::Text("blue".into())),
                    property_change(pip, "colour", PropertyValue::Text("blue".into())),
                ],
                trait_change: Vec::new(),
            },
        )
        .await
        .unwrap();
    assert_eq!(accepted.activity.property_change.len(), 2);
    assert_eq!(
        accepted
            .activity
            .involved_entity
            .iter()
            .filter(|reference| reference.role == ActivityEntityRole::Target)
            .map(|reference| reference.entity.id)
            .collect::<Vec<_>>(),
        vec![mara]
    );
    let mara_history = world
        .list_activity(mara_user, ListActivity::default())
        .await
        .unwrap();
    let observed = mara_history
        .activity
        .iter()
        .find(|activity| activity.id == accepted.activity.id)
        .unwrap();
    assert_eq!(observed, &accepted.activity);
    assert_ne!(observed.actor_character.as_ref().unwrap().id, mara);

    let later_revision = world
        .list_entity_at_current_place(pip_user, ListEntityAtCurrentPlace::default())
        .await
        .unwrap()
        .place_revision;
    world
        .submit_action(
            pip_user,
            property_action(
                Uuid::new_v4(),
                later_revision,
                "The blue dye dries on Pip.",
                vec![property_change(
                    pip,
                    "surface",
                    PropertyValue::Text("dry".into()),
                )],
            ),
        )
        .await
        .unwrap();

    let retry = world
        .submit_interaction(
            pip_user,
            SubmitInteraction {
                request_id,
                expected_place_revision: revision,
                prose: "Pip splashes Mara with blue dye.".to_owned(),
                target_entity_id: vec![mara],
                property_change: vec![
                    property_change(pip, "colour", PropertyValue::Text("blue".into())),
                    property_change(mara, "colour", PropertyValue::Text("blue".into())),
                ],
                trait_change: Vec::new(),
            },
        )
        .await
        .unwrap();
    assert_eq!(retry, accepted);
    let current_revision = world
        .list_entity_at_current_place(pip_user, ListEntityAtCurrentPlace::default())
        .await
        .unwrap()
        .place_revision;
    let before: i64 =
        sqlx::query_scalar("SELECT count(*) FROM activity WHERE operation = 'submit_interaction'")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(
        world
            .submit_interaction(
                pip_user,
                SubmitInteraction {
                    request_id: Uuid::new_v4(),
                    expected_place_revision: current_revision,
                    prose: "Pip cannot affect a bystander without targeting them.".to_owned(),
                    target_entity_id: vec![mara],
                    property_change: vec![property_change(
                        tomas,
                        "colour",
                        PropertyValue::Text("blue".into()),
                    )],
                    trait_change: Vec::new(),
                },
            )
            .await,
        Err(WorldError::PropertyEntityUnavailable)
    );
    let after: i64 =
        sqlx::query_scalar("SELECT count(*) FROM activity WHERE operation = 'submit_interaction'")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(after, before);
}

#[sqlx::test(migrations = "./migration")]
async fn current_property_read_paginates_local_facts_and_excludes_unplaced_entities(pool: PgPool) {
    let world = World::new(pool.clone());
    let user_id = create_user(&world).await;
    let actor = world
        .create_character(
            user_id,
            CreateCharacter {
                name: "Mara Venn".to_owned(),
                description: "Blond hair is introductory history.".to_owned(),
                property: Vec::new(),
                r#trait: Vec::new(),
            },
        )
        .await
        .unwrap();
    let place = world
        .create_entry_place(
            user_id,
            CreateEntryPlace {
                name: "North Gate".to_owned(),
                description: "The threshold.".to_owned(),
                property: vec![text_property("weather", "clear")],
                r#trait: Vec::new(),
            },
        )
        .await
        .unwrap();
    world.enter_world(user_id).await.unwrap();
    let revision = world
        .list_entity_at_current_place(user_id, ListEntityAtCurrentPlace::default())
        .await
        .unwrap()
        .place_revision;
    let mut change = (0..99)
        .map(|index| {
            property_change(
                actor.entity.id,
                format!("measure_{index}"),
                PropertyValue::Integer(index),
            )
        })
        .collect::<Vec<_>>();
    change.push(property_change(
        actor.entity.id,
        "hair_colour",
        PropertyValue::Text("red".into()),
    ));
    let accepted = world
        .submit_action(
            user_id,
            property_action(
                Uuid::new_v4(),
                revision,
                "Mara's hair turns red while ninety-nine measures settle.",
                change,
            ),
        )
        .await
        .unwrap();
    assert_eq!(accepted.activity.property_change.len(), 100);
    let remote = world
        .create_entity(
            user_id,
            CreateEntity {
                name: "Unplaced Almanac".to_owned(),
                description: "Not locally observable.".to_owned(),
                property: vec![text_property("secret_mark", "remote")],
                r#trait: Vec::new(),
            },
        )
        .await
        .unwrap();
    let first = world
        .get_entity_at_current_place(
            user_id,
            GetEntityAtCurrentPlace {
                entity_id: actor.entity.id,
                cursor: None,
                limit: 50,
            },
        )
        .await
        .unwrap();
    assert_eq!(first.current_state.association.len(), 50);
    assert!(first.current_state.next.is_some());
    let second = world
        .get_entity_at_current_place(
            user_id,
            GetEntityAtCurrentPlace {
                entity_id: actor.entity.id,
                cursor: first.current_state.next,
                limit: 50,
            },
        )
        .await
        .unwrap();
    assert_eq!(second.current_state.association.len(), 50);
    assert!(second.current_state.next.is_none());
    let all = first
        .current_state
        .association
        .into_iter()
        .chain(second.current_state.association)
        .collect::<Vec<_>>();
    assert!(all.iter().any(|association| matches!(
        association,
        EntityCurrentAssociation::Property { key, value }
            if key == "hair_colour" && value == &PropertyValue::Text("red".to_owned())
    )));
    let place_state = world
        .get_entity_at_current_place(
            user_id,
            GetEntityAtCurrentPlace {
                entity_id: place.entity.id,
                cursor: None,
                limit: 100,
            },
        )
        .await
        .unwrap();
    assert!(
        place_state
            .current_state
            .association
            .iter()
            .any(|association| matches!(
                association,
                EntityCurrentAssociation::Property { key, value }
                    if key == "weather" && value == &PropertyValue::Text("clear".to_owned())
            ))
    );
    assert_eq!(
        world
            .get_entity_at_current_place(
                user_id,
                GetEntityAtCurrentPlace {
                    entity_id: remote.id,
                    cursor: None,
                    limit: 100,
                },
            )
            .await,
        Err(WorldError::EntityAtCurrentPlaceUnavailable)
    );
    assert_eq!(
        world
            .get_entity_at_current_place(
                user_id,
                GetEntityAtCurrentPlace {
                    entity_id: actor.entity.id,
                    cursor: None,
                    limit: 0,
                },
            )
            .await,
        Err(WorldError::InvalidEntityLimit)
    );
}

#[sqlx::test(migrations = "./migration")]
async fn concurrent_world_first_key_use_reuses_type_and_rolls_back_type_conflict(pool: PgPool) {
    let world = World::new(pool.clone());
    let first_user = create_user(&world).await;
    let second_user = create_user(&world).await;
    let same_first = {
        let world = world.clone();
        tokio::spawn(async move {
            world
                .create_entity(
                    first_user,
                    CreateEntity {
                        name: "First Grey Stone".to_owned(),
                        description: "A concurrent first use.".to_owned(),
                        property: vec![text_property("surface", "rough")],
                        r#trait: Vec::new(),
                    },
                )
                .await
        })
    };
    let same_second = {
        let world = world.clone();
        tokio::spawn(async move {
            world
                .create_entity(
                    second_user,
                    CreateEntity {
                        name: "Second Grey Stone".to_owned(),
                        description: "Another concurrent first use.".to_owned(),
                        property: vec![text_property("surface", "smooth")],
                        r#trait: Vec::new(),
                    },
                )
                .await
        })
    };
    assert!(same_first.await.unwrap().is_ok());
    assert!(same_second.await.unwrap().is_ok());
    let same_state: (i64, i64) = sqlx::query_as(
        r#"
        SELECT
            (SELECT count(*) FROM property_key WHERE key = 'surface'),
            (SELECT count(*) FROM entity_property_history
             JOIN property_key ON property_key.id = entity_property_history.property_key_id
             WHERE property_key.key = 'surface')
        "#,
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(same_state, (1, 2));

    let third_user = create_user(&world).await;
    let fourth_user = create_user(&world).await;
    let text = {
        let world = world.clone();
        tokio::spawn(async move {
            world
                .create_entity(
                    third_user,
                    CreateEntity {
                        name: "Text Weight".to_owned(),
                        description: "One type must win.".to_owned(),
                        property: vec![text_property("weight", "heavy")],
                        r#trait: Vec::new(),
                    },
                )
                .await
        })
    };
    let integer = {
        let world = world.clone();
        tokio::spawn(async move {
            world
                .create_entity(
                    fourth_user,
                    CreateEntity {
                        name: "Integer Weight".to_owned(),
                        description: "The other type must roll back.".to_owned(),
                        property: vec![integer_property("weight", 12)],
                        r#trait: Vec::new(),
                    },
                )
                .await
        })
    };
    let text = text.await.unwrap();
    let integer = integer.await.unwrap();
    assert_eq!(usize::from(text.is_ok()) + usize::from(integer.is_ok()), 1);
    assert_eq!(
        if text.is_err() {
            text.err()
        } else {
            integer.err()
        },
        Some(WorldError::PropertyKeyConflict)
    );
    let conflict_state: (i64, i64, i64) = sqlx::query_as(
        r#"
        SELECT
            (SELECT count(*) FROM property_key WHERE key = 'weight'),
            (SELECT count(*) FROM entity WHERE name IN ('Text Weight', 'Integer Weight')),
            (SELECT count(*) FROM activity
             JOIN activity_entity ON activity_entity.activity_id = activity.id
             JOIN entity ON entity.id = activity_entity.entity_id
             WHERE entity.name IN ('Text Weight', 'Integer Weight'))
        "#,
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(conflict_state, (1, 1, 1));
}

#[sqlx::test(migrations = "./migration")]
async fn reversed_combined_state_actions_at_distinct_places_complete_without_deadlock(
    pool: PgPool,
) {
    let world = World::new(pool.clone());
    let first_user = create_user(&world).await;
    let first_character = world
        .create_character(first_user, character("Northern Surveyor"))
        .await
        .unwrap();
    let first_place = world
        .create_entry_place(first_user, place("North Gate"))
        .await
        .unwrap();
    world.enter_world(first_user).await.unwrap();

    let second_user = create_user(&world).await;
    let second_character = world
        .create_character(second_user, character("Southern Surveyor"))
        .await
        .unwrap();
    let second_place_entity = world
        .create_entity(second_user, entity("South Gate"))
        .await
        .unwrap();
    let second_place_genesis: Uuid = sqlx::query_scalar(
        "SELECT activity_id FROM activity_entity WHERE entity_id = $1 AND role = 'subject'",
    )
    .bind(second_place_entity.id.0)
    .fetch_one(&pool)
    .await
    .unwrap();
    let second_enter_activity = Uuid::new_v4();
    let mut setup = pool.begin().await.unwrap();
    sqlx::query("INSERT INTO position_version (entity_id, activity_id, x_cm, y_cm, z_cm) VALUES ($1, $2, 100, 0, 0)")
        .bind(second_place_entity.id.0)
        .bind(second_place_genesis)
        .execute(&mut *setup)
        .await
        .unwrap();
    sqlx::query("INSERT INTO position (entity_id, current_activity_id) VALUES ($1, $2)")
        .bind(second_place_entity.id.0)
        .bind(second_place_genesis)
        .execute(&mut *setup)
        .await
        .unwrap();
    sqlx::query("INSERT INTO activity_position (activity_id, role, position_entity_id, position_activity_id) VALUES ($1, 'result', $2, $1)")
        .bind(second_place_genesis)
        .bind(second_place_entity.id.0)
        .execute(&mut *setup)
        .await
        .unwrap();
    sqlx::query(
        "INSERT INTO place (entity_id, is_entry, latest_activity_id) VALUES ($1, false, $2)",
    )
    .bind(second_place_entity.id.0)
    .bind(second_place_genesis)
    .execute(&mut *setup)
    .await
    .unwrap();
    sqlx::query("INSERT INTO place_map_index (place_entity_id, position_activity_id, x_cm, y_cm, z_cm) VALUES ($1, $2, 100, 0, 0)")
        .bind(second_place_entity.id.0)
        .bind(second_place_genesis)
        .execute(&mut *setup)
        .await
        .unwrap();
    sqlx::query(
        r#"
        INSERT INTO activity (
            id, operation, requested_by_user_id,
            actor_character_entity_id, context_place_entity_id
        )
        VALUES ($1, 'enter_world', $2, $3, $4)
        "#,
    )
    .bind(second_enter_activity)
    .bind(second_user.0)
    .bind(second_character.entity.id.0)
    .bind(second_place_entity.id.0)
    .execute(&mut *setup)
    .await
    .unwrap();
    sqlx::query("INSERT INTO position_version (entity_id, activity_id, x_cm, y_cm, z_cm) VALUES ($1, $2, 100, 0, 0)")
        .bind(second_character.entity.id.0)
        .bind(second_enter_activity)
        .execute(&mut *setup)
        .await
        .unwrap();
    sqlx::query("INSERT INTO position (entity_id, current_activity_id) VALUES ($1, $2)")
        .bind(second_character.entity.id.0)
        .bind(second_enter_activity)
        .execute(&mut *setup)
        .await
        .unwrap();
    sqlx::query("INSERT INTO activity_position (activity_id, role, position_entity_id, position_activity_id) VALUES ($1, 'result', $2, $1)")
        .bind(second_enter_activity)
        .bind(second_character.entity.id.0)
        .execute(&mut *setup)
        .await
        .unwrap();
    sqlx::query(
        "INSERT INTO activity_entity (activity_id, entity_id, role) VALUES ($1, $2, 'destination')",
    )
    .bind(second_enter_activity)
    .bind(second_place_entity.id.0)
    .execute(&mut *setup)
    .await
    .unwrap();
    sqlx::query("UPDATE character SET current_place_entity_id = $1 WHERE entity_id = $2")
        .bind(second_place_entity.id.0)
        .bind(second_character.entity.id.0)
        .execute(&mut *setup)
        .await
        .unwrap();
    sqlx::query("UPDATE place SET latest_activity_id = $1 WHERE entity_id = $2")
        .bind(second_enter_activity)
        .bind(second_place_entity.id.0)
        .execute(&mut *setup)
        .await
        .unwrap();
    setup.commit().await.unwrap();

    let first_revision = world
        .list_entity_at_current_place(first_user, ListEntityAtCurrentPlace::default())
        .await
        .unwrap()
        .place_revision;
    let second_revision = world
        .list_entity_at_current_place(second_user, ListEntityAtCurrentPlace::default())
        .await
        .unwrap()
        .place_revision;
    assert_ne!(first_place.entity.id, second_place_entity.id);

    let timed_options = pool
        .connect_options()
        .as_ref()
        .clone()
        .options([("statement_timeout", "5s"), ("lock_timeout", "5s")]);
    let timed_pool = PgPoolOptions::new()
        .max_connections(4)
        .connect_with(timed_options)
        .await
        .unwrap();
    let barrier = std::sync::Arc::new(tokio::sync::Barrier::new(3));
    let first = {
        let world = World::new(timed_pool.clone());
        let barrier = barrier.clone();
        tokio::spawn(async move {
            barrier.wait().await;
            world
                .submit_action(
                    first_user,
                    SubmitAction {
                        request_id: Uuid::new_v4(),
                        expected_place_revision: first_revision,
                        prose: "The northern surveyor fixes both calibration marks.".to_owned(),
                        consequence: ActionConsequence::ChangeEntityState(ChangeEntityState {
                            property_change: vec![
                                property_change(
                                    first_character.entity.id,
                                    "lock_alpha",
                                    PropertyValue::Integer(1),
                                ),
                                property_change(
                                    first_character.entity.id,
                                    "lock_beta",
                                    PropertyValue::Integer(2),
                                ),
                            ],
                            trait_change: vec![
                                establish_trait(first_character.entity.id, "Alpha lock observed."),
                                establish_trait(first_character.entity.id, "Beta lock observed."),
                            ],
                        }),
                    },
                )
                .await
        })
    };
    let second = {
        let world = World::new(timed_pool.clone());
        let barrier = barrier.clone();
        tokio::spawn(async move {
            barrier.wait().await;
            world
                .submit_action(
                    second_user,
                    SubmitAction {
                        request_id: Uuid::new_v4(),
                        expected_place_revision: second_revision,
                        prose: "The southern surveyor fixes both calibration marks.".to_owned(),
                        consequence: ActionConsequence::ChangeEntityState(ChangeEntityState {
                            property_change: vec![
                                property_change(
                                    second_character.entity.id,
                                    "lock_beta",
                                    PropertyValue::Integer(20),
                                ),
                                property_change(
                                    second_character.entity.id,
                                    "lock_alpha",
                                    PropertyValue::Integer(10),
                                ),
                            ],
                            trait_change: vec![
                                establish_trait(second_character.entity.id, "Beta lock observed."),
                                establish_trait(second_character.entity.id, "Alpha lock observed."),
                            ],
                        }),
                    },
                )
                .await
        })
    };
    barrier.wait().await;
    let first = first.await.unwrap().unwrap();
    let second = second.await.unwrap().unwrap();
    assert_eq!(first.activity.property_change.len(), 2);
    assert_eq!(second.activity.property_change.len(), 2);
    assert_eq!(first.activity.trait_change.len(), 2);
    assert_eq!(second.activity.trait_change.len(), 2);
    assert_eq!(
        first
            .activity
            .property_change
            .iter()
            .map(|change| change.key.as_str())
            .collect::<Vec<_>>(),
        vec!["lock_alpha", "lock_beta"]
    );
    assert_eq!(
        second
            .activity
            .property_change
            .iter()
            .map(|change| change.key.as_str())
            .collect::<Vec<_>>(),
        vec!["lock_alpha", "lock_beta"]
    );
    let counts: (i64, i64, i64, i64, i64) = sqlx::query_as(
        r#"
        SELECT
            (SELECT count(*) FROM property_key
             WHERE key IN ('lock_alpha', 'lock_beta')),
            (SELECT count(*) FROM entity_property_history
             JOIN property_key ON property_key.id = entity_property_history.property_key_id
             WHERE property_key.key IN ('lock_alpha', 'lock_beta')),
            (SELECT count(*) FROM entity_property
             JOIN property_key ON property_key.id = entity_property.property_key_id
             WHERE property_key.key IN ('lock_alpha', 'lock_beta')),
            (SELECT count(*) FROM entity_trait_version
             WHERE statement IN (
                'Alpha lock observed.',
                'Beta lock observed.'
             )),
            (SELECT count(*) FROM activity
             WHERE operation = 'submit_action'
               AND action_consequence = 'change_entity_state')
        "#,
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(counts, (2, 4, 4, 4, 2));
    timed_pool.close().await;
}

#[sqlx::test(migrations = "./migration")]
async fn property_storage_failure_rolls_back_entity_activity_key_and_place_revision(pool: PgPool) {
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
        .list_entity_at_current_place(user_id, ListEntityAtCurrentPlace::default())
        .await
        .unwrap()
        .place_revision;
    sqlx::raw_sql(
        r#"
        CREATE FUNCTION reject_property_history_insert() RETURNS trigger LANGUAGE plpgsql AS $$
        BEGIN
            RAISE EXCEPTION 'forced Property history failure';
        END;
        $$;
        CREATE TRIGGER reject_property_history_insert BEFORE INSERT ON entity_property_history
            FOR EACH ROW EXECUTE FUNCTION reject_property_history_insert();
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
                    name: "Rolled Back Property Entity".to_owned(),
                    description: "No partial bundle may survive.".to_owned(),
                    property: vec![text_property("colour", "red")],
                    r#trait: Vec::new(),
                },
            )
            .await,
        Err(WorldError::Unavailable)
    );
    let after = world
        .list_entity_at_current_place(user_id, ListEntityAtCurrentPlace::default())
        .await
        .unwrap()
        .place_revision;
    assert_eq!(after, before);
    let counts: (i64, i64, i64, i64) = sqlx::query_as(
        r#"
        SELECT
            (SELECT count(*) FROM entity WHERE name = 'Rolled Back Property Entity'),
            (SELECT count(*) FROM activity
             WHERE operation = 'create_entity' AND context_place_entity_id IS NOT NULL),
            (SELECT count(*) FROM property_key WHERE key = 'colour'),
            (SELECT count(*) FROM entity_property_history)
        "#,
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(counts, (0, 0, 0, 0));
}
