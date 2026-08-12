# Aicadia current build contract

This document is the executable game authority. Aicadia has one persistent `World`,
durable `User` records, shared `Entity` records, at most one owned `Character` Entity
role per User, and zero or one shared entry `Place`. A Character may remain unplaced
or explicitly enter that Place. An entered Character may perform one trail-marker
action that adds readable prose and one Entity at its exact current Place. Accepted
game mutations append immutable normalized `activity` in the same PostgreSQL
transaction as current state.

## World interface

The concrete `World` type is the only public game-behavior seam. HTTP and MCP are
thin adapters over these operations:

```rust
get_world() -> WorldView
create_user() -> Result<User, WorldError>                 // provisioning only
get_user(user_id) -> Result<User, WorldError>
get_character(user_id) -> Result<Character, WorldError>
create_character(user_id, input) -> Result<Character, WorldError>
create_entry_place(user_id, input) -> Result<Place, WorldError>
enter_world(user_id) -> Result<Character, WorldError>
list_activity(user_id, input) -> Result<ActivityPage, WorldError>
list_entity(input) -> Result<EntityPage, WorldError>
get_entity(entity_id) -> Result<Entity, WorldError>
create_entity(user_id, input) -> Result<Entity, WorldError>
list_entity_at_current_place(user_id, input)
    -> Result<CurrentPlaceEntityPage, WorldError>
list_activity_at_current_place(user_id, input)
    -> Result<CurrentPlaceActivityPage, WorldError>
submit_action(user_id, input) -> Result<AcceptedAction, WorldError>
```

`create_user` is internal provisioning, not player-facing. The other thirteen
capabilities ship through both HTTP and MCP as specified in
[Agent interface](agent-interface.md). Authentication remains deferred; adapters
currently obtain an untrusted development `UserId` from `Aicadia-User-Id` for
contextual operations.

Each explicit call stands alone. There is no durable game session and no server-side
Agent invocation or inference. Agents may reason and propose, but only World assigns
identities, validates commands and writes durable state. MCP exposes one current,
provider-neutral Agent play contract and complete tool descriptions as specified in
[Agent interface](agent-interface.md); a host supplies those instructions to its own
model, while typed World results remain authoritative.

## Supported local play

Local development supports one persistent World and one stable hidden development
User through the launcher described in [Local play](local-play.md). The User plays
and completes Character onboarding only through their own Agent. A same-origin
browser page is a read-only ledger of the World identity, shared Entity records and,
once a Character exists, that Character's accepted Activity/prose. It uses only the
existing `get_world`, `list_entity`, `get_entity` and `list_activity` HTTP reads.

The launcher prints but never invokes one explicit local player-adapter command. The
adapter starts Codex with an empty external workspace and isolated transient home,
requires current Aicadia MCP and injects the exact permanent player contract. It
inherits no development repository, personal skill, extra MCP or durable
conversation context and removes its owned temporary root after exit. Other Agent
hosts are conforming only when they provide the equivalent provider-neutral
contract, required-authority and fail-closed boundaries in
[Agent interface](agent-interface.md).

The ledger has no chat, proposals, confirmation, mutation, model invocation or
dedicated User, Character or Place view. Character and Place may appear only as the
typed references already present in records. Its contextual
`Aicadia-User-Id` value is hidden transport context, not authentication. The page is
not a player-facing capability and adds no `World` operation, game-data endpoint or
MCP tool. Accepted World calls remain the only durable state and Activity; private
Agent conversation is never stored.

## Current state

### User

```rust
struct User {
    id: UserId,
    created_at: DateTime<Utc>,
}
```

A User is a durable participant and request-provenance subject. It is not an Entity,
Character, Place, account model or authenticated identity. Each User owns at most one
Character. `create_user` creates only a User and never writes game activity.

### Entity

```rust
struct Entity {
    id: EntityId,
    name: String,
    description: String,
    introduced_by_user_id: UserId,
    introduced_at: DateTime<Utc>,
}
```

An Entity is one durable World subject that later participants must be able to refer
to again. Names are display text, not identifiers, and are not unique. Entity has no
type, kind, taxonomy, ownership or discovery claim. `introduced_by_user_id` says who
introduced the record, not who fictionally created, owns or discovered the subject.

An ordinary Entity may have zero or one explicit current Place relation. Absence is
valid and is never inferred from prose or Activity. `create_entity` accepts only
`name` and `description`, derives the introducing User, always creates a new unplaced
Entity and returns the complete Entity. Equal input remains two Entities. When the
User already owns a Character, the activity records that Character as actor and its
current Place as context when present; otherwise actor and context are absent.

