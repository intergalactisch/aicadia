use super::*;

fn collect_property_schema<'a>(value: &'a Value, name: &str, found: &mut Vec<&'a Value>) {
    if let Some(property) = value.get("properties").and_then(Value::as_object)
        && let Some(schema) = property.get(name)
    {
        found.push(schema);
    }
    match value {
        Value::Array(item) => {
            for value in item {
                collect_property_schema(value, name, found);
            }
        }
        Value::Object(item) => {
            for value in item.values() {
                collect_property_schema(value, name, found);
            }
        }
        _ => {}
    }
}

fn assert_course_schema(schema: &Value) {
    let alternative = schema["oneOf"]
        .as_array()
        .expect("Connection course should publish exact alternatives");
    assert_eq!(alternative.len(), 2);
    assert!(alternative.iter().any(|schema| schema["maxItems"] == 0));
    assert!(
        alternative
            .iter()
            .any(|schema| schema["minItems"] == 2 && schema["maxItems"] == 128)
    );
}

fn assert_position_description_schemas(schema: &Value) {
    let mut found = Vec::new();
    collect_property_schema(schema, "position_description", &mut found);
    assert_eq!(found.len(), 2);
    assert!(
        found
            .iter()
            .all(|schema| schema["minLength"] == 1 && schema["maxLength"] == 4_000)
    );
}

fn assert_string_bounds(schema: &Value, min: u64, max: u64) {
    assert_eq!(schema["minLength"], min, "unexpected schema: {schema}");
    assert_eq!(schema["maxLength"], max, "unexpected schema: {schema}");
}

fn assert_nullable_string_bounds(schema: &Value, min: u64, max: u64) {
    assert_string_bounds(schema, min, max);
    let nullable = schema["nullable"] == true
        || schema["type"]
            .as_array()
            .is_some_and(|types| types.contains(&json!("null")));
    assert!(nullable, "schema should allow null: {schema}");
}

fn assert_described_fields(properties: &Value, fields: &[&str]) {
    for field in fields {
        assert!(
            properties[*field]["description"]
                .as_str()
                .is_some_and(|description| !description.is_empty()),
            "{field} should publish one concise meaning"
        );
    }
}

fn assert_position_roles(schema: &Value) {
    let expected = vec![json!("origin"), json!("result")];
    if let Some(values) = schema["enum"].as_array() {
        assert_eq!(values, &expected);
        return;
    }
    let values = schema["oneOf"]
        .as_array()
        .expect("Position role should publish its two variants")
        .iter()
        .map(|variant| variant["const"].clone())
        .collect::<Vec<_>>();
    assert_eq!(values, expected);
}

#[sqlx::test(migrations = "./migration")]
async fn player_reads_remain_truthful_before_character_onboarding(pool: PgPool) {
    let world = World::new(pool);
    let user = world.create_user().await.expect("setup User should exist");
    let server = TestServer::start(world).await;

    let world_response = server
        .client
        .get(format!("{}/api/world", server.base_url))
        .send()
        .await
        .expect("World request should send");
    assert_eq!(world_response.status(), StatusCode::OK);

    let absent_operator_read = server
        .client
        .get(format!("{}/api/entity?limit=100", server.base_url))
        .send()
        .await
        .expect("absent operator Entity request should send");
    assert_eq!(
        absent_operator_read.status(),
        StatusCode::METHOD_NOT_ALLOWED
    );

    let activity_response = server
        .client
        .get(format!("{}/api/activity?limit=100", server.base_url))
        .header(USER_CONTEXT_HEADER, user.id.0.to_string())
        .send()
        .await
        .expect("pre-Character Activity request should send");
    assert_eq!(activity_response.status(), StatusCode::NOT_FOUND);
    let activity_error: Value = activity_response
        .json()
        .await
        .expect("pre-Character Activity error should be JSON");
    assert_eq!(error_code(&activity_error), "character_not_found");
}

