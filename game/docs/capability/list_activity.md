# `list_activity`

> **Role / side:** one capability contract / runtime side.
> **Authority:** what World accepts, validates, stores and records for `list_activity` — the Character's own Activity history.
> **Excludes:** how an Agent words this to a player — published as [its tool description](../../mcp/agent/tool/list_activity.md); player-facing conduct — defined in [Agent guidance and player-facing communication](../agent.md#agent-guidance-and-player-facing-communication); error codes and their transport mapping — defined in [canonical errors](../protocol.md#canonical-errors).

## Purpose

List Activity involving the derived current Character, newest first.

## Input

World call `list_activity(context.user_id, input)`; HTTP `GET /api/activity`; MCP `list_activity`. Optional `cursor` and `limit`; default 25, accepted 1–100.

## Validation

The Character is derived from User context; no Character id is accepted.

## Result

An Activity is returned exactly once when the Character is the stored actor or a role-linked involved Entity. Results order by `(occurred_at, id)` descending; `next` is absent when exhausted. Summaries remain typed.

## Activity footprint

None. Reads are not Activity.

## Annotations and retry class

Read-only and idempotent. A continuation copies `next` unchanged; the cursor is opaque and tied to this operation — constrained by [shared capability inputs](../protocol.md#shared-capability-inputs); this capability adds no local rule.

## Evidence obligations

World, HTTP and MCP must expose this same semantic contract, strict schema, result and canonical errors.
