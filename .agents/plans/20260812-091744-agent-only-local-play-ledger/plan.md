---
status: complete
created_at: 2026-08-12T09:17:44+02:00
updated_at: 2026-08-12T11:03:42+02:00
accepted_at: 2026-08-12T09:26:35+02:00
completed_at: 2026-08-12T11:03:42+02:00
---

# Agent-only local play with a persistent World ledger

## Outcome

A developer-User can start one persistent local Aicadia World, receive one stable
hidden development User identity, open a real conversation with their own Agent and
complete first-time Character onboarding there. The Agent privately offers exactly
three Character candidates, accepts selection and optional steering, previews the
exact final Character name and description, waits for explicit confirmation and then
uses the existing World capabilities to create the Character and enter the World.

Alongside that conversation, one small read-only browser page proves that the World
exists and remembers accepted changes. It shows the World connection, the shared
Entity ledger and the local Character's accepted Activity/prose ledger. It contains
no conversation or gameplay controls. Stopping and restarting the local process
preserves the same User, Character, placement, Entities, Activity and prose.

This is the highest-value current edge because Aicadia's first shared action is
already implemented and proven, but an ordinary User cannot yet start, reconnect and
inspect a persistent play session without assembling infrastructure by hand. The
final evidence must prove exactly the local runtime, stable context, read-only
inspection and restart persistence. It need not claim that one model's proposal
quality is good; the User's first real conversation is the next product experiment.

## Non-goals

- authentication, login, accounts, multiple local profiles or User switching;
- chat, prompts, a composer, proposal selection, confirmation, mutation, model calls
  or Agent orchestration in the web page;
- a dedicated User, current Character or current Place dashboard, map, model catalog
  or generic administration console;
- durable conversation, Agent reasoning, Character candidates, rejected choices,
  steering, drafts or confirmations;
- any new World operation, game-data HTTP endpoint, MCP tool, table, migration or
  Activity kind; the read-only document route `/` is the only new HTTP route;
- discovery, investigation rolls, movement, Place neighborhoods or later settlement
  behavior;
- a frontend framework, Node toolchain, design system, component library, websocket,
  server-sent events or background polling;
- automatically launching an Agent or spending model tokens on the User's behalf;
- proving semantic quality of generated Character candidates deterministically;
- making the development User id secret or treating it as authentication.

## Evidence and authorities

| Evidence or authority | Current fact | Consequence for this build |
| --- | --- | --- |
| `AGENTS.md` Boring Infrastructure and MVP filter | Authentication, OAuth and a web app are currently deferred; new accepted player behavior must first enter `docs/game/` | Explicit plan acceptance must narrowly admit this read-only local ledger while keeping web gameplay, auth and capability expansion deferred |
| `docs/game/README.md` | One persistent World, User, Character, Place, Entity, Activity/prose and the first action already have authoritative storage behavior | Reuse those models and writes; add no game state or World behavior |
| `docs/game/agent-interface.md` | Thirteen player capabilities have HTTP/MCP parity; World entry currently creates a Character immediately when absent | Evolve only the private Agent interaction before the existing `create_character` call |
| `src/server.rs::app` | The loopback Axum server exposes the thirteen APIs, OpenAPI and `/mcp`, but no `/` page | Serve one same-origin read-only HTML asset without adding an API capability |
| `src/main.rs` | `DATABASE_URL`, optional `AICADIA_DATABASE_NAME` and `AICADIA_PORT` already select and migrate a loopback server | A launcher can compose existing binaries instead of introducing a runtime service |
| `src/bin/aicadia-provision-user.rs` | Internal provisioning already creates one User and prints its wire representation | Provision once, store only that id locally and verify it on later starts |
| `.codex/config.toml` | The project MCP URL is already `http://127.0.0.1:3000/mcp` and maps `Aicadia-User-Id` from `AICADIA_USER_ID` | The launcher prints an exact environment-scoped Agent command; it does not rewrite personal Codex configuration |
| Current Codex MCP configuration reference | Project `.codex/config.toml` is shared by CLI, app and IDE; `env_http_headers` resolves header values from the Agent process environment | Codex CLI is the first fully deterministic local handoff; other clients can use the same configuration after receiving the same environment and restarting |
| `docs/concept/log/log.md#local-play-console-and-agent-onboarding--active-design` | The User fixed the absolute Agent-only conversation boundary and rejected premature User/Character/Place panels | The web surface is only a World, Entity and Activity/prose ledger |
| `.agents/backlog/items/agent-mediated-world-action.md` | The first private proposal/preview/confirmation and atomic World action is complete | Make the existing loop ordinarily playable instead of extending gameplay again |
| User choices in this design | One hidden seeded User, persistent storage, minimal design, maximal visible authoritative data and no web conversation | Optimize the slice for immediate local play and truthful observation |

