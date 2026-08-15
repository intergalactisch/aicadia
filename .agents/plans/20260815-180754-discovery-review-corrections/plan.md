---
status: draft
created_at: "2026-08-15T18:07:54+02:00"
updated_at: "2026-08-15T18:07:54+02:00"
accepted_at: null
completed_at: null
---

# Discovery loop review corrections

> **Role / side:** bounded correction plan / development side.
> **Authority:** defines the exact defects, corrections, decisions and evidence for closing the independent review of the delivered first investigation-and-discovery loop.
> **Excludes:** the discovery game contract itself, new mechanics and delivery evidence; those remain in `docs/game/`, the concept log and `docs/evidence/discovery.md`.

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
  build is uncommitted, has never run outside disposable/local databases and has no
  shipped rows (Current Means Current).
- No refactor of pre-existing Action/Interaction fingerprints, tests or docs beyond
  the touched seams; no server-wide restructuring beyond the composition nits named.
- No paid model call.

## Evidence and authorities

| Evidence or authority | Current fact | Consequence |
| --- | --- | --- |
| Review of 2026-08-15 (three independent readers, ladder rerun by Root: fmt, Clippy, 147/147 PostgreSQL tests, three shell suites green) | 0 P0, 1 P1, 7 P2, ~14 P3; delivered code and docs otherwise match the accepted contract | Correct, do not redesign |
| `src/world/investigation/model.rs:130-158` vs `src/world/activity.rs:3-40,284-323` | Discovery fingerprint re-implements helpers and omits the list tag the Action scheme emits before Traits | Property/Trait content collision → wrong equal-retry verdict; reuse existing helpers |
| `AGENTS.md` — Singular Domain Names, One Home Per Truth, Earn Your Spot, Current Means Current, Agent Capability Parity | `limits` is the only plural Aicadia field; parameters in four doc homes; two dead branches; unproven public error | Named corrections below |
| `docs/game/agent.md` (fixed tool template) | `tool/start_investigation.md` invents labels and lacks `Input meaning`; recovery for the three new codes duplicated in three places | Conform to the template; one recovery owner |
| `src/wire/test.rs:149-207`, `src/server/error.rs:35` | Hand-maintained code list without exhaustiveness; `_ =>` wildcard maps unknown codes to 400 | Make both compiler-checked |
| Working tree | Discovery delivery (T1–T5 of the accepted plan) is uncommitted on top of `6f95ee2` | The User decides whether to commit before or after this correction; T0 records the choice |

## Alignment

### Strategic

No new game value; this retires concrete correctness and ownership debt on the
just-delivered defining capability so the next edge (Place neighborhood/movement)
starts from a clean, exact discovery contract.

### Tactical

The corrections, by owner:

1. **Fingerprint (P1 + P2-3):** `discovery_fingerprint` uses the existing
   `fingerprint_field`, `fingerprint_property_input` and `fingerprint_trait_input`
   helpers (widened to `pub(super)`), emits a fixed length-prefixed tag before each
   list (`property`, then `trait`) so lists can never run into each other and an
   empty list stays distinguishable, and keeps its own domain tag
   `aicadia-submit-discovery-fingerprint-v1`. The Action scheme's conditional
   `initial_trait_v1` tag is a historical-compatibility shape and is not copied. A
   World test proves the reported collision pair now conflicts.
2. **Chance/attempt owners (P2-4, P3):** `ChancePolicy::resolve(count, draw)`
   returns the outcome so resolution lives in the chance component; the orchestrator
   drops the unreachable draw-range check and the redundant Place comparison; a test
   proves an in-window discovery lowers the outcome (draw 0.49 → Zero); the exact
   `now − 1 hour` boundary and the actual hoarding/admission statements are proved
   through production SQL (constants exposed to the schema tests like
   `CURRENT_ENTITY_STATE_SQL`); `commit.rs` reuses the existing placed-Entity lookup;
   `Cargo.toml` dependency order restored.
3. **Wire/server (P2-1, P2-2, P3):** field `limits` → `limit` in wire, OpenAPI,
   MCP schema, docs, capability doc, protocol, fake-Agent runner/schemas and
   fixture; wire error-code test becomes an exhaustive `match`; HTTP status mapping
   loses the `_ =>` wildcard and names every code; adapter tests cover
   `invalid_discovery`, `invalid_entity`, `invalid_property`, `invalid_trait` and
   `property_key_conflict` on `POST /api/discovery` and its MCP twin, both discovery
   errors on both adapters, `character_not_found`/`character_not_entered` on start,
   and HTTP retry `201`; `src/server/mod.rs` keeps only composition (MCP-only
   items move to `mcp.rs`, ledger constant to `http.rs`).
4. **Agent contract (P3):** `tool/start_investigation.md` follows the fixed
   template (`What it does · Use it when · Before you call · Input meaning · After
   the call · On failure · Never`); code-specific recovery for the three new codes
   moves to `instruction/15-recovery.md`; both tool descriptions keep one short
   generic failure line; `14-investigation.md` is wrapped like its siblings; the
   fixture is regenerated once at the end of the task and the diff reviewed.
5. **Docs (P2-5, P2-6, P2-7, P3):** chance/admission parameters and formula get
   one home in `docs/game/domain.md` (Investigation subject); `protocol.md`,
   `storage.md` and `capability/start_investigation.md` link instead of restating
   numbers; `capability/start_investigation.md` gains `## Validation`;
   `docs/evidence/property.md` names the current test and describes fifteen
   descriptions; `docs/evidence/entity-state.md` drops the planning label;
   `docs/concept/discovery.md` "Prototype under evaluation" gets its supersession
   banner; `domain.md` "Required evidence" collapses to a pointer at
   `adapter-parity.md`; the completed discovery plan's ownership row (`plan.md:255`)
   is corrected to the crate-internal test homes.

