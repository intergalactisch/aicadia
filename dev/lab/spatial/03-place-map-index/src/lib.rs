//! Disposable PostgreSQL Place-map index falsification fixture.
//!
//! This crate exercises only its scratch projection and query. It is not production
//! Aicadia code and must never be imported by the runtime.

#[cfg(test)]
mod tests {
    use serde_json::Value;
    use sqlx::{PgPool, Row};

    const ROW_COUNT: i64 = 1_000_000;
    const PAGE_SIZE: i64 = 100;
    const MAX_BOUNDED_BUFFER_VISITS: i64 = 100;

    const FIRST_PAGE_EXPLAIN: &str = r#"
        EXPLAIN (ANALYZE, BUFFERS, FORMAT JSON)
        SELECT place_entity_id, position_activity_id, x_cm, y_cm, z_cm
        FROM place_map_index
        WHERE x_cm BETWEEN $1 AND $2
          AND y_cm BETWEEN $3 AND $4
          AND z_cm BETWEEN $5 AND $6
        ORDER BY x_cm, y_cm, z_cm, place_entity_id
        LIMIT 100
    "#;

    const CONTINUED_PAGE_EXPLAIN: &str = r#"
        EXPLAIN (ANALYZE, BUFFERS, FORMAT JSON)
        SELECT place_entity_id, position_activity_id, x_cm, y_cm, z_cm
        FROM place_map_index
        WHERE x_cm BETWEEN $1 AND $2
          AND y_cm BETWEEN $3 AND $4
          AND z_cm BETWEEN $5 AND $6
          AND (x_cm, y_cm, z_cm, place_entity_id) > ($7, $8, $9, $10::uuid)
        ORDER BY x_cm, y_cm, z_cm, place_entity_id
        LIMIT 100
    "#;

    #[derive(Clone, Debug)]
    struct Cursor {
        x_cm: i64,
        y_cm: i64,
        z_cm: i64,
        place_entity_id: String,
    }

    #[derive(Clone, Debug)]
    struct Window {
        min_x: i64,
        max_x: i64,
        min_y: i64,
        max_y: i64,
        min_z: i64,
        max_z: i64,
    }

    #[derive(Clone, Debug)]
    struct PlanObservation {
        label: &'static str,
        raw_text: Vec<String>,
        returned_rows: i64,
        scan_rows_visited: i64,
        index_rows_visited: i64,
        rows_removed_by_filter: i64,
        shared_hit_blocks: i64,
        shared_read_blocks: i64,
        execution_time_ms: f64,
        node_types: Vec<String>,
    }

    impl PlanObservation {
        fn has_node(&self, node_type: &str) -> bool {
            self.node_types
                .iter()
                .any(|candidate| candidate == node_type)
        }

        fn emit(&self) {
            eprintln!(
                "METRIC {} returned={} scan_rows_visited={} index_rows_visited={} rows_removed_by_filter={} shared_hit_blocks={} shared_read_blocks={} execution_time_ms={:.3} nodes={}",
                self.label,
                self.returned_rows,
                self.scan_rows_visited,
                self.index_rows_visited,
                self.rows_removed_by_filter,
                self.shared_hit_blocks,
                self.shared_read_blocks,
                self.execution_time_ms,
                self.node_types.join("|")
            );
            eprintln!(
                "RAW_PLAN_BEGIN {}\n{}\nRAW_PLAN_END {}",
                self.label,
                self.raw_text.join("\n"),
                self.label
            );
        }
    }

    fn number_i64(node: &Value, key: &str) -> i64 {
        node.get(key).and_then(Value::as_i64).unwrap_or(0)
    }

    fn collect_plan_metrics(
        node: &Value,
        node_types: &mut Vec<String>,
        scan_rows_visited: &mut i64,
        index_rows_visited: &mut i64,
        rows_removed_by_filter: &mut i64,
    ) {
        let loops = number_i64(node, "Actual Loops").max(1);
        let visited = (number_i64(node, "Actual Rows")
            + number_i64(node, "Rows Removed by Filter")
            + number_i64(node, "Rows Removed by Index Recheck"))
            * loops;
        if let Some(node_type) = node.get("Node Type").and_then(Value::as_str) {
            node_types.push(node_type.to_owned());
            if node_type.ends_with("Scan") {
                *scan_rows_visited += visited;
            }
            if node_type.contains("Index") {
                *index_rows_visited += visited;
            }
        }
        *rows_removed_by_filter += number_i64(node, "Rows Removed by Filter") * loops;
        if let Some(children) = node.get("Plans").and_then(Value::as_array) {
            for child in children {
                collect_plan_metrics(
                    child,
                    node_types,
                    scan_rows_visited,
                    index_rows_visited,
                    rows_removed_by_filter,
                );
            }
        }
    }

