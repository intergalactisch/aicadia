# Protocol contract

> **Role / side:** Defines request context, wire shapes, retry identity, freshness, HTTP/MCP behavior and canonical errors / runtime side.
> **Authority:** Cross-adapter delivery semantics for the thirteen player capabilities.
> **Excludes:** Delivery status, rollout narrative and evidence results.

## Request context

Context-required operations receive exactly one `Aicadia-User-Id` UUID header. It is
untrusted development context, not authentication. Missing, malformed, duplicate,
comma-joined or unknown values are rejected before game behavior succeeds. Capability
input never accepts a User id. Character and contextual Place operations also accept
no Character id. `enter_world`, `submit_action` and `submit_interaction` accept no
Place id; World derives the exact Place from the current Character. The local
scoped Entity read accepts exactly one `entity_id` and no role, User, Character or
Place selector; `get_character` accepts no Entity selector.

MCP supports only the current `2026-07-28` revision. Every stateless request carries
its protocol version, client information and capabilities; Aicadia creates no
transport session. Current tool catalogs carry public cache metadata with `ttlMs: 0`.
Older revisions and `initialize`-session flows are unsupported.


## Delivery identity and exact-Place freshness

`request_id` is an Agent-generated UUID for one intended Action or Interaction. It
remains stable only across uncertain delivery retries and must not be reused for a
different mutation.
World derives a versioned SHA-256 `request_fingerprint` from a length-prefixed
encoding of the normalized request. Action fingerprints include the expected Place
revision, prose, consequence meaning and Entity introduction fields or combined
Property/Trait changes. Empty-initial-Trait introductions and single-kind current
state changes retain their historical semantic fingerprints; combined state and
non-empty initial Trait input use an unambiguous length-prefixed extension. Stored
historical Action tags decode to the current result shape but are not public inputs.
Interaction fingerprints include the expected Place revision,
prose, target Entity ids sorted by UUID bytes, Property changes and typed Trait
changes. Initial Property lists
sort by canonical key; change lists sort by normalized `(entity_id,key,type,value)`.
Trait change lists sort typed establish items by `(entity_id,statement)` and develop
items by `(trait_id,statement)`.
List order therefore cannot change retry identity. Fingerprints never hash raw JSON
or depend on field order.

All exact-Place reads return the same opaque `place_revision` when they observed the
same Place representation. Each Place stores one authoritative
`latest_activity_id`, pointing to the Activity most recently accepted as relevant to
that Place under its serialization lock. Internally the revision identifies the
Place and that target Activity as `(occurred_at, activity_id)`. Those Activity fields
identify the pointed-to record; timestamp comparison, UUID order and
`MAX(occurred_at, activity_id)` never determine which Activity was accepted latest.
The revision is a strong freshness validator for this representation, not an
authorization token, request id, global World version or Agent-built hash. Clients
copy the versioned URL-safe token unchanged. Each read uses one short read-only
Repeatable Read transaction so its derived Character, Place, pointer target, page
and revision form one per-call snapshot; no database transaction or durable session
spans Agent calls.

In one mutation transaction World locks the User and first looks up an accepted
`(requested_by_user_id, request_id)`. Equal operation and fingerprint return the canonical stored
result even if Character or Place state later changed. Different fingerprint returns
the operation's request-conflict result. For an unseen id, World derives and locks the Character's
current Place and compares `expected_place_revision` with the current revision before
the remaining validation and writes. A changed representation returns
`place_revision_conflict` and writes nothing. Activity at another Place does not
invalidate the token.

Every writer that changes this exact-Place representation takes the same Place lock
before acceptance. `create_entry_place` assigns its preallocated genesis Activity as
`latest_activity_id` when it inserts the new Place. `enter_world`, `create_entity`
when its acting Character is currently placed, although the new Entity remains
unplaced; `submit_action`; and `submit_interaction` lock the existing Place, insert
their Activity, then
atomically point that Place to the inserted Activity in the same transaction. A
failure rolls back both Activity and pointer change. Mutations at different Places
remain concurrent. Reads issue no nonce, reservation or preparation record, and no
global revision or counter exists.


