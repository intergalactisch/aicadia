---
status: active
created_at: "2026-08-21T21:37:53+02:00"
updated_at: "2026-08-21T21:50:53+02:00"
accepted_at: "2026-08-21T21:50:53+02:00"
completed_at: null
---

# Make and pressure one tick-based manipulation path: the table and the bomb

> **Role / side:** current tick-resolution experiment plan / development side.
> **Authority:** owns this lab's bounded outcome, task order, Agent-run budgets and
> evidence claim.
> **Excludes:** accepted Multiplayer behavior, production architecture, canonical
> vocabulary and delivery evidence; those remain in `game/docs/`, `dev/CONTEXT.md`,
> a later production plan and `dev/docs/evidence/`.

## Outcome

Two real BYO Agents on two different hosts — Claude Code `2.1.238` and Codex CLI
`0.149.0` — each submit one ordinary request inside one tick window against a
standalone simulated World behind a real MCP server: Mara's Agent "flips the table
in the Old Inn" and Bram's Agent, standing in the yard, "drops the bomb on the Old
Inn". The lab World merges the two requests because the table's current Place is
the Inn, chooses one still-connected participant content-blind, returns the whole
set inside that participant's still-open call, accepts that Agent's one follow-up
ordinary submission as the scene, returns the outcome to both open calls, starts the
Entity cooldown, and lets a present third Character's Agent (Nia, idle in the Inn)
read what happened and write her own consequence. Before any model runs, the same
tick core proves solo requests, merge, non-merge, capped crowds, fallback and
cooldown deterministically, and a cheap measurement establishes how long each host
keeps one tool call open.

This is the highest-value current edge because it makes the accepted core direction
exist in the small and tests its two unproven premises at once: that a BYO host
sustains "set returned in the open call, one follow-up call resolves" as one fluent
turn, and that an Agent can author one coherent, structurally valid scene from
overlapping requests that players then experience as multiplayer rather than error
handling. The final evidence may claim only that the exercised hosts, models,
fixture and simulated World completed this exact path, with measured timings and
captured stories judged blind by the User against a stale-error control.

## Non-goals

- No change to `game/docs/`, production `World`, PostgreSQL, migrations, the
  published MCP catalog, Agent instructions, authentication or authorization.
- No accepted tick length, cooldown length, participant ceiling, tool name, schema,
  request envelope, settlement fallback or canonical vocabulary; `tick`, `set` and
  `resolver` remain working terms.
- No Place nesting, city-scale scope, Position-on-Entity merge, spatial reach,
  World-level rules or remote structural links; the fixture has two flat Places.
- No bystander live wait, subscription, progress notification, elicitation or App
  UI; the present Character's ring is tested through an ordinary read only.
- No multi-instance tick storage, broker, fan-out, PostgreSQL contention, scale or
  production recovery claim.
- No claim about ChatGPT Apps, Claude Apps, OpenCode, other host versions, other
  models or players beyond the User's blind judgment.
- No reuse, import or repair of experiment 05, 06 or 07 code or consumed guards;
  only their recorded lessons apply.
- No Agent process beyond the budgets in this plan; zero automatic retries.

## Evidence and authorities

