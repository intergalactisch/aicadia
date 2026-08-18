//! Comparative PostgreSQL coordination fixture for operation-scoped freshness.
//!
//! This crate is independent from production Aicadia. It holds dependency semantics
//! constant while comparing only the row scope used to coordinate validation and
//! mutation.

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;
    use std::sync::Arc;
    use std::time::Duration;

    use sqlx::{PgPool, Postgres, Transaction};
    use tokio::sync::Barrier;
    use tokio::time::timeout;
    use uuid::Uuid;

    const PLACE: Uuid = Uuid::from_u128(0x100);
    const STONE: Uuid = Uuid::from_u128(0x200);
    const DOOR: Uuid = Uuid::from_u128(0x201);
    const LOCK_TIMEOUT: &str = "150ms";
    const TEST_TIMEOUT: Duration = Duration::from_secs(2);

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    enum Strategy {
        Place,
        Entity,
        Hybrid,
        Slot,
    }

    impl Strategy {
        fn name(self) -> &'static str {
            match self {
                Self::Place => "place",
                Self::Entity => "entity",
                Self::Hybrid => "hybrid",
                Self::Slot => "slot",
            }
        }
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    enum Expectation {
        Absent,
        Current(Uuid),
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    struct Dependency {
        entity_id: Uuid,
        key: String,
        expectation: Expectation,
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    struct Write {
        entity_id: Uuid,
        key: String,
        value: String,
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    struct Request {
        id: Uuid,
        strategy: Strategy,
        dependencies: Vec<Dependency>,
        writes: Vec<Write>,
        operation: String,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    enum Outcome {
        Accepted,
        DependencyConflict,
    }

    fn current(entity_id: Uuid, key: &str, activity_id: Uuid) -> Dependency {
        Dependency {
            entity_id,
            key: key.to_owned(),
            expectation: Expectation::Current(activity_id),
        }
    }

    fn absent(entity_id: Uuid, key: &str) -> Dependency {
        Dependency {
            entity_id,
            key: key.to_owned(),
            expectation: Expectation::Absent,
        }
    }

    fn write(entity_id: Uuid, key: &str, value: &str) -> Write {
        Write {
            entity_id,
            key: key.to_owned(),
            value: value.to_owned(),
        }
    }

    async fn set_lock_timeout(
        transaction: &mut Transaction<'_, Postgres>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(&format!("SET LOCAL lock_timeout = '{LOCK_TIMEOUT}'"))
            .execute(&mut **transaction)
            .await?;
        Ok(())
    }

    fn canonicalize(request: &mut Request) {
        request.dependencies.sort_unstable_by(|left, right| {
            left.entity_id
                .as_bytes()
                .cmp(right.entity_id.as_bytes())
                .then_with(|| left.key.cmp(&right.key))
        });
        request.writes.sort_unstable_by(|left, right| {
            left.entity_id
                .as_bytes()
                .cmp(right.entity_id.as_bytes())
                .then_with(|| left.key.cmp(&right.key))
        });
    }

    async fn lock_place_dependencies(
        transaction: &mut Transaction<'_, Postgres>,
        dependencies: &[Dependency],
    ) -> Result<(), sqlx::Error> {
        let entity_id = dependencies
            .iter()
            .map(|dependency| dependency.entity_id)
            .collect::<Vec<_>>();
        let mut place_id = sqlx::query_scalar::<_, Uuid>(
            "SELECT place_id FROM entity WHERE id = ANY($1::uuid[]) ORDER BY place_id",
        )
        .bind(&entity_id)
        .fetch_all(&mut **transaction)
        .await?;
        place_id.dedup();
        sqlx::query_scalar::<_, Uuid>(
            "SELECT id FROM place WHERE id = ANY($1::uuid[]) ORDER BY id FOR UPDATE",
        )
        .bind(&place_id)
        .fetch_all(&mut **transaction)
        .await?;
        Ok(())
    }

    async fn lock_entities(
        transaction: &mut Transaction<'_, Postgres>,
        entity_id: &[Uuid],
    ) -> Result<(), sqlx::Error> {
        sqlx::query_scalar::<_, Uuid>(
            "SELECT id FROM entity WHERE id = ANY($1::uuid[]) ORDER BY id FOR UPDATE",
        )
        .bind(entity_id)
        .fetch_all(&mut **transaction)
        .await?;
        Ok(())
    }

    async fn lock_entity_dependencies(
        transaction: &mut Transaction<'_, Postgres>,
        dependencies: &[Dependency],
    ) -> Result<(), sqlx::Error> {
        let entity_id = dependencies
            .iter()
            .map(|dependency| dependency.entity_id)
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        lock_entities(transaction, &entity_id).await
    }

    async fn lock_hybrid_dependencies(
        transaction: &mut Transaction<'_, Postgres>,
        dependencies: &[Dependency],
    ) -> Result<(), sqlx::Error> {
        let absent_entity_id = dependencies
            .iter()
            .filter(|dependency| dependency.expectation == Expectation::Absent)
            .map(|dependency| dependency.entity_id)
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        lock_entities(transaction, &absent_entity_id).await?;

        let present = dependencies
            .iter()
            .filter(|dependency| matches!(dependency.expectation, Expectation::Current(_)))
            .collect::<Vec<_>>();
        let entity_id = present
            .iter()
            .map(|dependency| dependency.entity_id)
            .collect::<Vec<_>>();
        let key = present
            .iter()
            .map(|dependency| dependency.key.as_str())
            .collect::<Vec<_>>();
        sqlx::query(
            r#"
            SELECT entity_id, key
            FROM property_current
            WHERE (entity_id, key) IN (
                SELECT submitted.entity_id, submitted.key
                FROM UNNEST($1::uuid[], $2::text[]) AS submitted(entity_id, key)
            )
            ORDER BY entity_id, key
            FOR UPDATE
            "#,
        )
        .bind(&entity_id)
        .bind(&key)
        .fetch_all(&mut **transaction)
        .await?;
        Ok(())
    }

    async fn lock_slot_dependencies(
        transaction: &mut Transaction<'_, Postgres>,
        dependencies: &[Dependency],
    ) -> Result<(), sqlx::Error> {
        let entity_id = dependencies
            .iter()
            .map(|dependency| dependency.entity_id)
            .collect::<Vec<_>>();
        let key = dependencies
            .iter()
            .map(|dependency| dependency.key.as_str())
            .collect::<Vec<_>>();
        sqlx::query(
            r#"
            INSERT INTO property_slot (entity_id, key, current_activity_id)
            SELECT submitted.entity_id, submitted.key, NULL
            FROM UNNEST($1::uuid[], $2::text[]) AS submitted(entity_id, key)
            ORDER BY submitted.entity_id, submitted.key
            ON CONFLICT (entity_id, key) DO NOTHING
            "#,
        )
        .bind(&entity_id)
        .bind(&key)
        .execute(&mut **transaction)
        .await?;
        sqlx::query(
            r#"
            SELECT entity_id, key
            FROM property_slot
            WHERE (entity_id, key) IN (
                SELECT submitted.entity_id, submitted.key
                FROM UNNEST($1::uuid[], $2::text[]) AS submitted(entity_id, key)
            )
            ORDER BY entity_id, key
            FOR UPDATE
            "#,
        )
        .bind(&entity_id)
        .bind(&key)
        .fetch_all(&mut **transaction)
        .await?;
        Ok(())
    }

    async fn find_current(
        transaction: &mut Transaction<'_, Postgres>,
        strategy: Strategy,
        entity_id: Uuid,
        key: &str,
    ) -> Result<Option<Uuid>, sqlx::Error> {
        let table = if strategy == Strategy::Slot {
            "property_slot"
        } else {
            "property_current"
        };
        sqlx::query_scalar(&format!(
            "SELECT current_activity_id FROM {table} WHERE entity_id = $1 AND key = $2"
        ))
        .bind(entity_id)
        .bind(key)
        .fetch_optional(&mut **transaction)
        .await
        .map(Option::flatten)
    }

    async fn submit_with_gate(
        pool: &PgPool,
        mut request: Request,
        after_validation: Option<Arc<Barrier>>,
    ) -> Result<Outcome, sqlx::Error> {
        canonicalize(&mut request);
        let mut transaction = pool.begin().await?;
        set_lock_timeout(&mut transaction).await?;

        match request.strategy {
            Strategy::Place => {
                lock_place_dependencies(&mut transaction, &request.dependencies).await?
            }
            Strategy::Entity => {
                lock_entity_dependencies(&mut transaction, &request.dependencies).await?
            }
            Strategy::Hybrid => {
                lock_hybrid_dependencies(&mut transaction, &request.dependencies).await?
            }
            Strategy::Slot => {
                lock_slot_dependencies(&mut transaction, &request.dependencies).await?
            }
        }

        for dependency in &request.dependencies {
            let found = find_current(
                &mut transaction,
                request.strategy,
                dependency.entity_id,
                &dependency.key,
            )
            .await?;
            let matches = match dependency.expectation {
                Expectation::Absent => found.is_none(),
                Expectation::Current(expected) => found == Some(expected),
            };
            if !matches {
                transaction.rollback().await?;
                return Ok(Outcome::DependencyConflict);
            }
        }

        if let Some(barrier) = after_validation {
            barrier.wait().await;
        }

        sqlx::query("INSERT INTO activity (id, strategy, operation) VALUES ($1, $2, $3)")
            .bind(request.id)
            .bind(request.strategy.name())
            .bind(&request.operation)
            .execute(&mut *transaction)
            .await?;

        for write in &request.writes {
            let previous = find_current(
                &mut transaction,
                request.strategy,
                write.entity_id,
                &write.key,
            )
            .await?;
            sqlx::query(
                r#"
                INSERT INTO property_history (
                    entity_id, key, activity_id, previous_activity_id, value
                ) VALUES ($1, $2, $3, $4, $5)
                "#,
            )
            .bind(write.entity_id)
            .bind(&write.key)
            .bind(request.id)
            .bind(previous)
            .bind(&write.value)
            .execute(&mut *transaction)
            .await?;
            sqlx::query(
                r#"
                INSERT INTO property_current (entity_id, key, current_activity_id)
                VALUES ($1, $2, $3)
                ON CONFLICT (entity_id, key) DO UPDATE
                SET current_activity_id = EXCLUDED.current_activity_id
                "#,
            )
            .bind(write.entity_id)
            .bind(&write.key)
            .bind(request.id)
            .execute(&mut *transaction)
            .await?;
            if request.strategy == Strategy::Slot {
                sqlx::query(
                    r#"
                    UPDATE property_slot
                    SET current_activity_id = $3
                    WHERE entity_id = $1 AND key = $2
                    "#,
                )
                .bind(write.entity_id)
                .bind(&write.key)
                .bind(request.id)
                .execute(&mut *transaction)
                .await?;
            }
        }

        transaction.commit().await?;
        Ok(Outcome::Accepted)
    }

    async fn submit(pool: &PgPool, request: Request) -> Result<Outcome, sqlx::Error> {
        submit_with_gate(pool, request, None).await
    }

    async fn current_id(pool: &PgPool, entity_id: Uuid, key: &str) -> Uuid {
        sqlx::query_scalar(
            "SELECT current_activity_id FROM property_current WHERE entity_id = $1 AND key = $2",
        )
        .bind(entity_id)
        .bind(key)
        .fetch_one(pool)
        .await
        .unwrap()
    }

    async fn slot_current_id(pool: &PgPool, entity_id: Uuid, key: &str) -> Uuid {
        sqlx::query_scalar(
            "SELECT current_activity_id FROM property_slot WHERE entity_id = $1 AND key = $2",
        )
        .bind(entity_id)
        .bind(key)
        .fetch_one(pool)
        .await
        .unwrap()
    }

    async fn run_pair(
        pool: &PgPool,
        first: Request,
        second: Request,
    ) -> [Result<Outcome, sqlx::Error>; 2] {
        let barrier = Arc::new(Barrier::new(3));
        let first_task = {
            let pool = pool.clone();
            let barrier = barrier.clone();
            tokio::spawn(async move {
                barrier.wait().await;
                submit(&pool, first).await
            })
        };
        let second_task = {
            let pool = pool.clone();
            let barrier = barrier.clone();
            tokio::spawn(async move {
                barrier.wait().await;
                submit(&pool, second).await
            })
        };
        barrier.wait().await;
        timeout(TEST_TIMEOUT, async {
            [first_task.await.unwrap(), second_task.await.unwrap()]
        })
        .await
        .expect("the controlled contender pair must remain bounded")
    }

    async fn run_pair_after_validation(
        pool: &PgPool,
        first: Request,
        second: Request,
    ) -> [Result<Outcome, sqlx::Error>; 2] {
        let barrier = Arc::new(Barrier::new(3));
        let first_task = {
            let pool = pool.clone();
            let barrier = barrier.clone();
            tokio::spawn(async move { submit_with_gate(&pool, first, Some(barrier)).await })
        };
        let second_task = {
            let pool = pool.clone();
            let barrier = barrier.clone();
            tokio::spawn(async move { submit_with_gate(&pool, second, Some(barrier)).await })
        };
        barrier.wait().await;
        timeout(TEST_TIMEOUT, async {
            [first_task.await.unwrap(), second_task.await.unwrap()]
        })
        .await
        .expect("the post-validation contender pair must remain bounded")
    }

    async fn independent_mixed_requests(pool: &PgPool, strategy: Strategy) -> [Request; 2] {
        let stone_state = current_id(pool, STONE, "state").await;
        let door_state = current_id(pool, DOOR, "state").await;
        [
            Request {
                id: Uuid::new_v4(),
                strategy,
                dependencies: vec![
                    current(STONE, "state", stone_state),
                    absent(DOOR, "first_mixed_mark"),
                ],
                writes: vec![write(STONE, "state", "first-independent-change")],
                operation: "mixed-current-stone-absent-door".to_owned(),
            },
            Request {
                id: Uuid::new_v4(),
                strategy,
                dependencies: vec![
                    absent(STONE, "second_mixed_mark"),
                    current(DOOR, "state", door_state),
                ],
                writes: vec![write(DOOR, "state", "second-independent-change")],
                operation: "mixed-absent-stone-current-door".to_owned(),
            },
        ]
    }

    fn assert_one_accept_one_conflict(result: [Result<Outcome, sqlx::Error>; 2]) {
        let result = result.map(Result::unwrap);
        assert_eq!(
            result
                .iter()
                .filter(|outcome| **outcome == Outcome::Accepted)
                .count(),
            1
        );
        assert_eq!(
            result
                .iter()
                .filter(|outcome| **outcome == Outcome::DependencyConflict)
                .count(),
            1
        );
    }

    async fn exercise_correctness(pool: PgPool, strategy: Strategy) {
        let before_color = current_id(&pool, STONE, "color").await;
        let current_ids = [Uuid::new_v4(), Uuid::new_v4()];
        let current_result = run_pair(
            &pool,
            Request {
                id: current_ids[0],
                strategy,
                dependencies: vec![current(STONE, "color", before_color)],
                writes: vec![write(STONE, "color", "black")],
                operation: "same-current-a".to_owned(),
            },
            Request {
                id: current_ids[1],
                strategy,
                dependencies: vec![current(STONE, "color", before_color)],
                writes: vec![write(STONE, "color", "white")],
                operation: "same-current-b".to_owned(),
            },
        )
        .await;
        assert_one_accept_one_conflict(current_result);

        let absent_key = format!("mark_{}", strategy.name());
        let absent_ids = [Uuid::new_v4(), Uuid::new_v4()];
        let absent_result = run_pair(
            &pool,
            Request {
                id: absent_ids[0],
                strategy,
                dependencies: vec![absent(STONE, &absent_key)],
                writes: vec![write(STONE, &absent_key, "circle")],
                operation: "same-absence-a".to_owned(),
            },
            Request {
                id: absent_ids[1],
                strategy,
                dependencies: vec![absent(STONE, &absent_key)],
                writes: vec![write(STONE, &absent_key, "cross")],
                operation: "same-absence-b".to_owned(),
            },
        )
        .await;
        assert_one_accept_one_conflict(absent_result);

        let stone_state = current_id(&pool, STONE, "state").await;
        let door_state = current_id(&pool, DOOR, "state").await;
        let causal_ids = [Uuid::new_v4(), Uuid::new_v4()];
        let causal_dependencies = vec![
            current(STONE, "state", stone_state),
            current(DOOR, "state", door_state),
        ];
        let causal_result = run_pair(
            &pool,
            Request {
                id: causal_ids[0],
                strategy,
                dependencies: causal_dependencies.clone(),
                writes: vec![write(STONE, "state", "through-door")],
                operation: "roll-through-open-door".to_owned(),
            },
            Request {
                id: causal_ids[1],
                strategy,
                dependencies: causal_dependencies,
                writes: vec![write(DOOR, "state", "closed")],
                operation: "close-clear-door".to_owned(),
            },
        )
        .await;
        assert_one_accept_one_conflict(causal_result);

        for request_ids in [current_ids, absent_ids, causal_ids] {
            let activity_count: i64 =
                sqlx::query_scalar("SELECT count(*) FROM activity WHERE id = ANY($1::uuid[])")
                    .bind(request_ids)
                    .fetch_one(&pool)
                    .await
                    .unwrap();
            assert_eq!(activity_count, 1);
        }
    }

    #[sqlx::test(migrations = "./migration")]
    async fn place_strategy_preserves_operation_scoped_correctness(pool: PgPool) {
        exercise_correctness(pool, Strategy::Place).await;
    }

    #[sqlx::test(migrations = "./migration")]
    async fn entity_strategy_preserves_operation_scoped_correctness(pool: PgPool) {
        exercise_correctness(pool, Strategy::Entity).await;
    }

    #[sqlx::test(migrations = "./migration")]
    async fn hybrid_strategy_preserves_operation_scoped_correctness(pool: PgPool) {
        exercise_correctness(pool, Strategy::Hybrid).await;
    }

    #[sqlx::test(migrations = "./migration")]
    async fn slot_strategy_preserves_operation_scoped_correctness(pool: PgPool) {
        exercise_correctness(pool, Strategy::Slot).await;
    }

    #[sqlx::test(migrations = "./migration")]
    async fn hybrid_mixed_lock_classes_abort_semantically_independent_work(pool: PgPool) {
        let [first, second] = independent_mixed_requests(&pool, Strategy::Hybrid).await;
        let request_ids = [first.id, second.id];
        let result = run_pair_after_validation(&pool, first, second).await;
        let accepted = result
            .iter()
            .filter(|result| matches!(result, Ok(Outcome::Accepted)))
            .count();
        assert!(accepted < 2, "hybrid unexpectedly accepted both requests");
        let error_count = result
            .iter()
            .filter_map(|result| result.as_ref().err())
            .inspect(|error| {
                assert!(matches!(
                    database_code(error).as_deref(),
                    Some("55P03" | "40P01")
                ));
            })
            .count();
        assert!(error_count >= 1);
        let activity_count: i64 =
            sqlx::query_scalar("SELECT count(*) FROM activity WHERE id = ANY($1::uuid[])")
                .bind(request_ids)
                .fetch_one(&pool)
                .await
                .unwrap();
        assert_eq!(activity_count as usize, accepted);
    }

    #[sqlx::test(migrations = "./migration")]
    async fn slot_total_order_accepts_the_same_independent_mixed_work(pool: PgPool) {
        let [first, second] = independent_mixed_requests(&pool, Strategy::Slot).await;
        let request_ids = [first.id, second.id];
        let result = run_pair_after_validation(&pool, first, second).await;
        assert!(
            result
                .iter()
                .all(|result| matches!(result, Ok(Outcome::Accepted)))
        );
        let activity_count: i64 =
            sqlx::query_scalar("SELECT count(*) FROM activity WHERE id = ANY($1::uuid[])")
                .bind(request_ids)
                .fetch_one(&pool)
                .await
                .unwrap();
        assert_eq!(activity_count, 2);
    }

    fn database_code(error: &sqlx::Error) -> Option<String> {
        error
            .as_database_error()
            .and_then(|error| error.code())
            .map(|code| code.into_owned())
    }

    #[sqlx::test(migrations = "./migration")]
    async fn held_coordinator_reveals_each_strategy_contention_radius(pool: PgPool) {
        let mut place_blocker = pool.begin().await.unwrap();
        sqlx::query("SELECT id FROM place WHERE id = $1 FOR UPDATE")
            .bind(PLACE)
            .fetch_one(&mut *place_blocker)
            .await
            .unwrap();
        let door_state = current_id(&pool, DOOR, "state").await;
        let place_request_id = Uuid::new_v4();
        let place_error = submit(
            &pool,
            Request {
                id: place_request_id,
                strategy: Strategy::Place,
                dependencies: vec![current(DOOR, "state", door_state)],
                writes: vec![write(DOOR, "state", "latched")],
                operation: "place-radius".to_owned(),
            },
        )
        .await
        .unwrap_err();
        assert_eq!(database_code(&place_error).as_deref(), Some("55P03"));
        place_blocker.rollback().await.unwrap();

        let mut entity_blocker = pool.begin().await.unwrap();
        sqlx::query("SELECT id FROM entity WHERE id = $1 FOR UPDATE")
            .bind(STONE)
            .fetch_one(&mut *entity_blocker)
            .await
            .unwrap();
        let stone_color = current_id(&pool, STONE, "color").await;
        let entity_stone_id = Uuid::new_v4();
        let entity_stone_error = submit(
            &pool,
            Request {
                id: entity_stone_id,
                strategy: Strategy::Entity,
                dependencies: vec![current(STONE, "color", stone_color)],
                writes: vec![write(STONE, "color", "red")],
                operation: "entity-radius-stone".to_owned(),
            },
        )
        .await
        .unwrap_err();
        assert_eq!(database_code(&entity_stone_error).as_deref(), Some("55P03"));
        let entity_door_id = Uuid::new_v4();
        let entity_door = submit(
            &pool,
            Request {
                id: entity_door_id,
                strategy: Strategy::Entity,
                dependencies: vec![absent(DOOR, "mark_entity_radius")],
                writes: vec![write(DOOR, "mark_entity_radius", "clear")],
                operation: "entity-radius-door".to_owned(),
            },
        )
        .await
        .unwrap();
        assert_eq!(entity_door, Outcome::Accepted);
        entity_blocker.rollback().await.unwrap();

        let mut hybrid_blocker = pool.begin().await.unwrap();
        sqlx::query(
            "SELECT entity_id FROM property_current WHERE entity_id = $1 AND key = 'state' FOR UPDATE",
        )
        .bind(STONE)
        .fetch_one(&mut *hybrid_blocker)
        .await
        .unwrap();
        let stone_state = current_id(&pool, STONE, "state").await;
        let hybrid_state_id = Uuid::new_v4();
        let hybrid_state_error = submit(
            &pool,
            Request {
                id: hybrid_state_id,
                strategy: Strategy::Hybrid,
                dependencies: vec![current(STONE, "state", stone_state)],
                writes: vec![write(STONE, "state", "fallen")],
                operation: "hybrid-radius-state".to_owned(),
            },
        )
        .await
        .unwrap_err();
        assert_eq!(database_code(&hybrid_state_error).as_deref(), Some("55P03"));
        let hybrid_color_id = Uuid::new_v4();
        let hybrid_color = submit(
            &pool,
            Request {
                id: hybrid_color_id,
                strategy: Strategy::Hybrid,
                dependencies: vec![current(STONE, "color", stone_color)],
                writes: vec![write(STONE, "color", "blue")],
                operation: "hybrid-radius-color".to_owned(),
            },
        )
        .await
        .unwrap();
        assert_eq!(hybrid_color, Outcome::Accepted);
        let hybrid_door_id = Uuid::new_v4();
        let current_door_state = current_id(&pool, DOOR, "state").await;
        let hybrid_door = submit(
            &pool,
            Request {
                id: hybrid_door_id,
                strategy: Strategy::Hybrid,
                dependencies: vec![current(DOOR, "state", current_door_state)],
                writes: vec![write(DOOR, "state", "barred")],
                operation: "hybrid-radius-door".to_owned(),
            },
        )
        .await
        .unwrap();
        assert_eq!(hybrid_door, Outcome::Accepted);
        hybrid_blocker.rollback().await.unwrap();

        let mut hybrid_absence_blocker = pool.begin().await.unwrap();
        sqlx::query("SELECT id FROM entity WHERE id = $1 FOR UPDATE")
            .bind(STONE)
            .fetch_one(&mut *hybrid_absence_blocker)
            .await
            .unwrap();
        let latest_stone_color = current_id(&pool, STONE, "color").await;
        let hybrid_fallback_stone_id = Uuid::new_v4();
        let hybrid_fallback_stone_error = submit(
            &pool,
            Request {
                id: hybrid_fallback_stone_id,
                strategy: Strategy::Hybrid,
                dependencies: vec![current(STONE, "color", latest_stone_color)],
                writes: vec![write(STONE, "color", "green")],
                operation: "hybrid-absence-fallback-radius-stone".to_owned(),
            },
        )
        .await
        .unwrap_err();
        assert_eq!(
            database_code(&hybrid_fallback_stone_error).as_deref(),
            Some("55P03")
        );
        let latest_door_state = current_id(&pool, DOOR, "state").await;
        let hybrid_fallback_door_id = Uuid::new_v4();
        let hybrid_fallback_door = submit(
            &pool,
            Request {
                id: hybrid_fallback_door_id,
                strategy: Strategy::Hybrid,
                dependencies: vec![current(DOOR, "state", latest_door_state)],
                writes: vec![write(DOOR, "state", "sealed")],
                operation: "hybrid-absence-fallback-radius-door".to_owned(),
            },
        )
        .await
        .unwrap();
        assert_eq!(hybrid_fallback_door, Outcome::Accepted);
        hybrid_absence_blocker.rollback().await.unwrap();

        for blocked_id in [
            place_request_id,
            entity_stone_id,
            hybrid_state_id,
            hybrid_fallback_stone_id,
        ] {
            let count: i64 = sqlx::query_scalar("SELECT count(*) FROM activity WHERE id = $1")
                .bind(blocked_id)
                .fetch_one(&pool)
                .await
                .unwrap();
            assert_eq!(count, 0);
        }
    }

    #[sqlx::test(migrations = "./migration")]
    async fn slot_current_lock_blocks_only_the_exact_existing_key(pool: PgPool) {
        let mut blocker = pool.begin().await.unwrap();
        sqlx::query(
            "SELECT entity_id FROM property_slot WHERE entity_id = $1 AND key = 'state' FOR UPDATE",
        )
        .bind(STONE)
        .fetch_one(&mut *blocker)
        .await
        .unwrap();

        let state_request_id = Uuid::new_v4();
        let state_error = submit(
            &pool,
            Request {
                id: state_request_id,
                strategy: Strategy::Slot,
                dependencies: vec![current(
                    STONE,
                    "state",
                    slot_current_id(&pool, STONE, "state").await,
                )],
                writes: vec![write(STONE, "state", "fallen")],
                operation: "slot-radius-state".to_owned(),
            },
        )
        .await
        .unwrap_err();
        assert_eq!(database_code(&state_error).as_deref(), Some("55P03"));

        let color_request_id = Uuid::new_v4();
        let color = submit(
            &pool,
            Request {
                id: color_request_id,
                strategy: Strategy::Slot,
                dependencies: vec![current(
                    STONE,
                    "color",
                    slot_current_id(&pool, STONE, "color").await,
                )],
                writes: vec![write(STONE, "color", "blue")],
                operation: "slot-radius-color".to_owned(),
            },
        )
        .await
        .unwrap();
        assert_eq!(color, Outcome::Accepted);

        let door_request_id = Uuid::new_v4();
        let door = submit(
            &pool,
            Request {
                id: door_request_id,
                strategy: Strategy::Slot,
                dependencies: vec![current(
                    DOOR,
                    "state",
                    slot_current_id(&pool, DOOR, "state").await,
                )],
                writes: vec![write(DOOR, "state", "barred")],
                operation: "slot-radius-door".to_owned(),
            },
        )
        .await
        .unwrap();
        assert_eq!(door, Outcome::Accepted);
        blocker.rollback().await.unwrap();

        let blocked_activity: i64 =
            sqlx::query_scalar("SELECT count(*) FROM activity WHERE id = $1")
                .bind(state_request_id)
                .fetch_one(&pool)
                .await
                .unwrap();
        assert_eq!(blocked_activity, 0);
    }

    #[sqlx::test(migrations = "./migration")]
    async fn shared_writer_rolls_back_activity_and_state_on_failure(pool: PgPool) {
        let request_id = Uuid::new_v4();
        let failed = submit(
            &pool,
            Request {
                id: request_id,
                strategy: Strategy::Hybrid,
                dependencies: vec![absent(STONE, "failed_mark")],
                writes: vec![write(STONE, "failed_mark", "__force_failure__")],
                operation: "forced-failure".to_owned(),
            },
        )
        .await;
        assert!(failed.is_err());
        let partial: (i64, i64, i64) = sqlx::query_as(
            r#"
            SELECT
                (SELECT count(*) FROM activity WHERE id = $1),
                (SELECT count(*) FROM property_history
                 WHERE entity_id = $2 AND key = 'failed_mark'),
                (SELECT count(*) FROM property_current
                 WHERE entity_id = $2 AND key = 'failed_mark')
            "#,
        )
        .bind(request_id)
        .bind(STONE)
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_eq!(partial, (0, 0, 0));
    }

    async fn serializable_causal_contender(
        pool: PgPool,
        barrier: Arc<Barrier>,
        request_id: Uuid,
        target_entity_id: Uuid,
        target_key: &str,
        value: &str,
    ) -> Result<(), sqlx::Error> {
        let mut transaction = pool.begin().await?;
        sqlx::query("SET TRANSACTION ISOLATION LEVEL SERIALIZABLE")
            .execute(&mut *transaction)
            .await?;
        set_lock_timeout(&mut transaction).await?;

        let stone_state = find_current(&mut transaction, Strategy::Hybrid, STONE, "state")
            .await?
            .expect("seeded Stone state");
        let door_state = find_current(&mut transaction, Strategy::Hybrid, DOOR, "state")
            .await?
            .expect("seeded Door state");
        barrier.wait().await;

        let previous = if target_entity_id == STONE {
            stone_state
        } else {
            door_state
        };
        sqlx::query(
            "INSERT INTO activity (id, strategy, operation) VALUES ($1, 'serializable', 'causal-write-skew')",
        )
        .bind(request_id)
        .execute(&mut *transaction)
        .await?;
        sqlx::query(
            r#"
            INSERT INTO property_history (
                entity_id, key, activity_id, previous_activity_id, value
            ) VALUES ($1, $2, $3, $4, $5)
            "#,
        )
        .bind(target_entity_id)
        .bind(target_key)
        .bind(request_id)
        .bind(previous)
        .bind(value)
        .execute(&mut *transaction)
        .await?;
        sqlx::query(
            r#"
            UPDATE property_current
            SET current_activity_id = $3
            WHERE entity_id = $1 AND key = $2
            "#,
        )
        .bind(target_entity_id)
        .bind(target_key)
        .bind(request_id)
        .execute(&mut *transaction)
        .await?;
        transaction.commit().await
    }

    #[sqlx::test(migrations = "./migration")]
    async fn serializable_setting_aborts_one_cross_causal_writer(pool: PgPool) {
        let barrier = Arc::new(Barrier::new(3));
        let request_ids = [Uuid::new_v4(), Uuid::new_v4()];
        let stone = {
            let pool = pool.clone();
            let barrier = barrier.clone();
            tokio::spawn(async move {
                serializable_causal_contender(
                    pool,
                    barrier,
                    request_ids[0],
                    STONE,
                    "state",
                    "through-door",
                )
                .await
            })
        };
        let door = {
            let pool = pool.clone();
            let barrier = barrier.clone();
            tokio::spawn(async move {
                serializable_causal_contender(
                    pool,
                    barrier,
                    request_ids[1],
                    DOOR,
                    "state",
                    "closed",
                )
                .await
            })
        };
        barrier.wait().await;
        let result = timeout(TEST_TIMEOUT, async {
            [stone.await.unwrap(), door.await.unwrap()]
        })
        .await
        .expect("the controlled serializable pair must remain bounded");
        assert_eq!(result.iter().filter(|result| result.is_ok()).count(), 1);
        let failure = result
            .iter()
            .find_map(|result| result.as_ref().err())
            .expect("one contender must abort");
        assert_eq!(database_code(failure).as_deref(), Some("40001"));
        let activity_count: i64 =
            sqlx::query_scalar("SELECT count(*) FROM activity WHERE id = ANY($1::uuid[])")
                .bind(request_ids)
                .fetch_one(&pool)
                .await
                .unwrap();
        assert_eq!(activity_count, 1);
    }

    #[tokio::test]
    #[ignore = "run after the sqlx tests to verify their disposable databases were removed"]
    async fn audit_sqlx_database_cleanup() {
        let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = PgPool::connect(&url).await.unwrap();
        let leaked: i64 = sqlx::query_scalar(
            r#"
            SELECT count(*)
            FROM _sqlx_test.databases
            WHERE test_path LIKE '%postgres_conflict_strategies%'
            "#,
        )
        .fetch_one(&pool)
        .await
        .unwrap();
        pool.close().await;
        assert_eq!(leaked, 0, "the lab left registered sqlx test databases");
    }
}