## Wire shapes

All JSON objects reject unknown fields. Successful operations return the result
directly without a `data` envelope. Timestamps are RFC 3339 strings, ids are UUID
strings, and revisions/cursors are opaque URL-safe strings.

```text
World       { name }
User        { id, created_at }
Entity      { id, name, description, introduced_by_user_id, introduced_at }
Place       { entity: Entity, is_entry }
Character   { entity: Entity, owner_user_id, current_place: Place | null }
EntitySummary { id, name }
CurrentPlaceEntityOutput { id, name, description }
CurrentPlaceOutput { id, name, description }
PlaceSummary  { entity: EntitySummary, is_entry }
PropertyValue { type: "text", text } | { type: "integer", integer }
EntityProperty { entity: EntitySummary, key, value: PropertyValue }
EntityTrait { id, statement }
EntityCurrentAssociation =
  { type: "property", property: { key, value: PropertyValue } } |
  { type: "trait", trait: EntityTrait }
EntityCurrentStatePage {
  association: [EntityCurrentAssociation],
  next: string | null
}
CharacterEntityStatePage {
  character: Character,
  place_revision: string | null,
  current_state: EntityCurrentStatePage
}

ActivityEntityReference {
  entity: EntitySummary,
  role: "subject" | "destination" | "location" | "target"
}
Activity {
  id,
  operation: "create_character" | "create_entity" |
             "create_entry_place" | "enter_world" | "submit_action" |
             "submit_interaction",
  actor_character: EntitySummary | null,
  context_place: PlaceSummary | null,
  involved_entity: [ActivityEntityReference],
  property_change: [EntityProperty],
  trait_change: [ActivityTraitChange],
  prose: string | null,
  occurred_at
}
EntityPage   { entity: [EntitySummary], next: string | null }
ActivityPage { activity: [Activity], next: string | null }
CurrentPlaceEntityPage {
  place: CurrentPlaceOutput,
  place_revision: string,
  entity: [CurrentPlaceEntityOutput],
  next: string | null
}
CurrentPlaceActivityPage {
  place: CurrentPlaceOutput,
  place_revision: string,
  activity: [Activity],
  next: string | null
}
CurrentPlaceEntityStatePage {
  place: CurrentPlaceOutput,
  place_revision: string,
  entity: CurrentPlaceEntityOutput,
  current_state: EntityCurrentStatePage
}
AcceptedAction {
  activity: Activity,
  consequence:
    { type: "introduce_entity", entity: Entity } |
    { type: "change_entity_state",
      property_change: [EntityProperty],
      trait_change: [ActivityTraitChange] },
  place: Place
}
AcceptedInteraction { activity: Activity, place: CurrentPlaceOutput }
```

`CurrentPlaceOutput` is deliberately the flat safe current-Place view: it contains
only the Place Entity's id, name and description. Unlike `Place`, it exposes neither
the complete Entity provenance nor `is_entry`. Current-Place pages and accepted
Interactions use this safe view; Character, entry and Action results continue to use
the complete `Place` shape where their contract requires it.

An Activity `location` role names the Place where that accepted Activity happened;
it is not limited to establishing a `subject`. A `target` role means only that the
Interaction actor directed the accepted outward behavior toward that Entity. It
never establishes the target's perception, consent, agreement, thought or response.

`requested_by_user_id`, accepted request id and fingerprint are internal Activity
provenance and are not exposed by history reads.

`EntityProperty` contains a safe Entity summary, canonical key and one tagged typed
value; no internal Property-key id, role, owner or control provenance is exposed.
Activity `property_change` is always present, sorted by Entity id then key, and empty
when that Activity changed none. In a current-state association page, Properties
sort before Traits, then by internal Property-key id or stable Trait id.

`ActivityTraitChange` is
`{type:"establish", entity:EntitySummary, trait:EntityTrait}` or
`{type:"develop", entity:EntitySummary, trait:EntityTrait, previous_statement}`.
Activity `trait_change` is always present, deterministically sorted and empty when
none changed; rows sort by owning Entity id, stable Trait id and lifecycle tag.
Activity, creation and mutation Entity/Place values remain compact
references/acknowledgements; only `get_character` and
`get_entity_at_current_place` are full player Entity fetches with current association
pages.


