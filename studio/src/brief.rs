//! One source-backed builder orientation, shared byte-for-byte by HTTP and CLI.

use std::{path::Path, time::Duration};

use sqlx::PgPool;

use super::{
    agent::{self, AgentSurface},
    live, model, plan,
    record::{LintFinding, Repository},
    register,
};
use crate::World;

const LIVE_TIMEOUT: Duration = Duration::from_millis(750);
const LATEST_DECISION_LIMIT: usize = 5;

#[derive(Clone, Debug)]
pub(crate) struct PlanBrief {
    pub title: String,
    pub status: String,
    pub path: String,
    pub task_total: usize,
    pub task_open: usize,
    pub has_open_questions: bool,
}

#[derive(Clone, Debug)]
pub(crate) struct DecisionBrief {
    pub date: String,
    pub topic: String,
    pub tag: String,
    pub text: String,
    pub path: String,
    pub anchor: String,
    line: usize,
    ordinal: usize,
}

#[derive(Clone, Debug)]
pub(crate) struct RecordBrief {
    pub title: String,
    pub status: String,
    pub path: String,
}

#[derive(Clone, Debug)]
pub(crate) struct LabBrief {
    pub title: String,
    pub verdict: String,
    pub status: String,
    pub path: String,
}

#[derive(Clone, Debug)]
pub(crate) struct LiveEstimate {
    pub table: String,
    pub rows: Option<i64>,
}

#[derive(Clone, Debug)]
pub(crate) enum LiveBrief {
    Available {
        estimate: Vec<LiveEstimate>,
        latest_migration: Option<String>,
        unapplied_migrations: usize,
    },
    Unavailable,
}

/// Current source fields and pointers used by both presentation surfaces.
#[derive(Clone, Debug)]
pub(crate) struct Brief {
    pub current_edge: Vec<plan::HorizonRow>,
    pub backlog: Vec<plan::HorizonRow>,
    pub plan: Vec<PlanBrief>,
    pub open_count: usize,
    pub decision: Vec<DecisionBrief>,
    pub model: Vec<(String, String)>,
    pub capability: Vec<(String, String)>,
    pub lab: Vec<LabBrief>,
    pub evidence: Vec<RecordBrief>,
    pub lint: Vec<LintFinding>,
    pub live: LiveBrief,
}

impl Brief {
    pub(crate) async fn project(
        repository: &Repository,
        surface: &AgentSurface,
        pool: Option<&PgPool>,
    ) -> Self {
        let horizon = plan::horizon(repository);
        let current_edge = horizon
            .iter()
            .filter(|row| row.horizon.eq_ignore_ascii_case("Now"))
            .cloned()
            .collect();
        let backlog = horizon
            .into_iter()
            .filter(|row| !row.horizon.eq_ignore_ascii_case("Done"))
            .collect();

        let plan = plan::plans(repository)
            .into_iter()
            .filter(plan::Plan::is_live)
            .map(|plan| PlanBrief {
                task_open: plan
                    .task
                    .iter()
                    .filter(|task| task.state != "completed")
                    .count(),
                task_total: plan.task.len(),
                has_open_questions: plan.open_question.is_some(),
                title: plan.title,
                status: plan.status,
                path: plan.path,
            })
            .collect();

        let mut decision = repository
            .in_home("decision-register")
            .into_iter()
            .map(register::parse)
            .flat_map(|register| {
                register.entry.into_iter().map(move |entry| DecisionBrief {
                    date: entry.date,
                    topic: entry.topic,
                    tag: entry.tag,
                    text: one_line(&entry.text),
                    path: register.path.clone(),
                    anchor: entry.anchor,
                    line: entry.line,
                    ordinal: entry.ordinal,
                })
            })
            .collect::<Vec<_>>();
        decision.sort_by(|left, right| {
            right
                .date
                .cmp(&left.date)
                .then(right.path.cmp(&left.path))
                .then(right.line.cmp(&left.line))
                .then(right.ordinal.cmp(&left.ordinal))
        });
        decision.truncate(LATEST_DECISION_LIMIT);

        let model = model::models(repository)
            .into_iter()
            .map(|model| (model.title, model.path))
            .collect();
        let capability = surface
            .tool
            .iter()
            .map(|tool| (tool.name.clone(), tool.capability_path.clone()))
            .collect();
        let lab = repository
            .in_home("lab-experiment")
            .into_iter()
            .map(|record| LabBrief {
                title: record.title.clone(),
                verdict: record
                    .front_matter
                    .scalar("verdict")
                    .unwrap_or("unlabeled")
                    .to_owned(),
                status: record
                    .front_matter
                    .scalar("status")
                    .unwrap_or("unlabeled")
                    .to_owned(),
                path: record.path.clone(),
            })
            .collect();
        let evidence = repository
            .in_home("evidence-slice")
            .into_iter()
            .map(|record| RecordBrief {
                title: record.title.clone(),
                status: record
                    .front_matter
                    .scalar("status")
                    .unwrap_or("unlabeled")
                    .to_owned(),
                path: record.path.clone(),
            })
            .collect();

        Self {
            current_edge,
            backlog,
            plan,
            open_count: plan::open_section(repository).len(),
            decision,
            model,
            capability,
            lab,
            evidence,
            lint: repository.lint(),
            live: live_brief(repository.root(), pool).await,
        }
    }

