# Entity and Place interest subscriptions at scale

> **Role / side:** sourced research report / development side.
> **Authority:** records this report's question, evidence, inferences and candidate implications.
> **Excludes:** product decisions and current implementation contracts; see `docs/game/`.

Date: 2026-08-16

Status: complete research; no subscription, World board, resource URI, gateway or
runtime design below is accepted Aicadia behavior

## Question and evidence boundary

How can millions of active Agent hosts express interest in exact Entities, their
Character's current Place and Agent-named affected Places, receive timely change
hints and recover current truth without:

- broadcasting every World change to every host;
- storing one durable receipt, cursor or copy per observer;
- asking World to decide what is narratively relevant;
- making a notification authoritative; or
- introducing several overlapping realtime systems before one is needed?

This report compares five candidate interest surfaces: one worldwide game board,
regional or Place boards, exact Entity/Place resources, one private Character
attention resource, and a hybrid discovery-to-exact-watch model. It uses current MCP
`2026-07-28`, first-party multiplayer-engine and pub/sub documentation, original
interest-management research, and a read-only audit of the current Aicadia checkout.

**Evidence** is directly supported by a cited primary or authoritative source.
**Inference** is analysis of that evidence. **Candidate implication** is an
unaccepted Aicadia direction. Vendor capacity claims are treated as patterns to test,
not proof of Aicadia capacity.

The report does not select SSE, WebSocket, PostgreSQL `NOTIFY` or a broker for the
transport path; those are separate transport and database-to-process questions. No
live host or million-connection system was tested.

## Result in one sentence

**Inference.** The smallest broadly useful interest system is not an event stream or
one personalized World bundle, but one generic operation over exact authorized World
resources: an active host listens to a bounded Agent-selected set of common Place and
Entity resource URIs, treats every message as a coalescible stale hint, and performs a
bounded authoritative read when explicitly needed.

**Candidate implication.** Start conceptually with:

```text
listen([current Place, selected Entity, ...]) -> "one of these resources is stale"
read(exact resource)                        -> current authorized World representation
```

The same primitive supports coarse local discovery and exact focused attention. A
worldwide board remains a serious discovery candidate, but should initially be a
bounded explicit read rather than a subscription: subscribing every active host to a
resource changed by every accepted World action creates global wake amplification.

This is one resource-interest system, not five transports or five domain models.
Subscription remains optional for correctness; a host that cannot listen can perform
the same reads at the start of an explicit User turn.

## Interest management is a delivery decision, not World meaning

### Proven game systems reduce the candidate set before per-connection work

