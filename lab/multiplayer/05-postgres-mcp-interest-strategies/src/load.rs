//! Bounded synthetic scale tier.
//!
//! One million interest records are generated and passed through the same strategy
//! key-selection code used by the direct tier. They are folded into cohort counts
//! instead of allocating one million sockets, channels or per-host dirty sets. A
//! bounded retained host sample then executes recipient matching, game-coverage
//! checks, per-host coalescing and movement churn. Full-population raw matches are
//! exact; full coalesced/refetch values are explicitly projections from the sample.

use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

use serde::{Deserialize, Serialize};

use crate::scenario::LatencyDescription;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ScaleKey {
    World,
    Area(u32),
    Place(u32),
    Entity(u32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyntheticInterest {
    pub host_ordinal: u64,
    pub area_id: u32,
    pub place_id: u32,
    pub selected_entity_id: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SyntheticChangeScope {
    Places {
        area_id: u32,
        place_ids: [u32; 2],
        place_count: u8,
    },
    Area(u32),
    World,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyntheticChange {
    pub ordinal: u64,
    pub entity_id: Option<u32>,
    pub scope: SyntheticChangeScope,
}

/// Implemented by the five strategies in `strategy.rs`. The compact key type is a
/// scale-test representation only; the branching semantics must be shared with the
/// UUID/resource-URI path rather than reimplemented in this module.
pub trait ScaleRouting {
    fn strategy_name(&self) -> &'static str;

    fn interest_keys(&self, interest: &SyntheticInterest, output: &mut Vec<ScaleKey>);

    fn change_keys(&self, change: &SyntheticChange, output: &mut Vec<ScaleKey>);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScaleConfig {
    pub interest_records: u64,
    pub changes: u64,
    pub sampled_hosts: usize,
    pub area_count: u32,
    pub place_count: u32,
    pub entity_count: u32,
    pub projected_rows_per_refetch: u64,
    pub projected_bytes_per_row: u64,
    /// The retained sample authoritatively refetches and clears every dirty key
    /// after this many changes. Repeats inside the interval actually coalesce.
    pub sampled_refetch_interval_changes: u64,
}

impl Default for ScaleConfig {
    fn default() -> Self {
        Self {
            interest_records: 1_000_000,
            changes: 10_000,
            sampled_hosts: 8_192,
            area_count: 128,
            place_count: 16_384,
            entity_count: 262_144,
            projected_rows_per_refetch: 4,
            projected_bytes_per_row: 256,
            sampled_refetch_interval_changes: 100,
        }
    }
}

impl ScaleConfig {
    fn validate(self) -> Result<Self, String> {
        if self.interest_records == 0 || self.changes == 0 {
            return Err("interest_records and changes must be non-zero".into());
        }
        if self.sampled_hosts == 0 || self.sampled_hosts as u64 > self.interest_records {
            return Err("sampled_hosts must be within 1..=interest_records".into());
        }
        if self.area_count < 2 || self.place_count < 4 || self.entity_count < 4 {
            return Err("scale cardinalities are too small for hot/quiet skew".into());
        }
        if self.place_count < self.area_count {
            return Err("place_count must be at least area_count".into());
        }
        if self.place_count / self.area_count < 2 {
            return Err("place_count must provide at least two Places per Area".into());
        }
        if self.sampled_refetch_interval_changes == 0 {
            return Err("sampled_refetch_interval_changes must be non-zero".into());
        }
        Ok(self)
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ActualScaleMeasurements {
    /// Records really generated and passed through `ScaleRouting::interest_keys`.
    pub interest_records_processed: u64,
    /// Compact cohort increments really performed; this can exceed host records.
    pub interest_keys_indexed: u64,
    pub unique_cohort_keys: u64,
    /// Changes really generated and passed through `ScaleRouting::change_keys`.
    pub changes_routed: u64,
    /// Exact sum of matching full-population cohort counts before host coalescing.
    pub full_population_raw_matches: u64,
    pub sampled_hosts_retained: u64,
    pub sampled_raw_matches: u64,
    /// Hosts deduplicated across resource keys for each individual change. This is
    /// used for coverage/noise only and is not cross-change coalescing.
    pub sampled_unique_recipient_matches: u64,
    /// New `(host, resource)` dirty keys emitted during bounded dirty periods.
    pub sampled_dirty_key_emissions: u64,
    /// Repeated matches suppressed while that exact dirty key remained pending.
    pub sampled_dirty_key_coalesced: u64,
    pub sampled_peak_pending_host_keys: u64,
    pub sampled_authoritative_refetches: u64,
    pub sampled_required_game_deliveries: u64,
    pub sampled_missed_game_deliveries: u64,
    pub sampled_irrelevant_wakeups: u64,
    pub sampled_within_area_movement_churn: u64,
    pub sampled_cross_area_movement_churn: u64,
    pub interest_ingest_micros: u64,
    /// Includes the O(sampled_hosts) semantic relevance oracle scan for each
    /// change. This is not pure Gateway routing time.
    pub routing_micros: u64,
    pub route_latency: LatencyDescription,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AnalyticalProjection {
    /// Sample dirty-period emission ratio applied to exact full raw matches.
    pub projected_full_dirty_key_emissions: u64,
    pub projected_full_coalesced_raw_matches: u64,
    /// The sample's within-Area watch replacement rate scaled to all records.
    pub projected_full_within_area_movement_churn: u64,
    /// The sample's cross-Area watch replacement rate scaled to all records.
    pub projected_full_cross_area_movement_churn: u64,
    /// Assumes one eventual authoritative refetch per projected coalesced dirty key.
    pub projected_refetches: u64,
    pub projected_refetch_rows: u64,
    pub projected_refetch_bytes: u64,
    pub method: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScaleTierReport {
    pub strategy: String,
    pub config: ScaleConfig,
    pub actual: ActualScaleMeasurements,
    pub projection: AnalyticalProjection,
    pub evidence_boundary: String,
    pub non_claims: Vec<String>,
}

/// Runs one strategy sequentially. It intentionally does not touch PostgreSQL or
/// MCP: those real seams belong to the 32-subscriber tier. Timing here describes
/// local key selection, cohort counting, sampled recipient matching and an
/// O(sampled_hosts) semantic relevance oracle scan. It is not Gateway timing.
pub fn run_scale_tier(
    routing: &impl ScaleRouting,
    config: ScaleConfig,
) -> Result<ScaleTierReport, String> {
    let config = config.validate()?;
    let mut cohort_counts: HashMap<ScaleKey, u64> = HashMap::new();
    let mut sampled_interests = Vec::with_capacity(config.sampled_hosts);
    let mut sampled_routes: HashMap<ScaleKey, Vec<u32>> = HashMap::new();
    let mut keys = Vec::with_capacity(8);
    let mut interest_keys_indexed = 0_u64;

    let ingest_started = Instant::now();
    for host_ordinal in 0..config.interest_records {
        let interest = synthetic_interest(host_ordinal, config);
        keys.clear();
        routing.interest_keys(&interest, &mut keys);
        keys.sort_unstable();
        keys.dedup();
        interest_keys_indexed = interest_keys_indexed.saturating_add(keys.len() as u64);
        for key in &keys {
            *cohort_counts.entry(*key).or_default() += 1;
        }

        if sampled_interests.len() < config.sampled_hosts {
            let sample_id = sampled_interests.len() as u32;
            sampled_interests.push(interest);
            for key in &keys {
                sampled_routes.entry(*key).or_default().push(sample_id);
            }
        }
    }
    let interest_ingest_micros = micros(ingest_started.elapsed());

    let mut full_population_raw_matches = 0_u64;
    let mut sampled_raw_matches = 0_u64;
    let mut sampled_unique_recipient_matches = 0_u64;
    let mut sampled_dirty_key_emissions = 0_u64;
    let mut sampled_dirty_key_coalesced = 0_u64;
    let mut sampled_authoritative_refetches = 0_u64;
    let mut sampled_required_game_deliveries = 0_u64;
    let mut sampled_missed_game_deliveries = 0_u64;
    let mut sampled_irrelevant_wakeups = 0_u64;
    let mut pending_host_keys: HashSet<(u32, ScaleKey)> = HashSet::new();
    let mut peak_pending_host_keys = 0_usize;
    let mut matched_hosts = Vec::with_capacity(config.sampled_hosts);
    let mut route_latency_micros = Vec::with_capacity(config.changes as usize);

    let routing_started = Instant::now();
    for ordinal in 0..config.changes {
        let change = synthetic_change(ordinal, config);
        keys.clear();
        let route_started = Instant::now();
        routing.change_keys(&change, &mut keys);
        keys.sort_unstable();
        keys.dedup();

        matched_hosts.clear();
        for key in &keys {
            full_population_raw_matches = full_population_raw_matches
                .saturating_add(cohort_counts.get(key).copied().unwrap_or_default());
            if let Some(hosts) = sampled_routes.get(key) {
                sampled_raw_matches = sampled_raw_matches.saturating_add(hosts.len() as u64);
                matched_hosts.extend_from_slice(hosts);
                for host_id in hosts {
                    if pending_host_keys.insert((*host_id, *key)) {
                        sampled_dirty_key_emissions += 1;
                    } else {
                        sampled_dirty_key_coalesced += 1;
                    }
                }
            }
        }
        peak_pending_host_keys = peak_pending_host_keys.max(pending_host_keys.len());
        matched_hosts.sort_unstable();
        matched_hosts.dedup();
        sampled_unique_recipient_matches =
            sampled_unique_recipient_matches.saturating_add(matched_hosts.len() as u64);

        let matched_set = matched_hosts.iter().copied().collect::<HashSet<_>>();
        for (sample_id, interest) in sampled_interests.iter().enumerate() {
            let required = change_relevant_to_interest(&change, interest);
            let delivered = matched_set.contains(&(sample_id as u32));
            sampled_required_game_deliveries += u64::from(required);
            sampled_missed_game_deliveries += u64::from(required && !delivered);
            sampled_irrelevant_wakeups += u64::from(!required && delivered);
        }
        route_latency_micros.push(micros(route_started.elapsed()));

        if (ordinal + 1) % config.sampled_refetch_interval_changes == 0 {
            sampled_authoritative_refetches =
                sampled_authoritative_refetches.saturating_add(pending_host_keys.len() as u64);
            pending_host_keys.clear();
        }
    }
    sampled_authoritative_refetches =
        sampled_authoritative_refetches.saturating_add(pending_host_keys.len() as u64);
    pending_host_keys.clear();
    let routing_micros = micros(routing_started.elapsed());

    let sampled_within_area_movement_churn = sampled_interests
        .iter()
        .map(|interest| {
            movement_churn(
                routing,
                *interest,
                movement_destination(*interest, config, MovementKind::WithinArea),
                &mut keys,
            ) as u64
        })
        .sum();
    let sampled_cross_area_movement_churn = sampled_interests
        .iter()
        .map(|interest| {
            movement_churn(
                routing,
                *interest,
                movement_destination(*interest, config, MovementKind::CrossArea),
                &mut keys,
            ) as u64
        })
        .sum();

    let projected_full_dirty_key_emissions = project_ratio(
        full_population_raw_matches,
        sampled_dirty_key_emissions,
        sampled_raw_matches,
    );
    let projected_full_coalesced_raw_matches =
        full_population_raw_matches.saturating_sub(projected_full_dirty_key_emissions);
    let projected_full_within_area_movement_churn = project_ratio(
        config.interest_records,
        sampled_within_area_movement_churn,
        config.sampled_hosts as u64,
    );
    let projected_full_cross_area_movement_churn = project_ratio(
        config.interest_records,
        sampled_cross_area_movement_churn,
        config.sampled_hosts as u64,
    );
    let projected_refetches = project_ratio(
        full_population_raw_matches,
        sampled_authoritative_refetches,
        sampled_raw_matches,
    );
    let projected_refetch_rows =
        projected_refetches.saturating_mul(config.projected_rows_per_refetch);
    let projected_refetch_bytes =
        projected_refetch_rows.saturating_mul(config.projected_bytes_per_row);

    Ok(ScaleTierReport {
        strategy: routing.strategy_name().to_owned(),
        config,
        actual: ActualScaleMeasurements {
            interest_records_processed: config.interest_records,
            interest_keys_indexed,
            unique_cohort_keys: cohort_counts.len() as u64,
            changes_routed: config.changes,
            full_population_raw_matches,
            sampled_hosts_retained: sampled_interests.len() as u64,
            sampled_raw_matches,
            sampled_unique_recipient_matches,
            sampled_dirty_key_emissions,
            sampled_dirty_key_coalesced,
            sampled_peak_pending_host_keys: peak_pending_host_keys as u64,
            sampled_authoritative_refetches,
            sampled_required_game_deliveries,
            sampled_missed_game_deliveries,
            sampled_irrelevant_wakeups,
            sampled_within_area_movement_churn,
            sampled_cross_area_movement_churn,
            interest_ingest_micros,
            routing_micros,
            route_latency: LatencyDescription::from_actual_samples(
                route_latency_micros,
                format!(
                    "{} local synthetic route-key and sampled-match operations, each including an O({}) sampled-host semantic relevance scan; not pure Gateway routing; no sockets or database",
                    config.changes, config.sampled_hosts
                ),
            ),
        },
        projection: AnalyticalProjection {
            projected_full_dirty_key_emissions,
            projected_full_coalesced_raw_matches,
            projected_full_within_area_movement_churn,
            projected_full_cross_area_movement_churn,
            projected_refetches,
            projected_refetch_rows,
            projected_refetch_bytes,
            method: format!(
                "exact raw cohort counts over {} streamed interests; dirty-key emissions/coalescing and refetch cost projected from {} retained executable hosts clearing pending keys every {} changes; within-Area and cross-Area movement churn projected separately from two executed replacements per sampled host",
                config.interest_records,
                config.sampled_hosts,
                config.sampled_refetch_interval_changes,
            ),
        },
        evidence_boundary: format!(
            "processed {} synthetic interest records and {} skewed changes through strategy routing; retained {} hosts for executable recipient/coalescing validation",
            config.interest_records, config.changes, config.sampled_hosts
        ),
        non_claims: vec![
            "not one million sockets, MCP streams, database clients or Users".into(),
            "not PostgreSQL, gateway-network or hosted throughput evidence".into(),
            "route timing includes an O(sampled_hosts) relevance oracle scan and is not actual Gateway timing".into(),
            "not a production memory or capacity benchmark".into(),
            "projected coalesced hints and refetch rows/bytes are not measured I/O".into(),
        ],
    })
}

fn synthetic_interest(host_ordinal: u64, config: ScaleConfig) -> SyntheticInterest {
    let mixed = mix64(host_ordinal.wrapping_add(0x9e37_79b9));
    let place_id = if host_ordinal.is_multiple_of(5) {
        0 // deliberately hot Place: 20% of all records
    } else {
        1 + (mixed % (config.place_count as u64 - 1)) as u32
    };
    let selected_entity_id = if host_ordinal % 10 < 3 {
        0 // deliberately hot Entity: 30% of all records
    } else {
        1 + (mix64(mixed) % (config.entity_count as u64 - 1)) as u32
    };
    SyntheticInterest {
        host_ordinal,
        area_id: place_id % config.area_count,
        place_id,
        selected_entity_id,
    }
}

fn synthetic_change(ordinal: u64, config: ScaleConfig) -> SyntheticChange {
    let bucket = ordinal % 100;
    let mixed = mix64(ordinal.wrapping_add(0x517c_c1b7));
    match bucket {
        // Hot Entity at the hot Place.
        0..=44 => SyntheticChange {
            ordinal,
            entity_id: Some(0),
            scope: one_place(0, config),
        },
        // Hot Place with many independent Entities.
        45..=69 => SyntheticChange {
            ordinal,
            entity_id: Some(1 + (mixed % (config.entity_count as u64 - 1)) as u32),
            scope: one_place(0, config),
        },
        // Quiet and distributed local changes; every fifth is an A/B-style pair.
        70..=84 => {
            let first = 1 + (mixed % (config.place_count as u64 - 1)) as u32;
            let second = 1 + (mix64(mixed) % (config.place_count as u64 - 1)) as u32;
            SyntheticChange {
                ordinal,
                entity_id: Some(1 + (mix64(mixed ^ 7) % (config.entity_count as u64 - 1)) as u32),
                scope: SyntheticChangeScope::Places {
                    area_id: first % config.area_count,
                    place_ids: [first, second],
                    place_count: if bucket.is_multiple_of(5) { 2 } else { 1 },
                },
            }
        }
        // A declared structural regional effect does not enumerate every Place.
        85..=92 => SyntheticChange {
            ordinal,
            entity_id: None,
            scope: SyntheticChangeScope::Area((mixed % config.area_count as u64) as u32),
        },
        // A directly injected World-scope effect; no caller or rights are modeled.
        93..=94 => SyntheticChange {
            ordinal,
            entity_id: None,
            scope: SyntheticChangeScope::World,
        },
        // New local discovery unknown to exact-only interests.
        _ => {
            let place_id = 1 + (mixed % (config.place_count as u64 - 1)) as u32;
            SyntheticChange {
                ordinal,
                entity_id: Some(1 + (mix64(mixed ^ 11) % (config.entity_count as u64 - 1)) as u32),
                scope: one_place(place_id, config),
            }
        }
    }
}

fn one_place(place_id: u32, config: ScaleConfig) -> SyntheticChangeScope {
    SyntheticChangeScope::Places {
        area_id: place_id % config.area_count,
        place_ids: [place_id, place_id],
        place_count: 1,
    }
}

fn change_relevant_to_interest(change: &SyntheticChange, interest: &SyntheticInterest) -> bool {
    let spatially_relevant = match change.scope {
        SyntheticChangeScope::Places {
            place_ids,
            place_count,
            ..
        } => place_ids[..place_count as usize].contains(&interest.place_id),
        SyntheticChangeScope::Area(area_id) => interest.area_id == area_id,
        SyntheticChangeScope::World => true,
    };
    spatially_relevant || change.entity_id == Some(interest.selected_entity_id)
}

fn movement_churn(
    routing: &impl ScaleRouting,
    interest: SyntheticInterest,
    moved: SyntheticInterest,
    scratch: &mut Vec<ScaleKey>,
) -> usize {
    scratch.clear();
    routing.interest_keys(&interest, scratch);
    let old = scratch.iter().copied().collect::<HashSet<_>>();
    scratch.clear();
    routing.interest_keys(&moved, scratch);
    let new = scratch.iter().copied().collect::<HashSet<_>>();
    old.symmetric_difference(&new).count()
}

#[derive(Clone, Copy)]
enum MovementKind {
    WithinArea,
    CrossArea,
}

fn movement_destination(
    interest: SyntheticInterest,
    config: ScaleConfig,
    kind: MovementKind,
) -> SyntheticInterest {
    let (area_id, place_id) = match kind {
        MovementKind::WithinArea => {
            let place_id = interest
                .place_id
                .checked_add(config.area_count)
                .filter(|forward| *forward < config.place_count)
                .unwrap_or_else(|| interest.place_id - config.area_count);
            (interest.area_id, place_id)
        }
        MovementKind::CrossArea => {
            let area_id = (interest.area_id + 1) % config.area_count;
            (area_id, area_id)
        }
    };
    SyntheticInterest {
        area_id,
        place_id,
        ..interest
    }
}

fn project_ratio(base: u64, sample_numerator: u64, sample_denominator: u64) -> u64 {
    if base == 0 || sample_numerator == 0 || sample_denominator == 0 {
        return 0;
    }
    ((base as u128 * sample_numerator as u128) / sample_denominator as u128).min(u64::MAX as u128)
        as u64
}

fn micros(duration: std::time::Duration) -> u64 {
    duration.as_micros().min(u64::MAX as u128) as u64
}

fn mix64(mut value: u64) -> u64 {
    value ^= value >> 30;
    value = value.wrapping_mul(0xbf58_476d_1ce4_e5b9);
    value ^= value >> 27;
    value = value.wrapping_mul(0x94d0_49bb_1331_11eb);
    value ^ (value >> 31)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Copy)]
    enum TestStrategy {
        Global,
        Place,
        Exact,
        Hybrid,
        Structural,
    }

    impl ScaleRouting for TestStrategy {
        fn strategy_name(&self) -> &'static str {
            match self {
                Self::Global => "global-firehose",
                Self::Place => "place",
                Self::Exact => "exact-only",
                Self::Hybrid => "flat-hybrid",
                Self::Structural => "structural",
            }
        }

        fn interest_keys(&self, interest: &SyntheticInterest, output: &mut Vec<ScaleKey>) {
            match self {
                Self::Global => output.push(ScaleKey::World),
                Self::Place => output.push(ScaleKey::Place(interest.place_id)),
                Self::Exact => output.push(ScaleKey::Entity(interest.selected_entity_id)),
                Self::Hybrid => {
                    output.push(ScaleKey::Place(interest.place_id));
                    output.push(ScaleKey::Entity(interest.selected_entity_id));
                }
                Self::Structural => {
                    output.push(ScaleKey::World);
                    output.push(ScaleKey::Area(interest.area_id));
                    output.push(ScaleKey::Place(interest.place_id));
                    output.push(ScaleKey::Entity(interest.selected_entity_id));
                }
            }
        }

        fn change_keys(&self, change: &SyntheticChange, output: &mut Vec<ScaleKey>) {
            if matches!(self, Self::Global) {
                output.push(ScaleKey::World);
                return;
            }
            if let Some(entity_id) = change.entity_id
                && matches!(self, Self::Exact | Self::Hybrid | Self::Structural)
            {
                output.push(ScaleKey::Entity(entity_id));
            }
            match (self, change.scope) {
                (
                    Self::Place | Self::Hybrid | Self::Structural,
                    SyntheticChangeScope::Places {
                        place_ids,
                        place_count,
                        ..
                    },
                ) => output.extend(
                    place_ids[..place_count as usize]
                        .iter()
                        .copied()
                        .map(ScaleKey::Place),
                ),
                (Self::Structural, SyntheticChangeScope::Area(area_id)) => {
                    output.push(ScaleKey::Area(area_id));
                }
                (Self::Structural, SyntheticChangeScope::World) => {
                    output.push(ScaleKey::World);
                }
                _ => {}
            }
        }
    }

    fn tiny_config() -> ScaleConfig {
        ScaleConfig {
            interest_records: 2_000,
            changes: 200,
            sampled_hosts: 256,
            area_count: 8,
            place_count: 64,
            entity_count: 256,
            ..ScaleConfig::default()
        }
    }

    #[test]
    fn scale_tier_distinguishes_coverage_from_noise() {
        let global = run_scale_tier(&TestStrategy::Global, tiny_config()).unwrap();
        let exact = run_scale_tier(&TestStrategy::Exact, tiny_config()).unwrap();
        let structural = run_scale_tier(&TestStrategy::Structural, tiny_config()).unwrap();

        assert_eq!(global.actual.sampled_missed_game_deliveries, 0);
        assert!(global.actual.sampled_irrelevant_wakeups > 0);
        assert!(exact.actual.sampled_missed_game_deliveries > 0);
        assert_eq!(structural.actual.sampled_missed_game_deliveries, 0);
    }

    #[test]
    fn scale_report_separates_measurements_from_projections() {
        let report = run_scale_tier(&TestStrategy::Hybrid, tiny_config()).unwrap();
        assert_eq!(report.actual.interest_records_processed, 2_000);
        assert_eq!(report.actual.changes_routed, 200);
        assert!(report.actual.interest_ingest_micros > 0);
        assert!(report.projection.method.contains("projected"));
        assert!(
            report
                .non_claims
                .iter()
                .any(|claim| claim.contains("sockets"))
        );
    }

    #[test]
    fn all_five_strategy_shapes_execute_the_same_fixed_workload() {
        for strategy in [
            TestStrategy::Global,
            TestStrategy::Place,
            TestStrategy::Exact,
            TestStrategy::Hybrid,
            TestStrategy::Structural,
        ] {
            let report = run_scale_tier(&strategy, tiny_config()).unwrap();
            assert_eq!(report.actual.interest_records_processed, 2_000);
            assert_eq!(report.actual.changes_routed, 200);
        }
    }

    #[test]
    fn movement_churn_separates_within_area_from_cross_area_replacement() {
        let hybrid = run_scale_tier(&TestStrategy::Hybrid, tiny_config()).unwrap();
        assert_eq!(
            hybrid.actual.sampled_within_area_movement_churn,
            2 * hybrid.actual.sampled_hosts_retained
        );
        assert_eq!(
            hybrid.actual.sampled_cross_area_movement_churn,
            2 * hybrid.actual.sampled_hosts_retained
        );

        let structural = run_scale_tier(&TestStrategy::Structural, tiny_config()).unwrap();
        assert_eq!(
            structural.actual.sampled_within_area_movement_churn,
            2 * structural.actual.sampled_hosts_retained
        );
        assert_eq!(
            structural.actual.sampled_cross_area_movement_churn,
            4 * structural.actual.sampled_hosts_retained
        );
    }
}
