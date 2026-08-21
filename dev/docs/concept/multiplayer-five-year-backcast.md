---
status: retained
---

# Five-year multiplayer backcast

> **Superseded live recommendation:** retained as exploration history after the
> 2026-08-19 [first-principles Multiplayer reset](multiplayer-first-principles.md).
> Its unaccepted direction no longer constrains the active grill.

> **Role / side:** retained five-year Multiplayer recommendation / development side.
> **Authority:** records the future observation, recommended multiplayer shape,
> scenario application, technical candidate, rejected alternatives, falsifiers and
> backward route produced by the 2026-08-19 `5year` exercise.
> **Excludes:** accepted Multiplayer Area choices, current game behavior, canonical
> vocabulary, production schema, API and implementation; those require later User
> decisions and their own authorities.

Date: 2026-08-19

Status: **retained superseded recommendation**. Nothing in this record was accepted.
`Operation settlement`, `attention set`, `coordinated operation` and every
illustrated input below remain historical working descriptions, not accepted
Aicadia vocabulary or public capability names.

The source basis is the retained research on [mass concurrency](../research/massively-concurrent-dynamic-world.md),
[multiplayer observation](../research/multiplayer-concurrency-and-world-observation.md),
[persistent multiplayer GX](../research/persistent-multiplayer-gx-concurrency-patterns.md),
[Agent-authored bounded intents](../research/agent-authored-world-intents.md),
[interest delivery](../research/entity-place-interest-subscriptions-at-scale.md),
[change propagation](../research/postgres-change-propagation-and-fanout.md) and the
previous [unified World-change candidate](../research/unified-world-change-system.md).
The fixed pressure is the [fourteen-scenario Multiplayer catalogue](../../areas/multiplayer/scenarios.md).
The retained [Multiplayer labs](../../lab/multiplayer/README.md) prove only their
bounded semantic and PostgreSQL fixtures.

## Executive recommendation

Five years from now, Aicadia should not have one generic multiplayer subsystem, one
universal `submit_change` tool, one Place revision or one proposal board through
which every kind of play must pass. Multiplayer should be a property of every
concrete game operation.

The recommended foundation is:

1. **Agents play through concrete capabilities.** An Agent introduces, changes,
   moves, investigates, interacts or joins a concrete future group activity. It
   does not program a generic World transition language over MCP.
2. **Every modifying capability follows one small settlement discipline.** Its
   contract names the actor, structural authority, exact facts that must still be
   current, bounded intended writes, Activity roles and result. The operation owns
   the meaning; shared internal code may own validation, idempotency, locking,
   Activity and commit mechanics.
3. **The World settles; it does not complete the Agent's story.** The explicitly
   invoked Agent supplies semantic judgment and every intended consequence. World
   validates only identity, authority, explicit structure, freshness, bounds,
   idempotency and atomic history.
4. **Concurrency policy is Agent work, not player work.** A player confirms one
   recognizable in-World result. Within that explicit turn, the Agent rereads and
   automatically preserves every current fact outside the action's explicit affected
   facts. Only a materially changed game outcome returns to the player. A true
   contradiction still needs one capability-owned game result; raw stale is a
   correctness signal, not sufficient GX.
5. **One accepted operation leaves one coherent result.** Current state and its one
   attributable Activity commit atomically. The same commit identifies the bounded
   authoritative resources whose views may now be stale; delivery remains derived,
   disposable and non-authoritative.
6. **Attention is a read-and-notice protocol, not another World truth.** An active
   host establishes an authorized bounded baseline, keeps a small explicit set of
   relevant resources live when its transport supports that, receives coalescible
   invalidations and refetches. No hint invokes an Agent or becomes Character
   memory.
7. **Multi-Agent assembly is a concrete game capability, never the default conflict
   handler.** Several eligible Agents may contribute to lifting, constructing,
   naming or deciding something only when that operation defines participants,
   inputs, bounds and deterministic settlement. A busy table does not automatically
   become a parliament.
