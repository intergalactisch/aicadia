//! The Overview page, the three section landings and the one stub table.
//!
//! A stub is a real page in the real shell that states, in one sentence, which
//! projection it will show. Every stub lives in [`STUB`] so the task that builds
//! the page removes exactly one row and adds its own route.

use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode, Uri},
    response::Response,
};
use maud::{Markup, html};

use super::{
    Action, Context, Crumb, Page, Panel, PanelItem, Plate, PlateRow, Reference, Seal, Section,
    Tone, doc_href, tree,
};
use crate::studio::{StudioState, model, plan};

/// One page a later task replaces, described by the projection it will show.
struct Stub {
    path: &'static str,
    section: Section,
    group: &'static str,
    title: &'static str,
    sentence: &'static str,
}

/// Every route that is navigable now and specialized later. Order is tree order.
const STUB: [Stub; 18] = [
    Stub {
        path: "/brief",
        section: Section::Overview,
        group: "Overview",
        title: "Builder brief",
        sentence: "This page will render the same Markdown orientation as the brief command: current edge, open questions, live plans, models, capabilities and the connected World.",
    },
    Stub {
        path: "/game/agent",
        section: Section::Game,
        group: "Agent surface",
        title: "Agent surface",
        sentence: "This page will show the compiled tool catalog, every published tool description and input schema, the assembled play instructions and the host requirements, byte-for-byte from the compiled server.",
    },
    Stub {
        path: "/game/vocabulary",
        section: Section::Game,
        group: "Reference",
        title: "Vocabulary",
        sentence: "This page will show every canonical term of the vocabulary with its definition, avoided wording and the model contract that owns it.",
    },
    Stub {
        path: "/game/storage",
        section: Section::Game,
        group: "Reference",
        title: "Storage",
        sentence: "This page will join the storage contract to every migration and to the tables each model folder claims.",
    },
    Stub {
        path: "/development/work",
        section: Section::Development,
        group: "Work",
        title: "Current edge",
        sentence: "This page will show the current edge, every live plan as a board read from its own task graph, the ordered backlog horizon and the capability map.",
    },
    Stub {
        path: "/development/decision",
        section: Section::Development,
        group: "Decisions",
        title: "Decision register",
        sentence: "This page will show every recorded choice of the concept log with its date, topic, tag and stable anchor.",
    },
    Stub {
        path: "/development/open",
        section: Section::Development,
        group: "Decisions",
        title: "Open questions",
        sentence: "This page will aggregate every open section of live concept records, live plans and backlog items.",
    },
    Stub {
        path: "/live/activity",
        section: Section::Live,
        group: "World chronicle",
        title: "World chronicle",
        sentence: "This page will show the newest accepted Activity of the connected World, with Place-scoped and Character-scoped chronicles.",
    },
    Stub {
        path: "/live/resolve",
        section: Section::Live,
        group: "Resolve an id",
        title: "Resolve an id",
        sentence: "This page will resolve one durable World id to the subject that owns it.",
    },
    Stub {
        path: "/live/entity",
        section: Section::Live,
        group: "Subjects",
        title: "Entities",
        sentence: "This page will list the Entities of the connected World with bounded keyset paging.",
    },
    Stub {
        path: "/live/character",
        section: Section::Live,
        group: "Subjects",
        title: "Characters",
        sentence: "This page will list the Characters of the connected World with their User and current Place.",
    },
    Stub {
        path: "/live/place",
        section: Section::Live,
        group: "Subjects",
        title: "Places",
        sentence: "This page will list the Places of the connected World with their entry state and membership.",
    },
    Stub {
        path: "/live/user",
        section: Section::Live,
        group: "Subjects",
        title: "Users",
        sentence: "This page will list the Users of the connected World with their Characters and investigation attempts.",
    },
    Stub {
        path: "/live/property-key",
        section: Section::Live,
        group: "State and history",
        title: "Property keys",
        sentence: "This page will list every canonical Property key with the Entities that currently carry it.",
    },
    Stub {
        path: "/live/trait",
        section: Section::Live,
        group: "State and history",
        title: "Traits",
        sentence: "This page will list every Trait lineage with its current version and establishing Activity.",
    },
    Stub {
        path: "/live/investigation",
        section: Section::Live,
        group: "State and history",
        title: "Investigation attempts",
        sentence: "This page will list every investigation attempt with its User, Place, chance and outcome.",
    },
    Stub {
        path: "/live/storage",
        section: Section::Live,
        group: "Storage",
        title: "Schema",
        sentence: "This page will show the introspected schema of the connected World: tables, columns, constraints, ordered foreign keys and indexes.",
    },
    Stub {
        path: "/live/migration",
        section: Section::Live,
        group: "Storage",
        title: "Migrations",
        sentence: "This page will list every applied migration next to the migration files this repository owns.",
    },
];

