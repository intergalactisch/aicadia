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
- During explicitly active play, one Aicadia realtime distribution capability on
  its deployed domain provides default Multiplayer attention to compatible
  ChatGPT-like Apps, Claude-like Apps, terminal clients and browsers. Each host is
  an adapter over the same capability; no browser is required. A compatible active
  client may follow bounded structurally authorized subjects and refetch current
  World state after a hint; the exact set must also cover play without a current
  Place and remains unchosen. BYO Agents continue to use MCP. A bounded waiting tool
  may return activity inside an already active Agent invocation, but notifications
  do not invoke an Agent, spend tokens, grant knowledge or participate in
  settlement. A closed, suspended or disconnected client loses only transient
  delivery and recovers through authorized reads.
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
- Every bounded Agent submission that can produce a durable World result uses one
  lifecycle. One complete, sufficiently authorized Agent result may finish in its
  first call. User and Agent choose no procedural mode. Pure reads, delivery hints
  and private Agent reasoning remain outside this lifecycle. Only an explicitly
  active User-owned Agent may know which meanings belong together and author their
  complete final state. World may validate exact structure, identity, authority,
  bounds, current facts, cooldown, package integrity, atomic commit and Activity; it
  may never infer semantic relatedness, mechanically combine complete Agent states,
  invoke an Agent or spend User tokens. If no active Agent has seen two contributions
  together, Aicadia has no semantic combined result yet.
- A true collision may take several seconds only when the Agent presents that time
  to the player as understandable in-World multiplayer activity and later grounds
  the outcome in canonical World state. A silent wait, spinner, protocol explanation
  or stale-error storm is not acceptable GX.
- Player-visible creative collision is an intended part of Multiplayer: when one
  already active Agent has legitimately received several bounded simultaneous
  contributions, it may use their meaning to author one coherent complete candidate
  that remains recognizably grounded in them. World still understands none of that
  meaning and may only validate and settle exact authorized structure. This chooses
  the experience, not which collisions qualify or how one candidate gains authority.
- World validates authority and exact provenance, never whether a combined result
  understood or fairly represented source intent. One ordinary source request is
  one indivisible authorized contribution. A later Agent may include it unchanged;
  every altered or newly invented fact instead requires that Agent's own
  pre-existing authority or pre-existing collective authority and is attributed to
  that author. Being selected to answer a collision grants opportunity, not power.
- World stores no semantic `collision` object. It may temporarily hold one or more
  complete Agent submissions in the common lifecycle. One may settle directly;
  several may be compatible, conflicting or otherwise related, but only an active
  Agent can understand that distinction and author a combined result.
- A temporarily unresolved set of several submissions has an immutable intake
  cutoff and hard settlement deadline. Later arrivals cannot extend it. After
  settlement, every involved Entity has a bounded cooldown so the accepted result
  cannot be changed again immediately. This is intentional game tempo, not a held
  database transaction or global pause.
- A state-changing attempt during Entity cooldown is not queued or executed later.
  World returns the current settled state and remaining cooldown immediately as a
  normal game outcome, so the active Agent can explain the temporary stability.
  Reads and unrelated play continue.
- Every World manipulation runs through one system that handles one request and
  many simultaneous requests identically: request → short per-subject tick → one
  input applied as authored, or several interacting inputs resolved by one
  content-blind chosen, still-connected requesting Agent that receives the whole
  set → authoritative state plus one attributing Activity → cooldown →
  interest-based notice → Agents narrate. A solo request also enters the tick. The
  chosen Agent is Aicadia's substitute for an MMO rule engine and belongs to the
  core path. `tick` is a working term.
- A request is the Agent's best-grounded package of the player's wish — exact
  subject, intended change, the state it was read on — and carries nothing about
  other players. The submitting Agent never reasons about concurrency; only the
  chosen Agent receives everything about others.
- The core path relies only on ordinary MCP tool calls — one that may stay open for
  the tick and resolution, one follow-up — so it works in Claude Code, Codex CLI,
  ChatGPT-like Apps, OpenCode and similar hosts. A participant's own call is its
  wait; other delivery is acceleration for hosts that have it.

### Rejected

