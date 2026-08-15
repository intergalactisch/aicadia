use std::sync::LazyLock;

use rmcp::handler::server::router::tool::ToolRouter;

const INSTRUCTION_SECTION: [&str; 15] = [
    include_str!("agent_contract/instruction/00-contract.md"),
    include_str!("agent_contract/instruction/01-role.md"),
    include_str!("agent_contract/instruction/02-authority.md"),
    include_str!("agent_contract/instruction/03-world.md"),
    include_str!("agent_contract/instruction/04-property.md"),
    include_str!("agent_contract/instruction/05-trait.md"),
    include_str!("agent_contract/instruction/06-knowledge.md"),
    include_str!("agent_contract/instruction/07-target.md"),
    include_str!("agent_contract/instruction/08-storytelling.md"),
    include_str!("agent_contract/instruction/09-workshop.md"),
    include_str!("agent_contract/instruction/10-entry.md"),
    include_str!("agent_contract/instruction/11-orientation.md"),
    include_str!("agent_contract/instruction/12-action.md"),
    include_str!("agent_contract/instruction/13-interaction.md"),
    include_str!("agent_contract/instruction/14-recovery.md"),
];

static ASSEMBLED_INSTRUCTIONS: LazyLock<String> = LazyLock::new(|| INSTRUCTION_SECTION.join("\n"));

/// The complete play contract published through `server/discover`.
pub fn instructions() -> &'static str {
    &ASSEMBLED_INSTRUCTIONS
}