/// `/` — the state of Aicadia. The full dashboard arrives with the Overview task.
pub(in crate::studio) async fn overview(
    State(state): State<StudioState>,
    header: HeaderMap,
    uri: Uri,
) -> Response {
    let context = match Context::build(&state, &header, &uri).await {
        Ok(context) => context,
        Err(response) => return response,
    };
    let repository = context.repository();
    let plan = plan::plans(repository);
    let pulse = context.pulse();

    let page = Page::new(Section::Overview, "State of Aicadia")
        .with_document_title("State of Aicadia · Aicadia Studio")
        .with_seal(vec![
            Seal::plain(chrono::Local::now().format("%A %-d %B %Y").to_string()),
            match pulse.database.as_deref() {
                Some(database) if pulse.is_connected() => {
                    Seal::toned(format!("Connected · {database}"), Tone::Moss)
                }
                _ => Seal::toned("World unavailable", Tone::Brick),
            },
        ])
        .with_lede(
            "One projection of the governed repository, the compiled server and the connected World.",
        )
        .with_plate(
            Plate::new(vec![
                PlateRow::text(
                    "Records",
                    super::count(repository.record().len(), "governed record", "governed records"),
                ),
                PlateRow::text("Models", model::models(repository).len().to_string()),
                PlateRow::text("Capabilities", context.surface().tool.len().to_string()),
                PlateRow::text("Plans", plan_summary(&plan)),
            ])
            .with_action(vec![Action::link("Builder brief", "/brief")])
            .with_authority(
                "This page owns nothing. Every line is read from a governed record, a compiled source or the connected World.",
            ),
        )
        .with_content(html! {
            section class="section" style="margin-top:0" {
                h2 { "Sections" }
                ul class="list" {
                    @for section in Section::ALL.into_iter().skip(1) {
                        li {
                            a href=(section.href()) { (section.label()) }
                            small { (section.side()) }
                        }
                    }
                }
            }
            (super::note(
                Tone::Amber,
                "The complete state dashboard — current edge, open questions, latest decisions, the World now, lab verdicts and documentation drift — arrives with the Overview task of the active plan.",
            ))
        })
        .with_related(vec![Panel::list("Constitution", constitution(&context))])
        .with_reference(Reference {
            title: "State of Aicadia".to_owned(),
            url: context.url("/"),
            context: "projection of the governed repository, not an authority".to_owned(),
        });

    context.render(page)
}

pub(in crate::studio) async fn game(
    State(state): State<StudioState>,
    header: HeaderMap,
    uri: Uri,
) -> Response {
    landing(&state, &header, &uri, Section::Game).await
}

pub(in crate::studio) async fn development(
    State(state): State<StudioState>,
    header: HeaderMap,
    uri: Uri,
) -> Response {
    landing(&state, &header, &uri, Section::Development).await
}

pub(in crate::studio) async fn live(
    State(state): State<StudioState>,
    header: HeaderMap,
    uri: Uri,
) -> Response {
    landing(&state, &header, &uri, Section::Live).await
}

/// One section landing: the section's own index, rendered from its tree.
async fn landing(state: &StudioState, header: &HeaderMap, uri: &Uri, section: Section) -> Response {
    let context = match Context::build(state, header, uri).await {
        Ok(context) => context,
        Err(response) => return response,
    };
    let repository = context.repository();

    let plate = match section {
        Section::Game => Plate::new(vec![
            PlateRow::text("Side", "runtime"),
            PlateRow::text("Models", model::models(repository).len().to_string()),
            PlateRow::text("Capabilities", context.surface().tool.len().to_string()),
            PlateRow::text(
                "Contracts",
                (repository.in_home("game-contract").len()
                    + repository.in_home("game-index").len())
                .to_string(),
            ),
        ]),
        Section::Development => Plate::new(vec![
            PlateRow::text("Side", "development"),
            PlateRow::text("Plans", plan_summary(&plan::plans(repository))),
            PlateRow::text(
                "Concept records",
                repository.in_home("concept-record").len().to_string(),
            ),
            PlateRow::text(
                "Research reports",
                repository.in_home("research-report").len().to_string(),
            ),
        ]),
        _ => Plate::new(vec![
            PlateRow::text(
                "Database",
                context
                    .pulse()
                    .database
                    .clone()
                    .unwrap_or_else(|| "not configured".to_owned()),
            ),
            PlateRow::text("Connection", context.pulse().state),
            PlateRow::text("Tables", model::storage_table(repository).len().to_string()),
            PlateRow::text(
                "Migrations",
                repository.in_home("migration").len().to_string(),
            ),
        ]),
    };

    let page = Page::new(section, section.label())
        .with_document_title(format!("{} · Aicadia Studio", section.label()))
        .with_seal(vec![Seal::plain(section.side())])
        .with_lede(lede(section))
        .with_plate(plate.with_authority(
            "Studio owns nothing on this page. Every entry is a governed record, a compiled source or a connected-World read.",
        ))
        .with_content(index_content(&context, section))
        .with_related(vec![related_panel(&context, section)])
        .with_reference(Reference {
            title: section.label().to_owned(),
            url: context.url(section.href()),
            context: "projection of the governed repository, not an authority".to_owned(),
        });

    context.render(page)
}

