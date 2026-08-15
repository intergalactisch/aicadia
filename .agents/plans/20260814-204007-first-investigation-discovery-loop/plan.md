---
status: complete
created_at: "2026-08-14T20:40:07+02:00"
updated_at: "2026-08-15T14:01:04+02:00"
accepted_at: "2026-08-15T11:26:34+02:00"
reaccepted_at: "2026-08-15T12:02:47+02:00"
completed_at: "2026-08-15T14:01:04+02:00"
---

# First complete investigation-and-discovery loop

> **Role / side:** consequential build plan / development side.
> **Authority:** defines the proposed outcome, execution boundary, task order and evidence claim for Aicadia's first complete investigation-and-discovery loop.
> **Excludes:** current executable game truth, decision rationale and delivery evidence; those remain in `docs/game/`, `docs/concept/` and `docs/evidence/`.

## Outcome

A User experiences the first complete act of discovery through their entered
Character and conforming Agent. The Agent grounds itself at the exact current Place
and asks World to investigate. World admits the request under one Character-wide
operational policy, performs exactly one authoritative chance roll and answers
`zero` or `positive`. After a positive result the Agent authors one found thing—
something that already existed in the World without anyone making it—previews it
naturally, and only after the User's explicit confirmation submits it. World rejects
atomically or commits exactly one placed Entity with its initial Properties and
Traits, one Activity with canonical prose and roles, and durable attempt/result
provenance. A co-present Character reads the same accepted find through existing
scoped reads. A zero honestly ends one attempt and leaves no Activity; a later
admitted request is a fresh independent roll even without intervening World change.

The discovery rule that gives the roll its purpose: what already existed in the
World enters only through investigation and roll (plants, tracks, ore, springs, ruin
fragments; later new Places), while what a Character makes, brings or places remains
ordinary confirmed Action introduction. World cannot enforce that distinction; the
Agent contract states it and deterministic Agent evidence exercises it.

The intended final evidence claim is:

> A clean-room conforming Agent can use only published Aicadia MCP to ground one
> entered Character at the exact current Place, start one World-admitted
> investigation, receive a retry-stable zero or positive result, and—after a
> positive result and the User's confirmation—submit one structurally bounded find
> that World either rejects atomically or commits once with exact Character, Place,
> attempt, result, prose and involved-Entity history; an independent co-present
> Character then reads the same accepted find. Equal delivery retries never reroll;
> later admitted requests are independent fresh attempts even without World change;
> several unconsumed positive attempts coexist across conversations without
> conversation identity entering World; admission and saturation are bounded
> per-User and per-Place computations that no instance count or conversation count
> can inflate. Neither call uses server-side inference, a durable Agent session or
> background token spend.

The User explicitly accepted the earlier plan on 2026-08-15 and T0 completed on
pushed commit `6f95ee2`. T1 review then exposed one material contradiction between a
mutable positive-response context, byte-identical retries and the intentionally
minimal attempt row. The User reaccepted every recommended correction on 2026-08-15;
T1–T5 then completed against that corrected contract.

## Non-goals

- Movement, a second Place, Place containment or adjacency, coordinates, routes,
  geometry, travel and arbitrary placement; discovering a new Place is the named
  second result kind and belongs to the later movement edge.
- More than one concrete result per positive attempt; a World-typed kind list
  (flora, fauna, material) or any server-authored name, prose or direction.
- A universal `Discovery`, `Observation`, knowledge, relationship, signal, event or
  generic consequence model; the broad inspection/query language from exploration.
- Semantic duplicate detection, contradiction inference, embeddings or search.
- Pity, streaks, scores, levels, currencies, clocks-as-gameplay, background
  simulation, automatic Agent activation, a durable Agent conversation, a listable
  stock of pending finds or process-local correctness state.
- Signed tokens, HMAC keys, secret rotation or launcher key configuration.
- Runtime-configurable chance or admission parameters; operator switches or
  environment variables that alter resolution.
- Persisted response snapshots, generic JSON payloads or as-of reconstruction solely
  to replay mutable Place context on a delivery retry.
- Authentication, OAuth, public hosting, non-loopback operation or production
  rate-limiting infrastructure beyond the per-User admission rows below.
- Any paid model call.

## Evidence and authorities

