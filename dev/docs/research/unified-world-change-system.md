---
status: pending
era: August Activity-Property-Trait
---

# Unified World change system

> **Role / side:** sourced architecture research / development side.
> **Authority:** records findings and candidate implications for one unified path from
> Agent-authored change through deterministic settlement and live observation.
> **Excludes:** product decisions, current game behavior and implementation contracts;
> see `dev/docs/concept/`, `game/docs/` and an accepted production plan.

## Question and evidence boundary

Can Aicadia use one simple core system for every accepted Entity change—creation,
Property and Trait development, later placement/relation changes, bounded
multi-Entity consequences, scoped World effects and optional collective assembly—
while also giving active Agents prompt, recoverable awareness and remaining correct
when millions of Agents act on distributed or deliberately hot subjects?

This report treats **one system** as one semantic and transactional change substrate,
not as one table, one process, one database key or necessarily one public tool. It
compares current Aicadia evidence with official documentation for transactional
change data, optimistic conflict sets, snapshot-plus-watch protocols, PostgreSQL
commit notification, MCP resource subscriptions and subject-routed transient
messaging. It does not establish a new `Change` domain type, generic event engine,
scope-effect mechanic, broker or database choice.

The existing retained multiplayer labs prove only their stated Rust or PostgreSQL
fixtures. No cited product proves Aicadia's semantics or million-Agent capacity.

## Result in one sentence

The strongest small architecture is **one bounded Agent-authored change package,
settled once by World, which atomically produces current state, one Activity and a
set of dirty resource identities; every live hint, refetch, replay-safe retry and
optional collective assembly refers back to that same package and authoritative
state**.

This is a unification, not a claim that every concern is the same:

| Concern | One-system role | Must not become |
| --- | --- | --- |
| Agent intent | compose one exact bounded package | executable prose or server inference |
| World settlement | validate structural authority, dependencies, bounds and atomicity | model judgment or conversational consensus |
| Current state | store the accepted Entity/fact result | a projection rebuilt from event replay |
| Activity | store one durable historical footprint of that acceptance | a second mutation authority or universal JSON event |
| Resource dirtiness | name which bounded authoritative reads may now be stale | recipient truth, per-Agent history or a global revision |
| Live delivery | carry coalescible dirty-resource hints | correctness, replay or automatic Agent invocation |
| Collective assembly | construct one candidate package when explicit authority requires it | a parallel state engine or traffic-triggered governance |

## Why the prior decomposition felt like several systems

The current exploration separately discussed exact Property coordination, Action
dependencies, Activity, Entity and Place listeners, proposal boards, consensus,
cooldown and PostgreSQL propagation. Those are real problems, but treating each as
an independent mechanism creates several identities for one World change:

- one request identity for mutation;
- another object for a proposal;
- another event for listeners;
- another cursor for catch-up;
- and possibly another result chosen by a vote.

That shape makes it unclear which object owns truth, which revision makes an Agent's
prepared context stale and whether a listener saw the same consequence that World
accepted. The evidence supports a stricter invariant:

> One accepted change has one stable request identity, one atomic settlement, one
> Activity and one mechanically derived set of dirty resources. Everything else is
> a view, transport or pre-settlement assembly of that same change.

`Activity` remains Aicadia's accepted history rather than being renamed to `event`.
The candidate working term **change package** names submitted transaction data; it
does not require a durable generic `change` table.

## Evidence from mature transactional systems

### Declarative transaction data can create and change several Entities at once

Datomic transaction data is a programmatically composable collection of assertions.
It supports temporary identifiers for Entities created inside the transaction and a
compare-and-swap form that requires an exact current value; `nil` can express
expected absence. The whole collection is accepted atomically. Datomic also reifies
transaction metadata for audit.

