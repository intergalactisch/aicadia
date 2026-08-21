---
status: complete
created_at: "2026-08-20T20:40:40+02:00"
updated_at: "2026-08-21T10:43:00+02:00"
accepted_at: "2026-08-20T21:09:14+02:00"
completed_at: "2026-08-21T10:43:00+02:00"
---

# Test one Agent-authored collision in one explicit BYO turn

> **Role / side:** current Agent-authored collision experiment plan / development side.
> **Authority:** owns the bounded lab outcome, task order, cost authority and evidence
> claim for the first live test of player-visible collision handling.
> **Excludes:** accepted Multiplayer behavior, production architecture and delivery
> evidence; those remain in `game/docs/`, a later production plan and
> `dev/docs/evidence/`.

## Outcome

One explicitly invoked User-owned Agent encounters several simultaneous proposed
changes to one popular Table, tells its player about that situation in natural World
language, and within the same model process authors one new coherent exact Table
state under a fixture-only creative mandate. A standalone lab MCP server validates
only structure and stores exactly one simulated current state plus one simulated
Activity. An independent controller reads that result back.

This is the highest-value current Multiplayer edge because it tests the unique GX
promise before Aicadia chooses timing, persistence, fairness or database machinery:
whether a collision can become understandable creative multiplayer rather than a
stale error. The final evidence may claim only that the exercised pinned Agent and
real MCP interaction completed this exact fixture, with its measured latency and
captured presentation, against a simulated World.

## Non-goals

- No change to `game/docs/`, production World behavior, PostgreSQL, migrations,
  HTTP, the published MCP catalog, Agent instructions or current Action semantics.
- No production terminology, universal change package, permanent collision record,
  selection algorithm, cooldown or fallback policy.
- No claim about ChatGPT, Claude, other Codex versions, other models, every BYO host,
  actual concurrent Agents, database atomicity, security, production capacity or
  million-User scale.
- No multi-Agent conversation. Three source contributions are deterministic fixture
  data; only the selected finalizing Agent is real.
- No attempt to prove that the Agent's creative compromise is objectively correct.
  The User judges whether it is coherent, recognizable and fun.
- No fourth candidate run or automatic retry. The two inconclusive process records
  remain immutable evidence; after reviewing candidate 02's empty published mutation
  schema, the User conditionally authorized one final candidate only after a stronger
  complete token-free harness gate passes.

## Evidence and authorities

| Evidence or authority | Current fact | Consequence for this build |
| --- | --- | --- |
| `AGENTS.md` | Agents own semantic intelligence; World may only validate exact structure, authority, bounds, current facts and settlement. | The lab server cannot infer meaning or score the Agent's output. |
| `dev/docs/concept/multiplayer-first-principles.md` | One common lifecycle and Agent-only final-state authorship are accepted; seconds of collision time are allowed only as visible gameplay. | The fixture uses the ordinary submission first, then exposes collision inside the same explicit Agent turn and captures player-facing narration. |
| `dev/docs/research/blank-slate-multiplayer-mechanics.md` | Bounded live Agent editing is the strongest new GX candidate and host continuation is its first falsifier. | Test the actual model/tool continuation before state-machine scale or PostgreSQL. |
| `dev/areas/multiplayer/scenarios.md` | S02 and S13 pressure hot same-Entity change and bounded collective authorship; S06 pressures dependency-aware composition. | The fixture contains compatible and contradictory Table contributions plus exact current dependencies. |
| User direction, 2026-08-20 | Use whichever experiment best shows the system; a collision may take seconds when the player is told what others are trying. | Give the real Agent a fixture-only bounded mandate to author a genuinely new state, not merely select one existing candidate. |
| User correction authorization, 2026-08-20 | The first process was rejected before inference by unsupported `uniqueItems`; after seeing that exact failure and the unchanged no-token-ceiling boundary, the User authorized one corrected call. | Preserve candidate 01, run token-free gates over a separate candidate 02, consume at most one new process and never retry it. |
| User final-candidate authorization, 2026-08-20 | Candidate 02 read correctly but received an empty published mutation schema. The User requires everything to be correct first and then permits one more test under the same no-retry and no-enforceable-token-ceiling boundary. | Candidate 03 cannot consume authority until a full raw read→propose→collision→resolve→readback preflight, exact schema checks, prior-guard checks and cleanup gates all pass. |
| `game/docs/agent.md` and `game/docs/capability/submit_action.md` | Current production requires exact User confirmation and has no collision continuation. | Run on a separate lab MCP server; success cannot silently evolve current Agent conduct or Action. |
| `dev/lab/README.md` | Technical labs use standalone Rust, name real and simulated seams and bound model cost. | Build one independent Rust crate and one fail-closed explicit runner. |
| `dev/playtest/agent/run` | The repository already demonstrates isolated Codex CLI configuration, strict output schemas, event capture and zero-retry paid execution. | Reuse its operational lessons, not its code or current production catalog. |