| Evidence or authority | Current fact | Consequence for this build |
| --- | --- | --- |
| `AGENTS.md` — Terry, Game Progress First, Plan Before Build, Built For Massive Concurrency, Every World Action Leaves History | The next build must deliver the highest-value smallest complete game outcome, must not preclude millions of concurrent Characters, and stores exact history atomically | Start takes no Place lock, every read is bounded, admission and saturation are per-User/per-Place; the accepted find writes Activity in the same transaction |
| User choices 2026-08-14/15 (concept log, "grill active" and "grill resumed") | Discovery rule, first result, confirmation, zero, chance, admission, history and names are decided; T1 reopened only positive-response context versus retry storage | Reacceptance of the recommended stable response authorizes T1–T5 without reopening the rest |
| `.agents/backlog/README.md` | Sol-medium validation is complete; this loop is the sole `Now / Active` item | Keep one active edge and close it only after the exact discovery evidence claim passes |
| `docs/game/README.md`, `deferred.md`, `protocol.md`, `domain.md`, `storage.md` | Thirteen capabilities; investigation, rolls and discovery absent; closed `Activity.operation` enum; `(requested_by_user_id, request_id)` retry index on `activity` only; `place.latest_activity_id` is the exact-Place revision; User→Place lock order | The catalog becomes fifteen; `Activity.operation` gains `submit_discovery` (a schema change visible in every Activity-returning tool); attempts need their own retry key; the commit reuses the Place pointer advance |
| `docs/game/agent.md` | Every mutating workshop tool requires a complete natural preview and explicit User confirmation; confirmation is a contract obligation, not a wire field | `submit_discovery` inherits that obligation; `start_investigation` is not a player-visible World change and needs none |
| `docs/concept/discovery.md` and log "Stochastic discovery rolls" | Confirmed: World-first resolution, no pity, several scope-bound attempts, Character-wide admission; the signed `roll_token` transport direction is now superseded by the durable attempt row | Concept record is updated in the same change; superseded direction is named, not erased |
| `src/world/{mod,common,mutation,activity}.rs`, `Cargo.toml` | `World` is a struct over `PgPool` (`mod.rs:104`), `lock_user`/`lock_place` (`common.rs:3,30`), fingerprints via `sha2`, `uuid` v4; no `rand`, no `hmac`; `src/world/mod.rs` has 141 production lines, `src/server.rs` 1,108 unsplit | The investigation Module composes existing helpers; `rand` is the one new crate; the server split remains part of T3 |
| Migrations `0001`–`0009` | `0009_uniform_entity_state.sql` is the latest committed migration | Discovery is `0010` |
| Baseline commit `6f95ee2` | The complete uniform Entity-state build, its evidence and plans are committed and pushed; format, Clippy, 119/119 PostgreSQL tests and all three shell suites pass | T0 is complete; later failures are evaluated against this fixed green baseline |

## Alignment

### Strategic

The current game lets Characters enter, act, interact and change local state. It
does not yet deliver Aicadia's defining promise: Agents intelligently investigate
one shared World and help it acquire state that was already there. This loop makes
the World the source of what exists and the Agent the source of what it means:
uncertainty is resolved by World, authorship stays with the User's Agent, and the
result is shared history. It deliberately evolves the contract instead of stretching
`submit_action` prose into an undeclared mechanic.

The next concrete risk after this build is repeat play at one exact Place: the
saturation formula makes a well-searched Place recover only as ordinary play
continues, and discovery of new Places requires movement. Evidence should point next
to Place neighborhood/movement; this plan prebuilds neither.

### Tactical

1. The Agent reads World, its Character, exact-Place Entities/Activity and any
   selected Entity state through existing scoped capabilities.
2. The Agent calls `start_investigation { request_id }`. World derives User,
   Character and current Place; input selects nothing else.
3. Under the User lock, World resolves an equal retry first, applies rate admission,
   gathers the bounded chance signal, draws once and inserts the `zero` or `positive`
   attempt. Only a new positive beyond the hoarding cap then voids the oldest prior
   live positive with `id <> new_attempt_id` by referencing the inserted attempt. No
   Place lock, Activity or pointer change occurs.
4. `zero`: the Agent renders one honest unsuccessful search in-world and continues
   with ordinary play or a later investigation. `positive`: the retry-stable response
   carries only the stored outcome, attempt id and immutable generic limits.
5. After a positive the Agent re-reads the exact current Place, selected Entity
   state and recent exact-Place Activity through existing bounded capabilities. It
   authors exactly one find—an ordinary Entity with name, description and 0–100
   initial Properties/Traits—coherent with those current reads, previews it
   completely in the User's language and obtains explicit confirmation. The User
   advises but never selects focus, odds, roll or result.
6. The Agent calls `submit_discovery { request_id, attempt_id, prose, find }`. World
   locks User then Place, verifies an own, positive, unconsumed attempt whose Place
   is the Character's current Place, validates the find with the existing Entity
   rules, creates and places the Entity, writes Activity `submit_discovery` with
   prose and `subject`/`location` roles, marks the attempt consumed and advances the
   Place pointer in one transaction.
7. Actor and co-present Characters read the find and its Activity through existing
   exact-Place reads; rejected calls, zero attempts and private reasoning leave no
   history.