The first `submit_action` consequence also creates an Entity, but World places that
Entity at the acting Character's derived current Place in the same transaction. The
bundle and `create_entity` reuse private validation and insertion behavior; one
public capability never invokes the other.

### Character

```rust
struct Character {
    entity: Entity,
    owner_user_id: UserId,
    current_place: Option<Place>,
}
```

A Character is a User-owned Entity role. `character.entity_id` is both its primary
key and Entity foreign key; there is no Character surrogate id or copied Entity
state. `create_character` accepts only `name` and `description`, derives the owner,
and atomically creates Entity, Character and activity. Concurrent creates for one
User yield exactly one Character without an orphan Entity.

Character creation always leaves `current_place` absent. Absence means the Character
exists but has not entered the World; it is not a missing lookup or unknown
coordinate. `get_character` returns the complete current Place when present.

### Place and World entry

```rust
struct Place {
    entity: Entity,
    is_entry: bool,
}
```

A Place is an Entity role whose stable identity is `place.entity_id`. Zero entry
Places is valid before genesis; at most one row may have `is_entry = true`.

`create_entry_place(user_id, {name, description})`:

- derives an existing Character from User context and accepts no ids;
- requires that Character to be unplaced;
- uses the same trimming and semantic bounds as Entity creation;
- lets the first connected Agent author the entry Place name and description;
- atomically creates Entity, Place and activity;
- permits exactly one winner under concurrent requests and leaves no orphan Entity.

This is World genesis, not discovery. A second entry Place is rejected.

`enter_world(user_id)` accepts no payload, Character id or Place id. World derives
the current Character and the entry Place, then atomically sets
`character.current_place_entity_id` only when it is absent and writes one activity.
A Character may remain unplaced indefinitely. Retrying or racing a successful entry
returns the same Character and does not append another activity. This operation
cannot select a destination and is not movement.

Coordinates, geometry, boundaries, containment, routes, distance, Place
neighborhoods and additional Places are absent.

### Agent-mediated trail-marker action

The first bundled player action has one concrete consequence. It is not a generic
action engine or mutation language:

```rust
struct SubmitAction {
    request_id: Uuid,
    expected_place_revision: PlaceRevision,
    prose: String,
    consequence: IntroduceEntity,
}

struct IntroduceEntity {
    name: String,
    description: String,
}

struct AcceptedAction {
    activity: Activity,
    entity: Entity,
    place: Place,
}

struct PlaceRevision {
    place_entity_id: EntityId,
    occurred_at: DateTime<Utc>,
    activity_id: ActivityId,
}
```

The wire tags the sole consequence as `introduce_entity`. Input accepts no User,
Character, Entity or Place selector and no effective time. World derives the User's
Character and exact current Place; a missing Character is not found and an unplaced
Character cannot act. World assigns the new Entity and Activity identities and the
acceptance time.

One accepted call atomically creates the Entity, places it at the derived Place,
inserts one Activity with one canonical prose value and records explicit Entity
roles. Partial acceptance is forbidden. Rejected calls, stale calls and retries add
no Activity. The returned `AcceptedAction` is the canonical stored result.

The required Agent interaction happens before `submit_action`: compose context from
published reads, show exactly three private grounded proposals, incorporate the
User's selection and optional steering, then show the exact final prose and
structured consequence. The Agent may call `submit_action` only after one explicit
User confirmation of that complete package. Proposals, steering, drafts and
confirmation are private conversation and never World state. World enforces the
submitted package deterministically; it does not and cannot verify the conversational
workflow or semantic prose quality.

### Delivery identity and exact-Place freshness

`request_id` is an Agent-generated UUID for one intended action. It remains stable
only across uncertain delivery retries and must not be reused for another action.
World derives a versioned SHA-256 `request_fingerprint` from a length-prefixed
encoding of the normalized expected Place revision, prose, consequence tag, Entity
name and Entity description. It never hashes raw JSON or depends on field order.

Both exact-Place reads return the same opaque `place_revision` when they observed the
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
`(requested_by_user_id, request_id)`. Equal fingerprint returns the canonical stored
result even if Character or Place state later changed. Different fingerprint returns
`action_request_conflict`. For an unseen id, World derives and locks the Character's
current Place and compares `expected_place_revision` with the current revision before
the remaining validation and writes. A changed representation returns
`place_revision_conflict` and writes nothing. Activity at another Place does not
invalidate the token.

