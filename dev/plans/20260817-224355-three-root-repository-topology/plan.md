---
status: complete
created_at: "2026-08-17T22:43:55+02:00"
updated_at: "2026-08-17T23:24:41+02:00"
accepted_at: "2026-08-17T22:48:53+02:00"
completed_at: "2026-08-17T23:24:41+02:00"
---

# Put Aicadia's game, development system and Studio in three explicit roots

> **Role / side:** proportional repository-topology build plan / development side.
> **Authority:** owns the bounded execution state for moving the current repository into `game/`, `dev/` and `studio/` while preserving its game, MCP, Studio and development behavior.
> **Excludes:** new game behavior, a development MCP implementation and historical compatibility paths; the final authorities will live in `game/docs/`, `dev/docs/`, `dev/backlog/` and `dev/plans/`.

## Outcome

The repository has three obvious owned roots:

```text
/
├── game/       # the shipped World, HTTP and player MCP surface
├── dev/        # plans, backlog, skills, research, evidence, lab and playtests
└── studio/     # the local development application and its web presentation
```

A fresh builder can tell which side owns a file from its path. `game/` is a Rust
package containing the World and its public HTTP/MCP adapters, migrations, game
contracts, player Agent text, game tests and the player-side launcher. `dev/` is a
Rust package plus development workspace containing all current development records,
skills, experiments, playtest runners and their local state. `studio/` is a Rust
package containing the local Studio application, pages, assets, tests and Studio
launcher. The repository root is only the Cargo workspace and the thin conventions
that clients require at a repository root.

The current dirty worktree is the input to the move: all completed Studio-plan
changes and untracked Studio source files are moved, not discarded or reconstructed
from `HEAD`. Tracked and ignored playtest state is moved byte-for-byte, including
permissions. No database, playtest evidence or local profile is reset.

The exact evidence claim is: the final worktree has no current source-of-truth
content left in the former `src/`, `docs/`, `tests/`, `tools/`, `web/`, `lab/`,
`.agents/backlog/` or `.agents/plans/` homes; the root Cargo workspace builds all
three packages; all database-backed and database-free tests pass; the combined
`aicadia` process starts through `cargo dev`; its Studio pages respond; `/mcp`
serves the current stateless discovery contract and publishes exactly the same 15
player tools as before; OpenAPI,
compiled Agent instructions and the checked-in catalog still agree; and the
player-side adapter can connect without receiving any Studio or development tool.

## Non-goals

- No new player capability, MCP tool, HTTP route, World behavior, schema, migration,
  stored state, ownership rule or Activity behavior.
- No development MCP server yet. `dev/mcp/` is added only when a concrete tool is
  designed and implemented; an empty placeholder would violate Earn Your Spot.
- No Studio feature or visual redesign beyond the path and package changes required
  to keep the just-completed Studio implementation working.
- No compatibility aliases for the old project paths. Current code, docs, scripts
  and historical records are rewritten to their new paths because the project is
  still wholly under development.
- No deletion, regeneration or cleanup of playtest data, database data, local
  profiles, screenshots, evidence or completed plans.
- No paid Agent/model call, new dependency service, commit, push or deployment.

## Evidence and authorities

