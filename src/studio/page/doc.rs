//! `/doc/{*path}` — the record reader over every governed record.
//!
//! The record's own Markdown is rendered with the projection's heading ids, its
//! internal links are rewritten to Studio routes and everything around it (role
//! header, front matter, outline, backlinks) is projected, never authored.

use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode, Uri},
    response::Response,
};
use maud::{Markup, PreEscaped, html};
use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};

use super::{
    Action, Context, Crumb, OutlineItem, Page, Panel, PanelItem, Plate, PlateRow, Reference, Seal,
    Section, Tone, doc_href, home_label, section_for_home,
};
use crate::studio::{
    StudioState,
    record::{Kind, LinkTarget, Record, Repository, Value, resolve_link},
};

pub(in crate::studio) async fn document(
    State(state): State<StudioState>,
    header: HeaderMap,
    uri: Uri,
    Path(path): Path<String>,
) -> Response {
    let context = match Context::build(&state, &header, &uri).await {
        Ok(context) => context,
        Err(response) => return response,
    };
    let Some(record) = context.repository().get(&path) else {
        return not_found(&context, &path);
    };
    context.render(page(&context, record))
}

/// The 404 page: honest, inside the shell, never a dead end.
fn not_found(context: &Context, path: &str) -> Response {
    let page = Page::new(Section::Overview, "No governed record at this path")
        .with_document_title("Not found · Aicadia Studio")
        .with_crumb(vec![Crumb::link("Overview", "/"), Crumb::here("Not found")])
        .with_seal(vec![Seal::toned("Not found", Tone::Brick)])
        .with_lede("Studio reads only the governed roots of this repository.")
        .with_plate(Plate::new(vec![PlateRow::fact("Requested", path)]))
        .with_content(html! {
            (super::empty_state("No governed record matches this path."))
            ul class="list" {
                @for section in Section::ALL {
                    li { a href=(section.href()) { (section.label()) } }
                }
            }
        });

    context.render_status(page, StatusCode::NOT_FOUND)
}

/// One governed record as a Studio page.
fn page(context: &Context, record: &Record) -> Page {
    let repository = context.repository();
    let section = section_for_home(record.home_id());
    let home = home_label(record.home_id());
    let backlink = repository.backlink(&record.path);
    let url = context.url(&doc_href(&record.path));

    let mut plate_row = vec![
        PlateRow::text("Home", home),
        PlateRow::fact("Path", &record.path),
        PlateRow::text(
            "Referenced by",
            super::count(backlink.len(), "record", "records"),
        ),
    ];
    plate_row.extend(front_matter_row(record));

    let mut plate = Plate::new(plate_row).with_action(vec![
        Action::copy(
            "Copy reference",
            Reference {
                title: record.title.clone(),
                url: url.clone(),
                context: record.path.clone(),
            }
            .line(),
        ),
        Action::copy("Copy path", &record.path),
    ]);
    if let Some(role) = &record.role_header {
        plate = plate.with_authority(format!("Authority: {}", role.authority));
    }

    Page::new(section, &record.title)
        .with_document_title(format!(
            "{} · {} · Aicadia Studio",
            record.title,
            section.label()
        ))
        .with_crumb(vec![
            Crumb::link(section.label(), section.href()),
            Crumb::here(home),
            Crumb::here(&record.title),
        ])
        .with_seal(seal(record))
        .with_plate(plate)
        .with_content(content(record, repository))
        .with_related(related(record, repository))
        .with_reference(Reference {
            title: record.title.clone(),
            url,
            context: record.path.clone(),
        })
}

/// Status, verdict, side and frozen state, each from the record's own metadata.
fn seal(record: &Record) -> Vec<Seal> {
    let mut seal = Vec::new();
    for key in ["status", "verdict"] {
        if let Some(value) = record.front_matter.scalar(key) {
            seal.push(Seal::status(value));
        }
    }
    if let Some(kind) = record.front_matter.scalar("kind") {
        seal.push(Seal::plain(kind));
    }
    if let Some(role) = &record.role_header
        && !role.side.is_empty()
    {
        seal.push(Seal::plain(format!("{} side", role.side)));
    }
    if record.metadata_frozen || record.link_frozen {
        seal.push(Seal::toned("Frozen", Tone::Slate));
    }
    seal
}

/// The front-matter keys that belong on the plate rather than in the panel.
fn front_matter_row(record: &Record) -> Vec<PlateRow> {
    let mut row = Vec::new();
    if let Some(table) = record.front_matter.list("storage_table")
        && !table.is_empty()
    {
        row.push(PlateRow::fact("Tables", table.join(" · ")));
    }
    if let Some(updated) = record.front_matter.scalar("updated_at") {
        row.push(PlateRow::text("Updated", updated));
    }
    row
}

