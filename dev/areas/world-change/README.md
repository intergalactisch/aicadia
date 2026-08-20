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
- The first of those changes is itself one complete atomic package: B's Entity and
  Place role, direct Position, optional initial Properties and Traits—including
  authored landscape state—and explicit A→B Connection commit with one Activity
  while the Character remains at A. Partial Place or Connection acceptance is
  invalid; the later walking change remains a separate Activity.
- For that package, the Agent submits B's exact absolute World point as structured
  whole-centimetre `x`, `y` and `z`, with `z` as height, and names A's expected
  Position revision. World performs deterministic numeric and freshness validation
  only; it never parses spatial prose or a unit, and never chooses the point.
- The coordinate convention is fixed: greater `x` means east, greater `y` north and
  greater `z` up. World validates submitted numbers against that convention but
  never interprets compass prose to produce them.
- Absolute Position values use the permanent `(0, 0, 0)` World origin. No proposal
  names, locks or mutates that origin; it has no identity or current-state row.
- For the first slice, numeric validation is limited to the Position representation's
  technical validity and still-unchosen representable bounds. World applies no
  gameplay distance from A and does not reject a structurally valid far destination.
- Nearby established Places are bounded read context, not mutation dependencies or
  spatial locks. Their concurrent appearance does not conflict with B; retries and
  competing use of the same discovery opportunity remain exact request- and
  opportunity-scoped conflicts.
- Discovery may instead settle against a User-chosen existing Place C. That bounded
  alternative package creates only the explicit allowed A→C Connection and one
  Activity while consuming the opportunity; it never rewrites C or treats numeric
  proximity as the causal basis.
- Settlement creates one newly identified A→C alternative when that is what the User
  confirms, even if another equal-looking Connection already exists or commits
  concurrently. Explicitly choosing one returned existing Connection creates no
  mutation and preserves the opportunity. Only retrying the exact accepted request
  returns its original Connection and Activity automatically.
- Future Entity state must support an unnamed persistent Position between Places
  without manufacturing a Place identity for that point.
- Agent-authored relationship meaning may be open and precise; World must not require
  every possible predicate to be an enum or treat its wording as executable mechanics.
- Each open Relation has one stable non-Entity identity, directed Entity endpoints
  and free name and description. A later change addresses that exact Relation;
  several Relations between the same endpoints remain independent current subjects.
- Connection remains a separately owned exact Place-to-Place fact rather than a
  server-owned type inside the open Agent-authored Relation layer.
- Establishing a Connection includes one Agent-authored name and description for
  presentation. World stores but never interprets them as direction, access, method,
  geometry or another mechanical consequence.
- A Position proposal explicitly chooses either the permanent World origin or
  exactly one reference Entity and supplies all three whole-centimetre offsets. An
  Entity-relative Position mechanically follows that reference's resolved Position;
  World validates this structure instead of deriving it from Agent-authored wording.
- A successfully resolved Position read returns its calculated current World point and its
  stored basis together. The immediate reference Entity and offsets explain an
  Entity-relative Position, while one consistent revision boundary grounds a later
  proposal; World stores no duplicate resolved point.
- If bounded read work cannot reach the absolute basis, World returns only the
  eligible immediate stored basis. It neither invents nor serves a stale World point,
  and rejects an Action that structurally depends on that unresolved point.
- A Position change may never create a reference cycle. Deliberately impossible
  spatial experience remains valid through exact explicit Connection or Action
  changes and authored meaning; World validates their own structure without forcing
  Connection topology to agree with coordinates.
- Settlement accepts a relative Position mutation only after bounded validation
  reaches an absolute basis without the changed Entity and the examined revisions
  remain current at commit. A detected cycle, exhausted proof or concurrent stale
  dependency rejects the whole mutation and its Activity.
- Re-referencing contains the complete intended new reference plus `x`, `y` and `z`;
  World never calculates an implicit preserve-world or preserve-offset outcome. The
  Agent uses the cup's durable Trait and may update current Position description to
  compose and narrate those values, but World validates no semantic promise such as
  “always two centimetres.”
- An attempted action and its proposed result may differ. Agent reasoning may turn
  “put the cup on the table” into the explicit result “the cup now floats ten
  centimetres above it,” but World receives only the bounded exact Position, Trait
  and description consequences and never invents the twist itself.
- Unforeseen results are a desired future, non-default form of play. The User's
  working label `Chaos` does not introduce a World model, operation or server-side
  randomness; any later result still has to arrive as exact bounded state and Activity.
- Its wider contract is deliberately deferred to a separate grill after the spatial
  foundation; no temporary general operation is added to spatial in the meantime.
- A proposal may include optional Position `description` for Agent narration. World
  stores it only as authored text and never treats it as coordinates, following
  behavior, geometry, a Relation or authority to change another Entity.