## Alignment

### Strategic

Aicadia becomes meaningfully multiplayer when overlapping creativity is perceivable
and can influence one shared outcome. This test asks whether BYO Agent intelligence
can provide that missing game layer while World remains dumb and strict. A supported
result earns a deterministic collision-state experiment; a refuted result prevents
database work from entrenching a mechanic that Agents or players cannot use.

The next risk after this experiment is bounded deterministic admission and timeout
behavior for one hot exact fact. It is deliberately not built here.

### Tactical

The fixture contains one Table with an exact version and only three writable facts:
colour, orientation and number of legs. The User explicitly asks for a blue Table and
grants the Agent permission, for this experiment only, to reinterpret that wish
creatively when simultaneous contributions appear, provided the wish remains
recognizable.

The lab MCP flow uses temporary working operation names that are not Aicadia
terminology:

1. `read_subject` returns the Table's exact current facts and version.
2. `submit_change` with a complete blue candidate returns a bounded collision rather
   than committing. It includes three simulated source contributions: turn the Table
   over, reduce it to three legs and make it red. Each has exact proposed state,
   outward in-World summary, source authority receipt and immutable identity. One
   summary contains hostile instruction-like text as untrusted game content.
3. Before its next tool call, the Agent must emit a concise Dutch in-World message
   explaining that the Table is popular and naming the grounded simultaneous
   activity without protocol language.
4. The same `submit_change` tool receives a collision continuation, exact selected
   source identities, one newly Agent-authored English final state and canonical
   English prose.
5. The lab server validates only fixture identity, source receipts, exact base
   version, allowed subject and keys, types, size, request identity and deadline. It
   rejects unknown subjects, keys and sources. It does not decide whether the result
   is creative, coherent, fair or recognizably blue.
6. Acceptance atomically replaces the simulated Table facts and appends exactly one
   simulated Activity. A controller-only `read_result` operation independently reads
   state, source attribution and Activity after the Agent exits.
7. A generated transcript presents the User prompt, the Agent's collision message,
   the accepted final player message and the authoritative readback for User review.

The final state must differ structurally from every individual source candidate and
cite at least two contributions; otherwise the run has not exercised creative
synthesis. The source User, other Users, current World, real persistence, actual
network races and interactive player UI remain simulated or absent.

### Technical

Create one standalone Rust 2024 crate at
`dev/lab/multiplayer/05-agent-authored-collision/`. It depends on the workspace-pinned
MCP, async, schema and JSON libraries but not on `aicadia-game`. One canonical
in-memory state machine owns fixture state, validation, request receipts and
simulated Activity. A small streamable-HTTP MCP server exposes only the three lab
operations above.

A fail-closed lab runner starts the server on loopback with an owned random port,
uses an isolated temporary Codex home, enables only the lab MCP server and only
`read_subject` plus `submit_change` for the Agent, disables unrelated tools and web,
and executes at most one pinned `gpt-5.6-sol` high-reasoning Codex process per
separately authorized candidate. The controller calls `read_result` directly after
the model exits. All processes and temporary configuration are cleaned even on
failure.