fn content(record: &Record, repository: &Repository) -> Markup {
    html! {
        @if let Some(error) = &record.front_matter_error {
            (super::note(
                Tone::Brick,
                &format!("Front matter line {}: {}", error.line, error.message),
            ))
        }
        article class="prose" { (body(record, repository)) }
    }
}

/// The record body: Markdown with projection heading ids and rewritten links,
/// or one SQL migration as a code block.
fn body(record: &Record, repository: &Repository) -> Markup {
    if record.kind == Kind::Sql {
        return PreEscaped(record.html());
    }

    let mut heading = record.heading.iter();
    let mut open: Option<(u8, String)> = None;
    let mut disabled = false;
    // The role header has one home on the page: the plate and its own panel.
    let mut role_header = record.role_header.is_some();
    let mut skipping = false;
    let mut event = Vec::new();

    for raw in Parser::new_ext(&record.body, Options::all()) {
        if skipping {
            if matches!(raw, Event::End(TagEnd::BlockQuote(_))) {
                skipping = false;
            }
            continue;
        }
        match raw {
            Event::Start(Tag::BlockQuote(_)) if role_header => {
                role_header = false;
                skipping = true;
            }
            // Raw HTML in a governed record is shown as text, never executed.
            Event::Html(text) | Event::InlineHtml(text) => event.push(Event::Text(text)),
            Event::Start(Tag::Heading { level, .. }) => {
                let level = level as u8;
                let id = heading
                    .next()
                    .map_or_else(String::new, |heading| heading.id.clone());
                event.push(Event::Html(
                    if id.is_empty() {
                        format!("<h{level}>")
                    } else {
                        format!("<h{level} id=\"{}\">", attribute(&id))
                    }
                    .into(),
                ));
                open = Some((level, id));
            }
            Event::End(TagEnd::Heading(_)) => {
                if let Some((level, id)) = open.take() {
                    if (2..=3).contains(&level) && !id.is_empty() {
                        event.push(Event::Html(
                            format!(
                                " <a class=\"anchor\" href=\"#{0}\" aria-label=\"Link to this heading\">#</a>",
                                attribute(&id)
                            )
                            .into(),
                        ));
                    }
                    event.push(Event::Html(format!("</h{level}>").into()));
                }
            }
            Event::Start(Tag::Link { dest_url, .. }) => {
                match rewrite(record, repository, &dest_url) {
                    Route::Studio(href) => {
                        event.push(Event::Html(
                            format!("<a href=\"{}\">", attribute(&href)).into(),
                        ));
                    }
                    Route::External(href) => event.push(Event::Html(
                        format!("<a href=\"{}\" rel=\"noopener\">", attribute(&href)).into(),
                    )),
                    Route::Outside => {
                        disabled = true;
                        event.push(Event::Html(
                            "<span class=\"mute\" title=\"outside Studio\">".into(),
                        ));
                    }
                }
            }
            Event::End(TagEnd::Link) => {
                event.push(Event::Html(
                    if disabled { "</span>" } else { "</a>" }.into(),
                ));
                disabled = false;
            }
            other => event.push(other),
        }
    }

    let mut html = String::new();
    pulldown_cmark::html::push_html(&mut html, event.into_iter());
    PreEscaped(html)
}

/// Where one Markdown link points inside Studio.
enum Route {
    Studio(String),
    External(String),
    /// A repository file that Studio does not govern; shown as plain text.
    Outside,
}

fn rewrite(record: &Record, repository: &Repository, target: &str) -> Route {
    match resolve_link(&record.path, target) {
        LinkTarget::External => Route::External(target.to_owned()),
        LinkTarget::SameFile { anchor } if anchor.is_empty() => {
            Route::Studio(doc_href(&record.path))
        }
        LinkTarget::SameFile { anchor } => Route::Studio(format!("#{anchor}")),
        LinkTarget::Repository { path, anchor } => {
            if repository.get(&path).is_none() {
                return Route::Outside;
            }
            Route::Studio(match anchor {
                Some(anchor) => format!("{}#{anchor}", doc_href(&path)),
                None => doc_href(&path),
            })
        }
    }
}

fn related(record: &Record, repository: &Repository) -> Vec<Panel> {
    let mut panel = Vec::new();

    if let Some(role) = &record.role_header {
        let mut item = vec![("Role".to_owned(), role.role.clone())];
        if !role.side.is_empty() {
            item.push(("Side".to_owned(), role.side.clone()));
        }
        if let Some(excludes) = &role.excludes {
            item.push(("Excludes".to_owned(), excludes.clone()));
        }
        panel.push(Panel::definition("Role header", item));
    }

    if !record.front_matter.is_empty() {
        panel.push(Panel::definition(
            "Front matter",
            record
                .front_matter
                .field()
                .iter()
                .map(|field| (field.key.clone(), scalar(&field.value)))
                .collect(),
        ));
    }

    let outline: Vec<OutlineItem> = record
        .heading
        .iter()
        .filter(|heading| (2..=3).contains(&heading.level))
        .map(|heading| OutlineItem {
            id: heading.id.clone(),
            title: heading.title.clone(),
            level: heading.level,
        })
        .collect();
    if !outline.is_empty() {
        panel.push(Panel::outline("On this page", outline));
    }

    panel.push(Panel::list(
        "Referenced by",
        referenced_by(record, repository),
    ));

    let links_to = links_to(record, repository);
    if !links_to.is_empty() {
        panel.push(Panel::list("Links to", links_to));
    }

    panel
}