Concrete boundary cases: retry of a zero or positive start with the same
`request_id` returns the identical body without rolling. Start has no semantic body,
fingerprint or content-conflict branch. A rejected admission creates no attempt; a
fourth unconsumed positive voids the oldest; a consumed, voided, foreign or
other-Place attempt fails
the commit neutrally; a stale `place_revision` never applies because the commit
binds none; unrelated Actions at the Place never invalidate an attempt.

### Technical

- **World behavior:** two new public `World` methods, `start_investigation` and
  `submit_discovery`. One private `investigation` Module behind them.
- **PostgreSQL (`migration/0010_investigation.sql`):**
  ```text
  investigation_attempt(
    id UUID PK,
    requested_by_user_id FK user NOT NULL,
    request_id UUID NOT NULL,
    character_entity_id FK character NOT NULL,
    place_entity_id FK place NOT NULL,
    outcome TEXT NOT NULL CHECK IN ('zero','positive'),
    consumed_by_activity_id UUID NULL UNIQUE FK activity,
    voided_by_attempt_id UUID NULL FK investigation_attempt,
    created_at timestamptz NOT NULL,
    CHECK (outcome = 'positive' OR (consumed_by_activity_id IS NULL AND voided_by_attempt_id IS NULL)),
    CHECK (consumed_by_activity_id IS NULL OR voided_by_attempt_id IS NULL),
    CHECK (voided_by_attempt_id IS NULL OR voided_by_attempt_id <> id)
  )
  UNIQUE (requested_by_user_id, request_id)                       -- retry identity
  INDEX  (requested_by_user_id, created_at DESC)                  -- admission window
  partial INDEX (requested_by_user_id, created_at)
    WHERE outcome = 'positive' AND consumed_by_activity_id IS NULL
      AND voided_by_attempt_id IS NULL                            -- hoarding cap
  INDEX activity(context_place_entity_id, occurred_at DESC, id DESC)
    WHERE context_place_entity_id IS NOT NULL                    -- bounded Place window
  activity.operation CHECK gains 'submit_discovery';
  the prose/request-provenance CHECK admits 'submit_discovery'.
  CREATE OR REPLACE FUNCTION validate_entity_trait_version_activity()
    also admits Trait versions owned by 'submit_discovery'.
  ```
  Attempt rows are immutable except the two nullable lifecycle columns, each set
  once. No generic table, no counter column, no session.
- **Chance:** private `ChancePolicy` (pure `probability(signal) -> f64`, typed
  parameters `p_max = 1/2`, `p_min = 1/10`, `h = 6`, `W = 48`) and private
  `ChanceSource` trait with `OsChance` (`rand`) in production and a scripted source
  in crate-internal tests, injected at World construction only through a test-only
  crate-private helper so the public `World::new` seam remains singular. Signal:
  number of `submit_discovery` Activities among the last `W` Activities at the
  Place—one bounded read used only by the chance component.
- **Admission:** under the User lock, resolve retry then count attempts in the rolling
  hour (`A = 12`) using one PostgreSQL `statement_timestamp()` and inclusive lower
  boundary, and reject before rolling when full. Store that same database timestamp
  on the inserted attempt. After the draw and attempt insert, only a new positive
  that exceeds `P = 3` unconsumed positives voids the oldest prior live positive
  satisfying `id <> new_attempt_id`, ordered by `(created_at ASC, id ASC)`, and points
  it at the now-existing new attempt. The schema also rejects self-void provenance.
  Parameters are typed constants in the attempt owner; changing them is a documented
  contract change.
- **Concurrency:** start locks User only; commit locks User then Place (existing
  order); attempts bind no `place_revision`; unrelated Place changes never stale an
  attempt; conflicting commits of the same attempt serialize on the User lock and
  the second sees a consumed attempt.
- **Idempotency:** start has no semantic input beyond `request_id`, so
  `(requested_by_user_id, request_id)` alone returns the stored attempt outcome/id
  plus immutable limits with no fingerprint or content-conflict branch. Discovery
  fingerprints normalized input with the existing SHA-256 scheme (`find.property`
  sorted by canonical key, `find.trait` by statement); equal retries return the
  canonical committed body and different content conflicts.
- **Errors:** `investigation_not_admitted` (429),
  `discovery_attempt_unavailable` (409, neutral) and `discovery_request_conflict`
  (409), plus `invalid_discovery` only for invalid discovery prose, beside existing
  `character_not_found`, `character_not_entered`,
  `invalid_entity`, `invalid_property`, `invalid_trait`,
  `property_key_conflict`, `invalid_request` and `unavailable`.
- **Adapters:** `POST /api/investigation` (200) and `POST /api/discovery` (201);
  MCP tools with the same names; OpenAPI, compiler-generated schemas, descriptions
  and the checked-in catalog fixture change together. The `Activity.operation` enum
  change is an accepted fixture change for every Activity-returning tool.
