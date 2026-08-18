use super::*;

#[sqlx::test(migrations = "./migration")]
async fn interaction_http_and_mcp_share_strict_commit_errors_and_scoped_results(pool: PgPool) {
    let world = World::new(pool);
    let actor = world.create_user().await.expect("actor User should exist");
    let target = world.create_user().await.expect("target User should exist");
    let server = TestServer::start(world).await;

    let actor_character = server
        .tool(
            "create_character",
            json!({"name": "Pip", "description": "A tiny grey wanderer."}),
            Some(actor.id.0),
        )
        .await;
    let actor_entity_id = structured(&actor_character)["entity"]["id"].clone();
    server
        .tool(
            "create_character",
            json!({"name": "Mara", "description": "A patient keeper of the square."}),
            Some(target.id.0),
        )
        .await;
    server
        .tool(
            "create_entry_place",
            json!({
                "name": "First Landing",
                "description": "A quiet square where paths begin."
            }),
            Some(actor.id.0),
        )
        .await;
    for user_id in [actor.id.0, target.id.0] {
        server.tool("enter_world", json!({}), Some(user_id)).await;
    }

    let actor_context: Value = server
        .client
        .get(format!("{}/api/place/current/entity", server.base_url))
        .header(USER_CONTEXT_HEADER, actor.id.0.to_string())
        .send()
        .await
        .expect("actor context should send")
        .json()
        .await
        .expect("actor context should be JSON");
    assert_eq!(actor_context["entity"].as_array().map(Vec::len), Some(1));
    let target_entity = &actor_context["entity"][0];
    assert_eq!(target_entity["name"], "Mara");
    for value in [&actor_context["place"], target_entity] {
        assert_eq!(
            value
                .as_object()
                .expect("scoped result should be an object")
                .keys()
                .cloned()
                .collect::<BTreeSet<_>>(),
            BTreeSet::from(["description".to_owned(), "id".to_owned(), "name".to_owned()])
        );
    }
    let serialized_context = actor_context.to_string();
    for forbidden in [
        "introduced_by_user_id",
        "owner_user_id",
        "is_entry",
        "user_controlled",
        "npc",
    ] {
        assert!(!serialized_context.contains(forbidden));
    }

    let target_context = server
        .tool("list_entity_at_current_place", json!({}), Some(target.id.0))
        .await;
    let rat = structured(&target_context)["entity"]
        .as_array()
        .expect("target context should contain safe Entities")
        .iter()
        .find(|entity| entity["name"] == "Pip")
        .expect("Mara should receive Pip as an ordinary local subject");
    assert_eq!(rat["description"], "A tiny grey wanderer.");
    assert_eq!(
        rat.as_object()
            .expect("safe contextual Entity should be an object")
            .keys()
            .cloned()
            .collect::<BTreeSet<_>>(),
        BTreeSet::from(["description".to_owned(), "id".to_owned(), "name".to_owned()])
    );
    for forbidden in [
        "introduced_by_user_id",
        "owner_user_id",
        "character",
        "user_controlled",
        "control",
        "npc",
    ] {
        assert!(
            !rat.to_string().to_lowercase().contains(forbidden),
            "the rat's typed player result must not disclose {forbidden}"
        );
    }

    let request_id = Uuid::new_v4();
    let interaction = json!({
        "request_id": request_id,
        "expected_place_revision": actor_context["place_revision"],
        "prose": "Pip darts in a small circle around Mara's boots, then sits very still.",
        "target_entity_id": [target_entity["id"]]
    });
    let response = server
        .client
        .post(format!("{}/api/interaction", server.base_url))
        .header(USER_CONTEXT_HEADER, actor.id.0.to_string())
        .json(&interaction)
        .send()
        .await
        .expect("HTTP Interaction should send");
    assert_eq!(response.status(), StatusCode::CREATED);
    let accepted: Value = response
        .json()
        .await
        .expect("accepted Interaction should be JSON");
    assert_eq!(accepted["activity"]["operation"], "submit_interaction");
    assert_eq!(accepted["activity"]["actor_character"]["name"], "Pip");
    assert_eq!(accepted["activity"]["prose"], interaction["prose"]);
    assert_eq!(accepted["activity"]["property_change"], json!([]));
    assert_eq!(
        accepted["activity"]["involved_entity"]
            .as_array()
            .expect("Interaction involvement should be an array")
            .iter()
            .map(|reference| reference["role"].as_str().expect("role should be text"))
            .collect::<BTreeSet<_>>(),
        BTreeSet::from(["location", "target"])
    );
    assert_eq!(
        accepted["place"]
            .as_object()
            .expect("accepted Place should be safe")
            .keys()
            .cloned()
            .collect::<BTreeSet<_>>(),
        BTreeSet::from(["description".to_owned(), "id".to_owned(), "name".to_owned()])
    );
    for forbidden in ["introduced_by_user_id", "owner_user_id", "response"] {
        assert!(!accepted.to_string().contains(forbidden));
    }

    let retry = server
        .tool("submit_interaction", interaction.clone(), Some(actor.id.0))
        .await;
    assert_eq!(structured(&retry), &accepted);

    let target_history = server
        .tool("list_activity", json!({"limit": 100}), Some(target.id.0))
        .await;
    let observed = structured(&target_history)["activity"]
        .as_array()
        .expect("target history should be an array")
        .iter()
        .find(|activity| activity["id"] == accepted["activity"]["id"])
        .expect("target history should contain the outward Interaction");
    assert_eq!(observed, &accepted["activity"]);
    assert_eq!(observed["prose"], interaction["prose"]);

    let changed = json!({
        "request_id": request_id,
        "expected_place_revision": actor_context["place_revision"],
        "prose": "Different outward behavior under an accepted request id.",
        "target_entity_id": [target_entity["id"]]
    });
    let response = server
        .client
        .post(format!("{}/api/interaction", server.base_url))
        .header(USER_CONTEXT_HEADER, actor.id.0.to_string())
        .json(&changed)
        .send()
        .await
        .expect("conflicting HTTP Interaction should send");
    assert_eq!(response.status(), StatusCode::CONFLICT);
    let conflict_http: Value = response.json().await.expect("conflict should be JSON");
    let conflict_mcp = server
        .tool("submit_interaction", changed, Some(actor.id.0))
        .await;
    assert_eq!(error_code(&conflict_http), "interaction_request_conflict");
    assert_eq!(mcp_error(&conflict_mcp), conflict_http);

    let fresh_context = server
        .tool("list_entity_at_current_place", json!({}), Some(actor.id.0))
        .await;
    let fresh_revision = structured(&fresh_context)["place_revision"].clone();
    let empty_target = json!({
        "request_id": Uuid::new_v4(),
        "expected_place_revision": fresh_revision,
        "prose": "This has no directed target.",
        "target_entity_id": []
    });
    let response = server
        .client
        .post(format!("{}/api/interaction", server.base_url))
        .header(USER_CONTEXT_HEADER, actor.id.0.to_string())
        .json(&empty_target)
        .send()
        .await
        .expect("invalid HTTP Interaction should send");
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let invalid_http: Value = response.json().await.expect("invalid input should be JSON");
    let mut empty_target_mcp = empty_target;
    empty_target_mcp["request_id"] = json!(Uuid::new_v4());
    let invalid_mcp = server
        .tool("submit_interaction", empty_target_mcp, Some(actor.id.0))
        .await;
    assert_eq!(error_code(&invalid_http), "invalid_interaction");
    assert_eq!(invalid_http["error"]["field"], "target_entity_id");
    assert_eq!(mcp_error(&invalid_mcp), invalid_http);

    let unavailable = json!({
        "request_id": Uuid::new_v4(),
        "expected_place_revision": fresh_revision,
        "prose": "Pip cannot direct an Interaction at Pip.",
        "target_entity_id": [actor_entity_id]
    });
    let response = server
        .client
        .post(format!("{}/api/interaction", server.base_url))
        .header(USER_CONTEXT_HEADER, actor.id.0.to_string())
        .json(&unavailable)
        .send()
        .await
        .expect("unavailable HTTP target should send");
    assert_eq!(response.status(), StatusCode::CONFLICT);
    let unavailable_http: Value = response
        .json()
        .await
        .expect("unavailable target should be JSON");
    let mut unavailable_mcp_input = unavailable;
    unavailable_mcp_input["request_id"] = json!(Uuid::new_v4());
    let unavailable_mcp = server
        .tool(
            "submit_interaction",
            unavailable_mcp_input,
            Some(actor.id.0),
        )
        .await;
    assert_eq!(
        error_code(&unavailable_http),
        "interaction_target_unavailable"
    );
    assert_eq!(mcp_error(&unavailable_mcp), unavailable_http);
}
