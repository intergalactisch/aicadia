use std::{
    collections::{BTreeMap, BTreeSet, HashSet},
    num::NonZeroUsize,
    time::{Duration, Instant},
};

use anyhow::{Context, Result, bail};
use postgres_mcp_interest_strategies_lab::{
    gateway::{Gateway, GatewayBounds, GatewayMetrics, HostId, ResourceUri},
    mcp::{
        InterestMcpServer, LoopbackMcpServer, McpLabClient, McpResourceSubscription, spawn_loopback,
    },
    scenario::{
        CHANGES_PER_STRATEGY, ChangeClass, ExpectedOutcomeOracle, HostInterest, LatencyDescription,
        LifecycleAction, REAL_SUBSCRIBERS, ScenarioFixture, ScenarioStep,
    },
    strategy::{ChangeScope, ResourceKey, Strategy},
    world::{
        ChangeRequest, NOTIFY_CHANNEL, ResourceMutation, SeedResource, World, WorldMetricsSnapshot,
    },
};
use serde::Serialize;
use serde_json::json;
use sqlx::PgPool;
use tokio::time::timeout;
use uuid::Uuid;

const STRATEGY_TIMEOUT: Duration = Duration::from_secs(30);
const UPDATE_TIMEOUT: Duration = Duration::from_secs(2);
const SUBSCRIPTION_CAPACITY: NonZeroUsize = NonZeroUsize::new(16).unwrap();

struct RealHost {
    interest: HostInterest,
    gateway_index: usize,
    client: Option<McpLabClient>,
    subscription: Option<McpResourceSubscription>,
    watched: HashSet<ResourceUri>,
    pending: HashSet<ResourceUri>,
}

impl RealHost {
    fn new(interest: HostInterest) -> Self {
        Self {
            gateway_index: interest.host_id as usize % 2,
            interest,
            client: None,
            subscription: None,
            watched: HashSet::new(),
            pending: HashSet::new(),
        }
    }

    fn connected(&self) -> bool {
        self.client.is_some()
    }
}

#[derive(Default, Serialize)]
struct RealStrategyMetrics {
    strategy: String,
    actual_mcp_subscribers: u64,
    gateway_instances: u64,
    committed_changes: u64,
    activities_written: u64,
    database_statements: u64,
    world_notifications_sent: u64,
    unroutable_commits: u64,
    gateway_0_database_notifications: u64,
    gateway_1_database_notifications: u64,
    gateway_listener_errors: u64,
    malformed_notifications: u64,
    dropped_hints: u64,
    overload_terminations: u64,
    closed_sink_terminations: u64,
    acknowledged_resource_uris: u64,
    expected_resource_updates: u64,
    observed_resource_updates: u64,
    required_live_deliveries: u64,
    missed_live_deliveries: u64,
    missed_recoveries: u64,
    irrelevant_wakeups: u64,
    raw_hints: u64,
    emitted_hints: u64,
    coalesced_hints: u64,
    coalesced_dirty_checks: u64,
    peak_pending_keys: u64,
    pending_keys_end: u64,
    refetches: u64,
    refetch_rows: u64,
    refetch_bytes: u64,
    subscription_churn: u64,
    notification_queue_usage: f64,
    mutation_latency: LatencyDescription,
    actual_notification_latency: LatencyDescription,
    quiet_subject_actual_notification_latency: LatencyDescription,
    model_calls: u64,
    elapsed_millis: u64,
}

struct RunTopology {
    gateways: [Gateway; 2],
    servers: Option<[LoopbackMcpServer; 2]>,
    hosts: BTreeMap<u32, RealHost>,
}

