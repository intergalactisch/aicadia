//! The Game section: runtime contracts joined to the compiled Agent surface and
//! the connected PostgreSQL structure.
//!
//! This module authors presentation only. Model and capability membership comes
//! from the repository projection, tools come from the compiled MCP catalog, and
//! columns and foreign keys come from PostgreSQL on each explicit request.

use std::time::Duration;

use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode, Uri},
    response::Response,
};
use maud::{Markup, PreEscaped, html};
use rmcp::model::Tool;
use sqlx::{FromRow, PgPool};

use super::{
    Action, Context, Crumb, Page, Panel, PanelItem, Plate, PlateRow, Reference, Seal, Section,
    Tone, count, doc_href, empty_state, home_label, note,
};
use crate::{
    StudioState,
    agent::ToolContract,
    model::{self, Model},
    record::{Backlink, Record, Repository, render_markdown},
};

const SCHEMA_LIMIT: i64 = 4_096;
const SCHEMA_TIMEOUT: Duration = Duration::from_secs(2);

/// `/game` — the complete runtime-side orientation.
pub(crate) async fn index(
    State(state): State<StudioState>,
    header: HeaderMap,
    uri: Uri,
) -> Response {
    let context = match Context::build(&state, &header, &uri).await {
        Ok(context) => context,
        Err(response) => return response,
    };
    context.render(index_page(&context))
}

/// `/game/model/{id}` — one model contract plus its realized tables and joins.
pub(crate) async fn model_page(
    State(state): State<StudioState>,
    header: HeaderMap,
    uri: Uri,
    Path(id): Path<String>,
) -> Response {
    let context = match Context::build(&state, &header, &uri).await {
        Ok(context) => context,
        Err(response) => return response,
    };
    let models = model::models(context.repository());
    let Some(selected) = models.iter().find(|model| model.id == id) else {
        return not_found(&context, "model", &id);
    };
    let schema = if context.pulse().is_connected() {
        read_model_schema(&state.pool, &selected.storage_table).await
    } else {
        SchemaState::Unavailable
    };
    context.render(model_detail_page(&context, selected, schema))
}

/// `/game/capability/{name}` — the owning capability and its compiled operation.
pub(crate) async fn capability_page(
    State(state): State<StudioState>,
    header: HeaderMap,
    uri: Uri,
    Path(name): Path<String>,
) -> Response {
    let catalog = aicadia::server::mcp_tool_catalog(state.world.clone());
    let context = match Context::build(&state, &header, &uri).await {
        Ok(context) => context,
        Err(response) => return response,
    };
    let Some(contract) = context.surface().tool(&name) else {
        return not_found(&context, "capability", &name);
    };
    let compiled = catalog.iter().find(|tool| tool.name == name);
    context.render(capability_detail_page(&context, contract, compiled))
}

/// `/game/tool/{name}` — one exact compiled MCP catalog entry.
pub(crate) async fn tool_page(
    State(state): State<StudioState>,
    header: HeaderMap,
    uri: Uri,
    Path(name): Path<String>,
) -> Response {
    let catalog = aicadia::server::mcp_tool_catalog(state.world.clone());
    let context = match Context::build(&state, &header, &uri).await {
        Ok(context) => context,
        Err(response) => return response,
    };
    let Some(tool) = catalog.iter().find(|tool| tool.name == name) else {
        return not_found(&context, "tool", &name);
    };
    let Some(contract) = context.surface().tool(&name) else {
        return not_found(&context, "tool contract", &name);
    };
    context.render(tool_detail_page(&context, tool, contract))
}

/// `/game/agent` — exact assembled instructions, sections and complete catalog.
pub(crate) async fn agent_page(
    State(state): State<StudioState>,
    header: HeaderMap,
    uri: Uri,
) -> Response {
    let catalog = aicadia::server::mcp_tool_catalog(state.world.clone());
    let context = match Context::build(&state, &header, &uri).await {
        Ok(context) => context,
        Err(response) => return response,
    };
    context.render(agent_surface_page(&context, &catalog))
}

/// `/game/vocabulary` — every canonical term and its matching model folder.
pub(crate) async fn vocabulary_page(
    State(state): State<StudioState>,
    header: HeaderMap,
    uri: Uri,
) -> Response {
    let context = match Context::build(&state, &header, &uri).await {
        Ok(context) => context,
        Err(response) => return response,
    };
    context.render(vocabulary_content_page(&context))
}