- Every proposal that changes Position explicitly keeps, replaces or removes its
  description. World settles that outcome against the same Position revision in the
  same transaction, without a separately conflicting description write or lock.
- A Position has at most one current description, which may contain several sentences
  within ordinary text bounds. Multiple durable independent meanings stay as Traits
  of the Entity; Position does not become another Trait-owning subject.
- Position description is returned only as part of the complete Position. World
  provides no separate description audience, query, count or redaction rule.
- Entity knowledge and remembered Position do not grant current Position access.
  Only a concrete bounded read that currently returns the Entity supplies its
  Position; remembered state remains stale.
- Agent-authored Trait and description content remains semantically free within
  ordinary text bounds. An Agent may use it to withhold detail in conversation, but
  World creates no Position-specific denial or confidentiality state from it.
- The User-facing creative rule belongs to the Agent: it refuses to author
  perception, knowledge or another current experience for a different User's
  Character. World cannot interpret prose to enforce that authorship boundary.
- Any Agent acting through its Character may in principle propose a new current state
  for ordinary World content, including content first authored by another Agent.
  Surprising causal meaning remains Agent-authored, while the proposal explicitly
  names every Entity and exact change World is asked to commit.
- Direct change proposals may rely only on current subjects and Relations the acting
  Character is structurally eligible to know. World rejects guessed hidden subjects
  or bases without revealing whether they exist.
- For a remote button and bomb, an eligible open Relation may supply causal meaning
  to the Agent and be named as current context, but it never executes or authorizes
  anything. The Agent previews and submits one exact bounded Action; World validates
  Relation identity, endpoints and revision when claimed, plus ordinary authority,
  freshness, idempotency and every explicit mutation, without spatial locality or
  semantic interpretation.
- That accepted remote Action settles all exact named state and Activity atomically.
  World infers no blast radius, downstream target, automatic chain or delayed Agent
  call; any additional affected Entity must be explicit and ordinarily authorized.
- Remembering that a Relation existed earlier may justify a new investigation but
  cannot serve as the fresh structural basis for directly changing its hidden endpoint.
- A change composes only the Relation, Position, movement, visibility and action
  facts its concrete scene needs; no generic Containment consequence is inferred.

### Rejected

- A server ontology, heuristic or LLM that infers semantic effects from names,
  prose, Property values or Trait wording.
- A generic JSON patch or arbitrary code surface for World mutation.
- A `world_change` model, table or endpoint, and a standalone visibility editor
  outside a concrete Action or Introduction.
- Event sourcing, `world_event`, generic `rule` or universal consequence machinery in the current MVP.
- Global World revisions, counters or locks used to settle unrelated changes.
- Treating subscriptions, transport traffic or rejected attempts as World history.
- Treating a closed Relation enum as the complete vocabulary of Agent-authored World meaning.
- Treating open Relation wording as an executable rule, remote permission or bypass
  of ordinary World authority.
- Permanent author- or controller-exclusive mutation rights over ordinary World content.
- Treating unguessable identifiers or Agent instructions as protection for hidden World state.
- Locking or versioning a map region merely to prevent equal or nearby Place
  Positions, or inferring Place identity from those Positions.

### Not yet chosen

- Whether durable structural relationships share a Relation base identity and type
  discriminator, open authored statement or only a bounded read over separate exact
  facts, and how those layers avoid duplicating one truth.
- The first general multi-subject proposal that current concrete operations cannot express cleanly.
- The exact causal-dependency and freshness contract when concurrent changes cross.
- How a bounded chain reaction terminates, detects loops and records its involved subjects.
- Which collective ratification mechanism may authorize one shared outcome.
- How a non-Place Position grounds an Entity and its Activity when no
  Place is the direct spatial address.
- Which concrete bounded reads may select an Entity beyond the current Place and
  therefore return its complete Position when one exists.
- Which future concrete gameplay earns World-enforced privacy or visibility state;
  no Position-specific exception or placeholder seam exists in the foundation.
- After the spatial foundation, what exact invocation, confirmation and affected-
  subject bounds the non-default unforeseen-result direction requires.

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
- Current technical candidate — the [completed spatial technical synthesis](../../docs/concept/spatial-five-year-backcast.md#technical-synthesis-after-the-completed-grill) translates the chosen spatial-change direction into a falsifiable transaction and dependency shape without authorizing implementation.
- Related synthesis — [Multiplayer](../multiplayer/README.md), [Movement](../movement/README.md) and [Agent Play](../agent-play/README.md).
- Exact behavior and delivery — [`game/docs/`](../../../game/docs/README.md) and [`dev/docs/evidence/`](../../docs/evidence/README.md).
