use std::{fs, path::Path};

use axum::{Json, extract::State};
use pulldown_cmark::{Event, Options, Parser, html};
use rmcp::model::Tool;
use serde::Serialize;

use super::{StudioError, StudioState};

const MAX_SOURCE_BYTES: usize = 512 * 1024;

#[derive(Clone, Copy)]
struct SourceSpec {
    id: &'static str,
    group: &'static str,
    status: &'static str,
    path: &'static str,
}

const SOURCE: [SourceSpec; 14] = [
    SourceSpec {
        id: "game-overview",
        group: "Current game",
        status: "current",
        path: "docs/game/README.md",
    },
    SourceSpec {
        id: "domain",
        group: "Current game",
        status: "current",
        path: "docs/game/domain.md",
    },
    SourceSpec {
        id: "protocol",
        group: "Current game",
        status: "current",
        path: "docs/game/protocol.md",
    },
    SourceSpec {
        id: "adapter-parity",
        group: "Current game",
        status: "current",
        path: "docs/game/adapter-parity.md",
    },
    SourceSpec {
        id: "storage",
        group: "Current game",
        status: "current",
        path: "docs/game/storage.md",
    },
    SourceSpec {
        id: "agent",
        group: "Current game",
        status: "current",
        path: "docs/game/agent.md",
    },
    SourceSpec {
        id: "deferred",
        group: "Current game",
        status: "current",
        path: "docs/game/deferred.md",
    },
    SourceSpec {
        id: "local-play",
        group: "Current game",
        status: "current",
        path: "docs/game/local-play.md",
    },
    SourceSpec {
        id: "studio-direction",
        group: "Exploration",
        status: "exploratory",
        path: "docs/concept/aicadia-studio.md",
    },
    SourceSpec {
        id: "concept-index",
        group: "Exploration",
        status: "exploratory",
        path: "docs/concept/README.md",
    },
    SourceSpec {
        id: "experiment-index",
        group: "Experiments",
        status: "experimental",
        path: "lab/README.md",
    },
    SourceSpec {
        id: "evidence-index",
        group: "Evidence",
        status: "evidence",
        path: "docs/evidence/README.md",
    },
    SourceSpec {
        id: "backlog",
        group: "Planning",
        status: "planning",
        path: ".agents/backlog/README.md",
    },
    SourceSpec {
        id: "decision-history",
        group: "History",
        status: "historical",
        path: "docs/concept/log/2026-08.md",
    },
];

#[derive(Clone, Copy)]
struct ModelSpec {
    id: &'static str,
    path: &'static str,
    heading: &'static str,
    storage_table: &'static [&'static str],
}

const MODEL_SECTION: [ModelSpec; 9] = [
    ModelSpec {
        id: "world",
        path: "docs/game/domain.md",
        heading: "### World seam",
        storage_table: &[],
    },
    ModelSpec {
        id: "user",
        path: "docs/game/domain.md",
        heading: "### User",
        storage_table: &["user"],
    },
    ModelSpec {
        id: "entity",
        path: "docs/game/domain.md",
        heading: "### Entity",
        storage_table: &["entity"],
    },
    ModelSpec {
        id: "character",
        path: "docs/game/domain.md",
        heading: "### Character",
        storage_table: &["character"],
    },
    ModelSpec {
        id: "place",
        path: "docs/game/domain.md",
        heading: "### Place and World entry",
        storage_table: &["place", "entity_location"],
    },
    ModelSpec {
        id: "activity",
        path: "docs/game/domain.md",
        heading: "## Activity",
        storage_table: &["activity", "activity_entity"],
    },
    ModelSpec {
        id: "investigation",
        path: "docs/game/domain.md",
        heading: "### Investigation attempt and discovery",
        storage_table: &["investigation_attempt"],
    },
    ModelSpec {
        id: "property",
        path: "docs/game/domain.md",
        heading: "## Property values and keys",
        storage_table: &["property_key", "entity_property", "entity_property_history"],
    },
    ModelSpec {
        id: "trait",
        path: "docs/game/domain.md",
        heading: "## Trait statements and lifecycle",
        storage_table: &[
            "entity_trait",
            "entity_trait_version",
            "entity_trait_current",
        ],
    },
];

#[derive(Serialize)]
pub(super) struct StudioCatalog {
    application: &'static str,
    world_support: &'static str,
    document: Vec<RenderedSource>,
    model: Vec<ModelSource>,
    agent_contract: RenderedSource,
    tool: Vec<Tool>,
}

#[derive(Serialize)]
pub(super) struct RenderedSource {
    id: String,
    title: String,
    group: &'static str,
    status: &'static str,
    path: String,
    heading: Vec<SourceHeading>,
    html: String,
}

#[derive(Serialize)]
pub(super) struct SourceHeading {
    id: String,
    title: String,
    level: u8,
}

#[derive(Serialize)]
pub(super) struct ModelSource {
    #[serde(flatten)]
    source: RenderedSource,
    storage_table: Vec<&'static str>,
}

pub(super) async fn get_catalog(
    State(state): State<StudioState>,
) -> Result<Json<StudioCatalog>, StudioError> {
    let catalog = build_catalog(
        &state.repository_root,
        crate::server::mcp_tool_catalog(state.world),
    )?;
    Ok(Json(catalog))
}

