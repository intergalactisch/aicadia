#[cfg(test)]
use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;

use chrono::{DateTime, Utc};
use sha2::{Digest, Sha256};
use sqlx::{FromRow, PgPool, Postgres, Transaction};
use thiserror::Error;
use uuid::Uuid;

const MAX_ENTITY_NAME_LENGTH: usize = 120;
const MAX_ENTITY_DESCRIPTION_LENGTH: usize = 4_000;
const MAX_POSITION_DESCRIPTION_LENGTH: usize = 4_000;
const MAX_COORDINATE_CM: i64 = 1_000_000_000_000_000;
const MAX_PLACE_WINDOW_SPAN_CM: i64 = 100_000_000;
const MAX_ACTION_PROSE_LENGTH: usize = 4_000;
const MAX_INTERACTION_PROSE_LENGTH: usize = 4_000;
const MAX_INTERACTION_TARGET_COUNT: usize = 100;
const DEFAULT_PAGE_SIZE: u16 = 25;
const MAX_PAGE_SIZE: u16 = 100;
const WORLD_NAME: &str = "Aicadia";

mod activity;
mod common;
mod entity_trait;
mod error;
mod investigation;
mod model;
mod movement;
mod mutation;
mod property;
mod read;
mod spatial;
#[cfg(test)]
mod spatial_read_plan_test;

pub use activity::*;
pub use error::*;
pub use investigation::{
    AcceptedDiscovery, ConnectionInput, ConnectionPointInput, DirectPositionInput,
    DiscoveryDestinationInput, DiscoveryKind, DiscoveryOriginInput, DiscoveryResultInput,
    InvestigationAttemptId, InvestigationLimit, InvestigationOutcome, InvestigationResult,
    PlaceEntityInput, StartInvestigation, SubmitDiscovery,
};
pub use model::*;
pub use movement::{AcceptedMovement, MoveCharacter, MovementDirection, MovementTarget};
pub use property::*;
pub use spatial::ActivityPositionRole;

use common::*;
use entity_trait::*;
use read::{CURRENT_ENTITY_STATE_SQL, find_connection_by_id};
use spatial::*;

#[derive(Clone, Copy)]
enum PropertyQueryKind {
    Write,
    CurrentRead,
    Hydration,
}

#[derive(Clone, Copy)]
enum TraitQueryKind {
    Write,
    CurrentRead,
    Hydration,
}

#[derive(Clone, Copy)]
enum SpatialReadQueryKind {
    PlaceCandidate,
    PlaceHydration,
    ConnectionAnchor,
    ConnectionCandidate,
    ConnectionHydration,
    ConnectionGet,
    ConnectionCourse,
}

#[cfg(test)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct PropertyQueryCount {
    write: usize,
    current_read: usize,
    hydration: usize,
}

#[cfg(test)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct TraitQueryCount {
    write: usize,
    current_read: usize,
    hydration: usize,
}

#[cfg(test)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct SpatialReadQueryCount {
    place_candidate: usize,
    place_hydration: usize,
    connection_anchor: usize,
    connection_candidate: usize,
    connection_hydration: usize,
    connection_get: usize,
    connection_course: usize,
}

#[cfg(test)]
tokio::task_local! {
    static PROPERTY_QUERY_COUNT: RefCell<PropertyQueryCount>;
}

#[cfg(test)]
tokio::task_local! {
    static TRAIT_QUERY_COUNT: RefCell<TraitQueryCount>;
}

#[cfg(test)]
tokio::task_local! {
    static SPATIAL_READ_QUERY_COUNT: RefCell<SpatialReadQueryCount>;
}

#[inline]
fn record_property_query(kind: PropertyQueryKind) {
    #[cfg(test)]
    let _ = PROPERTY_QUERY_COUNT.try_with(|count| match kind {
        PropertyQueryKind::Write => count.borrow_mut().write += 1,
        PropertyQueryKind::CurrentRead => count.borrow_mut().current_read += 1,
        PropertyQueryKind::Hydration => count.borrow_mut().hydration += 1,
    });
    #[cfg(not(test))]
    let _ = kind;
}

#[inline]
fn record_trait_query(kind: TraitQueryKind) {
    #[cfg(test)]
    let _ = TRAIT_QUERY_COUNT.try_with(|count| match kind {
        TraitQueryKind::Write => count.borrow_mut().write += 1,
        TraitQueryKind::CurrentRead => count.borrow_mut().current_read += 1,
        TraitQueryKind::Hydration => count.borrow_mut().hydration += 1,
    });
    #[cfg(not(test))]
    let _ = kind;
}

#[inline]
fn record_spatial_read_query(kind: SpatialReadQueryKind) {
    #[cfg(test)]
    let _ = SPATIAL_READ_QUERY_COUNT.try_with(|count| {
        let mut count = count.borrow_mut();
        match kind {
            SpatialReadQueryKind::PlaceCandidate => count.place_candidate += 1,
            SpatialReadQueryKind::PlaceHydration => count.place_hydration += 1,
            SpatialReadQueryKind::ConnectionAnchor => count.connection_anchor += 1,
            SpatialReadQueryKind::ConnectionCandidate => count.connection_candidate += 1,
            SpatialReadQueryKind::ConnectionHydration => count.connection_hydration += 1,
            SpatialReadQueryKind::ConnectionGet => count.connection_get += 1,
            SpatialReadQueryKind::ConnectionCourse => count.connection_course += 1,
        }
    });
    #[cfg(not(test))]
    let _ = kind;
}

#[derive(Clone)]

pub struct World {
    pool: PgPool,
    chance: Arc<dyn investigation::chance::ChanceSource>,
}

