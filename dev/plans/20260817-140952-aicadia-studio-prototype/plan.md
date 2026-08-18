---
status: complete
created_at: "2026-08-17T14:09:52+02:00"
updated_at: "2026-08-17T14:59:03+02:00"
accepted_at: "2026-08-17T14:33:14+02:00"
completed_at: "2026-08-17T14:59:03+02:00"
---

# Rust Aicadia Studio prototype on the existing local application

> **Role / side:** proportional prototype build plan / development side.
> **Authority:** owns the bounded implementation and evidence state for rebuilding the existing local ledger into the first Aicadia Studio comparison prototype.
> **Excludes:** game behavior, a final Studio layout and future multi-World domain/storage semantics; those remain governed by `game/docs/`, later User feedback and a later accepted production decision.

## Outcome

The developer runs the existing `cargo dev` command and the existing Rust local
application opens **Aicadia Studio** at `/`. The English, read-only interface has
`Game` and `Live` as its primary surfaces and offers three materially different
layouts on the existing route for comparison. `Game` projects current model meaning,
MCP tools, rules, open questions, experiments and evidence from their owning
repository sources. `Live` browses the one actual local World through bounded Entity,
Character, Place, Activity and storage-schema views.

Rust owns all source discovery, Markdown rendering, exact tool/schema assembly,
PostgreSQL reads, bounds and HTTP responses. The existing browser asset remains
HTML/CSS/JavaScript for presentation and variant switching only; no Python, Node,
separate application process or generated authoritative content is introduced. The
existing ledger is absorbed rather than retained as a second browser.

The exact evidence claim is: `cargo dev` still provisions and reuses one local World,
starts one Rust process and opens one Studio URL; all Studio reads are loopback-only,
GET-only and bounded; three selectable layouts display the same real source-backed
Game catalog and real local-World data; current game mutation, MCP and Agent behavior
remain unchanged. This does not select the final layout, prove complete documentation
coverage or introduce delivered multiple-World support.

## Non-goals

- No Python, Node server, frontend framework, WebAssembly application, second local
  process, lab implementation or static documentation generator.
- No schema, migration, game mutation, new player capability, MCP tool, published
  Agent-text change, background polling, authentication or browser gameplay.
- No final layout choice or automatic promotion of prototype rendering into settled
  production UI.
- No durable `WorldId`, `world` table, `world_id` foreign keys or fake second World.
  The UI labels the single actual local connection honestly.
- No unrestricted SQL console, table dump, repository file browser, code editor or
  terminal. Studio-only storage inspection is read-only, bounded and allowlisted.
- No manually duplicated model field list, tool catalog, rule text or evidence
  narrative in browser code.

## Evidence and authorities

| Evidence or authority | Current fact | Consequence for this build |
| --- | --- | --- |
| `docs/README.md` | Runtime/development homes and reference direction currently classify `web/` as the runtime ledger. | Deliberately reclassify the Studio module/assets as development-side projection while keeping game behavior independent of their meaning. |
| `game/docs/local-play.md` | `cargo dev` opens one narrow read-only ledger at `/`. | Evolve the accepted local development surface into read-only Studio and retain the no-chat/no-mutation boundary. |
| `src/bin/aicadia-dev.rs`, `studio/tools/aicadia-local`, `.cargo/config.toml` | `cargo dev` already launches the Rust-backed local application and opens `/`. | Preserve the command and single-process workflow; change the opened label/page, not the developer entrypoint. |
| `game/src/server/http.rs`, `web/index.html` | Rust currently embeds and serves the self-contained ledger while browser JS reads World/Entity/Activity HTTP data. | Reuse the existing route and data path; move Studio-specific Rust projection/read seams out of thin game adapters. |
| `game/docs/domain.md`, `game/docs/storage.md`, `migration/**` | Current model meaning and relational realization already have owning sources. | Rust projects the sources and live schema; UI authors no duplicate field/relationship truth. |
| `game/mcp/agent/tool/*.md`, `game/src/server/mcp.rs`, `/api/openapi.json` | Exact tool descriptions, registered tool catalog and public schemas already exist. | Studio exposes the exact assembled runtime catalog/schema through a read-only Studio projection rather than maintaining another list. |
| `dev/docs/concept/`, `lab/`, `dev/docs/evidence/`, `dev/backlog/` | Exploration, experiments, evidence and planning have distinct authorities. | Catalog keeps provenance/state visible and never presents these as current World rules. |
| Current `World` model | One pool-backed World named `Aicadia`; no durable World record/id exists. | Live shows one selected local connection and chooses no multi-World persistence model. |
| User correction, 2026-08-17 | Aicadia applications must be Rust; the existing Rust application that shows data may be rebuilt into Studio. | Supersede the rejected Python/lab plan and rebuild the existing local application in place. |
| `dev/plans/20260816-153410-multiplayer-lab/plan.md` | Separate multiplayer plan remains draft with unrelated dirty work. | Preserve all multiplayer/research changes and make no multiplayer decision. |

