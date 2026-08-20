# `create_character`

> **Role / side:** one capability contract / runtime side.
> **Authority:** what World accepts, validates, stores and records for `create_character` — creation of the User's one unplaced Character with initial state.
> **Excludes:** how an Agent words this to a player — published as [its tool description](../../mcp/agent/tool/create_character.md); the Character workshop and its confirmation — defined in [Required Character workshop and World-entry flow](../agent.md#required-character-workshop-and-world-entry-flow); error codes and their transport mapping — defined in [canonical errors](../protocol.md#canonical-errors).

## Purpose

Create the current User's one unplaced Character Entity role with independent 0–100 optional initial Properties and Traits; accepts no ids.

## Input

`{ name, description, property, trait }`; both state lists default to `[]`. World call `create_character(context.user_id, input)`; HTTP `POST /api/character`; MCP `create_character`.

## Validation

The User may own at most one Character. Initial Trait items contain only one `statement` and establish new World-assigned lineages. Name and description — constrained by [shared value validation](../domain.md#shared-value-validation); Property and Trait items — constrained by [Property](../model/property/README.md) and [Trait](../model/trait/README.md); this capability adds only the local rules stated above.

## Result

Atomically creates Entity, Character, Activity and initial Property/Trait state.
Concurrent creates for one User yield exactly one Character without orphan state.
The result has no Position or current Place: the Character exists but has not yet
entered the World. This is distinct from an entered Character between Places, which
has Position but no current Place.

## Activity footprint

One `create_character` Activity with the new Character Entity as `subject`.

## Annotations and retry class

Additive and non-idempotent: a repeated call is a second create, which the one-Character rule rejects.

## Evidence obligations

World, HTTP and MCP must expose this same semantic contract, strict schema, result and canonical errors.
