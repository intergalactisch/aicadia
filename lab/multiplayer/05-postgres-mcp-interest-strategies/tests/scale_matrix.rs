use postgres_mcp_interest_strategies_lab::{
    load::{ScaleConfig, run_scale_tier},
    strategy::Strategy,
};

/// Expensive by unit-test standards, but deliberately bounded and runnable as the
/// experiment's release synthetic tier. It processes records, not live Users.
#[test]
#[ignore = "run explicitly in release mode: one million synthetic interests per strategy"]
fn million_interest_strategy_matrix() {
    for strategy in Strategy::ALL {
        let report = run_scale_tier(&strategy, ScaleConfig::default()).unwrap();
        println!("{}", serde_json::to_string(&report).unwrap());
        assert_eq!(report.actual.interest_records_processed, 1_000_000);
        assert_eq!(report.actual.changes_routed, 10_000);
        assert_eq!(report.actual.sampled_hosts_retained, 8_192);
    }
}