## MCP publication invariants

Every tool declares `destructiveHint: false` and `openWorldHint: false`.
`submit_action` and `submit_interaction` are nevertheless irreversible World history
and their capability descriptions state the confirmation requirement. Exact descriptions,
JSON Schemas and annotations are compiler-generated and pinned by
`tests/agent-tool-catalog.json`.

## Shared capability inputs

HTTP `enter_world` has no request body; MCP supplies the required empty object.
`get_world` and `get_user` likewise use empty MCP input. `get_character` accepts only
optional current-state `cursor` and `limit`. `get_entity_at_current_place` requires
one `entity_id` and accepts optional current-state `cursor`/`limit`. Entity and
Activity lists accept optional `cursor` and `limit`; every limit defaults to 25 and
must be 1 through 100. Each cursor is opaque and tied to its operation. Clients copy
`next` unchanged and must not decode, edit or reuse it across list operations.

Current-state associations form one tagged page: Properties sort before Traits,
then by internal Property-key id or stable Trait id. Its cursor binds the selected
Entity, nullable current Place identity/revision and last typed sort key. Every
continuation repeats the same Entity and revision; changed state rejects with
`place_revision_conflict`, and a no-longer-local selected Entity rejects neutrally.

Interaction prose has the same semantic bounds as Action prose. The target list must
contain 1 through 100 distinct UUIDs, is semantically unordered and is normalized by
sorting UUID bytes for the request fingerprint. It accepts no User, actor, Place,
effective time, separate consequence object or target metadata.
`property_change` defaults to `[]`, contains 0–100 unique `(entity_id,key)` pairs and
may name only the actor or an explicit target.

Trait statements are trimmed non-NUL PostgreSQL `text` of 1–4,000 Unicode
characters. Creation `trait` contains 0–100 establishment statements. Action and
Interaction `trait_change` contain 0–100 typed establish/develop items and may
coexist with Property changes; `change_entity_state` requires at least one list item.
Duplicate lifecycle items and unchanged development reject. World also evaluates
the intended post-package active statement set per Entity: development into another
unchanged active statement, two developments to the same final statement and an
establishment plus development to the same final statement reject, while a statement
vacated by another development in the same unordered package may be reused. The
complete Action or Interaction rolls back, including any Interaction Property
changes; semantic near-duplicates and contradictions remain valid. Trait prose is
never executable.

Property keys contain 1–64 ASCII lower-snake-case characters, start with a letter
and match `^[a-z][a-z0-9]*(?:_[a-z0-9]+)*$`. Text values are trimmed non-NUL Unicode
of length 1–4,000; integer values are signed 64-bit integers. Initial and change
lists are semantically unordered. Same key/type reuses the canonical key; same
key/different type conflicts. World infers no aliases or synonyms.

## HTTP contract

- Reads return `200 OK`.
- `create_character`, `create_entry_place`, `create_entity`, `submit_action` and
  `submit_interaction` return `201 Created`; an equal delivery retry for either
  confirmed mutation returns the same status and canonical body.
- `enter_world` returns `200 OK` on first acceptance and delivery retries.
- JSON/query decoding failures and unknown fields return canonical
  `invalid_request` errors.
- `GET /api/openapi.json` publishes exactly the thirteen player
  operation IDs above with shared schemas and no provisioning or operator reads.

The server binds only to loopback. MCP accepts an absent `Origin` for non-browser
clients, accepts the server's exact local origin, and rejects foreign origins.

## Canonical errors

```json
{
  "error": {
    "code": "invalid_action",
    "message": "Action prose is empty.",
    "field": "prose",
    "reason": "empty"
  }
}
```

