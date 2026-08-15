---
status: complete
created_at: "2026-08-14T22:38:43+02:00"
updated_at: "2026-08-14T23:06:46+02:00"
accepted_at: "2026-08-14T22:45:17+02:00"
spend_authorized_at: "2026-08-14T22:38:43+02:00"
completed_at: "2026-08-14T23:06:46+02:00"
---

# Post-correction Property and Trait live validation

## Outcome

Run exactly one fresh paid Property candidate and one fresh paid Trait candidate
against the already-corrected current controllers. A successful Property candidate
must carry one grounded, previewed and confirmed initial Property through
authoritative HTTP and an independent observer. A successful Trait candidate must
carry one established Trait through same-lineage Interaction development,
authoritative HTTP and Mara's independent observation. Each candidate instead
remains an exact terminal failure if any required phase rejects it.

This is the smallest safe way to retire the two concrete false-negative controller
risks exposed by the completed validation plan. It changes no game behavior or
public contract. Final evidence must distinguish reached, passed, failed and
unstarted phases, exact model-call usage and ownership-verified cleanup.

## Non-goals

- No World, schema, migration, HTTP/MCP, Agent-catalog or game-contract change.
- No retry, replacement, fallback or compatibility path after either new candidate.
- No historical manifest edit or relabelling of any earlier failed candidate.
- No discovery-plan decision or implementation.

## Evidence and authorities

| Evidence or authority | Current fact | Consequence for this build |
| --- | --- | --- |
| [Completed prior validation](../20260814-204720-current-property-trait-live-validation/plan.md) | Both one-shot candidates failed after three calls at now-corrected HTTP-controller checks; final Sol High audit returned GO | Reuse the reviewed current controllers, never relabel the old failures |
| [Property evidence](../../../docs/evidence/property.md) | `run-Cx1eI1zC` reached exact World, HTTP and observer content but terminal-failed on the observer's valid optional `limit:100`; current validator is corrected token-free | Preserve the failed manifest and claim no completed paid candidate or retry |
| [Trait evidence](../../../docs/evidence/trait.md) | Exact-digest `candidate-idl47NKn` completed all nine gates in seven calls with owned cleanup | Claim the earned complete live Trait result and no second-candidate authority |
| `tests/agent-playtest.sh`, `tests/trait-playtest.sh` | Current fake suites exercise the Property limit regression and both Trait HTTP validators | Both are green after the final token-free correction |
| User authorization at `2026-08-14T22:38:43+02:00` | One Property and one Trait paid attempt were authorized and are consumed | No retry or further paid execution is authorized |
| [Discovery draft](../20260814-204007-first-investigation-discovery-loop/plan.md) | Discovery is restored unchanged to `Now / Proposed` with unresolved questions | Validation added no capability or discovery decision |

## Alignment

### Strategic

Property and Trait are already delivered World capabilities. The current concrete
risk is whether a real Agent can complete their confirmed shared-world flows through
the published contract and an independent Character can retrieve the accepted
state. Retiring that risk is a bounded prerequisite to trusting these capabilities
in ordinary play; discovery remains the next product-design edge afterward.

### Tactical

The Property scenario remains one initial text Property
`material = weathered cedar` on an introduced marker. The Trait scenario remains
Pip establishing one non-executable characterization through Action, developing the
same stable Trait through an outward Interaction targeting Mara, and Mara retrieving
the current Trait and authorized Activity. Both use disposable two-User Worlds,
withheld selection and confirmation, exact current-state reads and no retry.

### Technical

World, PostgreSQL, transactions, Activity, HTTP, MCP and tool descriptions are
unchanged. Work is operational: preserve the consumed Trait evidence beneath a
mode-700 archive without changing bytes or permissions; run current fake suites,
focused deterministic parity and public zero-model preflights; execute the existing
Property runner once and the exact-digest Trait runner once; then align evidence and
planning. Property is bounded to at most four and Trait to at most seven
`gpt-5.6-sol` high process calls. Codex CLI 0.147.0 exposes no enforceable token
ceiling, so process calls and emitted usage are the honest spend boundary.

## Decisions, assumptions and open questions

### Confirmed decisions

- Exactly one new candidate per runner, zero retries — explicit User authorization
  and the prior controller failures justify a fresh attempt, not an open-ended loop.
- Property runs before Trait — its four-call path is the smaller first validation;
  a shared infrastructure or ownership failure stops before Trait spend.
- Preserve `candidate-consumed` and `candidate-rI28yzhw` together in a new private
  archive, with stable content/permission fingerprints and a mode-600 index — the
  Trait runner may create one new sentinel only after history is recoverably moved.
