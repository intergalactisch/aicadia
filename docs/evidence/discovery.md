# Investigation and discovery evidence

> **Role / side:** first complete investigation/discovery delivery record / evidence bridge.
> **Authority:** owns deterministic delivery status, retained validation and review boundaries for this slice.
> **Excludes:** current game rules, decision rationale and runner operation; see `docs/game/`, `docs/concept/discovery.md` and `runner/agent-playtest.md`.

## Delivered outcome

The first complete investigation-and-discovery loop is delivered through one shared
`World` implementation, HTTP and MCP. An entered Character can receive a durable,
retry-stable zero or positive attempt. A positive can be completed once as one found
Entity with initial Property and Trait state, canonical prose, exact Place and
Character roles, consuming-attempt provenance and one immutable Activity in the
same transaction. The published player catalog contains fifteen capabilities.

The [current game contract](../game/README.md) owns the executable rules. Migration
`0010_investigation.sql`, `src/world/investigation/`, `src/wire/investigation.rs` and
`src/server/` implement them. The server split leaves HTTP and MCP as private thin
adapters over the same `World`; production chance uses fallible OS entropy behind a
private seam, while chance-dependent tests use only a crate-internal scripted source.

## Four separate evidence classes

### Runtime and PostgreSQL

The focused investigation suite passes 20/20 and the complete World suite passes
86/86 against PostgreSQL. It covers zero and positive storage, equal and fresh
requests, restart reconstruction, inclusive admission, positive-only FIFO voiding,
tied ordering, foreign/zero/consumed/voided/unplaced/moved attempts, request
conflicts, concurrent starts and commits, exact observer history, transaction
rollback, lifecycle immutability and bounded query plans for rate, live positives
and the last-48 exact-Place Activity window.

`tests/aicadia-local.sh` separately proves real launcher persistence. It creates an
owned disposable World, starts an investigation, accepts either natural outcome,
stops and restarts the launcher, retries the same request and requires byte-canonical
JSON equality plus exactly one attempt row. Ownership-verified cleanup leaves no
matching database. This is the restart/persistence proof; fake-Agent fixtures are
not cited for it.

### Adapter contract

Crate-internal deterministic server tests cover zero and positive output through
both HTTP and MCP, post-positive re-grounding, exact accepted discovery, equal retry,
strict unknown-field rejection and canonical errors. The complete server suite
passes 14/14. OpenAPI, MCP `tools/list`, the Agent instruction/tool sources and the
checked-in fixture agree on the exact fifteen-tool order, closed schemas, status
mapping and annotations. This proves adapter behavior; fake JSONL does not replace
it.

### Fake-Agent orchestration

The internal-only token-free controller executes seven isolated fake-Codex
processes through the real runner boundary. One resumed actor session handles an
uncertain-delivery zero and equal retry, a fresh positive and equal retry,
post-positive current re-grounding, a complete Property/Trait/prose preview and a
separately confirmed commit. A distinct co-present observer derives the accepted
Entity from orientation and reads the same current state and Activity. Separate
roles exercise admission, unavailable-attempt and request-conflict recovery.

The runner validates paired Aicadia-only JSONL events, exact session resume,
least-privilege tool sets, ids derived from results, equal revisions, exact preview
continuity, Property/Trait Activity changes, observer state/history, isolated
working directories, sanitized environments, no fallback authority and no
background trigger. Its manifest records seven fake processes,
`codex_invoked:false`, `model_calls:0` and owned deployment cleanup. This proves
controller orchestration and recovery discipline, not arbitrary model quality or
live World persistence.

### Static validator units

Thirteen adversarial trace mutations must fail, including start before grounding,
submit after zero, skipped re-grounding, pre-confirmation submit, changed retry,
invalid preview Property, changed accepted Trait, changed observer state, leaked
mechanics, fallback, background work, missing observer and incomplete recovery.
These mutations harden only the downstream trace validator; they are not Agent,
adapter or database evidence.

## Retained live local smoke

On 2026-08-15 one additional token-free smoke ran the real server and migrations
against a fresh ownership-tagged PostgreSQL database. Two disposable Users created
distinct co-present Characters. The live MCP catalog exposed fifteen tools; the
first natural investigation was positive, its HTTP result and equal MCP retry were
canonical-equal, and one MCP `submit_discovery` plus equal HTTP retry returned the
same accepted Activity, Entity and Place. The second Character independently read
the found Entity's exact Property, Trait and `submit_discovery` Activity through the
scoped HTTP reads.

