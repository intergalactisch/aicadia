//! The complete, stable index of one section, computed from the projection on
//! every request. Nothing here is hand-maintained: models, capabilities, records,
//! plans, experiments and tables come from the governed roots, the compiled tool
//! catalog and the migrations.

use maud::{Markup, html};

use super::{Context, Section, doc_href};
use crate::{model, plan, register};

/// One entry of a section tree.
#[derive(Clone, Debug)]
pub(super) struct Leaf {
    label: String,
    href: Option<String>,
    note: Option<String>,
    child: Vec<Leaf>,
    /// A long leaf list is disclosed by `<details>` instead of always open.
    collapsible: bool,
    current: bool,
}

impl Leaf {
    fn link(label: impl Into<String>, href: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            href: Some(href.into()),
            note: None,
            child: Vec::new(),
            collapsible: false,
            current: false,
        }
    }

    fn folder(label: impl Into<String>, child: Vec<Self>) -> Self {
        Self {
            label: label.into(),
            href: None,
            note: None,
            child,
            collapsible: true,
            current: false,
        }
    }

    fn with_note(mut self, note: impl Into<String>) -> Self {
        self.note = Some(note.into());
        self
    }

    fn with_child(mut self, child: Vec<Self>) -> Self {
        self.child = child;
        self
    }

    fn counted(self, amount: usize, singular: &str, plural: &str) -> Self {
        let note = super::count(amount, singular, plural);
        self.with_note(note)
    }
}

/// One labelled block of a section tree; the leading block carries no label.
#[derive(Clone, Debug)]
pub(super) struct Group {
    label: Option<String>,
    item: Vec<Leaf>,
}

impl Group {
    fn lead(item: Vec<Leaf>) -> Self {
        Self { label: None, item }
    }

    fn named(label: impl Into<String>, item: Vec<Leaf>) -> Self {
        Self {
            label: Some(label.into()),
            item,
        }
    }
}

/// The whole tree of one section.
#[derive(Clone, Debug)]
pub(super) struct Tree {
    section: Section,
    group: Vec<Group>,
}

/// Build the tree of one section and mark the current page.
pub(super) fn build(context: &Context, section: Section, current_path: &str) -> Tree {
    let mut tree = Tree {
        section,
        group: match section {
            Section::Overview => overview(context),
            Section::Game => game(context),
            Section::Development => development(context),
            Section::Live => live(context),
        },
    };
    if !mark_groups(&mut tree.group, current_path)
        && let Some(parent) = canonical_parent(current_path)
    {
        mark_groups(&mut tree.group, &parent);
    }
    tree
}

/// Mark the first link whose canonical page equals the current path.
fn mark_groups(group: &mut [Group], current_path: &str) -> bool {
    let mut marked = false;
    for group in group {
        mark(&mut group.item, current_path, &mut marked);
    }
    marked
}

fn mark(item: &mut [Leaf], current_path: &str, marked: &mut bool) {
    for leaf in item {
        if !*marked
            && leaf.href.as_deref().is_some_and(|href| {
                href.split_once('#').map_or(href, |(path, _)| path) == current_path
            })
        {
            leaf.current = true;
            *marked = true;
        }
        mark(&mut leaf.child, current_path, marked);
    }
}

/// Detail routes inherit the stable collection or capability entry in the tree.
fn canonical_parent(current_path: &str) -> Option<String> {
    if let Some(name) = current_path.strip_prefix("/game/tool/") {
        return Some(format!("/game/capability/{name}"));
    }

    [
        "/live/activity",
        "/live/entity",
        "/live/place",
        "/live/character",
        "/live/user",
        "/live/property-key",
        "/live/trait",
        "/live/investigation",
        "/live/storage",
    ]
    .into_iter()
    .find(|parent| current_path.starts_with(&format!("{parent}/")))
    .map(str::to_owned)
}

fn overview(context: &Context) -> Vec<Group> {
    let repository = context.repository();

    vec![
        Group::lead(vec![
            Leaf::link("Overview", "/"),
            Leaf::link("Builder brief", "/brief").with_note("Markdown"),
        ]),
        Group::named(
            "Sections",
            vec![
                Leaf::link("Game", "/game").with_note(super::count(
                    model::models(repository).len(),
                    "model",
                    "models",
                )),
                Leaf::link("Development", "/development").with_note(super::count(
                    plan::plans(repository)
                        .iter()
                        .filter(|plan| plan.is_live())
                        .count(),
                    "live plan",
                    "live plans",
                )),
                Leaf::link("Live", "/live").with_note(if context.pulse().is_connected() {
                    "connected"
                } else {
                    "unavailable"
                }),
            ],
        ),
    ]
}

