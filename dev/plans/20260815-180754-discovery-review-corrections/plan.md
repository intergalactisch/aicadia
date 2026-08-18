---
status: complete
created_at: "2026-08-15T18:07:54+02:00"
updated_at: "2026-08-15T22:25:00+02:00"
accepted_at: "2026-08-15T20:56:59+02:00"
completed_at: "2026-08-15T22:25:00+02:00"
---

# Discovery loop review corrections

> **Role / side:** bounded correction plan / development side.
> **Authority:** defines the exact defects, corrections, decisions and evidence for closing the independent review of the delivered first investigation-and-discovery loop.
> **Excludes:** the discovery game contract itself, new mechanics and delivery evidence; those remain in `game/docs/`, the concept log and `dev/docs/evidence/discovery.md`.

## Outcome

The delivered investigation-and-discovery loop becomes exactly what its accepted
contract says, with no duplicated invariant, no unproven public error, no plural
Aicadia name, one home per parameter, and Agent texts that follow the published
template. Player-visible behavior is unchanged except for two deliberate contract
corrections that the User accepts with this plan: a discovery retry with different
Property/Trait content can never be mistaken for an equal retry, and the
investigation result field `limits` becomes singular `limit`.

The intended evidence claim is:

> Every P1–P3 finding of the 2026-08-15 review is either corrected in its owning
> surface or explicitly rejected with reason; the complete validation ladder passes
> on PostgreSQL; the catalog fixture changes only by the accepted rename and
> description corrections; and no authority carries a stale count, test name,
> planning label or duplicated parameter.

## Non-goals

- No change to zero/positive semantics, admission thresholds, chance formula
  values, hoarding rule, payload content, confirmation boundary or operation names.
- No reopening of the reaccepted "positive = permission, then re-read" payload.
- No compatibility path for the corrected discovery fingerprint: the discovery
  build exists only in local commit `60ce27c`, has never run outside
  disposable/local databases and has no shipped rows (Current Means Current).
- No refactor of pre-existing Action/Interaction fingerprints, tests or docs beyond
  the touched seams; no server-wide restructuring beyond the composition nits named.
- No paid model call.

## Evidence and authorities

| Evidence or authority | Current fact | Consequence |
| --- | --- | --- |
| Review of 2026-08-15 (three independent readers, ladder rerun by Root: fmt, Clippy, 147/147 PostgreSQL tests, three shell suites green) | 0 P0, 1 P1, 7 P2, ~14 P3; delivered code and docs otherwise match the accepted contract | Correct, do not redesign |
| `game/src/world/investigation/model.rs:130-158` vs `game/src/world/activity.rs:3-40,284-323` | Discovery fingerprint re-implements helpers and omits the list tag the Action scheme emits before Traits | Property/Trait content collision → wrong equal-retry verdict; reuse existing helpers |
| `AGENTS.md` — Singular Domain Names, One Home Per Truth, Earn Your Spot, Current Means Current, Agent Capability Parity | `limits` is the only plural Aicadia field; parameters in four doc homes; two dead branches; unproven public error | Named corrections below |
| `game/docs/agent.md` (fixed tool template) | `tool/start_investigation.md` invents labels and lacks `Input meaning`; recovery for the three new codes duplicated in three places | Conform to the template; one recovery owner |
| `game/src/wire/test.rs:149-207`, `game/src/server/error.rs:35` | Hand-maintained code list without exhaustiveness; `_ =>` wildcard maps unknown codes to 400 | Make both compiler-checked |
| Commit `60ce27c` | The discovery delivery is committed on top of `6f95ee2`; working tree clean | Corrections land as a reviewable diff on that fixed baseline |

## Alignment

### Strategic

No new game value; this retires concrete correctness and ownership debt on the
just-delivered defining capability so the next edge (Place neighborhood/movement)
starts from a clean, exact discovery contract.

### Tactical

The corrections, by owner:

1. **Fingerprint (P1 + P2-3):** `discovery_fingerprint` uses the existing
   `fingerprint_field`, `fingerprint_property_input` and `fingerprint_trait_input`
   helpers (widened to `pub(super)`), emits a fixed length-prefixed tag **and the
   item count** before each list (`property`, then `trait`) so the field sequence is
   self-delimiting—a bare tag is not enough because a Property key may be `trait`
   and a Trait statement `text`—and keeps its own domain tag
   `aicadia-submit-discovery-fingerprint-v1`. The Action scheme's conditional
   `initial_trait_v1` tag is a historical-compatibility shape and is not copied. A
   World test proves the reported collision pair now conflicts.
