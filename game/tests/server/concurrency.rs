use super::*;

#[sqlx::test(migrations = "./migration")]
async fn http_concurrency_preserves_single_genesis_and_retry_safe_entry(pool: PgPool) {
    let world = World::new(pool.clone());
    let first_user = world.create_user().await.unwrap();
    let second_user = world.create_user().await.unwrap();
    world
        .create_character(
            first_user.id,
            aicadia::CreateCharacter {
                name: "Mara Venn".to_owned(),
                description: "A careful surveyor.".to_owned(),
                property: Vec::new(),
                r#trait: Vec::new(),
            },
        )
        .await
        .unwrap();
    world
        .create_character(
            second_user.id,
            aicadia::CreateCharacter {
                name: "Tomas Reed".to_owned(),
                description: "A patient observer.".to_owned(),
                property: Vec::new(),
                r#trait: Vec::new(),
            },
        )
        .await
        .unwrap();
    let server = TestServer::start(world).await;
    let first_genesis = server
        .client
        .post(format!("{}/api/place/entry", server.base_url))
        .header(USER_CONTEXT_HEADER, first_user.id.0.to_string())
        .json(&json!({"name": "North Gate", "description": "First candidate."}))
        .send();
    let second_genesis = server
        .client
        .post(format!("{}/api/place/entry", server.base_url))
        .header(USER_CONTEXT_HEADER, second_user.id.0.to_string())
        .json(&json!({"name": "South Gate", "description": "Second candidate."}))
        .send();
    let (first_genesis, second_genesis) = tokio::join!(first_genesis, second_genesis);
    let status = [
        first_genesis.unwrap().status(),
        second_genesis.unwrap().status(),
    ];
    assert_eq!(
        status
            .iter()
            .filter(|status| **status == StatusCode::CREATED)
            .count(),
        1
    );
    assert_eq!(
        status
            .iter()
            .filter(|status| **status == StatusCode::CONFLICT)
            .count(),
        1
    );
    let place_count: i64 = sqlx::query_scalar("SELECT count(*) FROM place")
        .fetch_one(&pool)
        .await
        .unwrap();
    let entity_count: i64 = sqlx::query_scalar("SELECT count(*) FROM entity")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(place_count, 1);
    assert_eq!(entity_count, 3);

    let enter_url = format!("{}/api/world/entry", server.base_url);
    let first_entry = server
        .client
        .post(&enter_url)
        .header(USER_CONTEXT_HEADER, first_user.id.0.to_string())
        .send();
    let second_entry = server
        .client
        .post(&enter_url)
        .header(USER_CONTEXT_HEADER, first_user.id.0.to_string())
        .send();
    let (first_entry, second_entry) = tokio::join!(first_entry, second_entry);
    let first_entry = first_entry.unwrap();
    let second_entry = second_entry.unwrap();
    assert_eq!(first_entry.status(), StatusCode::OK);
    assert_eq!(second_entry.status(), StatusCode::OK);
    assert_eq!(
        first_entry.json::<Value>().await.unwrap(),
        second_entry.json::<Value>().await.unwrap()
    );
    let enter_count: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM activity WHERE operation = 'enter_world' AND requested_by_user_id = $1",
    )
    .bind(first_user.id.0)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(enter_count, 1);
}