fn game(context: &Context) -> Vec<Group> {
    let repository = context.repository();
    let model = model::models(repository);
    let surface = context.surface();
    let capability: Vec<Leaf> = surface
        .tool
        .iter()
        .map(|tool| Leaf::link(&tool.name, format!("/game/capability/{}", tool.name)))
        .collect();
    let capability_count = capability.len();
    let model_count = model.len();
    let model_leaf = model
        .iter()
        .map(|model| {
            let leaf = Leaf::link(&model.title, format!("/game/model/{}", model.id));
            if model.storage_table.is_empty() {
                leaf
            } else {
                leaf.counted(model.storage_table.len(), "table", "tables")
            }
        })
        .collect();

    let mut contract: Vec<Leaf> = repository
        .in_home("game-index")
        .into_iter()
        .chain(repository.in_home("game-contract"))
        .map(|record| {
            let href = if record.path == "game/docs/deferred.md" {
                "/game/deferred".to_owned()
            } else {
                doc_href(&record.path)
            };
            Leaf::link(&record.title, href)
        })
        .collect();
    contract.sort_by(|left, right| left.label.cmp(&right.label));
    let contract_count = contract.len();

    vec![
        Group::lead(vec![Leaf::link("Overview", "/game")]),
        Group::named(
            "Models",
            vec![Leaf::folder("Durable models", model_leaf).with_note(model_count.to_string())],
        ),
        Group::named(
            "Capabilities",
            vec![
                Leaf::folder("Published capabilities", capability)
                    .with_note(capability_count.to_string()),
            ],
        ),
        Group::named(
            "Agent surface",
            vec![
                Leaf::link("Play contract", doc_href("game/docs/agent.md"))
                    .with_note(super::count(surface.section.len(), "section", "sections")),
                Leaf::link("Tool catalog", "/game/agent#tool-catalog")
                    .with_note(surface.tool.len().to_string()),
                Leaf::link("Host requirements", "/game/agent#host-requirements"),
            ],
        ),
        Group::named(
            "Contract",
            vec![Leaf::folder("Runtime contracts", contract).with_note(contract_count.to_string())],
        ),
        Group::named(
            "Reference",
            vec![
                Leaf::link("Vocabulary", "/game/vocabulary").with_note(super::count(
                    model::vocabulary(repository).len(),
                    "term",
                    "terms",
                )),
                Leaf::link("Storage", "/game/storage").with_note(super::count(
                    repository.in_home("migration").len(),
                    "migration",
                    "migrations",
                )),
            ],
        ),
    ]
}

