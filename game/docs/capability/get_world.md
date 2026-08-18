# `get_world`

> **Role / side:** one capability contract / runtime side.
> **Authority:** what World accepts, validates, stores and records for `get_world` — the identity of the one shared World.
> **Excludes:** how an Agent words this to a player — published as [its tool description](../../mcp/agent/tool/get_world.md); player-facing conduct — defined in [Agent guidance and player-facing communication](../agent.md#agent-guidance-and-player-facing-communication); error codes and their transport mapping — defined in [canonical errors](../protocol.md#canonical-errors).

## Purpose

Get the identity of the one persistent shared World.

## Input

World call `get_world()`; HTTP `GET /api/world`; MCP `get_world`. User context is absent. HTTP has no request body and MCP uses an empty object.

## Validation

All input selectors and unknown fields are rejected.

## Result

`World { name }`.

## Activity footprint

None. Reads are not Activity.

## Annotations and retry class

Read-only and idempotent; a caller may repeat it.

## Evidence obligations

World, HTTP and MCP must expose this same semantic contract, strict schema, result and canonical errors.
