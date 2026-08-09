# Aicadia MVP

Status: World core and Agent interface contract; authentication and OAuth are
deferred

This document defines the first executable Aicadia model. It is intentionally
limited to one persistent World, durable Users, shared create/read-only Entities and
one owned Character per User. Code, database migrations and tests must agree with
this document.

## Domain model

### World

There is exactly one World: Aicadia. The World is both the running deployment and
the Rust module that owns all current game behaviour. It is not a database row, has
no identifier and cannot be selected by a caller. Every Entity in the deployment is
part of this World.

The World module has this complete interface:

```rust
impl World {
    fn get_world(&self) -> WorldView;

    async fn create_user(&self) -> Result<User, WorldError>;

    async fn get_user(
        &self,
        user_id: UserId,
    ) -> Result<User, WorldError>;

    async fn get_character(
        &self,
        user_id: UserId,
    ) -> Result<Character, WorldError>;

    async fn create_character(
        &self,
        user_id: UserId,
        input: CreateCharacter,
    ) -> Result<Character, WorldError>;

    async fn list_entity(
        &self,
        input: ListEntity,
    ) -> Result<EntityPage, WorldError>;

    async fn get_entity(
        &self,
        entity_id: EntityId,
    ) -> Result<Entity, WorldError>;

    async fn create_entity(
        &self,
        introduced_by_user_id: UserId,
        input: CreateEntity,
    ) -> Result<Entity, WorldError>;
}
```

`World` is a concrete type backed by a PostgreSQL connection pool. There is no
storage trait or repository interface. PostgreSQL is the only storage implementation
in this MVP.

### User

A User is a durable participant record:

```rust
struct User {
    id: UserId,
    created_at: DateTime<Utc>,
}
```

`create_user` takes no input, generates the id on the server and returns the stored
User. It exists for provisioning and tests; it is not an HTTP operation or MCP tool.
`get_user` returns one User by id.

A User is not an Entity and does not represent a location or presence inside the
World. The current model does not attach an external identity or profile to a User.

### Character

A Character is the User-owned role of the Entity through which later player behavior
can enter the World. Each User may own zero or one Character. A Character has no
separate identity or duplicated Entity state:

```rust
struct Character {
    entity: Entity,
    owner_user_id: UserId,
}
```

`create_character(user_id, input)` accepts only `name` and `description`, derives
ownership from the supplied current User, and applies the same trimming, length and
NUL rules as Entity creation. It atomically creates the Entity and Character role.
The Entity stores that User as `introduced_by_user_id`, while the role records the
distinct ownership meaning as `owner_user_id`. A failed or concurrent losing create
leaves neither an orphan Entity nor a Character row. Repeating after success returns
`CharacterAlreadyExists`; it never returns the existing Character as a create
success.

`get_character(user_id)` derives the only possible Character from its owning User.
Neither operation accepts an Entity id. A Character may exist without an established
Place: Character creation introduces identity and ownership only and never places the
Character. Location, movement, investigation, rolls, discovery, sessions and
authentication remain absent.

### Entity

An Entity is one thing or concept that needs a stable identity in the shared World.
The operational test is whether a later caller must be able to refer to exactly the
same subject, independently of the sentence in which it first appeared. If a word,
substance, amount, property or incidental detail only helps describe another Entity,
it stays text in that Entity's `description`.

Concrete examples define the boundary:

- `The bridge is made of wood` does not introduce `wood`; it is a material value in
  the bridge description.
- A deliberately shared material concept such as `amberwood` may be an Entity when
  callers need to refer to that same concept again. It receives no special material
  type or taxonomy field.
- `The flask contains water` does not make the water or each quantity of it an
  Entity. The shared substance concept `water` may be introduced separately only
  when it needs its own stable identity.
- A particular old tree may be an Entity when callers need to find or mention that
  same tree again. The generic word `tree` and incidental scenery trees are not
  automatically Entities.
- A particular named lake may be an Entity even though its water changes. Its water
  and every drop in it do not automatically become Entities.

The caller, not the World, applies this stable-reference test. The World validates
only the deterministic input rules in this contract and never extracts Entities from
text, resolves identities or deduplicates candidates.

An accepted Entity is stored as:

```rust
struct Entity {
    id: EntityId,
    name: String,
    description: String,
    introduced_by_user_id: UserId,
    introduced_at: DateTime<Utc>,
}
```

An Agent proposes the candidate `name` and `description` on behalf of a User. The
HTTP or MCP adapter supplies its request context's User id to `World` as
`introduced_by_user_id`; the Agent does not choose that id in the operation payload.
The World validates the candidate, verifies that the User exists, generates `id` and
`introduced_at`, stores the Entity and returns it. The candidate has no durable
identity before that successful call.

