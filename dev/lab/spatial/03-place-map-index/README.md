---
question: Can the accepted one-index Place-map projection keep exact box and keyset pages bounded, and what is the smallest PostgreSQL-only correction when it cannot?
verdict: refuted
status: kept
real_seam: [Rust 2024, Tokio, SQLx disposable databases, PostgreSQL 17.8 planner, table and index storage, exact million-row synthetic projections, exact box and keyset queries, EXPLAIN ANALYZE BUFFERS, VACUUM ANALYZE, commit and ownership-safe cleanup]
simulated_seam: [production migration and World schema, Position exact recheck, authorization, HTTP and MCP, concurrent production traffic, failover, hosted latency, production throughput]
informs: dev/plans/20260820-071639-spatial-exploration-system/plan.md#t1e--falsify-the-place-map-index-before-production
---

# PostgreSQL Place-map index

> **Role / side:** retained PostgreSQL spatial-index experiment / development side.
> **Authority:** owns the fixtures, observations, bounded verdict and reproduction commands for T1E's Place-map projection experiment.
> **Excludes:** accepted spatial behavior, production schema, migration design and delivery proof; those remain in `game/docs/`, the accepted spatial plan and `dev/docs/evidence/`.

## Question

Does the accepted rebuildable `place_map_index` with one covering
`(x_cm, y_cm, z_cm, place_entity_id)` B-tree keep the exact S1 inclusive box,
keyset, tuple order and `LIMIT 100` query bounded under dense and adversarial
million-row distributions?

After that candidate failed, the same lab compared only the smallest symmetric
PostgreSQL correction allowed by T1E: add covering `(y_cm, z_cm, x_cm,
place_entity_id)` and then `(z_cm, x_cm, y_cm, place_entity_id)` B-trees to the
same rebuildable projection. No tested index is canonical World truth.

## Fixed fixture

The standalone Rust/SQLx crate creates one disposable PostgreSQL database per case.
Each database contains exactly 1,000,000 projection rows and the same five selected
columns. Every first page uses all six inclusive box bounds, tuple order
`(x_cm, y_cm, z_cm, place_entity_id)` and `LIMIT 100`. Every continuation adds the
strict row-tuple comparison against the first page's last tuple. The selected
Position activity id is covered by every candidate index.

Three independent distributions were measured:

- **Dense:** all 1,000,000 rows are at `(0, 0, 0)`.
- **Cross-axis adversarial:** X spans `-500000..499999`; only the final 200 rows
  have Y=0 while the other 999,800 have Y=1,000,000; Z is 0. The box makes X broad
  and Y selective.
- **Rotated adversarial:** X and Y both span `-500000..499999`; only the final 200
  rows have Z=0 while the other 999,800 have Z=1,000,000. The box makes X and Y
  broad and only Z selective.

The executable gates reject a final candidate if a page returns more than 100 rows,
uses a sequential scan or touches more than 100 shared buffer blocks. Dense pages
must expose exactly 100 index rows. Each adversarial first page has exactly 200
qualifying rows and each continuation has 100.

## Observations

| Distribution and candidate | First page | Continued page | Bounded observation |
| --- | --- | --- | --- |
| Dense, one X-leading index | 100 index rows, 4 shared blocks, 0.030 ms | 100 index rows, 5 blocks, 0.035 ms | Supported this distribution. |
| Dense, X-, Y- and Z-leading indexes | 100 index rows, 4 blocks, 0.033 ms | 100 index rows, 5 blocks, 0.040 ms | Supported this distribution. |
| Cross-axis, one X-leading index | parallel sequential scan, about 1,000,002 scan rows, 999,801 removed, 10,438 blocks, 27.706 ms | 100 index rows, 5 blocks, 0.039 ms | Refuted by the first page. |
| Cross-axis, X- and Y-leading indexes | 200 index rows, 5 blocks, 0.070 ms | 100 index rows, 5 blocks, 0.035 ms | Supported this distribution. |
| Cross-axis, X-, Y- and Z-leading indexes | 200 index rows, 5 blocks, 0.068 ms | 100 index rows, 5 blocks, 0.034 ms | Supported this distribution. |
| Rotated, one X-leading index | parallel sequential scan, about 1,000,002 scan rows, 999,801 removed, 10,438 blocks, 29.644 ms | 100 index rows, 5 blocks, 0.033 ms | Refuted by the first page. |
| Rotated, X- and Y-leading indexes | X-leading index scan across 9,348 blocks, 45.688 ms | 100 index rows, 5 blocks, 0.034 ms | Refuted by the first page's proportional index work. |
| Rotated, X-, Y- and Z-leading indexes | 200 index rows, 5 blocks, 0.077 ms | 100 index rows, 5 blocks, 0.030 ms | Supported this distribution. |

The exact retained `EXPLAIN (ANALYZE, BUFFERS)` output is in
`observed-plans.txt`. Timings are local observations, not latency promises. Parallel
worker row counts in PostgreSQL's text plan are per-loop rounded values, hence the
reported approximately 1,000,002 scan rows; the authoritative fixture count is
exactly 1,000,000 in every database.

## Verdict

**The accepted one-index candidate is refuted.** It is excellent at the million-row
dense point and at continuations near the selected tail, but its cross-axis first
page performed work proportional to the whole fixture and violated the no-sequential-
scan gate.

**The two-index candidate is also refuted as the general T1E correction.** It fixes
the cross-axis fixture but fails after the selectivity is rotated to Z.

**Three symmetric covering B-trees are supported only for the three fixed
fixtures.** The final first and continuation pages used no sequential scan and
touched five shared blocks. This is the smallest tested PostgreSQL-only correction
that survived every T1E falsifier without PostGIS, canonical cells or another World
truth.

The final X-, Y- and Z-leading index set was installed before both pages in every
distribution; the dense, cross-axis and rotated final measurements therefore test
the same three-index candidate.

Three axis-leading B-trees do **not** mathematically bound all possible correlated
3D distributions, prove production throughput or choose the production design.
Another correlation can still make every individual axis broad while their
conjunction is sparse. Root must correct the accepted technical seam before T2 and
retain exact Position recheck; this refuted verdict cannot silently bless the
original migration design.

## Real and absent seams

Real observations come from PostgreSQL 17.8 planning and execution, B-tree storage,
one million committed rows per disposable database, exact SQL predicates, keyset
continuation, buffer accounting and SQLx cleanup. The experiment does not exercise
the production schema or migration, `World`, authorization, exact Position recheck,
HTTP/MCP, concurrent traffic, hosted I/O, failover, vacuum behavior under mutation
or sustained load. It proves no latency or million-User capacity claim.

## Reproduce

Run all three focused fixtures serially:

```sh
DATABASE_URL=postgres://localhost/postgres cargo test \
  --manifest-path dev/lab/spatial/03-place-map-index/Cargo.toml \
  -- --nocapture --test-threads=1
```

The observed result was `3 passed; 0 failed; 1 ignored`. Then independently verify
that SQLx removed every registered disposable database:

```sh
DATABASE_URL=postgres://localhost/postgres cargo test \
  --manifest-path dev/lab/spatial/03-place-map-index/Cargo.toml \
  audit_sqlx_database_cleanup -- --ignored --test-threads=1
```