| Code | Meaning | HTTP |
| --- | --- | --- |
| `user_context_required` | context header absent | `400` |
| `invalid_request` | malformed header, body, query, id, cursor or opaque Place revision | `400` |
| `invalid_entity` | Entity semantic text invalid | `400` |
| `invalid_character` | Character semantic text invalid | `400` |
| `invalid_place` | Place semantic text invalid | `400` |
| `invalid_action` | action prose or consequence text invalid | `400` |
| `invalid_interaction` | Interaction prose or target count invalid | `400` |
| `invalid_property` | Property list bound, key, value, tag or duplicate key/Entity-key invalid | `400` |
| `invalid_trait` | Trait list/lifecycle/statement invalid, exact duplicate or unchanged development | `400` |
| `invalid_entity_limit` | Entity limit outside 1 through 100 | `400` |
| `invalid_activity_limit` | Activity limit outside 1 through 100 | `400` |
| `user_not_found` | contextual User absent | `404` |
| `entity_not_found` | selected Entity absent | `404` |
| `character_not_found` | contextual User owns no Character | `404` |
| `entry_place_not_found` | World genesis has not established an entry Place | `404` |
| `character_already_exists` | contextual User already owns a Character | `409` |
| `character_already_entered` | operation requires an unplaced Character | `409` |
| `character_not_entered` | contextual action or Place read requires a placed Character | `409` |
| `entry_place_already_exists` | World already has its one entry Place | `409` |
| `action_request_conflict` | request id was accepted with different normalized content | `409` |
| `interaction_request_conflict` | request id was accepted with different normalized Interaction content | `409` |
| `interaction_target_unavailable` | one or more submitted targets are duplicated, self, absent, remote or no longer co-present; no distinction is exposed | `409` |
| `property_entity_unavailable` | one or more Property subjects are absent, remote, departed or ineligible for this Action/Interaction; no distinction is exposed | `409` |
| `entity_at_current_place_unavailable` | selected scoped Entity is absent, remote, departed or otherwise ineligible; no distinction is exposed | `409` |
| `trait_unavailable` | selected Trait/owning Entity is absent, remote, departed, stale or otherwise ineligible; no distinction is exposed | `409` |
| `property_key_conflict` | canonical key already exists with another immutable value type | `409` |
| `place_revision_conflict` | exact current Place representation changed after the read | `412` |
| `unavailable` | World storage could not complete the request | `503` |

A malformed target UUID is `invalid_request`; an empty or over-100 target list is
`invalid_interaction`. Every well-formed but ineligible target set uses the one
neutral `interaction_target_unavailable` result and writes nothing. A malformed revision is `invalid_request` with field
`expected_place_revision`; a well-formed revision for an older or different Place is
`place_revision_conflict`. Semantic errors identify their exact field and reason.

Malformed Property JSON is `invalid_request`. Semantic key/value/list and duplicate
violations are `invalid_property` and write nothing. Every well-formed missing,
remote or otherwise ineligible Property subject uses neutral
`property_entity_unavailable`; World never discloses existence, role or control.
Same canonical key with another type uses `property_key_conflict`. All three reject
their complete enclosing operation atomically.

Malformed Trait JSON is `invalid_request`. Semantic statement/lifecycle/count and
exact duplicate/no-op or intended-final-active-set violations use `invalid_trait`.
A well-formed missing,
remote, departed, stale or ineligible Trait mutation uses neutral
`trait_unavailable`; a scoped Entity fetch uses neutral
`entity_at_current_place_unavailable`. None reveals existence, role or control, and
every mutation error rolls back its complete Action/Interaction package.

MCP game failures are successful JSON-RPC tool responses with `isError: true` and one
text content block containing the same error object. Protocol framing, unknown tools,
unsupported versions and origin rejection remain MCP protocol errors outside this
game error contract.

## Parity evidence

Automated tests require:

1. Executable catalog tests prove that `get_entity_at_current_place` replaces
   `list_entity_property_at_current_place` in the exact thirteen-name OpenAPI/MCP
   catalog; loopback operator Entity reads remain absent.
2. MCP descriptions, annotations and schemas equal the checked-in fixture.
3. Character creation and reads expose `current_place: null`, then both adapters
   expose the complete same entry Place after World entry.