8. **Hot subjects coordinate locally and degrade as game mechanics.** One fact that
   millions of Agents try to contradict is necessarily ordered. A later measured
   hotspot may receive one lazy virtual execution lane and a bounded action frame,
   keyed below Entity where invariants allow it. Admission still protects quiet
   work, but raw stale or busy may not become the ordinary player-facing rhythm.
9. **Occurrences are stored once; audience work stays proportional to active
   interest.** Structural eligibility and privacy are checked before delivery. A
   million active recipients still cost a million deliveries, so payloads are tiny,
   shared work is reused and slow consumers converge by refetch rather than replay.
10. **Infrastructure may change without changing the game contract.** Start with
    PostgreSQL and stateless World instances. Exact coordination, notification,
    routing, projections, partitions or a later distributed store are replaceable
    implementations behind the same concrete operation and read contracts, earned
    only by measured pressure.

The central correction to the previous direction is subtle but material: Aicadia
needs one **discipline** for settling accepted operations, not one universal
**domain object or public package** that pretends creation, movement, an explosion,
ambient noticing and communal authorship have the same game meaning. Truth stays
singular because every operation commits through World; the game remains legible
because each capability says exactly what it does.

## The future World this must support

### Ordinary play after five years

One explorer investigates an unnamed edge of the World while another group builds
inside a distant settlement and thousands of Characters gather around a famous
bridge. The independent work does not touch a World-wide sequence or Place-wide
mutation lane. Every accepted operation remains visible later through its exact
subjects and Activity.

At the bridge, two Agents prepare changes from overlapping context. Each rereads the
small exact facts it needs immediately before submission. If painting one rail and
adding a marker to another are independent, the Agents preserve all unaffected
current facts and both commit without exposing concurrency language to their Users.
If both demand contradictory values for one exact fact, the bridge capability's
accepted game rule orders, contests or deliberately coordinates them. The database
does not get to invent that GX by returning an error. If the players deliberately
want to raise one arch together, they enter a concrete coordinated bridge operation
whose participant and settlement rules are part of that mechanic rather than
inferred from crowd size.

An Agent presses a structurally connected remote control. A concrete remote-action
capability decides whether the button and remote subject may be changed in one
bounded operation. World never traverses arbitrary Relation prose or invents a
blast. If the scene requires a later intelligent response, another User must invoke
an Agent for that later Action; a notification cannot spend the User's tokens.

An occurrence makes local context stale for active Characters whose authorized
attention intersects its structural scope. Their hosts receive at most a small
notice and refetch. Their Agents decide how the occurrence reads as sight, sound or
meaning from authoritative Character and scene context. A disconnected Character
does not secretly perceive it; on return its Agent can read current state and
bounded public or involved history without claiming continuous attention.

An Agent can also work on a distant known Entity while its Character participates
locally, when a concrete capability grants that remote basis. Multiplayer relevance
is therefore not identical to proximity. The active set may include current
Character context, current Place and a few exact known subjects without turning the
whole World into a live feed.

### Culture and fun that emerged

- Places develop through many attributable interventions rather than permanent
  first-author ownership or anonymous last-write-wins state.
- Crowds are socially meaningful because a shared subject stays one subject. The
  server does not secretly clone a monument or festival to escape contention.
- Agents negotiate surprising consequences in player language, but submit small
  exact operations that other Agents can inspect and react to.
- Collaborative rituals emerge around concrete constructions, discoveries and
  decisions; the World records participation without assigning ranks, points or
  universal voting power.
- Returning players encounter changed current truth plus selected history, not an
  impossible transcript of everything that happened while they were away.
- A famous Entity can remain globally knowable while only a bounded explicit set of
  active hosts keeps it live.

### Abuse that also emerged

- Bots hammer one public Entity with distinct request identities to exhaust locks,
  retries or Activity storage.
- Griefers alternate compatible-looking facts so other Agents repeatedly prepare
  stale multi-subject operations.
- Attackers subscribe to enormous resource sets, reconnect repeatedly, or use
  notification timing and neutral errors to probe private Places and Relations.