impl World {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool,
            chance: Arc::new(investigation::chance::OsChance),
        }
    }

    #[cfg(test)]
    pub(crate) fn with_scripted_chance(pool: PgPool, draw: Vec<f64>) -> Self {
        Self {
            pool,
            chance: Arc::new(investigation::chance::ScriptedChance::new(draw)),
        }
    }

    pub fn get_world(&self) -> WorldView {
        WorldView {
            name: WORLD_NAME.to_owned(),
        }
    }

    async fn begin(
        &self,
        operation: &'static str,
    ) -> Result<Transaction<'_, Postgres>, WorldError> {
        self.pool
            .begin()
            .await
            .map_err(|error| storage_error(operation, error))
    }

    async fn begin_repeatable_read(
        &self,
        operation: &'static str,
    ) -> Result<Transaction<'_, Postgres>, WorldError> {
        let mut transaction = self.begin(operation).await?;
        sqlx::query("SET TRANSACTION ISOLATION LEVEL REPEATABLE READ READ ONLY")
            .execute(&mut *transaction)
            .await
            .map_err(|error| storage_error(operation, error))?;
        Ok(transaction)
    }

    async fn begin_spatial_read(
        &self,
        operation: &'static str,
    ) -> Result<Transaction<'_, Postgres>, WorldError> {
        let mut transaction = self.begin_repeatable_read(operation).await?;
        sqlx::query("SET LOCAL statement_timeout = '3s'")
            .execute(&mut *transaction)
            .await
            .map_err(|error| spatial_read_error(operation, error))?;
        Ok(transaction)
    }

    async fn begin_spatial_mutation(
        &self,
        operation: &'static str,
    ) -> Result<Transaction<'_, Postgres>, WorldError> {
        let mut transaction = self.begin(operation).await?;
        sqlx::query("SET LOCAL statement_timeout = '3s'")
            .execute(&mut *transaction)
            .await
            .map_err(|error| storage_error(operation, error))?;
        sqlx::query("SET LOCAL lock_timeout = '500ms'")
            .execute(&mut *transaction)
            .await
            .map_err(|error| storage_error(operation, error))?;
        Ok(transaction)
    }
}

#[cfg(test)]
mod spatial_read_query_count_test {
    use super::*;

    #[sqlx::test(migrations = "./migration")]
    async fn spatial_reads_use_one_fixed_query_per_selected_data_seam(pool: PgPool) {
        SPATIAL_READ_QUERY_COUNT
            .scope(RefCell::new(SpatialReadQueryCount::default()), async move {
                let world = World::new(pool.clone());
                let user = world.create_user().await.unwrap();
                let character = world
                    .create_character(
                        user.id,
                        CreateCharacter {
                            name: "Query Reader".to_owned(),
                            description: "Reads bounded spatial state.".to_owned(),
                            property: Vec::new(),
                            r#trait: Vec::new(),
                        },
                    )
                    .await
                    .unwrap();
                world
                    .create_entry_place(
                        user.id,
                        CreateEntryPlace {
                            name: "Query Origin".to_owned(),
                            description: "Origin for bounded query evidence.".to_owned(),
                            property: Vec::new(),
                            r#trait: Vec::new(),
                        },
                    )
                    .await
                    .unwrap();
                let entered = world.enter_world(user.id).await.unwrap();
                let source = entered.current_place.unwrap();

                world
                    .list_place(
                        user.id,
                        ListPlace {
                            min_x_cm: 0,
                            max_x_cm: 0,
                            min_y_cm: 0,
                            max_y_cm: 0,
                            min_z_cm: 0,
                            max_z_cm: 0,
                            cursor: None,
                            limit: 100,
                        },
                    )
                    .await
                    .unwrap();
                world
                    .list_connection(
                        user.id,
                        ListConnection {
                            place_id: source.entity.id,
                            cursor: None,
                            limit: 100,
                        },
                    )
                    .await
                    .unwrap();

                let destination_id = EntityId(Uuid::new_v4());
                let activity_id = ActivityId(Uuid::new_v4());
                let connection_id = ConnectionId(Uuid::new_v4());
                let mut transaction = pool.begin().await.unwrap();
                sqlx::query("INSERT INTO entity (id, name, description, introduced_by_user_id) VALUES ($1, 'Query Destination', 'Destination for query evidence.', $2)")
                    .bind(destination_id.0)
                    .bind(user.id.0)
                    .execute(&mut *transaction)
                    .await
                    .unwrap();
                sqlx::query(
                    r#"
                    INSERT INTO activity (
                        id, operation, requested_by_user_id,
                        actor_character_entity_id, context_place_entity_id,
                        prose, request_id, request_fingerprint
                    ) VALUES (
                        $1, 'submit_discovery', $2, $3, $4,
                        'A query destination and Connection are established.', $5, $6
                    )
                    "#,
                )
                .bind(activity_id.0)
                .bind(user.id.0)
                .bind(character.entity.id.0)
                .bind(source.entity.id.0)
                .bind(Uuid::new_v4())
                .bind(vec![12_u8; 32])
                .execute(&mut *transaction)
                .await
                .unwrap();
                sqlx::query("INSERT INTO activity_entity (activity_id, entity_id, role) VALUES ($1, $2, 'subject'), ($1, $2, 'destination'), ($1, $3, 'location')")
                    .bind(activity_id.0)
                    .bind(destination_id.0)
                    .bind(source.entity.id.0)
                    .execute(&mut *transaction)
                    .await
                    .unwrap();
                let destination = insert_root_position(
                    &mut transaction,
                    destination_id,
                    activity_id,
                    [100, 0, 0],
                    None,
                    "spatial_read_query_count_test",
                )
                .await
                .unwrap();
                sqlx::query("INSERT INTO place (entity_id, is_entry, latest_activity_id) VALUES ($1, false, $2)")
                    .bind(destination_id.0)
                    .bind(activity_id.0)
                    .execute(&mut *transaction)
                    .await
                    .unwrap();
                insert_place_map_projection(
                    &mut transaction,
                    destination_id,
                    &destination,
                    "spatial_read_query_count_test",
                )
                .await
                .unwrap();
                sqlx::query(
                    r#"
                    INSERT INTO connection (
                        id, source_place_entity_id, destination_place_entity_id,
                        source_position_activity_id, destination_position_activity_id,
                        allows_reverse, has_course, name, description,
                        created_by_activity_id
                    ) VALUES ($1, $2, $3, $4, $5, true, false,
                              'Query Connection', 'Unshaped query evidence.', $6)
                    "#,
                )
                .bind(connection_id.0)
                .bind(source.entity.id.0)
                .bind(destination_id.0)
                .bind(source.position.position_revision.activity_id().0)
                .bind(activity_id.0)
                .bind(activity_id.0)
                .execute(&mut *transaction)
                .await
                .unwrap();
                sqlx::query("INSERT INTO activity_connection (activity_id, connection_id) VALUES ($1, $2)")
                    .bind(activity_id.0)
                    .bind(connection_id.0)
                    .execute(&mut *transaction)
                    .await
                    .unwrap();
                transaction.commit().await.unwrap();

                world
                    .list_connection(
                        user.id,
                        ListConnection {
                            place_id: source.entity.id,
                            cursor: None,
                            limit: 100,
                        },
                    )
                    .await
                    .unwrap();
                world
                    .get_connection(
                        user.id,
                        GetConnection {
                            place_id: source.entity.id,
                            connection_id,
                        },
                    )
                    .await
                    .unwrap();
                assert_eq!(
                    SPATIAL_READ_QUERY_COUNT.with(|count| *count.borrow()),
                    SpatialReadQueryCount {
                        place_candidate: 1,
                        place_hydration: 1,
                        connection_anchor: 2,
                        connection_candidate: 2,
                        connection_hydration: 1,
                        connection_get: 1,
                        connection_course: 1,
                    }
                );
            })
            .await;
    }
}

