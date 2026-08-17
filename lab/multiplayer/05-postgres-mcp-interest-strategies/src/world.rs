use std::collections::HashSet;
use std::str::FromStr;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use anyhow::{Context, Result, anyhow, bail};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{FromRow, PgPool};
use tokio::time::timeout;
use uuid::Uuid;

use crate::gateway::ResourceUri;
use crate::mcp::{AuthoritativeResource, AuthoritativeResourceReader};
use crate::strategy::{ChangeScope, ResourceKey, Strategy};

pub const NOTIFY_CHANNEL: &str = "aicadia_lab_resource_updated";
pub const MAX_ACTIVITY_ROWS: usize = 256;
pub const MAX_OPERATION_BYTES: usize = 128;
pub const MAX_RESOURCE_NAME_BYTES: usize = 128;
pub const MAX_RESOURCE_STATE_BYTES: usize = 4_096;
pub const MAX_RESOURCE_RESPONSE_BYTES: usize = 2 * 1_024 * 1_024;
pub const NOTIFY_TIMEOUT: std::time::Duration = std::time::Duration::from_millis(100);
const MAX_MUTATIONS: usize = 16;
const MAX_AFFECTED_PLACES: usize = 32;
const MAX_CHILDREN: usize = 128;

static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migration");

#[derive(Clone)]
pub struct World {
    pool: PgPool,
    notification_pool: PgPool,
    metrics: Arc<WorldMetrics>,
}

impl World {
    pub fn new(pool: PgPool) -> Self {
        Self {
            notification_pool: pool.clone(),
            pool,
            metrics: Arc::new(WorldMetrics::default()),
        }
    }

