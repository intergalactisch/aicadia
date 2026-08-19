# Multiplayer

> **Role / side:** current Multiplayer development synthesis / development side.
> **Authority:** owns the current meaning, boundary, decisions, unresolved landscape, components and directional technical model for Multiplayer.
> **Excludes:** selected work, exact game behavior, sourced findings, experiment verdicts and delivery claims; those remain in `dev/backlog/` and plans, `game/docs/`, `dev/docs/research/`, `dev/lab/` and `dev/docs/evidence/`.

## Meaning

Multiplayer concerns many explicitly invoked, User-owned Agents acting on and
learning about overlapping subjects in one persistent shared World. The World must
settle each bounded action deterministically, preserve coherent current truth and
history, and let a returning Agent recover useful context without turning delivery
into truth or spending tokens in the background.

## Boundary

### This is

- Concurrent action on the same or related Characters, Places and Entities.
- Bounded observation, attention and reconnect behavior over shared durable truth.
- Explicit conflict and causal-dependency handling for one deliberately hot subject.
- Deterministic collective settlement when several eligible Agents must produce one result.

### This is not

- A server that invokes Agents, interprets prose or spends User tokens automatically.
- A promise that every Character receives, stores or understands every World change.
- A global World lock, revision, feed or process-local correctness mechanism.
- Network delivery, a subscription hint or personal memory being authoritative World truth.

## Decisions

### Chosen

- Persistent placement and active attention are separate facts.
- Attention is explicit and opt-in; presence alone does not activate an Agent.
- Durable public history and a User's private remembered experience are separate.
- World determines structural eligibility for information; the Agent decides how to present eligible facts.
- Conflicts follow exact bounded subjects and declared causal dependencies, not a global revision.
- Delivery hints may be disposable or coalesced because clients recover from authoritative bounded reads.
- Two Characters must eventually be able to retain the same unnamed Position
  between Places; World may not force that Position to become a Place
  merely so multiplayer presence can exist there.
- Ordinary current World content is jointly changeable rather than permanently
  locked to its first author or Entity controller. Every accepted change remains
  attributable and concurrent attempts still settle against exact current subjects.
- Relation visibility is Character-specific and may hide the Relation's existence,
  endpoints and current state even when one endpoint is otherwise observable. A
  guessed identifier cannot widen that knowledge or mutation boundary.
- A Character may retain an attributed memory of a previously observed Relation,
  while its current hidden state remains unreadable and inactionable as a direct target.

### Rejected

- Treating one Place as a universal visibility boundary, lock or infrastructure shard.
- Letting an Agent decide which protected World facts it is eligible to read.
- Making audible, visible or otherwise semantic observation a server-inferred first slice.
- Writing a durable delivery row for every possible recipient of every change.
- Using last-write-wins, CRDT merge or prose similarity to settle semantic conflicts.
- Treating original authorship as permanent exclusive mutation authority over ordinary World content.
- Relying on identifier secrecy, client filtering or Agent obedience to protect a hidden Relation.

### Not yet chosen

- The explicit connected-User, mutation-rate, latency, rejection and recovery targets for production scale.
- The exact active-observer subscription and reconnect contract.
- The dependency tokens and conflict rules for multi-subject change packages.
- Admission and overload behavior for one extremely busy Place or Entity.
- The exact co-location, proximity and contention rules for shared Positions.
- Which exact spatial, Relation and observation facts make a Relation knowable
  now, and how a grounded investigation may test remembered but non-current knowledge.
- Whether one confirmed proposal may atomically combine an authored Relation with
  the exact Position, attachment or other structural fact on which it is based.
- The first deterministic collective-settlement capability and its eligible participants.

## Research needed

- Measure PostgreSQL contention and bounded reads for deliberately hot Places and Entities.
- Verify which current Agent hosts and transports can use hints, resume markers and refetch patterns.
- Test reconnect summaries that recover current truth plus relevant recent context without per-recipient queues.
- Pressure-test authorization-aware, paginated Relation reads for one very hot Entity
  without a shared counter, endpoint-row lock or disclosure of hidden endpoints.
- Compare bounded collective-settlement mechanisms against the multiplayer scenario catalogue.

## Components

| Component | Current meaning |
| --- | --- |
| Truth | Authoritative current state and atomic Activity, read from World. |
| Contention | The smallest subject and transaction scope that must serialize one change. |
| Attention | An explicit, temporary interest in eligible changes; not placement or Agent activation. |
| Delivery | Best-effort notice that prompts an authoritative refetch; not a truth store. |
| Recovery | A bounded current baseline and relevant history after missed delivery or disconnect. |
| Collective outcome | One deterministically settled result from explicitly eligible Agent inputs. |
| Scenarios | The [fourteen hard cases](scenarios.md) used to pressure decisions and experiments. |

## Technical model

### Delivered

The current World uses PostgreSQL as authority, commits state and attributable
Activity atomically, supports bounded history reads and safe confirmed retries, and
serializes contextual writes through the affected Place. It has no live subscription
or collective-settlement capability. Exact delivered behavior remains in
[`game/docs/`](../../../game/docs/README.md).

### Directional

Changes name exact affected subjects and causal dependencies; World validates them
inside the smallest safe transaction. A durable occurrence is stored once. Eligible
clients may receive a disposable hint and recover through a bounded baseline plus
authoritative history, without server-side Agent invocation.

### Absent

Production subscriptions, a delivery broker, per-observer truth, global revisions,
background Agents, semantic merge logic, a general event/rule engine and collective
settlement are absent. Their mention here does not authorize their implementation.

## Sources

- Prepared pressure — [Multiplayer scenario catalogue](scenarios.md).
- Retained rationale — [mass concurrency and living World direction](../../docs/concept/concurrency-and-world-dynamics.md).
- Sourced findings — [multiplayer concurrency and World observation](../../docs/research/multiplayer-concurrency-and-world-observation.md) and the [spatial multiplayer foundation](../../docs/research/spatial-multiplayer-foundation.md).
- Experiments — [Multiplayer Lab track](../../lab/multiplayer/README.md).
- Exact behavior and delivery — [`game/docs/`](../../../game/docs/README.md) and [`dev/docs/evidence/`](../../docs/evidence/README.md).