2. **Chance/attempt owners (P2-4, P3):** `ChancePolicy::resolve(count, draw)`
   returns the outcome so resolution lives in the chance component; the orchestrator
   drops the unreachable draw-range check and the redundant Place comparison; a test
   proves an in-window discovery lowers the outcome (draw 0.49 → Zero); the exact
   rolling-hour window and the actual hoarding/admission statements are proved
   through production SQL constants from crate-internal tests (the existing
   `property_query_count_test` pattern; `tests/world/` sees no private SQL and no
   public World surface is added); the exact microsecond inclusivity is not
   reachable through `statement_timestamp()` and is protected by the shared constant
   instead; `commit.rs` reuses the existing placed-Entity lookup; `Cargo.toml`
   dependency order restored.
3. **Wire/server (P2-1, P2-2, P3):** field `limits` → `limit` in wire, OpenAPI,
   MCP schema, docs, capability doc, protocol, fake-Agent runner/schemas and
   fixture; wire error-code test becomes an exhaustive `match`; HTTP status mapping
   loses the `_ =>` wildcard and names every code; adapter tests cover
   `invalid_discovery`, `invalid_entity`, `invalid_property`, `invalid_trait` and
   `property_key_conflict` on `POST /api/discovery` and its MCP twin, both discovery
   errors on both adapters, `character_not_found`/`character_not_entered` on start,
   and HTTP retry `201`; `game/src/server/mod.rs` keeps only composition (MCP-only
   items move to `mcp.rs`, ledger constant to `http.rs`).
4. **Agent contract (P3):** `tool/start_investigation.md` follows the fixed
   template (`What it does · Use it when · Before you call · Input meaning · After
   the call · On failure · Never`); code-specific recovery for the three new codes
   moves to `instruction/15-recovery.md`; both tool descriptions keep one short
   generic failure line; `14-investigation.md` is wrapped like its siblings; the
   fixture is regenerated once at the end of the task and the diff reviewed.
5. **Docs (P2-5, P2-6, P2-7, P3):** chance/admission parameters and formula get
   one home in `game/docs/domain.md` (Investigation subject); `protocol.md`,
   `storage.md` and `capability/start_investigation.md` link instead of restating
   numbers; `capability/start_investigation.md` gains `## Validation`;
   `dev/docs/evidence/property.md` names the current test and describes fifteen
   descriptions; `dev/docs/evidence/entity-state.md` drops the planning label;
   `dev/docs/concept/discovery.md` "Prototype under evaluation" gets its supersession
   banner; `domain.md` "Required evidence" collapses to a pointer at
   `adapter-parity.md`; the completed discovery plan's ownership row (`plan.md:255`)
   is corrected to the crate-internal test homes.

### Technical

- **World:** `game/src/world/investigation/{model,chance,attempt,commit,mod,test}.rs`,
  `game/src/world/activity.rs` (helper visibility only), `tests/world/investigation_schema.rs`.
- **Wire/server:** `game/src/wire/{investigation,test,error}.rs`, `game/src/server/{mod,http,mcp,error,investigation_test}.rs`,
  `tests/server/protocol.rs`, `game/mcp/tool-catalog.json`.
- **Agent:** `game/mcp/agent/instruction/{14-investigation,15-recovery}.md`,
  `game/mcp/agent/tool/{start_investigation,submit_discovery}.md`,
  `game/mcp/agent.rs` pins, `dev/playtest/agent/run`, `dev/playtest/agent/schema/*`,
  `dev/tests/agent-playtest.sh`.
- **Docs:** `game/docs/{domain,protocol,storage,adapter-parity}.md`,
  `game/docs/capability/start_investigation.md`, `dev/docs/evidence/{property,entity-state,discovery}.md`,
  `dev/docs/concept/discovery.md`, concept log, `dev/plans/20260814-204007-.../plan.md:255`.
- **Not applicable:** schema/migration (no relation change), storage locks,
  authentication, operations.

## Decisions, assumptions and open questions

### Decisions accepted with this plan

- Fingerprint correction ships without a compatibility path (no shipped rows).
- `limits` → `limit` (singular; alternative "flatten into `result_count`/`kind`"
  rejected because the grouping conveys the immutable envelope). Public contract
  change, catalog fixture regenerated.
- HTTP status mapping and wire code list become exhaustive; a future code without an
  explicit status fails compilation.
- `game/docs/domain.md` is the single home of chance/admission parameters.
- Recovery for `investigation_not_admitted`, `discovery_attempt_unavailable` and
  `discovery_request_conflict` is owned by `instruction/15-recovery.md`.

### Explicitly rejected review items (with reason)

- Moving the bounded Place-window read out of `attempt.rs` into a new file: one
  query does not earn a module; the ownership row is reworded instead.
- Splitting `protocol.md` merely for the 400-line bound: it shrinks by moving the
  parameters out.

### Open questions

None material.

## Implementation map

