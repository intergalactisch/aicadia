use super::*;

#[sqlx::test(migrations = "./migration")]
async fn trait_action_uniformly_establishes_develops_reads_and_reconstructs_retry(pool: PgPool) {
    let world = World::new(pool.clone());
    let (place, participant) = entered_characters(&world, &["Mara", "Pip"]).await;
    let (mara_user, mara) = participant[0];
    let (_, pip) = participant[1];
    let revision = world
        .list_entity_at_current_place(mara_user, ListEntityAtCurrentPlace::default())
        .await
        .unwrap()
        .place_revision;
    let bowl = world
        .submit_action(mara_user, action(Uuid::new_v4(), revision, "Copper Bowl"))
        .await
        .unwrap();
    let bowl = introduced_entity(&bowl).id;
    let revision = world
        .list_entity_at_current_place(mara_user, ListEntityAtCurrentPlace::default())
        .await
        .unwrap()
        .place_revision;
    let establishment_revision = revision;
    let request_id = Uuid::new_v4();
    let establishment = vec![
        establish_trait(pip, "Waits for a second echo."),
        establish_trait(
            place.entity.id,
            "Holds every departing footstep for a breath.",
        ),
        establish_trait(mara, "Jumps unusually high."),
        establish_trait(bowl, "Rings only after the hand withdraws."),
    ];
    let accepted = world
        .submit_action(
            mara_user,
            trait_action(
                request_id,
                establishment_revision,
                "Mara notices four lasting characterizations.",
                establishment.clone(),
            ),
        )
        .await
        .unwrap();
    let established = accepted_trait_change(&accepted);
    assert_eq!(established, accepted.activity.trait_change);
    assert_eq!(established.len(), 4);
    assert!(
        established
            .iter()
            .all(|change| matches!(change, ActivityTraitChange::Establish { .. }))
    );
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

    let trait_by_entity = established
        .iter()
        .map(|change| match change {
            ActivityTraitChange::Establish { entity, r#trait } => (entity.id, r#trait.id),
            ActivityTraitChange::Develop { .. } => unreachable!(),
        })
        .collect::<std::collections::HashMap<_, _>>();
    let revision = world
        .list_entity_at_current_place(mara_user, ListEntityAtCurrentPlace::default())
        .await
        .unwrap()
        .place_revision;
    let developed = world
        .submit_action(
            mara_user,
            trait_action(
                Uuid::new_v4(),
                revision,
                "Each earlier characterization develops without changing identity.",
                vec![
                    develop_trait(trait_by_entity[&bowl], "Rings before the hand withdraws."),
                    develop_trait(
                        trait_by_entity[&mara],
                        "Lands quietly after impossible jumps.",
                    ),
                    establish_trait(mara, "Refuses every invitation to jump."),
                    develop_trait(
                        trait_by_entity[&place.entity.id],
                        "Releases departing footsteps into the dawn.",
                    ),
                    develop_trait(trait_by_entity[&pip], "Moves on the second echo."),
                ],
            ),
        )
        .await
        .unwrap();
    assert_eq!(accepted_trait_change(&developed).len(), 5);
    for change in accepted_trait_change(&developed) {
        if let ActivityTraitChange::Develop {
            r#trait,
            previous_statement,
            ..
        } = change
        {
            assert_eq!(
                trait_by_entity
                    .values()
                    .filter(|id| **id == r#trait.id)
                    .count(),
                1
            );
            assert!(!previous_statement.is_empty());
        }
    }

    for entity_id in [mara, pip, bowl, place.entity.id] {
        let page = world
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
        assert!(
            page.current_state
                .association
                .iter()
                .any(|association| matches!(
                    association,
                    EntityCurrentAssociation::Trait(r#trait)
                        if r#trait.id == trait_by_entity[&entity_id]
                ))
        );
    }

    let mut reordered = establishment;
    reordered.reverse();
    let retry = world
        .submit_action(
            mara_user,
            trait_action(
                request_id,
                establishment_revision,
                "Mara notices four lasting characterizations.",
                reordered,
            ),
        )
        .await
        .unwrap();
    assert_eq!(retry, accepted);
    assert_eq!(
        world
            .submit_action(
                mara_user,
                trait_action(
                    request_id,
                    establishment_revision,
                    "Mara notices four lasting characterizations.",
                    vec![establish_trait(mara, "Changed retry content.")],
                ),
            )
            .await,
        Err(WorldError::ActionRequestConflict)
    );
    let lineage_count: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM entity_trait_version WHERE trait_id = ANY($1::uuid[])",
    )
    .bind(
        trait_by_entity
            .values()
            .map(|trait_id| trait_id.0)
            .collect::<Vec<_>>(),
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(lineage_count, 8);
}

#[sqlx::test(migrations = "./migration")]
async fn interaction_trait_and_property_changes_are_atomic_target_scoped_and_retryable(
    pool: PgPool,
) {
    let world = World::new(pool.clone());
    let (_, participant) = entered_characters(&world, &["Pip", "Mara", "Tomas"]).await;
    let (pip_user, pip) = participant[0];
    let (mara_user, mara) = participant[1];
    let (tomas_user, tomas) = participant[2];
    let original_revision = world
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
                expected_place_revision: original_revision,
                prose: "Pip and Mara leave matching marks after the exchange.".to_owned(),
                target_entity_id: vec![mara],
                property_change: vec![
                    property_change(pip, "mark", PropertyValue::Text("silver".into())),
                    property_change(mara, "mark", PropertyValue::Text("silver".into())),
                ],
                trait_change: vec![
                    establish_trait(mara, "Answers only after Pip lowers a hand."),
                    establish_trait(pip, "Lowers a hand before asking twice."),
                ],
            },
        )
        .await
        .unwrap();
    assert_eq!(accepted.activity.property_change.len(), 2);
    assert_eq!(accepted.activity.trait_change.len(), 2);
    assert_eq!(accepted.activity.actor_character.as_ref().unwrap().id, pip);
    assert!(accepted.activity.involved_entity.iter().any(|reference| {
        reference.entity.id == mara && reference.role == ActivityEntityRole::Target
    }));
    assert!(
        !accepted
            .activity
            .involved_entity
            .iter()
            .any(|reference| reference.role == ActivityEntityRole::Subject)
    );
    let mara_history = world
        .list_activity(
            mara_user,
            ListActivity {
                cursor: None,
                limit: 100,
            },
        )
        .await
        .unwrap();
    let historical = mara_history
        .activity
        .iter()
        .find(|activity| activity.id == accepted.activity.id)
        .expect("an explicit target may read the Interaction Activity");
    assert_eq!(historical.trait_change, accepted.activity.trait_change);
    assert!(
        !world
            .list_activity(
                tomas_user,
                ListActivity {
                    cursor: None,
                    limit: 100,
                },
            )
            .await
            .unwrap()
            .activity
            .iter()
            .any(|activity| activity.id == accepted.activity.id)
    );

    let trait_by_entity = accepted
        .activity
        .trait_change
        .iter()
        .map(|change| match change {
            ActivityTraitChange::Establish { entity, r#trait } => (entity.id, r#trait.id),
            ActivityTraitChange::Develop { .. } => unreachable!(),
        })
        .collect::<std::collections::HashMap<_, _>>();
    let revision = world
        .list_entity_at_current_place(pip_user, ListEntityAtCurrentPlace::default())
        .await
        .unwrap()
        .place_revision;
    let developed = world
        .submit_interaction(
            pip_user,
            SubmitInteraction {
                request_id: Uuid::new_v4(),
                expected_place_revision: revision,
                prose: "The exchange changes both familiar characterizations.".to_owned(),
                target_entity_id: vec![mara],
                property_change: Vec::new(),
                trait_change: vec![
                    develop_trait(trait_by_entity[&mara], "Answers before Pip lowers a hand."),
                    develop_trait(trait_by_entity[&pip], "Asks once before lowering a hand."),
                ],
            },
        )
        .await
        .unwrap();
    assert!(
        developed
            .activity
            .trait_change
            .iter()
            .all(|change| matches!(change, ActivityTraitChange::Develop { .. }))
    );

    let retry = world
        .submit_interaction(
            pip_user,
            SubmitInteraction {
                request_id,
                expected_place_revision: original_revision,
                prose: "Pip and Mara leave matching marks after the exchange.".to_owned(),
                target_entity_id: vec![mara],
                property_change: vec![
                    property_change(mara, "mark", PropertyValue::Text("silver".into())),
                    property_change(pip, "mark", PropertyValue::Text("silver".into())),
                ],
                trait_change: vec![
                    establish_trait(pip, "Lowers a hand before asking twice."),
                    establish_trait(mara, "Answers only after Pip lowers a hand."),
                ],
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
    let before_activity: i64 =
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
                    prose: "Tomas was not part of this exchange.".to_owned(),
                    target_entity_id: vec![mara],
                    property_change: vec![property_change(
                        pip,
                        "rollback_marker",
                        PropertyValue::Text("must not persist".into()),
                    )],
                    trait_change: vec![establish_trait(
                        tomas,
                        "Responds despite not being a target.",
                    )],
                },
            )
            .await,
        Err(WorldError::TraitUnavailable)
    );
    let after: (i64, i64) = sqlx::query_as(
        r#"
        SELECT
            (SELECT count(*) FROM activity WHERE operation = 'submit_interaction'),
            (SELECT count(*)
             FROM entity_property
             JOIN property_key ON property_key.id = entity_property.property_key_id
             WHERE entity_property.entity_id = $1 AND property_key.key = 'rollback_marker')
        "#,
    )
    .bind(pip.0)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(after, (before_activity, 0));
}

#[sqlx::test(migrations = "./migration")]
async fn action_and_interaction_reject_every_duplicate_intended_active_trait_shape_atomically(
    pool: PgPool,
) {
    let world = World::new(pool.clone());
    let (_, participant) = entered_characters(&world, &["Pip", "Mara"]).await;
    let (pip_user, pip) = participant[0];
    let (_, mara) = participant[1];
    let revision = world
        .list_entity_at_current_place(pip_user, ListEntityAtCurrentPlace::default())
        .await
        .unwrap()
        .place_revision;
    let established = world
        .submit_action(
            pip_user,
            trait_action(
                Uuid::new_v4(),
                revision,
                "Pip establishes two distinct active characterizations.",
                vec![
                    establish_trait(pip, "First distinct active statement."),
                    establish_trait(pip, "Second distinct active statement."),
                ],
            ),
        )
        .await
        .unwrap();
    let trait_by_statement = accepted_trait_change(&established)
        .iter()
        .map(|change| match change {
            ActivityTraitChange::Establish { r#trait, .. } => {
                (r#trait.statement.as_str(), r#trait.id)
            }
            ActivityTraitChange::Develop { .. } => unreachable!(),
        })
        .collect::<std::collections::HashMap<_, _>>();
    let first_trait_id = trait_by_statement["First distinct active statement."];
    let second_trait_id = trait_by_statement["Second distinct active statement."];
    let revision = world
        .list_entity_at_current_place(pip_user, ListEntityAtCurrentPlace::default())
        .await
        .unwrap()
        .place_revision;
    let before: (i64, i64, i64, i64, i64, i64) = sqlx::query_as(
        r#"
        SELECT
            (SELECT count(*) FROM activity),
            (SELECT count(*) FROM entity_trait),
            (SELECT count(*) FROM entity_trait_version),
            (SELECT count(*) FROM entity_trait_current),
            (SELECT count(*) FROM property_key WHERE key = 'rollback_marker'),
            (SELECT count(*) FROM entity_property_history)
        "#,
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    let duplicate_shapes = || {
        vec![
            vec![develop_trait(
                first_trait_id,
                "Second distinct active statement.",
            )],
            vec![
                develop_trait(first_trait_id, "Shared intended active statement."),
                develop_trait(second_trait_id, "Shared intended active statement."),
            ],
            vec![
                establish_trait(pip, "Establish/develop shared intended statement."),
                develop_trait(
                    first_trait_id,
                    "Establish/develop shared intended statement.",
                ),
            ],
        ]
    };

    for trait_change in duplicate_shapes() {
        assert_eq!(
            world
                .submit_action(
                    pip_user,
                    trait_action(
                        Uuid::new_v4(),
                        revision,
                        "This duplicate intended active state must roll back.",
                        trait_change,
                    ),
                )
                .await,
            Err(WorldError::InvalidTrait)
        );
    }

    for trait_change in duplicate_shapes() {
        assert_eq!(
            world
                .submit_interaction(
                    pip_user,
                    SubmitInteraction {
                        request_id: Uuid::new_v4(),
                        expected_place_revision: revision,
                        prose: "This duplicate Interaction package must roll back.".to_owned(),
                        target_entity_id: vec![mara],
                        property_change: vec![property_change(
                            pip,
                            "rollback_marker",
                            PropertyValue::Text("must not persist".to_owned()),
                        )],
                        trait_change,
                    },
                )
                .await,
            Err(WorldError::InvalidTrait)
        );
    }

    let after: (i64, i64, i64, i64, i64, i64) = sqlx::query_as(
        r#"
        SELECT
            (SELECT count(*) FROM activity),
            (SELECT count(*) FROM entity_trait),
            (SELECT count(*) FROM entity_trait_version),
            (SELECT count(*) FROM entity_trait_current),
            (SELECT count(*) FROM property_key WHERE key = 'rollback_marker'),
            (SELECT count(*) FROM entity_property_history)
        "#,
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(after, before);
    let current: Vec<(Uuid, String)> = sqlx::query_as(
        r#"
        SELECT current.trait_id, version.statement
        FROM entity_trait_current AS current
        JOIN entity_trait_version AS version
          ON version.trait_id = current.trait_id
         AND version.entity_id = current.entity_id
         AND version.activity_id = current.current_activity_id
        WHERE current.trait_id = ANY($1::uuid[])
        ORDER BY current.trait_id
        "#,
    )
    .bind(vec![first_trait_id.0, second_trait_id.0])
    .fetch_all(&pool)
    .await
    .unwrap();
    assert_eq!(current.len(), 2);
    assert_eq!(
        current
            .iter()
            .map(|(_, statement)| statement.as_str())
            .collect::<std::collections::HashSet<_>>(),
        [
            "First distinct active statement.",
            "Second distinct active statement."
        ]
        .into_iter()
        .collect()
    );
}

#[sqlx::test(migrations = "./migration")]
async fn action_trait_package_reuses_a_statement_vacated_by_another_lineage(pool: PgPool) {
    let world = World::new(pool.clone());
    let (_, participant) = entered_characters(&world, &["Pip"]).await;
    let (pip_user, pip) = participant[0];
    let revision = world
        .list_entity_at_current_place(pip_user, ListEntityAtCurrentPlace::default())
        .await
        .unwrap()
        .place_revision;
    let established = world
        .submit_action(
            pip_user,
            trait_action(
                Uuid::new_v4(),
                revision,
                "Pip reveals two distinct characterizations.",
                vec![
                    establish_trait(pip, "Waits until the third knock."),
                    establish_trait(pip, "Answers before the first echo."),
                ],
            ),
        )
        .await
        .unwrap();
    let trait_by_statement = accepted_trait_change(&established)
        .iter()
        .map(|change| match change {
            ActivityTraitChange::Establish { r#trait, .. } => {
                (r#trait.statement.as_str(), r#trait.id)
            }
            ActivityTraitChange::Develop { .. } => unreachable!(),
        })
        .collect::<std::collections::HashMap<_, _>>();
    let vacating_trait_id = trait_by_statement["Waits until the third knock."];
    let reusing_trait_id = trait_by_statement["Answers before the first echo."];
    let revision = world
        .list_entity_at_current_place(pip_user, ListEntityAtCurrentPlace::default())
        .await
        .unwrap()
        .place_revision;
    let request_id = Uuid::new_v4();
    let obstructive_order = vec![
        develop_trait(reusing_trait_id, "Waits until the third knock."),
        develop_trait(vacating_trait_id, "Leaves before the third knock."),
    ];
    let accepted = world
        .submit_action(
            pip_user,
            trait_action(
                request_id,
                revision,
                "One characterization moves on as another inherits its exact wording.",
                obstructive_order.clone(),
            ),
        )
        .await
        .expect("post-package uniqueness must allow reuse after a same-package vacancy");
    assert_eq!(
        accepted_trait_change(&accepted),
        accepted.activity.trait_change
    );
    let developed_by_id = accepted_trait_change(&accepted)
        .iter()
        .map(|change| match change {
            ActivityTraitChange::Develop {
                entity,
                r#trait,
                previous_statement,
            } => {
                assert_eq!(entity.id, pip);
                (
                    r#trait.id,
                    (previous_statement.as_str(), r#trait.statement.as_str()),
                )
            }
            ActivityTraitChange::Establish { .. } => unreachable!(),
        })
        .collect::<std::collections::HashMap<_, _>>();
    assert_eq!(developed_by_id.len(), 2);
    assert_eq!(
        developed_by_id[&vacating_trait_id],
        (
            "Waits until the third knock.",
            "Leaves before the third knock."
        )
    );
    assert_eq!(
        developed_by_id[&reusing_trait_id],
        (
            "Answers before the first echo.",
            "Waits until the third knock."
        )
    );

    let current = world
        .get_character(
            pip_user,
            GetEntityCurrentState {
                cursor: None,
                limit: 100,
            },
        )
        .await
        .unwrap();
    let current_by_id = current
        .current_state
        .association
        .iter()
        .filter_map(|association| match association {
            EntityCurrentAssociation::Trait(r#trait) => {
                Some((r#trait.id, r#trait.statement.as_str()))
            }
            EntityCurrentAssociation::Property { .. } => None,
        })
        .collect::<std::collections::HashMap<_, _>>();
    assert_eq!(
        current_by_id[&vacating_trait_id],
        "Leaves before the third knock."
    );
    assert_eq!(
        current_by_id[&reusing_trait_id],
        "Waits until the third knock."
    );

    let predecessor_by_id = sqlx::query_as::<_, (Uuid, Uuid, String)>(
        r#"
        SELECT trait_id, previous_activity_id, statement
        FROM entity_trait_version
        WHERE activity_id = $1
        ORDER BY trait_id
        "#,
    )
    .bind(accepted.activity.id.0)
    .fetch_all(&pool)
    .await
    .unwrap()
    .into_iter()
    .map(|(trait_id, previous_activity_id, statement)| {
        (trait_id, (previous_activity_id, statement))
    })
    .collect::<std::collections::HashMap<_, _>>();
    assert_eq!(predecessor_by_id.len(), 2);
    for trait_id in [vacating_trait_id, reusing_trait_id] {
        assert_eq!(
            predecessor_by_id[&trait_id.0].0, established.activity.id.0,
            "each developed version must point to its own established predecessor Activity"
        );
    }

    let historical = world
        .list_activity(
            pip_user,
            ListActivity {
                cursor: None,
                limit: 100,
            },
        )
        .await
        .unwrap()
        .activity
        .into_iter()
        .find(|activity| activity.id == accepted.activity.id)
        .expect("the accepted Trait development must remain readable as Activity history");
    assert_eq!(historical.trait_change, accepted.activity.trait_change);

    let mut retry_order = obstructive_order;
    retry_order.reverse();
    let retry = world
        .submit_action(
            pip_user,
            trait_action(
                request_id,
                revision,
                "One characterization moves on as another inherits its exact wording.",
                retry_order,
            ),
        )
        .await
        .unwrap();
    assert_eq!(retry, accepted);
}

#[sqlx::test(migrations = "./migration")]
async fn trait_validation_combined_cursor_unavailability_and_concurrency_are_closed(pool: PgPool) {
    let world = World::new(pool.clone());
    let user_id = create_user(&world).await;
    let character = world
        .create_character(
            user_id,
            CreateCharacter {
                name: "Mara".to_owned(),
                description: "A bounded Trait subject.".to_owned(),
                property: vec![
                    text_property("colour", "amber"),
                    integer_property("leg_count", 2),
                ],
            },
        )
        .await
        .unwrap();
    let unplaced = world
        .get_character(
            user_id,
            GetEntityCurrentState {
                cursor: None,
                limit: 1,
            },
        )
        .await
        .unwrap();
    assert_eq!(unplaced.place_revision, None);
    assert_eq!(unplaced.current_state.association.len(), 1);
    let unplaced_cursor = unplaced.current_state.next;
    world
        .create_entry_place(user_id, place("North Gate"))
        .await
        .unwrap();
    world.enter_world(user_id).await.unwrap();
    assert_eq!(
        world
            .get_character(
                user_id,
                GetEntityCurrentState {
                    cursor: unplaced_cursor,
                    limit: 1,
                },
            )
            .await,
        Err(WorldError::PlaceRevisionConflict)
    );

    let revision = world
        .list_entity_at_current_place(user_id, ListEntityAtCurrentPlace::default())
        .await
        .unwrap()
        .place_revision;
    let established = world
        .submit_action(
            user_id,
            trait_action(
                Uuid::new_v4(),
                revision,
                "Mara's established characterization becomes World state.",
                vec![establish_trait(
                    character.entity.id,
                    "Waits for a second echo.",
                )],
            ),
        )
        .await
        .unwrap();
    let trait_id = match &accepted_trait_change(&established)[0] {
        ActivityTraitChange::Establish { r#trait, .. } => r#trait.id,
        ActivityTraitChange::Develop { .. } => unreachable!(),
    };
    let first = world
        .get_character(
            user_id,
            GetEntityCurrentState {
                cursor: None,
                limit: 2,
            },
        )
        .await
        .unwrap();
    assert!(
        first
            .current_state
            .association
            .iter()
            .all(|association| matches!(association, EntityCurrentAssociation::Property { .. }))
    );
    let stale_cursor = first.current_state.next;
    let revision = first.place_revision.unwrap();
    world
        .submit_action(
            user_id,
            trait_action(
                Uuid::new_v4(),
                revision,
                "A second valid but contradictory characterization is accepted.",
                vec![establish_trait(
                    character.entity.id,
                    "Never waits for an echo.",
                )],
            ),
        )
        .await
        .unwrap();
    assert_eq!(
        world
            .get_character(
                user_id,
                GetEntityCurrentState {
                    cursor: stale_cursor,
                    limit: 2,
                },
            )
            .await,
        Err(WorldError::PlaceRevisionConflict)
    );
    let fresh = world
        .get_character(
            user_id,
            GetEntityCurrentState {
                cursor: None,
                limit: 100,
            },
        )
        .await
        .unwrap();
    assert_eq!(fresh.current_state.association.len(), 4);
    assert!(matches!(
        fresh.current_state.association[0],
        EntityCurrentAssociation::Property { .. }
    ));
    assert!(matches!(
        fresh.current_state.association[1],
        EntityCurrentAssociation::Property { .. }
    ));
    assert!(
        fresh.current_state.association[2..]
            .iter()
            .all(|association| matches!(association, EntityCurrentAssociation::Trait(_)))
    );

    let revision = fresh.place_revision.unwrap();
    for invalid in [
        Vec::new(),
        vec![
            establish_trait(character.entity.id, " Duplicate statement. "),
            establish_trait(character.entity.id, "Duplicate statement."),
        ],
        vec![develop_trait(trait_id, "Waits for a second echo.")],
        vec![
            develop_trait(trait_id, "First proposed successor."),
            develop_trait(trait_id, "Second proposed successor."),
        ],
        vec![establish_trait(
            character.entity.id,
            "Waits for a second echo.",
        )],
        (0..101)
            .map(|index| establish_trait(character.entity.id, format!("Bound {index}.")))
            .collect(),
    ] {
        assert_eq!(
            world
                .submit_action(
                    user_id,
                    trait_action(
                        Uuid::new_v4(),
                        revision,
                        "This invalid Trait package writes nothing.",
                        invalid,
                    ),
                )
                .await,
            Err(WorldError::InvalidTrait)
        );
    }
    assert_eq!(
        world
            .submit_action(
                user_id,
                trait_action(
                    Uuid::new_v4(),
                    revision,
                    "A missing Trait remains neutrally unavailable.",
                    vec![develop_trait(
                        EntityTraitId(Uuid::new_v4()),
                        "Cannot develop an unavailable lineage.",
                    )],
                ),
            )
            .await,
        Err(WorldError::TraitUnavailable)
    );

    let remote = world
        .create_entity(user_id, entity("Unplaced Remote Trait Subject"))
        .await
        .unwrap();
    let revision = world
        .list_entity_at_current_place(user_id, ListEntityAtCurrentPlace::default())
        .await
        .unwrap()
        .place_revision;
    assert_eq!(
        world
            .submit_action(
                user_id,
                trait_action(
                    Uuid::new_v4(),
                    revision,
                    "A remote Entity cannot receive a contextual Trait.",
                    vec![establish_trait(remote.id, "Cannot be reached here.")],
                ),
            )
            .await,
        Err(WorldError::TraitUnavailable)
    );

    let barrier = std::sync::Arc::new(tokio::sync::Barrier::new(3));
    let first = {
        let world = world.clone();
        let barrier = barrier.clone();
        tokio::spawn(async move {
            barrier.wait().await;
            world
                .submit_action(
                    user_id,
                    trait_action(
                        Uuid::new_v4(),
                        revision,
                        "The first concurrent successor is proposed.",
                        vec![develop_trait(trait_id, "Moves on the second echo.")],
                    ),
                )
                .await
        })
    };
    let second = {
        let world = world.clone();
        let barrier = barrier.clone();
        tokio::spawn(async move {
            barrier.wait().await;
            world
                .submit_action(
                    user_id,
                    trait_action(
                        Uuid::new_v4(),
                        revision,
                        "The second concurrent successor is proposed.",
                        vec![develop_trait(trait_id, "Moves before the second echo.")],
                    ),
                )
                .await
        })
    };
    barrier.wait().await;
    let result = [first.await.unwrap(), second.await.unwrap()];
    assert_eq!(result.iter().filter(|result| result.is_ok()).count(), 1);
    assert_eq!(
        result
            .iter()
            .filter(|result| **result == Err(WorldError::PlaceRevisionConflict))
            .count(),
        1
    );
    let version_count: i64 =
        sqlx::query_scalar("SELECT count(*) FROM entity_trait_version WHERE trait_id = $1")
            .bind(trait_id.0)
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(version_count, 2);
}

#[sqlx::test(migrations = "./migration")]
async fn every_creation_route_remains_trait_free(pool: PgPool) {
    let world = World::new(pool.clone());
    let user_id = create_user(&world).await;
    world
        .create_character(user_id, character("Trait-free Character"))
        .await
        .unwrap();
    world
        .create_entry_place(user_id, place("Trait-free Place"))
        .await
        .unwrap();
    world
        .create_entity(user_id, entity("Trait-free ordinary Entity"))
        .await
        .unwrap();
    world.enter_world(user_id).await.unwrap();
    let revision = world
        .list_entity_at_current_place(user_id, ListEntityAtCurrentPlace::default())
        .await
        .unwrap()
        .place_revision;
    world
        .submit_action(
            user_id,
            action(Uuid::new_v4(), revision, "Trait-free introduced Entity"),
        )
        .await
        .unwrap();
    let count: (i64, i64, i64) = sqlx::query_as(
        "SELECT (SELECT count(*) FROM entity_trait), (SELECT count(*) FROM entity_trait_version), (SELECT count(*) FROM entity_trait_current)",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(count, (0, 0, 0));
}

#[sqlx::test(migrations = "./migration")]
async fn trait_world_storage_failure_rolls_back_activity_lineage_pointer_and_revision(
    pool: PgPool,
) {
    let world = World::new(pool.clone());
    let (place, participant) = entered_characters(&world, &["Mara"]).await;
    let (user_id, character_id) = participant[0];
    let revision = world
        .list_entity_at_current_place(user_id, ListEntityAtCurrentPlace::default())
        .await
        .unwrap()
        .place_revision;
    sqlx::raw_sql(
        r#"
        CREATE FUNCTION fail_trait_current_write() RETURNS trigger
        LANGUAGE plpgsql AS $$
        BEGIN
            RAISE EXCEPTION 'forced Trait pointer failure';
        END;
        $$;
        CREATE TRIGGER fail_trait_current_write
            BEFORE INSERT OR UPDATE ON entity_trait_current
            FOR EACH STATEMENT EXECUTE FUNCTION fail_trait_current_write();
        "#,
    )
    .execute(&pool)
    .await
    .unwrap();
    assert_eq!(
        world
            .submit_action(
                user_id,
                trait_action(
                    Uuid::new_v4(),
                    revision,
                    "This accepted package must roll back on storage failure.",
                    vec![establish_trait(
                        character_id,
                        "Must leave no partial lineage."
                    )],
                ),
            )
            .await,
        Err(WorldError::Unavailable)
    );
    let count: (i64, i64, i64, i64) = sqlx::query_as(
        r#"
        SELECT
            (SELECT count(*) FROM activity WHERE action_consequence = 'change_entity_trait'),
            (SELECT count(*) FROM entity_trait),
            (SELECT count(*) FROM entity_trait_version),
            (SELECT count(*) FROM entity_trait_current)
        "#,
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(count, (0, 0, 0, 0));
    let after = world
        .list_entity_at_current_place(user_id, ListEntityAtCurrentPlace::default())
        .await
        .unwrap();
    assert_eq!(after.place.entity.id, place.entity.id);
    assert_eq!(after.place_revision, revision);
}