    fn observe(label: &'static str, raw: Value, raw_text: Vec<String>) -> PlanObservation {
        let root = &raw[0];
        let plan = &root["Plan"];
        let mut node_types = Vec::new();
        let mut scan_rows_visited = 0;
        let mut index_rows_visited = 0;
        let mut rows_removed_by_filter = 0;
        collect_plan_metrics(
            plan,
            &mut node_types,
            &mut scan_rows_visited,
            &mut index_rows_visited,
            &mut rows_removed_by_filter,
        );
        let returned_rows = number_i64(plan, "Actual Rows");
        let execution_time_ms = root["Execution Time"].as_f64().unwrap_or(0.0);
        let shared_hit_blocks = number_i64(plan, "Shared Hit Blocks");
        let shared_read_blocks = number_i64(plan, "Shared Read Blocks");
        PlanObservation {
            label,
            raw_text,
            returned_rows,
            scan_rows_visited,
            index_rows_visited,
            rows_removed_by_filter,
            shared_hit_blocks,
            shared_read_blocks,
            execution_time_ms,
            node_types,
        }
    }

    async fn row_count(pool: &PgPool) -> i64 {
        sqlx::query_scalar("SELECT count(*) FROM place_map_index")
            .fetch_one(pool)
            .await
            .unwrap()
    }

    async fn relation_blocks(pool: &PgPool, relation: &str) -> i64 {
        sqlx::query_scalar(
            "SELECT pg_relation_size($1::regclass) / current_setting('block_size')::bigint",
        )
        .bind(relation)
        .fetch_one(pool)
        .await
        .unwrap()
    }

    async fn load_dense(pool: &PgPool) {
        sqlx::query(
            r#"
            INSERT INTO place_map_index (
                place_entity_id, position_activity_id, x_cm, y_cm, z_cm
            )
            SELECT
                lpad(to_hex(series), 32, '0')::uuid,
                lpad(to_hex(series + 1000000), 32, '0')::uuid,
                0, 0, 0
            FROM generate_series(1, 1000000) AS series
            "#,
        )
        .execute(pool)
        .await
        .unwrap();
        sqlx::query("VACUUM (ANALYZE) place_map_index")
            .execute(pool)
            .await
            .unwrap();
        assert_eq!(row_count(pool).await, ROW_COUNT);
    }

    async fn load_adversarial(pool: &PgPool) {
        sqlx::query(
            r#"
            INSERT INTO place_map_index (
                place_entity_id, position_activity_id, x_cm, y_cm, z_cm
            )
            SELECT
                lpad(to_hex(series), 32, '0')::uuid,
                lpad(to_hex(series + 1000000), 32, '0')::uuid,
                series - 500001,
                CASE WHEN series > 999800 THEN 0 ELSE 1000000 END,
                0
            FROM generate_series(1, 1000000) AS series
            "#,
        )
        .execute(pool)
        .await
        .unwrap();
        sqlx::query("VACUUM (ANALYZE) place_map_index")
            .execute(pool)
            .await
            .unwrap();
        assert_eq!(row_count(pool).await, ROW_COUNT);
    }

    async fn load_rotated_adversarial(pool: &PgPool) {
        sqlx::query(
            r#"
            INSERT INTO place_map_index (
                place_entity_id, position_activity_id, x_cm, y_cm, z_cm
            )
            SELECT
                lpad(to_hex(series), 32, '0')::uuid,
                lpad(to_hex(series + 1000000), 32, '0')::uuid,
                series - 500001,
                series - 500001,
                CASE WHEN series > 999800 THEN 0 ELSE 1000000 END
            FROM generate_series(1, 1000000) AS series
            "#,
        )
        .execute(pool)
        .await
        .unwrap();
        sqlx::query("VACUUM (ANALYZE) place_map_index")
            .execute(pool)
            .await
            .unwrap();
        assert_eq!(row_count(pool).await, ROW_COUNT);
    }