    #[cfg(test)]
    fn with_notification_pool(pool: PgPool, notification_pool: PgPool) -> Self {
        Self {
            pool,
            notification_pool,
            metrics: Arc::new(WorldMetrics::default()),
        }
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    pub async fn migrate(&self) -> Result<()> {
        MIGRATOR.run(&self.pool).await?;
        Ok(())
    }

    /// Clears only this disposable lab schema's rows.
    pub async fn reset(&self) -> Result<()> {
        sqlx::query("TRUNCATE TABLE activity_resource, activity, resource")
            .execute(&self.pool)
            .await?;
        self.reset_metrics();
        Ok(())
    }

    /// Seeds fixture current state without creating gameplay Activity.
    pub async fn seed_resources(&self, resources: &[SeedResource]) -> Result<()> {
        if resources.is_empty() {
            bail!("seed must contain at least the World resource");
        }
        let mut resources = resources.to_vec();
        resources.sort_unstable_by_key(|resource| resource.key);
        validate_unique_keys(resources.iter().map(|resource| resource.key))?;

        let mut transaction = self.pool.begin().await?;
        for resource in resources {
            validate_resource_shape(resource.key, resource.parent)?;
            validate_resource_content(&resource.name, &resource.state)?;
            let columns = ResourceColumns::from_key(resource.key);
            sqlx::query(
                r#"
                INSERT INTO resource (
                    uri, kind, parent_uri, area_id, place_id, entity_id,
                    name, state, version, updated_activity_id
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8::jsonb, $9, NULL)
                ON CONFLICT (uri) DO UPDATE SET
                    parent_uri = EXCLUDED.parent_uri,
                    name = EXCLUDED.name,
                    state = EXCLUDED.state,
                    version = EXCLUDED.version,
                    updated_activity_id = NULL
                "#,
            )
            .bind(resource.key.uri())
            .bind(resource.key.kind())
            .bind(resource.parent.map(ResourceKey::uri))
            .bind(columns.area_id)
            .bind(columns.place_id)
            .bind(columns.entity_id)
            .bind(resource.name)
            .bind(resource.state.to_string())
            .bind(Uuid::new_v4())
            .execute(&mut *transaction)
            .await
            .with_context(|| format!("seed resource {}", resource.key))?;
        }
        transaction.commit().await?;
        Ok(())
    }

    /// Commits current state plus exactly one Activity, then attempts a lossy hint.
    /// A failed post-commit NOTIFY never changes the accepted World result.
    pub async fn apply_change(
        &self,
        strategy: Strategy,
        request: ChangeRequest,
    ) -> Result<CommittedChange> {
        validate_change(&request)?;

        let mut affected_place_ids = request.affected_place_ids.clone();
        affected_place_ids.sort_unstable();
        affected_place_ids.dedup();
        let mut mutations = request.mutations.clone();
        mutations.sort_unstable_by_key(|mutation| mutation.key);
        validate_unique_keys(mutations.iter().map(|mutation| mutation.key))?;

        let changed_entity_ids = mutations
            .iter()
            .filter_map(|mutation| match mutation.key {
                ResourceKey::Entity(id) => Some(id),
                _ => None,
            })
            .collect::<Vec<_>>();
        let resource_uris = mutations
            .iter()
            .map(|mutation| mutation.key.uri())
            .collect::<Vec<_>>();
        let scope_area_id = match request.scope {
            ChangeScope::Area { area_id } => Some(area_id),
            ChangeScope::Local | ChangeScope::World => None,
        };

        let mut transaction = self.pool.begin().await?;
        validate_references(
            &mut transaction,
            &request,
            &affected_place_ids,
            &mutations,
            &self.metrics,
        )
        .await?;

        self.metrics.record_statement();
        let recorded_at: DateTime<Utc> = sqlx::query_scalar(
            r#"
            INSERT INTO activity (
                id, operation, scope_kind, scope_area_id,
                primary_entity_id, primary_place_id,
                affected_place_ids, resource_uris
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING recorded_at
            "#,
        )
        .bind(request.change_id)
        .bind(&request.operation)
        .bind(scope_name(request.scope))
        .bind(scope_area_id)
        .bind(request.primary_entity_id)
        .bind(request.primary_place_id)
        .bind(&affected_place_ids)
        .bind(&resource_uris)
        .fetch_one(&mut *transaction)
        .await
        .context("insert exactly one Activity")?;

        let mut resource_versions = Vec::with_capacity(mutations.len());
        for mutation in mutations {
            validate_resource_shape(mutation.key, mutation.parent)?;
            let columns = ResourceColumns::from_key(mutation.key);
            let version = Uuid::new_v4();
            self.metrics.record_statement();
            let updated = sqlx::query(
                r#"
                INSERT INTO resource (
                    uri, kind, parent_uri, area_id, place_id, entity_id,
                    name, state, version, updated_activity_id
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8::jsonb, $9, $10)
                ON CONFLICT (uri) DO UPDATE SET
                    parent_uri = EXCLUDED.parent_uri,
                    name = EXCLUDED.name,
                    state = EXCLUDED.state,
                    version = EXCLUDED.version,
                    updated_activity_id = EXCLUDED.updated_activity_id
                WHERE resource.kind = EXCLUDED.kind
                    AND resource.area_id IS NOT DISTINCT FROM EXCLUDED.area_id
                    AND resource.place_id IS NOT DISTINCT FROM EXCLUDED.place_id
                    AND resource.entity_id IS NOT DISTINCT FROM EXCLUDED.entity_id
                RETURNING uri
                "#,
            )
            .bind(mutation.key.uri())
            .bind(mutation.key.kind())
            .bind(mutation.parent.map(ResourceKey::uri))
            .bind(columns.area_id)
            .bind(columns.place_id)
            .bind(columns.entity_id)
            .bind(mutation.name)
            .bind(mutation.state.to_string())
            .bind(version)
            .bind(request.change_id)
            .fetch_optional(&mut *transaction)
            .await
            .with_context(|| format!("write current resource {}", mutation.key))?;
            if updated.is_none() {
                bail!(
                    "resource identity does not match its stable URI: {}",
                    mutation.key
                );
            }
            resource_versions.push(ResourceVersion {
                resource: mutation.key,
                version,
            });
        }

        let activity_resource_uri =
            activity_resource_keys(request.scope, &affected_place_ids, &resource_versions)
                .into_iter()
                .map(ResourceKey::uri)
                .collect::<Vec<_>>();
        self.metrics.record_statement();
        sqlx::query(
            r#"
            INSERT INTO activity_resource (activity_id, resource_uri, recorded_at)
            SELECT $1, resource_uri, $3
            FROM UNNEST($2::text[]) AS submitted(resource_uri)
            "#,
        )
        .bind(request.change_id)
        .bind(&activity_resource_uri)
        .bind(recorded_at)
        .execute(&mut *transaction)
        .await
        .context("index bounded Activity resource views")?;

        transaction.commit().await.context("commit World change")?;
        self.metrics.mutations.fetch_add(1, Ordering::Relaxed);

        let committed = CommittedChange {
            change_id: request.change_id,
            scope: request.scope,
            primary_entity_id: request.primary_entity_id,
            primary_place_id: request.primary_place_id,
            affected_place_ids,
            changed_entity_ids,
            resource_versions,
        };
        let payload = NotificationPayload {
            resources: strategy
                .resources_for_change(&committed)
                .into_iter()
                .map(ResourceKey::uri)
                .collect(),
            committed_at_unix_micros: Some(Utc::now().timestamp_micros()),
        };
        if payload.resources.is_empty() {
            return Ok(committed);
        }
        let encoded = serde_json::to_string(&payload)?;
        self.metrics
            .notification_payload_bytes
            .fetch_add(encoded.len() as u64, Ordering::Relaxed);
        self.metrics
            .notification_attempts
            .fetch_add(1, Ordering::Relaxed);
        self.metrics.record_statement();
        match timeout(
            NOTIFY_TIMEOUT,
            sqlx::query("SELECT pg_notify($1, $2)")
                .bind(NOTIFY_CHANNEL)
                .bind(encoded)
                .execute(&self.notification_pool),
        )
        .await
        {
            Ok(Ok(_)) => {
                self.metrics
                    .notifications_sent
                    .fetch_add(1, Ordering::Relaxed);
            }
            Ok(Err(_)) | Err(_) => {
                self.metrics
                    .notification_failures
                    .fetch_add(1, Ordering::Relaxed);
            }
        }
        Ok(committed)
    }

    pub async fn read_resource(
        &self,
        key: ResourceKey,
        activity_limit: usize,
    ) -> Result<ResourceSnapshot> {
        if activity_limit == 0 || activity_limit > MAX_ACTIVITY_ROWS {
            bail!("activity_limit must be within 1..={MAX_ACTIVITY_ROWS}");
        }
        self.metrics.reads.fetch_add(1, Ordering::Relaxed);

        let current = if key == ResourceKey::Global {
            None
        } else {
            self.metrics.record_statement();
            Some(fetch_resource(&self.pool, key).await?)
        };
        let mut children = match key {
            ResourceKey::Global | ResourceKey::Entity(_) => Vec::new(),
            ResourceKey::World | ResourceKey::Area(_) | ResourceKey::Place(_) => {
                self.metrics.record_statement();
                fetch_children(&self.pool, key).await?
            }
        };
        let children_truncated = children.len() > MAX_CHILDREN;
        children.truncate(MAX_CHILDREN);
        self.metrics.record_statement();
        let recent_activities = fetch_activities(&self.pool, key, activity_limit as i64).await?;
        let rows = current.iter().count() + children.len() + recent_activities.len();
        let snapshot = ResourceSnapshot {
            resource: key,
            current,
            children,
            children_truncated,
            recent_activities,
        };
        let response_bytes = serde_json::to_vec(&snapshot)?.len();
        if response_bytes > MAX_RESOURCE_RESPONSE_BYTES {
            bail!(
                "resource response exceeds {MAX_RESOURCE_RESPONSE_BYTES} bytes: {response_bytes}"
            );
        }
        self.metrics
            .rows_read
            .fetch_add(rows as u64, Ordering::Relaxed);
        Ok(snapshot)
    }

    pub async fn read_resource_uri(
        &self,
        uri: &str,
        activity_limit: usize,
    ) -> Result<ResourceSnapshot> {
        self.read_resource(ResourceKey::from_str(uri)?, activity_limit)
            .await
    }

    pub fn metrics(&self) -> WorldMetricsSnapshot {
        WorldMetricsSnapshot {
            mutations: self.metrics.mutations.load(Ordering::Relaxed),
            reads: self.metrics.reads.load(Ordering::Relaxed),
            rows_read: self.metrics.rows_read.load(Ordering::Relaxed),
            database_statements: self.metrics.database_statements.load(Ordering::Relaxed),
            notification_attempts: self.metrics.notification_attempts.load(Ordering::Relaxed),
            notifications_sent: self.metrics.notifications_sent.load(Ordering::Relaxed),
            notification_failures: self.metrics.notification_failures.load(Ordering::Relaxed),
            notification_payload_bytes: self
                .metrics
                .notification_payload_bytes
                .load(Ordering::Relaxed),
        }
    }

    pub fn reset_metrics(&self) {
        self.metrics.mutations.store(0, Ordering::Relaxed);
        self.metrics.reads.store(0, Ordering::Relaxed);
        self.metrics.rows_read.store(0, Ordering::Relaxed);
        self.metrics.database_statements.store(0, Ordering::Relaxed);
        self.metrics
            .notification_attempts
            .store(0, Ordering::Relaxed);
        self.metrics.notifications_sent.store(0, Ordering::Relaxed);
        self.metrics
            .notification_failures
            .store(0, Ordering::Relaxed);
        self.metrics
            .notification_payload_bytes
            .store(0, Ordering::Relaxed);
    }
}

impl AuthoritativeResourceReader for World {
    fn accepts_resource(&self, resource: &ResourceUri) -> bool {
        ResourceKey::from_str(resource.as_str()).is_ok()
    }

    async fn read_resource(
        &self,
        resource: &ResourceUri,
        activity_limit: usize,
    ) -> Result<AuthoritativeResource> {
        let snapshot = self
            .read_resource_uri(resource.as_str(), activity_limit)
            .await?;
        let rows = snapshot.row_count() as u64;
        AuthoritativeResource::new(snapshot, rows)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SeedResource {
    pub key: ResourceKey,
    pub parent: Option<ResourceKey>,
    pub name: String,
    pub state: Value,
}

impl SeedResource {
    pub fn new(
        key: ResourceKey,
        parent: Option<ResourceKey>,
        name: impl Into<String>,
        state: Value,
    ) -> Self {
        Self {
            key,
            parent,
            name: name.into(),
            state,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResourceMutation {
    pub key: ResourceKey,
    pub parent: Option<ResourceKey>,
    pub name: String,
    pub state: Value,
}

impl ResourceMutation {
    pub fn new(
        key: ResourceKey,
        parent: Option<ResourceKey>,
        name: impl Into<String>,
        state: Value,
    ) -> Self {
        Self {
            key,
            parent,
            name: name.into(),
            state,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChangeRequest {
    pub change_id: Uuid,
    pub operation: String,
    pub scope: ChangeScope,
    pub primary_entity_id: Option<Uuid>,
    pub primary_place_id: Option<Uuid>,
    pub affected_place_ids: Vec<Uuid>,
    pub mutations: Vec<ResourceMutation>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CommittedChange {
    pub change_id: Uuid,
    pub scope: ChangeScope,
    pub primary_entity_id: Option<Uuid>,
    pub primary_place_id: Option<Uuid>,
    pub affected_place_ids: Vec<Uuid>,
    pub changed_entity_ids: Vec<Uuid>,
    pub resource_versions: Vec<ResourceVersion>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ResourceVersion {
    pub resource: ResourceKey,
    pub version: Uuid,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResourceSnapshot {
    pub resource: ResourceKey,
    pub current: Option<ResourceState>,
    pub children: Vec<ResourceState>,
    /// True when more current children exist than this bounded snapshot returns.
    pub children_truncated: bool,
    pub recent_activities: Vec<Activity>,
}

impl ResourceSnapshot {
    pub fn row_count(&self) -> usize {
        self.current.iter().count() + self.children.len() + self.recent_activities.len()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResourceState {
    pub resource: ResourceKey,
    pub parent: Option<ResourceKey>,
    pub name: String,
    pub state: Value,
    pub version: Uuid,
    pub updated_activity_id: Option<Uuid>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Activity {
    pub id: Uuid,
    pub operation: String,
    pub scope: ChangeScope,
    pub primary_entity_id: Option<Uuid>,
    pub primary_place_id: Option<Uuid>,
    pub affected_place_ids: Vec<Uuid>,
    pub resource_uris: Vec<String>,
    pub recorded_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NotificationPayload {
    pub resources: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub committed_at_unix_micros: Option<i64>,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct WorldMetricsSnapshot {
    pub mutations: u64,
    pub reads: u64,
    pub rows_read: u64,
    /// Attempted SQL statements in apply/read/notify paths. This excludes
    /// BEGIN/COMMIT, migrations, reset and fixture seeding.
    pub database_statements: u64,
    pub notification_attempts: u64,
    pub notifications_sent: u64,
    pub notification_failures: u64,
    pub notification_payload_bytes: u64,
}

#[derive(Default)]
struct WorldMetrics {
    mutations: AtomicU64,
    reads: AtomicU64,
    rows_read: AtomicU64,
    database_statements: AtomicU64,
    notification_attempts: AtomicU64,
    notifications_sent: AtomicU64,
    notification_failures: AtomicU64,
    notification_payload_bytes: AtomicU64,
}

impl WorldMetrics {
    fn record_statement(&self) {
        self.database_statements.fetch_add(1, Ordering::Relaxed);
    }
}

#[derive(Clone, Copy)]
struct ResourceColumns {
    area_id: Option<Uuid>,
    place_id: Option<Uuid>,
    entity_id: Option<Uuid>,
}

impl ResourceColumns {
    fn from_key(key: ResourceKey) -> Self {
        match key {
            ResourceKey::Global | ResourceKey::World => Self {
                area_id: None,
                place_id: None,
                entity_id: None,
            },
            ResourceKey::Area(id) => Self {
                area_id: Some(id),
                place_id: None,
                entity_id: None,
            },
            ResourceKey::Place(id) => Self {
                area_id: None,
                place_id: Some(id),
                entity_id: None,
            },
            ResourceKey::Entity(id) => Self {
                area_id: None,
                place_id: None,
                entity_id: Some(id),
            },
        }
    }
}

#[derive(FromRow)]
struct ResourceRow {
    uri: String,
    parent_uri: Option<String>,
    name: String,
    state: String,
    version: Uuid,
    updated_activity_id: Option<Uuid>,
}

impl TryFrom<ResourceRow> for ResourceState {
    type Error = anyhow::Error;

    fn try_from(row: ResourceRow) -> Result<Self> {
        Ok(Self {
            resource: ResourceKey::from_str(&row.uri)?,
            parent: row
                .parent_uri
                .as_deref()
                .map(ResourceKey::from_str)
                .transpose()?,
            name: row.name,
            state: serde_json::from_str(&row.state)?,
            version: row.version,
            updated_activity_id: row.updated_activity_id,
        })
    }
}

#[derive(FromRow)]
struct ActivityRow {
    id: Uuid,
    operation: String,
    scope_kind: String,
    scope_area_id: Option<Uuid>,
    primary_entity_id: Option<Uuid>,
    primary_place_id: Option<Uuid>,
    affected_place_ids: Vec<Uuid>,
    resource_uris: Vec<String>,
    recorded_at: DateTime<Utc>,
}

impl TryFrom<ActivityRow> for Activity {
    type Error = anyhow::Error;

    fn try_from(row: ActivityRow) -> Result<Self> {
        let scope = match (row.scope_kind.as_str(), row.scope_area_id) {
            ("local", None) => ChangeScope::Local,
            ("area", Some(area_id)) => ChangeScope::Area { area_id },
            ("world", None) => ChangeScope::World,
            _ => bail!("invalid stored Activity scope"),
        };
        Ok(Self {
            id: row.id,
            operation: row.operation,
            scope,
            primary_entity_id: row.primary_entity_id,
            primary_place_id: row.primary_place_id,
            affected_place_ids: row.affected_place_ids,
            resource_uris: row.resource_uris,
            recorded_at: row.recorded_at,
        })
    }
}

async fn fetch_resource(pool: &PgPool, key: ResourceKey) -> Result<ResourceState> {
    let row = sqlx::query_as::<_, ResourceRow>(
        r#"
        SELECT uri, parent_uri, name, state::text AS state, version, updated_activity_id
        FROM resource
        WHERE uri = $1
        "#,
    )
    .bind(key.uri())
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| anyhow!("unknown resource: {key}"))?;
    row.try_into()
}

async fn fetch_children(pool: &PgPool, key: ResourceKey) -> Result<Vec<ResourceState>> {
    let rows = sqlx::query_as::<_, ResourceRow>(
        r#"
        SELECT uri, parent_uri, name, state::text AS state, version, updated_activity_id
        FROM resource
        WHERE parent_uri = $1
        ORDER BY uri
        LIMIT $2
        "#,
    )
    .bind(key.uri())
    .bind((MAX_CHILDREN + 1) as i64)
    .fetch_all(pool)
    .await?;
    rows.into_iter().map(TryInto::try_into).collect()
}

async fn fetch_activities(pool: &PgPool, key: ResourceKey, limit: i64) -> Result<Vec<Activity>> {
    // recorded_at/id is only a stable bounded presentation order, never commit or
    // causal ordering. Activity ids remain the durable identity.
    let rows = match key {
        ResourceKey::Global => {
            sqlx::query_as::<_, ActivityRow>(
                r#"
                SELECT id, operation, scope_kind, scope_area_id,
                       primary_entity_id, primary_place_id,
                       affected_place_ids, resource_uris, recorded_at
                FROM activity
                ORDER BY recorded_at DESC, id DESC
                LIMIT $1
                "#,
            )
            .bind(limit)
            .fetch_all(pool)
            .await?
        }
        ResourceKey::World
        | ResourceKey::Area(_)
        | ResourceKey::Place(_)
        | ResourceKey::Entity(_) => {
            sqlx::query_as::<_, ActivityRow>(
                r#"
                SELECT activity.id, activity.operation, activity.scope_kind,
                       activity.scope_area_id, activity.primary_entity_id,
                       activity.primary_place_id, activity.affected_place_ids,
                       activity.resource_uris, activity.recorded_at
                FROM activity_resource
                JOIN activity ON activity.id = activity_resource.activity_id
                WHERE activity_resource.resource_uri = $1
                ORDER BY activity_resource.recorded_at DESC,
                         activity_resource.activity_id DESC
                LIMIT $2
                "#,
            )
            .bind(key.uri())
            .bind(limit)
            .fetch_all(pool)
            .await?
        }
    };
    rows.into_iter().map(TryInto::try_into).collect()
}

fn validate_change(request: &ChangeRequest) -> Result<()> {
    if request.operation.trim().is_empty() || request.operation.len() > MAX_OPERATION_BYTES {
        bail!("operation must contain 1..={MAX_OPERATION_BYTES} bytes");
    }
    if request.mutations.is_empty() || request.mutations.len() > MAX_MUTATIONS {
        bail!("a change must contain 1..={MAX_MUTATIONS} resource mutations");
    }
    if request.affected_place_ids.len() > MAX_AFFECTED_PLACES {
        bail!("a change may affect at most {MAX_AFFECTED_PLACES} Places");
    }
    for mutation in &request.mutations {
        validate_resource_content(&mutation.name, &mutation.state)?;
    }
    match request.scope {
        ChangeScope::Local => {
            if request.affected_place_ids.is_empty() {
                bail!("a local change must declare at least one affected Place");
            }
        }
        ChangeScope::Area { area_id } => {
            if !request
                .mutations
                .iter()
                .any(|mutation| mutation.key == ResourceKey::Area(area_id))
            {
                bail!("an area change must update its one Area scope resource");
            }
        }
        ChangeScope::World => {
            if !request
                .mutations
                .iter()
                .any(|mutation| mutation.key == ResourceKey::World)
            {
                bail!("a World change must update the one World scope resource");
            }
        }
    }
    Ok(())
}

fn validate_resource_content(name: &str, state: &Value) -> Result<()> {
    if name.is_empty() || name.len() > MAX_RESOURCE_NAME_BYTES {
        bail!("resource name must contain 1..={MAX_RESOURCE_NAME_BYTES} bytes");
    }
    let state_bytes = serde_json::to_vec(state)?.len();
    if state_bytes > MAX_RESOURCE_STATE_BYTES {
        bail!("resource state exceeds {MAX_RESOURCE_STATE_BYTES} bytes: {state_bytes}");
    }
    Ok(())
}

fn activity_resource_keys(
    scope: ChangeScope,
    affected_place_ids: &[Uuid],
    resource_versions: &[ResourceVersion],
) -> Vec<ResourceKey> {
    let mut resources = affected_place_ids
        .iter()
        .copied()
        .map(ResourceKey::Place)
        .collect::<Vec<_>>();
    resources.extend(match scope {
        ChangeScope::Local => None,
        ChangeScope::Area { area_id } => Some(ResourceKey::Area(area_id)),
        ChangeScope::World => Some(ResourceKey::World),
    });
    resources.extend(resource_versions.iter().map(|version| version.resource));
    resources.sort_unstable();
    resources.dedup();
    resources
}

fn validate_resource_shape(key: ResourceKey, parent: Option<ResourceKey>) -> Result<()> {
    let valid = matches!(
        (key, parent),
        (ResourceKey::World, None)
            | (ResourceKey::Area(_), Some(ResourceKey::World))
            | (ResourceKey::Place(_), Some(ResourceKey::Area(_)))
            | (ResourceKey::Entity(_), Some(ResourceKey::Place(_)))
    );
    if !valid {
        bail!("invalid resource hierarchy for {key}");
    }
    Ok(())
}

fn validate_unique_keys(keys: impl IntoIterator<Item = ResourceKey>) -> Result<()> {
    let mut seen = HashSet::new();
    for key in keys {
        if !seen.insert(key) {
            bail!("duplicate resource mutation: {key}");
        }
    }
    Ok(())
}

async fn validate_references(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    request: &ChangeRequest,
    affected_place_ids: &[Uuid],
    mutations: &[ResourceMutation],
    metrics: &WorldMetrics,
) -> Result<()> {
    if !affected_place_ids.is_empty() {
        metrics.record_statement();
        let found: i64 = sqlx::query_scalar(
            "SELECT count(*) FROM resource WHERE kind = 'place' AND place_id = ANY($1)",
        )
        .bind(affected_place_ids)
        .fetch_one(&mut **transaction)
        .await?;
        if found != affected_place_ids.len() as i64 {
            bail!("an affected Place does not exist");
        }
    }
    if let ChangeScope::Area { area_id } = request.scope {
        let area_uri = ResourceKey::Area(area_id).uri();
        metrics.record_statement();
        let outside: i64 = sqlx::query_scalar(
            r#"
            SELECT count(*)
            FROM resource
            WHERE kind = 'place'
              AND place_id = ANY($1)
              AND parent_uri <> $2
            "#,
        )
        .bind(affected_place_ids)
        .bind(area_uri)
        .fetch_one(&mut **transaction)
        .await?;
        if outside != 0 {
            bail!("an area change declared a Place outside its Area");
        }
    }
    if let Some(place_id) = request.primary_place_id {
        ensure_resource_exists(transaction, ResourceKey::Place(place_id), metrics).await?;
    }
    if let Some(entity_id) = request.primary_entity_id {
        let is_created = mutations
            .iter()
            .any(|mutation| mutation.key == ResourceKey::Entity(entity_id));
        if !is_created {
            ensure_resource_exists(transaction, ResourceKey::Entity(entity_id), metrics).await?;
        }
    }
    Ok(())
}

async fn ensure_resource_exists(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    key: ResourceKey,
    metrics: &WorldMetrics,
) -> Result<()> {
    metrics.record_statement();
    let found: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM resource WHERE uri = $1)")
        .bind(key.uri())
        .fetch_one(&mut **transaction)
        .await?;
    if !found {
        bail!("referenced resource does not exist: {key}");
    }
    Ok(())
}

const fn scope_name(scope: ChangeScope) -> &'static str {
    match scope {
        ChangeScope::Local => "local",
        ChangeScope::Area { .. } => "area",
        ChangeScope::World => "world",
    }
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, Instant};

    use serde_json::json;
    use sqlx::postgres::{PgListener, PgPoolOptions};
    use tokio::time::timeout;

    use super::*;

    const AREA: Uuid = Uuid::from_u128(1);
    const PLACE: Uuid = Uuid::from_u128(2);
    const TREE: Uuid = Uuid::from_u128(3);

    fn fixture() -> Vec<SeedResource> {
        vec![
            SeedResource::new(
                ResourceKey::World,
                None,
                "World",
                json!({"weather":"clear"}),
            ),
            SeedResource::new(
                ResourceKey::Area(AREA),
                Some(ResourceKey::World),
                "North",
                json!({"season":"summer"}),
            ),
            SeedResource::new(
                ResourceKey::Place(PLACE),
                Some(ResourceKey::Area(AREA)),
                "Grove",
                json!({}),
            ),
            SeedResource::new(
                ResourceKey::Entity(TREE),
                Some(ResourceKey::Place(PLACE)),
                "Tree",
                json!({"state":"standing"}),
            ),
        ]
    }

    fn local_change(change_id: Uuid, parent: ResourceKey) -> ChangeRequest {
        ChangeRequest {
            change_id,
            operation: "change_tree".into(),
            scope: ChangeScope::Local,
            primary_entity_id: Some(TREE),
            primary_place_id: Some(PLACE),
            affected_place_ids: vec![PLACE],
            mutations: vec![ResourceMutation::new(
                ResourceKey::Entity(TREE),
                Some(parent),
                "Tree",
                json!({"state":"fallen"}),
            )],
        }
    }

    #[sqlx::test(migrations = "./migration")]
    async fn accepted_change_is_atomic_and_notifies_after_commit(pool: PgPool) -> Result<()> {
        let world = World::new(pool.clone());
        world.seed_resources(&fixture()).await?;
        let mut listener = PgListener::connect_with(&pool).await?;
        listener.listen(NOTIFY_CHANNEL).await?;

        let change_id = Uuid::new_v4();
        world
            .apply_change(
                Strategy::PlaceAndExact,
                local_change(change_id, ResourceKey::Place(PLACE)),
            )
            .await?;

        let notification = timeout(Duration::from_secs(2), listener.recv()).await??;
        let payload: NotificationPayload = serde_json::from_str(notification.payload())?;
        assert_eq!(
            payload.resources,
            vec![
                ResourceKey::Place(PLACE).uri(),
                ResourceKey::Entity(TREE).uri()
            ]
        );
        let metrics = world.metrics();
        assert_eq!(metrics.notifications_sent, 1);
        assert_eq!(metrics.notification_failures, 0);
        assert_eq!(metrics.database_statements, 6);
        let snapshot = world.read_resource(ResourceKey::Entity(TREE), 8).await?;
        assert_eq!(snapshot.current.unwrap().state, json!({"state":"fallen"}));
        assert_eq!(snapshot.recent_activities.len(), 1);
        assert_eq!(snapshot.recent_activities[0].id, change_id);
        let activity_count: i64 = sqlx::query_scalar("SELECT count(*) FROM activity")
            .fetch_one(&pool)
            .await?;
        assert_eq!(activity_count, 1);
        let activity_resource_count: i64 =
            sqlx::query_scalar("SELECT count(*) FROM activity_resource")
                .fetch_one(&pool)
                .await?;
        assert_eq!(activity_resource_count, 2);
        assert_eq!(world.metrics().database_statements, 8);
        Ok(())
    }

    #[sqlx::test(migrations = "./migration")]
    async fn failed_current_write_rolls_back_its_activity(pool: PgPool) -> Result<()> {
        let world = World::new(pool.clone());
        world.seed_resources(&fixture()).await?;
        let missing_place = ResourceKey::Place(Uuid::new_v4());

        assert!(
            world
                .apply_change(
                    Strategy::ExactOnly,
                    local_change(Uuid::new_v4(), missing_place),
                )
                .await
                .is_err()
        );
        let activity_count: i64 = sqlx::query_scalar("SELECT count(*) FROM activity")
            .fetch_one(&pool)
            .await?;
        assert_eq!(activity_count, 0);
        let activity_resource_count: i64 =
            sqlx::query_scalar("SELECT count(*) FROM activity_resource")
                .fetch_one(&pool)
                .await?;
        assert_eq!(activity_resource_count, 0);
        let snapshot = world.read_resource(ResourceKey::Entity(TREE), 8).await?;
        assert_eq!(snapshot.current.unwrap().state, json!({"state":"standing"}));
        Ok(())
    }

    #[sqlx::test(migrations = "./migration")]
    async fn explicit_world_scope_changes_only_its_scope_resource(pool: PgPool) -> Result<()> {
        let world = World::new(pool.clone());
        world.seed_resources(&fixture()).await?;
        let mut listener = PgListener::connect_with(&pool).await?;
        listener.listen(NOTIFY_CHANNEL).await?;

        let change_id = Uuid::new_v4();
        world
            .apply_change(
                Strategy::Structural,
                ChangeRequest {
                    change_id,
                    operation: "change_world_weather".into(),
                    scope: ChangeScope::World,
                    primary_entity_id: Some(TREE),
                    primary_place_id: Some(PLACE),
                    affected_place_ids: Vec::new(),
                    mutations: vec![ResourceMutation::new(
                        ResourceKey::World,
                        None,
                        "World",
                        json!({"weather":"storm"}),
                    )],
                },
            )
            .await?;

        let notification = timeout(Duration::from_secs(2), listener.recv()).await??;
        let payload: NotificationPayload = serde_json::from_str(notification.payload())?;
        assert_eq!(payload.resources, vec![ResourceKey::World.uri()]);

        let world_snapshot = world.read_resource(ResourceKey::World, 8).await?;
        assert_eq!(
            world_snapshot.current.unwrap().state,
            json!({"weather":"storm"})
        );
        assert_eq!(world_snapshot.recent_activities.len(), 1);
        assert_eq!(world_snapshot.recent_activities[0].id, change_id);

        let place_snapshot = world.read_resource(ResourceKey::Place(PLACE), 8).await?;
        assert_eq!(place_snapshot.current.unwrap().state, json!({}));
        assert!(place_snapshot.recent_activities.is_empty());
        let tree_snapshot = world.read_resource(ResourceKey::Entity(TREE), 8).await?;
        assert_eq!(
            tree_snapshot.current.unwrap().state,
            json!({"state":"standing"})
        );
        assert!(tree_snapshot.recent_activities.is_empty());
        let activity_resource_count: i64 =
            sqlx::query_scalar("SELECT count(*) FROM activity_resource")
                .fetch_one(&pool)
                .await?;
        assert_eq!(activity_resource_count, 1);
        Ok(())
    }

    #[test]
    fn stored_content_has_explicit_byte_bounds() {
        assert!(validate_resource_content("name", &json!({"ok": true})).is_ok());
        assert!(
            validate_resource_content(
                &"n".repeat(MAX_RESOURCE_NAME_BYTES + 1),
                &json!({"ok": true})
            )
            .is_err()
        );
        assert!(
            validate_resource_content(
                "name",
                &json!({"value": "x".repeat(MAX_RESOURCE_STATE_BYTES)})
            )
            .is_err()
        );
    }

    #[sqlx::test(migrations = "./migration")]
    async fn post_commit_notification_starvation_is_bounded_and_lossy(pool: PgPool) -> Result<()> {
        let notification_pool = PgPoolOptions::new()
            .max_connections(1)
            .acquire_timeout(Duration::from_secs(1))
            .connect_with((*pool.connect_options()).clone())
            .await?;
        let held_notification_connection = notification_pool.acquire().await?;
        let world = World::with_notification_pool(pool.clone(), notification_pool.clone());
        world.seed_resources(&fixture()).await?;

        let change_id = Uuid::new_v4();
        let started = Instant::now();
        let committed = world
            .apply_change(
                Strategy::PlaceAndExact,
                local_change(change_id, ResourceKey::Place(PLACE)),
            )
            .await?;
        assert_eq!(committed.change_id, change_id);
        assert!(started.elapsed() < Duration::from_millis(500));

        let metrics = world.metrics();
        assert_eq!(metrics.mutations, 1);
        assert_eq!(metrics.notification_attempts, 1);
        assert_eq!(metrics.notifications_sent, 0);
        assert_eq!(metrics.notification_failures, 1);
        assert_eq!(metrics.database_statements, 6);
        let snapshot = world.read_resource(ResourceKey::Entity(TREE), 8).await?;
        assert_eq!(snapshot.current.unwrap().state, json!({"state":"fallen"}));
        assert_eq!(snapshot.recent_activities.len(), 1);

        drop(held_notification_connection);
        notification_pool.close().await;
        Ok(())
    }

    #[sqlx::test(migrations = "./migration")]
    async fn child_baseline_reports_truncation_and_skips_leaf_query(pool: PgPool) -> Result<()> {
        let world = World::new(pool);
        let mut resources = fixture();
        for ordinal in 0..128_u128 {
            resources.push(SeedResource::new(
                ResourceKey::Entity(Uuid::from_u128(1_000 + ordinal)),
                Some(ResourceKey::Place(PLACE)),
                format!("Entity {ordinal}"),
                json!({}),
            ));
        }
        world.seed_resources(&resources).await?;

        let place = world.read_resource(ResourceKey::Place(PLACE), 8).await?;
        assert_eq!(place.children.len(), MAX_CHILDREN);
        assert!(place.children_truncated);
        assert_eq!(world.metrics().database_statements, 3);

        let entity = world.read_resource(ResourceKey::Entity(TREE), 8).await?;
        assert!(entity.children.is_empty());
        assert!(!entity.children_truncated);
        assert_eq!(world.metrics().database_statements, 5);
        Ok(())
    }
}