#[sqlx::test(migrations = "./migration")]
async fn catalog_exposes_exactly_the_nineteen_player_capabilities(pool: PgPool) {
    let server = TestServer::start(World::new(pool)).await;

    let openapi: Value = server
        .client
        .get(format!("{}/api/openapi.json", server.base_url))
        .send()
        .await
        .expect("OpenAPI request should send")
        .json()
        .await
        .expect("OpenAPI should be JSON");
    let operation_id = openapi["paths"]
        .as_object()
        .expect("OpenAPI should have paths")
        .values()
        .flat_map(|path| {
            path.as_object()
                .expect("OpenAPI path should be an object")
                .values()
        })
        .map(|operation| {
            operation["operationId"]
                .as_str()
                .expect("operation should have an id")
        })
        .collect::<BTreeSet<_>>();
    assert_eq!(operation_id, CAPABILITY.into_iter().collect());
    for unavailable_player_operation in [
        "list_entity",
        "get_entity",
        "count_entity",
        "count_character",
        "count_user",
        "count_building",
    ] {
        assert!(
            !operation_id.contains(unavailable_player_operation),
            "player OpenAPI must not publish global probe {unavailable_player_operation}"
        );
    }
    assert!(
        openapi["paths"]["/api/entity"].get("get").is_none(),
        "loopback operator Entity list must be absent from the player OpenAPI catalog"
    );
    assert!(
        openapi["paths"].get("/api/entity/{entity_id}").is_none(),
        "loopback operator Entity lookup must be absent from the player OpenAPI catalog"
    );
    assert_eq!(
        openapi["paths"]["/api/entity"]["post"]["responses"]["201"]["description"],
        "Created Entity"
    );
    assert_eq!(
        openapi["paths"]["/api/action"]["post"]["responses"]["201"]["description"],
        "Accepted action"
    );
    assert_eq!(
        openapi["paths"]["/api/action"]["post"]["responses"]["412"]["description"],
        "Exact current Place changed after it was read"
    );
    assert_eq!(
        openapi["paths"]["/api/interaction"]["post"]["responses"]["201"]["description"],
        "Accepted Interaction"
    );
    assert_eq!(
        openapi["paths"]["/api/place/current/entity/{entity_id}"]["get"]["responses"]["200"]["description"],
        "One exact-local Entity with one bounded current-state page"
    );
    for schema in [
        "SubmitActionInput",
        "AcceptedActionOutput",
        "SubmitInteractionInput",
        "AcceptedInteractionOutput",
        "CurrentPlaceEntityPageOutput",
        "CurrentPlaceActivityPageOutput",
        "CharacterEntityStatePageOutput",
        "CurrentPlaceEntityStatePageOutput",
        "StartInvestigationInput",
        "InvestigationResultOutput",
        "SubmitDiscoveryInput",
        "AcceptedDiscoveryOutput",
    ] {
        assert!(
            openapi["components"]["schemas"].get(schema).is_some(),
            "OpenAPI should publish shared schema {schema}"
        );
    }
    assert!(openapi.to_string().contains("ErrorDetail"));
    assert!(!openapi.to_string().contains("create_user"));

    let (status, discover) = server
        .mcp(
            "server/discover",
            None,
            json!({}),
            None,
            Some(&server.origin),
        )
        .await;
    assert_eq!(status, StatusCode::OK, "unexpected discover: {discover}");
    assert_eq!(
        discover["result"]["supportedVersions"],
        json!([PROTOCOL_VERSION])
    );
    assert_eq!(discover["result"]["capabilities"], json!({"tools": {}}));
    assert_eq!(discover["result"]["instructions"], *MCP_INSTRUCTIONS);

    let (status, listed) = server
        .mcp("tools/list", None, json!({}), None, Some(&server.origin))
        .await;
    assert_eq!(status, StatusCode::OK, "unexpected tools/list: {listed}");
    assert_eq!(listed["result"]["cacheScope"], "public");
    assert_eq!(listed["result"]["ttlMs"], 0);
    let tools = listed["result"]["tools"]
        .as_array()
        .expect("tools/list should return an array");
    let expected_tools: Value =
        serde_json::from_str(MCP_TOOL_CATALOG).expect("tool fixture should be valid JSON");
    assert_eq!(
        listed["result"]["tools"], expected_tools,
        "the checked-in catalog must equal the runtime catalog after central Agent descriptions are applied"
    );
    assert_eq!(tools.len(), 19);
    let tool_name = tools
        .iter()
        .map(|tool| tool["name"].as_str().expect("tool name should be text"))
        .collect::<BTreeSet<_>>();
    assert_eq!(tool_name, CAPABILITY.into_iter().collect());

    let discovery_tool = tools
        .iter()
        .find(|tool| tool["name"] == "submit_discovery")
        .expect("submit_discovery should publish its bounded schema");
    assert_course_schema(
        &discovery_tool["inputSchema"]["$defs"]["ConnectionInputWire"]["properties"]["course"],
    );
    assert_position_description_schemas(&discovery_tool["inputSchema"]);
    assert_course_schema(
        &openapi["components"]["schemas"]["ConnectionInputWire"]["properties"]["course"],
    );
    assert_position_description_schemas(&json!([
        openapi["components"]["schemas"]["DiscoveryOriginInputWire"],
        openapi["components"]["schemas"]["DiscoveryResultInputWire"]
    ]));

    let connection_tool = tools
        .iter()
        .find(|tool| tool["name"] == "get_connection")
        .expect("get_connection should publish its bounded schema");
    let place_tool = tools
        .iter()
        .find(|tool| tool["name"] == "list_place")
        .expect("list_place should publish its bounded schema");
    let list_connection_tool = tools
        .iter()
        .find(|tool| tool["name"] == "list_connection")
        .expect("list_connection should publish its bounded schema");
    assert_eq!(
        connection_tool["outputSchema"]["$defs"]["ConnectionPointOutput"]["properties"]["ordinal"]
            ["maximum"],
        127
    );
    assert_eq!(
        openapi["components"]["schemas"]["ConnectionPointOutput"]["properties"]["ordinal"]["maximum"],
        127
    );
    for (name, schema) in [
        (
            "MCP Position description",
            &connection_tool["outputSchema"]["$defs"]["PositionOutput"]["properties"]["description"],
        ),
        (
            "MCP Connection shape description",
            &connection_tool["outputSchema"]["properties"]["shape_description"],
        ),
        (
            "OpenAPI Position description",
            &openapi["components"]["schemas"]["PositionOutput"]["properties"]["description"],
        ),
        (
            "OpenAPI Connection shape description",
            &openapi["components"]["schemas"]["ConnectionOutput"]["properties"]["shape_description"],
        ),
    ] {
        assert!(!schema.is_null(), "{name} should exist");
        assert_nullable_string_bounds(schema, 1, 4_000);
    }
    assert_course_schema(&connection_tool["outputSchema"]["properties"]["course"]);
    assert_course_schema(
        &openapi["components"]["schemas"]["ConnectionOutput"]["properties"]["course"],
    );
    for schema in [
        &connection_tool["outputSchema"]["properties"],
        &connection_tool["outputSchema"]["$defs"]["PlacePositionOutput"]["properties"],
        &openapi["components"]["schemas"]["ConnectionOutput"]["properties"],
        &openapi["components"]["schemas"]["PlacePositionOutput"]["properties"],
    ] {
        assert_string_bounds(&schema["name"], 1, 120);
        assert_string_bounds(&schema["description"], 1, 4_000);
    }
    for schema in [
        &list_connection_tool["outputSchema"]["$defs"]["ConnectionSummaryOutput"]["properties"],
        &openapi["components"]["schemas"]["ConnectionSummaryOutput"]["properties"],
    ] {
        assert_string_bounds(&schema["name"], 1, 120);
        assert_string_bounds(&schema["description"], 1, 4_000);
    }
    for schema in [
        &connection_tool["outputSchema"]["$defs"]["PositionOutput"]["properties"]["position_revision"],
        &openapi["components"]["schemas"]["PositionOutput"]["properties"]["position_revision"],
    ] {
        assert_string_bounds(schema, 1, 256);
    }
    for schema in [
        &place_tool["outputSchema"]["properties"]["next"],
        &list_connection_tool["outputSchema"]["properties"]["next"],
        &openapi["components"]["schemas"]["PlacePageOutput"]["properties"]["next"],
        &openapi["components"]["schemas"]["ConnectionPageOutput"]["properties"]["next"],
    ] {
        assert_string_bounds(schema, 1, 512);
    }
    assert_eq!(
        place_tool["outputSchema"]["properties"]["place"]["maxItems"],
        100
    );
    assert_eq!(
        list_connection_tool["outputSchema"]["properties"]["connection"]["maxItems"],
        100
    );
    assert_eq!(
        openapi["components"]["schemas"]["PlacePageOutput"]["properties"]["place"]["maxItems"],
        100
    );
    assert_eq!(
        openapi["components"]["schemas"]["ConnectionPageOutput"]["properties"]["connection"]["maxItems"],
        100
    );
    let movement_tool = tools
        .iter()
        .find(|tool| tool["name"] == "move_character")
        .expect("move_character should publish its bounded schema");
    for name in ["origin_segment_ordinal", "target_segment_ordinal"] {
        let mut found = Vec::new();
        collect_property_schema(&movement_tool["inputSchema"], name, &mut found);
        assert_eq!(found.len(), 1);
        assert_eq!(found[0]["maximum"], 126);
        found.clear();
        collect_property_schema(
            &openapi["components"]["schemas"]["MovementTargetInput"],
            name,
            &mut found,
        );
        assert_eq!(found.len(), 1);
        assert_eq!(found[0]["maximum"], 126);
    }
    for schema in [
        &movement_tool["outputSchema"]["$defs"]["ActivityConnectionReferenceOutput"]["properties"]
            ["name"],
        &openapi["components"]["schemas"]["ActivityConnectionReferenceOutput"]["properties"]["name"],
    ] {
        assert_string_bounds(schema, 1, 120);
    }
    for schema in [
        &movement_tool["inputSchema"]["properties"]["expected_position_revision"],
        &openapi["components"]["schemas"]["MoveCharacterInput"]["properties"]["expected_position_revision"],
    ] {
        assert_string_bounds(schema, 1, 256);
    }
    let activity_position =
        &movement_tool["outputSchema"]["$defs"]["ActivityPositionReferenceOutput"]["properties"];
    let activity_connection =
        &movement_tool["outputSchema"]["$defs"]["ActivityConnectionReferenceOutput"]["properties"];
    let openapi_activity_position =
        &openapi["components"]["schemas"]["ActivityPositionReferenceOutput"]["properties"];
    let openapi_activity_connection =
        &openapi["components"]["schemas"]["ActivityConnectionReferenceOutput"]["properties"];
    for properties in [activity_position, openapi_activity_position] {
        assert_described_fields(properties, &["entity", "role", "position"]);
    }
    for properties in [activity_connection, openapi_activity_connection] {
        assert_described_fields(
            properties,
            &["id", "name", "source_place_id", "destination_place_id"],
        );
        for id in ["id", "source_place_id", "destination_place_id"] {
            assert_eq!(properties[id]["format"], "uuid");
        }
    }
    for properties in [
        &movement_tool["outputSchema"]["$defs"]["ActivityOutput"]["properties"],
        &openapi["components"]["schemas"]["ActivityOutput"]["properties"],
    ] {
        assert_eq!(properties["involved_position"]["maxItems"], 3);
        assert_eq!(properties["involved_connection"]["maxItems"], 1);
    }
    assert_position_roles(&movement_tool["outputSchema"]["$defs"]["ActivityPositionRoleOutput"]);
    assert_position_roles(&openapi["components"]["schemas"]["ActivityPositionRoleOutput"]);
    for removed in ["list_entity", "get_entity"] {
        assert!(!tool_name.contains(removed));
    }
    let interaction = tools
        .iter()
        .find(|tool| tool["name"] == "submit_interaction")
        .expect("submit_interaction should be in the fixed catalog");
    assert_eq!(interaction["annotations"]["readOnlyHint"], false);
    assert_eq!(interaction["annotations"]["idempotentHint"], true);
    assert_eq!(interaction["annotations"]["openWorldHint"], false);
    let property = tools
        .iter()
        .find(|tool| tool["name"] == "get_entity_at_current_place")
        .expect("get_entity_at_current_place should be in the fixed catalog");
    assert_eq!(property["annotations"]["readOnlyHint"], true);
    assert_eq!(property["annotations"]["destructiveHint"], false);
    assert_eq!(property["annotations"]["idempotentHint"], true);
    assert_eq!(property["annotations"]["openWorldHint"], false);
    for name in ["start_investigation", "submit_discovery", "move_character"] {
        let tool = tools
            .iter()
            .find(|tool| tool["name"] == name)
            .unwrap_or_else(|| panic!("{name} should be in the fixed catalog"));
        assert_eq!(tool["annotations"]["readOnlyHint"], false);
        assert_eq!(tool["annotations"]["destructiveHint"], false);
        assert_eq!(tool["annotations"]["idempotentHint"], true);
        assert_eq!(tool["annotations"]["openWorldHint"], false);
    }
    for forbidden_fragment in ["count_", "notify", "notification", "background_agent"] {
        assert!(
            tool_name
                .iter()
                .all(|name| !name.contains(forbidden_fragment)),
            "the player catalog must not expose {forbidden_fragment} capability"
        );
    }
    let (status, _) = server.mcp("tools/list", None, json!({}), None, None).await;
    assert_eq!(status, StatusCode::OK, "an absent Origin must be accepted");
    let foreign_origin = format!(
        "http://localhost:{}",
        server.origin.rsplit_once(':').unwrap().1
    );
    let (status, _) = server
        .mcp("tools/list", None, json!({}), None, Some(&foreign_origin))
        .await;
    assert_eq!(status, StatusCode::FORBIDDEN);
}