| Surface | Current state | Intended change | Invariants |
| --- | --- | --- | --- |
| `game/src/world/investigation/model.rs`, `activity.rs` | Duplicated fingerprint helpers, no list tags | Reuse helpers, add tags, keep v1 domain tag | Byte-equal retries still equal; changed content conflicts |
| `chance.rs`, `mod.rs` | Resolution in orchestrator; dead branches | `ChancePolicy::resolve`; remove branches | Formula, defaults, injection unchanged |
| `attempt.rs`, `commit.rs`, schema tests | Re-typed test SQL; duplicated lookup | Expose production SQL constants; reuse lookup | Bounded reads, indexes unchanged |
| `game/src/wire/investigation.rs`, `wire/test.rs`, `server/error.rs` | `limits`; hand list; wildcard | `limit`; exhaustive match; explicit statuses | Same codes/statuses as `protocol.md` |
| `game/src/server/{mod,http,mcp}.rs` | Shared items in `mod.rs` | Composition-only `mod.rs` | Byte-equal routes/tools/OpenAPI |
| `game/src/server/investigation_test.rs`, `tests/server/protocol.rs` | Single-sided/missing cases | Add named adapter tests | No probabilistic assertion |
| Agent contract sources, fixture, runner | Off-template description; triplicated recovery | Template-conformant; one recovery owner; regenerate fixture once | Natural in-world language |
| `game/docs/**`, evidence, concept, old plan row | Four parameter homes; stale pointers/labels | One home; corrected pointers; banner | No delivery-status prose in `game/docs/` |
| Concept log | — | One entry per accepted correction | Append-only |

## Execution contract

Root owns scope, plan state, integration and the final claim. On 2026-08-15 the User chose delegation: T1–T3 each go to one sub-agent (T1 on the session model, T2/T3 may use Opus) with this plan path and one task id; the agent re-reads live files, changes only its owned surfaces, never edits this plan, runs the task's focused evidence and returns raw results. Root reviews the diff, reruns focused evidence and sets task state before the next task starts. No task starts while `draft`.

## Task graph

Allowed states are `pending`, `in_progress`, `completed` and `blocked`.

| ID | State | Depends | Parallel-safe | Objective | Owned surfaces | Evidence |
| --- | --- | --- | --- | --- | --- | --- |
| T0 | completed | accepted plan | no | Baseline recorded | none | User committed the discovery delivery as `60ce27c` before this correction; Root's ladder (fmt, Clippy, 147/147 PostgreSQL tests, three shell suites) passed on that exact tree; working tree clean |
| T1 | completed | T0 | no | World corrections | `game/src/world/**`, `tests/world/investigation_schema.rs`, `Cargo.toml` | Fingerprint = helpers + tag + item count per list; both collision pairs (original and tag-lookalike) conflict; in-window saturation lowers outcome; EXPLAIN plans on production SQL constants and rolling-hour width proved in-crate; Root reran `--lib world::investigation` 25/25, `--test world` 84/84, Action fingerprint tests unchanged, Clippy/fmt/`git diff --check` clean |
| T2 | completed | T1 | no | Wire/server corrections | `game/src/wire/**`, `game/src/server/**`, `tests/server/**`, runner/schemas, fixture (+ World `InvestigationLimit`/`limit` rename) | Fixture diff = exactly the four `limits`→`limit` lines; wire code list and HTTP status map are wildcard-free exhaustive matches; every published discovery rejection, both start Character errors and HTTP retry 201 proven on both adapters; `server/mod.rs` composition-only; Root reran server 6/6 + 15/15, wire 5/5, agent_contract 6/6, fake-Agent suite, Clippy/fmt/`git diff --check` clean |
| T3 | completed | T2 | no | Agent contract and docs | `game/mcp/agent/**`, fixture, `docs/**`, old plan row | `tool/start_investigation.md` on the fixed template with `Input meaning`; three discovery recoveries owned by `15-recovery.md` only; `14-investigation.md` wrapped; `domain.md#investigation-chance-and-admission` is the one parameter home (values verified against code), protocol/storage/capability link to it; `## Validation` added; nine domain-only evidence obligations moved into `adapter-parity.md`; evidence pointers/labels fixed; prototype banner; old plan row corrected; fixture diff = two descriptions + rename; 92 links/anchors resolve; agent_contract 6/6, server protocol 6/6, fake-Agent suite green |
| T4 | completed | T3 | no | Ladder, trail, closure | concept log, `dev/docs/evidence/discovery.md`, `dev/playtest/trait/schema/live-candidate.sha256`, this plan | Full ladder green: fmt, Clippy, `git diff --check`, 152/152 PostgreSQL tests (51 lib, 2 helper, 15 server, 84 World), fake-Agent, Trait (digest refrozen for changed sources) and local suites; concept example on current spelling; log entry; evidence section; plan `complete` |

