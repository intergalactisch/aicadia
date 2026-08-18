//! The Development section: repository direction, decisions, open work,
//! research, plans, experiments, evidence and the build constitution.
//!
//! This module owns presentation only. Every status, count, question, task,
//! verdict, seam and rule body below comes from the repository projection.

use std::collections::{BTreeMap, BTreeSet};

use axum::{
    extract::{Query, State},
    http::{HeaderMap, Uri},
    response::Response,
};
use maud::{Markup, PreEscaped, html};
use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};
use serde::Deserialize;

use super::{
    Action, Context, Crumb, Page, Panel, PanelItem, Plate, PlateRow, Reference, Seal, Section,
    Tone, doc_href,
};
use crate::{
    StudioState, home, plan,
    record::{LinkTarget, Record, Repository, resolve_link, resolve_reference},
    register::{self, Entry, Register},
};

const DEVELOPMENT_PATH: &str = "/development";

/// `/development` — one source-backed index of the development side.
pub(crate) async fn development(
    State(state): State<StudioState>,
    header: HeaderMap,
    uri: Uri,
) -> Response {
    let context = match Context::build(&state, &header, &uri).await {
        Ok(context) => context,
        Err(response) => return response,
    };
    let repository = context.repository();
    let plans = plan::plans(repository);
    let decision_count = registers(repository)
        .iter()
        .map(|register| register.entry.len())
        .sum::<usize>();
    let task_count = plans
        .iter()
        .filter(|plan| plan.is_live())
        .map(|plan| plan.task.len())
        .sum::<usize>();

    let page = Page::new(Section::Development, "Development")
        .with_document_title("Development · Aicadia Studio")
        .with_lede(
            "Direction, recorded choices, open work, research, experiments, evidence and the rules that govern every build.",
        )
        .with_plate(
            Plate::new(vec![
                PlateRow::text(
                    "Direction",
                    super::count(
                        repository.in_home("concept-record").len(),
                        "live record",
                        "live records",
                    ),
                ),
                PlateRow::text(
                    "Decisions",
                    super::count(decision_count, "entry", "entries"),
                ),
                PlateRow::text(
                    "Open",
                    super::count(
                        plan::open_section(repository).len(),
                        "section",
                        "sections",
                    ),
                ),
                PlateRow::text("Live plan tasks", task_count.to_string()),
            ])
            .with_authority(
                "This index owns no development state. It projects the governed records linked from each destination.",
            ),
        )
        .with_content(development_index(repository))
        .with_related(vec![Panel::list(
            "Authorities",
            authority_items(repository),
        )])
        .with_reference(reference(&context, "Development", DEVELOPMENT_PATH));

    context.render(page)
}

/// `/development/direction` — every live concept record and its confirmed/open headings.
pub(crate) async fn direction(
    State(state): State<StudioState>,
    header: HeaderMap,
    uri: Uri,
) -> Response {
    let context = match Context::build(&state, &header, &uri).await {
        Ok(context) => context,
        Err(response) => return response,
    };
    let records = context.repository().in_home("concept-record");
    let confirmed_count = records
        .iter()
        .flat_map(|record| &record.heading)
        .filter(|heading| heading.title.starts_with("Confirmed"))
        .count();
    let open_count = records
        .iter()
        .flat_map(|record| &record.heading)
        .filter(|heading| heading.title.starts_with("Open"))
        .count();

    let page = development_page(&context, "Direction", "/development/direction")
        .with_lede(
            "Live exploration records, with every confirmed and still-open section linked at its owning heading.",
        )
        .with_plate(
            Plate::new(vec![
                PlateRow::text("Records", records.len().to_string()),
                PlateRow::text("Confirmed sections", confirmed_count.to_string()),
                PlateRow::text("Open sections", open_count.to_string()),
            ])
            .with_authority(
                "Each status and heading is read from its concept record; accepted game behavior still lives only in game/docs/.",
            ),
        )
        .with_content(direction_content(&records));

    context.render(page)
}

/// Facets of the append-only decision register. Empty values mean no facet.
#[derive(Clone, Debug, Default, Deserialize)]
pub(crate) struct DecisionFilter {
    q: Option<String>,
    date: Option<String>,
    topic: Option<String>,
    tag: Option<String>,
}

