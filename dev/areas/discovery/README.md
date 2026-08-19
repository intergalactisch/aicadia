# Discovery

> **Role / side:** current Discovery development synthesis / development side.
> **Authority:** owns the current meaning, boundary, decisions, unresolved landscape, components and directional technical model for Discovery.
> **Excludes:** selected work, exact game behavior, sourced findings, experiment verdicts and delivery claims; those remain in `dev/backlog/` and plans, `game/docs/`, `dev/docs/research/`, `dev/lab/` and `dev/docs/evidence/`.

## Meaning

Discovery is the investigation path through which naturally occurring things that
nobody made or placed can become established shared World state. World resolves an
authoritative opportunity first; after a positive result, the Agent authors one
grounded candidate and the User confirms the complete preview before World commits it.

## Boundary

### This is

- A Character-grounded investigation followed by a World-owned chance result.
- A bounded opportunity for an Agent to author plausible new shared state.
- Explicit User confirmation of the full candidate before mutation.
- A future way to establish natural objects and, when separately designed, new Places.

### This is not

- The way a Character makes, brings or places an object; ordinary Action owns that.
- A User selecting the mechanical focus, odds, seed or result.
- World inferring plausible content, running an LLM or invoking an Agent automatically.
- Guaranteed novelty, success, movement or a new Place on every investigation.

## Decisions

### Chosen

- World resolves the investigation chance before an Agent may author candidate content.
- The Agent selects an investigation from current Character context but cannot control the roll.
- A positive opportunity is retry-stable and remains bound to its authoritative Character context.
- The Agent re-grounds after a positive result and previews the complete candidate for User confirmation.
- Naturally occurring finds use Discovery; made, brought or placed things use ordinary Action.
- The first slice can commit exactly one found Entity and no new Place or movement.
- For the selected future new-Place scene, independently confirmed discoveries stay
  distinct unless exact existing identity is structurally established; World does
  not infer sameness from names or descriptions.
- The selected new-Place discovery establishes one destination and its explicit
  connectedness while the Character remains at the origin; entering it belongs to
  a later deliberate Movement action.
- That established connection explicitly names its allowed direction or directions;
  Discovery does not imply a return path.
- Future spatial Discovery must be able to occur at an unnamed persistent Position
  between Places without first turning it into a new Place.

### Rejected

- Letting the User or Agent provide odds, seed, roll result or number of opportunities.
- Asking the Agent to invent content before World establishes a positive opportunity.
- Using Agent, connection or conversation restarts to reset discovery context.
- Adding a generic Discovery domain object when attempt provenance is sufficient.
- Treating every investigation as successful or every found thing as geographically new.

### Not yet chosen

- Whether later Discovery can establish several connected Entities or Places at once.
- How repeated investigation balances scarcity, novelty and hoarding without scores.
- Which future Character or Place facts may shape bounded chance tables.
- How discoveries expose information differently to present, distant or returning Characters.
- How later evidence relates two independently discovered subjects that may be the same.
- How an investigation is grounded at a non-Place Position and which broader
  Place or terrain context, if any, also applies.

## Research needed

- Exercise Agent comprehension of the natural-find versus made-or-placed boundary.
- Test repeated investigation behavior, retry stability and abuse pressure at multiplayer scale.
- Design new-Place discovery only together with the Place and Movement Areas.
- Study bounded novelty and opportunity rules without server semantic inference or score mechanics.

## Components

| Component | Current meaning |
| --- | --- |
| Investigation | The Agent-selected action grounded in current Character context. |
| Attempt | World-owned technical provenance and retry-stable chance resolution. |
| Opportunity | A positive authoritative result that permits one bounded candidate. |
| Re-grounding | A fresh authoritative read before candidate authoring and confirmation. |
| Candidate | Agent-authored concrete state constrained by the opportunity. |
| Confirmation | User approval of the complete preview before World mutation. |
| Find | The accepted Entity and Activity committed to the shared World. |

## Technical model

### Delivered

The current capability exposes an investigation attempt, retry-stable World result,
one positive-opportunity token, complete Entity preview and confirmed commit with
Activity. It can establish exactly one Entity in the existing Place. Exact inputs,
states and errors remain in [`game/docs/`](../../../game/docs/README.md).

### Directional

Future discovery keeps the same responsibility split: World owns bounded mechanical
opportunity and structural validation; an explicitly invoked Agent authors meaning;
the User confirms; one transaction commits exact state and history. Place or
multi-Entity expansion requires separate accepted contracts.

### Absent

New-Place discovery, several results per attempt, server-authored content, Agent
background invocation, caller-controlled odds, a generic Discovery table and
universal novelty mechanics are absent.

## Sources

- Retained rationale and corrections — [Discovery and investigation](../../docs/concept/discovery.md).
- Exact behavior — [current game contract](../../../game/docs/README.md).
- Related synthesis — [Place](../place/README.md), [Agent Play](../agent-play/README.md) and [World Change](../world-change/README.md).
- Experiments and delivery — [`dev/lab/`](../../lab/README.md) and [`dev/docs/evidence/`](../../docs/evidence/README.md).
