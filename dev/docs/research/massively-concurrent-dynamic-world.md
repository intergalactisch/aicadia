---
status: pending
era: August Activity-Property-Trait
---

# Massively concurrent dynamic World architecture

> **Role / side:** sourced research report / development side.
> **Authority:** records this report's question, evidence, inferences and candidate implications.
> **Excludes:** product decisions and current implementation contracts; see `game/docs/`.

Date: 2026-08-16

Status: research; no architecture or game direction below is accepted Aicadia behavior

## Question and evidence boundary

How can Aicadia remain correct, bounded and interesting when millions of Users
discover one sparse shared World, including a crowd observing or acting on the same
Entity or Place?

“Millions” must eventually name connected Users, request rate, read/write mix,
hot-subject skew, burst length, latency, allowed rejection, history horizon and
recovery objectives. No current load evidence supports such a production claim.

This report uses PostgreSQL and other first-party sources plus a local code audit.
**Evidence** is cited fact; **inference** is analysis; **candidate implication** is an
unaccepted Aicadia direction requiring contract choice, plan acceptance and proof.

## Core result

**Evidence.** PostgreSQL MVCC lets reads use snapshots while transactions write, but
writers locking the same row wait; deadlocks remain possible and Serializable
transactions may need whole-transaction retry. [MVCC](https://www.postgresql.org/docs/current/mvcc-intro.html),
[locking](https://www.postgresql.org/docs/current/explicit-locking.html),
[isolation](https://www.postgresql.org/docs/current/transaction-iso.html)

**Inference.** Millions of Users spread across independent subjects are a scale-out
problem. Millions of contradictory mutations to one canonical fact are an ordering
and admission problem. No shard, cache, CRDT or fleet makes all incompatible values
simultaneously true.

**Candidate implication.** “Handles the crowd” should mean every request gets a
bounded accepted, conflicted, busy or rejected result without duplicate history,
split current state, indefinite wait or retry collapse—not that every intent succeeds.

## Current Aicadia audit

| Evidence | Established strength | Concrete scale risk |
| --- | --- | --- |
| Contextual writers lock User then Place and advance `place.latest_activity_id` ([storage](../../../game/docs/storage.md#postgresql-model-and-indexes), [code](../../../game/src/world/common.rs#L30-L42)) | Deterministic exact-Place freshness; unrelated Places can write concurrently | Every writer at one Place shares one mutation lane |
| Property/Trait pointers use stable lock order ([storage](../../../game/docs/storage.md#postgresql-model-and-indexes)) | Same-lineage conflict and deadlock ownership are scoped | The preceding Place lock is broader than those pointer conflicts |
| Trait-bearing Action fetches all locally eligible Entity ids ([caller](../../../game/src/world/mutation.rs#L369-L379), [query](../../../game/src/world/common.rs#L102-L143)) | Authoritative eligibility | Work and Rust memory grow with Place occupancy despite bounded submitted input |
| Personal history, local Entity and privacy-filtered Place history return keyset pages up to 100 ([reads](../../../game/src/world/read.rs#L161-L430)) | Output and page size are bounded | `UNION`, visibility and sorting work are not proven independent of total occupancy/history |
| The local pool has ten connections ([startup](../../../studio/src/bin/aicadia.rs#L23-L30)) | Connections are finite | Hot-Place lock waiters can occupy the pool and starve quiet work |
| Investigation has per-User admission and a bounded Place-history tail ([storage](../../../game/docs/storage.md#postgresql-model-and-indexes)) | Retry and chance work are bounded locally | No accepted Place/target overload contract covers a many-User burst |
| Additional Places, movement, clocks and background simulation are absent ([deferred](../../../game/docs/deferred.md#absent)) | No speculative scale machinery | One entry Place is not evidence for a distributed living World |

The Place lock is a correct simple MVP mechanism. Measurement must decide when a
narrower public freshness and serialization contract earns its complexity.

## Architecture findings

### Conflict follows causality

**Evidence.** Row locks block writers of the same row, not ordinary readers. SSI can
preserve serializable outcomes without a global lock, but serialization failures
still require retry. Advisory-lock meaning is application-owned and session locks can
outlive transaction rollback. [Locking](https://www.postgresql.org/docs/current/explicit-locking.html),
[SSI paper](https://www.vldb.org/pvldb/vol5/p1850_danrkports_vldb2012.pdf)

**Candidate implication.** Classify hot-subject work before choosing a mechanism:

| Intent | Smallest candidate owner | Bounded semantic result |
| --- | --- | --- |
| Observe | revision-labelled Entity/Place projection | explicit writer or bounded-stale source |
| Append Activity without target-state change | request id plus exact context dependencies | independent append only if Place freshness deliberately narrows |
| Change independent state | submitted Property/Trait pointers in stable order | unrelated lineages remain concurrent |
| Change the same pointer | pointer lock or compare-and-swap | one order/winner; typed conflict for losers |
| Reveal one unknown | stable transition key and unique constraint | concurrent observers return one shared winner |
| Inform a million Characters | bounded pull/interest lens | no synchronous fan-out or Agent wake-up |

Absence dependencies also need an owner: “Property X did not exist” cannot be safely
validated by checking it before the transaction and then locking an unrelated row.

### Hot rows and bounded queries

**Evidence.** HOT can avoid new index entries only when indexed values do not change
and the page has room. Old tuple versions still need vacuum. Multicolumn B-tree scans
are most efficient when leading columns are constrained. [HOT](https://www.postgresql.org/docs/current/storage-hot.html),
[vacuum](https://www.postgresql.org/docs/current/routine-vacuuming.html),
[multicolumn indexes](https://www.postgresql.org/docs/current/indexes-multicolumn.html)

**Inference.** HOT may reduce index churn for `place.latest_activity_id`; it does not
remove its row lock, WAL, dead tuples or vacuum pressure. `LIMIT 100` bounds output,
not work done before finding those rows.

**Candidate implication.** For hot writers measure lock wait, WAL, dead tuples and
vacuum lag. For each player read, require adversarial `EXPLAIN (ANALYZE, BUFFERS)`
evidence that rows/buffers stay proportional to `limit` plus one declared bound.
Validate only submitted bounded Trait ids if the game semantics permit it.

### Idempotency and overload

**Evidence.** AWS separates caller intent from equal parameters and uses a client
request id for safe retry. PostgreSQL `ON CONFLICT` atomically selects an alternative
under a uniqueness conflict. Google SRE warns that queues and retries amplify
overload; load shedding preserves useful work. `NOWAIT` fails instead of waiting,
while `SKIP LOCKED` gives an inconsistent view intended for queue-like consumers.
[AWS](https://aws.amazon.com/builders-library/making-retries-safe-with-idempotent-APIs/),
[`INSERT`](https://www.postgresql.org/docs/current/sql-insert.html),
[Google SRE](https://sre.google/sre-book/addressing-cascading-failures/),
[`SELECT`](https://www.postgresql.org/docs/current/sql-select.html#SQL-FOR-UPDATE-SHARE)

**Inference.** Request identity prevents duplicate acceptance after uncertainty; it
does not protect against a storm of distinct valid ids. A blocked database connection
also steals capacity from independent subjects.

**Candidate implication.** Keep canonical mutation identity at the writer. Bound
body/cardinality, server time, pool acquisition, lock/statement time, retry budget
and instance concurrency; add per-User/subject admission only when fairness semantics
are explicit. A busy rejection writes no World history. Never use `SKIP LOCKED` to
make a canonical subject look absent. Raising connection counts alone can multiply
memory and contention. [Connections](https://www.postgresql.org/docs/current/runtime-config-connection.html),
[resources](https://www.postgresql.org/docs/current/runtime-config-resource.html)

### Partitioning, replicas and CDC

**Evidence.** Partitioning supports pruning and cheap bulk maintenance, but too many
surviving partitions increase planning/memory cost. Unique/primary constraints must
include every partition key; moving a row between partitions can yield serialization
failure. [Partitioning](https://www.postgresql.org/docs/current/ddl-partitioning.html)
Spanner separately warns against monotonically increasing leading keys because they
hotspot. [Spanner](https://docs.cloud.google.com/spanner/docs/schema-design#choose_a_primary_key_to_prevent_hotspots)

**Inference.** Time-partitioning Activity can complicate global `(user, request_id)`
uniqueness. Place partitioning leaves one hot Place together. Hashing distributes
many Places, not one row every writer updates.

**Evidence.** Streaming replication is asynchronous by default. Hot standby is
read-only, can lag and may cancel conflicting queries. Logical slots retain WAL and
can redeliver recent changes after crash; logical replication does not copy DDL,
sequence state or large objects. [Streaming](https://www.postgresql.org/docs/current/warm-standby.html),
[standby](https://www.postgresql.org/docs/current/hot-standby.html),
[decoding](https://www.postgresql.org/docs/current/logicaldecoding-explanation.html),
[restrictions](https://www.postgresql.org/docs/current/logical-replication-restrictions.html)
`LISTEN`/`NOTIFY` is a commit notification with a setup race, not a replayable log.
[`LISTEN`](https://www.postgresql.org/docs/current/sql-listen.html),
[`NOTIFY`](https://www.postgresql.org/docs/current/sql-notify.html)

**Candidate implication.** Label reads `writer-required`, `bounded-stale` or
`immutable-history`. Replicas may serve only the latter two; mutation preconditions
return to the writer. CDC may build idempotent, rebuildable, privacy-filtered cache,
map or search views with monitored lag/WAL. Earn partitioning only from measured
pruning, size, maintenance or distribution pressure, after proving uniqueness,
foreign keys, history, movement and repartitioning. Operational cells never become
Place identity.

### A living World is sparse and interest-bounded

**Evidence.** Evennia derives on-demand stages from stored starts and thresholds,
but requires ticking when skipped intermediate stages have side effects. Unreal
separates per-Actor relevancy from a Replication Graph that reuses replication lists
at large actor/connection counts. [Evennia](https://www.evennia.com/docs/latest/Components/OnDemandHandler.html),
[relevancy](https://dev.epicgames.com/documentation/en-us/unreal-engine/actor-relevancy-in-unreal-engine),
[Replication Graph](https://dev.epicgames.com/documentation/en-us/unreal-engine/replication-graph-in-unreal-engine)
Aicadia's research likewise separates clock from work and observation from state.
[World time](world-time-and-sparse-simulation.md#reusable-design-patterns-and-trade-offs),
[locality](locality-co-presence-and-observation.md#implications-for-an-open-ended-agentic-shared-world)

**Inference.** Richness comes from durable shared consequences, unknown space,
local detail, return-visible change and causal history—not simulating every potential
plant, NPC or second. A million observers should not create a million copies or pushes.

**Candidate implication.** Add one concrete mechanic at a time using exactly one of:

1. stable procedural context keyed by subject, rule version, position and time bucket;
2. generated-once shared materialization with a stable transition key;
3. on-demand stage/deadline derivation when skipped stages have no effects; or
4. bounded catch-up with a monotonic watermark when absence changes stored state.

Relevancy stays capability-specific: presence, sight, interaction, history and map
delivery need not share one universal neighborhood. No form may activate an Agent.

## Phased evidence gates

1. **Define the claim.** Fix load, skew, burst, latency, rejection, RPO/RTO and the
   observe/append/mutate/reveal/fan-out conflict matrix. No shard choice yet.
2. **Baseline adversarially.** Compare one hot subject with independent subjects
   while scaling occupancy/history. Capture p50/p95/p99, pool/lock waits, outcomes,
   rows/buffers, CPU/I/O, WAL, dead tuples and vacuum. No uniform-load “million” claim.
3. **Close amplification and overload.** Correct proven query shapes and wait budgets.
   Exit only when the hot subject cannot starve a quiet control. No larger pool,
   unbounded queue, skipped lock or silent drop as the answer.
4. **Narrow one semantic conflict.** For one capability, prove scoped dependencies,
   same-pointer conflict, independent concurrency, absence, deadlock order, retry and
   rollback. No generic revision, last-write-wins or CRDT without a game merge law.
5. **Prove one sparse loop.** Specify clock, transition key, rule version, inactive
   semantics, randomness and history. Cross restart, long silence, deployment and
   concurrent first observation. Work must follow touched state; no reroll or Agent call.
6. **Offload selected reads.** Introduce the smallest earned replica/cache/projection;
   test lag, duplicate change, outage, WAL retention, rebuild, failover and privacy.
7. **Partition, then shard only if earned.** Rehearse uniqueness, foreign keys,
   movement, vacuum, restore and repartitioning. Sharding begins only after the single
   writer remains proven limiting; any World instance must still route a request.

## Required decisions and no-go conclusions

Before “millions” is credible, decide: what the number means; allowed busy behavior;
which hot intents change target state; which require total order; exact dependency
and absence scopes; bounded-query proof; retry/backoff; stale-read classes;
read-your-write; projection rebuild/lag limits; partition-key uniqueness; movement
across partitions; discovery transition identity; clock/rule/randomness after long
silence; returning-Character context; relevancy per capability; privacy in derived
views; and the measured threshold earning each new infrastructure layer.

Hard no-go's are: global World lock/revision/counter/tick/feed; process-local
correctness; unbounded waits or queues; background Agent/LLM work; fresh procedural
truth per observer; cache/replica/CDC authority; `SKIP LOCKED` canonical state;
semantic identity derived from cell/shard/host; speculative event sourcing, rule
engine, CRDT or simulation framework; and any scale claim without skewed overload,
failover and recovery evidence.

## Primary source index

- PostgreSQL: [MVCC](https://www.postgresql.org/docs/current/mvcc-intro.html),
  [locking](https://www.postgresql.org/docs/current/explicit-locking.html),
  [isolation](https://www.postgresql.org/docs/current/transaction-iso.html),
  [partitioning](https://www.postgresql.org/docs/current/ddl-partitioning.html),
  [indexes](https://www.postgresql.org/docs/current/indexes-multicolumn.html),
  [HOT](https://www.postgresql.org/docs/current/storage-hot.html),
  [vacuum](https://www.postgresql.org/docs/current/routine-vacuuming.html),
  [replication](https://www.postgresql.org/docs/current/warm-standby.html),
  [logical decoding](https://www.postgresql.org/docs/current/logicaldecoding.html) and
  [logical restrictions](https://www.postgresql.org/docs/current/logical-replication-restrictions.html).
- [PostgreSQL SSI paper](https://www.vldb.org/pvldb/vol5/p1850_danrkports_vldb2012.pdf),
  [Google SRE overload](https://sre.google/sre-book/addressing-cascading-failures/),
  [Spanner key design](https://docs.cloud.google.com/spanner/docs/schema-design),
  [AWS idempotent retries](https://aws.amazon.com/builders-library/making-retries-safe-with-idempotent-APIs/),
  [Evennia on-demand state](https://www.evennia.com/docs/latest/Components/OnDemandHandler.html)
  and [Unreal Replication Graph](https://dev.epicgames.com/documentation/en-us/unreal-engine/replication-graph-in-unreal-engine).