- Treating one Place as a universal visibility boundary, lock or infrastructure shard.
- Letting an Agent decide which protected World facts it is eligible to read.
- Making audible, visible or otherwise semantic observation a server-inferred first slice.
- Treating an ordinary World/HTTP/MCP read or retry as a Character Observation;
  repeated personal occurrences require an explicit accepted in-World act and may
  not become a stored view counter or global reverse observer list.
- Writing a durable delivery row for every possible recipient of every change.
- Using last-write-wins, CRDT merge or prose similarity to settle semantic conflicts.
- Letting World infer permitted semantic parts, reinterpret a source contribution
  or certify that an Agent-authored result was fair to its sources.
- Treating original authorship as permanent exclusive mutation authority over ordinary World content.
- Relying on identifier secrecy, client filtering or Agent obedience to protect a hidden Relation.
- Claiming that Agent-only Position withholding is World-enforced privacy or a
  security guarantee.
- Treating any Relation as executable permission merely because its Agent-authored
  name or description sounds causal.
- Requiring the submitting Agent to declare causal dependencies, preservation
  conditions, combinability or indivisibility markings, mandates, bids or intent
  announcements; concurrency is never the submitting Agent's concern.
- Letting the first request land immediately while only later arrivals enter a
  tick; every request enters the tick.

### Not yet chosen

- What a tick technically is: where pending requests live without process-local
  state, how calls held on different instances are released at settlement, how
  exact subject scope is derived from a request, and its measured length together
  with the cooldown length.
- How the set reaches the chosen Agent on each target host as one fluent turn
  (returned in the open submit call plus one resolving call is the working shape),
  what other participants and bystanders see meanwhile, and how Agents learn that a
  tick exists for a subject.
- Whether a tick may become a bounded gathering or discussion-like surface where
  non-requesting Agents can respond to a pending manipulation without all-to-all
  chat, listener authority or server-invoked Agents.
- Whether attributable Activities, Interactions and ordinary Entities can form the
  non-blocking shared context for complex work instead of a special proposal or chat
  service.
- Which bounded authorized read exposes current state plus temporarily retained
  Agent submissions or attributable Activities, and how it returns the exact
  subject identities and versions a later wait accepts without identifier guessing.
- Which true collisions qualify for the chosen short creative opportunity, which
  already active participating Agent may author its combined complete candidate,
  and when the GX gain justifies added model latency and collision-triggered tempo.
- When a selected Agent may omit an indivisible source contribution, and when
  deliberately collective work needs later human or Agent ratification despite the
  ordinary pre-existing-authority rule.
- Whether previously Agent-authored, collectively accepted deterministic behavior
  may execute repeated known mechanics at server speed, and whether any safe bounded
  execution surface is simple enough for Aicadia.
- The exact structural conflict scope that determines which simultaneous complete
  packages contend for one current state; only an Agent can decide which meanings a
  later combined state should include.
- How current affected state remains stable enough for Agents to understand shared
  history without holding an open request, transaction, connection or Entity lock.
- The exact duration of the short simultaneous selection opportunity, settlement
  deadline and Entity cooldown.
- Who may participate, how many Agent-authored candidates fit, how one exact final
  package becomes authorized and what happens without an authorized result before a
  hard deadline.
- The exact in-World presentation an Agent gives when a state-changing attempt meets
  Entity cooldown, without exposing protocol state or requiring prescribed prose.
- How current Agents or previously accepted Agent-authored deterministic behavior
  produce compatible, contradictory and deliberately joint results without World
  inference or network-arrival authority.
- Whether automatic Agent preservation of unrelated current facts remains necessary
  after World produces one verified in-World outcome.
- Whether an earned hotspot uses only PostgreSQL coordination or one lazy virtual
  execution lane keyed to the smallest exact conflict subject. Such a lane would be
  replaceable infrastructure, not an Entity, canonical content copy or truth store.
- The explicit connected-User, mutation-rate, latency, rejection and recovery targets for production scale.
- Which public web transport, authenticated watch contract, host adapter contract
  and bounded message shape let compatible ChatGPT-like Apps, Claude-like Apps,
  terminal clients and browsers surface default realtime attention. The retained
  Codex `0.149.0` experiment already rejects standard MCP resources as an assumed
  universal ambient path for every BYO host. Experiment 07 supports the local
  HTTP, browser and official MCP wait mechanism but leaves Codex continuation
  inconclusive because its first read circularly required an undiscoverable exact
  subject.