Every writer that changes this exact-Place representation takes the same Place lock
before acceptance. `create_entry_place` assigns its preallocated genesis Activity as
`latest_activity_id` when it inserts the new Place. `enter_world`, `create_entity`
when its acting Character is currently placed, although the new Entity remains
unplaced; and `submit_action` lock the existing Place, insert their Activity, then
atomically point that Place to the inserted Activity in the same transaction. A
failure rolls back both Activity and pointer change. Mutations at different Places
remain concurrent. Reads issue no nonce, reservation or preparation record, and no
global revision or counter exists.

## Activity history

Current state remains authoritative and is never rebuilt by replay. Each accepted
covered mutation appends one immutable activity and its normalized involved-Entity
relations in the same transaction:

```rust
struct Activity {
    id: ActivityId,
    operation: ActivityOperation,
    actor_character: Option<EntitySummary>,
    context_place: Option<PlaceSummary>,
    involved_entity: Vec<ActivityEntityReference>,
    prose: Option<String>,
    occurred_at: DateTime<Utc>,
}

enum ActivityOperation {
    CreateCharacter,
    CreateEntity,
    CreateEntryPlace,
    EnterWorld,
    SubmitAction,
}

enum ActivityEntityRole {
    Subject,
    Destination,
    Location,
}
```

The stored `activity.requested_by_user_id` retains accountable internal request
provenance but is intentionally absent from player-visible history. Stable ids and
server-owned roles have these exact meanings:

| Operation | Actor Character | Context Place | Involved Entity |
| --- | --- | --- | --- |
| `create_character` | absent | absent | new Character Entity as `subject` |
| `create_entity` | current Character when one exists | its current Place when present | new Entity as `subject` |
| `create_entry_place` | proposing unplaced Character | absent | new Place Entity as `subject` |
| `enter_world` | entering Character | entry Place | entry Place Entity as `destination` |
| `submit_action` | acting Character | derived current Place | new Entity as `subject`; current Place as `location` |

Only accepted `submit_action` Activity has non-null prose, request id and request
fingerprint. Existing and other new Activity keeps those fields null. Activity rows,
relations and accepted prose reject update and delete. Reads, rejected requests,
transport traffic, conversation text and private Agent reasoning are not activity.
There is no JSON event payload, universal event abstraction or event sourcing.

One Activity owns one canonical prose value. Personal and Place-local history reads
return that same record rather than copied lens-specific text. Every history lens
orders canonical records by `(occurred_at, id)`; World acceptance is the only current
time axis and Agents cannot backdate or reorder history.

### Migration boundary

Before activity storage, every Entity with a Character role could only have resulted
from `create_character`, and every Entity without that role could only have resulted
from `create_entity`. Migration `0004_world_entry_activity.sql` backfills exactly
those derivable operation, responsible User, subject Entity and original
`introduced_at` facts. The old schema retained no acting Character or Place context,
so both remain null. No entry or placement history is fabricated.

The action migration leaves historical Activity prose, request id and fingerprint
null and assigns no historical Entity location. Current Entity location remains
ordinary authoritative state, not a replayed projection or inference from Activity.

### Personal history read

`list_activity` derives the current Character from User context and accepts no
Character id. It returns an activity exactly once when that Character is either the
stored actor or a role-linked involved Entity. It orders by `(occurred_at, id)`
descending and uses a typed cursor internally:

```rust
struct ListActivity {
    cursor: Option<ActivityCursor>,
    limit: u16,
}

struct ActivityCursor {
    occurred_at: DateTime<Utc>,
    activity_id: ActivityId,
}
```

The default limit is 25; accepted limits are 1 through 100. `next` is absent when no
further row exists. Actor Character, context Place and involved Entity references are
typed summaries.

### Exact current Place reads

`list_entity_at_current_place` and `list_activity_at_current_place` derive the
Character and its exact current Place from User context and accept no Character or
Place selector. They reject an unplaced Character instead of returning an empty page.
Each response contains the complete derived Place, its opaque `place_revision`, one
typed page and `next`.

The Entity read lists only ordinary Entities with an explicit current relation to
that exact Place. It orders by `(introduced_at, id)` descending. The Activity read
lists each Activity once when the Place is its stored context or is linked through an
involved Entity role. It orders by `(occurred_at, id)` descending and includes the
canonical optional prose. Both reuse their existing typed cursor families, default
limit 25 and accepted limit 1 through 100.

