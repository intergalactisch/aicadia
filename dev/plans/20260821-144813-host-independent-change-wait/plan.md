---
status: complete
created_at: "2026-08-21T14:48:13+02:00"
updated_at: "2026-08-21T15:25:06+02:00"
accepted_at: "2026-08-21T14:58:58+02:00"
completed_at: "2026-08-21T15:25:06+02:00"
---

# Prove one bounded change wait across active clients

> **Role / side:** current host-independent realtime experiment plan / development side.
> **Authority:** owns this lab's bounded outcome, execution order, Agent-run cost
> boundary and evidence claim.
> **Excludes:** accepted game behavior, production realtime architecture and
> delivery evidence; those remain in `game/docs/`, a later production plan and
> `dev/docs/evidence/`.

## Outcome

One simulated World subject starts as a brown Table at version `1`. A terminal
client, a portable browser component and one explicitly invoked BYO Agent all read
that same current state and begin the same bounded wait with version `1`. A guarded
controller changes the Table once to blue version `2`. Each active client receives
only that the subject may be stale, performs an authoritative reread and observes
blue version `2`; the Agent then explains the change to its User.

The terminal and component use one lab-only HTTP operation. The Agent uses one MCP
tool with the same semantic input and output, backed by the exact same waiting
function. A deterministic stage first proves immediate mismatch, change during
registration, timeout, disconnect recovery, coalescing, request bounds and one hot
subject with `10,000` logical waiters. Only after every zero-token and browser
preflight passes may the lab launch one pinned `codex-cli 0.149.0` plus
`gpt-5.6-sol` high-reasoning Agent run with no retry.

The evidence claim is deliberately narrow: one bounded version-aware wait can be a
single host-independent semantic mechanism across a real loopback HTTP client, a
real browser component and one real Codex MCP tool call, without automatic Agent
invocation or durable per-recipient state. The simulated World and local fan-out do
not prove production PostgreSQL correctness, deployed-domain behavior, ChatGPT or
Claude host compatibility, multi-instance delivery or million-connection capacity.

## Non-goals

- No change to `game/docs/`, production `World`, PostgreSQL, migrations, current
  HTTP or MCP surfaces, Agent instructions, authentication or authorization.
- No accepted product name, final public operation name, final subject identity,
  final timing, Place dependency or spatial attention anchor.
- No collision intake, semantic request combination, finalizer, cooldown,
  settlement or multiplayer fairness decision.
- No server-side Agent, automatic model invocation, notification-to-model wake-up,
  background token spend or second Agent process.
- No WebSocket, SSE replay protocol, webhook, broker, outbox, CDC or production
  deployment.
- No claim that the portable component ran inside ChatGPT, Claude or another MCP App
  host. It runs in a real browser as a host simulation only.
- No TLS, proxy, mobile suspension, browser-background survival, cross-region or
  network-partition claim.
- No durable observer registration, delivery receipt, missed-event counter, global
  cursor, per-recipient row or replay backlog.
- No import, copy or promotion of lab code into production.

## Evidence and authorities