## Alignment

### Strategic

The build converts Aicadia from a proven server workflow into a game the User can
actually revisit. The strategic value is not a dashboard: it is the short feedback
loop between Agent conversation, authoritative World acceptance and visible durable
consequences. That loop is needed before judging onboarding quality, discovery
prompts or later world expansion.

It deliberately evolves the current build contract only at the product boundary:
local operation becomes supported and Character creation gains a required private
Agent workshop. World authority, the thirteen-capability surface and all durable
models stay fixed. T1 narrowly corrects the always-on “web app deferred” rule so it
admits this one read-only local ledger but no browser gameplay or general web-client
scope. After this build, the next concrete game risk is whether a User
finds Agent-led onboarding and early World exploration understandable and
compelling—not whether the server can store another speculative concept.

### Tactical

The smallest complete player flow is:

1. The User runs one local launcher from the repository. It ensures the dedicated
   local database exists, starts the existing server and reuses one private local
   profile. On the first run only, it provisions one User after migrations complete.
2. The launcher opens the read-only ledger and prints the exact command that starts
   a Codex CLI conversation with `AICADIA_USER_ID` scoped to that Agent process. It
   never starts Codex itself.
3. The Agent calls `get_character`. If no Character exists, it privately presents
   exactly three concrete candidates, each with the name and description World would
   receive. The User selects one and may steer it.
4. The Agent shows the resulting exact name and description and waits for an explicit
   confirmation. Only then does it call `create_character` once. Candidates, steering
   and confirmation are not sent to World and are not stored.
5. The Agent follows the existing entry flow unchanged: if the confirmed Character
   is unplaced it calls `enter_world`; only the existing `entry_place_not_found`
   genesis path permits one `create_entry_place`, followed by another `enter_world`.
   This build adds no second proposal ceremony for the entry Place.
6. The User continues playing in the Agent with the already proven action workshop.
   Every accepted command still passes through MCP to World and writes its existing
   state plus immutable Activity/prose in one transaction.
7. The browser initially reads World and shared Entity state. Before Character
   onboarding, personal Activity is an honest empty/unavailable state. After accepted
   Character actions, switching back to or refreshing the page shows the durable
   records. Restarting the launcher returns to the same records and User context.

The browser has exactly three information regions on one direct page:

- a compact World name and connection state;
- a shared Entity ledger, newest first, with lazy expansion to the existing
  `get_entity` fields except the deliberately hidden introducing User UUID; and
- accepted Activity/prose for the local Character, newest first, including actor,
  context Place, involved Entity roles, operation, time and canonical prose when
  present.

Both ledgers use the existing typed cursors and expose `Load older` only when a next
page exists. Initial load, explicit Refresh and returning focus to the page re-read
state; there is no background polling. Character and Place may appear as typed
references inside records, but neither becomes a separate page or panel. The hidden
User id is transport context and is never rendered.

### Technical

**World, PostgreSQL and history.** No `World` method, schema or migration changes.
The launcher uses a fixed validated local database name, `aicadia_local` by default,
and never drops it. Existing migrations run on every server start. Existing World
transactions remain the sole state writers and existing Activity rows remain the
complete historical footprint.