fn development(context: &Context) -> Vec<Group> {
    let repository = context.repository();
    let plan = plan::plans(repository);
    let entry: usize = repository
        .in_home("decision-register")
        .into_iter()
        .map(|record| register::parse(record).entry.len())
        .sum();
    let open = plan::open_section(repository).len();

    let mut rule: Vec<Leaf> = ["AGENTS.md", "dev/docs/README.md", "dev/CONTEXT.md"]
        .into_iter()
        .filter_map(|path| repository.get(path))
        .map(|record| Leaf::link(&record.title, doc_href(&record.path)))
        .collect();
    rule.extend(
        repository
            .in_home("skill")
            .into_iter()
            .map(|record| Leaf::link(&record.title, doc_href(&record.path))),
    );
    let rule_count = rule.len();
    let direction: Vec<Leaf> = repository
        .in_home("concept-record")
        .into_iter()
        .map(|record| {
            let leaf = Leaf::link(&record.title, doc_href(&record.path));
            match record.front_matter.scalar("status") {
                Some(status) => leaf.with_note(status),
                None => leaf,
            }
        })
        .collect();
    let direction_count = direction.len();

    vec![
        Group::lead(vec![Leaf::link("Overview", "/development")]),
        Group::named(
            "Work",
            vec![
                Leaf::link("Current edge", "/development/work"),
                Leaf::folder(
                    "Plans",
                    plan.iter()
                        .map(|plan| {
                            Leaf::link(&plan.title, doc_href(&plan.path)).with_note(&plan.status)
                        })
                        .collect(),
                )
                .with_note(plan.len().to_string()),
                record_leaf(context, "dev/backlog/README.md"),
                record_leaf(context, "dev/backlog/capability-map.md"),
            ],
        ),
        Group::named(
            "Direction",
            vec![
                Leaf::link("Overview", "/development/direction"),
                Leaf::folder("Records", direction).with_note(direction_count.to_string()),
            ],
        ),
        Group::named(
            "Decisions",
            vec![
                Leaf::link("Register", "/development/decision")
                    .with_note(super::count(entry, "entry", "entries")),
                Leaf::link("Open questions", "/development/open")
                    .with_note(super::count(open, "section", "sections")),
            ],
        ),
        Group::named(
            "Research",
            vec![
                Leaf::link("Overview", "/development/research"),
                Leaf::folder(
                    "Reports",
                    repository
                        .in_home("research-report")
                        .into_iter()
                        .map(|record| Leaf::link(&record.title, doc_href(&record.path)))
                        .collect(),
                )
                .with_note(repository.in_home("research-report").len().to_string()),
            ],
        ),
        Group::named(
            "Lab",
            vec![
                Leaf::link("Overview", "/development/lab"),
                Leaf::folder("Tracks", lab(context))
                    .with_note(repository.in_home("lab-track").len().to_string()),
            ],
        ),
        Group::named(
            "Evidence",
            vec![
                Leaf::link("Overview", "/development/evidence"),
                Leaf::folder(
                    "Slices",
                    repository
                        .in_home("evidence-slice")
                        .into_iter()
                        .map(|record| Leaf::link(&record.title, doc_href(&record.path)))
                        .collect(),
                )
                .with_note(repository.in_home("evidence-slice").len().to_string()),
                Leaf::folder(
                    "Runners",
                    repository
                        .in_home("runner-contract")
                        .into_iter()
                        .map(|record| Leaf::link(&record.title, doc_href(&record.path)))
                        .collect(),
                )
                .with_note(repository.in_home("runner-contract").len().to_string()),
            ],
        ),
        Group::named(
            "Rules",
            vec![
                Leaf::link("Build rules", "/development/rules"),
                Leaf::folder("Sources", rule).with_note(rule_count.to_string()),
            ],
        ),
    ]
}

/// Every lab track with the experiments that live inside its own directory.
fn lab(context: &Context) -> Vec<Leaf> {
    let repository = context.repository();
    let experiment = repository.in_home("lab-experiment");

    repository
        .in_home("lab-track")
        .into_iter()
        .map(|track| {
            let directory = crate::home::directory(&track.path);
            let child: Vec<Leaf> = experiment
                .iter()
                .filter(|record| record.path.starts_with(&format!("{directory}/")))
                .map(|record| {
                    let leaf = Leaf::link(&record.title, doc_href(&record.path));
                    match record.front_matter.scalar("verdict") {
                        Some(verdict) => leaf.with_note(verdict),
                        None => leaf,
                    }
                })
                .collect();
            Leaf::link(&track.title, doc_href(&track.path))
                .counted(child.len(), "experiment", "experiments")
                .with_child(child)
        })
        .collect()
}

fn live(context: &Context) -> Vec<Group> {
    let table = model::storage_table(context.repository());

    vec![
        Group::lead(vec![
            Leaf::link("Overview", "/live"),
            Leaf::link("World chronicle", "/live/activity"),
            Leaf::link("Resolve an id", "/live/resolve"),
        ]),
        Group::named(
            "Subjects",
            vec![
                Leaf::link("Entities", "/live/entity"),
                Leaf::link("Characters", "/live/character"),
                Leaf::link("Places", "/live/place"),
                Leaf::link("Users", "/live/user"),
            ],
        ),
        Group::named(
            "State and history",
            vec![
                Leaf::link("Property keys", "/live/property-key"),
                Leaf::link("Traits", "/live/trait"),
                Leaf::link("Investigation attempts", "/live/investigation"),
            ],
        ),
        Group::named(
            "Storage",
            vec![
                Leaf::link("Schema", "/live/storage").counted(table.len(), "table", "tables"),
                Leaf::link("Migrations", "/live/migration").with_note(super::count(
                    context.repository().in_home("migration").len(),
                    "migration",
                    "migrations",
                )),
                Leaf::folder(
                    "Tables",
                    table
                        .iter()
                        .map(|table| {
                            Leaf::link(&table.name, format!("/live/storage/{}", table.name))
                        })
                        .collect(),
                )
                .with_note(table.len().to_string()),
            ],
        ),
    ]
}