`introduced_by_user_id` attributes the introduction action to the User. Introduction
does not mean that the User created the subject inside the fiction, owns it,
discovered it, controls it or appears as it. `introduced_at` records when the Entity
entered the shared World data; it is not the subject's fictional creation, birth or
discovery time.

The World performs this explicit request but never invents, generates or spawns an
Entity autonomously. The technical CRUD operation remains `create_entity`; there is
no separate `introduce_entity` operation.

Entity names are display text, not identifiers, and are not unique. References use
only `entity_id`. The current interface deliberately has no update or delete
operation. The current model has no Entity type, kind, taxonomy or automatic
classification.

## Calls and sessions

HTTP and MCP are thin adapters over the same `World` interface. They own transport
parsing, request context and error presentation, but no game behaviour. The complete
transport contract is [Agent interface](agent-interface.md). `World` types are not
themselves the wire contract.

There is no durable domain session: no login state, online state, session table,
conversation identifier or game memory between calls. Every World method call stands
alone. MCP `2025-11-25` clients use a temporary in-memory transport session required
by that protocol revision; it is discarded on server restart and stores no domain,
User or authentication state. MCP `2026-07-28` remains stateless and carries its own
protocol metadata. Either revision may carry User request context, but neither
persists it as game state. Future authentication state also stays outside the World
interface.

Shared reads do not receive a User or caller because their current result does not
vary by caller. `get_world`, `list_entity` and `get_entity` therefore require no User
context at the transport. `get_user`, `get_character`, `create_character` and
`create_entity` derive the User from request context.

The adapters do not authenticate the caller. Their current User header is an
untrusted, local-development assertion. `World` only verifies that the supplied User
exists. Authentication and OAuth are not part of this contract.

## Operations

The operation names use standard CRUD verbs and qualify the resource because they
share one World interface:

- `get_world` returns the name `Aicadia`.
- `create_user` creates and returns a User for internal provisioning and tests.
- `get_user` returns one User by `user_id`.
- `get_character` returns the Character owned by `user_id`.
- `create_character` atomically creates the one Character owned by an existing User
  and its shared Entity.
- `list_entity` returns a page of Entity summaries.
- `get_entity` returns one Entity by `entity_id`.
- `create_entity` accepts and stores one Entity candidate introduced by an existing
  User, then returns the Entity.

The seven operations other than `create_user` are available through both HTTP and
MCP. There is no `update` or `delete` behavior in the MVP.

## Entity listing

Entity summaries are ordered by `(introduced_at, id)`, both descending. The typed
input and output are:

```rust
struct ListEntity {
    cursor: Option<EntityCursor>,
    limit: u16,
}

struct EntityCursor {
    introduced_at: DateTime<Utc>,
    entity_id: EntityId,
}

struct EntitySummary {
    id: EntityId,
    name: String,
}

struct EntityPage {
    entity: Vec<EntitySummary>,
    next: Option<EntityCursor>,
}
```

`cursor` defaults to `None`. `limit` defaults to `25` and must be from `1` through
`100`. `next` is `None` when no further row exists. Entity descriptions are omitted
from summaries.

## Validation and invariants

The World module validates every input. PostgreSQL repeats the structural
constraints it can enforce:

- `name` is trimmed before validation and storage;
- `name` contains from 1 through 120 Unicode characters after trimming;
- `name` does not contain U+0000 (NUL);
- `description` is trimmed before validation and storage;
- `description` contains from 1 through 4,000 Unicode characters after trimming;
- `description` does not contain U+0000 (NUL);
- `create_entity` accepts only an existing `UserId`;
- Character creation accepts only an existing `UserId` and the same Entity text
  boundary;
- `character.entity_id` is both its primary key and a foreign key to `entity.id`;
- `character.owner_user_id` is unique and references its owning User;
- creating an Entity and Character role is one PostgreSQL transaction;
- Entity identity and User ownership remain separately typed UUID values;
- `limit` is from 1 through 100;
- no uniqueness rule applies to `name` or `description`.

The server treats `name` and `description` as text. It does not infer additional
meaning, extract a type or taxonomy, create related Entities or make an LLM call.

`create_entity` is not idempotent. Repeating a successful call creates another
Entity, even when both text fields are equal. The World does not infer that equal or
similar candidates refer to the same subject.

## Errors

The World interface distinguishes these current failures:

- invalid Entity input;
- invalid Character input;
- invalid Entity list limit;
- User not found;
- Entity not found;
- Character not found;
- Character already exists;
- PostgreSQL unavailable.

Errors never expose SQL, credentials or stack traces.

When PostgreSQL fails, `World` emits one redacted JSON diagnostic to server stderr
with only the owning module, World operation, fixed failure category, unavailable
status and retry guidance. The Agent still receives only the canonical `unavailable`
error. Diagnostics never include SQL, database error text, credentials, input
content or stack traces.

