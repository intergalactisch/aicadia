//! Experimental hot-resource fan-out isolation fixture.
//!
//! This module compares only an in-memory routing seam. It does not exercise the
//! experiment's Gateway, PostgreSQL listener, MCP transport, network or World.
//! Timing is descriptive; exact delivery, coalescing and bounded pending state
//! are the correctness evidence.

use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fmt,
    sync::{Barrier, Mutex, OnceLock},
    thread,
    time::{Duration, Instant},
};

pub const HOT_RECIPIENTS: usize = 4_096;
pub const QUIET_RESOURCES: usize = 64;
pub const FIXED_SUBSCRIPTION_PAIRS: usize = HOT_RECIPIENTS + QUIET_RESOURCES;

const HOT_RESOURCE: ResourceId = ResourceId(0);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ResourceId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RecipientId(pub u64);

#[derive(Debug, Clone, Copy)]
pub struct RouterBounds {
    pub max_subscription_pairs: usize,
    pub max_pending_keys: usize,
}

/// Explicit capacity for one stripe. A dynamic adapter supplies the complete
/// plan up front, so adding stripes cannot silently multiply one global bound.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct StripeCapacity {
    pub max_subscription_pairs: usize,
    pub max_pending_keys: usize,
}

impl RouterBounds {
    pub const fn fixed_workload() -> Self {
        Self {
            max_subscription_pairs: FIXED_SUBSCRIPTION_PAIRS,
            max_pending_keys: FIXED_SUBSCRIPTION_PAIRS,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FixtureError(&'static str);

impl fmt::Display for FixtureError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.0)
    }
}

impl Error for FixtureError {}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RouteObservation {
    pub matched: u64,
    pub newly_dirty: u64,
    pub coalesced: u64,
    pub dropped: u64,
    /// Newly dirty recipients are returned for nonblocking delivery after the
    /// stripe lock has been released.
    pub newly_dirty_recipients: Vec<RecipientId>,
    pub lock_wait: Duration,
    pub latency: Duration,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct PendingKey {
    resource: ResourceId,
    recipient: RecipientId,
}

#[derive(Default)]
struct StripeState {
    subscriptions: HashMap<ResourceId, Vec<RecipientId>>,
    subscription_pair_count: usize,
    subscription_pair_capacity: usize,
    pending: HashSet<PendingKey>,
    pending_capacity: usize,
}

/// One bounded resource-indexed router with a configurable number of locks.
///
/// A route holds only the stripe selected by its resource. A hot resource still
/// performs its unavoidable local `O(subscribers)` work; resources mapped to a
/// different stripe do not share that lock. Fixed stripes can still collide,
/// which the workload reports explicitly.
pub struct StripedRouter {
    stripes: Box<[Mutex<StripeState>]>,
}

impl StripedRouter {
    pub fn fixed_fixture(stripe_count: usize) -> Result<Self, FixtureError> {
        let hot = (0..HOT_RECIPIENTS).map(|recipient| {
            (
                HOT_RESOURCE,
                RecipientId(u64::try_from(recipient).expect("recipient fits in u64")),
            )
        });
        let quiet = (1..=QUIET_RESOURCES).map(|resource| {
            (
                ResourceId(u64::try_from(resource).expect("resource fits in u64")),
                RecipientId(
                    u64::try_from(HOT_RECIPIENTS + resource - 1).expect("recipient fits in u64"),
                ),
            )
        });
        Self::from_subscriptions(
            stripe_count,
            RouterBounds::fixed_workload(),
            hot.chain(quiet),
        )
    }

