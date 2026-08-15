---
status: active
created_at: "2026-08-14T20:40:07+02:00"
updated_at: "2026-08-15T11:33:23+02:00"
accepted_at: "2026-08-15T11:26:34+02:00"
completed_at: null
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

Every material question is resolved and recorded in the concept log. The User
explicitly accepted this plan on 2026-08-15. T0's validation ladder is green but the
verified uniform-state baseline is not committed, so T0 is blocked and no runtime
or `docs/game/` change starts before the User records that baseline.

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
- Authentication, OAuth, public hosting, non-loopback operation or production
  rate-limiting infrastructure beyond the per-User admission rows below.
- Any paid model call.

## Evidence and authorities

| Evidence or authority | Current fact | Consequence for this build |
| --- | --- | --- |
| `AGENTS.md` — Terry, Game Progress First, Plan Before Build, Built For Massive Concurrency, Every World Action Leaves History | The next build must deliver the highest-value smallest complete game outcome, must not preclude millions of concurrent Characters, and stores exact history atomically | Start takes no Place lock, every read is bounded, admission and saturation are per-User/per-Place; the accepted find writes Activity in the same transaction |
| User choices 2026-08-14/15 (concept log, "grill active" and "grill resumed") | Discovery rule, Q1–Q7, confirmation boundary, zero semantics, attempt representation, chance component, admission and names are decided | This plan folds them in; acceptance of the plan authorizes T1–T5 |
| `.agents/backlog/README.md` | Sol-medium validation is complete; this loop is the sole `Now / Active` item | Keep one active edge and close it only after the exact discovery evidence claim passes |
| `docs/game/README.md`, `deferred.md`, `protocol.md`, `domain.md`, `storage.md` | Thirteen capabilities; investigation, rolls and discovery absent; closed `Activity.operation` enum; `(requested_by_user_id, request_id)` retry index on `activity` only; `place.latest_activity_id` is the exact-Place revision; User→Place lock order | The catalog becomes fifteen; `Activity.operation` gains `submit_discovery` (a schema change visible in every Activity-returning tool); attempts need their own retry key; the commit reuses the Place pointer advance |
| `docs/game/agent.md` | Every mutating workshop tool requires a complete natural preview and explicit User confirmation; confirmation is a contract obligation, not a wire field | `submit_discovery` inherits that obligation; `start_investigation` is not a player-visible World change and needs none |
| `docs/concept/discovery.md` and log "Stochastic discovery rolls" | Confirmed: World-first resolution, no pity, several scope-bound attempts, Character-wide admission; the signed `roll_token` transport direction is now superseded by the durable attempt row | Concept record is updated in the same change; superseded direction is named, not erased |
| `src/world/{mod,common,mutation,activity}.rs`, `Cargo.toml` | `World` is a struct over `PgPool` (`mod.rs:104`), `lock_user`/`lock_place` (`common.rs:3,30`), fingerprints via `sha2`, `uuid` v4; no `rand`, no `hmac`; `src/world/mod.rs` has 141 production lines, `src/server.rs` 1,108 unsplit | The investigation Module composes existing helpers; `rand` is the one new crate; the server split remains part of T3 |
| Migrations `0001`–`0009` | `0009_uniform_entity_state.sql` is the latest (uncommitted) | Discovery is `0010` |
| Working tree | The complete uniform Entity-state build, its evidence and plans are uncommitted; format, Clippy, 119/119 PostgreSQL tests and all three shell suites pass | T0 remains blocked until the User commits that exact baseline; no baseline failure may be attributed to this build |

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
   attempt. Only a new positive beyond the hoarding cap then voids the oldest live
   positive by referencing the inserted attempt. No Place lock, Activity or pointer
   change occurs.
4. `zero`: the Agent renders one honest unsuccessful search in-world and continues
   with ordinary play or a later investigation. `positive`: the response carries the
   attempt id, generic limits and bounded context (safe Place view, the Place's own
   Property/Trait state, the accepted discoveries among the last W Place Activities).