| Evidence or authority | Current fact | Consequence for this build |
| --- | --- | --- |
| User direction, 2026-08-17 | Existing code must be divided into `game/`, `dev/` and `studio/`; playtest folders belong under `dev/`; work is complete only when compilation, game MCP and Studio start are proven. | This plan performs a physical move and full path rewrite, not just a new documentation convention. |
| User direction in the preceding structure discussion | `.agents` is not an Aicadia content home; skills should have one canonical home while Codex and Claude Code retain their required discovery paths. | Canonical skills move to `dev/skills/`; root `.agents/skills/` and `.claude/skills/` contain only relative discovery symlinks. |
| Current dirty worktree | The completed Studio plan is implemented but not committed; 49 tracked files differ and new Studio source/test files are untracked. | The live filesystem, not `HEAD`, is moved. Before/after manifests prove that no dirty or untracked file is lost. |
| `cargo brief`, 2026-08-17 | The Studio plan is complete, docs lint is clean and the compiled player catalog has 15 capabilities; no new game edge is active. | This reorganization is the selected development edge and may not change the game contract or tool set. |
| `cargo test --workspace --all-targets`, 2026-08-17 | The repository compiles and 86 tests pass; 32 database tests stop only because `DATABASE_URL` is absent. | Source compilation is a known baseline. Final evidence must additionally provide a disposable PostgreSQL URL and pass the database suite. |
| `Cargo.toml`, `.cargo/config.toml` | One package currently owns game, Studio and development binaries; `cargo dev` and `cargo brief` are established entry points. | The root becomes a virtual workspace, but those user-facing commands and the `aicadia` binary name remain stable. |
| `game/src/server/mcp.rs`, `game/src/server/http.rs`, `game/src/world/` | MCP and HTTP are thin adapters over the same `World`; the MCP lives at `/mcp`. | They move together into the game package; the adapter is not made dependent on Studio or dev. |
| `studio/src/`, `web/`, `tests/studio/` | Studio is read-only development presentation over repository and PostgreSQL state. | It moves to the Studio package and depends on the game package only through explicit read-only compiled-surface functions and World types. |
| `dev/.playtest/entity-state/`, `dev/.playtest/agent/`, `dev/.playtest/trait/`, `dev/.local/` | One playtest root is tracked; the others and the local profile are ignored; scripts hardcode the current root paths. | They move under `dev/.playtest/` and `dev/.local/`; scripts, ignores and evidence references change together, with file hashes and modes checked. |
| `AGENTS.md` — Current Means Current, One Home Per Truth, Flat Over Clever, Earn Your Spot | Superseded current paths must be removed; each truth has one home; no speculative empty surface. | The old homes disappear, root adapters never duplicate skill content, and no empty `dev/mcp/` is created. |

## Alignment

### Strategic

The current blocker is structural ambiguity: game runtime, development machinery
and the Studio application are interleaved in one package and several generic root
folders. That makes the intended separation between player MCP tools and future
builder MCP tools depend on memory. The three-root topology makes that boundary a
filesystem and Cargo dependency boundary before any development MCP is introduced.
It does not advance World behavior directly, but it retires the concrete ownership
risk the User selected before subsequent capabilities and tooling are built.

### Tactical

The actor is a local builder. They run root commands (`cargo dev`, `cargo brief`,
`cargo test --workspace --all-targets`) while Cargo selects packages underneath.
The running `aicadia` binary composes exactly two runtime surfaces: the game router,
including `/mcp`, and the read-only Studio router. Development playtest runners are
explicit shell or Rust commands and never join the game server. A future dev MCP can
therefore be placed in `dev/` and configured separately without ever entering the
player catalog.

Allowed: `game/src/server/mcp.rs` registers a player capability and its contract is
stored under `game/mcp/`. Rejected: a Studio or development tool is registered in
that router. Allowed: a future `dev/mcp/` package is launched only by developer
client configuration. Rejected: the combined game process auto-starts it.

No accepted World state changes in this build, so no new Activity footprint applies.

### Technical

- Root `Cargo.toml` becomes a virtual workspace with members `game`, `dev` and
  `studio`, resolver 3 and shared dependency/version declarations. Root
  `.cargo/config.toml` keeps `cargo dev` and `cargo brief` aliases.
- `game` is package `aicadia-game` with library name `aicadia`, preserving existing
  game integration-test imports. It owns World, wire contracts, HTTP, MCP, Agent
  contract assembly, migrations, the provisioning binary and game tests.
- `studio` is package `aicadia-studio`. It depends on the game library and owns the
  `aicadia`, `aicadia-dev` and `aicadia-brief` binaries. The `aicadia` binary remains
  the only workspace binary with that name and composes game and Studio routers.
- `dev` is package `aicadia-dev-tools` and initially owns the existing
  `aicadia-playtest-database` binary. It is not linked into the running game.
- Studio's required compiled projections become the smallest intentional public
  game-library seams: the MCP catalog, OpenAPI document and assembled Agent
  instructions. They are read-only values; World behavior and player protocol do
  not change.
- The static player-facing Markdown parts and the checked catalog move to
  `game/mcp/`; the Rust adapter stays conventional Rust source under
  `game/src/server/`. Thus all player MCP assets are beneath `game/` without adding
  a second crate or path-attribute trick.
