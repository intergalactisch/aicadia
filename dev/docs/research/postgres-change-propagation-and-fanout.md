---
status: pending
era: August Activity-Property-Trait
---

# PostgreSQL change propagation and subscription fan-out

> **Role / side:** sourced research report / development side.
> **Authority:** records this report's question, evidence, inferences and candidate implications.
> **Excludes:** product decisions and current implementation contracts; see `game/docs/`.

Date: 2026-08-16

Status: research; no propagation mechanism, board shape or infrastructure choice
below is accepted Aicadia behavior

## Question and evidence boundary

After one Aicadia World transaction has committed authoritative state and Activity,
how can multiple server or gateway processes promptly tell currently connected hosts
that an Entity-, Place-, region- or board-shaped resource may have changed?

The examined mechanisms are PostgreSQL `LISTEN`/`NOTIFY`, bounded authoritative
polling, logical decoding/change-data capture (CDC), a transactional outbox and the
minimum relevant broker alternatives. The hostile cases are:

- many independent World changes distributed over millions of subjects;
- one extremely hot Entity or Place;
- one logical global game board;
- many gateway processes and one million active subscribers;
- duplicate, late, reordered or lost notification delivery;
- slow consumers, reconnect storms and database failover; and
- no second authority, automatic Agent invocation or correctness state in a process.

This report uses PostgreSQL, SQLx, Debezium and broker-owner documentation plus a
read-only audit of the current checkout. **Evidence** is a directly sourced fact.
**Inference** is analysis. **Candidate implication** is an unaccepted Aicadia
direction that still requires a product choice, accepted plan and exact evidence.

