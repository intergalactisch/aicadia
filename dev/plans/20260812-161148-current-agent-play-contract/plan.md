---
status: complete
created_at: "2026-08-12T16:11:48+02:00"
updated_at: "2026-08-12T21:54:35+02:00"
accepted_at: "2026-08-12T21:36:04+02:00"
completed_at: "2026-08-12T21:54:35+02:00"
---

# Required current immersive Agent play contract

## Outcome

Every conforming interactive Agent host must enter one explicit Aicadia player mode:
current stateless MCP `2026-07-28` is required, the one provider- and model-neutral
play contract governs the conversation, and Aicadia MCP results are the only
authority for live game state. The Agent translates typed internal records into
grounded, inviting language about named people, locations, things and events. It
never exposes record categories, fields, absent-value syntax, protocol operations,
storage or delivery work in the play conversation. World remains the dumb, strict
processor that deterministically accepts only valid current commands and exposes
the same accepted state to every User.

This remains the highest-value current edge because two real local conversations
proved the World loop but not the player experience. The first exposed UUID,
package and commit language. In the second, the printed Codex command omitted the
required current MCP feature; after startup failed, the coding Agent read repository
contracts and source, queried the HTTP API and even submitted the confirmed action
over HTTP. It later answered a normal location question with `Entity`, `Place`,
`current_place`, field absence and capability language. The result was factually
accurate but ceased to be a game.

Final evidence must prove the local handoff starts outside the development
repository, requires Aicadia MCP before play, injects the same runtime contract at
the local host boundary and provides no repository/API fallback. The contract must
make player mode permanent for the conversation: even questions about what
something is or can do are answered through observable situation and playable
affordances, never through implementation vocabulary. All thirteen capabilities and
existing World semantics remain intact, external integration stays capability-based
rather than provider-allowlisted, and no compatibility transport, narrative linter,
content mapper or server-side inference is introduced.

## Non-goals

- Guarantee identical wording or instruction-following from arbitrary LLMs; model
  output remains nondeterministic.
- Identify, certify, rank or allowlist providers, models, clients or tools.
- Inspect, persist, score or lint private Agent conversation.
- Add a forbidden-word list, field-to-copy mapping, deterministic prose grader or
  server-side narrative projection.
- Claim that the server can prove a human performed a private confirmation.
- Add MCP prompts, resources or tools, a prompt database, dynamic rule storage,
  schema or migration changes, authentication, durable conversation or server-side
  model calls.
- Change game HTTP shapes, World operations, consequence semantics, Activity meaning
  or the read-only browser ledger.
- Rename internal domain types or MCP tools merely to influence model wording.
- Spend tokens on another live/model-driven playtest; that requires a separately
  accepted evidence claim and explicit authorization.

## Evidence and authorities