- **Submit result:** `submit_discovery` returns exactly `{activity, entity, place}`.
  Initial Property/Trait state is not duplicated; it is visible through the Activity
  and the existing paginated `get_entity_at_current_place` read.
- **Agent experience:** new instruction section distinguishing found from made,
  free start, post-positive current re-grounding, confirmed commit, zero rendering
  and no ids in play; two tool descriptions in the fixed template.
- **Tests:** World tests own resolution, admission, retry, consumption, voiding,
  history and concurrency through the scripted chance source; server tests own
  strict decoding, parity, catalog and the two-Character observer outcome
  in-process; the token-free fake Agent suite owns clean-room grounding and
  presentation. No probabilistic assertion anywhere.
- **Documentation:** accepted behavior in `docs/game/`; rationale in
  `docs/concept/discovery.md`; trail in the concept log; forward state in the
  backlog; proof in `docs/evidence/discovery.md`.
- **Dependencies:** `rand` is added; nothing else.

### Module and ownership design

Dependency direction is one-way:

`HTTP/MCP Adapters -> strict wire shapes -> World Interface -> private investigation
Module -> existing Entity/Property/Trait/Activity persistence helpers -> PostgreSQL`.

| Concern | Owner | Interface and limits |
| --- | --- | --- |
| Public game behavior | existing `World` | `start_investigation`, `submit_discovery`; no second public seam |
| Orchestration | `src/world/investigation/mod.rs` | Sequences admission → signal → draw → store, and verify → validate → commit; no SQL of its own beyond calling owners, no chance math, no wire types |
| Types and normalization | `src/world/investigation/model.rs` | Typed inputs/outputs, limits, context, fingerprint normalization; re-exported through `world`/`lib.rs`; no rows or secrets |
| Chance | `src/world/investigation/chance.rs` | `ChancePolicy` parameters and pure probability; private `ChanceSource`, `OsChance` and scripted unit-test source; no SQL, lifecycle, transport or public injection seam |
| Attempt lifecycle and admission | `src/world/investigation/attempt.rs` | Attempt rows, rate/hoarding admission, FIFO voiding, retry reconstruction, consumption primitive; parameters `A`, `P`; no chance math, no candidate semantics |
| Atomic commit | `src/world/investigation/commit.rs` | Validation order and the one transaction: verify attempt, validate find via existing Entity/Property/Trait helpers, insert Entity/placement/state/Activity/roles, consume attempt, advance pointer |
| Concrete Entity/Property/Trait/Activity rules | existing `src/world/{model,property,entity_trait,activity,common}.rs` | Keep their invariants; expose only the smallest private helper two World paths need |
| Wire | `src/wire/investigation.rs` + `wire/error.rs` | Strict inputs/outputs/conversions, canonical error mapping; no game decision |
| HTTP / MCP / composition / errors | `src/server/{http,mcp,mod,error}.rs` after the T3 split | Thin Adapters; no mechanics |
| Agent explanation | one instruction section + `tool/start_investigation.md`, `tool/submit_discovery.md` | Natural play and recovery only |
| Evidence | `src/world/investigation/test.rs`, `src/server/investigation_test.rs` (crate-internal, scripted chance), `tests/world/investigation_schema.rs`, `tests/server/**` | Split by observable rule, not by private function |

The former `authority.rs` is dropped: with the attempt id as authority there is no
separate authentication concern. A file is split only for two independent reasons to
change and merged when there is one. Architecture is an execution gate: review
rejects pass-through modules, duplicated validation, transport-owned mechanics,
cyclic imports and process-global state.

## Decisions, assumptions and open questions

### Confirmed decisions

All recorded in `docs/concept/log/2026-08.md` ("grill active" 2026-08-14 and
"grill resumed" 2026-08-15) and reflected in `docs/concept/discovery.md`:

- One complete loop is the selected edge; roll and commit are not separate outcomes.
- Discovery rule: pre-existing → investigation and roll; made/brought/placed →
  ordinary Action. Q1: one found Entity at the exact current Place with
  Agent-authored Properties/Traits; World types no kind; new Places are the second
  kind with movement.
- The earlier mutable positive payload is reopened by T1 review; see the one open
  reacceptance choice below.
- Confirmation: start needs none; the find is previewed and committed only after
  explicit User confirmation; the User holds no roll/odds/result authority.
- Zero writes no Activity and moves no pointer; unconsumed positives likewise.
- Q5: durable `investigation_attempt` row; attempt id is the opaque authority; the
  signed `roll_token` direction is superseded; no secrets.
- Chance source: `rand` OS entropy behind an injected `ChanceSource`; scripted in
  tests; reachable only through World construction.
- Q2: `ChancePolicy` component, `p = p_min + (p_max − p_min)·2^(−n/h)` on recent
  discovery density (defaults 1/2, 1/10, 6, W = 48); no pity, no time, no totals.