- Final review uses one read-only `gpt-5.6-sol` high subagent — this repeats the
  independent Terry/ownership/evidence gate requested for the paid validation.

### Reversible assumptions

- The current post-correction Trait digest remains `e940c03e…3ae84`; recompute it
  immediately before and after public preflight, and stop on any drift.
- Existing top-level Trait preflight directories are immutable supporting evidence
  and do not block the runner; archive only the consumed sentinel and candidate.

### Open questions

- None. The User accepted this plan at `2026-08-14T22:45:17+02:00`; both one-shot
  authorizations are consumed and no further spend is permitted.

## Implementation map

| Surface | Current state | Intended change | Invariants |
| --- | --- | --- | --- |
| `.aicadia-trait-playtest/` | Old sentinel/candidate are fingerprinted in `archive-post-correction-rI28yzhw`; the new sentinel and successful `candidate-idl47NKn` consume the fresh attempt | Retain both generations unchanged | Original bytes/modes unchanged; prior archives/preflights untouched |
| `tools/agent-playtest` | One paid candidate terminal-failed only because valid observer `limit:100` was rejected; current validator and fake regression are corrected | Retain the failed run and current token-free fix | Four calls consumed, zero retries, exact cleanup |
| `tools/trait-playtest` and live digest | Exact digest `e940c03e…3ae84` completed all nine live gates | Retain successful candidate and consumed sentinel | Seven calls consumed, zero retries, exact digest and owned cleanup |
| `docs/evidence/`, concept log, backlog and this plan | New outcomes are recorded; final audit and ordering restoration remain | Close only after independent GO | Historical facts immutable; no Property runner success relabelling |

## Execution contract

Root owns outcome, scope, plan state, integration and the final evidence claim. The
final delegated Agent is read-only, receives T6, re-reads the live repository and
private redacted manifests, starts no model playtest and changes no file. Paid tasks
are sequential and never delegated.

## Task graph

Allowed states are `pending`, `in_progress`, `completed` and `blocked`.

| ID | State | Depends | Parallel-safe | Objective | Owned surfaces | Evidence |
| --- | --- | --- | --- | --- | --- | --- |
| T1 | completed | — | no | Accept and activate the exact fresh one-shot boundary | plan, concept log, backlog | User accepted at `2026-08-14T22:45:17+02:00`; one Now/Active edge |
| T2 | completed | T1 | no | Preserve the consumed Trait candidate and clear only its active sentinel/candidate paths | ignored Trait evidence root | 55 paths preserved byte-for-byte and mode-for-mode; exact two-entry move; no active candidate residue |
| T3 | completed | T2 | no | Earn current zero-model readiness for both runners | fake suites, digest, private preflights | Both fake suites, 16 Property and 12 Trait World tests, both parity tests and both public preflights passed; zero model calls; owned drops; digest stable |
| T4 | completed | T3 | no | Execute the fresh Property candidate once | ignored Property evidence | `run-Cx1eI1zC` truthfully failed only at observer validation after four calls; observer evidence is substantively exact but used allowed `limit:100`; owned drop passed |
| T5 | completed | T4 | no | Execute the exact-digest fresh Trait candidate once | ignored Trait evidence | `candidate-idl47NKn` completed all nine frozen gates in exactly seven calls with Mara observation and ownership-verified drop |
| T6 | completed | T5 | no | Align authorities and obtain independent Sol High final audit | evidence docs, concept log, backlog, plan | Initial stale-planning P3 corrected; re-audit GO with P0–P3=0 and zero audit model/spend/mutations; discovery restored |

## Task details

### T1 — Accept and activate

**Objective:** Make the new cost, archive and evidence boundary authoritative.

**Actions:**

1. Present this plan and wait for explicit acceptance.
2. On acceptance, mark it active, set validation `Now / Active` and move the
   unchanged discovery draft to `Next / Proposed` for this build only.

**Invariants:**

- No archive move, database operation or Codex model process occurs while draft.
- Spend authority covers one Property and one Trait candidate only.

**Evidence:**

- Plan and backlog scan — accepted timestamp and exactly one active edge.

**Stop conditions:**

- Revise and regain acceptance if calls, retries, scenarios, archive scope or final
  evidence claim changes.

### T2 — Archive consumed Trait evidence

**Objective:** Preserve the old candidate exactly while making one fresh Trait
candidate operationally possible.

**Actions:**

1. Verify the root, sentinel, candidate and all contained paths are real, private
   and terminal with ownership-verified cleanup.
