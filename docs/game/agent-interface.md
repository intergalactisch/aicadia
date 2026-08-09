# Agent interface

Status: authoritative MVP contract for HTTP and MCP

This document defines every game capability currently available to an Agent. HTTP
and MCP are two thin adapters over the same `World` interface. They expose the same
meaning, validation and errors; neither adapter contains game behaviour.

The word _player-facing_ in this document means that the operation is part of the
published Agent capability catalog. It does not mean that the caller has been
cryptographically authenticated. Authentication, OAuth and authorization remain
deferred.

## Topology and seam

```text
HTTP JSON ─┐
           ├── World ── PostgreSQL
MCP ───────┘
```

`World` is the one interface at the seam. Both adapters call its methods directly.
PostgreSQL, SQL, migrations and connection pooling stay behind that seam. The
adapters may parse transport values, build request context, encode cursors and map
errors, but they never repeat or replace `World` validation.

The HTTP API and MCP endpoint run against the same `World` and therefore the same
persistent shared data. A result created through one adapter is immediately readable
through the other.

## Capability catalog

The catalog is closed and complete:

| Capability | `World` call | HTTP operation | MCP tool | User context |
| --- | --- | --- | --- | --- |
| `get_world` | `get_world()` | `GET /api/world` | `get_world` | none |
| `get_user` | `get_user(context.user_id)` | `GET /api/user` | `get_user` | required |
| `get_character` | `get_character(context.user_id)` | `GET /api/character` | `get_character` | required |
| `create_character` | `create_character(context.user_id, input)` | `POST /api/character` | `create_character` | required |
| `list_entity` | `list_entity(input)` | `GET /api/entity` | `list_entity` | none |
| `get_entity` | `get_entity(entity_id)` | `GET /api/entity/{entity_id}` | `get_entity` | none |
| `create_entity` | `create_entity(context.user_id, input)` | `POST /api/entity` | `create_entity` | required |

`create_user` is deliberately absent. It is an internal provisioning operation used
by an operator or test, not a Player capability. `World::new`, database access,
migrations, diagnostics, administration and operational controls are also absent and
must never be published as Agent tools.

No capability is available only through a UI. A future UI must use this same
published interface.

## Local endpoint and origin

The unauthenticated MVP is local-only:

- the server binds to `127.0.0.1`, not every network interface;
- the canonical development origin is `http://127.0.0.1:3000`;
- HTTP routes use that origin directly, for example
  `http://127.0.0.1:3000/api/world`;
- MCP uses Streamable HTTP at `http://127.0.0.1:3000/mcp`;
- OpenAPI is served at `http://127.0.0.1:3000/api/openapi.json`;
- Streamable HTTP validates `Origin`: an absent header is accepted for local
  command-line and Agent clients, the exact configured origin is accepted, and any
  other value is rejected;
- browser CORS must never use a wildcard origin.

`localhost` and `127.0.0.1` are different browser origins. Documentation and tests
use `127.0.0.1` consistently. Binding or publishing this unauthenticated interface
beyond loopback is outside the MVP.

### Start a local Agent task

The repository's `.codex/config.toml` registers the canonical MCP endpoint as
`aicadia` for trusted Codex tasks started in this repository. Start PostgreSQL, then
export the database URL and start the game server in one terminal:

```sh
export DATABASE_URL=postgres://localhost/aicadia
cargo run
```

An environment assignment written directly before `cargo run` applies only to that
one process. A second terminal does not inherit it. In the terminal that will
provision the User, set the same URL and run the existing operator binary:

```sh
export DATABASE_URL=postgres://localhost/aicadia
cargo run --quiet --bin aicadia-provision-user
```

The command prints one JSON User, for example
`{"id":"54c18ce9-9ce9-4e17-b52c-04fd57ad8529","created_at":"..."}`. Copy its
`id`, then expose it before starting Codex:

```sh
export AICADIA_USER_ID=54c18ce9-9ce9-4e17-b52c-04fd57ad8529
```

Start a new Codex task after setting the variable. Codex reads project MCP
configuration and environment-backed headers when a task starts; an already running
task or sub-Agent does not hot-reload the MCP tool registry. Restart it or start a
new task after changing the server configuration or `AICADIA_USER_ID`.