- Which exact structurally grounded representation or bounded set of representations
  anchors default attention both with and without a current Place; no spatial model
  or universal context resource is accepted here.
- The exact active-observer authorization, movement, reconnect,
  coalescing and slow-consumer contract.
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

- Pressure the common World path against one Action, movement, discovery, remote
  multi-Entity change and deliberate joint construction before choosing timing or
  storage representation.
- Pressure attributable World-history as shared Agent context against S02, S06, S10,
  S11 and S13 without claiming World can recognize semantic relatedness.
- Establish which BYO hosts can continue one already active User invocation after a
  bounded MCP result, while treating that continuation as optional acceleration.
- Test communication without final settlement across two real BYO hosts: one Codex
  CLI Agent and one Claude Code Agent submit different complete changes for the same
  Entity, receive the same content-free delivery semantics and independently reread
  and present the other's attributable contribution. Keep smaller-model
  comprehension and real ChatGPT/Claude App embedding as explicit follow-up seams.
- Directly test whether one real BYO Agent can receive a bounded immutable collision
  set after its first tool call, author one exact authorized combined state and
  explain the result within a GX-acceptable delay.
- Test player-visible collision presentation separately from settlement: determine
  whether target BYO hosts can show an Agent-authored in-World update between tool
  calls and whether a final recap alone still makes the delay legible and fun.
- Compare short content-blind selection, bounded same-invocation handoff and later
  World-history synthesis without importing majority, score or listener authority.
- Research a minimal deterministic Agent-authored execution surface separately from
  arbitrary scripts, including security, resource bounds, authority and upgrades.
- Pressure final reread and cooldown-backed stability against one deliberately hot
  Entity and griefing.
- Pressure exact-current-fact and whole-Entity cooldown against independent changes,
  deliberate stability, griefing and one extremely hot Entity.
- Measure PostgreSQL contention and bounded reads for deliberately hot Places and Entities.
- Verify which current Agent hosts and transports can use hints, resume markers and
  refetch patterns; the pinned Codex `0.149.0` standard resource path is already
  refuted by experiment 06.
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

