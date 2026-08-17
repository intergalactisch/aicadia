---
status: active
created_at: "2026-08-17T16:16:15+02:00"
updated_at: "2026-08-17T18:05:00+02:00"
accepted_at: "2026-08-17T18:05:00+02:00"
completed_at: null
---

# Aicadia Studio as the seamless development environment over one repository structure

> **Role / side:** proportional Studio-and-documentation build plan / development side.
> **Authority:** owns the bounded execution state for turning Aicadia Studio into the single reference, live-World browser and builder-orientation environment, and for the repository-structure conventions that make Studio maintenance-free.
> **Excludes:** game behavior, new player capabilities, multiplayer product decisions and Studio write access; see `docs/game/`, `docs/concept/concurrency-and-world-dynamics.md` and later accepted plans.

## Outcome

The developer opens one running `cargo dev` Studio and can answer, from one place
and without opening a terminal or a file tree, the seven questions that currently
cost the most overview:

1. **What do player Agents receive?** The exact published play contract, every tool
   description, input schema, annotation, HTTP route and capability contract, shown
   byte-for-byte from the compiled server, next to the host requirements.
2. **What are the models and how do they relate?** One folder per model in
   `docs/game/model/<model>/`; each Studio model page joins that owning contract to
   the canonical vocabulary term, the realized PostgreSQL tables, ordered foreign
   keys, the capabilities that link to it and every concept, research, log, plan and
   evidence record that references it.
3. **What was decided and what is still open?** A filterable decision register over
   the append-only concept log plus one aggregated view of every "Open …" section in
   live concept records, draft plans and backlog items.
4. **What is under construction?** The current edge, every `draft`/`active` plan
   rendered as a task board from its own task graph and the ordered backlog horizon.
5. **Which experiments ran and what did they find?** Every lab track and experiment
   with question, real/simulated seams, verdict, status and the decision it informs.
6. **What is actually in the World right now?** A bounded, cross-linked browser over
   Users, Characters, Places, Entities, Property keys, Traits, investigation
   attempts, the Activity chronicle and every PostgreSQL table's structure and rows,
   with an id resolver and stable per-record URLs.
7. **How do I hand this to a builder Agent?** Every page has a stable path URL and a
   copyable reference; one `/brief` page and the same-code `cargo brief` command
   print the current orientation as Markdown for a fresh builder session.

The invariant that makes this sustainable: **Studio owns no content and no
allowlist.** Everything Studio shows is discovered by convention from governed
repository roots, compiled Rust (tool catalog, instructions, routes, migrations) or
the connected PostgreSQL schema and rows. Record metadata lives once, in the record's
own front matter, and the same Rust parser that feeds Studio runs inside `cargo test`
so drift fails the build with the file named. Adding a model is creating a folder;
adding a table is a migration; adding a tool is compiling; adding a plan, experiment,
research report or evidence slice is creating the file in its home. None of these
requires a Studio edit, and there is no separate "update Studio" step.

The exact evidence claim is: from one fresh `cargo dev`, every governed Markdown
record, model, capability, tool, decision entry, plan task, lab verdict, evidence
slice and bounded live World record above is reachable through a stable path URL
and copyable reference; no browser or Rust file contains an authored copy of any
projected rule, field, relation, tool text or status; every record-metadata field
has exactly one home; `cargo test` proves home mapping, front-matter validity,
role headers, live-scope link/anchor integrity and model-to-migration table
completeness; `cargo brief` and `/brief` render the same Markdown orientation from
the same Rust projection; and existing game, HTTP, MCP, Agent text, schema and
local-launch behavior remain unchanged except for the explicitly accepted removal of
the Studio-only loopback Entity reads from the game HTTP surface.

## Non-goals

- No game behavior, migration, World mutation, new player capability, Agent-text
  change, browser gameplay, model invocation, background polling or authentication.
- No Studio write path: Studio never edits, generates or commits repository files,
  never writes the database and never records a decision on the developer's behalf.
- No second authority: no Studio-owned metadata store, generated documentation
  committed as truth, hand-maintained catalog in Rust or JavaScript, or status
  duplicated between an index and its record.
- No arbitrary repository browser, source-code viewer, unrestricted SQL console,
  arbitrary file download, World-row backup, retained snapshot history or schema
  diff tool.
- No indexed World-data search engine, new game index or exact global row counts;
  bounded reads, keyset paging, planner estimates and an exact-id resolver only.
- No frontend framework, Node/Python application, external runtime or second server
  process; Rust remains the application layer.
- No task-management product, ticket system or new planning artifact type; the
  existing plan task graph, backlog horizon and concept log remain the planning
  homes and Studio renders them.
- No change to the append-only concept log except new entries; no rewriting of
  frozen history to fit the new parser.
- No multiplayer, concurrency or living-World product decision; that grill stays in
  its own record and plan.

## Evidence and authorities

