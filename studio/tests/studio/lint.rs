use std::collections::BTreeSet;

use aicadia_studio::{model, plan, record::Repository, register};

fn repository() -> Repository {
    Repository::load(aicadia_studio::workspace_root())
        .expect("the governed roots parse within their read bounds")
}

#[test]
fn the_documentation_lint_is_clean() {
    let repository = repository();
    let finding = repository.lint();
    assert!(
        finding.is_empty(),
        "documentation lint found {} problem(s):\n{}",
        finding.len(),
        finding
            .iter()
            .map(|finding| {
                format!(
                    "  {} :: {}{} :: {}",
                    finding.rule,
                    finding.path,
                    finding
                        .line
                        .map(|line| format!(":{line}"))
                        .unwrap_or_default(),
                    finding.message
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    );
}

#[test]
fn every_governed_home_that_must_exist_has_records() {
    let repository = repository();
    for home in [
        "constitution",
        "build-constitution",
        "entry-pointer",
        "vocabulary",
        "game-index",
        "model",
        "capability",
        "game-contract",
        "concept-index",
        "concept-record",
        "decision-register",
        "research-index",
        "research-report",
        "methodology-index",
        "methodology-record",
        "area-index",
        "development-area",
        "area-record",
        "evidence-index",
        "evidence-slice",
        "runner-contract",
        "backlog-index",
        "capability-map",
        "backlog-item",
        "plan",
        "skill",
        "lab-index",
        "lab-track",
        "lab-experiment",
        "agent-contract-index",
        "agent-instruction",
        "agent-tool-text",
        "migration",
    ] {
        assert!(
            !repository.in_home(home).is_empty(),
            "home `{home}` has no record"
        );
    }
}

#[test]
fn every_migration_table_is_claimed_by_exactly_one_model_folder() {
    let repository = repository();
    let models = model::models(&repository);
    let tables = model::storage_table(&repository);
    assert!(models.len() >= 9, "only {} model folders", models.len());
    let claimed = models
        .iter()
        .flat_map(|model| model.storage_table.iter().cloned())
        .collect::<Vec<_>>();
    let unique = claimed.iter().cloned().collect::<BTreeSet<_>>();
    assert_eq!(claimed.len(), unique.len(), "a table is claimed twice");
    let existing = tables
        .iter()
        .map(|table| table.name.clone())
        .collect::<BTreeSet<_>>();
    assert_eq!(unique, existing, "claimed and migrated tables differ");
}

#[test]
fn every_capability_document_has_published_tool_text_and_vice_versa() {
    let repository = repository();
    let stem = |path: &str| {
        std::path::Path::new(path)
            .file_stem()
            .and_then(|stem| stem.to_str())
            .map(str::to_owned)
            .expect("record paths have file stems")
    };
    let capability = repository
        .in_home("capability")
        .into_iter()
        .map(|record| stem(&record.path))
        .collect::<BTreeSet<_>>();
    let tool_text = repository
        .in_home("agent-tool-text")
        .into_iter()
        .map(|record| stem(&record.path))
        .collect::<BTreeSet<_>>();
    assert_eq!(capability, tool_text);
    assert_eq!(capability.len(), 15);
}

#[test]
fn the_decision_register_plans_and_horizon_parse() {
    let repository = repository();
    let august = register::parse(
        repository
            .get("dev/docs/concept/log/2026-08.md")
            .expect("the August register exists"),
    );
    assert!(august.entry.len() > 300);
    assert!(august.entry.iter().all(|entry| !entry.anchor.is_empty()));
    let plans = plan::plans(&repository);
    assert!(plans.iter().any(|plan| plan.is_live()));
    for plan in plans.iter().filter(|plan| plan.is_live()) {
        for task in &plan.task {
            assert!(
                task.is_known_state(),
                "{} task {} has unknown state `{}`",
                plan.path,
                task.id,
                task.state
            );
        }
    }
    assert!(!plan::horizon(&repository).is_empty());
}
