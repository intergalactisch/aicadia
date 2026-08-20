# `get_connection`

> **Role / side:** one capability contract / runtime side.
> **Authority:** the World-side contract of `get_connection`: anchored identity, complete immutable course result, validation and retry class.
> **Excludes:** Agent wording — published from [Agent contract sources](../../mcp/agent/README.md); Connection identity and course invariants — defined in [Connection](../model/connection/README.md); canonical errors — defined in [canonical errors](../protocol.md#canonical-errors).

## Purpose

Get one selected Connection incident to one exact Place, including its complete
bounded optional course.

## Input

World call `get_connection(context.user_id, input)`; HTTP
`GET /api/place/{place_id}/connection/{connection_id}`; MCP `get_connection`.

```rust
struct GetConnection {
    place_id: EntityId,
    connection_id: ConnectionId,
}
```

## Validation

The contextual User must own a Character with a current Position. The selected
Connection must exist and be incident to the selected existing Place. An absent
Connection, absent Place or valid Connection not incident to that Place shares one
neutral `connection_not_found` result.

## Result

Returns the complete Connection: stable id, source and destination Place summaries
with endpoint Positions, direction, name, description, optional shape description
and either zero or 2–128 ordered course points. It returns no current traveller,
progress, total count or inferred terrain.

## Activity footprint

None. Reads are not Activity.

## Annotations and retry class

Read-only and idempotent.

## Evidence obligations

World, HTTP and MCP must expose one identical bounded result, neutral anchored lookup
and canonical errors. Evidence must prove no unselected Connection course is
hydrated.