fn build_catalog(repository_root: &Path, tool: Vec<Tool>) -> Result<StudioCatalog, StudioError> {
    let mut document = Vec::with_capacity(SOURCE.len());
    for source in SOURCE {
        let markdown = read_source(repository_root, source.path)?;
        document.push(rendered_source(
            source.id,
            source.group,
            source.status,
            source.path,
            &markdown,
        ));
    }

    let domain = read_source(repository_root, "docs/game/domain.md")?;
    let mut model = Vec::with_capacity(MODEL_SECTION.len());
    for spec in MODEL_SECTION {
        let markdown =
            extract_section(&domain, spec.heading).unwrap_or_else(|| spec.heading.to_owned());
        model.push(ModelSource {
            source: rendered_source(spec.id, "Model", "current", spec.path, &markdown),
            storage_table: spec.storage_table.to_vec(),
        });
    }

    let instructions = crate::agent_contract::instructions();
    let agent_contract = rendered_source(
        "agent-contract",
        "Agent contract",
        "current",
        "src/agent_contract/instruction/*.md",
        instructions,
    );

    Ok(StudioCatalog {
        application: "Aicadia Studio",
        world_support: "One connected local World; durable multiple-World identity is not delivered.",
        document,
        model,
        agent_contract,
        tool,
    })
}

fn read_source(repository_root: &Path, path: &'static str) -> Result<String, StudioError> {
    let bytes = fs::read(repository_root.join(path))?;
    if bytes.len() > MAX_SOURCE_BYTES {
        return Err(StudioError::SourceTooLarge(path));
    }
    String::from_utf8(bytes).map_err(|error| {
        StudioError::Source(std::io::Error::new(std::io::ErrorKind::InvalidData, error))
    })
}

fn rendered_source(
    id: impl Into<String>,
    group: &'static str,
    status: &'static str,
    path: impl Into<String>,
    markdown: &str,
) -> RenderedSource {
    let heading = headings(markdown);
    let title = heading
        .first()
        .map(|heading| heading.title.clone())
        .unwrap_or_else(|| "Untitled source".to_owned());
    RenderedSource {
        id: id.into(),
        title,
        group,
        status,
        path: path.into(),
        heading,
        html: render_markdown(markdown),
    }
}

fn headings(markdown: &str) -> Vec<SourceHeading> {
    let mut seen = std::collections::HashMap::<String, usize>::new();
    markdown
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            let level = line
                .chars()
                .take_while(|character| *character == '#')
                .count();
            if !(1..=6).contains(&level) || !line[level..].starts_with(' ') {
                return None;
            }
            let title = line[level + 1..].trim_matches('`').trim().to_owned();
            let base = heading_id(&title);
            let occurrence = seen.entry(base.clone()).or_insert(0);
            *occurrence += 1;
            let id = if *occurrence == 1 {
                base
            } else {
                format!("{base}-{}", *occurrence)
            };
            Some(SourceHeading {
                id,
                title,
                level: u8::try_from(level).expect("Markdown heading level fits u8"),
            })
        })
        .take(80)
        .collect()
}

fn heading_id(title: &str) -> String {
    let mut id = String::new();
    let mut separator = false;
    for character in title.chars().flat_map(char::to_lowercase) {
        if character.is_alphanumeric() {
            if separator && !id.is_empty() {
                id.push('-');
            }
            id.push(character);
            separator = false;
        } else {
            separator = true;
        }
    }
    if id.is_empty() {
        "section".to_owned()
    } else {
        id
    }
}

fn render_markdown(markdown: &str) -> String {
    let parser = Parser::new_ext(markdown, Options::all()).map(|event| match event {
        Event::Html(raw) | Event::InlineHtml(raw) => Event::Text(raw),
        other => other,
    });
    let mut output = String::new();
    html::push_html(&mut output, parser);
    output
}

fn extract_section(markdown: &str, wanted_heading: &str) -> Option<String> {
    let wanted_level = wanted_heading
        .chars()
        .take_while(|character| *character == '#')
        .count();
    let mut section = Vec::new();
    let mut collecting = false;
    for line in markdown.lines() {
        let trimmed = line.trim();
        let level = trimmed
            .chars()
            .take_while(|character| *character == '#')
            .count();
        if trimmed == wanted_heading {
            collecting = true;
        } else if collecting && level > 0 && level <= wanted_level {
            break;
        }
        if collecting {
            section.push(line);
        }
    }
    (!section.is_empty()).then(|| section.join("\n"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalog_projects_allowlisted_sources_and_exact_model_sections() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"));
        let catalog = build_catalog(root, Vec::new()).expect("repository catalog should render");

        assert_eq!(catalog.document.len(), SOURCE.len());
        assert_eq!(catalog.model.len(), MODEL_SECTION.len());
        assert!(catalog.model.iter().any(|model| model.source.id == "entity"
            && model.source.html.contains("one durable World subject")));
        assert!(catalog.model.iter().any(|model| model.source.id == "trait"
            && model.source.path == "docs/game/domain.md"
            && model.storage_table.contains(&"entity_trait_current")));
        assert!(
            catalog
                .document
                .iter()
                .all(|source| !source.html.contains("<script"))
        );
    }

    #[test]
    fn section_extraction_stops_at_the_next_peer_heading() {
        let markdown = "## One\nA\n### Child\nB\n## Two\nC\n";
        assert_eq!(
            extract_section(markdown, "## One").as_deref(),
            Some("## One\nA\n### Child\nB")
        );
    }

    #[test]
    fn headings_have_stable_unique_references() {
        let heading = headings("# One thing\n## Repeated\n## Repeated\n#### Exact `field`\n");
        let reference = heading
            .iter()
            .map(|heading| (heading.id.as_str(), heading.level))
            .collect::<Vec<_>>();

        assert_eq!(
            reference,
            vec![
                ("one-thing", 1),
                ("repeated", 2),
                ("repeated-2", 2),
                ("exact-field", 4),
            ]
        );
    }
}