- Studio and launcher repository-root discovery changes from `CARGO_MANIFEST_DIR`
  to its parent workspace root. All file discovery and lint roots point to
  `game/docs/`, `dev/docs/`, `dev/plans/`, `dev/backlog/` and `dev/lab/`.
- Shell launchers resolve the workspace root from their new physical locations.
  Player connection state lives in `dev/.local/`, but `game/tools/aicadia-agent`
  remains the sole player-side adapter and exposes only the game MCP endpoint.
- No game query, lock, transaction, index or correctness state changes. Millions of
  Users and one hot Entity therefore have exactly the same transaction and
  contention behavior before and after this build.

## Decisions, assumptions and open questions

### Confirmed decisions

- The three canonical roots are exactly `game/`, `dev/` and `studio/` — User choice.
- Playtest programs, schemas, evidence state and local playtest output belong under
  `dev/` — User choice.
- Current project paths do not need historical compatibility aliases — User choice.
- Player MCP and development MCP are separate runtime/configuration concerns; only
  the existing player MCP is in scope now — preceding discussion and User choice.
- One canonical skill lives at `dev/skills/build-aicadia`; client-specific root
  locations are discovery symlinks, not duplicate sources — preceding discussion.
- Root-only ecosystem files remain at root: `AGENTS.md`, `CLAUDE.md`, `Cargo.toml`,
  `Cargo.lock`, `.cargo/`, `.codex/`, `.gitignore`, `.agents/skills/` and
  `.claude/skills/`.
- Existing dirty and untracked Studio changes are user work and are preserved.

### Reversible assumptions

- Keep one three-member Cargo workspace rather than multiple independent workspaces;
  this preserves one lockfile and one validation command and can be split later.
- Keep the library name `aicadia` while naming its package `aicadia-game`; this
  avoids an unrelated source-wide domain import rename.
- Store development local connection state in `dev/.local/` alongside, but separate
  from, `dev/.playtest/`; both remain ignored except the currently tracked
  entity-state evidence, which remains tracked after its move.
- Move the current root screenshots to `studio/evidence/` because they demonstrate
  Studio presentation, not game truth.
- Mechanically update path references inside retained historical plans and logs.
  Their decisions and timestamps remain untouched; only now-invalid repository
  addresses change.

### Open questions

None. The User accepted the Cargo package boundary, public internal-workspace seams
and authoritative paths before implementation.

## Implementation map

| Current surface | Final home |
| --- | --- |
| `src/world/`, `src/wire/` | `game/src/world/`, `game/src/wire/` |
| `src/server/`, `src/agent_contract.rs` | `game/src/server/`, `game/src/agent_contract.rs` |
| `src/agent_contract/**/*.md`, `tests/agent-tool-catalog.json` | `game/mcp/agent/`, `game/mcp/tool-catalog.json` |
| `docs/game/` | `game/docs/` |
| `migration/` | `game/migration/` |
| `tests/world/`, game-only `tests/server/` | `game/tests/world/`, `game/tests/server/` |
| `src/bin/aicadia-provision-user.rs` | `game/src/bin/aicadia-provision-user.rs` |
| `tools/aicadia-agent` | `game/tools/aicadia-agent` |
| `.agents/backlog/`, `.agents/plans/` | `dev/backlog/`, `dev/plans/` |
| `.agents/skills/build-aicadia/` | `dev/skills/build-aicadia/` plus root Codex/Claude discovery symlinks |
| `CONTEXT.md` | `dev/CONTEXT.md` |
| `docs/README.md`, `docs/concept/`, `docs/research/`, `docs/evidence/` | `dev/docs/README.md`, `dev/docs/concept/`, `dev/docs/research/`, `dev/docs/evidence/` |
| `lab/` | `dev/lab/` |
| `tools/agent-playtest*`, `tools/trait-playtest*` | `dev/playtest/agent/`, `dev/playtest/trait/` |
| `tests/agent-playtest.sh`, `tests/trait-playtest.sh` | `dev/tests/` |
| `src/bin/aicadia-playtest-database.rs` | `dev/src/bin/aicadia-playtest-database.rs` |
| `.aicadia-entity-state-playtest/` | `dev/.playtest/entity-state/` |
| `.aicadia-playtest/`, `.aicadia-trait-playtest/` | `dev/.playtest/agent/`, `dev/.playtest/trait/` |
| `.aicadia-local/` | `dev/.local/` |
| `src/studio/` | `studio/src/` as the Studio library modules |
| `src/main.rs` | `studio/src/bin/aicadia.rs` |
| `src/bin/aicadia-dev.rs`, `src/bin/aicadia-brief.rs` | `studio/src/bin/` |
| `web/` | `studio/web/` |
| `tests/studio/`, Studio/composition parts of `tests/server/` | `studio/tests/` |
| `tests/aicadia-local.sh`, `tools/aicadia-local` | `studio/tests/aicadia-local.sh`, `studio/tools/aicadia-local` |
| root Studio screenshots | `studio/evidence/` |