Governing exploration: `dev/docs/concept/aicadia-studio.md`. No backlog game edge is
claimed: this build retires a concrete development-orientation blocker and changes
only the supported local development surface.

## Alignment

### Strategic

Studio adds no player capability, but it lets the developer reliably understand the
current game, live World and uncertainty before choosing the next capability. That
directly reduces the present risk of building against stale, incomplete or wrongly
classified knowledge. Reusing the existing Rust local application avoids a second
tool and tests the interface against real World density. The next concrete risk is
whether one tested hierarchy actually improves game-development decisions.

### Tactical

One developer starts `cargo dev`, opens `/`, switches among three information
hierarchies with `?variant=`, and navigates `Game` or `Live`. All variants consume
one canonical Rust-produced read model. `Live` exposes bounded pages for actual
Entities, Character roles, Place roles and Activity plus allowlisted schema metadata;
`Game` exposes source-rendered model, capability, tool, rule and development-state
material. No request mutates World state, so no Activity footprint exists or is
required.

### Technical

Add one small `studio/src/` development-side Rust module, merged into the local Axum
application at startup. It receives a cloned `PgPool` for Studio-only repeatable,
read-only, bounded operator queries and a trusted compile-time repository root for
allowlisted source projection. It serves `/`, static Studio assets and explicit
`/studio/api/**` GET endpoints excluded from OpenAPI and MCP. Existing game HTTP/MCP
routes remain thin adapters over `World`.

Rust renders Markdown through one conventional Rust library only if required by the
first page; it never writes generated documentation. The frontend is split only as
needed for clarity (`web/index.html`, CSS and presentation JS) and contains no
canonical catalog data. Three full-page variants remain on the existing `/` route
until the User chooses; a floating switcher and shareable query parameter are
prototype-only. Every Studio list uses an explicit maximum and indexed ordering;
there is no all-World aggregate, full-table count or hot shared correctness row.

Real seams: repository sources, current compiled MCP/OpenAPI catalog, the actual
local PostgreSQL schema/data, current launcher and browser. Unproven seams: final UI,
remote hosting, auth, multiple Worlds and million-User operational throughput. The
bounded read design is scale-safe in shape but this local prototype makes no scale
measurement claim.

## Decisions, assumptions and open questions

### Confirmed decisions

- Name `Aicadia Studio`; English UI; `Game` and `Live` — User direction recorded in
  the Studio concept/log.
- Always use Rust for the application, parsing, catalog assembly, database access and
  server behavior; rebuild the existing local app instead of adding a Python/lab app
  — User correction 2026-08-17.
- Preserve `cargo dev` as the one command and `/` as the existing local browser
  location — smallest in-place continuation of current operation.
- Browser HTML/CSS/JavaScript remains presentation-only, matching the current app;
  no independent frontend/runtime framework is introduced.
- Project mechanically available structure and author semantic meaning once in the
  owning Markdown source; no committed generated catalog.
- Compare three structurally different layouts on the existing route before keeping
  one — prototype/UI workflow.

### Reversible assumptions

- A dedicated `studio/src/` module cleanly separates development projection from the
  `World` interface and thin game adapters; exact internal module splits may change.
- One conventional Rust Markdown renderer may be added if source excerpts cannot be
  made readable without it; this is implementation-only and earns no new authority.
- The first Game catalog may be intentionally bounded to the current overview,
  Entity/Property/Trait, exact MCP tools and leading development-state indexes; the
  architecture must permit later source types without hardcoded duplicate content.

### Open questions

- Which of the three layouts or combination becomes the retained Studio interface —
  intentionally resolved by this comparison, not before it.
- Whether future production Studio uses live source parsing, a Rust-generated
  rebuildable catalog or both — decide only after measured parsing/navigation pain.
- How durable multiple Worlds are identified and stored — future game-domain choice;
  no UI element may imply delivery now.

## Implementation map

