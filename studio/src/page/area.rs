//! Development Areas: current subject syntheses, never a second Work queue.

use std::collections::{BTreeMap, BTreeSet};

use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode, Uri},
    response::Response,
};
use maud::{Markup, PreEscaped, html};

use super::{
    Action, Context, Crumb, Page, Panel, PanelItem, Plate, PlateRow, Reference, Seal, Section,
    Tone, doc_href,
};
use crate::{
    StudioState, home,
    record::{LinkTarget, Record, Repository, heading_id, render_markdown},
};

const AREAS_PATH: &str = "/dev/areas";

/// `/dev/areas` — every conventionally discovered Development Area.
pub(crate) async fn index(
    State(state): State<StudioState>,
    header: HeaderMap,
    uri: Uri,
) -> Response {
    let context = match Context::build(&state, &header, &uri).await {
        Ok(context) => context,
        Err(response) => return response,
    };
    let areas = context.repository().in_home("development-area");
    let mut workbooks = Vec::new();
    for record in areas {
        let workbook = match AreaWorkbook::parse(&record.body) {
            Ok(workbook) => workbook,
            Err(error) => return invalid_workbook(&context, record, &error),
        };
        workbooks.push((record, workbook));
    }
    let page = area_page(&context, "Development Areas", AREAS_PATH)
        .with_lede(
            "The current development truth for six overlapping subjects: boundaries, decisions, unresolved landscape, concepts and technical direction.",
        )
        .with_plate(
            Plate::new(vec![PlateRow::text(
                "Areas",
                super::count(workbooks.len(), "area", "areas"),
            )])
            .with_authority(
                "Areas own durable current synthesis. Work alone owns which question is selected, its plan and execution state.",
            ),
        )
        .with_content(area_index(&workbooks));

    context.render(page)
}

/// `/dev/areas/{area}` — one source-backed subject overview.
pub(crate) async fn detail(
    State(state): State<StudioState>,
    header: HeaderMap,
    uri: Uri,
    Path(area): Path<String>,
) -> Response {
    let context = match Context::build(&state, &header, &uri).await {
        Ok(context) => context,
        Err(response) => return response,
    };
    let path = format!("dev/areas/{area}/README.md");
    let Some(record) = context
        .repository()
        .get(&path)
        .filter(|record| record.home_id() == "development-area")
    else {
        return not_found(&context, &area);
    };
    let records = area_records(context.repository(), &area);
    let workbook = match AreaWorkbook::parse(&record.body) {
        Ok(workbook) => workbook,
        Err(error) => return invalid_workbook(&context, record, &error),
    };
    let route = format!("{AREAS_PATH}/{area}");
    let mut page = Page::new(Section::Development, &record.title)
        .with_document_title(format!("{} · Development · Aicadia Studio", record.title))
        .with_crumb(vec![
            Crumb::link("Development", "/dev"),
            Crumb::link("Areas", AREAS_PATH),
            Crumb::here(&record.title),
        ])
        .with_seal(vec![Seal::plain("Development Area")])
        .with_lede(first_paragraph(&workbook.meaning))
        .with_plate(
            Plate::new(vec![
                PlateRow::text(
                    "Decided",
                    super::count(
                        workbook.chosen.item_count + workbook.rejected.item_count,
                        "statement",
                        "statements",
                    ),
                ),
                PlateRow::text(
                    "Open landscape",
                    super::count(
                        workbook.not_yet_chosen.item_count
                            + workbook.research_needed.item_count,
                        "statement",
                        "statements",
                    ),
                ),
                PlateRow::text(
                    "Prepared material",
                    super::count(records.len(), "record", "records"),
                ),
                PlateRow::fact("Source", &record.path),
            ])
            .with_action(vec![Action::link("Read area source", doc_href(&record.path))])
            .with_authority(
                "This source owns current Area synthesis; exact behavior stays in game/docs and Work owns selection, priority and execution.",
            ),
        )
        .with_content(workbook_content(
            context.repository(),
            record,
            &workbook,
            &area,
            &records,
        ))
        .with_reference(Reference {
            title: record.title.clone(),
            url: context.url(&route),
            context: record.path.clone(),
        });
    let related = area_related(record, context.repository(), &area);
    if !related.is_empty() {
        page = page.with_related(vec![Panel::list("Owning sources", related)]);
    }
    context.render(page)
}

