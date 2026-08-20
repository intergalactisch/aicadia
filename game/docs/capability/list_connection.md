# `list_connection`

> **Role / side:** one capability contract / runtime side.
> **Authority:** the World-side contract of `list_connection`: incident-Connection selection, summary shape, validation and retry class.
> **Excludes:** Agent wording — published from [Agent contract sources](../../mcp/agent/README.md); complete course reads — narrowed in [`get_connection`](get_connection.md); canonical errors — defined in [canonical errors](../protocol.md#canonical-errors).

## Purpose

List bounded summaries of immutable Connections touching one exact shared Place.

## Input

World call `list_connection(context.user_id, input)`; HTTP
`GET /api/place/{place_id}/connection`; MCP `list_connection`.

```rust
struct ListConnection {
    place_id: EntityId,
    cursor: Option<String>,
    limit: Option<u16>,
}
```

## Validation

The contextual User must own a Character with a current Position. `place_id` must
name an existing positioned Place. Limit defaults to 25 and is 1–100. The opaque
cursor binds the anchor Place and last Connection id.

## Result

Returns `{ place, connection, next }`. Each summary contains stable Connection id,
source and destination Place summaries with complete endpoint Positions,
`allows_reverse`, name, description and `has_course`. It omits shape description and
course points. Connections incident as source or destination are merged once and
ordered by id bytes ascending. The response has no total count, traveller state or
semantic deduplication.

## Activity footprint

None. Reads are not Activity.

## Annotations and retry class

Read-only and idempotent. A continuation copies `next` unchanged and repeats the
same Place anchor.

## Evidence obligations

World, HTTP and MCP must expose identical incident selection, summary, pagination
and errors. Evidence must prove one bounded query plan and no course hydration.