| Evidence or authority | Current fact | Consequence for this build |
| --- | --- | --- |
| User direction, 2026-08-17 | The developer loses overview across rules, models, decisions, open questions, active work, experiments and live data; Studio must become the one reference, live-data browser and development environment; maintaining Studio must never be a separate chore; documentation structure may change, including one folder per model. | Studio becomes a convention-driven projection; repository conventions are redesigned so records carry their own metadata; the plan covers docs, projection, presentation, Live reads, lint tests and builder orientation together. |
| User direction, 2026-08-17 (second) | The current Studio interface is not attractive and its navigation and sub-navigation do not work well; everything must become better and more beautiful. | Visual design and the navigation model are redesigned in this plan (T4, D11) rather than retained from the unified Studio; the design must be specified concretely before pages are built. |
| `AGENTS.md` — One Home Per Truth, Every Choice Leaves A Trail, Current Means Current, Flat Over Clever, Earn Your Spot | Truth has one home; choices are recorded in the log; superseded paths are removed; no clever indirection. | Volatile record fields get one home (front matter or one index table); Studio allowlists and the SPA state machine are removed once replaced; new conventions are enforced by tests rather than reminders. |
| `docs/README.md` home table and sweep rules | Placement is decided per home; link, anchor, duplication, status and old-token sweeps are described but no tooling executes them. | The Studio projection mirrors the home table and turns the described sweeps into `cargo test` assertions within their stated scope. |
| `docs/game/domain.md` (358 lines) | Nine model/seam sections live in one file with Rust snippets, value rules and the Activity operation table; `## Shared value validation` is an empty heading. | Model contracts move to `docs/game/model/<model>/README.md` under a move map; `domain.md` keeps the overview and cross-model rules. |
| `src/studio/catalog.rs` | Fourteen hand-listed sources and nine hand-listed model sections with hardcoded storage tables feed Studio. | Both allowlists are deleted; discovery is by governed root and model folder; storage tables come from model front matter and are checked against migrations. |
| `src/studio/live.rs`, `web/studio.js` | Live reads Entity/Character/Place lists, one Entity/Activity detail and schema; personal Activity depends on the launcher's User id passed through a URL fragment and session storage; the browser holds a 1,245-line route/state/render tree. | Live gains operator-only bounded reads for every durable subject and a generic bounded row viewer; the fragment/session-storage path and the SPA state machine are retired under D1. |
| `docs/game/README.md` capability catalog | Loopback `GET /api/entity` and `GET /api/entity/{entity_id}` exist "only to the supported local Studio Live view". | Once Studio owns its reads, those routes are removed from the game HTTP surface (D7). |
| `docs/concept/log/2026-08.md` (3,165 lines) and `log/README.md` | Entries are `# date` / `## topic` / `- tag: text` bullets, with a second `**Status:** / **Choice:**` style; a tag vocabulary exists. | The register parser accepts both shapes, gives each entry a stable `date/topic-slug` anchor and exposes tag, topic and date facets; new entries keep the bullet-tag shape. |
| `.agents/plans/*/plan.md` front matter and task-graph tables | Plans already carry `status`, timestamps and a `ID/State/Depends/Parallel-safe/...` table. | Plans are the task boards; Studio parses them unchanged. |
| `.agents/backlog/README.md`, items | The horizon table carries order and state; item files repeat `Status:`. | The table becomes the single home for order and state; item files drop the duplicated line (D5). |
| `lab/README.md`, `lab/multiplayer/*/README.md` | Experiment READMEs carry `Status:` and `Verdict:` lines; the track index repeats verdict and status. | Experiment metadata moves to front matter; track indexes become link lists with question text. |
| `docs/research/README.md`, `docs/concept/README.md`, `docs/evidence/README.md` | Index tables carry per-record status/standing. | Volatile status moves into each record's front matter; indexes keep links and stable navigation text only. |
| `docs/concept/aicadia-studio.md`, August log | One Atlas-derived Studio with `Game`/`Live` is the recorded direction; open decisions include Markdown metadata standardization and Rust source parsing versus generation. | This plan resolves those open decisions and, if accepted, supersedes the two-section navigation with `Overview · Game · Development · Live` (D2). |
| `.agents/skills/build-aicadia/SKILL.md` — Find the current edge | Builders orient with `git status`, `rg` and narrow reads. | `cargo brief` becomes an additional first read for orientation; the skill gains one pointer (D8). |
| Migrations `0001`–`0010`, `docs/game/storage.md` | Indexes exist for Entity pagination, actor and Place chronicles, User attempts, Place membership; no global `(occurred_at, id)` Activity index. | Live chronicles are Place- and Character-scoped by index; a global newest-first read is a labeled local-development sort under D6. |
| `.agents/plans/20260814-130554-documentation-architecture/` | The last documentation restructure used per-task move-map fragments and a link sweep. | The model-folder split reuses the move-map method and the new executable link sweep. |

## Alignment

### Strategic

No next game edge is selected. The concrete risk on the path to the next edge is
that the developer cannot see, in one place, what the game already promises, what
was decided, what remains open and what the World contains — so the next decision is
made from partial memory and every builder session starts with expensive
re-orientation. This plan retires that risk and the bookkeeping burden that causes
it: it does not add a player capability, but it makes every future capability
cheaper to choose, plan, build and prove, and it makes the multiplayer grill in
`docs/concept/concurrency-and-world-dynamics.md` navigable. The following game risk
is whichever concrete capability the User selects next from the backlog horizon.

### Tactical

The actor is the local developer (and, through `cargo brief`, a builder Agent). On
`cargo dev`, explicit navigation or Refresh, Studio scans governed repository roots,
reads compiled Rust catalogs and performs bounded PostgreSQL reads. It renders one
page per stable path, records nothing, mutates nothing and invokes no model. Missing
front matter, an unmapped file, a broken live-scope link, an unclaimed table, an
unknown id, a truncated list, an unavailable database or a schema over bound are
explicit visible states — on the page, on the Overview and in the failing test —
never silent omissions. No accepted World state changes, so no Activity footprint
applies.

Concrete allowed and rejected examples of "seamless":

- Allowed: a builder creates `docs/game/model/relation/README.md` with
  `storage_table: [entity_relation]` and a migration; the model appears in Game,
  its tables in Live, its page joins meaning to columns. Rejected: editing a Rust
  or JavaScript list to make it appear.
