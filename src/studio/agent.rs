//! The compiled Agent surface: instruction sections, tool catalog and their contracts.

use rmcp::model::Tool;
use utoipa::openapi::{PathItem, path::Operation};

use super::record::{Repository, headings};

/// One published instruction section, exactly as the server assembles it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InstructionSection {
    /// The file stem, such as `04-property`.
    pub id: String,
    pub title: String,
    pub path: String,
    pub text: &'static str,
}

/// One compiled tool joined to its capability contract, tool text and HTTP route.
#[derive(Clone, Debug)]
pub struct ToolContract {
    pub name: String,
    /// The published Agent-facing description, exactly as MCP serves it.
    pub description: Option<String>,
    pub input_schema: serde_json::Value,
    pub capability_path: String,
    pub capability_exists: bool,
    pub text_path: String,
    pub text_exists: bool,
    pub method: Option<String>,
    pub http_path: Option<String>,
}

impl ToolContract {
    /// `GET /api/world`, or `None` while the tool has no compiled HTTP operation.
    pub fn route(&self) -> Option<String> {
        Some(format!(
            "{} {}",
            self.method.as_ref()?,
            self.http_path.as_ref()?
        ))
    }
}

/// The complete Agent-facing surface, all of it read from compiled sources.
#[derive(Clone, Debug)]
pub struct AgentSurface {
    pub section: Vec<InstructionSection>,
    pub instructions: &'static str,
    pub tool: Vec<ToolContract>,
}

impl AgentSurface {
    pub fn tool(&self, name: &str) -> Option<&ToolContract> {
        self.tool.iter().find(|tool| tool.name == name)
    }
}

/// Project the compiled Agent surface and join it to the repository contracts.
pub fn surface(repository: &Repository, catalog: Vec<Tool>) -> AgentSurface {
    let section = crate::agent_contract::instruction_section()
        .map(|(path, text)| InstructionSection {
            id: super::home::file_name(path)
                .trim_end_matches(".md")
                .to_owned(),
            title: headings(text)
                .first()
                .map_or_else(|| path.to_owned(), |heading| heading.title.clone()),
            path: path.to_owned(),
            text,
        })
        .collect();

    let operation = http_operation();
    let tool = catalog
        .into_iter()
        .map(|tool| {
            let name = tool.name.to_string();
            let capability_path = format!("docs/game/capability/{name}.md");
            let text_path = format!("src/agent_contract/tool/{name}.md");
            let route = operation
                .iter()
                .find(|(operation_id, _, _)| *operation_id == name);
            ToolContract {
                capability_exists: repository.get(&capability_path).is_some(),
                text_exists: repository.get(&text_path).is_some(),
                description: tool.description.as_ref().map(ToString::to_string),
                input_schema: serde_json::Value::Object((*tool.input_schema).clone()),
                method: route.map(|(_, method, _)| (*method).to_owned()),
                http_path: route.map(|(_, _, path)| path.clone()),
                capability_path,
                text_path,
                name,
            }
        })
        .collect();

    AgentSurface {
        section,
        instructions: crate::agent_contract::instructions(),
        tool,
    }
}

/// Every compiled HTTP operation as `(operation id, method, path)`.
fn http_operation() -> Vec<(String, &'static str, String)> {
    let document = crate::server::openapi_document();
    let mut operation = Vec::new();
    for (path, item) in &document.paths.paths {
        for (method, candidate) in method(item) {
            let Some(candidate) = candidate else { continue };
            let Some(id) = candidate.operation_id.clone() else {
                continue;
            };
            operation.push((id, method, path.clone()));
        }
    }
    operation.sort();
    operation
}

fn method(item: &PathItem) -> [(&'static str, &Option<Operation>); 8] {
    [
        ("GET", &item.get),
        ("PUT", &item.put),
        ("POST", &item.post),
        ("DELETE", &item.delete),
        ("OPTIONS", &item.options),
        ("HEAD", &item.head),
        ("PATCH", &item.patch),
        ("TRACE", &item.trace),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::studio::record::Repository;

    fn fixture() -> Repository {
        let root = crate::studio::test_directory("agent-fixture");
        for path in [
            "docs/game/capability/get_world.md",
            "src/agent_contract/tool/get_world.md",
        ] {
            let full = root.join(path);
            std::fs::create_dir_all(full.parent().expect("parent")).expect("directory");
            std::fs::write(full, "# `get_world`\n").expect("record");
        }
        Repository::load(&root).expect("fixture loads")
    }

    fn catalog() -> Vec<Tool> {
        let pool =
            sqlx::PgPool::connect_lazy("postgresql:///unused").expect("a lazy pool never connects");
        crate::server::mcp_tool_catalog(crate::World::new(pool))
    }

    #[test]
    fn the_instruction_outline_is_the_sixteen_published_section_files() {
        let surface = surface(&fixture(), Vec::new());

        assert_eq!(surface.section.len(), 16);
        assert_eq!(surface.section[0].id, "00-contract");
        assert_eq!(surface.section[4].id, "04-property");
        assert!(
            surface
                .section
                .iter()
                .all(|section| !section.title.is_empty()
                    && section.path.starts_with("src/agent_contract/instruction/"))
        );
        assert_eq!(
            surface.instructions,
            crate::agent_contract::instructions(),
            "the outline never re-assembles the published bytes"
        );
        for section in &surface.section {
            assert!(surface.instructions.contains(section.text));
        }
    }

    #[tokio::test]
    async fn every_compiled_tool_joins_a_capability_contract_tool_text_and_http_route() {
        let surface = surface(&fixture(), catalog());

        assert_eq!(surface.tool.len(), 15);
        let world = surface.tool("get_world").expect("get_world is published");
        assert_eq!(world.route().as_deref(), Some("GET /api/world"));
        assert_eq!(world.capability_path, "docs/game/capability/get_world.md");
        assert!(world.capability_exists);
        assert!(world.text_exists);
        assert!(
            world
                .description
                .as_ref()
                .is_some_and(|description| !description.is_empty())
        );
        assert!(world.input_schema.is_object());

        let action = surface.tool("submit_action").expect("submit_action");
        assert_eq!(action.route().as_deref(), Some("POST /api/action"));
        assert!(
            !action.capability_exists && !action.text_exists,
            "the fixture only holds the get_world contracts"
        );
        assert!(surface.tool.iter().all(|tool| tool.route().is_some()));
    }
}
