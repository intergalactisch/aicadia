# `create_entity`

> **Role / side:** one capability contract / runtime side.
> **Authority:** what World accepts, validates, stores and records for `create_entity` — creation of one unplaced shared Entity with initial state.
> **Excludes:** how an Agent words this to a player — published as [its tool description](../../mcp/agent/tool/create_entity.md); the authority boundary for Entity creation — defined in [Required Character workshop and World-entry flow](../agent.md#required-character-workshop-and-world-entry-flow); error codes and their transport mapping — defined in [canonical errors](../protocol.md#canonical-errors).

## Purpose

Ask World to create one unplaced shared stable referent with independent 0–100 optional initial Properties and Traits; equal retries create another Entity.

## Input

World call `create_entity(context.user_id, input)`; HTTP `POST /api/entity`; MCP `create_entity`. Input is `{ name, description, property, trait }`; both state lists default to `[]`.

## Validation

The Property list contains 0–100 unique canonical keys. The Trait list contains 0–100 unique normalized statements and establishes new World-assigned lineages. Name, description, Property and Trait items — constrained by [shared value validation](../domain.md#shared-value-validation), [Property](../model/property/README.md) and [Trait](../model/trait/README.md); this capability adds only the local rules stated above.

## Result

World derives the introducing User, always creates a new unplaced Entity and returns the complete Entity. Equal input remains two Entities. Entity, Activity and initial Property/Trait history and current pointers commit as one bundle.

## Activity footprint

The current Character is actor when one exists and its current Place is context when present; the new Entity is `subject`.

## Annotations and retry class

Additive and non-idempotent: equal input creates another Entity, so a repeated call is a second creation.

## Evidence obligations

World, HTTP and MCP must expose this same semantic contract, strict schema, result and canonical errors.