- Allowed: a builder appends a `- accepted: …` entry to the August log; it appears
  in Decisions with its anchor. Rejected: writing the same decision into a Studio
  file.
- Allowed: a lab experiment sets `verdict: refuted` in its own front matter and the
  track index, Lab section, Overview and `cargo brief` all reflect it. Rejected:
  updating the track table by hand as well.
- Allowed: a plan task moves to `completed` in the plan's task graph; the Work board
  updates. Rejected: a separate task file or status field for Studio.
- Rejected: a record without front matter in a home that requires it, or a table
  claimed by two models — `cargo test` fails and names the file.

### Technical

- `World` behavior, migrations, mutation transactions, idempotency and locks: not
  changed. Studio-only SQL stays separated from World reads and remains read-only.
- Repository projection: one Rust module scans the governed roots named in
  `docs/README.md`, parses a strict front-matter subset (`key: scalar` and
  `key: [a, b]`, no nesting, unknown keys rejected per home), role headers, headings
  with stable ids, Markdown links (resolved to repository paths and anchors), the
  concept-log register, plan front matter plus task graphs, backlog horizon tables
  and model folders. It computes backlinks over all live records. No YAML dependency
  is added; the subset is hand-parsed and tested.
- Compiled surface: tool catalog, assembled instructions and instruction section
  files, HTTP route table, migration list and the constant World name come from the
  existing Rust functions and `include_str!` sources, never from copies.
- PostgreSQL: every list is keyset-paged with `LIMIT` ≤ 100 over an existing index
  or primary key; totals are `pg_class.reltuples` estimates labeled as such; the
  generic row viewer selects `row_to_json` from one introspected `public` table
  ordered by its primary key with row-comparison keyset, quoted identifiers and
  bound parameters; the schema reader keeps its hard caps. Millions of rows and one
  hot Entity change no Studio query plan except the accepted labeled global sort in
  D6, which is a local-development read and never a game read.
- Presentation: under D1 (accepted), pages are Rust server-rendered HTML with
  path routes, a newly designed CSS system (D11) and one small
  progressive-enhancement script for copy, filter-loaded-rows, keyboard navigation
  and mobile disclosure.
  Under the alternative, the JSON API and SPA remain and grow. Either way browser
  assets contain no rule, field, relation, tool or status copy.
- Builder orientation: `/brief` and `cargo brief` render the same Rust projection
  as Markdown; the CLI degrades to repository-only content when no database is
  reachable.
- Tests: docs-lint tests live beside Studio tests, run in `cargo test`, and use the
  same parser as the pages; live tests use the disposable database.
- HTTP/MCP parity: not applicable; Studio routes stay absent from OpenAPI and MCP.
  Removing the two loopback game reads (D7) is a deletion from OpenAPI, not a
  capability change.

## Decisions, assumptions and open questions

### Confirmed decisions

- Studio remains English, Rust-owned, read-only, loopback and served by `cargo dev`;
  browser assets are presentation only — recorded in `docs/concept/aicadia-studio.md`
  and the August log.
- Studio owns no content and no allowlist: discovery is by governed root, model
  folder, compiled catalog and connected schema — User direction, 2026-08-17.
- Volatile record fields (status, standing, verdict, state, horizon) have exactly one
  home; stable navigation text (title, question, theme) may be repeated in a
  human-readable index — application of One Home Per Truth to this build.
- One folder per model under `docs/game/model/<model>/` with `README.md` as the
  contract entry and further concern files allowed inside the folder — User
  direction, 2026-08-17.
- The parser that feeds Studio also runs in `cargo test`; drift is a failing test
  that names the file — User direction that Studio maintenance is never a chore.
- Every page has a stable path URL and copyable reference — retained from the
  unified Studio plan.
- **D1 resolved:** Studio pages are Rust server-rendered HTML with path routes;
  the JSON API and the SPA route/state machine are removed once replaced; browser
  code is one small enhancement script — User choice, 2026-08-17.
- **D2 resolved:** primary navigation is `Overview · Game · Development · Live`,
  mirroring the runtime side, development side and connection of the documentation
  constitution and superseding the two-section direction — User choice, 2026-08-17.
- **D3 resolved:** nine model folders (`world`, `user`, `entity`, `character`,
  `place`, `activity`, `property`, `trait`, `investigation-attempt`, the last
  holding the chance/admission table); `domain.md` remains as the overview and
  cross-model rules with a stable path — User choice, 2026-08-17.
- **D4 resolved:** volatile fields (status, standing, verdict, track status) live
  only in each record's front matter; concept, research, evidence and lab index
  READMEs become link lists with stable navigation text; Studio, Overview and
  `cargo brief` are the status views — User choice, 2026-08-17.
- **D5 resolved:** the backlog horizon table is the single home for order and
  state; item files drop their `Status:` line; idea-only rows keep a state without an
  item file — User choice, 2026-08-17.
- **D6 resolved:** Live offers indexed Place and Character chronicles plus one
  global newest-first World chronicle (`LIMIT` ≤ 100, keyset on `(occurred_at, id)`)
  explicitly labeled as a local-development sort of the Activity table and never a
  game read; no new index — User choice, 2026-08-17.
- **D7 resolved:** Live adds operator-only bounded reads for Users, investigation
  attempts, Property keys, Trait lineages, full Entity history, an id resolver and a
  generic bounded row viewer per introspected table; `docs/game/local-play.md`
  changes to "exposes bounded rows of the local World"; the Studio-only loopback
  `GET /api/entity` and `GET /api/entity/{entity_id}` are removed from the game HTTP
  surface and OpenAPI; MCP is unchanged — User choice, 2026-08-17.