/// `/game/storage` — the owning storage contract and discovered migration list.
pub(crate) async fn storage_page(
    State(state): State<StudioState>,
    header: HeaderMap,
    uri: Uri,
) -> Response {
    let context = match Context::build(&state, &header, &uri).await {
        Ok(context) => context,
        Err(response) => return response,
    };
    context.render(storage_contract_page(&context))
}

/// `/game/deferred` — negative current scope from its sole authority.
pub(crate) async fn deferred_page(
    State(state): State<StudioState>,
    header: HeaderMap,
    uri: Uri,
) -> Response {
    let context = match Context::build(&state, &header, &uri).await {
        Ok(context) => context,
        Err(response) => return response,
    };
    let Some(record) = context.repository().get("game/docs/deferred.md") else {
        return not_found(&context, "governed record", "game/docs/deferred.md");
    };
    context.render(authority_page(
        &context,
        record,
        "Deferred game scope",
        "Negative current scope is shown only from its owning runtime contract.",
        "/game/deferred",
    ))
}

fn index_page(context: &Context) -> Page {
    let repository = context.repository();
    let models = model::models(repository);
    let tools = &context.surface().tool;
    let capabilities = repository.in_home("capability");
    let contracts = game_contracts(repository);
    let reference = page_reference(context, "Game", "/game", "runtime-side projection");

    Page::new(Section::Game, "Game")
        .with_document_title("Game · Aicadia Studio")
        .with_lede("The current runtime contract, its durable models and the exact surface published to player Agents.")
        .with_plate(
            Plate::new(vec![
                PlateRow::text("Models", models.len().to_string()),
                PlateRow::text("Capabilities", capabilities.len().to_string()),
                PlateRow::text("Published tools", tools.len().to_string()),
            ])
            .with_action(vec![Action::copy("Copy reference", reference.line())])
            .with_authority("Repository model folders and capability records, compiled MCP catalog and PostgreSQL migrations."),
        )
        .with_content(html! {
            section class="section" {
                h2 { "Models" span class="count" { (count(models.len(), "model", "models")) } }
                p class="section-note" { "Each folder owns one durable game concept and claims its realized tables." }
                ul class="list" role="list" {
                    @for item in &models {
                        li data-model=(item.id) {
                            a href=(model_href(&item.id)) { (&item.title) }
                            small { (count(item.storage_table.len(), "table", "tables")) }
                        }
                    }
                }
            }
            section class="section" {
                h2 { "Capabilities" span class="count" { (count(tools.len(), "published tool", "published tools")) } }
                p class="section-note" { "Every capability joins its owning contract to the exact compiled MCP and HTTP operation." }
                (tool_table(tools))
            }
            section class="section" {
                h2 { "Agent surface" }
                dl class="meta" {
                    dt { "Published instructions" }
                    dd { a href="/game/agent#assembled-instructions" { (count(context.surface().section.len(), "source section", "source sections")) } }
                    dt { "Tool catalog" }
                    dd { a href="/game/agent#tool-catalog" { (count(tools.len(), "compiled entry", "compiled entries")) } }
                    dt { "Host contract" }
                    dd { a href=(doc_href("game/docs/agent.md")) { "Agent play contract" } }
                    dt { "Local adapter" }
                    dd { a href=(doc_href("game/docs/local-play.md")) { "Local play" } }
                }
            }
            section class="section" {
                h2 { "Runtime contracts" span class="count" { (count(contracts.len(), "record", "records")) } }
                ul class="list" role="list" {
                    @for record in contracts {
                        li {
                            a href=(contract_href(record)) { (&record.title) }
                            small { (home_label(record.home_id())) }
                        }
                    }
                }
            }
        })
        .with_related(vec![Panel::list(
            "Reference",
            vec![
                PanelItem::link("Vocabulary", "/game/vocabulary")
                    .with_note(count(model::vocabulary(repository).len(), "term", "terms")),
                PanelItem::link("Storage", "/game/storage")
                    .with_note(count(model::storage_table(repository).len(), "table", "tables")),
                PanelItem::link("Deferred scope", "/game/deferred"),
            ],
        )])
        .with_reference(reference)
}