| Evidence or authority | Current fact | Consequence for this build |
| --- | --- | --- |
| User direction on 2026-08-12 | Aicadia is current-only, must contain no dead compatibility code, must remain open across providers, and external documentation may require clients to support the selected current standard. | Keep only stateless `2026-07-28`; publish a capability requirement rather than restoring an older lifecycle. |
| User direction on 2026-08-12 after reviewing the latest play conversation | Player-facing communication must always be narrative and must never expose technical categories such as `Entity`, `Place` or `null`. | Remove the in-conversation technical-mode exception. Internal vocabulary remains exact in code and wire contracts but is always translated at the player boundary. |
| Real local conversation `019ff753-4e0b-7693-8245-0bcb861989f1` | MCP startup failed, then the Agent used repository files, Rust/SQL contracts and direct HTTP as a substitute. It narrated launcher, port and API work and later explained location through internal types and fields. | A player host must fail closed when Aicadia MCP is unavailable and may not derive live game authority from repository, HTTP, database, shell, browser or remembered state. The local Codex start must run outside the repository and require MCP. |
| Real local answer to the Discovery Rack location question | The accurate facts were that the rack and workbench both stand at First Landing, but the answer expanded them into role, relation, field and capability explanations. | Mechanics questions need a positive rendering rule: answer what is present, where it is and what the player can currently do, with named in-world referents and no schema exposition. |
| Real current Codex startup on 2026-08-12 | `codex-cli 0.147.0` sent `initialize`; Aicadia returned JSON-RPC `-32601`, so startup failed before tools were available. | The server behaved as specified, but the launcher gave an incomplete Codex handoff command. Correct the handoff rather than the protocol boundary. |
| OpenAI's tagged `codex-cli 0.147.0` source | Its default MCP mode proposes `2025-06-18`; its `2026-07-28` discovery mode exists behind the off-by-default `mcp_2026_07_28` feature. | The local Codex command must explicitly enable that feature. This is a client-specific invocation of a provider-neutral protocol requirement, not a server-side client branch. |
| `AGENTS.md` | World is dumb and strict; Agent intelligence is User-owned; all thirteen player capabilities retain World/HTTP/MCP semantic parity; deferred behavior stays absent. | Strengthen current-only governance without adding inference or changing the capability catalog. |
| `game/docs/agent-interface.md` | Two MCP revisions are documented and private Agent workshops carry the current narration and confirmation obligations. | Replace the two-revision contract with current `2026-07-28` only and define where global play guidance, tool-local rules, World facts and private conversation come from. |
| `game/src/server.rs::MCP_INSTRUCTIONS` | One monolithic instruction string is published through MCP while thirteen tool descriptions live inline. | Move all Agent-facing wording to one small runtime module and keep tool behavior unchanged. |
| `game/src/server.rs::app` and `ServerHandler::supported_protocol_versions` | `initialize` is rejected, only `2026-07-28` is advertised and every request is forced through the stateless transport. | Preserve this exact current server boundary; no runtime protocol change is needed. |
| `studio/tools/aicadia-local`, `game/docs/local-play.md` and `studio/tests/aicadia-local.sh` | The launcher prints `AICADIA_USER_ID='…' codex`, and the lifecycle test enforces that incomplete in-repository command. | Print one explicit `game/tools/aicadia-agent` player command. That adapter supplies the feature, required MCP config and isolated working directory, while the launcher continues to print but never invoke it. |
| `dev/playtest/agent/run` isolated Agent setup | Existing bounded evidence already starts Codex from an empty temporary directory, makes Aicadia MCP required and supplies its connection through explicit config overrides. | Reuse this proven small host pattern for ordinary local play, without reusing the paid harness, schemas, model pin or least-privilege tool lists. |
| Official OpenAI Codex skill/config guidance | Repository instructions and skills are discovered from the working directory upward to the repository root; project configuration is likewise working-directory scoped. | Starting the player in the Aicadia repository inherently exposes build instructions and skills. The local player working directory must be outside that repository. |
| `rmcp = 3.1.1` local source | The pinned SDK implements current `2026-07-28` stateless discovery and request metadata. | No dependency change or speculative protocol shim is needed. |
| MCP specification and maintainer guidance | `server/discover.instructions` is natural-language guidance but discovery and host injection are not universally mandatory; prompts/resources are not stronger mandatory instruction channels. | Publish one contract through current discovery, repeat only critical operation-local rules in tool descriptions and document the honest host boundary. |
| `tests/server.rs::action_http_and_mcp_share_commit_retry_visibility_and_errors` | A second User already observes the same accepted Entity and Activity/prose at the same Place. | Preserve this as the shared-World outcome proof; no new synchronization layer is needed. |

Governing current authorities are `AGENTS.md`, `dev/CONTEXT.md` and
`game/docs/agent-interface.md`. The completed Agent-action and local-ledger backlog
items remain historical evidence; this plan creates the next `Now` item without
rewriting their completed outcomes. Sourced MCP findings belong in `dev/docs/research/`
and the accepted choice belongs once in `dev/docs/concept/log/log.md`.

## Alignment

### Strategic

The User should inhabit one shared world rather than watch a database transaction
conducted through a coding assistant. One Aicadia-owned player contract improves
that experience for every conforming Agent host without taking ownership of its
model or tokens. One capability requirement keeps the server small and integration
open: any provider or host may connect when it implements stateless MCP
`2026-07-28`, injects the published contract and treats the connection as required.
The next concrete game risk remains the quality of grounded narration across varied
models; proving that requires separate scenario-based model evidence rather than
pretending deterministic server tests can judge prose.

### Tactical

The slice changes the Agent wording and local player-host envelope, not World meaning
or server transport. A compatible MCP host receives one global contract and thirteen
tools through `server/discover`. The Agent privately chooses the relevant fixed
method from actual tool results: Character workshop, entry, orientation, action
proposal, confirmation, affordance explanation or conflict recovery. It then renders
facts through named in-world subjects and ordinary language in the User's language:
where something stands, what happened, what is visible or established and what can
currently be attempted. Internal domain categories, wire fields, missing-value
syntax, capabilities and delivery work never appear in the player conversation.

Player mode has no technical submode. A request to explain implementation belongs in
a separate developer conversation; within play, questions such as whether the rack
has a location are answered directly (both the rack and workbench stand at First
Landing) without explaining how that fact is represented. Stored World prose remains
English. The Agent previews the complete intended meaning in the User's language and
submits semantically identical English content without showing payload labels or raw
English transport text. Typed result fields and structured consequences remain
authoritative; prose and Agent framing never create extra state or mechanics.

The server continues to reject malformed context, unknown fields, invalid state,
stale Place revisions, mismatched idempotent retries and unsupported consequences,
and continues to write accepted state plus Activity atomically. It does not pretend
to validate tone, creativity or a private human confirmation. HTTP remains fully
available with the same thirteen semantic capabilities. The MCP endpoint remains
strictly stateless `2026-07-28`; pre-discovery lifecycle clients are unsupported.

### Technical

