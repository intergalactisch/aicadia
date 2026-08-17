# Domain contract

> **Role / side:** Domain overview and cross-model rules / runtime side.
> **Authority:** the current World overview, the shared value validation and error taxonomy that no single model owns, and the domain-wide evidence obligation.
> **Excludes:** per-model contracts, delivery status, rollout narrative and evidence results; see `model/` and `docs/evidence/`.

This document is the current game authority. Aicadia has one persistent `World`,
durable `User` records, shared `Entity` records, at most one owned `Character` Entity
role per User, and zero or one shared entry `Place`. A Character may remain unplaced
or explicitly enter that Place. Every Entity may carry zero or more compact typed
Properties and stable, developing, non-executable Traits, established at creation or
changed through confirmed local Actions and actor/target Interactions. An entered
Character may submit one Action that introduces one Entity with initial Property and
Trait state, or atomically combines 0–100 exact-local Property changes with 0–100
Trait establishments/developments across exact-local Entities; a state-change Action
requires at least one change. One directed Interaction toward 1–100 existing
co-present Entities may carry optional actor/target Property changes and 0–100 mixed
Trait establishments/developments without authoring their responses. Accepted game mutations append
immutable normalized `activity` in the same PostgreSQL transaction as current state.
An entered Character may also begin one World-resolved investigation and, after a
positive result and User-confirmed Agent authorship, establish exactly one found
Entity at that Place with the same initial Property/Trait rules and attributable
Activity.

## Model contracts

Each durable subject, role, seam and state carries its own contract in `model/`:

- [World seam](model/world/README.md) — the single public game-behavior seam.
- [User](model/user/README.md) — the durable participant and request-provenance subject.
- [Entity](model/entity/README.md) — the durable World subject and what it may carry.
- [Character](model/character/README.md) — the User-owned Entity role.
- [Place](model/place/README.md) — the Place Entity role and World entry.
- [Activity](model/activity/README.md) — immutable normalized history.
- [Property](model/property/README.md) — canonical `key = value` Entity state.
- [Trait](model/trait/README.md) — stable, developing Entity statements.
- [Investigation attempt](model/investigation-attempt/README.md) — discovery provenance, chance and admission.

## Shared value validation

Entity, Character and entry Place input is trimmed, requires 1 through 120 Unicode
characters for `name` and 1 through 4,000 for `description`, and rejects U+0000.
Action and Interaction prose use the same normalization, require 1 through 4,000
Unicode characters and reject U+0000. PostgreSQL repeats the stored text invariants.

## Error taxonomy

World distinguishes malformed request or revision input; invalid Entity, Character,
Place, Action, Interaction, discovery prose, Property or Trait input; invalid Entity or Activity
limit; User, Entity,
Character or entry Place not found; unplaced Character; existing Character,
already-entered Character or existing entry Place; request-id conflict; neutral
Interaction-target, discovery-attempt, Property-Entity, scoped-Entity or Trait
unavailability; Action, Interaction or discovery request-id conflict; investigation
admission; Property-key conflict;
exact-Place revision conflict; and unavailable storage.
Adapters expose the canonical spellings and status mapping in
[Protocol contract](protocol.md#canonical-errors).

## Required evidence

The executable evidence obligations for every rule in this contract and in the
[model contracts](#model-contracts) are owned by the
[Adapter parity contract](adapter-parity.md#cross-contract-evidence-obligations).
