# Aicadia current build contract

This document is the executable game authority. Aicadia currently has one persistent
`World`, durable `User` records, shared `Entity` records, at most one owned
`Character` Entity role per User, and zero or one shared entry `Place`. A Character
may remain unplaced or explicitly enter that Place. Accepted game mutations append
immutable normalized `activity` in the same PostgreSQL transaction as current state.

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
```

`create_user` is internal provisioning, not player-facing. The other ten capabilities
ship through both HTTP and MCP as specified in [Agent interface](agent-interface.md).
Authentication remains deferred; adapters currently obtain an untrusted development
`UserId` from `Aicadia-User-Id` for contextual operations.

Each explicit call stands alone. There is no durable game session and no server-side
Agent invocation or inference.

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

`create_entity` accepts only `name` and `description`, derives the introducing User,
and always creates a new id. Equal input remains two Entities. When the User already
owns a Character, the activity records that Character as actor and its current Place
as context when present; otherwise actor and context are absent.

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

Coordinates, geometry, boundaries, containment, routes, distance and additional
Places are absent.

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
    occurred_at: DateTime<Utc>,
}

enum ActivityOperation {
    CreateCharacter,
    CreateEntity,
    CreateEntryPlace,
    EnterWorld,
}

enum ActivityEntityRole {
    Subject,
    Destination,
}
```

The stored `activity.requested_by_user_id` retains accountable internal request
provenance but is intentionally absent from player-visible history. Stable ids and
server-owned roles have these exact first-slice meanings:

| Operation | Actor Character | Context Place | Involved Entity |
| --- | --- | --- | --- |
| `create_character` | absent | absent | new Character Entity as `subject` |
| `create_entity` | current Character when one exists | its current Place when present | new Entity as `subject` |
| `create_entry_place` | proposing unplaced Character | absent | new Place Entity as `subject` |
| `enter_world` | entering Character | entry Place | entry Place Entity as `destination` |

Activity rows and relations reject update and delete. Reads, rejected requests,
transport traffic, conversation text and private Agent reasoning are not activity.
There is no JSON event payload, universal event abstraction or event sourcing.

### Migration boundary

Before activity storage, every Entity with a Character role could only have resulted
from `create_character`, and every Entity without that role could only have resulted
from `create_entity`. Migration `0004_world_entry_activity.sql` backfills exactly
those derivable operation, responsible User, subject Entity and original
`introduced_at` facts. The old schema retained no acting Character or Place context,
so both remain null. No entry or placement history is fabricated.

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
typed summaries. Global, Place-wide, Entity-wide and other-Character history reads
remain deferred.

## Entity listing

`list_entity` is a shared read with no User context. It orders summaries by
`(introduced_at, id)` descending and uses `EntityCursor` with the same 25 default and
1-through-100 bound. Responses contain `id` and `name`, not descriptions. Cursors are
opaque strings at HTTP and MCP.

## Validation and errors

Entity, Character and entry Place input is trimmed, requires 1 through 120 Unicode
characters for `name` and 1 through 4,000 for `description`, and rejects U+0000.
PostgreSQL repeats the stored Entity length and trimming invariants.

World distinguishes invalid Entity, Character and Place input; invalid Entity or
activity limit; User, Entity, Character or entry Place not found; Character already
exists; Character already entered; entry Place already exists; and unavailable
storage. Adapters own wire spelling and HTTP status mapping.

## PostgreSQL model and indexes

```text
user(id PK, created_at)
entity(id PK, name, description, introduced_by_user_id FK user, introduced_at)
character(entity_id PK/FK entity, owner_user_id UNIQUE/FK user,
          current_place_entity_id NULL/FK place)
place(entity_id PK/FK entity, is_entry)
activity(id PK, operation, requested_by_user_id FK user,
         actor_character_entity_id NULL/FK character,
         context_place_entity_id NULL/FK place, occurred_at)
activity_entity(activity_id FK activity, entity_id FK entity, role,
                PK(activity_id, entity_id, role))
```

Indexes exist only for current behavior:

- `entity(introduced_at DESC, id DESC)` serves shared Entity pagination;
- unique `character(owner_user_id)` serves contextual lookup and one-Character
  arbitration;
- partial unique `place(is_entry) WHERE is_entry` arbitrates World genesis;
- partial `activity(actor_character_entity_id, occurred_at DESC, id DESC)` and
  `activity_entity(entity_id, activity_id)` serve personal history selection;
- primary-key indexes serve role joins and involved-Entity lookup.

Short contextual mutations lock only their responsible User row to keep actor and
Place capture consistent with concurrent Character state changes. This is a local
implementation choice, not a universal World concurrency policy.

## Required evidence

Tests must prove persistence; shared Entity behavior; text validation; both cursor
bounds and UUID tie-breaks; exact Character identity and ownership; unplaced
creation; concurrent Character and entry-Place arbitration without orphans; derived
retry-safe World entry; atomic rollback of state and history; immutable history;
actor-or-involvement authorization without duplicates; retained historical Place
context; honest null actor/Place semantics; and HTTP/MCP/catalog parity for all ten
player capabilities.

## Explicitly deferred

Authentication, OAuth, web UI, movement, additional Places, coordinates, routes,
investigation, rolls, discovery, claims, generic events, event sourcing, global or
other-Character history, replay, as-of state, scores, currencies, clocks, background
simulation, Agent sessions and server-side intelligence are absent.
