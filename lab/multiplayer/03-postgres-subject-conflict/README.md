---
question: Do affected-Entity locks preserve exact conflicts while every disjoint same-Place path is wholly independent from the Place row?
verdict: refuted
status: kept
real_seam: [Rust compiler, test runner, Tokio scheduling, SQLx pool and disposable-database machinery, local PostgreSQL server, SQL transactions, FOR UPDATE and foreign-key row locks, lock timeouts, current/absence rechecks, unique root/successor indexes, foreign keys, commit and rollback]
simulated_seam: [World operations, Character ownership and placement, input normalization, canonical fingerprinting, authorization, privacy, HTTP, MCP, Agents, LLMs, retry transport, production connection pooling, admission]
informs: .agents/plans/20260816-153410-multiplayer-lab/plan.md#t3--prove-one-unified-semantic-change-kernel
---

# PostgreSQL subject-conflict experiment

> **Role / side:** retained PostgreSQL subject-conflict experiment / development side.
> **Authority:** records this fixture, its bounded observations, verdict and artifact status.
> **Excludes:** current game behavior, production schema or transaction design, migration approval and scale proof.

## Pending decision

The preceding in-memory
[`02-subject-conflict`](../02-subject-conflict/README.md) fixture showed that exact
Property current/absence expectations can represent the intended conflict semantics
without one Place revision. This experiment tests the remaining PostgreSQL race and
lock question for T3 of the accepted
[`multiplayer-lab` plan](../../../.agents/plans/20260816-153410-multiplayer-lab/plan.md).
It informs but does not make the transaction decision owned by the active
[`concurrency-and-world-dynamics`](../../../docs/concept/concurrency-and-world-dynamics.md)
exploration.

## Falsifiable question

Can conservative affected-Entity locking plus an exact current/absence recheck make
same-Property mutations correct, while ordinary disjoint work at the same Place is
completely independent of the Place row—including a new Entity placement while that
Place row is held `FOR UPDATE`?

The question deliberately combines correctness and the strongest proposed
independence claim. The verdict is `refuted` if either fails. A narrower result may
still show which part remains useful.

## Hypothesis

Within the fixed scratch schema and bounds:

- an existing Door Property mutation and a new Entity introduction both complete
  while another transaction holds the Place row `FOR UPDATE`;
- holding the Stone Entity row prevents Stone state mutation but does not prevent
  Door state mutation;
- two concurrent mutations based on the same current Property tip yield exactly one
  accepted successor and one dependency conflict;
- two concurrent mutations expecting the same absent Property yield exactly one
  root/current pointer and one dependency conflict; and
- accepted retry identity and transaction rollback create neither duplicate nor
  partial Activity/state.

## Fixture

The standalone Rust 2024 crate contains one scratch migration and one test module:

- [`migration/0001_subject_conflict.sql`](migration/0001_subject_conflict.sql)
  creates one Place, placed Entity rows with a real foreign key to that Place, one
  immutable-style Activity footprint per accepted request, predecessor-linked
  Property history and one current pointer;
- [`src/lib.rs`](src/lib.rs) implements only `introduce_entity` and one-Property
  submission helpers needed by the tests;
- affected state mutation first locks its exact Entity row `FOR UPDATE`, then
  rechecks `Current(activity_id)` or `Absent` before writing Activity, history and
  current state in one transaction; and
- PostgreSQL root/successor uniqueness is an independent database backstop.

Run from the repository root:

```sh
DATABASE_URL=postgres://localhost/postgres \
  cargo test \
  --manifest-path lab/multiplayer/03-postgres-subject-conflict/Cargo.toml \
  -- --test-threads=1
```

After that successful run, verify SQLx's disposable-database registry explicitly:

```sh
DATABASE_URL=postgres://localhost/postgres \
  cargo test \
  --manifest-path lab/multiplayer/03-postgres-subject-conflict/Cargo.toml \
  audit_sqlx_database_cleanup -- --ignored --test-threads=1
```

## Exact bounds

- Token, model and Agent calls: zero.
- External services: one local PostgreSQL server reached through
  `postgres://localhost/postgres`; no HTTP, MCP, broker or remote network service.
- Tests: five focused `#[sqlx::test]` cases plus one separately invoked cleanup
  audit.
- Isolation: SQLx creates one uniquely named disposable database per focused test,
  applies only this crate's scratch migration and drops that database after success.
- Pool: SQLx's test pool limit is at most five connections per test.
- Time: each candidate transaction sets PostgreSQL `lock_timeout` to 150 ms; every
  concurrent assertion has a two-second Tokio timeout.
- Fixture subjects: one Place, one seeded Stone, one seeded Door, one seeded current
  Property on each and at most two new Entities or two competing requests per test.
- Mutation size: exactly one Property per state request; no multi-Entity transaction.
- Identity: every new request, Activity and Entity uses a fresh UUID; fixed UUIDs
  exist only for the isolated seeded fixture.