**Local profile.** `.aicadia-local/profile.json` is ignored by Git. Its directory is
mode `0700`, the file is written atomically with mode `0600`, and it contains only a
version, the selected database name and the stable User UUID—never database
credentials or Agent conversation. On restart, the launcher verifies both database
identity and `/api/user`; a missing or mismatched profile fails visibly rather than
silently provisioning another User.

**Launcher.** `tools/aicadia-local` is one Bash entry point. It validates required
local programs and the database name, uses `DATABASE_URL` when supplied and the
documented local default otherwise, ensures the database exists without a drop path,
builds the existing server/provisioner binaries, starts the server on loopback, waits
for its structured ready event, provisions or verifies the User, opens the ledger
unless `--no-open` is supplied, prints the ledger URL and prints:

```text
AICADIA_USER_ID='<stable-uuid>' codex
```

The command is run from the trusted project root so the existing project-scoped MCP
configuration applies. Ctrl-C stops only the owned server process. The database and
profile remain. The launcher never invokes `codex`, an OpenAI API or any model.

**Browser context.** The launcher appends the User UUID in the URL fragment, which is
not sent in the HTTP request. The page copies it into `sessionStorage`, immediately
removes the fragment from the visible URL and uses it only as the
`Aicadia-User-Id` header for the contextual Activity read. The profile remains the
source of truth; the fragment is neither login nor authorization.

**Web asset.** `web/index.html` contains the complete semantic HTML, CSS and
JavaScript. `src/server.rs` serves it at `/` with `include_str!`, so there is no file
server, runtime asset lookup or frontend build. The JavaScript issues only GETs to
`/api/world`, `/api/entity`, `/api/entity/{id}` and `/api/activity`. It handles
`character_not_found` as pre-onboarding state and renders all other canonical errors
without inventing fallback data. It cannot call any mutating route.

**Visual contract.** One white/neutral direct surface, dark readable text, one muted
green connection accent, horizontal rules and data tables/description lists. No
cards, shadows, decorative icons, metrics, gradients or dashboard chrome. UUIDs,
timestamps and raw structured values use the system monospace stack. Desktop tables
remain dense; narrow screens preserve the complete data with controlled horizontal
overflow or stacked labelled rows. Native system fonts and CSS avoid a network or
asset dependency.

**Agent contract.** The first, self-contained part of `MCP_INSTRUCTIONS`, the
`create_character` tool description and `docs/game/agent-interface.md` specify the
three-candidate, steering, exact-preview and confirmation sequence. They also say
that World cannot prove the private interaction and that only the confirmed
`create_character` call is durable. The existing action workshop remains intact.
Catalog fixture and server assertions change with those descriptions; the public
operation count remains exactly thirteen.

**Concurrency and idempotency.** No new gameplay concurrency seam. Existing
`create_character`, entry and action behavior continue to handle duplicate and
concurrent attempts. Local database creation is single-machine startup plumbing; an
existing database is reused and a database-creation race must converge on that same
validated name or fail without changing the profile.

**Errors.** Operational failures use explicit stderr plus non-zero exit: unavailable
Postgres, invalid database name, missing dependency, occupied port, server readiness
timeout, corrupt profile, missing profiled User or unexpected database identity. The
page distinguishes disconnected API, no Character yet and valid empty ledgers. It
never turns an error into an empty successful record set.

## Decisions, assumptions and open questions

### Confirmed decisions

- Conversation, proposals, steering and confirmation exist only in the User's Agent;
  the web page is permanently read-only for this slice—User choice and concept log.
- The first console proves World persistence through World, Entity and Activity/prose
  data only; a visible User/Character/Place dashboard is not earned—User correction
  and concept log.
- One seeded development User is hidden plumbing, not an account or authentication
  model—current build contract and User choice.
- Character onboarding uses exactly three transient Agent candidates and one exact
  confirmed `create_character`; World stores only the accepted result—current private
  workshop pattern applied to the accepted onboarding direction.
- Entry after Character creation keeps the existing no-id flow and does not gain a
  separate three-choice workshop—KISS boundary of this plan.
- The launcher reuses a persistent database and stable profile and has no destructive
  database path—required return-and-remember outcome.
