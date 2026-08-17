---
question: Which bounded coordinator preserves operation-scoped present, absent and mixed dependencies without false Place- or lock-class conflicts?
verdict: refuted
status: kept
real_seam: [Rust/Tokio execution, SQLx pools and disposable databases, local PostgreSQL, READ COMMITTED, SERIALIZABLE SSI, FOR UPDATE and foreign-key row locks, controlled concurrent connections, lock timeout SQLSTATE 55P03, serialization SQLSTATE 40001, uniqueness constraints, commit and rollback]
simulated_seam: [World authority, User and Character authority, placement-version checks, normalized public input, request fingerprints, retry admission, HTTP, MCP, Agents, LLMs, hosted pooling, load distribution, operations]
informs: .agents/plans/20260816-153410-multiplayer-lab/plan.md#t3--prove-one-unified-semantic-change-kernel
---

# PostgreSQL conflict-strategy comparison

> **Role / side:** retained comparative PostgreSQL experiment / development side.
> **Authority:** records this fixture, its bounded matrix, verdict and artifact status.
> **Excludes:** current game behavior, a production transaction/schema choice, migration approval and scale proof.

## Pending decision

Multiplayer Q5 accepts operation-scoped freshness: unrelated same-Place change does
not conflict, while a changed bounded causal dependency or mandatory World
invariant does. Earlier experiment 03 tested one conservative Entity coordinator,
not several implementation forms. This follow-up compares credible coordinators
without reopening the accepted semantics.

It informs T3 of the accepted
[`multiplayer-lab` plan](../../../.agents/plans/20260816-153410-multiplayer-lab/plan.md)
and the active
[`concurrency-and-world-dynamics`](../../../docs/concept/concurrency-and-world-dynamics.md)
exploration. It cannot choose production behavior or schema by itself.

## Falsifiable question

Can four `READ COMMITTED` coordination strategies preserve the same current,
expected-absence and cross-causal outcomes, accept semantically independent mixed
present/absent work and exhibit bounded lock radii? Can one controlled PostgreSQL
`SERIALIZABLE` challenger prevent a bidirectional cross-causal write skew without
explicit dependency locks?

The hypothesis is supported only if every fully exercised strategy produces one
accepted request, one dependency conflict and one Activity for each true conflict;
blocked work remains bounded and leaves no Activity; and narrower strategies allow
the exact independent work claimed below.

## Compared setups

All explicit-lock variants use one shared operation-scoped dependency and writer
implementation. Only the coordinator changes:

| Setup | Coordinator | Intended comparison role |
| --- | --- | --- |
| Place control | Lock each involved Place row, then recheck exact dependencies | Negative control for false same-Place contention; not a product candidate after Q5 |
| Entity | Lock each involved Entity row in stable id order | Small conservative candidate, including expected absence and multi-Entity reads |
| Hybrid current/absence | Lock existing `(Entity, key)` current rows; lock an Entity for expected absence | Finer first candidate whose mixed lock classes are deliberately adversarially tested |
| Exact Property slot | Ensure and lock one stable `(Entity, key)` row with nullable current Activity for both present and absent facts | Uniform total-order candidate that spends one persistent slot concept to avoid the hybrid lock-class cycle |
| `SERIALIZABLE` challenger | No explicit dependency coordinator in the one exercised cross-causal case; PostgreSQL SSI detects write skew | A meaningful isolation-setting challenger, not a fully evaluated candidate |

The comparison order is correctness, atomicity, bounded false contention, bounded
work and conceptual size. It is not requests-per-second ranking.

## Fixture and run commands

The standalone Rust 2024 crate has one scratch migration and one canonical test
model:

- [`migration/0001_conflict_strategies.sql`](migration/0001_conflict_strategies.sql)
  creates one honest Place-to-Entity foreign key, immutable-style Property history,
  one current pointer, one optional-current Property slot and one Activity per
  accepted request;