#[cfg(test)]
mod property_query_count_test {
    use super::*;
    use sqlx::PgPool;

    fn reset_property_query_count() {
        PROPERTY_QUERY_COUNT.with(|count| *count.borrow_mut() = PropertyQueryCount::default());
    }

    fn property_query_count() -> PropertyQueryCount {
        PROPERTY_QUERY_COUNT.with(|count| *count.borrow())
    }

    fn changes(entity_id: EntityId, count: usize) -> Vec<EntityPropertyChangeInput> {
        (0..count)
            .map(|index| EntityPropertyChangeInput {
                entity_id,
                key: format!("measure_{index}"),
                value: PropertyValue::Integer(index as i64),
            })
            .collect()
    }

    async fn submit_change(
        world: &World,
        user_id: UserId,
        entity_id: EntityId,
        revision: PlaceRevision,
        count: usize,
    ) -> AcceptedAction {
        world
            .submit_action(
                user_id,
                SubmitAction {
                    request_id: Uuid::new_v4(),
                    expected_place_revision: revision,
                    prose: format!("Mara records {count} exact physical measures."),
                    consequence: ActionConsequence::ChangeEntityState(ChangeEntityState {
                        property_change: changes(entity_id, count),
                        trait_change: Vec::new(),
                    }),
                },
            )
            .await
            .expect("bounded Property Action should be accepted")
    }

    #[sqlx::test(migrations = "./migration")]
    async fn one_and_one_hundred_properties_use_constant_query_counts(pool: PgPool) {
        PROPERTY_QUERY_COUNT
            .scope(
                RefCell::new(PropertyQueryCount::default()),
                assert_one_and_one_hundred_properties_use_constant_query_counts(pool),
            )
            .await;
    }

    async fn assert_one_and_one_hundred_properties_use_constant_query_counts(pool: PgPool) {
        let world = World::new(pool);
        let user = world.create_user().await.unwrap();
        let character = world
            .create_character(
                user.id,
                CreateCharacter {
                    name: "Mara Venn".to_owned(),
                    description: "A careful surveyor.".to_owned(),
                    property: Vec::new(),
                    r#trait: Vec::new(),
                },
            )
            .await
            .unwrap();
        world
            .create_entry_place(
                user.id,
                CreateEntryPlace {
                    name: "North Gate".to_owned(),
                    description: "A wind-worn stone gate.".to_owned(),
                    property: Vec::new(),
                    r#trait: Vec::new(),
                },
            )
            .await
            .unwrap();
        world.enter_world(user.id).await.unwrap();
        let initial = world
            .get_character(user.id, GetEntityCurrentState::default())
            .await
            .unwrap();

        reset_property_query_count();
        submit_change(
            &world,
            user.id,
            character.entity.id,
            initial.place_revision.unwrap(),
            1,
        )
        .await;
        let one_write = property_query_count();

        reset_property_query_count();
        let one_current = world
            .get_character(
                user.id,
                GetEntityCurrentState {
                    cursor: None,
                    limit: 100,
                },
            )
            .await
            .unwrap();
        assert_eq!(one_current.current_state.association.len(), 1);
        let one_current_count = property_query_count();

        reset_property_query_count();
        let one_history = world
            .list_activity(
                user.id,
                ListActivity {
                    cursor: None,
                    limit: 1,
                },
            )
            .await
            .unwrap();
        assert_eq!(one_history.activity[0].property_change.len(), 1);
        let one_hydration = property_query_count();

        reset_property_query_count();
        submit_change(
            &world,
            user.id,
            character.entity.id,
            one_current.place_revision.unwrap(),
            100,
        )
        .await;
        let hundred_write = property_query_count();

        reset_property_query_count();
        let hundred_current = world
            .get_character(
                user.id,
                GetEntityCurrentState {
                    cursor: None,
                    limit: 100,
                },
            )
            .await
            .unwrap();
        assert_eq!(hundred_current.current_state.association.len(), 100);
        let hundred_current_count = property_query_count();

        reset_property_query_count();
        let hundred_history = world
            .list_activity(
                user.id,
                ListActivity {
                    cursor: None,
                    limit: 1,
                },
            )
            .await
            .unwrap();
        assert_eq!(hundred_history.activity[0].property_change.len(), 100);
        let hundred_hydration = property_query_count();

        assert_eq!(one_write, hundred_write);
        assert_eq!(
            one_write,
            PropertyQueryCount {
                write: 5,
                current_read: 0,
                hydration: 2,
            }
        );
        assert_eq!(one_current_count, hundred_current_count);
        assert_eq!(
            one_current_count,
            PropertyQueryCount {
                write: 0,
                current_read: 1,
                hydration: 0,
            }
        );
        assert_eq!(one_hydration, hundred_hydration);
        assert_eq!(
            one_hydration,
            PropertyQueryCount {
                write: 0,
                current_read: 0,
                hydration: 1,
            }
        );
    }

