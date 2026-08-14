use super::*;

#[sqlx::test(migrations = "./migration")]
async fn http_and_mcp_share_successful_world_state(pool: PgPool) {
    let world = World::new(pool);
    let user = world.create_user().await.expect("setup User should exist");
    let second_user = world.create_user().await.expect("second User should exist");
    let server = TestServer::start(world).await;

    let http_world: Value = server
        .client
        .get(format!("{}/api/world", server.base_url))
        .send()
        .await
        .expect("World request should send")
        .json()
        .await
        .expect("World should be JSON");
    let mcp_world = server.tool("get_world", json!({}), None).await;
    assert_eq!(http_world, *structured(&mcp_world));

    let http_user: Value = server
        .client
        .get(format!("{}/api/user", server.base_url))
        .header(USER_CONTEXT_HEADER, user.id.0.to_string())
        .send()
        .await
        .expect("User request should send")
        .json()
        .await
        .expect("User should be JSON");
    let mcp_user = server.tool("get_user", json!({}), Some(user.id.0)).await;
    assert_eq!(http_user, *structured(&mcp_user));

    let response = server
        .client
        .post(format!("{}/api/character", server.base_url))
        .header(USER_CONTEXT_HEADER, user.id.0.to_string())
        .json(&json!({
            "name": "Mara Venn",
            "description": "A careful surveyor at the edge of the known World."
        }))
        .send()
        .await
        .expect("Character create should send");
    assert_eq!(response.status(), StatusCode::CREATED);
    let http_character: Value = response.json().await.expect("Character should be JSON");
    assert_eq!(http_character["owner_user_id"], user.id.0.to_string());
    let mcp_character = server
        .tool("get_character", json!({}), Some(user.id.0))
        .await;
    assert_eq!(http_character, structured(&mcp_character)["character"]);
    let character_entity: Value = server
        .client
        .get(format!(
            "{}/api/entity/{}",
            server.base_url,
            http_character["entity"]["id"]
                .as_str()
                .expect("Character Entity id should be text")
        ))
        .send()
        .await
        .expect("operator Entity lookup should send")
        .json()
        .await
        .expect("operator Entity lookup should be JSON");
    assert_eq!(character_entity, http_character["entity"]);

    let mcp_second_character = server
        .tool(
            "create_character",
            json!({
                "name": "Tomas Reed",
                "description": "A patient observer of changes in the shared World."
            }),
            Some(second_user.id.0),
        )
        .await;
    let http_second_character: Value = server
        .client
        .get(format!("{}/api/character", server.base_url))
        .header(USER_CONTEXT_HEADER, second_user.id.0.to_string())
        .send()
        .await
        .expect("Character read should send")
        .json()
        .await
        .expect("Character should be JSON");
    assert_eq!(
        http_second_character["character"],
        *structured(&mcp_second_character)
    );

    let response = server
        .client
        .post(format!("{}/api/place/entry", server.base_url))
        .header(USER_CONTEXT_HEADER, user.id.0.to_string())
        .json(&json!({
            "name": "North Gate",
            "description": "The one established entry into the shared World."
        }))
        .send()
        .await
        .expect("entry Place create should send");
    assert_eq!(response.status(), StatusCode::CREATED);
    let entry_place: Value = response.json().await.expect("entry Place should be JSON");
    assert_eq!(entry_place["is_entry"], true);
    let entered = server.tool("enter_world", json!({}), Some(user.id.0)).await;
    assert_eq!(
        structured(&entered)["current_place"],
        entry_place,
        "MCP entry should use the HTTP-created server-derived Place"
    );
    let response = server
        .client
        .post(format!("{}/api/world/entry", server.base_url))
        .header(USER_CONTEXT_HEADER, second_user.id.0.to_string())
        .send()
        .await
        .expect("second Character entry should send");
    assert_eq!(response.status(), StatusCode::OK);
    let http_entered: Value = response
        .json()
        .await
        .expect("entered Character should be JSON");
    assert_eq!(http_entered["current_place"], entry_place);

    let first_activity: Value = server
        .client
        .get(format!("{}/api/activity?limit=1", server.base_url))
        .header(USER_CONTEXT_HEADER, user.id.0.to_string())
        .send()
        .await
        .expect("activity page should send")
        .json()
        .await
        .expect("activity page should be JSON");
    assert_eq!(first_activity["activity"][0]["operation"], "enter_world");
    let activity_cursor = first_activity["next"]
        .as_str()
        .expect("earlier personal activity should produce a cursor");
    let next_activity = server
        .tool(
            "list_activity",
            json!({"cursor": activity_cursor, "limit": 1}),
            Some(user.id.0),
        )
        .await;
    assert_eq!(
        structured(&next_activity)["activity"][0]["operation"],
        "create_entry_place"
    );

    let response = server
        .client
        .post(format!("{}/api/entity", server.base_url))
        .header(USER_CONTEXT_HEADER, user.id.0.to_string())
        .json(&json!({
            "name": "Old Willow",
            "description": "A mature willow beside Glassmere Lake."
        }))
        .send()
        .await
        .expect("Entity create should send");
    assert_eq!(response.status(), StatusCode::CREATED);
    let http_created: Value = response.json().await.expect("Entity should be JSON");
    let operator_read: Value = server
        .client
        .get(format!(
            "{}/api/entity/{}",
            server.base_url,
            http_created["id"]
                .as_str()
                .expect("Entity id should be text")
        ))
        .send()
        .await
        .expect("operator Entity lookup should send")
        .json()
        .await
        .expect("operator Entity lookup should be JSON");
    assert_eq!(http_created, operator_read);

    let mcp_created = server
        .tool(
            "create_entity",
            json!({
                "name": "Glassmere Lake",
                "description": "A lake shared by every participant."
            }),
            Some(user.id.0),
        )
        .await;
    let mcp_created = structured(&mcp_created);
    let http_list: Value = server
        .client
        .get(format!("{}/api/entity", server.base_url))
        .send()
        .await
        .expect("Entity list should send")
        .json()
        .await
        .expect("Entity list should be JSON");
    assert!(
        http_list["entity"]
            .as_array()
            .expect("Entity page should contain an array")
            .iter()
            .any(|entity| entity["id"] == mcp_created["id"])
    );

    let first_page: Value = server
        .client
        .get(format!("{}/api/entity?limit=1", server.base_url))
        .send()
        .await
        .expect("first page should send")
        .json()
        .await
        .expect("first page should be JSON");
    let cursor = first_page["next"]
        .as_str()
        .expect("two Entities should produce a cursor");
    let second_page: Value = server
        .client
        .get(format!(
            "{}/api/entity?cursor={cursor}&limit=1",
            server.base_url
        ))
        .send()
        .await
        .expect("second operator Entity page should send")
        .json()
        .await
        .expect("second operator Entity page should be JSON");
    assert_ne!(
        first_page["entity"][0]["id"],
        second_page["entity"][0]["id"]
    );
}