Keep the private `agent_contract` module, explicit `initialize` rejection,
`NeverSessionManager`, required per-request metadata and sole advertised revision
`2026-07-28` unchanged. Rewrite the single included player contract and operation
descriptions around a positive rendering boundary and fail-closed authority rule.
Add one thin local `game/tools/aicadia-agent` adapter: it validates the existing local
profile/server, creates an empty temporary working directory outside the Git
checkout, injects the exact same included contract as Codex developer instructions,
makes the Aicadia MCP server required through command-line config, enables the
current protocol and executes Codex only after the User explicitly runs the command.
It cleans only its owned temporary directory after Codex exits. Do not add protocol
negotiation, a gateway, a feature registry, a server-side client branch or any
transport session.

No `World`, PostgreSQL, migration, wire schema, HTTP operation or transaction change
is applicable. Existing catalog-fixture equality, HTTP/MCP parity and cross-User
visibility tests remain the executable interface proof. A fake Codex executable
proves only the local adapter's working directory, exact configuration, environment,
cleanup and no-launcher-invocation behavior; it does not claim prose quality.
Compiler warnings and Clippy fail the build; no custom narrative linter or model
grader is added.

## Decisions, assumptions and open questions

### Confirmed decisions

- Aicadia-owned implementation is current-only: it serves only stateless MCP
  `2026-07-28`, with no compatibility bridge, dormant flag, `allow(dead_code)`
  suppression or dead code.
- Client diversity is capability-based and standards-based, never provider-, model-
  or client-name based; no allowlist or certification registry is added.
- One provider-neutral Agent play contract is published through current MCP
  discovery, with critical operation-local requirements in the matching tool
  descriptions.
- A player conversation is permanently player-facing. It never switches into
  protocol, schema, server or data-model explanation, even when a User asks how an
  in-world subject works; implementation questions belong in a separate development
  context.
- Internal nouns stay precise in code and wire contracts. The Agent renders their
  meaning rather than renaming the model: named people, locations, things and events;
  absence becomes a natural fact rather than `null` or a missing field.
- Aicadia MCP is the sole authority for live game state. Repository files, source,
  direct HTTP, PostgreSQL, shell, browser, logs and remembered prior conversation are
  never fallback sources for play. When MCP is unavailable, the player session stops
  before gameplay and no mutation is attempted.
- The Agent remains free in reasoning and wording; World alone is authoritative and
  validates deterministic domain meaning.
- Private conversation is neither transmitted to nor validated by World. An MCP
  input ceremony cannot prove human intent and is therefore not added as false
  assurance in this slice.
- No new live model run is authorized by acceptance of this build plan.
- Compatible Agent hosts must implement stateless MCP `2026-07-28`; provider, model,
  tool choice and internal architecture otherwise remain unrestricted. They must
  inject the global contract, keep raw tool/protocol progress out of the player UI
  and fail the player session closed when required Aicadia discovery cannot complete.

### Reversible assumptions

- `game/mcp/agent.rs` is the smallest ownership seam because both discovery and
  thirteen macro descriptions consume its constants; confirm that the existing
  `rmcp` attribute accepts constant expressions during T2 and keep the constants in
  `server.rs` if that evidence fails rather than introducing generation machinery.
- Existing strict World tests sufficiently cover the server-side semantic boundary;
  T3 may add a focused regression only if review finds a concrete uncovered current
  invariant, without changing public behavior.
- A small interactive Codex adapter can reuse the already proven command-line MCP
  overrides and empty temporary working-directory pattern from `dev/playtest/agent/run`
  without importing its model pin, output schemas, restricted tool lists or evidence
  orchestration; validate the exact invocation with a fake executable before any
  real Codex run.

### Open questions

- None. The User explicitly accepted permanent player mode, a complete User-language
  rendering before confirmation, sole MCP authority and the isolated local
  player-host adapter on 2026-08-12. Arbitrary protocol callers may still ignore
  optional MCP guidance; only hosts satisfying the published player-host contract
  may claim a conforming Aicadia play experience.

## Implementation map

