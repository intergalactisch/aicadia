//! Overview priority view and the raw Markdown builder brief.

use axum::{
    extract::State,
    http::{HeaderMap, StatusCode, Uri, header},
    response::{IntoResponse, Response},
};
use maud::{Markup, html};

use super::{Context, Page, Reference, Section, Tone, doc_href};
use crate::{
    StudioState,
    brief::{self, Brief, LiveBrief},
};

/// `/` — the immediate work, the integrity signal and compact system orientation.
pub(crate) async fn overview(
    State(state): State<StudioState>,
    header: HeaderMap,
    uri: Uri,
) -> Response {
    let context = match Context::build(&state, &header, &uri).await {
        Ok(context) => context,
        Err(response) => return response,
    };
    let pool = context.pulse().is_connected().then_some(&state.pool);
    let brief = Brief::project(context.repository(), context.surface(), pool).await;
    let markdown = brief.markdown();

    let page = Page::new(Section::Overview, "State of Aicadia")
        .with_document_title("State of Aicadia · Aicadia Studio")
        .with_lede(
            "The current build, its next pressure and the integrity signals that deserve attention now.",
        )
        .with_content(overview_content(&brief, &markdown))
        .with_reference(Reference {
            title: "State of Aicadia".to_owned(),
            url: context.url("/"),
            context: "projection of governed sources, not an authority".to_owned(),
        });

    context.render(page)
}

/// `/brief` — the exact Markdown emitted by `cargo brief` for the same state.
pub(crate) async fn brief(State(state): State<StudioState>) -> Response {
    match brief::markdown(state.repository_root.as_path(), Some(&state.pool)).await {
        Ok(markdown) => (
            [
                (header::CONTENT_TYPE, "text/markdown; charset=utf-8"),
                (header::CACHE_CONTROL, "no-cache"),
            ],
            markdown,
        )
            .into_response(),
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            [(header::CONTENT_TYPE, "text/plain; charset=utf-8")],
            format!("Aicadia brief could not be projected: {error}\n"),
        )
            .into_response(),
    }
}

fn overview_content(brief: &Brief, markdown: &str) -> Markup {
    let active_plan = brief.plan.iter().find(|plan| plan.status == "active");
    let other_plan = brief
        .plan
        .iter()
        .filter(|plan| plan.status != "active")
        .collect::<Vec<_>>();

    html! {
        div class="overview-actions" {
            button class="btn" type="button" data-copy=(markdown) { "Copy builder brief" }
            a class="btn btn-quiet" href="/brief" { "Open Markdown" }
        }

        section class="section overview-priority" {
            h2 { "Current work" }
            @if brief.current_edge.is_empty() {
                p class="section-note" { "No game-development edge is selected in the backlog horizon." }
            } @else {
                ul class="list" role="list" {
                    @for edge in &brief.current_edge {
                        li {
                            @if let Some(path) = &edge.item_path {
                                a href=(doc_href(path)) { (&edge.item) }
                            } @else {
                                span { (&edge.item) }
                            }
                            small { (&edge.outcome) }
                        }
                    }
                }
            }

            @if let Some(plan) = active_plan {
                ul class="overview-plan" role="list" {
                    (plan_item(plan))
                }
            } @else {
                (super::empty_state("No active build plan is present."))
            }
            @if !other_plan.is_empty() {
                details class="secondary-disclosure compact" {
                    summary { (super::count(other_plan.len(), "draft plan", "draft plans")) }
                    ul class="overview-plan" role="list" {
                        @for plan in other_plan { (plan_item(plan)) }
                    }
                }
            }
        }

        section class="section" {
            h2 { "Needs attention" }
            div class="attention-row" {
                div {
                    h3 class="subhead" { "Documentation integrity" }
                    @if brief.lint.is_empty() {
                        (super::note(Tone::Moss, "No documentation lint findings."))
                    } @else {
                        ul class="warning-list" role="list" data-lint-count=(brief.lint.len()) {
                            @for finding in &brief.lint {
                                li {
                                    a href=(doc_href(&finding.path)) {
                                        (&finding.path)
                                        @if let Some(line) = finding.line { (format!(":{line}")) }
                                    }
                                    p { (finding.message.as_str()) }
                                    small { (finding.rule) }
                                }
                            }
                        }
                    }
                }
                div {
                    h3 class="subhead" { "Open questions" }
                    p class="overview-number" { (brief.open_count) }
                    p { a href="/development/open" { "Review source-owned open sections" } }
                }
            }
        }

        section class="section" {
            h2 { "System shape" }
            dl class="overview-metrics" {
                div { dt { "Models" } dd { a href="/game" { (brief.model.len()) } } }
                div { dt { "Agent capabilities" } dd { a href="/game/agent" { (brief.capability.len()) } } }
                div { dt { "Evidence slices" } dd { a href="/development/evidence" { (brief.evidence.len()) } } }
                div { dt { "Lab verdicts" } dd { a href="/development/lab" { (brief.lab.len()) } } }
            }
            (live_summary(&brief.live))
        }

        section class="section" {
            h2 { "Latest decisions" }
            @if brief.decision.is_empty() {
                (super::empty_state("No decision entries are projected."))
            } @else {
                ol class="decision-stream" role="list" {
                    @for decision in brief.decision.iter().take(3) {
                        li {
                            details {
                                summary { (&decision.date) " · " (&decision.topic) " · " (&decision.tag) }
                                p { (&decision.text) }
                                a href=(format!("{}#{}", doc_href(&decision.path), decision.anchor)) { "Open source" }
                            }
                        }
                    }
                }
                p class="section-action" { a href="/development/decision" { "Browse the decision register" } }
            }
        }

        details class="secondary-disclosure" {
            summary { "Lab and evidence status" }
            div class="secondary-grid" {
                section {
                    h2 { "Lab verdicts" }
                    (record_status_list(brief.lab.iter().map(|item| (&item.title, &item.verdict, &item.path))))
                }
                section {
                    h2 { "Evidence" }
                    (record_status_list(brief.evidence.iter().map(|item| (&item.title, &item.status, &item.path))))
                }
            }
        }
    }
}