/// Rewrites the pinned catalog from the live runtime after an accepted contract
/// change. It is ignored on purpose: run it explicitly, review the resulting
/// `git diff game/mcp/tool-catalog.json`, then let the pin test above prove the
/// new publication. It uses `jq` (already required by the shell suites) so the
/// runtime key order is preserved exactly as the fixture stores it.
#[sqlx::test(migrations = "./migration")]
#[ignore = "rewrites game/mcp/tool-catalog.json; run explicitly after an accepted contract change"]
async fn regenerate_agent_tool_catalog_fixture(pool: PgPool) {
    let server = TestServer::start(World::new(pool)).await;
    let response = server
        .mcp_raw_response(
            json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": "tools/list",
                "params": {"_meta": request_meta()},
            }),
            &[
                ("MCP-Protocol-Version", PROTOCOL_VERSION),
                ("Mcp-Method", "tools/list"),
                ("Origin", &server.origin),
            ],
        )
        .await;
    assert_eq!(response.status(), StatusCode::OK);
    let raw = response
        .bytes()
        .await
        .expect("tools/list body should be readable");
    let mut jq = std::process::Command::new("jq")
        .arg(".result.tools")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("jq is required to regenerate the catalog fixture");
    std::io::Write::write_all(jq.stdin.as_mut().expect("jq stdin should be open"), &raw)
        .expect("tools/list bytes should reach jq");
    let output = jq.wait_with_output().expect("jq should finish");
    assert!(
        output.status.success(),
        "jq failed to extract .result.tools"
    );
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/mcp/tool-catalog.json");
    std::fs::write(path, output.stdout).expect("catalog fixture should be writable");
}