| Surface | Current state | Intended change | Invariants |
| --- | --- | --- | --- |
| `studio/src/**`, `src/lib.rs`, `src/main.rs` | No Studio module; main starts only game server app. | Add Rust Studio catalog/read router and merge it into the same loopback process. | `World` behavior and game adapter semantics remain unchanged. |
| `game/src/server/http.rs` | Owns `/` ledger plus game HTTP/OpenAPI. | Remove browser ownership; retain only game/loopback data endpoints. | MCP/HTTP catalog and responses unchanged. |
| `web/**` | One self-contained ledger page. | Rebuild into source-free Studio presentation with three variants and optional clarity-driven asset split. | No canonical rules/models/tools copied into assets; GET-only. |
| `studio/tools/aicadia-local`, `src/bin/aicadia-dev.rs`, `.cargo/config.toml` | `cargo dev` launches Rust server and prints/opens Ledger URL. | Preserve command; print/open Studio URL and build required Rust binary. | Profile, database reuse, Agent command and cleanup unchanged. |
| `tests/server/**`, `studio/tests/aicadia-local.sh`, optional focused Studio test crate | Pin current ledger and launcher. | Move browser assertions to Studio, pin bounded read-only/catalog behavior and unchanged game catalog. | Existing game test meaning stays green. |
| `docs/README.md` | `web/` is runtime ledger; no Studio role. | Place `studio/src/` and `web/` as the local development Studio projection. | One home per truth and one-way game meaning remain intact. |
| `game/docs/local-play.md` | Supported local browser is narrow ledger. | Define `cargo dev` opening read-only Studio with Live absorbing ledger behavior. | Agent remains only gameplay conversation; no browser mutation/model. |
| `dev/docs/evidence/local-play.md`, completed local-ledger backlog item | Evidence/current pointer describes ledger. | Record the evolved local browser proof and point completed ledger scope to its Studio successor without duplicating status. | Evidence owns delivery detail; backlog remains planning state. |
| Studio concept/log and this plan | Prior draft selected rejected Python/lab seam. | Correct active concept, append correction to log, execute revised Rust plan after acceptance. | Append-only history preserves the rejected direction. |
| Schema, migrations, `game/docs/domain`, published Agent contract | Existing accepted game truth. | No change. | No new World identity, capability or mutation. |

## Execution contract

Root owns scope, integration, plan state and evidence. No delegation is required.
Preserve all unrelated dirty multiplayer/research surfaces; concept-log work is
append-only. Stop if implementation requires game mutation/schema behavior, an
external service, a non-Rust application layer or a second authority.

## Task graph

| ID | State | Depends | Parallel-safe | Objective | Owned surfaces | Evidence |
| --- | --- | --- | --- | --- | --- | --- |
| T1 | completed | — | no | Add the bounded source-backed Rust Studio projection and live operator reads. | `Cargo.toml`, `studio/src/**`, `src/lib.rs`, `src/main.rs`, narrow `game/src/server/**` seam, focused tests | Rust tests prove allowlisted sources, exact tool catalog, read-only bounded Entity/Character/Place/schema results and unchanged game routes. |
| T2 | completed | T1 | no | Rebuild the existing ledger route into three comparable Studio layouts. | `web/**`, launcher label/open behavior, browser/launcher tests | `cargo dev` page uses real T1 data; three variants are switchable and visually distinct on desktop/mobile. |
| T3 | completed | T2 | no | Align local-operation authority and exact delivery evidence. | `docs/README.md`, `game/docs/local-play.md`, `dev/docs/evidence/local-play.md`, local-ledger backlog item, Studio concept/log, plan state | Link/duplication review plus full validation ladder. |

## Task details

### T1 — Rust Studio projection

**Objective:** One loopback Rust process exposes a bounded, read-only Studio catalog
and actual local-World inspection without changing `World`, MCP or game HTTP meaning.

**Actions:**

1. Add a `studio/src/` router/state that owns trusted source roots and a cloned pool;
   merge it with the game router only in the local Rust application.
2. Project the selected current model/document/tool/development indexes from their
   owning files or compiled runtime catalog and render source content in Rust.
3. Add allowlisted, paginated Studio reads for Character roles, Place roles and
   storage schema; reuse existing bounded Entity/Activity reads where appropriate.
4. Keep every Studio endpoint GET-only, loopback-only, absent from MCP/OpenAPI and
   explicit about current/exploratory/experimental/evidence provenance.

**Invariants:**

- No game mutation, new game read capability or Agent-visible knowledge surface.
- No arbitrary filesystem path or SQL input; all sources/tables are allowlisted.
- No unbounded queries, totals, cross-World aggregation or process-local correctness
  state.

**Evidence:**

- Focused Rust tests exercise catalog provenance, exact tool count/content, hard
  pagination bounds, read-only Character/Place/schema results and invalid input.
- Existing server catalog/parity tests remain green and show no new MCP/OpenAPI path.

**Stop conditions:**

- Stop if catalog meaning must be manually duplicated or if Studio requires changing
  a game contract instead of projecting it.

### T2 — Existing-route UI comparison

**Objective:** `cargo dev` opens the Studio with three useful, radically different
layouts over the same real data.

**Actions:**

1. Replace the ledger presentation at `/` with a shared data-loading shell and three
   layouts: reference-first, model/workbench-first and live-World-first.