fn model_detail_page(context: &Context, selected: &Model, schema: SchemaState) -> Page {
    let repository = context.repository();
    let record = repository
        .get(&selected.path)
        .expect("a projected model always has its owning record");
    let capabilities = backlink_records(repository, &selected.backlink, "capability");
    let reference = page_reference(
        context,
        &selected.title,
        &model_href(&selected.id),
        &selected.path,
    );
    let mut related = vec![Panel::list(
        "Storage tables",
        selected
            .storage_table
            .iter()
            .map(|table| PanelItem::link(table, format!("/live/storage/{table}")))
            .collect(),
    )];
    related.push(Panel::list(
        "Capabilities",
        capabilities
            .iter()
            .map(|record| PanelItem::link(&record.title, capability_href(file_stem(&record.path))))
            .collect(),
    ));
    related.extend(backlink_panels(repository, &selected.backlink));

    Page::new(Section::Game, &selected.title)
        .with_lede(
            selected
                .term
                .as_ref()
                .map_or_else(|| "No canonical vocabulary term is joined to this model folder.".to_owned(), |term| term.definition.clone()),
        )
        .with_crumb(vec![
            Crumb::link("Game", "/game"),
            Crumb::here("Models"),
            Crumb::here(&selected.title),
        ])
        .with_plate(
            Plate::new(vec![
                PlateRow::fact("Model id", &selected.id),
                PlateRow::fact("Source", &selected.path),
                PlateRow::text("Storage", count(selected.storage_table.len(), "table", "tables")),
                PlateRow::text("Capabilities", capabilities.len().to_string()),
            ])
            .with_action(vec![
                Action::copy("Copy reference", reference.line()),
                Action::link("Open owning record", doc_href(&selected.path)),
            ]),
        )
        .with_content(html! {
            section class="section" {
                h2 { "Contract" }
                (record_markup(record))
            }
            @if let Some(term) = &selected.term {
                section class="section" {
                    h2 { "Canonical vocabulary" }
                    dl class="meta" {
                        dt { "Term" }
                        dd { a href=(format!("/game/vocabulary#{}", term.id)) { (&term.name) } }
                        dt { "Definition" }
                        dd { (&term.definition) }
                        @if let Some(avoid) = &term.avoid {
                            dt { "Avoid" }
                            dd { (avoid) }
                        }
                    }
                }
            }
            section class="section" {
                h2 { "Realized PostgreSQL structure" }
                (schema_markup(schema, &selected.storage_table))
            }
            @if !selected.concern.is_empty() {
                section class="section" {
                    h2 { "Model concerns" span class="count" { (count(selected.concern.len(), "record", "records")) } }
                    ul class="list" role="list" {
                        @for path in &selected.concern {
                            @if let Some(concern) = repository.get(path) {
                                li { a href=(doc_href(path)) { (&concern.title) } small { (path) } }
                            }
                        }
                    }
                }
            }
        })
        .with_related(related)
        .with_reference(reference)
}

fn capability_detail_page(
    context: &Context,
    contract: &ToolContract,
    compiled: Option<&Tool>,
) -> Page {
    let repository = context.repository();
    let record = repository.get(&contract.capability_path);
    let title = record.map_or_else(|| contract.name.clone(), |record| record.title.clone());
    let reference = page_reference(
        context,
        &title,
        &capability_href(&contract.name),
        &contract.capability_path,
    );
    let route = contract
        .route()
        .unwrap_or_else(|| "Not published over HTTP".to_owned());

    Page::new(Section::Game, &title)
        .with_lede("One player capability joined to its owning contract and exact compiled publication.")
        .with_seal(if compiled.is_some() {
            Vec::new()
        } else {
            vec![Seal::status("Unavailable")]
        })
        .with_crumb(vec![
            Crumb::link("Game", "/game"),
            Crumb::here("Capabilities"),
            Crumb::here(&contract.name),
        ])
        .with_plate(
            Plate::new(vec![
                PlateRow::fact("MCP", &contract.name),
                PlateRow::fact("HTTP", &route),
                PlateRow::fact("Authority", &contract.capability_path),
            ])
            .with_action(vec![
                Action::copy("Copy reference", reference.line()),
                Action::link("Open tool", tool_href(&contract.name)),
            ])
            .with_authority(record.and_then(|record| record.role_header.as_ref()).map_or("Capability contract", |role| &role.authority)),
        )
        .with_content(html! {
            section class="section" {
                h2 { "Capability contract" }
                @match record {
                    Some(record) => (record_markup(record)),
                    None => (note(Tone::Brick, "The compiled tool has no owning capability record.")),
                }
            }
            section class="section" {
                h2 { "Published tool description" }
                @match contract.description.as_deref() {
                    Some(description) => article class="prose" { (PreEscaped(render_markdown(description))) },
                    None => (empty_state("The compiled tool publishes no description.")),
                }
            }
            section class="section" {
                h2 { "Input schema" }
                (json_block(&contract.input_schema))
            }
            section class="section" {
                h2 { "Annotations" }
                @match compiled.and_then(|tool| tool.annotations.as_ref()) {
                    Some(annotations) => (json_block(annotations)),
                    None => (empty_state("The compiled tool publishes no annotations.")),
                }
            }
        })
        .with_related(vec![Panel::list(
            "Evidence and references",
            record.map_or_else(Vec::new, |record| referenced_by(repository, record)),
        )])
        .with_reference(reference)
}