| Surface | Current state | Intended change | Invariants |
| --- | --- | --- | --- |
| `AGENTS.md` | No explicit current-only rule. | Add one compact Terry heuristic covering current standards, removal of superseded code and provider-agnostic support. | Historical records remain history; current consumer evidence can justify a future explicit exception. |
| `dev/docs/research/README.md`, new current-MCP research note | No durable record of instruction-delivery limits used by this decision. | Record primary MCP findings and exact implications. | Research informs but does not govern the build. |
| `game/docs/README.md`, `game/docs/agent-interface.md`, `game/docs/agent-playtest.md` | Dual-revision MCP and technically worded workshops. | Define current-only transport, context sources, player-facing communication and honest evidence boundary; correct stale legacy claims. | Thirteen capabilities, English stored content and shared-World meaning remain unchanged. |
| `dev/docs/concept/log/log.md` | No accepted current-only Agent-contract choice. | Record accepted/rejected/deferred choices once. | Exploration history does not override `game/docs`. |
| `dev/backlog/README.md`, new current item | No `Now` item. | Add one active forward item for this accepted build and later mark Done only with full evidence. | At most one item is active. |
| `game/mcp/agent.rs`, `src/lib.rs` | Agent wording lives in `server.rs`. | Own global instructions and all tool descriptions in one private module. | No runtime configuration, provider branch or unused public interface. |
| `src/agent-play-contract.txt`, `game/mcp/agent.rs`, `game/mcp/tool-catalog.json` | One runtime contract hides transport work but still calls `Character`, `Place` and `Entity` normal player language and permits a technical submode. Tool descriptions expose internal representation densely. | Define permanent player mode, sole-authority/fail-closed behavior, positive narrative rendering, User-language semantic preview and operation-local non-disclosure cues. Update only description fixtures. | Tool names, schemas, annotations, ordering, preconditions and World calls remain exact. |
| `game/src/server.rs`, `tests/server.rs` | Modern-only server behavior and exact negative/positive evidence are already correct. | Keep implementation unchanged; rerun discovery, exact contract, catalog and rejection proof after wording changes. | Sole `2026-07-28` support, thirteen tools, HTTP routes, World calls and errors remain identical. |
| New `game/tools/aicadia-agent`, `studio/tools/aicadia-local`, `studio/tests/aicadia-local.sh` | Printed Codex handoff starts in the development repository, omits the required feature and lets startup failure fall through into a coding task. | Add one explicit local player adapter using an external empty temporary cwd, the same contract as developer instructions, required current MCP config and the stable User environment. Print but never invoke it from the launcher; fake-test arguments, cwd and cleanup. | No provider/model pin, MCP tool allowlist, automatic Agent start, credential persistence or gameplay mutation in the launcher/adapter. |
| `game/docs/local-play.md`, `game/docs/agent-interface.md`, research and concept record | Server contract is modern-only, but local startup, host conformance and permanent player-language boundaries are incomplete. | Document the two-command local flow, generic host requirements, sole-authority failure behavior and honest nondeterminism boundary; record why repository/HTTP fallback and in-play technical mode are rejected. | No provider allowlist or promise that arbitrary nonconforming Agent hosts produce the experience. |

## Execution contract

Root owns outcome, scope, plan state, integration and the final evidence claim. T7
and T8 are parallel-safe because their write surfaces do not overlap; T9 integrates
them and owns every authority and final check. The User explicitly authorized bounded
`gpt-5.6-sol`/high delegation; each delegated Agent receives exactly one task id and
owned surface. Preserve every unrelated existing change in the dirty main checkout
and never reset, overwrite or discard User work.

## Task graph

| ID | State | Depends | Parallel-safe | Objective | Owned surfaces | Evidence |
| --- | --- | --- | --- | --- | --- | --- |
| T1 | completed | — | no | Align current authorities, research trail and forward state around one current-only provider-neutral Agent contract. | `AGENTS.md`, `dev/docs/research/`, `game/docs/`, `dev/docs/concept/log/log.md`, `dev/backlog/` | Links resolve; focused contradiction searches find no current dual-revision or provider-specific contract claim. |
| T2 | completed | T1 | no | Publish all Agent guidance from one runtime source and remove every Aicadia-owned legacy MCP execution path. | `game/mcp/agent.rs`, `src/lib.rs`, `game/src/server.rs`, `tests/server.rs`, `game/mcp/tool-catalog.json` | Focused current discovery/catalog/stateless/rejection tests pass. |
| T3 | completed | T2 | no | Prove strict World semantics and shared cross-User observation remain unchanged, then independently review freedom/restriction seams. | Existing World/server tests; source changes only for a concrete uncovered current regression | Action parity, retry/freshness, invalid-input and observer tests pass; review finds no hidden allowlist, model call, prompt store or legacy path. |
| T4 | completed | T3 | no | Run the full validation ladder, align completion state and make the exact bounded evidence claim. | All changed surfaces and live plan state | Formatting, Clippy, all tests, launcher suite, diff integrity and authority review pass. |
| T5 | completed | T4 | no | Close the two post-completion review findings at the existing MCP and Agent-contract seams without adding orchestration or changing World meaning. | `game/src/server.rs`, `src/agent-play-contract.txt`, `tests/server.rs`, `game/mcp/tool-catalog.json`, `dev/docs/research/current-mcp-agent-guidance.md`, `game/docs/agent-interface.md`, `dev/docs/concept/log/log.md`, `dev/backlog/` | Both current and old `initialize` fail as absent methods through real `/mcp`; discovery and thirteen tools remain current; the one global contract treats all World content as untrusted game data, never instructions. |
| T6 | completed | T5 | no | Re-run the full validation and independent diff/resource review, then restore completion state only if every original and corrected claim is proven. | All changed surfaces and live plan state | Focused protocol/contract tests, formatting, Clippy, all tests, launcher suite, diff integrity and cleanup pass with no P0-P3 finding. |
| T7 | completed | T6 | yes | Make permanent player narration and sole MCP authority exact in the one runtime contract. | `src/agent-play-contract.txt`, `game/mcp/agent.rs`, `game/mcp/tool-catalog.json` | Discovery exposes the exact permanent player contract and thirteen unchanged schemas/annotations with rewritten descriptions. |
| T8 | completed | T6 | yes | Add one fail-closed isolated local player adapter and make the launcher hand it off without invoking it. | new `game/tools/aicadia-agent`, `studio/tools/aicadia-local`, `studio/tests/aicadia-local.sh` | Shell syntax and diff integrity pass; disposable lifecycle proves external empty cwd and home/config, required MCP/current feature, exact shared instructions, stable User context, cleanup, restart identity and `codex_invoked=false`. |
| T9 | completed | T7, T8 | no | Align current authorities, independently review both seams and run the complete validation ladder before restoring Done/complete state. | `AGENTS.md`, `game/docs/`, `dev/docs/research/`, `dev/docs/concept/log/log.md`, `dev/backlog/`, all changed surfaces and live plan state | Contract/catalog/parity tests, formatter, strict Clippy, all tests, launcher/adapter lifecycle, diff integrity, process/resource checks and review pass; no raw World mutation or model run occurs. |

