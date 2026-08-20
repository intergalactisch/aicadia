use super::*;

#[sqlx::test(migrations = "./migration")]
async fn http_and_mcp_share_canonical_capability_errors(pool: PgPool) {
    let world = World::new(pool);
    let user = world.create_user().await.expect("setup User should exist");
    let server = TestServer::start(world).await;

    let response = server
        .client
        .get(format!("{}/api/character", server.base_url))
        .header(USER_CONTEXT_HEADER, user.id.0.to_string())
        .send()
        .await
        .expect("missing Character request should send");
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    let missing_character_http: Value = response.json().await.expect("error should be JSON");
    let missing_character_mcp = server
        .tool("get_character", json!({}), Some(user.id.0))
        .await;
    assert_eq!(error_code(&missing_character_http), "character_not_found");
    assert_eq!(mcp_error(&missing_character_mcp), missing_character_http);

    let invalid_character = json!({"name": "   ", "description": "Valid"});
    let response = server
        .client
        .post(format!("{}/api/character", server.base_url))
        .header(USER_CONTEXT_HEADER, user.id.0.to_string())
        .json(&invalid_character)
        .send()
        .await
        .expect("invalid Character request should send");
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let invalid_character_http: Value = response.json().await.expect("error should be JSON");
    let invalid_character_mcp = server
        .tool("create_character", invalid_character, Some(user.id.0))
        .await;
    assert_eq!(error_code(&invalid_character_http), "invalid_character");
    assert_eq!(mcp_error(&invalid_character_mcp), invalid_character_http);

    let valid_character = json!({"name": "Mara Venn", "description": "A surveyor."});
    server
        .tool("create_character", valid_character.clone(), Some(user.id.0))
        .await;
    let response = server
        .client
        .post(format!("{}/api/character", server.base_url))
        .header(USER_CONTEXT_HEADER, user.id.0.to_string())
        .json(&valid_character)
        .send()
        .await
        .expect("duplicate Character request should send");
    assert_eq!(response.status(), StatusCode::CONFLICT);
    let duplicate_character_http: Value = response.json().await.expect("error should be JSON");
    let duplicate_character_mcp = server
        .tool("create_character", valid_character, Some(user.id.0))
        .await;
    assert_eq!(
        error_code(&duplicate_character_http),
        "character_already_exists"
    );
    assert_eq!(
        mcp_error(&duplicate_character_mcp),
        duplicate_character_http
    );

    let response = server
        .client
        .post(format!("{}/api/world/entry", server.base_url))
        .header(USER_CONTEXT_HEADER, user.id.0.to_string())
        .send()
        .await
        .expect("entry without a Place should send");
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    let no_entry_http: Value = response.json().await.expect("error should be JSON");
    let no_entry_mcp = server.tool("enter_world", json!({}), Some(user.id.0)).await;
    assert_eq!(error_code(&no_entry_http), "entry_place_not_found");
    assert_eq!(mcp_error(&no_entry_mcp), no_entry_http);

    let invalid_place = json!({"name": "   ", "description": "Valid"});
    let response = server
        .client
        .post(format!("{}/api/place/entry", server.base_url))
        .header(USER_CONTEXT_HEADER, user.id.0.to_string())
        .json(&invalid_place)
        .send()
        .await
        .expect("invalid entry Place should send");
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let invalid_place_http: Value = response.json().await.expect("error should be JSON");
    let invalid_place_mcp = server
        .tool("create_entry_place", invalid_place, Some(user.id.0))
        .await;
    assert_eq!(error_code(&invalid_place_http), "invalid_place");
    assert_eq!(mcp_error(&invalid_place_mcp), invalid_place_http);

    server
        .tool(
            "create_entry_place",
            json!({"name": "North Gate", "description": "The shared entry."}),
            Some(user.id.0),
        )
        .await;
    let duplicate_place = json!({"name": "Other Gate", "description": "Must not exist."});
    let response = server
        .client
        .post(format!("{}/api/place/entry", server.base_url))
        .header(USER_CONTEXT_HEADER, user.id.0.to_string())
        .json(&duplicate_place)
        .send()
        .await
        .expect("duplicate entry Place should send");
    assert_eq!(response.status(), StatusCode::CONFLICT);
    let duplicate_place_http: Value = response.json().await.expect("error should be JSON");
    let duplicate_place_mcp = server
        .tool("create_entry_place", duplicate_place, Some(user.id.0))
        .await;
    assert_eq!(
        error_code(&duplicate_place_http),
        "entry_place_already_exists"
    );
    assert_eq!(mcp_error(&duplicate_place_mcp), duplicate_place_http);

    let response = server
        .client
        .get(format!("{}/api/user", server.base_url))
        .send()
        .await
        .expect("missing-context request should send");
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let missing_http: Value = response.json().await.expect("error should be JSON");
    let missing_mcp = server.tool("get_user", json!({}), None).await;
    assert_eq!(
        missing_http,
        json!({
            "error": {
                "code": "user_context_required",
                "message": "Aicadia-User-Id is required."
            }
        })
    );
    assert_eq!(error_code(&missing_http), "user_context_required");
    let missing_mcp_error = mcp_error(&missing_mcp);
    assert_eq!(
        mcp_error_code(&missing_mcp_error),
        error_code(&missing_http)
    );
    assert_eq!(missing_mcp["result"]["isError"], true);
    assert!(missing_mcp["result"].get("structuredContent").is_none());
    assert_eq!(
        missing_mcp["result"]["content"].as_array().unwrap().len(),
        1
    );

    let response = server
        .client
        .get(format!("{}/api/user", server.base_url))
        .header(USER_CONTEXT_HEADER, "not-a-uuid")
        .send()
        .await
        .expect("malformed-context request should send");
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let malformed_http: Value = response.json().await.expect("error should be JSON");
    let (_, malformed_mcp) = server
        .mcp_with_user_context(
            "tools/call",
            Some("get_user"),
            json!({"name": "get_user", "arguments": {}}),
            Some("not-a-uuid"),
            None,
        )
        .await;
    assert_eq!(error_code(&malformed_http), "invalid_request");
    assert_eq!(malformed_http["error"]["field"], USER_CONTEXT_HEADER);
    assert_eq!(malformed_http["error"]["reason"], "invalid_uuid");
    assert_eq!(
        mcp_error_code(&mcp_error(&malformed_mcp)),
        "invalid_request"
    );

    let user_context = user.id.0.to_string();
    let response = server
        .client
        .get(format!("{}/api/user", server.base_url))
        .header(USER_CONTEXT_HEADER, &user_context)
        .header(USER_CONTEXT_HEADER, &user_context)
        .send()
        .await
        .expect("duplicate-context request should send");
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let duplicate_http: Value = response.json().await.expect("error should be JSON");
    let (status, duplicate_mcp) = server
        .mcp_with_user_contexts(
            "tools/call",
            Some("get_user"),
            json!({"name": "get_user", "arguments": {}}),
            &[&user_context, &user_context],
            None,
        )
        .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(error_code(&duplicate_http), "invalid_request");
    assert_eq!(duplicate_http["error"]["field"], USER_CONTEXT_HEADER);
    assert_eq!(duplicate_http["error"]["reason"], "multiple_values");
    assert_eq!(mcp_error(&duplicate_mcp), duplicate_http);

    let comma_separated = format!("{user_context}, {user_context}");
    let response = server
        .client
        .get(format!("{}/api/user", server.base_url))
        .header(USER_CONTEXT_HEADER, comma_separated)
        .send()
        .await
        .expect("comma-separated context request should send");
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let comma_http: Value = response.json().await.expect("error should be JSON");
    assert_eq!(comma_http["error"]["reason"], "multiple_values");

    let unknown_user = Uuid::new_v4();
    let response = server
        .client
        .get(format!("{}/api/user", server.base_url))
        .header(USER_CONTEXT_HEADER, unknown_user.to_string())
        .send()
        .await
        .expect("unknown-context request should send");
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    let unknown_http: Value = response.json().await.expect("error should be JSON");
    let unknown_mcp = server.tool("get_user", json!({}), Some(unknown_user)).await;
    assert_eq!(error_code(&unknown_http), "user_not_found");
    assert_eq!(
        mcp_error_code(&mcp_error(&unknown_mcp)),
        error_code(&unknown_http)
    );

    let response = server
        .client
        .get(format!("{}/api/character", server.base_url))
        .header(USER_CONTEXT_HEADER, unknown_user.to_string())
        .send()
        .await
        .expect("unknown Character owner request should send");
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    let unknown_character_http: Value = response.json().await.expect("error should be JSON");
    let unknown_character_mcp = server
        .tool("get_character", json!({}), Some(unknown_user))
        .await;
    assert_eq!(error_code(&unknown_character_http), "user_not_found");
    assert_eq!(mcp_error(&unknown_character_mcp), unknown_character_http);

    let valid_entity = json!({"name": "Context Matrix", "description": "Not stored."});
    let response = server
        .client
        .post(format!("{}/api/entity", server.base_url))
        .json(&valid_entity)
        .send()
        .await
        .expect("missing create context should send");
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let missing_create_http: Value = response.json().await.expect("error should be JSON");
    let missing_create_mcp = server
        .tool("create_entity", valid_entity.clone(), None)
        .await;
    assert_eq!(mcp_error(&missing_create_mcp), missing_create_http);

    let response = server
        .client
        .post(format!("{}/api/entity", server.base_url))
        .header(USER_CONTEXT_HEADER, "not-a-uuid")
        .json(&valid_entity)
        .send()
        .await
        .expect("malformed create context should send");
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let malformed_create_http: Value = response.json().await.expect("error should be JSON");
    let (_, malformed_create_mcp) = server
        .mcp_with_user_context(
            "tools/call",
            Some("create_entity"),
            json!({"name": "create_entity", "arguments": valid_entity.clone()}),
            Some("not-a-uuid"),
            None,
        )
        .await;
    assert_eq!(mcp_error(&malformed_create_mcp), malformed_create_http);

    let response = server
        .client
        .post(format!("{}/api/entity", server.base_url))
        .header(USER_CONTEXT_HEADER, &user_context)
        .header(USER_CONTEXT_HEADER, &user_context)
        .json(&valid_entity)
        .send()
        .await
        .expect("duplicate create context should send");
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let duplicate_create_http: Value = response.json().await.expect("error should be JSON");
    let (_, duplicate_create_mcp) = server
        .mcp_with_user_contexts(
            "tools/call",
            Some("create_entity"),
            json!({"name": "create_entity", "arguments": valid_entity.clone()}),
            &[&user_context, &user_context],
            None,
        )
        .await;
    assert_eq!(mcp_error(&duplicate_create_mcp), duplicate_create_http);

    let response = server
        .client
        .post(format!("{}/api/entity", server.base_url))
        .header(USER_CONTEXT_HEADER, unknown_user.to_string())
        .json(&valid_entity)
        .send()
        .await
        .expect("unknown create context should send");
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    let unknown_create_http: Value = response.json().await.expect("error should be JSON");
    let unknown_create_mcp = server
        .tool("create_entity", valid_entity, Some(unknown_user))
        .await;
    assert_eq!(mcp_error(&unknown_create_mcp), unknown_create_http);

    let response = server
        .client
        .post(format!("{}/api/entity", server.base_url))
        .header(USER_CONTEXT_HEADER, user.id.0.to_string())
        .json(&json!({"name": "   ", "description": "Valid"}))
        .send()
        .await
        .expect("invalid Entity request should send");
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let invalid_http: Value = response.json().await.expect("error should be JSON");
    let invalid_mcp = server
        .tool(
            "create_entity",
            json!({"name": "   ", "description": "Valid"}),
            Some(user.id.0),
        )
        .await;
    assert_eq!(error_code(&invalid_http), "invalid_entity");
    assert_eq!(invalid_http["error"]["field"], "name");
    assert_eq!(invalid_http["error"]["reason"], "empty");
    assert_eq!(mcp_error(&invalid_mcp), invalid_http);

    for limit in [-1, 0, 101, 65_536] {
        let response = server
            .client
            .get(format!("{}/api/activity?limit={limit}", server.base_url))
            .header(USER_CONTEXT_HEADER, user.id.0.to_string())
            .send()
            .await
            .expect("out-of-range activity limit request should send");
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        let limit_http: Value = response.json().await.expect("error should be JSON");
        let limit_mcp = server
            .tool("list_activity", json!({"limit": limit}), Some(user.id.0))
            .await;
        assert_eq!(error_code(&limit_http), "invalid_activity_limit");
        assert_eq!(mcp_error(&limit_mcp), limit_http);
    }
}