    pub fn from_subscriptions(
        stripe_count: usize,
        bounds: RouterBounds,
        subscriptions: impl IntoIterator<Item = (ResourceId, RecipientId)>,
    ) -> Result<Self, FixtureError> {
        if stripe_count == 0 {
            return Err(FixtureError("stripe count must be non-zero"));
        }
        if bounds.max_subscription_pairs == 0 || bounds.max_pending_keys == 0 {
            return Err(FixtureError("router bounds must be non-zero"));
        }

        let mut staged = vec![HashMap::<ResourceId, Vec<RecipientId>>::new(); stripe_count];
        let mut unique = HashSet::new();
        for (resource, recipient) in subscriptions {
            if !unique.insert(PendingKey {
                resource,
                recipient,
            }) {
                continue;
            }
            if unique.len() > bounds.max_subscription_pairs {
                return Err(FixtureError("subscription-pair bound exceeded"));
            }
            let stripe = stripe_index(resource, stripe_count);
            staged[stripe].entry(resource).or_default().push(recipient);
        }
        if unique.len() > bounds.max_pending_keys {
            return Err(FixtureError(
                "pending bound cannot hold one key per subscription pair",
            ));
        }

        let capacities: Vec<_> = staged
            .iter()
            .map(|subscriptions| {
                let count = subscriptions.values().map(Vec::len).sum();
                StripeCapacity {
                    max_subscription_pairs: count,
                    max_pending_keys: count,
                }
            })
            .collect();
        let router = Self::empty(capacities)?;
        for subscriptions in staged {
            for (resource, recipients) in subscriptions {
                for recipient in recipients {
                    if !router.subscribe(resource, recipient)? {
                        return Err(FixtureError("duplicate staged subscription pair"));
                    }
                }
            }
        }
        Ok(router)
    }

    /// Create an empty dynamic router with one explicit capacity entry per
    /// stripe. The sum is the configured router bound; capacity is never
    /// multiplied implicitly by `stripe_count`.
    pub fn empty(
        capacities: impl IntoIterator<Item = StripeCapacity>,
    ) -> Result<Self, FixtureError> {
        let capacities: Vec<_> = capacities.into_iter().collect();
        if capacities.is_empty() {
            return Err(FixtureError("at least one stripe capacity is required"));
        }
        if capacities
            .iter()
            .any(|capacity| capacity.max_pending_keys < capacity.max_subscription_pairs)
        {
            return Err(FixtureError(
                "each stripe needs one pending slot per subscription pair",
            ));
        }
        let stripes = capacities
            .into_iter()
            .map(|capacity| {
                Mutex::new(StripeState {
                    subscriptions: HashMap::new(),
                    subscription_pair_count: 0,
                    subscription_pair_capacity: capacity.max_subscription_pairs,
                    pending: HashSet::with_capacity(capacity.max_pending_keys),
                    pending_capacity: capacity.max_pending_keys,
                })
            })
            .collect();
        Ok(Self { stripes })
    }

    pub fn stripe_count(&self) -> usize {
        self.stripes.len()
    }

    pub fn subscription_pair_count(&self) -> usize {
        self.stripes
            .iter()
            .map(|stripe| {
                stripe
                    .lock()
                    .unwrap_or_else(std::sync::PoisonError::into_inner)
                    .subscription_pair_count
            })
            .sum()
    }

    pub fn configured_capacity(&self) -> StripeCapacity {
        self.stripes
            .iter()
            .fold(StripeCapacity::default(), |mut total, stripe| {
                let stripe = stripe
                    .lock()
                    .unwrap_or_else(std::sync::PoisonError::into_inner);
                total.max_subscription_pairs += stripe.subscription_pair_capacity;
                total.max_pending_keys += stripe.pending_capacity;
                total
            })
    }

    pub fn stripe_for(&self, resource: ResourceId) -> usize {
        stripe_index(resource, self.stripes.len())
    }

    /// Register one exact pair under only its resource stripe.
    pub fn subscribe(
        &self,
        resource: ResourceId,
        recipient: RecipientId,
    ) -> Result<bool, FixtureError> {
        let mut stripe = self.stripes[self.stripe_for(resource)]
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if stripe
            .subscriptions
            .get(&resource)
            .is_some_and(|recipients| recipients.contains(&recipient))
        {
            return Ok(false);
        }
        if stripe.subscription_pair_count >= stripe.subscription_pair_capacity {
            return Err(FixtureError("stripe subscription-pair bound exceeded"));
        }
        stripe
            .subscriptions
            .entry(resource)
            .or_default()
            .push(recipient);
        stripe.subscription_pair_count += 1;
        Ok(true)
    }

