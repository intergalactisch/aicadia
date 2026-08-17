use sqlx::PgPool;

#[tokio::test]
#[ignore = "run after the SQLx suites to verify their disposable databases were removed"]
async fn audit_interest_lab_sqlx_database_cleanup() {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url)
        .await
        .expect("connect cleanup audit");
    let owned_test_paths = [
        "listener_failure_recovery::fatal_listener_loss_ends_mcp_stream_and_replacement_baseline_recovers",
        "mcp::tests::postgres_commit_notify_mcp_listen_and_read_form_one_real_chain",
        "real_strategy_matrix::all_five_strategies_use_the_same_real_postgres_gateway_mcp_chain",
        "world::tests::accepted_change_is_atomic_and_notifies_after_commit",
        "world::tests::failed_current_write_rolls_back_its_activity",
        "world::tests::explicit_world_scope_changes_only_its_scope_resource",
        "world::tests::post_commit_notification_starvation_is_bounded_and_lossy",
        "world::tests::child_baseline_reports_truncation_and_skips_leaf_query",
    ];
    let leaked: Vec<(String, String)> = sqlx::query_as(
        r#"
        SELECT db_name, test_path
        FROM _sqlx_test.databases
        WHERE test_path = ANY($1)
        ORDER BY test_path
        "#,
    )
    .bind(&owned_test_paths[..])
    .fetch_all(&pool)
    .await
    .expect("read SQLx disposable-database registry");
    pool.close().await;

    assert!(
        leaked.is_empty(),
        "the interest lab left registered SQLx test databases: {leaked:?}"
    );
}
