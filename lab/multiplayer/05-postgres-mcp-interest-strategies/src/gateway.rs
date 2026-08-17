//! Experimental PostgreSQL notification gateway.
//!
//! Correctness stays in PostgreSQL and authoritative resource reads. This module
//! only holds bounded, transient interest and one coalescible dirty bit for each
//! active `(host, resource)` pair.

use std::{
    collections::{HashMap, HashSet},
    fmt,
    str::FromStr,
    sync::{
        Arc, Mutex, Weak,
        atomic::{AtomicU64, Ordering},
    },
};

use anyhow::{Context, Result, anyhow, bail};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use sqlx::postgres::PgListener;
use tokio::{sync::mpsc, task::JoinHandle};
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

use crate::strategy::ResourceKey;

pub const DEFAULT_NOTIFICATION_CHANNEL: &str = "aicadia_lab_resource_updated";

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct HostId(String);

impl HostId {
    pub fn new(value: impl Into<String>) -> Result<Self> {
        let value = value.into();
        if value.is_empty() || value.len() > 128 {
            bail!("host id must contain 1..=128 bytes");
        }
        if !value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.'))
        {
            bail!("host id contains a character outside [A-Za-z0-9._-]");
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for HostId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl FromStr for HostId {
    type Err = anyhow::Error;

    fn from_str(value: &str) -> Result<Self> {
        Self::new(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ResourceUri(String);

impl ResourceUri {
    /// Accept only URI shapes owned by the experiment's strategy model. There
    /// are no wildcard subscriptions and World scope carries no role semantics.
    pub fn parse(value: impl Into<String>) -> Result<Self> {
        let value = value.into();
        value
            .parse::<ResourceKey>()
            .with_context(|| format!("invalid resource URI: {value}"))?;
        Ok(Self(value))
    }

    pub fn global() -> Self {
        Self(ResourceKey::Global.uri())
    }

    pub fn world() -> Self {
        Self(ResourceKey::World.uri())
    }

    pub fn area(id: Uuid) -> Self {
        Self(ResourceKey::Area(id).uri())
    }

    pub fn place(id: Uuid) -> Self {
        Self(ResourceKey::Place(id).uri())
    }

    pub fn entity(id: Uuid) -> Self {
        Self(ResourceKey::Entity(id).uri())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ResourceUri {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl FromStr for ResourceUri {
    type Err = anyhow::Error;

    fn from_str(value: &str) -> Result<Self> {
        Self::parse(value)
    }
}

impl From<ResourceKey> for ResourceUri {
    fn from(value: ResourceKey) -> Self {
        Self(value.uri())
    }
}

impl TryFrom<&ResourceUri> for ResourceKey {
    type Error = anyhow::Error;

    fn try_from(value: &ResourceUri) -> Result<Self> {
        value.as_str().parse()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CompactNotification {
    pub resources: Vec<String>,
    /// Writer-provided microseconds since the Unix epoch. It is descriptive
    /// latency evidence only; routing and ordering never depend on it.
    #[serde(default)]
    pub committed_at_unix_micros: Option<i64>,
}

#[derive(Debug, Clone, Copy)]
pub struct GatewayBounds {
    pub max_subscriptions: usize,
    pub max_resources_per_subscription: usize,
    pub sink_capacity: usize,
    pub max_pending_keys: usize,
    pub max_payload_bytes: usize,
    pub max_resources_per_notification: usize,
}

impl Default for GatewayBounds {
    fn default() -> Self {
        Self {
            max_subscriptions: 4_096,
            max_resources_per_subscription: 64,
            sink_capacity: 64,
            max_pending_keys: 262_144,
            max_payload_bytes: 7_900,
            max_resources_per_notification: 64,
        }
    }
}

impl GatewayBounds {
    fn validate(self) -> Result<Self> {
        if self.max_subscriptions == 0
            || self.max_resources_per_subscription == 0
            || self.sink_capacity == 0
            || self.max_pending_keys == 0
            || self.max_payload_bytes == 0
            || self.max_resources_per_notification == 0
        {
            bail!("all gateway bounds must be non-zero");
        }
        Ok(self)
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize)]
pub struct GatewayMetrics {
    pub database_notifications: u64,
    pub database_listener_errors: u64,
    pub listener_failure_terminations: u64,
    pub malformed_notifications: u64,
    pub resources_received: u64,
    pub resources_without_interest: u64,
    pub raw_matched_hints: u64,
    pub emitted_hints: u64,
    pub coalesced_hints: u64,
    pub dropped_full_sinks: u64,
    pub dropped_pending_bound: u64,
    pub overload_terminations: u64,
    pub closed_sink_terminations: u64,
    pub cleared_by_read: u64,
    pub restored_after_failed_read: u64,
    pub authoritative_reads: u64,
    pub authoritative_rows: u64,
    pub authoritative_bytes: u64,
    pub subscription_registrations: u64,
    pub subscription_unregistrations: u64,
    pub rejected_subscriptions: u64,
    pub active_subscriptions: u64,
    pub peak_subscriptions: u64,
    pub pending_keys: u64,
    pub peak_pending_keys: u64,
    pub notification_latency_samples: u64,
    pub notification_latency_micros_total: u64,
    pub notification_latency_micros_max: u64,
}

#[derive(Default)]
struct AtomicMetrics {
    database_notifications: AtomicU64,
    database_listener_errors: AtomicU64,
    listener_failure_terminations: AtomicU64,
    malformed_notifications: AtomicU64,
    resources_received: AtomicU64,
    resources_without_interest: AtomicU64,
    raw_matched_hints: AtomicU64,
    emitted_hints: AtomicU64,
    coalesced_hints: AtomicU64,
    dropped_full_sinks: AtomicU64,
    dropped_pending_bound: AtomicU64,
    overload_terminations: AtomicU64,
    closed_sink_terminations: AtomicU64,
    cleared_by_read: AtomicU64,
    restored_after_failed_read: AtomicU64,
    authoritative_reads: AtomicU64,
    authoritative_rows: AtomicU64,
    authoritative_bytes: AtomicU64,
    subscription_registrations: AtomicU64,
    subscription_unregistrations: AtomicU64,
    rejected_subscriptions: AtomicU64,
    peak_subscriptions: AtomicU64,
    peak_pending_keys: AtomicU64,
    notification_latency_samples: AtomicU64,
    notification_latency_micros_total: AtomicU64,
    notification_latency_micros_max: AtomicU64,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RouteOutcome {
    pub matched: u64,
    pub emitted: u64,
    pub coalesced: u64,
    pub dropped: u64,
    pub terminated: u64,
}

struct SubscriptionEntry {
    host_id: HostId,
    resources: HashSet<ResourceUri>,
    sender: mpsc::Sender<ResourceUri>,
}

#[derive(Default)]
struct RouterState {
    next_subscription_id: u64,
    subscriptions: HashMap<u64, SubscriptionEntry>,
    subscription_by_host: HashMap<HostId, u64>,
    routes: HashMap<ResourceUri, HashSet<u64>>,
    dirty: HashSet<(HostId, ResourceUri)>,
    listener_failed: bool,
}

struct GatewayInner {
    bounds: GatewayBounds,
    state: Mutex<RouterState>,
    metrics: AtomicMetrics,
    cancellation: CancellationToken,
    listener_task: Mutex<Option<JoinHandle<()>>>,
}

impl Drop for GatewayInner {
    fn drop(&mut self) {
        self.cancellation.cancel();
        if let Ok(task) = self.listener_task.get_mut()
            && let Some(task) = task.take()
        {
            task.abort();
        }
    }
}

#[derive(Clone)]
pub struct Gateway {
    inner: Arc<GatewayInner>,
}

impl Gateway {
    pub fn new(bounds: GatewayBounds) -> Result<Self> {
        let bounds = bounds.validate()?;
        Ok(Self {
            inner: Arc::new(GatewayInner {
                bounds,
                state: Mutex::new(RouterState::default()),
                metrics: AtomicMetrics::default(),
                cancellation: CancellationToken::new(),
                listener_task: Mutex::new(None),
            }),
        })
    }

    /// Connect one real PostgreSQL listener for this gateway instance before
    /// returning, so commits after this call cannot race listener setup.
    pub async fn connect(database_url: &str, channel: &str, bounds: GatewayBounds) -> Result<Self> {
        validate_channel(channel)?;
        let mut listener = PgListener::connect(database_url)
            .await
            .context("connect PgListener")?;
        listener.listen(channel).await.context("LISTEN channel")?;
        Self::with_listener(listener, bounds)
    }

    pub async fn connect_with(pool: &PgPool, channel: &str, bounds: GatewayBounds) -> Result<Self> {
        validate_channel(channel)?;
        let mut listener = PgListener::connect_with(pool)
            .await
            .context("connect PgListener through pool options")?;
        listener.listen(channel).await.context("LISTEN channel")?;
        Self::with_listener(listener, bounds)
    }

    fn with_listener(mut listener: PgListener, bounds: GatewayBounds) -> Result<Self> {
        let gateway = Self::new(bounds)?;
        let worker = gateway.clone();
        let cancellation = gateway.inner.cancellation.clone();
        let task = tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = cancellation.cancelled() => break,
                    received = listener.recv() => match received {
                        Ok(notification) => worker.handle_payload(notification.payload()),
                        Err(_) => {
                            worker.inner.metrics.database_listener_errors.fetch_add(1, Ordering::Relaxed);
                            terminate_all_subscriptions(&worker.inner);
                            break;
                        }
                    }
                }
            }
        });
        *gateway
            .inner
            .listener_task
            .lock()
            .expect("listener task mutex poisoned") = Some(task);
        Ok(gateway)
    }

    pub fn bounds(&self) -> GatewayBounds {
        self.inner.bounds
    }

    pub fn register(
        &self,
        host_id: HostId,
        resources: impl IntoIterator<Item = ResourceUri>,
    ) -> Result<GatewayRegistration> {
        let resources: HashSet<_> = resources.into_iter().collect();
        let (sender, receiver) = mpsc::channel(self.inner.bounds.sink_capacity);
        let id = self.insert_subscription(host_id, resources.clone(), sender)?;
        Ok(GatewayRegistration {
            id,
            accepted: resources,
            receiver,
            inner: Arc::downgrade(&self.inner),
        })
    }

    fn insert_subscription(
        &self,
        host_id: HostId,
        resources: HashSet<ResourceUri>,
        sender: mpsc::Sender<ResourceUri>,
    ) -> Result<u64> {
        if resources.len() > self.inner.bounds.max_resources_per_subscription {
            return self.reject("subscription resource bound exceeded");
        }

        let mut state = self.inner.state.lock().expect("gateway state poisoned");
        if state.listener_failed {
            drop(state);
            return self.reject("gateway listener failed; connect through a replacement gateway");
        }
        if state.subscriptions.len() >= self.inner.bounds.max_subscriptions {
            drop(state);
            return self.reject("gateway subscription bound exceeded");
        }
        if state.subscription_by_host.contains_key(&host_id) {
            drop(state);
            return self.reject("a host may hold only one active lab subscription");
        }
        state.next_subscription_id = state.next_subscription_id.wrapping_add(1);
        let id = state.next_subscription_id;
        for resource in &resources {
            state.routes.entry(resource.clone()).or_default().insert(id);
        }
        state.subscription_by_host.insert(host_id.clone(), id);
        state.subscriptions.insert(
            id,
            SubscriptionEntry {
                host_id,
                resources,
                sender,
            },
        );
        let active = state.subscriptions.len() as u64;
        drop(state);

        self.inner
            .metrics
            .subscription_registrations
            .fetch_add(1, Ordering::Relaxed);
        update_max(&self.inner.metrics.peak_subscriptions, active);
        Ok(id)
    }

    fn reject<T>(&self, message: &str) -> Result<T> {
        self.inner
            .metrics
            .rejected_subscriptions
            .fetch_add(1, Ordering::Relaxed);
        Err(anyhow!(message.to_owned()))
    }

    pub fn handle_payload(&self, payload: &str) {
        self.inner
            .metrics
            .database_notifications
            .fetch_add(1, Ordering::Relaxed);
        if payload.len() > self.inner.bounds.max_payload_bytes {
            self.inner
                .metrics
                .malformed_notifications
                .fetch_add(1, Ordering::Relaxed);
            return;
        }
        let Ok(notification) = serde_json::from_str::<CompactNotification>(payload) else {
            self.inner
                .metrics
                .malformed_notifications
                .fetch_add(1, Ordering::Relaxed);
            return;
        };
        if notification.resources.is_empty()
            || notification.resources.len() > self.inner.bounds.max_resources_per_notification
        {
            self.inner
                .metrics
                .malformed_notifications
                .fetch_add(1, Ordering::Relaxed);
            return;
        }

        if let Some(committed_at) = notification.committed_at_unix_micros
            && let Ok(now) = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)
        {
            let now = i64::try_from(now.as_micros()).unwrap_or(i64::MAX);
            let latency = now.saturating_sub(committed_at).max(0) as u64;
            self.inner
                .metrics
                .notification_latency_samples
                .fetch_add(1, Ordering::Relaxed);
            self.inner
                .metrics
                .notification_latency_micros_total
                .fetch_add(latency, Ordering::Relaxed);
            update_max(&self.inner.metrics.notification_latency_micros_max, latency);
        }

        let mut unique = HashSet::with_capacity(notification.resources.len());
        for raw in notification.resources {
            let Ok(resource) = ResourceUri::parse(raw) else {
                self.inner
                    .metrics
                    .malformed_notifications
                    .fetch_add(1, Ordering::Relaxed);
                return;
            };
            unique.insert(resource);
        }
        for resource in unique {
            self.route(resource);
        }
    }

    pub fn route(&self, resource: ResourceUri) -> RouteOutcome {
        self.inner
            .metrics
            .resources_received
            .fetch_add(1, Ordering::Relaxed);
        let mut outcome = RouteOutcome::default();
        let mut state = self.inner.state.lock().expect("gateway state poisoned");
        let Some(subscription_ids) = state.routes.get(&resource).cloned() else {
            drop(state);
            self.inner
                .metrics
                .resources_without_interest
                .fetch_add(1, Ordering::Relaxed);
            return outcome;
        };

        for subscription_id in subscription_ids {
            let Some(entry) = state.subscriptions.get(&subscription_id) else {
                continue;
            };
            outcome.matched += 1;
            let host_id = entry.host_id.clone();
            let sender = entry.sender.clone();
            let key = (host_id, resource.clone());
            if state.dirty.contains(&key) {
                outcome.coalesced += 1;
                continue;
            }
            if state.dirty.len() >= self.inner.bounds.max_pending_keys {
                outcome.dropped += 1;
                outcome.terminated += 1;
                self.inner
                    .metrics
                    .dropped_pending_bound
                    .fetch_add(1, Ordering::Relaxed);
                self.inner
                    .metrics
                    .overload_terminations
                    .fetch_add(1, Ordering::Relaxed);
                remove_subscription(&mut state, subscription_id);
                continue;
            }
            state.dirty.insert(key.clone());
            match sender.try_send(resource.clone()) {
                Ok(()) => outcome.emitted += 1,
                Err(mpsc::error::TrySendError::Full(_)) => {
                    outcome.dropped += 1;
                    outcome.terminated += 1;
                    self.inner
                        .metrics
                        .dropped_full_sinks
                        .fetch_add(1, Ordering::Relaxed);
                    self.inner
                        .metrics
                        .overload_terminations
                        .fetch_add(1, Ordering::Relaxed);
                    remove_subscription(&mut state, subscription_id);
                }
                Err(mpsc::error::TrySendError::Closed(_)) => {
                    outcome.dropped += 1;
                    outcome.terminated += 1;
                    self.inner
                        .metrics
                        .closed_sink_terminations
                        .fetch_add(1, Ordering::Relaxed);
                    remove_subscription(&mut state, subscription_id);
                }
            }
        }
        let pending = state.dirty.len() as u64;
        drop(state);

        self.inner
            .metrics
            .raw_matched_hints
            .fetch_add(outcome.matched, Ordering::Relaxed);
        self.inner
            .metrics
            .emitted_hints
            .fetch_add(outcome.emitted, Ordering::Relaxed);
        self.inner
            .metrics
            .coalesced_hints
            .fetch_add(outcome.coalesced, Ordering::Relaxed);
        self.inner
            .metrics
            .subscription_unregistrations
            .fetch_add(outcome.terminated, Ordering::Relaxed);
        update_max(&self.inner.metrics.peak_pending_keys, pending);
        outcome
    }

    /// Clear before starting the authoritative read. A concurrent notification
    /// after this point can establish a new dirty bit and will not be erased by
    /// completion of the older read.
    pub fn begin_authoritative_read(&self, host_id: &HostId, resource: &ResourceUri) -> bool {
        let removed = self
            .inner
            .state
            .lock()
            .expect("gateway state poisoned")
            .dirty
            .remove(&(host_id.clone(), resource.clone()));
        if removed {
            self.inner
                .metrics
                .cleared_by_read
                .fetch_add(1, Ordering::Relaxed);
        }
        removed
    }

    /// Restore only after a failed read. This does not enqueue another hint:
    /// the host already initiated the read and can retry explicitly.
    pub fn restore_after_failed_read(&self, host_id: &HostId, resource: &ResourceUri) {
        let mut state = self.inner.state.lock().expect("gateway state poisoned");
        if !state.subscription_by_host.contains_key(host_id)
            || state.dirty.len() >= self.inner.bounds.max_pending_keys
        {
            return;
        }
        if state.dirty.insert((host_id.clone(), resource.clone())) {
            self.inner
                .metrics
                .restored_after_failed_read
                .fetch_add(1, Ordering::Relaxed);
            update_max(
                &self.inner.metrics.peak_pending_keys,
                state.dirty.len() as u64,
            );
        }
    }

    pub fn record_authoritative_read(&self, rows: u64, bytes: u64) {
        self.inner
            .metrics
            .authoritative_reads
            .fetch_add(1, Ordering::Relaxed);
        self.inner
            .metrics
            .authoritative_rows
            .fetch_add(rows, Ordering::Relaxed);
        self.inner
            .metrics
            .authoritative_bytes
            .fetch_add(bytes, Ordering::Relaxed);
    }

    pub fn is_dirty(&self, host_id: &HostId, resource: &ResourceUri) -> bool {
        self.inner
            .state
            .lock()
            .expect("gateway state poisoned")
            .dirty
            .contains(&(host_id.clone(), resource.clone()))
    }

    pub fn metrics(&self) -> GatewayMetrics {
        let state = self.inner.state.lock().expect("gateway state poisoned");
        let active_subscriptions = state.subscriptions.len() as u64;
        let pending_keys = state.dirty.len() as u64;
        drop(state);
        let metrics = &self.inner.metrics;
        GatewayMetrics {
            database_notifications: load(&metrics.database_notifications),
            database_listener_errors: load(&metrics.database_listener_errors),
            listener_failure_terminations: load(&metrics.listener_failure_terminations),
            malformed_notifications: load(&metrics.malformed_notifications),
            resources_received: load(&metrics.resources_received),
            resources_without_interest: load(&metrics.resources_without_interest),
            raw_matched_hints: load(&metrics.raw_matched_hints),
            emitted_hints: load(&metrics.emitted_hints),
            coalesced_hints: load(&metrics.coalesced_hints),
            dropped_full_sinks: load(&metrics.dropped_full_sinks),
            dropped_pending_bound: load(&metrics.dropped_pending_bound),
            overload_terminations: load(&metrics.overload_terminations),
            closed_sink_terminations: load(&metrics.closed_sink_terminations),
            cleared_by_read: load(&metrics.cleared_by_read),
            restored_after_failed_read: load(&metrics.restored_after_failed_read),
            authoritative_reads: load(&metrics.authoritative_reads),
            authoritative_rows: load(&metrics.authoritative_rows),
            authoritative_bytes: load(&metrics.authoritative_bytes),
            subscription_registrations: load(&metrics.subscription_registrations),
            subscription_unregistrations: load(&metrics.subscription_unregistrations),
            rejected_subscriptions: load(&metrics.rejected_subscriptions),
            active_subscriptions,
            peak_subscriptions: load(&metrics.peak_subscriptions),
            pending_keys,
            peak_pending_keys: load(&metrics.peak_pending_keys),
            notification_latency_samples: load(&metrics.notification_latency_samples),
            notification_latency_micros_total: load(&metrics.notification_latency_micros_total),
            notification_latency_micros_max: load(&metrics.notification_latency_micros_max),
        }
    }

    pub async fn shutdown(&self) {
        self.inner.cancellation.cancel();
        let task = self
            .inner
            .listener_task
            .lock()
            .expect("listener task mutex poisoned")
            .take();
        if let Some(task) = task {
            let _ = task.await;
        }
    }
}

pub struct GatewayRegistration {
    id: u64,
    accepted: HashSet<ResourceUri>,
    receiver: mpsc::Receiver<ResourceUri>,
    inner: Weak<GatewayInner>,
}

impl GatewayRegistration {
    pub fn accepted(&self) -> &HashSet<ResourceUri> {
        &self.accepted
    }

    pub async fn recv(&mut self) -> Option<ResourceUri> {
        self.receiver.recv().await
    }

    pub fn try_recv(&mut self) -> Result<ResourceUri, mpsc::error::TryRecvError> {
        self.receiver.try_recv()
    }
}

impl Drop for GatewayRegistration {
    fn drop(&mut self) {
        let Some(inner) = self.inner.upgrade() else {
            return;
        };
        unregister(&inner, self.id);
    }
}

fn unregister(inner: &Arc<GatewayInner>, subscription_id: u64) -> bool {
    let mut state = inner.state.lock().expect("gateway state poisoned");
    if !remove_subscription(&mut state, subscription_id) {
        return false;
    }
    drop(state);
    inner
        .metrics
        .subscription_unregistrations
        .fetch_add(1, Ordering::Relaxed);
    true
}

fn terminate_all_subscriptions(inner: &Arc<GatewayInner>) -> u64 {
    let mut state = inner.state.lock().expect("gateway state poisoned");
    let terminated = state.subscriptions.len() as u64;
    state.subscriptions.clear();
    state.subscription_by_host.clear();
    state.routes.clear();
    state.dirty.clear();
    state.listener_failed = true;
    drop(state);
    inner
        .metrics
        .subscription_unregistrations
        .fetch_add(terminated, Ordering::Relaxed);
    inner
        .metrics
        .listener_failure_terminations
        .fetch_add(terminated, Ordering::Relaxed);
    terminated
}

fn remove_subscription(state: &mut RouterState, subscription_id: u64) -> bool {
    let Some(entry) = state.subscriptions.remove(&subscription_id) else {
        return false;
    };
    state.subscription_by_host.remove(&entry.host_id);
    for resource in &entry.resources {
        if let Some(ids) = state.routes.get_mut(resource) {
            ids.remove(&subscription_id);
            if ids.is_empty() {
                state.routes.remove(resource);
            }
        }
    }
    for resource in &entry.resources {
        state
            .dirty
            .remove(&(entry.host_id.clone(), resource.clone()));
    }
    true
}

fn validate_channel(channel: &str) -> Result<()> {
    if channel.is_empty()
        || channel.len() > 63
        || !channel
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
    {
        bail!("invalid PostgreSQL channel name")
    }
    Ok(())
}

fn load(value: &AtomicU64) -> u64 {
    value.load(Ordering::Relaxed)
}

fn update_max(value: &AtomicU64, candidate: u64) {
    value.fetch_max(candidate, Ordering::Relaxed);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn resource(seed: u128) -> ResourceUri {
        ResourceUri::entity(Uuid::from_u128(seed))
    }

    #[tokio::test]
    async fn repeated_dirtiness_coalesces_until_authoritative_read() {
        let gateway = Gateway::new(GatewayBounds::default()).unwrap();
        let host = HostId::new("host-a").unwrap();
        let watched = resource(1);
        let mut registration = gateway.register(host.clone(), [watched.clone()]).unwrap();

        assert_eq!(gateway.route(watched.clone()).emitted, 1);
        assert_eq!(gateway.route(watched.clone()).coalesced, 1);
        assert_eq!(registration.recv().await, Some(watched.clone()));
        assert!(gateway.is_dirty(&host, &watched));
        assert!(gateway.begin_authoritative_read(&host, &watched));
        assert!(!gateway.is_dirty(&host, &watched));
        assert_eq!(gateway.route(watched.clone()).emitted, 1);
        assert_eq!(registration.recv().await, Some(watched));

        let metrics = gateway.metrics();
        assert_eq!(metrics.raw_matched_hints, 3);
        assert_eq!(metrics.emitted_hints, 2);
        assert_eq!(metrics.coalesced_hints, 1);
        assert_eq!(metrics.peak_pending_keys, 1);
    }

    #[tokio::test]
    async fn full_sink_terminates_subscription_and_clears_all_host_dirtiness() {
        let bounds = GatewayBounds {
            sink_capacity: 1,
            ..GatewayBounds::default()
        };
        let gateway = Gateway::new(bounds).unwrap();
        let host = HostId::new("slow-host").unwrap();
        let first = resource(1);
        let second = resource(2);
        let mut registration = gateway
            .register(host.clone(), [first.clone(), second.clone()])
            .unwrap();

        assert_eq!(gateway.route(first.clone()).emitted, 1);
        let overflow = gateway.route(second.clone());
        assert_eq!(overflow.dropped, 1);
        assert_eq!(overflow.terminated, 1);
        assert!(!gateway.is_dirty(&host, &first));
        assert!(!gateway.is_dirty(&host, &second));
        assert_eq!(registration.recv().await, Some(first));
        assert_eq!(registration.recv().await, None);
        assert_eq!(gateway.route(second).matched, 0);
        let metrics = gateway.metrics();
        assert_eq!(metrics.dropped_full_sinks, 1);
        assert_eq!(metrics.overload_terminations, 1);
        assert_eq!(metrics.subscription_unregistrations, 1);
        assert_eq!(metrics.active_subscriptions, 0);
        assert_eq!(metrics.pending_keys, 0);
    }

    #[test]
    fn unregister_removes_only_its_bounded_resource_set() {
        let gateway = Gateway::new(GatewayBounds::default()).unwrap();
        let first_host = HostId::new("first-host").unwrap();
        let other_host = HostId::new("other-host").unwrap();
        let first_resource = resource(1);
        let other_resource = resource(2);
        let first = gateway
            .register(first_host.clone(), [first_resource.clone()])
            .unwrap();
        let _other = gateway
            .register(other_host.clone(), [other_resource.clone()])
            .unwrap();

        gateway.route(first_resource.clone());
        gateway.route(other_resource.clone());
        drop(first);

        assert!(!gateway.is_dirty(&first_host, &first_resource));
        assert!(gateway.is_dirty(&other_host, &other_resource));
        assert_eq!(gateway.metrics().active_subscriptions, 1);
        assert_eq!(gateway.metrics().pending_keys, 1);
    }

    #[test]
    fn payload_and_uri_shapes_are_strict_and_bounded() {
        assert!(ResourceUri::parse("aicadia://world").is_ok());
        assert!(ResourceUri::parse("aicadia://global").is_ok());
        assert!(ResourceUri::parse("aicadia://place/not-a-uuid").is_err());
        assert!(ResourceUri::parse("aicadia://place/*").is_err());

        let gateway = Gateway::new(GatewayBounds::default()).unwrap();
        gateway.handle_payload(r#"{"resources":["aicadia://world"]}"#);
        gateway.handle_payload(r#"{"resources":["file:///wrong"]}"#);
        gateway.handle_payload(r#"{"resources":[],"extra":true}"#);
        let metrics = gateway.metrics();
        assert_eq!(metrics.database_notifications, 3);
        assert_eq!(metrics.malformed_notifications, 2);
        assert_eq!(metrics.resources_received, 1);
    }
}
