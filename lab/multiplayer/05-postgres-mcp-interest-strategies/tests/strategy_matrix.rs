use std::collections::{BTreeMap, BTreeSet};

use postgres_mcp_interest_strategies_lab::{
    scenario::{ExpectedOutcomeOracle, HostInterest, LifecycleAction, ScenarioFixture},
    strategy::Strategy,
    world::CommittedChange,
};
use uuid::Uuid;

#[derive(Default)]
struct StrategyVerdict {
    missed_live: u64,
    missed_recovery: u64,
    irrelevant_wakeups: u64,
}

#[test]
fn all_strategies_run_the_same_game_coverage_oracle() {
    let fixture = ScenarioFixture::fixed();
    let program = fixture.program();
    let mut verdicts = BTreeMap::new();

    for strategy in Strategy::ALL {
        let mut oracle = ExpectedOutcomeOracle::new(&fixture);
        let mut interests = fixture
            .hosts
            .iter()
            .cloned()
            .map(|interest| (interest.host_id, interest))
            .collect::<BTreeMap<_, _>>();
        let mut connected = fixture
            .hosts
            .iter()
            .map(|interest| (interest.host_id, true))
            .collect::<BTreeMap<_, _>>();
        let mut verdict = StrategyVerdict::default();
        let mut change_ordinal = 0_u128;

        for case in &program.cases {
            for step in &case.steps {
                let expected = oracle.begin_step(step);
                let recovery = apply_actions(&step.actions_before, &mut interests, &mut connected);
                for _ in 0..step.repetitions {
                    change_ordinal += 1;
                    let changed = committed_change(step, change_ordinal);
                    let changed_resources = strategy.resources_for_change(&changed);
                    let live_notified = interests
                        .values()
                        .filter(|interest| connected[&interest.host_id])
                        .filter(|interest| {
                            strategy
                                .resources_for_interest(&interest.interest_spec())
                                .iter()
                                .any(|resource| changed_resources.contains(resource))
                        })
                        .map(|interest| interest.host_id)
                        .collect::<BTreeSet<_>>();
                    let coverage = expected.evaluate(&live_notified, &recovery);
                    verdict.missed_live += coverage.missed_live_hosts.len() as u64;
                    verdict.missed_recovery += coverage.missed_recovery_hosts.len() as u64;
                    verdict.irrelevant_wakeups += coverage.irrelevant_wakeups.len() as u64;
                }
                apply_actions(&step.actions_after, &mut interests, &mut connected);
                oracle.finish_step(step);
            }
        }
        verdicts.insert(strategy.name(), verdict);
    }

    let global = &verdicts["global_firehose"];
    assert_eq!(global.missed_live, 0);
    assert!(global.irrelevant_wakeups > 0);

    assert!(verdicts["place"].missed_live > 0);
    assert!(verdicts["exact_only"].missed_live > 0);
    assert!(verdicts["place_and_exact"].missed_live > 0);

    let structural = &verdicts["structural"];
    assert_eq!(structural.missed_live, 0);
    assert_eq!(structural.missed_recovery, 0);
    assert_eq!(structural.irrelevant_wakeups, 0);
}

fn committed_change(
    step: &postgres_mcp_interest_strategies_lab::scenario::ScenarioStep,
    ordinal: u128,
) -> CommittedChange {
    let affected_place_ids = step.affected_place_ids();
    CommittedChange {
        change_id: Uuid::from_u128(ordinal),
        scope: step.change_scope(),
        primary_entity_id: step.entity_id,
        primary_place_id: affected_place_ids.first().copied(),
        affected_place_ids,
        changed_entity_ids: step.entity_id.into_iter().collect(),
        resource_versions: Vec::new(),
    }
}

fn apply_actions(
    actions: &[LifecycleAction],
    interests: &mut BTreeMap<u32, HostInterest>,
    connected: &mut BTreeMap<u32, bool>,
) -> BTreeSet<u32> {
    let mut refetched = BTreeSet::new();
    for action in actions {
        match *action {
            LifecycleAction::Disconnect { host_id } => connected.insert(host_id, false),
            LifecycleAction::ReconnectAndRefetch { host_id } => {
                connected.insert(host_id, true);
                refetched.insert(host_id);
                None
            }
            LifecycleAction::Move {
                host_id,
                area_id,
                place_id,
            } => {
                let interest = interests.get_mut(&host_id).expect("fixture host");
                interest.area_id = area_id;
                interest.place_id = place_id;
                None
            }
        };
    }
    refetched
}