/// `/development/decision` — the filterable append-only concept-log register.
pub(crate) async fn decision(
    State(state): State<StudioState>,
    header: HeaderMap,
    uri: Uri,
    Query(filter): Query<DecisionFilter>,
) -> Response {
    let context = match Context::build(&state, &header, &uri).await {
        Ok(context) => context,
        Err(response) => return response,
    };
    let register = registers(context.repository());
    let mut entry = decision_entries(&register);
    let total = entry.len();
    entry.retain(|(_, entry)| decision_matches(entry, &filter));
    let showing = entry.len();

    let page = development_page(&context, "Decision register", "/development/decision")
        .with_lede(
            "Every recorded choice, correction, rejection and deferral, filterable by its source-owned date, topic, tag and text.",
        )
        .with_plate(
            Plate::new(vec![
                PlateRow::text("Entries", total.to_string()),
                PlateRow::text("Showing", showing.to_string()),
                PlateRow::text("Periods", register.len().to_string()),
            ])
            .with_action(vec![Action::link("Clear filters", "/development/decision")])
            .with_authority(
                "The register is parsed from dev/docs/concept/log; Studio adds only facets and stable page anchors.",
            ),
        )
        .with_content(decision_content(
            context.repository(),
            &register,
            &entry,
            &filter,
            total,
        ))
        .with_related(vec![Panel::list(
            "Register sources",
            register
                .iter()
                .map(|register| {
                    PanelItem::link(&register.title, doc_href(&register.path))
                        .with_note(super::count(register.entry.len(), "entry", "entries"))
                })
                .collect(),
        )]);

    context.render(page)
}

/// `/development/open` — every parsed live `Open …` section.
pub(crate) async fn open(
    State(state): State<StudioState>,
    header: HeaderMap,
    uri: Uri,
) -> Response {
    let context = match Context::build(&state, &header, &uri).await {
        Ok(context) => context,
        Err(response) => return response,
    };
    let open = plan::open_section(context.repository());

    let page = development_page(&context, "Open questions", "/development/open")
        .with_seal(vec![Seal::status("Open")])
        .with_lede(
            "Every open section in a live concept record, draft or active plan, or backlog item, without a second question list.",
        )
        .with_plate(
            Plate::new(vec![PlateRow::text("Sections", open.len().to_string())])
                .with_authority(
                    "The heading and body remain owned by the linked source; completed and frozen plans are excluded by the projection.",
                ),
        )
        .with_content(open_content(context.repository(), &open))
        .with_related(vec![Panel::list(
            "Sources",
            source_items(context.repository(), open.iter().map(|section| section.path.as_str())),
        )]);

    context.render(page)
}

/// `/development/research` — the research index; each report opens in the shared reader.
pub(crate) async fn research(
    State(state): State<StudioState>,
    header: HeaderMap,
    uri: Uri,
) -> Response {
    let context = match Context::build(&state, &header, &uri).await {
        Ok(context) => context,
        Err(response) => return response,
    };
    context.render(record_index_page(
        &context,
        "Research",
        "/development/research",
        "Research reports and their source-owned standing; each report opens in the governed record reader.",
        &["research-report"],
    ))
}

/// `/development/evidence` — delivery slices and runner contracts.
pub(crate) async fn evidence(
    State(state): State<StudioState>,
    header: HeaderMap,
    uri: Uri,
) -> Response {
    let context = match Context::build(&state, &header, &uri).await {
        Ok(context) => context,
        Err(response) => return response,
    };
    context.render(record_index_page(
        &context,
        "Evidence",
        "/development/evidence",
        "Delivery evidence and evidence-producing runner contracts, read from their owning records.",
        &["evidence-slice", "runner-contract"],
    ))
}

/// `/development/work` — current edge, live plan boards, horizon and capability map.
pub(crate) async fn work(
    State(state): State<StudioState>,
    header: HeaderMap,
    uri: Uri,
) -> Response {
    let context = match Context::build(&state, &header, &uri).await {
        Ok(context) => context,
        Err(response) => return response,
    };
    let plans = plan::plans(context.repository());
    let live = plans
        .iter()
        .filter(|plan| plan.is_live())
        .collect::<Vec<_>>();
    let live_task_count = live.iter().map(|plan| plan.task.len()).sum::<usize>();
    let horizon = plan::horizon(context.repository());
    let current_edge_count = horizon
        .iter()
        .filter(|row| row.horizon.eq_ignore_ascii_case("Now"))
        .count();

    let page = development_page(&context, "Current edge and work", "/development/work")
        .with_lede(
            "The selected edge, every draft or active plan as its own task board, and the ordered backlog horizon.",
        )
        .with_plate(
            Plate::new(vec![
                PlateRow::text("Current edges", current_edge_count.to_string()),
                PlateRow::text("Live plans", live.len().to_string()),
                PlateRow::text("Live plan tasks", live_task_count.to_string()),
                PlateRow::text("Horizon rows", horizon.len().to_string()),
            ])
            .with_authority(
                "Plan status and tasks live in each plan; backlog order and state live only in the horizon table.",
            ),
        )
        .with_content(work_content(context.repository(), &plans, &horizon))
        .with_related(vec![Panel::list(
            "Planning homes",
            ["dev/backlog/README.md", "dev/backlog/capability-map.md"]
                .into_iter()
                .filter_map(|path| context.repository().get(path))
                .map(|record| PanelItem::link(&record.title, doc_href(&record.path)))
                .collect(),
        )]);

    context.render(page)
}

