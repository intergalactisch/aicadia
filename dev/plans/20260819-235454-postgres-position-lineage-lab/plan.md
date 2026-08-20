---
status: complete
created_at: "2026-08-19T23:54:54+02:00"
updated_at: "2026-08-20T07:09:28+02:00"
accepted_at: "2026-08-20T06:56:16+02:00"
completed_at: "2026-08-20T07:09:28+02:00"
---

# Prove the PostgreSQL Position lineage and carrier boundary

> **Role / side:** draft execution plan for one retained PostgreSQL spatial lab / development side.
> **Authority:** owns the bounded fixture, tasks and evidence claim for testing Position resolution, concurrent cycle prevention, atomic history and carrier-local conflict isolation.
> **Excludes:** accepted game behavior, production schema, public capabilities and delivery proof; those remain in `game/docs/`, a later production plan and `dev/docs/evidence/`.

## Outcome

Produce one retained, standalone Rust/PostgreSQL experiment that supports, refutes or
narrows the current Position transaction candidate before Aicadia builds its first
playable spatial slice.

The exact evidence claim is bounded: **inside the scratch schema and fixed workloads,
direct and Entity-relative Position resolve to one exact whole-centimetre World
point; a concurrent A→B/B→A re-reference race cannot commit a cycle; a carrier move
changes one canonical Position rather than its descendants; carrier-local work that
does not need the external World point remains independent while an external-point
dependency serializes honestly; checked coordinate failure and injected write
failure leave no partial Position or Activity; and exact retries create no duplicate
history.**

The experiment may be inconclusive. It must never turn its positive result into a
production-correctness, latency, throughput or million-User claim.

## Non-goals

- No `game/docs`, production migration, `World` behavior, HTTP, MCP, public Agent
  text or Studio behavior changes.
- No Position, Place, Area, Connection, Relation, privacy or movement product choice;
  the lab tests the already negotiated technical candidate only.
- No PostGIS, map window, nearby query, projection, Area geometry, Connection course,
  pathfinding or visibility test.
- No benchmark, soak test, parameter sweep or claim that 1,000 descendants represent
  production load.
- No fixed semantic Position-chain depth, global World revision, region lock,
  descendant history write or process-local correctness state.
- No import, copy or promotion of lab code into runtime code.
- No Agent/model call, token spend, remote service or external database.

## Evidence and authorities

| Evidence or authority | Current fact | Consequence for this build |
| --- | --- | --- |
| User acceptance, 2026-08-19 | The User accepted the proposed real-PostgreSQL Position lab as the next step. | The outcome is fixed; an unwritten implementation was not accepted, so this complete draft precedes execution. |
| `dev/docs/concept/spatial-five-year-backcast.md#technical-synthesis-after-the-completed-grill` | The current candidate uses one Position per Entity, World or one Entity reference, checked centimetre arithmetic, exact revisions, stable locks, Activity and no descendant canonical rewrites. | The fixture must test those seams without accepting its illustrative schema. |
| `dev/areas/place/scenarios.md` SP07, SP09, SP12 and SP15 | Nested birdhouse placement, a moving ship, a hot unnamed point and impossible Connection cycles pressure lineage, fan-out and cycle boundaries. | Exercise nested and carrier Position; keep Connection cycles separate from invalid Position-reference cycles. |
| `dev/docs/research/spatial-five-year-foundation.md` | Primary-source research says relative placement removes descendant writes but not bounded resolution work; PostgreSQL recursive queries and locks still need proof. | Use a real database and expose chain work, lock scope and non-claims. |
| `dev/lab/README.md` and `dev/lab/spatial/README.md` | Technical labs are standalone Rust crates, retained but non-authoritative, with exact real and simulated seams. | Place the artifact at `dev/lab/spatial/02-postgres-position-lineage/` and index it. |
| `dev/lab/multiplayer/04-postgres-conflict-strategies/README.md` | A mixed current-row/Entity-fallback lock strategy was refuted; foreign-key and mixed lock classes can defeat paper ordering. | Acquire one explicit ordered coordinator set before writes and test actual lock compatibility instead of asserting it. |
| `/opt/homebrew/opt/postgresql@17/bin/pg_isready -h localhost -p 5432 -d postgres` | Local PostgreSQL 17 accepted connections before planning. | Use disposable SQLx databases at `postgres://localhost/postgres`; no remote or shared production data is touched. |
| Existing multiplayer PostgreSQL labs | SQLx 0.8, Tokio, UUID fixtures, local READ COMMITTED transactions, fixed lock timeouts and cleanup audits already run reproducibly in this repository. | Reuse the dependency versions and operation pattern, not their domain model or code. |

## Alignment

### Strategic

