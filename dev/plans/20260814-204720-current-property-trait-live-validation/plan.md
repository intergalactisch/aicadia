---
status: complete
created_at: "2026-08-14T20:47:24+02:00"
updated_at: "2026-08-14T22:03:30+02:00"
accepted_at: "2026-08-14T21:25:00+02:00"
spend_authorized_at: "2026-08-14T20:47:24+02:00"
completed_at: "2026-08-14T22:03:30+02:00"
---

# Current Property and Trait live validation

## Outcome

Make both retained paid playtest paths truthful against the current thirteen-tool
Agent contract, then run exactly one new Property candidate and one new Trait
candidate with `gpt-5.6-sol` at high reasoning. The Property candidate must let one
Agent ground, preview and confirm an Entity introduction with one initial Property,
then let an independent Agent retrieve that Entity's current combined state. The
Trait candidate must establish one Entity-owned Trait through Action, develop the
same stable lineage through Interaction and let an independent Agent retrieve its
current state and authorized history.

The final evidence claim is deliberately exact: each successful candidate proves
only its frozen scenario over the current MCP contract. A failed candidate remains
one exact failure and is never relabelled as gameplay success. This is the
highest-value current edge because deterministic Property/Trait behavior is already
delivered, while the Property runner is bound to a removed read capability and the
Trait runner still contains the live-only validator drift that blocked its last
paid candidate.

## Non-goals

- Changing World behavior, PostgreSQL schema, migrations, HTTP/MCP semantics or the
  thirteen player capabilities.
- Expanding the paid claim to every Property creation/change route, arbitrary Agent
  quality, other models, retries or background execution.
- Rewriting or relabelling any historical paid candidate, usage record or failure.
- Running more than one new candidate per runner under this plan.
- Starting investigation, discovery, movement or another gameplay edge.

## Evidence and authorities

| Evidence or authority | Current fact | Consequence for this build |
| --- | --- | --- |
| `game/docs/capability/get_entity_at_current_place.md` and current catalog fixture | Current Entity state is one combined Property/Trait page through `get_entity_at_current_place`; the standalone Property read is absent | The Property runner must use only the current catalog and retrieve the introduced Entity through the scoped read |
| `dev/playtest/agent/run` and token-free preflight on 2026-08-14 | Preflight passes, but the live runtime gate still expects removed `list_entity_property_at_current_place` | Currentize the runner, prompts, allowlists, schemas, fake proof and runtime gate without changing historical evidence |
| `dev/playtest/trait/run:999-1085`, `dev/docs/evidence/trait.md` and token-free preflight on 2026-08-14 | Exact valid `Pip`/`Mara` outputs are rejected only by live `startswith("Pip ")`/`startswith("Mara ")` validators; the audited digest also drifted after accepted repository changes | Make fake/live validation share the exact-name contract, add regressions and freeze a fresh digest after all bound bytes settle |
| `dev/docs/evidence/property.md` | Deterministic Property proof is complete; no paid invocation currently counts as Property evidence | A successful new current candidate may earn only the bounded initial-Property/observation claim |
| `dev/docs/evidence/trait.md` | Deterministic Trait proof is complete; all retained paid candidates are exact failures before Trait mutation | Preserve those failures and record only the newly earned result |
| User direction, 2026-08-14 | The User requested Terry execution, authorized the paid tests now without another later spend prompt and requested a Sol High subagent for final testing | Plan acceptance activates two one-shot authorizations: Property at most four model process calls and Trait at most seven, both zero-retry; final review is delegated read-only to `gpt-5.6-sol`/high |
| Concurrent draft `dev/plans/20260814-204007-first-investigation-discovery-loop/plan.md` and backlog | Investigation/discovery is `Now / Proposed` with material grill questions and no accepted executable contract | On acceptance this live validation becomes the sole `Now / Active`; discovery moves only to `Next / Proposed` and its draft, concept work and open questions remain untouched |
| Existing dirty worktree | Local-development and documentation changes already belong to the User | Do not edit or revert those surfaces except append-only concept-log and narrowly overlapping evidence/backlog alignment required here |

## Alignment

### Strategic

Property and Trait state make discovered people and things persist as more than
introductory prose. Proving that a real connected Agent can author, confirm and
later retrieve those facts retires a concrete player-interface risk before Aicadia
adds investigation and discovery on top of them. The build fulfils the current
contract; it does not evolve game behavior.

### Tactical

