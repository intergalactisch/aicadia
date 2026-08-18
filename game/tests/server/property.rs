use super::*;

#[sqlx::test(migrations = "./migration")]
async fn property_http_and_mcp_share_creation_bulk_change_history_and_strict_errors(pool: PgPool) {
    let world = World::new(pool.clone());
    let actor = world.create_user().await.expect("actor User should exist");
    let target = world.create_user().await.expect("target User should exist");
    let server = TestServer::start(world).await;

    let actor_property = text_property("actor", 100);
    let actor_response = server
        .client
        .post(format!("{}/api/character", server.base_url))
        .header(USER_CONTEXT_HEADER, actor.id.0.to_string())
        .json(&json!({
            "name": "Mara",
            "description": "A careful surveyor.",
            "property": actor_property
        }))
        .send()
        .await
        .expect("Property-bearing Character request should send");
    assert_eq!(actor_response.status(), StatusCode::CREATED);
    let actor_character: Value = actor_response
        .json()
        .await
        .expect("Property-bearing Character should be JSON");
    let actor_entity_id = actor_character["entity"]["id"].clone();

    let target_character = server
        .tool(
            "create_character",
            json!({
                "name": "Pip",
                "description": "A tiny grey wanderer.",
                "property": []
            }),
            Some(target.id.0),
        )
        .await;
    let target_entity_id = structured(&target_character)["entity"]["id"].clone();

    let remote_property = text_property("remote", 100);
    let remote_response = server
        .client
        .post(format!("{}/api/entity", server.base_url))
        .header(USER_CONTEXT_HEADER, actor.id.0.to_string())
        .json(&json!({
            "name": "Remote Herbarium",
            "description": "An unplaced collection outside local play.",
            "property": remote_property
        }))
        .send()
        .await
        .expect("Property-bearing Entity request should send");
    assert_eq!(remote_response.status(), StatusCode::CREATED);
    let remote_entity: Value = remote_response
        .json()
        .await
        .expect("Property-bearing Entity should be JSON");
    let remote_entity_id = remote_entity["id"].clone();

    let place_property = text_property("place", 100);
    let place = server
        .tool(
            "create_entry_place",
            json!({
                "name": "First Landing",
                "description": "A quiet square where paths begin.",
                "property": place_property
            }),
            Some(actor.id.0),
        )
        .await;
    let place = structured(&place).clone();
    let place_entity_id = place["entity"]["id"].clone();

    for user_id in [actor.id.0, target.id.0] {
        server.tool("enter_world", json!({}), Some(user_id)).await;
    }

    let context = server
        .tool(
            "list_entity_at_current_place",
            json!({"limit": 100}),
            Some(actor.id.0),
        )
        .await;
    let introduced_property = text_property("marker", 100);
    let introduce = json!({
        "request_id": Uuid::new_v4(),
        "expected_place_revision": structured(&context)["place_revision"],
        "prose": "Mara sets a hundred-marked cedar post beside the landing.",
        "consequence": {
            "type": "introduce_entity",
            "name": "Cedar Post",
            "description": "A cedar post covered in small deliberate marks.",
            "property": introduced_property
        }
    });
    let introduced = server
        .tool("submit_action", introduce, Some(actor.id.0))
        .await;
    let introduced = structured(&introduced).clone();
    assert_eq!(introduced["consequence"]["type"], "introduce_entity");
    assert_eq!(
        introduced["activity"]["property_change"]
            .as_array()
            .map(Vec::len),
        Some(100)
    );
    let ordinary_entity_id = introduced["consequence"]["entity"]["id"].clone();

    let (property_revision, local_property) =
        collect_http_current_property(&server, actor.id.0).await;
    assert_eq!(local_property.len(), 300);
    assert!(
        local_property
            .iter()
            .all(|property| property["entity"]["id"] != remote_entity_id),
        "an unplaced Entity's Properties must not leak into the local page"
    );
    for property in &local_property {
        assert_eq!(
            property
                .as_object()
                .expect("Property row should be an object")
                .keys()
                .cloned()
                .collect::<BTreeSet<_>>(),
            BTreeSet::from(["entity".to_owned(), "key".to_owned(), "value".to_owned()])
        );
        assert_eq!(
            property["entity"]
                .as_object()
                .expect("Property Entity should be a summary")
                .keys()
                .cloned()
                .collect::<BTreeSet<_>>(),
            BTreeSet::from(["id".to_owned(), "name".to_owned()])
        );
    }
    for forbidden in [
        "property_key_id",
        "owner_user_id",
        "introduced_by_user_id",
        "user_controlled",
        "npc",
    ] {
        assert!(
            !Value::Array(local_property.clone())
                .to_string()
                .contains(forbidden)
        );
    }

    let first_property_page: Value = server
        .client
        .get(format!(
            "{}/api/place/current/entity/{}?limit=1",
            server.base_url,
            actor_entity_id.as_str().expect("actor id should be text")
        ))
        .header(USER_CONTEXT_HEADER, actor.id.0.to_string())
        .send()
        .await
        .expect("first paged Property request should send")
        .json()
        .await
        .expect("first paged Property response should be JSON");
    let property_cursor = first_property_page["current_state"]["next"]
        .as_str()
        .expect("three hundred local Properties should produce a cursor");
    let second_property_page = server
        .tool(
            "get_entity_at_current_place",
            json!({"entity_id": actor_entity_id, "cursor": property_cursor, "limit": 1}),
            Some(actor.id.0),
        )
        .await;
    assert_eq!(
        structured(&second_property_page)["place_revision"],
        first_property_page["place_revision"]
    );
    assert_ne!(
        structured(&second_property_page)["current_state"]["association"][0],
        first_property_page["current_state"]["association"][0]
    );
    assert_cross_operation_cursor_rejected(
        &server,
        "/api/place/current/activity",
        "list_activity_at_current_place",
        property_cursor,
        Some(actor.id.0),
    )
    .await;
    for (query, mcp_input, expected_code) in [
        (
            "cursor=not-a-property-cursor",
            json!({"entity_id": actor_entity_id, "cursor": "not-a-property-cursor"}),
            "invalid_request",
        ),
        (
            "limit=0",
            json!({"entity_id": actor_entity_id, "limit": 0}),
            "invalid_entity_limit",
        ),
        (
            "limit=101",
            json!({"entity_id": actor_entity_id, "limit": 101}),
            "invalid_entity_limit",
        ),
    ] {
        let response = server
            .client
            .get(format!(
                "{}/api/place/current/entity/{}?{query}",
                server.base_url,
                actor_entity_id.as_str().expect("actor id should be text")
            ))
            .header(USER_CONTEXT_HEADER, actor.id.0.to_string())
            .send()
            .await
            .expect("invalid Property page request should send");
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        let http_error: Value = response
            .json()
            .await
            .expect("invalid Property page error should be JSON");
        let mcp = server
            .tool("get_entity_at_current_place", mcp_input, Some(actor.id.0))
            .await;
        assert_eq!(error_code(&http_error), expected_code);
        assert_eq!(mcp_error(&mcp), http_error);
    }

    let mut explosion_change = vec![
        json!({"entity_id": place_entity_id, "key": "surface", "value": {"type": "text", "text": "blackened"}}),
        json!({"entity_id": ordinary_entity_id, "key": "surface", "value": {"type": "text", "text": "blackened"}}),
        json!({"entity_id": target_entity_id, "key": "surface", "value": {"type": "text", "text": "blackened"}}),
        json!({"entity_id": actor_entity_id, "key": "surface", "value": {"type": "text", "text": "blackened"}}),
    ];
    explosion_change.extend((0..96).map(|index| {
        json!({
            "entity_id": actor_entity_id,
            "key": format!("blast_{index:03}"),
            "value": {"type": "integer", "integer": index}
        })
    }));
    let explosion = json!({
        "request_id": Uuid::new_v4(),
        "expected_place_revision": property_revision,
        "prose": "A sudden blast blackens everyone and everything around the landing.",
        "consequence": {
            "type": "change_entity_state",
            "property_change": explosion_change
        }
    });
    let explosion_response = server
        .client
        .post(format!("{}/api/action", server.base_url))
        .header(USER_CONTEXT_HEADER, actor.id.0.to_string())
        .json(&explosion)
        .send()
        .await
        .expect("Property explosion should send");
    assert_eq!(explosion_response.status(), StatusCode::CREATED);
    let accepted_explosion: Value = explosion_response
        .json()
        .await
        .expect("accepted Property explosion should be JSON");
    assert_eq!(
        accepted_explosion["consequence"]["type"],
        "change_entity_state"
    );
    assert_eq!(
        accepted_explosion["consequence"]["property_change"]
            .as_array()
            .map(Vec::len),
        Some(100)
    );
    assert_eq!(
        accepted_explosion["activity"]["property_change"],
        accepted_explosion["consequence"]["property_change"]
    );
    let explosion_retry = server
        .tool("submit_action", explosion.clone(), Some(actor.id.0))
        .await;
    assert_eq!(structured(&explosion_retry), &accepted_explosion);

    let after_explosion = server
        .tool(
            "get_entity_at_current_place",
            json!({"entity_id": actor_entity_id, "limit": 100}),
            Some(actor.id.0),
        )
        .await;
    let interaction = json!({
        "request_id": Uuid::new_v4(),
        "expected_place_revision": structured(&after_explosion)["place_revision"],
        "prose": "Mara brushes ash from Pip and from her own coat without deciding Pip's response.",
        "target_entity_id": [target_entity_id],
        "property_change": [
            {"entity_id": target_entity_id, "key": "ash", "value": {"type": "text", "text": "brushed away"}},
            {"entity_id": actor_entity_id, "key": "ash", "value": {"type": "text", "text": "brushed away"}}
        ]
    });
    let accepted_interaction = server
        .tool("submit_interaction", interaction.clone(), Some(actor.id.0))
        .await;
    let accepted_interaction = structured(&accepted_interaction).clone();
    assert_eq!(
        accepted_interaction["activity"]["property_change"]
            .as_array()
            .map(Vec::len),
        Some(2)
    );
    assert!(accepted_interaction.get("response").is_none());
    assert!(!accepted_interaction.to_string().contains("owner_user_id"));
    let interaction_retry = server
        .client
        .post(format!("{}/api/interaction", server.base_url))
        .header(USER_CONTEXT_HEADER, actor.id.0.to_string())
        .json(&interaction)
        .send()
        .await
        .expect("Interaction retry should send");
    assert_eq!(interaction_retry.status(), StatusCode::CREATED);
    assert_eq!(
        interaction_retry
            .json::<Value>()
            .await
            .expect("Interaction retry should be JSON"),
        accepted_interaction
    );

    let (special_revision, _) = collect_mcp_current_property(&server, actor.id.0).await;
    let provenance_like_change = json!([
        {
            "entity_id": ordinary_entity_id,
            "key": "user_controlled",
            "value": {"type": "text", "text": "a phrase painted for a guessing game"}
        },
        {
            "entity_id": ordinary_entity_id,
            "key": "npc",
            "value": {"type": "integer", "integer": 7}
        },
        {
            "entity_id": ordinary_entity_id,
            "key": "owner_user_id",
            "value": {"type": "text", "text": "a fictional catalogue label"}
        }
    ]);
    let expected_provenance_like_change = json!([
        {
            "entity": {"id": ordinary_entity_id, "name": "Cedar Post"},
            "key": "npc",
            "value": {"type": "integer", "integer": 7}
        },
        {
            "entity": {"id": ordinary_entity_id, "name": "Cedar Post"},
            "key": "owner_user_id",
            "value": {"type": "text", "text": "a fictional catalogue label"}
        },
        {
            "entity": {"id": ordinary_entity_id, "name": "Cedar Post"},
            "key": "user_controlled",
            "value": {"type": "text", "text": "a phrase painted for a guessing game"}
        }
    ]);
    let provenance_like_action = json!({
        "request_id": Uuid::new_v4(),
        "expected_place_revision": special_revision,
        "prose": "Mara paints three deliberately misleading catalogue labels on the cedar post; they classify no person or controller.",
        "consequence": {
            "type": "change_entity_state",
            "property_change": provenance_like_change
        }
    });
    let provenance_like_response = server
        .client
        .post(format!("{}/api/action", server.base_url))
        .header(USER_CONTEXT_HEADER, actor.id.0.to_string())
        .json(&provenance_like_action)
        .send()
        .await
        .expect("provenance-like Property Action should send");
    assert_eq!(provenance_like_response.status(), StatusCode::CREATED);
    let accepted_provenance_like: Value = provenance_like_response
        .json()
        .await
        .expect("provenance-like Property Action should be JSON");
    assert_eq!(
        accepted_provenance_like["consequence"]["property_change"], expected_provenance_like_change,
        "canonical provenance-like keys must be accepted as ordinary fictional Property data"
    );
    assert_eq!(
        accepted_provenance_like["activity"]["property_change"],
        expected_provenance_like_change
    );

    let (http_special_revision, http_current_property) =
        collect_http_current_property(&server, actor.id.0).await;
    let (mcp_special_revision, mcp_current_property) =
        collect_mcp_current_property(&server, actor.id.0).await;
    assert_eq!(http_special_revision, mcp_special_revision);
    assert_eq!(http_current_property, mcp_current_property);
    for (key, value) in [
        (
            "user_controlled",
            json!({"type": "text", "text": "a phrase painted for a guessing game"}),
        ),
        ("npc", json!({"type": "integer", "integer": 7})),
        (
            "owner_user_id",
            json!({"type": "text", "text": "a fictional catalogue label"}),
        ),
    ] {
        let property = http_current_property
            .iter()
            .find(|property| {
                property["entity"]["id"] == ordinary_entity_id && property["key"] == key
            })
            .unwrap_or_else(|| panic!("current Property output should retain ordinary key {key}"));
        assert_eq!(property["value"], value);
        assert_eq!(
            property
                .as_object()
                .expect("current Property should be an object")
                .keys()
                .cloned()
                .collect::<BTreeSet<_>>(),
            BTreeSet::from(["entity".to_owned(), "key".to_owned(), "value".to_owned()])
        );
        assert_eq!(
            property["entity"],
            json!({"id": ordinary_entity_id, "name": "Cedar Post"}),
            "a provenance-like fictional key must not classify or expose control of its Entity"
        );
    }

    let ordinary_after_labels = server
        .tool(
            "list_entity_at_current_place",
            json!({"limit": 100}),
            Some(actor.id.0),
        )
        .await;
    let ordinary_after_labels = structured(&ordinary_after_labels)["entity"]
        .as_array()
        .expect("current Entity page should contain local subjects")
        .iter()
        .find(|entity| entity["id"] == ordinary_entity_id)
        .expect("the labelled cedar post should remain locally selectable");
    assert_eq!(
        ordinary_after_labels
            .as_object()
            .expect("current Entity should be an object")
            .keys()
            .cloned()
            .collect::<BTreeSet<_>>(),
        BTreeSet::from(["description".to_owned(), "id".to_owned(), "name".to_owned()]),
        "fictional keys must not add Character, NPC, owner, User or control classification fields"
    );

    let history: Value = server
        .client
        .get(format!("{}/api/activity?limit=100", server.base_url))
        .header(USER_CONTEXT_HEADER, actor.id.0.to_string())
        .send()
        .await
        .expect("Property history should send")
        .json()
        .await
        .expect("Property history should be JSON");
    let history = history["activity"]
        .as_array()
        .expect("personal history should contain Activities");
    let mcp_history = server
        .tool("list_activity", json!({"limit": 100}), Some(actor.id.0))
        .await;
    let mcp_history = structured(&mcp_history)["activity"]
        .as_array()
        .expect("MCP personal history should contain Activities");
    let provenance_like_activity_id = accepted_provenance_like["activity"]["id"].clone();
    for authorized_history in [history, mcp_history] {
        let activity = authorized_history
            .iter()
            .find(|activity| activity["id"] == provenance_like_activity_id)
            .expect("authorized Activity should retain provenance-like fictional keys");
        assert_eq!(activity["property_change"], expected_provenance_like_change);
        for property in activity["property_change"]
            .as_array()
            .expect("Activity Property changes should be an array")
        {
            assert_eq!(
                property
                    .as_object()
                    .expect("Activity Property should be an object")
                    .keys()
                    .cloned()
                    .collect::<BTreeSet<_>>(),
                BTreeSet::from(["entity".to_owned(), "key".to_owned(), "value".to_owned()])
            );
            assert_eq!(
                property["entity"],
                json!({"id": ordinary_entity_id, "name": "Cedar Post"})
            );
        }
    }
    for (activity_id, expected_change) in [
        (
            accepted_explosion["activity"]["id"].clone(),
            accepted_explosion["activity"]["property_change"].clone(),
        ),
        (
            accepted_interaction["activity"]["id"].clone(),
            accepted_interaction["activity"]["property_change"].clone(),
        ),
        (
            introduced["activity"]["id"].clone(),
            introduced["activity"]["property_change"].clone(),
        ),
        (
            accepted_provenance_like["activity"]["id"].clone(),
            accepted_provenance_like["activity"]["property_change"].clone(),
        ),
    ] {
        let activity = history
            .iter()
            .find(|activity| activity["id"] == activity_id)
            .expect("exact Property-changing Activity should be visible");
        assert_eq!(activity["property_change"], expected_change);
    }
    for operation in ["create_character", "create_entity", "create_entry_place"] {
        let activity = history
            .iter()
            .find(|activity| activity["operation"] == operation)
            .expect("each creation Activity should be visible");
        assert_eq!(
            activity["property_change"].as_array().map(Vec::len),
            Some(100),
            "{operation} should hydrate all initial Properties"
        );
    }

    let fresh = server
        .tool(
            "get_entity_at_current_place",
            json!({"entity_id": actor_entity_id, "limit": 1}),
            Some(actor.id.0),
        )
        .await;
    let fresh_revision = structured(&fresh)["place_revision"].clone();

    for malformed in [
        json!({
            "request_id": Uuid::new_v4(),
            "expected_place_revision": fresh_revision,
            "prose": "A malformed Entity id must fail at the wire boundary.",
            "consequence": {
                "type": "change_entity_state",
                "property_change": [
                    {"entity_id": "not-a-uuid", "key": "wire", "value": {"type": "text", "text": "no"}}
                ]
            }
        }),
        json!({
            "request_id": Uuid::new_v4(),
            "expected_place_revision": fresh_revision,
            "prose": "An unsupported Property value tag must fail at the wire boundary.",
            "consequence": {
                "type": "change_entity_state",
                "property_change": [
                    {"entity_id": actor_entity_id, "key": "wire", "value": {"type": "boolean", "boolean": true}}
                ]
            }
        }),
        json!({
            "request_id": Uuid::new_v4(),
            "expected_place_revision": fresh_revision,
            "prose": "An unknown nested Property field must fail at the wire boundary.",
            "consequence": {
                "type": "change_entity_state",
                "property_change": [
                    {"entity_id": actor_entity_id, "key": "wire", "value": {"type": "text", "text": "no"}, "unexpected": true}
                ]
            }
        }),
    ] {
        let response = server
            .client
            .post(format!("{}/api/action", server.base_url))
            .header(USER_CONTEXT_HEADER, actor.id.0.to_string())
            .json(&malformed)
            .send()
            .await
            .expect("malformed Property Action should send");
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        let http_error: Value = response
            .json()
            .await
            .expect("malformed Property error should be JSON");
        let mcp = server
            .tool("submit_action", malformed, Some(actor.id.0))
            .await;
        assert_eq!(error_code(&http_error), "invalid_request");
        assert_eq!(mcp_error(&mcp), http_error);
    }

    let remote_change = json!({
        "request_id": Uuid::new_v4(),
        "expected_place_revision": fresh_revision,
        "prose": "This cannot reach the remote herbarium.",
        "consequence": {
            "type": "change_entity_state",
            "property_change": [
                {"entity_id": remote_entity_id, "key": "leaked", "value": {"type": "text", "text": "no"}}
            ]
        }
    });
    let remote_response = server
        .client
        .post(format!("{}/api/action", server.base_url))
        .header(USER_CONTEXT_HEADER, actor.id.0.to_string())
        .json(&remote_change)
        .send()
        .await
        .expect("remote Property Action should send");
    assert_eq!(remote_response.status(), StatusCode::CONFLICT);
    let remote_http: Value = remote_response
        .json()
        .await
        .expect("remote Property error should be JSON");
    let mut remote_mcp_input = remote_change;
    remote_mcp_input["request_id"] = json!(Uuid::new_v4());
    let remote_mcp = server
        .tool("submit_action", remote_mcp_input, Some(actor.id.0))
        .await;
    assert_eq!(error_code(&remote_http), "property_entity_unavailable");
    assert_eq!(mcp_error(&remote_mcp), remote_http);

    let non_target_change = json!({
        "request_id": Uuid::new_v4(),
        "expected_place_revision": fresh_revision,
        "prose": "This names a local Entity that is not an Interaction target.",
        "target_entity_id": [target_entity_id],
        "property_change": [
            {"entity_id": ordinary_entity_id, "key": "leaked", "value": {"type": "text", "text": "no"}}
        ]
    });
    let non_target_response = server
        .client
        .post(format!("{}/api/interaction", server.base_url))
        .header(USER_CONTEXT_HEADER, actor.id.0.to_string())
        .json(&non_target_change)
        .send()
        .await
        .expect("non-target Property Interaction should send");
    assert_eq!(non_target_response.status(), StatusCode::CONFLICT);
    let non_target_http: Value = non_target_response
        .json()
        .await
        .expect("non-target Property error should be JSON");
    let mut non_target_mcp_input = non_target_change;
    non_target_mcp_input["request_id"] = json!(Uuid::new_v4());
    let non_target_mcp = server
        .tool("submit_interaction", non_target_mcp_input, Some(actor.id.0))
        .await;
    assert_eq!(error_code(&non_target_http), "property_entity_unavailable");
    assert_eq!(mcp_error(&non_target_mcp), non_target_http);

    let invalid_action = |property_change: Vec<Value>, prose: &str| {
        json!({
            "request_id": Uuid::new_v4(),
            "expected_place_revision": fresh_revision,
            "prose": prose,
            "consequence": {
                "type": "change_entity_state",
                "property_change": property_change
            }
        })
    };
    let invalid = [
        (
            invalid_action(Vec::new(), "An empty Property consequence must fail."),
            "invalid_action",
            "empty",
        ),
        (
            invalid_action(
                vec![
                    json!({"entity_id": actor_entity_id, "key": "duplicate", "value": {"type": "text", "text": "first"}}),
                    json!({"entity_id": actor_entity_id, "key": "duplicate", "value": {"type": "text", "text": "second"}}),
                ],
                "A duplicate Entity and key pair must fail.",
            ),
            "invalid_property",
            "duplicate",
        ),
        (
            invalid_action(
                vec![
                    json!({"entity_id": actor_entity_id, "key": "Bad Key", "value": {"type": "text", "text": "no"}}),
                ],
                "An invalid canonical key must fail.",
            ),
            "invalid_property",
            "invalid_format",
        ),
    ];
    for (input, code, reason) in invalid {
        let response = server
            .client
            .post(format!("{}/api/action", server.base_url))
            .header(USER_CONTEXT_HEADER, actor.id.0.to_string())
            .json(&input)
            .send()
            .await
            .expect("invalid Property Action should send");
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        let http_error: Value = response
            .json()
            .await
            .expect("invalid Property error should be JSON");
        let mut mcp_input = input;
        mcp_input["request_id"] = json!(Uuid::new_v4());
        let mcp = server
            .tool("submit_action", mcp_input, Some(actor.id.0))
            .await;
        assert_eq!(error_code(&http_error), code);
        assert_eq!(http_error["error"]["reason"], reason);
        assert_eq!(mcp_error(&mcp), http_error);
    }

    let overflow_change = (0..101)
        .map(|index| {
            json!({
                "entity_id": actor_entity_id,
                "key": format!("overflow_change_{index:03}"),
                "value": {"type": "integer", "integer": index}
            })
        })
        .collect();
    let overflow_action = invalid_action(
        overflow_change,
        "One hundred and one Property changes must fail atomically.",
    );
    let overflow_response = server
        .client
        .post(format!("{}/api/action", server.base_url))
        .header(USER_CONTEXT_HEADER, actor.id.0.to_string())
        .json(&overflow_action)
        .send()
        .await
        .expect("overflow Property Action should send");
    assert_eq!(overflow_response.status(), StatusCode::BAD_REQUEST);
    let overflow_http: Value = overflow_response
        .json()
        .await
        .expect("overflow Property error should be JSON");
    let overflow_mcp = server
        .tool("submit_action", overflow_action, Some(actor.id.0))
        .await;
    assert_eq!(error_code(&overflow_http), "invalid_property");
    assert_eq!(overflow_http["error"]["reason"], "out_of_range");
    assert_eq!(mcp_error(&overflow_mcp), overflow_http);

    let conflict = invalid_action(
        vec![json!({
            "entity_id": actor_entity_id,
            "key": "actor_000",
            "value": {"type": "integer", "integer": 1}
        })],
        "A canonical key cannot change its value type.",
    );
    let conflict_response = server
        .client
        .post(format!("{}/api/action", server.base_url))
        .header(USER_CONTEXT_HEADER, actor.id.0.to_string())
        .json(&conflict)
        .send()
        .await
        .expect("Property type conflict should send");
    assert_eq!(conflict_response.status(), StatusCode::CONFLICT);
    let conflict_http: Value = conflict_response
        .json()
        .await
        .expect("Property type conflict should be JSON");
    let mut conflict_mcp_input = conflict;
    conflict_mcp_input["request_id"] = json!(Uuid::new_v4());
    let conflict_mcp = server
        .tool("submit_action", conflict_mcp_input, Some(actor.id.0))
        .await;
    assert_eq!(error_code(&conflict_http), "property_key_conflict");
    assert_eq!(mcp_error(&conflict_mcp), conflict_http);

    let stale = json!({
        "request_id": Uuid::new_v4(),
        "expected_place_revision": first_property_page["place_revision"],
        "prose": "A stale Property consequence must not be accepted.",
        "consequence": {
            "type": "change_entity_state",
            "property_change": [
                {"entity_id": actor_entity_id, "key": "stale", "value": {"type": "text", "text": "no"}}
            ]
        }
    });
    let stale_response = server
        .client
        .post(format!("{}/api/action", server.base_url))
        .header(USER_CONTEXT_HEADER, actor.id.0.to_string())
        .json(&stale)
        .send()
        .await
        .expect("stale Property Action should send");
    assert_eq!(stale_response.status(), StatusCode::PRECONDITION_FAILED);

    let overflow_property = text_property("overflow", 101);
    let overflow_entity = json!({
        "name": "Overflow Entity",
        "description": "This Entity must roll back completely.",
        "property": overflow_property
    });
    let overflow_entity_response = server
        .client
        .post(format!("{}/api/entity", server.base_url))
        .header(USER_CONTEXT_HEADER, actor.id.0.to_string())
        .json(&overflow_entity)
        .send()
        .await
        .expect("overflow Entity request should send");
    assert_eq!(overflow_entity_response.status(), StatusCode::BAD_REQUEST);
    let overflow_entity_http: Value = overflow_entity_response
        .json()
        .await
        .expect("overflow Entity error should be JSON");
    let overflow_entity_mcp = server
        .tool("create_entity", overflow_entity, Some(actor.id.0))
        .await;
    assert_eq!(error_code(&overflow_entity_http), "invalid_property");
    assert_eq!(overflow_entity_http["error"]["field"], "property");
    assert_eq!(overflow_entity_http["error"]["reason"], "out_of_range");
    assert_eq!(mcp_error(&overflow_entity_mcp), overflow_entity_http);

    let rejected_state: (i64, i64, i64) = sqlx::query_as(
        r#"
        SELECT
          (SELECT count(*) FROM entity WHERE name = 'Overflow Entity'),
          (SELECT count(*) FROM property_key WHERE key = 'leaked'),
          (SELECT count(*) FROM property_key WHERE key LIKE 'overflow_change_%')
        "#,
    )
    .fetch_one(&pool)
    .await
    .expect("rejected Property state should be queryable");
    assert_eq!(rejected_state, (0, 0, 0));
}