- Q4: `A = 12` admitted attempts per rolling hour; `P = 3` unconsumed positives,
  oldest voided beyond that; retries never count twice; hidden thresholds.
- Q3: rejects only invalid values, foreign/non-positive/consumed/voided/other-Place
  attempts (neutral), Character no longer at the attempt's Place, and request-id
  content conflicts; no name uniqueness; no revision binding.
- Q6: Activity `submit_discovery` with actor, context Place, `subject` Entity,
  `location` Place, canonical prose, `consumed_by_activity_id` as provenance link.
- Q7 operation names, routes, statuses and fifteen-capability target remain
  confirmed; the unreachable `investigation_request_conflict` error is removed in
  the recommended revision because start has no semantic input besides request id.
- Built For Massive Concurrency governs every seam of this build.
- Process: the server split stays inside T3; T0 verifies the committed baseline.
- The User explicitly accepted this complete plan on 2026-08-15.
- No paid model call is authorized.

### Reversible assumptions

- Exact wire field order, description wording and test file split are refined during
  execution without re-acceptance.
- Parameter defaults (`p_max`, `p_min`, `h`, `W`, `A`, `P`) are the accepted contract
  values; changing them later is a documented contract change, not a re-plan.
- The read-only ledger changes only if its operation rendering would misrepresent
  `submit_discovery` Activity.

### Reaccepted T1 corrections

Independent Sol Medium and Sol High T1 review exposed one material response choice:

- **Reaccepted:** `start_investigation` returns only retry-stable `zero` or
  `positive`, the attempt id and immutable result limits. After `positive`, the Agent
  re-grounds through existing exact-Place, Entity-state and Activity reads before
  authoring. The attempt drops `request_fingerprint`, and start drops the unreachable
  `investigation_request_conflict` error. This preserves byte-identical retries,
  avoids snapshots/as-of state and keeps unrelated Place changes from staling the
  attempt.
- **Rejected by the review:** persisting a complete response snapshot duplicates
  mutable World state and expands schema; recomputing fresh context on retry breaks
  byte identity and may return context remote from the attempt after future movement.

The User reaccepted this correction on 2026-08-15 and unblocked T1. The same
acceptance resolves six dependent public/operational details:

- `submit_discovery` returns exactly Activity, the found Entity and its Place; current
  state is read rather than duplicated;
- strict transport decoding plus complete prose/Entity/Property/Trait normalization
  and canonical discovery fingerprinting happen before the User lock, matching
  existing Action/Interaction idempotency. `invalid_discovery` is prose-only; the
  existing typed errors own find fields. Under the lock an equal committed retry
  returns first, changed content conflicts, then Character/attempt availability and
  database-dependent validation follow;
- start returns `character_not_found` or `character_not_entered`; submit returns
  `character_not_found` when no Character exists and neutral
  `discovery_attempt_unavailable` when it is unplaced or no longer at the attempt
  Place;
- rolling-hour admission uses PostgreSQL `statement_timestamp()`, an inclusive lower
  boundary and the same stored attempt time; FIFO is `(created_at ASC, id ASC)`;
- start request ids live in the attempt namespace. State-changing Action,
  Interaction and discovery request ids keep the existing shared Activity namespace;
  reusing one UUID once in each namespace is valid;
- T1 publishes the final accepted fifteen-capability contract in `docs/game/`
  without delivery-status prose. Temporary contract/runtime divergence exists only
  as active plan state until T3; evidence records delivery later.

The storage implementation also adds the earned partial Place-Activity ordering
index specified above so the last-`W` chance signal examines a bounded hot-Place
window. This technical correction follows the accepted concurrency invariant and
does not add another User choice.

## Implementation map

