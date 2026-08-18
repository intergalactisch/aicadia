use super::*;

#[sqlx::test(migrations = "./migration")]
async fn http_and_mcp_share_initial_property_trait_creation_on_all_routes(pool: PgPool) {
    let world = World::new(pool);
    let actor = world.create_user().await.unwrap();
    let target = world.create_user().await.unwrap();
    let server = TestServer::start(world).await;

    let actor_response = server
        .client
        .post(format!("{}/api/character", server.base_url))
        .header(USER_CONTEXT_HEADER, actor.id.0.to_string())
        .json(&json!({
            "name": "Mara",
            "description": "A careful frog keeper.",
            "property": [{"key":"leg_count","value":{"type":"integer","integer":2}}],
            "trait": [{"statement":"Recognizes each frog by its landing sound."}]
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(actor_response.status(), StatusCode::CREATED);
    let actor_character: Value = actor_response.json().await.unwrap();
    let actor_entity_id = actor_character["entity"]["id"].clone();
    let actor_state = server
        .tool("get_character", json!({"limit":100}), Some(actor.id.0))
        .await;
    let actor_association = structured(&actor_state)["current_state"]["association"]
        .as_array()
        .unwrap();
    assert_eq!(actor_association.len(), 2);
    assert_eq!(actor_association[0]["type"], "property");
    assert_eq!(actor_association[1]["type"], "trait");

    server
        .tool(
            "create_character",
            json!({
                "name":"Pip",
                "description":"A heat-scorched three-legged frog.",
                "property":[{"key":"leg_count","value":{"type":"integer","integer":3}}],
                "trait":[{"statement":"Jumps unusually high."}]
            }),
            Some(target.id.0),
        )
        .await;
    let place = server
        .tool(
            "create_entry_place",
            json!({
                "name":"Frog Court",
                "description":"A warm stone court.",
                "property":[{"key":"surface","value":{"type":"text","text":"scorched stone"}}],
                "trait":[{"statement":"Returns each landing as a sharp echo."}]
            }),
            Some(actor.id.0),
        )
        .await;
    assert_eq!(structured(&place)["entity"]["name"], "Frog Court");

    let ordinary_response = server
        .client
        .post(format!("{}/api/entity", server.base_url))
        .header(USER_CONTEXT_HEADER, actor.id.0.to_string())
        .json(&json!({
            "name":"Warm Basin",
            "description":"A shallow copper basin.",
            "property":[{"key":"material","value":{"type":"text","text":"copper"}}],
            "trait":[{"statement":"Sings after an unusually high landing."}]
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(ordinary_response.status(), StatusCode::CREATED);

    for user_id in [actor.id.0, target.id.0] {
        server.tool("enter_world", json!({}), Some(user_id)).await;
    }
    let context = server
        .tool(
            "list_entity_at_current_place",
            json!({"limit":100}),
            Some(actor.id.0),
        )
        .await;
    let introduced = server
        .tool(
            "submit_action",
            json!({
                "request_id":Uuid::new_v4(),
                "expected_place_revision":structured(&context)["place_revision"],
                "prose":"Mara settles a springboard beside the court.",
                "consequence":{
                    "type":"introduce_entity",
                    "name":"Copper Springboard",
                    "description":"A low flexible copper board.",
                    "property":[{"key":"material","value":{"type":"text","text":"copper"}}],
                    "trait":[{"statement":"Throws three-legged frogs into unusually high arcs."}]
                }
            }),
            Some(actor.id.0),
        )
        .await;
    let introduced = structured(&introduced);
    assert_eq!(
        introduced["activity"]["property_change"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
    assert_eq!(
        introduced["activity"]["trait_change"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
    let introduced_id = introduced["consequence"]["entity"]["id"].clone();

    let fetched = server
        .tool(
            "get_entity_at_current_place",
            json!({"entity_id":introduced_id,"limit":100}),
            Some(actor.id.0),
        )
        .await;
    let association = structured(&fetched)["current_state"]["association"]
        .as_array()
        .unwrap();
    assert_eq!(association.len(), 2);
    assert_eq!(association[0]["type"], "property");
    assert_eq!(association[1]["type"], "trait");

    let history = server
        .tool("list_activity", json!({"limit":100}), Some(actor.id.0))
        .await;
    let activity = structured(&history)["activity"].as_array().unwrap();
    for operation in [
        "create_character",
        "create_entry_place",
        "create_entity",
        "submit_action",
    ] {
        assert!(activity.iter().any(|row| {
            row["operation"] == operation
                && row["trait_change"]
                    .as_array()
                    .is_some_and(|change| change.len() == 1)
        }));
    }
    assert_eq!(
        actor_entity_id,
        structured(&actor_state)["character"]["entity"]["id"]
    );
}