#[sqlx::test(migrations = "./migration")]
async fn current_mcp_remains_stateless_and_requires_per_request_metadata(pool: PgPool) {
    let server = TestServer::start(World::new(pool)).await;

    for (id, version) in [(1, PROTOCOL_VERSION), (2, "2025-11-25")] {
        let response = server
            .mcp_raw_response(
                json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "method": "initialize",
                    "params": {
                        "protocolVersion": version,
                        "capabilities": {},
                        "clientInfo": {
                            "name": "unsupported-initialize-test",
                            "version": "0.1.0"
                        }
                    }
                }),
                &[],
            )
            .await;
        assert!(
            response.headers().get("Mcp-Session-Id").is_none(),
            "initialize must not create a transport session"
        );
        let (status, body) = TestServer::response(response).await;
        assert_protocol_error(status, &body, StatusCode::OK, -32601);
    }

    let mut params = json!({});
    params
        .as_object_mut()
        .expect("MCP params should be an object")
        .insert("_meta".to_owned(), request_meta());

    let response = server
        .mcp_raw_response(
            json!({
                "jsonrpc": "2.0",
                "id": 3,
                "method": "tools/list",
                "params": params
            }),
            &[
                ("MCP-Protocol-Version", PROTOCOL_VERSION),
                ("Mcp-Method", "tools/list"),
            ],
        )
        .await;
    assert_eq!(response.status(), StatusCode::OK);
    assert!(
        response.headers().get("Mcp-Session-Id").is_none(),
        "MCP 2026 must not create a transport session"
    );
    assert_eq!(
        response
            .headers()
            .get("Content-Type")
            .and_then(|value| value.to_str().ok()),
        Some("application/json")
    );
    let listed: Value = response
        .json()
        .await
        .expect("stateless MCP response should be JSON");
    assert!(listed["result"]["tools"].is_array());

    let (status, missing_meta) = server
        .mcp_raw(
            json!({
                "jsonrpc": "2.0",
                "id": 4,
                "method": "tools/list",
                "params": {}
            }),
            &[
                ("MCP-Protocol-Version", PROTOCOL_VERSION),
                ("Mcp-Method", "tools/list"),
            ],
        )
        .await;
    assert_protocol_error(status, &missing_meta, StatusCode::BAD_REQUEST, -32602);

    let unsupported_version = "2025-11-25";
    let (status, unsupported) = server
        .mcp_raw(
            json!({
                "jsonrpc": "2.0",
                "id": 5,
                "method": "tools/list",
                "params": {
                    "_meta": {
                        "io.modelcontextprotocol/protocolVersion": unsupported_version,
                        "io.modelcontextprotocol/clientInfo": {
                            "name": "unsupported-version-test",
                            "version": "0.1.0"
                        },
                        "io.modelcontextprotocol/clientCapabilities": {}
                    }
                }
            }),
            &[
                ("MCP-Protocol-Version", unsupported_version),
                ("Mcp-Method", "tools/list"),
            ],
        )
        .await;
    assert_protocol_error(status, &unsupported, StatusCode::BAD_REQUEST, -32022);
}