Without `AICADIA_USER_ID`, an Agent can still use `get_world`, `list_entity` and
`get_entity`. `get_user`, `get_character`, `create_character` and `create_entity`
remain discoverable but return
`user_context_required` when called because their request has no User context.

### Live cross-User playtest

The [Agent playtest](agent-playtest.md) is the executable acceptance proof that two
real, sequential Codex Agents can share one Entity through Aicadia MCP. Its normal
preflight spends no tokens. A paid run requires an exact confirmation flag, creates
an isolated disposable World, gives each Agent only the tools needed for its role,
collects correlated evidence, and drops the disposable database after validation.

```sh
tools/agent-playtest preflight
tools/agent-playtest run --confirm-token-spend
```

## User request context

`get_user`, `get_character`, `create_character` and `create_entity` require this HTTP
header:

```http
Aicadia-User-Id: 54c18ce9-9ce9-4e17-b52c-04fd57ad8529
```

The request must contain exactly one `Aicadia-User-Id` header value, and that value
must be one UUID identifying an existing User. Repeated header fields and a single
field containing multiple comma-separated values both count as multiple values and
are rejected. For the HTTP API the value is read from that request. For MCP over
Streamable HTTP, the MCP client sends the same header on the request carrying the
tool call. Character tool input never accepts `owner_user_id` or `entity_id`, and Entity
creation never accepts `user_id` or `introduced_by_user_id`, so an Agent cannot
select a different owner or introducer inside its tool arguments.

The header is an untrusted local-development assertion, not authentication. Anyone
who knows a User id can currently present it. The adapter must not call this secure,
authenticated or authorized. OAuth or another later authenticator will replace only
the creation of this request context; it will still pass a trusted `UserId` to the
same `World` methods.

Request metadata and the transport connection are not a domain session. There is no
session table, login state, online state, conversation id or persisted Agent
identity. The one `/mcp` endpoint supports two protocol revisions:

- MCP `2025-11-25` uses the revision's initialize handshake and a temporary,
  in-memory transport session. The client returns `Mcp-Session-Id` on later requests.
  The session disappears when the server stops and contains no game or User state.
- MCP `2026-07-28` remains stateless. Every call stands alone, carries its required
  per-request protocol metadata and receives no `Mcp-Session-Id`.

Both revisions call the same seven tools and the same `World` methods. A legacy
transport session is connection plumbing only: it does not authenticate a User,
retain a conversation or enter `World` or game storage. Context-required tool calls
still carry `Aicadia-User-Id` on the request that invokes the tool.

Context failures are deterministic:

- missing `Aicadia-User-Id` on a context-required operation returns
  `user_context_required`;
- a malformed value returns `invalid_request` with field
  `Aicadia-User-Id` and reason `invalid_uuid`;
- multiple values return `invalid_request` with field `Aicadia-User-Id` and reason
  `multiple_values`;
- a well-formed id that does not identify a User returns `user_not_found`.

The three read-only shared-world operations do not require the header because their
result does not vary by User.

## Wire conventions

- Request and response field names are lower `snake_case`.
- UUID values are lowercase UUID strings on output; UUID input is case-insensitive.
- Timestamps are RFC 3339 strings in UTC.
- HTTP JSON requests use `Content-Type: application/json`.
- Successful HTTP reads return `200 OK`; `create_character` and `create_entity`
  return `201 Created`.
- Successful operations return their result object directly, without a `data`
  wrapper.
- Unknown JSON fields are rejected.
- MCP successful `structuredContent` is the same object as the corresponding HTTP
  JSON response. A short English text rendering may additionally appear in MCP
  `content`.

### Shared result shapes

`WorldView`:

```json
{
  "name": "Aicadia"
}
```

`User`:

```json
{
  "id": "54c18ce9-9ce9-4e17-b52c-04fd57ad8529",
  "created_at": "2026-08-07T12:00:00Z"
}
```

`Character`:

```json
{
  "entity": {
    "id": "bf734a6f-1502-4453-9279-4c0f091d943f",
    "name": "Mara Venn",
    "description": "A careful surveyor at the edge of the known World.",
    "introduced_by_user_id": "54c18ce9-9ce9-4e17-b52c-04fd57ad8529",
    "introduced_at": "2026-08-07T12:00:30Z"
  },
  "owner_user_id": "54c18ce9-9ce9-4e17-b52c-04fd57ad8529"
}
```

The Character has no separate id or copied Entity fields. `entity.id` is its only
World identity and `owner_user_id` is the ownership relation derived from request
context; neither is Agent input. `entity.introduced_by_user_id` remains introduction
attribution and must not be interpreted as ownership.

`EntitySummary`:

```json
{
  "id": "4506c2a9-b4b8-4053-864e-d5f1c3f49eeb",
  "name": "Old Willow"
}
```

`Entity`:

```json
{
  "id": "4506c2a9-b4b8-4053-864e-d5f1c3f49eeb",
  "name": "Old Willow",
  "description": "A mature willow beside Glassmere Lake.",
  "introduced_by_user_id": "54c18ce9-9ce9-4e17-b52c-04fd57ad8529",
  "introduced_at": "2026-08-07T12:01:00Z"
}
```

## Opaque Entity cursor

The adapter presents the typed `EntityCursor` as one opaque, URL-safe string. Clients
and Agents must only copy a returned `next` value into the following `cursor` input.
They must not parse, construct or modify it.

`list_entity` accepts:

```json
{
  "cursor": null,
  "limit": 25
}
```

Both fields are optional. `cursor` defaults to `null`; `limit` defaults to `25` and
must be from `1` through `100`. The result is:

```json
{
  "entity": [
    {
      "id": "4506c2a9-b4b8-4053-864e-d5f1c3f49eeb",
      "name": "Old Willow"
    }
  ],
  "next": "opaque-url-safe-value-or-null"
}
```

`next` is `null` when there is no following page. A malformed cursor returns
`invalid_request` with field `cursor` and reason `malformed`. Cursor encoding carries
no authority and creates no server-side session. Its internal representation is not
part of the interface and may change as long as cursors already issued by the same
deployed version remain usable.

## HTTP operations

OpenAPI `operationId` is exactly the capability name in this table.

| Method and path | Input | Success |
| --- | --- | --- |
| `GET /api/world` | none | `200` with `WorldView` |
| `GET /api/user` | `Aicadia-User-Id` header | `200` with `User` |
| `GET /api/character` | context header; no query or Entity id | `200` with `Character` |
| `POST /api/character` | context header plus `CreateCharacter` JSON | `201` with `Character` |
| `GET /api/entity` | optional `cursor` string and `limit` integer query fields | `200` with `EntityPage` |
| `GET /api/entity/{entity_id}` | UUID path field | `200` with `Entity` |
| `POST /api/entity` | context header plus `CreateEntity` JSON | `201` with `Entity` |

`CreateCharacter` and `CreateEntity` each contain exactly:

```json
{
  "name": "Old Willow",
  "description": "A mature willow beside Glassmere Lake."
}
```

The adapter forwards both strings to `World`. `World` trims and validates them as
specified in [Aicadia MVP](README.md). Character creation derives the owning User,
creates its Entity atomically and returns `409 Conflict` if that User already
owns a Character. Repeating a successful `POST /api/entity` creates a distinct
Entity; that operation is not idempotent.

## MCP tools

MCP publishes exactly the seven tools returned by `tools/list`. Tool names equal the
capability names and OpenAPI operation IDs.

### Descriptions

The tool descriptions are normative:

| Tool | Description |
| --- | --- |
| `get_world` | Get the identity of the one persistent shared Aicadia World. No User context is required. |
| `get_user` | Get the durable User represented by this request's `Aicadia-User-Id` context. This tool does not accept a User id and does not authenticate the caller. |
| `get_character` | Get the Character role owned by the current User. The result embeds its shared Entity and explicit owner; the Character has no separate id. This tool derives the User from `Aicadia-User-Id` and accepts no ids. |
| `create_character` | Create the one Character role owned by the current User and its shared Entity. The result embeds that Entity and explicit owner; the Character has no separate id. This tool derives the User from `Aicadia-User-Id` and accepts no ids. |
| `list_entity` | List shared Entities from newest to oldest. `limit` defaults to 25 and must be 1 through 100. Copy `next` into `cursor` to read the following page; do not interpret the cursor. |
| `get_entity` | Get one shared Entity by its stable Entity id. |
| `create_entity` | Create one shared Entity for a stable referent introduced by the current User. Use this only when later participants must refer to the same subject. This does not assert fictional creation, ownership or discovery, and repeating it creates another Entity. |