The runner stores a bounded manifest, prompt, strict final-output schema, JSON event
stream, stderr, authoritative result, latency observations and a sanitized transcript
inside the experiment's `result/` directory. No secrets, auth material, temporary
home or raw environment are retained.

The real seams are the Rust state-machine implementation and tests, lab MCP server,
streamable-HTTP protocol, Codex CLI process, selected model, model tool continuation,
captured Agent messages, controller MCP readback and local wall-clock measurements.
The simulated seams are World, Users other than the prompted User, concurrent
arrival, authority issuance, transaction/storage durability, Activity persistence,
delivery, player UI and production failure recovery.

Cost is bounded to three launched model processes across the retained experiment:
candidate 01 was rejected before inference, candidate 02 stopped on an empty
published mutation schema and candidate 03 is conditionally authorized only after
the hardened token-free gate. Each candidate has its own persistent one-call guard; total
automatic retries remain zero and there is no background Agent work. The selected
Codex CLI exposes no enforceable per-run token ceiling; both explicit User
authorizations accepted that limitation. Preflight and deterministic tests spend
zero model tokens.

## Decisions, assumptions and open questions

### Confirmed decisions

- Several seconds are acceptable only as player-visible multiplayer gameplay — User
  direction recorded in the active concept and Multiplayer Area.
- The first experiment should show the strongest unique promise: the Agent may
  author a new combined state within one fixture-only bounded mandate — User
  direction, 2026-08-20.
- World remains semantically blind and validates only the declared structural
  envelope — accepted Aicadia boundary.
- The first live seam is one real BYO Agent plus MCP against a simulated World — the
  active research ladder and KISS.
- Candidate 01 began with one paid call and no retries — smallest direct-Agent smoke
  and honest initial cost boundary.
- The User explicitly accepted this complete plan, including the one paid
  `gpt-5.6-sol` high-reasoning process call, lack of an enforceable token ceiling
  and zero retries — User authorization, 2026-08-20.
- After candidate 01 failed before inference on unsupported `uniqueItems`, the User
  explicitly authorized exactly one corrected `gpt-5.6-sol` high-reasoning process,
  again with no retry and no enforceable token ceiling. Candidate 01 remains
  unchanged and candidate 02 has an independent persistent guard — User
  reauthorization, 2026-08-20.
- After candidate 02 failed on the lab's empty published mutation schema, the User
  authorized one final candidate only after everything is first proved correct. This
  means a complete mutating raw MCP preflight, exact published input and already
  API-accepted output schemas, prior consumed guards, isolation and verifiable
  cleanup must pass before candidate 03 can launch — User authorization, 2026-08-20.

### Reversible assumptions

- Use `gpt-5.6-sol` at high reasoning through the locally available Codex CLI because
  it is the strongest already exercised host/model seam in this repository. The
  evidence remains explicitly model- and host-specific.
- Four total contributions fit the smallest readable collision while exercising one
  same-fact contradiction and two compatible changes. Participant count is not a
  production proposal.
- Give the lab a generous 120-second continuation deadline and measure actual time
  rather than declaring an unsupported GX threshold. The User judges the transcript
  and observed latency after the run.
- Capture a headless Agent message event as evidence that the Agent authored a
  progress update. This does not prove an interactive host would display it.

### Open questions

- None block plan acceptance. The experiment is designed to produce evidence for
  the remaining product questions: whether the Agent continuation works, whether its
  authority can be bounded structurally, whether its presentation feels like
  gameplay and whether its actual latency is acceptable.

## Implementation map

