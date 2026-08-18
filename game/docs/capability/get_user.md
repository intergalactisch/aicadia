# `get_user`

> **Role / side:** one capability contract / runtime side.
> **Authority:** what World accepts, validates, stores and records for `get_user` — the durable User derived from request context.
> **Excludes:** how an Agent words this to a player — published as [its tool description](../../mcp/agent/tool/get_user.md); player-facing conduct — defined in [Agent guidance and player-facing communication](../agent.md#agent-guidance-and-player-facing-communication); error codes and their transport mapping — defined in [canonical errors](../protocol.md#canonical-errors).

## Purpose

Get the durable User derived from request context; accepts no id.

## Input

World call `get_user(context.user_id)`; HTTP `GET /api/user`; MCP `get_user`. User context is required and MCP uses an empty object.

## Validation

Capability input never accepts a User id; the User comes from request context — constrained by [request context](../protocol.md#request-context); this capability adds nothing to it.

## Result

`User { id, created_at }`.

## Activity footprint

None. Reads are not Activity.

## Annotations and retry class

Read-only and idempotent; a caller may repeat it.

## Evidence obligations

World, HTTP and MCP must expose this same semantic contract, strict schema, result and canonical errors.
