# Agent interface

This is the current player-facing wire contract. `World` owns semantics; local HTTP
and MCP expose the same ten capabilities. PostgreSQL, migrations, provisioning and
operational controls remain behind that seam.

## Capability catalog

Catalog order is deterministic:

| Capability | World call | HTTP | MCP | User context |
| --- | --- | --- | --- | --- |
| `get_world` | `get_world()` | `GET /api/world` | `get_world` | absent |
| `get_user` | `get_user(context.user_id)` | `GET /api/user` | `get_user` | required |
| `get_character` | `get_character(context.user_id)` | `GET /api/character` | `get_character` | required |
| `create_character` | `create_character(context.user_id, input)` | `POST /api/character` | `create_character` | required |
| `create_entry_place` | `create_entry_place(context.user_id, input)` | `POST /api/place/entry` | `create_entry_place` | required |
| `enter_world` | `enter_world(context.user_id)` | `POST /api/world/entry` | `enter_world` | required |
| `list_activity` | `list_activity(context.user_id, input)` | `GET /api/activity` | `list_activity` | required |
| `list_entity` | `list_entity(input)` | `GET /api/entity` | `list_entity` | absent |
| `get_entity` | `get_entity(input.entity_id)` | `GET /api/entity/{entity_id}` | `get_entity` | absent |
| `create_entity` | `create_entity(context.user_id, input)` | `POST /api/entity` | `create_entity` | required |

`create_user` is deliberately absent. Database creation, migration, diagnostics,
administration and every other operational action are not Agent capabilities.

## Request context

Context-required operations receive exactly one `Aicadia-User-Id` UUID header. It is
untrusted development context, not authentication. Missing, malformed, duplicate,
comma-joined or unknown values are rejected before game behavior succeeds. Capability
input never accepts a User id. Character, entry Place and personal activity operations
also accept no Character id; `enter_world` accepts no Place id or payload.

Both supported MCP protocol revisions (`2025-11-25` and `2026-07-28`) apply the same
header and game contract to every request. Current MCP is stateless. The newer
revision receives complete public cache metadata with `ttlMs: 0`.

## Recommended World-entry flow

An Agent needs no User-supplied id, Place choice or confirmation:

1. Call `get_character`.
2. Only when it returns `character_not_found`, call `create_character` with the
   Character's semantic name and description. Creation deliberately returns
   `current_place: null`; introducing a Character does not place it.
3. If the Character's `current_place` is complete rather than null, it has already
   entered and the flow is finished.
4. Otherwise call `enter_world` with empty input. World derives both the Character
   and the one entry Place.
5. Only when `enter_world` returns `entry_place_not_found`, call
   `create_entry_place` once with the semantic name and description for World
   genesis, then call `enter_world` again.
6. If `create_entry_place` returns `entry_place_already_exists`, another concurrent
   Agent won genesis. Do not propose another Place; call `enter_world` again.
7. Call `list_activity` when accepted-action history is relevant. A delivery retry
   of successful `enter_world` returns the same placement without adding another
   Activity.

This first-use error path is deliberate because zero entry Places is valid before
genesis. `create_entry_place` never creates later Places, and no current tool performs
movement, discovery or arbitrary placement.

## Wire shapes

All JSON objects reject unknown fields. Successful operations return the result
directly without a `data` envelope. Timestamps are RFC 3339 strings and ids are UUID
strings.

```text
World       { name }
User        { id, created_at }
Entity      { id, name, description, introduced_by_user_id, introduced_at }
Place       { entity: Entity, is_entry }
Character   { entity: Entity, owner_user_id, current_place: Place | null }
EntitySummary { id, name }
PlaceSummary  { entity: EntitySummary, is_entry }

ActivityEntityReference {
  entity: EntitySummary,
  role: "subject" | "destination"
}
Activity {
  id,
  operation: "create_character" | "create_entity" |
             "create_entry_place" | "enter_world",
  actor_character: EntitySummary | null,
  context_place: PlaceSummary | null,
  involved_entity: [ActivityEntityReference],
  occurred_at
}
EntityPage   { entity: [EntitySummary], next: string | null }
ActivityPage { activity: [Activity], next: string | null }
```

`requested_by_user_id` is internal activity provenance and is not exposed by
`list_activity`.

### Inputs

`create_character`, `create_entry_place` and `create_entity` accept exactly:

```json
{"name": "North Gate", "description": "The one established entry Place."}
```