fn tool_detail_page(context: &Context, tool: &Tool, contract: &ToolContract) -> Page {
    let title = tool.name.to_string();
    let path = tool_href(&tool.name);
    let reference = page_reference(
        context,
        &title,
        &path,
        &format!("compiled MCP tool `{}`", tool.name),
    );
    let route = contract
        .route()
        .unwrap_or_else(|| "Not published over HTTP".to_owned());

    Page::new(Section::Game, &title)
        .with_lede("The complete catalog entry compiled into the current MCP server.")
        .with_crumb(vec![
            Crumb::link("Game", "/game"),
            Crumb::link("Agent surface", "/game/agent"),
            Crumb::here(tool.name.as_ref()),
        ])
        .with_plate(
            Plate::new(vec![
                PlateRow::fact("MCP", tool.name.as_ref()),
                PlateRow::fact("HTTP", &route),
                PlateRow::fact("Description source", &contract.text_path),
            ])
            .with_action(vec![
                Action::copy("Copy reference", reference.line()),
                Action::link("Open capability", capability_href(&contract.name)),
            ])
            .with_authority("Compiled server catalog; no Studio-owned tool metadata."),
        )
        .with_content(html! {
            section class="section" {
                h2 { "Complete compiled entry" }
                (json_block(tool))
            }
            section class="section" {
                h2 { "Published description" }
                @if let Some(description) = &tool.description {
                    article class="prose" { (PreEscaped(render_markdown(description))) }
                } @else {
                    (empty_state("The compiled tool publishes no description."))
                }
            }
        })
        .with_related(vec![
            Panel::list(
                "Contract",
                vec![
                    PanelItem::link("Capability contract", capability_href(&contract.name)),
                    PanelItem::link("Tool text source", doc_href(&contract.text_path)),
                    PanelItem::link("Agent surface", "/game/agent#tool-catalog"),
                ],
            ),
            Panel::list(
                "Evidence and references",
                context
                    .repository()
                    .get(&contract.capability_path)
                    .map_or_else(Vec::new, |record| {
                        referenced_by(context.repository(), record)
                    }),
            ),
        ])
        .with_reference(reference)
}

fn agent_surface_page(context: &Context, catalog: &[Tool]) -> Page {
    let surface = context.surface();
    let reference = page_reference(
        context,
        "Agent surface",
        "/game/agent",
        "compiled MCP discovery surface",
    );
    let agent_contract = context.repository().get("game/docs/agent.md");
    let local_play = context.repository().get("game/docs/local-play.md");

    Page::new(Section::Game, "Agent surface")
        .with_lede("The exact assembled play instructions and complete tool catalog player Agents receive from the compiled server.")
        .with_crumb(vec![Crumb::link("Game", "/game"), Crumb::here("Agent surface")])
        .with_plate(
            Plate::new(vec![
                PlateRow::text("Instruction sections", surface.section.len().to_string()),
                PlateRow::text("Tools", catalog.len().to_string()),
                PlateRow::fact("Authority", "compiled server"),
            ])
            .with_action(vec![Action::copy("Copy reference", reference.line())])
            .with_authority("The same compiled instruction and catalog functions used by the MCP server."),
        )
        .with_content(html! {
            section class="section" id="host-requirements" {
                h2 { "Host requirements" }
                @match agent_contract {
                    Some(record) => article class="prose" { (PreEscaped(render_markdown(&markdown_section(record, "agent-guidance-and-player-facing-communication")))) },
                    None => (note(Tone::Brick, "The Agent play contract is unavailable.")),
                }
            }
            section class="section" {
                h2 { "Local adapter command shape" }
                @match local_play {
                    Some(record) => article class="prose" { (PreEscaped(render_markdown(&markdown_section(record, "start-the-agent-conversation")))) },
                    None => (note(Tone::Brick, "The local-play contract is unavailable.")),
                }
            }
            section class="section" id="assembled-instructions" {
                h2 { "Assembled instructions" span class="count" { (count(surface.section.len(), "source section", "source sections")) } }
                p class="section-note" { "The byte sequence below is returned directly by the compiled assembly function." }
                details {
                    summary class="btn" { "Show exact published instructions" }
                    pre data-assembled-instructions { code { (surface.instructions) } }
                }
                h3 class="subhead" { "Source outline" }
                ul class="list" role="list" {
                    @for section in &surface.section {
                        li data-instruction=(section.id) {
                            a href=(doc_href(&section.path)) { (&section.title) }
                            small { (&section.path) }
                        }
                    }
                }
            }
            section class="section" id="tool-catalog" {
                h2 { "Tool catalog" span class="count" { (count(catalog.len(), "compiled entry", "compiled entries")) } }
                (tool_table(&surface.tool))
                details {
                    summary class="btn" { "Show exact catalog JSON" }
                    (json_block(catalog))
                }
            }
        })
        .with_related(vec![Panel::list("Owning contracts", vec![
                PanelItem::link("Agent play contract", doc_href("game/docs/agent.md")),
                PanelItem::link("Local play", doc_href("game/docs/local-play.md")),
                PanelItem::link("Agent contract sources", doc_href("game/mcp/agent/README.md")),
            ])])
        .with_reference(reference)
}