The companion multiplayer report already establishes the external invariant: store
one World consequence, send a coalescible refresh hint and recover through an
authoritative bounded read. This report does not repeat the game-observation or MCP
analysis. [Realtime fan-out](multiplayer-concurrency-and-world-observation.md#realtime-fan-out-and-durable-catch-up),
[MCP subscription boundary](mcp-subscriptions-and-collective-agent-intents.md#streamable-http-lifecycle-and-loss-model)

## Result

**Inference.** For disposable resource invalidation, the smallest plausible internal
path is not a durable event platform:

```text
World commit remains authoritative
    -> best-effort compact PostgreSQL notification
    -> one database listener per serving gateway process
    -> gateway marks matching local subscriptions dirty and coalesces repeats
    -> connected host receives a small refresh hint
    -> current resource is read and authorized again
```

The notification can be late, duplicated, reordered or absent. It carries no state
delta and settles nothing. Startup, reconnect and any explicit Agent turn recover
from current PostgreSQL state. Process-local connection and interest maps are
acceptable transient delivery state only because their loss cannot change World
truth.

**Candidate implication.** Start by evaluating one generic subject invalidation
shape, not one messaging subsystem per Entity type. One compact message names a
bounded set of changed resource keys; the serving gateway decides which of its own
active streams care. Do not open a database connection per Agent or per Entity, do
not store a durable broker cursor per Character, and do not introduce CDC, an outbox
or a broker until one measured failure of the simpler path earns it.

**Inference.** This is not yet a scale claim. Direct PostgreSQL broadcast sends every
change to every listening gateway process. It can be an excellent small beginning
and still become the first measured bottleneck as gateway count or write rate grows.
The external resource/read contract must therefore be independent of the internal
publisher so a later sharded relay does not change game semantics.

## Current Aicadia seam

**Evidence.** `World` currently owns one cloneable `PgPool`; normal mutations begin a
transaction, store current state and Activity, commit, then return the accepted
result. A representative Action advances the current Place revision before commit.
[World](../../../game/src/world/mod.rs#L108-L156),
[Action commit](../../../game/src/world/mutation.rs#L452-L468),
[Interaction commit](../../../game/src/world/mutation.rs#L614-L635)

**Evidence.** Startup configures a ten-connection SQLx pool. The HTTP/MCP server has
no PostgreSQL listener or shared publisher. MCP uses a stateless handler manager and
advertises tools only; the existing server therefore has no resource-subscription
delivery path. [startup](../../../studio/src/bin/aicadia.rs#L19-L31),
[server](../../../game/src/server/mod.rs#L42-L62),
[MCP capabilities](../../../game/src/server/mcp.rs#L504-L512)

**Inference.** No existing code seam proves whether publication belongs immediately
after a `World` commit, in a shared application wrapper or in a database-derived
relay. A database trigger avoids a forgotten application call but runs inside the
authoritative transaction. A post-commit application call preserves mutation
availability but has a crash window. CDC removes that application call but adds a
durable operational pipeline. That placement is a design choice, not a research
fact.

**Candidate implication.** A lab experiment should name its exact seam and claim. A
simulation or in-memory broadcaster can prove coalescing and local routing; it cannot
prove PostgreSQL commit, reconnect or failover behavior. A PostgreSQL experiment
should use the pinned SQLx `0.8.6` API and exercise an actual separate listener
session.

## `LISTEN`/`NOTIFY`: what it guarantees

### Commit relation and ordering

**Evidence.** `NOTIFY` sends to every session currently listening to the named
channel in the same database. When executed in a transaction, delivery happens only
if and after that transaction commits; a listening client receives notifications
only between its own transactions. PostgreSQL preserves send order within one
transaction and commit order between notifying transactions. Identical
channel/payload pairs may be folded only when repeated inside the same transaction;
notifications from different transactions are not folded.
[PostgreSQL `NOTIFY`](https://www.postgresql.org/docs/current/sql-notify.html)

**Inference.** An in-transaction notification can never announce an aborted World
change. A notification sent in a separate transaction after the World commit does
not inherit the World transactions' order: concurrent callers can commit World
changes in one order and run their later `NOTIFY` statements in another. That is safe
only when the message means “this resource may be stale,” never “apply this delta
next.”

**Evidence.** A payload is under 8,000 bytes in the default configuration. PostgreSQL
recommends storing larger data in a table and sending its key. Channel names and
payload meaning are entirely application-defined, and notifications are visible to
all database users. [`NOTIFY`](https://www.postgresql.org/docs/current/sql-notify.html)

**Candidate implication.** Put no World prose, private state or authorization result
in a database notification. At most carry opaque resource kind/id and an optional
change token. If one accepted Agent-authored effect names more subjects than fit in a
bounded compact message, either send bounded chunks or send one durable Activity key
for gateway refetch; the exact choice depends on the accepted affected-scope bound.

### Initial race, disconnect and recovery

**Evidence.** `LISTEN` itself takes effect at commit. PostgreSQL documents an initial
race and prescribes this sequence: commit `LISTEN`, inspect current database state in
a new transaction, then rely on later notifications. Initial notifications can
duplicate state already observed. A session's registrations disappear when the
session ends. [`LISTEN`](https://www.postgresql.org/docs/current/sql-listen.html)

**Evidence.** SQLx `PgListener` auto-reconnects and re-subscribes, but its own
documentation states that notifications received while the connection was lost are
transient and will not be returned. `try_recv` lets the application observe a lost
connection and perform recovery; `recv` reconnects transparently without supplying
the missed interval. [`PgListener` 0.8.6](https://docs.rs/sqlx-postgres/0.8.6/sqlx_postgres/struct.PgListener.html)

**Candidate implication.** A gateway lifecycle can close the race without replay:

1. establish and commit one internal database `LISTEN`;
2. authoritatively reconcile only that gateway's active resource interests in
   bounded batches;
3. treat subsequent messages as dirty flags;
4. on detected loss, reconnect/re-listen before reconciling again; and
5. after an external host reconnects, make it re-read its current resources.

This sequence permits duplicate refreshes. It cannot reconstruct an every-change
feed, nor should it pretend to.

### Queue, payload and mutation availability

**Evidence.** PostgreSQL holds undelivered notifications in a shared queue. With the
current default `max_notify_queue_pages`, 8 KiB pages permit up to 8 GiB. A listener
that remains in a long transaction can prevent cleanup. PostgreSQL logs warnings
after half the queue is occupied; `pg_notification_queue_usage()` exposes the used
fraction. If the queue fills, a transaction executing `NOTIFY` fails at commit.
[`NOTIFY`](https://www.postgresql.org/docs/current/sql-notify.html),
[`max_notify_queue_pages`](https://www.postgresql.org/docs/current/runtime-config-resource.html),
[`pg_notification_queue_usage`](https://www.postgresql.org/docs/current/functions-info.html)

**Inference.** Putting a refresh-only `NOTIFY` or notification trigger inside the
World transaction couples World write availability to the health of a disposable
hint queue. Sending after commit prevents that coupling but introduces the explicit
crash/error gap between durable commit and notification. That gap is acceptable only
because current state/Activity, not the hint, is recovery truth.

**Candidate implication.** For the KISS invalidation path, prefer a bounded
post-commit attempt plus authoritative recovery over making a full notification
queue able to reject an otherwise valid World mutation. If later gameplay requires
proof that every commit was eventually published, that is a stronger contract and
earns a transactional outbox or CDC; it must not be smuggled into “realtime.”

### Connection and process amplification

**Evidence.** `LISTEN` is session state. SQLx's pool-backed `PgListener` holds a pool
connection and re-acquires on reconnect. PostgreSQL uses a server process per client
connection; `max_connections` is finite and increasing it allocates additional
server resources. [`PgListener`](https://docs.rs/sqlx-postgres/0.8.6/sqlx_postgres/struct.PgListener.html),
[PostgreSQL connections](https://www.postgresql.org/docs/current/runtime-config-connection.html),
[connection process model](https://www.postgresql.org/docs/current/connect-estab.html)

**Inference.** One listener per Agent, Entity or external SSE stream is categorically
wrong. One long-lived listener per gateway process costs one database session; using
the current ten-connection request pool would reserve ten percent of that process's
pool even when no mutation is executing. A separate listener connection protects
request-pool capacity but still consumes one PostgreSQL connection per process.

**Candidate implication.** Treat direct listeners as a deliberately bounded gateway
tier and measure their database connection budget. Autoscaling arbitrary request
workers must not silently create arbitrary new listeners. This operational bound
does not create gameplay server affinity: any World instance may still accept a
mutation, and current resources remain readable after any gateway dies.

## Bounded polling

There are two materially different things called polling.

### Poll current subscribed resources

**Inference.** A gateway can batch the unique Entity/Place/board-partition keys used
by its local connections, read their current revisions or current representations,
and compare them with its last observed values. This catches all missed invalidations
without reconstructing intermediate changes. Work is proportional to active unique
interests, not total World size or connection count if same-subject interests are
deduplicated inside the gateway.

For gateway `g`, let `K_g` be unique locally subscribed resource keys, `b` the maximum
keys per query and `T` the reconciliation interval. The database sees approximately
`sum_g ceil(K_g / b) / T` reconciliation queries and `sum_g K_g / T` checked resource
rows. One million connections on the same tree still produce one tree check per
gateway per interval; one million unrelated trees still require work proportional to
one million active interests.

**Candidate implication.** Use current-resource polling as startup/reconnect recovery
and possibly a slow safety reconciliation, not as high-frequency per-connection
polling. Bound batch cardinality, pages, query time and reconnect concurrency. If an
explicit Agent turn already performs authoritative orientation, correctness does not
require a permanent periodic poll at all.

### Poll an append-only global change feed

**Inference.** Polling `Activity WHERE (occurred_at,id) > cursor` is not automatically
gap-free. Independent transactions can allocate timestamps or UUIDs before commit
and then commit in the reverse order, so a row can become visible later while sorting
before a saved cursor. The existing multiplayer report details this ordering trap.
[Catch-up ordering](multiplayer-concurrency-and-world-observation.md#catch-up-has-a-non-obvious-ordering-problem)

**Candidate implication.** Do not add one global World sequence to “fix” polling; it
would be a hot serialization row and a global feed. A no-gap delivery projection
would need its own partitioned watermark, overlap/deduplication with a declared
lateness bound, or WAL-derived order. If exact every-change replay is not gameplay,
current-state reconciliation remains far simpler.

## Transactional outbox

**Evidence.** Debezium defines the outbox pattern as inserting a dedicated outbox row
with internal state so downstream exchange cannot diverge from the committed
database change. Its event router expects insert-only outbox rows, uses a unique event
id for duplicate handling and can use an aggregate id as a message key for partition
ordering. [Debezium outbox event router](https://debezium.io/documentation/reference/stable/transformations/outbox-event-router.html)

**Evidence.** PostgreSQL `SKIP LOCKED` gives an inconsistent general-purpose view but
is explicitly suitable for multiple consumers claiming work from a queue-like table.
[`SELECT ... SKIP LOCKED`](https://www.postgresql.org/docs/current/sql-select.html)

**Inference.** An outbox removes the post-commit dual-write gap: current World state,
Activity and an explicit publication record commit together. A bounded relay can
claim batches and retry. It also introduces a table, indexes, retention/cleanup,
claim leases or locks, retry/deduplication, poison handling, lag monitoring and a
publisher process. It does not by itself broadcast to every gateway: one claiming
relay receives a row once, so another fan-out transport is still needed, while every
gateway independently polling the outbox multiplies reads and consumer-watermark
state.

**Candidate implication.** An outbox is justified only if “every committed change is
eventually emitted to a downstream system” becomes a real invariant, for example an
external durable search projection. It is excess ceremony for a refresh hint whose
consumer always refetches current state. If earned, an outbox event should identify
an idempotent rebuildable invalidation, not duplicate full World truth.

## Logical decoding and CDC

**Evidence.** PostgreSQL logical decoding turns WAL changes into an application
stream. A logical slot supplies changes from one database in source order, persists
independently of the connection and normally emits a change once. After a crash its
checkpointed position may move backward and redeliver recent changes, so consumers
must handle duplicates. Only one receiver can consume a slot at a time; most
independent consumers require separate slots.
[logical decoding](https://www.postgresql.org/docs/current/logicaldecoding-explanation.html)

**Evidence.** A slot retains required WAL and catalog rows even without a connected
consumer. PostgreSQL warns that this can consume storage and, in extreme cases,
threaten database operation. `max_slot_wal_keep_size` can bound WAL retention at the
cost of making a lagged slot unusable. Logical failover slots require explicit
primary/standby configuration and must actually be synchronized before promotion.
[slot retention](https://www.postgresql.org/docs/current/logicaldecoding-explanation.html),
[WAL retention](https://www.postgresql.org/docs/current/warm-standby.html),
[logical failover](https://www.postgresql.org/docs/current/logical-replication-failover.html)

**Evidence.** Debezium's PostgreSQL connector reads logical decoding through a
replication slot. Its deployment requires logical WAL, replication privileges,
publication/slot configuration, persistent offset state and operational care for
WAL retention and failover. Multiple independent connectors require separate slots;
sharing one slot between competing connectors can silently divide changes rather
than broadcast them. [Debezium PostgreSQL connector](https://debezium.io/documentation/reference/stable/connectors/postgresql.html)

**Inference.** CDC is a strong source for a durable, replayable, rebuildable
projection, but it is not a direct millions-of-gateways subscription mechanism. One
CDC consumer still needs a broker or routing tier to broadcast. One slot per gateway
would multiply privileged connections, retained positions and failover burden.
Database row changes also do not automatically say which semantic MCP resources are
affected when one Action touches Activity, Entity state, Traits and Places; that
mapping still belongs to deterministic application logic or an explicit outbox.

**Candidate implication.** Earn CDC when measured projection lag/recovery or
post-commit gaps matter enough to operate WAL slots and a relay. Do not introduce it
to make a disposable dirty flag “reliable,” and never let a CDC consumer become the
only copy of a World consequence.

## Broker alternatives: what they would and would not fix

**Evidence.** Core NATS and Redis Pub/Sub are live, at-most-once transports: an
offline subscriber misses a message. NATS disconnects slow consumers to protect the
system. Redis 7 sharded Pub/Sub restricts a message to one cluster shard instead of
propagating it to every node. Durable NATS JetStream consumers add persisted stream
state, acknowledgement, redelivery and flow control.
[NATS delivery](https://docs.nats.io/nats-concepts/what-is-nats),
[NATS slow consumers](https://docs.nats.io/running-a-nats-service/nats_admin/slow_consumers),
[Redis Pub/Sub](https://redis.io/docs/latest/develop/pubsub/),
[JetStream consumers](https://docs.nats.io/nats-concepts/jetstream/consumers)

**Inference.** A small redundant relay plus a subject-sharded live broker can remove
all-gateway `LISTEN` fan-out and PostgreSQL connection growth. It does not remove
per-recipient network delivery, and its lossy semantics are no stronger than the
desired hint. JetStream/Kafka-style durability adds another retained log and
consumer state; one durable consumer per Character recreates the million-consumer
problem outside PostgreSQL.

**Candidate implication.** A broker is a later operational substitution behind the
same invalidation contract, not part of the first domain model. Add one only after
direct PostgreSQL evidence shows listener connection, notification throughput or
all-gateway amplification is the limiting edge. Prefer live subject routing for
disposable hints; use durable streams only for a separately accepted replayable
projection.

## One global game board versus scoped invalidation

### A global board need not be a global row or feed

**Inference.** “One board for the whole game” can mean one logical read interface,
not one physical row, one revision or one broadcast topic. A bounded board can query
indexed durable Entity/Place/Activity data by stable partition and keyset page:

```text
global board interface
    -> board partition selected by stable region/subject family
    -> bounded page of durable current subjects or recent Activity
    -> each result keeps its own subject/revision identity
```

No write updates a global `board_revision`. A root resource can describe stable
partitions and query capabilities; it need not change whenever a tree changes. The
logical board may also be an eventual, rebuildable read projection, provided its lag
is explicit and mutation preconditions return to authoritative World state.

**Candidate implication.** This preserves the compelling player concept of one
shared World board without creating one global correctness lane. Exact whole-World
snapshot freshness, a total World change order and notification of every change to
every board viewer are different, much more expensive contracts and should remain
absent unless gameplay proves their value.

### Amplification can be stated before it is measured

Let:

- `lambda_s` = committed changes per second for subject/partition `s`;
- `G` = gateway processes directly listening to one global PostgreSQL channel;
- `G_s` = gateways with at least one current interest in `s`;
- `C_s` = active external connections subscribed to `s`; and
- `C` = all active connections.

**Inference.** One global PostgreSQL notification channel creates approximately
`G * sum_s(lambda_s)` database-to-gateway deliveries. It is independent of the
number of external subscribers, but every gateway must inspect every change.

A literal global board stream that pushes every change to every connection creates
approximately `C * sum_s(lambda_s)` external deliveries. A subject- or region-routed
path reduces routing work to `sum_s(lambda_s * G_s)` and necessary outbound work to
`sum_s(lambda_s * C_s)`. The last term cannot be made sublinear when all one million
connected Users deliberately subscribe to the same hot tree: one million recipients
require one million deliveries, though repeated changes can be coalesced before each
recipient refetches.

**Candidate implication.** Direct all-gateway `LISTEN` can start small only with an
explicitly bounded `G`. If `G * total change rate` becomes material, the next
candidate is coarse subject/region routing—not a global revision. PostgreSQL channels
could be partition-scoped, but dynamic interest registrations and database sessions
must be load-tested; a broker relay becomes worthwhile precisely when it can route
only to `G_s` without moving authority.

### Hot Entity and slow consumers

**Evidence.** PostgreSQL folds identical notifications only within one transaction,
not across separate hot-Entity commits. NATS' documented response to a slow live
consumer is to protect the system and disconnect it rather than retain an unbounded
backlog. [`NOTIFY`](https://www.postgresql.org/docs/current/sql-notify.html),
[NATS slow consumers](https://docs.nats.io/running-a-nats-service/nats_admin/slow_consumers)

**Inference.** A gateway should not queue one item for every intermediate color or
shape change. It can hold a bounded dirty bit/latest known token for each active
resource stream. If another change arrives before the host refetches, the bit remains
dirty. A bounded slow consumer is disconnected or loses hints and later refetches;
World state is untouched.

**Candidate implication.** The first hot-subject proof should vary commit rate,
gateway count, subscribers per gateway, slow-consumer fraction and reconnect burst.
It should report both raw notification amplification and coalesced outbound hints.
Uniformly distributing changes over subjects would not test the relevant risk.

## Mechanism comparison

| Mechanism | Recovery and ordering | Multi-instance fan-out | Hot-subject/operational cost | KISS fit for disposable hints |
| --- | --- | --- | --- | --- |
| Post-commit `NOTIFY` plus current refetch | notification gap and post-commit reorder allowed; current state recovers | every listening gateway receives it | one DB notification per attempt per gateway; one listener connection per gateway; local coalescing | strongest first candidate, if explicitly lossy |
| In-transaction/trigger `NOTIFY` | announces commits in commit order; no replay after listener loss | every listening gateway receives it | notification queue can fail World commit; trigger hides publication but not queue coupling | simple code, wrong availability coupling for a disposable hint |
| Batched current-resource polling | latest state is exact; intermediate changes intentionally collapse | each gateway polls only unique local interests | bounded query/row cost proportional to active interests and interval | excellent recovery/fallback; expensive as high-frequency primary path |
| Global append-feed polling | requires a real no-gap cursor or overlap/dedup bound | every consumer keeps a cursor | global ordering/watermark temptation; repeated global reads | reject unless every-change replay is accepted gameplay |
| Transactional outbox plus relay | durable row, retry and dedup; per-key order can be designed | still needs broadcast transport after the relay | schema, writes, cleanup, claims, retry, lag and relay | only after eventual publication is a real invariant |
| Logical decoding/CDC | WAL replay; duplicates after crash; slot/failover operations | one consumer per slot, then a routing/broker tier | WAL retention, privileged slots, schema mapping and failover | projection infrastructure, not a first live hint path |
| Live sharded broker | lossy like the hint; current state recovers | routes to interested gateway subjects | another service; slow-client and reconnect operations | later substitute when direct PostgreSQL amplification is measured |
| Durable broker stream | acknowledgement/replay/redelivery | scalable routing with stateful consumers | second log plus consumer state and deduplication | avoid for per-Character hints; reserve for an accepted durable projection |

## Failure and overload matrix

| Failure | Authoritative result | Candidate recovery | Forbidden dependency |
| --- | --- | --- | --- |
| World process dies before commit | no accepted change | idempotent caller retry | notification claiming success |
| World process dies after commit before post-commit notify | state + Activity exist | next read or bounded reconciliation | rolling state back because hint was absent |
| listener starts amid writes | current state exists | commit `LISTEN`, then initial read | treating first received message as a complete baseline |
| listener/gateway disconnects | current state exists | re-listen, reconcile, external refetch | expecting SQLx reconnect to replay missed messages |
| duplicate or reordered hint | latest current state exists | coalesce and re-read | applying notification deltas in arrival order |
| notification queue fills | depends on publication seam | keep hint outside mutation or use accepted durable publication contract | disposable queue rejecting accepted gameplay invisibly |
| one gateway is slow | current state exists | bounded buffer, collapse dirty state, disconnect/refetch | unbounded per-connection history |
| one Entity is globally hot | ordered World writes remain subject-scoped | one gateway hint per dirty interval, bounded admission | one global board row/revision updated per change |
| PostgreSQL primary changes | survival follows database HA contract | reconnect/re-listen/reconcile; CDC needs synchronized failover slot | subscription transport claiming a stronger RPO than PostgreSQL |

## Evidence gates before any production choice

1. **Name the semantic promise.** Is a host promised only eventual current refresh,
   bounded-latency refresh while connected, or complete occurrence replay? No
   transport comparison is meaningful until this is explicit.
2. **Measure direct `LISTEN`/`NOTIFY`.** Use real PostgreSQL and SQLx `0.8.6`; vary
   World commits/second, payload cardinality, gateway listener count and one hot
   subject. Capture mutation latency, notify errors,
   `pg_notification_queue_usage`, DB CPU, listener lag and reconnect gaps.
3. **Prove coalescing.** One fast and one deliberately slow host subscribe to one hot
   resource. Memory and pending work stay bounded; both recover the latest state; no
   Agent is invoked.
4. **Prove quiet-subject isolation.** A hot tree may saturate hints without delaying
   an unrelated World mutation or bounded read beyond its declared budget.
5. **Compare board shapes.** Run identical load through one global channel, coarse
   region/board-partition routing and exact subject routing. Report
   `G * total rate`, `sum(lambda_s * G_s)`, outbound deliveries and subscription churn.
6. **Break every connection.** Drop the writer after commit, the listener during a
   burst, the gateway during fan-out and PostgreSQL during listener recovery. Every
   accepted state remains recoverable by an authoritative read.
7. **Earn the next layer.** Introduce an outbox, CDC or broker experiment only against
   the measured direct-path bottleneck; compare added failure modes and operations,
   not just throughput.

## Required decisions and no-go conclusions

Research cannot choose whether connected hosts need bounded-latency freshness or
mere next-turn freshness; whether every intermediate Activity must be observed;
which resources form one Agent's active interest; whether one logical game board is
current state, recent Activity or both; which stable region/subject key partitions a
board; how many direct listener processes are allowed; or when a missed hint should
trigger periodic reconciliation.

The evidence rejects:

- an Agent, Entity or external subscription owning a PostgreSQL connection;
- notification payloads as World truth, deltas, authorization or Agent knowledge;
- one global World/board revision, sequence, row or mandatory broadcast feed;
- one durable broker consumer or outbox cursor per Character;
- timestamp-plus-UUID presented as a no-gap commit cursor;
- unbounded gateway queues, reconnect scans or per-notification refetches;
- a database trigger whose disposable notification can unexpectedly become a World
  mutation availability dependency;
- one logical replication slot shared by competing independent consumers;
- a replication slot per gateway without explicit WAL/failover operations;
- adding a broker before direct all-gateway amplification is measured; and
- any “millions” claim that omits total write rate, gateway count, interest skew,
  slow clients, coalescing ratio, reconnect burst, database connection budget and
  failover recovery.

## Primary source index

Checked on 2026-08-16:

- PostgreSQL: [`LISTEN`](https://www.postgresql.org/docs/current/sql-listen.html),
  [`NOTIFY`](https://www.postgresql.org/docs/current/sql-notify.html),
  [notification queue usage](https://www.postgresql.org/docs/current/functions-info.html),
  [resource configuration](https://www.postgresql.org/docs/current/runtime-config-resource.html),
  [connections](https://www.postgresql.org/docs/current/runtime-config-connection.html),
  [`SELECT ... SKIP LOCKED`](https://www.postgresql.org/docs/current/sql-select.html),
  [logical decoding](https://www.postgresql.org/docs/current/logicaldecoding-explanation.html),
  [replication slots](https://www.postgresql.org/docs/current/warm-standby.html) and
  [logical failover](https://www.postgresql.org/docs/current/logical-replication-failover.html).
- SQLx: [`PgListener` 0.8.6](https://docs.rs/sqlx-postgres/0.8.6/sqlx_postgres/struct.PgListener.html).
- Debezium: [PostgreSQL connector](https://debezium.io/documentation/reference/stable/connectors/postgresql.html)
  and [outbox event router](https://debezium.io/documentation/reference/stable/transformations/outbox-event-router.html).
- NATS: [delivery model](https://docs.nats.io/nats-concepts/what-is-nats),
  [slow consumers](https://docs.nats.io/running-a-nats-service/nats_admin/slow_consumers)
  and [JetStream consumers](https://docs.nats.io/nats-concepts/jetstream/consumers).
- Redis: [Pub/Sub delivery and sharding](https://redis.io/docs/latest/develop/pubsub/).
