use super::*;

#[sqlx::test(migrations = "./migration")]
async fn trait_http_and_mcp_share_contextual_change_entity_fetch_and_history(pool: PgPool) {
    let world = World::new(pool);
    let actor = world.create_user().await.unwrap();
    let target = world.create_user().await.unwrap();
    let server = TestServer::start(world).await;

    let actor_character = server
        .tool(
            "create_character",
            json!({"name":"Mara","description":"A careful surveyor.","property":[{"key":"pace","value":{"type":"integer","integer":2}}]}),
            Some(actor.id.0),
        )
        .await;
    let actor_entity_id = structured(&actor_character)["entity"]["id"].clone();
    let target_character = server
        .tool(
            "create_character",
            json!({"name":"Pip","description":"A tiny grey wanderer."}),
            Some(target.id.0),
        )
        .await;
    let target_entity_id = structured(&target_character)["entity"]["id"].clone();
    server
        .tool(
            "create_entry_place",
            json!({"name":"First Landing","description":"A quiet square."}),
            Some(actor.id.0),
        )
        .await;
    for user_id in [actor.id.0, target.id.0] {
        server.tool("enter_world", json!({}), Some(user_id)).await;
    }

    let context = server
        .tool("list_entity_at_current_place", json!({}), Some(actor.id.0))
        .await;
    let place_entity_id = structured(&context)["place"]["id"].clone();
    let introduced = server
        .tool(
            "submit_action",
            json!({
                "request_id":Uuid::new_v4(),
                "expected_place_revision":structured(&context)["place_revision"],
                "prose":"Mara sets a small brass bell beside the landing.",
                "consequence":{"type":"introduce_entity","name":"Brass Bell","description":"A small bell with a clear tone."}
            }),
            Some(actor.id.0),
        )
        .await;
    let ordinary_entity_id = structured(&introduced)["consequence"]["entity"]["id"].clone();
    let context = server
        .tool("list_entity_at_current_place", json!({}), Some(actor.id.0))
        .await;
    let establish = json!({
        "request_id": Uuid::new_v4(),
        "expected_place_revision": structured(&context)["place_revision"],
        "prose": "Mara watches Pip wait for the returning echo.",
        "consequence": {
            "type": "change_entity_trait",
            "trait_change": [
                {"type":"establish","entity_id":actor_entity_id,"statement":"Waits for the second echo before moving."},
                {"type":"establish","entity_id":target_entity_id,"statement":"Jumps unusually high when startled."},
                {"type":"establish","entity_id":place_entity_id,"statement":"Carries a returning echo beneath its arches."},
                {"type":"establish","entity_id":ordinary_entity_id,"statement":"Rings with an unusually clear second tone."}
            ]
        }
    });
    let response = server
        .client
        .post(format!("{}/api/action", server.base_url))
        .header(USER_CONTEXT_HEADER, actor.id.0.to_string())
        .json(&establish)
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    let accepted: Value = response.json().await.unwrap();
    assert_eq!(accepted["consequence"]["type"], "change_entity_trait");
    assert_eq!(
        accepted["activity"]["trait_change"]
            .as_array()
            .map(Vec::len),
        Some(4)
    );
    assert_eq!(accepted["activity"]["property_change"], json!([]));
    let actor_trait_id = accepted["consequence"]["trait_change"]
        .as_array()
        .unwrap()
        .iter()
        .find(|change| change["entity"]["id"] == actor_entity_id)
        .unwrap()["trait"]["id"]
        .clone();

    let actor_state = server
        .tool(
            "get_entity_at_current_place",
            json!({"entity_id":actor_entity_id,"limit":1}),
            Some(actor.id.0),
        )
        .await;
    assert_eq!(
        structured(&actor_state)["current_state"]["association"][0]["type"],
        "property"
    );
    let stale_cursor = structured(&actor_state)["current_state"]["next"].clone();
    let interaction = json!({
        "request_id": Uuid::new_v4(),
        "expected_place_revision": structured(&actor_state)["place_revision"],
        "prose": "Mara taps twice; Pip springs closer.",
        "target_entity_id": [target_entity_id],
        "property_change": [{"entity_id":target_entity_id,"key":"stance","value":{"type":"text","text":"close"}}],
        "trait_change": [{"type":"develop","trait_id":actor_trait_id,"statement":"Waits for Pip's second footfall before moving."}]
    });
    let developed = server
        .tool("submit_interaction", interaction.clone(), Some(actor.id.0))
        .await;
    let developed = structured(&developed);
    assert_eq!(developed["activity"]["trait_change"][0]["type"], "develop");
    assert_eq!(
        developed["activity"]["trait_change"][0]["previous_statement"],
        "Waits for the second echo before moving."
    );
    assert_eq!(
        developed["activity"]["property_change"]
            .as_array()
            .map(Vec::len),
        Some(1)
    );
    assert!(developed.get("response").is_none());

    let stale = server
        .tool(
            "get_entity_at_current_place",
            json!({"entity_id":actor_entity_id,"cursor":stale_cursor,"limit":1}),
            Some(actor.id.0),
        )
        .await;
    assert_eq!(error_code(&mcp_error(&stale)), "place_revision_conflict");

    let page: Value = server
        .client
        .get(format!("{}/api/character?limit=100", server.base_url))
        .header(USER_CONTEXT_HEADER, actor.id.0.to_string())
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(page["character"]["entity"]["id"], actor_entity_id);
    let current_trait = page["current_state"]["association"]
        .as_array()
        .unwrap()
        .iter()
        .find(|association| association["type"] == "trait")
        .unwrap();
    assert_eq!(current_trait["trait"]["id"], actor_trait_id);
    assert_eq!(
        current_trait["trait"]["statement"],
        "Waits for Pip's second footfall before moving."
    );
    assert!(!page.to_string().contains("owner_user_id\":null"));

    let no_op = json!({
        "request_id":Uuid::new_v4(),
        "expected_place_revision":page["place_revision"],
        "prose":"Nothing changes.",
        "consequence":{"type":"change_entity_trait","trait_change":[{"type":"develop","trait_id":actor_trait_id,"statement":"Waits for Pip's second footfall before moving."}]}
    });
    let no_op_response = server.tool("submit_action", no_op, Some(actor.id.0)).await;
    assert_eq!(error_code(&mcp_error(&no_op_response)), "invalid_trait");

    let target_first: Value = server
        .client
        .get(format!(
            "{}/api/place/current/entity/{}?limit=1",
            server.base_url,
            target_entity_id.as_str().unwrap()
        ))
        .header(USER_CONTEXT_HEADER, actor.id.0.to_string())
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(
        target_first["current_state"]["association"][0]["type"],
        "property"
    );
    let target_second = server.tool(
        "get_entity_at_current_place",
        json!({"entity_id":target_entity_id,"cursor":target_first["current_state"]["next"],"limit":1}),
        Some(actor.id.0)
    ).await;
    assert_eq!(
        structured(&target_second)["place_revision"],
        target_first["place_revision"]
    );
    assert_eq!(
        structured(&target_second)["current_state"]["association"][0]["type"],
        "trait"
    );

    let (removed_status, removed) = server
        .mcp(
            "tools/call",
            Some("list_entity_property_at_current_place"),
            json!({"name":"list_entity_property_at_current_place","arguments":{}}),
            Some(actor.id.0),
            None,
        )
        .await;
    assert_eq!(removed_status, StatusCode::BAD_REQUEST);
    assert_eq!(removed["error"]["code"], -32602);
}