After the move, `src/`, `docs/`, `tests/`, `tools/`, `web/`, `lab/`, the old
`.agents/backlog/` and `.agents/plans/` must be absent. This plan itself
continues at `dev/plans/20260817-224355-three-root-repository-topology/plan.md`.

## Execution contract

- Preserve the current dirty tree. Capture a sorted tracked/untracked manifest,
  `git diff --binary`, file modes and SHA-256 manifests for all four private roots
  before moving. Use moves and focused patches; never reset or check out user work.
- Convert the root package to the workspace and move one ownership slice at a time.
  Compile after each package boundary, so path failures remain localized.
- Do not delete an old home until `rg`, Git status and the relevant package tests
  prove every live file and reference has a final home.
- Keep secrets and local profile contents out of command output. Compare file names,
  modes, counts and hashes; do not print private file contents.
- The tracked/ignored status of every private file must remain the same after the
  path change, except for the intended pathname itself.
- Record the accepted topology in the active concept trail and current August log
  during implementation; update `AGENTS.md`, `CLAUDE.md`, documentation placement,
  skill instructions, Studio projection and every retained link in the same change.
- No task may change the 15-name player catalog. Any catalog, schema, migration,
  World behavior or player-instruction diff stops execution and requires a revised
  accepted plan.

## Task graph

| ID | State | Depends | Parallel-safe | Objective | Owned surfaces | Evidence |
| --- | --- | --- | --- | --- | --- | --- |
| T1 | completed | — | no | Snapshot the live tree and establish the root Cargo workspace plus canonical directory skeleton without losing dirty/private state. | root Cargo/config/ignore files; manifests; directory moves needed to establish packages | Before/after file and private-state manifests; `cargo metadata --no-deps`; `git diff --check` |
| T2 | completed | T1 | no | Move the complete game runtime and prove World, HTTP and player MCP behavior are unchanged. | `game/`; game portions of former `src/`, `tests/`, `game/docs/`, `migration/`, player launcher | `cargo test -p aicadia-game --all-targets`; exact catalog/OpenAPI/Agent-contract checks |
| T3 | completed | T2 | no | Move all development authorities, skills, lab, playtests and local state; replace root skill content with discovery adapters. | `dev/`; `.agents/skills/`; `.claude/skills/`; former development docs/records/scripts/state | dev package/tests; symlink resolution; playtest dry/help checks; pre/post private-state count/mode/hash equality |
| T4 | completed | T3 | no | Move Studio and composition binaries, update read-only game seams and make every Studio projection use the new authorities. | `studio/`; former Studio source/assets/tests/tools/screenshots; minimal game exports | `cargo test -p aicadia-studio --all-targets`; Studio route and composition tests |
| T5 | completed | T4 | no | Rewrite every current and retained historical repository path and crystallize the accepted topology in its proper authorities. | root constitutions; `dev/docs/`; `dev/backlog/`; `dev/plans/`; `dev/skills/`; Studio lint rules | `cargo brief`; docs-lint tests; old-path and broken-link sweeps; no duplicate skill source |
| T6 | completed | T5 | no | Run the full database-backed validation and directly prove the game MCP and Studio from one clean start. | whole workspace; disposable PostgreSQL database; local combined process | full validation ladder; direct MCP discovery/tools-list; Studio HTTP smoke; adapter isolation |

