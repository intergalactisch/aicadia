# `get_character`

> **Role / side:** one capability contract / runtime side.
> **Authority:** what World accepts, validates, stores and records for `get_character` — the current User's Character with its Position, Place and one current-state page.
> **Excludes:** how an Agent words this to a player — published as [its tool description](../../mcp/agent/tool/get_character.md); the first-use and World-entry sequence — defined in [Required Character workshop and World-entry flow](../agent.md#required-character-workshop-and-world-entry-flow); error codes and their transport mapping — defined in [canonical errors](../protocol.md#canonical-errors).

## Purpose

Get the current User's Character, nullable complete Position and Place, nullable
Place revision and one paginated combined current Property/Trait association page;
accepts no ids.

## Input

World call `get_character(context.user_id, input)`; HTTP `GET /api/character?cursor&limit`; MCP `get_character`. It accepts only optional current-state `cursor` and `limit`.

## Validation

The derived Character must exist. Pagination and freshness of the current-state page
— constrained by [wire shapes](../wire.md#wire-shapes) and
[shared capability inputs](../protocol.md#shared-capability-inputs); this capability
adds no local rule.

## Result

It returns the Character, complete current Position when present, complete current
Place when present, nullable `place_revision` and one bounded combined page of that
Character Entity's current Property/Trait associations. A positioned Character with
no current Place is between Places, not unentered.

## Activity footprint

None. Reads are not Activity.

## Annotations and retry class

Read-only and idempotent. A continuation copies `next` unchanged and repeats the same
Entity, Position revision and nullable Place revision. Changed Position rejects with
`position_revision_conflict`; changed Place state rejects with
`place_revision_conflict` — constrained by
[shared capability inputs](../protocol.md#shared-capability-inputs); this capability
adds no local rule.

## Evidence obligations

World, HTTP and MCP must expose this same semantic contract, strict schema, result and canonical errors.
