---
status: pending
era: August Activity-Property-Trait
---

# Multiplayer concurrency and World observation

> **Role / side:** sourced research report / development side.
> **Authority:** records this report's question, evidence, inferences and candidate implications.
> **Excludes:** product decisions and current implementation contracts; see `game/docs/`.

Date: 2026-08-16

Status: research; no architecture or game behavior below is accepted Aicadia behavior

## Question and evidence boundary

Which proven multiplayer and shared-world techniques can support one persistent
World in which millions of Characters may act, discover and gather, including an
extreme hotspot where very many Characters occupy one semantic Place and need to
learn about a shared occurrence such as a large stone being dropped?

The concrete requirements under examination are:

- one authoritative accepted consequence, not one mutable truth per client;
- concurrent independent work without a Place-wide correctness bottleneck;
- deterministic sight, hearing or other observation eligibility;
- one durable occurrence rather than one stored copy per observer;
- low-latency notification for active clients and bounded recovery after loss;
- no automatic Agent invocation or hidden token spend; and
- failure and overload behavior that preserves World correctness.

This report uses only project-owned contracts, official engine or platform
documentation, first-party engineering accounts and original papers. **Evidence**
means a sourced fact. **Inference** means analysis of that fact. **Candidate
implication** means an unaccepted direction for Aicadia. Vendor-reported tests are
not treated as independent proof, and a system that supports thousands is not
presented as proof of millions.

## Core result

No researched system makes an arbitrarily dense, fully interactive crowd cheap.
They reduce work by separating four responsibilities:

1. one authority accepts and stores gameplay consequences;
2. operational ownership partitions simulation only where causal work can be split;
3. interest management selects and prioritizes an observer's relevant subset; and
4. transient replication is allowed to degrade because durable state can be read
   again.