- The web page reuses existing reads and creates no UI-only projection or
  capability—Agent capability parity and Dumb And Strict Server rules.
- No automated model call is part of build or validation—No Unconscious Token Burn.

### Reversible assumptions

- Codex CLI is the first documented Agent surface because an environment-scoped
  `AICADIA_USER_ID` and the existing project MCP config make it reproducible in one
  command. The app and IDE share the config but may require environment setup and a
  restart; support can be added later without changing World behavior.
- PostgreSQL client tooling is present in the local development environment. T2
  verifies this before any write and documents `DATABASE_URL`; if absent, it stops
  instead of adding a database-management abstraction.
- Port `3000` and database name `aicadia_local` are the local defaults. Existing
  `AICADIA_PORT` and a narrowly validated test override may change them without
  altering the player contract.
- The newest 100 rows per first page are enough for immediate load; existing cursors
  and `Load older` preserve full traversal when the local World grows.

### Open questions

- None material. Actual Character-candidate quality and the felt onboarding flow are
  deliberately the first User playtest after this build, not a reason to add more
  server or web behavior beforehand.

## Implementation map

| Surface | Current state | Intended change | Invariants |
| --- | --- | --- | --- |
| `AGENTS.md` | All web-app work is deferred | After acceptance, narrowly permit one read-only local World ledger while continuing to defer web gameplay, authentication and general web-app scope | Compact always-on rule; no UI capability bypass |
| `docs/game/README.md` | Auth and web UI are deferred; persistent World behavior is authoritative | Define supported local-play boundary, hidden development context and read-only ledger | No web mutation, server inference or new game capability |
| `docs/game/agent-interface.md` | Missing Character is created immediately | Require three private candidates, steering, exact preview and confirmation before `create_character` | Proposals remain private; thirteen operations unchanged |
| `docs/game/local-play.md` | Absent | Add one operational start/connect/stop/restart guide with exact commands and boundaries | No duplicate game contract; no implied auth or token automation |
| `tools/aicadia-local` | Absent | Compose persistent DB, server, stable User profile, ledger URL and Agent command | Loopback only; no database drop; no Agent/model invocation |
| `.aicadia-local/profile.json`, `.gitignore` | No local profile path | Ignore and safely manage one versioned local identity file | Stable UUID; private permissions; no credentials |
| `web/index.html` | Absent | Add one self-contained read-only responsive World/Entity/Activity ledger | GET-only; no User id display; no chat or game controls |
| `src/server.rs::app` | No root route; existing thirteen APIs and MCP | Serve embedded HTML at `/`; refine Character-onboarding MCP instructions and tool description | OpenAPI and capability count stay thirteen; adapters stay thin |
| `tests/server.rs` | HTTP/MCP parity and catalog behavior covered | Assert root asset, GET-only boundary, pre-Character handling contract and revised MCP instruction/catalog parity | Existing API/action tests stay green |
| `tests/agent-tool-catalog.json` | Exact generated thirteen-tool fixture | Regenerate only changed Character description/instructions when required | No operation/schema drift |
| `tests/aicadia-local.sh` | Absent | Prove first provision, restart reuse, fail-closed profile handling, non-destructive stop and printed handoff | Isolated test database/state; cleanup belongs to test harness only |
| `.agents/backlog/items/local-agent-play-ledger.md` | Absent | Track this as the one current edge | Planning state only; links to contract and plan |
| `docs/concept/log/log.md` | Active design records confirmed boundary | Link the formal plan and later record acceptance/completion corrections | Full choice recorded once; no stale “awaiting” state |

## Execution contract

The dedicated orchestrator task owns outcome, scope, plan state, integration and the
final evidence claim. The User explicitly authorized that orchestrator to run as
`gpt-5.6-sol` with `xhigh` reasoning and to delegate dependency-ready plan tasks to
`gpt-5.6-sol` sub-Agents with `high` reasoning. Each delegated Agent receives this
plan path and one task id, re-reads the live repository, changes only its owned
surfaces, runs focused evidence and returns raw results. Tasks sharing
`src/server.rs` or `tests/server.rs` remain sequential; the orchestrator reviews and
integrates every result before advancing the live task state.

