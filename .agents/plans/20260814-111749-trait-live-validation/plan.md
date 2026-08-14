---
status: complete
created_at: "2026-08-14T11:17:49+02:00"
updated_at: "2026-08-14T13:26:52+02:00"
accepted_at: "2026-08-14T11:21:38+02:00"
spend_authorized_at: "2026-08-14T13:02:10+02:00"
completed_at: "2026-08-14T13:26:52+02:00"
---

# Trait live validation after controller correction

## Outcome

Run at most one new current-protocol live candidate to establish whether a real
User-steered Agent can complete the already delivered Trait flow: ground through
Aicadia MCP, establish one Entity-owned Trait through a confirmed Action, develop
the same stable Trait through a later confirmed Interaction, and let a second Agent
retrieve the resulting Entity state and authorized Activity history. World remains
the sole validator and writer.

The valuable result is exact live evidence. A successful candidate earns only the
full end-to-end claim above. A failed candidate is retained as one exact failure and
never becomes gameplay success. This is the highest-value current risk because the
deterministic Trait contract is delivered, while all three paid candidates stopped
before any post-setup Agent submit or Trait mutation.

## Non-goals

- Changing Trait, Action, Interaction, Activity, HTTP or MCP behavior.
- Adding another protocol, compatibility path, retry or fallback.
- Reusing or rewriting historical candidate evidence.
- Running more than one candidate under this plan.
- Treating plan acceptance, archive work, preflight or audit GO as paid-run
  authorization.
- Starting investigation/discovery or another game-development edge.

## Evidence and authorities

| Evidence or authority | Current fact | Consequence for this build |
| --- | --- | --- |
| `docs/game/trait-playtest.md` and private `candidate-63hjH4HW` | The current-only MCP `2026-07-28` candidate consumed the exact digest authorization and failed after two model calls solely on live-only preview-name validator drift | Preserve the exact failure; no retry, new authorization or live Trait success |
| `.agents/plans/20260813-200829-entity-trait-development/plan.md` | Deterministic Trait delivery is complete; both paid candidates are exact failures | Preserve that completed plan and never relabel either failure |
| `.aicadia-trait-playtest/archive-replacement-a564-ydttdFfc` | The consumed a564 sentinel, 51-file candidate and two six-file preflights are preserved as 64 unchanged historical files plus one private archive index; both candidate manifests end `ownership_verified_and_dropped`/database `dropped` | Keep this mode-700 archive unchanged beside the private T3 preflight evidence |
| `.aicadia-trait-playtest/archive-original-f38-MmwRmcBv` | The original 68-file failure archive is private and independently verified | Keep it in place and unchanged |
| `tools/trait-playtest` | `gpt-5.6-sol`, high reasoning, at most seven model process calls, zero retries, exact digest gate and name-plus-token database cleanup are enforced | Reuse the audited controller; do not create another runner or bypass its gates |
| User acceptances, 2026-08-14 | Plan acceptance authorized T2/T3 and the separate fresh GO at `2026-08-14T13:02:10+02:00` authorized one T4 candidate; that authorization is consumed | Plan closed after independent T5 GO; controller correction and new live proof remain deferred behind draft documentation-architecture work |

## Alignment

### Strategic

Aicadia already supports stable Entity-owned Trait development across World,
PostgreSQL, HTTP and MCP. The remaining concrete risk is whether a real connected
Agent can carry that contract through selection, confirmation, mutation and later
observation. Retiring that risk advances the shared-world game more than adding a
new capability on top of unearned live evidence.

### Tactical

Use the existing two-User disposable scenario. Pip and Mara receive separate Users
and Characters at one entry Place. Pip's Agent grounds through reads, proposes three
directions, previews the selected Action and submits it once after confirmation.
World establishes one Pip-owned Trait and writes Activity. Later the same Agent
previews and submits one Interaction that develops the same Trait id. HTTP verifies
both authoritative results. Mara's separate Agent then retrieves Pip and authorized
Activity, proving current state plus continuity. Setup writes are acknowledged;
only post-setup Agent submissions count toward the live Trait claim.