    async fn explain_first(pool: &PgPool, label: &'static str, window: &Window) -> PlanObservation {
        let row = sqlx::query(FIRST_PAGE_EXPLAIN)
            .bind(window.min_x)
            .bind(window.max_x)
            .bind(window.min_y)
            .bind(window.max_y)
            .bind(window.min_z)
            .bind(window.max_z)
            .fetch_one(pool)
            .await
            .unwrap();
        let text_sql = FIRST_PAGE_EXPLAIN.replace("FORMAT JSON", "FORMAT TEXT");
        let raw_text = sqlx::query(&text_sql)
            .bind(window.min_x)
            .bind(window.max_x)
            .bind(window.min_y)
            .bind(window.max_y)
            .bind(window.min_z)
            .bind(window.max_z)
            .fetch_all(pool)
            .await
            .unwrap()
            .into_iter()
            .map(|row| row.try_get(0).unwrap())
            .collect();
        observe(label, row.try_get(0).unwrap(), raw_text)
    }

    async fn explain_continued(
        pool: &PgPool,
        label: &'static str,
        window: &Window,
        cursor: &Cursor,
    ) -> PlanObservation {
        let row = sqlx::query(CONTINUED_PAGE_EXPLAIN)
            .bind(window.min_x)
            .bind(window.max_x)
            .bind(window.min_y)
            .bind(window.max_y)
            .bind(window.min_z)
            .bind(window.max_z)
            .bind(cursor.x_cm)
            .bind(cursor.y_cm)
            .bind(cursor.z_cm)
            .bind(&cursor.place_entity_id)
            .fetch_one(pool)
            .await
            .unwrap();
        let text_sql = CONTINUED_PAGE_EXPLAIN.replace("FORMAT JSON", "FORMAT TEXT");
        let raw_text = sqlx::query(&text_sql)
            .bind(window.min_x)
            .bind(window.max_x)
            .bind(window.min_y)
            .bind(window.max_y)
            .bind(window.min_z)
            .bind(window.max_z)
            .bind(cursor.x_cm)
            .bind(cursor.y_cm)
            .bind(cursor.z_cm)
            .bind(&cursor.place_entity_id)
            .fetch_all(pool)
            .await
            .unwrap()
            .into_iter()
            .map(|row| row.try_get(0).unwrap())
            .collect();
        observe(label, row.try_get(0).unwrap(), raw_text)
    }

    async fn page_cursor(pool: &PgPool, window: &Window) -> Cursor {
        let row = sqlx::query(
            r#"
            SELECT x_cm, y_cm, z_cm, place_entity_id::text AS place_entity_id
            FROM place_map_index
            WHERE x_cm BETWEEN $1 AND $2
              AND y_cm BETWEEN $3 AND $4
              AND z_cm BETWEEN $5 AND $6
            ORDER BY x_cm, y_cm, z_cm, place_entity_id
            LIMIT 1 OFFSET 99
            "#,
        )
        .bind(window.min_x)
        .bind(window.max_x)
        .bind(window.min_y)
        .bind(window.max_y)
        .bind(window.min_z)
        .bind(window.max_z)
        .fetch_one(pool)
        .await
        .unwrap();
        Cursor {
            x_cm: row.try_get("x_cm").unwrap(),
            y_cm: row.try_get("y_cm").unwrap(),
            z_cm: row.try_get("z_cm").unwrap(),
            place_entity_id: row.try_get("place_entity_id").unwrap(),
        }
    }

    async fn add_smallest_alternative(pool: &PgPool) {
        sqlx::query(
            r#"
            CREATE INDEX place_map_index_yzx_entity_index
                ON place_map_index (y_cm, z_cm, x_cm, place_entity_id)
                INCLUDE (position_activity_id)
            "#,
        )
        .execute(pool)
        .await
        .unwrap();
        sqlx::query("ANALYZE place_map_index")
            .execute(pool)
            .await
            .unwrap();
    }