    #[sqlx::test(migrations = "./migration")]
    async fn live_property_queries_have_bound_set_based_plans(pool: PgPool) {
        let world = World::new(pool.clone());
        let user = world.create_user().await.unwrap();
        let character = world
            .create_character(
                user.id,
                CreateCharacter {
                    name: "Index Seer".to_owned(),
                    description: "A careful surveyor.".to_owned(),
                    property: Vec::new(),
                    r#trait: Vec::new(),
                },
            )
            .await
            .unwrap();
        let _place = world
            .create_entry_place(
                user.id,
                CreateEntryPlace {
                    name: "North Gate".to_owned(),
                    description: "A wind-worn stone gate.".to_owned(),
                    property: Vec::new(),
                    r#trait: Vec::new(),
                },
            )
            .await
            .unwrap();
        world.enter_world(user.id).await.unwrap();
        let revision = world
            .get_character(user.id, GetEntityCurrentState::default())
            .await
            .unwrap()
            .place_revision
            .unwrap();
        let accepted = submit_change(&world, user.id, character.entity.id, revision, 100).await;
        let activity_id = accepted.activity.id.0;
        let key = (0..100)
            .map(|index| format!("measure_{index}"))
            .collect::<Vec<_>>();
        let property_key_id: Vec<i64> = sqlx::query_scalar(
            "SELECT id FROM property_key WHERE key = ANY($1::text[]) ORDER BY key",
        )
        .bind(&key)
        .fetch_all(&pool)
        .await
        .unwrap();
        assert_eq!(property_key_id.len(), 100);
        sqlx::query("ANALYZE property_key, entity_property_history, entity_property")
            .execute(&pool)
            .await
            .unwrap();

        let mut transaction = pool.begin().await.unwrap();
        sqlx::query("SET LOCAL enable_seqscan = off")
            .execute(&mut *transaction)
            .await
            .unwrap();

        let hydration_explain = format!("EXPLAIN (COSTS OFF) {HYDRATE_ENTITY_PROPERTY_CHANGE_SQL}");
        let hydration_plan = sqlx::query_scalar::<_, String>(&hydration_explain)
            .bind(vec![activity_id])
            .fetch_all(&mut *transaction)
            .await
            .unwrap()
            .join("\n");
        assert!(
            hydration_plan.contains("entity_property_history_activity_index"),
            "the live bounded Activity hydration query must use its declared index: {hydration_plan}"
        );

        let current_explain = format!("EXPLAIN (COSTS OFF) {CURRENT_ENTITY_STATE_SQL}");
        let current_plan = sqlx::query_scalar::<_, String>(&current_explain)
            .bind(character.entity.id.0)
            .bind(Option::<i16>::None)
            .bind(Option::<i64>::None)
            .bind(Option::<Uuid>::None)
            .bind(101_i64)
            .fetch_all(&mut *transaction)
            .await
            .unwrap()
            .join("\n");
        assert!(
            current_plan.contains("entity_property_pkey")
                && current_plan.contains("entity_property_history_pkey")
                && current_plan.contains("entity_trait_current_entity_id_trait_id_index"),
            "the live combined current-read query must use Property and Trait current indexes: {current_plan}"
        );

        let value_type = vec!["integer"; 100];
        let key_insert_explain = format!("EXPLAIN (COSTS OFF) {INSERT_PROPERTY_KEY_SQL}");
        let key_insert_plan = sqlx::query_scalar::<_, String>(&key_insert_explain)
            .bind(&key)
            .bind(&value_type)
            .bind(activity_id)
            .fetch_all(&mut *transaction)
            .await
            .unwrap()
            .join("\n");
        assert!(
            key_insert_plan.contains("Conflict Arbiter Indexes: property_key_key_key")
                && key_insert_plan.contains("Function Scan on submitted"),
            "the live bulk key-arbitration write must use one UNNEST and its unique index: {key_insert_plan}"
        );

        let entity_id = vec![character.entity.id.0; 100];
        let pointer_lock_explain =
            format!("EXPLAIN (COSTS OFF) {LOCK_CURRENT_ENTITY_PROPERTY_SQL}");
        let pointer_lock_plan = sqlx::query_scalar::<_, String>(&pointer_lock_explain)
            .bind(&entity_id)
            .bind(&property_key_id)
            .fetch_all(&mut *transaction)
            .await
            .unwrap()
            .join("\n");
        assert!(
            pointer_lock_plan.contains("entity_property_pkey")
                && pointer_lock_plan.contains("Function Scan on submitted"),
            "the live pointer-lock query must use one submitted set and the current-state key: {pointer_lock_plan}"
        );

        let previous_activity_id = vec![Some(activity_id); 100];
        let text_value = vec![None::<String>; 100];
        let integer_value = (0..100).map(Some).collect::<Vec<_>>();
        let history_insert_explain =
            format!("EXPLAIN (COSTS OFF) {INSERT_ENTITY_PROPERTY_HISTORY_SQL}");
        let history_insert_plan = sqlx::query_scalar::<_, String>(&history_insert_explain)
            .bind(&entity_id)
            .bind(&property_key_id)
            .bind(Uuid::new_v4())
            .bind(&previous_activity_id)
            .bind(&value_type)
            .bind(&text_value)
            .bind(&integer_value)
            .fetch_all(&mut *transaction)
            .await
            .unwrap()
            .join("\n");
        assert!(
            history_insert_plan.contains("Function Scan on submitted")
                && history_insert_plan
                    .contains("Sort Key: submitted.entity_id, submitted.property_key_id"),
            "the live history write must remain one sorted set-based INSERT: {history_insert_plan}"
        );

        let pointer_upsert_explain =
            format!("EXPLAIN (COSTS OFF) {UPSERT_CURRENT_ENTITY_PROPERTY_SQL}");
        let pointer_upsert_plan = sqlx::query_scalar::<_, String>(&pointer_upsert_explain)
            .bind(&entity_id)
            .bind(&property_key_id)
            .bind(Uuid::new_v4())
            .fetch_all(&mut *transaction)
            .await
            .unwrap()
            .join("\n");
        assert!(
            pointer_upsert_plan.contains("Conflict Arbiter Indexes: entity_property_pkey")
                && pointer_upsert_plan.contains("Function Scan on submitted"),
            "the live current-pointer write must use one UNNEST and the pointer key: {pointer_upsert_plan}"
        );
        transaction.rollback().await.unwrap();
    }
}

