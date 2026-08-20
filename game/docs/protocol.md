# Protocol contract

> **Role / side:** Defines request context, retry identity, freshness, HTTP/MCP behavior and canonical errors / runtime side.
> **Authority:** Cross-adapter delivery semantics for the nineteen player capabilities.
> **Excludes:** response shapes — defined in [Wire](wire.md); delivery status and evidence — recorded in `dev/docs/evidence/`.

## Request context

Context-required operations receive exactly one `Aicadia-User-Id` UUID header. It is
untrusted development context, not authentication. Missing, malformed, duplicate,
comma-joined or unknown values are rejected before game behavior succeeds. Capability
input never accepts a User id. Character and contextual Place operations also accept
no Character id. `enter_world`, `submit_action` and `submit_interaction` accept no
Place id; World derives the exact Place from the current Character.
`start_investigation` and `submit_discovery` likewise accept no Character selector;
start derives Position and optional current Place, while discovery may explicitly
select existing origin/destination Places only inside its connected-Place result.
`move_character` accepts no Character or Place selector. Spatial reads accept only
their exact box or Place/Connection anchors. The local
scoped Entity read accepts exactly one `entity_id` and no role, User, Character or
Place selector; `get_character` accepts no Entity selector.

MCP supports only the current `2026-07-28` revision. Every stateless request carries
its protocol version, client information and capabilities; Aicadia creates no
transport session. Current tool catalogs carry public cache metadata with `ttlMs: 0`.
Older revisions and `initialize`-session flows are unsupported.

## Delivery identity and exact-Place freshness

`request_id` is an Agent-generated UUID for one intended Action, Interaction,
discovery or Movement. It
remains stable only across uncertain delivery retries and must not be reused for a
different mutation.
World derives a versioned SHA-256 `request_fingerprint` from a length-prefixed
encoding of the normalized request. Action fingerprints include the expected Place
revision, prose, consequence meaning and Entity introduction fields including
optional Position description, or combined
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
unplaced; `submit_action`; `submit_interaction`; and
`submit_discovery.entity_at_position` with a current Place lock the existing Place,
insert their Activity, then
atomically point that Place to the inserted Activity in the same transaction. A
failure rolls back both Activity and pointer change. Mutations at different Places
remain concurrent. Reads issue no nonce, reservation or preparation record, and no
global revision or counter exists.

Connected-Place discovery and Movement neither depend on nor advance this broad
pointer. Their freshness is the exact attempt Position/current Place or submitted
Position revision. This spatial contract does not remove the older pointer from
local Action, Interaction or Entity-discovery behavior.

### Position freshness and Movement delivery identity

Every complete Position output carries one opaque `position_revision` encoding the
subject Entity and establishing Activity. It is current only when that exact version
is still the Entity's current Position. It is not a Place revision, permission,
timestamp ordering, coordinate hash or global revision.

`move_character.request_id` uses the shared Activity namespace. Its fingerprint
includes Connection id, expected Position revision, direction and the complete
tagged target. Same operation and fingerprint returns the stored canonical result
before current Character checks. Different content—or reuse of that request id by
Action, Interaction or discovery—returns `movement_request_conflict`.

For an unseen id, World locks User then Character, checks the current Position
revision, reads the immutable Connection and validates exact progress. A stale token
returns `position_revision_conflict`; a bounded lock or statement timeout returns
`temporarily_unavailable`. Neither writes Activity or Position. Accepted Movement
appends Activity and a new Position version in one transaction.

### Investigation retry identity

`start_investigation.request_id` is an Agent-generated UUID in the attempt namespace.
The other semantic input is `kind`; World stores it directly and needs no request
fingerprint. Under the User lock, an existing `(requested_by_user_id, request_id)`
attempt with the same kind returns its stored id, outcome and immutable limit before
admission or another roll. Another kind returns
`investigation_request_conflict`. The response contains no mutable Position or Place
context, so equal retries remain byte-identical after unrelated World changes.