Large-world exploration and settlement cannot safely build on Position until moving
carriers and concurrent reference changes preserve one coherent World. This lab
retires the highest-impact correctness risk behind the selected second-Place and
Movement edge: a cycle or descendant rewrite would corrupt a core shared-world
invariant, while a carrier-wide lock would make ships and settlements collapse under
multiplayer use.

The lab advances a concrete game outcome rather than generic infrastructure. If its
candidate survives, the next plan can build direct Position, a second Place,
Connection and Character Movement without inventing a different spatial foundation.
If it fails, the result identifies the exact transaction seam that must change before
any production schema is proposed.

### Tactical

Use one scratch World containing direct Entities, nested Entities, a Ship, Cabin and
1,000 relative descendants plus two independent Entities A and B. Implement only the
Position and small local-state operations required by the assertions. Every accepted
lab mutation appends one Activity, one immutable Position version where applicable
and one current-pointer change in a single transaction.

The focused matrix contains eight cases:

1. direct, nested and at least 64-level resolution produces exact centimetre sums
   without a fixed depth of 32;
2. the lab-local symmetric coordinate bound and checked `i128` addition reject an
   invalid resolved point before durable acceptance;
3. synchronized A→B and B→A proposals both inspect the initial state, then the
   ordered lock/recheck protocol permits at most one and leaves an acyclic result;
4. moving Ship writes one Ship Position version and no descendant Position or
   Activity while sampled descendants resolve through the new Ship point;
5. a held Ship Position writer does not block a Cabin-local state change that needs
   no external point, while an operation explicitly resolving Cabin's World point
   waits or returns the bounded lock result;
6. `FOR NO KEY UPDATE` candidate writer coordination remains compatible with the
   exact foreign-key participation inserts the fixture expects, while `FOR SHARE`
   ancestor dependencies conflict with a Position writer as intended;
7. an exact `(requester, request_id, fingerprint)` retry returns the original
   Activity and a changed fingerprint conflicts without another write; and
8. an injected constraint failure after Activity insertion rolls back Activity,
   Position version and current-pointer work together.

All concurrency assertions use fixed barriers and bounded waits. The lab prints or
asserts the complete relevant current pointers, versions and Activities after each
case. No success criterion is requests per second.

### Technical

Create a standalone Rust 2024 crate using SQLx, Tokio and UUID versions aligned with
the existing PostgreSQL labs. `#[sqlx::test]` creates one disposable database per
test from one scratch migration. Tests run serially only where local database setup
or fixed race choreography requires it; concurrency inside a test uses separate pool
connections.

The scratch schema contains only:

- `entity` as stable coordinator identity;
- immutable `activity` with requester, request id, fingerprint and operation;
- immutable `position_version` plus one `position` current pointer;
- the minimum immutable/current local-state rows needed to distinguish interior
  work from external Position dependency; and
- constraints and indexes needed by the eight falsifiers.

The first lock candidate discovers the reference chain in a terminating recursive
query under a transaction-local statement timeout, collects all involved Entity ids,
sorts them, acquires `FOR NO KEY UPDATE` for the changed Entity and `FOR SHARE` for
Position dependencies in that one order, then re-reads revisions before validation
and write. If PostgreSQL lock behavior or implicit foreign-key locks break this
protocol, record `refuted` or `inconclusive`; do not add an unplanned coordinator or
silently switch isolation.

Fixed experimental bounds:

- Agent/model calls and tokens: zero;
- databases: one SQLx disposable database per focused test;
- focused tests: exactly eight, plus one separately invoked ignored cleanup audit;
- pool: at most five connections per test;
- isolation: PostgreSQL `READ COMMITTED` only;
- concurrent actors: two per race;
- descendant fixture: exactly 1,000, with bounded sampled readback;
- deep-chain fixture: at least 64 and no unbounded generated depth;
- lock/statement timeout: 150 ms for deliberate blocking observations;
- outer concurrent assertion timeout: two seconds; and
- coordinate candidate: one clearly lab-local symmetric `BIGINT` bound selected so
  every addition and validation uses checked Rust `i128` arithmetic.

Real seams are Rust/Tokio scheduling, SQLx pools and disposable databases, local
PostgreSQL 17, READ COMMITTED, recursive SQL, row and foreign-key locks, uniqueness,
commit/rollback and checked integer arithmetic. Simulated or absent seams are the
production `World`, current migrations, User/Character authority, public input,
HTTP, MCP, Agents, LLMs, Position visibility, PostGIS/index projections, Area,
Connection, Relation, hosted pooling, failover, load distribution and operations.

## Decisions, assumptions and open questions

### Confirmed decisions

