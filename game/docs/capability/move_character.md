# `move_character`

> **Role / side:** one capability contract / runtime side.
> **Authority:** the World-side contract of `move_character`: exact traversal input, validation, Character state change, Activity footprint and retry class.
> **Excludes:** Agent wording — published from [Agent contract sources](../../mcp/agent/README.md); Connection and Position state — defined in [Connection](../model/connection/README.md) and [Position](../model/position/README.md); canonical errors — defined in [canonical errors](../protocol.md#canonical-errors).

## Purpose

After explicit User confirmation, move the contextual Character completely over one
allowed Connection or partially to one exact point on its shaped course.

## Input

World call `move_character(context.user_id, input)`; HTTP
`POST /api/character/movement`; MCP `move_character`.

## Contract

```rust
struct MoveCharacter {
    request_id: Uuid,
    connection_id: ConnectionId,
    expected_position_revision: PositionRevision,
    direction: MovementDirection,
    target: MovementTarget,
}

enum MovementDirection {
    SourceToDestination,
    DestinationToSource,
}

enum MovementTarget {
    Complete,
    Partial {
        origin_segment_ordinal: u8,
        target_segment_ordinal: u8,
        x_cm: i64,
        y_cm: i64,
        z_cm: i64,
    },
}
```

Input accepts no User, Character, Place, duration, travel cost, terrain, journey or
prose. World derives the controlled Character. `complete` means the exact opposite
endpoint in the submitted allowed direction. `partial` is admitted only for a shaped
Connection and names the segment containing the current Position, the segment
containing the target point and that exact point.

## Validation

World resolves an accepted retry first, then locks the responsible User and
Character and compares the expected Position revision. At an endpoint, Character
current Place and endpoint must agree. After a partial move, the unique
non-self-intersecting course plus `origin_segment_ordinal` establishes current
progress. Direction must be allowed; the target must lie exactly on the selected
course segment and make strict forward progress in that direction. Complete travel
over an unshaped Connection is valid only from its exact endpoint.

All segment, intersection and progress checks use checked integer cross and dot
products. Invalid, stale, reverse-disallowed, off-course and non-progress input
writes nothing.

## Result

One acceptance appends a new Character Position version and one Activity atomically.
Complete arrival uses the exact opposite endpoint coordinates and sets that Place as
current. Partial travel uses the submitted exact point and clears current Place. The
new Character Position description is absent; World never copies or interprets the
Connection or endpoint description as Character state.
No Connection row, journey, traveller state, timer, fraction, trace or count changes.
Another Character can traverse the same Connection concurrently and writes only its
own state.

The canonical result is `{ activity, character, connection }`; Character contains
the accepted Position and optional current Place, while Connection is the complete
immutable selected value.

## Activity footprint

One immutable `move_character` Activity records the Character as actor, old Position
as `origin`, new Position as `result` and the traversed Connection. The origin Place
is `location` when present; complete arrival also records the destination Place as
`destination` and context. Movement Activity has no prose.

## Annotations and retry class

Modifying and irreversible World history; idempotent by Activity request id and the
complete normalized Movement input. The same id and input returns the accepted
result. Changed input or reuse across another Activity operation returns
`movement_request_conflict`. The call triggers no Agent, notification or background
process.

## Evidence obligations

World, HTTP and MCP must prove unshaped completion, shaped forward and reverse
completion, partial progress, exact retry, stale Position, rollback, 500-millisecond
lock timeout and independent travellers with the same semantic result and errors.
