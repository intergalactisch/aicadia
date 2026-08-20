# `create_entry_place`

> **Role / side:** one capability contract / runtime side.
> **Authority:** what World accepts, validates, stores and records for `create_entry_place` — World genesis of the one shared entry Place.
> **Excludes:** how an Agent words this to a player — published as [its tool description](../../mcp/agent/tool/create_entry_place.md); the genesis branch of the Character workshop — defined in [Required Character workshop and World-entry flow](../agent.md#required-character-workshop-and-world-entry-flow); error codes and their transport mapping — defined in [canonical errors](../protocol.md#canonical-errors).

## Purpose

Create the one shared entry Place at World origin with independent 0–100 optional
initial Properties and Traits from the current unplaced Character.

## Input

World call `create_entry_place(context.user_id, input)`; HTTP `POST /api/place/entry`; MCP `create_entry_place`. Input is `{ name, description, property, trait }`; both state lists default to `[]`.

`create_character`, `create_entry_place` and `create_entity` accept exactly:

```json
{
  "name": "North Gate",
  "description": "The one established entry Place.",
  "property": [
    {"key": "surface", "value": {"type": "text", "text": "weathered stone"}},
    {"key": "arch_count", "value": {"type": "integer", "integer": 3}}
  ],
  "trait": [
    {"statement": "Carries every returning footstep as a low echo."}
  ]
}
```

## Validation

The derived Character must exist with no Position and no current Place. Zero entry
Places is valid before genesis; exactly one concurrent request may establish the sole
entry Place. Name, description, Property and Trait items — constrained by
[shared value validation](../domain.md#shared-value-validation),
[Property](../model/property/README.md) and [Trait](../model/trait/README.md); this
capability adds only the unentered-Character and single-genesis rules.

## Result

Atomically creates Entity, direct Position `(0, 0, 0)`, Place, Activity and initial
Property/Trait state. Position description is absent. This is World genesis, not
discovery; a second entry Place is rejected with
`entry_place_already_exists` without orphan state.

## Activity footprint

One `create_entry_place` Activity: proposing unplaced Character as actor, no context
Place, new Place Entity as `subject` and new Place Position as `result`.

## Annotations and retry class

Additive and non-idempotent: a repeated call after a successful genesis is rejected as a second entry Place.

## Evidence obligations

World, HTTP and MCP must expose this same semantic contract, strict schema, result and canonical errors.