Exact stored Place equality is the complete inclusion rule. These reads do not imply
visibility, co-presence, distance, containment, adjacency or a metric radius.

## Entity listing

`list_entity` is a shared read with no User context. It orders summaries by
`(introduced_at, id)` descending and uses `EntityCursor` with the same 25 default and
1-through-100 bound. Responses contain `id` and `name`, not descriptions. Cursors are
opaque strings at HTTP and MCP.

## Validation and errors

Entity, Character and entry Place input is trimmed, requires 1 through 120 Unicode
characters for `name` and 1 through 4,000 for `description`, and rejects U+0000.
Action prose uses the same normalization, requires 1 through 4,000 Unicode
characters and rejects U+0000. PostgreSQL repeats the stored text invariants.

World distinguishes malformed request or revision input; invalid Entity, Character,
Place or action text; invalid Entity or Activity limit; User, Entity, Character or
entry Place not found; unplaced Character; existing Character, already-entered
Character or existing entry Place; request-id conflict; exact-Place revision
conflict; and unavailable storage.
Adapters expose the canonical spellings and status mapping in
[Agent interface](agent-interface.md).

## PostgreSQL model and indexes

```text
user(id PK, created_at)
entity(id PK, name, description, introduced_by_user_id FK user, introduced_at)
character(entity_id PK/FK entity, owner_user_id UNIQUE/FK user,
          current_place_entity_id NULL/FK place)
place(entity_id PK/FK entity, is_entry, latest_activity_id FK activity NOT NULL)
entity_location(entity_id PK/FK entity, place_entity_id FK place)
activity(id PK, operation, requested_by_user_id FK user,
         actor_character_entity_id NULL/FK character,
         context_place_entity_id NULL/FK place, prose NULL,
         request_id UUID NULL, request_fingerprint BYTEA NULL, occurred_at)
activity_entity(activity_id FK activity, entity_id FK entity, role,
                PK(activity_id, entity_id, role))
```

Indexes exist only for current behavior:

- `entity(introduced_at DESC, id DESC)` serves shared Entity pagination;
- unique `character(owner_user_id)` serves contextual lookup and one-Character
  arbitration;
- partial unique `place(is_entry) WHERE is_entry` arbitrates World genesis;
- `entity_location(place_entity_id, entity_id)` serves exact-Place Entity lookup;
- partial `activity(actor_character_entity_id, occurred_at DESC, id DESC)` and
  `activity_entity(entity_id, activity_id)` serve personal and Place history;
- partial unique `activity(requested_by_user_id, request_id) WHERE request_id IS NOT
  NULL` serves accepted action retry lookup; action fingerprints are exactly 32 bytes;
- primary-key indexes serve role joins and involved-Entity lookup.

Short contextual mutations lock their responsible User row. Place-relevant writers
also lock the affected Place as specified above, serializing state changes at one
Place and making `place.latest_activity_id`, rather than Activity timestamp or UUID
ordering, authoritative for its latest accepted representation. This imposes no
global World lock, revision or counter. Existing Activity immutability also protects
accepted prose, request identity and fingerprint.

## Required evidence

Tests must retain all prior evidence and additionally prove:

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
- all thirteen player capabilities have one semantic World/HTTP/MCP contract,
  strict schemas, complete catalog/OpenAPI publication and matching errors;
- the local launcher preserves one database and User across restart, refuses
  concurrent or unprofiled reuse that could create a second User, and never starts
  Codex itself; its printed adapter isolates workspace, home/configuration and
  transient conversation state while requiring current Aicadia MCP;
- the browser ledger uses only the four accepted GET reads, hides User UUIDs, remains
  responsive and keyboard-operable, and renders identical accepted ids and prose
  before and after restart; and
- one authorized paid clean-room Agent run demonstrates grounded reads, exactly three
  proposals, withheld selection and steering, exact preview, explicit confirmation,
  one commit and independent observation after token-free preflight. A failed or
  inconclusive run is not retried without new User authorization.

## Explicitly deferred

Authentication, OAuth, browser gameplay, general web UI beyond the supported local
read-only ledger, movement, additional Places, coordinates, routes, Place containment
and adjacency, metric neighborhoods, investigation, rolls, discovery, claims,
generic action or consequence engines, multiple consequences, Entity update or
movement, generic events, event sourcing, global World revisions, durable proposal
or Agent sessions, replay, as-of state, scores, currencies, clocks, background
simulation and server-side intelligence are absent.