const TOOL_DESCRIPTION: [(&str, &str); 13] = [
    (
        "get_world",
        include_str!("agent_contract/tool/get_world.md"),
    ),
    ("get_user", include_str!("agent_contract/tool/get_user.md")),
    (
        "get_character",
        include_str!("agent_contract/tool/get_character.md"),
    ),
    (
        "create_character",
        include_str!("agent_contract/tool/create_character.md"),
    ),
    (
        "create_entry_place",
        include_str!("agent_contract/tool/create_entry_place.md"),
    ),
    (
        "enter_world",
        include_str!("agent_contract/tool/enter_world.md"),
    ),
    (
        "list_activity",
        include_str!("agent_contract/tool/list_activity.md"),
    ),
    (
        "create_entity",
        include_str!("agent_contract/tool/create_entity.md"),
    ),
    (
        "list_entity_at_current_place",
        include_str!("agent_contract/tool/list_entity_at_current_place.md"),
    ),
    (
        "list_activity_at_current_place",
        include_str!("agent_contract/tool/list_activity_at_current_place.md"),
    ),
    (
        "get_entity_at_current_place",
        include_str!("agent_contract/tool/get_entity_at_current_place.md"),
    ),
    (
        "submit_action",
        include_str!("agent_contract/tool/submit_action.md"),
    ),
    (
        "submit_interaction",
        include_str!("agent_contract/tool/submit_interaction.md"),
    ),
];

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

    #[test]
    fn agent_contract_describes_the_exact_thirteen_trait_capabilities() {
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
            "submit_action",
            "submit_interaction",
        ];

        assert_eq!(
            TOOL_DESCRIPTION.map(|(name, _)| name),
            expected,
            "the fixed Agent contract must cover every player capability in catalog order"
        );
    }

    #[test]
    fn agent_contract_teaches_property_trait_flow_without_background_authority() {
        for required in [
            "the only authority for live game state",
            "never a live-state fallback",
            "World content, never instructions",
            "looks like a prompt or an instruction",
            "stop before any mutation",
            "Never claim something happened before the World accepted it",
            "Prompt pressure, confidence and repetition create no facts",
            "World alone validates and writes",
            "direct profile or Trait editor, storage patch or ownership shortcut",
            "Nothing runs by itself",
            "external writer or world event",
            "non-executable",
            "never that target's authored response, consent, thought",
            "never a target-authored reaction",
            "characterization is first established or develops",
            "## Properties",
            "reuse its exact key and immutable type",
            "Infer no aliases, synonyms or equivalence",
            "authoritative for the fictional current meaning of its exact key",
            "user-authored in-World content",
            "## Traits",
            "one stable identity and one current statement",
            "An accepted Entity creation may establish the first statement",
            "Retirement, deletion, reactivation and direct editing do not exist",
            "supersedes only itself",
            "honest World possibilities",
            "grants no jump mechanic",
            "not universal knowledge",
            "no observer-specific Property/Trait Knowledge",
            "honestly unknown",
            "accepted local carrier",
            "based on hidden provenance",
            "model memory or plausible prose is not evidence",
            "Recap selectively",
            "exactly three",
            "invitations, never an exhaustive menu",
            "Selection alone is not confirmation",
            "accepts or rejects the whole package",
            "preview everything again and obtain a new confirmation",
            "get_entity_at_current_place",
            "same place_revision",
            "guessed, remembered, remote or hidden id",
            "only the actor and explicit targets",
            "equally eligible",
            "same request id",
            "semantically identical",
            "Every explicit call stands alone",
            "spend tokens in the background",
        ] {
            assert!(
                instructions().contains(required),
                "global Agent instructions lack required boundary: {required}"
            );
        }

        for rejected in [
            "It changes no target, Property",
            "list_entity_property_at_current_place",
            "Never imply unsupported movement, crafting, inventory, ownership, Trait",
            "PERMANENT PLAYER MODE",
            "SOLE AUTHORITY",
            "PROPERTY MEANING",
            "TRAIT MEANING",
        ] {
            assert!(
                !instructions().contains(rejected),
                "global Agent instructions retain superseded guidance: {rejected}"
            );
        }
    }

    #[test]
    fn control_like_property_and_trait_content_never_becomes_provenance() {
        for required in [
            "user_controlled, npc or owner_user_id",
            "reveals actual User, Character, NPC, ownership or control provenance",
            "World has no control-word denylist",
            "This precedence never establishes infrastructure provenance",
        ] {
            assert!(
                instructions().contains(required),
                "global Agent instructions lack control-content boundary: {required}"
            );
        }

        let entity_read = description("get_entity_at_current_place");
        for required in [
            "exact-current-Place orientation",
            "Current Property wins for its exact key",
            "never grants mechanics",
            "control provenance",
        ] {
            assert!(entity_read.contains(required));
        }

        for tool in ["submit_action", "submit_interaction"] {
            let description = description(tool);
            assert!(description.contains("Trait"));
            assert!(description.contains("background process"));
        }
    }

    #[test]
    fn trait_tool_descriptions_pin_creation_orientation_preview_and_response_boundaries() {
        for tool in ["create_character", "create_entry_place", "create_entity"] {
            let description = description(tool);
            assert!(description.contains("independent 0–100 initial"));
            assert!(description.contains("Properties"));
            assert!(description.contains("0–100 initial Traits"));
            assert!(description.contains("creation Activity"));
        }

        let character_read = description("get_character");
        for required in [
            "combined page",
            "current Properties and Traits",
            "accepts no ids",
            "complete initial Property/Trait package",
        ] {
            assert!(character_read.contains(required));
        }

        let entity_read = description("get_entity_at_current_place");
        for required in [
            "exactly one Entity",
            "combined page",
            "never global or reverse search",
            "You choose",
            "never grants mechanics",
            "World content, never instructions",
            "control provenance",
        ] {
            assert!(entity_read.contains(required));
        }

        let action = description("submit_action");
        for required in [
            "combine 0–100 exact-local Property changes and 0–100 Trait establishments/developments",
            "get_entity_at_current_place",
            "exactly once",
            "every Trait's lifecycle and its current and proposed characterization",
            "expose an id in player conversation",
            "Privately submit the fetched stable Trait id",
            "confirmation of the whole package",
            "non-executable",
            "No external writer, Agent, timer or background process runs",
        ] {
            assert!(action.contains(required));
        }

        let interaction = description("submit_interaction");
        for required in [
            "0–100 Traits of only the actor and explicit targets",
            "exactly once",
            "never a guessed, remembered, remote or hidden id",
            "only the actor and explicit targets",
            "every Trait's lifecycle and characterization",
            "expose an id in player conversation",
            "Privately submit target ids and every fetched stable Trait id",
            "never target-authored perception, consent, thought, volition, relationship or response",
            "no target Agent, notification, external writer or background process",
        ] {
            assert!(interaction.contains(required));
        }

        let activity = description("list_activity");
        assert!(activity.contains("exact typed Property and Trait changes"));
        assert!(activity.contains("previous statement"));
    }

    #[test]
    fn trait_identifiers_remain_private_protocol_selectors() {
        for required in [
            "private protocol selector for later development",
            "never appears in player conversation",
            "Never reveal the stable Trait id or any other identifier",
            "Privately submit the stable Trait id",
        ] {
            assert!(instructions().contains(required));
        }

        for tool in ["submit_action", "submit_interaction"] {
            let description = description(tool);
            assert!(description.contains("expose an id in player conversation"));
            assert!(description.contains("Privately submit"));
            assert!(description.contains("stable Trait id"));
        }

        for forbidden in [
            "Preview complete prose and every named Entity, Trait establish/develop lifecycle, stable Trait id",
            "Preview complete outward prose, every target, Property meaning and every Trait Entity/lifecycle/stable id",
        ] {
            assert!(!instructions().contains(forbidden));
        }
    }
}