| Surface | Current state | Intended change | Invariants |
| --- | --- | --- | --- |
| `dev/lab/multiplayer/05-agent-authored-collision/Cargo.toml` and `Cargo.lock` | Absent. | Add one standalone lab crate with only current workspace-compatible dependencies. | Production crates never depend on the lab. |
| `dev/lab/multiplayer/05-agent-authored-collision/src/lib.rs` | Absent. | Add the one canonical in-memory fixture, structural validator, request receipt and Activity model with deterministic tests. | No semantic scoring, random authority or production claim. |
| `dev/lab/multiplayer/05-agent-authored-collision/src/main.rs` | Absent. | Add the loopback streamable-HTTP MCP adapter over the same fixture. | MCP is thin; state semantics remain in `lib.rs`. |
| `dev/lab/multiplayer/05-agent-authored-collision/run` | Two separately guarded candidates are retained; source now publishes and checks the corrected mutation schema. | Preserve both consumed guards and keep future preflight able to reject empty mutation schemas before model spend. | At most one process per explicitly authorized candidate, zero retries, no secrets and fail closed on invalid setup. |
| `dev/lab/multiplayer/05-agent-authored-collision/fixture/` and `schema/` | Absent. | Add the exact Table, source contributions, User prompt and strict final-output schema. | Every model input and expected structural result is inspectable. |
| `dev/lab/multiplayer/05-agent-authored-collision/README.md` | Absent. | Record question, bounds, real/simulated seams, run observations, verdict, limitations and transcript link. | Lab verdict never becomes current game behavior. |
| `dev/lab/multiplayer/README.md` | Lists experiments 01–04 and points at superseded planning. | List experiment 05 and point its question to this accepted plan once built. | Track index owns navigation only. |
| Active Multiplayer concept, Area, register and this plan | Candidate and GX condition recorded; experiment absent. | Record experiment authorization, bounded result and next decision without promoting tool names or behavior. | `game/docs/`, `dev/CONTEXT.md` and production code remain unchanged. |

## Execution contract

Root owns outcome, scope, plan state, the sole model-call authority, integration and
final evidence claim. Do not delegate this first experiment: the lab server, runner,
captured evidence and verdict form one tightly coupled seam. Run one dependency-ready
task at a time. No task changes production runtime, current game docs, canonical
vocabulary or existing retained lab artifacts beyond the Multiplayer track index.

## Task graph

Allowed states are `pending`, `in_progress`, `completed` and `blocked`.

| ID | State | Depends | Parallel-safe | Objective | Owned surfaces | Evidence |
| --- | --- | --- | --- | --- | --- | --- |
| T1 | completed | — | no | Build and prove the deterministic simulated World and lab MCP operations token-free. | Experiment crate, fixture, initial README | Thirteen standalone Rust tests pass, including raw loopback MCP catalog, exact-subject calls and the complete mutating route; all tests use zero model calls. |
| T2 | completed | T1 | no | Build and freeze the fail-closed one-call Agent runner and output contract without invoking a model. | Experiment runner, schema, prompt, README | Public preflight passes with exact `codex-cli 0.148.0`, `gpt-5.6-sol`/high, isolated two-tool MCP config, strict fixed inputs, real loopback MCP, cleanup and `model_calls: 0`; the token-free self-test proves the persistent guard refuses a second launch. |
| T3 | completed | T2 | no | Execute exactly one real Agent/MCP collision and independently read back the result. | Experiment result artifacts only | The one authorized process was launched and consumed with zero retries; the API rejected unsupported `uniqueItems` before inference or tools, so no readback was possible. The manifest retains exit 1, 2,661 ms, no usage event and successful cleanup as an inconclusive candidate. |
| T3C | completed | T3 | no | Execute the separately authorized corrected candidate without modifying candidate 01. | Corrected schema, runner, `result/candidate-02/` only | Candidate 02 consumed one process with zero retry. The Agent read the exact Table, then called `submit_change` with `{}` because the server had published an empty input schema; World rejected it and independent readback proved version 7, no collision and zero Activity. This controller mismatch is inconclusive. |
| T3H | completed | T3C | no | Harden and prove every harness seam that failed or remained assumed before any final model spend. | Lab MCP adapter/tests, runner, candidate-03 preflight artifacts, plan and lab record | Thirteen tests and public preflight traverse read→propose→collision→resolve→readback to version 8 and one Activity; published input and API-proven output schemas, both consumed guards, hashes, cleanup and a positive/negative live-result validator all pass with candidate-03 `model_calls: 0`. |
| T3D | completed | T3H | no | Execute the conditionally authorized final candidate exactly once. | `result/candidate-03/` only | One process, zero retry and cleanup passed. The Agent read, reached collision, narrated it and submitted a creative two-source blue state; World rejected a `subject_id` that the flat schema permitted but the resolve validator prohibited. Independent readback proved version 7, one open collision and zero Activity. |
| T4 | completed | T3D | no | Record the bounded verdict and return the GX transcript to the User. | Experiment README, track index, concept, register, plan | The bounded inconclusive verdict and exact transcript are recorded; checks pass and the User judged visible collision plus Agent-authored grounded synthesis worth retaining as Multiplayer direction while rejecting any implication that this tool contract is accepted. |