| Surface | Current state | Intended change | Invariants |
| --- | --- | --- | --- |
| `docs/concept/discovery.md`, concept log | Rationale updated with resolved frontier | Append acceptance/completion milestones only | Superseded direction stays named |
| `.agents/backlog/README.md`, `items/first-investigation-discovery-loop.md` | Sole `Now / Active` item after Sol-medium completion and plan acceptance | `Done` on exact completion evidence | At most one `Active` item |
| `docs/game/README.md`, `domain.md`, `protocol.md`, `adapter-parity.md`, `agent.md`, `storage.md`, `deferred.md`, `capability/start_investigation.md`, `capability/submit_discovery.md` | Thirteen capabilities; discovery absent | Publish the final accepted fifteen-capability contract in T1 with no delivery-status prose | Delivery state remains in plan/evidence; runtime catches up in T2/T3 |
| `CONTEXT.md` | Vocabulary stops at Trait | Add `investigation`, `attempt`, `discovery` (find) after T1 | Only canonical terms |
| `migration/0010_investigation.sql` | Schema through `0009` | Attempt relation, checks, three indexes, operation/prose CHECK extension | Immutable history; no counter or generic table |
| `src/world/investigation/{mod,model,chance,attempt,commit}.rs`, `Cargo.toml` (`rand`) | Absent | Deep private Module; two public `World` methods | World derives authority; bounded reads; User-only lock on start |
| `src/wire/investigation.rs`, `wire/{mod,error}.rs`, `lib.rs` | Thirteen-capability wire | Strict shapes and error mapping | Unknown fields reject; no internal fields |
| `src/server.rs` → `src/server/{mod,http,mcp,error}.rs` | One 1,108-line file | Behavior-preserving split, then two routes/tools, OpenAPI, catalog | Byte-equal existing catalog/OpenAPI before additions |
| `src/agent_contract/instruction/`, `tool/`, `agent_contract.rs`, `tests/agent-tool-catalog.json` | Thirteen descriptions | New section, two descriptions, fixture regenerated once | Natural in-world language |
| `src/world/investigation/test.rs`, crate-internal server investigation tests, integration schema/catalog tests | Absent | Chance-dependent evidence stays crate-internal behind the private scripted seam; public-shape evidence stays in integration tests | No probabilistic assertion or public test constructor |
| `tests/agent-playtest.sh`, `tools/agent-playtest` | Thirteen-tool fake contract | Extend for grounding, zero handling, confirmed find | Token-free |
| `web/index.html` | Generic ledger | Only if `submit_discovery` rendering would mislead | Read-only |
| `docs/evidence/discovery.md`, index | Absent | Exact run/test/review status on completion | No rules or planning state |

## Execution contract

Root owns outcome, scope, plan state, integration and the final evidence claim. The
accepted implementation tasks remain sequential because each establishes the next
task's contract or runtime seam. Root delegates each dependency-ready task to Sol
Medium or Sol High, while independent read-only review and verification may run in
parallel. Every Agent receives this plan path and exactly one eligible task id,
re-reads live files, changes only its owned surfaces, runs focused evidence and
returns raw results; only Root updates plan state and integrates the evidence claim.

No task enters `in_progress` while `status: draft`. Acceptance records
`status: active` and `accepted_at`.

## Task graph

Allowed states are `pending`, `in_progress`, `completed` and `blocked`.

| ID | State | Depends | Parallel-safe | Objective | Owned surfaces | Evidence |
| --- | --- | --- | --- | --- | --- | --- |
| T0 | completed | accepted plan | no | Verified baseline | none (verification only) | Commit `6f95ee2` is pushed with migration `0009`; format, Clippy, 119/119 PostgreSQL tests and all three shell suites passed |
| T1 | completed | T0 + reaccepted plan | no | Publish the final accepted contract without delivery-status prose | `docs/game/**`, `CONTEXT.md`, `docs/concept/discovery.md`, backlog | Sol Medium publication and Sol High independent review passed with no P0–P3 findings; links, anchors, headers, 15/15/15 catalog cardinality and `git diff --check` pass |
| T2 | completed | T1 | no | Deep private investigation Module and `0010` schema | `migration/0010_*.sql`, `src/world/investigation/**`, `src/world/mod.rs`, smallest existing helpers, `Cargo.toml`, `src/lib.rs` World-type exports, integration schema tests; compile-only exhaustive arms in `src/wire/{output,error}.rs` | Sol High implementation and Sol Medium review passed; 20/20 investigation and 86/86 World tests, format, Clippy and diff check are green; sole full-suite failure is the T3-owned 13-tool catalog fixture |
| T3 | completed | T2 | no | Server split, strict wire, HTTP/MCP/OpenAPI parity, Agent contract | `src/wire/**`, `src/server/**`, `src/agent_contract/**`, catalog fixture, integration protocol tests | Sol High implementation and Sol Medium audit passed; identical 13/13 checks bracketed the split, final fixture structural comparison preserves every old tool, 3/3 deterministic adapter tests, 14/14 server tests and 147/147 full Rust tests pass |
| T4 | completed | T3 | no | End-to-end outcome evidence without paid models | fake Agent suite, local tooling tests, ledger if earned | Seven executed isolated fake-Agent phases prove the zero/positive/confirm/commit/observer/recovery discipline; real disposable PostgreSQL restart preserves one canonical attempt; 147 Rust tests and all shell suites pass with no Codex/model call or leaked database |
| T5 | completed | T4 | no | Review, evidence, closure | `docs/evidence/**`, log, backlog, this plan | Terry closure and exact four-class evidence recorded; Standards and Spec re-reviews both GO with 0 P0–P3; backlog is Done with no active edge; plan is `complete` |

## Task details

### T0 — Verified baseline

**Objective:** Implementation starts from a committed, green baseline that includes
the uniform Entity-state build.