### Technical

No domain, schema, HTTP, MCP or runner code change is planned. The operational seams
are the ignored evidence root, digest-bound token-free preflight, independently
audited readiness, one separately authorized controller invocation and exact final
authority alignment. Every candidate database uses the existing exact
name-plus-unguessable-token ownership check before drop. On ownership ambiguity the
runner retains recovery evidence and does not guess.

## Decisions, assumptions and open questions

### Confirmed decisions

- Only MCP `2026-07-28` is current; no initialize compatibility, downgrade or
  fallback is allowed.
- The two previous candidates and their consumed authorizations remain historical
  failures.
- Trait state is Entity-owned; User intent/confirmation, Agent authorship and World
  validation/write remain distinct.
- The separate fresh User GO at `2026-08-14T13:02:10+02:00` authorized only the
  exact published T4 command for one candidate. `candidate-63hjH4HW` consumed it;
  failure authorizes no retry or other candidate.
- The User directed this plan to close after T5 independent review. That review
  returned GO with P0/P2/P3 at zero and the known controller P1 deferred. Controller
  correction and any new live proof are deferred behind the still-draft
  documentation-architecture plan; that plan is not started or activated here.

### Reversible assumptions

- T2 and T3 reproduced repository digest `6649959c7f230f2240f8d1b7e67081c20c473c00654ef36409fa439a8d85a824` before archive work and public preflight respectively.
- A machine-specific `DATABASE_URL` is supplied at execution time and never stored
  in repository configuration or digest material.
- Codex remains exactly `codex-cli 0.147.0`; semantic version/config checks, not one
  developer's executable path, decide readiness.

### Resolved spend outcome

- The User explicitly authorized the exact digest-bound paid command at
  `2026-08-14T13:02:10+02:00`. `candidate-63hjH4HW` started once, consumed that
  authorization and failed; no retry or new authorization exists.

## Implementation map

| Surface | Current state | Intended change | Invariants |
| --- | --- | --- | --- |
| `.aicadia-trait-playtest/` | Two historical archives, `preflight-HMxwGPCF`, the consumed sentinel and private `candidate-63hjH4HW` exact failure | Preserve every private artifact after closure | Root/directories 700, files 600; no live process |
| `tools/trait-playtest`, schemas, digest | Audited corrected runner at digest `6649959c…` | No code change; reproduce digest and use existing gates | Seven process calls maximum, zero retries, current MCP only, no public overrides |
| Disposable PostgreSQL database | Absent after candidate cleanup `ownership_verified_and_dropped` | No further database action | Never drop without ownership proof |
| Trait World/HTTP/MCP contract | Deterministically delivered | No change | Entity ownership, stable Trait id, atomic Activity history and adapter parity remain exact |
| `docs/game/`, concept log, backlog and this plan | Trait deterministic delivery remains Done; live success is unearned and T5 review returned GO | Record the exact failure and completed closure | No active edge; do not activate or edit the chosen draft documentation plan |

## Execution contract

Root owns outcome, scope, plan state, integration and the final evidence claim. One
Sol High implementation Agent may own one dependency-ready task at a time and must
re-read live files, preserve private evidence, run its focused checks and return raw
results. A separate Sol High Agent performs readiness and final read-only reviews.
No tasks run in parallel because they share evidence and authority state.

## Task graph