/// `/development/lab` — every track and experiment with its complete metadata.
pub(crate) async fn lab(State(state): State<StudioState>, header: HeaderMap, uri: Uri) -> Response {
    let context = match Context::build(&state, &header, &uri).await {
        Ok(context) => context,
        Err(response) => return response,
    };
    let tracks = context.repository().in_home("lab-track");
    let experiments = context.repository().in_home("lab-experiment");

    let page = development_page(&context, "Lab", "/development/lab")
        .with_lede(
            "Tracks and bounded experiments with their question, real and simulated seams, verdict, artifact status and informed decision.",
        )
        .with_plate(
            Plate::new(vec![
                PlateRow::text("Tracks", tracks.len().to_string()),
                PlateRow::text("Experiments", experiments.len().to_string()),
            ])
            .with_authority(
                "Experiment metadata is read only from each README front matter; retained artifacts never become production authority.",
            ),
        )
        .with_content(lab_content(context.repository(), &tracks, &experiments));

    context.render(page)
}

/// `/development/rules` — every build heuristic from `AGENTS.md`.
pub(crate) async fn rules(
    State(state): State<StudioState>,
    header: HeaderMap,
    uri: Uri,
) -> Response {
    let context = match Context::build(&state, &header, &uri).await {
        Ok(context) => context,
        Err(response) => return response,
    };
    let record = context
        .repository()
        .get("AGENTS.md")
        .expect("AGENTS.md is a governed required record");
    let heuristic = heuristic_headings(record);

    let page = development_page(&context, "Build rules", "/development/rules")
        .with_lede(
            "The build heuristics of AGENTS.md, rendered directly from the constitution that governs this repository.",
        )
        .with_plate(
            Plate::new(vec![
                PlateRow::text("Heuristics", heuristic.len().to_string()),
                PlateRow::fact("Authority", &record.path),
            ])
            .with_action(vec![Action::link("Read complete constitution", doc_href(&record.path))])
            .with_authority(
                record
                    .role_header
                    .as_ref()
                    .map_or("AGENTS.md".to_owned(), |role| role.authority.clone()),
            ),
        )
        .with_content(rules_content(context.repository(), record, &heuristic))
        .with_related(vec![Panel::list(
            "Constitutions",
            authority_items(context.repository()),
        )]);

    context.render(page)
}

fn development_page(context: &Context, title: &str, path: &str) -> Page {
    Page::new(Section::Development, title)
        .with_crumb(vec![
            Crumb::link("Development", DEVELOPMENT_PATH),
            Crumb::here(title),
        ])
        .with_reference(reference(context, title, path))
}

fn reference(context: &Context, title: &str, path: &str) -> Reference {
    Reference {
        title: title.to_owned(),
        url: context.url(path),
        context: "projection of governed development records".to_owned(),
    }
}

fn development_index(repository: &Repository) -> Markup {
    let plan = plan::plans(repository);
    let experiment = repository.in_home("lab-experiment").len();
    let evidence = repository.in_home("evidence-slice").len();
    let research = repository.in_home("research-report").len();
    let rules = repository
        .get("AGENTS.md")
        .map_or(0, |record| heuristic_headings(record).len());

    html! {
        section class="section" {
            h2 { "Explore the development side" }
            ul class="list" role="list" {
                (index_row("Direction", "/development/direction", repository.in_home("concept-record").len(), "record", "records"))
                (index_row("Decision register", "/development/decision", registers(repository).iter().map(|register| register.entry.len()).sum(), "entry", "entries"))
                (index_row("Open questions", "/development/open", plan::open_section(repository).len(), "section", "sections"))
                (index_row("Research", "/development/research", research, "report", "reports"))
                (index_row("Current edge and work", "/development/work", plan.iter().filter(|plan| plan.is_live()).count(), "live plan", "live plans"))
                (index_row("Lab", "/development/lab", experiment, "experiment", "experiments"))
                (index_row("Evidence", "/development/evidence", evidence, "slice", "slices"))
                (index_row("Build rules", "/development/rules", rules, "heuristic", "heuristics"))
            }
        }
    }
}

fn index_row(label: &str, href: &str, amount: usize, singular: &str, plural: &str) -> Markup {
    html! { li { a href=(href) { (label) } small { (super::count(amount, singular, plural)) } } }
}

fn direction_content(records: &[&Record]) -> Markup {
    html! {
        div data-direction-record-count=(records.len()) {
            @for record in records {
                @let confirmed = record.heading.iter().filter(|heading| heading.title.starts_with("Confirmed")).collect::<Vec<_>>();
                @let open = record.heading.iter().filter(|heading| heading.title.starts_with("Open")).collect::<Vec<_>>();
                section class="section" id=(home::file_name(&record.path).trim_end_matches(".md")) {
                    h2 { a href=(doc_href(&record.path)) { (&record.title) } }
                    div class="seals" {
                        @if let Some(status) = record.front_matter.scalar("status") {
                            span class=(status_class(status)) { (status) }
                        }
                        span class="seal seal-plain" { (record.path.as_str()) }
                    }
                    div class="state-grid" {
                        section class="state" {
                            h2 { "Confirmed sections" }
                            (heading_list(record, &confirmed, "No confirmed section is named in this record."))
                        }
                        section class="state" {
                            h2 { "Open sections" }
                            (heading_list(record, &open, "No open section is named in this record."))
                        }
                    }
                }
            }
        }
    }
}