5. The Agent authors exactly one find—an ordinary Entity with name, description and
   0–100 initial Properties/Traits—coherent with the context, previews it completely
   in the User's language and obtains explicit confirmation. The User advises but
   never selects focus, odds, roll or result.
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
`request_id` returns the identical body without rolling; a different body under the
same id conflicts; a rejected admission creates no attempt; a fourth unconsumed
positive voids the oldest; a consumed, voided, foreign or other-Place attempt fails
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
    request_fingerprint BYTEA NOT NULL CHECK octet_length = 32,
    character_entity_id FK character NOT NULL,
    place_entity_id FK place NOT NULL,
    outcome TEXT NOT NULL CHECK IN ('zero','positive'),
    consumed_by_activity_id UUID NULL UNIQUE FK activity,
    voided_by_attempt_id UUID NULL FK investigation_attempt,
    created_at timestamptz NOT NULL,
    CHECK (outcome = 'positive' OR (consumed_by_activity_id IS NULL AND voided_by_attempt_id IS NULL)),
    CHECK (consumed_by_activity_id IS NULL OR voided_by_attempt_id IS NULL)
  )
  UNIQUE (requested_by_user_id, request_id)                       -- retry identity
  INDEX  (requested_by_user_id, created_at DESC)                  -- admission window
  partial INDEX (requested_by_user_id, created_at)
    WHERE outcome = 'positive' AND consumed_by_activity_id IS NULL
      AND voided_by_attempt_id IS NULL                            -- hoarding cap
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
  Place—one bounded read that also yields the payload's `recent_discovery` list.
- **Admission:** under the User lock, resolve retry then count attempts in the rolling
  hour (`A = 12`) and reject before rolling when full. After the draw and attempt
  insert, only a new positive that exceeds `P = 3` unconsumed positives voids the
  oldest live positive and points it at the now-existing new attempt. Parameters are
  typed constants in the attempt owner; changing them is a documented contract
  change.
- **Concurrency:** start locks User only; commit locks User then Place (existing
  order); attempts bind no `place_revision`; unrelated Place changes never stale an
  attempt; conflicting commits of the same attempt serialize on the User lock and
  the second sees a consumed attempt.
- **Idempotency:** both operations fingerprint normalized input with the existing
  SHA-256 scheme (`find.property` sorted by canonical key, `find.trait` by
  statement); equal retries return canonical bodies; different content conflicts.
- **Errors:** `investigation_not_admitted` (429), `investigation_request_conflict`
  (409), `discovery_attempt_unavailable` (409, neutral), `discovery_request_conflict`
  (409), `invalid_discovery` (400) beside existing `character_not_found`,
  `character_not_entered`, `invalid_entity`, `invalid_property`, `invalid_trait`,
  `property_key_conflict`, `invalid_request` and `unavailable`.
- **Adapters:** `POST /api/investigation` (200) and `POST /api/discovery` (201);
  MCP tools with the same names; OpenAPI, compiler-generated schemas, descriptions
  and the checked-in catalog fixture change together. The `Activity.operation` enum
  change is an accepted fixture change for every Activity-returning tool.
- **Agent experience:** new instruction section distinguishing found from made,
  free start, confirmed commit, zero rendering, no ids in play; two tool
  descriptions in the fixed template.
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
| Evidence | `tests/world/investigation/`, `tests/server/investigation/` | Split by observable rule, not by private function |

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
- Positive payload: attempt id, generic limits, bounded context (Place, Place state,
  discoveries among the last `W` Place Activities). No counts, ranking or prose.
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
- Q7: `start_investigation` / `submit_discovery`, routes, statuses and error codes as
  above; fifteen capabilities.
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

### Open questions

None material. The User accepted this plan on 2026-08-15.

## Implementation map

| Surface | Current state | Intended change | Invariants |
| --- | --- | --- | --- |
| `docs/concept/discovery.md`, concept log | Rationale updated with resolved frontier | Append acceptance/completion milestones only | Superseded direction stays named |
| `.agents/backlog/README.md`, `items/first-investigation-discovery-loop.md` | Sole `Now / Active` item after Sol-medium completion and plan acceptance | `Done` on exact completion evidence | At most one `Active` item |
| `docs/game/README.md`, `domain.md`, `protocol.md`, `agent.md`, `storage.md`, `deferred.md`, `capability/start_investigation.md`, `capability/submit_discovery.md` | Thirteen capabilities; discovery absent | Publish the accepted contract as implementation-pending target in T1, marked so until T3 ships | Truthful about executable status at every step |
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
| T0 | blocked | accepted plan | no | Verified baseline | none (verification only) | Validation is green; the User still needs to commit the uniform build so the evidence applies to a fixed baseline |
| T1 | pending | T0 | no | Publish the accepted contract as implementation-pending target | `docs/game/**`, `CONTEXT.md`, `docs/concept/discovery.md`, backlog | Zero open questions; capability, wire, storage, history, error and Agent contract documented; item `Ready` |
| T2 | pending | T1 | no | Deep private investigation Module and `0010` schema | `migration/0010_*.sql`, `src/world/investigation/**`, `src/world/mod.rs`, smallest existing helpers, `Cargo.toml`, integration schema tests | World tests prove admission, zero/positive, retry, voiding, consumption, atomic commit, history, concurrency, restart |
| T3 | pending | T2 | no | Server split, strict wire, HTTP/MCP/OpenAPI parity, Agent contract | `src/wire/**`, `src/server/**`, `src/agent_contract/**`, catalog fixture, integration protocol tests | Pre/post split byte-equal; parity, errors, catalog, two-Character observer test green |
| T4 | pending | T3 | no | End-to-end outcome evidence without paid models | fake Agent suite, local tooling tests, ledger if earned | Clean-room fake Agent grounds, handles zero, confirms and commits a find; full ladder green |
| T5 | pending | T4 | no | Review, evidence, closure | `docs/evidence/**`, log, backlog, this plan | No unresolved P0–P3; exact claim recorded; `complete` |