| ID | State | Depends | Parallel-safe | Objective | Owned surfaces | Evidence |
| --- | --- | --- | --- | --- | --- | --- |
| T1 | completed | — | no | Accept and publish the exact validation boundary | plan, concept log, backlog/map, current Trait docs | Accepted at `2026-08-14T11:21:38+02:00`; exactly one Trait live-validation `Now / Active` edge is published, T2/T3 alone are token-free authorized and no spend/model/candidate claim is made |
| T2 | completed | T1 | no | Preserve a564 evidence and produce a clean active root | ignored Trait evidence root, plan evidence record | 64 historical files/3 directories/67 paths retain path `3e596225…`, content `a009db36…` and permission `da4d6314…`; the mode-600 index declares the exact SHA-256 serialization, f38 remains 68 files with content `77f796a9…`/permission `1c61d45a…`, and T2 ended with only two archives active |
| T3 | completed | T2 | no | Earn token-free readiness GO for digest `6649959c…` | ignored preflight evidence, plan/docs readiness status | `preflight-HMxwGPCF` returns token-free GO with zero candidate/model activity, current MCP, 13 live-equal tools, 6 schemas and ownership-verified drop; after the exact two P3 corrections, independent re-review returned GO with no P0–P3 finding |
| T4 | completed | T3 + fresh explicit spend GO received 2026-08-14 | no | Run at most one paid live candidate | ignored candidate evidence only | `candidate-63hjH4HW` consumed the authorization and two model calls; Action proposal passed, then a valid tool-free preview was falsely rejected by live-only `startswith("Pip ")` drift before any submit/Trait mutation/HTTP gate; zero retries and ownership-verified drop |
| T5 | completed | T4 | no | Align and independently review the exact earned outcome | plan, docs/game, backlog/map, concept log | Independent GO for completed-as-failed alignment: P0/P2/P3=0; known deferred P1 is live Action `startswith("Pip ")` and analogous unreached Interaction `startswith("Pip ")`/`startswith("Mara ")` drift against prompt/fake exact names |

## Task details

### T1 — Accept the validation boundary

**Objective:** Make the exact cost, protocol, ownership, evidence and no-retry
boundary authoritative before any operational change.

**Actions:**

1. Present this complete plan and wait for explicit acceptance.
2. On acceptance, set the plan active and make Trait live validation the only
   `Now / Active` backlog edge without changing the delivered game contract.

**Invariants:**

- Plan acceptance authorized only T2/T3 token-free work. The later fresh T4 spend
  GO was received at `2026-08-14T13:02:10+02:00` for the exact published command.

**Evidence:**

- Plan status/acceptance timestamp, exact one-Active scan and authority links pass.

**Stop conditions:**

- Keep the plan draft if the User changes model, effort, process-call maximum,
  retry, protocol, ownership, success claim or spend boundary.

### T2 — Archive the consumed a564 evidence

**Objective:** Preserve the second failed candidate exactly while restoring an
unambiguous active evidence root for a new preflight.

**Actions:**

1. Verify no playtest process remains and both previous candidate databases have
   terminal ownership-verified cleanup.
2. Record count, relative paths, content fingerprints and permission fingerprints
   for `candidate-consumed`, `candidate-ydttdFfc`, `preflight-RO7ap0Rk` and
   `preflight-cEeVelIZ`.
3. Move exactly those four entries into a new mode-700 a564 replacement archive;
   keep `archive-original-f38-MmwRmcBv` byte- and permission-unchanged.
4. Recompute identical counts/fingerprints/modes and confirm no top-level sentinel
   or candidate directory remains.

**Invariants:**

- No historical file content is edited, deleted or made public.
- No database, server, Codex command, preflight or model process runs.

**Evidence:**

- Private archive manifest plus independent read-only verification of exact
  before/after identity and clean active-root gates.

**Result:** Completed without preflight, database, server, Codex, model or candidate
activity. Both failed candidate manifests were terminal
`ownership_verified_and_dropped` with database `dropped`, and no relevant process
was live. Exact moved set `candidate-consumed`, `candidate-ydttdFfc`,
`preflight-RO7ap0Rk`, `preflight-cEeVelIZ` retained 64 files, 3 directories and 67
relative paths: path-set SHA-256
`3e596225da39f7324700a05e3008854e08154eede7edd3131be85b871a37960d`,
path-plus-bytes SHA-256
`a009db36cf09e4eda6b4b006fd62bda44876aeb0c96d04f419fef1e1001eae38`
and path/type/mode SHA-256
`da4d631445756aa29bd66bd9011434e9ca70047dd7c062c1dc503fa23dfa114c`
before and after the explicit move. Mode-600 `archive-manifest.json` records this
identity and declares the exact bytewise/NUL-delimited SHA-256 serialization,
making the mode-700 replacement archive 65 files and 4 directories total.
Original f38 remained 68 files/6 directories/74 paths with content SHA-256
`77f796a98427bc5948bc474c708e0b52de15e5ffd85ed27768f01526ee738db0`
and permission SHA-256
`1c61d45a8343001e06eede6c68bc8e2ed9832d08cb6e8e859e97da08ec2d25dc`.
At T2 completion the active root contained exactly the original and replacement
archive directories; T3 then created only its private preflight directory.