#[sqlx::test(migrations = "./migration")]
async fn all_five_strategies_use_the_same_real_postgres_gateway_mcp_chain(
    pool: PgPool,
) -> Result<()> {
    let fixture = ScenarioFixture::fixed();
    assert_eq!(
        fixture.program().committed_change_count(),
        CHANGES_PER_STRATEGY
    );
    assert_eq!(fixture.hosts.len(), REAL_SUBSCRIBERS);

    let mut reports = BTreeMap::new();
    for strategy in Strategy::ALL {
        let report = timeout(
            STRATEGY_TIMEOUT,
            run_strategy(pool.clone(), &fixture, strategy),
        )
        .await
        .with_context(|| {
            format!(
                "{} exceeded the 30-second direct-tier bound",
                strategy.name()
            )
        })??;
        println!("{}", serde_json::to_string(&report)?);
        reports.insert(strategy.name(), report);
    }

    let global = &reports["global_firehose"];
    assert_eq!(global.missed_live_deliveries, 0);
    assert!(global.irrelevant_wakeups > 0);

    assert!(reports["place"].missed_live_deliveries > 0);
    assert!(reports["exact_only"].missed_live_deliveries > 0);
    assert!(reports["place_and_exact"].missed_live_deliveries > 0);

    let structural = &reports["structural"];
    assert_eq!(structural.missed_live_deliveries, 0);
    assert_eq!(structural.missed_recoveries, 0);
    assert_eq!(structural.irrelevant_wakeups, 0);

    for report in reports.values() {
        assert_eq!(report.actual_mcp_subscribers, REAL_SUBSCRIBERS as u64);
        assert_eq!(report.gateway_instances, 2);
        assert_eq!(report.committed_changes, CHANGES_PER_STRATEGY as u64);
        assert_eq!(report.activities_written, CHANGES_PER_STRATEGY as u64);
        assert!(
            report.database_statements
                >= report.committed_changes.saturating_mul(3) + report.refetches
        );
        assert_eq!(
            report.gateway_0_database_notifications,
            report.world_notifications_sent
        );
        assert_eq!(
            report.gateway_1_database_notifications,
            report.world_notifications_sent
        );
        assert_eq!(report.model_calls, 0);
        assert_eq!(report.gateway_listener_errors, 0);
        assert_eq!(report.malformed_notifications, 0);
        assert_eq!(report.dropped_hints, 0);
        assert_eq!(report.overload_terminations, 0);
        assert_eq!(report.closed_sink_terminations, 0);
        assert_eq!(report.pending_keys_end, 0);
        assert_eq!(
            report.observed_resource_updates,
            report.expected_resource_updates
        );
        assert_eq!(
            report.world_notifications_sent + report.unroutable_commits,
            CHANGES_PER_STRATEGY as u64
        );
        assert!(report.elapsed_millis < STRATEGY_TIMEOUT.as_millis() as u64);
    }
    Ok(())
}