- Agents submit packages at every cardinality bound, construct deadlock-prone
  subject orders or hide unbounded semantic consequences behind small prose.
- A charismatic or coordinated group tries to turn listener count, proposal volume
  or token spend into authority over communal state.
- A global occurrence or popular subject produces a refetch stampede even though
  the change itself was stored only once.
- Malicious World prose and Agent proposals carry prompt injection or persuasive
  content into other Agents' contexts.

### Failures the mature system survives

- A World instance fails after commit but before replying. The same request identity
  returns the one accepted result and never duplicates Activity.
- A gateway loses, duplicates or reorders invalidations. Authorized refetch returns
  current truth; delivery never mutates World.
- A hot subject reaches its wait or admission bound. That subject returns a bounded
  busy result while quiet subjects retain database and instance capacity.
- A slow host falls behind. Repeated dirtiness coalesces; the host receives a gap or
  baseline requirement and refetches current state plus bounded relevant history.
- A projection, replica or subject router is stale or unavailable. It may delay a
  read or hint but cannot authorize or settle a modification.
- A multi-subject operation finds one stale, missing, private or unauthorized
  subject. The complete operation rejects without partial current state, Activity or
  dirty-resource output.
- Operational repartitioning moves authority behind the scenes. Stable Entity,
  Place and Activity identities and the result of the same bounded operation remain
  unchanged.

## What survived the backcast

| Surviving truth | Player or Agent consequence | Technical consequence |
| --- | --- | --- |
| Concrete operation meaning | an Agent can explain and confirm one recognizable game action | operation-specific input, authority, errors and result; no generic public patch language |
| Shared settlement discipline | every accepted change feels coherent and retry-safe | bounded decode, idempotency, exact preconditions, stable coordination, atomic state plus Activity |
| Exact invalidating facts | unrelated play composes while contradictions do not | logical per-fact dependencies including expected absence; representation remains replaceable |
| Automatic non-interference | a player names the game result while the Agent absorbs compatible collision | bounded reread; explicit affected facts; preserve every other current fact; renewed player choice only for a materially changed outcome |
| Singular current truth | every Agent returns to the same World answer | one authoritative state transition; no CRDT or per-observer canon |
| Attributable history | players can establish who changed what and where | one immutable Activity with stable subject roles in the same transaction |
| Separate attention | active play feels live without background thought | baseline, explicit interest, lossy hint, authorized refetch and explicit Agent invocation |
| Bounded recovery | returning does not require replaying the World | current state first, then paginated public/involved/exact-subject history lenses |
| Explicit cooperative mechanics | several Agents can genuinely do something together | operation-owned eligibility, phase/input bounds and deterministic settlement |
| Game-owned hotspot outcomes | famous shared subjects remain real without collapsing quiet play or becoming error screens | local coordination, bounded admission and operation-specific order, frame, contest or cooperation |
| Replaceable operations infrastructure | growth does not rename World concepts | stateless instances, routed subject work and earned delivery/storage layers |

## Recommended conceptual model

### 1. Multiplayer is a loop, not a model hierarchy

The smallest player-facing loop is:

1. the explicitly invoked Agent reads bounded authoritative context;
2. the Agent reasons about one concrete game capability;
3. it presents the complete recognizable in-World result and receives User
   confirmation without asking for a concurrency policy;
4. an optional MCP listener may already have marked its bounded context dirty; in
   every case the Agent rereads the exact dependent facts before submission and
   preserves every current fact outside the action's explicit affected facts;
5. it submits one bounded idempotent operation with the exact current facts on which
   its final result depends;
6. World accepts or returns one exact structural result; a true contention outcome
   follows the concrete capability's accepted game rule rather than generic merge;
7. one acceptance stores current state and Activity atomically;
8. relevant active hosts may learn only that authorized context is stale; and
9. every host refetches World truth before presenting or preparing another change.

This loop is the common MCP comprehension target. It does not require Agents to
understand locks, rows, partitions, notification brokers or a universal transaction
AST.

### 2. A concrete capability owns its semantic boundary

