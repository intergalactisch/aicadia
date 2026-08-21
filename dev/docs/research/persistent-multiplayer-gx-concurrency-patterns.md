---
status: superseded
era: August Activity-Property-Trait
---

# Persistent multiplayer GX and concurrency patterns

> **Superseded research direction:** retained for its sourced architecture
> comparison after the 2026-08-19
> [first-principles resolution reset](multiplayer-first-principles-resolution.md).
> Its Agent-rebase-first candidate no longer sets the active decision order.

> **Role / side:** sourced research report / development side.
> **Authority:** records this report's question, evidence, inferences and candidate implications.
> **Excludes:** product decisions and current implementation contracts; see `game/docs/`.

Date: 2026-08-19

Status: research; no architecture, term or game behavior below is accepted Aicadia
behavior

## Question and evidence boundary

Which game-development and distributed-runtime patterns can keep one persistent
canonical World fun and legible when thousands of explicitly invoked Agents try to
act at the same physical moment, including a deliberately hot Entity? Would an
Entity-sized micro-instance solve the problem, or merely move the serial bottleneck?

The User explicitly rejected a design whose technically correct ordinary outcome
is a stream of stale or busy results. This report therefore separates correctness
from GX: a system may preserve singular truth and still be a poor multiplayer game.

Sources are official engine or platform documentation and first-party engineering
accounts. **Evidence** is sourced behavior. **Inference** is analysis of that
behavior. **Candidate implication** is an unaccepted Aicadia direction. Vendor
limits and accounts are not production evidence for Aicadia.

## Core result

No architecture can accept two contradictory values for one canonical fact at one
logical instant. Somewhere, the authority must order, reject or transform them. The
choice that determines GX is therefore not whether serialization exists, but:

1. how small the serialized subject is;
2. which concurrent actions are structurally compatible;
3. how much harmless collision an Agent may absorb inside the User's explicit turn;
4. which true conflicts become recognizable game mechanics; and
5. how overload stays local without turning normal play into error handling.

“Micro-instance” names two materially different designs:

- a **content copy** gives different groups separate versions of a map, scene or
  Entity and therefore multiplies current truth; and
- a **virtual execution authority** gives one canonical coordination subject one
  lazy, movable single-writer lane while durable state remains singular.

Content copies are a poor foundation for Aicadia's canonical shared World. A virtual
execution authority is a plausible later implementation seam, but one authority per
whole Entity is often too coarse and one hot authority remains serial. The useful
key is the smallest concrete fact set whose invariant requires one order, not the
largest Entity or Place that happens to contain it.

## Pattern comparison

| Pattern family | What the player gains | Technical truth under pressure | Candidate Aicadia fit |
| --- | --- | --- | --- |
| Map, region or match copies | more players get a responsive local scene | each copy has different live state; cross-copy canon needs another mechanism | reject for canonical World changes; possibly useful only for explicitly non-canonical presentation later |
| One authoritative scene or simulation owner | coherent ordering and tick behavior inside a bounded scene | one dense causally coupled scene remains one hot lane; migration and failure need protocols | useful only for a later concrete real-time scene, not as the universal World model |
| Virtual actor or durable object | stable logical identity, lazy activation and horizontal scale across many independent subjects | calls to one actor are serialized; one hot actor cannot scale out without changing its state boundary | promising internal execution seam when keyed below Entity where game invariants allow it |
| Optimistic database transactions | cheap independent work through stateless servers | contradictions surface as retries, stale or aborts; database correctness does not create good GX | retain as truth floor, but do not expose it as the ordinary player rhythm |
| Fixed tick or action frame | simultaneous-looking inputs can be ordered or batched consistently | the frame still needs deterministic compatibility and conflict rules; one frame can overload | promising for an earned hot operation or coordination subject, not a global World tick |
| Local slowdown or bounded admission | one hotspot does not consume the entire service and participants stay relatively fair | accepted throughput is still bounded and queues must remain finite | protect quiet play; express pressure in game terms rather than raw infrastructure errors |
| Interest management and replication filtering | crowds remain observable at useful fidelity | reduces read/delivery work, not canonical write conflict | required but orthogonal to settlement |
| CRDT or last-write-wins merge | fewer explicit conflicts for mathematically mergeable data | semantic game outcomes may diverge from User intent or silently erase causality | retain only for a future fact whose game meaning is proven commutative; reject as universal settlement |

