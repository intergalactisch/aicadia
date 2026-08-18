//! Plans, their task graphs, the backlog horizon and every still-open section.

use super::{
    home,
    record::{LinkTarget, Record, Repository, heading_id},
};

/// The task-graph header row a plan must carry for its board to be read.
const TASK_HEADER: [&str; 7] = [
    "ID",
    "State",
    "Depends",
    "Parallel-safe",
    "Objective",
    "Owned surfaces",
    "Evidence",
];

/// The backlog horizon header row.
const HORIZON_HEADER: [&str; 4] = ["Horizon", "Item", "State", "Concrete outcome"];

/// One row of a plan's task graph.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Task {
    pub id: String,
    pub state: String,
    pub depends: String,
    pub parallel_safe: String,
    pub objective: String,
    pub owned_surface: String,
    pub evidence: String,
}

impl Task {
    /// Task states are `pending`, `in_progress`, `completed` and `blocked`.
    pub fn is_known_state(&self) -> bool {
        matches!(
            self.state.as_str(),
            "pending" | "in_progress" | "completed" | "blocked"
        )
    }
}

/// One plan record projected as a board.
#[derive(Clone, Debug)]
pub struct Plan {
    pub path: String,
    /// The plan directory name, which is its stable id.
    pub id: String,
    pub title: String,
    pub status: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub accepted_at: Option<String>,
    pub completed_at: Option<String>,
    pub backlog_item: Option<String>,
    pub task: Vec<Task>,
    /// The body of the plan's `### Open questions` section, when it has one.
    pub open_question: Option<String>,
}

impl Plan {
    /// A plan is live while it is neither complete nor superseded.
    pub fn is_live(&self) -> bool {
        matches!(self.status.as_str(), "draft" | "active")
    }
}

/// Every plan in the repository, newest directory first.
pub fn plans(repository: &Repository) -> Vec<Plan> {
    let mut plan = repository
        .in_home("plan")
        .into_iter()
        .map(parse_plan)
        .collect::<Vec<_>>();
    plan.sort_by(|left, right| right.id.cmp(&left.id));
    plan
}

fn parse_plan(record: &Record) -> Plan {
    let directory = home::directory(&record.path);
    let id = home::file_name(directory).to_owned();
    let scalar = |key: &str| record.front_matter.scalar(key).map(str::to_owned);
    Plan {
        path: record.path.clone(),
        id,
        title: record.title.clone(),
        status: scalar("status").unwrap_or_else(|| "unknown".to_owned()),
        created_at: scalar("created_at"),
        updated_at: scalar("updated_at"),
        accepted_at: scalar("accepted_at"),
        completed_at: scalar("completed_at"),
        backlog_item: scalar("backlog_item"),
        task: task_graph(&record.body),
        open_question: section(&record.body, "Open questions", 3),
    }
}

/// The first table whose header row is exactly the task-graph header.
pub fn task_graph(body: &str) -> Vec<Task> {
    let Some(row) = table(body, &TASK_HEADER) else {
        return Vec::new();
    };
    row.into_iter()
        .map(|cell| Task {
            id: cell[0].clone(),
            state: cell[1].clone(),
            depends: cell[2].clone(),
            parallel_safe: cell[3].clone(),
            objective: cell[4].clone(),
            owned_surface: cell[5].clone(),
            evidence: cell[6].clone(),
        })
        .collect()
}

/// One row of the backlog horizon table.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HorizonRow {
    pub horizon: String,
    pub item: String,
    /// The linked item file, when the row has one.
    pub item_path: Option<String>,
    pub state: String,
    pub outcome: String,
}

/// The ordered backlog horizon, which is the single home for item order and state.
pub fn horizon(repository: &Repository) -> Vec<HorizonRow> {
    let Some(record) = repository.get("dev/backlog/README.md") else {
        return Vec::new();
    };
    let Some(row) = table(&record.body, &HORIZON_HEADER) else {
        return Vec::new();
    };
    row.into_iter()
        .map(|cell| HorizonRow {
            horizon: cell[0].clone(),
            item_path: first_link(record, &cell[1]),
            item: strip_markdown_link(&cell[1]),
            state: cell[2].clone(),
            outcome: cell[3].clone(),
        })
        .collect()
}

fn first_link(record: &Record, cell: &str) -> Option<String> {
    let target = cell.split_once("](")?.1.split_once(')')?.0.to_owned();
    match super::record::resolve_link(&record.path, &target) {
        LinkTarget::Repository { path, .. } => Some(path),
        _ => None,
    }
}