Every modifying capability should answer the same questions in its own contract:

- Who is the acting Character and accountable User?
- What structural fact grants this operation authority here and now?
- Which exact current or expected-absent facts could invalidate the intended result?
- Which bounded current facts will change if accepted?
- Must all changes be atomic, or is a later response a separate explicit Action?
- Which stable subjects and roles enter the one Activity?
- Which authoritative resources are different after commit?
- Which precise retry, stale, busy, unavailable and invalid outcomes can the Agent
  act upon without learning protected facts?

The answer is deliberately different for movement, discovery, Interaction, remote
activation and collaborative construction. Shared settlement code may consume their
validated form, but it may not erase those differences from `World`, HTTP, MCP or
the Agent-facing descriptions.

### 3. Agents absorb compatible collision; true conflict is game-owned

Ordinary operations do not wait for every nearby or interested Agent. They settle
against exact current facts. Before final submission, the explicitly invoked Agent
automatically rereads and recomposes its bounded operation while preserving every
current fact outside the action's explicit affected facts. The player names and
confirms the game result, not which versions, fields or concurrent changes to retain.
The Agent may not silently alter that recognizable result, expand affected subjects
or continue as a background Agent.

This keeps the World responsive and prevents one offline User, one slow LLM or one
million spectators from becoming a lock while using the Agent's semantic ability to
hide harmless storage races. World does not interpret compatibility from prose: the
concrete capability and submitted exact state make it structurally checkable.

If two outcomes still contradict one exact fact, serialization cannot make both
true. The concrete capability must eventually own a recognizable game result such
as immediate order, a bounded action frame, explicit contest or an opened
cooperative operation. Until one concrete capability chooses that rule, raw stale
remains an honest technical boundary but not an accepted Multiplayer foundation.

An MCP resource listener can shorten the stale window by marking one exact bounded
current-context resource dirty while the User considers an action. It is optional:
the host may miss every hint, and the next explicit Agent turn still rereads before
its write. A notification neither invokes the Agent nor settles the conflict. A
published uncommitted intent would be a separate explicit cooperative or contest
mechanic, not ordinary transport behavior.

When several Characters must contribute before one result may exist, the concrete
capability introduces that participation explicitly. It defines a bounded subject,
eligible participants, accepted input shape, expiry or completion condition and one
deterministic final settlement. Agents may supply candidate meaning; World owns
identity, eligibility, bounds, time and the exact accepted state transition.

This is not a generic consensus layer. A future bridge-raising operation may require
three controlled Characters or three distinct structural contributions. A communal
name may use another accepted rule. Neither rule applies to painting a table simply
because many Users are watching it.

### 4. Observation is authorized invalidation plus refetch

World truth does not contain durable subscriptions, recipient queues or personal
awareness for every ambient occurrence. The smallest live contract has four steps:

1. **baseline:** read the current authorized Character, Place or Entity resource;
2. **attention:** explicitly retain a small connection-owned set of those resource
   identities while the User keeps that play context active;
3. **notice:** receive a content-free or minimally typed invalidation that may be
   coalesced, duplicated or lost; and
4. **refetch:** read current state and a bounded authorized Activity lens.

World-owned privacy and structural eligibility happen before a resource identity or
hint reaches a host. The Agent then decides which eligible facts matter and how to
render them. A notification is never evidence that a Character perceived something
or that an LLM processed it.

A portable explicit-turn read remains the recovery floor. A host is fully live only
when it also supports an accepted attention path during an explicit active play
context. Host-specific adapters may implement that semantic contract, but provider
names never enter game authority or capability behavior.

### 5. History is complete; attention is deliberately incomplete

Every accepted state-changing operation remains durable and queryable. That does not
mean every Character receives or remembers every operation. Recovery begins with
exact current state and then reads bounded lenses over the one Activity store:

- the acting or directly involved Character's history;
- public history for an exact eligible Place or subject;
- exact Entity history for a known subject; and
- later mechanic-specific history only when a concrete feature earns it.

