# `enter_world`

> **Role / side:** one capability contract / runtime side.
> **Authority:** what World accepts, validates, stores and records for `enter_world` — placing the unplaced Character at the entry Place.
> **Excludes:** how an Agent words this to a player — published as [its tool description](../../mcp/agent/tool/enter_world.md); the World-entry sequence — defined in [Required Character workshop and World-entry flow](../agent.md#required-character-workshop-and-world-entry-flow); error codes and their transport mapping — defined in [canonical errors](../protocol.md#canonical-errors).

## Purpose

Place the current unplaced Character at the server-derived entry Place; retry returns the same placement.

## Input

World call `enter_world(context.user_id)`; HTTP `POST /api/world/entry` with no request body; MCP `enter_world` with an empty object. It accepts no Character or Place id.

## Validation

World derives the Character and entry Place. The Character must be unplaced; genesis must already have established the entry Place. The Character may remain unplaced indefinitely.

## Result

Atomically sets `character.current_place_entity_id` only when absent. Retrying or racing a successful entry returns the same Character. The destination cannot be selected and this is not movement.

## Activity footprint

First acceptance appends one `enter_world` Activity with entering Character as actor, entry Place as context and `destination`. A successful retry appends none.

## Annotations and retry class

Modifying and idempotent: a retry after a successful entry returns the same placement without new Activity.

## Evidence obligations

World, HTTP and MCP must expose this same semantic contract, strict schema, result and canonical errors.