#[cfg(test)]
mod trait_persistence_test {
    use super::*;
    use sqlx::PgPool;

    async fn append_trait_action(
        transaction: &mut Transaction<'_, Postgres>,
        user_id: UserId,
        involved: &[(EntityId, ActivityEntityRole)],
    ) -> ActivityId {
        let fingerprint = vec![8_u8; 32];
        append_activity(
            transaction,
            ActivityDraft {
                operation: ActivityOperation::SubmitAction,
                requested_by_user_id: user_id,
                actor_character_entity_id: None,
                context_place_entity_id: None,
                involved,
                prose: Some("A Trait changes."),
                request_id: Some(Uuid::new_v4()),
                request_fingerprint: Some(&fingerprint),
                action_consequence: Some("change_entity_state"),
            },
            "trait_test",
        )
        .await
        .unwrap()
    }

    fn establish(entity_id: EntityId, statement: impl Into<String>) -> TraitWrite {
        TraitWrite::Establish {
            entity_id,
            statement: statement.into(),
        }
    }

    fn develop(trait_id: Uuid, statement: impl Into<String>) -> TraitWrite {
        TraitWrite::Develop {
            trait_id,
            statement: statement.into(),
        }
    }

    #[test]
    fn trait_normalization_enforces_bounds_text_and_exact_input_duplicates() {
        let entity_id = EntityId(Uuid::new_v4());
        let trait_id = Uuid::new_v4();
        assert_eq!(
            normalize_trait_writes(Vec::new(), false),
            Err(TraitNormalizationError::InvalidCount)
        );
        assert_eq!(normalize_trait_writes(Vec::new(), true), Ok(Vec::new()));
        assert_eq!(
            normalize_trait_writes(vec![establish(entity_id, " \t ")], false),
            Err(TraitNormalizationError::InvalidStatement(
                InvalidReason::Empty
            ))
        );
        assert_eq!(
            normalize_trait_writes(vec![develop(trait_id, "has\0nul")], false),
            Err(TraitNormalizationError::InvalidStatement(
                InvalidReason::ContainsNul
            ))
        );
        assert_eq!(
            normalize_trait_writes(
                vec![establish(
                    entity_id,
                    "x".repeat(MAX_TRAIT_STATEMENT_LENGTH + 1)
                )],
                false,
            ),
            Err(TraitNormalizationError::InvalidStatement(
                InvalidReason::TooLong
            ))
        );
        assert_eq!(
            normalize_trait_writes(
                vec![
                    establish(entity_id, "Same statement."),
                    establish(entity_id, " Same statement. "),
                ],
                false,
            ),
            Err(TraitNormalizationError::DuplicateEstablishment)
        );
        assert_eq!(
            normalize_trait_writes(
                vec![develop(trait_id, "First."), develop(trait_id, "Second.")],
                false,
            ),
            Err(TraitNormalizationError::DuplicateDevelopment)
        );
        assert_eq!(
            normalize_trait_writes(
                (0..=MAX_TRAIT_COUNT)
                    .map(|index| establish(entity_id, format!("Statement {index}.")))
                    .collect(),
                false,
            ),
            Err(TraitNormalizationError::InvalidCount)
        );
        let normalized = normalize_trait_writes(
            vec![
                develop(trait_id, " Developed statement. "),
                establish(entity_id, " Established statement. "),
            ],
            false,
        )
        .unwrap();
        assert_eq!(normalized[0].statement(), "Established statement.");
        assert_eq!(normalized[1].statement(), "Developed statement.");
    }