    /// Markdown only: projected fields and pointers, with no authored summary.
    pub(crate) fn markdown(&self) -> String {
        let mut out = String::from(
            "# Aicadia builder brief\n\n> Generated orientation only; the linked repository records remain authoritative.\n\n",
        );

        out.push_str("## Current edge\n\n");
        if self.current_edge.is_empty() {
            out.push_str("- No edge selected.\n");
        } else {
            for row in &self.current_edge {
                push_horizon(&mut out, row);
            }
        }

        out.push_str("\n## Live plans\n\n");
        if self.plan.is_empty() {
            out.push_str("- No draft or active plan.\n");
        } else {
            for plan in &self.plan {
                out.push_str(&format!(
                    "- [{}]({}) — {} · {} of {} tasks open",
                    plan.title, plan.path, plan.status, plan.task_open, plan.task_total
                ));
                if plan.has_open_questions {
                    out.push_str(&format!(
                        " · [open questions]({}#open-questions)",
                        plan.path
                    ));
                }
                out.push_str(".\n");
            }
        }

        out.push_str(&format!(
            "\n## Open landscape\n\n- [{} unresolved sections](/dev/open).\n",
            self.open_count
        ));

        out.push_str("\n## Latest decisions\n\n");
        if self.decision.is_empty() {
            out.push_str("- No decision entries.\n");
        } else {
            for decision in &self.decision {
                out.push_str(&format!(
                    "- {} · {} · {} ([source]({}#{}))\n",
                    decision.date, decision.topic, decision.tag, decision.path, decision.anchor
                ));
            }
        }

        out.push_str("\n## Backlog\n\n");
        if self.backlog.is_empty() {
            out.push_str("- No non-completed horizon rows.\n");
        } else {
            for row in &self.backlog {
                push_horizon(&mut out, row);
            }
        }

        out.push_str("\n## Game surface\n\n### Models\n\n");
        for (title, path) in &self.model {
            out.push_str(&format!("- [{title}]({path})\n"));
        }
        out.push_str("\n### Capabilities\n\n");
        for (name, path) in &self.capability {
            out.push_str(&format!("- [`{name}`]({path})\n"));
        }
        out.push_str(
            "\n### Agent surface\n\n- [Compiled Agent surface](/game/agent)\n- [Agent contract sources](game/mcp/agent/README.md)\n",
        );

        out.push_str("\n## Lab verdicts\n\n");
        if self.lab.is_empty() {
            out.push_str("- No experiment verdicts.\n");
        } else {
            for experiment in &self.lab {
                out.push_str(&format!(
                    "- [{}]({}) — {} · {}\n",
                    experiment.title, experiment.path, experiment.verdict, experiment.status
                ));
            }
        }

        out.push_str("\n## Evidence\n\n");
        if self.evidence.is_empty() {
            out.push_str("- No evidence slices.\n");
        } else {
            for evidence in &self.evidence {
                out.push_str(&format!(
                    "- [{}]({}) — {}\n",
                    evidence.title, evidence.path, evidence.status
                ));
            }
        }

        out.push_str("\n## Documentation lint\n\n");
        if self.lint.is_empty() {
            out.push_str("- Clean.\n");
        } else {
            for finding in &self.lint {
                let line = finding
                    .line
                    .map_or_else(String::new, |line| format!(":{line}"));
                out.push_str(&format!(
                    "- [{}{}](/doc/{}) — {}: {}\n",
                    finding.path, line, finding.path, finding.rule, finding.message
                ));
            }
        }

        out.push_str("\n## Live World\n\n");
        match &self.live {
            LiveBrief::Unavailable => {
                out.push_str(
                    "- Unavailable; repository orientation is complete without a database.\n",
                );
            }
            LiveBrief::Available {
                estimate,
                latest_migration,
                unapplied_migrations,
            } => {
                out.push_str(&format!(
                    "- Latest applied migration: {}.\n",
                    latest_migration.as_deref().unwrap_or("none")
                ));
                out.push_str(&format!(
                    "- Unapplied repository migrations: {unapplied_migrations}.\n"
                ));
                for table in estimate {
                    let rows = table
                        .rows
                        .map_or_else(|| "not analyzed".to_owned(), |rows| rows.to_string());
                    out.push_str(&format!(
                        "- `{}`: {} rows (planner estimate).\n",
                        table.table, rows
                    ));
                }
            }
        }

        out
    }
}