## Task details

### T1 — Preserve the live tree and establish the workspace

1. Record `git status`, tracked/untracked paths and a binary diff without modifying
   the worktree. Record counts, modes and SHA-256 manifests for the four private
   roots without printing their contents.
2. Change root `Cargo.toml` into a resolver-3 workspace with `game`, `dev` and
   `studio` members and shared package/dependency declarations. Keep the lockfile
   and aliases at root.
3. Create only directories that immediately receive current files. Move the live
   dirty and untracked files; do not reconstruct them from Git.
4. Add each member manifest with the package/binary identities in Technical.
5. Run `cargo metadata --no-deps`, `cargo fmt --all --check` and
   `git diff --check`. Resolve ownership/path errors before continuing.

### T2 — Isolate the game package and its MCP

1. Move World, wire, server, Agent-contract assembly, migrations, game docs, game
   tests, provisioning and player adapter according to the map.
2. Put the existing Agent Markdown and checked catalog under `game/mcp/`; update
   `include_str!`, fixtures, migration macros and launcher root calculation.
3. Split mixed server tests: game/router/MCP behavior stays in `game/tests`; only
   combined Studio composition checks move later to `studio/tests`.
4. Expose only the compiled read-only catalog/OpenAPI/instruction functions Studio
   needs. Do not expose mutable server state or a second route implementation.
5. Run all game tests with a disposable PostgreSQL database and compare the sorted
   tool names, schemas and instructions with the pre-move baseline.

### T3 — Move the development system and playtest state

1. Move backlog, all plans (including this one), skill, concept/research/evidence
   docs, placement constitution, vocabulary, lab and existing playtest programs.
2. Move tracked and ignored private roots to `dev/.playtest/` and `dev/.local/`.
   Update `.gitignore`, runners and evidence paths. Verify identical count, mode,
   size and SHA-256 manifests before considering the old roots removable.
3. Move the existing playtest-database binary into the dev package. Keep all runner
   inputs explicit and keep token-spending playtests uninvoked.
4. Replace `.agents/skills/build-aicadia` with a relative symlink to
   `dev/skills/build-aicadia`; add the equivalent `.claude/skills/build-aicadia`
   symlink. Verify both resolve to the same `SKILL.md` inode/content.
5. Update the skill's paths and run the existing playtest shell tests plus safe
   `--help`/validation modes. Do not call a model or reset state.

### T4 — Isolate Studio and restore combined composition

1. Move the complete current `studio/src/`, new untracked Studio pages/tests,
   `web/`, Studio tests, launcher, brief binary and screenshots into `studio/`.
2. Make Studio's library consume `aicadia` through the game package dependency.
   Make repository discovery start at the workspace root and use all final homes.
3. Keep the combined `aicadia` binary in Studio: construct one pool/World, merge
   `aicadia::server::app` with the read-only Studio app, and retain signal handling,
   loopback enforcement and launch behavior.
4. Update the local launcher to find the new binary, game player adapter and
   `dev/.local/` state without broadening what it passes to the player client.
5. Run all Studio, brief, page, launcher and combined-router tests.

### T5 — Align every repository authority and reference

1. Update root `AGENTS.md` and `CLAUDE.md`; update `dev/docs/README.md` home tables,
   the build skill, backlog and Studio projection/lint configuration to final paths.
2. Record the accepted three-root choice once in the active concept record/current
   August log and link to it elsewhere. Preserve the semantic content and dates of
   completed plans and evidence while rewriting obsolete path references.
3. Sweep all tracked text for the old authoritative prefixes and classify every
   occurrence. Permit only command outputs/examples that explicitly describe the
   former layout; fix every live link, command and include path.
4. Prove there is no canonical content under the root client adapters, no orphaned
   current file, no duplicate authority and no now-empty legacy root directory.
5. Run `cargo brief` and the shared documentation lint over the new roots.

### T6 — Prove the final system end to end

1. Provision one disposable PostgreSQL database without touching the developer's
   existing database/profile. Export its URL only for the bounded validation
   commands and remove it afterward.
2. Run formatting, metadata, all targets, all package/unit/integration/shell tests,
   docs lint and repository sweeps.
