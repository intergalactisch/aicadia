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
- A genuine repeat view is an explicit accepted in-World Observation, not an API
  read. Several immutable Observations may concern the same Character and subject;
  each may carry bounded Agent-authored text which is private attributed memory of
  the observing Character, may be wrong and never becomes shared subject truth.
- Observation does not automatically establish Knowledge, recognition or known
  identity. A distant unclear figure may be observed without exposing the exact
  Character behind it, while every Observation still stores one required exact model
  reference for deterministic validation and repeat matching.
- Observation admits an explicit expandable model list: `entity`, `place` and
  `connection` now. Runtime class/table names and speculative types are rejected;
  adding another model requires its own accepted behavior and authorization.
- World tools bound and authorize the structural information a Character receives;
  the Agent decides what the Character could notice and explicitly authors the
  resulting Observations. World stores no visibility candidate list, arrival proof
  or semantic sight decision.
- One Observation action may write several one-model private accounts plus its own
  Activity atomically and idempotently. It may follow Movement, but Movement is not a
  required receipt and recording failure leaves it intact.
- World checks observer control, admitted alias, target existence and bounded
  idempotent storage, but no spatial plausibility, sight or subject revision. Writes
  remain observer-owned and never update, lock or count against the observed subject.
- Within one explicitly User-invoked in-World Agent turn, a private Observation batch
  needs no separate confirmation and is disclosed to the User in that response.
  Shared mutations still require confirmation; subscriptions never invoke BYO Agents
  or create Observations in the background.
- Before narrating an exact model as new, familiar or significant, the Agent reads
  one bounded grouped history through `list_observation` for the exact models it is
  currently considering. Each model group carries several newest-first accounts,
  times and available Activity locations with independent continuation. World stores
  no recognition flag, count or `often` label and never interprets the recurrence.
- A mistaken private Observation is corrected only by appending another ordinary
  Observation account. World stores no correction reference or active version;
  immutable chronological history and Agent interpretation preserve the mistake and
  its later reinterpretation.
- Ordinary spatial context is shared World geography rather than Character-owned
  Knowledge. Protected geography requires a later explicit access model.
- Conflicts follow exact bounded subjects and declared causal dependencies, not a global revision.
- Delivery hints may be disposable or coalesced because clients recover from authoritative bounded reads.
- Two Characters must eventually be able to retain the same unnamed Position
  between Places; World may not force that Position to become a Place
  merely so multiplayer presence can exist there.
- Open-space `enter_world` draws a random Position broadly around one of three offered
  Places without scanning, avoiding or reserving current occupancy. A Character may
  arrive near nobody or near other Characters; co-position remains valid and creates
  no shared counter or correctness lock.
- Possible proximity at entry does not automatically reveal another Character or
  create Observation. Entry stops after placement; a later explicitly User-invoked
  exploration step obtains bounded current context and the Agent decides what is
  noticed.
- Ordinary current World content is jointly changeable rather than permanently
  locked to its first author or Entity controller. Every accepted change remains
  attributable and concurrent attempts still settle against exact current subjects.
- Every open Relation has its own stable non-Entity identity. Changes conflict on
  that exact Relation rather than either endpoint Entity, and an endpoint carries no
  shared Relation count, lock or revision even when it becomes extremely popular.
- Relation reads are bounded by endpoint, direction, cursor and limit. Several
  independent Relations may coexist between the same Entity pair without prose
  similarity, last-write-wins or a universal graph traversal merging them.
- Relation visibility is Character-specific and may hide the Relation's existence,
  endpoints and current state even when one endpoint is otherwise observable. A
  guessed identifier cannot widen that knowledge or mutation boundary.
- A Character may retain an attributed memory of a previously observed Relation,
  while its current hidden state remains unreadable and inactionable as a direct target.
- A remembered point may remain Knowledge, but it is not fresh current Position and
  cannot ground a mutation. The foundation exposes no direct current-Position lookup
  merely because a Character once knew an Entity.
- A bounded current Place read includes complete Position for each positioned Entity
  it already returns. This is one paginated response rather than a per-Entity protocol
  query, and the foundation performs no Character-specific Position redaction.
- A successfully resolved eligible read returns both the calculated current World point and the
  stored Position basis under one freshness boundary. This does not grant new
  separate eligibility, and no global resolved-point cache becomes authoritative.
- When bounded work cannot reach an absolute basis, the same read returns only the
  immediate eligible stored basis. Actions needing the exact point fail closed, so
  load or deep content never turns stale coordinates into multiplayer truth or causes
  descendant-wide rewrite fan-out.