## PostgreSQL schema

There is deliberately no `world` table and no `world_id` column. The `user` table is
quoted in SQL because `USER` has special meaning there; the domain and table term
remain User and `user`.

```sql
CREATE TABLE "user" (
    id uuid PRIMARY KEY,
    created_at timestamptz NOT NULL DEFAULT now()
);

CREATE TABLE entity (
    id uuid PRIMARY KEY,
    name text NOT NULL,
    description text NOT NULL,
    introduced_by_user_id uuid NOT NULL REFERENCES "user" (id),
    introduced_at timestamptz NOT NULL DEFAULT clock_timestamp(),
    CHECK (char_length(name) BETWEEN 1 AND 120),
    CHECK (name = btrim(name)),
    CHECK (char_length(description) BETWEEN 1 AND 4000),
    CHECK (description = btrim(description))
);

CREATE INDEX entity_introduced_at_id_index
    ON entity (introduced_at DESC, id DESC);

CREATE TABLE character (
    entity_id uuid NOT NULL,
    owner_user_id uuid NOT NULL,
    CONSTRAINT character_pkey PRIMARY KEY (entity_id),
    CONSTRAINT character_entity_id_fkey
        FOREIGN KEY (entity_id) REFERENCES entity (id) ON DELETE RESTRICT,
    CONSTRAINT character_owner_user_id_key UNIQUE (owner_user_id),
    CONSTRAINT character_owner_user_id_fkey
        FOREIGN KEY (owner_user_id) REFERENCES "user" (id) ON DELETE RESTRICT
);
```

The current relationships and their database cost are explicit:

| Relationship | Meaning | Enforcement and index |
| --- | --- | --- |
| `entity.introduced_by_user_id → user.id` | Attribution: which User introduced the Entity record | Foreign key only; no current query needs another index |
| `character.entity_id → entity.id` | Role: this Entity is also a Character | Primary key and foreign key; the primary-key index serves the join |
| `character.owner_user_id → user.id` | Ownership: this User owns this Character | Foreign key and unique index; the index serves `get_character` and concurrent-create arbitration |

Introduction and ownership are deliberately different relationships even though
`create_character` currently supplies the same contextual User for both. No generic
role table, polymorphic reference or extra relationship index exists.

## Required tests

Core tests cross the same World interface used by both adapters and run against a
local PostgreSQL server. Adapter parity is specified in
[Agent interface](agent-interface.md). The paid two-Agent acceptance path is
specified separately in [Agent playtest](agent-playtest.md); its token-free
preflight is safe to run without starting an Agent.

1. `get_world` returns `Aicadia`.
2. `create_user` stores and returns a User with a generated id.
3. `get_user` returns that User unchanged.
4. `get_user` reports an unknown User id as not found.
5. `create_entity` stores its supplied existing User id as introducer.
6. `create_entity` rejects an unknown User id without inserting an Entity.
7. `create_entity` returns an Entity that `get_entity` reads unchanged.
8. Entities introduced by two Users appear together in the same Entity list.
9. Two equal `create_entity` inputs create two Entity ids.
10. Every invalid Entity field is rejected without inserting an Entity.
11. `get_entity` reports an unknown Entity id as not found.
12. `list_entity` has stable reverse introduction order and no row appears twice while
    following its typed cursors.
13. `list_entity` enforces its default and maximum page size.
14. Restarting `World` against the same database preserves Users and Entities.
15. Character creation persists explicit ownership and returns its complete shared
    Entity without creating a separate Character identity or copying Entity fields.
16. Character reads derive the User, distinguish an unknown User from a missing
    Character, and accept no caller-selected Entity id.
17. Character creation reuses Entity text validation and rejects a second Character
    for one User.
18. Two concurrent Character creates yield exactly one winner, one Character and one
    Entity with no orphan.

Run the complete suite with local PostgreSQL running and a role that may create and
drop SQLx test databases:

```sh
DATABASE_URL=postgres://localhost/postgres cargo test
```

## Non-goals

This MVP does not model:

- authentication, OAuth or authorization;
- a public `create_user` transport operation;
- more than one World;
- a Character's location or movement;
- links or structured statements between Entities;
- changes to or deletion of an existing Entity;
- a history of changes;
- places, investigation, rolls, discovery or simulation;
- background work initiated by the server;
- server-side interpretation of Entity text;
- automatic Entity extraction, identity resolution or deduplication;
- Entity types, kinds, taxonomy or structured material values;
- browser-facing game screens.

Anything outside the eight World operations and the two adapter presentations defined
in [Agent interface](agent-interface.md) is outside this implementation contract.
