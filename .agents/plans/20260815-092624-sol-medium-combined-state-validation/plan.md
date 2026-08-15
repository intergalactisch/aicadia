---
status: complete
created_at: "2026-08-15T09:26:24+02:00"
updated_at: "2026-08-15T09:58:01+02:00"
accepted_at: "2026-08-15T09:29:59+02:00"
completed_at: "2026-08-15T09:58:01+02:00"
---

# Sol-medium combined Entity-state live validation

> **Role / side:** bounded paid-validation plan / development side.
> **Authority:** defines the exact candidate, spend, operation and evidence claim for one Sol-medium combined Property/Trait MCP validation.
> **Excludes:** executable game behavior, broader model quality and discovery; see `docs/game/`, `docs/evidence/entity-state.md` and the discovery draft.

## Outcome

Prove whether one clean-room `gpt-5.6-sol` Agent at `medium` reasoning can use only
the published Aicadia MCP contract to ground one entered Character, keep selection
and confirmation private, preview one natural combined state change, and—only after
explicit confirmation—submit exactly one `change_entity_state` Action containing
one Property and one Trait establishment. Independent HTTP reads must prove that the
same Action committed both changes atomically in one Activity.

The exact earned claim is deliberately narrow: one pinned Sol-medium candidate
understood the current MCP grounding, confirmation boundary and combined Action
call in this frozen scenario. Failure is equally terminal evidence. It does not
prove creation routes, Interaction, Trait development, other models or general play
quality.

## Non-goals

- No game, schema, World, HTTP, MCP or Agent-contract change.
- No discovery, movement, new tool, Trait mechanic or server-side inference.
- No second candidate, retry, replacement or recovery of a failed model phase.
- No mutation or relabelling of retained Property/Trait candidates, manifests,
  archives or consumed sentinels.
- No universal token maximum claim: Codex CLI exposes no enforceable per-run token
  ceiling.

## Evidence and authorities

| Evidence or authority | Current fact | Consequence for this validation |
| --- | --- | --- |
| User request on 2026-08-15 | Test with a real Sol-medium Agent whether it understands MCP and the combined call | Pin exactly `gpt-5.6-sol` / `medium` and test the current live contract |
| [Agent contract](../../../docs/game/agent.md) | Action requires current MCP grounding, three directions, complete preview, explicit confirmation and one submit | Split those responsibilities across three resumed process calls |
| [Action contract](../../../docs/game/capability/submit_action.md) | `change_entity_state` accepts independent Property and Trait lists atomically | Require both lists non-empty in the one candidate commit |
| [Uniform deterministic evidence](../../../docs/evidence/entity-state.md) | World/HTTP/MCP behavior is deterministic, but the build made no fresh model call | Earn only the missing model-comprehension claim |
| Existing playtest runners | Agent runner is Sol-high/Property-only; Trait runner is Sol-high/Trait-only and its candidate sentinel is consumed | Create an isolated bounded Entity-state runner and evidence root; preserve both runners' evidence |
| Sol-medium read-only design audit | Three resumed calls plus an independent HTTP gate are sufficient; no existing command can make the exact claim | Build and freeze the exact candidate before any model invocation |

## Alignment

### Strategic

The combined state capability matters in play only if an ordinary Agent can ground
and use it without exposing protocol mechanics or collapsing confirmation. This
validation retires that concrete integration risk without expanding the game. The
discovery draft remains the next product-design edge afterward.

### Tactical

A disposable World contains one entered Character, Pip, described as a small
ash-dusted frog and carrying no current Property or Trait. One Agent session runs:

1. `propose`: call exactly `get_world`, `get_character`,
   `list_entity_at_current_place` and `list_activity_at_current_place`, once each,
   with one equal non-null `place_revision`; mutate nothing and return exactly three
   grounded directions.
2. `preview`: with no tool available, receive selection plus free steering toward
   recording `leg_count = 3` and one marker-bound unusually-high-jumping Trait;
   return the complete natural Action preview without ids or protocol language.
3. `commit`: after a separate explicit confirmation, resume the same session with
   only `submit_action`; generate one fresh request id and make exactly one completed
   call using the retained revision, unchanged prose/meaning, one Property change
   and one Trait establishment.

An operator-only HTTP gate then verifies the accepted Character, Property current
and history, World-assigned Trait root/current version, canonical consequence,
prose, one deduplicated subject role and one location role all name the same
Activity. The database is dropped only after exact name plus random ownership-token
readback.