## Task details

### T1 — Current authority and research

**Objective:** Every governing document describes one current MCP contract, the
player-facing communication method and the strict/free seam without stale dual-mode
or provider-specific direction.

**Actions:**

1. Add the accepted compact current-only Terry rule to `AGENTS.md`.
2. Record primary MCP instruction/discovery/control findings in `dev/docs/research/` and
   link them from its index.
3. Update `game/docs/` with the exact context hierarchy, play methods, technical-mode
   exception, typed-state authority and current-only protocol boundary.
4. Record the accepted, rejected and deferred choices in the concept log and create
   the one `Now` backlog item.

**Invariants:**

- No new domain noun is added to `dev/CONTEXT.md`; the Agent contract is an interface
  contract, not a World subject.
- Historical plans and concept records remain intact.
- No code or executable contract changes in this task.

**Evidence:**

- Focused searches for `2025-11-25`, `legacy`, provider names and conflicting Agent
  context claims — no stale current-authority contradiction.
- `git diff --check -- AGENTS.md docs dev/backlog` — clean authority edits.

**Stop conditions:**

- Stop if documentation exposes a material choice about authentication, new World
  behavior, durable conversation or an Agent-owned identity.

### T2 — One current runtime contract

**Objective:** Current MCP publishes one exact global contract and thirteen exact
tool descriptions from one module, with no Aicadia-owned legacy transport support.

**Actions:**

1. Move global instructions and tool descriptions into the private Agent-contract
   module and wire existing MCP macros and discovery to its constants.
2. Rewrite the wording to be provider-neutral, grounded and player-facing while
   keeping every current semantic prerequisite exact.
3. Disable legacy session mode, advertise only `2026-07-28` and remove branches that
   exist solely for older-protocol cache behavior.
4. Delete legacy test helpers and stateful behavior tests; retain one focused
   fail-closed rejection proof for unsupported protocol versions.
5. Update the exact tool-catalog fixture without changing its thirteen names,
   schemas, annotations or ordering except for accepted descriptions.

**Invariants:**

- No World, wire, HTTP, database or capability-catalog change.
- No provider/client inspection, allowlist, prompt/resource/tool addition, server
  session or model call.
- No duplicated old constants or dead compatibility branches remain.

**Evidence:**

- `DATABASE_URL=postgres://localhost:5433/postgres cargo test --test server catalog_`
  — exact one-version discovery, instructions and tool catalog.
- Focused current stateless metadata and unsupported-version tests — current calls
  succeed and old versions fail closed without a session.
- `cargo fmt --all -- --check` and focused `cargo clippy` — no warnings.

**Stop conditions:**

- Stop if `rmcp` cannot reference contract constants without code generation, or if
  current-only operation requires a dependency fork, custom protocol or loss of any
  existing current client behavior.

### T3 — Strict World and freedom seam review

**Objective:** Prove that freer Agent narration cannot bypass typed World rules and
that accepted state remains shared across Users, without pretending to understand
prose server-side.

**Actions:**

1. Run focused malformed-input, unsupported-consequence, idempotency, freshness,
   atomic Activity and cross-User observer tests.
2. Review every changed tool description against its World call and checked-in
   schema; add only a concrete missing current regression test if found.
3. Search executable sources for provider/model/client branches, LLM calls, prompt
   persistence, narrative scoring, legacy sessions and dead-code suppression.

**Invariants:**

- Free prose is not interpreted as state; the one structured consequence remains
  the only action state change.
- No private transcript enters World or Activity.
- No live model call or external mutation is performed.

**Evidence:**

- Focused `tests/world.rs` and `tests/server.rs` filters for action, parity, context,
  invalid request and observer visibility — all pass.
- Independent source/diff review — no P0-P3 scope, truth or integration finding.

**Stop conditions:**

- Stop if a current accepted World invariant is not deterministic, or if fixing it
  would change public semantics, schema, Activity meaning or the thirteen-tool
  surface.

### T4 — Full validation and completion

