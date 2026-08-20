# `list_activity_at_current_place`

> **Role / side:** one capability contract / runtime side.
> **Authority:** what World accepts, validates, stores and records for `list_activity_at_current_place` — the canonical Activity page of the exact current Place.
> **Excludes:** how an Agent words this to a player — published as [its tool description](../../mcp/agent/tool/list_activity_at_current_place.md); player-facing conduct — defined in [Agent guidance and player-facing communication](../agent.md#agent-guidance-and-player-facing-communication); error codes and their transport mapping — defined in [canonical errors](../protocol.md#canonical-errors).

## Purpose

List canonical non-Interaction Place history plus only Interactions in which the derived Character is actor or target.

## Input

World call `list_activity_at_current_place(context.user_id, input)`; HTTP `GET /api/place/current/activity`; MCP `list_activity_at_current_place`. Optional operation-bound `cursor` and `limit`.

## Validation

World derives the Character and exact current Place; an unentered Character or an
entered Character currently between Places is rejected. The Place must be stored
context or linked in any involved-Entity role.

## Result

Returns safe current Place, its freshness revision, one canonical Activity page and `next`, ordered by `(occurred_at, id)` descending. Non-Interaction Activity is retained; Interaction is included only for actor or explicit target, never merely for a co-present bystander.

## Activity footprint

None. Reads are not Activity.

## Annotations and retry class

Read-only and idempotent. A continuation copies `next` unchanged; the cursor is opaque and tied to this operation — constrained by [shared capability inputs](../protocol.md#shared-capability-inputs); this capability adds no local rule.

## Evidence obligations

World, HTTP and MCP must expose this same semantic contract, strict schema, result and canonical errors.