#[sqlx::test(migrations = "./migration")]
async fn action_http_and_mcp_share_commit_retry_visibility_and_errors(pool: PgPool) {
    let world = World::new(pool.clone());
    let actor = world.create_user().await.expect("actor User should exist");
    let observer = world
        .create_user()
        .await
        .expect("observer User should exist");
    let server = TestServer::start(world).await;

    server
        .tool(
            "create_character",
            json!({"name": "Mara Venn", "description": "A careful surveyor."}),
            Some(actor.id.0),
        )
        .await;
    server
        .tool(
            "create_character",
            json!({"name": "Tomas Reed", "description": "A patient observer."}),
            Some(observer.id.0),
        )
        .await;

    let unplaced_response = server
        .client
        .get(format!("{}/api/place/current/entity", server.base_url))
        .header(USER_CONTEXT_HEADER, observer.id.0.to_string())
        .send()
        .await
        .expect("unplaced exact-Place read should send");
    assert_eq!(unplaced_response.status(), StatusCode::CONFLICT);
    let unplaced_http: Value = unplaced_response
        .json()
        .await
        .expect("unplaced error should be JSON");
    let unplaced_mcp = server
        .tool(
            "list_entity_at_current_place",
            json!({}),
            Some(observer.id.0),
        )
        .await;
    assert_eq!(error_code(&unplaced_http), "character_not_entered");
    assert_eq!(mcp_error(&unplaced_mcp), unplaced_http);

    let created_place = server
        .tool(
            "create_entry_place",
            json!({
                "name": "North Gate",
                "description": "The one established entry into the shared World."
            }),
            Some(actor.id.0),
        )
        .await;
    let created_place = structured(&created_place).clone();
    let scoped_place = json!({
        "id": created_place["entity"]["id"],
        "name": created_place["entity"]["name"],
        "description": created_place["entity"]["description"]
    });
    for user_id in [actor.id.0, observer.id.0] {
        let response = server
            .client
            .post(format!("{}/api/world/entry", server.base_url))
            .header(USER_CONTEXT_HEADER, user_id.to_string())
            .send()
            .await
            .expect("World entry should send");
        assert_eq!(response.status(), StatusCode::OK);
        let entered: Value = response.json().await.expect("entry should be JSON");
        assert_eq!(entered["current_place"], created_place);
    }

    let entity_context: Value = server
        .client
        .get(format!("{}/api/place/current/entity", server.base_url))
        .header(USER_CONTEXT_HEADER, actor.id.0.to_string())
        .send()
        .await
        .expect("exact-Place Entity read should send")
        .json()
        .await
        .expect("exact-Place Entity page should be JSON");
    let activity_context = server
        .tool(
            "list_activity_at_current_place",
            json!({}),
            Some(actor.id.0),
        )
        .await;
    let activity_context = structured(&activity_context);
    assert_eq!(
        entity_context["place_revision"], activity_context["place_revision"],
        "independent exact-Place reads should expose one shared revision"
    );
    assert_eq!(entity_context["place"], scoped_place);
    assert_eq!(activity_context["place"], scoped_place);
    assert_eq!(entity_context["entity"].as_array().map(Vec::len), Some(1));
    assert_eq!(entity_context["entity"][0]["name"], "Tomas Reed");
    assert_eq!(
        entity_context["entity"][0]
            .as_object()
            .expect("current-Place Entity should be an object")
            .keys()
            .cloned()
            .collect::<BTreeSet<_>>(),
        BTreeSet::from(["description".to_owned(), "id".to_owned(), "name".to_owned()])
    );

    let bad_limit_response = server
        .client
        .get(format!(
            "{}/api/place/current/entity?limit=0",
            server.base_url
        ))
        .header(USER_CONTEXT_HEADER, actor.id.0.to_string())
        .send()
        .await
        .expect("invalid exact-Place Entity limit should send");
    assert_eq!(bad_limit_response.status(), StatusCode::BAD_REQUEST);
    let bad_limit_http: Value = bad_limit_response
        .json()
        .await
        .expect("invalid Entity limit should be JSON");
    let bad_limit_mcp = server
        .tool(
            "list_entity_at_current_place",
            json!({"limit": 0}),
            Some(actor.id.0),
        )
        .await;
    assert_eq!(error_code(&bad_limit_http), "invalid_entity_limit");
    assert_eq!(mcp_error(&bad_limit_mcp), bad_limit_http);

    let bad_cursor_response = server
        .client
        .get(format!(
            "{}/api/place/current/activity?cursor=not-a-cursor",
            server.base_url
        ))
        .header(USER_CONTEXT_HEADER, actor.id.0.to_string())
        .send()
        .await
        .expect("invalid exact-Place Activity cursor should send");
    assert_eq!(bad_cursor_response.status(), StatusCode::BAD_REQUEST);
    let bad_cursor_http: Value = bad_cursor_response
        .json()
        .await
        .expect("invalid Activity cursor should be JSON");
    let bad_cursor_mcp = server
        .tool(
            "list_activity_at_current_place",
            json!({"cursor": "not-a-cursor"}),
            Some(actor.id.0),
        )
        .await;
    assert_eq!(error_code(&bad_cursor_http), "invalid_request");
    assert_eq!(mcp_error(&bad_cursor_mcp), bad_cursor_http);

    let invalid_action = json!({
        "request_id": Uuid::new_v4(),
        "expected_place_revision": entity_context["place_revision"],
        "prose": "   ",
        "consequence": {
            "type": "introduce_entity",
            "name": "Rejected Marker",
            "description": "This marker must not exist."
        }
    });
    let invalid_response = server
        .client
        .post(format!("{}/api/action", server.base_url))
        .header(USER_CONTEXT_HEADER, actor.id.0.to_string())
        .json(&invalid_action)
        .send()
        .await
        .expect("invalid action should send");
    assert_eq!(invalid_response.status(), StatusCode::BAD_REQUEST);
    let invalid_http: Value = invalid_response
        .json()
        .await
        .expect("invalid action error should be JSON");
    let mut invalid_mcp_input = invalid_action;
    invalid_mcp_input["request_id"] = json!(Uuid::new_v4());
    let invalid_mcp = server
        .tool("submit_action", invalid_mcp_input, Some(actor.id.0))
        .await;
    assert_eq!(
        invalid_http,
        json!({
            "error": {
                "code": "invalid_action",
                "message": "Action prose is empty.",
                "field": "prose",
                "reason": "empty"
            }
        })
    );
    assert_eq!(mcp_error(&invalid_mcp), invalid_http);

    let malformed_action = json!({
        "request_id": Uuid::new_v4(),
        "expected_place_revision": "not-a-revision",
        "prose": "This package must not be accepted.",
        "consequence": {
            "type": "introduce_entity",
            "name": "Rejected Marker",
            "description": "This marker must not exist."
        }
    });
    let malformed_response = server
        .client
        .post(format!("{}/api/action", server.base_url))
        .header(USER_CONTEXT_HEADER, actor.id.0.to_string())
        .json(&malformed_action)
        .send()
        .await
        .expect("malformed action should send");
    assert_eq!(malformed_response.status(), StatusCode::BAD_REQUEST);
    let malformed_http: Value = malformed_response
        .json()
        .await
        .expect("malformed error should be JSON");
    let malformed_mcp = server
        .tool("submit_action", malformed_action, Some(actor.id.0))
        .await;
    assert_eq!(error_code(&malformed_http), "invalid_request");
    assert_eq!(malformed_http["error"]["field"], "expected_place_revision");
    assert_eq!(mcp_error(&malformed_mcp), malformed_http);

    let unsupported_consequence = json!({
        "request_id": Uuid::new_v4(),
        "expected_place_revision": entity_context["place_revision"],
        "prose": "This unsupported consequence must not change the World.",
        "consequence": {
            "type": "move_character",
            "name": "Impossible Passage",
            "description": "This consequence is outside the current action surface."
        }
    });
    let unsupported_response = server
        .client
        .post(format!("{}/api/action", server.base_url))
        .header(USER_CONTEXT_HEADER, actor.id.0.to_string())
        .json(&unsupported_consequence)
        .send()
        .await
        .expect("unsupported HTTP consequence should send");
    assert_eq!(unsupported_response.status(), StatusCode::BAD_REQUEST);
    let unsupported_http: Value = unsupported_response
        .json()
        .await
        .expect("unsupported consequence error should be JSON");
    let unsupported_mcp = server
        .tool("submit_action", unsupported_consequence, Some(actor.id.0))
        .await;
    assert_eq!(error_code(&unsupported_http), "invalid_request");
    assert_eq!(mcp_error(&unsupported_mcp), unsupported_http);
    let unsupported_writes: (i64, i64) = sqlx::query_as(
        r#"
        SELECT
            (SELECT count(*) FROM entity WHERE name = 'Impossible Passage'),
            (SELECT count(*) FROM activity WHERE operation = 'submit_action')
        "#,
    )
    .fetch_one(&pool)
    .await
    .expect("unsupported consequence write counts should load");
    assert_eq!(unsupported_writes, (0, 0));

    let request_id = Uuid::new_v4();
    let action = json!({
        "request_id": request_id,
        "expected_place_revision": entity_context["place_revision"],
        "prose": "Mara braces a carved cedar marker beside the crossing.",
        "consequence": {
            "type": "introduce_entity",
            "name": "Cedar Crossing Marker",
            "description": "A waist-high cedar marker carved with three crossing lines."
        }
    });
    let accepted_response = server
        .client
        .post(format!("{}/api/action", server.base_url))
        .header(USER_CONTEXT_HEADER, actor.id.0.to_string())
        .json(&action)
        .send()
        .await
        .expect("action should send");
    assert_eq!(accepted_response.status(), StatusCode::CREATED);
    let accepted: Value = accepted_response
        .json()
        .await
        .expect("accepted action should be JSON");
    assert_eq!(accepted["activity"]["operation"], "submit_action");
    assert_eq!(
        accepted["activity"]["prose"],
        "Mara braces a carved cedar marker beside the crossing."
    );
    assert_eq!(accepted["consequence"]["type"], "introduce_entity");
    assert_eq!(
        accepted["consequence"]["entity"]["name"],
        "Cedar Crossing Marker"
    );
    assert_eq!(accepted["place"], created_place);
    let role = accepted["activity"]["involved_entity"]
        .as_array()
        .expect("Activity roles should be an array")
        .iter()
        .map(|reference| reference["role"].as_str().expect("role should be a string"))
        .collect::<BTreeSet<_>>();
    assert_eq!(role, BTreeSet::from(["location", "subject"]));

    let http_retry = server
        .client
        .post(format!("{}/api/action", server.base_url))
        .header(USER_CONTEXT_HEADER, actor.id.0.to_string())
        .json(&action)
        .send()
        .await
        .expect("HTTP delivery retry should send");
    assert_eq!(http_retry.status(), StatusCode::CREATED);
    assert_eq!(
        http_retry
            .json::<Value>()
            .await
            .expect("HTTP delivery retry should be JSON"),
        accepted
    );
    let retry = server
        .tool("submit_action", action.clone(), Some(actor.id.0))
        .await;
    assert_eq!(structured(&retry), &accepted);

    let observer_entities = server
        .tool(
            "list_entity_at_current_place",
            json!({}),
            Some(observer.id.0),
        )
        .await;
    let observer_entities = structured(&observer_entities)["entity"]
        .as_array()
        .expect("current-Place Entities should be an array");
    assert!(observer_entities.iter().any(|entity| {
        entity["id"] == accepted["consequence"]["entity"]["id"]
            && entity["name"] == "Cedar Crossing Marker"
            && entity["description"]
                == "A waist-high cedar marker carved with three crossing lines."
    }));
    for entity in observer_entities {
        assert_eq!(
            entity
                .as_object()
                .expect("current-Place Entity should be an object")
                .keys()
                .cloned()
                .collect::<BTreeSet<_>>(),
            BTreeSet::from(["description".to_owned(), "id".to_owned(), "name".to_owned()])
        );
    }
    let observer_activity: Value = server
        .client
        .get(format!(
            "{}/api/place/current/activity?limit=1",
            server.base_url
        ))
        .header(USER_CONTEXT_HEADER, observer.id.0.to_string())
        .send()
        .await
        .expect("observer Activity read should send")
        .json()
        .await
        .expect("observer Activity page should be JSON");
    assert_eq!(observer_activity["activity"][0], accepted["activity"]);
    assert_ne!(
        observer_activity["place_revision"],
        entity_context["place_revision"]
    );
    let activity_cursor = observer_activity["next"]
        .as_str()
        .expect("Place Activity should have another page");
    let next_observer_activity = server
        .tool(
            "list_activity_at_current_place",
            json!({"cursor": activity_cursor, "limit": 1}),
            Some(observer.id.0),
        )
        .await;
    assert_eq!(
        structured(&next_observer_activity)["place_revision"],
        observer_activity["place_revision"]
    );
    assert_ne!(
        structured(&next_observer_activity)["activity"][0]["id"],
        observer_activity["activity"][0]["id"]
    );

    let changed = json!({
        "request_id": request_id,
        "expected_place_revision": entity_context["place_revision"],
        "prose": "Different content under an accepted request id.",
        "consequence": {
            "type": "introduce_entity",
            "name": "Conflicting Marker",
            "description": "This marker must not be created."
        }
    });
    let conflict_response = server
        .client
        .post(format!("{}/api/action", server.base_url))
        .header(USER_CONTEXT_HEADER, actor.id.0.to_string())
        .json(&changed)
        .send()
        .await
        .expect("conflicting retry should send");
    assert_eq!(conflict_response.status(), StatusCode::CONFLICT);
    let conflict_http: Value = conflict_response
        .json()
        .await
        .expect("conflict should be JSON");
    let conflict_mcp = server
        .tool("submit_action", changed, Some(actor.id.0))
        .await;
    assert_eq!(error_code(&conflict_http), "action_request_conflict");
    assert_eq!(mcp_error(&conflict_mcp), conflict_http);

    let stale = json!({
        "request_id": Uuid::new_v4(),
        "expected_place_revision": entity_context["place_revision"],
        "prose": "A stale action must not change the World.",
        "consequence": {
            "type": "introduce_entity",
            "name": "Stale Marker",
            "description": "This marker must not be created."
        }
    });
    let stale_response = server
        .client
        .post(format!("{}/api/action", server.base_url))
        .header(USER_CONTEXT_HEADER, actor.id.0.to_string())
        .json(&stale)
        .send()
        .await
        .expect("stale action should send");
    assert_eq!(stale_response.status(), StatusCode::PRECONDITION_FAILED);
    let stale_http: Value = stale_response
        .json()
        .await
        .expect("freshness error should be JSON");
    let mut stale_mcp_input = stale;
    stale_mcp_input["request_id"] = json!(Uuid::new_v4());
    let stale_mcp = server
        .tool("submit_action", stale_mcp_input, Some(actor.id.0))
        .await;
    assert_eq!(error_code(&stale_http), "place_revision_conflict");
    assert_eq!(mcp_error(&stale_mcp), stale_http);

    let latest_revision = observer_activity["place_revision"].clone();
    let second_action = json!({
        "request_id": Uuid::new_v4(),
        "expected_place_revision": latest_revision,
        "prose": "Tomas sets a second marker where travelers can compare the routes.",
        "consequence": {
            "type": "introduce_entity",
            "name": "Route Comparison Marker",
            "description": "A second cedar marker with two route notches."
        }
    });
    let second_accepted = server
        .tool("submit_action", second_action, Some(observer.id.0))
        .await;
    assert_eq!(
        structured(&second_accepted)["activity"]["operation"],
        "submit_action"
    );

    let first_entity_page: Value = server
        .client
        .get(format!(
            "{}/api/place/current/entity?limit=1",
            server.base_url
        ))
        .header(USER_CONTEXT_HEADER, actor.id.0.to_string())
        .send()
        .await
        .expect("first exact-Place Entity page should send")
        .json()
        .await
        .expect("first exact-Place Entity page should be JSON");
    let entity_cursor = first_entity_page["next"]
        .as_str()
        .expect("two placed Entities should produce a cursor");
    let second_entity_page = server
        .tool(
            "list_entity_at_current_place",
            json!({"cursor": entity_cursor, "limit": 1}),
            Some(actor.id.0),
        )
        .await;
    assert_eq!(
        structured(&second_entity_page)["place_revision"],
        first_entity_page["place_revision"]
    );
    assert_ne!(
        structured(&second_entity_page)["entity"][0]["id"],
        first_entity_page["entity"][0]["id"]
    );

    let global_entity_page: Value = server
        .client
        .get(format!("{}/api/entity?limit=1", server.base_url))
        .send()
        .await
        .expect("global Entity cursor source should send")
        .json()
        .await
        .expect("global Entity cursor source should be JSON");
    let global_entity_cursor = global_entity_page["next"]
        .as_str()
        .expect("global Entity state should have another page")
        .to_owned();
    let personal_activity_page: Value = server
        .client
        .get(format!("{}/api/activity?limit=1", server.base_url))
        .header(USER_CONTEXT_HEADER, actor.id.0.to_string())
        .send()
        .await
        .expect("personal Activity cursor source should send")
        .json()
        .await
        .expect("personal Activity cursor source should be JSON");
    let personal_activity_cursor = personal_activity_page["next"]
        .as_str()
        .expect("personal Activity state should have another page")
        .to_owned();
    let operation = [
        (
            "/api/activity",
            "list_activity",
            personal_activity_cursor,
            Some(actor.id.0),
        ),
        (
            "/api/place/current/entity",
            "list_entity_at_current_place",
            entity_cursor.to_owned(),
            Some(actor.id.0),
        ),
        (
            "/api/place/current/activity",
            "list_activity_at_current_place",
            activity_cursor.to_owned(),
            Some(actor.id.0),
        ),
    ];
    for (http_path, tool_name, _, user_id) in &operation {
        assert_cross_operation_cursor_rejected(
            &server,
            http_path,
            tool_name,
            &global_entity_cursor,
            *user_id,
        )
        .await;
    }
    for (source_index, (_, _, source_cursor, _)) in operation.iter().enumerate() {
        for (target_index, (http_path, tool_name, _, user_id)) in operation.iter().enumerate() {
            if source_index != target_index {
                assert_cross_operation_cursor_rejected(
                    &server,
                    http_path,
                    tool_name,
                    source_cursor,
                    *user_id,
                )
                .await;
            }
        }
    }
}