/// `/dev/areas/{area}/scenarios` — scan-first projection of one scenario catalogue.
pub(crate) async fn scenarios(
    State(state): State<StudioState>,
    header: HeaderMap,
    uri: Uri,
    Path(area): Path<String>,
) -> Response {
    let context = match Context::build(&state, &header, &uri).await {
        Ok(context) => context,
        Err(response) => return response,
    };
    let area_path = format!("dev/areas/{area}/README.md");
    let scenario_path = format!("dev/areas/{area}/scenarios.md");
    let Some(area_record) = context
        .repository()
        .get(&area_path)
        .filter(|record| record.home_id() == "development-area")
    else {
        return not_found(&context, &area);
    };
    let Some(record) = context
        .repository()
        .get(&scenario_path)
        .filter(|record| record.home_id() == "area-record")
    else {
        return not_found(&context, &format!("{area}/scenarios"));
    };
    let catalogue = match ScenarioCatalogue::parse(&record.body) {
        Ok(catalogue) => catalogue,
        Err(error) => return invalid_catalogue(&context, record, &error),
    };
    let route = format!("{AREAS_PATH}/{area}/scenarios");
    let page_title = format!("{} scenarios", area_record.title);
    let page = Page::new(Section::Development, &page_title)
        .with_document_title(format!(
            "{page_title} · Development · Aicadia Studio"
        ))
        .with_crumb(vec![
            Crumb::link("Development", "/dev"),
            Crumb::link("Areas", AREAS_PATH),
            Crumb::link(&area_record.title, format!("{AREAS_PATH}/{area}")),
            Crumb::here("Scenarios"),
        ])
        .with_seal(vec![Seal::plain("Area overview")])
        .with_lede(
            "Hard cases that this development area must be able to reason about consistently.",
        )
        .with_plate(
            Plate::new(vec![
                PlateRow::text(
                    "Scenarios",
                    super::count(catalogue.scenario.len(), "scenario", "scenarios"),
                ),
                PlateRow::fact("Source", &record.path),
            ])
            .with_action(vec![Action::link("Read complete source", doc_href(&record.path))])
            .with_authority(
                "Development pressure, not current game contract. A scenario fixes a hard case without choosing its outcome.",
            ),
        )
        .with_content(scenario_content(&catalogue))
        .with_related(vec![Panel::list(
            "Area",
            vec![
                PanelItem::link(&area_record.title, format!("{AREAS_PATH}/{area}")),
                PanelItem::link("Catalogue source", doc_href(&record.path)),
            ],
        )])
        .with_reference(Reference {
            title: page_title,
            url: context.url(&route),
            context: record.path.clone(),
        });
    context.render(page)
}

fn area_page(context: &Context, title: &str, path: &str) -> Page {
    Page::new(Section::Development, title)
        .with_document_title(format!("{title} · Development · Aicadia Studio"))
        .with_crumb(vec![Crumb::link("Development", "/dev"), Crumb::here(title)])
        .with_reference(Reference {
            title: title.to_owned(),
            url: context.url(path),
            context: "projection of governed Development Area records".to_owned(),
        })
}

fn area_index(areas: &[(&Record, AreaWorkbook)]) -> Markup {
    html! {
        section class="section" data-area-count=(areas.len()) {
            h2 { "Browse areas" }
            ul class="area-list" role="list" {
                @for (record, workbook) in areas {
                    @let id = area_id(record);
                    li data-area=(id)
                        data-chosen-count=(workbook.chosen.item_count)
                        data-rejected-count=(workbook.rejected.item_count)
                        data-not-yet-chosen-count=(workbook.not_yet_chosen.item_count)
                        data-research-needed-count=(workbook.research_needed.item_count) {
                        a href=(format!("{AREAS_PATH}/{id}")) { (&record.title) }
                        p { (first_paragraph(&workbook.meaning)) }
                        dl class="area-summary" aria-label=(format!("{} current-truth summary", record.title)) {
                            div { dt { "Chosen" } dd { (workbook.chosen.item_count) } }
                            div { dt { "Rejected" } dd { (workbook.rejected.item_count) } }
                            div { dt { "Not yet chosen" } dd { (workbook.not_yet_chosen.item_count) } }
                            div { dt { "Research needed" } dd { (workbook.research_needed.item_count) } }
                        }
                    }
                }
            }
        }
    }
}