3. Start `cargo dev --no-open` from the repository root and wait for readiness.
   Verify `/`, `/game`, `/development`, `/live` and `/brief` return successful
   Studio responses.
4. Perform real stateless streamable-HTTP `server/discover` and `tools/list`
   requests against `/mcp`; assert exactly the 15 checked player tools and no
   Studio/dev tool. The current MCP 2026 contract deliberately creates no transport
   session and does not implement legacy `initialize`.
   Verify `/api/openapi.json` and the compiled catalog remain aligned.
5. Run the local adapter regression against the started process and prove it
   connects to `/mcp` using only the player surface. Stop the process and clean only
   the disposable database/runtime files created by this task.
6. Rerun the pre/post file and private-state manifests, update task states and mark
   this plan complete only when every completion condition holds.

## Validation ladder

1. `cargo metadata --no-deps` resolves exactly the `aicadia-game`,
   `aicadia-dev-tools` and `aicadia-studio` members.
2. `cargo fmt --all --check` and `git diff --check` pass.
3. `cargo test -p aicadia-game --all-targets` passes with disposable PostgreSQL.
4. `cargo test -p aicadia-dev-tools --all-targets` and the migrated safe shell
   regression tests pass without invoking an Agent/model.
5. `cargo test -p aicadia-studio --all-targets` passes with disposable PostgreSQL.
6. `cargo test --workspace --all-targets` passes with disposable PostgreSQL.
7. `cargo brief` succeeds and reports clean documentation over the final homes.
8. Tracked-reference, broken-link, include-path, old-home and duplicate-skill sweeps
   pass; all private-state hashes and modes match their pre-move values.
9. One root `cargo dev --no-open` start serves all required Studio pages and the
   exact existing MCP/OpenAPI capability set.
10. The player adapter reaches `/mcp` and can neither see nor invoke a Studio/dev
    tool.

## Completion result

- The root workspace resolves exactly `aicadia-game`, `aicadia-dev-tools` and
  `aicadia-studio`; formatting, strict Clippy, metadata and diff checks pass.
- `cargo test --workspace --all-targets` passed against disposable PostgreSQL with
  266 executed tests and one explicitly ignored catalog-fixture generator. Both
  token-free playtest suites passed without invoking Codex, a model or a server.
- A disposable `cargo dev --no-open` start served `/`, `/game`, `/development`,
  `/live` and `/brief`. Direct `server/discover` and `tools/list` returned the
  assembled Agent instructions and exactly 15 game tools; their names equal both
  `game/mcp/tool-catalog.json` and the 15 OpenAPI operation ids, with no Studio or
  development capability.
- The complete launcher/Agent-adapter lifecycle passed with `codex_invoked=false`.
  Its owned database, listener and runtime state were cleaned; the pre-existing
  port-3000 process was untouched.
- All 405 private files across the four moved roots retain their byte content,
  sizes and permissions. Ignored roots remain ignored and the 52 tracked
  entity-state files remain eligible for tracking at their new paths.

## Change control

Stop, revise this plan and request renewed acceptance if execution requires a game
schema or behavior change, a player catalog/instruction change, a fourth Cargo
package, a new dev MCP, deletion/reset of private data, a compatibility layer, an
external service, paid token spend or a materially different root ownership rule.
Ordinary compile fixes, relative-path corrections and visibility changes already
specified in this plan do not require replanning when they preserve the stated
interfaces and evidence claim.

## Completion conditions

- All current files have exactly one owned final home and the legacy content roots
  are absent.
- The dirty/untracked Studio implementation and every private playtest/local file
  are preserved; tracked/ignored status, content and permissions are accounted for.
- Root Cargo commands work; all workspace tests pass against disposable PostgreSQL.
- The combined application starts with `cargo dev`; Studio's five smoke routes and
  brief work from that process.
- Direct MCP `server/discover` and `tools/list` prove exactly the unchanged 15 game tools;
  no dev or Studio tool is present and OpenAPI/Agent contracts agree.
- Root `.agents` and `.claude` are thin client discovery surfaces only; canonical
  Aicadia development content lives under `dev/`.
- The accepted structure and path changes are recorded in the repository trail,
  this plan is `complete` at its `dev/plans/` path and no required work remains.
