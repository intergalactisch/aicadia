# World Change

> **Role / side:** current World Change development synthesis / development side.
> **Authority:** owns the current meaning, boundary, decisions, unresolved landscape, components and directional technical model for changing shared World state.
> **Excludes:** selected work, exact game behavior, sourced findings, retained operation rationale and delivery claims; those remain in `dev/backlog/` and plans, `game/docs/`, `dev/docs/research/`, `dev/docs/concept/` and `dev/docs/evidence/`.

## Meaning

World Change concerns how an explicitly invoked Agent proposes a bounded change to
shared state and how the dumb, strict World validates, settles and records it. The
Agent supplies semantic intelligence; World owns identity, authority, freshness,
structural dependencies, bounds, atomic current state and attributable history.

## Boundary

### This is

- Bounded typed proposals that name exact actors, subjects, intended state and causal dependencies.
- Deterministic admission, authority, freshness and structural validation by World.
- Atomic current-state change and durable attributable Activity.
- One conceptual lens across Entity introduction, Action, Interaction, Discovery and future consequences.

### This is not

- World inferring physics, causality, scope or preferred outcomes from prose.
- A generic patch endpoint, arbitrary script or semantic merge engine.
- Event sourcing, a universal event payload, a rule engine or background simulation.
- Permission to introduce a general change kernel before a concrete behavior requires it.

## Decisions

### Chosen

- One `World` interface owns game behavior; HTTP and MCP are thin adapters over the same semantics.
- Agents author meaning while World validates only deterministic structural truth it owns.
- Every accepted mutation commits current state and attributable Activity in the same transaction.
- Each proposal uses exact stable subject identities, explicit roles and bounded intended state.
- User confirmation covers the complete package before submission.
- Entity state uses uniform Property and Trait concepts; operations keep their concrete game meaning.
- In the selected spatial scene, establishing a discovered Place and later entering
  it are separate confirmed changes rather than one combined mutation.
- Future Entity state must support an unnamed persistent Position between Places
  without manufacturing a Place identity for that point.
- Agent-authored relationship meaning may be open and precise; World must not require
  every possible predicate to be an enum or treat its wording as executable mechanics.
- A proposed precise arrangement explicitly distinguishes current independent state
  from state that must remain relative when another Entity changes. World validates
  that chosen structure instead of deriving it from the Agent's description.
- Any Agent acting through its Character may in principle propose a new current state
  for ordinary World content, including content first authored by another Agent.
  Surprising causal meaning remains Agent-authored, while the proposal explicitly
  names every Entity and exact change World is asked to commit.
- Direct change proposals may rely only on current subjects and Relations the acting
  Character is structurally eligible to know. World rejects guessed hidden subjects
  or bases without revealing whether they exist.
- Remembering that a Relation existed earlier may justify a new investigation but
  cannot serve as the fresh structural basis for directly changing its hidden endpoint.
- A change composes only the Relation, Position, movement, visibility and action
  facts its concrete scene needs; no generic Containment consequence is inferred.

### Rejected

- A server ontology, heuristic or LLM that infers semantic effects from names, prose or Property values.
- A generic JSON patch or arbitrary code surface for World mutation.
- Event sourcing, `world_event`, generic `rule` or universal consequence machinery in the current MVP.
- Global World revisions, counters or locks used to settle unrelated changes.
- Treating subscriptions, transport traffic or rejected attempts as World history.
- Treating a closed Relation enum as the complete vocabulary of Agent-authored World meaning.
- Permanent author- or controller-exclusive mutation rights over ordinary World content.
- Treating unguessable identifiers or Agent instructions as protection for hidden World state.

### Not yet chosen

- Whether durable structural relationships share a Relation base identity and type
  discriminator, open authored statement or only a bounded read over separate exact
  facts, and how those layers avoid duplicating one truth.
- The first general multi-subject proposal that current concrete operations cannot express cleanly.
- How explicit remote causes and multi-Place consequences are authorized and bounded.
- Which physical context or explicit remote basis a Character must have before its
  Agent may change another Entity, without turning World into a physics interpreter.
- The exact causal-dependency and freshness contract when concurrent changes cross.
- How a bounded chain reaction terminates, detects loops and records its involved subjects.
- Which collective ratification mechanism may authorize one shared outcome.
- How a non-Place Position grounds an Entity and its Activity when no
  Place is the direct spatial address.

## Research needed

- Test a unified change shape against the multiplayer scenarios without erasing concrete operation meaning.
- Compare exact dependency and conflict models under one deliberately hot subject.
- Design remote consequence and cycle bounds that World can validate without semantic inference.
- Prove atomic history and bounded readback for multi-subject changes in PostgreSQL.

## Components

| Component | Current meaning |
| --- | --- |
| Actor | The authenticated User and controlled Character responsible for the attempt. |
| Subject | Each stable Character, Place or Entity identity whose state participates. |
| Proposal | Agent-authored typed intended state and explicit causal basis. |
| Authority | Control, placement and capability facts World can validate. |
| Freshness | Exact subject state on which the proposal depends. |
| Settlement | One deterministic admission or rejection at the smallest transaction scope. |
| History | Durable Activity naming who acted, what changed, when, where and which subjects were involved. |
| Scenarios | The spatial state and remote-consequence cases in the [spatial scenario catalogue](../place/scenarios.md). |

## Technical model

### Delivered

Concrete create, Action, Interaction and Discovery packages validate through one
`World` seam, mutate PostgreSQL current state and write Activity atomically. Entity
state can carry bounded Properties and Traits. Exact operations, inputs and errors
remain in [`game/docs/`](../../../game/docs/README.md).

### Directional

When a concrete scenario earns it, one bounded typed proposal may compose several
exact subjects, intended states and causal dependencies. World locks only the
smallest structural basis, validates freshness and authority, and commits the result
and history atomically without understanding the prose meaning.

### Absent

A generic World-change kernel, universal event payload, event sourcing, server
physics, automatic chain reactions, remote-cause capability, rule engine, background
simulation and collective ratification are absent.

## Sources

- Sourced findings — [unified World change system](../../docs/research/unified-world-change-system.md).
- Retained rationale — [Entity state](../../docs/concept/entity-state.md), [Interaction](../../docs/concept/interaction.md) and [mass concurrency and living World direction](../../docs/concept/concurrency-and-world-dynamics.md).
- Prepared pressure — [Multiplayer scenario catalogue](../multiplayer/scenarios.md).
- Prepared spatial pressure — [Spatial scenario catalogue](../place/scenarios.md).
- Related synthesis — [Multiplayer](../multiplayer/README.md), [Movement](../movement/README.md) and [Agent Play](../agent-play/README.md).
- Exact behavior and delivery — [`game/docs/`](../../../game/docs/README.md) and [`dev/docs/evidence/`](../../docs/evidence/README.md).