4. Entry Place creation through one adapter is used by entry through the other.
5. HTTP and MCP personal and exact-Place pagination share typed opaque cursor and
   revision semantics.
6. Both adapters return the same canonical context, semantic, not-found, delivery
   conflict and freshness errors.
7. One adapter can submit an Interaction whose canonical Activity/prose and complete
   target set an explicit target reads through the other adapter, while a non-target
   bystander receives no Interaction and still sees ordinary same-Place trail-marker
   history.
8. Missing, fabricated, duplicate, self, remote and changed-Place targets share one
   neutral error and atomic rollback; equal retries normalize target order.
9. All four Entity-creation routes share the same independent optional 0–100 initial
   Property and Trait shapes and atomic result semantics across adapters.
10. One adapter can submit combined multi-Entity Action changes and optional
   actor/target Interaction changes whose sorted typed Activity history and current
   local values the other adapter reads without role/control leakage.
11. Property bounds, duplicates, key/type conflict, neutral Entity eligibility,
   retry normalization and full rollback have the same canonical HTTP/MCP errors.
12. Stateless MCP `2026-07-28` exposes all thirteen tools, the one global play
   contract and complete cache metadata without creating a transport session; older
   revisions fail closed.
13. All creation routes establish optional 0–100 initial Traits; mixed 0–100 Action
    establishment/development and optional 0–100 actor/target Interaction Trait changes preserve
    stable ids, immutable roots/versions/current pointers, exact Activity history,
    Property coexistence and atomic rollback. Deferred bounded per-Trait commit
    checks enforce exactly one root, exactly one current pointer and a pointer at the
    lineage tip, rejecting incomplete root/pointer state, current deletion or
    backtracking and successor-without-advance.
14. Statement bounds, exact duplicate/no-op and post-package active-set cases,
    same-package vacated-statement reuse, semantic contradiction, neutral
    eligibility, retry reconstruction, concurrency, branch prevention, deadlock and
    set-based 100-item query plans match across World/HTTP/MCP.
15. `get_character` and scoped `get_entity_at_current_place` return combined 1/100-
    association pages with exact cursor/Place-revision semantics; compact orientation,
    mutation acknowledgements and Activity Entity references never recursively carry
    current state.
16. Agent contract evidence proves full natural Trait preview, whole-package User
    confirmation/rejection, no direct Trait editor, no hidden post-confirm mutation
    and no executable interpretation of Trait prose.

## Cross-contract evidence obligations
- one accepted trail-marker package atomically writes one placed Entity, one Activity,
  canonical prose, exact actor, context, subject and location roles;
- every validation, storage and stale-revision failure rolls back every package row;
- equal request retries return the canonical result, changed content under one id
  conflicts, and accepted identity resolves before later Character/Place preconditions;
- exact-Place pages, pointer targets and revisions are consistent snapshots;
  same-Place writers serialize and advance the pointer even across equal timestamps
  or clock rollback, unrelated Places do not conflict, and malformed tokens are
  rejected;
- a second Character at the Place can read the marker and same Activity/prose;
- one actor can submit 1–100 distinct co-present targets as one Activity, a target
  can recall the canonical outward behavior and co-target set, a reverse response is
  a new Activity, and a non-target bystander receives no Interaction automatically;
- missing, fabricated, duplicate, self, remote and no-longer-present targets return
  the same neutral error and leave no Activity or partial target rows;
- equal Interaction retries ignore target order and return the canonical result,
  while changed content under the same request id conflicts;
- non-Interaction Place history, including trail markers, remains available to
  Characters currently there under the existing scoped-Place rule;
- the thirteen player capabilities have one semantic World/HTTP/MCP contract,
  strict schemas, complete catalog/OpenAPI publication and matching errors;
- the local launcher preserves one database and User across restart, refuses
  concurrent or unprofiled reuse that could create a second User, and never starts
  Codex itself; its printed adapter isolates workspace, home/configuration and
  transient conversation state while requiring current Aicadia MCP;
- the browser ledger uses only the four accepted GET reads, hides User UUIDs, remains
  responsive and keyboard-operable, and renders identical accepted ids and prose
  before and after restart; and