### Annotations

MCP annotations are hints to clients, not replacements for server validation.
`openWorldHint` is `false` because these tools operate only on Aicadia and never on
arbitrary external systems.

| Tool | `title` | `readOnlyHint` | `destructiveHint` | `idempotentHint` | `openWorldHint` |
| --- | --- | --- | --- | --- | --- |
| `get_world` | `Get world` | `true` | `false` | `true` | `false` |
| `get_user` | `Get user` | `true` | `false` | `true` | `false` |
| `get_character` | `Get character` | `true` | `false` | `true` | `false` |
| `create_character` | `Create character` | `false` | `false` | `false` | `false` |
| `list_entity` | `List entity` | `true` | `false` | `true` | `false` |
| `get_entity` | `Get entity` | `true` | `false` | `true` | `false` |
| `create_entity` | `Create entity` | `false` | `false` | `false` | `false` |

### Input schemas

`get_world`, `get_user` and `get_character`:

```json
{
  "type": "object",
  "properties": {},
  "additionalProperties": false
}
```

`list_entity`:

```json
{
  "type": "object",
  "properties": {
    "cursor": {
      "type": ["string", "null"],
      "description": "Opaque cursor copied from the preceding result's next field."
    },
    "limit": {
      "type": "integer",
      "minimum": 1,
      "maximum": 100,
      "default": 25
    }
  },
  "additionalProperties": false
}
```

`get_entity`:

```json
{
  "type": "object",
  "properties": {
    "entity_id": {
      "type": "string",
      "format": "uuid",
      "description": "Stable id of the Entity to return."
    }
  },
  "required": ["entity_id"],
  "additionalProperties": false
}
```

`create_character` and `create_entity`:

```json
{
  "type": "object",
  "properties": {
    "name": {
      "type": "string",
      "minLength": 1,
      "maxLength": 120,
      "description": "Display name. The World trims it and accepts 1 through 120 Unicode characters."
    },
    "description": {
      "type": "string",
      "minLength": 1,
      "maxLength": 4000,
      "description": "Description. The World trims it and accepts 1 through 4,000 Unicode characters."
    }
  },
  "required": ["name", "description"],
  "additionalProperties": false
}
```

JSON Schema length hints do not replace `World` validation. In particular, trimming,
Unicode character counts and NUL rejection remain deterministic World behaviour.

### Output schemas

Each MCP tool declares an `outputSchema`. Required fields and
`additionalProperties: false` apply at every object level.

`get_world`:

```json
{
  "type": "object",
  "properties": {"name": {"type": "string"}},
  "required": ["name"],
  "additionalProperties": false
}
```

`get_user`:

```json
{
  "type": "object",
  "properties": {
    "id": {"type": "string", "format": "uuid"},
    "created_at": {"type": "string", "format": "date-time"}
  },
  "required": ["id", "created_at"],
  "additionalProperties": false
}
```

`get_character` and `create_character`:

```json
{
  "$defs": {
    "EntityOutput": {
      "type": "object",
      "properties": {
        "id": {"type": "string", "format": "uuid"},
        "name": {"type": "string"},
        "description": {"type": "string"},
        "introduced_by_user_id": {"type": "string", "format": "uuid"},
        "introduced_at": {"type": "string", "format": "date-time"}
      },
      "required": ["id", "name", "description", "introduced_by_user_id", "introduced_at"],
      "additionalProperties": false
    }
  },
  "type": "object",
  "properties": {
    "entity": {"$ref": "#/$defs/EntityOutput"},
    "owner_user_id": {"type": "string", "format": "uuid"}
  },
  "required": ["entity", "owner_user_id"],
  "additionalProperties": false
}
```

`list_entity`:

```json
{
  "type": "object",
  "properties": {
    "entity": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "id": {"type": "string", "format": "uuid"},
          "name": {"type": "string"}
        },
        "required": ["id", "name"],
        "additionalProperties": false
      }
    },
    "next": {"type": ["string", "null"]}
  },
  "required": ["entity", "next"],
  "additionalProperties": false
}
```

`get_entity` and `create_entity`:

```json
{
  "type": "object",
  "properties": {
    "id": {"type": "string", "format": "uuid"},
    "name": {"type": "string"},
    "description": {"type": "string"},
    "introduced_by_user_id": {"type": "string", "format": "uuid"},
    "introduced_at": {"type": "string", "format": "date-time"}
  },
  "required": [
    "id",
    "name",
    "description",
    "introduced_by_user_id",
    "introduced_at"
  ],
  "additionalProperties": false
}
```

`EntitySummary` requires `id: string(uuid)` and `name: string`. `Entity` requires
`id: string(uuid)`, `name: string`, `description: string`,
`introduced_by_user_id: string(uuid)` and `introduced_at: string(date-time)`. These
are the same shapes shown under **Shared result shapes** and used by OpenAPI.

## Errors

Every capability execution error uses one canonical envelope:

```json
{
  "error": {
    "code": "invalid_entity",
    "message": "Entity name is empty.",
    "field": "name",
    "reason": "empty"
  }
}
```

`field` and `reason` are omitted when they add no information. Messages are stable,
concise English explanations for a human or Agent; callers branch on `code`, not on
`message`.

| Code | Meaning | HTTP status |
| --- | --- | --- |
| `invalid_request` | Malformed transport input, UUID or cursor | `400` |
| `user_context_required` | Required `Aicadia-User-Id` is absent | `400` |
| `invalid_entity` | `name` or `description` violates World validation | `400` |
| `invalid_character` | Character `name` or `description` violates the shared Entity text validation | `400` |
| `invalid_entity_limit` | `limit` is outside `1..=100` | `400` |
| `user_not_found` | Context names no stored User | `404` |
| `entity_not_found` | `entity_id` names no stored Entity | `404` |
| `character_not_found` | The current User owns no Character | `404` |
| `character_already_exists` | The current User already owns a Character | `409` |
| `unavailable` | PostgreSQL or the World implementation is unavailable | `503` |

For `invalid_entity` and `invalid_character`, `field` is `name` or `description`;
`reason` is `empty`, `contains_nul` or `too_long`. Errors never expose SQL,
credentials, internal paths or stack traces.

An MCP capability error sets `isError: true`, omits `structuredContent` and includes
the canonical error envelope serialized as one text block in `content`. Omitting
structured output prevents an error from violating the tool's success
`outputSchema`. A successful tool call has `resultType: "complete"`, returns matching
`structuredContent`, includes compact serialized JSON as a compatibility text block
and sets `isError: false` or omits it. Invalid MCP framing, an unknown tool name,
missing or mismatched routing or protocol metadata, and JSON-RPC failures remain MCP
protocol errors rather than game capability errors.

## Discovery

Agents must be able to discover the entire interface without reading source code:

- MCP `server/discover` advertises exactly `2025-11-25` and `2026-07-28` and only
  the `tools` server capability. Prompts, resources, sampling, roots, tasks,
  subscriptions and dynamic tool-list changes are not advertised.
- MCP `tools/list` returns exactly the seven catalog entries, descriptions,
  annotations, input schemas and output schemas defined above, in catalog order.
  The static catalog uses `cacheScope: "public"`, a finite non-negative `ttlMs` and
  no `listChanged` capability.
- Every Streamable HTTP request carries the MCP headers and request metadata
  required by its negotiated revision. MCP `2025-11-25` tool requests use their
  initialized transport session and do not need MCP 2026 per-request `_meta`; MCP
  `2026-07-28` requests are stateless and require that metadata. Protocol data never
  identifies a User; `Aicadia-User-Id` remains separate request context.
- `GET /api/openapi.json` describes exactly the seven HTTP operations with matching
  `operationId` values, shared schemas and canonical error responses.
- `create_user` and every operational control are absent from both documents.
- This Markdown document explains the semantic rules that schemas alone cannot
  express.