    #[sqlx::test(migrations = "./migration")]
    async fn mixed_trait_writer_establishes_develops_hydrates_and_rejects_exact_noops(
        pool: PgPool,
    ) {
        let world = World::new(pool.clone());
        let user = world.create_user().await.unwrap();
        let first = world
            .create_entity(
                user.id,
                CreateEntity {
                    name: "First Trait Writer Subject".to_owned(),
                    description: "A first subject.".to_owned(),
                    property: Vec::new(),
                    r#trait: Vec::new(),
                },
            )
            .await
            .unwrap();
        let second = world
            .create_entity(
                user.id,
                CreateEntity {
                    name: "Second Trait Writer Subject".to_owned(),
                    description: "A second subject.".to_owned(),
                    property: Vec::new(),
                    r#trait: Vec::new(),
                },
            )
            .await
            .unwrap();
        let eligible = [first.id, second.id];

        let mut transaction = pool.begin().await.unwrap();
        let establish_activity_id = append_trait_action(
            &mut transaction,
            user.id,
            &[
                (first.id, ActivityEntityRole::Subject),
                (second.id, ActivityEntityRole::Subject),
            ],
        )
        .await;
        let initial = normalize_trait_writes(
            vec![
                establish(first.id, "Startles at every hard sound."),
                establish(second.id, "Keeps watch beside the gate."),
            ],
            false,
        )
        .unwrap();
        let established =
            write_trait_changes(&mut transaction, establish_activity_id, &initial, &eligible)
                .await
                .unwrap();
        transaction.commit().await.unwrap();
        assert_eq!(established.len(), 2);
        assert!(
            established
                .iter()
                .all(|change| change.lifecycle == StoredTraitLifecycle::Establish)
        );
        let first_trait_id = established
            .iter()
            .find(|change| change.entity_id == first.id)
            .unwrap()
            .trait_id;

        let mut transaction = pool.begin().await.unwrap();
        let mixed_activity_id = append_trait_action(
            &mut transaction,
            user.id,
            &[
                (first.id, ActivityEntityRole::Subject),
                (second.id, ActivityEntityRole::Subject),
            ],
        )
        .await;
        let mixed = normalize_trait_writes(
            vec![
                establish(second.id, "Listens for footsteps beyond the arch."),
                develop(
                    first_trait_id,
                    "Waits for the second echo before springing.",
                ),
            ],
            false,
        )
        .unwrap();
        let mixed_change =
            write_trait_changes(&mut transaction, mixed_activity_id, &mixed, &eligible)
                .await
                .unwrap();
        transaction.commit().await.unwrap();
        assert_eq!(mixed_change.len(), 2);
        let developed = mixed_change
            .iter()
            .find(|change| change.lifecycle == StoredTraitLifecycle::Develop)
            .unwrap();
        assert_eq!(developed.trait_id, first_trait_id);
        assert_eq!(
            developed.previous_statement.as_deref(),
            Some("Startles at every hard sound.")
        );
        assert_eq!(
            developed.statement,
            "Waits for the second echo before springing."
        );

        for invalid in [
            establish(first.id, "Waits for the second echo before springing."),
            develop(
                first_trait_id,
                "Waits for the second echo before springing.",
            ),
        ] {
            let mut transaction = pool.begin().await.unwrap();
            let invalid_activity_id = append_trait_action(
                &mut transaction,
                user.id,
                &[(first.id, ActivityEntityRole::Subject)],
            )
            .await;
            let invalid = normalize_trait_writes(vec![invalid], false).unwrap();
            assert!(matches!(
                write_trait_changes(&mut transaction, invalid_activity_id, &invalid, &eligible,)
                    .await,
                Err(TraitPersistenceError::InvalidInput)
            ));
            transaction.rollback().await.unwrap();
        }
        let activity_count: i64 = sqlx::query_scalar(
            "SELECT count(*) FROM activity WHERE action_consequence = 'change_entity_state'",
        )
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_eq!(activity_count, 2);
        let lineage: Vec<(Uuid, Option<Uuid>, String)> = sqlx::query_as(
            r#"
            SELECT activity_id, previous_activity_id, statement
            FROM entity_trait_version
            WHERE trait_id = $1
            ORDER BY activity_id
            "#,
        )
        .bind(first_trait_id)
        .fetch_all(&pool)
        .await
        .unwrap();
        assert_eq!(lineage.len(), 2);
        assert!(lineage.iter().any(|(_, previous, statement)| {
            *previous == Some(establish_activity_id.0)
                && statement == "Waits for the second echo before springing."
        }));
    }

    #[sqlx::test(migrations = "./migration")]
    async fn one_and_one_hundred_trait_establishments_use_constant_query_counts(pool: PgPool) {
        TRAIT_QUERY_COUNT
            .scope(
                RefCell::new(TraitQueryCount::default()),
                assert_constant_trait_query_counts(pool),
            )
            .await;
    }

    async fn assert_constant_trait_query_counts(pool: PgPool) {
        let world = World::new(pool.clone());
        let user = world.create_user().await.unwrap();
        let entity = world
            .create_entity(
                user.id,
                CreateEntity {
                    name: "Bounded Trait Subject".to_owned(),
                    description: "A subject with many bounded Traits.".to_owned(),
                    property: Vec::new(),
                    r#trait: Vec::new(),
                },
            )
            .await
            .unwrap();

        async fn write_counted(
            pool: &PgPool,
            user_id: UserId,
            entity_id: EntityId,
            count: usize,
        ) -> (TraitQueryCount, Vec<StoredTraitChange>) {
            let mut transaction = pool.begin().await.unwrap();
            let activity_id = append_trait_action(
                &mut transaction,
                user_id,
                &[(entity_id, ActivityEntityRole::Subject)],
            )
            .await;
            let writes = normalize_trait_writes(
                (0..count)
                    .map(|index| establish(entity_id, format!("Batch {count} statement {index}.")))
                    .collect(),
                false,
            )
            .unwrap();
            TRAIT_QUERY_COUNT.with(|queries| *queries.borrow_mut() = TraitQueryCount::default());
            let stored = write_trait_changes(&mut transaction, activity_id, &writes, &[entity_id])
                .await
                .unwrap();
            let count = TRAIT_QUERY_COUNT.with(|queries| *queries.borrow());
            transaction.commit().await.unwrap();
            (count, stored)
        }

        let (one_count, one) = write_counted(&pool, user.id, entity.id, 1).await;
        let (hundred_count, hundred) = write_counted(&pool, user.id, entity.id, 100).await;
        assert_eq!(one.len(), 1);
        assert_eq!(hundred.len(), 100);
        assert_eq!(one_count, hundred_count);
        assert_eq!(
            one_count,
            TraitQueryCount {
                write: 5,
                current_read: 0,
                hydration: 1,
            }
        );
    }