### Technical

Add one current evidence runner, strict schemas and a permanent token-free fake
suite under `tools/entity-state-playtest*` and `tests/entity-state-playtest.sh`.
The runner owns a separate mode-700 `.aicadia-entity-state-playtest` root and
mode-600 artifacts. Its candidate digest binds the Rust/migration/runtime contract,
exact thirteen-tool catalog, runner, prompts, schemas, validators, model/effort,
three-call/zero-retry budget and cleanup algorithm.

Before any candidate, token-free tests and public preflight must prove Codex CLI
`0.147.0`, login, Sol-medium availability, session resume, strict output schemas,
direct-only Aicadia MCP `2026-07-28`, runtime catalog equality and ownership-safe
disposable database lifecycle. The accepted digest is named explicitly on the live
command. An atomic `candidate-consumed` sentinel is written before database or model
work. Every process usage event is retained; no token ceiling is inferred.

## Decisions, assumptions and open questions

### Confirmed decisions

- Use a Sol-medium Agent and perform a real MCP/model test — User request.
- Test the combined Property/Trait Action, not another single-kind call — the exact
  unresolved integration risk after uniform state delivery.
- Root orchestrates; the requested Sol-medium subagent implements and operates one
  accepted dependency-ready task at a time.

### Reversible assumptions

- One Agent session and one Character are sufficient because the claim is call
  comprehension, not independent observation; HTTP is the authoritative independent
  state gate.
- The dedicated runner is smaller and safer than resetting or adding modes to
  consumed Trait evidence; focused diff review must confirm no unnecessary copied
  surface remains.

### Open questions

- **Acceptance A — spend:** authorize at most three paid `gpt-5.6-sol` `medium`
  process calls, zero retries and no enforceable token maximum. Any failed phase
  consumes the only candidate.
- **Acceptance B — evidence claim:** accept one Character Action with
  `leg_count = 3` plus one unusually-high-jumping Trait and an independent HTTP gate
  as sufficient proof of MCP/confirmation/combined-call comprehension.

Implementation and preflight remain blocked until the User explicitly accepts both
details with this plan. Preflight itself is token-free but creates and drops a
disposable database, so it also waits for acceptance.

## Implementation map

| Surface | Current state | Intended change | Invariants |
| --- | --- | --- | --- |
| `tools/entity-state-playtest`, schema directory, permanent shell test | Absent | Add one three-phase Sol-medium controller, strict validators, fake suite and frozen digest | No game behavior; exact three calls, zero retries |
| `.aicadia-entity-state-playtest/` | Absent and ignored private operation state | Create isolated preflight/candidate artifacts and one atomic sentinel | Mode 700/600; ownership-safe cleanup; existing evidence untouched |
| `docs/evidence/runner/`, `docs/evidence/entity-state.md` | Deterministic uniform proof and no fresh model claim | Record operation contract, then exact success or failure | Never overclaim beyond the one candidate |
| Concept log, backlog and this plan | Validation requested; discovery is `Now / Proposed` | Keep the validation interlude current through terminal evidence, then restore discovery unchanged | At most one active plan/item; material spend changes require re-acceptance |

## Execution contract

Root owns scope, plan state, digest authorization, integration and the final evidence
claim. The requested Sol-medium subagent receives this plan and one dependency-ready
task, re-reads live files, changes only assigned surfaces, runs focused evidence and
returns raw results. It may invoke the paid candidate only in T3 after root verifies
the accepted digest and token-free GO; it may never retry.

## Task graph

| ID | State | Depends | Parallel-safe | Objective | Owned surfaces | Evidence |
| --- | --- | --- | --- | --- | --- | --- |
| T1 | completed | — | no | Freeze the minimal candidate controller | temporary runner and strict schemas | syntax, catalog/schema contract and digest determinism passed |
| T2 | completed | T1 | no | Prove zero-model readiness and authorize one exact digest | private preflight artifacts | public preflight GO with `model_calls:0` and verified drop |
| T3 | completed | T2 | no | Run exactly one candidate and close exact evidence | private candidate, public evidence/concept/backlog/plan | terminal proposal failure, usage, verified database drop, no retry |

## Terminal result and User correction

