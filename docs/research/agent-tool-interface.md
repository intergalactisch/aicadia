---
status: load-bearing
era: August Activity-Property-Trait
---

# Agent tool interface

> **Role / side:** sourced research report / development side.
> **Authority:** records this report's question, sources, findings and implications.
> **Excludes:** product decisions and current implementation contracts; see `docs/game/`.

Status: research, checked 2026-08-07

## Question

What is the smallest correct way to expose Aicadia's Rust `World` interface to
AI agents as MCP tools, including names, input and output schemas, discovery,
transport and errors?

This record uses only the published MCP `2026-07-28` specification and the
official Rust SDK maintained by the Model Context Protocol project. It does not
select an Aicadia implementation by itself; accepted current behavior belongs in
`docs/game/`.

## Findings from primary sources

### Tools and schemas

MCP tools are server-exposed functions discovered through `tools/list` and invoked
through `tools/call`. A tool definition has a unique programmatic `name`, a useful
human-readable `description`, an `inputSchema` and optionally an `outputSchema`.
Tool names should be 1 through 128 characters, case-sensitive, unique within the
server, and limited to ASCII letters, digits, underscore, hyphen and dot. Both
schemas use JSON Schema 2020-12 by default. An input schema is an object; a tool
without arguments should explicitly reject properties. An output schema may
describe any JSON value in protocol revision `2026-07-28`.
([MCP tools specification](https://modelcontextprotocol.io/specification/2026-07-28/server/tools))

A successful structured result is returned in `structuredContent` and must conform
to `outputSchema` when that schema is present. For compatibility, the specification
also recommends returning a serialized representation in a text content block.
Tool annotations such as `readOnlyHint`, `destructiveHint`, `idempotentHint` and
`openWorldHint` are hints, not authorization or trusted enforcement data.
([MCP tools specification](https://modelcontextprotocol.io/specification/2026-07-28/server/tools))

### Discovery and capability negotiation

An MCP `2026-07-28` server must implement `server/discover`. Its result advertises
supported protocol versions and server capabilities. A client may skip discovery
and make a call directly, but discovery gives it the capability set and supported
versions in one response. A server that advertises the `tools` capability must
answer `tools/list`. Tool order should be deterministic; the result can be cached.
The `listChanged` capability is only appropriate when the server can notify clients
that its tool catalog changed. Modern `server/discover` and `tools/list` complete
results must include a non-negative `ttlMs` and a `cacheScope`; `public` is suitable
when the catalog is identical for every caller.
([MCP discovery](https://modelcontextprotocol.io/specification/2026-07-28/server/discover),
[MCP tools specification](https://modelcontextprotocol.io/specification/2026-07-28/server/tools),
[MCP caching](https://modelcontextprotocol.io/specification/2026-07-28/server/utilities/caching))

Revision `2026-07-28` has no initialization handshake and no protocol-level
session. Every request carries its protocol version and client capabilities in
`_meta.io.modelcontextprotocol/*`. If application state must span calls, the
application returns an explicit opaque handle and requires the caller to supply it
again; it must not hide that state in a connection session.
([MCP transport overview](https://modelcontextprotocol.io/specification/2026-07-28/basic/transports),
[stateful-tool guidance](https://modelcontextprotocol.io/specification/2026-07-28/server/tools#stateful-tools))

### Transports

The two standard transports are:

- `stdio`: a client launches the server as a subprocess and exchanges one
  newline-delimited JSON-RPC message per line over standard input and output.
  Logging may use standard error; no other text may be written to standard output.
  ([MCP stdio transport](https://modelcontextprotocol.io/specification/2026-07-28/basic/transports/stdio))
- Streamable HTTP: a server exposes one MCP endpoint accepting POST. Every JSON-RPC
  message is a separate request and the response is either one JSON object or a
  request-scoped SSE stream. Revision `2026-07-28` removed the standalone GET stream
  and protocol session. Streamable HTTP servers must validate `Origin`; local
  servers should bind to `127.0.0.1`; and servers should authenticate connections.
  ([MCP Streamable HTTP transport](https://modelcontextprotocol.io/specification/2026-07-28/basic/transports/streamable-http))

For Streamable HTTP, every request carries `MCP-Protocol-Version` and `Mcp-Method`.
A `tools/call` also carries `Mcp-Name`. These headers mirror body data; a server
must reject a mismatch. The JSON-RPC body remains the source of truth.
([MCP Streamable HTTP request metadata](https://modelcontextprotocol.io/specification/2026-07-28/basic/transports/streamable-http#request-metadata))

### Errors

The specification separates two mechanisms:

- Protocol errors are JSON-RPC errors for an unknown tool, a malformed
  `tools/call` envelope, an unsupported operation or an exceptional server failure.
- Tool execution errors are ordinary tool results with `isError: true` and
  actionable text. Input validation, downstream API failures and business-rule
  failures belong here so the model can correct its arguments and retry.

Servers must validate tool input and sanitize output. Clients should show tool
input to the user before sensitive operations, impose timeouts and make tool
execution errors available to the model.
([MCP tool error handling and security](https://modelcontextprotocol.io/specification/2026-07-28/server/tools#error-handling))

### Authorization

MCP authorization is optional. When authorization is implemented for HTTP, the MCP
authorization specification applies; stdio implementations should obtain any
credentials from their environment rather than run the HTTP authorization flow.
This makes OAuth optional for a local, non-public Aicadia MVP. It does not make an
unauthenticated public write endpoint safe: Streamable HTTP still recommends
authentication, and its mandatory `Origin` validation is not user identity or
authorization.
([MCP authorization](https://modelcontextprotocol.io/specification/2026-07-28/basic/authorization),
[MCP Streamable HTTP security](https://modelcontextprotocol.io/specification/2026-07-28/basic/transports/streamable-http#security--endpoint))

### Official Rust implementation

`rmcp` is the official Tokio-based Rust SDK. Its server feature and tool macros can
derive tool input and output schemas from Rust types using Serde and Schemars, and
its Streamable HTTP service can be mounted as a Tower service. Version `3.0.0`
introduced MCP `2026-07-28` support; the latest official release on the research
date is `3.1.1`. The SDK serves modern Streamable HTTP requests statelessly and
expects shared persistent state such as a PostgreSQL pool to be held outside the
per-request handler.
([official Rust SDK](https://github.com/modelcontextprotocol/rust-sdk),
[`rmcp` 3.0 release](https://github.com/modelcontextprotocol/rust-sdk/releases/tag/rmcp-v3.0.0),
[`rmcp` 3.1.1 release](https://github.com/modelcontextprotocol/rust-sdk/releases/tag/rmcp-v3.1.1))

## Implications and recommendation for Aicadia

### Smallest boundary

Use this call path:

```text
agent -> MCP adapter -> World -> PostgreSQL
web client -> HTTP API adapter -> World -> PostgreSQL
```

The MCP adapter must not call the HTTP API and the HTTP API must not call the MCP
adapter. Both are thin transport adapters over the same concrete `World` core. They
may share transport DTO definitions so their field names, validation expectations
and successful data shapes cannot drift. Only `World` owns game validation and
storage behavior.

Every current operation needed for play should be available to an agent as one MCP
tool with the exact core operation name. Internal lifecycle operations do not become
game tools merely because they are public Rust methods. In the current contract,
`create_user` remains provisioning for core setup and tests, so it is not an MCP
tool. This gives the following complete initial catalog, in this deterministic
order:

1. `get_world`
2. `get_user`
3. `list_entity`
4. `get_entity`
5. `create_entity`

Use these snake-case names without a redundant `aicadia_` prefix: the MCP server
already scopes uniqueness. If an agent host aggregates multiple servers, that host
owns collision disambiguation.

### Proposed tool contract

All object schemas should set `additionalProperties: false`. UUIDs are JSON strings
with `format: uuid`; timestamps are UTC RFC 3339 strings with `format: date-time`.
The adapter supplies documented defaults before calling the core. Every field shown
in an output shape below is required; `next` is required but nullable. Every input
field is required unless the table calls it optional.

| Tool | Input | Structured output | Annotations |
|---|---|---|---|
| `get_world` | empty object | `{ name }` | read-only, idempotent, non-destructive, closed-world |
| `get_user` | `{ user_id }` | `{ id, created_at }` | read-only, idempotent, non-destructive, closed-world |
| `list_entity` | optional `cursor`, optional `limit` defaulting to 25 | `{ entity, next }` | read-only, idempotent, non-destructive, closed-world |
| `get_entity` | `{ entity_id }` | full Entity | read-only, idempotent, non-destructive, closed-world |
| `create_entity` | `{ introduced_by_user_id, name, description }` | full Entity | modifying, additive, non-idempotent, closed-world |

`closed-world` above means `openWorldHint: false`: each tool touches only Aicadia,
not arbitrary external systems. These annotations communicate behavior; the server
must enforce behavior independently.

The reusable output shapes are:

```text
WorldView    = { name: string }
User         = { id: uuid, created_at: date-time }
Entity       = {
  id: uuid,
  name: string,
  description: string,
  introduced_by_user_id: uuid,
  introduced_at: date-time
}
EntityCursor = { introduced_at: date-time, entity_id: uuid }
EntitySummary = { id: uuid, name: string }
EntityPage   = { entity: EntitySummary[], next: EntityCursor | null }
```

The tool descriptions are part of the model-facing contract and should say exactly:

- `get_world`: "Return the shared Aicadia World. No User is required."
- `get_user`: "Return one existing User by id. This does not authenticate the
  caller."
- `list_entity`: "List shared Entities newest first. Pass `next` back as `cursor`
  to continue. `limit` defaults to 25 and must be 1 through 100."
- `get_entity`: "Return one shared Entity by id."
- `create_entity`: "Create one stable World referent introduced by an existing
  User. This records who introduced the Entity; it does not mean the User created,
  owns or discovered the subject in the fiction. Names need not be unique. Repeating
  a successful call creates another Entity."

The `create_entity` input schema must require all three fields. It should document
the World constraints: both text fields are trimmed; `name` contains 1 through 120
Unicode characters; `description` contains 1 through 4,000; and neither may contain
U+0000. The adapter should deserialize and then let `World::create_entity` perform
the authoritative validation.

The `list_entity` cursor should keep the core shape instead of introducing a second
opaque token format in the first MVP. Its schema is an optional `EntityCursor`; the
successful output always contains `next`, either another cursor object or `null`.

### Result and error mapping

For each success, return:

- `resultType: "complete"` as required by the modern protocol model;
- `structuredContent` matching the declared output schema;
- one text content block containing compact serialized JSON for compatibility;
- `isError: false` or omit it when the SDK does so conventionally.

Map `WorldError` to tool execution errors as follows. Each result sets
`isError: true`, omits `structuredContent`, and contains the listed actionable text
without SQL, credentials or stack traces:

| World error | Tool error text |
|---|---|
| invalid name or description | `invalid_entity: <field> is <empty, too long, or contains U+0000>` |
| invalid list limit | `invalid_entity_limit: limit must be from 1 through 100` |
| User not found | `user_not_found: introduced_by_user_id does not identify an existing User` for `create_entity`; `user_not_found: user_id does not identify an existing User` for `get_user` |
| Entity not found | `entity_not_found: entity_id does not identify an existing Entity` |
| storage unavailable | `world_unavailable: the World could not complete the request; retry later` |

Unknown tool names, malformed JSON-RPC and invalid request envelope shapes remain
protocol errors. A syntactically valid tool call with invalid game arguments is a
tool execution error, including JSON arguments that deserialize but violate a World
constraint. Transport/schema deserialization failures require an adapter test to
ensure they are surfaced in the model-visible category expected by the MCP
specification rather than leaking an internal SDK error.

### Discovery

Advertise only the `tools` server capability. Do not advertise prompts, resources,
sampling, roots, tasks, subscriptions or dynamic tool-list changes. The catalog is
static for this MVP, so `listChanged` is absent or false. Return the five tools in
the order above and give `tools/list` a public cache scope with a finite TTL; a code
release changes the catalog and therefore invalidates that cached response.

Use server name `aicadia` and a build version for diagnostics only. Do not treat
client information from request metadata as a User identity: it is self-reported
protocol metadata, not authentication. Until authentication exists,
`introduced_by_user_id` remains an explicit `create_entity` argument whose existence
the World checks. Do not add `x-mcp-header` annotations: none of these five inputs
needs intermediary routing, and the standard `Mcp-Name` header already identifies
the invoked tool.

### Transport and deployment

Implement one transport first: stateless Streamable HTTP at `/mcp`. Aicadia is one
remote shared World, whereas stdio describes a client-launched local subprocess. A
second stdio binary would add a second deployment shape without enabling the shared
multiplayer loop. The official SDK's in-process transport is sufficient for adapter
tests.

For the unauthenticated MVP, bind the HTTP service to `127.0.0.1`, validate `Origin`
and do not expose it to an untrusted network. OAuth and account binding remain
deferred. Before a write-capable `/mcp` or HTTP API becomes publicly reachable,
authentication and authorization must be introduced together so the adapter derives
the acting User instead of trusting arbitrary `introduced_by_user_id` input.

Use the current published `rmcp` release, pinned through Cargo rather than a Git
`main` dependency. Enable only the server, schema and Streamable HTTP features
needed by this adapter; do not enable OAuth, tasks, prompts or resources.

## Verification target

A later implementation should not be considered complete until automated tests
prove:

- `server/discover` advertises protocol `2026-07-28` and only the tools capability;
- `tools/list` returns exactly the five documented tools in deterministic order,
  with schemas and annotations matching this contract;
- every successful MCP tool returns the same data as calling the corresponding
  `World` method directly;
- every `WorldError` maps to the documented model-visible tool execution error;
- unknown tools and malformed protocol envelopes remain JSON-RPC errors;
- two independent MCP requests observe the same PostgreSQL-backed World without an
  MCP or domain session;
- Streamable HTTP rejects an invalid `Origin` and mismatched protocol routing
  headers;
- the MCP conformance suite for `2026-07-28` passes for the implemented capability
  set.

## Sources

- [MCP specification: tools](https://modelcontextprotocol.io/specification/2026-07-28/server/tools)
- [MCP specification: discovery](https://modelcontextprotocol.io/specification/2026-07-28/server/discover)
- [MCP specification: transport overview](https://modelcontextprotocol.io/specification/2026-07-28/basic/transports)
- [MCP specification: stdio](https://modelcontextprotocol.io/specification/2026-07-28/basic/transports/stdio)
- [MCP specification: Streamable HTTP](https://modelcontextprotocol.io/specification/2026-07-28/basic/transports/streamable-http)
- [MCP specification: authorization](https://modelcontextprotocol.io/specification/2026-07-28/basic/authorization)
- [MCP specification: caching](https://modelcontextprotocol.io/specification/2026-07-28/server/utilities/caching)
- [Official MCP Rust SDK](https://github.com/modelcontextprotocol/rust-sdk)
- [Official `rmcp` 3.1.1 release](https://github.com/modelcontextprotocol/rust-sdk/releases/tag/rmcp-v3.1.1)