### Technical

- **World:** `src/world/investigation/{model,chance,attempt,commit,mod,test}.rs`,
  `src/world/activity.rs` (helper visibility only), `tests/world/investigation_schema.rs`.
- **Wire/server:** `src/wire/{investigation,test,error}.rs`, `src/server/{mod,http,mcp,error,investigation_test}.rs`,
  `tests/server/protocol.rs`, `tests/agent-tool-catalog.json`.
- **Agent:** `src/agent_contract/instruction/{14-investigation,15-recovery}.md`,
  `src/agent_contract/tool/{start_investigation,submit_discovery}.md`,
  `src/agent_contract.rs` pins, `tools/agent-playtest`, `tools/agent-playtest-schema/*`,
  `tests/agent-playtest.sh`.
- **Docs:** `docs/game/{domain,protocol,storage,adapter-parity}.md`,
  `docs/game/capability/start_investigation.md`, `docs/evidence/{property,entity-state,discovery}.md`,
  `docs/concept/discovery.md`, concept log, `.agents/plans/20260814-204007-.../plan.md:255`.
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
- `docs/game/domain.md` is the single home of chance/admission parameters.
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
| `src/world/investigation/model.rs`, `activity.rs` | Duplicated fingerprint helpers, no list tags | Reuse helpers, add tags, keep v1 domain tag | Byte-equal retries still equal; changed content conflicts |
| `chance.rs`, `mod.rs` | Resolution in orchestrator; dead branches | `ChancePolicy::resolve`; remove branches | Formula, defaults, injection unchanged |
| `attempt.rs`, `commit.rs`, schema tests | Re-typed test SQL; duplicated lookup | Expose production SQL constants; reuse lookup | Bounded reads, indexes unchanged |
| `src/wire/investigation.rs`, `wire/test.rs`, `server/error.rs` | `limits`; hand list; wildcard | `limit`; exhaustive match; explicit statuses | Same codes/statuses as `protocol.md` |
| `src/server/{mod,http,mcp}.rs` | Shared items in `mod.rs` | Composition-only `mod.rs` | Byte-equal routes/tools/OpenAPI |
| `src/server/investigation_test.rs`, `tests/server/protocol.rs` | Single-sided/missing cases | Add named adapter tests | No probabilistic assertion |
| Agent contract sources, fixture, runner | Off-template description; triplicated recovery | Template-conformant; one recovery owner; regenerate fixture once | Natural in-world language |
| `docs/game/**`, evidence, concept, old plan row | Four parameter homes; stale pointers/labels | One home; corrected pointers; banner | No delivery-status prose in `docs/game/` |
| Concept log | — | One entry per accepted correction | Append-only |

## Execution contract

Root executes sequentially; no delegation planned. No task starts while `draft`.

## Task graph

Allowed states are `pending`, `in_progress`, `completed` and `blocked`.

| ID | State | Depends | Parallel-safe | Objective | Owned surfaces | Evidence |
| --- | --- | --- | --- | --- | --- | --- |
| T0 | pending | accepted plan | no | Baseline recorded | none | User decision on committing the discovery delivery recorded; ladder green on that tree |
| T1 | pending | T0 | no | World corrections | `src/world/**`, `tests/world/investigation_schema.rs`, `Cargo.toml` | Collision pair conflicts; in-window saturation; boundary and production-SQL plans; `cargo test --test world`, `--lib world` green |
| T2 | pending | T1 | no | Wire/server corrections | `src/wire/**`, `src/server/**`, `tests/server/**`, runner/schemas, fixture | Exhaustive code/status checks compile; new adapter tests green; fixture diff = rename only |
| T3 | pending | T2 | no | Agent contract and docs | `src/agent_contract/**`, fixture, `docs/**`, old plan row | Template scans; `rg` shows one parameter home, no stale test name/label; links resolve; fixture diff = descriptions only |
| T4 | pending | T3 | no | Ladder, trail, closure | concept log, `docs/evidence/discovery.md`, this plan | Full ladder green; log entries; plan `complete` |

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

1. Rename `limits` → `limit` in `src/wire/investigation.rs` (struct name
   `InvestigationLimitOutput`), OpenAPI/MCP schemas follow; update
   `src/server/investigation_test.rs`, `tools/agent-playtest` (result keys), runner
   schemas, `tests/agent-playtest.sh`.
2. `src/wire/test.rs`: replace the list with an exhaustive `match ErrorCode` →
   spelling; `src/server/error.rs`: exhaustive status match, no wildcard.
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
2. `docs/game/domain.md`: one Investigation parameters/formula block; `protocol.md`,
   `storage.md`, `capability/start_investigation.md` link to it and drop numbers;
   `capability/start_investigation.md` gains `## Validation`; `domain.md` "Required
   evidence" → pointer to `adapter-parity.md`; rename `limits`→`limit` in all docs
   and `CONTEXT.md` if present.
3. `docs/evidence/property.md`, `docs/evidence/entity-state.md`,
   `docs/concept/discovery.md` prototype banner, old plan `plan.md:255`.

**Evidence:** `rg -n "limits|thirteen_trait|Now / Proposed"` clean in the touched
authorities; relative link check; `cargo test --lib agent_contract`;
`bash tests/agent-playtest.sh`.

### T4 — Ladder, trail, closure

Full ladder (`fmt`, `clippy`, `cargo test --all-targets --all-features` on
PostgreSQL, three shell suites); concept-log entries (fingerprint correction, `limit`
rename, parameter home, recovery ownership, exhaustive error mapping); update
`docs/evidence/discovery.md` counts and limitations; set `complete`.

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
