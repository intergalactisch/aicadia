//! Experimental PostgreSQL Position-lineage fixture.
//!
//! This standalone crate exercises only its scratch schema. It is not production
//! Aicadia code and must never be imported by the runtime.

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, BTreeSet};
    use std::sync::Arc;
    use std::time::Duration;

    use sqlx::{PgPool, Postgres, Row, Transaction};
    use tokio::sync::Barrier;
    use tokio::time::timeout;
    use uuid::Uuid;

    const COORDINATE_LIMIT: i128 = 9_000_000_000_000_000;
    const LOCK_TIMEOUT: &str = "150ms";
    const STATEMENT_TIMEOUT: &str = "150ms";
    const OUTER_TIMEOUT: Duration = Duration::from_secs(2);
    const SEED_REQUESTER: Uuid = Uuid::from_u128(1);

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    struct Point {
        x: i128,
        y: i128,
        z: i128,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    struct PositionValue {
        reference_entity_id: Option<Uuid>,
        x: i64,
        y: i64,
        z: i64,
    }

    impl PositionValue {
        fn world(x: i64, y: i64, z: i64) -> Self {
            Self {
                reference_entity_id: None,
                x,
                y,
                z,
            }
        }

        fn relative(reference_entity_id: Uuid, x: i64, y: i64, z: i64) -> Self {
            Self {
                reference_entity_id: Some(reference_entity_id),
                x,
                y,
                z,
            }
        }
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    struct ChainNode {
        entity_id: Uuid,
        activity_id: Uuid,
        reference_entity_id: Option<Uuid>,
        x: i64,
        y: i64,
        z: i64,
        cycle: bool,
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    enum LabError {
        MissingPosition,
        Cycle,
        OutOfBounds,
        RevisionConflict,
        FingerprintConflict,
        Sql(String, Option<String>),
    }

    impl From<sqlx::Error> for LabError {
        fn from(error: sqlx::Error) -> Self {
            let code = error
                .as_database_error()
                .and_then(|database| database.code())
                .map(|code| code.into_owned());
            Self::Sql(error.to_string(), code)
        }
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    enum Submission {
        Accepted(Uuid),
        Replay(Uuid),
    }

    struct PositionRequest<'a> {
        requester_id: Uuid,
        request_id: Uuid,
        fingerprint: &'a str,
        entity_id: Uuid,
        expected_activity_id: Option<Uuid>,
        value: PositionValue,
        description: Option<&'a str>,
        synchronize_after_discovery: Option<Arc<Barrier>>,
        inject_failure: bool,
    }

    async fn set_timeouts(transaction: &mut Transaction<'_, Postgres>) -> Result<(), sqlx::Error> {
        sqlx::query(&format!("SET LOCAL lock_timeout = '{LOCK_TIMEOUT}'"))
            .execute(&mut **transaction)
            .await?;
        sqlx::query(&format!(
            "SET LOCAL statement_timeout = '{STATEMENT_TIMEOUT}'"
        ))
        .execute(&mut **transaction)
        .await?;
        Ok(())
    }

    async fn discover_chain(
        transaction: &mut Transaction<'_, Postgres>,
        entity_id: Uuid,
    ) -> Result<Vec<ChainNode>, sqlx::Error> {
        let rows = sqlx::query(
            r#"
            WITH RECURSIVE chain AS (
                SELECT
                    version.entity_id,
                    version.activity_id,
                    version.reference_entity_id,
                    version.x,
                    version.y,
                    version.z,
                    ARRAY[version.entity_id]::uuid[] AS path,
                    false AS cycle,
                    0 AS depth
                FROM position current
                JOIN position_version version
                  ON version.entity_id = current.entity_id
                 AND version.activity_id = current.current_activity_id
                WHERE current.entity_id = $1

                UNION ALL

                SELECT
                    version.entity_id,
                    version.activity_id,
                    version.reference_entity_id,
                    version.x,
                    version.y,
                    version.z,
                    chain.path || version.entity_id,
                    version.entity_id = ANY(chain.path),
                    chain.depth + 1
                FROM chain
                JOIN position current
                  ON current.entity_id = chain.reference_entity_id
                JOIN position_version version
                  ON version.entity_id = current.entity_id
                 AND version.activity_id = current.current_activity_id
                WHERE chain.reference_entity_id IS NOT NULL
                  AND NOT chain.cycle
            )
            SELECT entity_id, activity_id, reference_entity_id, x, y, z, cycle
            FROM chain
            ORDER BY depth
            "#,
        )
        .bind(entity_id)
        .fetch_all(&mut **transaction)
        .await?;

        rows.into_iter()
            .map(|row| {
                Ok(ChainNode {
                    entity_id: row.try_get("entity_id")?,
                    activity_id: row.try_get("activity_id")?,
                    reference_entity_id: row.try_get("reference_entity_id")?,
                    x: row.try_get("x")?,
                    y: row.try_get("y")?,
                    z: row.try_get("z")?,
                    cycle: row.try_get("cycle")?,
                })
            })
            .collect()
    }

    fn validate_chain(chain: &[ChainNode]) -> Result<(), LabError> {
        let Some(last) = chain.last() else {
            return Err(LabError::MissingPosition);
        };
        if chain.iter().any(|node| node.cycle) {
            return Err(LabError::Cycle);
        }
        if last.reference_entity_id.is_some() {
            return Err(LabError::MissingPosition);
        }
        Ok(())
    }

    fn checked_add(value: i128, addend: i64) -> Result<i128, LabError> {
        let result = value
            .checked_add(i128::from(addend))
            .ok_or(LabError::OutOfBounds)?;
        if !(-COORDINATE_LIMIT..=COORDINATE_LIMIT).contains(&result) {
            return Err(LabError::OutOfBounds);
        }
        Ok(result)
    }

    fn resolve_chain(chain: &[ChainNode]) -> Result<Point, LabError> {
        validate_chain(chain)?;
        let mut point = Point { x: 0, y: 0, z: 0 };
        for node in chain {
            point.x = checked_add(point.x, node.x)?;
            point.y = checked_add(point.y, node.y)?;
            point.z = checked_add(point.z, node.z)?;
        }
        Ok(point)
    }

    fn resolve_proposal(
        value: PositionValue,
        reference_chain: &[ChainNode],
    ) -> Result<Point, LabError> {
        let base = if value.reference_entity_id.is_some() {
            resolve_chain(reference_chain)?
        } else {
            Point { x: 0, y: 0, z: 0 }
        };
        Ok(Point {
            x: checked_add(base.x, value.x)?,
            y: checked_add(base.y, value.y)?,
            z: checked_add(base.z, value.z)?,
        })
    }

    fn revision_snapshot(chain: &[ChainNode]) -> Vec<(Uuid, Uuid)> {
        chain
            .iter()
            .map(|node| (node.entity_id, node.activity_id))
            .collect()
    }

    async fn lock_candidate_entities(
        transaction: &mut Transaction<'_, Postgres>,
        changed_entity_id: Uuid,
        reference_chain: &[ChainNode],
    ) -> Result<(), LabError> {
        let mut lock = BTreeMap::new();
        lock.insert(changed_entity_id, true);
        for node in reference_chain {
            if node.entity_id == changed_entity_id {
                return Err(LabError::Cycle);
            }
            lock.entry(node.entity_id).or_insert(false);
        }

        for (entity_id, writer) in lock {
            let found: Option<Uuid> = if writer {
                sqlx::query_scalar("SELECT id FROM entity WHERE id = $1 FOR NO KEY UPDATE")
                    .bind(entity_id)
                    .fetch_optional(&mut **transaction)
                    .await?
            } else {
                sqlx::query_scalar("SELECT id FROM entity WHERE id = $1 FOR SHARE")
                    .bind(entity_id)
                    .fetch_optional(&mut **transaction)
                    .await?
            };
            if found.is_none() {
                return Err(LabError::RevisionConflict);
            }
        }
        Ok(())
    }

    async fn current_position_activity(
        transaction: &mut Transaction<'_, Postgres>,
        entity_id: Uuid,
    ) -> Result<Option<Uuid>, sqlx::Error> {
        sqlx::query_scalar("SELECT current_activity_id FROM position WHERE entity_id = $1")
            .bind(entity_id)
            .fetch_optional(&mut **transaction)
            .await
    }

    async fn find_request(
        transaction: &mut Transaction<'_, Postgres>,
        requester_id: Uuid,
        request_id: Uuid,
        fingerprint: &str,
    ) -> Result<Option<Submission>, LabError> {
        let existing: Option<(Uuid, String)> = sqlx::query_as(
            "SELECT id, fingerprint FROM activity WHERE requester_id = $1 AND request_id = $2",
        )
        .bind(requester_id)
        .bind(request_id)
        .fetch_optional(&mut **transaction)
        .await?;
        match existing {
            Some((activity_id, stored)) if stored == fingerprint => {
                Ok(Some(Submission::Replay(activity_id)))
            }
            Some(_) => Err(LabError::FingerprintConflict),
            None => Ok(None),
        }
    }

    async fn write_position_locked(
        transaction: &mut Transaction<'_, Postgres>,
        request: &PositionRequest<'_>,
        activity_id: Uuid,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO activity (
                id, requester_id, request_id, fingerprint, entity_id, operation
            ) VALUES ($1, $2, $3, $4, $5, 'set_position')
            "#,
        )
        .bind(activity_id)
        .bind(request.requester_id)
        .bind(request.request_id)
        .bind(request.fingerprint)
        .bind(request.entity_id)
        .execute(&mut **transaction)
        .await?;
        sqlx::query(
            r#"
            INSERT INTO position_version (
                entity_id, activity_id, previous_activity_id,
                reference_entity_id, x, y, z, description
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
        )
        .bind(request.entity_id)
        .bind(activity_id)
        .bind(request.expected_activity_id)
        .bind(request.value.reference_entity_id)
        .bind(request.value.x)
        .bind(request.value.y)
        .bind(request.value.z)
        .bind(request.description)
        .execute(&mut **transaction)
        .await?;

        if let Some(expected) = request.expected_activity_id {
            let changed = sqlx::query(
                r#"
                UPDATE position
                SET current_activity_id = $3
                WHERE entity_id = $1 AND current_activity_id = $2
                "#,
            )
            .bind(request.entity_id)
            .bind(expected)
            .bind(activity_id)
            .execute(&mut **transaction)
            .await?;
            if changed.rows_affected() != 1 {
                return Err(sqlx::Error::RowNotFound);
            }
        } else {
            sqlx::query("INSERT INTO position (entity_id, current_activity_id) VALUES ($1, $2)")
                .bind(request.entity_id)
                .bind(activity_id)
                .execute(&mut **transaction)
                .await?;
        }

        if request.inject_failure {
            sqlx::query(
                r#"
                INSERT INTO local_state_version (
                    entity_id, activity_id, previous_activity_id, value
                ) VALUES ($1, $2, NULL, '__force_failure__')
                "#,
            )
            .bind(request.entity_id)
            .bind(activity_id)
            .execute(&mut **transaction)
            .await?;
        }
        Ok(())
    }

    async fn set_position(
        pool: &PgPool,
        request: PositionRequest<'_>,
    ) -> Result<Submission, LabError> {
        let mut transaction = pool.begin().await?;
        set_timeouts(&mut transaction).await?;

        if let Some(replay) = find_request(
            &mut transaction,
            request.requester_id,
            request.request_id,
            request.fingerprint,
        )
        .await?
        {
            transaction.rollback().await?;
            return Ok(replay);
        }

        let discovered = match request.value.reference_entity_id {
            Some(reference) => discover_chain(&mut transaction, reference).await?,
            None => Vec::new(),
        };
        if let Some(barrier) = &request.synchronize_after_discovery {
            barrier.wait().await;
        }
        if discovered
            .iter()
            .any(|node| node.entity_id == request.entity_id)
        {
            transaction.rollback().await?;
            return Err(LabError::Cycle);
        }
        if request.value.reference_entity_id.is_some() {
            validate_chain(&discovered)?;
        }
        let discovered_revisions = revision_snapshot(&discovered);

        lock_candidate_entities(&mut transaction, request.entity_id, &discovered).await?;

        if current_position_activity(&mut transaction, request.entity_id).await?
            != request.expected_activity_id
        {
            transaction.rollback().await?;
            return Err(LabError::RevisionConflict);
        }

        let rechecked = match request.value.reference_entity_id {
            Some(reference) => discover_chain(&mut transaction, reference).await?,
            None => Vec::new(),
        };
        if revision_snapshot(&rechecked) != discovered_revisions {
            transaction.rollback().await?;
            return Err(LabError::RevisionConflict);
        }
        if rechecked
            .iter()
            .any(|node| node.entity_id == request.entity_id)
        {
            transaction.rollback().await?;
            return Err(LabError::Cycle);
        }
        resolve_proposal(request.value, &rechecked)?;

        let activity_id = Uuid::new_v4();
        write_position_locked(&mut transaction, &request, activity_id).await?;
        transaction.commit().await?;
        Ok(Submission::Accepted(activity_id))
    }

    async fn seed_position(
        pool: &PgPool,
        entity_id: Uuid,
        name: &str,
        value: PositionValue,
    ) -> Uuid {
        let activity_id = Uuid::new_v4();
        let mut transaction = pool.begin().await.unwrap();
        sqlx::query("INSERT INTO entity (id, name) VALUES ($1, $2)")
            .bind(entity_id)
            .bind(name)
            .execute(&mut *transaction)
            .await
            .unwrap();
        sqlx::query(
            r#"
            INSERT INTO activity (
                id, requester_id, request_id, fingerprint, entity_id, operation
            ) VALUES ($1, $2, $1, 'seed', $3, 'seed_position')
            "#,
        )
        .bind(activity_id)
        .bind(SEED_REQUESTER)
        .bind(entity_id)
        .execute(&mut *transaction)
        .await
        .unwrap();
        sqlx::query(
            r#"
            INSERT INTO position_version (
                entity_id, activity_id, previous_activity_id,
                reference_entity_id, x, y, z, description
            ) VALUES ($1, $2, NULL, $3, $4, $5, $6, NULL)
            "#,
        )
        .bind(entity_id)
        .bind(activity_id)
        .bind(value.reference_entity_id)
        .bind(value.x)
        .bind(value.y)
        .bind(value.z)
        .execute(&mut *transaction)
        .await
        .unwrap();
        sqlx::query("INSERT INTO position (entity_id, current_activity_id) VALUES ($1, $2)")
            .bind(entity_id)
            .bind(activity_id)
            .execute(&mut *transaction)
            .await
            .unwrap();
        transaction.commit().await.unwrap();
        activity_id
    }

    async fn seed_relative_batch(pool: &PgPool, reference: Uuid, entity_id: &[Uuid]) {
        let activity_id = entity_id.iter().map(|_| Uuid::new_v4()).collect::<Vec<_>>();
        let names = entity_id
            .iter()
            .enumerate()
            .map(|(index, _)| format!("Descendant {index}"))
            .collect::<Vec<_>>();
        let mut transaction = pool.begin().await.unwrap();
        sqlx::query(
            r#"
            INSERT INTO entity (id, name)
            SELECT * FROM UNNEST($1::uuid[], $2::text[])
            "#,
        )
        .bind(entity_id)
        .bind(&names)
        .execute(&mut *transaction)
        .await
        .unwrap();
        sqlx::query(
            r#"
            INSERT INTO activity (
                id, requester_id, request_id, fingerprint, entity_id, operation
            )
            SELECT seed.activity_id, $1, seed.activity_id, 'seed', seed.entity_id,
                   'seed_position'
            FROM UNNEST($2::uuid[], $3::uuid[])
                 AS seed(activity_id, entity_id)
            "#,
        )
        .bind(SEED_REQUESTER)
        .bind(&activity_id)
        .bind(entity_id)
        .execute(&mut *transaction)
        .await
        .unwrap();
        sqlx::query(
            r#"
            INSERT INTO position_version (
                entity_id, activity_id, previous_activity_id,
                reference_entity_id, x, y, z, description
            )
            SELECT seed.entity_id, seed.activity_id, NULL, $1, 0, 0, 0, NULL
            FROM UNNEST($2::uuid[], $3::uuid[])
                 AS seed(entity_id, activity_id)
            "#,
        )
        .bind(reference)
        .bind(entity_id)
        .bind(&activity_id)
        .execute(&mut *transaction)
        .await
        .unwrap();
        sqlx::query(
            r#"
            INSERT INTO position (entity_id, current_activity_id)
            SELECT * FROM UNNEST($1::uuid[], $2::uuid[])
            "#,
        )
        .bind(entity_id)
        .bind(&activity_id)
        .execute(&mut *transaction)
        .await
        .unwrap();
        transaction.commit().await.unwrap();
    }

    async fn resolve_current(pool: &PgPool, entity_id: Uuid) -> Result<Point, LabError> {
        let mut transaction = pool.begin().await?;
        set_timeouts(&mut transaction).await?;
        let chain = discover_chain(&mut transaction, entity_id).await?;
        let point = resolve_chain(&chain)?;
        transaction.commit().await?;
        Ok(point)
    }

    async fn resolve_for_operation(pool: &PgPool, entity_id: Uuid) -> Result<Point, LabError> {
        let mut transaction = pool.begin().await?;
        set_timeouts(&mut transaction).await?;
        let discovered = discover_chain(&mut transaction, entity_id).await?;
        validate_chain(&discovered)?;
        let snapshot = revision_snapshot(&discovered);
        let ids = discovered
            .iter()
            .map(|node| node.entity_id)
            .collect::<BTreeSet<_>>();
        for id in ids {
            sqlx::query_scalar::<_, Uuid>("SELECT id FROM entity WHERE id = $1 FOR SHARE")
                .bind(id)
                .fetch_one(&mut *transaction)
                .await?;
        }
        let rechecked = discover_chain(&mut transaction, entity_id).await?;
        if revision_snapshot(&rechecked) != snapshot {
            transaction.rollback().await?;
            return Err(LabError::RevisionConflict);
        }
        let point = resolve_chain(&rechecked)?;
        transaction.commit().await?;
        Ok(point)
    }

    async fn set_local_state(
        pool: &PgPool,
        requester_id: Uuid,
        request_id: Uuid,
        entity_id: Uuid,
        value: &str,
    ) -> Result<Uuid, LabError> {
        let mut transaction = pool.begin().await?;
        set_timeouts(&mut transaction).await?;
        sqlx::query_scalar::<_, Uuid>("SELECT id FROM entity WHERE id = $1 FOR NO KEY UPDATE")
            .bind(entity_id)
            .fetch_one(&mut *transaction)
            .await?;
        let previous: Option<Uuid> =
            sqlx::query_scalar("SELECT current_activity_id FROM local_state WHERE entity_id = $1")
                .bind(entity_id)
                .fetch_optional(&mut *transaction)
                .await?;
        let activity_id = Uuid::new_v4();
        sqlx::query(
            r#"
            INSERT INTO activity (
                id, requester_id, request_id, fingerprint, entity_id, operation
            ) VALUES ($1, $2, $3, $4, $5, 'set_local_state')
            "#,
        )
        .bind(activity_id)
        .bind(requester_id)
        .bind(request_id)
        .bind(format!("local:{value}"))
        .bind(entity_id)
        .execute(&mut *transaction)
        .await?;
        sqlx::query(
            r#"
            INSERT INTO local_state_version (
                entity_id, activity_id, previous_activity_id, value
            ) VALUES ($1, $2, $3, $4)
            "#,
        )
        .bind(entity_id)
        .bind(activity_id)
        .bind(previous)
        .bind(value)
        .execute(&mut *transaction)
        .await?;
        if previous.is_some() {
            sqlx::query("UPDATE local_state SET current_activity_id = $2 WHERE entity_id = $1")
                .bind(entity_id)
                .bind(activity_id)
                .execute(&mut *transaction)
                .await?;
        } else {
            sqlx::query("INSERT INTO local_state (entity_id, current_activity_id) VALUES ($1, $2)")
                .bind(entity_id)
                .bind(activity_id)
                .execute(&mut *transaction)
                .await?;
        }
        transaction.commit().await?;
        Ok(activity_id)
    }

    fn sql_state(error: &LabError) -> Option<&str> {
        match error {
            LabError::Sql(_, Some(code)) => Some(code.as_str()),
            _ => None,
        }
    }

    async fn assert_acyclic(pool: &PgPool, entity_id: Uuid) {
        let mut transaction = pool.begin().await.unwrap();
        set_timeouts(&mut transaction).await.unwrap();
        let chain = discover_chain(&mut transaction, entity_id).await.unwrap();
        validate_chain(&chain).unwrap();
        transaction.rollback().await.unwrap();
    }

    #[sqlx::test(migrations = "./migration")]
    async fn direct_nested_and_sixty_four_level_positions_resolve_exactly(pool: PgPool) {
        let root = Uuid::from_u128(0x100);
        seed_position(&pool, root, "Root", PositionValue::world(100, 200, 300)).await;
        assert_eq!(
            resolve_current(&pool, root).await.unwrap(),
            Point {
                x: 100,
                y: 200,
                z: 300
            }
        );

        let mut reference = root;
        let mut last = root;
        for depth in 1..=64_u128 {
            last = Uuid::from_u128(0x100 + depth);
            seed_position(
                &pool,
                last,
                &format!("Nested {depth}"),
                PositionValue::relative(reference, 1, 2, 3),
            )
            .await;
            reference = last;
        }
        let resolved = resolve_current(&pool, last).await.unwrap();
        assert_eq!(
            resolved,
            Point {
                x: 164,
                y: 328,
                z: 492
            }
        );
        let state: (i64, i64, i64) = sqlx::query_as(
            "SELECT (SELECT count(*) FROM entity), (SELECT count(*) FROM position), (SELECT count(*) FROM activity)",
        )
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_eq!(state, (65, 65, 65));
        eprintln!("case 1: point={resolved:?}, state={state:?}");
    }

    #[sqlx::test(migrations = "./migration")]
    async fn symmetric_bound_rejects_invalid_resolved_points_atomically(pool: PgPool) {
        let positive_root = Uuid::from_u128(0x200);
        let negative_root = Uuid::from_u128(0x201);
        seed_position(
            &pool,
            positive_root,
            "Positive boundary",
            PositionValue::world(COORDINATE_LIMIT as i64, 0, 0),
        )
        .await;
        seed_position(
            &pool,
            negative_root,
            "Negative boundary",
            PositionValue::world(-(COORDINATE_LIMIT as i64), 0, 0),
        )
        .await;
        let positive_child = Uuid::from_u128(0x202);
        let negative_child = Uuid::from_u128(0x203);
        sqlx::query(
            "INSERT INTO entity (id, name) VALUES ($1, 'Positive child'), ($2, 'Negative child')",
        )
        .bind(positive_child)
        .bind(negative_child)
        .execute(&pool)
        .await
        .unwrap();
        let requester = Uuid::new_v4();
        let requests = [Uuid::new_v4(), Uuid::new_v4()];
        let positive = set_position(
            &pool,
            PositionRequest {
                requester_id: requester,
                request_id: requests[0],
                fingerprint: "positive-overflow",
                entity_id: positive_child,
                expected_activity_id: None,
                value: PositionValue::relative(positive_root, 1, 0, 0),
                description: None,
                synchronize_after_discovery: None,
                inject_failure: false,
            },
        )
        .await;
        let negative = set_position(
            &pool,
            PositionRequest {
                requester_id: requester,
                request_id: requests[1],
                fingerprint: "negative-overflow",
                entity_id: negative_child,
                expected_activity_id: None,
                value: PositionValue::relative(negative_root, -1, 0, 0),
                description: None,
                synchronize_after_discovery: None,
                inject_failure: false,
            },
        )
        .await;
        assert_eq!(positive, Err(LabError::OutOfBounds));
        assert_eq!(negative, Err(LabError::OutOfBounds));
        let partial: (i64, i64, i64) = sqlx::query_as(
            r#"
            SELECT
                (SELECT count(*) FROM activity WHERE request_id = ANY($1::uuid[])),
                (SELECT count(*) FROM position_version WHERE entity_id = ANY($2::uuid[])),
                (SELECT count(*) FROM position WHERE entity_id = ANY($2::uuid[]))
            "#,
        )
        .bind(requests)
        .bind([positive_child, negative_child])
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_eq!(partial, (0, 0, 0));
        eprintln!("case 2: positive={positive:?}, negative={negative:?}, partial={partial:?}");
    }

    #[sqlx::test(migrations = "./migration")]
    async fn synchronized_cross_reference_race_commits_at_most_one_edge(pool: PgPool) {
        let a = Uuid::from_u128(0x300);
        let b = Uuid::from_u128(0x301);
        let a_revision = seed_position(&pool, a, "A", PositionValue::world(0, 0, 0)).await;
        let b_revision = seed_position(&pool, b, "B", PositionValue::world(10, 0, 0)).await;
        let barrier = Arc::new(Barrier::new(3));
        let requester = Uuid::new_v4();
        let a_request = Uuid::new_v4();
        let b_request = Uuid::new_v4();
        let a_task = {
            let pool = pool.clone();
            let barrier = barrier.clone();
            tokio::spawn(async move {
                set_position(
                    &pool,
                    PositionRequest {
                        requester_id: requester,
                        request_id: a_request,
                        fingerprint: "a-to-b",
                        entity_id: a,
                        expected_activity_id: Some(a_revision),
                        value: PositionValue::relative(b, 0, 0, 0),
                        description: None,
                        synchronize_after_discovery: Some(barrier),
                        inject_failure: false,
                    },
                )
                .await
            })
        };
        let b_task = {
            let pool = pool.clone();
            let barrier = barrier.clone();
            tokio::spawn(async move {
                set_position(
                    &pool,
                    PositionRequest {
                        requester_id: requester,
                        request_id: b_request,
                        fingerprint: "b-to-a",
                        entity_id: b,
                        expected_activity_id: Some(b_revision),
                        value: PositionValue::relative(a, 0, 0, 0),
                        description: None,
                        synchronize_after_discovery: Some(barrier),
                        inject_failure: false,
                    },
                )
                .await
            })
        };
        barrier.wait().await;
        let result = timeout(OUTER_TIMEOUT, async {
            [a_task.await.unwrap(), b_task.await.unwrap()]
        })
        .await
        .expect("the synchronized cross-reference race must remain bounded");
        let accepted = result
            .iter()
            .filter(|outcome| matches!(outcome, Ok(Submission::Accepted(_))))
            .count();
        assert_eq!(accepted, 1);
        assert_acyclic(&pool, a).await;
        assert_acyclic(&pool, b).await;
        let state: (i64, i64, Uuid, Uuid) = sqlx::query_as(
            r#"
            SELECT
                (SELECT count(*) FROM activity WHERE request_id = ANY($1::uuid[])),
                (SELECT count(*) FROM position_version WHERE entity_id = ANY($2::uuid[])),
                (SELECT current_activity_id FROM position WHERE entity_id = $3),
                (SELECT current_activity_id FROM position WHERE entity_id = $4)
            "#,
        )
        .bind([a_request, b_request])
        .bind([a, b])
        .bind(a)
        .bind(b)
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_eq!((state.0, state.1), (1, 3));
        assert_eq!(
            usize::from(state.2 != a_revision) + usize::from(state.3 != b_revision),
            1
        );
        eprintln!("case 3: outcomes={result:?}, state={state:?}");
    }

    #[sqlx::test(migrations = "./migration")]
    async fn moving_carrier_writes_only_carrier_and_resolves_descendants(pool: PgPool) {
        let ship = Uuid::from_u128(0x400);
        let ship_revision =
            seed_position(&pool, ship, "Ship", PositionValue::world(100, 200, 300)).await;
        let descendants = (0..1_000_u128)
            .map(|index| Uuid::from_u128(0x10_000 + index))
            .collect::<Vec<_>>();
        seed_relative_batch(&pool, ship, &descendants).await;
        let before: (i64, i64) = sqlx::query_as(
            "SELECT (SELECT count(*) FROM position_version), (SELECT count(*) FROM activity)",
        )
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_eq!(before, (1_001, 1_001));

        let move_request = Uuid::new_v4();
        let moved = set_position(
            &pool,
            PositionRequest {
                requester_id: Uuid::new_v4(),
                request_id: move_request,
                fingerprint: "ship-move",
                entity_id: ship,
                expected_activity_id: Some(ship_revision),
                value: PositionValue::world(500, 600, 700),
                description: Some("The ship has moved."),
                synchronize_after_discovery: None,
                inject_failure: false,
            },
        )
        .await
        .unwrap();
        assert!(matches!(moved, Submission::Accepted(_)));
        let after: (i64, i64, i64, i64) = sqlx::query_as(
            r#"
            SELECT
                (SELECT count(*) FROM position_version),
                (SELECT count(*) FROM activity),
                (SELECT count(*) FROM position_version
                 WHERE entity_id = ANY($1::uuid[])),
                (SELECT count(*) FROM activity
                 WHERE entity_id = ANY($1::uuid[]) AND operation = 'set_position')
            "#,
        )
        .bind(&descendants)
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_eq!(after, (1_002, 1_002, 1_000, 0));
        for index in [0, 499, 999] {
            assert_eq!(
                resolve_current(&pool, descendants[index]).await.unwrap(),
                Point {
                    x: 500,
                    y: 600,
                    z: 700
                }
            );
        }
        eprintln!("case 4: move={moved:?}, before={before:?}, after={after:?}");
    }

    #[sqlx::test(migrations = "./migration")]
    async fn carrier_local_work_stays_independent_and_external_resolution_serializes(pool: PgPool) {
        let ship = Uuid::from_u128(0x500);
        let cabin = Uuid::from_u128(0x501);
        let ship_revision =
            seed_position(&pool, ship, "Ship", PositionValue::world(100, 100, 100)).await;
        seed_position(
            &pool,
            cabin,
            "Cabin",
            PositionValue::relative(ship, 10, 20, 30),
        )
        .await;

        let mut writer = pool.begin().await.unwrap();
        set_timeouts(&mut writer).await.unwrap();
        sqlx::query_scalar::<_, Uuid>("SELECT id FROM entity WHERE id = $1 FOR NO KEY UPDATE")
            .bind(ship)
            .fetch_one(&mut *writer)
            .await
            .unwrap();

        let local_request = Uuid::new_v4();
        let local_activity = timeout(
            OUTER_TIMEOUT,
            set_local_state(&pool, Uuid::new_v4(), local_request, cabin, "lamp lit"),
        )
        .await
        .expect("Cabin-local work must not wait for the Ship")
        .unwrap();

        let blocked = timeout(OUTER_TIMEOUT, resolve_for_operation(&pool, cabin))
            .await
            .expect("external resolution must return its bounded lock outcome")
            .unwrap_err();
        assert_eq!(
            sql_state(&blocked),
            Some("57014"),
            "the equal 150 ms statement bound fired while the shared lock waited"
        );

        let move_request = Uuid::new_v4();
        let move_activity = Uuid::new_v4();
        let move_operation = PositionRequest {
            requester_id: Uuid::new_v4(),
            request_id: move_request,
            fingerprint: "held-ship-move",
            entity_id: ship,
            expected_activity_id: Some(ship_revision),
            value: PositionValue::world(1_000, 2_000, 3_000),
            description: None,
            synchronize_after_discovery: None,
            inject_failure: false,
        };
        write_position_locked(&mut writer, &move_operation, move_activity)
            .await
            .unwrap();
        writer.commit().await.unwrap();

        assert_eq!(
            resolve_for_operation(&pool, cabin).await.unwrap(),
            Point {
                x: 1_010,
                y: 2_020,
                z: 3_030
            }
        );
        let state: (Uuid, String, i64, i64) = sqlx::query_as(
            r#"
            SELECT current.current_activity_id, version.value,
                   (SELECT count(*) FROM activity WHERE request_id = $2),
                   (SELECT count(*) FROM activity WHERE request_id = $3)
            FROM local_state current
            JOIN local_state_version version
              ON version.entity_id = current.entity_id
             AND version.activity_id = current.current_activity_id
            WHERE current.entity_id = $1
            "#,
        )
        .bind(cabin)
        .bind(local_request)
        .bind(move_request)
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_eq!(state, (local_activity, "lamp lit".to_owned(), 1, 1));
        let position_state: (Uuid, i64, i64) = sqlx::query_as(
            r#"
            SELECT
                (SELECT current_activity_id FROM position WHERE entity_id = $1),
                (SELECT count(*) FROM position_version WHERE entity_id = $1),
                (SELECT count(*) FROM position_version WHERE entity_id = $2)
            "#,
        )
        .bind(ship)
        .bind(cabin)
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_eq!(position_state, (move_activity, 2, 1));
        eprintln!("case 5: blocked={blocked:?}, local={state:?}, position={position_state:?}");
    }

    #[sqlx::test(migrations = "./migration")]
    async fn lock_modes_match_foreign_keys_and_position_dependencies(pool: PgPool) {
        let ship = Uuid::from_u128(0x600);
        let ship_revision = seed_position(&pool, ship, "Ship", PositionValue::world(0, 0, 0)).await;

        let mut writer = pool.begin().await.unwrap();
        set_timeouts(&mut writer).await.unwrap();
        sqlx::query_scalar::<_, Uuid>("SELECT id FROM entity WHERE id = $1 FOR NO KEY UPDATE")
            .bind(ship)
            .fetch_one(&mut *writer)
            .await
            .unwrap();

        let child = Uuid::from_u128(0x601);
        let child_activity = Uuid::new_v4();
        let mut participant = pool.begin().await.unwrap();
        set_timeouts(&mut participant).await.unwrap();
        timeout(OUTER_TIMEOUT, async {
            sqlx::query("INSERT INTO entity (id, name) VALUES ($1, 'Child')")
                .bind(child)
                .execute(&mut *participant)
                .await?;
            sqlx::query(
                r#"
                INSERT INTO activity (
                    id, requester_id, request_id, fingerprint, entity_id, operation
                ) VALUES ($1, $2, $1, 'fk-participant', $3, 'seed_position')
                "#,
            )
            .bind(child_activity)
            .bind(SEED_REQUESTER)
            .bind(child)
            .execute(&mut *participant)
            .await?;
            sqlx::query(
                r#"
                INSERT INTO position_version (
                    entity_id, activity_id, previous_activity_id,
                    reference_entity_id, x, y, z, description
                ) VALUES ($1, $2, NULL, $3, 0, 0, 0, NULL)
                "#,
            )
            .bind(child)
            .bind(child_activity)
            .bind(ship)
            .execute(&mut *participant)
            .await?;
            sqlx::query("INSERT INTO position (entity_id, current_activity_id) VALUES ($1, $2)")
                .bind(child)
                .bind(child_activity)
                .execute(&mut *participant)
                .await?;
            participant.commit().await
        })
        .await
        .expect("foreign-key key-share participation must remain bounded")
        .unwrap();
        writer.rollback().await.unwrap();

        let mut dependency = pool.begin().await.unwrap();
        set_timeouts(&mut dependency).await.unwrap();
        sqlx::query_scalar::<_, Uuid>("SELECT id FROM entity WHERE id = $1 FOR SHARE")
            .bind(ship)
            .fetch_one(&mut *dependency)
            .await
            .unwrap();
        let blocked_request = Uuid::new_v4();
        let blocked = timeout(
            OUTER_TIMEOUT,
            set_position(
                &pool,
                PositionRequest {
                    requester_id: Uuid::new_v4(),
                    request_id: blocked_request,
                    fingerprint: "blocked-by-share",
                    entity_id: ship,
                    expected_activity_id: Some(ship_revision),
                    value: PositionValue::world(1, 0, 0),
                    description: None,
                    synchronize_after_discovery: None,
                    inject_failure: false,
                },
            ),
        )
        .await
        .expect("the conflicting writer must return its bounded lock result")
        .unwrap_err();
        assert_eq!(
            sql_state(&blocked),
            Some("57014"),
            "the equal 150 ms statement bound fired while NO KEY UPDATE waited on SHARE"
        );
        dependency.rollback().await.unwrap();
        let state: (i64, i64, Uuid, i64) = sqlx::query_as(
            r#"
            SELECT
                (SELECT count(*) FROM position WHERE entity_id = $1),
                (SELECT count(*) FROM activity WHERE request_id = $2),
                (SELECT current_activity_id FROM position WHERE entity_id = $3),
                (SELECT count(*) FROM position_version WHERE entity_id = $3)
            "#,
        )
        .bind(child)
        .bind(blocked_request)
        .bind(ship)
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_eq!(state, (1, 0, ship_revision, 1));
        eprintln!("case 6: child_state={state:?}, blocked={blocked:?}");
    }

    #[sqlx::test(migrations = "./migration")]
    async fn exact_retry_replays_and_changed_fingerprint_conflicts(pool: PgPool) {
        let entity = Uuid::from_u128(0x700);
        let seed = seed_position(&pool, entity, "Cup", PositionValue::world(0, 0, 0)).await;
        let requester = Uuid::new_v4();
        let request_id = Uuid::new_v4();
        let submit = |fingerprint: &'static str| PositionRequest {
            requester_id: requester,
            request_id,
            fingerprint,
            entity_id: entity,
            expected_activity_id: Some(seed),
            value: PositionValue::world(10, 20, 30),
            description: Some("Two centimetres above the table."),
            synchronize_after_discovery: None,
            inject_failure: false,
        };
        let first = set_position(&pool, submit("same-request")).await.unwrap();
        let activity_id = match first {
            Submission::Accepted(id) => id,
            Submission::Replay(_) => panic!("first submission cannot be a replay"),
        };
        assert_eq!(
            set_position(&pool, submit("same-request")).await.unwrap(),
            Submission::Replay(activity_id)
        );
        assert_eq!(
            set_position(&pool, submit("changed-request")).await,
            Err(LabError::FingerprintConflict)
        );
        let state: (i64, i64, Uuid) = sqlx::query_as(
            r#"
            SELECT
                (SELECT count(*) FROM activity WHERE requester_id = $1 AND request_id = $2),
                (SELECT count(*) FROM position_version WHERE entity_id = $3),
                (SELECT current_activity_id FROM position WHERE entity_id = $3)
            "#,
        )
        .bind(requester)
        .bind(request_id)
        .bind(entity)
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_eq!(state, (1, 2, activity_id));
        eprintln!("case 7: activity={activity_id}, state={state:?}");
    }

    #[sqlx::test(migrations = "./migration")]
    async fn injected_failure_rolls_back_activity_version_and_pointer(pool: PgPool) {
        let entity = Uuid::from_u128(0x800);
        let seed = seed_position(&pool, entity, "Cup", PositionValue::world(1, 2, 3)).await;
        let requester = Uuid::new_v4();
        let request_id = Uuid::new_v4();
        let failed = set_position(
            &pool,
            PositionRequest {
                requester_id: requester,
                request_id,
                fingerprint: "forced-failure",
                entity_id: entity,
                expected_activity_id: Some(seed),
                value: PositionValue::world(4, 5, 6),
                description: None,
                synchronize_after_discovery: None,
                inject_failure: true,
            },
        )
        .await;
        assert!(matches!(failed, Err(LabError::Sql(_, _))));
        let state: (i64, i64, Uuid, i64, i64) = sqlx::query_as(
            r#"
            SELECT
                (SELECT count(*) FROM activity WHERE requester_id = $1 AND request_id = $2),
                (SELECT count(*) FROM position_version WHERE entity_id = $3),
                (SELECT current_activity_id FROM position WHERE entity_id = $3),
                (SELECT count(*) FROM local_state_version WHERE entity_id = $3),
                (SELECT count(*) FROM local_state WHERE entity_id = $3)
            "#,
        )
        .bind(requester)
        .bind(request_id)
        .bind(entity)
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_eq!(state, (0, 1, seed, 0, 0));
        assert_eq!(
            resolve_current(&pool, entity).await.unwrap(),
            Point { x: 1, y: 2, z: 3 }
        );
        eprintln!("case 8: failure={failed:?}, state={state:?}");
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
            WHERE test_path LIKE '%postgres_position_lineage_lab%'
            "#,
        )
        .fetch_one(&pool)
        .await
        .unwrap();
        pool.close().await;
        assert_eq!(leaked, 0, "the lab left registered SQLx test databases");
    }
}