## Task graph

Allowed states are `pending`, `in_progress`, `completed` and `blocked`.

| ID | State | Depends | Parallel-safe | Objective | Owned surfaces | Evidence |
| --- | --- | --- | --- | --- | --- | --- |
| T1 | completed | — | no | Accept and publish the exact local-play and Agent-onboarding contract | `AGENTS.md`, `docs/game/README.md`, `docs/game/agent-interface.md`, `docs/game/local-play.md`, concept log, backlog | Authority review finds no web mutation, extra capability or stale boundary |
| T2 | completed | T1 | no | One command starts/restarts the same local World and stable hidden User | `tools/aicadia-local`, `.gitignore`, `tests/aicadia-local.sh` | Isolated first-start/restart/failure lifecycle passes without production drop or Codex call |
| T3 | completed | T2 | no | The server exposes one self-contained read-only ledger over existing GET APIs | `web/index.html`, root route and focused `tests/server.rs` changes | Server tests and browser inspection prove data, empty/error states, responsive layout and no mutation surface |
| T4 | completed | T3 | no | A connected Agent receives the exact Character workshop before existing World entry | MCP instructions/tool metadata in `src/server.rs`, `tests/server.rs`, `tests/agent-tool-catalog.json` | Exact thirteen-tool catalog and initialization tests pass with action workshop intact |
| T5 | completed | T4 | no | Prove the complete local persistence and inspection outcome, then make it usable by the User | integration evidence plus final docs/plan/backlog state | Create accepted state without a model, inspect it in browser, restart, verify identical ids/history, run full integrity ladder |

### Task timestamps

| ID | Started at | Completed at | Reopened at |
| --- | --- | --- | --- |
| T1 | 2026-08-12T09:29:06+02:00 | 2026-08-12T09:36:11+02:00 | — |
| T2 | 2026-08-12T09:36:11+02:00 | 2026-08-12T10:41:54+02:00 | 2026-08-12T10:37:34+02:00 |
| T3 | 2026-08-12T09:46:56+02:00 | 2026-08-12T10:19:21+02:00 | — |
| T4 | 2026-08-12T10:19:21+02:00 | 2026-08-12T10:25:47+02:00 | — |
| T5 | 2026-08-12T10:25:47+02:00 | 2026-08-12T11:03:42+02:00 | — |

## Task details

### T1 — Freeze the playable-local contract

**Objective:** The current authorities say exactly what the launcher, Agent and
ledger do before code changes begin.

**Actions:**

1. Narrowly amend the always-on web deferral in `AGENTS.md`: this local read-only
   ledger is accepted, while browser gameplay, authentication and general web-app
   scope remain deferred.
2. Promote the confirmed design from concept history into `docs/game/README.md` and
   `docs/game/agent-interface.md` without changing World semantics.
3. Add one compact `docs/game/local-play.md` for the actual start, Agent handoff,
   stop, restart and reset boundary; do not duplicate the full API contract.
4. Align this plan and the one `Now` backlog item after explicit acceptance.

**Invariants:**

- “Web is read-only; Agent is the conversation” is an absolute boundary.
- A development User header is context, never authentication.
- The Agent workshop is private; accepted World calls alone leave Activity.
- No current gameplay operation or database model is added.

**Evidence:**

- focused text search for web mutation, chat, auth and capability claims;
- manual cross-read of `README.md`, `agent-interface.md`, `local-play.md`, plan and
  backlog—one current contract with no contradictory status.

**Stop conditions:**

- Stop and return to the User if implementing the desired onboarding would require a
  new World mutation, durable session, browser action or visible User/Character/Place
  model.

### T2 — Build the non-destructive local runtime

**Objective:** First start provisions exactly one User; every later start reuses the
same database and User while Ctrl-C only stops the server.

**Actions:**

1. Implement the validated Bash launcher and private versioned profile.
2. Compose the existing server and provisioner; do not duplicate World provisioning
   in shell or SQL.
3. Print the exact browser URL, MCP URL and environment-scoped Codex command without
   executing any Agent.
