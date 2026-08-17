use sqlx::PgPool;

#[tokio::test]
#[ignore = "run after the SQLx suite to verify its disposable databases were removed"]
async fn audit_concurrent_entity_lab_sqlx_database_cleanup() {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url)
        .await
        .expect("connect cleanup audit");
    let owned_test_paths = [
        "semantic_matrix::accepted_retry_replays_and_changed_fingerprint_conflicts",
        "semantic_matrix::agent_declared_cross_fact_dependency_controls_sequential_validity",
        "semantic_matrix::canonical_multi_fact_locks_avoid_deadlock_and_revalidate_all_facts",
        "semantic_matrix::exact_fact_read_stays_on_primary_key_with_100k_unrelated_slots",
        "semantic_matrix::expected_absence_is_one_exact_coordinator",
        "semantic_matrix::independent_facts_on_one_tree_compose_without_entity_lock",
        "semantic_matrix::injected_failure_rolls_back_activity_slot_history_and_request",
        "semantic_matrix::one_multi_fact_request_commits_one_activity_and_two_histories",
        "semantic_matrix::quiet_entity_progresses_independently",
        "semantic_matrix::rejected_and_busy_absence_leave_no_slot_but_accepted_absence_does",
        "semantic_matrix::same_fact_has_one_accept_and_one_dependency_conflict",
        "semantic_matrix::stale_placement_rejects_without_request_or_history",
        "semantic_matrix::structurally_invalid_write_is_rejected_before_world_state",
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
        "the concurrent Entity lab left registered SQLx test databases: {leaked:?}"
    );
}