fn workbook_content(
    repository: &Repository,
    record: &Record,
    workbook: &AreaWorkbook,
    area: &str,
    records: &[&Record],
) -> Markup {
    html! {
        div class="area-workbook"
            data-chosen-count=(workbook.chosen.item_count)
            data-rejected-count=(workbook.rejected.item_count)
            data-not-yet-chosen-count=(workbook.not_yet_chosen.item_count)
            data-research-needed-count=(workbook.research_needed.item_count)
            data-area-record-count=(records.len()) {
            section class="section area-meaning" {
                h2 { "Meaning" }
                div class="prose" { (source_markdown(repository, &record.path, &workbook.meaning)) }
            }
            section class="section" {
                h2 { "Boundary" }
                div class="area-boundary" {
                    article class="boundary boundary-is" {
                        h3 { "This is" }
                        div class="prose" { (source_markdown(repository, &record.path, &workbook.is_scope.markdown)) }
                    }
                    article class="boundary boundary-is-not" {
                        h3 { "This is not" }
                        div class="prose" { (source_markdown(repository, &record.path, &workbook.is_not_scope.markdown)) }
                    }
                }
            }
            section class="section" {
                h2 { "Decisions and open landscape" }
                div class="area-state-grid" {
                    article class="area-state area-state-chosen" {
                        h3 { "Chosen" span class="count" { (super::count(workbook.chosen.item_count, "statement", "statements")) } }
                        div class="prose" { (source_markdown(repository, &record.path, &workbook.chosen.markdown)) }
                    }
                    article class="area-state area-state-rejected" {
                        h3 { "Rejected" span class="count" { (super::count(workbook.rejected.item_count, "statement", "statements")) } }
                        div class="prose" { (source_markdown(repository, &record.path, &workbook.rejected.markdown)) }
                    }
                    article class="area-state area-state-open" {
                        h3 { "Not yet chosen" span class="count" { (super::count(workbook.not_yet_chosen.item_count, "statement", "statements")) } }
                        div class="prose" { (source_markdown(repository, &record.path, &workbook.not_yet_chosen.markdown)) }
                    }
                    article class="area-state area-state-research" {
                        h3 { "Research needed" span class="count" { (super::count(workbook.research_needed.item_count, "statement", "statements")) } }
                        div class="prose" { (source_markdown(repository, &record.path, &workbook.research_needed.markdown)) }
                    }
                }
            }
            section class="section" {
                h2 { "Components and concepts" }
                div class="prose area-components" { (source_markdown(repository, &record.path, &workbook.components)) }
            }
            section class="section" {
                h2 { "Technical model" }
                div class="area-model" {
                    section class="model-layer model-delivered" {
                        h3 { "Delivered" }
                        div class="prose" { (source_markdown(repository, &record.path, &workbook.delivered_model)) }
                    }
                    section class="model-layer model-directional" {
                        h3 { "Directional" }
                        div class="prose" { (source_markdown(repository, &record.path, &workbook.directional_model)) }
                    }
                    section class="model-layer model-absent" {
                        h3 { "Absent" }
                        div class="prose" { (source_markdown(repository, &record.path, &workbook.absent_model)) }
                    }
                }
            }
            section class="section" {
                h2 { "Sources" }
                div class="prose" { (source_markdown(repository, &record.path, &workbook.sources)) }
            }
            @if !records.is_empty() {
                section class="section" {
                    h2 { "Prepared material" }
                    ul class="area-list" role="list" {
                        @for prepared in records {
                            @let scenario_count = scenario_heading_count(prepared);
                            li {
                                a href=(area_record_href(area, prepared)) { (&prepared.title) }
                                p {
                                    @if scenario_count > 0 {
                                        (super::count(scenario_count, "scenario", "scenarios"))
                                        " · "
                                    }
                                    (prepared.role_header.as_ref().map_or("Prepared Area record", |role| role.role.as_str()))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn area_related(record: &Record, repository: &Repository, area: &str) -> Vec<PanelItem> {
    let own_prefix = format!("dev/areas/{area}/");
    let mut seen = BTreeSet::new();
    let mut item = Vec::new();
    for link in &record.link {
        let LinkTarget::Repository { path, .. } = &link.resolved else {
            continue;
        };
        if path.starts_with(&own_prefix) || !seen.insert(path.clone()) {
            continue;
        }
        let Some(target) = repository.get(path) else {
            continue;
        };
        item.push(
            PanelItem::link(&link.text, doc_href(&target.path))
                .with_note(super::home_label(target.home_id())),
        );
    }
    item
}

fn area_records<'a>(repository: &'a Repository, area: &str) -> Vec<&'a Record> {
    let prefix = format!("dev/areas/{area}/");
    repository
        .in_home("area-record")
        .into_iter()
        .filter(|record| record.path.starts_with(&prefix))
        .collect()
}

fn area_id(record: &Record) -> &str {
    home::directory(&record.path)
        .rsplit('/')
        .next()
        .unwrap_or("")
}

fn area_record_href(area: &str, record: &Record) -> String {
    if home::file_name(&record.path) == "scenarios.md" {
        format!("{AREAS_PATH}/{area}/scenarios")
    } else {
        doc_href(&record.path)
    }
}

const AREA_WORKBOOK_SHAPE: [(u8, &str); 15] = [
    (2, "Meaning"),
    (2, "Boundary"),
    (3, "This is"),
    (3, "This is not"),
    (2, "Decisions"),
    (3, "Chosen"),
    (3, "Rejected"),
    (3, "Not yet chosen"),
    (2, "Research needed"),
    (2, "Components"),
    (2, "Technical model"),
    (3, "Delivered"),
    (3, "Directional"),
    (3, "Absent"),
    (2, "Sources"),
];

#[derive(Clone, Debug, PartialEq, Eq)]
struct WorkbookList {
    markdown: String,
    item_count: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct AreaWorkbook {
    meaning: String,
    is_scope: WorkbookList,
    is_not_scope: WorkbookList,
    chosen: WorkbookList,
    rejected: WorkbookList,
    not_yet_chosen: WorkbookList,
    research_needed: WorkbookList,
    components: String,
    delivered_model: String,
    directional_model: String,
    absent_model: String,
    sources: String,
}

impl AreaWorkbook {
    fn parse(body: &str) -> Result<Self, String> {
        let lines = body.lines().collect::<Vec<_>>();
        let actual = lines
            .iter()
            .filter_map(|line| workbook_heading(line))
            .filter(|(level, _)| (2..=3).contains(level))
            .collect::<Vec<_>>();

        for &(level, title) in &AREA_WORKBOOK_SHAPE {
            let count = actual
                .iter()
                .filter(|(actual_level, actual_title)| {
                    *actual_level == level && *actual_title == title
                })
                .count();
            match count {
                0 => {
                    return Err(format!(
                        "Area workbook lacks `{}`",
                        workbook_label(level, title)
                    ));
                }
                1 => {}
                _ => {
                    return Err(format!(
                        "Area workbook repeats `{}`",
                        workbook_label(level, title)
                    ));
                }
            }
        }

        if let Some((level, title)) = actual
            .iter()
            .find(|heading| !AREA_WORKBOOK_SHAPE.contains(heading))
        {
            return Err(format!(
                "Area workbook has unexpected `{}`",
                workbook_label(*level, title)
            ));
        }
        if actual != AREA_WORKBOOK_SHAPE {
            return Err("Area workbook headings are out of order".to_owned());
        }

        let section = |level, title| workbook_section(&lines, level, title);
        Ok(Self {
            meaning: required_workbook_prose(section(2, "Meaning"), "Meaning")?,
            is_scope: workbook_list(section(3, "This is"), "This is")?,
            is_not_scope: workbook_list(section(3, "This is not"), "This is not")?,
            chosen: workbook_list(section(3, "Chosen"), "Chosen")?,
            rejected: workbook_list(section(3, "Rejected"), "Rejected")?,
            not_yet_chosen: workbook_list(section(3, "Not yet chosen"), "Not yet chosen")?,
            research_needed: workbook_list(section(2, "Research needed"), "Research needed")?,
            components: required_workbook_prose(section(2, "Components"), "Components")?,
            delivered_model: required_workbook_prose(section(3, "Delivered"), "Delivered")?,
            directional_model: required_workbook_prose(section(3, "Directional"), "Directional")?,
            absent_model: required_workbook_prose(section(3, "Absent"), "Absent")?,
            sources: required_workbook_prose(section(2, "Sources"), "Sources")?,
        })
    }
}

fn workbook_heading(line: &str) -> Option<(u8, &str)> {
    let level = line
        .chars()
        .take_while(|character| *character == '#')
        .count();
    if !(1..=6).contains(&level) || line.as_bytes().get(level) != Some(&b' ') {
        return None;
    }
    Some((u8::try_from(level).ok()?, line[level + 1..].trim()))
}

fn workbook_label(level: u8, title: &str) -> String {
    format!("{} {title}", "#".repeat(usize::from(level)))
}

fn workbook_section(lines: &[&str], level: u8, title: &str) -> String {
    let wanted = workbook_label(level, title);
    let Some(start) = lines.iter().position(|line| line.trim_end() == wanted) else {
        return String::new();
    };
    let end = lines[start + 1..]
        .iter()
        .position(|line| workbook_heading(line).is_some_and(|(next_level, _)| next_level <= level))
        .map_or(lines.len(), |offset| start + 1 + offset);
    lines[start + 1..end].join("\n").trim().to_owned()
}

fn required_workbook_prose(markdown: String, title: &str) -> Result<String, String> {
    if markdown.is_empty() {
        Err(format!("Area workbook `{title}` is empty"))
    } else {
        Ok(markdown)
    }
}

fn workbook_list(markdown: String, title: &str) -> Result<WorkbookList, String> {
    let markdown = required_workbook_prose(markdown, title)?;
    let item_count = markdown
        .lines()
        .filter(|line| line.trim_start().starts_with("- "))
        .count();
    if item_count == 0 && markdown.trim() != "None." {
        return Err(format!(
            "Area workbook `{title}` must use bullet items or `None.`"
        ));
    }
    Ok(WorkbookList {
        markdown,
        item_count,
    })
}

fn first_paragraph(body: &str) -> String {
    let mut paragraph = Vec::new();
    let mut collecting = false;
    for line in body.lines() {
        let trimmed = line.trim();
        if collecting && trimmed.is_empty() {
            break;
        }
        if !collecting {
            if trimmed.is_empty()
                || trimmed.starts_with('#')
                || trimmed.starts_with('>')
                || trimmed.starts_with("---")
            {
                continue;
            }
            collecting = true;
        }
        paragraph.push(trimmed);
    }
    paragraph.join(" ")
}

fn scenario_heading_count(record: &Record) -> usize {
    record
        .heading
        .iter()
        .filter(|heading| scenario_heading(&heading.title).is_some())
        .count()
}

fn scenario_content(catalogue: &ScenarioCatalogue) -> Markup {
    html! {
        div data-scenario-count=(catalogue.scenario.len()) {
            details class="secondary-disclosure scenario-method" {
                summary { "How these scenarios are used" }
                div class="prose" { (plain_markdown(&catalogue.method)) }
            }
            section class="section" {
                h2 { "Scenario overview" }
                div class="scenario-grid" {
                    @for scenario in &catalogue.scenario {
                        article class="scenario-card" id=(&scenario.anchor) {
                            details {
                                summary {
                                    span class="scenario-id" { (&scenario.id) }
                                    span class="scenario-name" {
                                        strong { (&scenario.title) }
                                        small { (&scenario.pressure) }
                                    }
                                }
                                div class="prose scenario-detail" { (plain_markdown(&scenario.body)) }
                            }
                        }
                    }
                }
            }
            details class="secondary-disclosure scenario-coverage" {
                summary { "Coverage matrix for future experiments" }
                div class="prose" { (plain_markdown(&catalogue.coverage)) }
            }
        }
    }
}

fn source_markdown(repository: &Repository, source_path: &str, source: &str) -> Markup {
    super::development::markdown(repository, source_path, source)
}

fn plain_markdown(source: &str) -> Markup {
    PreEscaped(render_markdown(source))
}

fn not_found(context: &Context, requested: &str) -> Response {
    let page = area_page(context, "Development Area not found", AREAS_PATH)
        .with_seal(vec![Seal::toned("Not found", Tone::Brick)])
        .with_plate(Plate::new(vec![PlateRow::fact("Requested", requested)]))
        .with_content(super::empty_state(
            "No governed Development Area record matches this path.",
        ));
    context.render_status(page, StatusCode::NOT_FOUND)
}

fn invalid_catalogue(context: &Context, record: &Record, error: &str) -> Response {
    let page = area_page(context, "Scenario catalogue is invalid", AREAS_PATH)
        .with_seal(vec![Seal::toned("Invalid source", Tone::Brick)])
        .with_plate(Plate::new(vec![PlateRow::fact("Source", &record.path)]))
        .with_content(super::note(Tone::Brick, error));
    context.render_status(page, StatusCode::INTERNAL_SERVER_ERROR)
}

fn invalid_workbook(context: &Context, record: &Record, error: &str) -> Response {
    let page = area_page(context, "Development Area is invalid", AREAS_PATH)
        .with_seal(vec![Seal::toned("Invalid source", Tone::Brick)])
        .with_plate(Plate::new(vec![PlateRow::fact("Source", &record.path)]))
        .with_content(super::note(Tone::Brick, error));
    context.render_status(page, StatusCode::INTERNAL_SERVER_ERROR)
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Scenario {
    id: String,
    title: String,
    pressure: String,
    anchor: String,
    body: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ScenarioCatalogue {
    method: String,
    scenario: Vec<Scenario>,
    coverage: String,
}

impl ScenarioCatalogue {
    fn parse(body: &str) -> Result<Self, String> {
        let lines = body.lines().collect::<Vec<_>>();
        let method_start = position(&lines, "## How to use a scenario")?;
        let index_start = position(&lines, "## Scenario index")?;
        let coverage_start = position(&lines, "## Coverage matrix for future experiments")?;
        if !(method_start < index_start && index_start < coverage_start) {
            return Err("scenario catalogue sections are out of order".to_owned());
        }

        let mut index = Vec::new();
        let mut seen = BTreeSet::new();
        for line in &lines[index_start + 1..coverage_start] {
            let Some((id, title, pressure)) = scenario_index_row(line) else {
                continue;
            };
            if !seen.insert(id.clone()) {
                return Err(format!("scenario index repeats {id}"));
            }
            index.push((id, title, pressure));
        }
        if index.is_empty() {
            return Err("scenario index contains no Sxx rows".to_owned());
        }

        let mut detail = BTreeMap::new();
        for (line_index, line) in lines.iter().enumerate() {
            let Some((id, title)) = line.strip_prefix("## ").and_then(scenario_heading) else {
                continue;
            };
            let end = lines[line_index + 1..]
                .iter()
                .position(|line| line.starts_with("## "))
                .map_or(lines.len(), |offset| line_index + 1 + offset);
            if detail
                .insert(
                    id.clone(),
                    (
                        title,
                        lines[line_index + 1..end].join("\n").trim().to_owned(),
                    ),
                )
                .is_some()
            {
                return Err(format!("scenario detail repeats {id}"));
            }
        }

        let mut scenario = Vec::new();
        for (id, title, pressure) in index {
            let Some((detail_title, body)) = detail.remove(&id) else {
                return Err(format!("scenario index {id} has no detail section"));
            };
            if detail_title != title {
                return Err(format!(
                    "scenario {id} title differs between index and detail"
                ));
            }
            let heading = format!("{id} — {title}");
            scenario.push(Scenario {
                id,
                title,
                pressure,
                anchor: heading_id(&heading),
                body,
            });
        }
        if let Some(extra) = detail.keys().next() {
            return Err(format!("scenario detail {extra} is absent from the index"));
        }

        Ok(Self {
            method: lines[method_start..index_start]
                .join("\n")
                .trim()
                .to_owned(),
            scenario,
            coverage: lines[coverage_start..].join("\n").trim().to_owned(),
        })
    }
}

fn position(lines: &[&str], heading: &str) -> Result<usize, String> {
    lines
        .iter()
        .position(|line| *line == heading)
        .ok_or_else(|| format!("scenario catalogue lacks `{heading}`"))
}

fn scenario_index_row(line: &str) -> Option<(String, String, String)> {
    let cell = line
        .trim()
        .trim_matches('|')
        .split('|')
        .map(str::trim)
        .collect::<Vec<_>>();
    if cell.len() != 3 || !scenario_id(cell[0]) {
        return None;
    }
    Some((cell[0].to_owned(), cell[1].to_owned(), cell[2].to_owned()))
}

fn scenario_heading(heading: &str) -> Option<(String, String)> {
    let (id, title) = heading.split_once(" — ")?;
    scenario_id(id).then(|| (id.to_owned(), title.to_owned()))
}

fn scenario_id(id: &str) -> bool {
    id.len() == 3 && id.starts_with('S') && id.as_bytes()[1..].iter().all(u8::is_ascii_digit)
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use axum::{body::to_bytes, extract::State, http::HeaderMap};

    use super::*;

    #[test]
    fn the_real_catalogue_joins_fourteen_index_rows_to_fourteen_details() {
        let source = include_str!("../../../dev/areas/multiplayer/scenarios.md");
        let catalogue = ScenarioCatalogue::parse(source).expect("real catalogue should parse");

        assert_eq!(catalogue.scenario.len(), 14);
        assert_eq!(catalogue.scenario.first().unwrap().id, "S01");
        assert_eq!(catalogue.scenario.last().unwrap().id, "S14");
        assert!(catalogue.method.contains("Observation layers"));
        assert!(catalogue.coverage.contains("| S14 |"));
    }

    #[test]
    fn a_mismatched_index_and_detail_fails_closed() {
        let source = "## How to use a scenario\nUse it.\n\n## Scenario index\n| ID | Scenario | Primary pressure |\n| --- | --- | --- |\n| S01 | Indexed title | Pressure |\n\n## S01 — Different title\nDetail.\n\n## Coverage matrix for future experiments\nNone.";

        assert_eq!(
            ScenarioCatalogue::parse(source),
            Err("scenario S01 title differs between index and detail".to_owned())
        );
    }

    const WORKBOOK: &str = "# Place\n\n> **Role / side:** Area / development side.\n> **Authority:** current synthesis.\n\n## Meaning\n\nPlace concerns durable location.\n\n## Boundary\n\n### This is\n\n- Stable Place identity.\n\n### This is not\n\n- A database partition.\n\n## Decisions\n\n### Chosen\n\n- Place is an Entity role.\n\n### Rejected\n\nNone.\n\n### Not yet chosen\n\n- Boundary geometry.\n\n## Research needed\n\n- Hot Place admission.\n\n## Components\n\n| Component | Meaning |\n| --- | --- |\n| Placement | Current direct Place. |\n\n## Technical model\n\n### Delivered\n\nExact Place placement is delivered.\n\n### Directional\n\nTyped topology is directional.\n\n### Absent\n\nGeometry is absent.\n\n## Sources\n\n- Runtime behavior — defined in `game/docs/`.\n";

    #[test]
    fn an_area_workbook_preserves_every_current_truth_distinction() {
        let workbook = AreaWorkbook::parse(WORKBOOK).expect("workbook should parse");

        assert_eq!(workbook.meaning, "Place concerns durable location.");
        assert_eq!(workbook.is_scope.item_count, 1);
        assert_eq!(workbook.is_not_scope.item_count, 1);
        assert_eq!(workbook.chosen.item_count, 1);
        assert_eq!(workbook.rejected.item_count, 0);
        assert_eq!(workbook.not_yet_chosen.item_count, 1);
        assert_eq!(workbook.research_needed.item_count, 1);
        assert!(workbook.components.contains("Placement"));
        assert!(workbook.delivered_model.contains("delivered"));
        assert!(workbook.directional_model.contains("directional"));
        assert!(workbook.absent_model.contains("absent"));
        assert!(workbook.sources.contains("game/docs"));
    }

    #[test]
    fn an_area_workbook_fails_closed_on_shape_and_state_ambiguity() {
        let missing = WORKBOOK.replace("## Sources", "## References");
        assert_eq!(
            AreaWorkbook::parse(&missing),
            Err("Area workbook lacks `## Sources`".to_owned())
        );

        let repeated = WORKBOOK.replace("## Sources", "## Sources\n\nOne.\n\n## Sources");
        assert_eq!(
            AreaWorkbook::parse(&repeated),
            Err("Area workbook repeats `## Sources`".to_owned())
        );

        let prose_state = WORKBOOK.replace(
            "- Boundary geometry.",
            "Boundary geometry remains undecided.",
        );
        assert_eq!(
            AreaWorkbook::parse(&prose_state),
            Err("Area workbook `Not yet chosen` must use bullet items or `None.`".to_owned())
        );
    }

    #[test]
    fn all_six_real_area_workbooks_follow_the_current_truth_shape() {
        let sources = [
            (
                "Multiplayer",
                include_str!("../../../dev/areas/multiplayer/README.md"),
            ),
            ("Place", include_str!("../../../dev/areas/place/README.md")),
            (
                "Movement",
                include_str!("../../../dev/areas/movement/README.md"),
            ),
            (
                "Discovery",
                include_str!("../../../dev/areas/discovery/README.md"),
            ),
            (
                "Agent Play",
                include_str!("../../../dev/areas/agent-play/README.md"),
            ),
            (
                "World Change",
                include_str!("../../../dev/areas/world-change/README.md"),
            ),
        ];

        assert_eq!(sources.len(), 6);
        for (title, source) in sources {
            let workbook = AreaWorkbook::parse(source)
                .unwrap_or_else(|error| panic!("{title} workbook should parse: {error}"));
            assert!(
                !workbook.meaning.is_empty(),
                "{title} must explain its meaning"
            );
            assert!(
                workbook.is_scope.item_count > 0,
                "{title} must say what it is"
            );
            assert!(
                workbook.is_not_scope.item_count > 0,
                "{title} must say what it is not"
            );
            assert!(
                workbook.chosen.item_count > 0,
                "{title} must expose choices"
            );
            assert!(
                workbook.not_yet_chosen.item_count > 0,
                "{title} must expose unresolved choices"
            );
        }
    }

    #[tokio::test]
    async fn an_invalid_real_area_source_returns_an_honest_server_error() {
        let root = crate::test_directory("invalid-area-route");
        let path = root.join("dev/areas/broken/README.md");
        std::fs::create_dir_all(path.parent().expect("Area parent should exist"))
            .expect("Area directory should be created");
        std::fs::write(
            &path,
            "# Broken\n\n> **Role / side:** broken Area fixture / development side.\n> **Authority:** invalid fixture.\n\n## Meaning\n\nOnly one section.\n",
        )
        .expect("Area fixture should be written");
        let pool = sqlx::PgPool::connect_lazy("postgresql://127.0.0.1:1/unused")
            .expect("a lazy pool should parse");
        let state = crate::StudioState {
            world: crate::World::new(pool.clone()),
            pool,
            repository_root: Arc::new(root),
        };

        let response = index(
            State(state),
            HeaderMap::new(),
            Uri::from_static("/dev/areas"),
        )
        .await;
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
        let body = to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("error response body should read");
        let html = String::from_utf8(body.to_vec()).expect("response should be UTF-8");
        assert!(html.contains("Development Area is invalid"));
        assert!(html.contains("Area workbook lacks `## Boundary`"));
        assert!(html.contains("dev/areas/broken/README.md"));
    }
}
