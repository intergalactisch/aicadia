//! Fixed game scenarios and the strategy-independent coverage oracle.
//!
//! The oracle says which connected hosts need a live stale hint for the game
//! scenario. It deliberately knows nothing about resource routing. A strategy may
//! therefore be fast and still fail coverage, or cover the scenario while waking
//! irrelevant hosts. PostgreSQL, gateway and MCP runners report their observations
//! through the structs at the end of this module.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::strategy::{ChangeScope, InterestSpec};

pub const REAL_SUBSCRIBERS: usize = 32;
pub const CHANGES_PER_STRATEGY: usize = 100;

pub type ScenarioHostId = u32;

const fn fixture_id(value: u128) -> Uuid {
    Uuid::from_u128(value)
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScenarioFixture {
    pub world_id: Uuid,
    pub north_area_id: Uuid,
    pub south_area_id: Uuid,
    pub place_a_id: Uuid,
    pub place_b_id: Uuid,
    pub quiet_place_id: Uuid,
    pub remote_place_id: Uuid,
    pub selected_tree_id: Uuid,
    pub unselected_local_entity_id: Uuid,
    pub discovered_entity_id: Uuid,
    pub bomb_entity_id: Uuid,
    pub place_b_entity_id: Uuid,
    pub quiet_entity_id: Uuid,
    pub hosts: Vec<HostInterest>,
}

impl ScenarioFixture {
    /// Four cohorts of eight hosts keep the real MCP tier small but make local,
    /// regional, World-wide and quiet-subject outcomes independently observable.
    pub fn fixed() -> Self {
        let north_area_id = fixture_id(0x10);
        let south_area_id = fixture_id(0x20);
        let place_a_id = fixture_id(0xa0);
        let place_b_id = fixture_id(0xb0);
        let quiet_place_id = fixture_id(0xc0);
        let remote_place_id = fixture_id(0xd0);
        let selected_tree_id = fixture_id(0x100);

        let hosts = (0..REAL_SUBSCRIBERS as ScenarioHostId)
            .map(|host_id| {
                let (area_id, place_id) = match host_id / 8 {
                    0 => (north_area_id, place_a_id),
                    1 => (north_area_id, place_b_id),
                    2 => (south_area_id, quiet_place_id),
                    _ => (south_area_id, remote_place_id),
                };
                // Only half of Place A explicitly selected the Tree. This makes
                // exact-only routing's discovery gap visible without inventing
                // semantic relevance in the router.
                let selected_entity_ids =
                    if (host_id < 8 && host_id % 2 == 0) || matches!(host_id, 24 | 26) {
                        vec![selected_tree_id]
                    } else {
                        Vec::new()
                    };
                HostInterest {
                    host_id,
                    area_id,
                    place_id,
                    selected_entity_ids,
                }
            })
            .collect();

        Self {
            world_id: fixture_id(0x01),
            north_area_id,
            south_area_id,
            place_a_id,
            place_b_id,
            quiet_place_id,
            remote_place_id,
            selected_tree_id,
            unselected_local_entity_id: fixture_id(0x101),
            discovered_entity_id: fixture_id(0x102),
            bomb_entity_id: fixture_id(0x103),
            place_b_entity_id: fixture_id(0x104),
            quiet_entity_id: fixture_id(0x105),
            hosts,
        }
    }

    pub fn program(&self) -> ScenarioProgram {
        use ChangeClass::*;

        let local_a = |class: ChangeClass, entity_id: Uuid, repetitions: usize| ScenarioStep {
            name: class.as_str(),
            class,
            repetitions,
            entity_id: Some(entity_id),
            audience: if matches!(
                class,
                SelectedTree
                    | HotEntity
                    | QuietIsolationHotPrelude
                    | SlowConsumerCoalescing
                    | ListenerLoss
                    | ReconnectRefetch
            ) {
                SemanticAudience::PlacesAndSelectedEntity {
                    place_ids: vec![self.place_a_id],
                    entity_id,
                }
            } else {
                SemanticAudience::Places(vec![self.place_a_id])
            },
            actions_before: Vec::new(),
            actions_after: Vec::new(),
            slow_consumer: None,
        };

        let cases = vec![
            ScenarioCase::one(local_a(SelectedTree, self.selected_tree_id, 1)),
            ScenarioCase::one(local_a(
                UnselectedLocalEntity,
                self.unselected_local_entity_id,
                1,
            )),
            ScenarioCase::one(local_a(NewEntityDiscovery, self.discovered_entity_id, 1)),
            ScenarioCase::one(ScenarioStep {
                name: BombPlacesAAndB.as_str(),
                class: BombPlacesAAndB,
                repetitions: 1,
                entity_id: Some(self.bomb_entity_id),
                audience: SemanticAudience::Places(vec![self.place_a_id, self.place_b_id]),
                actions_before: Vec::new(),
                actions_after: Vec::new(),
                slow_consumer: None,
            }),
            ScenarioCase::one(ScenarioStep {
                name: RegionalEffect.as_str(),
                class: RegionalEffect,
                repetitions: 1,
                entity_id: None,
                audience: SemanticAudience::Area(self.north_area_id),
                actions_before: Vec::new(),
                actions_after: Vec::new(),
                slow_consumer: None,
            }),
            ScenarioCase::one(ScenarioStep {
                name: WorldScopeEffect.as_str(),
                class: WorldScopeEffect,
                repetitions: 1,
                entity_id: None,
                audience: SemanticAudience::World,
                actions_before: Vec::new(),
                actions_after: Vec::new(),
                slow_consumer: None,
            }),
            ScenarioCase {
                name: "move-a-to-b",
                steps: vec![
                    ScenarioStep {
                        name: "move-activity-a-to-b",
                        class: MoveAtoB,
                        repetitions: 1,
                        entity_id: None,
                        audience: SemanticAudience::Places(vec![self.place_a_id, self.place_b_id]),
                        actions_before: Vec::new(),
                        actions_after: vec![LifecycleAction::Move {
                            host_id: 0,
                            area_id: self.north_area_id,
                            place_id: self.place_b_id,
                        }],
                        slow_consumer: None,
                    },
                    ScenarioStep {
                        name: "post-move-place-b-change",
                        class: MoveAtoB,
                        repetitions: 1,
                        entity_id: Some(self.place_b_entity_id),
                        audience: SemanticAudience::Places(vec![self.place_b_id]),
                        actions_before: Vec::new(),
                        actions_after: Vec::new(),
                        slow_consumer: None,
                    },
                ],
            },
            ScenarioCase::one(local_a(HotEntity, self.selected_tree_id, 40)),
            ScenarioCase::one(local_a(HotPlace, self.unselected_local_entity_id, 25)),
            ScenarioCase {
                name: "quiet-isolation",
                steps: vec![
                    local_a(QuietIsolationHotPrelude, self.selected_tree_id, 10),
                    ScenarioStep {
                        name: "quiet-place-change",
                        class: QuietIsolation,
                        repetitions: 1,
                        entity_id: Some(self.quiet_entity_id),
                        audience: SemanticAudience::Places(vec![self.quiet_place_id]),
                        actions_before: Vec::new(),
                        actions_after: Vec::new(),
                        slow_consumer: None,
                    },
                ],
            },
            ScenarioCase::one(ScenarioStep {
                slow_consumer: Some(0),
                ..local_a(SlowConsumerCoalescing, self.selected_tree_id, 14)
            }),
            ScenarioCase {
                name: "listener-loss-reconnect-refetch",
                steps: vec![
                    ScenarioStep {
                        actions_before: vec![LifecycleAction::Disconnect { host_id: 0 }],
                        ..local_a(ListenerLoss, self.selected_tree_id, 1)
                    },
                    ScenarioStep {
                        actions_before: vec![LifecycleAction::ReconnectAndRefetch { host_id: 0 }],
                        ..local_a(ReconnectRefetch, self.selected_tree_id, 1)
                    },
                ],
            },
        ];

        let program = ScenarioProgram { cases };
        debug_assert_eq!(program.committed_change_count(), CHANGES_PER_STRATEGY);
        program
    }

    pub fn area_ids(&self) -> [Uuid; 2] {
        [self.north_area_id, self.south_area_id]
    }

    pub fn place_ids(&self) -> [Uuid; 4] {
        [
            self.place_a_id,
            self.place_b_id,
            self.quiet_place_id,
            self.remote_place_id,
        ]
    }

    pub fn existing_entity_ids(&self) -> [Uuid; 5] {
        [
            self.selected_tree_id,
            self.unselected_local_entity_id,
            self.bomb_entity_id,
            self.place_b_entity_id,
            self.quiet_entity_id,
        ]
    }

    pub fn all_entity_ids(&self) -> [Uuid; 6] {
        let [tree, unselected, bomb, place_b, quiet] = self.existing_entity_ids();
        [
            tree,
            unselected,
            self.discovered_entity_id,
            bomb,
            place_b,
            quiet,
        ]
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HostInterest {
    pub host_id: ScenarioHostId,
    pub area_id: Uuid,
    pub place_id: Uuid,
    pub selected_entity_ids: Vec<Uuid>,
}

impl HostInterest {
    pub fn interest_spec(&self) -> InterestSpec {
        InterestSpec {
            current_area_id: Some(self.area_id),
            current_place_id: Some(self.place_id),
            exact_entity_ids: self.selected_entity_ids.clone(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChangeClass {
    SelectedTree,
    UnselectedLocalEntity,
    NewEntityDiscovery,
    BombPlacesAAndB,
    RegionalEffect,
    WorldScopeEffect,
    MoveAtoB,
    HotEntity,
    HotPlace,
    QuietIsolationHotPrelude,
    QuietIsolation,
    SlowConsumerCoalescing,
    ListenerLoss,
    ReconnectRefetch,
}

impl ChangeClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SelectedTree => "selected-tree",
            Self::UnselectedLocalEntity => "unselected-local-entity",
            Self::NewEntityDiscovery => "new-entity-discovery",
            Self::BombPlacesAAndB => "bomb-places-a-b",
            Self::RegionalEffect => "regional-effect",
            Self::WorldScopeEffect => "world-scope-effect",
            Self::MoveAtoB => "move-a-to-b",
            Self::HotEntity => "hot-entity",
            Self::HotPlace => "hot-place",
            Self::QuietIsolationHotPrelude => "quiet-isolation-hot-prelude",
            Self::QuietIsolation => "quiet-isolation",
            Self::SlowConsumerCoalescing => "slow-consumer-coalescing",
            Self::ListenerLoss => "listener-loss",
            Self::ReconnectRefetch => "reconnect-refetch",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SemanticAudience {
    Places(Vec<Uuid>),
    PlacesAndSelectedEntity {
        place_ids: Vec<Uuid>,
        entity_id: Uuid,
    },
    Area(Uuid),
    World,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LifecycleAction {
    Disconnect {
        host_id: ScenarioHostId,
    },
    ReconnectAndRefetch {
        host_id: ScenarioHostId,
    },
    Move {
        host_id: ScenarioHostId,
        area_id: Uuid,
        place_id: Uuid,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ScenarioStep {
    pub name: &'static str,
    pub class: ChangeClass,
    pub repetitions: usize,
    pub entity_id: Option<Uuid>,
    pub audience: SemanticAudience,
    pub actions_before: Vec<LifecycleAction>,
    pub actions_after: Vec<LifecycleAction>,
    pub slow_consumer: Option<ScenarioHostId>,
}

impl ScenarioStep {
    /// Scope declared by the scenario, never inferred from Entity meaning.
    pub fn change_scope(&self) -> ChangeScope {
        match &self.audience {
            SemanticAudience::Places(_) | SemanticAudience::PlacesAndSelectedEntity { .. } => {
                ChangeScope::Local
            }
            SemanticAudience::Area(area_id) => ChangeScope::Area { area_id: *area_id },
            SemanticAudience::World => ChangeScope::World,
        }
    }

    pub fn affected_place_ids(&self) -> Vec<Uuid> {
        match &self.audience {
            SemanticAudience::Places(place_ids)
            | SemanticAudience::PlacesAndSelectedEntity { place_ids, .. } => place_ids.clone(),
            SemanticAudience::Area(_) | SemanticAudience::World => Vec::new(),
        }
    }

    pub fn change_scope_and_affected_places(&self) -> (ChangeScope, Vec<Uuid>) {
        (self.change_scope(), self.affected_place_ids())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ScenarioCase {
    pub name: &'static str,
    pub steps: Vec<ScenarioStep>,
}

impl ScenarioCase {
    fn one(step: ScenarioStep) -> Self {
        Self {
            name: step.name,
            steps: vec![step],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ScenarioProgram {
    pub cases: Vec<ScenarioCase>,
}

impl ScenarioProgram {
    pub fn committed_change_count(&self) -> usize {
        self.cases
            .iter()
            .flat_map(|case| &case.steps)
            .map(|step| step.repetitions)
            .sum()
    }
}

#[derive(Debug, Clone)]
struct OracleHostState {
    area_id: Uuid,
    place_id: Uuid,
    selected_entity_ids: BTreeSet<Uuid>,
    connected: bool,
}

/// Stateful because movement and reconnect change later expected outcomes.
pub struct ExpectedOutcomeOracle {
    hosts: BTreeMap<ScenarioHostId, OracleHostState>,
}

impl ExpectedOutcomeOracle {
    pub fn new(fixture: &ScenarioFixture) -> Self {
        Self {
            hosts: fixture
                .hosts
                .iter()
                .map(|host| {
                    (
                        host.host_id,
                        OracleHostState {
                            area_id: host.area_id,
                            place_id: host.place_id,
                            selected_entity_ids: host.selected_entity_ids.iter().copied().collect(),
                            connected: true,
                        },
                    )
                })
                .collect(),
        }
    }

    /// Apply lifecycle work that occurs before this step, then compute the
    /// strategy-independent game audience. The caller applies `actions_after`
    /// only after every repeated commit in the step has run.
    pub fn begin_step(&mut self, step: &ScenarioStep) -> ExpectedOutcome {
        let mut recovery_required = BTreeSet::new();
        for action in &step.actions_before {
            self.apply(action, &mut recovery_required);
        }

        let live_required = self
            .hosts
            .iter()
            .filter(|(_, host)| host.connected && audience_contains(&step.audience, host))
            .map(|(host_id, _)| *host_id)
            .collect::<BTreeSet<_>>();
        let irrelevant_if_woken = self
            .hosts
            .iter()
            .filter(|(host_id, host)| host.connected && !live_required.contains(host_id))
            .map(|(host_id, _)| *host_id)
            .collect();

        ExpectedOutcome {
            live_required,
            recovery_required,
            irrelevant_if_woken,
        }
    }

    pub fn finish_step(&mut self, step: &ScenarioStep) {
        let mut ignored_recovery = BTreeSet::new();
        for action in &step.actions_after {
            self.apply(action, &mut ignored_recovery);
        }
    }

    fn apply(
        &mut self,
        action: &LifecycleAction,
        recovery_required: &mut BTreeSet<ScenarioHostId>,
    ) {
        match *action {
            LifecycleAction::Disconnect { host_id } => {
                self.hosts
                    .get_mut(&host_id)
                    .expect("fixture host")
                    .connected = false;
            }
            LifecycleAction::ReconnectAndRefetch { host_id } => {
                self.hosts
                    .get_mut(&host_id)
                    .expect("fixture host")
                    .connected = true;
                recovery_required.insert(host_id);
            }
            LifecycleAction::Move {
                host_id,
                area_id,
                place_id,
            } => {
                let host = self.hosts.get_mut(&host_id).expect("fixture host");
                host.area_id = area_id;
                host.place_id = place_id;
            }
        }
    }
}

fn audience_contains(audience: &SemanticAudience, host: &OracleHostState) -> bool {
    match audience {
        SemanticAudience::Places(place_ids) => place_ids.contains(&host.place_id),
        SemanticAudience::PlacesAndSelectedEntity {
            place_ids,
            entity_id,
        } => place_ids.contains(&host.place_id) || host.selected_entity_ids.contains(entity_id),
        SemanticAudience::Area(area_id) => host.area_id == *area_id,
        SemanticAudience::World => true,
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExpectedOutcome {
    pub live_required: BTreeSet<ScenarioHostId>,
    pub recovery_required: BTreeSet<ScenarioHostId>,
    pub irrelevant_if_woken: BTreeSet<ScenarioHostId>,
}

impl ExpectedOutcome {
    pub fn evaluate(
        &self,
        live_notified: &BTreeSet<ScenarioHostId>,
        authoritative_refetched: &BTreeSet<ScenarioHostId>,
    ) -> CoverageVerdict {
        CoverageVerdict {
            missed_live_hosts: self
                .live_required
                .difference(live_notified)
                .copied()
                .collect(),
            missed_recovery_hosts: self
                .recovery_required
                .difference(authoritative_refetched)
                .copied()
                .collect(),
            irrelevant_wakeups: live_notified
                .intersection(&self.irrelevant_if_woken)
                .copied()
                .collect(),
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoverageVerdict {
    pub missed_live_hosts: BTreeSet<ScenarioHostId>,
    pub missed_recovery_hosts: BTreeSet<ScenarioHostId>,
    pub irrelevant_wakeups: BTreeSet<ScenarioHostId>,
}

impl CoverageVerdict {
    pub fn game_coverage_passes(&self) -> bool {
        self.missed_live_hosts.is_empty() && self.missed_recovery_hosts.is_empty()
    }
}

/// Actual observations supplied by the real World seam for one committed step.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorldStepReport {
    pub committed_changes: u64,
    pub activities_written: u64,
    pub notification_attempts: u64,
    pub notification_errors: u64,
    pub database_queries: u64,
    pub notification_queue_usage: f64,
    pub mutation_latency_micros: Vec<u64>,
}

/// Actual observations supplied by both transient gateway instances.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GatewayStepReport {
    pub database_notifications: u64,
    pub accepted_resource_filters: u64,
    pub raw_matched_hints: u64,
    pub emitted_hints: u64,
    pub coalesced_hints: u64,
    pub peak_pending_keys: u64,
    pub subscription_registrations: u64,
    pub subscription_unregistrations: u64,
    pub notification_latency_micros: Vec<u64>,
}

/// Actual observations supplied by the loopback MCP clients and resource reads.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct McpStepReport {
    pub actual_subscribers: u64,
    pub live_notified_hosts: BTreeSet<ScenarioHostId>,
    pub authoritative_refetched_hosts: BTreeSet<ScenarioHostId>,
    pub refetches: u64,
    pub refetch_rows: u64,
    pub refetch_bytes: u64,
    pub model_calls: u64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ScenarioStepReport {
    pub scenario: String,
    pub step: String,
    pub world: WorldStepReport,
    pub gateway: GatewayStepReport,
    pub mcp: McpStepReport,
    pub coverage: CoverageVerdict,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ScenarioRunReport {
    pub strategy: String,
    pub real_postgres: bool,
    pub real_loopback_mcp: bool,
    pub gateway_processes: u64,
    pub actual_mcp_subscribers: u64,
    pub committed_changes: u64,
    pub steps: Vec<ScenarioStepReport>,
    pub aggregate: AggregateMetrics,
}

/// Common matrix columns. The runner computes latency percentiles from bounded
/// actual samples; descriptions must retain topology and workload alongside them.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AggregateMetrics {
    pub required_live_deliveries: u64,
    pub missed_live_deliveries: u64,
    pub missed_recoveries: u64,
    pub irrelevant_wakeups: u64,
    pub raw_hints: u64,
    pub emitted_hints: u64,
    pub coalesced_hints: u64,
    pub peak_pending_keys: u64,
    pub refetches: u64,
    pub refetch_rows: u64,
    pub refetch_bytes: u64,
    pub subscription_churn: u64,
    pub mutation_latency: LatencyDescription,
    pub notification_latency: LatencyDescription,
    pub quiet_subject_latency: LatencyDescription,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct LatencyDescription {
    pub samples: u64,
    pub p50_micros: u64,
    pub p95_micros: u64,
    pub p99_micros: u64,
    pub max_micros: u64,
    pub description: String,
}

impl LatencyDescription {
    pub fn from_actual_samples(mut samples: Vec<u64>, description: impl Into<String>) -> Self {
        samples.sort_unstable();
        Self {
            samples: samples.len() as u64,
            p50_micros: percentile(&samples, 50),
            p95_micros: percentile(&samples, 95),
            p99_micros: percentile(&samples, 99),
            max_micros: samples.last().copied().unwrap_or_default(),
            description: description.into(),
        }
    }
}

fn percentile(samples: &[u64], percentile: usize) -> u64 {
    if samples.is_empty() {
        return 0;
    }
    let index = ((samples.len() - 1) * percentile).div_ceil(100);
    samples[index]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixed_program_commits_exactly_one_hundred_changes() {
        let fixture = ScenarioFixture::fixed();
        assert_eq!(fixture.hosts.len(), REAL_SUBSCRIBERS);
        assert_eq!(
            fixture.program().committed_change_count(),
            CHANGES_PER_STRATEGY
        );
    }

    #[test]
    fn oracle_covers_unknown_local_regional_and_world_change() {
        let fixture = ScenarioFixture::fixed();
        let program = fixture.program();
        let mut oracle = ExpectedOutcomeOracle::new(&fixture);

        let selected = oracle.begin_step(&program.cases[0].steps[0]);
        assert_eq!(selected.live_required.len(), 10);
        assert!(selected.live_required.contains(&24));

        let unselected = oracle.begin_step(&program.cases[1].steps[0]);
        assert_eq!(unselected.live_required, (0..8).collect());

        let regional = oracle.begin_step(&program.cases[4].steps[0]);
        assert_eq!(regional.live_required, (0..16).collect());

        let worldwide = oracle.begin_step(&program.cases[5].steps[0]);
        assert_eq!(worldwide.live_required, (0..32).collect());
    }

    #[test]
    fn movement_and_reconnect_change_later_expectations() {
        let fixture = ScenarioFixture::fixed();
        let program = fixture.program();
        let mut oracle = ExpectedOutcomeOracle::new(&fixture);

        let movement = &program.cases[6];
        oracle.begin_step(&movement.steps[0]);
        oracle.finish_step(&movement.steps[0]);
        let post_move = oracle.begin_step(&movement.steps[1]);
        assert_eq!(post_move.live_required.len(), 9);
        assert!(post_move.live_required.contains(&0));

        let loss = &program.cases[11];
        let while_lost = oracle.begin_step(&loss.steps[0]);
        assert!(!while_lost.live_required.contains(&0));
        let reconnected = oracle.begin_step(&loss.steps[1]);
        assert!(reconnected.recovery_required.contains(&0));
    }

    #[test]
    fn verdict_keeps_coverage_and_noise_separate() {
        let expected = ExpectedOutcome {
            live_required: [1, 2].into_iter().collect(),
            recovery_required: [3].into_iter().collect(),
            irrelevant_if_woken: [4, 5].into_iter().collect(),
        };
        let verdict = expected.evaluate(&[1, 4].into_iter().collect(), &[3].into_iter().collect());
        assert_eq!(verdict.missed_live_hosts, [2].into_iter().collect());
        assert_eq!(verdict.irrelevant_wakeups, [4].into_iter().collect());
        assert!(!verdict.game_coverage_passes());
    }

    #[test]
    fn latency_description_is_explicitly_sampled() {
        let latency = LatencyDescription::from_actual_samples(
            vec![1, 2, 3, 4, 100],
            "five bounded local samples",
        );
        assert_eq!(latency.samples, 5);
        assert_eq!(latency.p50_micros, 3);
        assert_eq!(latency.p95_micros, 100);
        assert_eq!(latency.max_micros, 100);
    }
}