## Task details

### T1 — World corrections

1. Widen `fingerprint_field`, `fingerprint_property_input`, `fingerprint_trait_input`
   to `pub(super)`; rewrite `discovery_fingerprint` to: domain tag, attempt id,
   prose, name, description, tag `property`, property list, tag `trait`, trait list.
2. Add `ChancePolicy::resolve(discovery_count: u32, draw: f64) -> InvestigationOutcome`;
   orchestrator calls it; delete the range check and the redundant comparison.
3. Expose admission/hoarding/window SQL as `pub(super)`/crate constants used by both
   production and `tests/world/investigation_schema.rs`; replace re-typed predicates;
   add the exact boundary test through `start_investigation` (insert attempts with
   chosen `created_at`, then call the production path).
4. Add tests: collision pair → `DiscoveryRequestConflict`; in-window discovery with
   draw 0.49 → Zero; empty-vs-non-empty list distinction.
5. Reuse the existing placed-Entity lookup in `commit.rs`; restore `Cargo.toml` order.

**Invariants:** no behavior change other than the fingerprint separator; no new
public World surface; scripted chance still test-only.

**Evidence:** `cargo test --lib world::investigation`, `cargo test --test world`,
`cargo clippy -D warnings`.

**Stop:** if any existing fingerprint test for Actions/Interactions changes.

### T2 — Wire/server corrections

1. Rename `limits` → `limit` in `game/src/wire/investigation.rs` (struct name
   `InvestigationLimitOutput`), OpenAPI/MCP schemas follow; update
   `game/src/server/investigation_test.rs`, `dev/playtest/agent/run` (result keys), runner
   schemas, `dev/tests/agent-playtest.sh`.
2. `game/src/wire/test.rs`: replace the list with an exhaustive `match ErrorCode` →
   spelling; `game/src/server/error.rs`: exhaustive status match, no wildcard.
3. Add adapter tests: typed find errors and `invalid_discovery` on both adapters;
   `discovery_request_conflict` and `discovery_attempt_unavailable` on both;
   start `character_not_found`/`character_not_entered`; HTTP retry `201`.
4. Move MCP-only constants/helpers to `mcp.rs`, `LEDGER_HTML` to `http.rs`;
   `mod.rs` = Router/adapter wiring and `app()` only.
5. Regenerate the fixture; review that the diff is exactly the rename.

**Evidence:** `cargo test --lib server`, `cargo test --test server`, fixture diff.

### T3 — Agent contract and docs

1. Rewrite `tool/start_investigation.md` on the fixed template with `Input meaning`;
   shorten both tool failure lines to the generic form; move the three recoveries
   into `instruction/15-recovery.md`; wrap `14-investigation.md`; update
   `agent_contract.rs` pins; regenerate the fixture once; review the diff.
2. `game/docs/domain.md`: one Investigation parameters/formula block; `protocol.md`,
   `storage.md`, `capability/start_investigation.md` link to it and drop numbers;
   `capability/start_investigation.md` gains `## Validation`; `domain.md` "Required
   evidence" → pointer to `adapter-parity.md`; rename `limits`→`limit` in all docs
   and `dev/CONTEXT.md` if present.
3. `dev/docs/evidence/property.md`, `dev/docs/evidence/entity-state.md`,
   `dev/docs/concept/discovery.md` prototype banner, old plan `plan.md:255`.

**Evidence:** `rg -n "limits|thirteen_trait|Now / Proposed"` clean in the touched
authorities; relative link check; `cargo test --lib agent_contract`;
`bash dev/tests/agent-playtest.sh`.

### T4 — Ladder, trail, closure

Full ladder (`fmt`, `clippy`, `cargo test --all-targets --all-features` on
PostgreSQL, three shell suites); concept-log entries (fingerprint correction, `limit`
rename, parameter home, recovery ownership, exhaustive error mapping); update
`dev/docs/evidence/discovery.md` counts and limitations; set `complete`.

## Validation ladder

1. **Focused:** new World/server tests named above.
2. **Contract:** full suite; fixture diffs limited to rename + descriptions;
   OpenAPI/MCP parity; three shell suites.
3. **Outcome:** collision pair conflicts; every published discovery error is
   proven on both adapters; one parameter home; template-conformant descriptions.
4. **Integrity:** `git diff --check`, focused diff review, no unrelated change.

## Change control

Stop, return to `draft` and re-accept if any correction would change zero/positive
semantics, thresholds, formula values, payload, names beyond `limit`, or history.

## Completion conditions

- T0–T4 `completed`; ladder green on PostgreSQL;
- every review finding corrected or explicitly rejected here with reason;
- no stale authority, duplicated parameter, off-template description or unproven
  public error remains;
- `status: complete` and `completed_at` recorded.
