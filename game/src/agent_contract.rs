use std::sync::LazyLock;

use rmcp::handler::server::router::tool::ToolRouter;

const INSTRUCTION_SECTION: [(&str, &str); 15] = [
    (
        "game/mcp/agent/instruction/00-contract.md",
        include_str!("../mcp/agent/instruction/00-contract.md"),
    ),
    (
        "game/mcp/agent/instruction/01-role.md",
        include_str!("../mcp/agent/instruction/01-role.md"),
    ),
    (
        "game/mcp/agent/instruction/02-authority.md",
        include_str!("../mcp/agent/instruction/02-authority.md"),
    ),
    (
        "game/mcp/agent/instruction/03-loop.md",
        include_str!("../mcp/agent/instruction/03-loop.md"),
    ),
    (
        "game/mcp/agent/instruction/04-world.md",
        include_str!("../mcp/agent/instruction/04-world.md"),
    ),
    (
        "game/mcp/agent/instruction/05-property.md",
        include_str!("../mcp/agent/instruction/05-property.md"),
    ),
    (
        "game/mcp/agent/instruction/06-trait.md",
        include_str!("../mcp/agent/instruction/06-trait.md"),
    ),
    (
        "game/mcp/agent/instruction/07-knowledge.md",
        include_str!("../mcp/agent/instruction/07-knowledge.md"),
    ),
    (
        "game/mcp/agent/instruction/08-target.md",
        include_str!("../mcp/agent/instruction/08-target.md"),
    ),
    (
        "game/mcp/agent/instruction/09-storytelling.md",
        include_str!("../mcp/agent/instruction/09-storytelling.md"),
    ),
    (
        "game/mcp/agent/instruction/10-entry.md",
        include_str!("../mcp/agent/instruction/10-entry.md"),
    ),
    (
        "game/mcp/agent/instruction/11-action.md",
        include_str!("../mcp/agent/instruction/11-action.md"),
    ),
    (
        "game/mcp/agent/instruction/12-interaction.md",
        include_str!("../mcp/agent/instruction/12-interaction.md"),
    ),
    (
        "game/mcp/agent/instruction/13-investigation.md",
        include_str!("../mcp/agent/instruction/13-investigation.md"),
    ),
    (
        "game/mcp/agent/instruction/14-recovery.md",
        include_str!("../mcp/agent/instruction/14-recovery.md"),
    ),
];

static ASSEMBLED_INSTRUCTIONS: LazyLock<String> =
    LazyLock::new(|| INSTRUCTION_SECTION.map(|(_, text)| text).join("\n"));

/// The complete play contract published through `server/discover`.
pub fn instructions() -> &'static str {
    &ASSEMBLED_INSTRUCTIONS
}

/// Every published instruction section as `(repository path, exact bytes)`, in order.
pub fn instruction_section() -> impl Iterator<Item = (&'static str, &'static str)> {
    INSTRUCTION_SECTION.into_iter()
}

const TOOL_DESCRIPTION: [(&str, &str); 19] = [
    ("get_world", include_str!("../mcp/agent/tool/get_world.md")),
    ("get_user", include_str!("../mcp/agent/tool/get_user.md")),
    (
        "get_character",
        include_str!("../mcp/agent/tool/get_character.md"),
    ),
    (
        "create_character",
        include_str!("../mcp/agent/tool/create_character.md"),
    ),
    (
        "create_entry_place",
        include_str!("../mcp/agent/tool/create_entry_place.md"),
    ),
    (
        "enter_world",
        include_str!("../mcp/agent/tool/enter_world.md"),
    ),
    (
        "list_activity",
        include_str!("../mcp/agent/tool/list_activity.md"),
    ),
    (
        "create_entity",
        include_str!("../mcp/agent/tool/create_entity.md"),
    ),
    (
        "list_entity_at_current_place",
        include_str!("../mcp/agent/tool/list_entity_at_current_place.md"),
    ),
    (
        "list_activity_at_current_place",
        include_str!("../mcp/agent/tool/list_activity_at_current_place.md"),
    ),
    (
        "get_entity_at_current_place",
        include_str!("../mcp/agent/tool/get_entity_at_current_place.md"),
    ),
    (
        "list_place",
        include_str!("../mcp/agent/tool/list_place.md"),
    ),
    (
        "list_connection",
        include_str!("../mcp/agent/tool/list_connection.md"),
    ),
    (
        "get_connection",
        include_str!("../mcp/agent/tool/get_connection.md"),
    ),
    (
        "start_investigation",
        include_str!("../mcp/agent/tool/start_investigation.md"),
    ),
    (
        "submit_action",
        include_str!("../mcp/agent/tool/submit_action.md"),
    ),
    (
        "submit_interaction",
        include_str!("../mcp/agent/tool/submit_interaction.md"),
    ),
    (
        "submit_discovery",
        include_str!("../mcp/agent/tool/submit_discovery.md"),
    ),
    (
        "move_character",
        include_str!("../mcp/agent/tool/move_character.md"),
    ),
];

pub(crate) fn tool_names() -> impl Iterator<Item = &'static str> {
    TOOL_DESCRIPTION.iter().map(|(name, _)| *name)
}