    #[sqlx::test(migrations = "./migration")]
    async fn live_trait_duplicate_query_is_set_bounded_and_indexed(pool: PgPool) {
        let world = World::new(pool.clone());
        let user = world.create_user().await.unwrap();
        let entity = world
            .create_entity(
                user.id,
                CreateEntity {
                    name: "Indexed Active Trait Subject".to_owned(),
                    description: "Carries one hundred active indexed Traits.".to_owned(),
                    property: Vec::new(),
                    r#trait: Vec::new(),
                },
            )
            .await
            .unwrap();
        let mut transaction = pool.begin().await.unwrap();
        let activity_id = append_trait_action(
            &mut transaction,
            user.id,
            &[(entity.id, ActivityEntityRole::Subject)],
        )
        .await;
        let writes = normalize_trait_writes(
            (0..100)
                .map(|index| establish(entity.id, format!("Indexed active statement {index}.")))
                .collect(),
            false,
        )
        .unwrap();
        write_trait_changes(&mut transaction, activity_id, &writes, &[entity.id])
            .await
            .unwrap();
        transaction.commit().await.unwrap();

        sqlx::query("ANALYZE entity_trait_version, entity_trait_current")
            .execute(&pool)
            .await
            .unwrap();
        let proposed_entity_id = vec![entity.id.0; 100];
        let proposed_statement = (0..100)
            .map(|index| format!("Indexed active statement {index}."))
            .collect::<Vec<_>>();
        let mut transaction = pool.begin().await.unwrap();
        sqlx::query("SET LOCAL enable_seqscan = off")
            .execute(&mut *transaction)
            .await
            .unwrap();
        let explain = format!("EXPLAIN (COSTS OFF) {LOCK_ACTIVE_ENTITY_TRAIT_STATEMENT_SQL}");
        let plan = sqlx::query_scalar::<_, String>(&explain)
            .bind(&proposed_entity_id)
            .bind(&proposed_statement)
            .bind(Vec::<Uuid>::new())
            .fetch_all(&mut *transaction)
            .await
            .unwrap()
            .join("\n");
        assert!(
            plan.contains("Function Scan on submitted")
                && plan.contains("entity_trait_current_entity_id_trait_id_index")
                && plan.contains("entity_trait_version_activity_entity_trait_index")
                && plan.contains("Limit"),
            "the live active-duplicate query must stay one bounded submitted set on declared indexes: {plan}"
        );
        transaction.rollback().await.unwrap();
    }

    #[sqlx::test(migrations = "./migration")]
    async fn public_trait_world_paths_are_set_bounded_for_one_and_one_hundred(pool: PgPool) {
        TRAIT_QUERY_COUNT
            .scope(
                RefCell::new(TraitQueryCount::default()),
                assert_public_trait_world_paths_are_set_bounded(pool),
            )
            .await;
    }

    async fn assert_public_trait_world_paths_are_set_bounded(pool: PgPool) {
        let world = World::new(pool);
        let user = world.create_user().await.unwrap();
        let character = world
            .create_character(
                user.id,
                CreateCharacter {
                    name: "Bounded Trait Character".to_owned(),
                    description: "Carries one or one hundred Traits.".to_owned(),
                    property: Vec::new(),
                    r#trait: Vec::new(),
                },
            )
            .await
            .unwrap();
        world
            .create_entry_place(
                user.id,
                CreateEntryPlace {
                    name: "Bounded Trait Place".to_owned(),
                    description: "One exact current Place.".to_owned(),
                    property: Vec::new(),
                    r#trait: Vec::new(),
                },
            )
            .await
            .unwrap();
        world.enter_world(user.id).await.unwrap();

        async fn submit_counted(
            world: &World,
            user_id: UserId,
            entity_id: EntityId,
            count: usize,
        ) -> TraitQueryCount {
            let revision = world
                .list_entity_at_current_place(user_id, ListEntityAtCurrentPlace::default())
                .await
                .unwrap()
                .place_revision;
            TRAIT_QUERY_COUNT.with(|queries| *queries.borrow_mut() = TraitQueryCount::default());
            world
                .submit_action(
                    user_id,
                    SubmitAction {
                        request_id: Uuid::new_v4(),
                        expected_place_revision: revision,
                        prose: format!("The World accepts {count} bounded Traits."),
                        consequence: ActionConsequence::ChangeEntityState(ChangeEntityState {
                            property_change: Vec::new(),
                            trait_change: (0..count)
                                .map(|index| EntityTraitChangeInput::Establish {
                                    entity_id,
                                    statement: format!("Public batch {count} statement {index}."),
                                })
                                .collect(),
                        }),
                    },
                )
                .await
                .unwrap();
            TRAIT_QUERY_COUNT.with(|queries| *queries.borrow())
        }

        let one_write = submit_counted(&world, user.id, character.entity.id, 1).await;
        TRAIT_QUERY_COUNT.with(|queries| *queries.borrow_mut() = TraitQueryCount::default());
        let one_current = world
            .get_character(
                user.id,
                GetEntityCurrentState {
                    cursor: None,
                    limit: 100,
                },
            )
            .await
            .unwrap();
        assert_eq!(one_current.current_state.association.len(), 1);
        let one_current_count = TRAIT_QUERY_COUNT.with(|queries| *queries.borrow());
        TRAIT_QUERY_COUNT.with(|queries| *queries.borrow_mut() = TraitQueryCount::default());
        let one_history = world
            .list_activity(
                user.id,
                ListActivity {
                    cursor: None,
                    limit: 1,
                },
            )
            .await
            .unwrap();
        assert_eq!(one_history.activity[0].trait_change.len(), 1);
        let one_history_count = TRAIT_QUERY_COUNT.with(|queries| *queries.borrow());

        let hundred_write = submit_counted(&world, user.id, character.entity.id, 100).await;
        TRAIT_QUERY_COUNT.with(|queries| *queries.borrow_mut() = TraitQueryCount::default());
        let hundred_current = world
            .get_character(
                user.id,
                GetEntityCurrentState {
                    cursor: None,
                    limit: 100,
                },
            )
            .await
            .unwrap();
        assert_eq!(hundred_current.current_state.association.len(), 100);
        let hundred_current_count = TRAIT_QUERY_COUNT.with(|queries| *queries.borrow());
        TRAIT_QUERY_COUNT.with(|queries| *queries.borrow_mut() = TraitQueryCount::default());
        let hundred_history = world
            .list_activity(
                user.id,
                ListActivity {
                    cursor: None,
                    limit: 1,
                },
            )
            .await
            .unwrap();
        assert_eq!(hundred_history.activity[0].trait_change.len(), 100);
        let hundred_history_count = TRAIT_QUERY_COUNT.with(|queries| *queries.borrow());

        assert_eq!(one_write, hundred_write);
        assert_eq!(
            one_write,
            TraitQueryCount {
                write: 5,
                current_read: 0,
                hydration: 2,
            }
        );
        assert_eq!(one_current_count, hundred_current_count);
        assert_eq!(
            one_current_count,
            TraitQueryCount {
                write: 0,
                current_read: 1,
                hydration: 0,
            }
        );
        assert_eq!(one_history_count, hundred_history_count);
        assert_eq!(
            one_history_count,
            TraitQueryCount {
                write: 0,
                current_read: 0,
                hydration: 1,
            }
        );
    }