- **D8 resolved:** `/brief` and a `cargo brief` alias to one Rust binary render the
  same Markdown orientation (pointers and current fields only, never prose
  synthesis); the CLI works without a database for repository content; the build
  skill's "Find the current edge" gains one pointer sentence — User choice,
  2026-08-17.
- **D9 resolved:** no working-tree panel; Studio spawns no external process and
  the working tree stays a terminal concern — User choice, 2026-08-17.
- **D10 resolved:** the documentation lint is one Rust function over the projection;
  it fails `cargo test` and is shown as clickable warnings on Overview and in the
  brief — User choice, 2026-08-17.
- **D11a resolved (navigation model):** one fixed primary navigation; per section
  a complete, stable tree sidebar (every model, capability, record, table listed;
  current page marked; nothing swaps on navigation); a breadcrumb/context strip with
  status badges and owning path on every page; a server-side jump box (`/` or
  Cmd-K) that resolves any known resource name, path, tool name or id to its page
  (not a search engine); a fixed Related column on detail pages for backlinks,
  tables, capabilities and evidence — User choice, 2026-08-17.
- **D11b resolved (visual direction):** light mode only (no dark theme), calm and
  clean, typography first, color reserved for status, equally careful presentation
  of prose records and data (tables, ids, schemas). No mockup variants: root makes
  the one design directly as the T4.0 stylesheet and reference pages, and the User
  reviews it on the built pages — User choice, 2026-08-17.

- **CSS approach resolved:** one hand-written `web/studio.css` design system with
  design tokens and a small set of component classes; no Tailwind, no CSS build
  tool, no generated stylesheet, no CDN — User choice, 2026-08-17.

### Reversible assumptions

- Governed roots and their homes mirror the `docs/README.md` home table:
  `docs/game/**`, `docs/concept/**` (log and archive as their own homes),
  `docs/research/**`, `docs/evidence/**`, `.agents/backlog/**`, `.agents/plans/**`,
  `.agents/skills/**`, `lab/**/README.md` and `lab/**/*.md`, `src/agent_contract/**`,
  `migration/*.sql`, plus `AGENTS.md`, `CLAUDE.md`, `CONTEXT.md`, `docs/README.md`.
  Frozen homes (archives, monthly logs, completed plans) are rendered but excluded
  from link and metadata lint per the sweep-scope rule.
- Front-matter vocabulary per home (validated by lint; unknown keys fail):
  - `docs/game/model/<model>/README.md`: `storage_table` (list, may be empty),
    optional `kind` (`subject | role | seam | history | state | provenance | participant`).
  - `docs/concept/*.md`: `status` (`active | live | retained | throwaway`).
  - `docs/research/*.md`: `status` (`pending | load-bearing | historical | superseded`), `era`.
  - `docs/evidence/*.md`: `status` (one line of text).
  - `lab/<track>/README.md`: `status` (`active | kept | superseded | discarded`).
  - `lab/<track>/<experiment>/README.md`: `question`, `verdict`
    (`supported | refuted | inconclusive | pending`), `status`, `real_seam` (list),
    `simulated_seam` (list), `informs` (repository path).
  - `.agents/plans/*/plan.md`: existing keys plus optional `backlog_item` (path).
  - `.agents/backlog/items/*.md`: none; state lives in the horizon table (D5).
  - `docs/game/capability/*.md`: none; metadata comes from the compiled catalog.
- Path scheme: `/`, `/brief`, `/doc/<repository path>`, `/game/model/<model>`,
  `/game/capability/<name>`, `/game/agent`, `/game/agent/tool/<name>`,
  `/game/vocabulary`, `/game/storage`, `/development/direction/<record>`,
  `/development/decision?date&topic&tag&q`, `/development/decision/<date>/<topic-slug>`,
  `/development/open`, `/development/research`, `/development/work`,
  `/development/plan/<dir>`, `/development/backlog`, `/development/lab/<track>/<experiment>`,
  `/development/evidence/<slice>`, `/development/rules`, `/live`,
  `/live/activity?place|character|operation&before`, `/live/activity/<id>`,
  `/live/entity`, `/live/entity/<id>`, `/live/place/<id>`, `/live/character/<id>`,
  `/live/user`, `/live/user/<id>`, `/live/investigation`, `/live/investigation/<id>`,
  `/live/property-key`, `/live/property-key/<key>`, `/live/trait/<id>`,
  `/live/storage`, `/live/storage/<table>`, `/live/storage/<table>/row?after`,
  `/live/migration`, `/live/resolve?id`, `/jump?q`. Query keys are the only
  paging/filter state.
- `maud` (or `askama`) is the one template dependency; the choice is verified
  against current axum 0.8 support before T4 starts and can change without changing
  any page contract.
- The T4.0 design (stylesheet and reference pages) lives beside this plan in
  `design/` until T4 moves the stylesheet into `web/`; reference pages remain frozen
  plan artifacts and are never served.
- Copied reference format stays
  `[Aicadia Studio · <title>](<url>) — <owning path or durable id>`.

### Open questions

None. Every material question is resolved above; the User reviews the T4.0 design
on the built pages before T5–T8 reuse it.

## Implementation map

