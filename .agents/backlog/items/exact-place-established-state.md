# Exact-Place established-state read

> **Role / side:** forward-planning item / development side.
> **Authority:** records this outcome's backlog state, dependencies and completion pointers.
> **Excludes:** current product contracts, decision rationale and detailed delivery evidence; see `docs/game/`, `docs/concept/log/` and `docs/evidence/`.

## Outcome

A placed Character can inspect established state at its server-derived exact current
Place, and another Character there reads the same state without supplying User,
Character or Place ids. This becomes one context source from which an Agent proposes
the next player action.

The exact-Place read is now supporting scope inside the active Agent-mediated World
action instead of a separate backlog outcome. Its dropped combined read/write plan is
`.agents/plans/20260810-191036-exact-place-established-state/plan.md`.

## Accepted facts

- exact stored Place equality is the complete inclusion rule for this first read;
- current location is explicit state, never inferred from Activity;
- User-without-Character is transient onboarding state, never playable; only one of
  three Agent-proposed candidates becomes a durable Character;
- future onboarding composes Character creation, genesis when needed and World entry
  as separate accepted actions in one guided flow;
- the Exact-Place read returns `character_not_found` without a Character and
  `character_not_entered` for an unplaced Character, never a misleading empty page;
- `list_entity_at_current_place` exposes the exact derived Place and Entity summaries
  through GET `/api/place/current/entity`;
- World, HTTP and MCP ship one semantic contract;
- investigation, rolls, claims, movement, containment and visibility remain absent.

## Superseded boundary

Entity and Activity/prose inspection remain separate composable typed reads in the
active action plan. A monolithic action-context response is rejected. Bounded
containment and adjacency belong to a later Place-neighborhood edge, not this exact
entry-Place evidence slice.

## Completion evidence

World and adapter tests must prove exact same-Place sharing, boundary exclusion,
pagination, errors, complete catalog parity and usefulness as Agent action context.