**Evidence:** the complete ladder passed on the exact committed tree: `cargo fmt
--all -- --check`, `cargo clippy --all-targets --all-features -- -D warnings`,
119/119 `cargo test --all-targets --all-features` tests against PostgreSQL,
`bash tests/aicadia-local.sh`, `bash tests/agent-playtest.sh` and
`bash tests/trait-playtest.sh`. Commit `6f95ee2` contains migration `0009`, is pushed
to `origin/codex/uniform-state-discovery-baseline` and left a clean working tree.

**Stop conditions:** any failure is a baseline issue outside this plan; stop and
report.

### T1 — Accepted contract published

**Objective:** The accepted behavior becomes one coherent final contract in
`docs/game/` before code changes; delivery status remains outside runtime docs.

**Actions:**

1. Add `capability/start_investigation.md` and `capability/submit_discovery.md`;
   extend `README.md` (fifteen rows), `domain.md` (operation, roles, attempt
   provenance), `protocol.md` (shapes, statuses, errors, retry identity for both
   calls, no revision binding), `adapter-parity.md` (cross-adapter evidence),
   `agent.md` (found-vs-made rule, free start,
   confirmed find, zero rendering, recovery), `storage.md` (`0010`), `deferred.md`
   (remove investigation/rolls/discovery; keep movement and new-Place discovery).
2. Publish the two capabilities without implementation-pending labels or delivery
   prose; active execution state remains in this plan and the backlog.
3. Add canonical vocabulary to `CONTEXT.md`; align `docs/concept/discovery.md`;
   keep the backlog item as the sole `Active` edge and link authorities.

**Invariants:** no runtime file changes; existing thirteen capabilities keep their
meaning; no paid-model, movement or generic-model scope enters.

**Evidence:** targeted `rg` scans (no provisional names remain), relative link check,
manual actor/action/state/ownership/history matrix.

**Stop conditions:** stop if publishing would require an undecided rule.

### T2 — Persistence and World authority

**Objective:** One World implementation admits, resolves and commits with exact
retry, concurrency and history semantics.

**Actions:**

1. Add `rand`; create `src/world/investigation/` per the ownership map; keep
   `World::new(pool)` as the public production constructor with `OsChance`, and use
   one crate-private test-only scripted-construction helper in crate-internal tests.
2. Write `migration/0010_investigation.sql` as specified, including the existing
   Trait Activity validator's explicit `submit_discovery` extension and the partial
   Place-Activity ordering index that bounds the last-`W` chance signal.
3. Implement `start_investigation`: begin → `lock_user` → retry lookup by User and
   request id → rate admission → bounded Place-window signal → `ChancePolicy` → draw
   → insert attempt → if the result is positive and exceeds `P`, FIFO-void the oldest
   prior live positive with `id <> new_attempt_id`, using the inserted attempt as
   provenance → return the stored outcome, attempt id and immutable limits.
4. Implement `submit_discovery`: normalize prose/find → canonical fingerprint →
   begin → `lock_user` → Activity-namespace retry/conflict lookup → Character lookup
   → attempt verification (own, positive, unconsumed, unvoided, current at the same
   Place) → `lock_place` → database-dependent validation via existing helpers →
   Entity, placement, state, Activity and roles → consume attempt → advance pointer
   → return exact `{activity, entity, place}`.
5. Reconstruct canonical retries for both calls before later preconditions.
6. Tests: bounds and absence; zero/positive through scripted draws; equal start retry
   returns byte-equal stable output and no second row; no unreachable start-conflict
   branch; discovery content conflict; admission rate
   window; hoarding void with provenance, explicit new-attempt exclusion and schema
   rejection of self-voiding; consumption; foreign/consumed/voided
   attempts neutral; unrelated Place Actions leave attempts valid; concurrent starts
   and concurrent commits of one attempt; rollback on every failure; history lenses
   show the find to a co-present Character; saturation signal counts only the last
   `W` Activities; query plans bounded.
7. Add only the exhaustive `submit_discovery` operation and accepted discovery-error
   mapping arms required for the existing wire crate to compile after World enum
   expansion, plus the `src/lib.rs` exports required to name the new public World
   method inputs/results. Routes, request/result wire shapes, OpenAPI, MCP and
   catalog remain T3.

**Invariants:** one `World` seam; no Place lock on start; no global state; no LLM;
no partial writes; scripted chance never reachable outside construction.

**Evidence:** crate-internal World investigation tests through the private scripted
seam, integration schema tests and raw transaction assertions.

**Stop conditions:** stop if correctness needs a session, global counter, secret or
second public seam.

### T3 — Protocol parity and Agent experience

**Objective:** HTTP and MCP expose the same accepted behavior; a conforming Agent can
play it without protocol leakage.

**Actions:**