- [`src/lib.rs`](src/lib.rs) implements the four explicit coordinators, one shared
  dependency validator/writer and the bounded serializable write-skew challenger;
- Great Stone starts with `state=standing` and `color=gray`; Quarry Door starts
  `state=open`; and
- dependencies are sorted by Entity id and Property key before any coordinator
  lock is acquired.

Run all focused tests from the repository root:

```sh
DATABASE_URL=postgres://localhost/postgres \
  cargo test \
  --manifest-path lab/multiplayer/04-postgres-conflict-strategies/Cargo.toml \
  -- --test-threads=1
```

Then verify SQLx cleanup separately:

```sh
DATABASE_URL=postgres://localhost/postgres \
  cargo test \
  --manifest-path lab/multiplayer/04-postgres-conflict-strategies/Cargo.toml \
  audit_sqlx_database_cleanup -- --ignored --test-threads=1
```

## Exact bounds and settings

- Token, model and Agent calls: zero.
- External services: one local PostgreSQL server; no HTTP, MCP, broker or remote
  service.
- Focused tests: ten, plus one separately invoked ignored cleanup audit.
- Databases: one SQLx-created disposable database per focused test, using only this
  scratch migration.
- Pool: SQLx's test pool limit is at most five connections per test.
- Explicit-lock isolation: PostgreSQL default `READ COMMITTED`.
- Challenger isolation: explicit PostgreSQL `SERIALIZABLE` for one two-transaction
  cross-causal case, with zero automatic retries.
- Lock bound: fixed `150 ms`; concurrent assertion bound: fixed two seconds.
- Subjects: one Place, one Stone and one Door.
- Correctness case per full strategy: two same-current requests, two same-absence
  requests and two requests under one explicit bidirectional clear-door invariant.
- Mixed-lock cases: two otherwise independent two-dependency requests force both
  hybrid lock classes, then repeat through exact Property slots.
- Mutation size: at most two dependencies and one Property write per request.
- No warm-up, duration run, throughput measure, percentile, pool sweep, retry sweep
  or PostgreSQL server tuning.

The fixed timeout distinguishes blocked from independent work inside this fixture;
it is not a proposed player-facing latency contract.

## Real and simulated seams

Real seams are Rust/Tokio execution, SQLx pools and disposable databases, local
PostgreSQL, `READ COMMITTED`, `SERIALIZABLE` SSI, actual `FOR UPDATE` and foreign-key
row locks, controlled concurrent connections, lock timeout SQLSTATE `55P03`,
serialization SQLSTATE `40001`, uniqueness constraints, commit and rollback.

The schema is reduced and independent from Aicadia's production migrations. World,
User and Character authority, placement-version checks, normalized public input,
request fingerprints, retry admission, HTTP, MCP, Agents, LLMs, hosted pooling,
load distribution and operations are simulated or absent. Activity id equals the
request id only as a fixture convenience.

## Observed matrix

All ten focused tests passed. The separate cleanup audit passed.

| Scenario | Place control | Entity | Hybrid | Exact slot | `SERIALIZABLE` challenger |
| --- | --- | --- | --- | --- | --- |
| Same current Property | 1 accept, 1 dependency conflict, 1 Activity | same | same | same | not tested |
| Same expected-absent Property | 1 accept, 1 dependency conflict, 1 Activity | same | same through Entity fallback | same through one created slot | not tested |
| Bidirectional Stone-clear/Door-open invariant | 1 accept, 1 dependency conflict, 1 Activity | same | same | same | 1 commit, 1 `40001`, 1 Activity |
| Independent mixed `Absent`/`Current` requests | not separately tested | not separately tested | at least one bounded `55P03` or `40P01`; both cannot accept | both accept, 2 Activities | not tested |
| Held Place row versus Door write | bounded `55P03` | not applicable | not applicable | not applicable | not tested |
| Held Stone Entity versus Stone/Door work | not separately tested | Stone blocked; Door succeeds | an Entity fallback interval can block Stone history through its FK; Door succeeds | not separately tested; the strategy itself acquires no Entity coordinator | not tested |
| Held exact `Stone.state` row | Place-wide coordinator | Entity-wide coordinator | `Stone.state` current blocks while `Stone.color` and Door succeed | `Stone.state` slot blocks while `Stone.color` and Door succeed | not tested |
| Forced write failure after Activity insert | shared path, not separately injected | shared path, not separately injected | full rollback directly exercised | shared path, not separately injected | not tested |