2. Preserve the hidden development User handling needed for personal Activity while
   adding `Game`/`Live`, provenance, Entity relations, tools and honest offline/error
   states.
3. Add shareable `?variant=` selection, keyboard cycling and a floating prototype
   switcher; annotate options for the ui.sh picker without making external loading a
   runtime requirement.
4. Update the launcher wording from Ledger URL to Studio URL without changing its
   database/profile/Agent lifecycle.

**Invariants:**

- No form, mutation, model invocation, automatic Agent launch or background polling.
- Browser assets contain presentation labels only, never copied rule/tool/model
  bodies or manually maintained schema lists.
- Desktop and mobile remain navigable; prototype controls are visibly non-product.

**Evidence:**

- Launcher regression test proves the same `cargo dev` command opens the Studio URL.
- Browser inspection and screenshots cover all variants at desktop/mobile widths,
  exact source-backed content and actual seeded World rows.
- HTML/source scan proves forbidden mutation surfaces remain absent.

**Stop conditions:**

- Stop before adding a frontend framework, Node toolchain, Python, WASM app or second
  process; revise and regain acceptance if one becomes necessary.

### T3 — Authority and evidence alignment

**Objective:** Current documentation describes exactly the new local Studio boundary
and the bounded proof delivered.

**Actions:**

1. Reclassify Studio code/assets in the documentation constitution without allowing
   game meaning to depend on development sources.
2. Evolve local-play wording from ledger to Studio, keep gameplay conversation and
   mutation exclusions exact, and update static pointers/evidence once.
3. Record the built comparison as active/inconclusive pending User layout selection;
   do not claim a final production design.

**Invariants:**

- Accepted game domain/capability semantics and published Agent text remain
  unchanged.
- Evidence detail lives only in `dev/docs/evidence/`; concept owns design rationale.

**Evidence:**

- Documentation link/placement/duplication scan and focused authority review.
- Full Rust, shell, formatting and lint validation remains green.

**Stop conditions:**

- Stop if the placement model would let runtime game behavior derive meaning from
  concept, plan, research, lab or evidence content.

## Validation ladder

1. **Focused:** Studio Rust tests, browser route assertions and launcher regression.
2. **Contract:** formatter; Clippy with warnings denied; all Rust tests; local launcher
   suite; exact MCP tool fixture/OpenAPI parity unchanged.
3. **Outcome:** run `cargo dev`, inspect real seeded data and source-backed Game
   material in all three desktop/mobile variants, and confirm direct source changes
   appear after the Rust process reload/restart without a copied catalog edit.
4. **Integrity:** `git diff --check`, focused diff review, link/authority scan and
   confirmation that all pre-existing multiplayer/research changes remain intact.

## Change control

Refine Rust module boundaries, exact bounded endpoints, parser details and visual
composition while the accepted in-place, read-only Studio outcome remains unchanged.
Return to `draft`, revise and regain acceptance for any game/public contract change,
schema/migration, non-Rust application, second process, persistence, external
service, token spend, authentication or multi-World identity decision.

## Completion conditions

- T1–T3 are completed and the validation ladder passes;
- `cargo dev` opens the existing Rust application as Aicadia Studio with three
  variants and real source/World information;
- no manual catalog duplication, mutation, new MCP/OpenAPI capability or false
  multiple-World claim remains;
- concept, current local-operation contract, evidence and placement constitution are
  aligned to the exact delivered boundary;
- the comparison remains explicitly unresolved until the User selects a layout;
- plan status becomes `complete` only after the bounded build/evidence claim passes.

## Completion record

Completed on 2026-08-17 without changing game schema, World behavior, OpenAPI,
published MCP capabilities or Agent-facing text. `cargo dev` starts one Rust
process and prints/opens Aicadia Studio. Fourteen allowlisted repository sources,
six current model sections and the exact compiled fifteen-tool catalog feed
`Game`; bounded Entity, Character, Place, personal Activity, Entity-state preview
and fourteen-table schema projection feed `Live`.

Atlas, Workbench and Observatory were inspected with real local data at desktop and
390px widths. Game/Live switching, keyboard/toolbar variant switching, live Entity
detail, responsive navigation, in-Studio known-source links and an empty browser
warning/error console passed. The layout decision remains deliberately unresolved.

Validation passed:

- `cargo fmt --all -- --check`;
- `cargo clippy --all-targets --all-features -- -D warnings`;
- `DATABASE_URL=postgres://localhost:5433/postgres cargo test --all-targets --all-features`
  with 158 passed and one explicitly ignored catalog-generator test;
- `DATABASE_URL=postgres://localhost:5433/postgres studio/tests/aicadia-local.sh`;
- Bash syntax and `git diff --check`; and
- focused current-authority/old-ledger wording scan with unrelated dirty
  multiplayer/research changes preserved.