/// A stub page for a route whose specialized page arrives in a later task.
pub(in crate::studio) async fn stub(
    State(state): State<StudioState>,
    header: HeaderMap,
    uri: Uri,
) -> Response {
    let context = match Context::build(&state, &header, &uri).await {
        Ok(context) => context,
        Err(response) => return response,
    };
    let Some(stub) = STUB.iter().find(|stub| stub.path == uri.path()) else {
        return context.render_status(
            Page::new(Section::Overview, "No Studio page at this path")
                .with_document_title("Not found · Aicadia Studio")
                .with_content(super::empty_state("No Studio page matches this path.")),
            StatusCode::NOT_FOUND,
        );
    };

    context.render(stub_page(&context, stub, stub.title.to_owned(), stub.path))
}

/// `/live/storage/{table}` — one table of the connected World, per migration name.
pub(in crate::studio) async fn stub_table(
    State(state): State<StudioState>,
    header: HeaderMap,
    uri: Uri,
    Path(table): Path<String>,
) -> Response {
    let context = match Context::build(&state, &header, &uri).await {
        Ok(context) => context,
        Err(response) => return response,
    };
    let known = model::storage_table(context.repository())
        .into_iter()
        .any(|known| known.name == table);
    if !known {
        return context.render_status(
            Page::new(Section::Live, "No such table")
                .with_document_title("Not found · Live · Aicadia Studio")
                .with_crumb(vec![
                    Crumb::link("Live", "/live"),
                    Crumb::link("Storage", "/live/storage"),
                    Crumb::here(&table),
                ])
                .with_content(super::empty_state(
                    "The migrations of this repository create no table with this name.",
                )),
            StatusCode::NOT_FOUND,
        );
    }

    let stub = Stub {
        path: "/live/storage",
        section: Section::Live,
        group: "Storage",
        title: "Table",
        sentence: "This page will show one table's columns, constraints and a bounded keyset page of its rows.",
    };
    let path = format!("/live/storage/{table}");
    context.render(stub_page(&context, &stub, table, &path))
}

fn stub_page(context: &Context, stub: &Stub, title: String, path: &str) -> Page {
    Page::new(stub.section, &title)
        .with_crumb(vec![
            Crumb::link(stub.section.label(), stub.section.href()),
            Crumb::here(stub.group),
            Crumb::here(&title),
        ])
        .with_seal(vec![Seal::toned("Page pending", Tone::Amber)])
        .with_lede(stub.sentence)
        .with_plate(
            Plate::new(vec![
                PlateRow::text("Section", stub.section.label()),
                PlateRow::fact("Route", path),
            ])
            .with_authority(
                "Studio shows no data it has not read. Until this page projects its source, it states what it will show and nothing else.",
            ),
        )
        .with_content(super::empty_state(
            "Nothing is projected here yet. The route is stable; the page is not built.",
        ))
        .with_reference(Reference {
            title,
            url: context.url(path),
            context: format!("Studio route {path}"),
        })
}

/// The section index as content: every tree group as a list of links.
///
/// The landing never links to itself, so a group holding only the section's own
/// Overview link is left out entirely.
fn index_content(context: &Context, section: Section) -> Markup {
    let tree = tree::build(context, section, context.path());

    html! {
        @for group in tree.group() {
            @let link = elsewhere(group.item(), context.path());
            @let folder: Vec<_> = group.item().iter().filter(|leaf| leaf.href().is_none()).collect();
            @if !link.is_empty() || !folder.is_empty() {
                section class="section" {
                    h2 { (group.label().unwrap_or("Start here")) }
                    (row_list(&link))
                    @for folder in folder {
                        h3 class="subhead" { (folder.label()) }
                        (row_list(&elsewhere(folder.child(), context.path())))
                    }
                }
            }
        }
    }
}

/// The leaves of one tree level that link somewhere other than the current page.
fn elsewhere<'a>(item: &'a [tree::Leaf], current_path: &str) -> Vec<&'a tree::Leaf> {
    item.iter()
        .filter(|leaf| leaf.href().is_some_and(|href| href != current_path))
        .collect()
}