fn vocabulary_content_page(context: &Context) -> Page {
    let terms = model::vocabulary(context.repository());
    let models = model::models(context.repository());
    let reference = page_reference(context, "Vocabulary", "/game/vocabulary", "dev/CONTEXT.md");

    Page::new(Section::Game, "Vocabulary")
        .with_lede("Canonical project terms, read in order from the one vocabulary authority and joined to matching model folders.")
        .with_crumb(vec![Crumb::link("Game", "/game"), Crumb::here("Vocabulary")])
        .with_plate(
            Plate::new(vec![
                PlateRow::text("Terms", terms.len().to_string()),
                PlateRow::fact("Authority", "dev/CONTEXT.md"),
            ])
            .with_action(vec![
                Action::copy("Copy reference", reference.line()),
                Action::link("Open owning record", doc_href("dev/CONTEXT.md")),
            ]),
        )
        .with_content(html! {
            section class="section" {
                h2 { "Terms" span class="count" { (count(terms.len(), "term", "terms")) } }
                @for term in &terms {
                    section class="section" id=(term.id) data-term=(term.id) {
                        h3 class="subhead" {
                            @if models.iter().any(|item| item.id == term.id) {
                                a href=(model_href(&term.id)) { (&term.name) }
                            } @else {
                                (&term.name)
                            }
                        }
                        dl class="meta" {
                            dt { "Definition" }
                            dd { (&term.definition) }
                            @if let Some(avoid) = &term.avoid {
                                dt { "Avoid" }
                                dd { (avoid) }
                            }
                            dt { "Source" }
                            dd { a href=(format!("{}#{}", doc_href("dev/CONTEXT.md"), term.id)) { "dev/CONTEXT.md" } }
                        }
                    }
                }
            }
        })
        .with_reference(reference)
}