**Evidence.** Epic's Replication Graph exists because evaluating every replicated
Actor against every connection bottlenecks CPU; it shares persistent replication
lists between connections. Epic's published example is 100 players and roughly
50,000 Actors, not a million-player instance. Iris likewise shares expensive work,
then filters and prioritizes per connection.
[Replication Graph](https://dev.epicgames.com/documentation/en-us/unreal-engine/replication-graph-in-unreal-engine),
[Iris components](https://dev.epicgames.com/documentation/en-us/unreal-engine/components-of-iris-in-unreal-engine),
[Iris filtering](https://dev.epicgames.com/documentation/en-us/unreal-engine/iris-filtering-in-unreal-engine)

**Evidence.** Linden Lab's historic interest-list test explicitly described its
then-current cost as approximately `agents * changes`; many agents in a rapidly
changing simulator were the hostile case. Improbable's first-party ScavLab account
calls all-to-all visibility a degenerate networking case: its vendor-reported 10,000
player test sent over 250 million updates per second, with most distant Characters
reduced to 2 Hz. The live event peak was 4,144 human players. These are useful
measurements, not independent validation of Improbable's broader product claims.
[Second Life interest-list test](https://wiki.secondlife.com/wiki/Interest_List_test),
[archived Improbable density account](https://web.archive.org/web/20210602131214/https://www.improbable.io/blog/intimacy-at-scale-building-an-architecture-for-density/)

**Inference.** Storing one occurrence removes write amplification but cannot remove
the physical cost of delivering information to every active recipient. If one
million clients must each receive a payload, at least one million deliveries exist.
If every participant's rapidly changing state must be sent to every other
participant, the relationship approaches quadratic work. A credible design must
therefore define fidelity, frequency, relevance, batching and allowed delay instead
of promising unrestricted all-to-all replication.

**Candidate implication.** Aicadia's useful advantage over an action MMO is that an
Agent consumes bounded semantic context per explicit turn rather than transforms at
30 or 60 Hz. Store one canonical Activity and current-state consequence. Deliver a
small, lossy "something relevant changed" hint to active connections, then recover
authoritative bounded context on the next explicit read. Never wake one million LLMs.

## Current Aicadia boundary

The present contract already establishes several important foundations:

- `World` is the sole deterministic behavior seam; calls are stateless and no server
  invokes an Agent ([World seam](../../../game/docs/model/world/README.md));
- one accepted mutation writes current state and one immutable Activity atomically;
  current state is not rebuilt by replay ([Activity](../../../game/docs/model/activity/README.md));
- exact-local Activity is stored once and read through personal or Place lenses;
  lens-specific prose is not copied ([Activity](../../../game/docs/model/activity/README.md));
- current Place history includes non-Interaction Activity, but a bystander cannot
  currently see an Interaction merely by being co-present
  ([Place Activity](../../../game/docs/capability/list_activity_at_current_place.md#result));
- every exact-Place writer currently locks one Place row and advances
  `latest_activity_id`, so all writers at one Place serialize
  ([delivery and freshness](../../../game/docs/protocol.md#delivery-identity-and-exact-place-freshness)); and
- movement, additional Places, generic observation, notifications, clocks,
  background simulation and background Agent activation remain absent
  ([deferred scope](../../../game/docs/deferred.md#absent)).

**Inference.** Activity already has the correct storage multiplicity for the stone:
one accepted occurrence. It does not yet define whether a bystander saw or heard it,
and prose or non-executable Traits cannot safely decide that. The Place revision is
a correct MVP freshness mechanism but also the current hotspot serialization lane.

## What the mature systems actually prove

### EVE Online: one universe, isolated hot systems and bounded slowdown

**Evidence.** CCP's 2023 Tranquility account says the historic universe used one
database behind proxy and solar-system simulation nodes. The then-current cluster
had 170 nodes for general solar-system simulation, while the hot Jita solar system
ran alone on one node and its market ran on another. Earlier CCP accounts describe a
node as the lowest simulation granularity and dedicated nodes as hosting one solar
system. [Tranquility Tech IV](https://www.eveonline.com/news/view/tranquility-tech-iv),
[cluster architecture](https://www.eveonline.com/news/view/my-node-was-equipped-with-the-following...)

**Evidence.** CCP isolates anticipated fleet-fight systems on a dedicated node.
Time Dilation slows the game clock so clock-linked work arrives less quickly while
preserving relative gameplay mechanics. CCP also documents its limit: at the 10%
floor, queues can still fill, modules can fail to cycle and a node can die.
[Time Dilation design](https://www.eveonline.com/news/view/introducing-time-dilation-tidi),
[production TiDi evidence](https://www.eveonline.com/news/view/tranquility-tech-iii-is-ready-for-you)

**Inference.** EVE proves that a persistent single universe does not imply one
simulation process. It also exposes the residual hotspot: isolating one semantic
area protects its neighbors but does not split tightly coupled work inside that
area. Stronger hardware and slower time raise the ceiling; neither makes the ceiling
disappear.

**Candidate implication.** Protect quiet Places from a hot Place by subject-scoped
admission, pools and routing. Do not assume "move the Place to a larger worker" is a
complete strategy. Time dilation becomes relevant only if Aicadia later has
clock-driven mechanics. Stateless Agent calls need bounded admission and explicit
`busy` results instead of a hidden slowed clock.

### Second Life: region ownership creates both scale and handoff seams

**Evidence.** Second Life divides its world into 256-metre Regions managed by
separate simulator programs. A viewer can communicate with multiple adjacent
simulators to present a seamless view. Crossing transfers control from a losing to a
gaining simulator; the official wiki notes temporary out-of-sync states, messages
that are not guaranteed to arrive in order and crossings that can fail.
[server architecture](https://wiki.secondlife.com/wiki/Server_architecture),
[region crossing](https://wiki.secondlife.com/wiki/Region_crossing)

**Evidence.** The simulator performs physics, object-state and visibility work and
prioritizes delivery. The same wiki describes time dilation under load and a Region
Conductor that starts Regions found down. The architecture page labels some of its
material obsolete, so it is evidence of the documented design family, not a precise
claim about every 2026 production component.
[server architecture](https://wiki.secondlife.com/wiki/Server_architecture)

**Inference.** A fixed Region owner gives a simple authoritative lane and bounded
working set, but creates migration, boundary and failure protocols. The technical
Region is visible in behavior when handoff breaks. Aicadia must not make a semantic
Place's stable identity depend on an operational owner or cell.

**Candidate implication.** If continuous simulation later earns dedicated workers,
use a separately mapped operational cell with one current owner and explicit
handoff. Persist authoritative state outside the process, reject writes from a stale
owner, and prove boundary cases. Do not introduce a cell server for present
request/response Activity; PostgreSQL transactions already provide the current
ownerless multi-instance authority seam.

### Unreal and Roblox: authority, relevance and presentation are different

**Evidence.** Unreal's default networking model is server-authoritative: the server
holds true game state and clients render approximations. It explicitly separates
essential gameplay state from cosmetic effects and requires developers to choose
what replicates to which connections.
[Unreal networking overview](https://dev.epicgames.com/documentation/en-us/unreal-engine/networking-overview-for-unreal-engine)

**Evidence.** Replication Graph shares precomputed actor lists; Iris keeps a
quantized state copy, shares expensive work and supports owner, connection, group and
dynamic filters. Under bandwidth saturation Unreal prioritizes Actors rather than
guaranteeing every update in the current network frame.
[Replication Graph](https://dev.epicgames.com/documentation/en-us/unreal-engine/replication-graph-in-unreal-engine),
[Iris filtering](https://dev.epicgames.com/documentation/en-us/unreal-engine/iris-filtering-in-unreal-engine),
[Actor priority](https://dev.epicgames.com/documentation/en-us/unreal-engine/actor-priority-in-unreal-engine)

**Evidence.** Roblox likewise calls the server the ultimate state authority.
Instance streaming sends a spatial subset around one or more replication foci; its
documentation warns that every additional focus increases server work and that one
player with nine moving foci can resemble ten players' streaming workload. Roblox
also recommends keeping cosmetic explosion visuals client-side while the server
retains only what it needs to determine the outcome.
[client-server runtime](https://create.roblox.com/docs/projects/client-server),
[instance streaming](https://create.roblox.com/docs/workspace/streaming),
[networking performance](https://create.roblox.com/docs/performance-optimization/improve)

**Inference.** These engines prove three separations:

- authoritative consequence is not a rendered or narrated effect;
- observer relevance is not ownership of truth; and
- sharing selection work between cohorts does not eliminate per-connection
  bandwidth or final filtering.

They do not prove a persistent million-player World: Epic's example is a
100-player match, and Roblox normally distributes an experience across server
instances and Places.

**Candidate implication.** Aicadia can compute a shared candidate set once per
Place/sensory scope and apply only cheap bounded observer filtering afterwards.
Client- or Agent-facing prose renders a canonical typed consequence; it never
decides whether that consequence happened.

### Improbable/SpatialOS: fine ownership can scale only separable work

**Evidence.** Improbable's first-party density report describes multithreaded Actor
work, lightweight server representations, dynamically added simulation processes,
hot backups, synthetic load and fidelity tiers. Its public event result was over
4,000 simultaneous human players; its 10,000 figure was a vendor test. The report
also says high-density scale requires changes across networking, rendering,
simulation and orchestration rather than one component.
[archived first-party density report](https://web.archive.org/web/20210602131214/https://www.improbable.io/blog/intimacy-at-scale-building-an-architecture-for-density/)

**Evidence.** Improbable's public SpatialOS Unity GDK repository is archived and
states that it targets the frozen legacy platform. Its README described multiple
server-side engines in one seamless world, but the repository's status makes it poor
support for a new dependency choice.
[archived official GDK](https://github.com/spatialos/gdk-for-unity)

**Evidence.** The official Unreal GDK's `0.14.0` release notes exposed a configurable
multi-worker load-balancing strategy, Actor authority migration, a sender reference
for ordering reliable cross-server calls, and a write fence for recovery/snapshot
ordering. The same notes include fixes for authority flicker and migration
diagnostics. That repository was archived in 2024.
[official release notes](https://github.com/spatialos/UnrealGDK/releases/tag/0.14.0),
[archived Unreal GDK](https://github.com/spatialos/UnrealGDK)

**Inference.** Dynamic worker count is useful when Actors or systems can be updated
independently. It cannot let two workers decide incompatible outcomes for the same
stone. SpatialOS's own ordering fences and authority-flicker fixes are evidence that
redistribution creates a correctness protocol, not merely a routing optimization.
The authority boundary must follow the smallest state whose invariants need one
decision, not simply split a crowded Place geometrically.

**Candidate implication.** Treat distributed simulation frameworks as pattern
evidence, not an infrastructure recommendation. Aicadia should first prove
subject-scoped PostgreSQL transactions and bounded observation. Only a measured
continuous-simulation need could earn dynamic worker ownership.

## The stone scenario, decomposed

The sentence "Mara lets a large stone fall" hides five different contracts:

| Contract | Concrete question | Candidate authoritative owner |
| --- | --- | --- |
| intent | Did Mara request this exact Action, and may she act here? | User/request id plus Character and submitted subjects |
| consequence | Did the stone move, break, damage or change anything? | exact affected Entity state in one World transaction |
| occurrence | What accepted historical fact remains? | one immutable Activity with actor, Place and involved Entities |
| signal | What visible, audible or tactile phenomenon did the mechanic produce? | typed deterministic output attached to the accepted consequence |
| observation | Which Character may retrieve which part, now or later? | bounded lens over presence, signal scope and access rules |

### Consequence cannot come from prose alone

**Evidence.** Current Aicadia Activity never infers Property or Trait state from
prose, and Traits are non-executable ([Entity](../../../game/docs/model/entity/README.md),
[Activity](../../../game/docs/model/activity/README.md)).

**Inference.** The server cannot decide that a "large" stone makes a loud sound,
that a deaf Character cannot hear it or that a wall occludes it unless those are
typed mechanics. Asking each observer's LLM would produce contradictory authority,
leak hidden information and spend tokens without an explicit user turn.

**Candidate implication.** A future concrete Action mechanic may return zero or a
small bounded set of typed signal descriptors, for example:

```text
signal {
  activity_id,
  channel: visible | audible,
  origin_place_id,
  scope: exact_place,
  mechanic_version
}
```

This is a research shape, not a proposed generic event engine. `scope` must be a
server-owned result of one accepted mechanic, not arbitrary prose or a player-chosen
global audience. Range, portals, sub-Places, occlusion or observer abilities are
added only when concrete behavior needs them. Exact-Place public occurrence is the
smallest testable first scope.

### Store one occurrence, derive an observation lens

One million possible witnesses should not create one million copies of Activity.
Four distinct persistence policies are possible:

| Policy | What survives | Scale property | Game meaning |
| --- | --- | --- | --- |
| Place memory | canonical Activity remains queryable at its Place | one occurrence plus indexed reads | later visitors can learn what happened there |
| presence-time eligibility | Character can later recover events from when it was present | requires durable movement/presence intervals but no receipt per event | Character was fictionally there even while its User was offline |
| active-attention delivery | only currently connected subscribers receive a transient hint | no durable personal witness claim | missed delivery says nothing about World truth |
| personal perception receipt | one row records that one Character perceived one Activity | up to observers × occurrences | durable private memory, but pathological crowd amplification |

**Candidate implication.** Prefer Place memory plus active-attention delivery unless
a concrete mechanic requires personal witness provenance. Presence-time eligibility
is a product decision that must precede its data model. Reject automatic personal
receipts for ambient crowd events.

This also preserves the distinction between:

- "the stone fell here" — one durable shared occurrence;
- "this active client should refresh" — transient delivery state; and
- "this Character personally saw it" — a separate, currently unchosen game fact.

### Some see, some hear

Different outcomes need a staged deterministic lens:

```text
shared candidates = signal scope and authoritative event-time location
allowed           = access, occlusion and channel rules
payload           = bounded ranking and current Agent context budget
```

**Inference.** If all Characters have only one exact semantic Place and no other
mechanical sensory state, World cannot honestly distinguish who sees and who hears.
It can only expose the occurrence as exact-Place knowledge. Individual sensory
differences require at least one of: finer Place membership, portal/containment
state, geometry, typed sensory capability, explicit attention, or event-specific
access. Each adds a different invariant and index.

**Candidate implication.** Do not overload `Place` to mean location, lock, worker,
auditory zone and visual field. Keep one stable semantic Place identity; define a
bounded scope per signal channel when the game earns it.

## Realtime fan-out and durable catch-up

### A live connection is a wake-up path, not authority

**Evidence.** Core NATS is fire-and-forget and an inactive subscriber misses a
message. JetStream instead stores messages, assigns sequence numbers and maintains
stateful consumer cursors with redelivery. This first-party documentation shows that
live pub/sub and durable replay are separate guarantees, not a mode switch with zero
cost. [NATS delivery model](https://docs.nats.io/nats-concepts/what-is-nats),
[JetStream streams and consumers](https://docs.nats.io/concepts/jetstream)

**Inference.** Creating one durable broker consumer per Character merely moves the
million-row problem into the broker. Aicadia already has immutable Activity and
current state, so a broker should not become a second World authority or the only
recovery source.

**Candidate implication.** If realtime delivery is accepted later, use this flow:

```text
explicit Action
    -> World transaction commits current state + one Activity
    -> rebuildable publisher emits {activity_id, scope_key, change_token}
    -> gateways coalesce and fan out a refresh hint to active scope subscribers
    -> client/host performs one bounded authoritative World read
    -> Agent sees the result only inside an explicit User/Agent turn
```

The hint may be duplicated, reordered, delayed or lost. The Activity id makes it
deduplicable; the authoritative read makes the hint disposable. Coalescing
"Place changed" from token A to token B is preferable to forcing every intermediate
Activity through every slow connection.

### MCP can carry a wake-up, not replay

**Evidence.** MCP `2026-07-28` delivers change notifications only on a
`subscriptions/listen` stream the client opts into. The Streamable HTTP specification
does not support resumable SSE through `Last-Event-ID`. The official Python SDK
describes abrupt loss as: listen again and refetch; it explicitly provides no replay.
[pinned MCP transport specification](https://github.com/modelcontextprotocol/modelcontextprotocol/blob/4df2d6b6e3588efb46e7542d98498e5c630a0a86/docs/specification/2026-07-28/basic/transports/streamable-http.mdx),
[pinned Python SDK client](https://github.com/modelcontextprotocol/python-sdk/blob/5285e936a9d52dd4ad4be3f49e3b2481552ad1e6/src/mcp/client/client.py)

**Inference.** The current open standard matches a refresh-hint design: an MCP host
may learn that a subscribed resource changed, but the notification is neither a
durable cursor nor proof that an Agent consumed the new content. A disconnected host
misses the interval and must refetch authoritative state after reopening a stream.

**Candidate implication.** If Aicadia later publishes realtime observation over MCP,
make a coalescible scope resource emit `notifications/resources/updated`; the host
then re-reads bounded World context. This remains opt-in and must not invoke an LLM.
The present Aicadia contract exposes no notification capability, so this is a future
protocol experiment rather than current behavior.

### Catch-up has a non-obvious ordering problem

**Evidence.** Lamport's original result distinguishes causally related events from
concurrent events and shows that a total presentation order is additional structure,
not causality itself. [Lamport, *Time, Clocks, and the Ordering of Events in a
Distributed System*](https://www.microsoft.com/en-us/research/publication/time-clocks-ordering-events-distributed-system/)

**Inference.** A timestamp plus UUID creates a stable sort, but not necessarily a
no-gap forward subscription cursor. Two independent transactions can obtain times or
ids, then commit in the reverse order. A poll that asks only for values after its
last timestamp can miss a late commit that sorts before that cursor. Conversely, a
single per-Place commit sequence gives exact catch-up but recreates a serialization
lane at the hottest Place.

Candidate options requiring proof are:

1. keep a Place serialization pointer and accept its write ceiling;
2. create an append sequence per bounded delivery partition, expose only its opaque
   delivery watermark and accept that lane;
3. use overlapping recent windows plus Activity-id deduplication, with an explicit
   maximum lateness/backlog bound; or
4. treat current state as exact recovery and recent Activity as best-effort context,
   so observing every intermediate occurrence is not a correctness guarantee.

**Candidate implication.** Decide whether complete no-gap observation history is a
game invariant before choosing a cursor. Do not silently call `(occurred_at, id)` a
commit cursor. A delivery watermark is transport/projection state, never a Place or
Entity revision and never World identity. Independent state changes need no global
gameplay order; same-subject preconditions and versions carry the order that matters.

## Authority and dynamic ownership

### Current request/response World

For current Aicadia, the smallest authority owner is the PostgreSQL transaction and
the exact mutable subject rows it validates. Any World instance can serve a request.
Independent subjects can proceed concurrently; incompatible changes to one Entity
must still choose an order or return a typed conflict.

**Candidate implication.** Narrow Place-wide freshness only when one capability has
specified all positive and absence dependencies. Append-only Activity that changes
no shared current fact may not need to serialize with every other local append, but
an Action whose meaning is grounded in "nothing changed anywhere in this Place"
does. Product semantics, not throughput aspiration, decides the dependency set.

### Future continuous simulation

A future clocked physics or NPC loop would introduce another kind of authority. One
active process may own a technical cell or subsystem while many gateway and World
instances route requests to it. Safe reassignment needs:

- a durable mapping from operational cell to current owner;
- an ownership generation or fencing value checked by the state store;
- bounded handoff state and an explicit point at which the old owner stops;
- authoritative checkpoint/recovery independent of process memory; and
- idempotent input identity across retry and owner failure.

**Evidence.** Google's Chubby paper describes a reliable, coarse-grained lock service
used for primary election and explicitly optimizes for reliability rather than high
write throughput. It is evidence for sparse ownership coordination, not for placing
every World mutation behind one global lease.
[Chubby paper](https://research.google/pubs/the-chubby-lock-service-for-loosely-coupled-distributed-systems/)

**Candidate implication.** Never equate this owner generation with Entity, Place or
Activity identity. Do not build it before a concrete current mechanic needs a
continuously active owner.

## Overload and graceful degradation

The consistency path and experience path should fail differently:

| Layer under pressure | Preserve | Allowed degradation |
| --- | --- | --- |
| canonical mutation | idempotency, atomic state and history, scoped conflict | bounded `busy`/conflict before work; no silent acceptance |
| authoritative read | privacy and a consistent bounded page | bounded stale class only when declared; retry/backoff |
| live hints | eventual refresh opportunity | coalesce, lower frequency, disconnect slow consumers, lose hints |
| Agent context | explicit user turn and grounded facts | fewer/ranked occurrences; explicit pagination |
| cosmetic rendering | canonical consequence | local low-fidelity sight/sound presentation |

**Candidate implication.** Apply admission before a request occupies a database
connection or hot lock. Bound queue depth, pool wait, lock wait, request work and
retry budget per User and contested subject. Isolate hot-subject capacity so it
cannot consume every connection needed by quiet Places.

Time dilation is a valid future mechanic only when:

- most overload is driven by game-time work;
- all affected timers are classified as dilated or wall-clock;
- the slowdown is visible and semantically fair; and
- a hard lower bound still produces a defined busy/failure result.

It is not a substitute for bounded database work, delivery backpressure or subject
admission.

## Failure recovery matrix

| Failure | Required durable truth | Candidate recovery | Must never happen |
| --- | --- | --- | --- |
| World instance dies before commit | none | caller retries same request id | partial consequence or Activity |
| World instance dies after commit before reply | accepted result | idempotent retry returns stored canonical result | duplicate Activity |
| publisher dies after commit | Activity/current state | CDC or bounded rescan republishes idempotently | mutation rollback because hint failed |
| gateway/broker loses hint | Activity/current state | next authoritative read catches up | hint becomes World truth |
| client disconnects | policy-dependent | Place read, presence-time lens or no recovery | automatic Agent run |
| future simulation owner dies | checkpoint plus ownership generation | fenced replacement owner resumes | old and new owners both write |
| hot Place exceeds budget | existing committed state | early busy, coalesced delivery, protected quiet capacity | unbounded queue or skipped validation |

## Candidate smallest multiplayer slice

The evidence supports one small, high-leverage experiment before any distributed
simulation platform:

> Two or more Characters at one exact Place can recover one accepted public local
> occurrence through a bounded authoritative read, while the occurrence is stored
> once, no Agent is activated, retries do not duplicate it and unrelated state
> changes do not require a global World order.

This slice would deliberately not yet promise literal hearing, line of sight,
offline witness memory, background notifications, geometry, a clock or one million
live sockets. It would establish the semantic seam on which those can be tested.

A later realtime experiment can add disposable refresh hints without changing the
World truth. A later sensory experiment can add exactly one typed channel and scope.
Each is a separate product choice and evidence claim.

## Required experiments before a scale claim

### Correctness matrix

Test at minimum:

- one request retried before, during and after uncertain delivery;
- independent Actions at the same Place;
- incompatible changes to the same stone state;
- an Action whose absence precondition races an introduction;
- one occurrence visible through multiple authorized lenses but stored once;
- duplicate, reordered and missing refresh hints;
- disconnect, catch-up and re-entry under each chosen witness policy; and
- technical owner or gateway replacement without semantic Place identity change.

### Adversarial load matrix

Fix and report all axes rather than one "million users" number:

- total accounts, placed Characters and simultaneously connected clients;
- active subscribers per scope and subscriptions per client;
- accepted and rejected intents per second;
- independent-subject versus same-Entity versus same-Place skew;
- Activity history and Place occupancy cardinality;
- payload bytes, changes per occurrence and observer fidelity tiers;
- burst length, p50/p95/p99 latency and allowed busy rate;
- database pool/lock wait, rows and buffers, WAL and vacuum pressure;
- gateway CPU/memory, outbound bytes, slow consumers and reconnect storm; and
- RPO/RTO for World truth, projection and live delivery separately.

Run a quiet-Place control during every hotspot test. A hot Place passes containment
only when it cannot exhaust quiet-Place mutation and read capacity.

### Observation-specific checks

For the stone scenario, verify exact counts:

- one Activity row and bounded signal rows independent of observer count;
- zero personal receipt rows unless that policy was explicitly chosen;
- one shared candidate computation per cohort where possible;
- final per-connection work and bandwidth measured, not hidden;
- no LLM or Agent process started by acceptance or delivery; and
- current state remains correct after every dropped or duplicated hint.

## Decision frontier

Research cannot decide these game-design questions:

1. Is a Character fictionally present and capable of later recollection whenever it
   remains placed, even while its User has no active connection?
2. May a Character entering a Place later learn its public history, or only facts it
   personally observed while present?
3. Is the first local occurrence merely "knowable at this Place", or must the first
   slice already distinguish sight from hearing?
4. Must an active observer receive every occurrence without a gap, or is exact
   current state plus bounded recent causal context sufficient?
5. Which independent local Actions may proceed despite another accepted Activity,
   and which explicitly depend on the whole Place representation being unchanged?
6. At overload, may the World reject a valid new intent as busy, and what fairness
   boundary applies: User, Character, Entity, Place or connection?

These choices determine whether Aicadia needs presence intervals, sensory mechanics,
an exact delivery sequence, personal observation state or a retained Place revision
lane. Infrastructure cannot answer them after the fact.

## No-go conclusions

The evidence rejects these as general answers:

- one Activity, notification or durable broker consumer per possible observer;
- automatic Agent calls when another Character acts;
- prose- or LLM-decided authoritative visibility, hearing or physics;
- one semantic Place equated with one permanent process, shard or fixed cell;
- a global World tick, revision, sequence or broadcast feed;
- claiming a Replication Graph, streaming engine or broker makes all-to-all density
  sublinear;
- treating a lossy realtime hint as the only record of an occurrence;
- treating timestamp order as commit order or total order as causality;
- a distributed simulation platform before a clocked mechanic proves its need; and
- any million-user claim without hotspot skew, slow consumers, reconnect storms,
  overload admission and recovery evidence.

## Primary source audit

Checked on 2026-08-16:

- Aicadia current domain, protocol, Place Activity and deferred-scope contracts.
- CCP/EVE first-party Tranquility architecture accounts from 2008 and 2023, Time
  Dilation design and production performance account. Architecture age is stated;
  no old capacity number is projected to current Aicadia scale.
- Epic official Unreal documentation for server authority, Replication Graph, Iris
  shared work/filtering and replication priority. Epic's published example is kept
  at its documented 100-player/50,000-Actor scope.
- Roblox official Creator documentation for server authority, instance streaming,
  replication foci and client-side cosmetic effects. Roblox is treated as an
  instanced platform, not proof of one persistent million-player space.
- Linden Lab's official Second Life wiki for Region ownership, crossing and the
  historic interest-list test. Pages' obsolete/historic standing is called out.
- Improbable's first-party ScavLab engineering account and official archived
  SpatialOS Unity and Unreal GDKs plus `0.14.0` release notes. Vendor tests and
  marketing claims are labelled accordingly.
- NATS official documentation for transient pub/sub versus stored streams and
  stateful durable consumers. It supplies a delivery pattern, not an Aicadia
  dependency recommendation.
- The MCP `2026-07-28` transport specification and official Python SDK at the pinned
  commits for opt-in listen streams, absent SSE resumption and re-listen/refetch.
- Leslie Lamport's original 1978 ordering paper via Microsoft Research and Google's
  original Chubby paper via Google Research.