fn row_list(leaf: &[&tree::Leaf]) -> Markup {
    html! {
        @if !leaf.is_empty() {
            ul class="list" {
                @for leaf in leaf {
                    @if let Some(href) = leaf.href() {
                        li {
                            a href=(href) { (leaf.label()) }
                            @if let Some(note) = leaf.note() { small { (note) } }
                        }
                    }
                }
            }
        }
    }
}

/// The one Related panel of a section landing.
///
/// The two repository sections show what they hold; Live holds no record of its
/// own, so it points at the contracts that govern what its reads may show.
fn related_panel(context: &Context, section: Section) -> Panel {
    if section == Section::Live {
        return Panel::list(
            "Contract",
            ["docs/game/local-play.md", "docs/game/storage.md"]
                .into_iter()
                .filter_map(|path| context.repository().get(path))
                .map(|record| {
                    PanelItem::link(&record.title, doc_href(&record.path))
                        .with_note(super::home_label(record.home_id()))
                })
                .collect(),
        );
    }
    Panel::list("Records in this section", home_count(context, section))
}

/// Every home this section renders, with the number of records it holds.
fn home_count(context: &Context, section: Section) -> Vec<PanelItem> {
    let mut count: Vec<(&'static str, usize)> = Vec::new();
    for record in context.repository().record() {
        let home = record.home_id();
        if super::section_for_home(home) != section {
            continue;
        }
        let label = super::home_label(home);
        match count.iter_mut().find(|(known, _)| *known == label) {
            Some((_, amount)) => *amount += 1,
            None => count.push((label, 1)),
        }
    }
    count.sort_by(|left, right| right.1.cmp(&left.1).then(left.0.cmp(right.0)));
    count
        .into_iter()
        .map(|(label, amount)| PanelItem::text(label).with_note(amount.to_string()))
        .collect()
}

fn lede(section: Section) -> &'static str {
    match section {
        Section::Game => {
            "The runtime side: every model contract, capability, published Agent text and storage rule the current game promises."
        }
        Section::Development => {
            "The development side: direction, decisions, open questions, research, work in progress, experiments, evidence and the build rules."
        }
        Section::Live => {
            "The connected local World: bounded, read-only reads over every durable subject, its history and its storage."
        }
        Section::Overview => "One projection of the repository, the compiled server and the World.",
    }
}

/// `2 active · 1 draft · 21 complete`, counted from the plans themselves.
fn plan_summary(plan: &[plan::Plan]) -> String {
    let mut count: Vec<(String, usize)> = Vec::new();
    for plan in plan {
        match count.iter_mut().find(|(status, _)| *status == plan.status) {
            Some((_, amount)) => *amount += 1,
            None => count.push((plan.status.clone(), 1)),
        }
    }
    count.sort_by(|left, right| right.1.cmp(&left.1).then(left.0.cmp(&right.0)));
    if count.is_empty() {
        return "none".to_owned();
    }
    count
        .into_iter()
        .map(|(status, amount)| format!("{amount} {status}"))
        .collect::<Vec<_>>()
        .join(" · ")
}

fn constitution(context: &Context) -> Vec<PanelItem> {
    ["AGENTS.md", "docs/README.md", "CONTEXT.md"]
        .into_iter()
        .filter_map(|path| context.repository().get(path))
        .map(|record| {
            PanelItem::link(super::home_label(record.home_id()), doc_href(&record.path))
                .with_note(&record.path)
        })
        .collect()
}

/// Every stub route, so the router registers exactly what the tree links to.
pub(in crate::studio) fn stub_path() -> impl Iterator<Item = &'static str> {
    STUB.iter().map(|stub| stub.path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_stub_route_is_unique_and_absolute() {
        let mut path: Vec<&str> = stub_path().collect();
        let total = path.len();
        path.sort_unstable();
        path.dedup();

        assert_eq!(path.len(), total, "a stub route is registered twice");
        assert!(path.iter().all(|path| path.starts_with('/')));
        assert!(STUB.iter().all(|stub| stub.sentence.ends_with('.')));
    }

    #[test]
    fn a_plan_summary_counts_the_plans_it_is_given() {
        let plan = [
            plan::Plan {
                path: "a".to_owned(),
                id: "a".to_owned(),
                title: "A".to_owned(),
                status: "active".to_owned(),
                created_at: None,
                updated_at: None,
                accepted_at: None,
                completed_at: None,
                backlog_item: None,
                task: Vec::new(),
                open_question: None,
            },
            plan::Plan {
                path: "b".to_owned(),
                id: "b".to_owned(),
                title: "B".to_owned(),
                status: "complete".to_owned(),
                created_at: None,
                updated_at: None,
                accepted_at: None,
                completed_at: None,
                backlog_item: None,
                task: Vec::new(),
                open_question: None,
            },
        ];

        assert_eq!(plan_summary(&plan), "1 active · 1 complete");
        assert_eq!(plan_summary(&[]), "none");
    }
}
