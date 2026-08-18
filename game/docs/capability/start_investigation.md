# `start_investigation`

> **Role / side:** one capability contract / runtime side.
> **Authority:** what World accepts, validates, stores and records for `start_investigation` — admission and resolution of one investigation attempt.
> **Excludes:** how an Agent words this to a player — published as [its tool description](../../mcp/agent/tool/start_investigation.md); the investigation and discovery conduct — defined in [Required investigation and discovery flow](../agent.md#required-investigation-and-discovery-flow); error codes and their transport mapping — defined in [canonical errors](../protocol.md#canonical-errors).

## Purpose

Ask World to admit and resolve one independent investigation for the entered
Character at its exact current Place before the Agent authors any find.

## Input

World call `start_investigation(context.user_id, input)`; HTTP `POST /api/investigation`; MCP `start_investigation`. Input is `StartInvestigation` below.

## Contract

```rust
struct StartInvestigation {
    request_id: Uuid,
}

struct InvestigationResult {
    attempt_id: Uuid,
    outcome: InvestigationOutcome, // Zero | Positive
    limit: InvestigationLimit,
}

struct InvestigationLimit {
    result_count: u8, // exactly 1
    kind: DiscoveryKind, // EntityAtCurrentPlace
}
```

Input accepts no User, Character, Place, focus, prose, seed, odds, result count or
retry count. World derives the User's Character and exact current Place.

Under the User lock, World first resolves an existing
`(requested_by_user_id, request_id)` attempt. Otherwise it uses PostgreSQL time to
apply the bounded per-User rolling-hour admission window, reads only a bounded tail
of Activities at the exact Place, resolves one server-authoritative chance roll and
stores one `zero` or `positive` attempt. A rejected admission stores nothing and
performs no roll. Only a newly inserted positive attempt can cause the oldest live
positive to be voided once the per-User live-positive bound is exceeded; the
candidate must be a prior live positive with `id <> new_attempt_id`, so the new
attempt never voids itself. Every window, admission and chance value — defined in
[investigation chance and admission](../model/investigation-attempt/README.md#investigation-chance-and-admission) —
stays an operational fact, never a player-visible mechanic or input.

The returned attempt id, stored outcome and immutable limit are the complete
result. They contain no mutable Place context, counts, odds, server prose, seed or
semantic discovery direction. A positive permits at most one found Entity at that
attempt's Place. `result_count` is that positive-attempt cap, not the number of finds
created by start; it remains `1` in the retry-stable zero body. Start does not itself
create an Entity, Activity or current state.

## Input example

```json
{
  "request_id": "0fb81b4f-70b4-4a9c-9850-44ffc28212c6"
}
```

## Validation

A missing Character returns `character_not_found` and an unplaced Character returns `character_not_entered`; a full per-User admission window returns `investigation_not_admitted` before any roll and stores nothing. Admission window and chance values — defined in [investigation chance and admission](../model/investigation-attempt/README.md#investigation-chance-and-admission); start retry identity — defined in [investigation retry identity](../protocol.md#investigation-retry-identity); this capability adds only the three local outcomes above.

## Result

```json
{
  "attempt_id": "27bb3450-4159-462f-bd9b-ce5617ceef21",
  "outcome": "positive",
  "limit": {
    "result_count": 1,
    "kind": "entity_at_current_place"
  }
}
```

`zero` has the same shape and ends that attempt without Activity or Place-pointer
change.

## Activity footprint

None. The internal attempt row is retry, admission and one-time-consumption
provenance, not player-visible history. Rejection and zero likewise write no
Activity and do not advance `place.latest_activity_id`.

## Annotations and retry class

Idempotent internal attempt creation by request id, with no Activity or current World-state change. The start request id belongs to the investigation-attempt namespace, separate from
the Activity namespace used by state-changing operations. Because `request_id` is
the only semantic input, an equal `(User, request_id)` always returns the same stored
body and never rerolls; there is no start fingerprint or content-conflict error.
Reusing the same UUID once in each namespace is valid.

## Evidence obligations

World, HTTP and MCP must expose the same admission, stable-result, retry and error
contract. Chance-dependent tests use a private scripted source and never make a
probabilistic assertion.