Both strings use the semantic bounds in [the build contract](README.md). HTTP
`enter_world` has no request body; MCP supplies the required empty object. `get_world`,
`get_user` and `get_character` likewise use empty MCP input.

`get_entity` accepts exactly `entity_id`. Entity and activity lists accept optional
`cursor` and `limit`; limit defaults to 25 and must be 1 through 100. Each cursor is
an opaque URL-safe string tied to its list type. Clients copy `next` unchanged and
must not decode, edit or reuse it across list operations.

## HTTP contract

- Reads return `200 OK`.
- `create_character`, `create_entry_place` and `create_entity` return `201 Created`.
- `enter_world` returns `200 OK` on first acceptance and delivery retries.
- JSON/query decoding failures return canonical `invalid_request` errors.
- `GET /api/openapi.json` publishes exactly the ten operation IDs above with shared
  schemas and no `create_user`.

The server binds only to loopback. MCP accepts an absent `Origin` for non-browser
clients, accepts the server's exact local origin, and rejects foreign origins.

## MCP tool descriptions and annotations

| Tool | Behavioral description | Annotation summary |
| --- | --- | --- |
| `get_world` | Get the identity of the one persistent shared World. | read-only, idempotent |
| `get_user` | Get the durable User derived from request context; accepts no id. | read-only, idempotent |
| `get_character` | Get the current User's Character including nullable complete Place; accepts no ids. | read-only, idempotent |
| `create_character` | Create the current User's one unplaced Character Entity role; accepts no ids. | additive, non-idempotent |
| `create_entry_place` | Create the one shared entry Place from the current unplaced Character; accepts semantic text only. | additive, non-idempotent |
| `enter_world` | Place the current unplaced Character at the server-derived entry Place; retry returns the same placement. | modifying, idempotent |
| `list_activity` | List activity involving the derived current Character, newest first. | read-only, idempotent |
| `list_entity` | List shared Entity summaries, newest first. | read-only, idempotent |
| `get_entity` | Get one shared Entity by stable id. | read-only, idempotent |
| `create_entity` | Create one shared stable referent; equal retries create another Entity. | additive, non-idempotent |

Every tool declares `destructiveHint: false` and `openWorldHint: false`. The exact
descriptions, JSON Schemas and annotations are compiler-generated and fixed by
`tests/agent-tool-catalog.json`.

## Canonical errors

```json
{
  "error": {
    "code": "invalid_place",
    "message": "Place name is empty.",
    "field": "name",
    "reason": "empty"
  }
}
```

| Code | Meaning | HTTP |
| --- | --- | --- |
| `user_context_required` | context header absent | `400` |
| `invalid_request` | malformed header, body, query, id or cursor | `400` |
| `invalid_entity` | Entity semantic text invalid | `400` |
| `invalid_character` | Character semantic text invalid | `400` |
| `invalid_place` | Place semantic text invalid | `400` |
| `invalid_entity_limit` | Entity limit outside 1 through 100 | `400` |
| `invalid_activity_limit` | activity limit outside 1 through 100 | `400` |
| `user_not_found` | contextual User absent | `404` |
| `entity_not_found` | selected Entity absent | `404` |
| `character_not_found` | contextual User owns no Character | `404` |
| `entry_place_not_found` | World genesis has not established an entry Place | `404` |
| `character_already_exists` | contextual User already owns a Character | `409` |
| `character_already_entered` | operation requires an unplaced Character | `409` |
| `entry_place_already_exists` | World already has its one entry Place | `409` |
| `unavailable` | World storage could not complete the request | `503` |

MCP game failures are successful JSON-RPC tool responses with `isError: true` and one
text content block containing the same error object. Protocol framing, unknown tools,
unsupported versions and origin rejection remain MCP protocol errors outside this
game error contract.

## Parity evidence

Automated tests require:

1. OpenAPI operation ids and MCP tool names equal the exact ten-name catalog.
2. MCP descriptions, annotations and schemas equal the checked-in fixture.
3. Character creation and reads expose `current_place: null`, then both adapters
   expose the complete same entry Place after World entry.
4. Entry Place creation through one adapter is used by entry through the other.
5. HTTP and MCP activity pagination share the same opaque cursor semantics.
6. Both adapters return the same canonical context, semantic, not-found and conflict
   errors.
7. Legacy stateful and current stateless MCP transports expose all ten tools while
   preserving the same game behavior.

The bounded live Agent playtest exercises this complete entry flow, personal Activity
comprehension and cross-User Entity observation. Its preflight verifies the complete
ten-capability catalog even though each playtest Agent receives only the
least-privilege subset needed for its role.