async fn run_strategy(
    pool: PgPool,
    fixture: &ScenarioFixture,
    strategy: Strategy,
) -> Result<RealStrategyMetrics> {
    let started = Instant::now();
    let world = World::new(pool.clone());
    world.reset().await?;
    world.seed_resources(&seed_fixture(fixture)).await?;

    let gateways = [
        Gateway::connect_with(&pool, NOTIFY_CHANNEL, GatewayBounds::default()).await?,
        Gateway::connect_with(&pool, NOTIFY_CHANNEL, GatewayBounds::default()).await?,
    ];
    let servers = [
        spawn_loopback(InterestMcpServer::new(gateways[0].clone(), world.clone())).await?,
        spawn_loopback(InterestMcpServer::new(gateways[1].clone(), world.clone())).await?,
    ];
    let urls = [servers[0].url.clone(), servers[1].url.clone()];
    let mut topology = RunTopology {
        gateways,
        servers: Some(servers),
        hosts: fixture
            .hosts
            .iter()
            .cloned()
            .map(|interest| (interest.host_id, RealHost::new(interest)))
            .collect(),
    };

    let mut acknowledged_resource_uris = 0_u64;
    let mut refetches = 0_u64;
    let mut refetch_rows = 0_u64;
    let mut refetch_bytes = 0_u64;
    for host_id in 0..REAL_SUBSCRIBERS as u32 {
        acknowledged_resource_uris +=
            connect_host(&urls, strategy, topology.hosts.get_mut(&host_id).unwrap()).await?;
        let _ = read_all_watched(
            topology.hosts.get_mut(&host_id).unwrap(),
            &mut refetches,
            &mut refetch_rows,
            &mut refetch_bytes,
        )
        .await?;
    }
    assert_eq!(topology.gateways[0].metrics().active_subscriptions, 16);
    assert_eq!(topology.gateways[1].metrics().active_subscriptions, 16);

    let mut oracle = ExpectedOutcomeOracle::new(fixture);
    let mut ordinal = 0_u128;
    let mut required_live_deliveries = 0_u64;
    let mut expected_resource_updates = 0_u64;
    let mut observed_resource_updates = 0_u64;
    let mut missed_live_deliveries = 0_u64;
    let mut missed_recoveries = 0_u64;
    let mut irrelevant_wakeups = 0_u64;
    let mut mutation_latencies = Vec::with_capacity(CHANGES_PER_STRATEGY);
    let mut notification_latencies = Vec::with_capacity(CHANGES_PER_STRATEGY);
    let mut quiet_latencies = Vec::new();
    let mut coalesced_dirty_checks = 0_u64;
    let mut expected_database_notifications = 0_u64;

    for case in &fixture.program().cases {
        for step in &case.steps {
            let expected = oracle.begin_step(step);
            let recovered = apply_actions(
                &urls,
                strategy,
                &step.actions_before,
                &mut topology.hosts,
                &mut acknowledged_resource_uris,
                &mut refetches,
                &mut refetch_rows,
                &mut refetch_bytes,
                (ordinal > 0).then(|| Uuid::from_u128(ordinal)),
            )
            .await?;

            for repetition in 0..step.repetitions {
                ordinal += 1;
                let request = change_request(fixture, step, ordinal);
                let mutation_started = Instant::now();
                let committed = world.apply_change(strategy, request).await?;
                mutation_latencies.push(micros(mutation_started.elapsed()));

                let changed = strategy
                    .resources_for_change(&committed)
                    .into_iter()
                    .map(ResourceUri::from)
                    .collect::<HashSet<_>>();
                let notify_started = Instant::now();
                if !changed.is_empty() {
                    expected_database_notifications += 1;
                    wait_for_gateway_notifications(
                        &topology.gateways,
                        expected_database_notifications,
                    )
                    .await?;
                }
                let mut live_notified = BTreeSet::new();
                let mut actual_updates_received = 0_u64;
                for host in topology.hosts.values_mut().filter(|host| host.connected()) {
                    let relevant = host
                        .watched
                        .intersection(&changed)
                        .cloned()
                        .collect::<HashSet<_>>();
                    if relevant.is_empty() {
                        continue;
                    }
                    let newly_dirty = relevant
                        .difference(&host.pending)
                        .cloned()
                        .collect::<HashSet<_>>();
                    coalesced_dirty_checks +=
                        relevant.len().saturating_sub(newly_dirty.len()) as u64;
                    expected_resource_updates += newly_dirty.len() as u64;
                    let mut observed = HashSet::with_capacity(newly_dirty.len());
                    for expected_resource in &newly_dirty {
                        let actual = timeout(
                            UPDATE_TIMEOUT,
                            host.subscription
                                .as_mut()
                                .expect("connected subscription")
                                .next_updated(),
                        )
                        .await
                        .with_context(|| {
                            format!(
                                "{} host {} missed MCP update for {}",
                                strategy.name(),
                                host.interest.host_id,
                                expected_resource
                            )
                        })??
                        .context("subscription ended before expected resource update")?;
                        if !newly_dirty.contains(&actual) {
                            bail!("MCP delivered unaccepted or unexpected resource {actual}");
                        }
                        if !observed.insert(actual.clone()) {
                            bail!(
                                "MCP delivered duplicate resource {actual} instead of the exact dirty resource set"
                            );
                        }
                        host.pending.insert(actual);
                        actual_updates_received += 1;
                    }
                    if observed != newly_dirty {
                        bail!(
                            "MCP resource delivery mismatch for host {}: expected {newly_dirty:?}, observed {observed:?}",
                            host.interest.host_id
                        );
                    }
                    observed_resource_updates += observed.len() as u64;
                    if relevant
                        .iter()
                        .any(|resource| host.pending.contains(resource))
                    {
                        live_notified.insert(host.interest.host_id);
                    }
                }
                let notification_latency = micros(notify_started.elapsed());
                if actual_updates_received > 0 {
                    notification_latencies.push(notification_latency);
                    if step.class == ChangeClass::QuietIsolation {
                        quiet_latencies.push(notification_latency);
                    }
                }

                let slow = step.slow_consumer;
                for host in topology.hosts.values_mut().filter(|host| host.connected()) {
                    if slow == Some(host.interest.host_id) && repetition + 1 < step.repetitions {
                        continue;
                    }
                    read_pending(host, &mut refetches, &mut refetch_rows, &mut refetch_bytes)
                        .await?;
                }

                let coverage = expected.evaluate(&live_notified, &recovered);
                required_live_deliveries += expected.live_required.len() as u64;
                missed_live_deliveries += coverage.missed_live_hosts.len() as u64;
                missed_recoveries += coverage.missed_recovery_hosts.len() as u64;
                irrelevant_wakeups += coverage.irrelevant_wakeups.len() as u64;
            }

            apply_actions(
                &urls,
                strategy,
                &step.actions_after,
                &mut topology.hosts,
                &mut acknowledged_resource_uris,
                &mut refetches,
                &mut refetch_rows,
                &mut refetch_bytes,
                Some(Uuid::from_u128(ordinal)),
            )
            .await?;
            oracle.finish_step(step);
        }
    }

    let world_metrics = world.metrics();
    let gateway_metrics = [
        topology.gateways[0].metrics(),
        topology.gateways[1].metrics(),
    ];
    let activities_written: i64 = sqlx::query_scalar("SELECT count(*) FROM activity")
        .fetch_one(&pool)
        .await?;
    let notification_queue_usage: f64 = sqlx::query_scalar("SELECT pg_notification_queue_usage()")
        .fetch_one(&pool)
        .await?;

    let report = build_report(
        strategy,
        started.elapsed(),
        world_metrics,
        gateway_metrics,
        acknowledged_resource_uris,
        expected_resource_updates,
        observed_resource_updates,
        required_live_deliveries,
        missed_live_deliveries,
        missed_recoveries,
        irrelevant_wakeups,
        coalesced_dirty_checks,
        refetches,
        refetch_rows,
        refetch_bytes,
        activities_written as u64,
        notification_queue_usage,
        mutation_latencies,
        notification_latencies,
        quiet_latencies,
    );

    shutdown_topology(&mut topology).await?;
    Ok(report)
}

