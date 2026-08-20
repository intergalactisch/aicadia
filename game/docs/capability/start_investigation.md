# `start_investigation`

> **Role / side:** one capability contract / runtime side.
> **Authority:** the World-side contract of `start_investigation`: typed kind selection, admission, Position grounding, stable result and retry class.
> **Excludes:** Agent wording — published as [its tool description](../../mcp/agent/tool/start_investigation.md); investigation conduct — defined in [Required investigation and discovery flow](../agent.md#required-investigation-and-discovery-flow); canonical errors — defined in [canonical errors](../protocol.md#canonical-errors).

## Purpose

Ask World to admit and resolve one independent investigation for the entered
Character at its exact current Position before the Agent authors a result.

## Input

World call `start_investigation(context.user_id, input)`; HTTP
`POST /api/investigation`; MCP `start_investigation`.

## Contract

```rust
struct StartInvestigation {
    request_id: Uuid,
    kind: DiscoveryKind,
}

enum DiscoveryKind {
    EntityAtPosition,
    ConnectedPlace,
}

struct InvestigationResult {
    attempt_id: Uuid,
    outcome: InvestigationOutcome,
    limit: InvestigationLimit,
}

struct InvestigationLimit {
    result_count: u8, // exactly 1
    kind: DiscoveryKind,
}
```

The Agent selects `entity_at_position` or `connected_place` from current
authoritative reads. Input accepts no User, Character, Place, Position, prose, seed,
odds, result count or retry count. World derives the User's Character, exact current
Position revision and optional current Place.

Under the User lock, World first resolves an existing `(User, request_id)` attempt.
The same kind returns the stored attempt; the other kind returns
`investigation_request_conflict`. Otherwise World uses PostgreSQL time for the
inclusive rolling-hour admission boundary and stored time, derives the bounded
Place signal or zero at a loose Position, resolves one authoritative chance roll and
stores one attempt. Admission and chance values — defined in
[investigation chance and admission](../model/investigation-attempt/README.md#investigation-chance-and-admission);
this capability adds no configurable value.

## Input example

```json
{
  "request_id": "0fb81b4f-70b4-4a9c-9850-44ffc28212c6",
  "kind": "connected_place"
}
```

## Validation

A missing Character returns `character_not_found`; a Character without Position
returns `character_not_entered`. A full admission window rejects before a roll and
stores nothing. Only a newly inserted positive can void the oldest prior live
positive when the per-User bound is exceeded; zero never voids another attempt and
the new attempt never voids itself.

## Result

Returns the World-assigned attempt id, stored `zero` or `positive` outcome and
immutable `{ result_count: 1, kind }` limit. It contains no Position, Place, count,
odds, server prose, seed or semantic suggestion. Start creates no Entity, Place,
Connection, Activity or current state.

## Activity footprint

None. Attempt provenance is internal and is not player-visible history.

## Annotations and retry class

Idempotent internal attempt creation by `(User, request_id, kind)`, with no Activity
or current World-state change. The request id belongs to the attempt namespace. A
same-kind retry never rerolls; another kind conflicts.

## Evidence obligations

World, HTTP and MCP must expose the same kind, admission, Position grounding,
stable-result, retry and error semantics. Chance-dependent tests use a private
scripted source and never make a probabilistic claim.