The matrix refutes the hybrid as a complete coordinator. Ordinary present
dependencies are exact-current scoped, but expected absence falls back to an Entity
lock. Two requests that combine those lock classes in opposite Entity/key patterns
both validate yet form a cycle through later history foreign-key `KEY SHARE` locks.
With the fixed bound, at least one semantically independent request aborts. Sorting
dependencies alone cannot impose one total order across Entity rows, current rows
and implicit foreign-key locks.

The exact slot variant represents present and absent with the same stable
`(Entity, key)` row, creates a missing slot with `ON CONFLICT DO NOTHING`, then locks
all slots in Entity/key order. It accepted both mixed requests and retained exact-key
contention for existing state. The cost is a persistent Property-slot concept and a
nullable current pointer that production Aicadia does not currently have.

The serializable result is deliberately partial. PostgreSQL SSI prevented the
controlled bidirectional Stone/Door write skew, but this experiment did not test same-current,
expected absence, retry budgets, abort storms or quiet-subject pool isolation under
`SERIALIZABLE`. It is a credible later challenger, not the current winner.

## Verdict

**Verdict: `refuted`.** The initial hybrid-is-smallest hypothesis fails inside the
expanded scratch matrix. All four explicit coordinators preserve the basic
same-current, same-absence and bidirectional invariant cases, but their mixed
dependency and false-contention behavior differs materially:

1. Place coordination is correct but needlessly blocks every same-Place subject.
2. Entity coordination is correct and simple but blocks independent keys on one hot
   Entity.
3. Hybrid current-row/Entity-fallback coordination is narrow in simple cases but can
   abort semantically independent mixed present/absent work through a cross-class
   lock cycle.
4. Exact Property slots give present and absent facts one total lock order and pass
   every exercised explicit-lock scenario, including the mixed falsifier.

The exact slot is therefore the strongest explicit coordinator **within this
exercised scratch matrix**. The experiment does not decide whether that extra
persistent concept is worth its production schema/API cost; conservative Entity
coordination remains the smaller safe comparator. This is evidence for the next
grill, not an accepted schema or implementation.

**Artifact status: `kept`.** Its code remains experimental and cannot be imported,
copied or promoted into production.

## Falsifier and non-claims

The refutation is no longer present if the exact mixed hybrid fixture accepts both
requests without changing its Entity fallback, current-row locks, history foreign
keys or fixed bounds. The supported slot sub-result is falsified if a repeat yields
two accepted same-current, same-absence or bidirectionally incompatible outcomes; a
partial failed Activity; an abort in the independent mixed-slot pair; or blocking of
`Stone.color` behind a held `Stone.state` slot.

Treat a strategy as inconclusive outside the exercised matrix. In particular, this
does not prove production Aicadia correctness, arbitrary multi-Entity deadlock
freedom, crash recovery, durable idempotency, movement safety, fair overload,
throughput, latency distribution, migration safety, subscription delivery or
million-User scale. The local serializable success cannot select a retry policy or
prove hotspot isolation.

## Downstream implication

The next transaction-design question can compare two honest productionward shapes:
conservative Entity coordination with known false contention, or one exact Property
slot that represents both present and absent dependencies uniformly. The public
dependency representation, Entity-wide invariants and idempotency contract remain
separate later questions. A broader serializable experiment is earned only if the
slot's schema cost later outweighs SSI's abort/retry risk.