fn heading_list(record: &Record, heading: &[&crate::record::Heading], empty: &str) -> Markup {
    if heading.is_empty() {
        return html! { p class="mute" { (empty) } };
    }
    html! {
        ul class="list" role="list" {
            @for heading in heading {
                li {
                    a href=(format!("{}#{}", doc_href(&record.path), heading.id)) { (&heading.title) }
                    small { (format!("h{}", heading.level)) }
                }
            }
        }
    }
}

fn registers(repository: &Repository) -> Vec<Register> {
    repository
        .in_home("decision-register")
        .into_iter()
        .map(register::parse)
        .collect()
}

fn decision_entries(register: &[Register]) -> Vec<(&Register, &Entry)> {
    let mut entry = register
        .iter()
        .flat_map(|register| register.entry.iter().map(move |entry| (register, entry)))
        .collect::<Vec<_>>();
    entry.sort_by(|(left_register, left), (right_register, right)| {
        right
            .date
            .cmp(&left.date)
            .then(right_register.period.cmp(&left_register.period))
            .then(left.line.cmp(&right.line))
    });
    entry
}

fn decision_matches(entry: &Entry, filter: &DecisionFilter) -> bool {
    let exact = |selected: &Option<String>, value: &str| {
        selected
            .as_deref()
            .filter(|selected| !selected.trim().is_empty())
            .is_none_or(|selected| selected.eq_ignore_ascii_case(value))
    };
    if !exact(&filter.date, &entry.date)
        || !exact(&filter.topic, &entry.topic)
        || !exact(&filter.tag, &entry.tag)
    {
        return false;
    }
    let Some(query) = filter
        .q
        .as_deref()
        .map(str::trim)
        .filter(|query| !query.is_empty())
    else {
        return true;
    };
    let query = query.to_lowercase();
    [
        entry.date.as_str(),
        entry.topic.as_str(),
        entry.sub_topic.as_deref().unwrap_or(""),
        entry.tag.as_str(),
        entry.qualifier.as_deref().unwrap_or(""),
        entry.text.as_str(),
    ]
    .iter()
    .any(|value| value.to_lowercase().contains(&query))
}