Reuse two existing disposable two-User scenarios. For Property, the first Agent
performs the existing three-phase Action workshop and submits one introduction with
`material = weathered cedar`; HTTP and a second Agent prove the exact accepted
Entity, Property and Activity. For Trait, Pip performs the existing three-phase
Action establishment and three-phase Interaction development; HTTP and Mara prove
stable lineage, current statement and authorized history. Selection, steering and
confirmation remain withheld by phase. Each candidate owns one disposable database
and drops it only after exact name-plus-token ownership verification.

### Technical

World, schema, transactions, idempotency, errors and adapters are unchanged. Work is
limited to runner catalogs/allowlists/prompts/validators, strict output schemas,
token-free controller tests, a fresh Trait digest, private ignored evidence and the
owning evidence/concept/backlog records. Historical Property-era fixtures may remain
only as explicit history tests; neither live path may consume a legacy capability.

## Decisions, assumptions and open questions

### Confirmed decisions

- Reuse and currentize `dev/playtest/agent/run` for the bounded Property candidate;
  do not add a third runner.
- Keep the Property claim to one Agent-authored initial Property plus independent
  current-state/history observation. Deterministic tests continue to own the wider
  creation, Action-change and Interaction-change contract.
- Correct the Trait live validators to the exact `Pip`/`Mara` names already required
  by prompts, schemas and fake proof; do not weaken them to prefixes.
- Preserve all historical candidates and their failure/success meanings byte-for-
  byte in ignored evidence.
- Plan acceptance activates the User's already-given one-shot spend authority; no
  second spend prompt is required. Property permits at most four paid model process
  calls and Trait at most seven, with zero retries and no enforceable per-run token
  ceiling.
- Use a `gpt-5.6-sol`/high subagent for independent final read-only audit. The paid
  gameplay calls remain owned by the audited runners, not by the reviewer.

### Reversible assumptions

- PostgreSQL administration remains available at execution time; both token-free
  preflights prove create/tag/read/drop ownership before spend.
- Codex remains exactly the runner-pinned CLI/model/effort contract; drift is a
  pre-spend NO-GO, not a reason to add compatibility.
- Existing unrelated dirty files remain stable while the fresh Trait digest is
  frozen; any later bound-byte drift stops before spend and is diagnosed.
- The discovery draft remains the next proposed edge after this bounded validation;
  only its backlog horizon changes temporarily and no discovery decision is made.

### Open questions

- None material. Plan acceptance confirms the exact two-scenario claim, maximum
  eleven paid process calls, zero retries and preservation boundary.

## Implementation map

| Surface | Current state | Intended change | Invariants |
| --- | --- | --- | --- |
| `dev/playtest/agent/run`, `dev/tests/agent-playtest.sh`, `dev/playtest/agent/schema/` | Runnable fake/preflight path but legacy live catalog/read assumptions | Use current catalog, four grounded Action reads, scoped Entity fetch for observer Property proof and aligned strict validation | One Action submit, withheld selection/confirmation, no legacy tool, no retry, ownership-safe cleanup |
| `dev/playtest/trait/run`, `dev/tests/trait-playtest.sh`, `dev/playtest/trait/schema/live-candidate.sha256` | Current scenario with live-only exact-name rejection and stale digest | Unify exact-name validators, add regression and freeze all settled bound bytes | Seven calls maximum, zero retries, stable Trait id, current protocol only |
| `dev/.playtest/agent/`, `dev/.playtest/trait/` | Ignored retained evidence roots | Add at most one new candidate per runner after GO preflights | Directories 700, files 600, historical artifacts unchanged, exact ownership before drop |
| `dev/docs/evidence/`, `dev/docs/concept/log/2026-08.md` | Accurately reports no paid Property proof and failed Trait validation | Record proposed boundary, then exact earned results once | One home per truth; no success claim before success |
| `dev/backlog/` and this plan | Investigation/discovery is the sole `Now / Proposed`; no plan is Active | On acceptance publish validation as `Now / Active` and move discovery to `Next / Proposed`; restore discovery to `Now / Proposed` after validation closes unless the User redirects | At most one current edge; both draft plans keep their distinct authority |
| `game/docs/`, World/schema/adapters | Delivered current contract | No change | Property/Trait semantics and parity remain exact |

## Execution contract

Root owns outcome, scope, plan state, integration, spend commands and the final
evidence claim. The requested Sol High subagent receives this plan path and the
final audit task only, re-reads live files, performs no paid call or mutation and
returns raw findings. No task runs in parallel with a paid candidate or evidence
alignment.