Hints may collapse a hundred changes into “this resource is stale.” A User who was
offline may learn what changed publicly, but the Agent may not rewrite that as
continuous personal sight or hearing. Strong personal detection, secrecy or response
guarantees require their own concrete mechanic and bounded durable state.

### 6. Scope comes from game structure, not multiplayer infrastructure

The ongoing spatial work will decide how exact Position, Place and structural reach
work. Multiplayer must consume those accepted truths; it must not invent a district,
area, proximity graph or universal neighborhood merely to route changes.

Likewise, a remote button needs a concrete structural or capability basis, not a
generic causal graph. An occurrence may make a compact authorized scope stale
without mutating every matched Entity. Literal state changes to many independent
Entities remain proportional bounded work.

“Everything appears blue” should first be modeled as an explicit World or spatially
scoped phenomenon that eligible Agents can read and interpret, if that gameplay is
accepted. It should not justify a universal effect engine. If every Entity's own
`color` fact must literally become blue, the change is honestly many bounded writes.

## Recommended technical shape

### 1. Logical operation input

The five-year contract needs these meanings, not necessarily one stored or public
struct:

- stable request identity and normalized fingerprint;
- accountable User and acting Character derived from authenticated context;
- concrete capability kind and its operation-specific bounded payload;
- exact stable subject identities;
- structural authority basis and versions the operation requires;
- typed expected current and expected-absent facts;
- typed intended current-state writes;
- stable Activity subject roles and canonical result shape; and
- mechanically derivable changed resource identities.

An Agent should not submit database versions it cannot understand. MCP capabilities
should expose opaque freshness tokens or concrete expected facts with clear retry
semantics. World may add mandatory invariants the Agent cannot omit.

### 2. Settlement path

For one modifying request, any World instance performs one bounded route:

1. strictly decode, normalize, cardinality-check and fingerprint before scarce work;
2. resolve an equal accepted retry or conflicting reuse of the request identity;
3. validate the Agent's final exact package against the concrete capability's typed
   authority, scope and bounds; World does not know the private confirmation or
   reason for the Agent;
4. acquire admission and database capacity without unbounded waiting;
5. coordinate the smallest logical conflict facts in one stable order;
6. re-read authority, privacy, structure, current values and expected absence inside
   the transaction;
7. apply the capability's complete intended state or nothing;
8. insert one Activity and exact subject roles;
9. derive changed authoritative resources from the accepted writes and occurrence;
10. commit once; and
11. return the canonical accepted or exact non-acceptance result that the Agent can
    translate into the capability's player-facing rhythm.

The logical conflict coordinate may be an existing current row, a conservative
Entity coordinator, a persistent exact slot or a later storage primitive. The
retained PostgreSQL labs show that exact Property slots handle present and absent
facts more uniformly than the tested hybrid, but they do not prove that every game
fact now deserves a slot table. The operation contract should commit to exact
semantics before schema chooses the cheapest proven representation.

### 3. Multi-subject atomicity

One operation may touch a hard-bounded set of subjects when partial acceptance would
make its defined game result false. It coordinates those subjects in stable identity
order and validates every dependency before writing. A missing or stale member
rejects the whole package.

This does not make arbitrary causal chains atomic. A remote button, an explosion and
a linked Entity introduction each need a concrete capability that states their
maximum subjects and authority. Work beyond that bound is either several separately
attributable explicit Actions or a later earned bounded workflow. World never walks
free Relations until no more consequences remain.

### 4. Hot-subject admission

Millions of distributed operations are served by distributed subjects. Millions of
incompatible writes to one fact remain one serial decision point. An Entity-sized
content copy is rejected because it would fork canonical truth. A measured hot fact
may later receive one lazy, movable virtual execution lane keyed to the smallest
coordination subject, not automatically the entire Entity. The lane may briefly
micro-batch mechanically compatible inputs, but durable truth remains PostgreSQL
state plus Activity and the lane may disappear without losing an accepted result.

The contract for that point should define:

- maximum decoded subjects and facts before database admission;
- pool-acquisition, lock and statement time budgets;
- one small Agent or server retry budget only where it cannot change the confirmed
  player outcome;
- a subject-scoped overload result and capability-owned GX with bounded retry
  guidance;
- no Activity for rejected, timed-out or shed work;
- metrics by operation and conflict coordinate, never an in-game counter; and
- a quiet control whose latency and acceptance remain protected during every load
  test.

Fairness cannot be inferred from a queue. If one concrete mechanic later needs
per-User, Character or participant fairness, its contract must choose that meaning.
Popular subjects are not cloned, skipped or silently made stale to improve graphs.

An action frame, virtual lane or queue is not itself a conflict resolver. If two
inputs require incompatible outcomes, the concrete game capability still decides
which order, contest or cooperation rule gives the collision meaning.

### 5. Delivery and recovery

The first implementation may use one compact post-commit PostgreSQL notification to
a bounded gateway fleet. Gateways keep ephemeral authorized attention membership,
coalesce repeated dirtiness by resource and push tiny invalidations to active hosts.
Hosts then refetch through ordinary bounded World reads.

If measured gateway count, write rate or connection fan-out defeats that path, a
subject router, outbox or change feed may replace it. Such a component carries no
game authority and owns no canonical replay. Reconnect always re-establishes
authorization and baseline before attention; private content never appears in a
global topic or content-bearing notification.

There is no global World cursor that every operation mutates. Resource freshness and
Activity pagination remain subject- or lens-scoped. A future transport watermark may
exist as rebuildable delivery state, but it cannot become World identity or
mutation correctness.

### 6. Operational growth

Five-year scale may require routing subjects to operational cells, partitioning
history, serving bounded-stale reads from replicas and rebuilding search or interest
indexes from authoritative truth. Those decisions remain invisible to Agents.

The semantic contract permits only bounded multi-subject operations, making their
future routing cost explicit. If a measured backend can no longer settle an accepted
cross-partition capability, Aicadia must either adopt storage that preserves that
atomic boundary or deliberately revise the game capability with User acceptance. It
may not silently make the same operation partially consistent.

## Scenario pressure

| Scenario | Five-year handling |
| --- | --- |
| S01 one Agent changes one Entity | concrete create and state-change capabilities share settlement discipline, Activity and retry without one public patch tool |
| S02 thousands change one table | exact invalidating facts compose or conflict; one hot fact is admitted and ordered, not merged semantically |
| S03 remote button | a concrete remote-operation authority names exact remote subjects; no prose or generic Relation triggers itself |
| S04 bomb in a house | one bounded capability may atomically change named subjects and publish one structurally scoped occurrence; it never discovers blast meaning |
| S05 music bomb | World filters private and structural scope; eligible Agents interpret hearing, while later history never proves personal perception |
| S06 explosion and table changes | exact causal dependencies reject only operations whose defined result became stale; stable subject order prevents deadlock |
| S07 “same” Entity | request or explicit shared materialization identity converges intended retries; names and descriptions never deduplicate World subjects |
| S08 linked Entity graph | only a concrete bounded creation capability earns package-local references and all-or-nothing graph materialization |
| S09 absent, stale or moved | neutral precise typed outcomes let the Agent refetch and deliberately replan; no implicit creation or name matching |
| S10 everything blue | one readable phenomenon may supply context; literal Entity rewrites remain proportional; no global revision or universal effect engine |
| S11 one hot Place | coordination follows exact facts and structural writes; subject admission protects quiet work and finite connection pools |
| S12 reconnect | baseline first, then bounded authorized history and new attention; no per-recipient replay or fabricated witnessing |
| S13 communal result | one explicit operation defines eligibility, bounded contributions and deterministic settlement into ordinary current state and Activity |
| S14 causal loop | no generic automatic traversal; one bounded operation rejects invalid structure or later explicit Actions remain inert until invoked |

## Alternatives rejected by the backcast

- **One generic MCP `submit_change` union.** It moves domain meaning into a large
  schema every Agent must decode and makes authority and errors harder to explain.