    async fn add_symmetric_third_index(pool: &PgPool) {
        sqlx::query(
            r#"
            CREATE INDEX place_map_index_zxy_entity_index
                ON place_map_index (z_cm, x_cm, y_cm, place_entity_id)
                INCLUDE (position_activity_id)
            "#,
        )
        .execute(pool)
        .await
        .unwrap();
        sqlx::query("ANALYZE place_map_index")
            .execute(pool)
            .await
            .unwrap();
    }

    fn assert_page_shape(observation: &PlanObservation) {
        assert!(
            !observation.has_node("Seq Scan"),
            "sequential scan: {observation:?}"
        );
        assert!(observation.returned_rows <= PAGE_SIZE);
    }

    #[sqlx::test(migrations = "./migration")]
    async fn dense_same_point_first_and_continued_pages_remain_bounded(pool: PgPool) {
        load_dense(&pool).await;
        let window = Window {
            min_x: -1,
            max_x: 1,
            min_y: -1,
            max_y: 1,
            min_z: -1,
            max_z: 1,
        };
        let cursor = page_cursor(&pool, &window).await;

        let candidate_first = explain_first(&pool, "dense_candidate_first", &window).await;
        let candidate_continued =
            explain_continued(&pool, "dense_candidate_continued", &window, &cursor).await;
        candidate_first.emit();
        candidate_continued.emit();
        assert_page_shape(&candidate_first);
        assert_page_shape(&candidate_continued);
        assert_eq!(candidate_first.returned_rows, PAGE_SIZE);
        assert_eq!(candidate_continued.returned_rows, PAGE_SIZE);
        assert!(candidate_first.index_rows_visited <= PAGE_SIZE);
        assert!(candidate_continued.index_rows_visited <= PAGE_SIZE);

        add_smallest_alternative(&pool).await;
        add_symmetric_third_index(&pool).await;
        let final_first = explain_first(&pool, "dense_three_index_first", &window).await;
        let final_continued =
            explain_continued(&pool, "dense_three_index_continued", &window, &cursor).await;
        final_first.emit();
        final_continued.emit();
        assert_page_shape(&final_first);
        assert_page_shape(&final_continued);
        assert_eq!(final_first.returned_rows, PAGE_SIZE);
        assert_eq!(final_continued.returned_rows, PAGE_SIZE);
        assert!(final_first.index_rows_visited <= PAGE_SIZE);
        assert!(final_continued.index_rows_visited <= PAGE_SIZE);
    }

    #[sqlx::test(migrations = "./migration")]
    async fn adversarial_cross_axis_refutes_one_and_bounds_final_three_indexes(pool: PgPool) {
        load_adversarial(&pool).await;
        let window = Window {
            min_x: -500000,
            max_x: 500000,
            min_y: 0,
            max_y: 0,
            min_z: 0,
            max_z: 0,
        };
        let cursor = page_cursor(&pool, &window).await;
        let candidate_index_blocks =
            relation_blocks(&pool, "place_map_index_xyz_entity_index").await;

        let candidate_first = explain_first(&pool, "adversarial_candidate_first", &window).await;
        let candidate_continued =
            explain_continued(&pool, "adversarial_candidate_continued", &window, &cursor).await;
        candidate_first.emit();
        candidate_continued.emit();
        assert_eq!(candidate_first.returned_rows, PAGE_SIZE);
        assert_eq!(candidate_continued.returned_rows, PAGE_SIZE);
        assert!(
            candidate_first.has_node("Seq Scan")
                || candidate_first.scan_rows_visited >= ROW_COUNT / 2
                || (candidate_first.shared_hit_blocks + candidate_first.shared_read_blocks)
                    >= candidate_index_blocks / 2,
            "the one-index candidate was not falsified: {candidate_first:?}"
        );

        add_smallest_alternative(&pool).await;
        let two_first = explain_first(&pool, "adversarial_two_index_first", &window).await;
        let two_continued =
            explain_continued(&pool, "adversarial_two_index_continued", &window, &cursor).await;
        two_first.emit();
        two_continued.emit();
        assert_page_shape(&two_first);
        assert_page_shape(&two_continued);
        assert_eq!(two_first.returned_rows, PAGE_SIZE);
        assert_eq!(two_continued.returned_rows, PAGE_SIZE);
        assert!(
            two_first.shared_hit_blocks + two_first.shared_read_blocks <= MAX_BOUNDED_BUFFER_VISITS
        );
        assert!(
            two_continued.shared_hit_blocks + two_continued.shared_read_blocks
                <= MAX_BOUNDED_BUFFER_VISITS
        );

        add_symmetric_third_index(&pool).await;
        let final_first = explain_first(&pool, "adversarial_three_index_first", &window).await;
        let final_continued =
            explain_continued(&pool, "adversarial_three_index_continued", &window, &cursor).await;
        final_first.emit();
        final_continued.emit();
        assert_page_shape(&final_first);
        assert_page_shape(&final_continued);
        assert_eq!(final_first.returned_rows, PAGE_SIZE);
        assert_eq!(final_continued.returned_rows, PAGE_SIZE);
        assert!(
            final_first.shared_hit_blocks + final_first.shared_read_blocks
                <= MAX_BOUNDED_BUFFER_VISITS
        );
        assert!(
            final_continued.shared_hit_blocks + final_continued.shared_read_blocks
                <= MAX_BOUNDED_BUFFER_VISITS
        );
    }