The User stopped the proposed permanent runner/fake-suite ceremony and requested one
small direct smoke test instead. The frozen candidate used one of its allowed three
model calls. Sol-medium made all four required MCP reads and produced three grounded
proposals, but substituted `shared-non-null-confirmed` for the equal authoritative
Place revision returned by the tools. The strict gate failed before preview or
`submit_action`; no World mutation or retry occurred. Ownership-verified cleanup
dropped the disposable database. Review then established that this was a prompt/
validator mismatch and therefore inconclusive. The User clarified that only the
combined call mattered. One subsequent direct Sol-medium process copied the exact
revision and successfully called `change_entity_state` once with both
`leg_count = 3` and a Trait establishment. HTTP proved both under one Activity, and
ownership-verified cleanup dropped that disposable database too. The temporary
runner was removed after preserving the private terminal evidence and public summary.

## Task details

### T1 — Frozen Sol-medium controller

**Objective:** A permanent token-free suite proves the exact three-phase candidate,
strict tool allowlists, combined payload, confirmation boundary and cleanup.

**Actions:**

1. Implement the dedicated runner, strict output schemas, fake fixtures and digest.
2. Pin premature mutation, extra/incomplete calls, preview drift, empty/single-kind
   packages, wrong revision/identity, HTTP mismatch and unsafe cleanup as failures.
3. Document the operation without authorizing a candidate.

**Invariants:**

- No retained evidence or executable game surface changes.
- No Codex/model process or database preflight in T1.

**Evidence:**

- `tests/entity-state-playtest.sh` and `bash -n` — token-free happy/failure paths.
- calculated/file digest equality and `git diff --check`.

**Stop conditions:** Stop on required game-contract change, copied obsolete runner
behavior, unverifiable session/tool boundary or any model invocation.

### T2 — Token-free readiness

**Objective:** The exact frozen digest is operationally safe before spend.

**Actions:**

1. Run the public preflight once against a disposable owned database.
2. Verify manifest truth and obtain read-only Sol-high review of controller, digest,
   ownership, spend and claim alignment.
3. Record the one exact candidate digest only after GO.

**Invariants:**

- `candidate_started:false`, `authorization_consumed:false`, `codex_invoked:false`,
  `model_calls:0`.
- No candidate command before both preflight and review are GO.

**Evidence:** public preflight manifest, drop log and review with P0–P3 zero.

**Stop conditions:** Stop on environment/catalog/digest drift, stale database,
unsafe cleanup or any audit finding.

### T3 — One terminal live candidate

**Objective:** One exact Sol-medium candidate truthfully succeeds or fails and leaves
no running process or owned database.

**Actions:**

1. Reverify digest, then atomically consume the sole authorization and run the exact
   three-call candidate with no retry.
2. Apply the independent HTTP gate and ownership-safe cleanup even on failure.
3. Record exact calls, usage, reached/unreached phases, terminal claim and restore
   discovery to `Now / Proposed`.

**Invariants:**

- Maximum three model process calls; zero retry/replacement.
- Failure is retained as failure; private prompts/events stay private.

**Evidence:** immutable candidate manifest/events, HTTP snapshots, cleanup log and
public evidence summary; final read-only alignment audit.

**Stop conditions:** Stop permanently after any model-phase/controller/gate failure;
do cleanup and evidence retention only.

## Validation ladder

1. **Focused:** fake controller happy path plus every fail-closed injection.
2. **Contract:** exact runtime catalog/schema/digest and token-free public preflight.
3. **Outcome:** one three-call Sol-medium candidate and independent combined-state
   HTTP proof, or one exact terminal failed claim.
4. **Integrity:** shell syntax, `git diff --check`, private-permission/ownership
   checks, no retained-evidence drift and two read-only reviews.

## Change control

Refine file names and stronger token-free validators in place while the pinned
model/effort, three-call/zero-retry spend, one combined Action, isolated evidence and
narrow claim stay unchanged. Return to `draft` and regain acceptance for another
model/effort, more calls, any retry, a different scenario, existing-evidence
mutation, weaker HTTP gate or broader claim.

## Completion conditions

- T1–T3 and the validation ladder reach their exact terminal states;
- the candidate consumes no more than three process calls and never retries;
- HTTP evidence and cleanup prove the exact success or bounded failure claim;
- retained evidence stays unchanged and authorities agree;
- discovery is restored unchanged to `Now / Proposed`; and
- plan status/completion time are recorded only after terminal evidence.