- **One universal change-package domain model.** A shared internal normalized form
  may be useful, but it may not become the place where every future game mechanic
  stores its meaning or invents generic consequences.
- **Place-wide or World-wide freshness.** It serializes unrelated play and turns a
  crowd into a correctness bottleneck.
- **Automatic proposal rounds on contention.** Popularity and traffic do not grant
  communal authority; slow or offline Agents would freeze ordinary play.
- **Last-write-wins, CRDT or prose merge for contradictory state.** These mechanisms
  cannot choose game meaning and obscure accountable conflict.
- **A generic causal, trigger or rule graph.** Arbitrary Agent-authored Relations
  cannot safely execute themselves, and recursive background consequences would
  spend authority the initiating User did not grant.
- **Durable recipient delivery and lossless ambient replay.** It multiplies storage
  and context by audience while still failing to prove perception.
- **Polling as the complete active experience.** Explicit reads are the recovery
  floor; accepted live hosts also need a bounded notice path during active play.
- **Notifications as Agent activation.** Delivery does not authorize token spend,
  judgment or a World mutation.
- **A live global World board.** It turns every change into a global wake multiplier.
  Bounded deliberate discovery may exist as a pull view without a global revision.
- **Speculative brokers, sharding and distributed consensus.** They do not solve one
  hot canonical fact and should appear only after measured PostgreSQL and gateway
  limits name the missing property.

## Falsifiers

The recommendation must change if one of these is demonstrated:

1. A representative group of Agents cannot reliably understand the common loop
   across concrete capabilities without one public universal package.
2. Concrete operation contracts necessarily duplicate enough state or transaction
   code to create divergent correctness despite a shared internal settlement seam.
3. The game requires arbitrary cross-capability composition inside one atomic User
   confirmation more often than bounded concrete capabilities can express.
4. A compelling core scene requires automatic multi-Agent deliberation on every
   contested change rather than explicit operation-owned cooperation.
5. Baseline, small attention set, coalescible hints and bounded history cannot make
   active or returning play feel present without durable per-Character ambient
   receipts.
6. Exact logical dependencies cannot be represented with bounded storage and
   transactions without reintroducing a broader conflict unit.
7. One accepted spatial or remote mechanic requires World semantic inference rather
   than Agent-authored meaning plus deterministic structural validation.
8. A real host matrix cannot maintain live attention inside explicit User-owned play
   without background Agent invocation; the product promise would then need an
   honest host or interaction redesign.

## Backward route from year five

### Years four and five

- Subject routing, history partitioning, replicas and derived interest/search views
  exist only where measured load earned them.
- Several concrete cooperative mechanics exist, each with distinct authority and
  settlement, while the ordinary operation loop stays unchanged.
- Hot-subject SLOs, recovery objectives and privacy probes are production evidence,
  not architectural claims.
- New Agent hosts implement the same read, confirm, settle, notice and refetch
  semantics without model allowlists.

### Years two and three

- Exact-fact conflict semantics have replaced Place-wide freshness for the concrete
  capabilities that need concurrency.
- Automatic Agent non-interference and bounded just-in-time reread have removed
  harmless collision from ordinary GX without exposing concurrency policy or
  permitting background play.
- One bounded multi-subject capability and one explicit cooperative capability have
  proved atomicity, overload and Agent comprehension.
- Resource attention and reconnect have passed real PostgreSQL, gateway, MCP-host,
  loss, privacy and slow-consumer tests.
- Operational cells or partitions are introduced only after one-writer measurements
  and a preservation suite prove they do not change World identity or behavior.

### Year one

- The current concrete Action, Interaction and Discovery contracts are audited
  against the common settlement questions without replacing them with a generic
  tool.
- One concrete capability proves that its Agent can preserve compatible concurrent
  state automatically and asks again only when the recognizable in-World result
  changes inside the same explicit invocation.
- One existing capability narrows its Place-wide dependency to the smallest exact
  accepted facts and proves one hot subject beside quiet controls in PostgreSQL.