## Task graph

Allowed states are `pending`, `in_progress`, `completed` and `blocked`.

| ID | State | Depends | Parallel-safe | Objective | Owned surfaces | Evidence |
| --- | --- | --- | --- | --- | --- | --- |
| T1 | completed | — | no | Accept and publish the exact current validation boundary | plan, concept log, backlog item/index | Accepted at `2026-08-14T21:25:00+02:00`; exactly one Now/Active edge |
| T2 | completed | T1 | no | Make both live controllers truthful against their existing scenario contracts | runner scripts, schemas, fake tests | Both token-free suites pass; current live paths use the current catalog and exact shared validators |
| T3 | completed | T2 | no | Earn current token-free readiness and freeze the exact Trait candidate | digest, private preflights | Property and Trait public preflights GO with zero model calls; final in-run Trait preflight used candidate digest `5bdfa…4061f`; ownership-verified cleanup |
| T4 | completed | T3 | no | Execute the two already-authorized one-shot paid candidates | ignored candidate evidence only | Both one-shot candidates terminal-failed after three calls at controller HTTP validation; accepted World mutations occurred only in disposable databases that were later dropped; exact usage and ownership-safe cleanup proved |
| T5 | completed | T4 | no | Align authorities and obtain independent Sol High final audit | evidence docs, concept log, backlog, plan | Independent `gpt-5.6-sol` high re-review GO with no P0-P3 finding after four corrected alignments; exact failures and cleanup reported |

## Task details

### T1 — Accept the validation boundary

**Objective:** Make the exact outcome, cost and evidence boundary authoritative
before controller changes or spend.

**Actions:**

1. Present this plan and wait for explicit acceptance.
2. On acceptance, set it active, append the accepted concept decision, publish one
   combined `Now / Active` validation item and move the unchanged discovery draft
   to `Next / Proposed` for the duration of this build.

**Invariants:**

- The User's spend authorization is recorded but cannot execute while this plan is
  draft.
- Existing dirty worktree changes remain untouched.
- No discovery question, contract surface, plan content or concept direction is
  answered or edited by this validation build.

**Evidence:**

- Plan timestamp plus an exact scan showing one active plan and backlog item.

**Stop conditions:**

- Keep the plan draft if the User changes the scenarios, model/effort, process-call
  maxima, retry count, ownership boundary or evidence claim.

### T2 — Correct and currentize both controllers

**Objective:** Make every token-free and live validator enforce the same current
contract before any candidate is possible.

**Actions:**

1. Currentize the Property runner's catalog, allowlists, prompts, strict observer
   output and HTTP/MCP validation around `get_entity_at_current_place`.
2. Preserve the old catalog fixture only in a clearly historical regression; remove
   it from public preflight and paid execution.
3. Replace Trait prefix-only live name checks with the exact names required by the
   prompt/fake controller and add a regression that exercises those live validators.
4. Keep candidate digest stale until every bound source/schema/runner byte is final.

**Invariants:**

- No game behavior, public schema or historical evidence is modified.
- Preview and commit packages remain byte-identical across withheld confirmation.
- No Codex model process or candidate starts.

**Evidence:**

- `dev/tests/agent-playtest.sh` — current Property controller happy/failure paths.
- `dev/tests/trait-playtest.sh` — exact live-validator/fake-controller parity.
- Focused catalog scan proves neither live path names the removed tool.

**Stop conditions:**

- Stop if currentizing the runner requires a game-contract change, compatibility
  path, looser confirmation, ambiguous Property observation or historical rewrite.

### T3 — Freeze and prove token-free readiness

**Objective:** Produce two current GO preflights with no model invocation.

**Actions:**

1. Compute and record the fresh Trait candidate digest after all bound bytes settle.
2. Run both permanent token-free suites and focused deterministic Property/Trait
   World plus HTTP/MCP parity tests.
3. Run both public preflights against PostgreSQL; verify exact live catalogs,
   schemas, CLI/model/effort, ownership-tagged database create/drop and zero model
   calls.
4. Confirm no bound-byte drift after preflight.

**Invariants:**

- Preflight never invokes `codex exec` and cannot consume either authorization.
- A NO-GO stops before T4; no compatibility or fallback is added.

**Evidence:**

- `dev/tests/agent-playtest.sh` and `dev/tests/trait-playtest.sh` pass.
- Focused Rust Property/Trait World and server parity suites pass.
- Both public preflight manifests report GO, `codex_invoked:false`, `model_calls:0`
  and `ownership_verified_and_dropped`.