## Evidence and implications

### EVE Online: isolate a hot scene, then slow its clock

**Evidence.** CCP describes solar systems as logical load-balancing units assigned
to single-threaded nodes. When one solar system exceeds one core, it cannot be split
further by that model. CCP moved other workloads such as character, market and
corporation work away from those nodes and can place expected fleet fights on
dedicated nodes.
[Character Nodes](https://www.eveonline.com/news/view/fixing-lag-character-nodes),
[Fleet Fight Notification](https://www.eveonline.com/news/view/fleet-fight-notification-tool)

**Evidence.** Time Dilation reduces the rate at which clock-linked work enters an
overloaded system so its task queue remains bounded and relative gameplay semantics
survive, though at a slower game clock.
[Introducing Time Dilation](https://www.eveonline.com/news/view/introducing-time-dilation-tidi)

**Inference.** Operational isolation protects the rest of a persistent universe;
it does not make one causally dense battle horizontally parallel. EVE turns overload
into a visible game-time condition rather than an arbitrary subset of lost commands.

**Candidate implication.** Aicadia needs subject-local overload containment. A
future concrete hot mechanic may stretch a local action cadence or admission window,
but ordinary stateless Agent actions do not justify a global tick or World clock.

### Guild Wars 2 and Nakama: copies scale sessions, not shared canon

**Evidence.** Guild Wars 2's megaserver dynamically creates multiple copies of a map
and assigns players to them. ArenaNet also documented that copied maps may have
different event state, making cross-map World state inaccurate.
[Megaserver introduction](https://www.guildwars2.com/en-gb/news/introducing-the-megaserver-system/),
[World bosses and events](https://www.guildwars2.com/en/news/the-megaserver-system-world-bosses-and-events/)

**Evidence.** Nakama's authoritative multiplayer runs a match on one responsible
server instance and exposes a fixed tick for real-time, active turn-based, passive
turn-based or session-based game logic.
[Authoritative multiplayer](https://heroiclabs.com/docs/nakama/concepts/multiplayer/authoritative/)

**Inference.** Copies and match instances work when the product permits separate
session truth. They do not preserve one monument, construction or current fact that
every Aicadia Agent can revisit. A fixed match tick can make a bounded scene
coherent, but server affinity and match lifecycle are not a suitable universal
domain boundary for a sparse persistent World.

**Candidate implication.** Never copy a canonical Entity merely to increase write
capacity. Borrow action-frame semantics only for a concrete operation whose inputs
really form one bounded scene or resolution window.

### Orleans and Durable Objects: virtual authority scales across subjects

**Evidence.** Orleans virtual grains have stable logical identities, activate on
demand, are placed dynamically and process their own work single-threadedly. Orleans
also supports opt-in distributed ACID transactions across grains.
[Benefits of Orleans](https://learn.microsoft.com/en-us/dotnet/orleans/benefits),
[Orleans transactions](https://learn.microsoft.com/en-us/dotnet/orleans/grains/transactions),
[Grain directory](https://learn.microsoft.com/en-us/dotnet/orleans/host/grain-directory)

**Evidence.** Cloudflare recommends one Durable Object per logical coordination
unit such as a chat room, game session, document or resource. Each object is a
globally unique single-threaded instance with colocated persistent storage. Its FAQ
states that one object is inherently single-threaded and gives a soft request-rate
limit while the number of independent objects scales horizontally.
[Rules of Durable Objects](https://developers.cloudflare.com/durable-objects/best-practices/rules-of-durable-objects/),
[Durable Objects FAQ](https://developers.cloudflare.com/durable-objects/reference/faq/)

**Inference.** A virtual actor is an excellent routing and race-ownership model for
many independent coordination subjects. It cannot make one popular subject execute
unbounded contradictory work in parallel. Cross-actor atomicity introduces another
coordination protocol, so an actor per Entity is not automatically simpler than a
bounded PostgreSQL transaction.

**Candidate implication.** If measurements later earn it, any World instance may
route a hot logical conflict subject to one lazy virtual execution lane. That lane
is not a durable game object, does not own accepted truth only in memory and may
move or disappear. PostgreSQL current state plus Activity remains authority. A lane
crash before commit permits idempotent retry; a crash after commit returns the same
accepted result. Start with the same logical semantics in PostgreSQL before adding
an actor runtime.

### Unreal and Roblox: prediction hides latency, authority still decides

**Evidence.** Unreal's Replication Graph shares relevance computation and persistent
Actor lists across connections to reduce replication cost. This is interest
management, not a multi-writer truth mechanism.
[Replication Graph](https://dev.epicgames.com/documentation/en-us/unreal-engine/replication-graph-in-unreal-engine)

**Evidence.** Roblox describes the server as the authority for game state while
clients may predict local results and are corrected when the server disagrees.
[Server authority](https://create.roblox.com/docs/projects/server-authority)

**Inference.** Presentation and semantic preparation can run ahead of final
authority, provided correction is explicit and only the server result becomes
canonical. Aicadia's Agent calls are slow semantic turns rather than frame-rate
input, so their advantage is not twitch prediction but bounded reinterpretation of
new facts before final submission.

**Candidate implication.** An Agent can read just before commit, preserve compatible
concurrent work and recompose its exact operation inside the same explicit User
invocation, if the User confirmed a bounded outcome rather than one frozen storage
patch. World performs no inference and spends no tokens; it still validates the one
exact final operation.

## Candidate Aicadia shape

The source patterns support a hybrid, not one imported architecture:

1. **One canonical World.** No canonical map, Place or Entity copies. PostgreSQL
   current state and the same-transaction Activity remain the durable truth.
2. **Concrete capability semantics.** Each operation defines which exact facts are
   independent, compatible, rebasable or exclusive. Infrastructure never infers
   compatibility from prose.
3. **Natural player intent plus Agent non-interference.** The User confirms the
   recognizable in-World result, not a concurrency policy. Inside that same explicit
   turn, the Agent rereads and preserves every current fact outside the action's
   explicit affected facts. It asks again only when the intended player-visible
   result materially changes.
4. **A quiet direct path.** Independent work uses ordinary bounded PostgreSQL
   transactions through any stateless World instance. No actor hop is required just
   because virtual routing may exist later.
5. **A lazy hot-subject lane.** Measured hot coordination subjects may gain one
   replaceable virtual executor keyed to the smallest exact conflict unit. It can
   order and briefly micro-batch structurally compatible inputs, while every
   accepted operation retains its own attributable Activity; accepted truth never
   exists only in that process.
6. **Operation-owned true-conflict rules.** If two intended outcomes remain
   incompatible after Agent rebase, their capability must define a game result:
   direct order, contest, explicit cooperative operation or another accepted rule.
   A generic stale error is not itself the game design.
7. **Bounded multi-subject settlement.** One hard-bounded operation uses stable
   subject order and one database transaction or one explicit coordinator; it does
   not make nested synchronous calls across independent virtual actors.
8. **Separate attention.** Interest filtering, notices and refetch keep crowds
   legible but never decide or authorize the canonical write.

The fifth point is intentionally an operational candidate, not a present build. A
short subject-local action frame could absorb bursts whose actions are mechanically
compatible, but a frame duration, queue policy and deterministic resolver require a
real game capability before they can be chosen.

## Why an AI-Agent game can do better than a conventional client

An ordinary client usually submits button presses against a fixed local state. An
Aicadia Agent can instead carry a bounded semantic contract from its User:

- decompose one broad wish into independent exact facts;
- reread authoritative context immediately before the final MCP operation;
- preserve another Character's compatible work;
- re-author the exact final package without another User interruption when the
  confirmed outcome has not materially changed;
- summarize one coherent result from many Activities rather than surface transport
  retries; and
- recognize when a true contradiction crosses the confirmed boundary and return to
  the User or enter an explicit multiplayer mechanic.

This is not permission for autonomous background play. The re-read and recompose
loop remains bounded within one explicitly invoked User-owned Agent turn. World does
not interpret the outcome boundary; the submitted operation must still state exact
structural dependencies and writes that World can validate deterministically.
Current `game/docs/agent.md` requires new preview and confirmation after any post-
confirmation edit, so adopting this candidate would require a later explicit product
decision and contract change; research alone does not authorize it.

## MCP listeners inside the action loop

**Evidence.** Current MCP lets a client opt into updates for exact resource URIs.
`notifications/resources/updated` carries the URI whose representation changed, not
the changed World state. The documented next step is `resources/read`, and the host
application decides whether and how to expose or include that resource.
[MCP resources and subscriptions](https://modelcontextprotocol.io/specification/2026-07-28/server/resources)

**Evidence.** Over Streamable HTTP, the client opens one long-lived
`subscriptions/listen` request whose SSE response carries the selected
notifications. The stream is not resumable through `Last-Event-ID`; disconnect ends
it and requires a new listen request.
[MCP Streamable HTTP](https://modelcontextprotocol.io/specification/2026-07-28/basic/transports/streamable-http)

**Inference.** A listener is an optional freshness accelerator, not a lock, conflict
resolver, Agent invocation or truth channel. It can tell an already connected host
that its bounded current-context representation is dirty while the User considers a
proposal. It cannot portably wake an idle Agent, guarantee that the model saw the
change or prevent two requests already in flight from colliding.

**Candidate implication.** The same action loop should work with and without push:

1. the host optionally listens to one exact, authorization-scoped current-context
   resource and reads its baseline;
2. the Player gives an ordinary in-World instruction and the Agent prepares its
   exact affected facts;
3. any number of resource-update notifications coalesce into one local `dirty`
   marker without invoking a model;
4. on the next explicit model turn and always immediately before a dependent write,
   the Agent refetches when dirty—or unconditionally when no listener is available;
5. it preserves all facts outside the action's explicit affected facts and submits
   one exact operation with current preconditions; and
6. World still validates and settles atomically, so losing every notification cannot
   corrupt truth.

One stable per-Character or current-context resource is likely clearer for limited
hosts than asking them to maintain subscriptions to every changing Entity. That
resource would be a bounded authorized view, not Character memory or another World
truth. A million active listeners at one hotspot still require up to a million small
deliveries; gateways may coalesce repeated dirtiness, but no protocol removes that
physical fan-out.

Publishing uncommitted player intent through a subscription is a different game
system. It would require an explicit bounded intent or collective-operation state
that Agents can read and act upon. Making every ordinary action publish “I am about
to…” would add latency, griefing and token-spend advantages, so it remains suitable
only for a deliberately opened cooperative or contest mechanic.

## Concrete pressure scene

Noor tells an Agent only: “Paint this Table blue.” Mara concurrently carves a map
into a distinct surface detail.

Under the candidate Agent rule, Noor never manages preservation. Her Agent rereads
just before submission, sees the carving and submits blue colour while preserving
it because carving lies outside the action's explicit affected fact. Both actions
receive their own Activity and neither User experiences a stale error. If Mara
instead paints the same colour fact red, removes the relevant surface or changes
Noor's authority, the intended in-World result is materially different. The
operation-specific true-conflict rule applies or the Agent asks Noor in game terms.

The technical distinction is exact: the World does not decide that carving and
colour are semantically compatible from their prose. The capability's typed state
and Noor's bounded confirmed preservation rule make that structural distinction.

## Remaining decisions

1. Should ordinary Agent conduct preserve every current fact outside the action's
   explicit affected facts automatically, asking the User again only when the
   intended in-World result materially changes?
2. For a truly incompatible hot fact, which game rhythm should its first concrete
   capability use: immediate order, a short action frame, explicit contest or an
   opened cooperative operation?
3. What exact subject is small enough for a virtual coordination lane in the first
   earned capability, and can PostgreSQL alone prove the same semantics first?
4. Which overload response is enjoyable and honest for that capability without
   granting crowd size, request volume or token spend authority?

These are product choices. The researched systems constrain their technical costs
but cannot choose Aicadia's GX.
