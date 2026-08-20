---
question: Can the candidate PostgreSQL Position lineage and carrier lock boundary survive the fixed eight-case fixture?
verdict: supported
status: kept
real_seam: [Rust 2024, Tokio, SQLx disposable databases, PostgreSQL 17, READ COMMITTED, recursive SQL, row locks, foreign-key locks, uniqueness, commit and rollback, checked i128 arithmetic]
simulated_seam: [production World, production migrations, authorization and privacy, HTTP and MCP, Agents and LLMs, PostGIS and map indexes, Area, Connection, Relation, hosted operation, production load]
informs: dev/docs/concept/spatial-five-year-backcast.md#technical-synthesis-after-the-completed-grill
---

# PostgreSQL Position lineage

> **Role / side:** retained PostgreSQL spatial experiment / development side.
> **Authority:** owns the fixture, observations, bounded verdict and reproduction commands for the fixed Position-lineage experiment.
> **Excludes:** accepted spatial behavior, production schema, public capabilities and delivery proof; those remain in `game/docs/`, a later accepted production plan and `dev/docs/evidence/`.

## Question

Can one scratch PostgreSQL model resolve direct and Entity-relative Position exactly,
prevent a concurrent reference cycle, move a carrier without descendant writes and
isolate carrier-local work while keeping Position and Activity atomic?

The question is deliberately narrower than Aicadia's complete spatial system. The
current recommendation that this experiment informs is defined in the [spatial
technical synthesis](../../../docs/concept/spatial-five-year-backcast.md#technical-synthesis-after-the-completed-grill).

## Fixed fixture

The standalone Rust 2024 crate uses SQLx disposable databases, Tokio and local
PostgreSQL 17 under `READ COMMITTED`. Its scratch schema contains only stable Entity
coordinator rows, immutable Activity, immutable Position versions with one current
pointer and the minimum local-state lineage needed for the Cabin test.

The fixed bounds are:

- exactly eight focused cases and one separately invoked ignored cleanup audit;
- at most five SQLx pool connections per case;
- two actors in each race;
- a 64-level relative chain and exactly 1,000 carrier descendants;
- 150 ms transaction-local lock and statement timeouts;
- a two-second outer concurrency bound; and
- whole-centimetre `BIGINT` storage within a lab-local symmetric
  `±9,000,000,000,000,000` bound, resolved with checked Rust `i128` addition.

## Observed matrix

| Case | Authoritative readback | Observation |
| --- | --- | --- |
| Direct and nested resolution | 65 Entities, Positions and Activities; the 64-level endpoint resolved to `(164, 328, 492)` | Supported exact accumulation without a fixed depth of 32. |
| Symmetric coordinate failure | Both positive and negative proposals returned `OutOfBounds`; request Activity, child version and child current rows were all zero | Supported validation before durable acceptance. |
| Concurrent A→B and B→A | Exactly one request Activity, three total Position versions across A and B and exactly one changed current pointer; both final chains were acyclic | Supported ordered Entity locks plus revision recheck for this synchronized race. |
| Moving Ship | Counts changed from 1,001 to 1,002 Position versions and Activities; descendants retained exactly 1,000 seed versions and zero move Activities | Supported one canonical Ship write; sampled descendants resolved through the new Ship point. |
| Cabin-local isolation | Cabin local state committed while Ship held its writer lock; external Cabin resolution blocked with `57014`, then resolved to `(1010, 2020, 3030)` after Ship committed | Supported independence only for work that does not need the external World point. |
| Lock compatibility | A relative child insert committed while Ship held `FOR NO KEY UPDATE`; a Ship writer blocked under `FOR SHARE` and wrote no Activity | Supported the exact foreign-key and dependency-lock participation in this schema. |
| Retry identity | The exact retry returned its original Activity; changed fingerprint conflicted; one request Activity and two total Entity Position versions remained | Supported sequential idempotent replay without duplicate history. |
| Injected failure | Constraint SQLSTATE `23514`; zero request Activities, one unchanged seed version and pointer, zero local-state rows | Supported transaction-wide rollback after Activity insertion. |

All eight focused cases passed. The ignored SQLx cleanup audit independently passed
and found no registered disposable database left behind.

## Timeout observation

Both deliberate lock waits returned PostgreSQL `57014`. Because `lock_timeout` and
`statement_timeout` were both 150 ms, the statement timeout won before a distinct
`55P03` lock-timeout result. The fixture therefore demonstrates a real bounded lock
wait and the intended lock conflict, but it does not choose or prove a production
error code or player-facing retry contract.

## Verdict

**Supported inside the fixed scratch fixture.** The candidate Position lineage,
checked resolution, ordered Entity lock/recheck protocol, atomic history and
carrier-local conflict boundary survived every planned falsifier on real local
PostgreSQL 17.

This verdict does not prove the production World or migration, authorization,
privacy, API/MCP/Agent behavior, typed production Activity dependencies, concurrent
first-Position retries, PostGIS or map indexing, failover, throughput, latency under
load or million-User capacity. The 1,000 descendants falsify descendant write
amplification in this operation; they are not a load test. Lab code may not be
imported, copied or promoted into the runtime.

## Reproduce

Run the focused suite:

```sh
DATABASE_URL=postgres://localhost/postgres cargo test --manifest-path dev/lab/spatial/02-postgres-position-lineage/Cargo.toml -- --test-threads=1
```

Then verify disposable database cleanup separately:

```sh
DATABASE_URL=postgres://localhost/postgres cargo test --manifest-path dev/lab/spatial/02-postgres-position-lineage/Cargo.toml audit_sqlx_database_cleanup -- --ignored --test-threads=1
```

The observed focused result was `8 passed; 0 failed; 1 ignored`; the cleanup result
was `1 passed; 0 failed`.