    #[sqlx::test(migrations = "./migration")]
    async fn rotated_adversarial_refutes_two_indexes_and_bounds_three_indexes(pool: PgPool) {
        load_rotated_adversarial(&pool).await;
        let window = Window {
            min_x: -500000,
            max_x: 500000,
            min_y: -500000,
            max_y: 500000,
            min_z: 0,
            max_z: 0,
        };
        let cursor = page_cursor(&pool, &window).await;

        let one_first = explain_first(&pool, "rotated_one_index_first", &window).await;
        let one_continued =
            explain_continued(&pool, "rotated_one_index_continued", &window, &cursor).await;
        one_first.emit();
        one_continued.emit();
        assert_eq!(one_first.returned_rows, PAGE_SIZE);
        assert_eq!(one_continued.returned_rows, PAGE_SIZE);
        assert!(
            one_first.has_node("Seq Scan")
                || one_first.scan_rows_visited >= ROW_COUNT / 2
                || one_first.shared_hit_blocks + one_first.shared_read_blocks > 1_000,
            "the one-index rotated candidate was not falsified: {one_first:?}"
        );

        add_smallest_alternative(&pool).await;
        let two_first = explain_first(&pool, "rotated_two_index_first", &window).await;
        let two_continued =
            explain_continued(&pool, "rotated_two_index_continued", &window, &cursor).await;
        two_first.emit();
        two_continued.emit();
        assert_eq!(two_first.returned_rows, PAGE_SIZE);
        assert_eq!(two_continued.returned_rows, PAGE_SIZE);
        assert!(
            two_first.has_node("Seq Scan")
                || two_first.scan_rows_visited >= ROW_COUNT / 2
                || two_first.shared_hit_blocks + two_first.shared_read_blocks > 1_000,
            "the two-index rotated candidate was not falsified: {two_first:?}"
        );

        add_symmetric_third_index(&pool).await;
        let three_first = explain_first(&pool, "rotated_three_index_first", &window).await;
        let three_continued =
            explain_continued(&pool, "rotated_three_index_continued", &window, &cursor).await;
        three_first.emit();
        three_continued.emit();
        assert_page_shape(&three_first);
        assert_page_shape(&three_continued);
        assert_eq!(three_first.returned_rows, PAGE_SIZE);
        assert_eq!(three_continued.returned_rows, PAGE_SIZE);
        assert!(
            three_first.shared_hit_blocks + three_first.shared_read_blocks
                <= MAX_BOUNDED_BUFFER_VISITS
        );
        assert!(
            three_continued.shared_hit_blocks + three_continued.shared_read_blocks
                <= MAX_BOUNDED_BUFFER_VISITS
        );
    }

    #[tokio::test]
    #[ignore = "run after the SQLx tests to verify their disposable databases were removed"]
    async fn audit_sqlx_database_cleanup() {
        let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = PgPool::connect(&url).await.unwrap();
        let leaked: i64 = sqlx::query_scalar(
            r#"
            SELECT count(*)
            FROM _sqlx_test.databases
            WHERE test_path LIKE '%postgres_place_map_index%'
            "#,
        )
        .fetch_one(&pool)
        .await
        .unwrap();
        pool.close().await;
        assert_eq!(leaked, 0, "the lab left registered SQLx test databases");
    }
}