- Cyclic Connection topology is ordinary explicit shared state and never requires a
  global graph lock or full-loop traversal for one move. Each movement settles on its
  exact origin, chosen direction and destination, while cyclic Position references
  remain invalid.
- Each Connection has its own stable identity and revision because several direct
  travel alternatives may join the same Places. Reads are bounded by one endpoint
  and cursor; writes conflict on the exact Connection or its optional reusable
  spatial shape, never on one endpoint-pair row, Place-wide count or graph revision.
- Establishing a Connection never locks or deduplicates by endpoint pair, direction,
  name, description or course. Independently confirmed concurrent alternatives may
  both commit; only an exact request retry or an explicitly selected existing
  Connection is reuse.
- Area records only exact positive coverage. Changing one Place's coverage conflicts
  on that exact Area and never rewrites every intersecting Connection; ordered
  crossings are derived from revision-specific inputs through bounded spatial reads.
- Place-role establishment conflicts on the exact Entity and creates no map-wide
  count or partition. Coordinate-window map reads use spatial indexes, independent
  result bounds and stable continuations, so one crowded city cannot force an
  unbounded response or lock quiet World regions.
- Partial or complete Connection Movement changes only the exact Character Position
  and writes Activity while reading the expected Connection revision. Thousands of
  Characters may occupy or traverse the same course without a Connection-wide
  traveller row, count, lock or progress update.
- A current eligible Relation may let an Agent understand and narrate that one button
  activates one distant bomb, but Relation grants no mechanical authority. The
  explicitly invoked Agent proposes the exact bounded remote Action; World applies
  ordinary subject eligibility, authority, revisions, idempotency and atomic Activity
  without interpreting the Relation text or requiring spatial proximity.
- A claimed Relation may be named as current causal context so World can validate its
  identity, endpoints and revision, but semantic mismatch remains Agent and User
  judgment. If ordinary authority forbids the Bomb change, the Relation never
  overrides that rejection.
- Relative Position writes fail closed unless bounded validation proves an absolute,
  acyclic chain against revisions that remain current at commit. Concurrent A→B and
  B→A attempts therefore cannot both be accepted, while unrelated Position writes
  share no global cycle lock or revision.
- A reference change writes one explicit complete new Position and conflicts only on
  the exact current Position and examined dependencies. No automatic preserve mode
  or descendant fan-out is selected from prose, even when the Agent honors a durable
  authored characteristic in its proposed offsets.
- Creative failure or escalation remains a bounded proposal over exact named
  subjects. An Agent cannot use “unexpected outcome” to modify another User's
  Character, omit dependencies or widen one cup Action into an unbounded Place effect.
- The Agent refuses a User request to author perception, knowledge or another
  current experience for a different User's Character. World never interprets
  prose to police that creative boundary.
- A conforming Agent may use Trait or Position-description meaning to withhold a
  Position detail in player conversation. Because World returned that data, this is
  explicitly not confidentiality against a modified or non-conforming Agent.

### Rejected

- Treating one Place as a universal visibility boundary, lock or infrastructure shard.
- Letting an Agent decide which protected World facts it is eligible to read.
- Making audible, visible or otherwise semantic observation a server-inferred first slice.
- Treating an ordinary World/HTTP/MCP read or retry as a Character Observation;
  repeated personal occurrences require an explicit accepted in-World act and may
  not become a stored view counter or global reverse observer list.
- Writing a durable delivery row for every possible recipient of every change.
- Using last-write-wins, CRDT merge or prose similarity to settle semantic conflicts.
- Treating original authorship as permanent exclusive mutation authority over ordinary World content.
- Relying on identifier secrecy, client filtering or Agent obedience to protect a hidden Relation.
- Claiming that Agent-only Position withholding is World-enforced privacy or a
  security guarantee.
- Treating any Relation as executable permission merely because its Agent-authored
  name or description sounds causal.

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
- How a later privacy and visibility design enforces private information without an
  audience-wide fan-out, global policy row or trust in Agent obedience.
- How one explicitly invited unforeseen result remains bounded to exact eligible
  subjects and current revisions without granting cross-User authority or creating
  a hot global mode.
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

The current spatial foundation supports shared geography while preserving the
Area's subject-local concurrency direction. Exact behavior is defined in
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
- Current spatial rationale and later concurrency risks are recorded in the [spatial technical synthesis](../../docs/concept/spatial-five-year-backcast.md#technical-synthesis-after-the-completed-grill).
- Exact behavior is defined in [`game/docs/`](../../../game/docs/README.md).
- Delivery history and current status: see [direct spatial exploration evidence](../../docs/evidence/spatial-exploration.md).