fn decision_content(
    repository: &Repository,
    register: &[Register],
    entries: &[(&Register, &Entry)],
    filter: &DecisionFilter,
    total: usize,
) -> Markup {
    let all = decision_entries(register);
    let date = facet_count(all.iter().map(|(_, entry)| entry.date.as_str()));
    let topic = facet_count(all.iter().map(|(_, entry)| entry.topic.as_str()));
    let tag = facet_count(all.iter().map(|(_, entry)| entry.tag.as_str()));
    let dates = unique(entries.iter().map(|(_, entry)| entry.date.as_str()));

    html! {
        form class="toolbar" action="/development/decision" method="get" role="search" {
            input type="search" name="q" value=[filter.q.as_deref()]
                placeholder="Filter loaded decision text…" aria-label="Filter decision text";
            @if let Some(value) = nonempty(&filter.date) { input type="hidden" name="date" value=(value); }
            @if let Some(value) = nonempty(&filter.topic) { input type="hidden" name="topic" value=(value); }
            @if let Some(value) = nonempty(&filter.tag) { input type="hidden" name="tag" value=(value); }
            button class="btn btn-small" type="submit" { "Search decisions" }
        }
        details class="facets" {
            summary class="btn btn-small btn-quiet" { "Refine by date, tag or topic" }
            div class="facet-grid" {
                section class="facet-group" {
                    h2 { "Date" }
                    div class="chips" {
                        @for (value, count) in &date {
                            a class=(chip_class(filter.date.as_deref(), value))
                                href=(facet_href(filter, "date", value)) aria-pressed=(is_selected(filter.date.as_deref(), value)) {
                                (value) small { (count) }
                            }
                        }
                    }
                }
                section class="facet-group" {
                    h2 { "Tag" }
                    div class="chips" {
                        @for (value, count) in &tag {
                            a class=(chip_class(filter.tag.as_deref(), value))
                                href=(facet_href(filter, "tag", value)) aria-pressed=(is_selected(filter.tag.as_deref(), value)) {
                                (value) small { (count) }
                            }
                        }
                    }
                }
                section class="facet-group" {
                    h2 { "Topic" }
                    div class="chips" {
                        @for (value, count) in &topic {
                            a class=(chip_class(filter.topic.as_deref(), value))
                                href=(facet_href(filter, "topic", value)) aria-pressed=(is_selected(filter.topic.as_deref(), value)) {
                                (value) small { (count) }
                            }
                        }
                    }
                }
            }
        }
        @if entries.is_empty() {
            (super::empty_state("No decision entry matches these facets."))
        } @else {
            div class="ledger" data-entry-count=(total) data-filtered-entry-count=(entries.len()) {
                @for date in dates {
                    @let day = entries.iter().filter(|(_, entry)| entry.date == date).copied().collect::<Vec<_>>();
                    @let topics = unique(day.iter().map(|(_, entry)| entry.topic.as_str()));
                    div class="ledger-date" { (&date) small { (super::count(day.len(), "entry", "entries")) } }
                    div class="ledger-day" {
                        @for topic in topics {
                            h3 { (&topic) }
                            @for (source, entry) in day.iter().filter(|(_, entry)| entry.topic == topic) {
                                @let id = decision_id(entry);
                                @let affected = affected_paths(repository, entry);
                                @let backlinks = entry_backlinks(repository, source, entry);
                                article class="entry" id=(&id) {
                                    div class="entry-tag" {
                                        span class=(status_class(&entry.tag)) { (&entry.tag) }
                                    }
                                    div class="entry-text" { (markdown(repository, &source.path, &entry.text)) }
                                    footer class="entry-foot" {
                                        a href=(format!("#{id}")) { "Deep link" }
                                        a href=(doc_href(&source.path)) { (format!("{} · line {}", source.period, entry.line)) }
                                        @if entry.sub_topic.is_some() || entry.qualifier.is_some() || !affected.is_empty() || !backlinks.is_empty() {
                                            details class="entry-provenance" {
                                                summary { "More context" }
                                                div class="entry-provenance-links" {
                                                    @if let Some(sub_topic) = &entry.sub_topic { span { (sub_topic) } }
                                                    @if let Some(qualifier) = &entry.qualifier { span { (qualifier) } }
                                                    @for path in &affected {
                                                        @if let Some(record) = repository.get(path) {
                                                            a href=(doc_href(&record.path)) { (&record.title) }
                                                        }
                                                    }
                                                    @for backlink in &backlinks {
                                                        @if let Some(record) = repository.get(backlink) {
                                                            a href=(doc_href(&record.path)) { (format!("Referenced by {}", record.title)) }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn decision_id(entry: &Entry) -> String {
    format!(
        "decision-{}-{}-{}",
        entry.date, entry.topic_id, entry.ordinal
    )
}

fn affected_paths(repository: &Repository, entry: &Entry) -> Vec<String> {
    let mut path = BTreeSet::new();
    for link in &entry.link {
        if let LinkTarget::Repository { path: target, .. } = &link.resolved
            && repository.get(target).is_some()
        {
            path.insert(target.clone());
        }
    }
    path.into_iter().collect()
}

fn entry_backlinks<'a>(
    repository: &'a Repository,
    register: &Register,
    entry: &Entry,
) -> Vec<&'a str> {
    let date_id = crate::record::heading_id(&entry.date);
    let sub_topic_id = entry.sub_topic_id.as_deref();
    let mut path = BTreeSet::new();
    for backlink in repository.backlink(&register.path) {
        let relevant = backlink.anchor.as_deref().is_some_and(|anchor| {
            anchor == date_id || anchor == entry.topic_id || sub_topic_id == Some(anchor)
        });
        if relevant {
            path.insert(backlink.path.as_str());
        }
    }
    path.into_iter().collect()
}

fn facet_count<'a>(value: impl Iterator<Item = &'a str>) -> Vec<(String, usize)> {
    let mut count = BTreeMap::<String, usize>::new();
    for value in value {
        *count.entry(value.to_owned()).or_default() += 1;
    }
    let mut count = count.into_iter().collect::<Vec<_>>();
    count.sort_by(|left, right| right.1.cmp(&left.1).then(left.0.cmp(&right.0)));
    count
}

fn unique<'a>(value: impl Iterator<Item = &'a str>) -> Vec<String> {
    let mut seen = BTreeSet::new();
    let mut unique = Vec::new();
    for value in value {
        if seen.insert(value) {
            unique.push(value.to_owned());
        }
    }
    unique
}

fn nonempty(value: &Option<String>) -> Option<&str> {
    value
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
}

fn is_selected(selected: Option<&str>, value: &str) -> &'static str {
    if selected.is_some_and(|selected| selected.eq_ignore_ascii_case(value)) {
        "true"
    } else {
        "false"
    }
}

fn chip_class(selected: Option<&str>, value: &str) -> &'static str {
    if is_selected(selected, value) == "true" {
        "chip is-on"
    } else {
        "chip"
    }
}

fn facet_href(filter: &DecisionFilter, facet: &str, value: &str) -> String {
    let mut item = Vec::<(&str, &str)>::new();
    if let Some(q) = nonempty(&filter.q) {
        item.push(("q", q));
    }
    for (key, selected) in [
        ("date", nonempty(&filter.date)),
        ("topic", nonempty(&filter.topic)),
        ("tag", nonempty(&filter.tag)),
    ] {
        if key == facet && selected.is_some_and(|selected| selected.eq_ignore_ascii_case(value)) {
            continue;
        }
        if key == facet {
            item.push((key, value));
        } else if let Some(selected) = selected {
            item.push((key, selected));
        }
    }
    if item.is_empty() {
        return "/development/decision".to_owned();
    }
    format!(
        "/development/decision?{}",
        item.into_iter()
            .map(|(key, value)| format!("{key}={}", encode_query(value)))
            .collect::<Vec<_>>()
            .join("&")
    )
}

fn encode_query(value: &str) -> String {
    let mut encoded = String::new();
    for byte in value.bytes() {
        if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b'~') {
            encoded.push(char::from(byte));
        } else {
            encoded.push_str(&format!("%{byte:02X}"));
        }
    }
    encoded
}

fn open_content(repository: &Repository, open: &[plan::OpenSection]) -> Markup {
    html! {
        div data-open-section-count=(open.len()) {
            @if open.is_empty() {
                (super::empty_state("No live development record contains an open section."))
            } @else {
                @for section in open {
                    @let source = repository.get(&section.path);
                    section class="section" id=(format!("open-{}", crate::record::heading_id(&format!("{}-{}", section.path, section.heading_id)))) {
                        h2 {
                            a href=(format!("{}#{}", doc_href(&section.path), section.heading_id)) { (&section.heading) }
                            span class="count" { (source.map_or(section.path.as_str(), |record| record.title.as_str())) }
                        }
                        div class="seals" {
                            span class="seal seal-plain" { (super::home_label(section.home_id)) }
                            @if let Some(status) = source.and_then(|record| record.front_matter.scalar("status")) {
                                span class=(status_class(status)) { (status) }
                            }
                        }
                        div class="prose" { (markdown(repository, &section.path, &section.body)) }
                    }
                }
            }
        }
    }
}

fn record_index_page(
    context: &Context,
    title: &str,
    path: &str,
    lede: &str,
    homes: &[&str],
) -> Page {
    let mut records = homes
        .iter()
        .flat_map(|home| context.repository().in_home(home))
        .collect::<Vec<_>>();
    records.sort_by(|left, right| left.title.cmp(&right.title));

    development_page(context, title, path)
        .with_lede(lede)
        .with_plate(
            Plate::new(vec![PlateRow::text("Records", records.len().to_string())])
                .with_authority(
                    "Titles, status and standing come from each record; the shared reader renders its complete body.",
                ),
        )
        .with_content(record_table(&records, title))
}

fn record_table(records: &[&Record], label: &str) -> Markup {
    html! {
        div class="data-wrap" data-record-count=(records.len()) {
            table class="data" {
                thead { tr { th { (label) } th { "Status" } th { "Owning path" } } }
                tbody {
                    @for record in records {
                        tr {
                            td { a class="row-link" href=(doc_href(&record.path)) { (&record.title) } }
                            td class="mute" { (record_status(record)) }
                            td class="fact" { (&record.path) }
                        }
                    }
                }
            }
        }
    }
}

fn record_status(record: &Record) -> &str {
    record
        .front_matter
        .scalar("status")
        .or_else(|| record.front_matter.scalar("standing"))
        .or_else(|| record.front_matter.scalar("verdict"))
        .unwrap_or("—")
}

fn work_content(
    repository: &Repository,
    plans: &[plan::Plan],
    horizon: &[plan::HorizonRow],
) -> Markup {
    let live = plans
        .iter()
        .filter(|plan| plan.is_live())
        .collect::<Vec<_>>();
    let live_task_count = live.iter().map(|plan| plan.task.len()).sum::<usize>();
    let current = horizon
        .iter()
        .filter(|row| row.horizon.eq_ignore_ascii_case("Now"))
        .collect::<Vec<_>>();

    html! {
        div data-plan-task-count=(live_task_count) data-live-plan-count=(live.len()) {
            section class="section" {
                h2 { "Current edge" }
                @if current.is_empty() {
                    (super::empty_state("The backlog horizon has no Now row, so no next game edge is selected."))
                } @else {
                    ul class="list" role="list" {
                        @for row in current { (horizon_list_row(row)) }
                    }
                }
            }
            section class="section" {
                h2 { "Live plan boards" span class="count" { (super::count(live.len(), "plan", "plans")) } }
                @if live.is_empty() {
                    (super::empty_state("No draft or active plan is present."))
                } @else {
                    @for plan in live { (plan_board(repository, plan)) }
                }
            }
            section class="section" {
                h2 { "Ordered backlog horizon" }
                div class="data-wrap" data-horizon-row-count=(horizon.len()) {
                    table class="data" {
                        thead { tr { th { "Horizon" } th { "Item" } th { "State" } th { "Concrete outcome" } } }
                        tbody {
                            @for row in horizon {
                                tr {
                                    td { (row.horizon.as_str()) }
                                    td {
                                        @if let Some(path) = &row.item_path {
                                            a class="row-link" href=(doc_href(path)) { (&row.item) }
                                        } @else { (&row.item) }
                                    }
                                    td { span class=(status_class(&row.state)) { (&row.state) } }
                                    td { (&row.outcome) }
                                }
                            }
                        }
                    }
                }
            }
            (capability_map(repository))
            section class="section" {
                h2 { "Completed and dropped plans" }
                ul class="list" role="list" {
                    @for plan in plans.iter().filter(|plan| !plan.is_live()) {
                        li { a href=(doc_href(&plan.path)) { (&plan.title) } small { (&plan.status) } }
                    }
                }
            }
        }
    }
}

fn plan_board(repository: &Repository, plan: &plan::Plan) -> Markup {
    let mut states = unique(plan.task.iter().map(|task| task.state.as_str()));
    states.sort_by_key(|state| task_state_order(state));
    html! {
        section class="section" id=(&plan.id) {
            h2 { a href=(doc_href(&plan.path)) { (&plan.title) } }
            div class="seals" {
                span class=(status_class(&plan.status)) { (&plan.status) }
                span class="seal seal-plain" { (super::count(plan.task.len(), "task", "tasks")) }
            }
            @if plan.task.is_empty() {
                (super::empty_state("This plan has no parsed task graph."))
            } @else {
                div class="board" data-plan-id=(&plan.id) data-task-count=(plan.task.len()) {
                    @for state in states {
                        @let tasks = plan.task.iter().filter(|task| task.state == state).collect::<Vec<_>>();
                        section class="lane" {
                            h3 { span { (&state) } span { (tasks.len()) } }
                            @for task in tasks {
                                article class="card" id=(format!("{}-{}", plan.id, task.id.to_lowercase())) {
                                    b { (&task.id) }
                                    div { (markdown(repository, &plan.path, &task.objective)) }
                                    @if task.depends != "—" {
                                        p class="task-dep" { "Depends on " code { (&task.depends) } }
                                    }
                                    details class="task-detail" {
                                        summary { "Implementation details" }
                                        dl class="meta" {
                                            dt { "Parallel" } dd { (&task.parallel_safe) }
                                            dt { "Surface" } dd { (markdown(repository, &plan.path, &task.owned_surface)) }
                                            dt { "Evidence" } dd { (markdown(repository, &plan.path, &task.evidence)) }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn task_state_order(state: &str) -> usize {
    match state {
        "in_progress" => 0,
        "pending" => 1,
        "blocked" => 2,
        "completed" => 3,
        _ => 4,
    }
}

fn horizon_list_row(row: &plan::HorizonRow) -> Markup {
    html! {
        li {
            @if let Some(path) = &row.item_path {
                a href=(doc_href(path)) { (&row.item) }
            } @else { span { (&row.item) } }
            small { (&row.state) }
        }
    }
}

fn capability_map(repository: &Repository) -> Markup {
    let Some(record) = repository.get("dev/backlog/capability-map.md") else {
        return Markup::default();
    };
    html! {
        section class="section" {
            h2 { a href=(doc_href(&record.path)) { (&record.title) } }
            p class="section-note" { (record.path.as_str()) }
            ul class="list" role="list" {
                @for heading in record.heading.iter().filter(|heading| heading.level == 2) {
                    li {
                        a href=(format!("{}#{}", doc_href(&record.path), heading.id)) { (&heading.title) }
                        small { "section" }
                    }
                }
            }
        }
    }
}

fn lab_content(repository: &Repository, tracks: &[&Record], experiments: &[&Record]) -> Markup {
    html! {
        div data-track-count=(tracks.len()) data-experiment-count=(experiments.len()) {
            @for track in tracks {
                @let directory = home::directory(&track.path);
                @let children = experiments.iter().filter(|experiment| experiment.path.starts_with(&format!("{directory}/"))).copied().collect::<Vec<_>>();
                section class="section" id=(home::file_name(directory)) {
                    h2 { a href=(doc_href(&track.path)) { (&track.title) } }
                    div class="seals" {
                        @if let Some(status) = track.front_matter.scalar("status") {
                            span class=(status_class(status)) { (status) }
                        }
                        span class="seal seal-plain" { (super::count(children.len(), "experiment", "experiments")) }
                    }
                    @for experiment in children {
                        article class="section" id=(home::file_name(home::directory(&experiment.path))) {
                            h3 class="subhead" { a href=(doc_href(&experiment.path)) { (&experiment.title) } }
                            div class="seals" {
                                @if let Some(verdict) = experiment.front_matter.scalar("verdict") {
                                    span class=(status_class(verdict)) { (verdict) }
                                }
                                @if let Some(status) = experiment.front_matter.scalar("status") {
                                    span class=(status_class(status)) { (status) }
                                }
                            }
                            dl class="meta" {
                                dt { "Question" }
                                dd { (experiment.front_matter.scalar("question").unwrap_or("—")) }
                                dt { "Real seams" }
                                dd { (front_matter_list(experiment, "real_seam")) }
                                dt { "Simulated seams" }
                                dd { (front_matter_list(experiment, "simulated_seam")) }
                                dt { "Informs" }
                                dd { (informs_link(repository, experiment)) }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn front_matter_list(record: &Record, key: &str) -> String {
    record.front_matter.list(key).map_or_else(
        || "—".to_owned(),
        |item| {
            if item.is_empty() {
                "—".to_owned()
            } else {
                item.join(" · ")
            }
        },
    )
}

fn informs_link(repository: &Repository, record: &Record) -> Markup {
    let Some(value) = record.front_matter.scalar("informs") else {
        return html! { "—" };
    };
    match resolve_reference(value) {
        LinkTarget::Repository { path, anchor } if repository.get(&path).is_some() => {
            let href = match anchor {
                Some(anchor) => format!("{}#{anchor}", doc_href(&path)),
                None => doc_href(&path),
            };
            html! { a href=(href) { (value) } }
        }
        _ => html! { code class="fact" { (value) } },
    }
}

fn heuristic_headings(record: &Record) -> Vec<&crate::record::Heading> {
    let mut inside = false;
    let mut heuristic = Vec::new();
    for heading in &record.heading {
        if heading.level == 2 {
            inside = heading.title == "Build Heuristics";
            continue;
        }
        if inside && heading.level == 3 {
            heuristic.push(heading);
        }
    }
    heuristic
}

fn rules_content(
    repository: &Repository,
    record: &Record,
    heuristic: &[&crate::record::Heading],
) -> Markup {
    html! {
        div data-heuristic-count=(heuristic.len()) {
            @for heading in heuristic {
                section class="section" id=(&heading.id) {
                    h2 {
                        a href=(format!("{}#{}", doc_href(&record.path), heading.id)) { (&heading.title) }
                    }
                    @if let Some(body) = plan::section(&record.body, &heading.title, heading.level) {
                        div class="prose" { (markdown(repository, &record.path, &body)) }
                    }
                }
            }
        }
    }
}

fn authority_items(repository: &Repository) -> Vec<PanelItem> {
    ["AGENTS.md", "dev/docs/README.md", "dev/CONTEXT.md"]
        .into_iter()
        .filter_map(|path| repository.get(path))
        .map(|record| {
            PanelItem::link(&record.title, doc_href(&record.path)).with_note(&record.path)
        })
        .collect()
}

fn source_items<'a>(
    repository: &Repository,
    path: impl Iterator<Item = &'a str>,
) -> Vec<PanelItem> {
    let mut seen = BTreeSet::new();
    path.filter(|path| seen.insert((*path).to_owned()))
        .filter_map(|path| repository.get(path))
        .map(|record| {
            PanelItem::link(&record.title, doc_href(&record.path))
                .with_note(super::home_label(record.home_id()))
        })
        .collect()
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

/// Render a record fragment with raw HTML escaped and governed links rewritten
/// to the shared Studio reader. The fragment never invents headings or content.
fn markdown(repository: &Repository, source_path: &str, source: &str) -> Markup {
    let mut event = Vec::new();
    let mut disabled_link = false;
    for item in Parser::new_ext(source, Options::all()) {
        match item {
            Event::Html(raw) | Event::InlineHtml(raw) => event.push(Event::Text(raw)),
            Event::Start(Tag::Link { dest_url, .. }) => {
                match resolve_link(source_path, &dest_url) {
                    LinkTarget::External => event.push(Event::Html(
                        format!(
                            "<a href=\"{}\" rel=\"noopener\">",
                            escape_attribute(&dest_url)
                        )
                        .into(),
                    )),
                    LinkTarget::SameFile { anchor } => {
                        let href = if anchor.is_empty() {
                            doc_href(source_path)
                        } else {
                            format!("{}#{anchor}", doc_href(source_path))
                        };
                        event.push(Event::Html(
                            format!("<a href=\"{}\">", escape_attribute(&href)).into(),
                        ));
                    }
                    LinkTarget::Repository { path, anchor } if repository.get(&path).is_some() => {
                        let href = match anchor {
                            Some(anchor) => format!("{}#{anchor}", doc_href(&path)),
                            None => doc_href(&path),
                        };
                        event.push(Event::Html(
                            format!("<a href=\"{}\">", escape_attribute(&href)).into(),
                        ));
                    }
                    LinkTarget::Repository { .. } => {
                        disabled_link = true;
                        event.push(Event::Html("<span class=\"mute\">".into()));
                    }
                }
            }
            Event::End(TagEnd::Link) => {
                event.push(Event::Html(
                    if disabled_link { "</span>" } else { "</a>" }.into(),
                ));
                disabled_link = false;
            }
            other => event.push(other),
        }
    }
    let mut output = String::new();
    pulldown_cmark::html::push_html(&mut output, event.into_iter());
    PreEscaped(output)
}

fn escape_attribute(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