/// Every governed record that links here, ordered by owning home then title.
fn referenced_by(record: &Record, repository: &Repository) -> Vec<PanelItem> {
    let mut item: Vec<(&str, &str, String)> = Vec::new();
    for backlink in repository.backlink(&record.path) {
        let Some(source) = repository.get(&backlink.path) else {
            continue;
        };
        let home = home_label(source.home_id());
        if item.iter().any(|(_, title, _)| *title == source.title) {
            continue;
        }
        item.push((home, &source.title, doc_href(&source.path)));
    }
    item.sort_by(|left, right| left.0.cmp(right.0).then(left.1.cmp(right.1)));
    item.into_iter()
        .map(|(home, title, href)| PanelItem::link(title, href).with_note(home))
        .collect()
}

/// Every governed record this record links to, once per target.
fn links_to(record: &Record, repository: &Repository) -> Vec<PanelItem> {
    let mut path: Vec<&str> = Vec::new();
    for link in &record.link {
        let LinkTarget::Repository { path: target, .. } = &link.resolved else {
            continue;
        };
        if target == &record.path || path.contains(&target.as_str()) {
            continue;
        }
        if repository.get(target).is_some() {
            path.push(target);
        }
    }
    path.sort_unstable();
    path.into_iter()
        .filter_map(|path| repository.get(path))
        .map(|target| {
            PanelItem::link(&target.title, doc_href(&target.path))
                .with_note(home_label(target.home_id()))
        })
        .collect()
}

fn scalar(value: &Value) -> String {
    match value {
        Value::Scalar(text) => text.clone(),
        Value::List(item) => {
            if item.is_empty() {
                "—".to_owned()
            } else {
                item.join(" · ")
            }
        }
    }
}

/// Escape one attribute value; heading ids and paths are the only inputs.
fn attribute(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn repository() -> Repository {
        Repository::load(env!("CARGO_MANIFEST_DIR")).expect("the governed roots parse")
    }

    #[test]
    fn every_record_body_carries_the_projection_heading_ids_and_studio_links() {
        let repository = repository();

        for record in repository.record() {
            if record.kind != Kind::Markdown {
                continue;
            }
            let html = body(record, &repository).into_string();
            for heading in record.heading.iter().filter(|heading| heading.level > 1) {
                assert!(
                    html.contains(&format!("id=\"{}\"", heading.id)),
                    "{} lost the heading id {}",
                    record.path,
                    heading.id
                );
            }
            assert!(
                !html.contains("href=\"../"),
                "{} kept a relative Markdown link",
                record.path
            );
        }
    }

    #[test]
    fn a_record_with_sections_gets_its_anchors_and_studio_links() {
        let repository = repository();
        let record = repository.get("AGENTS.md").expect("AGENTS.md exists");

        let html = body(record, &repository).into_string();

        assert!(html.contains("<h2 id=\"terry\">"));
        assert!(html.contains("class=\"anchor\""));

        let pointer = repository.get("CLAUDE.md").expect("CLAUDE.md exists");
        let pointer_html = body(pointer, &repository).into_string();
        assert!(pointer_html.contains("href=\"/doc/AGENTS.md\""));
        assert!(pointer_html.contains("href=\"/doc/docs/README.md\""));
    }

    #[test]
    fn a_migration_renders_as_sql_code() {
        let repository = repository();
        let record = repository
            .get("migration/0001_world.sql")
            .expect("the first migration exists");

        let html = body(record, &repository).into_string();

        assert!(html.starts_with("<pre>"));
        assert!(html.contains("language-sql"));
    }

    #[test]
    fn a_link_outside_the_governed_roots_is_disabled_rather_than_broken() {
        let repository = repository();
        let record = repository.get("AGENTS.md").expect("AGENTS.md exists");

        assert!(matches!(
            rewrite(record, &repository, "https://example.test/x"),
            Route::External(_)
        ));
        assert!(matches!(
            rewrite(record, &repository, "src/world/mod.rs"),
            Route::Outside
        ));
        assert!(matches!(
            rewrite(record, &repository, "docs/README.md#homes"),
            Route::Studio(_)
        ));
    }
}