4. Add isolated lifecycle tests for first start, restart, corrupt/stale profile,
   occupied port, process cleanup and absence of a database-drop path.

**Invariants:**

- Profile writes are atomic and private; credentials are never persisted.
- Database and User identity remain stable across normal stop/restart.
- Failure cannot silently create a second User or point the profile at another
  database.
- Tests may clean up an explicitly owned disposable test database; the shipped
  launcher cannot drop the local World.

**Evidence:**

- `/bin/bash -n tools/aicadia-local tests/aicadia-local.sh`;
- `/bin/bash tests/aicadia-local.sh` against an isolated test database—same User id
  after restart, non-zero failures for bad profile/port and no `codex` invocation;
- permissions and ignored-profile assertions.

**Stop conditions:**

- Stop if a safe persistent database cannot be selected without parsing or storing
  credentials, or if existing provisioning cannot be reused after migrations.

### T3 — Expose the read-only World ledger

**Objective:** One browser page truthfully renders existing persistent data without
becoming another game client.

**Actions:**

1. Add semantic single-page markup, direct CSS and GET-only JavaScript in one asset.
2. Serve it from `/` through the existing loopback router.
3. Implement World state, paged Entity summaries with lazy full details, paged
   Activity/prose, explicit refresh/focus refresh and truthful loading/empty/error
   states.
4. Add server tests for the route, embedded asset boundary and pre-Character
   `character_not_found` behavior.
5. Run the design and browser-control workflows for desktop and narrow-screen visual
   inspection; fix clipping, hierarchy, density and keyboard interaction.

**Invariants:**

- Every network call from the page is GET; no mutating URL or form control exists.
- No User id is visible; no dedicated Character or Place state is invented.
- Entity detail comes from `get_entity`; Activity/prose comes from `list_activity`.
- The page remains useful before and after Character onboarding.

**Evidence:**

- focused server tests for `/`, content type and existing API responses;
- static boundary assertion covering no form/composer and no POST/PUT/PATCH/DELETE;
- browser screenshots/inspection at desktop and mobile widths, keyboard expansion and
  actual initial/refresh/load-older flows.

**Stop conditions:**

- Stop if the page needs a new composite endpoint, projection or web-only domain
  state; simplify the presentation instead.

### T4 — Publish Character onboarding to the Agent

**Objective:** A fresh connected Agent is explicitly instructed to workshop and
confirm a Character before one existing creation call.

**Actions:**

1. Put the complete first-use workshop early in MCP initialization instructions.
2. Align `create_character` metadata and exact Agent documentation.
3. Update the generated catalog fixture only for intentional text changes and keep
   all current schemas and operations fixed.
4. Preserve the already proven next-action workshop verbatim in meaning.

**Invariants:**

- Exactly three candidates, optional steering, exact final preview and explicit
  confirmation happen before `create_character`.
- The Agent never asks for or invents User, Character or Place ids.
- The server does not claim it can validate private conversation.
- Tool count remains thirteen; World remains the only writer.

**Evidence:**

- `DATABASE_URL=postgres://localhost:5433/postgres cargo test --test server catalog`;
- focused MCP initialization and HTTP/MCP parity tests;
- exact review that current action instructions and schemas did not regress.

**Stop conditions:**

- Stop if catalog generation changes schemas/operations or the onboarding wording
  implies proposals are World state.

### T5 — Prove persistence and hand off real play

**Objective:** The built slice demonstrably survives restart and is ready for the
User's first real Agent conversation.

**Actions:**

1. Start an isolated local World through the launcher and record the stable User id.
2. Without a model call, drive the existing public World operations to establish a
   Character, entry and one accepted action; inspect the resulting Entity and
   Activity/prose through the real browser page.
3. Stop and restart the launcher, then prove the same User, Entity ids, placement,
   Activity ids and prose remain visible.
4. Verify the printed Codex command, project MCP config and exact thirteen-tool
   initialization path without automatically starting a model conversation.
5. Align completion state in contract, concept log, backlog and this plan. Give the
   User the two exact commands and one suggested first prompt for their conscious
   live playtest.

