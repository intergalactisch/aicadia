//! Experimental PostgreSQL subject-conflict fixture.
//!
//! The schema and submit functions are deliberately small and independent from
//! production Aicadia. They exercise real PostgreSQL transactions, row locks,
//! foreign-key locks, unique constraints and rollback only for the documented lab
//! scenarios.

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::time::Duration;

    use sqlx::{PgPool, Postgres, Transaction};
    use tokio::sync::Barrier;
    use tokio::time::timeout;
    use uuid::Uuid;

    const PLACE: Uuid = Uuid::from_u128(0x100);
    const STONE: Uuid = Uuid::from_u128(0x200);
    const DOOR: Uuid = Uuid::from_u128(0x201);
    const STONE_STATE_ACTIVITY: Uuid = Uuid::from_u128(0x300);
    const DOOR_STATE_ACTIVITY: Uuid = Uuid::from_u128(0x301);
    const LOCK_TIMEOUT: &str = "150ms";
    const TEST_TIMEOUT: Duration = Duration::from_secs(2);

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    enum PropertyExpectation {
        Absent,
        Current(Uuid),
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    enum Submission {
        Accepted(Uuid),
        Replay(Uuid),
        DependencyConflict,
        FingerprintConflict,
    }

    struct PropertyRequest<'a> {
        request_id: Uuid,
        fingerprint: &'a str,
        entity_id: Uuid,
        key: &'a str,
        expectation: PropertyExpectation,
        value: &'a str,
    }

    async fn set_lock_timeout(
        transaction: &mut Transaction<'_, Postgres>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(&format!("SET LOCAL lock_timeout = '{LOCK_TIMEOUT}'"))
            .execute(&mut **transaction)
            .await?;
        Ok(())
    }

    async fn accepted_request(
        transaction: &mut Transaction<'_, Postgres>,
        request_id: Uuid,
        fingerprint: &str,
    ) -> Result<Option<Submission>, sqlx::Error> {
        let row: Option<(Uuid, String)> =
            sqlx::query_as("SELECT id, fingerprint FROM activity WHERE request_id = $1")
                .bind(request_id)
                .fetch_optional(&mut **transaction)
                .await?;
        Ok(row.map(|(activity_id, stored_fingerprint)| {
            if stored_fingerprint == fingerprint {
                Submission::Replay(activity_id)
            } else {
                Submission::FingerprintConflict
            }
        }))
    }

    async fn submit_property(
        pool: &PgPool,
        request: PropertyRequest<'_>,
    ) -> Result<Submission, sqlx::Error> {
        let mut transaction = pool.begin().await?;
        set_lock_timeout(&mut transaction).await?;

        if let Some(existing) =
            accepted_request(&mut transaction, request.request_id, request.fingerprint).await?
        {
            transaction.rollback().await?;
            return Ok(existing);
        }

        let found: Option<Uuid> =
            sqlx::query_scalar("SELECT id FROM entity WHERE id = $1 FOR UPDATE")
                .bind(request.entity_id)
                .fetch_optional(&mut *transaction)
                .await?;
        if found.is_none() {
            transaction.rollback().await?;
            return Ok(Submission::DependencyConflict);
        }

        let current: Option<Uuid> = sqlx::query_scalar(
            "SELECT current_activity_id FROM property_current WHERE entity_id = $1 AND key = $2",
        )
        .bind(request.entity_id)
        .bind(request.key)
        .fetch_optional(&mut *transaction)
        .await?;
        let matches = match request.expectation {
            PropertyExpectation::Absent => current.is_none(),
            PropertyExpectation::Current(expected) => current == Some(expected),
        };
        if !matches {
            transaction.rollback().await?;
            return Ok(Submission::DependencyConflict);
        }

        sqlx::query(
            r#"
            INSERT INTO activity (id, request_id, fingerprint, entity_id, operation)
            VALUES ($1, $1, $2, $3, 'set_property')
            "#,
        )
        .bind(request.request_id)
        .bind(request.fingerprint)
        .bind(request.entity_id)
        .execute(&mut *transaction)
        .await?;
        sqlx::query(
            r#"
            INSERT INTO property_history (
                entity_id, key, activity_id, previous_activity_id, value
            )
            VALUES ($1, $2, $3, $4, $5)
            "#,
        )
        .bind(request.entity_id)
        .bind(request.key)
        .bind(request.request_id)
        .bind(current)
        .bind(request.value)
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
        .bind(request.entity_id)
        .bind(request.key)
        .bind(request.request_id)
        .execute(&mut *transaction)
        .await?;
        transaction.commit().await?;
        Ok(Submission::Accepted(request.request_id))
    }

    async fn introduce_entity(
        pool: &PgPool,
        request_id: Uuid,
        fingerprint: &str,
        entity_id: Uuid,
    ) -> Result<Submission, sqlx::Error> {
        let mut transaction = pool.begin().await?;
        set_lock_timeout(&mut transaction).await?;

        if let Some(existing) = accepted_request(&mut transaction, request_id, fingerprint).await? {
            transaction.rollback().await?;
            return Ok(existing);
        }

        sqlx::query("INSERT INTO entity (id, place_id, name) VALUES ($1, $2, 'Introduced')")
            .bind(entity_id)
            .bind(PLACE)
            .execute(&mut *transaction)
            .await?;
        sqlx::query(
            r#"
            INSERT INTO activity (id, request_id, fingerprint, entity_id, operation)
            VALUES ($1, $1, $2, $3, 'introduce_entity')
            "#,
        )
        .bind(request_id)
        .bind(fingerprint)
        .bind(entity_id)
        .execute(&mut *transaction)
        .await?;
        transaction.commit().await?;
        Ok(Submission::Accepted(request_id))
    }

    fn database_code(error: &sqlx::Error) -> Option<String> {
        error
            .as_database_error()
            .and_then(|error| error.code())
            .map(|code| code.into_owned())
    }

    #[sqlx::test(migrations = "./migration")]
    async fn held_place_update_lock_exposes_fk_boundary_but_not_existing_entity_work(pool: PgPool) {
        let mut blocker = pool.begin().await.unwrap();
        sqlx::query("SELECT id FROM place WHERE id = $1 FOR UPDATE")
            .bind(PLACE)
            .fetch_one(&mut *blocker)
            .await
            .unwrap();

        let door_request_id = Uuid::new_v4();
        let door = timeout(
            TEST_TIMEOUT,
            submit_property(
                &pool,
                PropertyRequest {
                    request_id: door_request_id,
                    fingerprint: "door-opens-v1",
                    entity_id: DOOR,
                    key: "state",
                    expectation: PropertyExpectation::Current(DOOR_STATE_ACTIVITY),
                    value: "open",
                },
            ),
        )
        .await
        .expect("Door work must remain bounded")
        .unwrap();
        assert_eq!(door, Submission::Accepted(door_request_id));

        let blocked_request_id = Uuid::new_v4();
        let blocked_entity_id = Uuid::new_v4();
        let blocked = timeout(
            TEST_TIMEOUT,
            introduce_entity(
                &pool,
                blocked_request_id,
                "blocked-introduction-v1",
                blocked_entity_id,
            ),
        )
        .await
        .expect("blocked introduction must stop at its lock timeout")
        .unwrap_err();
        assert_eq!(database_code(&blocked).as_deref(), Some("55P03"));
        let blocked_count: i64 =
            sqlx::query_scalar("SELECT count(*) FROM activity WHERE request_id = $1")
                .bind(blocked_request_id)
                .fetch_one(&pool)
                .await
                .unwrap();
        assert_eq!(blocked_count, 0);

        blocker.rollback().await.unwrap();

        let barrier = Arc::new(Barrier::new(3));
        let first = {
            let pool = pool.clone();
            let barrier = barrier.clone();
            tokio::spawn(async move {
                let request_id = Uuid::new_v4();
                barrier.wait().await;
                introduce_entity(&pool, request_id, "normal-introduction-a", Uuid::new_v4()).await
            })
        };
        let second = {
            let pool = pool.clone();
            let barrier = barrier.clone();
            tokio::spawn(async move {
                let request_id = Uuid::new_v4();
                barrier.wait().await;
                introduce_entity(&pool, request_id, "normal-introduction-b", Uuid::new_v4()).await
            })
        };
        barrier.wait().await;
        let (first, second) = timeout(TEST_TIMEOUT, async {
            (
                first.await.unwrap().unwrap(),
                second.await.unwrap().unwrap(),
            )
        })
        .await
        .expect("compatible normal introductions must complete");
        assert!(matches!(first, Submission::Accepted(_)));
        assert!(matches!(second, Submission::Accepted(_)));
    }

    #[sqlx::test(migrations = "./migration")]
    async fn hot_entity_lock_does_not_block_different_entity_work(pool: PgPool) {
        let mut blocker = pool.begin().await.unwrap();
        sqlx::query("SELECT id FROM entity WHERE id = $1 FOR UPDATE")
            .bind(STONE)
            .fetch_one(&mut *blocker)
            .await
            .unwrap();

        let stone_request_id = Uuid::new_v4();
        let door_request_id = Uuid::new_v4();
        let stone = submit_property(
            &pool,
            PropertyRequest {
                request_id: stone_request_id,
                fingerprint: "blocked-stone-v1",
                entity_id: STONE,
                key: "state",
                expectation: PropertyExpectation::Current(STONE_STATE_ACTIVITY),
                value: "fallen",
            },
        );
        let door = submit_property(
            &pool,
            PropertyRequest {
                request_id: door_request_id,
                fingerprint: "independent-door-v1",
                entity_id: DOOR,
                key: "state",
                expectation: PropertyExpectation::Current(DOOR_STATE_ACTIVITY),
                value: "open",
            },
        );
        let (stone, door) = timeout(TEST_TIMEOUT, async { tokio::join!(stone, door) })
            .await
            .expect("both bounded outcomes must complete");
        assert_eq!(database_code(&stone.unwrap_err()).as_deref(), Some("55P03"));
        assert_eq!(door.unwrap(), Submission::Accepted(door_request_id));
        blocker.rollback().await.unwrap();

        let retry = submit_property(
            &pool,
            PropertyRequest {
                request_id: stone_request_id,
                fingerprint: "blocked-stone-v1",
                entity_id: STONE,
                key: "state",
                expectation: PropertyExpectation::Current(STONE_STATE_ACTIVITY),
                value: "fallen",
            },
        )
        .await
        .unwrap();
        assert_eq!(retry, Submission::Accepted(stone_request_id));
    }

    #[sqlx::test(migrations = "./migration")]
    async fn concurrent_same_current_expectation_has_one_successor(pool: PgPool) {
        let barrier = Arc::new(Barrier::new(3));
        let request_ids = [Uuid::new_v4(), Uuid::new_v4()];
        let mut task = Vec::new();
        for (index, request_id) in request_ids.into_iter().enumerate() {
            let pool = pool.clone();
            let barrier = barrier.clone();
            task.push(tokio::spawn(async move {
                barrier.wait().await;
                submit_property(
                    &pool,
                    PropertyRequest {
                        request_id,
                        fingerprint: if index == 0 {
                            "stone-a-v1"
                        } else {
                            "stone-b-v1"
                        },
                        entity_id: STONE,
                        key: "state",
                        expectation: PropertyExpectation::Current(STONE_STATE_ACTIVITY),
                        value: if index == 0 { "fallen" } else { "split" },
                    },
                )
                .await
            }));
        }
        barrier.wait().await;
        let result = timeout(TEST_TIMEOUT, async {
            [
                task.remove(0).await.unwrap().unwrap(),
                task.remove(0).await.unwrap().unwrap(),
            ]
        })
        .await
        .expect("same-subject contenders must complete within the fixture bound");
        assert_eq!(
            result
                .iter()
                .filter(|result| matches!(result, Submission::Accepted(_)))
                .count(),
            1
        );
        assert_eq!(
            result
                .iter()
                .filter(|result| **result == Submission::DependencyConflict)
                .count(),
            1
        );
        let state: (i64, i64) = sqlx::query_as(
            r#"
            SELECT
                (SELECT count(*) FROM property_history
                 WHERE entity_id = $1 AND key = 'state' AND previous_activity_id = $2),
                (SELECT count(*) FROM activity WHERE request_id = ANY($3::uuid[]))
            "#,
        )
        .bind(STONE)
        .bind(STONE_STATE_ACTIVITY)
        .bind(request_ids)
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_eq!(state, (1, 1));
    }

    #[sqlx::test(migrations = "./migration")]
    async fn concurrent_expected_absence_has_one_root(pool: PgPool) {
        let barrier = Arc::new(Barrier::new(3));
        let request_ids = [Uuid::new_v4(), Uuid::new_v4()];
        let mut task = Vec::new();
        for (index, request_id) in request_ids.into_iter().enumerate() {
            let pool = pool.clone();
            let barrier = barrier.clone();
            task.push(tokio::spawn(async move {
                barrier.wait().await;
                submit_property(
                    &pool,
                    PropertyRequest {
                        request_id,
                        fingerprint: if index == 0 { "mark-a-v1" } else { "mark-b-v1" },
                        entity_id: STONE,
                        key: "mark",
                        expectation: PropertyExpectation::Absent,
                        value: if index == 0 { "circle" } else { "cross" },
                    },
                )
                .await
            }));
        }
        barrier.wait().await;
        let result = timeout(TEST_TIMEOUT, async {
            [
                task.remove(0).await.unwrap().unwrap(),
                task.remove(0).await.unwrap().unwrap(),
            ]
        })
        .await
        .expect("absence contenders must complete within the fixture bound");
        assert_eq!(
            result
                .iter()
                .filter(|result| matches!(result, Submission::Accepted(_)))
                .count(),
            1
        );
        assert_eq!(
            result
                .iter()
                .filter(|result| **result == Submission::DependencyConflict)
                .count(),
            1
        );
        let state: (i64, i64, i64) = sqlx::query_as(
            r#"
            SELECT
                (SELECT count(*) FROM property_history
                 WHERE entity_id = $1 AND key = 'mark' AND previous_activity_id IS NULL),
                (SELECT count(*) FROM property_current
                 WHERE entity_id = $1 AND key = 'mark'),
                (SELECT count(*) FROM activity WHERE request_id = ANY($2::uuid[]))
            "#,
        )
        .bind(STONE)
        .bind(request_ids)
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_eq!(state, (1, 1, 1));
    }

    #[sqlx::test(migrations = "./migration")]
    async fn accepted_retry_and_failed_write_preserve_atomicity(pool: PgPool) {
        let accepted_request_id = Uuid::new_v4();
        let accepted = submit_property(
            &pool,
            PropertyRequest {
                request_id: accepted_request_id,
                fingerprint: "accepted-v1",
                entity_id: DOOR,
                key: "state",
                expectation: PropertyExpectation::Current(DOOR_STATE_ACTIVITY),
                value: "open",
            },
        )
        .await
        .unwrap();
        assert_eq!(accepted, Submission::Accepted(accepted_request_id));
        let replay = submit_property(
            &pool,
            PropertyRequest {
                request_id: accepted_request_id,
                fingerprint: "accepted-v1",
                entity_id: DOOR,
                key: "state",
                expectation: PropertyExpectation::Current(DOOR_STATE_ACTIVITY),
                value: "open",
            },
        )
        .await
        .unwrap();
        assert_eq!(replay, Submission::Replay(accepted_request_id));
        let conflict = submit_property(
            &pool,
            PropertyRequest {
                request_id: accepted_request_id,
                fingerprint: "changed-v2",
                entity_id: DOOR,
                key: "state",
                expectation: PropertyExpectation::Current(DOOR_STATE_ACTIVITY),
                value: "closed-again",
            },
        )
        .await
        .unwrap();
        assert_eq!(conflict, Submission::FingerprintConflict);

        let failed_request_id = Uuid::new_v4();
        let failed = submit_property(
            &pool,
            PropertyRequest {
                request_id: failed_request_id,
                fingerprint: "forced-failure-v1",
                entity_id: STONE,
                key: "mark",
                expectation: PropertyExpectation::Absent,
                value: "__force_failure__",
            },
        )
        .await;
        assert!(failed.is_err());
        let partial: (i64, i64, i64) = sqlx::query_as(
            r#"
            SELECT
                (SELECT count(*) FROM activity WHERE request_id = $1),
                (SELECT count(*) FROM property_history
                 WHERE entity_id = $2 AND key = 'mark'),
                (SELECT count(*) FROM property_current
                 WHERE entity_id = $2 AND key = 'mark')
            "#,
        )
        .bind(failed_request_id)
        .bind(STONE)
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_eq!(partial, (0, 0, 0));

        let corrected = submit_property(
            &pool,
            PropertyRequest {
                request_id: failed_request_id,
                fingerprint: "corrected-v2",
                entity_id: STONE,
                key: "mark",
                expectation: PropertyExpectation::Absent,
                value: "circle",
            },
        )
        .await
        .unwrap();
        assert_eq!(corrected, Submission::Accepted(failed_request_id));
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
            WHERE test_path LIKE '%postgres_subject_conflict%'
            "#,
        )
        .fetch_one(&pool)
        .await
        .unwrap();
        pool.close().await;
        assert_eq!(leaked, 0, "the lab left registered sqlx test databases");
    }
}
