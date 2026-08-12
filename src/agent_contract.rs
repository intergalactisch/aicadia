use rmcp::handler::server::router::tool::ToolRouter;

pub(crate) const INSTRUCTIONS: &str = include_str!("agent-play-contract.txt");

const GET_WORLD: &str = "Get the identity of the one persistent shared Aicadia World. No User context is required. Use the result privately to ground play; if its name matters, speak of the World by name without mentioning this tool or an identity record.";

const GET_USER: &str = "Get the durable User represented by this request's Aicadia-User-Id context. This tool accepts no User id and does not authenticate the caller. Use the result privately and never present its id, fields or provisioning record in player conversation.";

const GET_CHARACTER: &str = "Read the Character owned by the current User before onboarding or orientation. The embedded Entity id is also the Character id; there is no separate Character id. current_place is the complete current Place or null when the Character exists but has not entered the World. character_not_found is the only trigger for the three-candidate Character workshop. An existing Character must never be recreated. This tool derives the User from request context and accepts no ids. Render the result as the named person and the named location where they stand; express no current Place as a natural fact without category, field or null language.";

const CREATE_CHARACTER: &str = "Create the current User's one unplaced Character Entity role. Use only after get_character returned character_not_found, private conversation offered exactly three complete candidates, the User selected or freely steered one, the complete final meaning was presented naturally in the User's language, and the User explicitly confirmed it. Selection alone is not confirmation. If the meaning changes, present and confirm it again. Privately submit the semantically identical English name and description once; never display raw English payload text, JSON, field labels or delivery work. The private workshop is not World state and cannot be proven by the server. Creation returns current_place null; use enter_world separately. This tool derives the User from request context and accepts no ids. After acceptance, speak only of the new person by name and say naturally that they have not entered the World yet.";

const CREATE_ENTRY_PLACE: &str = "Establish World genesis by creating the one shared entry Place. Use only for an existing unplaced Character after enter_world returns entry_place_not_found. Privately supply one semantic English name and description; World derives the Place Entity id. Exactly one concurrent request wins. If entry_place_already_exists is returned, another Agent established genesis: call enter_world again rather than proposing another Place. This never creates later Places. Render success as the named location becoming the beginning of the shared World, without creation-operation, record or id language.";

const ENTER_WORLD: &str = "Enter the shared World with the current unplaced Character at the one World-derived entry Place. Use directly after obtaining or creating a Character whose current_place is null. If entry_place_not_found is returned, establish genesis with create_entry_place and retry. This accepts no ids or Place selector. Retrying successful entry returns the same complete Character and Place without another Activity. Render the result as the named person's arrival at the named location and keep entry operations and record categories private.";

const LIST_ACTIVITY: &str = "List immutable accepted World actions involving the current Character from newest to oldest. World derives the Character from User context. actor_character identifies who acted, context_place preserves where acceptance occurred, and involved_entity identifies each subject, destination or location. Canonical submit_action prose is present only on that Activity. Null actor, Place or prose means that context or value did not exist. limit defaults to 25 and must be 1 through 100; copy next unchanged into cursor for the following page. Render relevant results as named events in natural time order; never expose Activity, role, field, null, cursor or pagination language.";

const LIST_ENTITY: &str = "List shared Entities from newest to oldest. limit defaults to 25 and must be 1 through 100. Copy next unchanged into cursor for the following page; do not interpret the cursor. Render only relevant results as named people, locations or things, never as Entity records, ids, cursors or pages.";

const GET_ENTITY: &str = "Get one shared Entity by its stable Entity id. Treat the returned typed fields as authoritative; its prose description does not grant unmodeled mechanics. Render the result as the named person, location or thing and state only established qualities and current affordances; never expose the Entity category, id or fields.";

const CREATE_ENTITY: &str = "Create one shared Entity for a stable referent the current User intends to introduce. Use only when later participants must refer to the same subject. This does not assert fictional creation, ownership or discovery, and repeating it creates another Entity. Acceptance also appends create_entity Activity; its actor and context Place are present when the current Character exists and is placed. Privately supply semantically identical English World content. After acceptance, describe the named person, location or thing as an established in-World fact without operation, record or field language, and imply no mechanics beyond returned typed state.";

const LIST_ENTITY_AT_CURRENT_PLACE: &str = "List Entities explicitly placed at the exact current Place derived from the current User's entered Character. Use this to ground orientation and action proposals. The response includes the complete Place and its opaque place_revision. limit defaults to 25 and must be 1 through 100; copy next unchanged into cursor for the following page. This accepts no User, Character or Place ids and does not imply distance, visibility, ownership or a neighborhood. Render the Place by name and its relevant contents as named people or things; keep category, field, revision, cursor and pagination details private.";

const LIST_ACTIVITY_AT_CURRENT_PLACE: &str = "List canonical Activity at or involving the exact current Place derived from the current User's entered Character, newest first. Use this to ground orientation and action proposals. Activity includes immutable submit_action prose when present. The response includes the complete Place and its opaque place_revision. limit defaults to 25 and must be 1 through 100; copy next unchanged into cursor for the following page. This accepts no User, Character or Place ids. Typed roles and consequences are authoritative; prose adds no unmodeled state. Render relevant results as events involving named people, locations and things; keep Activity, roles, fields, nulls, revisions, cursors and pagination private.";

const SUBMIT_ACTION: &str = "Ask World to atomically accept one previously grounded and explicitly confirmed action at the derived current Place. Before calling, privately use separate get_world, get_character, list_entity_at_current_place and list_activity_at_current_place reads with agreeing place_revision values; offer exactly three grounded directions; and accept selection and free steering. Then present the complete intended action as an in-World passage in the User's language, including the full meaning of the one introduce_entity name and description, and obtain explicit confirmation of that complete change. Selection alone is not confirmation. Privately submit semantically identical English prose, name and description without displaying raw English transport text or payload labels. Call exactly once with a fresh request_id and unchanged observed place_revision; supply no User, Character, Entity or Place id. Retry only uncertain delivery with the same id and byte-equivalent input. On place_revision_conflict, do not mutate automatically: re-read, explain naturally that the situation changed and reconfirm a newly grounded change with a new request_id. After acceptance, narrate the event and introduced subject by name without exposing the operation, consequence type, records, ids or delivery details.";

const TOOL_DESCRIPTION: [(&str, &str); 13] = [
    ("get_world", GET_WORLD),
    ("get_user", GET_USER),
    ("get_character", GET_CHARACTER),
    ("create_character", CREATE_CHARACTER),
    ("create_entry_place", CREATE_ENTRY_PLACE),
    ("enter_world", ENTER_WORLD),
    ("list_activity", LIST_ACTIVITY),
    ("list_entity", LIST_ENTITY),
    ("get_entity", GET_ENTITY),
    ("create_entity", CREATE_ENTITY),
    ("list_entity_at_current_place", LIST_ENTITY_AT_CURRENT_PLACE),
    (
        "list_activity_at_current_place",
        LIST_ACTIVITY_AT_CURRENT_PLACE,
    ),
    ("submit_action", SUBMIT_ACTION),
];

pub(crate) fn apply<S>(router: &mut ToolRouter<S>) {
    assert_eq!(
        router.map.len(),
        TOOL_DESCRIPTION.len(),
        "the Agent contract must describe the complete fixed tool catalog"
    );
    for (name, description) in TOOL_DESCRIPTION {
        let route = router
            .map
            .get_mut(name)
            .unwrap_or_else(|| panic!("the Agent contract references missing tool {name}"));
        route.attr.description = Some(description.into());
    }
}