| Evidence or authority | Current fact | Consequence for this build |
| --- | --- | --- |
| [`AGENTS.md`](../../../AGENTS.md) | World is dumb and strict; nothing server-side invokes an Agent or spends tokens; a direct MCP smoke starts with the smallest deterministic proof and names every real and simulated seam. | Token-free tick core first; every Agent process explicit, guarded and counted; the lab World never composes meaning. |
| [Active concept — corrected core direction](../../docs/concept/multiplayer-first-principles.md#corrected-core-direction-one-tick-based-system-for-every-manipulation) | Every manipulation enters a short per-subject tick; one input is applied as authored; several interacting inputs go as one set to one content-blind chosen, still-connected requester who resolves with one follow-up call; cooldown follows; the core path uses only ordinary tool calls. | The lab builds exactly this path and nothing beside it. |
| [Active concept — working tick definition](../../docs/concept/multiplayer-first-principles.md#working-technical-definition-of-a-tick-user-confirmed-as-clearer-2026-08-21) | Requests lie together when targets overlap: same subject, a target whose current Place is the other target, or a Position directly on it; three rings of knowledge; distributed consequences; lazy scale. | The fixture exercises same-subject and Place-membership overlap, a non-merging nested request, the participant ring and the present ring. |
| [Multiplayer Area](../../areas/multiplayer/README.md) | The tick system, the request shape and host smallest-common-denominator are `Chosen`; tick technicalities, per-host fluency and bystander presentation are `Not yet chosen`. | The lab informs those open items and accepts none of them. |
| [Scenario catalogue](../../areas/multiplayer/scenarios.md) S04, S06, S11 | A bomb in a house, an explosion intersecting table changes and one hot Place beside quiet work are fixed pressure cases. | The fixture is S04/S06 in miniature; the capped-crowd and nested-request tests cover the S11 and S10 boundary. |
| [Lab constitution](../../lab/README.md) | Standalone Rust, token-free first, real versus simulated seams, bounded Agent calls announced before execution. | One new crate under `dev/lab/multiplayer/08-tick-resolution/`; deterministic gates before any model. |
| [Experiment 05](../../lab/multiplayer/05-agent-authored-collision/README.md) | A flat multi-phase tool whose validator rejected a published field, and a process-wide strict output schema, made three pinned runs inconclusive. | One ordinary `submit` whose validator accepts every published field in every use; no process-wide output schema; plain final text. |
| [Experiment 06](../../lab/multiplayer/06-place-resource-subscription/README.md) | Pinned Codex never listed, read or followed an MCP resource. | The path relies on tool calls only. |
| [Experiment 07](../../lab/multiplayer/07-host-independent-change-wait/README.md) | A tool that required an exact identifier no prior call exposed produced 583 guesses; the bounded wait is not the participant's route. | `read_here` returns every exact id and version a later `submit` accepts; participants need no wait tool. |
| Locally installed `codex-cli 0.149.0` and Claude Code `2.1.238` | The two real BYO hosts available now. | Pin both exact versions in every manifest; generalize to no other host. |
| Workspace `rmcp = 3.1.1` | The pinned official Rust MCP SDK with streamable HTTP server support. | The lab server uses the same pinned SDK outside the production workspace. |
| User direction, 2026-08-21 | Cheap dry-runs before any pinned run; a blind fun judgment against a stale-error control; budgets released only at acceptance; the plan must not name models because another Agent may execute it while a different Agent reviews. | T2 and T3 use the cheapest sub-agent each host offers; T4 uses the strongest, recorded exactly in the manifest; the User judges in T5; execution and review are separate roles. |

## Alignment

### Strategic

Aicadia's multiplayer promise is that simultaneous play on one thing becomes a
shared moment — the table flies burning through the Inn — rather than a stale
error, while a quiet Inn stays fast and nobody's Agent runs in the background. This
lab is the first time that moment exists end to end with real hosts. A supported
result earns the production design of the tick (storage, exact envelope, timings);
a refuted host premise redirects the resolver route before anything is built; an
inconclusive harness result costs little because every seam is proved token-free
first. The next risk after this lab is multi-instance tick storage in PostgreSQL and
the bystander live layer; neither is folded in here.

### Tactical

The fixture has two flat Places and one simulated World:

- Place H, the Old Inn, and Place Y, the yard; no nesting or adjacency semantics.
- Entity T, the table: `colour brown`, `orientation upright`, `legs 4`, current
  Place H, version 1.
- Entity X, the bomb, held by Bram, current Place Y.
- Characters Mara (in H), Bram (in Y) and Nia (in H, idle). Each is User-controlled;
  only its own Agent may change it.
- A nested Place Z "the cellar" with current Place H and one Entity in it, used only
  by the deterministic non-merge test.

Two lab-only MCP tools, never production names:

- `read_here` — returns the caller's current Place, every Entity there with its
  exact lab id, version and state, and the attributed occurrences since an optional
  version the caller supplies. Every id and version a later `submit` accepts comes
  from this result.
- `submit` — one ordinary request: `targets` (exact ids with the versions read) and
  `wish` (the player's wish in the requester's words). A chosen resolver uses the
  same tool with `tick` (the id it received), `states` (one complete state per
  changed subject) and `story`; it may add `includes` (the contribution ids it kept
  unchanged). The published schema lists every field; the validator accepts every
  published field in every use and rejects only missing required fields, unknown
  ids, stale versions, uncovered facts and ceilings.

The tick, lab values only (reversible assumptions below):

1. A request opens a tick on its targets or joins an open tick whose targets
   overlap: same id, or a target whose current Place is the other tick's target.
   Place nesting is not followed. Each tick admits at most 16 participants; a later
   request is returned "this subject is busy, outcome follows" and receives the
   outcome when the tick settles.
2. The window is 8 seconds from the first request, immutable.
3. At cutoff with one input, World applies it as authored. With several, World
   chooses one still-connected participant by a seeded, content-blind draw and
   returns the set inside that participant's open call: every request (requester,
   targets, wish), bounded current state of the merged subjects and their Place, and
   a count when the set was capped.
4. The resolver has 90 seconds to `submit` its scene. World checks that every
   changed fact is covered by a request in the set kept unchanged or by ordinary
   jointly changeable content, and that no other player's Character changes. It
   commits one state per subject and one occurrence attributing every contribution,
   returns to every open call the settled scene plus one line per contribution (who
   wanted what, kept or not), and starts a 30-second cooldown on every changed
   Entity.
5. If the resolver disconnects, times out or submits an invalid scene, World
   applies the lowest-seeded valid request alone as authored and returns that to
   every open call; a new `submit` on a cooled Entity returns current state and
   remaining seconds.

The real-host scene: the runner starts Mara on Claude Code and Bram on Codex within
two seconds of each other with natural Dutch prompts that reveal no outcome; Nia's
Agent is started only after settlement and asked what happened around her. Success
requires both participant calls to return the same settled scene, the chosen
resolver to have made exactly two `submit` calls, Nia's Agent to name the bomb and
the table from `read_here` and to `submit` one consequence for Nia only, no
polling, and readback equal to the scene.

### Technical

Create one standalone Rust 2024 crate at `dev/lab/multiplayer/08-tick-resolution/`
outside the production workspace, pinning `rmcp 3.1.1` with streamable HTTP server
and client features. One in-memory fixture owns Places, Entities, Characters,
ticks, occurrences and observations; one tick state machine owns open, merge,
admit, choose, resolve, fallback, cooldown and release; thin adapters expose the two
MCP tools and a secret loopback controller for fixture setup, forced disconnect and
readback. Held calls are released by in-process notification; this is a stated
simulated seam, not a multi-instance design. Observations record adapter, tool,
requester, tick id, timings and tool-call counts, never secrets or model reasoning.

The host-budget stage reuses the same server with a controller-set artificial
`submit` delay, so "how long may a call stay open" is measured against the exact
transport the scene will use.

The runner creates task-specific isolated configurations for each host containing
only the lab MCP server, disables other MCP servers, web and shell access, pins both
host versions and the exact models, records prompts, instructions, version
manifests, JSONL observations, host event streams, final text, timings, controller
readback and sanitized transcripts, and verifies process and temporary-directory
cleanup. Pinned runs use persistent one-run guards that fail closed;
cheapest-sub-agent runs are counted against their own ceilings in the same way.

Real seams: the Rust tick state machine and tests, `rmcp 3.1.1` server and client,
streamable HTTP, raw loopback MCP, Codex CLI `0.149.0`, Claude Code `2.1.238`, the
recorded models, captured tool calls, timings, controller readback and cleanup.
Simulated or absent: World, persistence, authority issuance, authentication, Place
nesting and spatial reach, multi-instance release, broker, deployed transport,
Apps, UI, other hosts, production capacity.

## Decisions, assumptions and open questions

### Confirmed decisions

- Every manipulation enters one tick; one input is applied as authored; several
  interacting inputs are resolved by one content-blind chosen, still-connected
  requester who receives the set in its open call and resolves with one follow-up
  call; cooldown follows — User acceptance, 2026-08-21, active concept.
- A request carries only the player's grounded wish and nothing about other
  players; the chosen Agent receives everything about others — User correction,
  2026-08-21.
- The core path relies only on ordinary tool calls across hosts — User acceptance,
  2026-08-21.
- Requests lie together by structural overlap of targets; present Characters learn
  through their Agent's next read and write their own consequences — User found the
  working definition clearer, 2026-08-21.
- Cheap dry-runs precede any pinned run; the User judges fun blind against a
  stale-error control — User direction, 2026-08-21.
- The plan names no model. Each stage says "the cheapest sub-agent the executing
  host offers" or "the strongest the host offers"; every manifest records the exact
  host version and model actually used — User direction, 2026-08-21.
- Execution and review are separate roles: the Agent that executes T1–T4 may be a
  different Agent (for example a Codex/GPT Agent) from the one that reviews the raw
  evidence read-only before T5 closes (for example a Claude Agent) — User
  direction, 2026-08-21.
- One ordinary `submit` with a validator that accepts every published field, no
  process-wide output schema, and `read_here` returning every id and version a
  later call accepts — lessons of experiments 05 and 07.

### Reversible assumptions

- Lab values: 8-second window, 16-participant ceiling, 90-second resolver deadline,
  30-second cooldown, 180-second outer process deadline. They bound the lab and
  absorb cross-host start skew; none is a proposed game timing. T2's measured host
  ceiling may lower the window and deadline before T3.
- Fallback applies the lowest-seeded valid request alone, per the accepted
  content-blind fallback direction; leaving state unchanged is the alternative the
  deterministic tests also exercise.
- The seeded draw uses the tick id and requester ids only; it is content-blind by
  construction and not a fairness claim.
- Nia is a real third Agent process in the pinned run because the present ring is
  part of the claim; in dry-runs she may be the cheapest sub-agent.
- Dutch player prompts, English World content and lab-only tool names introduce no
  product vocabulary.

- Budgets, delegated by the User to Root and accepted with the plan: T2 at most
  eight cheapest-sub-agent processes per host; T3 at most five complete scene
  dry-runs, each up to three cheapest-sub-agent processes; T4 one pinned scene run
  of up to three strongest-sub-agent processes with zero automatic retries, no
  enforceable per-process token ceiling and a 180-second outer deadline per
  process. One second pinned scene run is permitted only when the first failed on a
  harness defect that was afterwards reproduced and fixed token-free in T1 and that
  fix is recorded before the second launch; a model-behavior failure earns no second
  run. The executing Agent records which exact models were used — User delegation
  and acceptance, 2026-08-21.
- A participant that is not chosen receives, in its returned call, the settled
  scene plus one line per contribution — who wanted what and whether it was kept —
  never the raw set with ids and versions, which is the resolver's input only. This
  gives every player's Agent grounded material for its story — User delegation and
  acceptance, 2026-08-21.

### Open questions

- None material. Lab values remain reversible assumptions; T2 may lower the window
  and deadline.

## Implementation map

| Surface | Current state | Intended change | Invariants |
| --- | --- | --- | --- |
| `dev/lab/multiplayer/08-tick-resolution/Cargo.toml` and `Cargo.lock` | Absent. | One standalone Rust lab crate pinning `rmcp 3.1.1` with server, client and streamable HTTP features. | Production workspace and lockfile untouched. |
| `dev/lab/multiplayer/08-tick-resolution/src/lib.rs` | Absent. | Fixture, tick state machine, coverage validation, fallback, cooldown, occurrences, observations. | One canonical state; no semantic inference; no recipient rows. |
| `dev/lab/multiplayer/08-tick-resolution/src/main.rs` | Absent. | Loopback streamable HTTP MCP server exposing `read_here` and `submit`; secret controller route for setup, delay, forced disconnect and readback. | Controller unreachable through MCP; validator accepts every published field. |
| `dev/lab/multiplayer/08-tick-resolution/src/client.rs` and tests | Absent. | Official-SDK client preflight driving read → submit → set → resolve → readback and the host-budget delay tool. | Token-free; independent readback. |
| `dev/lab/multiplayer/08-tick-resolution/fixture/` and `prompt/` | Absent. | Exact fixture JSON, server instructions, three natural Dutch prompts, stale-error control prompt. | Prompts reveal no outcome; English World content. |
| `dev/lab/multiplayer/08-tick-resolution/run` | Absent. | `test`, `preflight`, `budget`, `dry-run` and `live` modes with isolation, counters, guards, deadlines, cleanup and bounded capture. | No mode can exceed its accepted ceiling; live cannot launch before every gate passes. |
| `dev/lab/multiplayer/08-tick-resolution/result/` | Absent. | Manifests, observations, host events, transcripts, readbacks, budget table, blind-judgment record. | No secrets, temporary configurations or raw environment. |
| `dev/lab/multiplayer/08-tick-resolution/README.md` | Absent. | Question, fixture, seams, observations, verdicts per stage, downstream implication. | Lab result never becomes production behavior. |
| `dev/lab/multiplayer/README.md` | Lists 01–07. | Add 08 and its question. | Index owns navigation only. |
| Active concept, Multiplayer Area, concept log, this plan | Corrected direction and working tick definition recorded. | Record only bounded verdicts and the next decision. | `game/docs/`, `dev/CONTEXT.md` and production code unchanged. |

## Execution contract

Root — whichever Agent the User assigns to execute this plan, for example a
Codex/GPT Agent — owns outcome, scope, plan state, every Agent-process budget,
integration and the evidence it retains. Within execution do not split the work
across subagents: fixture, tick machine, adapters and runner are one tightly coupled
seam. Execute one dependency-ready task at a time. Review is a separate role: before
T5 closes, a different Agent (for example a Claude Agent) reads the raw retained
evidence read-only and checks each verdict against it; the executing Agent does not
classify the final verdicts alone. No task changes production runtime, current game
docs, canonical vocabulary or earlier lab artifacts.

## Task graph

Allowed states are `pending`, `in_progress`, `completed` and `blocked`.

| ID | State | Depends | Parallel-safe | Objective | Owned surfaces | Evidence |
| --- | --- | --- | --- | --- | --- | --- |
| T1 | pending | — | no | Build and prove the tick core, both tools and the raw MCP route without any model. | Lab crate, fixture, prompts, `run test` and `run preflight`, initial README | Deterministic tests and loopback preflight pass with zero Agent processes. |
| T2 | pending | T1 | no | Measure how long one tool call may stay open on each host and whether the turn continues. | `run budget`, budget table in `result/` | Per-host table for 5/15/30/60 s; recorded ceiling; window and deadline adjusted if needed. |
| T3 | pending | T2 | no | Run the complete scene with the cheapest sub-agent each host offers until the contract holds or the ceiling is reached. | `run dry-run`, dry-run results, token-free fixes | At least one complete accepted scene with both stories, Nia's consequence, tool-call counts and timings; every fix re-proved token-free. |
| T4 | pending | T3 | no | Execute one pinned scene run (a second only under the harness-defect rule) under guards. | `run live`, guards, live results | One pinned scene run, zero automatic retries, captured transcripts, readback and cleanup. |
| T5 | pending | T4 | no | Blind judgment, verdicts and alignment. | README, lab index, active concept, Area, log, this plan | User's blind judgment recorded; per-stage verdicts reconcile with raw evidence; Studio lint passes. |

## Task details

### T1 — Prove the tick core token-free

**Objective:** The tick state machine, coverage validation, both tools and the raw
MCP route behave exactly as the tactical section states, for the fixture only.

**Actions:**

1. Implement fixture, tick machine, seeded draw, coverage validation, fallback,
   cooldown, occurrences and observations.
2. Add the MCP server with `read_here` and `submit`, the controller, and the
   official-SDK client.
3. Add tests: solo request; flip + bomb merge through Place membership; bomb after
   settlement meets cooldown; 16 and 1,000 requests on one table with ceiling and
   count; a request on Z does not merge with a request on H; forced resolver
   disconnect falls back; a scene changing Nia is rejected; a scene with an
   uncovered fact is rejected; every published `submit` field is accepted in both
   uses; a schema/validator parity test.
4. Add the raw loopback preflight: read → two submits → set in the open call →
   resolve → both calls return → readback, with `model_calls: 0`.

**Invariants:**

- Zero model processes can launch from T1.
- World never infers meaning; merge uses only ids and stored placement.
- No recipient rows, no replay, no process-local state presented as a design.

**Evidence:**

- `cargo test --manifest-path dev/lab/multiplayer/08-tick-resolution/Cargo.toml` —
  all tests pass.
- `dev/lab/multiplayer/08-tick-resolution/run preflight` — bounded manifest with
  zero Agent processes.

**Stop conditions:**

- Stop if the pinned SDK cannot keep one tool call open for the window, if any
  merge or coverage case contradicts the working definition in a way that needs a
  User choice, or if cleanup is incomplete.

### T2 — Measure the per-host open-call budget

**Objective:** A table of whether each host returns a `submit` that stays open for
5, 15, 30 and 60 seconds and continues the turn afterwards.

**Actions:**

1. Controller sets an artificial delay; one process of the cheapest sub-agent the
   host offers, per host per duration, in isolated configuration, counted against
   the ceiling.
2. Record exit status, whether the tool result arrived, whether the turn produced a
   final message, wall time and any host timeout configuration involved.
3. Lower the lab window and resolver deadline if the ceiling requires it and record
   that change as a reversible assumption.

**Invariants:**

- At most eight processes per host; no retries; only the cheapest sub-agent.
- The delay tool changes no fixture state.

**Evidence:**

- `dev/lab/multiplayer/08-tick-resolution/run budget` — retained table per host.

**Stop conditions:**

- Stop and return to the User if a host cannot sustain the shortest useful window;
  the resolver route must then be redesigned before T3.

### T3 — Dry-run the scene with the cheapest sub-agents

**Objective:** One complete accepted scene on both hosts with the cheapest
sub-agent each host offers.

**Actions:**

1. Start Mara (Claude Code) and Bram (Codex) within two seconds; after settlement
   start Nia; capture everything.
2. After each dry-run, fix only contract, prompt or harness issues token-free and
   re-prove them through T1's tests and preflight before the next run.
3. Capture one stale-error control: the same scene against a controller-forced
   "stale, try again" response, for the blind judgment.

**Invariants:**

- At most five scene dry-runs; each counted; only the cheapest sub-agent.
- No prompt reveals the outcome; no host-specific tool or schema appears.

**Evidence:**

- Retained dry-run manifests: two `submit` calls by the resolver, one by the other
  participant, one `read_here` and one `submit` by Nia, zero polling, readback equal
  to the scene.

**Stop conditions:**

- Stop if the ceiling is reached without one accepted scene, or if a fix would
  change the working tick definition; return the plan to the User.

### T4 — One pinned scene run

**Objective:** The same scene with the strongest sub-agent each host offers,
recorded exactly, once.

**Actions:**

1. Re-run every T1 gate, verify host versions, models and isolation, consume the
   guard, run the scene and Nia, capture, read back and clean up.
2. Never retry automatically; a failure after launch is retained as the result.
3. Only if that failure is a harness defect: reproduce and fix it token-free in T1,
   record the fix, then consume a second and final guard for one more run.

**Invariants:**

- One pinned scene run, or two only under the recorded harness-defect rule; zero
  automatic retries; budgets as accepted.

**Evidence:**

- Retained live manifest, transcripts, observations, readback and cleanup proof.

**Stop conditions:**

- Any failed gate leaves the guard untouched and stops before launch.

### T5 — Judge blind and record

**Objective:** One inspectable verdict per stage and an aligned record.

**Actions:**

1. Present the User the pinned transcripts and the control without labels; record
   the judgment verbatim.
2. Have the separate reviewing Agent read the raw retained evidence and confirm or
   contest each proposed verdict; record its findings.
3. Classify per stage: tick core, host budget, dry-run contract, pinned scene, fun.
4. Align README, lab index, active concept, Area, log and this plan.

**Invariants:**

- Deterministic, host, Agent and judgment evidence stay separate claims.
- No lab name, value, prompt or tool becomes production vocabulary or contract.

**Evidence:**

- `cargo test -p aicadia-studio --lib` and `git diff --check` pass; verdicts
  reconcile mechanically with retained raw evidence.

**Stop conditions:**

- Stop and return to `draft` if a result would require a product behavior, public
  contract or further Agent run to interpret.

## Validation ladder

1. **Focused:** standalone tests for merge, non-merge, ceiling, draw, coverage,
   fallback, cooldown and schema/validator parity; loopback preflight.
2. **Contract:** the same `read_here`/`submit` contract serves participants,
   resolver and present Character; no published field is rejected; every id and
   version a call accepts came from a prior result.
3. **Outcome:** one pinned run where both hosts complete the tick as one fluent turn,
   the resolver's scene is accepted, Nia narrates and writes her consequence, and
   the User's blind judgment is recorded.
4. **Integrity:** `cargo test -p aicadia-studio --lib`, `git diff --check`, focused
   diff review and confirmation that production, `game/docs/`, `dev/CONTEXT.md` and
   earlier labs remain untouched.

## Change control

Refine lab paths, tests, fixture values and stronger evidence in place while the
accepted outcome, budgets and claim remain unchanged. Stop, keep or return to
`status: draft`, revise and request explicit re-acceptance when new evidence changes
the outcome, the working tick definition, the real/simulated seam boundary, the
host or model set, any Agent-process ceiling or the evidence claim.

## Completion conditions

- T1–T5 are `completed` and the validation ladder passes;
- no stage exceeded its accepted Agent-process ceiling and no run was retried
  automatically;
- each stage has one supported, refuted or inconclusive verdict grounded in its own
  retained evidence, plus the User's blind judgment;
- active concept, Area, log, lab index and this plan agree;
- `game/docs/`, `dev/CONTEXT.md`, production code and earlier labs are unchanged;
- `status: complete` and `completed_at` are recorded only after these conditions.
