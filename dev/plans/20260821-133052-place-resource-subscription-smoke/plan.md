---
status: complete
created_at: "2026-08-21T13:30:52+02:00"
updated_at: "2026-08-21T14:12:34+02:00"
accepted_at: "2026-08-21T13:49:08+02:00"
completed_at: "2026-08-21T14:12:34+02:00"
---

# Determine whether one real BYO host can follow a Place resource

> **Role / side:** current Place-resource subscription experiment plan / development side.
> **Authority:** owns this lab's bounded outcome, execution order, Agent-run cost
> boundary and evidence claim.
> **Excludes:** accepted game behavior, production Multiplayer architecture and
> delivery evidence; those remain in `game/docs/`, a later production plan and
> `dev/docs/evidence/`.

## Outcome

One explicitly invoked User-owned Agent enters one simulated Place through a real
Codex MCP host. The host reads the exact current-Place resource while it contains a
brown Table. A controller changes that resource once so the Table is blue. Without
polling or another User invocation, the host must have opened an exact MCP resource
subscription, receive the content-free change notification, reread the authoritative
Place resource and make the blue Table available to the still-active Agent. The
Agent then reports the changed fact in one exact, inspectable final line.

The experiment first proves the same server flow token-free with the official Rust
MCP client, including reread after reconnect. Only after that gate passes may one
real pinned Codex Agent run be launched with no retry.

The Place is only a concrete fixture. The User corrected that current Place is not
always present and cannot be Aicadia's universal attention anchor. The highest-value
technical edge remains whether the pinned host can follow any exact MCP resource at
all, because current public Codex documentation does not claim resource subscription
or notification presentation. The final evidence must determine whether this exact
pinned host path is supported, refuted or inconclusive. It may not claim generic
BYO-host support, select the eventual structurally grounded resource or establish
production readiness.

## Non-goals

- No change to `game/docs/`, production World behavior, PostgreSQL, migrations,
  HTTP, the published MCP catalog, Agent instructions or current action semantics.
- No server-side Agent, automatic model invocation, background token spend or new
  User turn in response to the notification.
- No multi-Agent collision, request settlement, finalizer, cooldown or semantic
  combination.
- No real Character movement, Place visibility, authorization, privacy, durable
  World history, multi-instance fan-out, broker or slow-consumer design.
- No polling fallback, raw WebSocket, webhook or alternate public transport.
- No claim about ChatGPT, Claude, other Codex versions, other models, disconnected
  model turns, interactive player UI, concurrent Users or million-connection scale.
- No reuse of the retained experiment 05 runner, consumed guards or lab code. Only
  its operational lessons about preflight, isolation, evidence and no retry apply.

## Evidence and authorities