fn plan_item(plan: &crate::brief::PlanBrief) -> Markup {
    html! {
        li {
            div class="overview-plan-heading" {
                a href=(doc_href(&plan.path)) { (&plan.title) }
                span class=(status_class(&plan.status)) { (&plan.status) }
            }
            p {
                (format!("{} of {} tasks still need work.", plan.task_open, plan.task_total))
                @if plan.has_open_questions {
                    " " a href=(format!("{}#open-questions", doc_href(&plan.path))) { "Review open questions" }
                }
            }
        }
    }
}

fn live_summary(live: &LiveBrief) -> Markup {
    match live {
        LiveBrief::Unavailable => super::note(
            Tone::Brick,
            "The live World is unavailable. Repository orientation remains complete.",
        ),
        LiveBrief::Available {
            estimate,
            latest_migration,
            unapplied_migrations,
        } => html! {
            div class="live-summary" {
                p {
                    "Connected World · "
                    (estimate.len()) " tables · latest migration "
                    code class="fact" { (latest_migration.as_deref().unwrap_or("none")) }
                    @if *unapplied_migrations > 0 {
                        " · " (unapplied_migrations) " unapplied"
                    }
                }
                details {
                    summary { "Table planner estimates" }
                    dl class="estimate-list" {
                        @for table in estimate {
                            dt { code { (&table.table) } }
                            dd {
                                (table.rows.map_or_else(|| "not analyzed".to_owned(), |rows| rows.to_string()))
                            }
                        }
                    }
                }
            }
        },
    }
}

fn record_status_list<'a>(
    item: impl Iterator<Item = (&'a String, &'a String, &'a String)>,
) -> Markup {
    let item = item.collect::<Vec<_>>();
    if item.is_empty() {
        return html! { p class="mute" { "None" } };
    }
    html! {
        ul class="list" role="list" {
            @for (title, status, path) in item {
                li {
                    a href=(doc_href(path)) { (title) }
                    small { (status) }
                }
            }
        }
    }
}

fn status_class(value: &str) -> &'static str {
    match Tone::of(value) {
        Tone::Moss => "seal seal-moss",
        Tone::Amber => "seal seal-amber",
        Tone::Brick => "seal seal-brick",
        Tone::Slate => "seal seal-slate",
        Tone::Plain => "seal seal-plain",
    }
}