For a new id, World derives the entered Character, exact Position revision and
optional current Place, uses one PostgreSQL `statement_timestamp()` for the inclusive
rolling-hour admission boundary and stored `created_at`, reads the bounded recent
Place Activity window or uses `n = 0` without a current Place, rolls once and inserts
the attempt. Rate rejection occurs before rolling and inserts nothing. Only a newly inserted
positive that takes the User beyond the live-positive bound voids the oldest prior live
positive satisfying `id <> new_attempt_id`, ordered by `(created_at ASC, id ASC)`, with the
new attempt as provenance. The new attempt can never void itself. Zero never triggers
voiding. Chance and admission values live in [Investigation attempt](model/investigation-attempt/README.md#investigation-chance-and-admission).

The attempt namespace is separate from the shared Activity request-id namespace. A
UUID may therefore identify one start attempt and, separately, one state-changing
Activity request without conflict.

### Discovery delivery identity

`submit_discovery.request_id` shares the Activity namespace with Action, Interaction
and Movement. Before the User lock, strict decoding and complete normalization reject
malformed or semantically invalid prose, Entity, Place, Position, Connection,
Property and Trait input and derive a versioned SHA-256 fingerprint from attempt id,
prose and the complete tagged result. Property and Trait sets normalize as elsewhere;
Connection course order remains significant. Tagged branches and item counts prevent
content from shifting between fields or result kinds.

Under the User lock, World first looks up an accepted Activity with the same
`(requested_by_user_id, request_id)`. The same operation and fingerprint return the
canonical tagged accepted result before later Character or attempt
preconditions. Different content—or reuse of that Activity request id across Action,
Interaction, discovery or Movement—returns `discovery_request_conflict`. Only an unseen id
continues to Character lookup, neutral attempt availability and database-dependent
find validation.

An available attempt is own, positive, unconsumed, unvoided, same kind and binds the
same Character, exact Position revision and nullable current Place still current at
settlement. A well-formed foreign, zero, consumed, voided, wrong-kind, unentered,
moved or changed-current-Place attempt returns `discovery_attempt_unavailable`
without distinguishing why. No Position or Place revision is submitted separately;
unrelated exact-Place activity does not stale the stored grounding.

Wire response shapes and their compact-versus-complete boundaries — defined in
[wire shapes](wire.md#wire-shapes); this protocol adds delivery, transport and error
semantics only.


## MCP publication invariants

Every tool declares `destructiveHint: false` and `openWorldHint: false`.
`submit_action`, `submit_interaction`, `submit_discovery` and `move_character` are nevertheless
irreversible World history and their capability descriptions state the confirmation
requirement. `start_investigation` stores internal attempt provenance but requires no
confirmation because it creates no player-visible World change. Exact descriptions,
JSON Schemas and annotations are compiler-generated and pinned by
`game/mcp/tool-catalog.json`; after an accepted contract change only the ignored
test `regenerate_agent_tool_catalog_fixture` rewrites that pin, and its diff is reviewed.

## Shared capability inputs

HTTP `enter_world` has no request body; MCP supplies the required empty object.
`get_world` and `get_user` likewise use empty MCP input. `get_character` accepts only
optional current-state `cursor` and `limit`. `get_entity_at_current_place` requires
one `entity_id` and accepts optional current-state `cursor`/`limit`. Entity and
Activity lists accept optional `cursor` and `limit`. `list_place` requires six box
bounds; its cursor binds that box. `list_connection` requires one Place id and
accepts cursor/limit; `get_connection` requires Place and Connection ids. Every page
limit defaults to 25 and
must be 1 through 100. Each cursor is opaque and tied to its operation. Clients copy
`next` unchanged and must not decode, edit or reuse it across list operations.

Current-state associations form one tagged page: Properties sort before Traits,
then by internal Property-key id or stable Trait id. Its cursor binds the selected
Entity, that Entity's nullable Position revision, nullable current Place
identity/revision and last typed sort key. Every continuation repeats the same
Entity and revisions. Changed Character Position rejects with
`position_revision_conflict`; changed Place state rejects with
`place_revision_conflict`; a no-longer-local selected Entity rejects neutrally.

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

`start_investigation` accepts exactly `request_id` and one kind:
`entity_at_position` or `connected_place`. `submit_discovery` accepts exactly
`request_id`, `attempt_id`, prose and one same-kind tagged `result`. Discovery prose
follows Action prose bounds. New Entity/Place branches contain name, description,
optional Position description and independent 0–100 initial Property and 0–100
initial Trait lists. New destination Position carries exact coordinates; new loose
origin reuses the attempt point. Existing branches carry one Place id. Connection
input carries name, description, optional shape description, reverse permission and
0 or 2–128 ordered exact points.

Every coordinate is an integer in the Position range. A Place box is inclusive,
orders each minimum before its maximum and spans at most `100_000_000` centimetres
per axis. `move_character` accepts request id, Connection id, expected Position
revision, direction and one complete/partial tagged target; it accepts no prose,
Character, Place, time, cost or terrain input.

## HTTP contract

- Reads return `200 OK`.
- `create_character`, `create_entry_place`, `create_entity`, `submit_action`,
  `submit_interaction` and `submit_discovery` return `201 Created`; an equal delivery
  retry for a confirmed mutation returns the same status and canonical body.
- `move_character` returns `200 OK` on first acceptance and equal delivery retry.
- `start_investigation` returns `200 OK` for both `zero` and `positive`, including an
  equal retry.
- `enter_world` returns `200 OK` on first acceptance and delivery retries.
- JSON/query decoding failures and unknown fields return canonical
  `invalid_request` errors.
- `GET /api/openapi.json` publishes exactly the nineteen player
  operation IDs above with shared schemas and no provisioning or operator reads.

The server binds only to loopback. MCP accepts an absent `Origin` for non-browser
clients, accepts the server's exact local origin, and rejects foreign origins.

## Canonical errors

```text
Error { code, message, field?: string, reason?: string }
```

| Code | Meaning | HTTP |
| --- | --- | --- |
| `user_context_required` | context header absent | `400` |
| `invalid_request` | malformed header, body, query, id, cursor or opaque Place/Position revision | `400` |
| `invalid_entity` | Entity semantic text invalid | `400` |
| `invalid_character` | Character semantic text invalid | `400` |
| `invalid_place` | Place semantic text invalid | `400` |
| `invalid_position` | coordinate or Position description invalid | `400` |
| `invalid_place_window` | box ordering, coordinate or span invalid | `400` |
| `invalid_connection` | Connection text, endpoint or course shape invalid | `400` |
| `invalid_movement` | Movement direction, tag, ordinal or target input invalid | `400` |
| `invalid_action` | action prose or consequence text invalid | `400` |
| `invalid_interaction` | Interaction prose or target count invalid | `400` |
| `invalid_discovery` | discovery prose invalid | `400` |
| `invalid_property` | Property list bound, key, value, tag or duplicate key/Entity-key invalid | `400` |
| `invalid_trait` | Trait list/lifecycle/statement invalid, exact duplicate or unchanged development | `400` |
| `invalid_entity_limit` | Entity limit outside 1 through 100 | `400` |
| `invalid_activity_limit` | Activity limit outside 1 through 100 | `400` |
| `invalid_place_limit` | Place limit outside 1 through 100 | `400` |
| `invalid_connection_limit` | Connection limit outside 1 through 100 | `400` |
| `user_not_found` | contextual User absent | `404` |
| `entity_not_found` | selected Entity absent | `404` |
| `character_not_found` | contextual User owns no Character | `404` |
| `place_not_found` | selected shared Place absent | `404` |
| `connection_not_found` | selected Connection absent or not incident to the anchor Place | `404` |
| `entry_place_not_found` | World genesis has not established an entry Place | `404` |
| `character_already_exists` | contextual User already owns a Character | `409` |
| `character_already_entered` | operation requires an unplaced Character | `409` |
| `character_not_entered` | contextual capability requires a Character Position but World entry has not occurred | `409` |
| `character_not_at_place` | entered Character is currently between Places but capability requires a current Place | `409` |
| `entry_place_already_exists` | World already has its one entry Place | `409` |
| `action_request_conflict` | request id was accepted with different normalized content | `409` |
| `interaction_request_conflict` | request id was accepted with different normalized Interaction content | `409` |
| `discovery_request_conflict` | Activity request id was accepted with different normalized discovery content or another operation | `409` |
| `movement_request_conflict` | Activity request id was accepted with different normalized Movement content or another operation | `409` |
| `investigation_request_conflict` | attempt request id was accepted for the other Investigation kind | `409` |
| `discovery_attempt_unavailable` | attempt is foreign, zero, consumed, voided, wrong-kind or no longer matches Character Position/current Place; no distinction is exposed | `409` |
| `place_unavailable` | selected discovery Place is absent, not a Place, stale or spatially ineligible; no distinction is exposed | `409` |
| `connection_unavailable` | Movement Connection is absent, stale or does not contain the Character's exact course position; no distinction is exposed | `409` |
| `connection_direction_disallowed` | selected Connection does not allow submitted direction | `409` |
| `movement_off_course` | Character or target point is not on the submitted exact course segment | `409` |
| `movement_no_progress` | target does not make strict progress in the submitted direction | `409` |
| `interaction_target_unavailable` | one or more submitted targets are duplicated, self, absent, remote or no longer co-present; no distinction is exposed | `409` |
| `property_entity_unavailable` | one or more Property subjects are absent, remote, departed or ineligible for this Action/Interaction; no distinction is exposed | `409` |
| `entity_at_current_place_unavailable` | selected scoped Entity is absent, remote, departed or otherwise ineligible; no distinction is exposed | `409` |
| `trait_unavailable` | selected Trait/owning Entity is absent, remote, departed, stale or otherwise ineligible; no distinction is exposed | `409` |
| `property_key_conflict` | canonical key already exists with another immutable value type | `409` |
| `place_revision_conflict` | exact current Place representation changed after the read | `412` |
| `position_revision_conflict` | Character Position changed after the read | `412` |
| `investigation_not_admitted` | per-User rolling admission window is full; no attempt or roll occurred | `429` |
| `temporarily_unavailable` | bounded spatial statement or lock budget expired; exact retry is safe | `503` |
| `unavailable` | World storage could not complete the request | `503` |

`invalid_place_window` identifies the first failing bound in `field` and reports
`out_of_range`, `before_minimum`, or `span_too_wide` in `reason`.

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
every mutation error rolls back its complete Action/Interaction/discovery package.

Malformed discovery JSON or UUIDs use `invalid_request`. Invalid prose uses only
`invalid_discovery`; result fields retain `invalid_entity`, `invalid_place`,
`invalid_position`, `invalid_connection`, `invalid_property`, `invalid_trait` and
`property_key_conflict`. Typed normalization happens before
locking, while equal accepted retry/conflict resolution precedes Character and
attempt availability. A missing Character uses `character_not_found`; start on a
Character without Position uses `character_not_entered`, while submit with changed
Position/current Place uses neutral `discovery_attempt_unavailable`. A well-formed
existing origin/destination selector that cannot be used returns neutral
`place_unavailable`. Every rejected submit leaves attempt lifecycle, Entity, Place,
Position, Connection, Activity and Place pointer unchanged.

Malformed Place box, Connection or Movement JSON and ids use `invalid_request`.
Semantic box bounds use `invalid_place_window`; semantic Connection content uses
`invalid_connection`. Anchored Connection read hides absent versus non-incident with
`connection_not_found`. Movement hides absent versus spatially ineligible with
`connection_unavailable`, then distinguishes a returned Connection's disallowed
direction, off-course target and non-progress target. A stale Position revision uses
`position_revision_conflict`. `temporarily_unavailable` means only the local
statement or lock budget expired; it writes nothing and the exact same request id and
input is safe to retry.

MCP game failures are successful JSON-RPC tool responses with `isError: true` and one
text content block containing the same error object. Protocol framing, unknown tools,
unsupported versions and origin rejection remain MCP protocol errors outside this
game error contract.

Proof obligations are owned by the [adapter parity contract](adapter-parity.md).