async fn wait_for_gateway_notifications(gateways: &[Gateway; 2], expected: u64) -> Result<()> {
    timeout(UPDATE_TIMEOUT, async {
        loop {
            if gateways
                .iter()
                .all(|gateway| gateway.metrics().database_notifications >= expected)
            {
                return;
            }
            tokio::task::yield_now().await;
        }
    })
    .await
    .with_context(|| format!("two PgListeners did not both observe notification {expected}"))?;
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn build_report(
    strategy: Strategy,
    elapsed: Duration,
    world: WorldMetricsSnapshot,
    gateways: [GatewayMetrics; 2],
    acknowledged_resource_uris: u64,
    expected_resource_updates: u64,
    observed_resource_updates: u64,
    required_live_deliveries: u64,
    missed_live_deliveries: u64,
    missed_recoveries: u64,
    irrelevant_wakeups: u64,
    coalesced_dirty_checks: u64,
    refetches: u64,
    refetch_rows: u64,
    refetch_bytes: u64,
    activities_written: u64,
    notification_queue_usage: f64,
    mutation_latencies: Vec<u64>,
    notification_latencies: Vec<u64>,
    quiet_latencies: Vec<u64>,
) -> RealStrategyMetrics {
    RealStrategyMetrics {
        strategy: strategy.name().to_owned(),
        actual_mcp_subscribers: REAL_SUBSCRIBERS as u64,
        gateway_instances: 2,
        committed_changes: world.mutations,
        activities_written,
        database_statements: world.database_statements,
        world_notifications_sent: world.notifications_sent,
        unroutable_commits: world.mutations.saturating_sub(world.notification_attempts),
        gateway_0_database_notifications: gateways[0].database_notifications,
        gateway_1_database_notifications: gateways[1].database_notifications,
        gateway_listener_errors: gateways
            .iter()
            .map(|metrics| metrics.database_listener_errors)
            .sum(),
        malformed_notifications: gateways
            .iter()
            .map(|metrics| metrics.malformed_notifications)
            .sum(),
        dropped_hints: gateways
            .iter()
            .map(|metrics| metrics.dropped_full_sinks + metrics.dropped_pending_bound)
            .sum(),
        overload_terminations: gateways
            .iter()
            .map(|metrics| metrics.overload_terminations)
            .sum(),
        closed_sink_terminations: gateways
            .iter()
            .map(|metrics| metrics.closed_sink_terminations)
            .sum(),
        acknowledged_resource_uris,
        expected_resource_updates,
        observed_resource_updates,
        required_live_deliveries,
        missed_live_deliveries,
        missed_recoveries,
        irrelevant_wakeups,
        raw_hints: gateways
            .iter()
            .map(|metrics| metrics.raw_matched_hints)
            .sum(),
        emitted_hints: gateways.iter().map(|metrics| metrics.emitted_hints).sum(),
        coalesced_hints: gateways.iter().map(|metrics| metrics.coalesced_hints).sum(),
        coalesced_dirty_checks,
        peak_pending_keys: gateways
            .iter()
            .map(|metrics| metrics.peak_pending_keys)
            .max()
            .unwrap_or_default(),
        pending_keys_end: gateways.iter().map(|metrics| metrics.pending_keys).sum(),
        refetches,
        refetch_rows,
        refetch_bytes,
        subscription_churn: gateways
            .iter()
            .map(|metrics| {
                metrics.subscription_registrations + metrics.subscription_unregistrations
            })
            .sum(),
        notification_queue_usage,
        mutation_latency: LatencyDescription::from_actual_samples(
            mutation_latencies,
            "100 actual PostgreSQL World commits, sequential, one writer",
        ),
        actual_notification_latency: LatencyDescription::from_actual_samples(
            notification_latencies,
            "commit return through actual rmcp resource updates across two PgListeners; coalesced dirty checks excluded",
        ),
        quiet_subject_actual_notification_latency: LatencyDescription::from_actual_samples(
            quiet_latencies,
            "actual quiet-Place rmcp update while hot-subject history remains bounded",
        ),
        model_calls: 0,
        elapsed_millis: elapsed.as_millis().min(u64::MAX as u128) as u64,
    }
}

async fn connect_host(urls: &[String; 2], strategy: Strategy, host: &mut RealHost) -> Result<u64> {
    let watched = strategy
        .resources_for_interest(&host.interest.interest_spec())
        .into_iter()
        .map(ResourceUri::from)
        .collect::<HashSet<_>>();
    let client = McpLabClient::connect(
        &urls[host.gateway_index],
        HostId::new(format!("{}-{}", strategy.name(), host.interest.host_id))?,
    )
    .await?;
    let subscription = client
        .listen(watched.iter().cloned(), SUBSCRIPTION_CAPACITY)
        .await?;
    if subscription.acknowledged() != &watched {
        bail!("server did not acknowledge the exact requested resource set");
    }
    host.client = Some(client);
    host.subscription = Some(subscription);
    host.watched = watched;
    host.pending.clear();
    Ok(host.watched.len() as u64)
}

#[allow(clippy::too_many_arguments)]
async fn apply_actions(
    urls: &[String; 2],
    strategy: Strategy,
    actions: &[LifecycleAction],
    hosts: &mut BTreeMap<u32, RealHost>,
    acknowledged_resource_uris: &mut u64,
    refetches: &mut u64,
    refetch_rows: &mut u64,
    refetch_bytes: &mut u64,
    latest_committed_change: Option<Uuid>,
) -> Result<BTreeSet<u32>> {
    let mut recovered = BTreeSet::new();
    for action in actions {
        match *action {
            LifecycleAction::Disconnect { host_id } => {
                disconnect_host(hosts.get_mut(&host_id).context("fixture host")?).await?;
            }
            LifecycleAction::ReconnectAndRefetch { host_id } => {
                let host = hosts.get_mut(&host_id).context("fixture host")?;
                *acknowledged_resource_uris += connect_host(urls, strategy, host).await?;
                let documents =
                    read_all_watched(host, refetches, refetch_rows, refetch_bytes).await?;
                if latest_committed_change.is_some_and(|change_id| {
                    documents
                        .iter()
                        .any(|document| document_contains_activity(document, change_id))
                }) {
                    recovered.insert(host_id);
                }
            }
            LifecycleAction::Move {
                host_id,
                area_id,
                place_id,
            } => {
                let host = hosts.get_mut(&host_id).context("fixture host")?;
                disconnect_host(host).await?;
                host.interest.area_id = area_id;
                host.interest.place_id = place_id;
                *acknowledged_resource_uris += connect_host(urls, strategy, host).await?;
                let _ = read_all_watched(host, refetches, refetch_rows, refetch_bytes).await?;
            }
        }
    }
    Ok(recovered)
}

async fn read_pending(
    host: &mut RealHost,
    refetches: &mut u64,
    refetch_rows: &mut u64,
    refetch_bytes: &mut u64,
) -> Result<()> {
    let pending = std::mem::take(&mut host.pending);
    for resource in pending {
        let _ = read_one(host, &resource, refetches, refetch_rows, refetch_bytes).await?;
    }
    Ok(())
}

async fn read_all_watched(
    host: &mut RealHost,
    refetches: &mut u64,
    refetch_rows: &mut u64,
    refetch_bytes: &mut u64,
) -> Result<Vec<serde_json::Value>> {
    let watched = host.watched.iter().cloned().collect::<Vec<_>>();
    let mut documents = Vec::with_capacity(watched.len());
    for resource in watched {
        documents.push(read_one(host, &resource, refetches, refetch_rows, refetch_bytes).await?);
    }
    host.pending.clear();
    Ok(documents)
}

async fn read_one(
    host: &mut RealHost,
    resource: &ResourceUri,
    refetches: &mut u64,
    refetch_rows: &mut u64,
    refetch_bytes: &mut u64,
) -> Result<serde_json::Value> {
    let read = host
        .client
        .as_ref()
        .context("read requires connected client")?
        .read_resource(resource)
        .await?;
    *refetches += 1;
    *refetch_rows += document_rows(&read.document);
    *refetch_bytes += read.bytes;
    Ok(read.document)
}

fn document_contains_activity(document: &serde_json::Value, change_id: Uuid) -> bool {
    document["recent_activities"]
        .as_array()
        .is_some_and(|activities| {
            activities
                .iter()
                .any(|activity| activity["id"] == change_id.to_string())
        })
}

fn document_rows(document: &serde_json::Value) -> u64 {
    u64::from(!document["current"].is_null())
        + document["children"]
            .as_array()
            .map_or(0, |rows| rows.len() as u64)
        + document["recent_activities"]
            .as_array()
            .map_or(0, |rows| rows.len() as u64)
}

async fn disconnect_host(host: &mut RealHost) -> Result<()> {
    if let Some(subscription) = host.subscription.take() {
        subscription.cancel().await?;
    }
    if let Some(client) = host.client.take() {
        client.close().await?;
    }
    host.watched.clear();
    host.pending.clear();
    Ok(())
}

async fn shutdown_topology(topology: &mut RunTopology) -> Result<()> {
    for host in topology.hosts.values_mut() {
        disconnect_host(host).await?;
    }
    if let Some([first, second]) = topology.servers.take() {
        first.shutdown().await?;
        second.shutdown().await?;
    }
    topology.gateways[0].shutdown().await;
    topology.gateways[1].shutdown().await;
    Ok(())
}

fn seed_fixture(fixture: &ScenarioFixture) -> Vec<SeedResource> {
    let mut resources = vec![SeedResource::new(
        ResourceKey::World,
        None,
        "World",
        json!({"fixture": fixture.world_id}),
    )];
    resources.extend([
        SeedResource::new(
            ResourceKey::Area(fixture.north_area_id),
            Some(ResourceKey::World),
            "North",
            json!({}),
        ),
        SeedResource::new(
            ResourceKey::Area(fixture.south_area_id),
            Some(ResourceKey::World),
            "South",
            json!({}),
        ),
    ]);
    for (place, area, name) in [
        (fixture.place_a_id, fixture.north_area_id, "Place A"),
        (fixture.place_b_id, fixture.north_area_id, "Place B"),
        (fixture.quiet_place_id, fixture.south_area_id, "Quiet Place"),
        (
            fixture.remote_place_id,
            fixture.south_area_id,
            "Remote Place",
        ),
    ] {
        resources.push(SeedResource::new(
            ResourceKey::Place(place),
            Some(ResourceKey::Area(area)),
            name,
            json!({}),
        ));
    }
    for (entity, place, name) in [
        (fixture.selected_tree_id, fixture.place_a_id, "Tree"),
        (
            fixture.unselected_local_entity_id,
            fixture.place_a_id,
            "Local Entity",
        ),
        (fixture.bomb_entity_id, fixture.place_a_id, "Bomb"),
        (
            fixture.place_b_entity_id,
            fixture.place_b_id,
            "Place B Entity",
        ),
        (
            fixture.quiet_entity_id,
            fixture.quiet_place_id,
            "Quiet Entity",
        ),
    ] {
        resources.push(SeedResource::new(
            ResourceKey::Entity(entity),
            Some(ResourceKey::Place(place)),
            name,
            json!({"revision": 0}),
        ));
    }
    resources
}

fn change_request(fixture: &ScenarioFixture, step: &ScenarioStep, ordinal: u128) -> ChangeRequest {
    let (scope, affected_place_ids) = step.change_scope_and_affected_places();
    let mutation = match (scope, step.entity_id) {
        (ChangeScope::Area { area_id }, _) => ResourceMutation::new(
            ResourceKey::Area(area_id),
            Some(ResourceKey::World),
            "Area",
            json!({"revision": ordinal}),
        ),
        (ChangeScope::World, _) => ResourceMutation::new(
            ResourceKey::World,
            None,
            "World",
            json!({"revision": ordinal}),
        ),
        (ChangeScope::Local, Some(entity_id)) => {
            let place_id = entity_place(fixture, step, entity_id, &affected_place_ids);
            ResourceMutation::new(
                ResourceKey::Entity(entity_id),
                Some(ResourceKey::Place(place_id)),
                format!("Entity {entity_id}"),
                json!({"revision": ordinal}),
            )
        }
        (ChangeScope::Local, None) => {
            let place_id = affected_place_ids[0];
            let area_id = if [fixture.place_a_id, fixture.place_b_id].contains(&place_id) {
                fixture.north_area_id
            } else {
                fixture.south_area_id
            };
            ResourceMutation::new(
                ResourceKey::Place(place_id),
                Some(ResourceKey::Area(area_id)),
                "Place",
                json!({"revision": ordinal}),
            )
        }
    };
    ChangeRequest {
        change_id: Uuid::from_u128(ordinal),
        operation: step.name.to_owned(),
        scope,
        primary_entity_id: step.entity_id,
        primary_place_id: affected_place_ids.first().copied(),
        affected_place_ids,
        mutations: vec![mutation],
    }
}

fn entity_place(
    fixture: &ScenarioFixture,
    step: &ScenarioStep,
    entity_id: Uuid,
    affected_places: &[Uuid],
) -> Uuid {
    if entity_id == fixture.place_b_entity_id {
        fixture.place_b_id
    } else if entity_id == fixture.quiet_entity_id || step.class == ChangeClass::QuietIsolation {
        fixture.quiet_place_id
    } else {
        affected_places
            .first()
            .copied()
            .unwrap_or(fixture.place_a_id)
    }
}

fn micros(duration: Duration) -> u64 {
    duration.as_micros().min(u64::MAX as u128) as u64
}