    /// Remove one exact pair and any coalesced dirty state it owns.
    pub fn unsubscribe(&self, resource: ResourceId, recipient: RecipientId) -> bool {
        let mut stripe = self.stripes[self.stripe_for(resource)]
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let Some(recipients) = stripe.subscriptions.get_mut(&resource) else {
            return false;
        };
        let Some(position) = recipients
            .iter()
            .position(|candidate| *candidate == recipient)
        else {
            return false;
        };
        recipients.swap_remove(position);
        if recipients.is_empty() {
            stripe.subscriptions.remove(&resource);
        }
        stripe.subscription_pair_count -= 1;
        stripe.pending.remove(&PendingKey {
            resource,
            recipient,
        });
        true
    }

    /// Clear before an authoritative read. A later route can establish a new
    /// dirty key without being erased when that older read completes.
    pub fn begin_authoritative_read(&self, resource: ResourceId, recipient: RecipientId) -> bool {
        self.stripes[self.stripe_for(resource)]
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .pending
            .remove(&PendingKey {
                resource,
                recipient,
            })
    }

    /// Restore one dirty key after an authoritative read fails, without
    /// rerouting the resource to other recipients.
    pub fn restore_after_failed_read(
        &self,
        resource: ResourceId,
        recipient: RecipientId,
    ) -> Result<bool, FixtureError> {
        let mut stripe = self.stripes[self.stripe_for(resource)]
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if !stripe
            .subscriptions
            .get(&resource)
            .is_some_and(|recipients| recipients.contains(&recipient))
        {
            return Ok(false);
        }
        let key = PendingKey {
            resource,
            recipient,
        };
        if stripe.pending.contains(&key) {
            return Ok(false);
        }
        if stripe.pending.len() >= stripe.pending_capacity {
            return Err(FixtureError("stripe pending-key bound exceeded"));
        }
        stripe.pending.insert(key);
        Ok(true)
    }

    pub fn route(&self, resource: ResourceId) -> RouteObservation {
        self.route_after(resource, || {})
    }

    fn route_after(&self, resource: ResourceId, before_lock: impl FnOnce()) -> RouteObservation {
        let started = Instant::now();
        before_lock();
        let waiting = Instant::now();
        let mut stripe = self.stripes[self.stripe_for(resource)]
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let lock_wait = waiting.elapsed();

        let mut observation = RouteObservation {
            lock_wait,
            ..RouteObservation::default()
        };
        let StripeState {
            subscriptions,
            pending,
            pending_capacity,
            ..
        } = &mut *stripe;
        let Some(recipients) = subscriptions.get(&resource) else {
            observation.latency = started.elapsed();
            return observation;
        };
        for &recipient in recipients {
            observation.matched += 1;
            let key = PendingKey {
                resource,
                recipient,
            };
            if pending.contains(&key) {
                observation.coalesced += 1;
            } else if pending.len() >= *pending_capacity {
                observation.dropped += 1;
            } else {
                pending.insert(key);
                observation.newly_dirty += 1;
                observation.newly_dirty_recipients.push(recipient);
            }
        }
        observation.latency = started.elapsed();
        observation
    }

    pub fn pending_count(&self) -> usize {
        self.stripes
            .iter()
            .map(|stripe| {
                stripe
                    .lock()
                    .unwrap_or_else(std::sync::PoisonError::into_inner)
                    .pending
                    .len()
            })
            .sum()
    }