| Surface | Current state | Intended change | Invariants |
| --- | --- | --- | --- |
| `docs/game/model/<model>/README.md` (new) | Model sections inside `domain.md`. | Nine model folders with front matter and moved contract text under a move map; further concern files allowed. | Meaning moves, never duplicates; each table claimed by exactly one model. |
| `docs/game/domain.md` | 358-line domain contract with model sections. | Overview, cross-model text validation, error taxonomy, evidence pointer; links to model folders. | Path stable; frozen citations keep old anchors. |
| `docs/game/README.md` | Reading order and capability catalog; loopback Entity-read note. | Reading order names `model/`; "Model contracts" link list; loopback note removed under D7. | Catalog table unchanged. |
| `docs/concept/*.md`, `docs/research/*.md`, `docs/evidence/*.md`, `lab/**/README.md` | Status in index tables and `Status:`/`Verdict:` lines. | Front matter per home vocabulary; index READMEs become link lists (D4). | Append-only log untouched; archives untouched. |
| `.agents/backlog/README.md`, `items/*.md` | Table state plus item `Status:` lines. | Table is the state home; items drop the line (D5). | Order and states unchanged in meaning. |
| `docs/README.md` | Home table, sweep rules, headers, size. | Add model-folder, record-metadata and lint rows/sections; state that sweeps execute in `cargo test`; update `src/studio/`, `web/`, tests rows. | Placement authority only. |
| `docs/game/local-play.md` | Two-section Studio, fragment User id, no rows. | Describe four sections, path links, bounded operator reads, row viewer, brief and D6/D7 boundaries. | Studio remains read-only, loopback and never player authority. |
| `docs/concept/aicadia-studio.md`, August log | Unified Studio direction; open metadata/generation decisions. | Record the selected convention-driven direction and close or supersede the open decisions. | History retained. |
| `src/studio/repository.rs`, `home.rs`, `register.rs`, `plan.rs`, `model.rs`, `agent.rs` (new) | Allowlists in `catalog.rs`. | Convention scan, parsers, backlinks, register, plans/backlog, model join, agent surface; `catalog.rs` deleted. | No content copy; bounded file sizes; unknown front-matter keys rejected. |
| `src/studio/live/*.rs` | Three lists, two details, schema. | Bounded reads for every durable subject, chronicles, resolver, estimates, migrations, row viewer. | Keyset, `LIMIT` ≤ 100, quoted identifiers, no writes. |
| `src/studio/page/*.rs`, `src/studio/mod.rs` | JSON routes plus static SPA. | Path-routed pages per section under D1; reference/copy; explicit partial states. | Absent from OpenAPI/MCP; GET only. |
| `src/studio/brief.rs`, `src/bin/aicadia-brief.rs`, `.cargo/config.toml` | None. | Markdown brief renderer, binary and `cargo brief` alias (D8). | Same projection; no database required for repository content. |
| `web/studio.css`, `web/studio.js`, `web/index.html` | Design system plus SPA that the User finds unattractive and hard to navigate. | Replace with the newly specified design system (D11) and one small enhancement script (D1). | No authored truth in browser assets. |
| `tests/studio/*.rs` (new) | Studio unit tests only. | Docs-lint suite and page/route tests. | Uses the same parsers as pages; live tests use the disposable database. |
| `src/server/http.rs`, `docs/game/README.md`, `docs/game/protocol.md` | Loopback Entity reads for Studio. | Remove under D7. | MCP catalog and every player capability unchanged. |
| `.agents/skills/build-aicadia/SKILL.md` | `rg`-based orientation. | One pointer to `cargo brief` (D8). | Skill routing otherwise unchanged. |
| `docs/evidence/local-play.md`, backlog | Unified Studio evidence. | Append completion evidence; backlog pointer. | History appended, not rewritten. |

## Execution contract

Root (Claude Code, orchestrating) owns outcome, scope, plan state, integration,
review and the final evidence claim; the User accepted delegation of task execution
to Opus builder subagents on 2026-08-17. A delegated builder receives this plan path
and one dependency-ready task id,
re-reads the live repository, changes only its owned surfaces, runs focused evidence
and returns raw results. T1 and T2 may run in parallel because their write surfaces
(Rust versus documentation) do not overlap and the front-matter vocabulary is fixed
above. T5, T6 and T7 may run in parallel after T4 because each owns distinct page
and read modules. Everything else is sequential.

## Task graph

Allowed states are `pending`, `in_progress`, `completed` and `blocked`.