    #[sqlx::test(migrations = "./migration")]
    async fn reversed_multi_trait_writes_complete_without_deadlock(pool: PgPool) {
        let world = World::new(pool.clone());
        let user = world.create_user().await.unwrap();
        let first = world
            .create_entity(
                user.id,
                CreateEntity {
                    name: "First Lock Subject".to_owned(),
                    description: "First lock-order subject.".to_owned(),
                    property: Vec::new(),
                    r#trait: Vec::new(),
                },
            )
            .await
            .unwrap();
        let second = world
            .create_entity(
                user.id,
                CreateEntity {
                    name: "Second Lock Subject".to_owned(),
                    description: "Second lock-order subject.".to_owned(),
                    property: Vec::new(),
                    r#trait: Vec::new(),
                },
            )
            .await
            .unwrap();
        let eligible = vec![first.id, second.id];
        let mut transaction = pool.begin().await.unwrap();
        let activity_id = append_trait_action(
            &mut transaction,
            user.id,
            &[
                (first.id, ActivityEntityRole::Subject),
                (second.id, ActivityEntityRole::Subject),
            ],
        )
        .await;
        let roots = normalize_trait_writes(
            vec![
                establish(first.id, "First root statement."),
                establish(second.id, "Second root statement."),
            ],
            false,
        )
        .unwrap();
        let established = write_trait_changes(&mut transaction, activity_id, &roots, &eligible)
            .await
            .unwrap();
        transaction.commit().await.unwrap();
        let first_trait_id = established
            .iter()
            .find(|change| change.entity_id == first.id)
            .unwrap()
            .trait_id;
        let second_trait_id = established
            .iter()
            .find(|change| change.entity_id == second.id)
            .unwrap()
            .trait_id;

        let run = |pool: PgPool,
                   user_id: UserId,
                   first_statement: &'static str,
                   second_statement: &'static str,
                   reverse: bool| {
            let eligible = eligible.clone();
            async move {
                let mut transaction = pool.begin().await.unwrap();
                let activity_id = append_trait_action(
                    &mut transaction,
                    user_id,
                    &[
                        (first.id, ActivityEntityRole::Subject),
                        (second.id, ActivityEntityRole::Subject),
                    ],
                )
                .await;
                let mut writes = vec![
                    develop(first_trait_id, first_statement),
                    develop(second_trait_id, second_statement),
                ];
                if reverse {
                    writes.reverse();
                }
                let writes = normalize_trait_writes(writes, false).unwrap();
                write_trait_changes(&mut transaction, activity_id, &writes, &eligible)
                    .await
                    .unwrap();
                transaction.commit().await.unwrap();
            }
        };
        let first_write = tokio::spawn(run(
            pool.clone(),
            user.id,
            "First concurrent statement A.",
            "Second concurrent statement A.",
            false,
        ));
        let second_write = tokio::spawn(run(
            pool.clone(),
            user.id,
            "First concurrent statement B.",
            "Second concurrent statement B.",
            true,
        ));
        tokio::time::timeout(std::time::Duration::from_secs(10), async {
            first_write.await.unwrap();
            second_write.await.unwrap();
        })
        .await
        .expect("stable Entity/Trait lock order must not deadlock");

        let versions: Vec<(Uuid, i64)> = sqlx::query_as(
            r#"
            SELECT trait_id, count(*)
            FROM entity_trait_version
            WHERE trait_id = ANY($1::uuid[])
            GROUP BY trait_id
            ORDER BY trait_id
            "#,
        )
        .bind(vec![first_trait_id, second_trait_id])
        .fetch_all(&pool)
        .await
        .unwrap();
        assert_eq!(versions.len(), 2);
        assert!(versions.iter().all(|(_, count)| *count == 3));
    }
}