**Objective:** The complete repository proves the bounded current-only Agent-contract
claim and all authorities agree.

**Actions:**

1. Run the complete automated validation ladder and local launcher lifecycle suite.
2. Review the final diff for unrelated changes, stale current claims and exact
   absence of legacy/dead/provider-specific code.
3. Mark the backlog item Done and the plan complete only after every condition is
   evidenced.

**Invariants:**

- No live Agent/model playtest and no token spend.
- Existing dirty main-tree User changes are preserved.
- Completion claim remains limited to contract delivery and deterministic World
  behavior, not universal LLM compliance or prose quality.

**Evidence:**

- `cargo fmt --all -- --check` — formatting clean.
- `cargo clippy --all-targets --all-features -- -D warnings` — no warnings or dead
  code.
- `DATABASE_URL=postgres://localhost:5433/postgres cargo test --all-targets` — full
  Rust suite passes.
- `DATABASE_URL=postgres://localhost:5433/postgres /bin/bash studio/tests/aicadia-local.sh`
  — local lifecycle remains correct and invokes no Agent.
- `git diff --check` plus focused final diff/status review — clean and scoped.

**Stop conditions:**

- Stop rather than weaken checks if any current behavior, shared-state proof or
  unrelated User change would be lost.

## Validation ladder

1. **Focused:** current discovery/catalog/stateless/rejection tests; strict World
   action and observer tests; documentation contradiction checks.
2. **Contract:** exact thirteen-tool catalog, HTTP/MCP semantic parity, typed errors,
   idempotency/freshness and cross-User Place visibility.
3. **Outcome:** raw current MCP discovery exposes the one permanent player contract
   and only `2026-07-28`; the launcher prints the isolated adapter command; fake-host
   evidence proves required MCP and no repository context; existing deterministic
   tests prove a second User reads the same accepted state. No Agent or model is
   invoked for this proof.
4. **Integrity:** formatter, Clippy with denied warnings, all Rust tests, local
   lifecycle, `git diff --check` and focused preservation review.

## Change control

Refine paths, task order and stronger evidence in place while the accepted outcome
and contract remain unchanged. Stop implementation, set `status: draft`, revise and
request explicit re-acceptance when new evidence changes the outcome, public
behavior, domain meaning, non-goals, irreversible state, external authority,
material cost or evidence claim.

In particular, adding MRTR confirmation, changing HTTP/MCP parity, interpreting
prose, adding a provider gate, adding a new tool/resource/prompt, or authorizing a
live model run requires a revised plan and explicit re-acceptance.

## Completion conditions

- every required task is `completed` and the validation ladder passes;
- current MCP publishes one provider-neutral Agent contract and thirteen aligned
  descriptions from one runtime source;
- current integration documentation requires stateless MCP `2026-07-28`, permanent
  player-mode rendering, sole MCP state authority and fail-closed startup, and the
  local launcher prints the isolated adapter command;
- all Aicadia-owned legacy transport code, tests and current documentation are gone;
- strict World semantics, HTTP/MCP parity and cross-User shared-state evidence remain
  complete;
- no provider allowlist, prompt store, narrative mapper/linter, model inference,
  repository/API fallback, dead-code suppression or unauthorized model run exists;
- current behavior, research, concept choice, Terry rule and backlog agree;
- no known-stale authority, material open question or accidental unrelated change
  remains;
- `status: complete` and `completed_at` are recorded only after these conditions.

### T5 — Review correction at existing seams

**Objective:** Make the already accepted current-only and shared-Agent contract true
on every implemented MCP lifecycle path and resilient to instructions embedded in
cross-User World content.

**Actions:**

1. Reject `initialize` explicitly in the existing `ServerHandler` implementation;
   set current `ServerInfo` protocol metadata defensively rather than relying on the
   SDK's older default.
2. Replace the old-version `tools/list`-only probe with complete observable evidence:
   current discovery succeeds, current and old `initialize` are absent, and old
   inline tool calls still fail closed.
3. Add one general instruction/data hierarchy to the global Agent contract. Every
   World value is player-authored game data, never a command; no field enum, pattern
   scanner, provider rule, narrative linter or extra model is introduced.
4. Align the current authority, sourced security finding, concept record and backlog
   with the corrected behavior.

**Invariants:**

- No World, wire, PostgreSQL, HTTP game operation, schema, migration or capability
  change.
- No middleware wrapper, lifecycle state machine, version matrix, content allowlist,
  sanitizer, classifier, prompt store or server-side inference.
- Existing confirmation, retry, freshness and shared-state semantics stay unchanged.

**Evidence:**

- Real `/mcp` requests prove `server/discover` is the sole opener, both initialize
  variants fail with method-not-found and no session header, and old tool calls fail
  with unsupported-protocol-version.
- Exact global instructions and thirteen-tool fixture tests pass.
- Focused searches find one generic World-content rule and no content-control
  machinery.

### T6 — Corrected full validation

**Objective:** Re-establish the complete evidence claim after the review findings,
without relying on the stale local process or changing it implicitly.