- One read resource gains a bounded live attention experiment plus authoritative
  refetch and reconnect; the World remains correct when every hint is lost.
- The first cooperative gameplay scene is selected before a proposal or voting
  schema is designed.

### Now

Do not build an actor runtime, action-frame queue, transaction mechanism or
notification path yet. First decide the Agent's ordinary non-interference rule. The
player should name and confirm only the in-World result. An optional MCP listener
may make the Agent's context dirty sooner, but the mandatory behavior is a bounded
reread before writing and automatic preservation of every fact the action does not
explicitly affect.

## Active grill

### Q1 challenged — correct serialization is not sufficient GX

The User did not accept first-valid-commit plus stale replan as the ordinary
foundation. At thousands of simultaneous submissions it would be technically
correct but could produce an unpleasant stream of errors and replanning. Research
across EVE, Guild Wars 2, Nakama, Orleans, Durable Objects, Unreal and Roblox
separates content copies, scene owners, virtual authorities, action frames, interest
management and prediction. None removes the one serial decision required by a truly
contradictory canonical fact.

The corrected recommendation keeps one canonical World and rejects canonical
Entity copies. It adds two layers before true conflict: exact structural
decomposition so compatible facts compose, and bounded Agent rebase inside the same
explicit User turn. A later measured hotspot may use a lazy virtual execution lane
and short action frame, but that is replaceable operational machinery and cannot
choose the game result of a contradiction.

### Q1a corrected — the player does not manage concurrent facts

The User challenged the question itself: preservation conditions and concurrency
policy should not be work for the player. That correction is accepted as the active
GX requirement. The exact automatic Agent rule remains unaccepted.

MCP listeners fit only as an optional freshness accelerator. An active host can
subscribe to one exact authorized current-context resource, coalesce update hints
into `dirty` and refetch on the next explicit turn. Core MCP sends a changed resource
URI, not new World truth; the host may disconnect, miss the hint or never invoke a
model. Correctness therefore remains final reread, exact preconditions and atomic
World settlement.

Publishing “another Agent is preparing an action” is different. It requires explicit
intent state and can support a later cooperative or contest capability, but adding it
to every ordinary action would create noise, latency and griefing leverage.

### Q1b — automatic non-interference

**Concrete scene.** Noor says only: “Paint this Table blue.” While Noor considers the
Agent's natural preview, Mara carves a map into a separately represented detail. An
MCP listener may mark the Table context dirty; without listener the Agent discovers
the same change during its mandatory final reread.

**Recommended answer.** The Agent automatically changes only the exact facts required
by Noor's intended in-World result and preserves every other current fact. It keeps
Mara's carving and submits blue without mentioning concurrency. It returns to Noor
in ordinary game language only if the current World makes the recognizable result
materially different—for example, Mara already painted the same surface red or the
surface no longer exists.

**Why.** This uses the distinctive strength of AI Agents to remove harmless
concurrency noise from GX while keeping World dumb and strict. The Agent spends
tokens only inside Noor's explicit turn; World receives and validates one exact
bounded operation. It does not solve a true blue-versus-red conflict—that later
requires a concrete game rule—but it prevents database representation from creating
false conflict.

**Current-contract consequence.** Current `game/docs/agent.md` requires fresh
preview and confirmation for every post-confirmation edit. Accepting this answer
would deliberately evolve that private Agent-conduct contract later; World would
still receive only the final exact package and could not infer or attest what the
User confirmed.

**Open User decision.** Should automatic non-interference be the ordinary Agent rule:
preserve every current fact outside the action's explicit affected facts, and involve
the player again only when the intended in-World result materially changes?

## Unchanged boundaries

This backcast changes no Multiplayer `Chosen` choice, `game/docs`, backlog order,
schema, API, code, MCP public text or delivery claim. The User-defined development
term GX is recorded in `dev/CONTEXT.md`. It deliberately
reopens the universal unified-change-package assumption while retaining the proven
requirements for exact conflict, atomic Activity, Agent-owned intelligence,
authorized attention, bounded recovery and no unconscious token spend.