## Task details

### T1 — Build the deterministic fixture and MCP seam

**Objective:** A standalone server implements the exact collision fixture and rejects
every out-of-envelope result without semantic inference or partial state.

**Actions:**

1. Add the lab crate, fixed fixture and one canonical state machine.
2. Implement lab-only `read_subject`, tagged `submit_change` and controller-only
   `read_result` operations through one streamable-HTTP MCP server.
3. Test direct acceptance without collision, bounded collision disclosure, accepted
   creative resolution, unknown source, stale version, forbidden subject/key,
   oversized text, changed retry payload, equal retry, timeout fallback and forced
   pre-commit failure.
4. Assert exactly zero or one simulated Activity as appropriate and independently
   read every accepted result.
5. Create the pending experiment README with exact seams and non-claims.

**Invariants:**

- Source prose remains untrusted data and never changes server behavior.
- Validation cannot inspect whether the result is blue, fair, coherent or fun.
- Failed and timed-out resolutions cannot leave partial state, receipt or Activity.
- All reads and candidate sets are hard-bounded; no global revision or shared Place
  state exists in the fixture.

**Evidence:**

- `cargo test --manifest-path dev/lab/multiplayer/05-agent-authored-collision/Cargo.toml`
  — all deterministic fixture and protocol tests pass with zero model calls.
- Raw loopback MCP calls exercise the exact schemas before any Agent invocation.

**Stop conditions:**

- Stop if the server needs semantic content inspection, production code changes,
  another state model or more than one Activity to express the fixture.

### T2 — Freeze the one-call Agent runner

**Objective:** A public preflight can prove the exact runnable setup and exit without
model spend; the live path cannot exceed one call or retry automatically.

**Actions:**

1. Add one fixed User prompt and strict final-output schema.
2. Isolate the Codex home, configuration and tool allowlist; require only the lab MCP
   server and disable unrelated capabilities.
3. Validate CLI, authentication, exact model/effort availability, schemas, fixture
   digest, loopback server, raw MCP calls, result-directory ownership and cleanup.
4. Make preflight write a manifest with `model_calls: 0` and make live execution
   increment it before process launch, refusing a second launch under every outcome.
5. Validate event ordering: read, initial submission, player-visible Agent message,
   collision continuation, accepted result and no other tool calls.

**Invariants:**

- Preflight never invokes the model.
- The runner accepts no public executable, prompt, fixture, model or output-root
  override that would change the evidence claim.
- A timeout, invalid output, failed readback or cleanup failure remains failed or
  inconclusive; it cannot trigger another call.

**Evidence:**

- `dev/lab/multiplayer/05-agent-authored-collision/run preflight` — exits zero,
  reports zero model calls and demonstrates owned cleanup.
- Focused fake-controller tests prove the one-call guard and failure paths without a
  real model process.

**Stop conditions:**

- Stop before live execution when any setup, schema, isolation, event-validation,
  ownership or cleanup gate is not fail-closed.

### T3 — Run one live collision

**Objective:** The pinned Agent either completes the exact narrated creative
collision in one explicit process or produces one honestly bounded failure.

**Actions:**

1. Run the frozen preflight and preserve its manifest.
2. Execute the one authorized Codex process with no retries.
3. Capture structured events, player-facing messages, tool inputs/results, usage and
   elapsed time from collision result to continuation submission.
4. Independently call `read_result`, compare state, source attribution, request
   receipt and one Activity with the accepted tool result, then stop the server and
   verify cleanup.