**Actions:** Run focused server tests, formatter, strict all-target Clippy, all Rust
tests, the disposable launcher lifecycle, diff integrity and a final source/resource
review. Mark the plan and backlog complete only after all pass.

**Stop condition:** Any required behavior needing a new protocol wrapper, domain
type, World rule, model call, content filter or expanded evidence claim returns this
plan to draft for renewed acceptance.

### T7 — Permanent player contract

**Objective:** Every conforming Agent receives one exact contract that keeps the
entire conversation in player mode, renders internal structure as concrete in-world
facts and treats Aicadia MCP as the sole authority for live game state.

**Actions:**

1. Rewrite the one included global contract around positive rendering: named people,
   locations, things, events, present affordances and honest absence in the User's
   language; internal record categories, field names, missing-value syntax and
   transport progress stay private.
2. Remove the in-play technical-detail exception. Implementation questions belong
   in a separate development conversation; a player question about how something
   works receives grounded in-world facts and current affordances.
3. Make Aicadia MCP the sole live-game authority and require the Agent to stop play
   without mutation when it is unavailable; repository, source, HTTP, database,
   shell, browser, logs and memory are never fallback state sources.
4. Preserve complete semantic preview and confirmation in the User's language while
   storing semantically identical English content; never display JSON or raw payload
   labels.
5. Align all thirteen operation descriptions and update only the exact description
   values in the catalog fixture.

**Invariants:**

- No change to World, wire, PostgreSQL, HTTP game operations, schema, migration,
  tool names, schemas, annotations, ordering or behavior.
- No initialized lifecycle, transport session, client-name branch, allowlist,
  gateway, content mapper, forbidden-word list, linter or server-side inference.
- Internal game/server vocabulary remains precise and available to the model for
  tool use; only the player-facing rendering boundary changes.

**Evidence:**

- Exact discovery/instructions and catalog-fixture tests pass.
- Structural fixture comparison proves the thirteen names, order, schemas and
  annotations remain unchanged apart from descriptions.
- Focused review finds one general rendering boundary and no mapping, filter, linter,
  model call or fallback data path.

**Stop condition:** Stop if reliable rendering would require interpreting prose or
private conversation server-side, changing a World type or introducing a content
mapping/filter.

### T8 — Isolated local player host

**Objective:** The local handoff starts one explicit player conversation outside the
development repository, requires current Aicadia MCP and cannot silently fall back
to repository/API operation when startup fails.

**Actions:**

1. Add `game/tools/aicadia-agent`, which validates a stable User UUID and reachable local
   profile/server, creates a mode-700 temporary player root with empty working and
   home/config directories outside the repository, and executes Codex there only
   after the User runs it. Copy only an available local authentication file into the
   private transient Codex home; inherit no User configuration, MCP servers, skills
   or durable conversation state.
2. Supply the current feature, exact Aicadia URL/header, `required=true` and the same
   included player contract through explicit command-line configuration. Do not pin
   provider, model or tool subset.
3. Disable obvious non-game web/development context only where the current Codex
   client supports a generic command-line setting; general host conformance still
   rests on sole MCP authority, not a client allowlist.
4. Change the launcher to print the exact adapter command and never invoke it.
5. Extend the disposable shell suite with a fake Codex executable that records cwd,
   arguments and environment; prove empty external cwd, required configuration,
   stable User context, cleanup after exit and fail-closed malformed/missing context.

**Invariants:**

- No automatic model invocation, model pin, tool allowlist, credentials in the
  profile, gameplay mutation or durable conversation.
- The adapter owns and removes only its exact temporary directory, including its
  transient authentication copy and conversation state; it never deletes the source
  authentication, persistent database, profile or repository state.
- No Codex CLI or model is invoked by automated evidence.

**Evidence:**

- `/bin/bash -n game/tools/aicadia-agent studio/tools/aicadia-local studio/tests/aicadia-local.sh`.
- The fake adapter lifecycle proves exact command shape, external empty cwd and
  isolated home/config, required MCP/current feature, developer instructions, User
  environment, no inherited extra MCP configuration and cleanup.
- The full disposable launcher lifecycle preserves one User and reports
  `codex_invoked=false`.

**Stop condition:** Stop if the installed current Codex configuration cannot require
MCP or isolate repository context without a wrapper that owns gameplay or provider
behavior.

### T9 — Authority alignment, review and full validation

**Objective:** Current documentation and forward state state exactly the permanent
player boundary and conforming-host requirement, and the complete repository proves
the bounded implementation claim.

**Actions:**

1. Update `game/docs/agent-interface.md` and `game/docs/local-play.md` with permanent
   player mode, positive rendering, sole MCP authority, two-command startup and the
   generic conforming-host requirements.
2. Add the compact, explicitly accepted permanent-player boundary to `AGENTS.md` so
   future game work cannot reintroduce an in-play technical mode or alternate live
   state authority.
3. Correct the research implication, concept record and backlog: the observed
   failure combined an incomplete client invocation with a repository/API fallback;
   restoring an initialized lifecycle, technical mode, word mapping or narrative
   server remains rejected.