**Stop conditions:**

- Stop on a live process, non-terminal cleanup, fingerprint/mode mismatch,
  unexpected top-level entry or ambiguous archive ownership.

### T3 — Token-free preflight and independent readiness review

**Objective:** Prove the exact corrected candidate is operationally ready without
starting a candidate or spending model tokens.

**Actions:**

1. Reproduce the frozen digest and run the public preflight with a caller-supplied
   local PostgreSQL administration URL.
2. Verify Codex version/login/config, `gpt-5.6-sol` high availability, only MCP
   `2026-07-28`, six schemas, thirteen live tools, runtime catalog equality and
   ownership-safe database cleanup.
3. Obtain an independent read-only GO/NO-GO review of archive identity, preflight
   manifest, call/retry boundary, current authorities and paid command.
4. If GO, present the exact digest-bound command and evidence summary to the User;
   do not execute it.

**Invariants:**

- Preflight records `candidate_started=false`, `authorization_consumed=false`,
  `codex_invoked=false`, `model_calls=0`.
- Running version/config inspection is allowed; `codex exec` is not.

**Evidence:**

- One private preflight manifest with GO, verified drop and independent review with
  no P0–P3 finding.

**Execution result:** Public token-free `preflight-HMxwGPCF` returned exit 0 and GO
for exact digest
`6649959c7f230f2240f8d1b7e67081c20c473c00654ef36409fa439a8d85a824`.
Its private manifest records `candidate_started=false`,
`authorization_consumed=false`, `codex_invoked=false`, `model_calls=0`,
`gpt-5.6-sol` high, maximum seven process calls, zero retries, Codex CLI 0.147.0
login verified, 13 live-equal tools, 6 passed schemas, database `dropped` and cleanup
`ownership_verified_and_dropped`. Digest-bound configuration validation proved MCP
`2026-07-28` enabled. Both archives retain their preflight fingerprints and no
top-level sentinel or candidate exists. This is preflight-ready evidence only:
the initial independent T3 review returned NO-GO with exactly two P3 findings. The
private index now machine-declares the exact hash serialization without changing any
historical bytes or hashes, and all current authorities now state the completed T2
and actual preflight result. Independent re-review then returned GO with no P0–P3
finding, completing T3 readiness. At that T3 boundary paid execution was unexecuted;
T4 subsequently ran exactly once as recorded below.

**Stop conditions:**

- Stop on digest drift, schema/catalog mismatch, unsupported current protocol,
  login/model drift, database ownership ambiguity or any candidate/model activity.

### T4 — Run one separately authorized candidate

**Objective:** Produce one authoritative live Trait continuity outcome under the
frozen controller.

**Actions:**

1. Use the fresh explicit User spend GO received at
   `2026-08-14T13:02:10+02:00` for the exact published command to start one
   candidate; its sentinel atomically consumes the authorization.
2. Execute the frozen Action proposal/preview/commit, first HTTP gate, later
   Interaction proposal/preview/commit, second HTTP gate and Mara observation.
3. Stop on the first mismatch and always perform exact ownership-verified cleanup.

**Invariants:**

- Maximum seven `gpt-5.6-sol` high model process calls, zero retries and no token
  ceiling claim.
- One candidate only; no protocol fallback or evidence-root reset after failure.
- Only World acceptance writes Trait state and Activity history.

