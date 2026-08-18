# `list_entity_at_current_place`

> **Role / side:** one capability contract / runtime side.
> **Authority:** what World accepts, validates, stores and records for `list_entity_at_current_place` — the safe list of Entities at the exact current Place.
> **Excludes:** how an Agent words this to a player — published as [its tool description](../../mcp/agent/tool/list_entity_at_current_place.md); player-facing conduct — defined in [Agent guidance and player-facing communication](../agent.md#agent-guidance-and-player-facing-communication); error codes and their transport mapping — defined in [canonical errors](../protocol.md#canonical-errors).

## Purpose

List safe descriptions of other Characters and ordinary Entities eligible at the derived exact current Place, exposing no role or control provenance.

## Input

World call `list_entity_at_current_place(context.user_id, input)`; HTTP `GET /api/place/current/entity`; MCP `list_entity_at_current_place`. Optional operation-bound `cursor` and `limit`.

## Validation

World derives the Character and exact current Place; an unplaced Character is rejected rather than receiving an empty page.

## Result

Returns the complete safe Place, `place_revision`, page and `next`. Excludes the requester; entries expose only stable id, name and description and order by `(introduced_at, id)` descending. The Place Entity remains an eligible Interaction target.

## Activity footprint

None. Reads are not Activity.

## Annotations and retry class

Read-only and idempotent. A continuation copies `next` unchanged; the cursor is opaque and tied to this operation — constrained by [shared capability inputs](../protocol.md#shared-capability-inputs); this capability adds no local rule.

## Evidence obligations

World, HTTP and MCP must expose this same semantic contract, strict schema, result and canonical errors.