/// Load and render the exact Markdown used by `cargo brief` and `/brief`.
pub async fn markdown(repository_root: &Path, pool: Option<&PgPool>) -> Result<String, String> {
    let repository = Repository::load(repository_root).map_err(|error| error.to_string())?;
    let catalog_pool = match pool {
        Some(pool) => pool.clone(),
        None => sqlx::postgres::PgPoolOptions::new()
            .connect_lazy("postgresql:///aicadia-brief")
            .map_err(|error| error.to_string())?,
    };
    let surface = agent::surface(
        &repository,
        aicadia::server::mcp_tool_catalog(World::new(catalog_pool)),
    );
    Ok(Brief::project(&repository, &surface, pool).await.markdown())
}

async fn live_brief(repository_root: &Path, pool: Option<&PgPool>) -> LiveBrief {
    let Some(pool) = pool else {
        return LiveBrief::Unavailable;
    };
    let read = async {
        let (estimate, migration) = tokio::join!(
            live::estimate::estimate(pool),
            live::migration::list_migration(pool, repository_root)
        );
        let estimate = estimate.ok()?;
        let migration = migration.ok()?;
        let latest_migration = migration.applied.last().map(|migration| {
            migration
                .file
                .clone()
                .unwrap_or_else(|| migration.version.to_string())
        });
        Some(LiveBrief::Available {
            estimate: estimate
                .table
                .into_iter()
                .map(|table| LiveEstimate {
                    table: table.table,
                    rows: table.row_estimate,
                })
                .collect(),
            latest_migration,
            unapplied_migrations: migration.unapplied_file.len(),
        })
    };
    tokio::time::timeout(LIVE_TIMEOUT, read)
        .await
        .ok()
        .flatten()
        .unwrap_or(LiveBrief::Unavailable)
}

fn push_horizon(out: &mut String, row: &plan::HorizonRow) {
    match &row.item_path {
        Some(path) => out.push_str(&format!(
            "- {} · [{}]({}) — {} · {}\n",
            row.horizon, row.item, path, row.state, row.outcome
        )),
        None => out.push_str(&format!(
            "- {} · {} — {} · {}\n",
            row.horizon, row.item, row.state, row.outcome
        )),
    }
}

fn one_line(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn repository_only_markdown_names_its_boundary_and_authorities() {
        let markdown = markdown(&crate::workspace_root(), None)
            .await
            .expect("the real repository projects");

        assert!(markdown.starts_with("# Aicadia builder brief\n"));
        assert!(markdown.contains("Generated orientation only"));
        assert!(markdown.contains("## Current edge"));
        assert!(markdown.contains("## Game surface"));
        assert!(markdown.contains("## Documentation lint"));
        assert!(markdown.contains("Live World\n\n- Unavailable"));
    }
}