1. Extract `src/server.rs` into `server/{mod,http,mcp,error}.rs` without changing
   any route, schema, tool, order or response; prove byte-equal catalog/OpenAPI.
2. Add `src/wire/investigation.rs` shapes and error conversion.
3. Add routes, MCP tools, OpenAPI entries, catalog ordering.
4. Rewrite the Agent instruction sections: found-vs-made rule, free start, zero
   rendering, confirmed find preview, recovery on each error; add two descriptions;
   regenerate the fixture once.
5. Cross-Adapter tests: zero, positive, post-positive re-grounding, commit, start
   retry, discovery retry/conflict, not admitted,
   attempt unavailable, strict decoding, catalog/schema, in-process two-Character
   observer outcome with scripted chance.

**Invariants:** natural, fail-closed, provider-neutral instructions; no User
mechanical control; no operational control as a tool; HTTP/MCP semantic parity.

**Evidence:** `cargo test --lib server::investigation_test -- --nocapture`; existing
catalog/protocol tests; identical 13/13 existing-server results immediately before
and after the mechanical split; fixture structural comparison limited to the two
additions plus the accepted `Activity.operation` and prose-description evolution.
Raw pre-addition catalog/OpenAPI hashes were not retained, so no byte-hash claim is
made; the independent T3 audit rates that missing artifact P3/non-blocking given the
bracketed split tests and exact final structural fixture proof.

### T4 — Complete outcome evidence

**Objective:** Demonstrate the promised loop and its honest limits without paid
models.

**Actions:** extend the token-free fake Agent contract (grounding, zero handling,
confirmed find, no fallback authority); run the full ladder; inspect the diff for
accidental expansion; verify restart preserves retry semantics on a disposable
database; confirm cleanup.

**Evidence:** `cargo fmt --all -- --check`; `cargo clippy --all-targets
--all-features -- -D warnings`; `cargo test --all-targets --all-features` on
PostgreSQL; `bash tests/aicadia-local.sh`, `bash tests/agent-playtest.sh`,
`bash tests/trait-playtest.sh`.

**Stop conditions:** any probabilistic flake, baseline failure, leaked secret,
model invocation or claim/evidence mismatch.

### T5 — Terry review, evidence and closure

**Objective:** Close only when outcome, contract, implementation, planning state and
evidence agree and the next risk is explicit.

**Actions:** Terry five-question and ownership review; fix P0–P3; write
`docs/evidence/discovery.md` and its index; append the completion log entry; mark the
item `Done`; record raw validation totals; set `complete`.

**Completion evidence:** [discovery evidence](../../../docs/evidence/discovery.md)
records the Terry review, exact 147-test and shell-ladder results, separate
runtime/adapter/fake-Agent/static-validator claims and the no-paid-model boundary.
Sol High Standards and Sol Medium Spec re-reviews both returned GO with zero P0–P3
after the runner, concept, backlog and evidence authorities were corrected.

**Invariants:** no paid-model, fun-quality, movement or repeat-play claim; no stale
authority.

## Validation ladder

1. **Focused:** World/schema and server investigation filters; Agent-contract pins;
   scripted zero/positive, retry, admission, voiding, consumption, conflict,
   rollback, restart and observer scenarios.
2. **Contract:** full PostgreSQL-backed suite; World/HTTP/MCP/OpenAPI catalog,
   schema and error parity; Agent/local/Trait suites; existing capabilities green
   except the accepted `Activity.operation` evolution.
3. **Outcome:** one clean-room fake Agent grounds, starts, handles zero and positive,
   confirms and commits one find; a co-present Character reads it; equal retries
   never reroll; a later admitted request rolls again without World change.
4. **Integrity:** `git diff --check`, `cargo fmt --all -- --check`, focused diff and
   ownership review, no secret or transcript stored, every choice recorded once.

## Change control

Refine paths, ordering and evidence in place while the accepted outcome and contract
stay unchanged. Stop, set `draft`, revise and re-accept when evidence changes the
outcome, public behavior, domain meaning, actor/authority split, zero/admission/
chance semantics, result kind, history, irreversible state, dependency set, material
cost or evidence claim.

## Completion conditions

- this revised plan was explicitly reaccepted (`active`) before implementation;
- T0–T5 `completed` and the ladder passes on real PostgreSQL;
- the exact outcome and evidence claim are demonstrated without paid-model claims;
- actor, action, state, ownership, nomenclature, history, World/HTTP/MCP behavior and
  Agent presentation agree in their owning authorities;
- one public World Interface, thin Adapters, no cyclic private dependencies, no
  duplicated invariant, no touched file with two independent reasons to change;
- the backlog item is `Done`, no active edge remains, the next risk is explicit;
- no stale authority, inherited failure, secret, process or unrelated change remains;
- `status: complete` and `completed_at` recorded only after these conditions.