    pub fn clear_pending(&self) -> usize {
        self.stripes
            .iter()
            .map(|stripe| {
                let mut stripe = stripe
                    .lock()
                    .unwrap_or_else(std::sync::PoisonError::into_inner);
                let removed = stripe.pending.len();
                stripe.pending.clear();
                removed
            })
            .sum()
    }
}

fn stripe_index(resource: ResourceId, stripe_count: usize) -> usize {
    usize::try_from(resource.0 % u64::try_from(stripe_count).expect("stripe count fits in u64"))
        .expect("stripe index fits in usize")
}

#[derive(Debug, Clone, Copy)]
pub struct FanoutWorkload {
    pub repeats: usize,
    pub hot_producers: usize,
    pub hot_routes_per_producer: usize,
    pub quiet_producers: usize,
    pub quiet_cycles: usize,
}

impl FanoutWorkload {
    pub const fn release_matrix() -> Self {
        Self {
            repeats: 7,
            hot_producers: 4,
            hot_routes_per_producer: 256,
            quiet_producers: 4,
            quiet_cycles: 256,
        }
    }

    #[cfg(test)]
    const fn focused() -> Self {
        Self {
            repeats: 2,
            hot_producers: 4,
            hot_routes_per_producer: 8,
            quiet_producers: 4,
            quiet_cycles: 8,
        }
    }