/// One tree link to a governed record, or an honest placeholder when it is absent.
fn record_leaf(context: &Context, path: &str) -> Leaf {
    match context.repository().get(path) {
        Some(record) => Leaf::link(&record.title, doc_href(&record.path)),
        None => Leaf {
            label: path.to_owned(),
            href: None,
            note: Some("missing".to_owned()),
            child: Vec::new(),
            collapsible: false,
            current: false,
        },
    }
}

/// The tree as the sticky section sidebar.
pub(super) fn render(tree: &Tree) -> Markup {
    html! {
        nav class="tree" id="tree" aria-label=(tree.section.label()) {
            div class="tree-mobile" {
                div class="tree-mobile-head" {
                    b { "Studio navigation" }
                    button class="tree-close" type="button" data-tree-close aria-label="Close navigation" { "Close" }
                }
                form class="tree-jump" role="search" action="/jump" method="get" aria-label="Jump to a resource" {
                    input type="search" name="q" autocomplete="off"
                        placeholder="Jump to model, tool, record, table…" aria-label="Jump to";
                }
                div class="tree-sections" aria-label="Studio sections" {
                    @for section in Section::ALL {
                        a href=(section.href())
                            aria-current=[(section == tree.section).then_some("page")] { (section.label()) }
                    }
                }
            }
            p class="tree-title" { (tree.section.side()) b { (tree.section.tree_name()) } }
            @for group in &tree.group {
                @match &group.label {
                    None => ul role="list" { @for leaf in &group.item { (leaf_markup(leaf)) } },
                    Some(label) => div class="tree-group" {
                        p class="tree-label" { (label) }
                        ul role="list" { @for leaf in &group.item { (leaf_markup(leaf)) } }
                    },
                }
            }
        }
        button class="tree-backdrop" type="button" data-tree-close aria-label="Close navigation" {}
    }
}

fn leaf_markup(leaf: &Leaf) -> Markup {
    html! {
        li {
            @if leaf.collapsible && leaf.href.is_none() {
                details open[has_current(leaf)] {
                    summary class="tree-summary" {
                        (leaf.label)
                        @if let Some(note) = &leaf.note { small { (note) } }
                    }
                    ul role="list" { @for child in &leaf.child { (leaf_markup(child)) } }
                }
            } @else {
                @match &leaf.href {
                    Some(href) => a href=(href) aria-current=[leaf.current.then_some("page")] {
                        (leaf.label)
                        @if let Some(note) = &leaf.note { small { (note) } }
                    },
                    None => span class="tree-summary" {
                        (leaf.label)
                        @if let Some(note) = &leaf.note { small { (note) } }
                    },
                }
                @if !leaf.child.is_empty() {
                    ul role="list" { @for child in &leaf.child { (leaf_markup(child)) } }
                }
            }
        }
    }
}

fn has_current(leaf: &Leaf) -> bool {
    leaf.current || leaf.child.iter().any(has_current)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_first_matching_link_is_the_only_current_one() {
        let mut item = vec![
            Leaf::link("World chronicle", "/live/activity"),
            Leaf::link("Activity", "/live/activity"),
        ];
        let mut marked = false;
        mark(&mut item, "/live/activity", &mut marked);

        assert!(item[0].current);
        assert!(!item[1].current);
    }

    #[test]
    fn a_nested_leaf_is_marked_as_well() {
        let mut item = vec![Leaf::folder(
            "Tables",
            vec![Leaf::link("entity", "/live/storage/entity")],
        )];
        let mut marked = false;
        mark(&mut item, "/live/storage/entity", &mut marked);

        assert!(item[0].child[0].current);
    }

    #[test]
    fn a_fragment_link_marks_its_canonical_page() {
        let mut item = vec![Leaf::link("Tool catalog", "/game/agent#tool-catalog")];
        let mut marked = false;
        mark(&mut item, "/game/agent", &mut marked);

        assert!(item[0].current);
    }

    #[test]
    fn tool_and_live_details_resolve_to_their_tree_parent() {
        assert_eq!(
            canonical_parent("/game/tool/create_entity").as_deref(),
            Some("/game/capability/create_entity")
        );
        assert_eq!(
            canonical_parent("/live/entity/00000000-0000-0000-0000-000000000000/property/1")
                .as_deref(),
            Some("/live/entity")
        );
        assert_eq!(
            canonical_parent("/live/activity/00000000-0000-0000-0000-000000000000").as_deref(),
            Some("/live/activity")
        );
    }
}