The accepted core direction sends every World manipulation through one tick-based
system. A request — the Agent's best-grounded package of the player's wish, carrying
nothing about other players — enters a short per-subject tick. One input is applied
as authored. Several interacting inputs are handed as one complete set to one
content-blind chosen, still-connected requesting Agent, which authors the outcome in
one follow-up call; World validates identity, authority, versions and structure,
commits current state plus one Activity attributing every contribution, returns the
outcome to every still-open call and starts the Entity cooldown. World never infers
meaning or combines packages; the chosen Agent is the substitute for an MMO rule
engine. The core path uses only ordinary MCP tool calls across hosts. The exact
technical tick, its length, the cooldown length and per-host fluency remain open; see
the [active concept](../../docs/concept/multiplayer-first-principles.md#corrected-core-direction-one-tick-based-system-for-every-manipulation).

Attributable World history and ordinary in-World content remain the shared Agent
memory for work that is not simultaneous. A later explicitly active Agent may cite
bounded prior Activities and author a new complete state. The chosen Agent's
same-invocation handoff is the core live path; durable progress still never depends
on that host remaining active because the deterministic fallback settles.
Simultaneous complete states that no Agent has seen together may only be selected,
ordered or left unchanged through content-blind World handling.

Three blank-slate scenario-driven designs independently converged on complete
Agent-authored candidates, exact declared dependencies, fact-sized structural
conflict, atomic state and Activity, lossy hints and bounded recovery. Their strongest
new GX candidate turns true same-fact collision into a bounded live edit by one
already active participating Agent. The first real-Agent experiment was
end-to-end inconclusive because its tool contract contradicted itself, but it showed
the Agent correctly narrating hostile simultaneous input and authoring a coherent
two-source state; the User judged that experience worth retaining as Multiplayer
direction. Real BYO host presentation, settlement, model latency, structural
accuracy and hostile collision remain unproved as a complete system.

Several seconds of true collision time are now acceptable only as visible gameplay:
the Agent must tell the player, in World terms, that several changes coincide and
must ground the eventual outcome in canonical state. Agent-authored synthesis of
several bounded contributions is now part of the intended experience; the exact
creative authority, collision eligibility and host/tool behavior needed to surface
and settle that moment remain unchosen.

A separate future research branch asks whether Agents can leave narrow deterministic
behavior behind for repeated known mechanics so World executes earlier authored
intelligence at server speed. This is not permission for arbitrary scripts or a
general rule engine. History lineage, live handoff, conflict scope, selection,
cooldown, executable behavior, attribution, implementation and public contract remain
unchosen and unauthorized.

MCP resource listeners remain a valid protocol-level attention accelerator: the
official-SDK experiment marked one exact bounded view dirty, refetched it and
recovered current state after reconnect. The sole pinned Codex `0.149.0` run did not
list, read or listen to that resource even though it connected successfully. Standard
MCP resources therefore cannot be assumed as Aicadia's universal default carrier;
the retained [experiment 06](../../lab/multiplayer/06-place-resource-subscription/README.md)
owns that bounded refutation. A listener still would not invoke an Agent, carry
authoritative content, prevent collision or replace final validation. Application
presentation and a bounded same-invocation tool return remain open transport
candidates, not chosen production behavior.

The retained [experiment 07](../../lab/multiplayer/07-host-independent-change-wait/README.md)
supports register-then-recheck, reconnect, coalescing, the shared HTTP/official-MCP
contract, a real browser adapter and `10,000` local waiters. Its sole Codex run never
entered the wait because `get_state` required the exact subject before any tool
exposed it. A future Agent-facing wait must accept only subject identities and known
versions obtained from a preceding authorized World result. The pinned Codex wait,
production transport, multi-instance fan-out and final attention scope remain
unproven and unaccepted.

Realtime delivery is only the attention path. The strongest current unaccepted
communication shape stores each bounded Agent contribution once, makes the exact
declared subject view stale and lets another explicitly active Agent reread eligible
current state plus attributable work before authoring another complete state. Agents
therefore communicate indirectly through World records rather than all-to-all chat;
World routes exact declared subjects but never infers semantic relatedness. The
exact read surface, final-author rule and settlement behavior remain unchosen.

### Absent

Production subscriptions, a delivery broker, per-observer truth, global revisions,
background Agents, semantic merge logic, a general event/rule engine and collective
settlement are absent. Their mention here does not authorize their implementation.

## Sources

- Prepared pressure — [Multiplayer scenario catalogue](scenarios.md).
- Retained rationale — [mass concurrency and living World direction](../../docs/concept/concurrency-and-world-dynamics.md).
- Active recommendation — [Multiplayer from first principles](../../docs/concept/multiplayer-first-principles.md); the [five-year Multiplayer backcast](../../docs/concept/multiplayer-five-year-backcast.md) is retained superseded exploration.
- Sourced findings — [Three blank-slate Multiplayer mechanics](../../docs/research/blank-slate-multiplayer-mechanics.md), [BYO-Agent coordination without server inference](../../docs/research/byo-agent-coordination-without-server-inference.md), the superseded but retained [recursive Agent synthesis analysis](../../docs/research/recursive-agent-world-state-synthesis.md), [Multiplayer resolution from first principles](../../docs/research/multiplayer-first-principles-resolution.md), [multiplayer concurrency and World observation](../../docs/research/multiplayer-concurrency-and-world-observation.md), [persistent multiplayer GX and concurrency patterns](../../docs/research/persistent-multiplayer-gx-concurrency-patterns.md) and the [spatial multiplayer foundation](../../docs/research/spatial-multiplayer-foundation.md).
- Experiments — [Multiplayer Lab track](../../lab/multiplayer/README.md).
- Current spatial rationale and later concurrency risks are recorded in the [spatial technical synthesis](../../docs/concept/spatial-five-year-backcast.md#technical-synthesis-after-the-completed-grill).
- Exact behavior is defined in [`game/docs/`](../../../game/docs/README.md).
- Delivery history and current status: see [direct spatial exploration evidence](../../docs/evidence/spatial-exploration.md).