**Evidence.** Unreal's Replication Graph was designed for large numbers of Actors and
connections. Epic says the ordinary strategy—having each Actor determine whether it
should update each client—becomes a CPU bottleneck. Its graph instead maintains
persistent shared Actor lists and builds replication lists per connection from them.
Epic's documented example is about 100 players and 50,000 replicated Actors, not
millions. The same guide recommends location groups or rooms, separate dormant lists,
and only a small class of always-relevant Actors.
[Epic Replication Graph](https://dev.epicgames.com/documentation/en-us/unreal-engine/replication-graph-in-unreal-engine)

**Evidence.** Unreal distinguishes per-connection relevancy from game authority and
can base it on ownership, attachment, distance or explicit game rules. Godot likewise
allows configured replicated properties to be made visible per peer through direct
visibility flags or filters, with manual or periodic recomputation.
[Epic Actor relevancy](https://dev.epicgames.com/documentation/en-us/unreal-engine/actor-relevancy-in-unreal-engine),
[Godot `MultiplayerSynchronizer`](https://docs.godotengine.org/en/stable/classes/class_multiplayersynchronizer.html)

**Evidence.** Academic Area-of-Interest research defines interest management as
selecting the subset of a shared virtual world whose updates should reach one avatar;
broadcasting all state changes is impractical. Original spatial publish/subscribe
work shows the basic trade-off: large fixed cells over-deliver, small cells cause
subscription churn, while continuously moving precise spatial subscriptions reduce
overlap error but require a spatial matcher.
[Ricci and Carlini, *Area of Interest Management in Massively Multiplayer Online
Games*](https://arpi.unipi.it/handle/11568/1055121),
[Hu, *Spatial Publish Subscribe*](https://mmve-workshop.org/2009/papers/p8.pdf)

**Inference.** The transferable pattern is not Unreal geometry or Godot peer IDs. It
is two-stage work:

1. select a small shared cohort such as one Place or one exact Entity;
2. apply only bounded connection-specific authorization and delivery work inside that
   cohort.

Re-running semantic reasoning for every Entity × Character pair would move Agent
intelligence into World and make the hottest crowd the most expensive case.

### Agent relevance and World eligibility are different predicates

**Candidate implication.** An explicitly invoked Agent decides:

- "this tree matters to my current intention";
- "I want to keep watching it"; and
- "this explosion claims consequences in Places A and B."

World or the authorization boundary decides only deterministic structural facts:

- this User currently controls this Character;
- the Character is currently at Place A;
- the tree is an exact-local Entity the current read contract permits;
- Places A and B are exact existing references admitted by the accepted scope rule;
- this caller may read this resource now; and
- this committed transaction actually changed the named subjects.

World does not infer that trees are visually interesting, explosions are audible or
blue paint is less destructive than felling. A listener proves neither perception nor
knowledge. It only identifies an active transport endpoint that asked to be told when
one authorized representation may be stale.

## Five candidate interest surfaces

### Player and discovery consequence first

| Candidate | Understandable game consequence | Important loss or risk |
|---|---|---|
| One worldwide board | Every active player can discover that one enormous shared World is moving, including distant surprises | Local exploration can be drowned by global noise; distant facts may become unjustified universal knowledge |
| Region or current-Place board | Players discover new people, things and local occurrences without naming them first | Every local change wakes everyone following that area, even when most do not care about the changed Entity |
| Exact Entity/Place resources | A player can keep focused attention on this tree and this Place with precise hints | A newly introduced local Entity is unknown until a broader Place read announces it |
| Private Character attention resource | The host follows one stable personalized resource while the Character moves and changes focus | World must compose or retain personalized attention, and shared hot-Entity work becomes harder to reuse |
| Hybrid discovery to exact watch | A coarse Place board reveals possibilities; the Agent adds exact watches only for chosen Entities | The host must update a small watch set, and the product must explain when focus begins and ends |

### Technical consequence at millions of Users

| Candidate | Routing key and read | Write/fan-out shape | Scale pressure |
|---|---|---|---|
| Worldwide board | one global resource; bounded newest page or digest | every qualifying World change invalidates one globally watched key | change rate × active hosts; global reconnect/read storm; any global revision or no-gap cursor becomes a hotspot |
| Region/Place board | one common resource per structural region or Place | one invalidation to the affected area cohort | hotspot is confined geographically but can still be a million-listener Place |
| Exact resources | one common key per Entity or Place; bounded exact read | only the changed subject cohorts are invalidated | many stable keys and bounded watches per connection; one hot Entity still has unavoidable O(listeners) egress |
| Private attention | one key per Character whose body aggregates dependencies | each shared change must discover and dirty many private keys | per-Character dependency state, personalized reads and weak shared caching/fan-out |
| Hybrid | Place key for discovery plus exact keys for focus | broad local hint plus precise subject hint; both coalescible | bounded subscription churn, duplicate hints and explicit watch limits |

## Candidate 1: one worldwide game board

### It has real game-design value

**Inference.** A worldwide board is not merely a technical anti-pattern. It can make
the persistence of one World tangible, create serendipity, surface distant cultures
and invite travel or response. It is the strongest candidate for the feeling that
millions of players are making history together.

Photon's first-party interest-group documentation makes the analogous distinction
concrete: group `0` broadcasts to every client in a room, while other groups deliver
only to subscribed clients. Photon presents area groups specifically as a way to
lower messages per second and raise feasible room population.
[Photon Interest Groups](https://doc.photonengine.com/realtime/current/gameplay/interestgroups)

### Subscription turns it into a global wake multiplier

**Evidence.** Ordinary pub/sub sends a publication to every active subscriber of its
subject. Core NATS names this one-to-many operation fan-out; messages with no active
interest can be discarded.
[NATS publish/subscribe](https://docs.nats.io/nats-concepts/core-nats/pubsub)

**Inference.** If the World accepts `W` board-visible changes per second and `C`
hosts subscribe to one global board, the uncoalesced delivery surface is proportional
to `W × C`. Persisting each change once does not remove those outbound bytes. With a
million connected hosts and a lively World, even content-free hints become the
dominant workload.

**Candidate implication.** Treat a global board as host-independent **pull** first:
an Agent explicitly asks for a bounded current page when the User wants global
discovery. It need not keep a million hosts continuously stale. If evidence later
shows that live global awareness is essential, test a shared coarse refresh cadence
or a very rare explicit worldwide-publication mechanic; neither should be smuggled
into every ordinary Activity.

### A global board must not require a global World revision

**Evidence.** Current Aicadia intentionally has one Place-local revision and no
global revision or counter. Each exact-Place write advances the Place pointer in its
own transaction; writes at different Places remain concurrent.
[Current protocol contract](../game/protocol.md#delivery-identity-and-exact-place-freshness)

**Inference.** One `world_board.latest_activity_id` row updated by every World action
would reintroduce the exact globally hot serialization point Aicadia forbids. A
globally ordered, no-gap subscription cursor would likewise require a commit-order
mechanism or a delivery partition whose ordering lane must be paid for.

**Candidate implication.** A read-only global discovery index could instead be
append-only and page a bounded newest window. Storage may later be partitioned by
time or stable hash so no semantic region becomes a permanent World shard. A fixed
bounded fan-in can merge partition heads for discovery. This does **not** provide
complete chronological catch-up: adding no-gap global order is a separate expensive
game invariant, not a free cursor field.

### Global visibility is a product and privacy rule

**Inference.** A raw feed of every accepted Activity would expose remote subjects,
private interactions and timing, undermine local discovery, and invite instruction
and spam attacks through player-authored content. A curated digest asks who decides
what is notable. World cannot make that semantic choice; deterministic newest-first
selection is neutral but likely dominated by the busiest hotspots.

**Candidate implication.** A global board is viable only after its contents have an
explicit game meaning—for example, deliberately Agent-authored public reports or a
bounded public discovery surface—and after abuse, authority and locality are decided.
The subscription mechanism cannot make all World history safe for global knowledge.

## Candidate 2: regional or current-Place boards

**Evidence.** Epic recommends location groups, rooms or zones whose persistent Actor
lists can be shared across connections. Photon maps interest groups to world areas,
and spatial pub/sub research models avatars as following cells or precise areas that
move with them. These systems make place-based discovery a first-class coarse filter,
not a per-object afterthought.
[Epic Replication Graph](https://dev.epicgames.com/documentation/en-us/unreal-engine/replication-graph-in-unreal-engine),
[Photon Interest Groups](https://doc.photonengine.com/realtime/current/gameplay/interestgroups),
[Hu, *Spatial Publish Subscribe*](https://mmve-workshop.org/2009/papers/p8.pdf)

**Inference.** A common Place board fits Aicadia's local discovery: it can become
stale when an Entity enters, leaves or is introduced, or when a locally authorized
Activity is accepted. An Agent need not know the new Entity URI in advance. The board
is also a natural recipient for an Agent-authored multi-Place effect: after an
accepted explosion package names A and B, the two common Place resources can be
invalidated without World inventing a blast radius.

**Candidate implication.** Start with structural Places already owned by World.
"Region" should exist only when the domain has an accepted structural region or
Place relation; subscription infrastructure must not invent semantic map shards.
One affected operation may name a small bounded list of exact Places.

**Inference.** Place boards contain the fan-out to local cohorts, but do not solve a
deliberately hot Place. One million listeners in one Place still require up to one
million outbound hints. Broad Place invalidation also causes false-positive refreshes
when many unrelated Entities change.

**Candidate implication.** A Place notification should remain content-free and
coalescible. The authoritative Place read is bounded and may reveal candidate Entity
URIs; the Agent then chooses which exact Entities merit continued focus.

## Candidate 3: exact Entity and Place resources

### Current MCP directly expresses exact-resource invalidation

**Evidence.** MCP resources have stable URIs. A client opts into update notifications
by listing specific resource URIs in `subscriptions/listen`; a
`notifications/resources/updated` message contains only the changed URI and the
client reads the resource again. Resource lists may vary by per-request authorization,
and resource permissions must be checked before operations. Core MCP defines URI
templates for discovery but the subscription filter itself is a list of exact URIs,
not a wildcard expression.
[MCP resources](https://modelcontextprotocol.io/specification/2026-07-28/server/resources)

**Inference.** Exact common resources match both MCP and the hot-Entity fan-out
shape. One tree has one resource key and one current representation regardless of the
number of listeners. Active gateways retain only ephemeral resource-to-connection
interest. A change can be computed and persisted once, then routed to every currently
interested connection.

**Candidate implication.** Keep transport identity separate from the Entity schema.
Illustrative, not chosen, URIs could distinguish a bounded current Place context and
one exact Entity current-state representation. They need not create tables or turn
resources into World subjects.

### Exact resources need a coarse discovery parent

**Inference.** Exact Entity watches cannot announce a new stone whose URI the Agent
does not yet know. Core MCP has no wildcard resource subscription. A Place resource
already known from current Character context can change, the host can refetch its
bounded Entity list, and an explicitly invoked Agent can then select the stone.

This makes an exact Place resource different from a private Character bundle: many
co-present hosts can share one routing key, even when the resulting body is
authorization-filtered.

## Candidate 4: one private Character-attention resource

**Inference.** A stable `current attention` resource is attractive to hosts: one
subscription could appear to follow movement automatically. Three implementations
all carry a cost:

1. include the whole current Place—unbounded in a crowded Place and not Agent-selected;
2. store the Agent's selected dependencies durably—one new per-Character state model
   plus writes whenever attention changes; or
3. retain the list only in one process—incorrect when any stateless server instance
   may serve the next read.

An authorization-derived current Place alias avoids stored session state, but its URI
is unique or private per Character. One shared tree change must then discover and
invalidate many personalized resources, and their bodies are harder to cache or
compute once.

**Candidate implication.** Reserve private Character resources for genuinely
private Character state. Do not make one personalized attention bundle the default
shared-world interest primitive unless a host experiment proves that exact URI lists
are unusable and a bounded dependency model is worth its durable ceremony.

## Candidate 5: hybrid discovery to exact watch

**Inference.** The hybrid is not two systems. It uses one resource-listen/read
primitive at two granularities:

```text
coarse: current Place resource changes
        -> bounded read discovers current local candidates

exact:  Agent selects the tree because it matters now
        -> host listens to the tree resource as well

later:  focus ends or Character moves
        -> host replaces the bounded exact watch set
```

This follows the Agent/World ownership split: Agent supplies semantic interest;
World supplies deterministic eligibility and current state. It also follows the game
systems' proven location-cohort plus exact-object pattern without importing their
frame simulation or geometry.

**Candidate implication.** Of the five candidates, this is the strongest KISS base:
one current-Place watch plus a small explicit set of selected exact Entities. Add a
global board as an explicit pull surface if its discovery value is accepted. Do not
automatically subscribe every Character to the global board.

## Lifecycle: movement, replacement, disconnect and refetch

### Interest changes are bounded connection state

**Evidence.** Photon lets clients add and remove group subscriptions dynamically,
while spatial pub/sub research identifies the cell-size trade-off: smaller cells
reduce false positives but increase subscription maintenance. MCP's listen filter is
fixed for one listen request; multiple listens may coexist, and an exact resource
list is supplied when opening one.
[Photon Interest Groups](https://doc.photonengine.com/realtime/current/gameplay/interestgroups),
[Hu, *Spatial Publish Subscribe*](https://mmve-workshop.org/2009/papers/p8.pdf),
[MCP subscriptions](https://modelcontextprotocol.io/specification/2026-07-28/basic/patterns/subscriptions)

**Candidate implication.** On an explicit move from Place A to B, a host can:

1. open a new bounded listen for B and still-relevant exact Entities;
2. wait for acknowledgment;
3. read B authoritatively; and
4. close the A listen.

The overlap may produce a harmless old-Place stale hint. Opening before reading
closes the initial listen/read race. If the connection breaks, re-listen and refetch;
no subscription state is required in World.

### No per-recipient durable receipt is needed

**Inference.** A live system inevitably stores ephemeral `(connection, resource)`
interest while the connection exists. That is operational delivery state, not a
Character fact. It can disappear with the gateway because the client re-declares it.

**Candidate implication.** Persist one accepted mutation and its existing Activity,
not one notification, witness or read cursor per listener. A disconnected host may
miss every hint and still recover current truth through the same bounded resource
read. Complete personal witness memory would be a separate game mechanic.

## Hot Entity, fan-out and backpressure

### One million listeners still means one million deliveries

**Inference.** If one million active hosts insist on hearing that the same tree
changed, no transport can make the final network egress O(1). The scalable objective
is instead:

- one World transaction and one Activity, independent of listeners;
- one small changed-resource publication into the delivery layer;
- one shared candidate/resource computation per revision where possible;
- ephemeral listener pointers distributed across gateways;
- bounded, coalesced output per connection; and
- zero automatic model calls.

The unavoidable O(listeners) work is copying a tiny hint to sockets, not re-running
World queries, LLMs or writing observer rows.

**Evidence.** NATS documents subject-based fan-out, wildcard subscription, and
efficient handling of tens of millions of subjects. Its gateway interest-only mode
forwards a subject to another cluster only when that cluster has expressed interest,
then fans out locally. These are first-party implementation claims, useful as proof
that a hierarchical interest graph is conventional—not proof that Aicadia needs NATS
or achieves its capacity.
[NATS subjects](https://docs.nats.io/nats-concepts/subjects),
[NATS gateway interest propagation](https://docs.nats.io/running-a-nats-service/configuration/gateways)

**Candidate implication.** A later gateway tier can shard open connections and keep
only local subscriber sets. The internal publisher need not know Users individually.
Do not add a broker before a small experiment demonstrates that the PostgreSQL-first
path cannot meet the exact connection and update load.

### Coalesce invalidations; never queue World history per socket

**Evidence.** NATS protects the system from slow consumers with bounded pending
buffers, message drops or disconnection; the application must detect loss and
recover. Redis Pub/Sub similarly documents at-most-once delivery and permanent loss
on disconnect, while recommending a durable store when replay is required.
[NATS slow consumers](https://docs.nats.io/running-a-nats-service/nats_admin/slow_consumers),
[Redis Pub/Sub](https://redis.io/docs/latest/develop/pubsub/)

**Inference.** Resource invalidation is especially coalescible: ten changes to the
tree before one host reads it still mean only "the tree representation is stale."
Intermediate Activity remains in the existing authoritative history if the game read
exposes it. The socket does not need ten deltas.

**Candidate implication.** Bound bytes and messages per stream, collapse duplicate
pending URIs, rate-limit repeated stale hints, and disconnect a slow consumer before
its buffer becomes unbounded. After any loss or reconnect, the host refetches. Quiet
Place capacity must be tested while one Entity is maximally hot.

### Avoid a refetch stampede

**Inference.** A hint does not require an immediate read and never invokes an Agent.
The safest host behavior is to mark an already-held resource stale, then refetch when
an explicit User turn actually needs it. If a host insists on eager refresh, bounded
jitter, request admission and shared revision caching become operational necessities.

**Candidate implication.** Measure notifications and authoritative reads separately.
One million delivered hints with only ten thousand subsequent explicit reads is a
different system from one million automatic reads and model turns.

## Authorization and privacy

**Evidence.** MCP permits the visible resource set to vary by per-request
authorization, requires resource URI validation and recommends checking resource
permissions before operations. MCP HTTP authorization is per request; it is not a
property of a remembered World session.
[MCP resources](https://modelcontextprotocol.io/specification/2026-07-28/server/resources),
[MCP authorization](https://modelcontextprotocol.io/specification/2026-07-28/basic/authorization)

**Inference.** Knowing or guessing an Entity URI cannot authorize its subscription or
read. The URI and notification timing can themselves disclose that an Entity, Place
or private interaction exists. Authorization only on initial listen is insufficient
when a Character later moves or an Entity leaves.

**Candidate implication.** Reuse the same deterministic eligibility predicate for
subscribe admission and authoritative read, and define revocation explicitly. A
movement or placement change can remove now-ineligible interests from ephemeral
gateway registries; a stale retained subscription must never enable a later read.

A common Place resource is safe for broad fan-out only if every notification and
returned field is visible to that whole cohort. Current Aicadia filters some
Interactions to their actor and targets. If a private Activity changes a common Place
revision, broadcasting "Place changed" may leak timing even when the body remains
hidden. Three candidate policies require a product choice:

1. a public Place resource invalidated only by cohort-public changes;
2. separate private Character/participant resources for private changes; or
3. accept explicitly that co-present Characters learn only that unspecified local
   activity occurred.

Do not hide this privacy choice inside a generic revision.

## Concrete Aicadia scenarios

### Paint one tree blue

1. The active Agent has current Place P and selected Tree T from authorized context.
2. Its host listens to common resources P and T.
3. The Agent submits its bounded change using current dependencies.
4. World serializes only the actual conflicting state, persists one accepted change
   and one Activity, then the delivery layer invalidates T and the relevant public or
   private P surface.
5. Every active T listener gets a stale hint; their Agents are not invoked.
6. On a later explicit turn, one host reads T and learns the latest current state.

If one million hosts watch T, persistence remains one change; outbound hints remain
up to one million. Competing paint and fell requests are settled by mutation
preconditions or any later accepted Agent collective mechanic, never by the number of
listeners.

### Discover a stone at the current Place

The stone had no known exact URI for this Agent. The accepted introduction invalidates
P. A bounded Place read reveals the new candidate. Only after the Agent decides the
stone matters does the host add its exact resource. No wildcard, global Entity list
or World semantic relevance function is required.

### An Agent-authored explosion affects Places A and B

Original interest-management research calls the analogous pair Area of Interest and
Area of Effect: a change goes to subscribers whose interest overlaps its effect, and
the effect may span multiple areas.
[Heger et al., *Towards an Interest Management Scheme for Peer-based Virtual
Environments*](https://eceasst.org/index.php/eceasst/article/view/2493)

For Aicadia, the important ownership differs from a physics engine:

1. the explicitly invoked proposing Agent names exact affected Places A and B and the
   bounded intended state changes;
2. World validates only the accepted structural scope, authority, versions and
   bounds;
3. one transaction persists the accepted consequences and Activity roles;
4. common resources A, B and exact modified Entities are invalidated; and
5. active hosts at A or B may later refetch.

The invalidation says neither "you heard a bomb" nor "your Agent agrees with its
meaning." Perception and collective judgment remain separate Agent/game mechanics.

## Current Aicadia seams

**Evidence.** Current exact-Place orientation already derives the User's Character
and current Place, returns a bounded Entity page and one Place revision. Exact Entity
state is available only for the Character, its current Place or a co-present placed
Entity, inside one repeatable-read transaction.
[Current Place Entity read](../../src/world/read.rs#L231),
[current exact Entity read](../../src/world/read.rs#L437)

**Evidence.** Current Agent guidance explicitly says to fetch one selected local
Entity only when it matters and stores no per-observer Property/Trait copy. The
published exact-Entity tool accepts no semantic-relevance selector.
[Agent knowledge contract](../../src/agent_contract/instruction/06-knowledge.md),
[`get_entity_at_current_place`](../../src/agent_contract/tool/get_entity_at_current_place.md)

**Inference.** Those are good read and eligibility seams for a future resource
experiment. They already separate Agent selection from deterministic World locality.
They are not yet a subscription API and the current Place-wide revision should not be
assumed to be the ideal long-term invalidation grain.

**Evidence.** Aicadia currently advertises only MCP tools, no resources or
subscription capability. The current server is loopback-only and uses stateless
Streamable HTTP handlers.
[Current MCP capabilities](../../src/server/mcp.rs#L452),
[current server transport](../../src/server/mod.rs#L42)

**Evidence.** Current `list_activity` is not a World feed: it returns only Activity in
which the current Character is actor or involved. A worldwide game board would be a
new game knowledge and privacy contract, not a new wrapper around that read.
[`list_activity` contract](../game/capability/list_activity.md)

## Choice criteria

Before accepting a surface, answer these in player terms and then technical terms:

1. **Discovery radius.** Should a normal active Character learn only current-Place
   candidates, a structural region, or bounded worldwide public reports?
2. **Timeliness.** Must it be pushed while active, or is explicit read at the next
   User turn sufficient?
3. **Completeness.** Is latest current state enough, or must every intermediate
   occurrence be recoverable without a gap?
4. **Agent ownership.** Which exact interests does the Agent select, and which narrow
   locality or authority checks may World perform deterministically?
5. **Privacy.** Is the URI, timing and existence of a change visible to every member
   of the routing cohort?
6. **Hot-subject promise.** What does a million-listener tree or Place guarantee:
   one coalesced stale hint, every occurrence, or just bounded pull?
7. **Watch bound.** How many current Place, regional and exact Entity resources may
   one active host follow?
8. **Churn.** Which explicit action changes the watch set, and can reconnect rebuild
   it without durable attention rows?
9. **Global order.** Does a worldwide board need only a bounded discovery page, or a
   complete no-gap order expensive enough to justify a sequencing lane?
10. **Host independence.** Can a non-subscribing host reach the same authorized truth
    through one bounded read?

## Falsifiers and smallest experiments

The exact-resource/hybrid candidate should be rejected or revised if any of these are
observed:

- target MCP hosts cannot keep or replace an exact-URI listen set, and cannot surface
  a stale resource without automatic model invocation;
- a bounded exact-URI list causes more host churn than one private resource under
  realistic movement and focus changes;
- authorization cannot revoke old Place/Entity notification timing without a
  per-notification database query or durable per-Character subscription model;
- a hot common Place resource forces one authoritative database read per listener
  immediately after every change;
- coalescing loses a game-required intermediate occurrence that no authoritative read
  can recover;
- an Entity change requires notifying an unbounded linked-Place set;
- one hot Entity or Place exhausts quiet-Place connection, read or mutation capacity;
- the worldwide board's player discovery value is materially higher only when live
  subscribed, and a bounded pull experiment fails to provide it; or
- users cannot understand the difference between following a resource, a Character
  perceiving an occurrence and an Agent being explicitly invoked.

### Minimal lab matrix

No production build is implied. A small Rust lab can compare:

| Variant | Active watches | Change burst | Required observation |
|---|---:|---:|---|
| worldwide subscribed board | every connection: 1 | independent World keys | total emitted hints and simulated refetch amplification |
| Place board | every connection: current Place | one normal and one hot Place | hot/quiet isolation and false-positive reads |
| exact resource | current Place + 1–8 Entities | one cold and one million-listener Entity | one durable change, bounded gateway memory, O(listener) bytes made explicit |
| private attention | one unique resource per Character | one shared Entity change | dependency lookup and personalized recomputation cost |
| hybrid | Place + selected exact Entity | discovery, focus, movement, reconnect | watch churn, missed hints and authoritative recovery |

Each variant should use disposable hints, the same bounded authoritative read model,
zero LLM calls and no per-recipient durable rows. The evidence claim must remain
simulation-only until a real MCP host, real PostgreSQL World and real transport are
each exercised at their own seam.

## Sources

Primary and authoritative sources used directly:

- [MCP `2026-07-28` resources](https://modelcontextprotocol.io/specification/2026-07-28/server/resources)
- [MCP `2026-07-28` subscriptions](https://modelcontextprotocol.io/specification/2026-07-28/basic/patterns/subscriptions)
- [MCP `2026-07-28` authorization](https://modelcontextprotocol.io/specification/2026-07-28/basic/authorization)
- [Epic Replication Graph](https://dev.epicgames.com/documentation/en-us/unreal-engine/replication-graph-in-unreal-engine)
- [Epic Actor relevancy](https://dev.epicgames.com/documentation/en-us/unreal-engine/actor-relevancy-in-unreal-engine)
- [Godot `MultiplayerSynchronizer`](https://docs.godotengine.org/en/stable/classes/class_multiplayersynchronizer.html)
- [Photon Interest Groups](https://doc.photonengine.com/realtime/current/gameplay/interestgroups)
- [Hu, *Spatial Publish Subscribe*](https://mmve-workshop.org/2009/papers/p8.pdf)
- [Heger et al., *Towards an Interest Management Scheme for Peer-based Virtual Environments*](https://eceasst.org/index.php/eceasst/article/view/2493)
- [Ricci and Carlini, *Area of Interest Management in Massively Multiplayer Online Games*](https://arpi.unipi.it/handle/11568/1055121)
- [NATS subjects](https://docs.nats.io/nats-concepts/subjects)
- [NATS publish/subscribe](https://docs.nats.io/nats-concepts/core-nats/pubsub)
- [NATS slow consumers](https://docs.nats.io/running-a-nats-service/nats_admin/slow_consumers)
- [NATS gateway interest propagation](https://docs.nats.io/running-a-nats-service/configuration/gateways)
- [Redis Pub/Sub](https://redis.io/docs/latest/develop/pubsub/)
- current Aicadia source and runtime contracts linked in [Current Aicadia seams](#current-aicadia-seams)