- Test the Position/PostgreSQL candidate before production planning — User accepted
  this as the next step and the technical synthesis records why.
- Use the retained Spatial lab and standalone Rust — project lab constitution and
  prototype routing require the production language for concurrency evidence.
- Keep Position-reference cycles invalid while Connection cycles remain artistically
  valid — current canonical vocabulary and SP15.
- Use no fixed semantic depth of 32 — current Place direction explicitly rejected it;
  only bounded request work is permitted.
- Keep Activity atomic with accepted current state — `AGENTS.md` and current World
  Change direction.
- Keep the artifact non-authoritative and non-promotable — `dev/lab/README.md`.

### Reversible assumptions

- SQLx/Tokio/UUID versions match the existing local labs — minimizes experimental
  setup and changes no runtime dependency.
- A 1,000-descendant fixture is enough to expose write amplification by row count —
  it is not a capacity or latency proxy.
- A lab-local coordinate bound below the full `BIGINT` range is sufficient to test
  checked arithmetic — its value never enters Aicadia vocabulary or a product
  contract.
- Compatible shared locks are the smallest first candidate for hot Connection-like
  dependency reads — this lab tests Position only and makes no Connection claim.

### Open questions

- Does the proposed Entity-row lock protocol survive the synchronized cross-reference
  race and foreign-key participation inserts? This is the experiment question, not
  an acceptance blocker.
- Does a local-only Cabin write remain independent while an external-point read
  conflicts exactly with Ship movement? This is the hot-carrier falsifier.
- Which exact failure or retry result appears under bounded lock timeout? The lab may
  report the observed PostgreSQL outcome but cannot turn it into a player-facing
  error contract.

No open question changes the agreed experiment outcome, domain meaning, public
contract, external authority, cost or evidence claim. The plan is ready for explicit
User acceptance while remaining `draft` until that acceptance occurs.

## Implementation map

| Surface | Current state | Intended change | Invariants |
| --- | --- | --- | --- |
| `dev/lab/spatial/README.md` | Indexes one paper model test. | Add experiment 02 with its exact PostgreSQL question. | Track remains non-authoritative. |
| `dev/lab/spatial/02-postgres-position-lineage/Cargo.toml` and `Cargo.lock` | Absent. | Add one standalone, unpublished Rust 2024 crate. | Root workspace and runtime dependencies remain unchanged. |
| `dev/lab/spatial/02-postgres-position-lineage/migration/0001_position_lineage.sql` | Absent. | Create only the scratch Entity, Position, local-state and Activity fixture. | Singular names; no production migration or hidden dependency. |
| `dev/lab/spatial/02-postgres-position-lineage/src/lib.rs` | Absent. | Implement the candidate lock/resolution/write functions and eight bounded SQLx tests. | One canonical experimental implementation; no runtime import. |
| `dev/lab/spatial/02-postgres-position-lineage/README.md` | Absent. | Record question, bounds, real/simulated seams, raw matrix, verdict, falsifiers and commands. | Verdict cannot exceed the exact fixture. |
| `dev/docs/concept/spatial-five-year-backcast.md` | Technical candidate awaits PostgreSQL proof. | Append only the bounded lab verdict and resulting technical implication. | Product choices remain unchanged; a refutation corrects the candidate. |
| `dev/docs/concept/log/2026-08.md` | Records synthesis but no runtime observation. | Append plan acceptance and final material verdict. | History remains append-only and distinguishes experiment from choice. |
| `dev/backlog/README.md` | PostgreSQL proof remains open under queued spatial outcomes. | Link the completed verdict and update the exact next technical risk only if evidence changes it. | No production state or contract is claimed from a lab. |

## Execution contract

Root owns outcome, scope, plan state, integration and the final evidence claim. No
delegation is required. The experiment stays inside its standalone crate and the
named development records. It stops before any production file, dependency, schema,
capability or runtime behavior changes.

## Task graph

Allowed states are `pending`, `in_progress`, `completed` and `blocked`.

| ID | State | Depends | Parallel-safe | Objective | Owned surfaces | Evidence |
| --- | --- | --- | --- | --- | --- | --- |
| T1 | completed | — | no | Build and run the eight-case real-PostgreSQL Position fixture with complete authoritative readback. | `dev/lab/spatial/02-postgres-position-lineage/` | Focused test command and separate cleanup audit. |
| T2 | completed | T1 | no | Record the bounded verdict and align the lab index, concept implication, log and backlog. | `dev/lab/spatial/README.md`, experiment `README.md`, active spatial concept, period log, backlog | Result-matrix audit, documentation lint and focused scope review. |

## Task details

### T1 — Execute the Position lineage fixture