| Evidence or authority | Current fact | Consequence for this build |
| --- | --- | --- |
| `AGENTS.md` | World is dumb and strict; subscriptions may announce change but never invoke an Agent, prove understanding or spend tokens. | The lab emits content-free stale hints, requires reread and starts only one explicitly authorized Agent process. |
| `dev/areas/multiplayer/README.md` | The User accepted one host-independent realtime capability for compatible Apps, terminals and browsers, with no required Place or browser owner. | Exercise multiple active-client adapters over one semantic wait; use one Entity fixture without selecting the final attention anchor. |
| `dev/docs/concept/multiplayer-first-principles.md` | The strongest working candidate supplies bounded subjects, known versions and a maximum wait, then rereads current truth. | Test that exact candidate and its registration race rather than another ambient subscription mechanism. |
| `dev/docs/research/realtime-agent-subscription-transports.md` | MCP resource subscription was refuted for pinned Codex, while a client-held wait plus authoritative read remains portable in principle. | Use an ordinary MCP tool call for the Agent and keep transport evidence distinct from Agent presentation. |
| `dev/lab/multiplayer/06-place-resource-subscription/README.md` | Official SDK subscription worked, but `codex-cli 0.149.0` never listed, read or followed the resource. | Do not reuse resource notification as the Agent route; directly test one bounded tool call that remains active until change or deadline. |
| `dev/lab/README.md` | Technical experiments default to standalone Rust; deterministic proof precedes one real Agent smoke and real/simulated seams stay separate. | Build one independent Rust crate, run all token-free and browser gates first and retain a bounded verdict even if the Agent route is refuted. |
| [OpenAI MCP server documentation](https://developers.openai.com/plugins/concepts/mcp-server) | Current OpenAI plugin flow discovers MCP tools, lets the model select them and returns their result to continue the conversation. | A blocking-but-bounded MCP tool is plausible but requires a direct host smoke. |
| [OpenAI MCP App UI documentation](https://developers.openai.com/plugins/build/chatgpt-ui) | An optional component may present an ongoing game and connect to declared domains. | A real browser component is a useful protocol preflight, but cannot be reported as ChatGPT-host evidence. |
| [Anthropic MCP connector documentation](https://platform.claude.com/docs/en/managed-agents/mcp-connector) | Claude exposes remote MCP tools within active Agent sessions; inactive-Agent notification wake-up is not promised. | Keep the operation tool-shaped and provider-neutral; make no Claude host claim without a later direct smoke. |
| Locally installed `codex-cli 0.149.0` | This is the available real BYO host and its ordinary MCP tool route is already established. | Pin the exact version and classify only its bounded wait behavior. |
| User acceptance, 2026-08-21 | One host-independent realtime semantic mechanism is accepted; the bounded wait remains a candidate to test. | The lab may plan that candidate but may not promote it without evidence and a later product choice. |

## Alignment

### Strategic

Aicadia's concurrent activity becomes gameplay only when an actively playing User
can notice it quickly through whichever client they brought. The accepted boundary
removes the browser as a required owner while preserving BYO Agents and the no-token
wake-up rule. This experiment tests the smallest mechanism that could let a
terminal, App component and active Agent participate in that same live experience.

A supported verdict earns a later decision about the actual public transport,
structural attention set and production fan-out. A refuted verdict prevents the
game from depending on long-held Agent tool calls or component behavior that the
tested host cannot sustain. An inconclusive verdict identifies an exact harness,
browser or provider boundary without changing current game behavior.

The next concrete risk after this lab is deployed multi-instance fan-out and host
compatibility on one real ChatGPT or Claude App surface. Neither is silently folded
into this local experiment.

### Tactical

The fixture owns one lab-only subject `lab://entity/table`. Its authoritative state
is `{ version: 1, description: "brown wooden table" }`. The controller can perform
exactly one atomic transition to version `2` with description `"blue wooden
table"`. Every read returns the complete current fixture state.

The candidate wait input contains at most eight exact subject/version pairs and a
lab-only maximum wait of thirty seconds. The result contains only `changed_subjects`
and `timed_out`; it never carries new Entity content. The shared waiting function:

1. validates the subject count, known versions and deadline bound;
2. registers transient notification receivers for the exact subjects;
3. rereads current versions after registration;
4. returns immediately when any known version differs;
5. otherwise waits for a relevant hint or the deadline;
6. rereads current versions before responding; and
7. returns only subjects whose current versions differ from the caller's versions.

Register-then-recheck must close the commit-during-registration race. A disconnected
client stores no server state; its next request with version `1` returns immediately
after the fixture has reached version `2`. A timeout reports no changed subject and
has no World or Activity footprint.

The deterministic stage proves:

- a prior change returns immediately;
- a change after registration wakes the wait;
- a commit forced into the registration boundary is not missed;
- timeout returns cleanly;
- disconnect plus retry with the old version recovers without replay;
- multiple rapid hints collapse to current-version comparison;
- nine subjects and an excessive deadline fail before registration; and
- `10,000` logical waiters on the same subject all recover after one transition,
  while the fixture performs one state change and stores no recipient records.

The direct smoke starts a fresh server and three clients. The terminal client and
browser component read version `1` and wait. The Codex Agent receives a natural
Dutch prompt to remain briefly with the Table and report the first change; the
prompt does not reveal the future colour or version. Its MCP server exposes only
lab `get_state` and `wait_for_change` tools. The controller changes the fixture only
after observations prove that terminal, component and MCP tool waits are all active.

Success requires all three paths to return the same changed subject, all three to
reread blue version `2`, and the Agent to report the new fact without another User
turn. A terminal or component poll that bypasses the wait does not count. The live
verdict is `supported`, `refuted` or `inconclusive` against this exact route.

### Technical

Create one standalone Rust 2024 crate at
`dev/lab/multiplayer/07-host-independent-change-wait/`, outside the production
workspace. One canonical in-memory fixture and one shared wait function back:

- a bounded JSON HTTP read and wait adapter for terminal and browser clients;
- a thin MCP `get_state` and `wait_for_change` adapter for the Agent; and
- a secret loopback-only controller route for the one transition.

Use asynchronous per-subject latest-version notification rather than an event queue.
Transient waiter registration may be process-local because the lab's correctness
source is the simulated in-memory fixture; the result cannot be promoted as a
production multi-instance design. Every wait performs the authoritative version
check after registration, so notification loss or coalescing cannot change the lab
result.

The portable component is one small HTML/JavaScript file served by the lab. It reads
the fixture, starts the HTTP wait, shows `waiting`, rereads after the hint and shows
blue version `2`. It contains no independent state machine or host-specific API. A
real browser execution proves browser HTTP behavior only; ChatGPT and Claude host
CSP, lifecycle and embedding remain simulated seams.

The terminal adapter is a small Rust subcommand using the same HTTP contract. The
MCP adapter uses the repository's pinned official Rust SDK. Server observations
record adapter, operation, subject, known version, registration, response and
timing, but no secrets or model reasoning.

The live runner pins `codex-cli 0.149.0` and `gpt-5.6-sol` with high reasoning,
creates an isolated temporary configuration containing only the lab MCP server and
permits one `codex exec` process. The process has a 120-second outer deadline and
zero retries. Codex may use multiple inference rounds inside that one run, and its
token count cannot be bounded exactly by the CLI. An atomic persistent guard is
consumed before launch; any later provider, browser, process or assertion failure
retains an `inconclusive` result and cannot authorize a retry.

The real seams are the Rust waiting function, local scheduling and race injection,
loopback HTTP, one real browser component, the terminal client, the official MCP
tool route, `codex-cli 0.149.0`, `gpt-5.6-sol` high, captured requests, Agent output
and wall-clock timings. The simulated seams are World, Entity and Activity
persistence, authentication, authorization, deployed HTTPS, proxy behavior,
ChatGPT and Claude embedding, multi-instance propagation, internal broker,
cross-region recovery and million-connection capacity. The `10,000`-waiter case is
a bounded algorithm pressure test only.

## Decisions, assumptions and open questions

### Confirmed decisions

- One host-independent realtime semantic capability serves compatible Apps,
  terminal clients and browsers; delivery never invokes an Agent — User acceptance,
  2026-08-21, recorded in the active Multiplayer concept and Area.
- The next work is a small experiment of the bounded wait candidate, not production
  implementation — User acceptance, 2026-08-21.
- Current truth is recovered through bounded authorized reread; no hint, connection
  or recipient state is World truth — accepted Multiplayer direction and
  `AGENTS.md`.
- A real ChatGPT or Claude App claim is excluded until that exact host is exercised
  directly — lab evidence boundary.
- The User accepted this complete plan, including one `gpt-5.6-sol` high Agent
  process, zero retries, a 120-second outer deadline, possible multiple internal
  inference rounds and no enforceable exact token ceiling — User acceptance,
  2026-08-21.

### Reversible assumptions

- Use one Entity-like Table fixture and version integers because they expose the
  transport and race question without choosing Place or spatial context. The names,
  URI and representation remain lab-only.
- Use at most eight subjects and a thirty-second maximum wait. Deterministic cases
  keep shorter deadlines where useful. The original five-second assumption proved
  too small for browser-tool overhead before any Agent execution; returning on the
  first hint keeps the normal path fast while the larger ceiling covers a real
  BYO-Agent turn.
- Use `10,000` logical waiters as a deliberately hot local pressure case. It can
  falsify an obviously unbounded algorithm but cannot establish production scale.
- Use a browser-served portable component as a simulation of an embedded App host.
  Its success proves only the direct browser adapter.
- Use `gpt-5.6-sol` high to reduce model-comprehension ambiguity while testing the
  real Codex host. The provider and model do not become production requirements.

### Open questions

- Whether one pinned Codex Agent can keep the bounded MCP tool call active, receive
  its result, reread current state and ground its answer is the lab's falsifiable
  evidence question, not a design choice blocking execution.
- No material design question blocks execution. The Codex bounded-wait behavior is
  the experiment's evidence question.

## Implementation map

| Surface | Current state | Intended change | Invariants |
| --- | --- | --- | --- |
| `dev/lab/multiplayer/07-host-independent-change-wait/Cargo.toml` and `Cargo.lock` | Absent. | Add one standalone Rust lab crate with HTTP, MCP and async test dependencies. | Production workspace and lockfile remain untouched. |
| `dev/lab/multiplayer/07-host-independent-change-wait/src/lib.rs` | Absent. | Add the single fixture, bounded wait function, race hooks and observation capture. | One canonical state; no semantic inference, recipient history or replay. |
| `dev/lab/multiplayer/07-host-independent-change-wait/src/main.rs` | Absent. | Add loopback HTTP, MCP and guarded controller adapters over the shared function. | Adapters contain no duplicated decision logic; controller is not Agent-accessible. |
| `dev/lab/multiplayer/07-host-independent-change-wait/src/bin/client.rs` | Absent. | Add the inspectable terminal read-wait-reread client. | It never polls or trusts hint contents. |
| `dev/lab/multiplayer/07-host-independent-change-wait/component/index.html` | Absent. | Add one portable component that renders waiting and authoritative reread state. | Browser presentation only; no second state machine or host claim. |
| `dev/lab/multiplayer/07-host-independent-change-wait/fixture/` | Absent. | Add exact initial state, server instructions and natural Agent prompt. | Future colour/version are absent from the User prompt. |
| `dev/lab/multiplayer/07-host-independent-change-wait/run` | Absent. | Add deterministic preflight and guarded live orchestration with deadlines and cleanup. | Live mode is impossible before every token-free, terminal and browser gate passes; no retry. |
| `dev/lab/multiplayer/07-host-independent-change-wait/result/` | Absent. | Retain bounded manifest, observations, client readbacks and sanitized Agent output. | No secret, temporary configuration or raw environment is retained. |
| `dev/lab/multiplayer/07-host-independent-change-wait/README.md` | Absent. | Record question, method, observations, real/simulated seams, verdict and next risk. | Lab result never becomes current game or production behavior. |
| `dev/lab/multiplayer/README.md` | Lists experiments 01–06. | Add experiment 07 after it exists. | Index owns navigation only. |
| Active Multiplayer concept, Area, concept log and this plan | Accepted boundary and candidate experiment are recorded. | Record only the bounded verdict and resulting next decision. | `game/docs/`, `dev/CONTEXT.md` and production code remain unchanged. |

## Execution contract

Root owns outcome, scope, plan state, the one Agent-run authority, integration and
the final evidence claim. Do not delegate this experiment: fixture, race-free wait,
HTTP/MCP adapters, browser orchestration and live guard form one tightly coupled
seam. Execute one dependency-ready task at a time. No task changes production
runtime, current game docs, canonical vocabulary or earlier lab artifacts.

## Task graph

Allowed states are `pending`, `in_progress`, `completed` and `blocked`.

| ID | State | Depends | Parallel-safe | Objective | Owned surfaces | Evidence |
| --- | --- | --- | --- | --- | --- | --- |
| T1 | completed | — | no | Build and prove the bounded version-aware wait, HTTP/MCP parity and hot-subject behavior without model spend. | Lab crate, terminal client, component, fixture, token-free runner and initial README | Eleven standalone Rust tests, loopback HTTP and official MCP preflight, 10,000 hot waiters and real-browser component preflight passed with zero Agent processes. |
| T2 | completed | T1 | no | Execute one guarded simultaneous terminal, component and real Codex Agent smoke. | Live runner, one-run guard and bounded `result/` evidence | Exactly one Codex process and zero retries ran for 121 seconds. The Agent made 583 unsuccessful reads against an undiscoverable exact subject, never entered the wait, World did not transition and cleanup passed; the route is inconclusive. |
| T3 | completed | T2 | no | Classify the bounded verdict and align experiment navigation and current exploration. | Experiment README, lab index, active concept, Area, log and plan | Inconclusive pinned-Agent verdict reconciled with raw observations; 11 lab tests, 73 Studio tests, builder brief and whitespace checks pass without a game contract change. |

## Task details

### T1 — Prove the mechanism token-free

**Objective:** The exact shared waiting function is race-safe for the lab fixture,
bounded under one hot subject and usable through terminal, browser and MCP adapters
without starting a model.

**Actions:**

1. Implement the single-versioned fixture, exact transition, bounded waiting
   function, forced registration-race hook and observations.
2. Add thin loopback HTTP, MCP and controller adapters over that function.
3. Add terminal and portable component clients that always reread after a hint.
4. Prove prior-change, after-registration, registration-race, timeout, disconnect,
   coalescing, invalid-bounds and `10,000`-waiter cases.
5. Run terminal and real-browser preflights and retain bounded zero-token evidence.

**Invariants:**

- T1 cannot launch Codex or any model.
- The hint never contains or replaces authoritative Entity state.
- No correctness property depends on notification delivery or a recipient record.
- Production crates, schema, public contracts and current behavior remain untouched.

**Evidence:**

- `cargo test --manifest-path dev/lab/multiplayer/07-host-independent-change-wait/Cargo.toml`
  — all boundedness, race, recovery, adapter and cleanup tests pass.
- `dev/lab/multiplayer/07-host-independent-change-wait/run preflight` — terminal
  and official MCP clients observe one hint then authoritative blue version `2`,
  with zero Agent processes.
- Real browser inspection of the lab component — the DOM progresses from brown
  version `1` through waiting to blue version `2` using the HTTP wait and reread;
  this is labelled browser-only evidence.

**Stop conditions:**

- Stop before T2 if any race can miss the transition, a client must poll, request
  bounds are bypassed, the browser path cannot be independently observed, cleanup
  is incomplete or the live guard/result already exists.

### T2 — Observe one real active Agent

**Objective:** One pinned Codex Agent uses the bounded MCP wait while terminal and
component waits are simultaneously active, then grounds its answer in a reread.

**Actions:**

1. Re-run every T1 gate, verify exact Codex version, isolated configuration and
   model availability, then atomically consume the one-run guard.
2. Start fresh server, terminal wait, browser component and one natural User-owned
   Codex Agent task.
3. Let the controller perform exactly one transition only after all three wait
   registrations are captured.
4. Enforce the outer deadline, never retry, collect independent authoritative
   readback and verify owned-process cleanup.
5. Sanitize and retain the manifest, inputs, observations, browser state, terminal
   output, Codex events, final answer, timing and controller readback.

**Invariants:**

- One `codex exec` process is the total Agent budget; internal inference rounds may
  vary and no failed run can be retried.
- The Agent receives no future colour/version in the prompt and cannot use a shell,
  web or unrelated MCP server.
- Terminal and browser clients do not depend on or communicate with the Agent.
- Independent server observations, not the Agent sentence, prove tool and read use.

**Evidence:**

- Retained `result/live/manifest.json` and observations — exact host/model version,
  three wait registrations, one controller transition, three dirty responses,
  three authoritative rereads, deadline and cleanup.
- Retained terminal, browser and Agent outputs — each reports blue version `2` only
  after its own reread; Agent output is grounded by captured MCP calls.
- Independent controller readback — fixture ended at blue version `2`; this proves
  fixture state only.

**Stop conditions:**

- A deterministic or browser failure before launch stops without consuming the
  guard. Any failure after process launch consumes the sole authority, is retained
  and produces `inconclusive` or `refuted` without retry.

### T3 — Record the bounded verdict

**Objective:** The lab has one inspectable verdict whose claim is no broader than
the exact simulated and real seams exercised.

**Actions:**

1. Reconcile fixture, HTTP, browser, MCP host, Agent, timing and cleanup evidence.
2. Classify `supported`, `refuted` or `inconclusive` using the predeclared criteria.
3. Record the verdict, real/simulated seams and next concrete risk in the lab README.
4. Align lab navigation, active Multiplayer exploration, Area, concept log and plan
   without changing current game behavior.

**Invariants:**

- A successful browser component is never called ChatGPT or Claude App evidence.
- A successful Codex answer never proves production scale or transport correctness
  beyond captured calls.
- No lab code or candidate name enters production or `game/docs/`.

**Evidence:**

- `cargo test -p aicadia-studio --lib` — governed documentation and navigation
  remain structurally valid.
- `cargo brief` — the completed lab and current decision trail are discoverable and
  documentation lint is clean.
- `git diff --check` plus focused diff review — no whitespace defect, accidental
  production change or unrelated User edit.

**Stop conditions:**

- Stop and return the plan to `draft` if the result would change current game
  behavior, public protocol, vocabulary, production architecture or authorize
  another Agent run.

## Validation ladder

1. **Focused:** standalone Rust tests for race, timeout, reconnect, bounds,
   coalescing and `10,000` waiters; terminal, MCP and browser preflights.
2. **Contract:** one shared waiting function produces matching HTTP and MCP semantic
   results; all clients reread; no recipient history or automatic model call exists.
3. **Outcome:** one guarded live run shows terminal, browser component and pinned
   Codex Agent simultaneously notice one change and independently reread it, or
   yields a mechanically bounded refuted/inconclusive verdict.
4. **Integrity:** `cargo test -p aicadia-studio --lib`, `cargo brief`,
   `git diff --check`, focused diff review and confirmation that production and
   unrelated User changes remain intact.

## Change control

Refine lab paths, task order, fixture mechanics and stronger evidence in place while
the accepted outcome and claim remain unchanged. Stop implementation, keep or set
`status: draft`, revise and request explicit re-acceptance when new evidence changes
the outcome, public behavior, product/domain meaning, non-goals, real/simulated seam
boundary, Agent count, token/cost boundary or evidence claim.

## Completion conditions

- every required task is `completed` and the validation ladder passes;
- exactly zero or one Agent process was launched according to the accepted guard,
  with no retry;
- the exact supported, refuted or inconclusive verdict is demonstrated;
- current exploration, lab navigation and decision history agree;
- production code, `game/docs/` and canonical vocabulary remain unchanged;
- no known-stale authority, material open question or accidental unrelated change
  remains; and
- `status: complete` and `completed_at` are recorded only after these conditions.
