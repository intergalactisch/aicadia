---
status: pending
era: August Activity-Property-Trait
---

# Multiplayer resolution from first principles

> **Role / side:** sourced research report / development side.
> **Authority:** records this report's question, evidence, inferences and candidate implications.
> **Excludes:** product decisions and current implementation contracts; see `game/docs/`.

Date: 2026-08-19

Status: research; the proposed resolution model, terms and mechanisms below are not
accepted Aicadia behavior

## Question and reset boundary

If Aicadia were designed today from scratch as a low-latency persistent MMO in which
every action is multiplayer and Users act through AI Agents, what single abstract
game model would let millions of Characters discover, create, manipulate and
assemble one shared World concurrently, fairly and enjoyably?

This research deliberately does not begin with the prior stale-retry, subscription,
virtual-actor, universal-package or PostgreSQL conclusions. It re-derives the
problem from game methods. Existing Aicadia constraints still bound a valid answer:
one canonical World, deterministic World authority, Agent-owned intelligence, no
background token spend, bounded work, attributable Activity and no global hot row,
lock, revision, counter or tick.

Evidence comes from official game, engine and runtime documentation, an original
game rulebook and original distributed-data research. Each source proves only its
own method. **Inference** and **candidate implication** remain unaccepted Aicadia
analysis.

## First-principles requirements

The system must make these simultaneously true:

1. A player expresses an in-World goal, never versions, locks, merge rules or retry
   policy.
2. Two geographically distant actions do not coordinate merely because they share
   one World.
3. Two causally overlapping actions do not gain their result solely from packet
   arrival order.
4. Independent, commutative, exclusive and deliberately cooperative actions do not
   pretend to have the same game meaning.
5. A popular subject can slow or abstract its own interaction without slowing quiet
   World work.
6. An Agent can invent meaning and exact intended consequences while World executes
   only deterministic, bounded installed mechanics.
7. Every accepted consequence has one current result and attributable history;
   observation, prediction and delivery never become authority.
8. The same conceptual contract works for one, two, thousands or millions of
   eligible intents, even though fidelity and admission must remain physically
   bounded.

## What established methods contribute

### Authoritative ticks: resolve input, then publish state