fn strip_markdown_link(cell: &str) -> String {
    match cell.split_once('[') {
        Some((_, rest)) => rest
            .split_once(']')
            .map_or(cell, |(text, _)| text)
            .to_owned(),
        None => cell.to_owned(),
    }
}

/// One still-open section of a live development record.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OpenSection {
    pub path: String,
    pub home_id: &'static str,
    pub heading: String,
    pub heading_id: String,
    pub level: u8,
    pub body: String,
}

/// Every `Open …` section of live concept records, live plans and backlog items.
pub fn open_section(repository: &Repository) -> Vec<OpenSection> {
    let mut open = Vec::new();
    for record in repository.record() {
        let home_id = record.home_id();
        let eligible = match home_id {
            "concept-record" | "backlog-item" => !record.metadata_frozen,
            "plan" => matches!(
                record.front_matter.scalar("status"),
                Some("draft" | "active")
            ),
            _ => false,
        };
        if !eligible {
            continue;
        }
        for heading in &record.heading {
            if !(2..=3).contains(&heading.level) || !heading.title.starts_with("Open") {
                continue;
            }
            let Some(body) = section(&record.body, &heading.title, heading.level) else {
                continue;
            };
            open.push(OpenSection {
                path: record.path.clone(),
                home_id,
                heading: heading.title.clone(),
                heading_id: heading.id.clone(),
                level: heading.level,
                body,
            });
        }
    }
    open
}

/// The body of one heading section, up to the next heading of the same or higher level.
pub fn section(body: &str, title: &str, level: u8) -> Option<String> {
    let wanted = format!("{} {title}", "#".repeat(usize::from(level)));
    let mut collecting = false;
    let mut collected: Vec<&str> = Vec::new();
    for line in body.lines() {
        let heading = line
            .chars()
            .take_while(|character| *character == '#')
            .count();
        if line.trim_end() == wanted {
            collecting = true;
            continue;
        }
        if collecting && (1..=6).contains(&heading) && heading <= usize::from(level) {
            break;
        }
        if collecting {
            collected.push(line);
        }
    }
    collecting.then(|| collected.join("\n").trim().to_owned())
}

/// Read a Markdown table whose header row equals `header`, returning its data rows.
fn table(body: &str, header: &[&str]) -> Option<Vec<Vec<String>>> {
    let line = body.lines().collect::<Vec<_>>();
    let start = line
        .iter()
        .position(|text| cells(text).is_some_and(|cell| cell == header))?;
    let mut row = Vec::new();
    for text in line.iter().skip(start + 1) {
        let Some(cell) = cells(text) else { break };
        if cell
            .iter()
            .all(|cell| cell.chars().all(|c| c == '-' || c == ':') && !cell.is_empty())
        {
            continue;
        }
        if cell.len() != header.len() {
            break;
        }
        row.push(cell);
    }
    Some(row)
}

fn cells(text: &str) -> Option<Vec<String>> {
    let trimmed = text.trim();
    let inner = trimmed.strip_prefix('|')?.strip_suffix('|')?;
    Some(
        inner
            .split('|')
            .map(|cell| cell.trim().to_owned())
            .collect(),
    )
}

