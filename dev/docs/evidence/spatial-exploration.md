---
status: Deterministic direct spatial exploration delivery complete through World, PostgreSQL, HTTP and MCP; no hosted-load or Agent-comprehension claim
---

# Direct spatial exploration evidence

> **Role / side:** direct spatial exploration delivery record / evidence bridge.
> **Authority:** owns S1 delivery status, exact deterministic proof, exercised seams and non-claims.
> **Excludes:** current game rules — defined in the [game contract](../../../game/docs/README.md); spatial rationale — defined in the [active synthesis](../concept/spatial-five-year-backcast.md); later-slice planning — defined in the [backlog](../../backlog/README.md).

## Delivered outcome

Direct spatial exploration S1 is delivered through the production `World`
implementation, PostgreSQL storage, HTTP and MCP. One entered Character can read a
bounded Place window, establish a new or existing connected destination without
moving, inspect the immutable Connection, stop at an unnamed course Position and
later arrive. A second User can read the same destination and Connection as shared
geography while retaining an independent Character Position.

The [current game contract](../../../game/docs/README.md) owns the exact executable
rules. Migration `0011_spatial_exploration.sql`, `game/src/world/`, `game/src/wire/`,
`game/src/server/` and the generated nineteen-tool Agent catalogue implement them.

## Exercised real seams

- Homebrew PostgreSQL 17.8 executes the production migrations, constraints, indexes,
  triggers, transactions and exact read queries in owned SQLx test databases.
- Production `World` code performs Position lineage, Place projection, Connection
  creation and reads, Investigation settlement, Discovery, Movement, idempotent
  reconstruction, rollback and Activity history.
- The real Axum HTTP router and MCP service call that same `World` and are checked
  against direct `World` and SQL readback. The generated MCP catalogue and OpenAPI
  surface are compiled from the current sources.
- Studio's real repository projection, documentation lint and Area renderer check
  current authority links and the stable `SP01`–`SP15` scenario catalogue.

## Deterministic outcome and state proof

Before Investigation, the first User reads a bounded window around A through MCP,
receives A and writes no Activity. The fixture then performs connected-Place
Investigation through MCP, submits Discovery through HTTP and retries it through
MCP. It reads the Connection summary without course hydration, then reads the
selected three-point course. The first Character moves through MCP to `(50, 0, 0)`
with no current Place, retries through HTTP, moves through HTTP to destination
`(200, 0, 0)` and retries through MCP. Discovery and both Movement retries return
their original accepted results and write no duplicate Activity.

The second User independently reads the destination through `list_place`, the same
Connection through both adapters and the Discovery Activity at its own current
Place. Direct `World` readback leaves the first Character at the destination and the
second at the origin. Direct SQL readback finds exactly one Connection, exactly two
`move_character` Activities, one `activity_connection` row per Movement and two
typed Position references per Movement with the exact `0 → 50 → 200` progression.
Reads add no Activity.

The remaining focused production fixtures prove the other accepted boundary cases:

- exact legacy backfill or fail-closed refusal, immutable Position lineage,
  positioned-Place projection rebuild, Connection identity/course constraints and
  typed Activity history;
- bounded Place and incident-Connection pagination, stable cursor progress under
  concurrent inserts, stale-projection rejection, at most 100 hydrated Places and
  no course expansion in a 100-Connection summary page;
- Entity-at-Position and every current/new/existing origin/destination Discovery
  variant, exact legacy and current retries, complete-package rollback, stale
  Position rejection and two concurrent equal-looking Connections that both commit;
- unshaped, shaped-forward, shaped-reverse, partial and complete Movement, checked
  integer geometry over the full admitted range, wrong-direction/off-course/stale
  rejection, injected rollback and two independent travellers; and
- retryable statement and Character-lock budgets that write nothing, while an
  unrelated Character still moves over the same immutable Connection.

## Bounded scale evidence

The production Place query forms pass dense, cross-axis and rotated fixtures of
exactly 1,000,000 projection rows, including deep continuation at row 900,000. The
production Connection continuation query remains bounded at row 900,000 of a
1,000,000-row hot endpoint. Fixed query-count instrumentation proves selected
spatial reads do not add per-row queries. These are deterministic local query-plan
and bound checks, not a production-throughput measurement.

## Protocol and catalogue parity

The compiled surface contains exactly 19 capability contracts, 19 Agent tool-text
sources, 19 generated catalogue tools and 19 HTTP/OpenAPI capabilities. Adapter
tests cover both Entity-discovery directions, all connected-Place variants, every
Movement shape, cross-adapter retries, semantic errors, two independent travellers
and the complete two-User flow. The checked-in catalogue equals the runtime MCP
catalogue, and Studio consumes that same compiled catalogue.

## Final validation

The 2026-08-20 completion ladder passes:

- `DATABASE_URL=postgres://localhost/postgres cargo test --workspace`: 324 passed,
  0 failed and only the explicit catalogue regenerator was ignored;
- `DATABASE_URL=postgres://localhost/postgres cargo test -p aicadia-studio --test studio`:
  52/52 passed, including the complete documentation and section-tree link checks;
- the exact nineteen-capability catalogue pin and compiled Studio/MCP parity tests;
- `cargo fmt --all -- --check` and strict all-target, all-feature Clippy;
- `git diff --check`, the focused S1 scope/authority review and a clean `cargo brief`.

T6's independent Standards and Spec reviews both reported zero findings. T7 changed
no gameplay, schema, protocol, public capability or generated catalogue; it repaired
the existing Studio Area-catalogue projection and aligned current development and
evidence authorities. T7's final Standards and Spec reviews also reported zero
findings.

## Simulated or absent seams and non-claims

Chance-dependent Investigation tests use the crate-internal scripted chance seam;
they do not prove operating-system entropy quality. The adapter tests use real local
HTTP and MCP services but no hosted network, failover, deployment, proxy or
production traffic. Million-row fixtures are synthetic and run without concurrent
hosted load. No paid or live model is invoked, so this evidence does not prove Agent
comprehension, conversation quality or player fun.

This record makes no production-throughput, hosted-latency, availability, privacy or
confidentiality claim. It proves no Area geometry or mechanics, Entity-relative
Position, moving carrier, distributed entry, Relation, Observation, Knowledge,
inventory, terrain effect, Route, travel time, Connection update/deletion/retirement
or other later-slice behavior. It also does not remove or scale-prove the older
exact-Place mutation freshness boundary.
