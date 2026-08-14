---
status: complete
created_at: "2026-08-12T23:06:56+02:00"
updated_at: "2026-08-13T08:55:00+02:00"
accepted_at: "2026-08-13T08:18:24+02:00"
completed_at: "2026-08-13T08:55:00+02:00"
---

# Play local Aicadia from Codex in the ChatGPT desktop app

## Outcome

The User opens an Aicadia-scoped Codex conversation in the already selected
Aicadia repository, and that host reaches the running local World exclusively
through stateless MCP `2026-07-28` using the existing stable development User.
Evidence must prove the desktop Codex host loads the exact Aicadia connection and
can discover the published player contract and thirteen tools without an
`initialize` path.

## Non-goals

- No clean player folder, skill, plugin, tunnel, hosted endpoint, OAuth or public
  distribution.
- No Aicadia code, schema, World behavior, HTTP/MCP contract or durable World data
  change.
- No legacy MCP compatibility path.
- No automatic game mutation merely to prove connectivity.

## Evidence and authorities

| Evidence or authority | Current fact | Consequence for this build |
| --- | --- | --- |
| `docs/game/local-play.md` | The stable local profile and current Agent contract are the supported local authority. | Reuse the running World and existing User; do not provision or reset. |
| `docs/game/agent-interface.md` | A conforming host requires stateless `2026-07-28`, discovery instructions, the complete catalog and fail-closed live authority. | Verification must inspect those exact facts. |
| `.codex/config.toml` | The repository already registers `http://127.0.0.1:3000/mcp` and resolves `Aicadia-User-Id` from `AICADIA_USER_ID`. | Keep the connection repository-scoped for this experiment. |
| `~/.codex/config.toml` and bundled desktop Codex `features list` | The desktop host shares this config and supports `mcp_2026_07_28`, currently disabled. | Enable only this feature globally. |
| `.aicadia-local/profile.json` and live port check | Database `aicadia_local`, stable User `f149fb45-9be1-494e-b2b7-79a56c39475e`, and loopback port 3000 are available. | Supply that UUID to the desktop process without writing it into the tracked repository. |
| `docs/research/chatgpt-local-and-hosted-mcp-access.md` | Stateless-only is confirmed and a skill is deferred until a demonstrated workflow gap. | Do not broaden this experiment. |

## Alignment

### Strategic

This retires the immediate access blocker between the existing local World and an
ordinary Codex conversation inside the ChatGPT desktop app. It advances usable
Agent-mediated play without adding another game surface. The next risk is whether
this less-isolated host actually preserves Aicadia's player-mode and fail-closed
contract.

### Tactical

The smallest slice starts the local server with the existing profile when it is not
already running, enables the existing protocol feature, supplies the existing
profile User to the desktop environment, restarts the desktop host, and observes
MCP discovery from the Aicadia repository. No state-changing World tool is needed.

### Technical

`World`, PostgreSQL, migrations, transactions, HTTP/MCP schemas and Activity are
unchanged. Only user-level Codex feature configuration and the reversible macOS
login-session environment are changed. The existing repository MCP registration
remains the server definition.

## Decisions, assumptions and open questions

### Confirmed decisions

- Use only stateless MCP `2026-07-28`; reject legacy `initialize` compatibility —
  explicit User direction, recorded in the research note.
- Run the experiment from the Aicadia repository; do not create a clean folder —
  explicit User direction.
- Defer an explicit play skill until after connectivity is proven — explicit User
  direction.

### Reversible assumptions

- `launchctl setenv AICADIA_USER_ID ...` makes the UUID available to a freshly
  started desktop process — verify after restart; unset it if the host does not
  resolve the header.
- Existing project instructions and tools may weaken host isolation but do not
  prevent a bounded connection check — do not claim full conformance from this
  experiment.

### Open questions

- Whether restarting the desktop application interrupts this active task before
  post-restart evidence can be collected. If it does, the persisted plan and
  configuration permit continuation in the reopened app; this affects execution
  continuity, not the chosen outcome.

## Implementation map

| Surface | Current state | Intended change | Invariants |
| --- | --- | --- | --- |
| Local Aicadia server | The profile exists but the process stopped before execution. | Start it through the supported local launcher without resetting data. | Preserve the existing database and stable User. |
| `~/.codex/config.toml` | `[features]` exists; `mcp_2026_07_28` is absent/off. | Add `mcp_2026_07_28 = true` once. | Preserve every other personal setting and plugin/MCP entry. |
| macOS login environment | `AICADIA_USER_ID` is unset. | Set it to the validated local profile UUID for newly launched app processes. | Never write the UUID into tracked repository configuration. |
| ChatGPT desktop app | Running with old process environment/config. | Fully restart after configuration. | Do not alter or archive tasks; do not send a gameplay prompt without explicit plan acceptance. |
| Aicadia repository task | Existing `.codex/config.toml` supplies the local URL and environment-backed header. | Open/use an Aicadia-scoped Codex task and inspect MCP status. | No clean folder and no change to `.codex/config.toml`. |

## Execution contract

