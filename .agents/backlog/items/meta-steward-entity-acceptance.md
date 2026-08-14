# Meta-steward Entity acceptance

> **Role / side:** forward-planning item / development side.
> **Authority:** records this outcome's backlog state, dependencies and completion pointers.
> **Excludes:** current product contracts, decision rationale and detailed delivery evidence; see `docs/game/`, `docs/concept/log/` and `docs/evidence/`.

Status: Dropped

## Outcome

An administrator consciously invokes a meta Agent, explicitly confirms one complete
Entity package and asks World to accept it through a private operator flow. Only World
creates the durable Entity, optional Place relation, Activity and private meta-admin
provenance. Unconfirmed, unchosen and rejected candidates leave no state.

The dropped draft plan is
`.agents/plans/20260811-083145-meta-steward-entity-acceptance/plan.md`.

This outcome was rejected because it incorrectly inserted a human administrator,
private confirmation and a privileged meta-steward path. The corrected direction is
one explicit Agent request followed by deterministic World acceptance or rejection.

## Accepted facts

- raw `create_entity` will no longer be a generic player HTTP/MCP capability;
- Agent proposes, administrator confirms and World alone accepts durable state;
- invocation is conscious; the server never calls an Agent or spends tokens;
- meta candidates are not Entities before acceptance;
- accepted Entity placement may target any existing Place and is independent of a
  Character's current Place;
- the following Exact-Place edge depends on legitimate established state from this
  writer flow.

## Open choices

The active grill must settle proposal count/choice, administrator authorization and
provenance, private provenance storage, delivery idempotency and the exact private
skill/CLI seam. The draft recommends three directions plus do nothing, a separately
confirmed final package, local process/database authorization with existing User
provenance, a narrow private meta marker, confirmation-key idempotency and no admin
HTTP/MCP surface.

## Completion evidence

World and operator evidence must prove exactly one confirmed accepted Entity and
history footprint; zero state for do-nothing, unconfirmed and rejected paths; private
meta provenance; and complete removal of raw create from player HTTP/MCP/OpenAPI.