The OpenAPI document and MCP schemas are executable interface artifacts, not a
second place for game rules. The `World` contract in [Aicadia MVP](README.md) remains
authoritative for behaviour.

## Examples

Read the World:

```sh
curl -sS http://127.0.0.1:3000/api/world
```

Read the current User:

```sh
curl -sS \
  -H 'Aicadia-User-Id: 54c18ce9-9ce9-4e17-b52c-04fd57ad8529' \
  http://127.0.0.1:3000/api/user
```

Create the current User's Character:

```sh
curl -sS \
  -X POST \
  -H 'Content-Type: application/json' \
  -H 'Aicadia-User-Id: 54c18ce9-9ce9-4e17-b52c-04fd57ad8529' \
  --data '{"name":"Mara Venn","description":"A careful surveyor at the edge of the known World."}' \
  http://127.0.0.1:3000/api/character
```

Create an Entity:

```sh
curl -sS \
  -X POST \
  -H 'Content-Type: application/json' \
  -H 'Aicadia-User-Id: 54c18ce9-9ce9-4e17-b52c-04fd57ad8529' \
  --data '{"name":"Old Willow","description":"A mature willow beside Glassmere Lake."}' \
  http://127.0.0.1:3000/api/entity
```

An MCP `create_entity` tool call supplies only capability input:

```json
{
  "name": "create_entity",
  "arguments": {
    "name": "Old Willow",
    "description": "A mature willow beside Glassmere Lake."
  }
}
```

The MCP transport request supplies `Aicadia-User-Id`; it is never embedded in
`arguments`.

## Capability-parity test

The adapter integration suite must prove all of the following against one temporary
PostgreSQL-backed World:

1. OpenAPI operation IDs and MCP `tools/list` names are the same exact seven-name set.
2. Neither catalog contains `create_user` or an operational capability.
3. `get_world` and `get_user` have equivalent HTTP and MCP success results.
4. An Entity created through HTTP can be read through MCP.
5. An Entity created through MCP appears in the HTTP Entity list.
6. A Character created through either adapter is returned through the other, is
   owned by the contextual User and embeds the exact Entity returned by `get_entity`.
7. HTTP and MCP expose the same normalized result fields and canonical error codes.
8. Both adapters reject the same invalid Entity and Character inputs through `World`
   validation.
9. Both adapters return the explicit Entity/Character not-found and
   Character-already-exists errors.
10. Context-required operations reject missing, malformed, duplicate and unknown User
   context; duplicate values produce `invalid_request`, field
   `Aicadia-User-Id`, reason `multiple_values`.
11. HTTP returns the documented `400` statuses for request, context and validation
    failures, `404` for unknown resources and `409` for a second Character.
12. MCP treats an unknown tool and missing or mismatched routing or protocol metadata
    as protocol errors, not tool execution errors.
13. Pagination can continue by copying each adapter's opaque `next` cursor.
14. An MCP `2025-11-25` client can initialize, acknowledge initialization and list
    all seven tools through one temporary transport session without MCP 2026
    per-request `_meta`.
15. An MCP `2026-07-28` request receives no transport session and is rejected before
    dispatch when required per-request protocol metadata is absent.

Generated ids and timestamps are checked for shape and persistence, not compared
literally across two separate create calls. Tests assert observable behaviour at the
adapter interfaces and never query transport implementation details.

## Adding a capability

A player-facing capability is incomplete until one change contains every item:

1. Add or confirm the typed operation on the `World` interface.
2. Implement all deterministic validation and game behaviour inside `World`.
3. Decide explicitly whether User context is required; never accept a caller-chosen
   User id in tool input when request context owns it.
4. Add the HTTP operation with an `operationId` equal to the capability name.
5. Add the identically named MCP tool with a complete description, annotations,
   input schema and output schema.
6. Use the shared result shapes and canonical error envelope in both adapters.
7. Add the operation to OpenAPI, MCP `tools/list`, this catalog and relevant examples.
8. Extend the capability-parity test with success, validation and cross-adapter state
   cases.
9. Confirm that provisioning, administration and operations remain unpublished.
10. Update [Aicadia MVP](README.md) in the same change.

If any item is missing, the capability does not ship. A UI may consume a shipped
capability but can never be its only implementation.