Root owns scope, configuration edits, restart coordination and evidence. No
delegation is needed. Existing unrelated research changes remain intact.

## Task graph

| ID | State | Depends | Parallel-safe | Objective | Owned surfaces | Evidence |
| --- | --- | --- | --- | --- | --- | --- |
| T0 | completed | — | no | Restore the stopped local server through the supported launcher. | Existing local Aicadia process and profile | Loopback health check succeeds and the stable User remains unchanged. |
| T1 | completed | T0 | no | Configure the desktop Codex host for current stateless Aicadia. | `~/.codex/config.toml`, macOS login environment | Parsed feature/config and resolved UUID checks. |
| T2 | completed | T1 | no | Restart the desktop host and prove repository-scoped Aicadia discovery without mutation. | ChatGPT desktop process and Aicadia-scoped task | Fresh app process and this resumed task expose the exact instructions and all thirteen Aicadia tools through current stateless MCP. |

## Task details

### T1 — Configure stateless local access

**Objective:** A fresh desktop Codex process can resolve the existing repository MCP
registration with current MCP enabled and the validated local User context.

**Actions:**

1. Patch only the existing `[features]` table in `~/.codex/config.toml` to enable
   `mcp_2026_07_28`.
2. Set `AICADIA_USER_ID` in the macOS login-session environment to the UUID verified
   from `.aicadia-local/profile.json`.
3. Parse and inspect the effective configuration with the bundled desktop Codex
   binary.

**Invariants:**

- Do not copy or replace the personal config wholesale.
- Do not expose the UUID as an MCP tool argument or commit it to the repository.
- Leave the running Aicadia process and database untouched.

**Evidence:**

- Bundled `codex features list` reports `mcp_2026_07_28` enabled.
- `launchctl getenv AICADIA_USER_ID` equals the profiled UUID.
- Bundled `codex mcp get aicadia --json` reports the existing loopback URL and
  environment-backed header.

**Stop conditions:**

- Stop before restart if personal configuration cannot be patched narrowly, the
  profile changes, Aicadia stops, or effective MCP configuration differs.

### T2 — Restart and observe discovery

**Objective:** The freshly started desktop host loads and discovers Aicadia from the
Aicadia repository without changing World state.

**Actions:**

1. Fully restart ChatGPT after T1 passes.
2. Open or continue an Aicadia-repository Codex task.
3. Inspect Aicadia MCP status and perform read-only discovery/catalog verification.

**Invariants:**

- Use no legacy initialization path.
- Make no state-changing game call and start no skill work.
- Do not call HTTP, PostgreSQL or repository data as fallback live-game authority.

**Evidence:**

- A fresh host shows Aicadia connected through `http://127.0.0.1:3000/mcp`.
- Discovery supplies the exact `src/agent-play-contract.txt` content.
- Tool listing equals the exact thirteen-name checked-in catalog.
- Server/test logs or request observation contain no `initialize` or transport
  session.

**Stop conditions:**

- Stop if the restart would terminate unrelated active work without a recoverable
  continuation, if the host attempts legacy initialization, or if exact discovery
  cannot be observed without a token-spending gameplay turn. A token-spending turn
  requires a separate explicit go at that point.

## Validation ladder

1. **Focused:** parse global config, inspect effective Aicadia MCP registration and
   confirm environment/profile equality.
2. **Contract:** compare discovered instructions and tool catalog with current
   checked-in authorities and reject `initialize` evidence.
3. **Outcome:** show Aicadia connected in a fresh Codex task scoped to this
   repository without World mutation.
4. **Integrity:** `git diff --check`, focused diff review and confirmation that no
   repository file beyond this plan and the existing research record changed.

## Change control

Stop, revise and request re-acceptance if execution would require a tracked UUID,
legacy MCP behavior, a skill/plugin, World mutation, token spend, a new folder,
OAuth, a tunnel or a broader user-level tool-policy change.

## Completion conditions

- T1 and T2 are completed and the validation ladder passes;
- the exact stateless local connection claim is demonstrated;
- no World state or public contract changed;
- no unrelated personal or repository configuration changed; and
- `status: complete` and `completed_at` are recorded only after these conditions.

## Completion evidence

- The restarted ChatGPT process began at `2026-08-13 08:53:57 +02:00`.
- Bundled desktop Codex reports `mcp_2026_07_28` enabled and resolves the
  repository-scoped `aicadia` server at `http://127.0.0.1:3000/mcp`.
- The login-session `AICADIA_USER_ID` equals the unchanged profile User
  `f149fb45-9be1-494e-b2b7-79a56c39475e`.
- Pre-restart protocol observation returned only `2026-07-28`, the exact shared
  instructions, the exact checked-in thirteen-tool catalog, JSON transport and no
  `Mcp-Session-Id` header. A legacy `initialize` could not have produced this
  discovery because Aicadia implements only the stateless path.
- After the real app restart, this resumed repository task exposes exactly the
  thirteen `mcp__aicadia__*` tools with no missing or extra Aicadia tool.
- No Aicadia tool was called and the World was not mutated during verification.