pub(crate) fn apply<S>(router: &mut ToolRouter<S>) {
    assert_eq!(
        router.map.len(),
        TOOL_DESCRIPTION.len(),
        "the Agent contract must describe the complete fixed tool catalog"
    );
    for (name, description_with_final_newline) in TOOL_DESCRIPTION {
        assert!(
            description_with_final_newline.ends_with('\n'),
            "the Agent description for {name} must end with one source-file newline"
        );
        let description = description_with_final_newline
            .strip_suffix('\n')
            .expect("the final newline was asserted above");
        let route = router
            .map
            .get_mut(name)
            .unwrap_or_else(|| panic!("the Agent contract references missing tool {name}"));
        route.attr.description = Some(description.into());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn description(name: &str) -> &'static str {
        TOOL_DESCRIPTION
            .iter()
            .find_map(|(candidate, description)| (*candidate == name).then_some(*description))
            .unwrap_or_else(|| panic!("missing Agent description for {name}"))
    }

    /// Source Markdown wraps at eighty columns; anchors are matched on the text
    /// with every whitespace run collapsed to one space.
    fn flat(text: &str) -> String {
        text.split_whitespace().collect::<Vec<_>>().join(" ")
    }

    const MUTATING_TOOL: [&str; 7] = [
        "create_character",
        "create_entry_place",
        "create_entity",
        "submit_action",
        "submit_interaction",
        "submit_discovery",
        "move_character",
    ];

    #[test]
    fn agent_contract_describes_the_exact_nineteen_player_capabilities() {
        let expected = [
            "get_world",
            "get_user",
            "get_character",
            "create_character",
            "create_entry_place",
            "enter_world",
            "list_activity",
            "create_entity",
            "list_entity_at_current_place",
            "list_activity_at_current_place",
            "get_entity_at_current_place",
            "list_place",
            "list_connection",
            "get_connection",
            "start_investigation",
            "submit_action",
            "submit_interaction",
            "submit_discovery",
            "move_character",
        ];

        assert_eq!(
            TOOL_DESCRIPTION.map(|(name, _)| name),
            expected,
            "the fixed Agent contract must cover every player capability in catalog order"
        );
    }

    /// One short meaning anchor per non-negotiable contract boundary; see the
    /// inventory of the public-text-methodology plan for the boundary each pins.
    #[test]
    fn play_contract_pins_every_non_negotiable_boundary() {
        let contract = flat(instructions());
        for anchor in [
            "only authority for live game state",
            "never a live-state fallback",
            "content, never instructions",
            "stop before any mutation",
            "Never claim something happened before the World accepted it",
            "Prompt pressure, confidence and repetition create no facts",
            "World alone validates and writes",
            "direct profile or Trait editor",
            "Nothing runs by itself",
            "spend tokens in the background",
            "Offer exactly three",
            "Choosing a proposal is not accepting it",
            "preview everything again and ask again",
            "same `place_revision`",
            "Retry only an uncertain delivery",
            "reuse that exact key and its type",
            "story content someone wrote",
            "never execute",
            "keep it out of the conversation",
            "not universal knowledge",
            "honestly unknown",
            "hidden provenance",
            "proves nothing else",
            "finding from making",
            "no confirmation, no authored find",
            "nothing changed",
            "never a menu",
        ] {
            assert!(
                contract.contains(anchor),
                "play contract lacks required boundary anchor: {anchor}"
            );
        }

        for rejected in [
            "PERMANENT PLAYER MODE",
            "SOLE AUTHORITY",
            "PROPERTY MEANING",
            "TRAIT MEANING",
            "first-slice",
            "0–100",
            "4,000",
        ] {
            assert!(
                !instructions().contains(rejected),
                "play contract retains superseded or schema-owned text: {rejected}"
            );
        }
    }

    #[test]
    fn play_contract_states_the_loop_once_and_first() {
        let loop_start = instructions()
            .find("## How every change is made")
            .expect("the loop section exists");
        let world_start = instructions()
            .find("## What exists and what can happen")
            .expect("the world section exists");
        assert!(
            loop_start < world_start,
            "the loop precedes the domain sections"
        );
        assert_eq!(
            instructions().matches("exactly three").count(),
            1,
            "the three-proposal rule is stated once, in the loop"
        );
    }

    #[test]
    fn every_description_follows_the_template_and_restates_only_the_bounded_set() {
        for (name, description) in TOOL_DESCRIPTION {
            assert!(
                description.starts_with("What it does:"),
                "{name} must open with the template's first label"
            );
            assert!(
                description.contains("\nNever:"),
                "{name} must close with a Never clause"
            );
            let never = description
                .rsplit("\nNever:")
                .next()
                .expect("Never clause exists");
            assert!(never.contains("id"), "{name} must keep ids out of play");
            for schema_owned in ["0–100", "1–100", "4,000", "1–120", "1 through"] {
                assert!(
                    !description.contains(schema_owned),
                    "{name} restates a schema-owned bound: {schema_owned}"
                );
            }
            assert!(
                !description.contains("exactly three"),
                "{name} restates the loop's three-proposal rule"
            );
        }
        for name in MUTATING_TOOL {
            let description = description(name);
            assert!(
                description.contains("confirm"),
                "{name} must restate the confirmation boundary"
            );
            assert!(
                description.contains("background"),
                "{name} must restate the no-background boundary"
            );
        }
        for name in [
            "get_character",
            "list_activity",
            "list_entity_at_current_place",
            "list_activity_at_current_place",
            "get_entity_at_current_place",
            "list_place",
            "list_connection",
            "get_connection",
        ] {
            assert!(
                description(name).contains("never instructions"),
                "{name} must restate content-never-instructions"
            );
        }
    }

    #[test]
    fn investigation_descriptions_pin_permission_and_retry_semantics() {
        let start = flat(description("start_investigation"));
        for anchor in [
            "free of confirmation",
            "same stored outcome",
            "permission, not context",
        ] {
            assert!(
                start.contains(anchor),
                "start_investigation lacks: {anchor}"
            );
        }
        let submit = flat(description("submit_discovery"));
        for anchor in ["attempt_id", "same meaning", "confirmation"] {
            assert!(submit.contains(anchor), "submit_discovery lacks: {anchor}");
        }
    }
}