4. Run focused contract/adapter tests, the full validation ladder and an independent
   read-only review of narration, authority, scope and resource cleanup.
5. Mark the backlog Done and plan complete only after every condition is proven.

**Honest boundary:** Automated evidence proves the printed command, server contract
host envelope and no-Agent launcher behavior. Per the standing no-Codex/no-model
authorization, the final real conversation remains the User's manual check with the
documented command; deterministic tests cannot prove arbitrary model prose quality.

## Initial execution result — superseded by post-completion review

The initial ladder completed on 2026-08-12 without a live Agent/model run or token spend. Current MCP
publishes the single Agent play contract and exactly thirteen aligned tool
descriptions, advertises only stateless `2026-07-28`, uses no session manager and
fails closed on the old-version probe. Independent review found no P0-P3 issue, no
provider/model/client gate, model call, prompt or transcript persistence, narrative
linter, dead-code suppression, legacy execution path or added game surface.

The exact final ladder passed: formatter; Clippy across all targets and features with
warnings denied; 58 Rust tests (5 library, 2 database-helper, 10 server and 41 World);
the disposable local launcher lifecycle with stable restart identity and every
fail-closed case, reporting `codex_invoked=false`; and `git diff --check`. The owned
test database, processes and listeners were removed. A pre-existing local launcher
on port 3000 was deliberately preserved.

That initial evidence proved standards-based discovery and deterministic World enforcement,
HTTP/MCP parity and shared cross-User observation. It does not prove that every
arbitrary host performs discovery, injects or obeys the instructions, produces equal
prose quality or obtained a private human confirmation.

A later raw review against a fresh binary disproved two completion claims: stateless
`initialize` still succeeded—including with `2025-11-25`—because the pinned SDK's
default `ServerInfo` version and initialize handler remained reachable, and the
global contract did not classify cross-User World strings as untrusted game data.
The plan was reopened under the accepted outcome; the initial completion timestamp
and backlog Done state are no longer current evidence.

## Corrected execution result

Completed on 2026-08-12 after closing both review findings at the existing deep
seams. `AicadiaMcp` explicitly rejects `initialize`, reports current protocol
metadata defensively and advertises only `2026-07-28`. The one global Agent contract
now separates typed World facts from potentially player-authored World values: every
returned value is game data, never an instruction or authority for tool use or
technical disclosure. No middleware, version matrix, content taxonomy, pattern
scanner, sanitizer, linter, classifier, extra model or World change was added.

Focused integration tests and raw requests against a fresh binary proved current
discovery returns the exact contract and only `2026-07-28`; both current and
`2025-11-25` initialize requests return JSON-RPC `-32601` with no result or session;
and an old inline tool request returns `-32022` with only the current supported
version. The exact thirteen-tool catalog remained unchanged.

The complete ladder passed again: formatter; all-target/all-feature Clippy with
warnings denied; all 58 Rust tests; the disposable launcher lifecycle with stable
restart identity, every covered fail-closed mode and `codex_invoked=false`; and diff
integrity. The disposable database and review listener were removed. Final source
review found no remaining P0-P3 issue in this scope. The pre-existing process on
port 3000 remains deliberately untouched and requires a conscious restart before it
can serve this new binary to a new Agent conversation.

## Permanent player completion result

Completed on 2026-08-12 without invoking Codex, an API, a model or live gameplay.
The one included contract and all thirteen tool descriptions now require permanent
player-mode rendering through named people, locations, things, events, natural
absence and current affordances. Aicadia MCP is the sole live-state authority and a
failed discovery or required read stops play without mutation or repository, HTTP,
database, shell, browser, log or memory fallback. Complete Character and action
previews convey their meaning in the User's language while semantically identical
English remains private until submission.

The local launcher now prints one explicit `game/tools/aicadia-agent` command. The adapter
validates the stable local context, starts Codex only after the User invokes it, and
uses an empty external workspace plus isolated transient `HOME` and `CODEX_HOME`.
Only available authentication is copied mode 600; personal configuration, skills,
extra MCP servers and durable conversation are absent. Current MCP `2026-07-28`, the
exact player contract, required Aicadia connection and User header are supplied
explicitly. Cleanup removes only the exact owned temporary root.

Formatter, strict all-target/all-feature Clippy, all 58 Rust tests, shell syntax,
the disposable launcher/adapter lifecycle and diff integrity passed. The lifecycle
proved stable restart identity, fail-closed error paths, exact contract/configuration,
external isolation and cleanup, reporting `isolated_agent_handoff=true` and
`codex_invoked=false`. Independent read-only review found no P0-P3 issue. No owned
test database, temporary player root, new listener or process remained. The
pre-existing server on port 3000 was deliberately preserved and requires a conscious
restart before manual play.

The bounded result proves contract delivery, host isolation and deterministic World
behavior. It cannot prove that an arbitrary model always follows prose instructions
or uses identical wording; the documented manual conversation remains the User's
separate explicit check.