**Execution result:** `candidate-63hjH4HW` consumed the exact digest
`6649959c7f230f2240f8d1b7e67081c20c473c00654ef36409fa439a8d85a824`
authorization and two `gpt-5.6-sol` high model process calls. Action proposal passed
after exactly `get_world`, `get_character`, `list_entity_at_current_place` and
`list_activity_at_current_place`. Action preview then exited 0 with zero tools
and returned the strict requested JSON, including `entity_name: "Pip"`; the live
validator alone required `startswith("Pip ")`, while the prompt and permanent fake
contract require exact natural name `Pip`. Validation therefore failed before
Action commit, every candidate HTTP gate, Interaction and Mara fetch. No post-setup
Agent submit or Trait mutation occurred; no candidate HTTP gate, Interaction or Mara
phase started, and there were zero submits. Total usage was 148068 input, 100352
cached input, 1722 output and 867 reasoning-output tokens. The private evidence is
retained, the disposable database was ownership-verified and dropped, no process
remains and no retry or new authorization exists.

**Evidence:**

- Private manifest/event/final/HTTP evidence proves the complete chain or retains
  one exact failure, usage and cleanup result.

**Stop conditions:**

- Stop immediately on an unexpected tool, proposal/preview drift, duplicate/missing
  submit, HTTP mismatch, ownership ambiguity or exceeded call boundary.

### T5 — Align and close

**Objective:** Make every current authority state exactly what the one candidate
proved and nothing more.

**Actions:**

1. Align current game docs, Trait playtest contract, concept log, backlog/map and
   plan with the exact manifest and cleanup facts.
2. Run final independent Terry/ownership review and close only with no P0–P3
   finding.
3. Return the validation edge to Done without automatically starting another edge.

**Invariants:**

- A failure never becomes live Trait success and never authorizes a retry.
- Private paths, tokens and credentials never enter tracked files.

**Evidence:**

- Authority matrix, link/stale/personal-path/legacy scans, digest reproduction,
  evidence permission verification and independent GO.

**Stop conditions:**

- Keep the plan active while any current authority, ownership fact or evidence claim
  disagrees.

**Closure result:** Independent T5 review returned GO for the exact completed-as-failed
alignment with P0/P2/P3 at zero. The known deferred P1 is the live-only Action
`startswith("Pip ")` drift and the analogous unreached Interaction
`startswith("Pip ")`/`startswith("Mara ")` drift versus prompt/fake exact names. No
fix, retry, live success or new authorization was added. The User chose draft plan
`.agents/plans/20260814-130554-documentation-architecture/plan.md` to be picked up
next. This plan closes without editing or activating that draft.

## Validation ladder

1. **Focused:** digest reproduction; valid/invalid date-time regressions; both
   token-free fake suites; archive count/content/permission identity.
2. **Contract:** public token-free preflight proves exact current MCP/catalog/schema,
   Codex boundary and ownership-safe database cleanup with zero model calls.
3. **Outcome:** one separately authorized candidate proves the full Action → HTTP →
   Interaction → HTTP → second-Agent observation chain, or retains one exact failed
   boundary without retry.
4. **Integrity:** independent Terry/ownership review, `git diff --check`, authority
   links/status, no personal tracked paths, no legacy protocol and unrelated user
   changes preserved.

## Change control

Refine paths, order and stronger token-free evidence in place while the accepted
outcome remains unchanged. Return this plan to draft and request re-acceptance if
model/effort, process-call maximum, retry, protocol, ownership, evidence claim,
material cost or paid command changes. Any paid execution always requires the
separate fresh T4 spend GO even after plan acceptance.

## Completion conditions

- T1–T5 are completed and the validation ladder passes;
- exactly one new candidate outcome and ownership-safe cleanup are retained;
- current docs, concept history, backlog and plan agree without overclaim;
- no retry, compatibility path, personal tracked path or accidental unrelated
  change remains;
- status becomes complete only after final independent GO.