| ID | State | Depends | Parallel-safe | Objective | Owned surfaces | Evidence |
| --- | --- | --- | --- | --- | --- | --- |
| T1 | pending | — | with T2 | Rust repository projection: governed-root scan, home mapping, front matter, role headers, headings, links, backlinks, decision register, plans/backlog, model folders, agent surface. | `src/studio/repository.rs`, `home.rs`, `register.rs`, `plan.rs`, `model.rs`, `agent.rs`, unit tests | Unit tests over fixtures and the real repository parse every governed record, resolve backlinks and register entries, and reject unknown keys. |
| T2 | pending | — | with T1 | Documentation restructure: model folders under a move map, slimmed `domain.md`, front matter adoption, index READMEs as link lists, backlog state home, `docs/README.md` conventions. | `docs/**`, `.agents/backlog/**`, `lab/**/README.md`, move map | Move map complete; every live link resolves; no volatile field has two homes; role headers intact. |
| T3 | pending | T1, T2 | no | Docs-lint suite in `cargo test`: home mapping, front matter, headers, live-scope links/anchors, model↔migration tables, capability doc↔tool↔route, register parse. | `tests/studio/lint.rs` | Suite passes on the restructured repository and fails with a named file when a fixture violates each rule. |
| T4 | pending | T1, T4.0 design | no | Presentation shell and new visual design: navigation model, layout, typography, color, density and interaction states specified and built; path routes; record reader `/doc/<path>` with outline and backlinks; references; partial/404 states. | `src/studio/mod.rs`, `src/studio/page/{shell,doc}.rs`, `web/**` | The one design is delivered and reviewed by the User on built pages before T5–T8 reuse it; route tests return HTML for every governed record; browser check at 1440/390 px; no console errors. |
| T5 | pending | T4, T2 | with T6, T7 | Game section: models, capabilities, agent surface, vocabulary, storage contract, deferred. | `src/studio/page/game*.rs` | Every model folder, capability and tool has a page joining owning text, compiled data, schema and backlinks; tests assert counts equal folder/catalog counts. |
| T6 | pending | T4, T2 | with T5, T7 | Development section: direction, decisions register, open questions, research, work (edge, plan boards, backlog, capability map), lab, evidence, rules. | `src/studio/page/development*.rs` | Register facets, plan boards from real plans, aggregated open sections and lab verdicts render; tests assert entry and task counts. |
| T7 | pending | — | with T5, T6 | Live reads: chronicles, full Entity/Place/Character/User/investigation/Property-key/Trait reads, resolver, estimates, migrations, row viewer; remove loopback game reads. | `src/studio/live/*.rs`, `src/server/http.rs`, `docs/game/README.md`, `docs/game/protocol.md`, tests | Disposable-database tests prove bounds, keyset, quoting, resolver and row viewer over every current table; OpenAPI no longer lists the two routes. |
| T8 | pending | T4, T7 | no | Live section pages over the T7 reads with cross-links, filters, load-more, truncation states and schema/row/migration views. | `src/studio/page/live*.rs` | Browser walk from World → Place → Character → Activity → Entity → Trait/Property key → table row and back through stable URLs. |
| T9 | pending | T5, T6, T8 | no | Overview dashboard, `/brief`, `cargo brief`, lint warnings surfaced, skill pointer. | `src/studio/page/overview.rs`, `src/studio/brief.rs`, `src/bin/aicadia-brief.rs`, `.cargo/config.toml`, skill | `cargo brief` and `/brief` are byte-identical for the same state; the CLI works without a database for repository content. |
| T10 | pending | T3–T9 | no | Contract alignment, cleanup of superseded Studio code, evidence and validation ladder. | `docs/game/local-play.md`, `docs/README.md`, concept record, log, backlog, evidence, tests | Ladder passes; no allowlist, SPA route state or stale two-section claim remains outside frozen history. |

## Task details

### T1 — Repository projection in Rust

**Objective:** Rust discovers and parses every governed record and compiled surface
without any hand-maintained list, and computes the joins Studio pages need.

**Actions:**

1. Implement the governed-root scan and the home mapping table mirroring
   `docs/README.md`; unmapped Markdown under a governed root is a reported error.
2. Implement the strict front-matter subset parser, per-home key vocabulary and
   validation; parse role headers into role, side, authority and excludes.
3. Reuse and extend heading extraction with stable ids; extract Markdown links,
   resolve them to repository paths and anchors, and compute backlinks over live
   records with frozen homes excluded.
4. Implement the concept-log register parser: date, topic, stable anchor, entries
   with tag prefix and qualifier for both bullet-tag and bold-status shapes.
5. Implement plan parsing (front matter, task graph table, "Open questions" section),
   backlog horizon table parsing and "Open …" section aggregation across live
   development-side records.
6. Implement the model-folder projection (front matter, contract pages, vocabulary
   term match by name from `CONTEXT.md`) and the agent-surface projection
   (instruction section files, assembled instructions, tool catalog, HTTP route table
   from the compiled router, capability contract paths).
7. Delete `src/studio/catalog.rs` allowlists once the projection supersedes them.

**Invariants:**

- No content is copied; every projected string comes from a file, compiled constant
  or database result.
- Read bounds per file and per root exist and fail explicitly.
- Unknown front-matter keys, malformed lists and unknown vocabulary values are errors.

**Evidence:**

- `cargo test studio::` — fixture tests per parser and a repository-wide parse.

**Stop conditions:**

- Stop if a governed record cannot be parsed without changing frozen history or
  inventing metadata that no home owns.

### T2 — Documentation restructure and metadata adoption

**Objective:** The repository carries the conventions Studio projects: model folders,
one-home volatile fields, front matter and link-list indexes.

**Actions:**

1. Write `move-map/t2.md` for every `domain.md` heading moved into
   `docs/game/model/<model>/README.md`, then move the text; keep `domain.md` as the
   overview and cross-model rules; fix the empty `## Shared value validation`.
2. Add front matter to concept, research, evidence, lab track and experiment records
   per the vocabulary; remove `Status:`/`Verdict:` prose lines they replace.
3. Convert `docs/concept/README.md`, `docs/research/README.md`,
   `docs/evidence/README.md`, `lab/README.md` and `lab/multiplayer/README.md`
   tables to link lists with stable navigation text (D4); move the evidence README's
   inline "Agent contract delivery" record to `docs/evidence/agent-contract.md`.
4. Make the backlog horizon table the state home and remove item `Status:` lines
   (D5).
5. Update `docs/README.md`: model-folder row, record-metadata section, lint
   section, sweep execution note, Studio/tests/web rows; update `docs/game/README.md`
   reading order and model link list; update `CONTEXT.md` only if a term changes.
6. Fix every live-scope link and anchor affected by the moves.

**Invariants:**

- Frozen history (archives, monthly logs, completed plans) is untouched.
- No rule text changes meaning; only location and metadata change.
- Every authority keeps its role header.

**Evidence:**

- The T3 lint suite (or, before T3 exists, `rg`-based checks recorded in the move
  map) shows every live link and anchor resolves and every record validates.