**Objective:** Eight deterministic tests expose whether the candidate Position
schema, resolution and transaction protocol satisfy every stated falsifier on real
local PostgreSQL.

**Actions:**

1. Create the standalone crate and scratch migration with exact immutability,
   lineage, current-pointer, idempotency and rollback constraints.
2. Implement bounded chain discovery, stable lock acquisition, revision recheck,
   checked resolution, Position mutation and minimal local-state mutation.
3. Implement exactly the eight fixed tests and full post-case state assertions.
4. Run the focused suite and separately verify that SQLx leaves no experiment
   databases behind.

**Invariants:**

- No Position cycle, unchecked integer conversion or descendant canonical write can
  count as acceptance.
- Every successful state change has exactly one Activity; every rejected or failed
  attempt has none.
- Interior independence is claimed only for an operation that does not resolve the
  external chain.
- Every timeout is bounded and classified; no blocking test can hang the suite.
- The artifact imports no production module and changes no root manifest.

**Evidence:**

- `DATABASE_URL=postgres://localhost/postgres cargo test --manifest-path dev/lab/spatial/02-postgres-position-lineage/Cargo.toml -- --test-threads=1` — exactly eight focused cases pass or produce one inspectable bounded refutation.
- `DATABASE_URL=postgres://localhost/postgres cargo test --manifest-path dev/lab/spatial/02-postgres-position-lineage/Cargo.toml audit_sqlx_database_cleanup -- --ignored --test-threads=1` — disposable database cleanup is independently verified.

**Stop conditions:**

- Stop before expanding scope if the candidate requires another isolation level,
  persistent coordinator, public error contract, production dependency or semantic
  depth limit. Record the exact failure and return the plan to `draft` if the evidence
  claim must change.
- Stop and report environmental blockage if local PostgreSQL cannot create and drop
  disposable SQLx databases; do not substitute an in-memory simulation.

### T2 — Record the bounded verdict

**Objective:** A future builder can reproduce exactly what the lab supported,
refuted or left inconclusive and can see the next productionward risk without
mistaking the artifact for Aicadia behavior.

**Actions:**

1. Write the experiment README from the executed fixture, including commands,
   bounds, state readback, matrix, verdict, falsifiers and non-claims.
2. Index experiment 02 in the Spatial track.
3. Append the bounded implication to the active spatial technical synthesis and the
   material result to the concept log.
4. Update only the affected backlog proof status or next risk; do not select or
   implement the production slice automatically.

**Invariants:**

- Lab owns observations only; concept owns the technical recommendation; backlog
  owns forward state; `game/docs` remains untouched.
- Real and simulated seams remain named and no production or million-User claim is
  borrowed from the local fixture.
- A refuted candidate is corrected everywhere current; it is never softened into a
  success to preserve the earlier synthesis.

**Evidence:**

- Focused `rg` audit — all eight cases, exact bounds, real/simulated seams, verdict,
  falsifier, commands and next risk are present once in their owning records.
- `cargo test -p aicadia-studio --test studio the_documentation_lint_is_clean` —
  experiment metadata, index and links satisfy repository placement rules.
- `git diff --check` — plan and lab changes contain no whitespace errors.

**Stop conditions:**

- Stop before changing vocabulary, accepted behavior, schema or public operations.
  A result that requires such a change returns to User negotiation and a revised
  production plan.

## Validation ladder

1. **Focused:** exactly eight SQLx/PostgreSQL cases and the separate cleanup audit
   execute with fixed bounds and complete current/history readback.
2. **Contract:** the experiment remains standalone; root workspace, `game/docs`,
   runtime schema/code, HTTP, MCP and public Agent text are byte-unchanged by this
   plan.
3. **Outcome:** the retained matrix supports, refutes or narrows every clause of the
   stated Position-lineage evidence claim and names the next productionward risk.
4. **Integrity:** `git diff --check`, Studio documentation lint, focused diff review
   and confirmation that unrelated User changes and governing authorities remain
   intact.

## Change control

Refine test implementation, scratch constraints and stronger assertions in place
while the eight cases, real/simulated seams and evidence claim remain unchanged.
Stop implementation, return `status` to `draft`, revise and request explicit
re-acceptance if evidence requires another isolation level, new persistent concept,
different case matrix, production surface, external authority, material cost or
broader claim.

## Completion conditions

- T1 and T2 are `completed` and the validation ladder passes;
- the exact bounded evidence claim is supported, refuted or narrowed honestly;
- the experiment README and Spatial index make reproduction and non-authority clear;
- active technical synthesis, concept history and backlog point to the one verdict
  without changing accepted product behavior;
- no known-stale authority, material open question or accidental unrelated change
  remains; and
- `status: complete` and `completed_at` are recorded only after these conditions.