**Stop conditions:**

- Stop before spend on digest drift, CLI/model mismatch, catalog/schema mismatch,
  database ownership ambiguity, cleanup failure or any nonzero model activity.

### T4 — Run one Property and one Trait candidate

**Objective:** Earn or exactly falsify each bounded live claim under the already-
given spend authorization.

**Actions:**

1. Run the Property candidate once: at most four `gpt-5.6-sol`/high process calls,
   zero retries.
2. Verify its terminal manifest, authoritative Property/Activity HTTP gate, observer
   result, usage accounting and ownership-safe cleanup before proceeding.
3. Run the exact-digest Trait candidate once: at most seven
   `gpt-5.6-sol`/high process calls, zero retries.
4. Verify its terminal manifest, both authoritative Trait gates, Mara observation,
   usage accounting and ownership-safe cleanup.

**Invariants:**

- First candidate start consumes that runner's one authorization; failure never
  retries automatically.
- A shared infrastructure/controller or ownership failure in Property stops before
  Trait spend. A scenario-specific terminal Property result does not get relabelled
  and may be followed by the independently ready Trait candidate.
- No private prompt/transcript/credential content enters public docs.

**Evidence:**

- Property command: `DATABASE_URL='postgres://localhost/postgres' dev/playtest/agent/run run --confirm-token-spend`.
- Trait command: `DATABASE_URL='postgres://localhost/postgres' dev/playtest/trait/run run --confirm-token-spend --candidate-digest '<fresh-digest>'`.
- Terminal private manifests and public redacted exact counts/results.

**Stop conditions:**

- Stop on ownership ambiguity, cleanup failure, unexpected retry/model call,
  candidate drift or a shared blocker that makes remaining spend unsafe.

### T5 — Align and independently audit the earned result

**Objective:** Leave one exact current status with no stale success or failure claim.

**Actions:**

1. Update Property/Trait evidence, runner operations, evidence index, concept log,
   backlog and plan to the exact terminal outcomes.
2. Ask a `gpt-5.6-sol`/high subagent for a read-only Terry/ownership/evidence audit
   of the complete diff and private manifest summaries.
3. Correct any in-scope P0-P3 finding, rerun token-free evidence as needed and close
   the plan only when authorities agree. Never rerun a paid candidate in T5.

**Invariants:**

- Historical candidate bytes and meanings remain unchanged.
- Only actual successful candidates earn live gameplay claims.

**Evidence:**

- Independent review reports GO with no P0-P3 finding, or all findings are corrected
  and re-reviewed.
- `git diff --check`, focused diff review, active-plan/backlog scan and link checks
  pass without altering unrelated User work.

**Stop conditions:**

- Keep the plan active with the exact blocker if evidence is ambiguous, cleanup is
  non-terminal or a material contract/cost change would be required.

## Validation ladder

1. **Focused:** both runner suites plus exact catalog/name-validator regressions.
2. **Contract:** deterministic World, PostgreSQL, HTTP and MCP Property/Trait parity;
   both zero-model public preflights and stable Trait digest.
3. **Outcome:** one terminal paid Property candidate and one terminal paid Trait
   candidate with exact reached-phase HTTP/Agent evidence, explicit unstarted phases,
   usage and ownership-safe cleanup; only a phase that actually passed may earn its
   gate or independent-observer claim.
4. **Integrity:** independent Sol High audit, `git diff --check`, focused diff review
   and confirmation that unrelated User changes and governing authorities remain
   intact.

## Change control

Refine paths, test fixtures and task ordering in place while the two scenarios,
maximum eleven paid process calls, zero retries and evidence claims remain unchanged.
Stop implementation, return `status: draft`, revise and request explicit
re-acceptance if evidence requires a game/public-contract change, another candidate,
more process calls, a retry, compatibility path, weaker ownership/confirmation or a
different live claim.

## Completion conditions

- T1–T5 are completed and the validation ladder passes;
- each paid candidate has exactly one terminal, truthfully classified outcome and
  ownership-verified cleanup;
- current evidence, concept history, planning and backlog agree without modifying
  game behavior or historical candidate meaning;
- the requested Sol High independent audit is GO with no unresolved P0-P3 finding;
- no known-stale authority, material open question or accidental unrelated change
  remains;
- `status: complete` and `completed_at` are recorded only after these conditions.
