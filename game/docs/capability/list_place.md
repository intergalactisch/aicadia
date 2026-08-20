# `list_place`

> **Role / side:** one capability contract / runtime side.
> **Authority:** the World-side contract of `list_place`: one bounded shared Place window, validation, result and retry class.
> **Excludes:** Agent wording — published from [Agent contract sources](../../mcp/agent/README.md); spatial play conduct — defined in [Required spatial exploration flow](../agent.md#required-spatial-exploration-flow); canonical errors — defined in [canonical errors](../protocol.md#canonical-errors).

## Purpose

List ordinary shared Places whose exact current Position lies inside one inclusive
axis-aligned World box.

## Input

World call `list_place(context.user_id, input)`; HTTP `GET /api/place`; MCP
`list_place`.

```rust
struct ListPlace {
    min_x_cm: i64,
    max_x_cm: i64,
    min_y_cm: i64,
    max_y_cm: i64,
    min_z_cm: i64,
    max_z_cm: i64,
    cursor: Option<String>,
    limit: Option<u16>,
}
```

## Validation

The contextual User must own a Character with a current Position. Each minimum must
not exceed its maximum, every coordinate must fit the Position range and each axis
span must be at most `100_000_000` centimetres. Limit defaults to 25 and is 1–100.
The opaque cursor binds the complete box and last sort tuple; changing either box or
operation invalidates it.

## Result

Returns `{ place, next }`. Each item contains the Place Entity's id, name and
description, `is_entry` and complete current Position. Results order by
`(x_cm, y_cm, z_cm, place_entity_id)` ascending and contain at most the requested
limit. The response has no total count, Connections, course points, Entity occupants,
Knowledge filter or read receipt.

Every selected candidate is rechecked against canonical current Position before it
is returned. Concurrent inserts before the cursor may require a fresh first page;
pagination promises bounded stable continuation, not a cross-call snapshot or exact
enumeration.

## Activity footprint

None. Reads never create Activity, Observation or Knowledge.

## Annotations and retry class

Read-only and idempotent for the per-call database snapshot. A continuation copies
`next` unchanged with the same box.

## Evidence obligations

World, HTTP and MCP must expose the same box, ordering, page, cursor and error
semantics. Production PostgreSQL evidence must prove canonical recheck and bounded
work for dense and sparse windows without a sequential scan or unbounded hydration.
