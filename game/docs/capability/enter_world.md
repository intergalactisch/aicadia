# `enter_world`

> **Role / side:** one capability contract / runtime side.
> **Authority:** what World accepts, validates, stores and records for `enter_world` — placing the unplaced Character at the entry Place.
> **Excludes:** how an Agent words this to a player — published as [its tool description](../../mcp/agent/tool/enter_world.md); the World-entry sequence — defined in [Required Character workshop and World-entry flow](../agent.md#required-character-workshop-and-world-entry-flow); error codes and their transport mapping — defined in [canonical errors](../protocol.md#canonical-errors).

## Purpose

Place the current unentered Character at the server-derived entry Place and its exact
Position; retry returns the same placement.

## Input

World call `enter_world(context.user_id)`; HTTP `POST /api/world/entry` with no request body; MCP `enter_world` with an empty object. It accepts no Character or Place id.

## Validation

World derives the Character and entry Place. The Character must have no Position and
no current Place; genesis must already have established the entry Place. The
Character may remain unentered indefinitely.

## Result

Atomically creates the Character's root Position equal to the entry Place Position
and sets `character.current_place_entity_id`. Retrying or racing a successful entry
returns the same Character. The destination cannot be selected and this is not
Movement.

## Activity footprint

First acceptance appends one `enter_world` Activity with entering Character as
actor, entry Place as context and `destination`, and new Character Position as
`result`. A successful retry appends none.

## Annotations and retry class

Modifying and idempotent: a retry after a successful entry returns the same placement without new Activity.

## Evidence obligations

World, HTTP and MCP must expose this same semantic contract, strict schema, result and canonical errors.