2. Record deterministic relative-path, content and permission fingerprints for
   exactly `candidate-consumed` and `candidate-rI28yzhw`.
3. Move exactly those entries into a new mode-700 archive, create a mode-600 index
   documenting the serialization recipe, and verify unchanged fingerprints.

**Invariants:**

- No historical byte or permission changes; no old preflight/archive moves.
- No top-level sentinel/candidate remains after success.

**Evidence:**

- Before/after counts and SHA-256 fingerprints plus exact top-level scan.

**Stop conditions:**

- Stop on symlink, unexpected mode/path, non-terminal cleanup or fingerprint drift.

### T3 — Re-earn token-free readiness

**Objective:** Prove the exact current controllers and disposable infrastructure
before either paid candidate.

**Actions:**

1. Run both permanent fake suites and the focused Property/Trait World and server
   parity tests.
2. Run both public preflights against PostgreSQL; verify exact catalog, schemas,
   CLI/model/effort, zero model calls and ownership-safe cleanup.
3. Recompute the Trait digest and require exact `e940c03e…3ae84` equality.

**Invariants:**

- Preflight cannot consume an authorization or invoke `codex exec`.
- Any drift or NO-GO stops before T4.

**Evidence:**

- Green suites/parity; Property preflight GO; Trait preflight manifest GO with
  `candidate_started:false`, `authorization_consumed:false`, `model_calls:0` and
  `ownership_verified_and_dropped`.

**Stop conditions:**

- Stop on catalog/schema/digest mismatch, cleanup ambiguity or any model activity.

### T4 — Run Property once

**Objective:** Earn or exactly falsify the complete corrected Property claim.

**Actions:**

1. Run `DATABASE_URL='postgres://localhost/postgres' tools/agent-playtest run --confirm-token-spend` once.
2. Inspect terminal phases, usage, authoritative HTTP, observer and cleanup.

**Invariants:**

- Maximum four model calls; zero retries; a terminal failure remains failed.

**Evidence:**

- One private terminal manifest and redacted exact result.

**Stop conditions:**

- Stop before Trait on a shared infrastructure/controller or ownership failure.

### T5 — Run Trait once

**Objective:** Earn or exactly falsify the complete corrected Trait claim.

**Actions:**

1. Run the exact public command from the final Trait preflight once.
2. Inspect all reached/unstarted phases, usage, both HTTP gates, Mara observation,
   sentinel and cleanup.

**Invariants:**

- Maximum seven model calls; zero retries; the new sentinel consumes authorization.

**Evidence:**

- One private terminal manifest and redacted exact result.

**Stop conditions:**

- Never retry or weaken a failed phase.

### T6 — Align and independently audit

**Objective:** Leave one truthful current status and restore the next gameplay edge.

**Actions:**

1. Update Property/Trait evidence, runner status pointers, concept log, backlog and
   plan to only the earned results.
2. Delegate a read-only Terry/ownership/evidence review to `gpt-5.6-sol` high.
3. Correct in-scope findings token-free, close this plan and restore discovery to
   `Now / Proposed` only after GO.

**Invariants:**

- Historical artifacts remain immutable; T6 never starts a paid candidate.

**Evidence:**

- Independent GO with no unresolved P0-P3 plus final integrity scans.

**Stop conditions:**

- Keep the plan active if evidence or cleanup is ambiguous.

## Validation ladder

1. **Focused:** both fake suites, archive fingerprints and digest equality.
2. **Contract:** focused World/server parity and both zero-model public preflights.
3. **Outcome:** one exact terminal candidate per runner with reached/unstarted
   phases, emitted usage and ownership-safe cleanup; success only when every frozen
   phase passes.
4. **Integrity:** independent Sol High audit, `git diff --check`, focused diff review
   and confirmation that unrelated user changes and all authorities remain intact.

## Change control

Refine paths, task order and stronger token-free evidence in place while the exact
two scenarios, maximum eleven paid process calls, zero retries and archive boundary
remain unchanged. Stop, return to `draft`, revise and request explicit re-acceptance
for another candidate, more calls, a retry, weaker history/ownership, public/game
behavior change or a different evidence claim.

## Completion conditions

- T1–T6 are completed and the validation ladder passes;
- both new candidates have one exact terminal outcome and ownership-safe cleanup;
- current evidence, concept history, backlog and plan agree without historical edit;
- independent Sol High review is GO with no unresolved P0-P3 finding;
- discovery is restored to `Now / Proposed` and no material question remains;
- `status: complete` and `completed_at` are recorded only after these conditions.