## Task details

### T0 — Verified baseline

**Objective:** Implementation starts from a committed, green baseline that includes
the uniform Entity-state build.

**Actions:** the complete ladder passed on the pending working tree: `cargo fmt
--all -- --check`, `cargo clippy --all-targets --all-features -- -D warnings`,
119/119 `cargo test --all-targets --all-features` tests against PostgreSQL,
`bash tests/aicadia-local.sh`, `bash tests/agent-playtest.sh` and
`bash tests/trait-playtest.sh`. The User must now commit that uniform-state baseline
(Root never commits unasked); Root then verifies HEAD contains migration `0009` and
the working tree introduces no unrelated delta before marking T0 complete.

**Current blocker:** HEAD `d720dff` contains migrations only through `0008`; the
uniform-state implementation, `0009`, tests, plans and evidence remain uncommitted.

**Stop conditions:** any failure is a baseline issue outside this plan; stop and
report.

### T1 — Accepted contract published

**Objective:** The accepted behavior becomes one coherent implementation-pending
contract in `docs/game/` before code changes.

**Actions:**

1. Add `capability/start_investigation.md` and `capability/submit_discovery.md`;
   extend `README.md` (fifteen rows), `domain.md` (operation, roles, attempt
   provenance), `protocol.md` (shapes, statuses, errors, retry identity for both
   calls, no revision binding), `agent.md` (found-vs-made rule, free start,
   confirmed find, zero rendering, recovery), `storage.md` (`0010`), `deferred.md`
   (remove investigation/rolls/discovery; keep movement and new-Place discovery).
2. Mark the two capabilities as accepted, implementation-pending until T3 ships;
   keep every executable claim truthful.
3. Add canonical vocabulary to `CONTEXT.md`; align `docs/concept/discovery.md`;
   set the backlog item to `Ready` and link authorities.

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
   Trait Activity validator's explicit `submit_discovery` extension.
3. Implement `start_investigation`: fingerprint → begin → `lock_user` → retry lookup
   → rate admission → bounded Place-window signal → `ChancePolicy` → draw → insert
   attempt → if the result is positive and exceeds `P`, FIFO-void the oldest live
   positive with the inserted attempt as provenance → return zero or positive with
   context.
4. Implement `submit_discovery`: fingerprint → begin → `lock_user` → retry lookup →
   attempt verification (own, positive, unconsumed, unvoided, same Place as current)
   → `lock_place` → find validation via existing helpers → Entity, placement,
   Properties, Traits, Activity, roles → consume attempt → advance pointer → return.
5. Reconstruct canonical retries for both calls before later preconditions.
6. Tests: bounds and absence; zero/positive through scripted draws; equal retry
   returns byte-equal output and no second row; content conflict; admission rate
   window; hoarding void with provenance; consumption; foreign/consumed/voided
   attempts neutral; unrelated Place Actions leave attempts valid; concurrent starts
   and concurrent commits of one attempt; rollback on every failure; history lenses
   show the find to a co-present Character; saturation signal counts only the last
   `W` Activities; query plans bounded.

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
5. Cross-Adapter tests: zero, positive, commit, retry/conflict, not admitted,
   attempt unavailable, strict decoding, catalog/schema, in-process two-Character
   observer outcome with scripted chance.

**Invariants:** natural, fail-closed, provider-neutral instructions; no User
mechanical control; no operational control as a tool; HTTP/MCP semantic parity.

**Evidence:** `cargo test --test server investigation`; existing catalog/protocol
tests; pre/post split comparison; fixture diff limited to the two additions plus the
`Activity.operation` enum.

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

- this plan was explicitly accepted (`active`) before implementation;
- T0–T5 `completed` and the ladder passes on real PostgreSQL;
- the exact outcome and evidence claim are demonstrated without paid-model claims;
- actor, action, state, ownership, nomenclature, history, World/HTTP/MCP behavior and
  Agent presentation agree in their owning authorities;
- one public World Interface, thin Adapters, no cyclic private dependencies, no
  duplicated invariant, no touched file with two independent reasons to change;
- the backlog item is `Done`, no active edge remains, the next risk is explicit;
- no stale authority, inherited failure, secret, process or unrelated change remains;
- `status: complete` and `completed_at` recorded only after these conditions.
