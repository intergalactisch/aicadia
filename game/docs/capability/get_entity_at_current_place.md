# `get_entity_at_current_place`

> **Role / side:** one capability contract / runtime side.
> **Authority:** what World accepts, validates, stores and records for `get_entity_at_current_place` — one exact-local Entity with its current-state page.
> **Excludes:** how an Agent words this to a player — published as [its tool description](../../mcp/agent/tool/get_entity_at_current_place.md); player-facing conduct — defined in [Agent guidance and player-facing communication](../agent.md#agent-guidance-and-player-facing-communication); error codes and their transport mapping — defined in [canonical errors](../protocol.md#canonical-errors).

## Purpose

Fetch one exact-local Entity selected from compact orientation with one paginated combined current Property/Trait association page and no role/control provenance.

## Input

World call `get_entity_at_current_place(context.user_id, input)`; HTTP `GET /api/place/current/entity/{entity_id}?cursor&limit`; MCP `get_entity_at_current_place`. Requires exactly one `entity_id`, with optional `cursor` and `limit`.

## Validation

Eligible subjects are the actor, current Place, co-present Characters and explicitly placed ordinary Entities. Missing, remote, departed and otherwise ineligible selection uses neutral `entity_at_current_place_unavailable`.

## Result

Returns one safe Entity with its complete current Position, safe current Place,
matching `place_revision` and one tagged current Property/Trait page. Orientation and
Activity references remain compact and never inline this page.

## Activity footprint

None. Reads are not Activity.

## Annotations and retry class

Read-only and idempotent. A continuation copies `next` unchanged and repeats the same
Entity, Position and Place revisions. Changed Place state rejects with
`place_revision_conflict`; a moved or no-longer-local Entity rejects neutrally —
constrained by [shared capability inputs](../protocol.md#shared-capability-inputs);
this capability adds no local rule.

## Evidence obligations

World, HTTP and MCP must expose this same semantic contract, strict schema, result and canonical errors.
