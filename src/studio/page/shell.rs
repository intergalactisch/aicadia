//! The one Studio layout: bar, section tree, main column, related panels,
//! reference line and toast. Every page is this shell plus its own content.

use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};
use maud::{DOCTYPE, Markup, html};

use super::{Action, Context, Crumb, Page, PanelBody, Plate, PlateValue, Reference, Section, tree};

/// The Studio mark as a self-contained tab icon; Studio loads no external asset.
const FAVICON: &str = "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 32 32'%3E%3Crect width='32' height='32' rx='4' fill='%231b1a17'/%3E%3Ctext x='16' y='23' font-family='Georgia,serif' font-size='20' font-weight='600' text-anchor='middle' fill='%23faf9f6'%3EA%3C/text%3E%3C/svg%3E";

/// The complete HTML document of one page.
pub(super) fn render(context: &Context, page: &Page) -> Markup {
    let tree = tree::build(context, page.section, context.path());

    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                meta name="color-scheme" content="light";
                title { (page.document_title) }
                link rel="icon" href=(FAVICON);
                link rel="stylesheet" href="/assets/studio.css";
                script src="/assets/studio.js" defer {}
            }
            body {
                a class="skip" href="#content" { "Skip to content" }
                (bar(context, page.section))
                div class="frame" {
                    (tree::render(&tree))
                    main class="main" id="content" tabindex="-1" {
                        @if !page.crumb.is_empty() { (crumbs(&page.crumb)) }
                        header class="head" {
                            @if !page.seal.is_empty() {
                                div class="seals" {
                                    @for seal in &page.seal {
                                        span class=(seal.tone.class()) { (seal.text) }
                                    }
                                }
                            }
                            h1 { (page.title) }
                            @if let Some(lede) = &page.lede { p class="lede" { (lede) } }
                            @if let Some(plate) = &page.plate { (plate_markup(plate)) }
                        }
                        div class=(if page.related.is_empty() { "body body-wide" } else { "body" }) {
                            div class="content" {
                                (page.content)
                                @if let Some(reference) = &page.reference { (colophon(reference)) }
                            }
                            @if !page.related.is_empty() {
                                aside class="related" aria-label="Related" {
                                    @for panel in &page.related { (panel_markup(panel)) }
                                }
                            }
                        }
                    }
                }
                div class="toast" role="status" aria-live="polite" hidden {}
            }
        }
    }
}

/// The sticky top bar: mark, sections, jump box, connection pulse and Refresh.
fn bar(context: &Context, current: Section) -> Markup {
    let pulse = context.pulse();
    let database = pulse.database.as_deref().unwrap_or("no database");
    let reading = if pulse.is_connected() {
        format!("read {}", pulse.read_at)
    } else {
        "unavailable".to_owned()
    };

    html! {
        header class="bar" {
            button class="menu-toggle" type="button" aria-label="Open section navigation"
                aria-expanded="false" aria-controls="tree" { "☰" }
            a class="mark" href="/" {
                span class="mark-glyph" aria-hidden="true" { "A" }
                span class="mark-name" { "Aicadia " small { "Studio" } }
            }
            nav class="sections" aria-label="Studio sections" {
                @for section in Section::ALL {
                    a href=(section.href())
                        aria-current=[(section == current).then_some("page")] { (section.label()) }
                }
            }
            div class="bar-end" {
                form class="jump" role="search" action="/jump" method="get"
                    aria-label="Jump to a resource" {
                    span class="jump-glyph" aria-hidden="true" { "⌕" }
                    input type="search" name="q" autocomplete="off"
                        placeholder="Jump to a model, capability, record or table…"
                        aria-label="Jump to";
                    kbd { "/" }
                }
                div class="pulse" data-state=(pulse.state) {
                    span class="pulse-dot" aria-hidden="true" {}
                    span { b { (database) } " · " (reading) }
                }
                a class="btn btn-quiet" href=(context.uri()) { "Refresh" }
            }
        }
    }
}

fn crumbs(crumb: &[Crumb]) -> Markup {
    html! {
        nav class="crumbs" aria-label="Breadcrumb" {
            @for (index, step) in crumb.iter().enumerate() {
                @if index > 0 { span class="sep" aria-hidden="true" { "/" } }
                @match &step.href {
                    Some(href) => a href=(href) { (step.label) },
                    None if index + 1 == crumb.len() => span class="here" { (step.label) },
                    None => span { (step.label) },
                }
            }
        }
    }
}

fn plate_markup(plate: &Plate) -> Markup {
    html! {
        div class="plate" {
            div class="plate-row" {
                @for row in &plate.row {
                    span class="plate-key" { (row.key) }
                    @match &row.value {
                        PlateValue::Text(text) => span { (text) },
                        PlateValue::Fact(fact) => code class="fact" { (fact) },
                    }
                }
            }
            @if !plate.action.is_empty() {
                div class="plate-actions" {
                    @for action in &plate.action { (action_markup(action)) }
                }
            }
            @if let Some(authority) = &plate.authority {
                p class="plate-authority" { (authority) }
            }
        }
    }
}

fn action_markup(action: &Action) -> Markup {
    html! {
        @match action {
            Action::Copy { label, text } => {
                button class="btn btn-small" type="button" data-copy=(text) { (label) }
            },
            Action::Link { label, href } => {
                a class="btn btn-small btn-quiet" href=(href) { (label) }
            },
        }
    }
}

fn panel_markup(panel: &super::Panel) -> Markup {
    let outline = matches!(panel.body, PanelBody::Outline(_));

    html! {
        section class=(if outline { "panel outline" } else { "panel" }) {
            h2 { (panel.title) }
            @match &panel.body {
                PanelBody::List(item) => {
                    @if item.is_empty() {
                        p class="empty" { "None" }
                    } @else {
                        ul {
                            @for entry in item {
                                li {
                                    @match &entry.href {
                                        Some(href) => a href=(href) { (entry.label) },
                                        None => span { (entry.label) },
                                    }
                                    @if let Some(note) = &entry.note { small { (note) } }
                                }
                            }
                        }
                    }
                },
                PanelBody::Outline(item) => {
                    @if item.is_empty() {
                        p class="empty" { "No headings" }
                    } @else {
                        @for entry in item {
                            a href=(format!("#{}", entry.id)) data-level=(entry.level) { (entry.title) }
                        }
                    }
                },
                PanelBody::Definition(item) => {
                    dl class="meta" {
                        @for (key, value) in item {
                            dt { (key) }
                            dd { (value) }
                        }
                    }
                },
            }
        }
    }
}

fn colophon(reference: &Reference) -> Markup {
    let line = reference.line();

    html! {
        footer class="colophon" {
            span { "Reference · " code class="fact" { (line) } }
            button class="btn btn-small btn-quiet" type="button" data-copy=(line) { "Copy" }
        }
    }
}

/// The one page rendered without the projection, when the projection itself failed.
pub(super) fn plain_error(title: &str, detail: &str) -> Response {
    let markup = html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                meta name="color-scheme" content="light";
                title { (title) " · Aicadia Studio" }
                link rel="stylesheet" href="/assets/studio.css";
            }
            body {
                main class="main" id="content" tabindex="-1" {
                    header class="head" { h1 { (title) } }
                    p class="note note-error" { (detail) }
                }
            }
        }
    };

    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Html(markup.into_string()),
    )
        .into_response()
}