#[sqlx::test(migrations = "./migration")]
async fn mcp_arguments_fail_closed_with_canonical_invalid_request_for_all_capabilities(
    pool: PgPool,
) {
    let world = World::new(pool);
    let user = world.create_user().await.expect("setup User should exist");
    let server = TestServer::start(world).await;

    for (name, contextual) in [
        ("get_world", false),
        ("get_user", true),
        ("get_character", true),
        ("create_character", true),
        ("create_entry_place", true),
        ("enter_world", true),
        ("list_activity", true),
        ("create_entity", true),
        ("list_entity_at_current_place", true),
        ("list_activity_at_current_place", true),
        ("get_entity_at_current_place", true),
        ("list_place", true),
        ("list_connection", true),
        ("get_connection", true),
        ("start_investigation", true),
        ("submit_action", true),
        ("submit_interaction", true),
        ("submit_discovery", true),
        ("move_character", true),
    ] {
        let response = server
            .tool(
                name,
                json!({"unexpected": true}),
                contextual.then_some(user.id.0),
            )
            .await;
        assert_eq!(
            response["result"]["isError"], true,
            "{name} should return a game error"
        );
        let error = mcp_error(&response);
        assert_eq!(
            error_code(&error),
            "invalid_request",
            "{name} should canonically reject malformed capability arguments"
        );
        assert!(
            response.get("error").is_none(),
            "{name} argument decoding must not escape as a JSON-RPC protocol error"
        );
    }

    let unknown_entity_body = json!({
        "name": "Must not exist",
        "description": "Unknown fields reject this body.",
        "unexpected": true
    });
    let http_response = server
        .client
        .post(format!("{}/api/entity", server.base_url))
        .header(USER_CONTEXT_HEADER, user.id.0.to_string())
        .json(&unknown_entity_body)
        .send()
        .await
        .expect("unknown HTTP Entity field should send");
    assert_eq!(http_response.status(), StatusCode::BAD_REQUEST);
    let http_error: Value = http_response
        .json()
        .await
        .expect("unknown HTTP Entity field should be JSON");
    let mcp_response = server
        .tool("create_entity", unknown_entity_body, Some(user.id.0))
        .await;
    assert_eq!(mcp_error(&mcp_response), http_error);

    let unknown_start = json!({
        "request_id": Uuid::new_v4(),
        "kind": "entity_at_position",
        "unexpected": true
    });
    let http_response = server
        .client
        .post(format!("{}/api/investigation", server.base_url))
        .header(USER_CONTEXT_HEADER, user.id.0.to_string())
        .json(&unknown_start)
        .send()
        .await
        .expect("unknown HTTP investigation field should send");
    assert_eq!(http_response.status(), StatusCode::BAD_REQUEST);
    let http_error: Value = http_response
        .json()
        .await
        .expect("unknown HTTP investigation field should be JSON");
    let mcp_response = server
        .tool("start_investigation", unknown_start, Some(user.id.0))
        .await;
    assert_eq!(error_code(&http_error), "invalid_request");
    assert_eq!(mcp_error(&mcp_response), http_error);

    let invalid_start_id = json!({"request_id": "not-a-uuid", "kind": "entity_at_position"});
    let http_response = server
        .client
        .post(format!("{}/api/investigation", server.base_url))
        .header(USER_CONTEXT_HEADER, user.id.0.to_string())
        .json(&invalid_start_id)
        .send()
        .await
        .expect("invalid HTTP investigation UUID should send");
    assert_eq!(http_response.status(), StatusCode::BAD_REQUEST);
    let http_error: Value = http_response
        .json()
        .await
        .expect("invalid HTTP investigation UUID should be JSON");
    let mcp_response = server
        .tool("start_investigation", invalid_start_id, Some(user.id.0))
        .await;
    assert_eq!(error_code(&http_error), "invalid_request");
    assert_eq!(mcp_error(&mcp_response), http_error);

    let unknown_discovery_find = json!({
        "request_id": Uuid::new_v4(),
        "attempt_id": Uuid::new_v4(),
        "prose": "This malformed discovery must not be accepted.",
        "result": {
            "type": "entity_at_position",
            "name": "Must not exist",
            "description": "An unknown nested field rejects the complete body.",
            "unexpected": true
        }
    });
    let http_response = server
        .client
        .post(format!("{}/api/discovery", server.base_url))
        .header(USER_CONTEXT_HEADER, user.id.0.to_string())
        .json(&unknown_discovery_find)
        .send()
        .await
        .expect("unknown nested HTTP discovery field should send");
    assert_eq!(http_response.status(), StatusCode::BAD_REQUEST);
    let http_error: Value = http_response
        .json()
        .await
        .expect("unknown nested HTTP discovery field should be JSON");
    let mcp_response = server
        .tool("submit_discovery", unknown_discovery_find, Some(user.id.0))
        .await;
    assert_eq!(error_code(&http_error), "invalid_request");
    assert_eq!(mcp_error(&mcp_response), http_error);

    let http_response = server
        .client
        .get(format!("{}/api/entity?unexpected=true", server.base_url))
        .send()
        .await
        .expect("absent global Entity list should send");
    assert_eq!(http_response.status(), StatusCode::METHOD_NOT_ALLOWED);

    let http_response = server
        .client
        .get(format!(
            "{}/api/place/current/entity/{}?unexpected=true",
            server.base_url,
            Uuid::new_v4()
        ))
        .header(USER_CONTEXT_HEADER, user.id.0.to_string())
        .send()
        .await
        .expect("unknown HTTP Property query field should send");
    assert_eq!(http_response.status(), StatusCode::BAD_REQUEST);
    let http_error: Value = http_response
        .json()
        .await
        .expect("unknown HTTP Property query field should be JSON");
    let mcp_response = server
        .tool(
            "get_entity_at_current_place",
            json!({"unexpected": true}),
            Some(user.id.0),
        )
        .await;
    assert_eq!(error_code(&http_error), "invalid_request");
    assert_eq!(mcp_error(&mcp_response), http_error);

    let malformed_action = json!({
        "request_id": Uuid::new_v4(),
        "expected_place_revision": "not-a-revision",
        "prose": "This action must not be decoded.",
        "consequence": {
            "type": "introduce_entity",
            "name": "Must not exist",
            "description": "An unknown nested field rejects the body.",
            "unexpected": true
        }
    });
    let http_response = server
        .client
        .post(format!("{}/api/action", server.base_url))
        .header(USER_CONTEXT_HEADER, user.id.0.to_string())
        .json(&malformed_action)
        .send()
        .await
        .expect("unknown nested HTTP action field should send");
    assert_eq!(http_response.status(), StatusCode::BAD_REQUEST);
    let http_error: Value = http_response
        .json()
        .await
        .expect("unknown nested HTTP action field should be JSON");
    let mcp_response = server
        .tool("submit_action", malformed_action, Some(user.id.0))
        .await;
    assert_eq!(mcp_error(&mcp_response), http_error);

    let http_response = server
        .client
        .get(format!("{}/api/entity/not-a-uuid", server.base_url))
        .send()
        .await
        .expect("absent global Entity detail should send");
    assert_eq!(http_response.status(), StatusCode::NOT_FOUND);
}