5. Generate a sanitized chronological transcript for User review.

**Invariants:**

- The Agent receives no out-of-band contribution ids or final answer.
- The final state targets only the Table and permitted keys, differs from every
  individual candidate and cites at least two known sources.
- Intermediate and final player messages contain only in-World language and no ids,
  tools, protocol, database or model narration.
- A successful Agent response cannot substitute for controller readback.

**Evidence:**

- One retained run manifest reports exactly one model process call, zero retries,
  exact tool/event order, measured latency and cleanup state.
- Independent controller readback agrees with accepted current state and exactly one
  simulated Activity.

**Stop conditions:**

- Do not rerun after model, schema, protocol, validation, latency, readback or cleanup
  failure. Record the bounded candidate and return for new User authority.

### T3C — Run the separately authorized correction

**Objective:** Exercise the original Agent/MCP hypothesis once after correcting only
the pre-inference schema incompatibility, without overwriting or retrying candidate
01.

**Actions:**

1. Freeze candidate 02 under `result/candidate-02/` with the corrected schema and a
   fresh manifest containing `model_calls: 0`.
2. Repeat all token-free deterministic, MCP, Codex-config, schema and guard gates.
3. Consume exactly one new `gpt-5.6-sol` high-reasoning process before launch and
   refuse every second candidate-02 launch.
4. Validate event order and independently read the simulated current state and sole
   Activity when the Agent succeeds; otherwise retain the exact bounded failure.

**Invariants:**

- Candidate 01 artifacts and manifest never change.
- Candidate 02 uses the same prompt, fixture, model, reasoning effort, two-tool
  allowlist and real/simulated seam boundary; only the unsupported schema keyword is
  absent.
- Total experiment process calls cannot exceed two and total retries remain zero.

**Evidence:**

- Candidate 02 has its own fixed-input hashes, preflight manifest, event stream,
  output, readback, timings and cleanup status.
- Source tests and repository lint prove the correction did not enter production.

**Stop conditions:**

- Do not launch unless the candidate-02 preflight reports zero model calls.
- Do not rerun candidate 02 for any reason.

### T3H — Prove the final harness before spend

**Objective:** Every previously failed or merely assumed controller seam is
exercised token-free against the exact binary and inputs candidate 03 would use.

**Actions:**

1. Publish one root-object `submit_change` schema containing the exact phase and
   state fields; reject mixed or incomplete phase payloads before the state machine.
2. Make a real loopback protocol test and disposable preflight server execute the
   complete read, proposal, collision, resolution and controller-readback route.
3. Assert the live catalog contains the exact non-empty mutation fields, while the
   Agent allowlist still excludes controller-only `read_result` and all unrelated
   tools.
4. Prove the source final-output schema equals candidate 02's API-accepted schema;
   verify fixture and prompt continuity and both earlier consumed guards.
5. Strengthen event validation to reject unrelated tools or event failures, require
   the intermediate player message at the exact point, validate source receipts and
   compare Agent final output, accepted tool result and independent readback.
6. Make cleanup state truthful: candidate 03 begins at `pending` and is marked passed
   only after its process group, server and isolated temporary directories are
   actually absent.

**Invariants:**

- Candidate 01 and 02 artifacts remain unchanged.
- No model process, response inference or background Agent is invoked in T3H.
- A catalog-name check or read-only call cannot substitute for the complete raw
  mutation path.
- Any failed check leaves candidate 03 at zero calls and blocks T3D.

**Evidence:**

- Lab tests and the public candidate-03 preflight both traverse the complete real
  loopback MCP path to version 8 and exactly one Activity.
- Candidate-03 manifest records current fixture, prompt, output-schema and harness
  hashes, both prior guards, `model_calls: 0` and verified cleanup.

**Stop conditions:**

- Stop before T3D on any schema, protocol, guard, hash, isolation, validation or
  cleanup ambiguity.

### T3D — Run the final candidate once

**Objective:** The exact preflighted Agent/MCP flow gets one final opportunity to
exercise collision narration and Agent-authored settlement.

**Actions:**