| Evidence or authority | Current fact | Consequence for this build |
| --- | --- | --- |
| `AGENTS.md` | Subscriptions may announce durable change but never invoke an Agent, prove understanding or spend tokens. | The update can reach only the already active host/Agent run and carries no authority. |
| `dev/areas/multiplayer/README.md` | Realtime attention should be default during active play, but Place is optional and cannot be the universal anchor; authoritative reread remains the correctness path. | Use one Place fixture to test exact-resource mechanics only and require a post-notification reread rather than trusting a payload. |
| `dev/docs/concept/multiplayer-first-principles.md` | Exact-resource attention is chosen as default GX, but actual pinned-host support remains unproved. | Test the host seam before designing production resources or fan-out. |
| `dev/docs/research/mcp-subscriptions-and-collective-agent-intents.md` | MCP `2026-07-28` supplies exact-URI listen, content-free update hints and read-after-hint; reconnect has no replay. | The deterministic client must listen before read, treat a hint as stale and reread after reconnect. |
| `dev/docs/research/realtime-agent-subscription-transports.md` | Subscription, host refresh and Agent processing are distinct evidence layers. | Capture server, client, host and Agent observations separately. |
| [Official OpenAI MCP documentation](https://learn.chatgpt.com/docs/extend/mcp) | Codex `0.149.0` documentation lists stdio, Streamable HTTP and server instructions, but does not promise MCP resource subscription or notification presentation. | A direct pinned-host smoke is required; protocol support may not be attributed to Codex without observation. |
| Locally installed `codex-cli 0.149.0` | This is the real available host version. | Pin and record this exact version; do not generalize the result. |
| Workspace `Cargo.toml` and local `rmcp 3.1.1` source | The current official Rust SDK has server and client support, including `listen()`. | Use one standalone Rust crate with the same pinned SDK for deterministic preflight. |
| `dev/lab/README.md` | A direct Agent smoke follows deterministic zero-token proof, has one falsifiable question and records real/simulated seams. | Build the protocol gate first and retain a bounded verdict even when it refutes the candidate. |
| `dev/lab/multiplayer/05-agent-authored-collision/` | Earlier live runs became inconclusive through controller/schema mismatches and every candidate guard was consumed. | Use no strict model-output schema, prove the complete raw protocol route first and create a new one-run guard. |
| User direction, 2026-08-21 | Realtime attention should be default GX, but current Place is not always present; the User authorized planning for at most one real Agent run with no retry. | Keep the lab's Place as a fixture only, avoid selecting a final anchor and ask explicit acceptance of the guarded run's exact cost limitation before execution. |

## Alignment

### Strategic

Aicadia's shared World feels multiplayer only if concurrent activity can become
noticeable while a player is present. Default exact-resource attention could provide
that awareness without making World smart or starting Agents in the background.
This lab retires only the transport feasibility risk: whether one real BYO host can
carry the standard MCP notification through to an already active Agent. Its Place
fixture does not decide which structural representation later provides attention
when a Character has no Place.

A supported verdict earns later design of exact resource contents, movement and
bounded interest. A refuted verdict prevents Aicadia from quietly depending on a
host behavior that the tested Codex version does not provide. An inconclusive
verdict identifies a concrete harness or provider failure without promoting the
mechanism.

The next risk after this lab is the product contract for what a host retains or
shows between explicit Agent turns. It is deliberately not designed here.

### Tactical

The fixture has one simulated Place named `Workshop`, revision `1`, containing one
simulated Entity named `Table` whose `colour` is `brown`. The server exposes exactly
one lab-only resource URI, `lab://place/current`. Its representation is bounded JSON
containing the Place revision and complete Entity facts needed for the observation.

The token-free stage executes two deterministic cases against a fresh fixture:

1. The official Rust MCP client opens `subscriptions/listen` for the exact URI,
   waits for acknowledgment, reads brown revision `1`, the controller changes the
   Table once to blue revision `2`, the server emits one update hint, and the client
   rereads blue revision `2`.
2. The client reads brown revision `1`, disconnects, the controller performs the
   same single change while no listener exists, and a new client listens then reads
   blue revision `2`. It must recover current truth without replay or a missed-event
   count.

After both cases and all isolation gates pass, the live stage starts a fresh fixture
and one isolated Codex process. The MCP server initialization instructions say that
this lab's exact resource is active-play context and that a changed resource must be
reread. The natural Dutch User prompt asks the Agent to remain in the
Workshop briefly and report the first environmental change; it does not mention a
subscription mode, protocol operation or polling loop.

The server exposes no Agent tools. A loopback-only controller endpoint, protected by
a runner-generated secret and excluded from MCP, performs the one brown-to-blue
change. The runner waits up to thirty seconds for the first Place read. After that
read it allows a short fixed interval for a listen request, performs the change once
regardless of whether listen was observed, and never mutates again. The complete
process has a 120-second external deadline.

The expected final line is plain text rather than a process-wide output schema:

```text
UPDATED_PLACE: Table colour is blue; revision is 2.
```

Success requires the server log to show initialization, exact listen acceptance,
the initial resource read, successful notification write and a distinct
post-notification resource read, followed by the exact Agent line. A polled reread
without an accepted listen does not count.

The live verdict is:

- `supported` when every required host and Agent observation occurs;
- `refuted` when token-free preflight passes and Codex connects successfully but the
  complete exact listen→notification→reread→Agent path does not occur before the
  deadline; or
- `inconclusive` only when the deterministic controller, environment, provider or
  process fails in a way that prevents observing the configured host capability.

Independent controller readback after the Agent exits must show blue revision `2`
in every live verdict where the mutation occurred. It proves fixture state only, not
that the host or Agent processed the change.

### Technical

Create one standalone Rust 2024 crate at
`dev/lab/multiplayer/06-place-resource-subscription/`. It pins `rmcp 3.1.1` with the
required server, client and Streamable HTTP features and remains outside the
production workspace. One canonical in-memory fixture owns current resource state,
revision, listener notification and bounded JSONL observations. A thin loopback
Streamable HTTP server exposes resource list/read/listen only. A separate guarded
controller route can advance the fixture exactly once.

A deterministic `rmcp` client drives both token-free cases. It verifies the exact
accepted filter, notification URI and authoritative resource body rather than
asserting only that an SSE frame existed. Tests use owned random loopback ports and
must clean up every listener and server process.

The live runner creates a task-specific temporary Codex configuration containing
only the lab MCP server, disables unrelated MCP servers, web and shell access, pins
the observed `codex-cli 0.149.0` host and runs one `gpt-5.6-sol` high-reasoning Agent
task. The strong already exercised model reduces model-comprehension ambiguity; the
evidence remains explicitly host- and model-specific. The runner records the prompt,
server instructions, version manifest, JSONL server observations, Codex JSON event
stream, stderr, timing, controller readback and sanitized final transcript.

“One Agent run” means one explicit `codex exec` process and zero automatic retries.
Codex may perform multiple internal inference round trips while executing that one
Agent task, and the CLI exposes no enforceable per-run token ceiling. The runner
therefore bounds launch count and wall time, not exact token spend. An atomic
persistent guard is created before process launch; any existing guard or result
causes a fail-closed refusal. A provider or process failure after launch consumes
the sole authority and is retained as `inconclusive`, never retried.

No secret, controller token, temporary configuration directory or raw environment
is retained. The result stores only sanitized, bounded evidence. The runner verifies
owned-process cleanup and refuses execution if token-free tests, raw protocol
preflight, exact Codex version, model availability, isolated configuration or result
directory checks fail.

The real seams are `rmcp 3.1.1`, the lab resource server, exact-resource listen and
notification over Streamable HTTP, the deterministic `rmcp` client, Codex CLI
`0.149.0`, the selected model, the already-active Agent task, captured host requests,
Agent output and local wall-clock observations. The simulated seams are World,
Character, Place and Entity persistence, authorization, movement, concurrent Users,
multi-instance fan-out, delivery broker, UI, reconnecting Codex model turns,
production failure recovery and million-connection capacity.

## Decisions, assumptions and open questions

### Confirmed decisions

- A compatible host should provide realtime attention by default during active play,
  while Place remains optional and the final structural anchor is unchosen;
  subscription stays non-authoritative and never invokes an Agent — User correction
  recorded in the Multiplayer Area and active concept.
- The smallest next experiment uses one Place, one change and one real BYO host —
  User acceptance, 2026-08-21.
- Deterministic zero-token proof must precede the sole real Agent run — User
  acceptance and Aicadia lab contract.
- The live experiment may launch at most one explicitly invoked Agent run and may
  never retry it — User acceptance, 2026-08-21.
- A refuted or inconclusive result is a valid completed lab outcome when classified
  against the exact evidence boundary — lab constitution and Terry.
- The User accepted this complete corrected plan, including one
  `gpt-5.6-sol` high-reasoning Codex Agent run, zero retries, a 120-second outer
  deadline, possible multiple internal inference rounds and no enforceable token
  ceiling — User acceptance, 2026-08-21.

### Reversible assumptions

- Use a `120`-second process deadline and a `30`-second initial-read deadline. These
  bound an orphaned lab process without proposing production GX timing; observed
  latencies are recorded for later design.
- Use `gpt-5.6-sol` with high reasoning because it is the strongest locally
  available, already exercised host/model seam and reduces comprehension ambiguity.
  The model choice does not become an allowlist or production requirement.
- Use one exact lab URI and one bounded JSON resource representation. Neither the URI
  nor the fixture schema becomes canonical Aicadia vocabulary or public contract.
- Require a natural player prompt and one exact final text line, not a strict output
  schema. This avoids repeating the prior controller mismatch while keeping the
  observation mechanically checkable.

### Open questions

- Whether Codex opens, uses and presents an MCP resource subscription is the lab's
  evidence question, not a choice blocking execution.

## Implementation map

| Surface | Current state | Intended change | Invariants |
| --- | --- | --- | --- |
| `dev/lab/multiplayer/06-place-resource-subscription/Cargo.toml` and `Cargo.lock` | Absent. | Add one standalone Rust lab crate with pinned current MCP client/server dependencies. | Production workspace and lockfile remain untouched. |
| `dev/lab/multiplayer/06-place-resource-subscription/src/lib.rs` | Absent. | Add the one canonical bounded fixture, exact resource representation, one-way transition and observation log. | No semantic inference, production domain dependency or second state copy. |
| `dev/lab/multiplayer/06-place-resource-subscription/src/main.rs` | Absent. | Add the loopback Streamable HTTP resource server and secret controller route. | Agent surface has no tools; controller is lab-only and inaccessible through MCP. |
| `dev/lab/multiplayer/06-place-resource-subscription/src/client.rs` and tests | Absent. | Add official-SDK listen/read/update/reconnect preflight over the same server. | Every test is token-free, bounded and independently asserts current state. |
| `dev/lab/multiplayer/06-place-resource-subscription/run` | Absent. | Add token-free and live modes, isolation gates, atomic one-run guard, deadlines, cleanup and bounded capture. | Live mode cannot launch before every deterministic gate passes and can never retry. |
| `dev/lab/multiplayer/06-place-resource-subscription/fixture/` | Absent. | Add exact Place JSON, server instructions, natural User prompt and expected final line. | Inputs are inspectable, English World content remains English and no product vocabulary is introduced. |
| `dev/lab/multiplayer/06-place-resource-subscription/result/` | Absent. | Retain one bounded live manifest, observations, sanitized output, authoritative readback and transcript after execution. | No secrets, temporary host configuration or environment values. |
| `dev/lab/multiplayer/06-place-resource-subscription/README.md` | Absent. | Record question, method, real/simulated seams, observation, verdict and downstream implication. | Lab result never becomes production behavior or general host support. |
| `dev/lab/multiplayer/README.md` | Lists experiments 01–05. | Add experiment 06 and its exact question after the lab exists. | Track index owns navigation only. |
| Active Multiplayer concept, Area, concept log and this plan | Default GX and unproved host support are recorded. | Record only the bounded experiment verdict and resulting next design question. | `game/docs/`, `dev/CONTEXT.md` and production code remain unchanged. |

## Execution contract

Root owns outcome, scope, plan state, the sole Agent-run authority, integration and
the final evidence claim. Do not delegate this experiment: the resource server,
deterministic client, guarded live runner and verdict are one tightly coupled seam.
Run one dependency-ready task at a time. No task changes production runtime,
current game docs, canonical vocabulary or retained experiment 05 artifacts.

## Task graph

Allowed states are `pending`, `in_progress`, `completed` and `blocked`.

| ID | State | Depends | Parallel-safe | Objective | Owned surfaces | Evidence |
| --- | --- | --- | --- | --- | --- | --- |
| T1 | completed | — | no | Build and prove exact Place listen, notification, reread and reconnect recovery without model spend. | Experiment crate, fixture, token-free runner, initial README | Four standalone Rust tests and the retained raw loopback preflight pass with exact observations and zero Agent processes. |
| T2 | completed | T1 | no | Execute the single guarded real Codex Agent run and capture all independent observations. | Live runner, one-run guard, bounded `result/` evidence | One process, zero retries, 11-second normal exit, exact host observations, Agent output, unchanged controller readback and successful cleanup are retained. |
| T3 | completed | T2 | no | Classify the bounded verdict and align experiment navigation and current exploration. | Experiment README, Multiplayer lab index, active concept, Area, log and plan | The refuted verdict reconciles mechanically with the retained server, host, Agent and controller evidence; Studio's 73 tests, standalone lab tests, builder brief and whitespace checks pass without a current game contract change. |

## Task details

### T1 — Prove the protocol path token-free

**Objective:** One official Rust MCP client deterministically observes the exact
connected update and recovers current state after reconnect against the same lab
server.

**Actions:**

1. Implement the single in-memory Place resource, one brown-to-blue transition,
   exact subscription notification and bounded observation capture.
2. Add the thin Streamable HTTP resource server and secret, loopback-only controller.
3. Add fresh-fixture tests for connected notification+reread and disconnected
   change+reconnect+reread.
4. Add raw protocol preflight that independently verifies advertised capabilities,
   exact resource contents, accepted subscription filter and cleanup.

**Invariants:**

- Zero Codex or model processes can launch from T1.
- Notification content never replaces the authoritative read.
- Reconnect uses no replay, delivery receipt or missed-event counter.
- Production crates, schemas, docs and current behavior remain untouched.

**Evidence:**

- `cargo test --manifest-path dev/lab/multiplayer/06-place-resource-subscription/Cargo.toml`
  — all fixture, client, transport, reconnect and cleanup tests pass.
- `dev/lab/multiplayer/06-place-resource-subscription/run preflight` — emits a
  bounded success manifest and proves that no Agent guard or result was consumed.

**Stop conditions:**

- Stop before T2 if the pinned SDK cannot express the exact current protocol, any
  resource/read/listen assertion fails, cleanup is incomplete, the live guard is
  already present or the required host/model is unavailable.

### T2 — Observe one real Codex host

**Objective:** Consume at most one explicitly authorized Agent run and capture
whether Codex performs the complete exact subscription path.

**Actions:**

1. Re-run every T1 gate, verify the exact Codex version and isolated configuration,
   then atomically create the persistent one-run guard before launch.
2. Start the fresh lab server, isolated Codex host and natural User prompt; perform
   the single controller change after the initial-read condition.
3. Enforce the 120-second outer deadline, never retry, independently read current
   state and verify cleanup.
4. Sanitize and retain the version manifest, prompt, instructions, server
   observations, Codex event stream, stderr, timing, final output and controller
   readback.

**Invariants:**

- One explicit Codex Agent task is the absolute lifetime maximum for this lab.
- No background process, automatic retry or second User/model task is permitted.
- A model-produced blue answer without accepted listen and post-notification reread
  is not subscription evidence.
- A server notification is not evidence that the Agent perceived it.

**Evidence:**

- `dev/lab/multiplayer/06-place-resource-subscription/run live` — either records one
  complete guarded run or refuses before launch with zero spend.
- Independent result validator — reconciles exact server observations, final Agent
  line, current fixture readback and owned-process cleanup without model judgment.

**Stop conditions:**

- If any preflight fails, do not create the guard or launch Codex.
- After process launch, retain every result including provider failure or timeout as
  the sole run; never repair and rerun without a newly drafted and explicitly
  accepted plan.

### T3 — Record the bounded verdict

**Objective:** Leave one inspectable supported, refuted or inconclusive verdict that
answers only the real-host question exercised.

**Actions:**

1. Classify the live result exactly from the accepted criteria and link every
   observation to bounded raw evidence.
2. Record real and simulated seams, latency, limitations and the precise next design
   implication in the experiment README.
3. Add experiment 06 to the Multiplayer lab index and align the active concept,
   Multiplayer Area, concept log and this plan without promoting lab behavior.

**Invariants:**

- Deterministic server evidence, host transport evidence and Agent presentation
  evidence remain separate claims.
- Refuted and inconclusive results remain first-class retained evidence.
- No lab URI, fixture schema, prompt wording or model name becomes production
  vocabulary, allowlist or contract.

**Evidence:**

- Verdict validator — the selected verdict matches all required and forbidden
  observations.
- `cargo test -p aicadia-studio --lib` — development documentation remains
  renderable and indexed.
- `git diff --check` — changed artifacts have no whitespace errors.

**Stop conditions:**

- Stop and return the plan to `draft` if interpreting the result would require a new
  product behavior, generic host claim, public contract or second Agent run.

## Validation ladder

1. **Focused:** standalone lab tests prove the exact listen/read/update/reconnect
   fixture, controller bounds, observation capture and cleanup without tokens.
2. **Contract:** raw preflight proves the server advertises and performs the exact
   current MCP resource flow before a Codex process can launch; the one-run guard
   proves no retry.
3. **Outcome:** one retained real run yields a mechanically supported, refuted or
   inconclusive verdict for Codex CLI `0.149.0` plus the pinned selected model, with
   separate host, Agent and authoritative-readback observations.
4. **Integrity:** `git diff --check`, `cargo test -p aicadia-studio --lib`, focused
   diff review and confirmation that unrelated User changes, `game/docs/`,
   production code and retained experiment 05 remain intact.

## Change control

Refine lab-local paths, test organization, timeout plumbing and stronger evidence in
place while the accepted question, one-run cost boundary and claim remain unchanged.
Stop implementation, keep or return `status: draft`, revise and request explicit
re-acceptance when new evidence changes the outcome, public behavior, domain meaning,
real/simulated seam boundary, selected host/model, external side effect, token-spend
boundary or verdict criteria.

## Completion conditions

- every required task is `completed` and the validation ladder passes;
- deterministic preflight consumed zero Agent runs and the live lab launched no more
  than one guarded Agent task with zero retries;
- one supported, refuted or inconclusive verdict is grounded in separate server,
  host, Agent and authoritative-readback observations;
- current concept choices, experiment navigation and concept log are aligned without
  changing `game/docs/`, canonical vocabulary or production runtime;
- no secret, controller token, temporary configuration, orphan process, stale
  authority or accidental unrelated change remains; and
- `status: complete` and `completed_at` are recorded only after these conditions.