**Invariants:**

- Deterministic evidence must not be relabelled as proof of subjective Agent quality.
- No paid/live model call occurs without separate explicit User action.
- Test cleanup removes only its owned disposable evidence; the ordinary local World
  remains persistent.

**Evidence:**

- before/after restart comparison of canonical HTTP data and browser rendering;
- `cargo fmt --check`;
- `cargo clippy --all-targets --all-features -- -D warnings`;
- full `DATABASE_URL=postgres://localhost:5433/postgres cargo test`;
- `/bin/bash -n tools/aicadia-local tests/aicadia-local.sh` and full launcher suite;
- `git diff --check`, focused diff review, no unintended migration/Cargo/API/tool
  changes and no unrelated user changes overwritten.

**Stop conditions:**

- Stop and report the exact boundary if persistence, stable identity, read-only
  inspection or MCP handoff cannot all be proven in one continuous local setup.

## Execution result

The deterministic completion setup used one owned disposable World. Across a normal
launcher stop/restart, User `172b5c83-ea21-45af-8da3-31dace3201af`, Character
`e28a709e-2555-4691-8df1-b287dd5bc9e8`, Place
`0976bb79-2971-4778-afa4-409065638537`, Entity
`adbb6fbd-941c-411c-8444-f7c052bfdf7d`, all four Activity ids and the exact prose
`Mara fixes a brass local ledger marker beside the North Gate.` remained identical.
Canonical before/after HTTP JSON compared byte-for-byte equal. A fresh browser page
rendered the same visible ids and prose, immediately removed the User fragment, hid
the User UUID and exposed no form or viewport overflow. The lifecycle suite also
proved missing-profile and concurrent-launch attempts fail closed with one User and
one listener. The exact thirteen-tool catalog and both Character/action workshops
remained intact. Formatting, strict Clippy, all 59 Rust tests, Bash syntax and full
launcher lifecycle, `git diff --check`, desktop/mobile/pagination/keyboard browser
checks and independent risky-seam review passed. The owned disposable database,
state, listener and tabs were removed. No model-driven gameplay run occurred.

The honest remaining boundary is subjective Agent onboarding quality and the User's
first real conversation. Public domain hosting is also outside this local plan: it
needs a separately accepted read-only exposure and production-operation contract.

## Validation ladder

1. **Focused:** Bash lifecycle tests, root-page server tests, MCP
   instruction/catalog tests and desktop/mobile browser inspection.
2. **Contract:** Exact thirteen capabilities, no World/schema/migration change,
   Agent-only conversation, GET-only browser and stable private User context across
   restart.
3. **Outcome:** One accepted Character/entry/action state is visible in the ledger,
   survives server restart with identical canonical ids/history, and the launcher
   prints a usable but unexecuted Agent command.
4. **Integrity:** `git diff --check`, strict Clippy, full tests, focused diff review,
   no accidental production database drop path and confirmation that unrelated dirty
   worktree changes remain intact.

## Change control

Refine paths, task order and stronger evidence in place while the accepted outcome
and contract remain unchanged. Stop implementation, set `status: draft`, revise and
request explicit re-acceptance when new evidence changes the Agent-only conversation
boundary, visible domain scope, public operations, durable identity, database
lifecycle, token-spend boundary or evidence claim.

## Completion conditions

- every required task is `completed` and the validation ladder passes;
- one persistent local World and hidden development User restart without identity or
  history loss;
- Character onboarding is published as an Agent-only three-candidate, steering,
  preview and confirmation flow before the existing World calls;
- the browser is demonstrably read-only and shows traversable Entity record detail
  (with User UUIDs deliberately hidden) plus complete visible Activity/prose data
  without dedicated User/Character/Place dashboards;
- the User receives an exact conscious Agent-start command and can begin the real
  onboarding experiment immediately;
- current behavior, concept choices, vocabulary and backlog are aligned;
- no known-stale authority, material open question or accidental unrelated change
  remains;
- `status: complete` and `completed_at` are recorded only after these conditions.