**Stop conditions:**

- Stop if a section has two plausible owning homes; record the question in the
  concept record and ask.

### T3 — Docs-lint suite

**Objective:** The documentation constitution's sweeps and this plan's conventions
run in `cargo test`.

**Actions:**

1. Add tests: every Markdown under a governed root maps to one home; every
   authority file has a role header; every record's front matter validates; every
   live-scope link and anchor resolves; every `CREATE TABLE` in `migration/` except
   `_sqlx_migrations` is claimed by exactly one model folder and every claimed table
   exists; every compiled tool has a capability document and tool description; the
   register parses every entry.
2. Add negative fixtures proving each rule fails with the offending path.

**Invariants:**

- Lint scope equals the sweep scope in `docs/README.md`; frozen homes are excluded.
- No database is required.

**Evidence:**

- `cargo test --test studio` — green on the restructured repository; each negative
  fixture red with a named path.

**Stop conditions:**

- Stop if a rule would require rewriting frozen history to pass.

### T4 — Presentation shell and visual design

**Objective:** One accessible, path-routed shell with a new, deliberately designed
visual system and navigation model renders any governed record with outline,
provenance, backlinks and copyable reference.

**Actions:**

0. Make the one design directly (D11b, no mockup variants): root delivers
   `design/studio.css` and static reference pages beside this plan (shell with
   primary navigation, tree sidebar, breadcrumb strip, jump box and Related column;
   a record/model page; the decisions register; a Live Entity page) covering page
   anatomy, typography scale, color and status palette, density, tables, code and
   Markdown rendering, empty/error/truncated states and keyboard behavior; light
   mode only. The User reviews the built pages; T4 then moves the stylesheet to
   `web/studio.css` and binds the templates to it. Reference pages are plan
   artifacts, never served.
1. Add the template dependency (D1), the four-section layout (D2), header with
   connection state and read time, contextual navigation, skip link, mobile
   disclosure and the new design system; drop the previous CSS where superseded.
2. Implement `/doc/<path>` with rendered Markdown, heading anchors, outline,
   role-header panel, front-matter panel, "referenced by" panel and reference
   copy; internal links rewrite to Studio routes.
3. Implement explicit not-found, unmapped, over-bound and database-unavailable
   states.
4. Add route tests over every governed record and a browser check at 1440/390 px.

**Invariants:**

- GET only; no polling; no browser-owned truth.
- Reduced motion, focus visibility and contrast are preserved.

**Evidence:**

- `cargo test studio::page` — every governed record returns 200 HTML with its title.
- Browser inspection: keyboard path, focus, mobile navigation, zero overflow, empty
  console.

**Stop conditions:**

- Stop if the User rejects the built design; revise `design/` before continuing.

### T5 — Game section

**Objective:** The runtime side is navigable as models, capabilities, the exact
Agent surface, vocabulary and storage contract.

**Actions:**

1. Model index and pages: contract pages from the folder, vocabulary term, kind,
   storage tables with introspected columns, ordered relations, capabilities and
   records that link to the model, evidence and concept backlinks.
2. Capability pages: contract document, published tool description, input schema,
   annotations, HTTP route, MCP name, evidence backlinks.
3. Agent surface: discovery shape, assembled instructions with per-file outline,
   tool catalog, host requirements from `agent.md`, adapter command shape from
   `local-play.md`; every byte from compiled sources or owning files.
4. Vocabulary page from `CONTEXT.md` terms with links to matching model pages;
   storage page from `storage.md` plus migration list; deferred page.

**Invariants:**

- Counts on pages equal folder and catalog counts; tests assert it.
- No tool text, schema or field is authored in Studio.

**Evidence:**

- `cargo test studio::game` and browser walk model → table → capability → tool →
  agent contract section and back.

**Stop conditions:**

- Stop if a join needs metadata that no home owns.

### T6 — Development section

**Objective:** Direction, decisions, open questions, research, work, lab, evidence
and build rules are browsable and filterable from their owning records.

**Actions:**

1. Direction: live concept records with confirmed and open sections highlighted.
2. Decisions: the register with date, topic, tag and text facets, stable anchors,
   links to affected authorities and backlinks to the entry.
3. Open: aggregated "Open …" sections from live concept records, draft plans and
   backlog items with source links.
4. Research index and reader; evidence index and reader; rules page from `AGENTS.md`
   heuristics headings.
5. Work: current edge, plans as boards from their task graphs, backlog horizon,
   capability map.
6. Lab: tracks, experiments, question, seams, verdict, status, informs.

**Invariants:**

- The register is read from the log; nothing is re-authored.
- Studio spawns no external process (D9).

**Evidence:**

- `cargo test studio::development` — entry, plan-task, experiment and open-section
  counts equal parsed counts; browser filter and deep-link checks.

**Stop conditions:**

- Stop if a facet requires a new metadata field the log README does not define.

### T7 — Live reads

**Objective:** Every durable World subject and every table has bounded, cross-linked
operator reads, and the Studio-only loopback game reads are removed.

**Actions:**

1. Chronicles: Place-scoped and Character-scoped by existing indexes; recent by id;
   the labeled global newest-first sort under D6; operation filter.
2. Detail reads: Entity (identity, roles, Place, current Properties with history,
   Traits with version lineage, participation, introduced-by), Place (members,
   Characters present, chronicle, latest Activity/revision), Character (owner,
   Place, actor chronicle, attempts), User (Character, attempts, request provenance),
   investigation attempt (outcome, consumed/voided lineage), Property key (type,
   first Activity, bounded holders), Trait (lineage).