- Scheduling: Tokio runs actual concurrent futures against separate PostgreSQL pool
  connections; this is not a load or duration test.

## Real and simulated seams

Real seams exercised here are the Rust compiler and test runner, Tokio scheduling,
SQLx pool and disposable-database machinery, a local PostgreSQL server, SQL
transactions, `FOR UPDATE`/foreign-key row locks, lock timeouts, current/absence
rechecks, unique root/successor indexes, foreign keys, commit and rollback.

The schema is a reduced experimental schema, not the production Aicadia migrations.
World operations, Character ownership and placement, input normalization, canonical
fingerprinting, authorization, privacy, HTTP, MCP, Agents, LLMs, retry transport,
production connection pooling and admission are simulated or absent. Consequently
these tests prove only the exact scratch SQL paths and fixtures they execute.

## Observations

All five focused tests passed, and the separate cleanup audit passed.

1. While another transaction held `place FOR UPDATE`, an existing Door Property
   change committed within the bound. That path locked the Door Entity and rechecked
   its Property state under that coordinator; it did not lock the Place row.
2. Under the same held Place lock, a new Entity introduction did **not** commit. Its
   real placement foreign-key check requested a PostgreSQL `KEY SHARE` row lock on
   the referenced Place, conflicted with `FOR UPDATE` and stopped at SQLSTATE
   `55P03` after the 150 ms bound. It left no Activity or Entity.
3. After removing that exclusive Place blocker, two normal same-Place Entity
   introductions completed concurrently. Their foreign-key `KEY SHARE` locks were
   compatible with each other. The result rejects “no Place-row touch,” not
   same-Place introduction parallelism in the absence of an exclusive Place writer.
4. Holding only the Stone Entity row caused a Stone mutation to stop at the bounded
   lock timeout while a Door mutation at the same Place committed. After the blocker
   rolled back, the previously unaccepted Stone request id could be submitted and
   accepted normally.
5. Two requests carrying the same current Stone Property tip produced one accepted
   successor, one dependency conflict, one successor history row and one request
   Activity.
6. Two requests expecting the same absent Stone Property produced one accepted
   root, one current pointer, one dependency conflict and one request Activity.
7. An equal accepted request replay returned the original Activity and a changed
   fingerprint conflicted without another write. A deliberately constraint-failed
   request rolled back its already-inserted Activity and all Property state; a later
   corrected submission with that still-unaccepted request id succeeded.
8. SQLx removed all five disposable databases after the successful tests. The
   separate audit found zero matching entries in `_sqlx_test.databases`.

## Why the combined hypothesis is refuted

Same-Property correctness and different-Entity isolation were supported, but the
strong statement that Entity introduction is wholly independent of the Place row
was false in the honest foreign-key schema. PostgreSQL must protect the referenced
Place key while inserting placement. Removing the explicit semantic Place lock and
mutable revision update is therefore different from claiming that PostgreSQL never
touches the Place row.

The useful narrower candidate is:

- ordinary Property work on existing Entities needs no Place row lock or update;
- normal introductions may proceed together because their foreign-key locks are
  compatible;
- an old or concurrent `FOR UPDATE` Place writer still blocks introductions; and
- a production transition cannot safely leave old exclusive Place writers active
  while claiming that all new subject-scoped paths are independent.

This experiment does not decide whether a future Place operation can use a narrower
lock mode, whether placement storage should change or how the production migration
must sequence old and new writers.

## Falsifier and follow-up boundary

The `refuted` verdict should be corrected only if the exact honest schema and held
`FOR UPDATE` fixture can reproducibly let introduction commit without removing or
weakening its Place-integrity check. A different schema, weaker blocker or missing
foreign key answers another question.

The supported sub-results become inconclusive if a repeated run shows two accepted
successors from one current tip, two accepted roots from one absence, cross-Entity
blocking, duplicate retry effects, a partial failed transaction or leaked SQLx test
databases.

## Artifact status and non-claims

**Artifact status: `kept`.** The failed strong hypothesis is useful: it prevents a
future plan from equating “no semantic Place conflict unit” with “no database lock
of any mode on a referenced Place.” The crate remains non-authoritative and must not
be imported, copied or promoted into production.

This experiment does not prove production Aicadia correctness, million-User
capacity, throughput, latency distribution, fairness, starvation resistance,
arbitrary deadlock freedom, crash recovery, multi-Entity atomicity, movement,
subscription, Activity ordering, Agent dependency selection or a safe migration.
It also does not prove that conservative Entity locking is the final hot-Entity
granularity; different Property keys on one Entity were deliberately not claimed to
run physically in parallel.

## Downstream implication

T3 can retain conservative affected-Entity locking plus exact current/absence
rechecks as a technically credible first candidate. The production plan must remove
or sequence every exclusive Place writer before claiming normal same-Place
introductions are independent, and it must describe unavoidable integrity lock modes
separately from gameplay conflict semantics. No new public contract follows from
this lab alone.