/// The heading id a plan's open-questions section is addressed by.
pub fn open_question_anchor() -> String {
    heading_id("Open questions")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::record::Repository;

    fn repository(file: &[(&str, &str)]) -> Repository {
        let root = crate::test_directory("plan-fixture");
        for (path, content) in file {
            let full = root.join(path);
            std::fs::create_dir_all(full.parent().expect("parent")).expect("directory");
            std::fs::write(full, content).expect("record");
        }
        Repository::load(&root).expect("fixture loads")
    }

    const PLAN: &str = "---\nstatus: active\ncreated_at: \"2026-08-17T16:16:15+02:00\"\nupdated_at: \"2026-08-17T18:05:00+02:00\"\naccepted_at: \"2026-08-17T18:05:00+02:00\"\ncompleted_at: null\nbacklog_item: dev/backlog/items/studio.md\n---\n\n# Studio plan\n\n> **Role / side:** proportional build plan / development side.\n> **Authority:** execution state.\n\n### Open questions\n\nNone. Every material question is resolved.\n\n## Task graph\n\n| ID | State | Depends | Parallel-safe | Objective | Owned surfaces | Evidence |\n| --- | --- | --- | --- | --- | --- | --- |\n| T1 | pending | — | with T2 | Projection | `studio/src/` | Unit tests |\n| T2 | completed | T1 | no | Docs | `docs/**` | Move map |\n\n## After\n\nText.\n";

    #[test]
    fn a_plan_projects_front_matter_task_graph_and_open_questions() {
        let repository = repository(&[("dev/plans/20260817-studio/plan.md", PLAN)]);
        let plan = plans(&repository);

        assert_eq!(plan.len(), 1);
        let plan = &plan[0];
        assert_eq!(plan.id, "20260817-studio");
        assert_eq!(plan.title, "Studio plan");
        assert_eq!(plan.status, "active");
        assert!(plan.is_live());
        assert_eq!(plan.completed_at.as_deref(), Some("null"));
        assert_eq!(
            plan.backlog_item.as_deref(),
            Some("dev/backlog/items/studio.md")
        );
        assert_eq!(
            plan.task,
            vec![
                Task {
                    id: "T1".to_owned(),
                    state: "pending".to_owned(),
                    depends: "—".to_owned(),
                    parallel_safe: "with T2".to_owned(),
                    objective: "Projection".to_owned(),
                    owned_surface: "`studio/src/`".to_owned(),
                    evidence: "Unit tests".to_owned(),
                },
                Task {
                    id: "T2".to_owned(),
                    state: "completed".to_owned(),
                    depends: "T1".to_owned(),
                    parallel_safe: "no".to_owned(),
                    objective: "Docs".to_owned(),
                    owned_surface: "`docs/**`".to_owned(),
                    evidence: "Move map".to_owned(),
                },
            ]
        );
        assert!(plan.task.iter().all(Task::is_known_state));
        assert_eq!(
            plan.open_question.as_deref(),
            Some("None. Every material question is resolved.")
        );
        assert_eq!(open_question_anchor(), "open-questions");
    }

    #[test]
    fn a_plan_without_the_exact_task_header_yields_no_board() {
        assert!(task_graph("| Id | State |\n| --- | --- |\n| T1 | pending |\n").is_empty());
    }

    #[test]
    fn the_horizon_table_carries_order_state_and_the_linked_item() {
        let repository = repository(&[(
            "dev/backlog/README.md",
            "# Backlog\n\n> **Role / side:** forward-planning index / development side.\n> **Authority:** horizon.\n\n| Horizon | Item | State | Concrete outcome |\n| --- | --- | --- | --- |\n| Done | [Local play](items/local.md) | Done | One local World. |\n| Later | Place neighborhood | Queued | Bounded neighbors. |\n",
        )]);

        assert_eq!(
            horizon(&repository),
            vec![
                HorizonRow {
                    horizon: "Done".to_owned(),
                    item: "Local play".to_owned(),
                    item_path: Some("dev/backlog/items/local.md".to_owned()),
                    state: "Done".to_owned(),
                    outcome: "One local World.".to_owned(),
                },
                HorizonRow {
                    horizon: "Later".to_owned(),
                    item: "Place neighborhood".to_owned(),
                    item_path: None,
                    state: "Queued".to_owned(),
                    outcome: "Bounded neighbors.".to_owned(),
                },
            ]
        );
    }

    #[test]
    fn open_sections_come_from_live_concept_records_live_plans_and_backlog_items() {
        let repository = repository(&[
            ("dev/plans/20260817-studio/plan.md", PLAN),
            (
                "dev/plans/20260810-old/plan.md",
                "---\nstatus: complete\n---\n\n# Old\n\n### Open questions\n\nStale.\n",
            ),
            (
                "dev/docs/concept/discovery.md",
                "---\nstatus: live\n---\n\n# Discovery\n\n> **Role / side:** live concept exploration / development side.\n> **Authority:** rationale.\n\n## Open decisions\n\nQ1 stays open.\n\n### Open follow-up\n\nQ2 too.\n\n## Closed\n\nDone.\n",
            ),
            (
                "dev/backlog/items/local.md",
                "# Local\n\n> **Role / side:** forward-planning item / development side.\n> **Authority:** state.\n\n## Open dependency\n\nNeeds a decision.\n",
            ),
        ]);

        let open = open_section(&repository)
            .into_iter()
            .map(|section| (section.path, section.heading, section.body))
            .collect::<Vec<_>>();

        assert_eq!(
            open,
            vec![
                (
                    "dev/backlog/items/local.md".to_owned(),
                    "Open dependency".to_owned(),
                    "Needs a decision.".to_owned()
                ),
                (
                    "dev/docs/concept/discovery.md".to_owned(),
                    "Open decisions".to_owned(),
                    "Q1 stays open.\n\n### Open follow-up\n\nQ2 too.".to_owned()
                ),
                (
                    "dev/docs/concept/discovery.md".to_owned(),
                    "Open follow-up".to_owned(),
                    "Q2 too.".to_owned()
                ),
                (
                    "dev/plans/20260817-studio/plan.md".to_owned(),
                    "Open questions".to_owned(),
                    "None. Every material question is resolved.".to_owned()
                ),
            ]
        );
    }
}