3. Resolver: one id → which primary keys or `request_id` hold it → link.
4. Estimates from `pg_class.reltuples`; migration list from `_sqlx_migrations` and
   repository files; generic row viewer with `row_to_json`, primary-key keyset,
   `quote_ident` and bound parameters over introspected tables.
5. Remove `GET /api/entity` and `GET /api/entity/{entity_id}` from the game router,
   OpenAPI, tests and `docs/game/README.md`/`protocol.md` (D7).

**Invariants:**

- `LIMIT` ≤ 100 everywhere; keyset over an index or primary key; row-comparison
  keyset for composite keys; no write; no new game index.
- Every unindexed path is either avoided or labeled as a local-development read.

**Evidence:**

- Disposable-database tests over every current table for bounds, keyset stability,
  identifier quoting, resolver hits/misses and the D6 label.
- Catalog and OpenAPI tests prove the two routes are absent and MCP is unchanged.

**Stop conditions:**

- Stop if a read needs a new index or an unbounded scan outside D6.

### T8 — Live section pages

**Objective:** The connected World is browsable as related domain views with
stable URLs, filters and honest truncation.

**Actions:**

1. Live overview with estimates, latest chronicle, entry Place, quick paths.
2. List and detail pages for every T7 read with cross-links, load-more via query
   keys, loaded-row filtering, truncation and error states.
3. Storage pages: schema, table detail, rows, migrations, snapshot download.

**Invariants:**

- Filters apply only to loaded rows and say so; direct id lookup is not search.

**Evidence:**

- Browser walk World → Place → Character → Activity → Entity → Trait/Property key
  → table row and back; every URL reloads to the same record.

**Stop conditions:**

- Stop if a page needs data outside T7.

### T9 — Overview, brief and builder orientation

**Objective:** One landing page and one Markdown brief let the developer zoom out
and hand a builder the current state.

**Actions:**

1. Overview: current edge/active plan or "no edge selected", draft plans, open
   count, latest decisions, lint warnings (D10), game surface counts, latest
   migration, live estimates, lab verdicts, evidence statuses, "copy brief".
2. `/brief` and `cargo brief`: the same Markdown from the same Rust functions
   (edge, plans with open questions, latest decisions, backlog, model/capability
   index with paths, agent surface pointer, lab verdicts, evidence statuses, lint
   warnings, live summary when reachable).
3. Add the `cargo brief` alias and one pointer sentence in the build skill (D8).

**Invariants:**

- The brief is pointers and current fields only; it states that it is generated
  and not an authority.

**Evidence:**

- Test: `/brief` body equals `cargo brief` output for one fixed state; CLI runs
  without `DATABASE_URL` and marks live sections unavailable.

**Stop conditions:**

- Stop if the brief would need to synthesize or summarize prose.

### T10 — Contract alignment, cleanup and evidence

**Objective:** Authorities describe exactly the delivered environment; superseded
Studio code is gone; the ladder passes.

**Actions:**

1. Update `docs/game/local-play.md`, `docs/README.md`, `docs/concept/aicadia-studio.md`,
   the August log, backlog pointer and `docs/evidence/local-play.md`.
2. Remove the SPA, JSON routes, catalog allowlists, fragment/session-storage User
   id path and any stale two-section wording outside frozen history.
3. Run the ladder and record only demonstrated evidence.

**Invariants:**

- History appended, never rewritten; unrelated working-tree changes preserved.

**Evidence:**

- The validation ladder below.

**Stop conditions:**

- Stop and return to `draft` if the outcome, a game contract or the evidence claim
  changed during execution.

## Validation ladder

1. **Focused:** parser fixtures, lint suite, page route tests, live read tests
   against the disposable database, brief equality test.
2. **Contract:** `cargo fmt --all -- --check`,
   `cargo clippy --all-targets --all-features -- -D warnings`,
   `DATABASE_URL=… cargo test --all-targets --all-features`, catalog/OpenAPI/MCP
   boundary tests, `bash -n tools/aicadia-local tests/aicadia-local.sh` and the
   disposable launcher lifecycle.
3. **Outcome:** from one fresh `cargo dev`, answer the seven questions in the
   Outcome section through the browser at desktop and 390 px, reload every visited
   URL, copy references, run `cargo brief` and confirm it matches `/brief`; add one
   temporary model folder and one temporary front-matter violation locally and
   observe the page appear and the lint fail, then remove them.
4. **Integrity:** `git diff --check`, focused diff review, stale-token scan for
   `studio/api`, `variant`, `Aicadia-User-Id` in Studio assets and two-section
   claims outside frozen history, confirmation that unrelated user changes and all
   governing authorities remain intact.

## Change control

Refine paths, page names, front-matter key names, task order and stronger evidence
in place while the accepted convention-driven, read-only, one-home outcome remains
unchanged. Stop implementation, keep or return `status: draft`, revise and request
explicit re-acceptance for any game behavior or schema change, new player
capability, Studio write path, generated authoritative documentation, new index,
unbounded read, authentication, external service, token spend or materially
different evidence claim.

## Completion conditions

- T1–T10 are `completed` and the validation ladder passes;
- every governed record, model, capability, tool, decision entry, plan task, lab
  verdict, evidence slice and bounded live record is reachable by stable path URL
  with a copyable reference;
- no Studio allowlist, authored copy of projected truth or duplicated volatile field
  remains; every record-metadata field has one home and `cargo test` proves the
  conventions;
- `/brief` and `cargo brief` render the same orientation;
- current operation, placement, concept direction, log, backlog, evidence and tests
  agree; and
- `status: complete` and `completed_at` are recorded only after those conditions.