[Datomic transaction data](https://docs.datomic.com/transactions/transaction-data-reference.html)
[Datomic compare-and-swap](https://docs.datomic.com/transactions/transaction-functions.html#db-cas)

**Inference.** Aicadia can let one Agent package create a new Entity, refer to it
elsewhere in the same package and require exact current or absent facts without
inventing a separate creation pipeline. The transferable pattern is declarative
transaction input plus compare-and-swap, not Datomic's append-only database model or
global transaction order.

### Precise conflict sets let independent facts remain concurrent

FoundationDB transactions record read and write conflict ranges. A transaction
fails if another committed write intersects what it depended on. Its documentation
also warns that hot keys remain inefficient and gives a rough redesign threshold of
10–100 writes per second for one key. Watches say only that a key changed; the value
read afterward may already reflect further changes.

[FoundationDB transactions, conflict ranges and watches](https://apple.github.io/foundationdb/developer-guide.html)

**Inference.** Aicadia's operation-scoped dependencies and exact Property slots are
instances of one general rule: every concrete mutation declares bounded fact
coordinates it read and writes, then World revalidates them atomically. A watch is a
dirty hint over the same coordinate, not the source of its current value. No storage
engine makes a genuinely contradictory hot fact parallel.

### Snapshot plus watch is one recovery protocol, not two truths

Kubernetes clients first get or list authoritative objects, then watch from a
resource version. If the historical window has been compacted, clients receive
`410 Gone`, discard the stale cache, perform a new authoritative read and restart
the watch. Bookmarks help progress but are not promised at a fixed interval.

[Kubernetes API watch and recovery](https://kubernetes.io/docs/reference/using-api/api-concepts/#efficient-detection-of-changes)

**Inference.** Aicadia should likewise make baseline/refetch and live interest two
phases of one resource protocol. It should not copy Kubernetes' cluster-wide
`resourceVersion`: Aicadia's correctness and contention must remain scoped to exact
facts and resources, with no global World cursor.

### MCP subscriptions already express content-free invalidation

The current MCP Python SDK's `subscriptions/listen` opens one opt-in stream, carries
typed resource-update events, coalesces identical waiting events and explicitly
provides no replay or automatic re-listen. A reconnecting client refetches the
resources it depends on. Resource update notifications identify the changed URI and
mean that it may need to be read again.

[MCP Python SDK subscription contract](https://py.sdk.modelcontextprotocol.io/v2/api/mcp/client/subscriptions/)
[MCP resource update schema](https://modelcontextprotocol.io/specification/2025-11-25/schema#notificationsresourcesupdated)

**Inference.** Exact Aicadia resource identities can be the common language between
World change output and active Agent-host interest. MCP is a suitable outer protocol
where the host supports it, but it does not invoke a model, preserve missed history
or authorize a World change.

### PostgreSQL can emit the first post-commit dirty hint without an outbox

PostgreSQL delivers `NOTIFY` only after transaction commit and suppresses duplicate
channel/payload pairs inside one transaction. `LISTEN` has an initial setup race;
the documented safe pattern is listen, commit, read current state, then consume
notifications. The notification queue is finite and a stuck listener can prevent
cleanup; a full queue can make a notifying transaction fail at commit.

[PostgreSQL `NOTIFY`](https://www.postgresql.org/docs/current/sql-notify.html)
[PostgreSQL `LISTEN`](https://www.postgresql.org/docs/current/sql-listen.html)

Logical decoding can later expose a coherent change stream, but replication slots
retain WAL and may redeliver after crash, so consumers must be idempotent and
operationally monitored.

[PostgreSQL logical decoding](https://www.postgresql.org/docs/current/logicaldecoding-explanation.html)

**Inference.** Aicadia can begin with one compact post-commit dirty signal per
accepted change and authoritative refetch. `NOTIFY` is not the long-term global
router: gateway count, write rate, queue pressure, reconnect behavior and failover
must earn a transactional outbox, CDC or subject broker.

### A future subject router can preserve the same semantics

Core NATS routes transient messages by hierarchical subject, supports wildcard
interest and advertises efficient handling of tens of millions of subjects. It is
at-most-once; slow consumers may lose delivery because protecting the system takes
priority over an individual consumer.

[NATS subject-based messaging](https://docs.nats.io/nats-concepts/subjects)
[NATS slow-consumer behavior](https://docs.nats.io/running-a-nats-service/nats_admin/slow_consumers)

**Inference.** If measurements defeat bounded PostgreSQL-to-gateway notification,
an at-most-once subject router is semantically compatible with coalescible dirty
hints. It is not yet earned. JetStream or another durable broker would duplicate
World history and per-consumer backlog unless a future lossless projection has a
concrete need.

## Candidate unified change package

The package below is a research grammar, not a public schema:

```text
change_package {
  request_id
  actor_character
  settlement_basis
  context
  expect[]
  mutate[]
  occurrence
  causal_activity[]
}
```

### `request_id`

One Agent-generated stable identity survives uncertain delivery. World fingerprints
the normalized complete package. Equal replay returns the already accepted Activity
and state result; different content under the same identity conflicts.

### `actor_character` and `settlement_basis`

The actor remains explicit. The settlement basis names mechanically checkable
authority supplied by the concrete capability: for example current Character
control, a future delegated capability or one already-settled collective decision.
It cannot be an Agent's unsupported statement that the change is legitimate.

### `context`

The package names the bounded structural context in which the operation is allowed:
current Place, explicit target Entities, admitted relations or an explicitly bounded
scope. Context never contains remote prose-derived selectors such as “all nearby
wooden things.”

### `expect[]`

Every causal fact that must remain true is explicit and typed:

```text
entity_exists(entity_id, identity_version)
property_current(entity_id, key, value_or_version)
property_absent(entity_id, key, absence_slot_version)
trait_current(trait_id, statement_or_version)
placement_current(entity_id, place_id, placement_version)
relation_current_or_absent(...)
scope_current(scope_id, version)
```

The concrete operation—not arbitrary Agent input—defines which expectation forms it
allows and which mandatory World invariants are added. World never assumes the Agent
listed every semantic dependency, but it rejects any submitted dependency it cannot
validate structurally.

### `mutate[]`

One bounded unordered list contains concrete typed operations. Candidate operation
families include:

```text
create_entity(temp_ref, identity, initial_property[], initial_trait[])
set_property(entity_ref, key, expected, value)
establish_trait(entity_ref, statement)
develop_trait(trait_id, expected, statement)
set_placement(entity_ref, expected, place_ref)
set_relation(...)
establish_occurrence(...)
establish_scope_effect(...)
```

Only operations earned by current gameplay enter the real catalog. `set_placement`,
relations, occurrences and scope effects remain absent today. The single engine
does not turn all domain meaning into Properties: each operation owns its structural
validation and history roles while sharing admission, idempotency, dependency,
transaction and invalidation machinery.

Temporary references allow a newly created Entity to receive Properties, Traits,
placement and relations inside the same package. World resolves them to stable
Entity ids once and stores the resolution with accepted request identity.

### `occurrence` and `causal_activity[]`

One optional bounded occurrence description provides canonical Agent-authored prose,
explicit involved roles and any concrete mechanic-owned observation carrier. Causal
Activity ids preserve an explicit trail for a later consequence. Neither field
executes prose or lets World infer physics.

## The deterministic settlement algorithm

One `World` change path can apply every concrete mutation family through the same
steps:

1. decode, bound, normalize and fingerprint the complete package;
2. resolve contextual User and Character and check equal accepted retry first;
3. validate that every referenced existing Entity, Place, Trait, relation and scope
   exists and is structurally available;
4. translate the concrete operations and expectations into exact internal conflict
   coordinates;
5. reject packages whose coordinate, item, byte, Place or transaction-span bounds
   are exceeded;
6. acquire or compare those coordinates in one stable canonical order, with a hard
   pool, statement and lock-wait budget;
7. revalidate all submitted expectations plus mandatory authority, placement,
   locality and uniqueness invariants;
8. apply every state mutation, one Activity, its involved-Entity/resource edges and
   accepted request identity in one PostgreSQL transaction;
9. commit or apply nothing; and
10. emit one opaque post-commit dirty signal keyed by the accepted Activity/change
    identity so gateways can fetch its exact dirty-resource set.

No Agent reasoning, proposal discussion or network delivery occurs while database
locks are held. A busy or stale rejection writes no Activity. A request targeting a
nonexistent Entity fails before mutation; World never creates a missing target as a
fallback.

## Conflict coordinates: one grammar, several exact facts

One system does not mean one Entity lock. Each concrete fact has the smallest stable
coordinate that can own current/absent truth:

| Fact | Candidate coordinate | Concurrent behavior |
| --- | --- | --- |
| Entity identity/existence | Entity identity slot | creation/retirement conflict; ordinary independent facts need not |
| Property | `(entity_id, property_key)` slot | different keys compose; same key orders or conflicts |
| Trait lineage | stable `trait_id` current pointer | independent Traits compose; same lineage orders |
| Placement | Entity placement slot | movement conflicts without locking every Entity at either Place |
| Relation | stable relation identity or canonical endpoint/kind slot | exact same relation/absence conflicts |
| Generated discovery | mechanic-owned materialization key | concurrent reveal converges on one result |
| Scope effect | stable scope/effect identity | one global effect is hot but does not rewrite every Entity |

This keeps one transaction protocol while avoiding a Place-wide or Entity-wide
revision. Cross-fact invariants remain possible: a package that depends on Door
openness and changes a Stone lists both coordinates and commits both consequences
atomically if current. Stable lock order prevents an Agent-selected deadlock order.

## Authoritative resources are views over the same accepted change

Every read/watch identity maps to bounded authoritative data, for example:

```text
aicadia://entity/{entity_id}
aicadia://place/{place_id}
aicadia://scope/{scope_id}
aicadia://change/{proposal_id}   # only if collective assembly is accepted
```

An accepted package derives dirty resources mechanically from actual mutation
targets, involved Places, admitted scope and concrete audience rules. The Agent may
declare effect scope, but cannot forge authorization by adding an arbitrary private
resource. Activity-to-resource edges are stored once with history; there is no row
per observer.

The resource protocol is:

1. open live interest for a bounded authorized resource set;
2. read one bounded authoritative baseline;
3. treat each URI notification as “this resource may be stale”;
4. coalesce identical pending dirtiness;
5. refetch current state plus a bounded recent Activity window; and
6. after disconnect, restart at step 1 rather than replaying every missed hint.

Prepared mutations never rely on receiving every hint. Their exact `expect[]` facts
are revalidated at settlement. A Place resource needs no hot semantic Place revision
merely to be dirty; append-only Activity/resource edges and current fact versions are
the authority. A snapshot/watch race is handled by listen-then-baseline plus
duplicate-tolerant refetch.

## One system across the concrete scenarios

### One Agent creates or enriches an Entity

The Agent submits one package with a temporary Entity reference, initial Properties
and Traits. World allocates one durable identity, applies all state and writes one
Activity. A later enrichment uses the same package type with exact existing Entity
and fact expectations. There is no independent create pipeline.

If the package refers to an Entity id that does not exist and contains no explicit
`create_entity` operation for its temporary reference, World rejects it atomically.

### Several Agents change one table concurrently

Suppose three prepared packages make the table blue, set its leg count to three and
mark it as overturned:

- if they touch three independent Property slots and declare no cross-dependency,
  all three may commit and the combined state is one blue, three-legged, overturned
  table;
- if two write `color`, one accepts and the other receives stale/conflict/busy;
- if overturning was reasoned from the table having four legs, that exact leg fact is
  an expectation and a concurrent three-leg change makes the overturn package stale;
- if an Agent omitted a semantically important dependency, World cannot invent it.
  A concrete mechanic, capability or collective assembly must supply a stronger
  basis when that risk matters.

Every accepted package invalidates the same table resource; subscribers may receive
three signals or one coalesced stale signal and then read the authoritative combined
state and bounded Activities.

### A bomb changes the table while another Agent edits it

The initiating Agent explicitly packages the bomb occurrence, affected Places and
exact Entity consequences it has reasoned about. If “the table flies through the
window” depends on table placement, the window relation or a just-added Property,
those facts are expectations. A concurrent relevant change makes the package stale;
an unrelated inscription need not.

World validates that named Entities and Places exist, are within the concrete
operation's bounded structurally admissible scope, are currently fresh and may be
changed by the submitted authority. It never derives blast radius, force or which
objects should move from prose. Settlement changes all named current state and one
Activity atomically, or nothing.

### Nearby active Agents hear a bomb except deaf Characters

The accepted occurrence may carry an Agent-authored, concrete-operation-admitted
sound carrier and explicit affected Place scope. World makes those Place resources
dirty for authorized active listeners. Each receiving User-owned Agent reads the
occurrence together with its own Character's current Entity state and decides the
grounded presentation. A deaf Character's conversation can therefore omit hearing
without World assigning semantic meaning to arbitrary Traits.

This does not prove secret-safe sensory simulation. If future gameplay requires
World to prevent even the receiving Agent from accessing a signal, a typed sensory
capability with deterministic eligibility must be designed; arbitrary Trait prose
cannot provide it.

### “Now everything is blue”

Two meanings must not be conflated:

1. **One World-wide condition changes how everything is encountered.** The compact
   representation is one scoped effect with World scope. It invalidates the World
   scope resource once; gateways fan one coalescible dirty bit to each active host,
   and Agents interpret that effect alongside local Entities. It does not update
   millions of Entity rows.
2. **Every existing Entity's own `color` Property is literally rewritten.** That is
   intrinsically at least one logical mutation per affected Entity. No “one system”
   can make the history, conflicts and storage of millions of distinct fact changes
   free. The request must be rejected as unbounded or executed as explicitly bounded
   batches with non-atomic partial progress and a different accepted game contract.

The first is the recommended research direction for storms, global light, shared
eras and other contextual conditions. World need only determine structural scope;
the Agent owns semantic interpretation. A generic precedence rule that silently
overwrites arbitrary Entity Properties would make World understand meaning and is
not implied.

One global change still requires roughly one outbound delivery per connected host.
The system removes million-row mutation and repeated computation; it cannot remove
the network lower bound. Delivery coalescing, gateway broadcast and the absence of
automatic LLM calls keep that lower bound bounded.

## Optional collective assembly stays inside the same system

Collective Agent work should assemble the same immutable package that a direct
change submits:

1. an explicit capability opens one pending package identity and fixed bounds;
2. eligible, explicitly invoked Agents contribute complete candidate packages or
   bounded amendments;
3. a predeclared deterministic eligibility and selection rule chooses one immutable
   package or no package;
4. the chosen package enters the ordinary World settlement algorithm unchanged;
5. only accepted settlement changes Entity state and creates the World Activity.

Proposal state may have its own bounded resource for live invalidation, but it is not
a second World-state authority. Listener count never grants standing; conversation
never commits state; no model runs automatically; no lock spans deliberation. This
mechanic is optional and should be added only for a concrete communal authority or
semantic-risk case that direct action cannot legitimately settle.

## Public Agent surface: one engine does not require one giant tool

Three semantic operations are sufficient at the architecture level:

| Surface | Purpose | Candidate protocol form |
| --- | --- | --- |
| read | get bounded authoritative state and recent Activity for one resource | MCP resource read and/or typed `read_resource` tool |
| listen | receive coalescible dirty-resource identities for the currently active bounded interest set | MCP `subscriptions/listen` or equivalent host adapter |
| submit | atomically offer one exact change package | typed `submit_change` tool |

If collective assembly is accepted, `open_change`, `contribute_change` and
`settle_change` may be resource-specific operations over the same package lifecycle.
They do not become alternative Entity mutation paths.

One enormous union-shaped `submit_change` tool may overburden Agent descriptions and
make authorization mistakes easier. The implementation may publish a few concrete
capability-specific tools that compile into the same internal package and settlement
engine. The non-negotiable invariant is **one mutation path**, not one tool name.
HTTP and MCP must expose the same semantic contract for every published capability.

## Million-Agent and hot-subject gate

### Distributed work

Random UUID-backed Entity/fact identities spread independent work and avoid one
monotonic insert key. Every request, query, lock, retry and notification is bounded
by submitted coordinates or authorized resource interest. Any World instance can
route and execute the request; no process-local subscription membership participates
in correctness.

### One hot Entity or Property

One contradictory fact remains one serial decision lane in PostgreSQL, FoundationDB,
Spanner, CockroachDB or any other strongly consistent store. Distributed databases
also document unsplittable hot-row/range bottlenecks.

[CockroachDB hotspot guidance](https://www.cockroachlabs.com/docs/stable/understand-hotspots)
[Spanner hotspot-safe key guidance](https://docs.cloud.google.com/spanner/docs/schema-and-data-model#primary-keys)

The truthful guarantee is bounded outcome, not unlimited success:

- accept one current package;
- return exact stale/conflict when dependencies lost;
- return retry-after/busy before waiters exhaust the pool;
- coalesce resource dirtiness;
- keep quiet subjects outside the hot subject's lock and queue; and
- optionally use one explicit collective settlement to absorb competing semantic
  intent when the game grants that authority.

### One hot Place

Place discovery and Activity indexing will remain physically hot when every accepted
change is locally relevant, but it must not become the mutation correctness lock for
independent Entities. A hot Place resource may degrade its recap fidelity and live
hint frequency within explicit bounds while exact Entity state remains correct.
Operational partitioning can place the Place's observation stream near its active
gateways without changing semantic Place identity.

### One global scope effect

One rare global scope row is deliberately hot but writes once. The router can publish
one World-scope subject per gateway, and gateways can share serialization and mark
all live sessions stale. Every active host still requires one bounded outbound
signal; no architecture can avoid that physical fan-out.

### Storage evolution without a second contract

The semantic contract stays stable across an earned operational ladder:

1. PostgreSQL transaction plus Activity/resource edges and `NOTIFY` to a bounded
   gateway fleet;
2. measured outbox or logical decoding if publisher recovery or gateway count
   defeats `NOTIFY`;
3. transient subject routing if cross-host interest volume defeats database-wide
   notification; and
4. operational partitioning or distributed transactional storage only after the
   single writer is proven limiting and cross-partition atomicity is explicitly
   bounded.

These are transport/storage substitutions under one change-resource contract, not
new gameplay systems. None should be built ahead of its measured gate.

## Failure and overload matrix

| Failure | Required result |
| --- | --- |
| duplicate submit after uncertain response | equal fingerprint returns the same accepted Activity/result |
| same id with different package | deterministic request conflict, no state change |
| target Entity absent | deterministic unavailable/not-found result, no implicit create |
| one expected fact changed | whole package stale, no partial state or Activity |
| two packages touch disjoint facts | may both commit without a Place-wide lane |
| lock wait exceeds subject budget | busy/conflict, connection released, no history |
| World crashes before commit | zero accepted state, Activity and dirty-resource edges |
| World commits then response is lost | retry recovers canonical result |
| notification lost or duplicated | active host refetches; correctness unchanged |
| gateway restarts | reconnect, listen, baseline; no per-recipient replay requirement |
| hot resource outruns host | one pending dirty bit, then current state plus bounded recent Activity |
| PostgreSQL listener stalls | queue metric/timeout ejects it before commits are endangered |
| broker redelivers or drops | dirty processing is idempotent; authoritative read converges |
| scope change targets unbounded individual mutations | reject or use accepted scope-effect mechanic, never fake atomicity |
| Agent omits a semantic consequence | World cannot repair it; capability, revalidation or collective assembly must own the risk |

## What one system deliberately does not solve

- It cannot make one million incompatible writes to one fact all succeed.
- It cannot infer that chopping, painting, deafness, blast radius or “blue” has a
  particular physical meaning.
- It cannot make one million network deliveries disappear.
- It cannot prove an Agent declared every semantically affected Entity or Place.
- It cannot invoke offline Agents without violating token ownership.
- It cannot make arbitrary cross-partition transactions cheap or available during
  every network failure.
- It cannot use conversational consensus as truth.
- It does not justify a universal event table, event sourcing, rule engine, global
  revision, global feed, per-recipient delivery row or speculative broker.

## Smallest decisive experiment sequence

### 1. Unified semantic kernel — standalone Rust

Build one dependency-free retained model with exactly one package type and one
settlement function. Exercise:

- create plus initial Properties/Traits through temporary references;
- existing Entity enrichment;
- nonexistent-target rejection;
- independent and conflicting table changes;
- a bomb package with explicit multi-Entity/Place consequences;
- a World-scoped effect versus rejected unbounded per-Entity rewrite;
- exact Activity count and mechanically derived dirty resources;
- equal retry, changed fingerprint, stale fact and busy outcome.

The proof is only semantic determinism of the fixture.

### 2. PostgreSQL transaction and hot-subject fixture

Use a scratch schema to compare exact fact coordinates, stable multi-coordinate lock
order, temporary Entity resolution, Activity/resource-edge atomicity and bounded
`NOWAIT`/lock-timeout behavior. Run one hot Property beside many quiet Properties and
one hot Place observation index. Record lock/pool waits, outcomes, WAL and query
buffers. This proves only the real PostgreSQL fixture, not hosted scale.

### 3. Post-commit dirty-resource delivery fixture

Commit a package with several dirty resources, send one compact `NOTIFY`, let bounded
gateway listeners fetch the resource set and coalesce by URI, then inject loss,
duplication, disconnect, slow consumers and listener restart. Compare `NOTIFY` with
one subject-router variant only if measured gateway amplification can change the
decision.

### 4. Direct MCP/host smoke

Only after the resource contract is fixed, prove the smallest native
`subscriptions/listen` or equivalent adapter path in each supported host. Verify
listen-before-baseline, URI invalidation, authorized refetch, coalescing and reconnect.
No host smoke claims Agent understanding unless one explicitly invoked Agent call is
part of that bounded run.

## Research conclusions

### Supported by evidence

1. Declarative multi-Entity transaction data, temporary references and exact
   compare-and-swap including expected absence are established patterns.
2. Fine conflict coordinates preserve independent concurrency; one hot coordinate
   remains a real bottleneck in centralized and distributed stores.
3. Baseline plus watch/refetch is a mature single recovery protocol.
4. MCP resource updates and database watches are invalidation hints, not current
   values, replay, authority or Agent invocation.
5. PostgreSQL can atomically couple commit with a first dirty notification, but its
   setup race, finite queue and broadcast shape require bounded listeners and
   monitored gates.
6. Subject routing can scale transient interest without becoming World truth;
   at-most-once delivery matches coalescible hints.

### Inferred candidate direction

1. Replace the current family of mutation paths with one bounded internal change
   package and deterministic settlement engine when a future accepted build earns
   that migration.
2. Derive Activity, resource dirtiness and retry result from the same accepted
   package in the same transaction.
3. Treat Entity, Place, World-scope and possible proposal resources as bounded views
   over that state, not separate systems.
4. Represent huge contextual change as one structurally scoped effect rather than
   millions of implicit Entity rewrites.
5. Let collective Agent work assemble the same package only when one explicit game
   authority requires it.
6. Begin with PostgreSQL and MCP-compatible invalidation; preserve a measured seam
   for a subject router and later operational partitioning without prebuilding them.

### Product decisions still required

1. Which first concrete operation catalog proves the unified package without
   introducing a generic consequence engine?
2. Does Aicadia accept a structurally scoped effect as current World state, and what
   exact read makes it available without World interpreting its semantic content?
3. Which mutation facts and scopes may one ordinary Character control, and which
   require capability or collective settlement?
4. Should Agents see one `submit_change` union or a few concrete tools that compile
   into the same engine?
5. What exact item, subject, Place, lock-wait, retry, recent-Activity and live-latency
   bounds define the first claim?
6. Which supported hosts can keep one explicit Agent invocation live enough to
   consume MCP resource changes without a server-triggered model call?

## Primary source ledger

| Source | Used for | Does not prove |
| --- | --- | --- |
| [Datomic transaction data](https://docs.datomic.com/transactions/transaction-data-reference.html) | declarative atomic package, tempids, transaction metadata | Aicadia storage model or scale |
| [Datomic transaction functions](https://docs.datomic.com/transactions/transaction-functions.html) | exact and absent compare-and-swap | semantic completeness |
| [FoundationDB developer guide](https://apple.github.io/foundationdb/developer-guide.html) | precise conflict sets, watches, hot-key limit | PostgreSQL behavior or unlimited hot writes |
| [Kubernetes API concepts](https://kubernetes.io/docs/reference/using-api/api-concepts) | baseline/watch/relist recovery | permission for a global Aicadia revision |
| [MCP Python subscription API](https://py.sdk.modelcontextprotocol.io/v2/api/mcp/client/subscriptions/) | current listen, coalescing, loss/refetch boundary | uniform host support or model invocation |
| [MCP resource update schema](https://modelcontextprotocol.io/specification/2025-11-25/schema) | URI invalidation semantics | World truth or replay |
| [PostgreSQL `LISTEN`](https://www.postgresql.org/docs/current/sql-listen.html) and [`NOTIFY`](https://www.postgresql.org/docs/current/sql-notify.html) | transaction delivery, setup race, queue and coalescing | durable replay or massive fan-out |
| [PostgreSQL logical decoding](https://www.postgresql.org/docs/current/logicaldecoding-explanation.html) | ordered durable database-change seam and duplicate recovery | game-level event semantics |
| [NATS subjects](https://docs.nats.io/nats-concepts/subjects) and [slow consumers](https://docs.nats.io/running-a-nats-service/nats_admin/slow_consumers) | future transient resource routing | need for a broker now |
| [CockroachDB hotspots](https://www.cockroachlabs.com/docs/stable/understand-hotspots) | distributed hot-row/range limit | choice of CockroachDB |
| [Spanner schema model](https://docs.cloud.google.com/spanner/docs/schema-and-data-model) | distributed key hotspot avoidance | choice of Spanner |