The owned server stopped and the ownership-verified database drop left zero matching
live/local test databases. No Codex or model process was invoked. This proves one
real local cross-adapter discovery and co-present readback; it is not a production,
load, repeated-roll or live-model claim.

## Final validation and review

The final 2026-08-15 ladder passed:

- `cargo fmt --all -- --check`;
- `cargo clippy --all-targets --all-features -- -D warnings`;
- PostgreSQL-backed `cargo test --all-targets --all-features`: 147 passed, 0 failed
  (45 library, 2 database-helper, 14 server and 86 World tests) at delivery; see
  the review corrections below for the current 152;
- `bash tests/aicadia-local.sh`, including
  `restart_investigation_retry_identical=true` and `codex_invoked=false`;
- `bash tests/agent-playtest.sh`;
- `bash tests/trait-playtest.sh`, including owned-database failure cleanup;
- shell syntax, JSON schema parsing and `git diff --check`; and
- cleanup verification across the owned local, Agent and Trait database prefixes:
  zero databases remained.

Independent Sol High/Medium reviews stopped earlier stages for a retry/context
contract contradiction, impossible FIFO provenance, missing Trait-validator and
bounded-index obligations, incomplete schema/concurrency tests, adapter evidence
gaps and an initially self-authored discovery trace. Each was corrected before its
stage passed. The final T4 audit returned GO with no P0-P3 finding. The closing
standards/spec review found stale evidence-runner, concept, backlog and closure text;
those authority surfaces were corrected without changing runtime behavior.

## Review corrections (2026-08-15)

An independent three-reader review of the committed delivery (`60ce27c`) found no
P0, one P1 and bounded P2/P3 debt. The accepted [correction
plan](../../.agents/plans/20260815-180754-discovery-review-corrections/plan.md)
closed every item without changing zero/positive semantics, thresholds, formula
values, payload or operation names:

- the discovery fingerprint reuses the shared helpers and tags each Property and
  Trait list with its item count; two collision pairs (Property/Trait content moved
  between lists, and tag-lookalike keys/statements) that previously hashed equal now
  return `discovery_request_conflict`;
- `ChancePolicy::resolve` owns resolution; an in-window discovery provably lowers
  the outcome; index plans and the rolling-hour window are proved on the production
  SQL constants from crate-internal tests;
- the investigation result field `limits` became singular `limit`; the wire
  error-code list and HTTP status map are wildcard-free exhaustive matches; every
  published discovery rejection, both start Character errors and HTTP retry `201`
  are proved on both adapters; `src/server/mod.rs` is composition-only;
- `tool/start_investigation.md` follows the fixed template, `15-recovery.md` owns
  the three discovery recoveries, `domain.md#investigation-chance-and-admission` is
  the one home of chance/admission values, and stale evidence pointers were fixed.

The rerun ladder on the corrected tree passed: `cargo fmt --all -- --check`,
`cargo clippy --all-targets --all-features -- -D warnings`, `git diff --check`,
PostgreSQL-backed `cargo test --all-targets --all-features` 152 passed, 0 failed
(51 library, 2 database-helper, 15 server, 84 World), `bash tests/agent-playtest.sh`,
`bash tests/trait-playtest.sh` (after refreezing the token-free candidate digest for
the changed sources) and `bash tests/aicadia-local.sh`. The catalog fixture changed
only by the `limit` rename and the two corrected descriptions.

## Terry closure

1. The highest-value edge was the first World-authoritative discovery that changes
   later shared play, not another isolated state primitive.
2. The build fulfills the accepted contract: World resolves uncertainty before
   Agent authorship, and only the confirmed find becomes shared state and history.
3. Actor, action, state and ownership align: the User owns admission scope, the
   Character supplies derived context, the attempt owns roll/lifecycle provenance,
   the Entity owns current Property/Trait state and Activity owns accepted history.
4. The slice stays small: one attempt relation, one private chance component, two
   capabilities and existing Entity/Activity state; no generic discovery model,
   signed token, session, movement or background system was added.
5. Each evidence class proves only its own claim, and the complete ladder plus
   independent reviews proves the integrated delivery boundary.

The next concrete risk is no longer discovery correctness but the current single
Place's limited spatial play and eventual contention. Place-neighborhood context and
movement remain queued, not accepted; no new `Now` edge is selected by this record.

## Evidence boundary

This completion claims deterministic World, schema, adapter, catalog, Agent-contract
and token-free controller behavior for the accepted first loop. It makes no paid or
live-model discovery claim, no fun or prose-quality claim, no statistical claim
about repeated natural rolls, and no movement, further-Place, richer subject-model
or production-scale load-test claim. No server-side inference, durable Agent session
or background token spend was introduced.