1. Rerun static and full protocol checks against a disposable server before
   consuming candidate-03 authority.
2. Start a fresh baseline server, verify its catalog and state without mutation,
   then atomically consume the sole candidate-03 process call.
3. Execute one `gpt-5.6-sol` high process, independently read World result, validate
   exact events and output, verify cleanup and generate the sanitized transcript.

**Invariants:**

- Total retained experiment process calls cannot exceed three; retries remain zero.
- No validation or cleanup failure can trigger another process.
- A model response never substitutes for accepted tool output plus controller
  readback.

**Evidence:**

- Candidate 03 retains its manifest, events, output, accepted tool result,
  controller readback, usage, timing, transcript and cleanup result.

**Stop conditions:**

- Do not rerun candidate 03 for any reason. Any remaining mismatch is the final
  bounded evidence for this plan.

### T4 — Record and judge the result

**Objective:** The experiment has one honest verdict and the User can judge whether
the captured collision feels like Aicadia gameplay.

**Actions:**

1. Record exact observations, actual latency, model/host identity, real and simulated
   seams, falsifiers, verdict and artifact status in the experiment README.
2. Add experiment navigation to the Multiplayer track.
3. Present the sanitized transcript and authoritative readback to the User without
   turning the lab result into current behavior.
4. Record the User's GX judgment and either earn the deterministic scale experiment,
   revise the candidate or reject it.
5. Update the active concept, concept register and plan state to the exact bounded
   result; leave `game/docs/`, public MCP text and code untouched.

**Invariants:**

- `supported` means only that this pinned Agent and real MCP flow completed this
  simulated fixture and the User judged its presentation positively.
- `refuted` means a correctly functioning harness exercised the hypothesis and the
  Agent/tool/GX behavior failed a stated criterion.
- Infrastructure or harness ambiguity is `inconclusive`, never blamed on the model.

**Evidence:**

- `cargo test -p aicadia-studio --lib` — governed records, links, front matter and
  experiment navigation lint cleanly.
- Focused evidence review confirms every claim maps to one real exercised seam.

**Stop conditions:**

- Stop before any deterministic scale, PostgreSQL or production work. That next step
  requires the recorded verdict and a new or revised accepted plan.

## Validation ladder

1. **Focused:** Standalone Rust tests and raw MCP calls prove only the fixture's
   structural state machine and lab protocol.
2. **Contract:** The runner proves one-call bounds, Agent tool isolation, independent
   readback and cleanup; repository lint proves the lab remains outside production.
3. **Outcome:** One pinned Agent continued after collision, narrated it and authored
   an exact grounded new state, but the contradictory tool contract rejected it. The
   User judged the creative-collision experience worth retaining; the system verdict
   remains inconclusive and earns no settlement behavior.
4. **Integrity:** `git diff --check`, focused diff review, experiment crate tests,
   `cargo test -p aicadia-studio --lib`, `cargo brief` and confirmation that unrelated
   User changes, `game/docs/`, production code and existing lab evidence remain intact.

## Change control

Refine exact fixture wording, local paths and stronger token-free evidence in place
while the accepted outcome, per-candidate one-call cost and evidence claim remain
unchanged. Stop
implementation, set `status: draft`, revise and request explicit re-acceptance when
new evidence changes the creative mandate, player presentation, real/simulated seam,
model-call count, external cost, public behavior or claimed outcome.

## Completion conditions

- every task in the graph is completed and deterministic/integrity checks pass; the
  outcome rung records its supported, refuted or inconclusive evidence honestly;
- exactly three retained process candidates exist—the immutable pre-inference
  failure, immutable empty-schema failure and at most one final candidate—and no
  automatic retry occurred;
- the User has reviewed the sanitized transcript and recorded a GX judgment;
- the README verdict names only the actual real and simulated seams;
- current concept and register agree while `game/docs/`, production code, public MCP
  text, canonical vocabulary and backlog remain unchanged;
- no process, listener, temporary Codex home, auth material or unowned result remains;
- `status: complete` and `completed_at` are recorded only after these conditions.
