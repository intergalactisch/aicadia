# Local Entity Property state

> **Role / side:** forward-planning item / development side.
> **Authority:** records this outcome's backlog state, dependencies and completion pointers.
> **Excludes:** current product contracts, decision rationale and detailed delivery evidence; see `docs/game/`, `docs/concept/log/` and `docs/evidence/`.

Status: Done

## Outcome

Every existing Entity-creation route can atomically establish 0–100 initial text or
integer Properties. Confirmed World Actions can atomically change 1–100 Properties
across 1–100 exact-current-Place Entities, and Interactions can optionally change
0–100 Properties of their actor/explicit targets. Current local and authorized
Activity reads expose exact typed state/history without role, control or global
knowledge leakage.

The accepted implementation plan is
`../../plans/20260813-171201-character-property-state/plan.md`; the design record is
[`docs/concept/entity-state.md`](../../../docs/concept/entity-state.md).
The executable contract is published in [`docs/game/`](../../../docs/game/README.md).

## Accepted contract

- `create_entity`, `create_character`, `create_entry_place` and
  `submit_action.introduce_entity` uniformly accept 0–100 initial Properties.
- `submit_action.change_entity_state` accepts Property-only or combined state
  packages, including 1–100 unique Entity/key writes over actor, current Place,
  co-present Characters and ordinary Entities without role or control branching.
- `submit_interaction` retains pure outward behavior with no changes and may
  atomically change only actor/explicit-target Properties.
- Agent-created canonical keys contain immutable English key and value type only;
  same key/different type conflicts, with no alias or inference.
- Outward/local current Properties and exact Activity changes are typed read facts;
structured current state wins over conflicting introductory prose.
- Control-like Property keys and values remain user-authored in-World content only;
  they never establish or reveal actual User, Character, NPC, ownership or control
  provenance and are not server-denylisted.
- Immutable history stores each value once; an Entity/key current row stores only
  its Activity pointer. Writes and reads are set-based and bounded at 100.

## Accepted authority boundary

Uniform local World Action eligibility includes other played Characters and the
current Place exactly like ordinary local Entities. This prevents control-provenance
probing and supports bounded causal events such as one explosion changing actor,
ordinary Entity and other Character together. A player never directly edits any
Property: User steering/confirmation, Agent proposal and World validation/write stay
separate. Later deterministic external-factor mechanics may reuse the private writer
only after acceptance; autonomous/background Agents, timers, `world_event` and
ungrounded simulation remain absent.

## Non-goals

Unset/deletion, aliases, possession/relations, volition/response/consent,
placement/movement, remote/cross-Place subjects, dynamic/prose selectors and
global/reverse search.

## Completion evidence

Delivery history and current status: see
[Property evidence](../../../docs/evidence/property.md) and
[uniform Entity-state evidence](../../../docs/evidence/entity-state.md).