#[sqlx::test(migrations = "./migration")]
async fn invalid_mcp_framing_stays_outside_the_game_error_contract(pool: PgPool) {
    let server = TestServer::start(World::new(pool)).await;
    let call = |name: &str, with_meta: bool| {
        let mut params = json!({"name": name, "arguments": {}});
        if with_meta {
            params
                .as_object_mut()
                .expect("tool params should be an object")
                .insert("_meta".to_owned(), request_meta());
        }
        json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "tools/call",
            "params": params
        })
    };

    let (status, body) = server
        .mcp_raw(
            call("not_a_tool", true),
            &[
                ("MCP-Protocol-Version", PROTOCOL_VERSION),
                ("Mcp-Method", "tools/call"),
                ("Mcp-Name", "not_a_tool"),
            ],
        )
        .await;
    assert_protocol_error(status, &body, StatusCode::BAD_REQUEST, -32602);

    for unavailable_player_tool in [
        "list_entity",
        "get_entity",
        "count_entity",
        "count_character",
        "count_user",
        "count_building",
    ] {
        let (status, body) = server
            .mcp_raw(
                call(unavailable_player_tool, true),
                &[
                    ("MCP-Protocol-Version", PROTOCOL_VERSION),
                    ("Mcp-Method", "tools/call"),
                    ("Mcp-Name", unavailable_player_tool),
                ],
            )
            .await;
        assert_protocol_error(status, &body, StatusCode::BAD_REQUEST, -32602);
    }

    for (headers, body) in [
        (
            vec![
                ("MCP-Protocol-Version", PROTOCOL_VERSION),
                ("Mcp-Name", "get_world"),
            ],
            call("get_world", true),
        ),
        (
            vec![
                ("MCP-Protocol-Version", PROTOCOL_VERSION),
                ("Mcp-Method", "tools/list"),
                ("Mcp-Name", "get_world"),
            ],
            call("get_world", true),
        ),
        (
            vec![
                ("MCP-Protocol-Version", PROTOCOL_VERSION),
                ("Mcp-Method", "tools/call"),
            ],
            call("get_world", true),
        ),
        (
            vec![
                ("MCP-Protocol-Version", PROTOCOL_VERSION),
                ("Mcp-Method", "tools/call"),
                ("Mcp-Name", "get_user"),
            ],
            call("get_world", true),
        ),
    ] {
        let (status, body) = server.mcp_raw(body, &headers).await;
        assert_protocol_error(status, &body, StatusCode::BAD_REQUEST, -32020);
    }

    let (status, body) = server
        .mcp_raw(
            call("get_world", false),
            &[
                ("MCP-Protocol-Version", PROTOCOL_VERSION),
                ("Mcp-Method", "tools/call"),
                ("Mcp-Name", "get_world"),
            ],
        )
        .await;
    assert_protocol_error(status, &body, StatusCode::BAD_REQUEST, -32602);

    let (status, body) = server
        .mcp_raw(
            call("get_world", true),
            &[("Mcp-Method", "tools/call"), ("Mcp-Name", "get_world")],
        )
        .await;
    assert_protocol_error(status, &body, StatusCode::BAD_REQUEST, -32020);
}