fn storage_contract_page(context: &Context) -> Page {
    let repository = context.repository();
    let Some(record) = repository.get("game/docs/storage.md") else {
        return Page::new(Section::Game, "Storage contract unavailable").with_content(note(
            Tone::Brick,
            "The owning storage contract is unavailable.",
        ));
    };
    let tables = model::storage_table(repository);
    let models = model::models(repository);
    let migrations = repository.in_home("migration");
    let reference = page_reference(context, "Storage", "/game/storage", &record.path);

    Page::new(Section::Game, "Storage")
        .with_lede("The PostgreSQL contract, its discovered migrations and the final table names those migrations realize.")
        .with_crumb(vec![Crumb::link("Game", "/game"), Crumb::here("Storage")])
        .with_plate(
            Plate::new(vec![
                PlateRow::text("Migrations", migrations.len().to_string()),
                PlateRow::text("Application tables", tables.len().to_string()),
                PlateRow::fact("Authority", &record.path),
            ])
            .with_action(vec![
                Action::copy("Copy reference", reference.line()),
                Action::link("Open owning record", doc_href(&record.path)),
            ])
            .with_authority(record.role_header.as_ref().map_or("Storage contract", |role| &role.authority)),
        )
        .with_content(html! {
            section class="section" {
                h2 { "Storage contract" }
                (record_markup(record))
            }
            section class="section" {
                h2 { "Realized tables" span class="count" { (count(tables.len(), "table", "tables")) } }
                div class="data-wrap" {
                    table class="data" {
                        thead { tr { th { "Table" } th { "Created by" } th { "Renamed by" } th { "Model" } } }
                        tbody {
                            @for table in &tables {
                                tr data-storage-table=(table.name) {
                                    td { a class="row-link" href=(format!("/live/storage/{}", table.name)) { code class="fact" { (&table.name) } } }
                                    td { a href=(doc_href(&format!("game/migration/{}", table.created_by))) { (&table.created_by) } }
                                    td class="mute" {
                                        @match &table.renamed_by {
                                            Some(path) => a href=(doc_href(&format!("game/migration/{path}"))) { (path) },
                                            None => "—",
                                        }
                                    }
                                    td {
                                        @match model::owning_model(&models, &table.name) {
                                            Some(owner) => a href=(model_href(&owner.id)) { (&owner.title) },
                                            None => span class="mute" { "Unclaimed" },
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            section class="section" {
                h2 { "Migrations" span class="count" { (count(migrations.len(), "migration", "migrations")) } }
                ul class="list" role="list" {
                    @for migration in migrations {
                        li data-migration=(migration.path) {
                            a href=(doc_href(&migration.path)) { (&migration.title) }
                            small { (&migration.path) }
                        }
                    }
                }
            }
        })
        .with_related(vec![Panel::list(
            "Connected World",
            vec![PanelItem::link("Inspect realized schema", "/live/storage")],
        )])
        .with_reference(reference)
}

fn authority_page(context: &Context, record: &Record, title: &str, lede: &str, path: &str) -> Page {
    let reference = page_reference(context, title, path, &record.path);
    Page::new(Section::Game, title)
        .with_lede(lede)
        .with_crumb(vec![Crumb::link("Game", "/game"), Crumb::here(title)])
        .with_plate(
            Plate::new(vec![PlateRow::fact("Authority", &record.path)])
                .with_action(vec![
                    Action::copy("Copy reference", reference.line()),
                    Action::link("Open owning record", doc_href(&record.path)),
                ])
                .with_authority(
                    record
                        .role_header
                        .as_ref()
                        .map_or("Runtime contract", |role| &role.authority),
                ),
        )
        .with_content(record_markup(record))
        .with_related(vec![Panel::list(
            "Referenced by",
            referenced_by(context.repository(), record),
        )])
        .with_reference(reference)
}

#[derive(FromRow)]
struct SchemaColumn {
    table_name: String,
    column_name: String,
    position: i32,
    data_type: String,
    nullable: bool,
    default_value: Option<String>,
}

#[derive(FromRow)]
struct SchemaRelation {
    name: String,
    table_name: String,
    columns: Vec<String>,
    referenced_table: String,
    referenced_columns: Vec<String>,
    on_update: String,
    on_delete: String,
}

enum SchemaState {
    Available {
        column: Vec<SchemaColumn>,
        relation: Vec<SchemaRelation>,
    },
    Unavailable,
    OverBound,
}

async fn read_model_schema(pool: &PgPool, table: &[String]) -> SchemaState {
    if table.is_empty() {
        return SchemaState::Available {
            column: Vec::new(),
            relation: Vec::new(),
        };
    }
    let future = async {
        let column = sqlx::query_as::<_, SchemaColumn>(
            r#"
            SELECT columns.table_name,
                   columns.column_name,
                   columns.ordinal_position::integer AS position,
                   columns.data_type,
                   columns.is_nullable = 'YES' AS nullable,
                   columns.column_default AS default_value
            FROM information_schema.columns
            WHERE columns.table_schema = 'public'
              AND columns.table_name = ANY($1)
            ORDER BY columns.table_name, columns.ordinal_position
            LIMIT $2
            "#,
        )
        .bind(table)
        .bind(SCHEMA_LIMIT + 1)
        .fetch_all(pool)
        .await?;
        let relation = sqlx::query_as::<_, SchemaRelation>(
            r#"
            SELECT relation_constraint.conname AS name,
                   source.relname AS table_name,
                   ARRAY(
                       SELECT source_attribute.attname
                       FROM unnest(relation_constraint.conkey) WITH ORDINALITY AS key(number, position)
                       JOIN pg_attribute AS source_attribute
                         ON source_attribute.attrelid = source.oid
                        AND source_attribute.attnum = key.number
                       ORDER BY key.position
                   ) AS columns,
                   target.relname AS referenced_table,
                   ARRAY(
                       SELECT target_attribute.attname
                       FROM unnest(relation_constraint.confkey) WITH ORDINALITY AS key(number, position)
                       JOIN pg_attribute AS target_attribute
                         ON target_attribute.attrelid = target.oid
                        AND target_attribute.attnum = key.number
                       ORDER BY key.position
                   ) AS referenced_columns,
                   CASE relation_constraint.confupdtype
                       WHEN 'a' THEN 'no action' WHEN 'r' THEN 'restrict'
                       WHEN 'c' THEN 'cascade' WHEN 'n' THEN 'set null'
                       WHEN 'd' THEN 'set default'
                   END AS on_update,
                   CASE relation_constraint.confdeltype
                       WHEN 'a' THEN 'no action' WHEN 'r' THEN 'restrict'
                       WHEN 'c' THEN 'cascade' WHEN 'n' THEN 'set null'
                       WHEN 'd' THEN 'set default'
                   END AS on_delete
            FROM pg_constraint AS relation_constraint
            JOIN pg_class AS source ON source.oid = relation_constraint.conrelid
            JOIN pg_namespace AS namespace ON namespace.oid = source.relnamespace
            JOIN pg_class AS target ON target.oid = relation_constraint.confrelid
            WHERE relation_constraint.contype = 'f'
              AND namespace.nspname = 'public'
              AND (source.relname = ANY($1) OR target.relname = ANY($1))
            ORDER BY source.relname, relation_constraint.conname
            LIMIT $2
            "#,
        )
        .bind(table)
        .bind(SCHEMA_LIMIT + 1)
        .fetch_all(pool)
        .await?;
        Ok::<_, sqlx::Error>((column, relation))
    };

    match tokio::time::timeout(SCHEMA_TIMEOUT, future).await {
        Ok(Ok((column, relation)))
            if column.len() <= SCHEMA_LIMIT as usize && relation.len() <= SCHEMA_LIMIT as usize =>
        {
            SchemaState::Available { column, relation }
        }
        Ok(Ok(_)) => SchemaState::OverBound,
        Ok(Err(error)) => {
            eprintln!("Studio model schema read failed: {error}");
            SchemaState::Unavailable
        }
        Err(_) => {
            eprintln!("Studio model schema read timed out");
            SchemaState::Unavailable
        }
    }
}

fn schema_markup(schema: SchemaState, claimed_table: &[String]) -> Markup {
    match schema {
        SchemaState::Unavailable => note(
            Tone::Brick,
            "Connected database structure is unavailable; model contracts and migration claims remain visible.",
        ),
        SchemaState::OverBound => note(
            Tone::Brick,
            "Connected database structure exceeds the bounded Studio schema read.",
        ),
        SchemaState::Available { .. } if claimed_table.is_empty() => {
            empty_state("This model claims no PostgreSQL table.")
        }
        SchemaState::Available { column, relation } => html! {
            @for table in claimed_table {
                h3 class="subhead" { a href=(format!("/live/storage/{table}")) { code class="fact" { (table) } } }
                @let table_column = column.iter().filter(|column| column.table_name == *table).collect::<Vec<_>>();
                @if table_column.is_empty() {
                    (note(Tone::Amber, "The claimed table is absent from the connected public schema."))
                } @else {
                    div class="data-wrap" {
                        table class="data" {
                            thead { tr { th class="num" { "Position" } th { "Column" } th { "Type" } th { "Nullable" } th { "Default" } } }
                            tbody {
                                @for item in table_column {
                                    tr data-column=(format!("{}.{}", item.table_name, item.column_name)) {
                                        td class="num" { (item.position) }
                                        td { code class="fact" { (&item.column_name) } }
                                        td { code class="fact" { (&item.data_type) } }
                                        td { (if item.nullable { "yes" } else { "no" }) }
                                        td class="fact" { (item.default_value.as_deref().unwrap_or("—")) }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            h3 class="subhead" { "Ordered foreign keys" }
            @if relation.is_empty() {
                (empty_state("No foreign key enters or leaves these tables."))
            } @else {
                div class="data-wrap" {
                    table class="data" {
                        thead { tr { th { "Constraint" } th { "From" } th { "To" } th { "On update" } th { "On delete" } } }
                        tbody {
                            @for item in relation {
                                tr data-relation=(item.name) {
                                    td { code class="fact" { (item.name) } }
                                    td { code class="fact" { (format!("{}({})", item.table_name, item.columns.join(", "))) } }
                                    td { a href=(format!("/live/storage/{}", item.referenced_table)) { code class="fact" { (format!("{}({})", item.referenced_table, item.referenced_columns.join(", "))) } } }
                                    td { (item.on_update) }
                                    td { (item.on_delete) }
                                }
                            }
                        }
                    }
                }
            }
        },
    }
}

fn tool_table(tools: &[ToolContract]) -> Markup {
    html! {
        div class="data-wrap" {
            table class="data" {
                thead { tr { th { "MCP tool" } th { "HTTP route" } th { "Capability contract" } } }
                tbody {
                    @for tool in tools {
                        tr data-tool=(tool.name) {
                            td { a class="row-link" href=(tool_href(&tool.name)) { code class="fact" { (&tool.name) } } }
                            td { code class="fact" { (tool.route().as_deref().unwrap_or("—")) } }
                            td { a href=(capability_href(&tool.name)) { (&tool.capability_path) } }
                        }
                    }
                }
            }
        }
    }
}

fn record_markup(record: &Record) -> Markup {
    html! { article class="prose" { (PreEscaped(record.html())) } }
}

fn json_block<T: serde::Serialize + ?Sized>(value: &T) -> Markup {
    let json = serde_json::to_string_pretty(value)
        .unwrap_or_else(|error| format!("JSON serialization failed: {error}"));
    html! { pre { code { (json) } } }
}

fn markdown_section(record: &Record, id: &str) -> String {
    let mut start = None;
    let mut level = 0;
    let mut offset = 0;
    for line in record.body.split_inclusive('\n') {
        let trimmed = line.trim();
        let heading_level = trimmed
            .chars()
            .take_while(|character| *character == '#')
            .count();
        if (1..=6).contains(&heading_level) && trimmed[heading_level..].starts_with(' ') {
            let title = trimmed[heading_level + 1..].trim_matches('`').trim();
            if start.is_none() && crate::record::heading_id(title) == id {
                start = Some(offset);
                level = heading_level;
            } else if start.is_some() && heading_level <= level {
                return record.body[start.expect("section start exists")..offset].to_owned();
            }
        }
        offset += line.len();
    }
    start.map_or_else(String::new, |start| record.body[start..].to_owned())
}

fn game_contracts(repository: &Repository) -> Vec<&Record> {
    let mut record = repository.in_home("game-index");
    record.extend(repository.in_home("game-contract"));
    record.sort_by(|left, right| left.title.cmp(&right.title));
    record
}

fn contract_href(record: &Record) -> String {
    match record.path.as_str() {
        "game/docs/storage.md" => "/game/storage".to_owned(),
        "game/docs/deferred.md" => "/game/deferred".to_owned(),
        "game/docs/agent.md" => "/game/agent".to_owned(),
        _ => doc_href(&record.path),
    }
}

fn backlink_records<'a>(
    repository: &'a Repository,
    backlink: &[Backlink],
    home: &str,
) -> Vec<&'a Record> {
    let mut record = backlink
        .iter()
        .filter_map(|backlink| repository.get(&backlink.path))
        .filter(|record| record.home_id() == home)
        .collect::<Vec<_>>();
    record.sort_by(|left, right| left.title.cmp(&right.title));
    record.dedup_by(|left, right| left.path == right.path);
    record
}

fn backlink_panels(repository: &Repository, backlink: &[Backlink]) -> Vec<Panel> {
    let group = [
        ("Evidence", "evidence-slice"),
        ("Concept", "concept-record"),
        ("Research", "research-report"),
        ("Plans", "plan"),
    ];
    group
        .into_iter()
        .filter_map(|(title, home)| {
            let items = backlink_records(repository, backlink, home)
                .into_iter()
                .map(|record| PanelItem::link(&record.title, doc_href(&record.path)))
                .collect::<Vec<_>>();
            (!items.is_empty()).then(|| Panel::list(title, items))
        })
        .collect()
}

fn referenced_by(repository: &Repository, record: &Record) -> Vec<PanelItem> {
    repository
        .backlink(&record.path)
        .iter()
        .filter_map(|backlink| repository.get(&backlink.path))
        .map(|source| {
            PanelItem::link(&source.title, doc_href(&source.path))
                .with_note(home_label(source.home_id()))
        })
        .collect()
}

fn page_reference(context: &Context, title: &str, path: &str, authority: &str) -> Reference {
    Reference {
        title: title.to_owned(),
        url: context.url(path),
        context: authority.to_owned(),
    }
}

fn not_found(context: &Context, kind: &str, id: &str) -> Response {
    context.render_status(
        Page::new(Section::Game, format!("No {kind} at this path"))
            .with_document_title("Not found · Game · Aicadia Studio")
            .with_crumb(vec![Crumb::link("Game", "/game"), Crumb::here("Not found")])
            .with_seal(vec![Seal::toned("Not found", Tone::Brick)])
            .with_plate(Plate::new(vec![PlateRow::fact("Requested", id)]))
            .with_content(empty_state(&format!(
                "No projected {kind} matches this path."
            ))),
        StatusCode::NOT_FOUND,
    )
}

fn model_href(id: &str) -> String {
    format!("/game/model/{id}")
}

fn capability_href(name: &str) -> String {
    format!("/game/capability/{name}")
}

fn tool_href(name: &str) -> String {
    format!("/game/tool/{name}")
}

fn file_stem(path: &str) -> &str {
    path.rsplit('/')
        .next()
        .unwrap_or(path)
        .trim_end_matches(".md")
}