**Evidence.** Valve's Source server processes incoming commands, simulation and game
rules in discrete ticks, then publishes selected snapshots. Clients use prediction,
interpolation and server-side lag compensation to reduce perceived latency and
network advantage.
[Source multiplayer networking](https://developer.valvesoftware.com/wiki/Source_Multiplayer_Networking)

**Evidence.** Photon Quantum and GGPO separate player input from deterministic game
state. They predict unverified frames for responsiveness, then roll back and
re-simulate when verified input differs. GGPO requires deterministic, serializable
state and fixed simulation quanta.
[Photon Quantum frames](https://doc.photonengine.com/quantum/current/manual/frames),
[GGPO developer guide](https://github.com/pond3r/ggpo/blob/master/doc/DeveloperGuide.md)

**Inference.** Low latency does not require the first arrival to become truth.
Players can see an immediate attempted or predicted presentation while one authority
resolves a bounded set of inputs. Rolling back an entire persistent World is a poor
fit, but the input/state separation and verified-result boundary transfer.

**Candidate implication.** Aicadia Agents submit bounded game intents rather than
authoritative state. World resolves them into one verified result; the Agent may
present anticipation but never claim it as accepted history.

### Simultaneous turns: fairness comes from a resolution law

**Evidence.** Diplomacy separates negotiation, private order writing, simultaneous
reveal and neutral order resolution. Legal orders can support, conflict, succeed or
fail according to one complete rule set; arrival order is not the mechanic.
[Diplomacy rulebook](https://media.wizards.com/2015/downloads/ah/diplomacy_rules.pdf)

**Inference.** Simultaneous multiplayer becomes legible when all inputs in one
moment are adjudicated together by a known rule. Aicadia cannot wait for every
offline Character or use world-sized turns, but a very short subject-local
resolution window can import the fairness property.

**Candidate implication.** Requests admitted to one local resolution frame should
be treated as a set, not secretly ordered by network arrival. The concrete mechanic
defines how that set becomes a result.

### Region simulation: parallelism follows independence, not fixed geography

**Evidence.** PaperMC Folia ticks independent World regions in parallel. Adjacent
regions cannot tick independently and must eventually merge; regions split again
when independent areas emerge. Each ticking region owns its local data while it
ticks.
[Folia overview](https://docs.papermc.io/folia/reference/overview/),
[Folia region logic](https://docs.papermc.io/folia/reference/region-logic/)

**Inference.** The transferable idea is dynamic causal grouping: independent work
runs in parallel, while interacting work temporarily shares one owner. Fixed map
regions are insufficient for Aicadia because a bounded remote operation may connect
distant subjects and two facts on one Entity may remain independent.

**Candidate implication.** Form ephemeral execution groups from overlapping exact
read/write footprints rather than assigning every Entity permanently to its own
instance or making Place the universal shard.

### CRDTs: automatic composition requires a type-specific law

**Evidence.** CRDTs permit uncoordinated replica updates and deterministic
convergence only because each data type defines mathematically valid concurrent
semantics. Operations in a commutative replicated data type commute when concurrent.
[CRDT survey](https://arxiv.org/abs/1805.06358),
[CRDTs without concurrency control](https://arxiv.org/abs/0907.0929)

**Inference.** “Merge automatically” is valid only when the game mechanic has an
actual composition law. Adding two distinct attachments may commute; setting one
exclusive current Position to two points does not. CRDT is evidence for
mechanic-specific algebra, not a universal World store.

**Candidate implication.** Let a concrete mechanic declare and execute a safe
commutative combination where that is its intended game meaning. Never infer
commutativity from prose or apply last-write-wins as a substitute.

### Extensible Worlds: state and executable mechanics are separate

**Evidence.** MUD's autonomous-world architecture separates table/component state
from registered systems that read and write it through one World entry point, with
write access scoped to resources. It is inspired by Entity Component System design.
[MUD introduction](https://v1.mud.dev/guides/introduction/)

**Evidence.** Second Life lets creators attach event-driven LSL scripts to objects,
but server execution has bounded queues and throttles. Its documentation warns that
large message fan-out can fill queues, drop events and cause lag.
[Getting started with LSL](https://wiki.secondlife.com/wiki/Getting_started_with_LSL),
[linked-message limits](https://wiki.secondlife.com/wiki/LlMessageLinked)

**Inference.** Agent-invented World behavior needs a separation between authored
meaning, persistent state and an installed deterministic execution rule. Arbitrary
always-running per-Entity programs create idle overhead, event storms and an
unbounded security surface.

**Candidate implication.** Initially, Agents use concrete installed mechanics. A
future Agent-authored mechanic should be a bounded declaration assembled from a
small audited execution algebra, installed only through an explicit accepted game
path. It should not be arbitrary server code or one resident script per Entity.

### Databases: serializability is a safety floor, not game resolution

**Evidence.** PostgreSQL Serializable permits commits only when their combined
effect is equivalent to some serial order, but applications must handle
serialization failures and retry. The database does not select a fun or fair order.
[PostgreSQL transaction isolation](https://www.postgresql.org/docs/current/transaction-iso.html)

**Inference.** A transaction can prove that accepted storage is coherent after a
game decision. It cannot decide which simultaneous actions should compose, compete
or cooperate. Exposing its aborts as ordinary play delegates game design to the
database scheduler.

## Core synthesis: every mechanic resolves a set of intents

The conventional request model is:

```text
apply(current state, one request) -> next state or error
```

The first-principles candidate for Aicadia is:

```text
resolve(current state, admitted concurrent intents) ->
    next state + attributable Activities + one result per intent
```

One Character acting alone is the one-intent case. Multiplayer is therefore not a
mode, adapter or subsystem; it is the function signature of every state-changing
mechanic.

The resolver receives typed, bounded inputs only. The Agent supplies meaning and an
exact proposed effect; World authenticates the actor, derives or verifies the exact
mechanical footprint and invokes the installed deterministic resolver. No generic
resolver interprets prose.

### The mechanic owns its concurrency law

Every state-changing mechanic must define before it can exist:

- eligible actor and structural authority;
- bounded input and exact current facts it may read;
- bounded facts it may affect and invariants across them;
- which inputs can compose mechanically;
- how overlapping inputs are resolved without packet-order authority;
- whether several inputs deliberately assemble one result;
- one result shape for every admitted input;
- current-state and Activity output; and
- overload behavior for one deliberately hot subject.

These are not necessarily public fields or a closed enum. They are the proof
obligations of a mechanic. Examples of mechanical laws include:

- **composition:** all disjoint or truly commutative effects survive;
- **exclusive resolution:** one mutually exclusive result is selected by the
  mechanic's deterministic or seeded fair rule, not by first packet;
- **joint resolution:** bounded eligible contributions intentionally produce one
  combined result; and
- **ordered transformation:** the mechanic defines an order independent of network
  arrival and revalidates each surviving transformation.

A mechanic may use more than one law across its bounded facts. The Agent cannot
choose a more favorable law per request, and listener count, request volume, model
provider or token spend never grants weight.

### Ephemeral causal execution groups

The working architectural description is an **ephemeral causal group**, not an
accepted domain term:

1. An intent names or deterministically yields a hard-bounded set of exact facts it
   may read and affect.
2. Intents whose footprints are disjoint execute independently on any World
   instance.
3. Intents admitted to the same short local frame and whose footprints overlap are
   resolved together.
4. A bounded multi-subject intent temporarily joins those subjects under one
   coordinator and one database transaction.
5. After resolution, the execution group disappears. No Entity, Place, shard or
   process identity changes.

This dynamically merges causally connected work and splits independent work, like
regionized simulation without assuming geography is the only cause. It can be
implemented first through PostgreSQL fact coordination and later through a lazy
virtual executor for measured hot groups. That implementation choice does not alter
the game contract.

### Subject-local resolution frames

A short frame gives the resolver a set rather than a packet-order list. A candidate
window might be tens or low hundreds of milliseconds—small beside an Agent turn,
but long enough to absorb ordinary network skew. The exact duration is a later
measurement, not a present choice.

There is no global tick or World frame. Each independent causal group advances
locally. A quiet group may resolve a one-intent frame immediately after the minimum
fairness window. A hot group micro-batches more inputs and may locally dilate its
frame within a hard bound. Quiet work is unaffected.

An in-memory frame is not accepted truth. A crash before commit permits idempotent
retry; current state and Activities become authoritative only in the one durable
commit. Frame assignment and retries need explicit fairness evidence before build.

## Fairness and fun

The framework can guarantee only mechanical fairness; each mechanic supplies its
game meaning. Its universal guardrails should be:

- packet arrival order inside a frame never decides a contested outcome;
- at most one eligible intent per Character for the same mechanic and conflict
  subject enters one frame, so request spam does not buy more chances;
- equal retries collapse by idempotent identity;
- a seeded selection, when a mechanic genuinely uses chance, depends on accepted
  subject, frame and eligible Character identities rather than client-supplied
  randomness;
- no player waits for every observer, offline Agent or unbounded queue;
- no rejected transport attempt becomes World history, while every accepted game
  consequence remains attributable; and
- the Agent receives an in-World outcome it can narrate, never a lock, revision or
  serialization exception as the final GX.

This does not mean every conflict becomes a lottery. Painting, movement, discovery,
construction and collective naming require different laws. The framework makes that
difference mandatory and executable rather than leaving it to incidental database
order.

## Low latency and MCP

The Agent sends an intent through one concrete MCP capability. The response may
remain open for the short local resolution frame and returns the verified outcome.
An Agent can present “you begin…” optimistically, but only the response establishes
what happened.

After commit, an optional exact-resource subscription marks relevant active context
dirty. It never participates in resolution, invokes an Agent or carries the new
truth. A refetch obtains the resolved state and bounded Activities. Uncommitted
intent is visible only when a concrete cooperative or contest mechanic explicitly
makes it durable readable state.

The public Agent surface remains concrete. Agents do not submit a generic resolver
program. Every tool description explains what simultaneous compatible, conflicting
and joint intents mean for that mechanic, so another Agent can predict the possible
results without learning infrastructure.

## Million-User and hot-subject pressure

Millions of Characters distributed across exact facts produce many independent
causal groups and scale horizontally. One famous subject with a million active
intents cannot be made free:

- gateways authenticate, deduplicate and enforce one eligible intent per Character
  before the hot executor;
- the resolver may reduce many commutative inputs in one batch when the mechanic's
  algebra permits it;
- an exclusive mechanic produces bounded state work even when many intents compete,
  but considering or attributing every admitted participant still has real cost;
- admission, frame size and output remain bounded, with a mechanic-specific crowd
  outcome rather than an unbounded queue;
- local frame dilation or reduced presentation fidelity protects correctness and
  fairness while quiet groups retain normal latency; and
- notification fan-out remains proportional to active recipients and may coalesce
  repeated dirtiness.

If a mechanic promises that every one of a million simultaneous individual inputs
is accepted, stored and delivered, then the cost is necessarily at least linear.
The design must instead decide which inputs are admitted, which jointly form one
game action and which players remain observers. Infrastructure cannot hide that
product decision.

## Representative game consequences

| Scene | Intent set | Candidate resolution |
| --- | --- | --- |
| Two explorers independently discover different things | disjoint discovery attempts | both resolve independently and leave separate Activities |
| Noor paints while Mara carves one Table | effects on distinct exact facts | one frame composes both; neither sees a conflict workflow |
| Noor and Mara both seize one movable object | mutually exclusive control inputs | the installed interaction mechanic resolves the set by its fair game rule; packet order is irrelevant |
| Many Characters raise one bridge | explicit joint contributions | one construction mechanic admits a bounded participant set and resolves one combined consequence with exact roles |
| A remote switch affects several subjects | one bounded multi-subject intent plus overlapping local intents | one temporary causal group resolves all overlapping footprints atomically or according to the installed mechanic |
| Thousands gather but only watch | no modifying intent from most Characters | interest delivery scales separately; spectators neither lock nor enter the resolver |

## Alternatives rejected by the synthesis

- **Immediate first-arrival commit everywhere:** low mechanism latency, but network
  advantage and database errors become game rules.
- **One global World tick or queue:** gives one order but creates a forbidden hot
  coordinate and couples unrelated play.
- **Permanent Entity micro-instances:** isolate many Entities but serialize unrelated
  facts on one popular Entity and complicate bounded cross-Entity actions.
- **Fixed spatial shards:** scale distance but fail remote causal actions, moving
  boundaries and one dense hotspot; content copies break canonical truth.
- **Universal CRDT state:** excellent only where a type-specific commutative game law
  exists; invalid for exclusive or causal outcomes.
- **Universal proposal or consensus phase:** lets slow/offline Agents and token spend
  obstruct ordinary play; retain only as an explicit joint mechanic.
- **Arbitrary Agent-authored server scripts:** powerful but unbounded in cost,
  authority and interaction complexity; installed mechanics need a small safe
  execution algebra.

## Falsifiers and open decisions

The candidate fails if:

1. a representative concrete mechanic cannot define its simultaneous-input result
   without a combinatorial interaction table;
2. overlapping multi-subject frames require a global sequencer for correctness;
3. the minimum fair frame adds player-visible delay with no compensating GX;
4. the exact footprint cannot be derived or supplied without trusting Agent prose;
5. a hot exclusive subject cannot protect quiet latency with bounded admission;
6. Agent-authored future mechanics require arbitrary code rather than a small safe
   deterministic algebra; or
7. current MCP hosts cannot keep one tool call open for the required bounded frame.

The first product decision is whether every modifying mechanic should semantically
resolve a short subject-local set of admitted intents, even when that set contains
only one intent. Exact frame duration, resolver laws, state representation, actor
runtime and Agent-authored mechanic installation depend on that answer.