#[sqlx::test(migrations = "./migration")]
async fn http_and_mcp_share_investigation_start_errors(pool: PgPool) {
    let world = World::new(pool);
    let owner = world.create_user().await.expect("setup User should exist");
    let bystander = world
        .create_user()
        .await
        .expect("Character-less User should exist");
    let server = TestServer::start(world).await;

    let response = server
        .client
        .post(format!("{}/api/investigation", server.base_url))
        .header(USER_CONTEXT_HEADER, bystander.id.0.to_string())
        .json(&json!({"request_id": Uuid::new_v4(), "kind": "entity_at_position"}))
        .send()
        .await
        .expect("Character-less investigation should send");
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    let no_character_http: Value = response.json().await.expect("error should be JSON");
    let no_character_mcp = server
        .tool(
            "start_investigation",
            json!({"request_id": Uuid::new_v4(), "kind": "entity_at_position"}),
            Some(bystander.id.0),
        )
        .await;
    assert_eq!(error_code(&no_character_http), "character_not_found");
    assert_eq!(mcp_error(&no_character_mcp), no_character_http);

    server
        .tool(
            "create_character",
            json!({"name": "Mara Venn", "description": "A surveyor."}),
            Some(owner.id.0),
        )
        .await;
    let response = server
        .client
        .post(format!("{}/api/investigation", server.base_url))
        .header(USER_CONTEXT_HEADER, owner.id.0.to_string())
        .json(&json!({"request_id": Uuid::new_v4(), "kind": "entity_at_position"}))
        .send()
        .await
        .expect("unplaced investigation should send");
    assert_eq!(response.status(), StatusCode::CONFLICT);
    let not_entered_http: Value = response.json().await.expect("error should be JSON");
    let not_entered_mcp = server
        .tool(
            "start_investigation",
            json!({"request_id": Uuid::new_v4(), "kind": "entity_at_position"}),
            Some(owner.id.0),
        )
        .await;
    assert_eq!(error_code(&not_entered_http), "character_not_entered");
    assert_eq!(mcp_error(&not_entered_mcp), not_entered_http);
}
