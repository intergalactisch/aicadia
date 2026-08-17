---
status: done
horizon: done
updated: 2026-08-09
---

# World entry with activity history

> **Role / side:** forward-planning item / development side.
> **Authority:** records this outcome's backlog state, dependencies and completion pointers.
> **Excludes:** current product contracts, decision rationale and detailed delivery evidence; see `docs/game/`, `docs/concept/log/` and `docs/evidence/`.

## Outcome

A User may introduce a Character without placing it. At a later Agent-chosen moment,
that Character can enter the one shared World at its server-derived entry Place. The
World retains immutable, queryable evidence of accepted game actions so it can later
answer where a Character was, what it did, and which Characters, Places and other
Entities were involved.

This vertical slice is complete. Its executable authority is recorded in
`docs/game/domain.md`, `docs/game/model/` and `docs/game/agent.md`.

## Why now

Character identity and ownership exist, but Character currently has no spatial game
behavior. Adding only a nullable Place foreign key would be unused plumbing. Adding
World entry without history would make the first meaningful Character transition
historically invisible and force a retrofit immediately afterward. This slice proves
both together with one concrete action.

## Player flow

1. `create_character` continues to create identity and ownership only. The returned
   Character has no current Place.
2. `get_character` exposes that absence explicitly; it never treats it as a missing
   lookup or unknown coordinate.
3. If the World has no entry Place, an Agent representing an existing unplaced
   Character may propose its name and description once. `create_entry_place` creates
   the Entity and Place role atomically. The User never supplies a Place id.
4. `enter_world` accepts no Place selector. World derives the Character from User
   context and assigns the established entry Place in one transaction.
5. A Character may remain unplaced indefinitely. Repeating or racing entry requests
   cannot create two placements or two entry Places.

Creating the first shared entry Place is World genesis, not discovery. It uses no
chance roll. Later Places enter through the discovery and expansion behavior, not
through repeated calls to `create_entry_place`.

## Current-state model

- `Place` is an Entity role and uses `place.entity_id` as its primary and foreign
  key. It has no second identity.
- `character.current_place_entity_id` is nullable and references `place.entity_id`.
- One explicitly marked entry Place may exist. Zero is valid before World genesis;
  exactly one is valid afterward.
- Character creation always leaves `current_place_entity_id` null.
- World entry copies the entry Place id server-side. Agent input can never select or
  override it.

Coordinates, geometry, boundaries, containment, routes, distance and movement are
absent from this slice.

## Activity-history spine

Current state remains in ordinary domain tables for fast reads. Each accepted
state-changing game operation appends one immutable `activity` record in the same
PostgreSQL transaction; current state is not rebuilt by replaying activities.

The minimum normalized shape is:

```text
activity
  id
  operation                         server-owned operation name
  requested_by_user_id             accountable request context
  actor_character_entity_id        nullable in-World actor
  context_place_entity_id          nullable Place at acceptance time
  occurred_at

activity_entity
  activity_id
  entity_id
  role                              server-owned role in this action
```

This supports the required history without a universal JSON payload:

- `actor_character_entity_id` answers which Character acted.
- `context_place_entity_id` preserves where the action happened even after movement.
- linked Entity ids with the current explicit roles `subject` and `destination`
  identify what was involved. Because Character and Place are Entity roles, they
  use the same stable reference system.
- `requested_by_user_id` preserves request provenance but need not be exposed as
  shared Agent-visible history.

Conversation text, rejected requests, reads, rate-limit decisions and private Agent
reasoning are not World activity. Concrete domain data remains in its concrete table;
`activity` records the accepted action and cross-cutting context, not a duplicate
snapshot or event-sourcing payload.

## First history coverage

The implementation records the existing and new accepted mutations:

- `create_character`: the new Character is linked as the action subject; there was
  no pre-existing Character actor or Place.
- `create_entity`: the Entity is linked as subject; World records the current
  Character and Place when they exist, otherwise both remain null.
- `create_entry_place`: the Place Entity is linked as subject; its proposing
  Character exists but is still unplaced.
- `enter_world`: the Character is actor, the entry Place is the accepted context and
  destination.

Operational `create_user` provisioning is not game activity. The migration backfills
pre-history Character Entities as `create_character` and all other pre-role Entities
as `create_entity`, retaining their derivable User, subject and timestamp. Actor and
Place context remain absent because the old schema did not retain them.

## First read boundary

One bounded, cursor-paginated personal history read derives the current Character
from User context and returns its activities in `(occurred_at, id)` descending order,
including typed Place and involved-Entity references. It accepts no Character id.
Shared Entity-, Place- and other-Character history queries remain later backlog work.

The read ships through `World`, HTTP and MCP with one semantic contract and catalog
entry. History that is written but cannot be retrieved through its promised scope is
not considered delivered.

## Required evidence

The slice is done only when:

1. Character creation still returns an unplaced Character.
2. Exactly one entry Place can win under concurrent genesis requests.
3. `enter_world` derives both Character and entry Place and is retry-safe.
4. Every accepted covered mutation and its activity either commit together or both
   roll back.
5. Past activity retains its Place and involved Entity ids after current state moves
   on.
6. Personal history authorization, ordering and pagination are proven through World,
   HTTP and MCP parity tests.
7. Current contract, Agent descriptions, canonical vocabulary, concept log and this
   backlog agree.

## Completion evidence

Delivery history and current status: see [World-entry evidence](../../../docs/evidence/world-entry.md).

## Accepted delivery choices

- The first connected Agent representing an existing unplaced Character may author
  the one entry Place's semantic name and description.
- Public operation names are `create_entry_place`, `enter_world` and
  `list_activity`.
- Existing rows are backfilled only with facts exactly derivable from stored state;
  unknown actor and Place context remain absent.
- `activity` is the singular current history noun.

## Explicitly deferred

- investigation rolls, roll tokens and discovery candidates;
- movement and additional Places;
- other-Character or global history browsing;
- as-of reconstruction and replay;
- narrative transcripts and private Agent memory;
- generic claims, event sourcing, scores, clocks and background simulation.