    fn validate(self) -> Result<Self, FixtureError> {
        if self.repeats == 0
            || self.hot_producers == 0
            || self.hot_routes_per_producer == 0
            || self.quiet_producers == 0
            || self.quiet_cycles == 0
        {
            return Err(FixtureError("all workload dimensions must be non-zero"));
        }
        if !QUIET_RESOURCES.is_multiple_of(self.quiet_producers) {
            return Err(FixtureError(
                "quiet producers must divide the fixed quiet resource count",
            ));
        }
        Ok(self)
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct DurationDistribution {
    pub samples: u64,
    pub p50_ns: u64,
    pub p95_ns: u64,
    pub p99_ns: u64,
    pub max_ns: u64,
}

impl DurationDistribution {
    fn from_samples(samples: &mut [Duration]) -> Self {
        if samples.is_empty() {
            return Self::default();
        }
        samples.sort_unstable();
        Self {
            samples: samples.len() as u64,
            p50_ns: nanos(samples[nearest_rank(samples.len(), 50)]),
            p95_ns: nanos(samples[nearest_rank(samples.len(), 95)]),
            p99_ns: nanos(samples[nearest_rank(samples.len(), 99)]),
            max_ns: nanos(*samples.last().expect("non-empty samples")),
        }
    }
}

fn nearest_rank(length: usize, percentile: usize) -> usize {
    (length * percentile).div_ceil(100).saturating_sub(1)
}

fn nanos(duration: Duration) -> u64 {
    u64::try_from(duration.as_nanos()).unwrap_or(u64::MAX)
}

fn wait_for_shared_start(signal: &OnceLock<Instant>) -> Instant {
    loop {
        if let Some(started) = signal.get() {
            return *started;
        }
        thread::yield_now();
    }
}

#[derive(Debug, Clone)]
pub struct FanoutMatrixRow {
    pub stripes: usize,
    pub repeats: usize,
    pub subscription_pairs_per_repeat: u64,
    pub pending_capacity_per_repeat: u64,
    pub hot_routes: u64,
    pub hot_raw_matches: u64,
    pub quiet_routes: u64,
    pub quiet_raw_matches: u64,
    pub newly_dirty: u64,
    pub coalesced: u64,
    pub dropped: u64,
    pub cleared_pending: u64,
    pub quiet_collision_routes: u64,
    pub quiet_disjoint_routes: u64,
    pub quiet_latency: DurationDistribution,
    pub quiet_collision_latency: DurationDistribution,
    pub quiet_disjoint_latency: DurationDistribution,
    pub quiet_lock_wait: DurationDistribution,
    pub hot_lock_wait: DurationDistribution,
    pub hot_routes_per_second: f64,
    pub hot_matches_per_second: f64,
}

#[derive(Default)]
struct WorkerResult {
    hot: bool,
    observations: Vec<RouteObservation>,
    collisions: Vec<bool>,
    elapsed: Duration,
}

pub fn run_fanout_matrix(workload: FanoutWorkload) -> Result<Vec<FanoutMatrixRow>, FixtureError> {
    let workload = workload.validate()?;
    [1, 8, 64]
        .into_iter()
        .map(|stripes| run_fanout_variant(stripes, workload))
        .collect()
}

pub fn run_fanout_variant(
    stripe_count: usize,
    workload: FanoutWorkload,
) -> Result<FanoutMatrixRow, FixtureError> {
    let workload = workload.validate()?;
    let mut hot_observations = Vec::new();
    let mut quiet_observations = Vec::new();
    let mut quiet_collisions = Vec::new();
    let mut hot_elapsed = Duration::ZERO;
    let mut cleared_pending = 0_u64;

    for _ in 0..workload.repeats {
        let router = StripedRouter::fixed_fixture(stripe_count)?;
        if router.configured_capacity()
            != (StripeCapacity {
                max_subscription_pairs: FIXED_SUBSCRIPTION_PAIRS,
                max_pending_keys: FIXED_SUBSCRIPTION_PAIRS,
            })
        {
            return Err(FixtureError("fixed variants have unequal total capacity"));
        }
        let workers = workload.hot_producers + workload.quiet_producers;
        let barrier = Barrier::new(workers + 1);
        let start_signal = OnceLock::new();
        let results = thread::scope(|scope| {
            let mut handles = Vec::with_capacity(workers);
            for _ in 0..workload.hot_producers {
                handles.push(scope.spawn(|| {
                    barrier.wait();
                    let started = wait_for_shared_start(&start_signal);
                    let observations = (0..workload.hot_routes_per_producer)
                        .map(|_| router.route(HOT_RESOURCE))
                        .collect();
                    WorkerResult {
                        hot: true,
                        observations,
                        collisions: Vec::new(),
                        elapsed: started.elapsed(),
                    }
                }));
            }
            for producer in 0..workload.quiet_producers {
                let barrier = &barrier;
                let router = &router;
                let start_signal = &start_signal;
                handles.push(scope.spawn(move || {
                    barrier.wait();
                    let started = wait_for_shared_start(start_signal);
                    let mut observations = Vec::with_capacity(
                        workload.quiet_cycles * QUIET_RESOURCES / workload.quiet_producers,
                    );
                    let mut collisions = Vec::with_capacity(observations.capacity());
                    for _ in 0..workload.quiet_cycles {
                        for resource in
                            ((producer + 1)..=QUIET_RESOURCES).step_by(workload.quiet_producers)
                        {
                            let resource = ResourceId(
                                u64::try_from(resource).expect("quiet resource fits in u64"),
                            );
                            collisions.push(
                                router.stripe_for(resource) == router.stripe_for(HOT_RESOURCE),
                            );
                            observations.push(router.route(resource));
                        }
                    }
                    WorkerResult {
                        hot: false,
                        observations,
                        collisions,
                        elapsed: started.elapsed(),
                    }
                }));
            }
            barrier.wait();
            start_signal
                .set(Instant::now())
                .expect("fan-out start signal is set once");
            handles
                .into_iter()
                .map(|handle| handle.join().expect("fan-out worker panicked"))
                .collect::<Vec<_>>()
        });

        hot_elapsed += results
            .iter()
            .filter(|result| result.hot)
            .map(|result| result.elapsed)
            .max()
            .expect("at least one hot worker");
        for mut result in results {
            if result.hot {
                hot_observations.append(&mut result.observations);
            } else {
                quiet_observations.append(&mut result.observations);
                quiet_collisions.append(&mut result.collisions);
            }
        }

        if router.pending_count() != FIXED_SUBSCRIPTION_PAIRS {
            return Err(FixtureError("unexpected final pending-key count"));
        }
        cleared_pending += router.clear_pending() as u64;
        if router.pending_count() != 0 {
            return Err(FixtureError("router teardown left pending keys"));
        }
    }

    let hot_routes = hot_observations.len() as u64;
    let quiet_routes = quiet_observations.len() as u64;
    let hot_raw_matches = hot_observations
        .iter()
        .map(|observation| observation.matched)
        .sum();
    let quiet_raw_matches = quiet_observations
        .iter()
        .map(|observation| observation.matched)
        .sum();
    let newly_dirty = hot_observations
        .iter()
        .chain(&quiet_observations)
        .map(|observation| observation.newly_dirty)
        .sum();
    let coalesced = hot_observations
        .iter()
        .chain(&quiet_observations)
        .map(|observation| observation.coalesced)
        .sum();
    let dropped = hot_observations
        .iter()
        .chain(&quiet_observations)
        .map(|observation| observation.dropped)
        .sum();

    let mut quiet_latency: Vec<_> = quiet_observations
        .iter()
        .map(|observation| observation.latency)
        .collect();
    let mut quiet_collision_latency: Vec<_> = quiet_observations
        .iter()
        .zip(&quiet_collisions)
        .filter_map(|(observation, collision)| collision.then_some(observation.latency))
        .collect();
    let mut quiet_disjoint_latency: Vec<_> = quiet_observations
        .iter()
        .zip(&quiet_collisions)
        .filter_map(|(observation, collision)| (!collision).then_some(observation.latency))
        .collect();
    let mut quiet_lock_wait: Vec<_> = quiet_observations
        .iter()
        .map(|observation| observation.lock_wait)
        .collect();
    let mut hot_lock_wait: Vec<_> = hot_observations
        .iter()
        .map(|observation| observation.lock_wait)
        .collect();
    let hot_seconds = hot_elapsed.as_secs_f64();

    Ok(FanoutMatrixRow {
        stripes: stripe_count,
        repeats: workload.repeats,
        subscription_pairs_per_repeat: FIXED_SUBSCRIPTION_PAIRS as u64,
        pending_capacity_per_repeat: FIXED_SUBSCRIPTION_PAIRS as u64,
        hot_routes,
        hot_raw_matches,
        quiet_routes,
        quiet_raw_matches,
        newly_dirty,
        coalesced,
        dropped,
        cleared_pending,
        quiet_collision_routes: quiet_collisions
            .iter()
            .filter(|collision| **collision)
            .count() as u64,
        quiet_disjoint_routes: quiet_collisions
            .iter()
            .filter(|collision| !**collision)
            .count() as u64,
        quiet_latency: DurationDistribution::from_samples(&mut quiet_latency),
        quiet_collision_latency: DurationDistribution::from_samples(&mut quiet_collision_latency),
        quiet_disjoint_latency: DurationDistribution::from_samples(&mut quiet_disjoint_latency),
        quiet_lock_wait: DurationDistribution::from_samples(&mut quiet_lock_wait),
        hot_lock_wait: DurationDistribution::from_samples(&mut hot_lock_wait),
        hot_routes_per_second: hot_routes as f64 / hot_seconds,
        hot_matches_per_second: hot_raw_matches as f64 / hot_seconds,
    })
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, mpsc};

    use super::*;

    fn assert_exact_counts(row: &FanoutMatrixRow, workload: FanoutWorkload) {
        let repeats = workload.repeats as u64;
        let expected_hot_routes =
            repeats * workload.hot_producers as u64 * workload.hot_routes_per_producer as u64;
        let expected_quiet_routes = repeats * workload.quiet_cycles as u64 * QUIET_RESOURCES as u64;
        let expected_new = repeats * FIXED_SUBSCRIPTION_PAIRS as u64;
        let expected_raw = expected_hot_routes * HOT_RECIPIENTS as u64 + expected_quiet_routes;

        assert_eq!(row.hot_routes, expected_hot_routes);
        assert_eq!(
            row.subscription_pairs_per_repeat,
            FIXED_SUBSCRIPTION_PAIRS as u64
        );
        assert_eq!(
            row.pending_capacity_per_repeat,
            FIXED_SUBSCRIPTION_PAIRS as u64
        );
        assert_eq!(
            row.hot_raw_matches,
            expected_hot_routes * HOT_RECIPIENTS as u64
        );
        assert_eq!(row.quiet_routes, expected_quiet_routes);
        assert_eq!(row.quiet_raw_matches, expected_quiet_routes);
        assert_eq!(row.newly_dirty, expected_new);
        assert_eq!(row.coalesced, expected_raw - expected_new);
        assert_eq!(row.dropped, 0);
        assert_eq!(row.cleared_pending, expected_new);
        assert_eq!(
            row.quiet_collision_routes + row.quiet_disjoint_routes,
            expected_quiet_routes
        );
        assert_eq!(row.quiet_latency.samples, expected_quiet_routes);
        assert_eq!(row.quiet_lock_wait.samples, expected_quiet_routes);
        assert_eq!(row.hot_lock_wait.samples, expected_hot_routes);
    }

    #[test]
    fn every_stripe_setting_preserves_exact_delivery_coalescing_and_teardown() {
        let workload = FanoutWorkload::focused();
        for row in run_fanout_matrix(workload).unwrap() {
            assert_exact_counts(&row, workload);
            let expected_collision_fraction = QUIET_RESOURCES / row.stripes;
            assert_eq!(
                row.quiet_collision_routes,
                (workload.repeats * workload.quiet_cycles * expected_collision_fraction) as u64
            );
        }
    }

    #[test]
    fn a_disjoint_stripe_routes_while_the_hot_stripe_is_held() {
        let router = Arc::new(StripedRouter::fixed_fixture(8).unwrap());
        let route_router = Arc::clone(&router);
        let hot_stripe = router.stripe_for(HOT_RESOURCE);
        let held = router.stripes[hot_stripe]
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let (attempted_tx, attempted_rx) = mpsc::channel();
        let (done_tx, done_rx) = mpsc::channel();
        let worker = thread::spawn(move || {
            let observation = route_router.route_after(ResourceId(1), || {
                attempted_tx.send(()).unwrap();
            });
            done_tx.send(observation).unwrap();
        });

        attempted_rx.recv().unwrap();
        let observation = done_rx
            .recv_timeout(Duration::from_secs(1))
            .expect("different stripe must not wait on held hot stripe");
        assert_eq!(observation.matched, 1);
        drop(held);
        worker.join().unwrap();
    }

    #[test]
    fn a_colliding_resource_cannot_route_while_the_hot_stripe_is_held() {
        let router = Arc::new(StripedRouter::fixed_fixture(8).unwrap());
        let route_router = Arc::clone(&router);
        let hot_stripe = router.stripe_for(HOT_RESOURCE);
        let held = router.stripes[hot_stripe]
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let (attempted_tx, attempted_rx) = mpsc::channel();
        let (done_tx, done_rx) = mpsc::channel();
        let worker = thread::spawn(move || {
            let observation = route_router.route_after(ResourceId(8), || {
                attempted_tx.send(()).unwrap();
            });
            done_tx.send(observation).unwrap();
        });

        attempted_rx.recv().unwrap();
        assert!(matches!(done_rx.try_recv(), Err(mpsc::TryRecvError::Empty)));
        drop(held);
        assert_eq!(done_rx.recv().unwrap().matched, 1);
        worker.join().unwrap();
    }

    #[test]
    fn registration_and_pending_state_are_bounded() {
        let too_small = RouterBounds {
            max_subscription_pairs: 1,
            max_pending_keys: 1,
        };
        assert_eq!(
            StripedRouter::from_subscriptions(
                1,
                too_small,
                [
                    (ResourceId(1), RecipientId(1)),
                    (ResourceId(2), RecipientId(2)),
                ],
            )
            .err(),
            Some(FixtureError("subscription-pair bound exceeded"))
        );

        let router = StripedRouter::fixed_fixture(64).unwrap();
        assert_eq!(router.subscription_pair_count(), FIXED_SUBSCRIPTION_PAIRS);
        assert_eq!(
            router.configured_capacity(),
            StripeCapacity {
                max_subscription_pairs: FIXED_SUBSCRIPTION_PAIRS,
                max_pending_keys: FIXED_SUBSCRIPTION_PAIRS,
            }
        );
        assert_eq!(router.pending_count(), 0);
    }

    #[test]
    fn dynamic_adapter_returns_new_recipients_and_clears_exactly_after_read() {
        let mut capacities = vec![StripeCapacity::default(); 8];
        capacities[1] = StripeCapacity {
            max_subscription_pairs: 2,
            max_pending_keys: 2,
        };
        let router = StripedRouter::empty(capacities).unwrap();
        let resource = ResourceId(1);
        let first = RecipientId(10);
        let second = RecipientId(11);

        assert!(router.subscribe(resource, first).unwrap());
        assert!(router.subscribe(resource, second).unwrap());
        assert!(!router.subscribe(resource, second).unwrap());
        let initial = router.route(resource);
        assert_eq!(initial.matched, 2);
        assert_eq!(initial.newly_dirty_recipients, vec![first, second]);
        assert_eq!(router.route(resource).coalesced, 2);

        assert!(router.begin_authoritative_read(resource, first));
        assert!(router.restore_after_failed_read(resource, first).unwrap());
        assert!(!router.restore_after_failed_read(resource, first).unwrap());
        assert!(router.begin_authoritative_read(resource, first));
        let after_read = router.route(resource);
        assert_eq!(after_read.newly_dirty_recipients, vec![first]);
        assert_eq!(after_read.coalesced, 1);
        assert!(router.unsubscribe(resource, second));
        assert!(!router.unsubscribe(resource, second));
        assert_eq!(router.subscription_pair_count(), 1);
        assert_eq!(router.pending_count(), 1);
        assert!(!router.restore_after_failed_read(resource, second).unwrap());
    }

    #[test]
    #[ignore = "descriptive fixed release matrix"]
    fn release_hot_fanout_matrix() {
        let workload = FanoutWorkload::release_matrix();
        println!(
            "hot_recipients={} quiet_resources={} repeats={} hot_producers={} hot_routes_per_producer={} quiet_producers={} quiet_cycles={}",
            HOT_RECIPIENTS,
            QUIET_RESOURCES,
            workload.repeats,
            workload.hot_producers,
            workload.hot_routes_per_producer,
            workload.quiet_producers,
            workload.quiet_cycles,
        );
        println!(
            "stripes | quiet routes collision/disjoint | quiet latency p50/p95/p99/max us | quiet wait p50/p95/p99/max us | hot wait p95/p99/max us | hot routes/s | hot matches/s"
        );
        for row in run_fanout_matrix(workload).unwrap() {
            assert_exact_counts(&row, workload);
            println!(
                "{} | {} {}/{} | {:.3}/{:.3}/{:.3}/{:.3} | {:.3}/{:.3}/{:.3}/{:.3} | {:.3}/{:.3}/{:.3} | {:.0} | {:.0}",
                row.stripes,
                row.quiet_routes,
                row.quiet_collision_routes,
                row.quiet_disjoint_routes,
                row.quiet_latency.p50_ns as f64 / 1_000.0,
                row.quiet_latency.p95_ns as f64 / 1_000.0,
                row.quiet_latency.p99_ns as f64 / 1_000.0,
                row.quiet_latency.max_ns as f64 / 1_000.0,
                row.quiet_lock_wait.p50_ns as f64 / 1_000.0,
                row.quiet_lock_wait.p95_ns as f64 / 1_000.0,
                row.quiet_lock_wait.p99_ns as f64 / 1_000.0,
                row.quiet_lock_wait.max_ns as f64 / 1_000.0,
                row.hot_lock_wait.p95_ns as f64 / 1_000.0,
                row.hot_lock_wait.p99_ns as f64 / 1_000.0,
                row.hot_lock_wait.max_ns as f64 / 1_000.0,
                row.hot_routes_per_second,
                row.hot_matches_per_second,
            );
        }
    }
}
