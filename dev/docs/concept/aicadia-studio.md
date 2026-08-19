---
status: active
---

# Aicadia Studio direction

> **Role / side:** active development-interface exploration / development side.
> **Authority:** owns the confirmed Aicadia Studio direction, rationale and open design decisions.
> **Excludes:** current game behavior, delivery evidence and final production implementation; see `game/docs/`, `dev/docs/evidence/` and an accepted build plan.

## Question

How can the developer step back from implementation and inspect Aicadia's accepted
game meaning, models, tools, live World state, unresolved questions, experiments and
evidence in one clear interface without creating a second documentation authority?

## Confirmed direction

- The application is named **Aicadia Studio** and its interface is English.
- Its primary navigation is `Overview · Game · Development · Live`: orientation,
  runtime-side truth, development-side state and the connected local World remain
  distinct while sharing one interface.
- `Game` projects accepted runtime documentation, domain models, exact compiled MCP
  tools, the assembled Agent surface, vocabulary, storage and deferrals from their
  owning sources.
- `Development` projects current Area syntheses, retained concept rationale,
  decisions, source-owned open landscape, research, work, experiments, evidence and
  build rules without becoming their authority.
- Within `Development`, `Areas` is the current development synthesis by durable
  subject. Each Area work document owns what the area means, what it is and is not,
  what has been chosen or rejected, what remains unchosen or needs research, which
  components it contains and the current directional technical model. Exact
  executable behavior remains defined in `game/docs/`; sourced findings, experiment
  verdicts, delivery evidence and the selected current edge remain in Research,
  Lab, Evidence and Work. Area overlap is allowed because Areas are flat navigation
  lenses, but the same underlying fact still has one owner and is linked rather
  than copied.
- The first flat Area set is `Multiplayer`, `Place`, `Movement`, `Discovery`,
  `Agent Play` and `World Change`. `Exploration` is not a separate Area in this set:
  it is a player experience composed especially from Place, Movement and Discovery.
  The singular route `/dev/areas/place` follows Aicadia's domain naming convention.
- `Multiplayer` retains its scenario catalogue as Area-owned prepared material
  rather than a child of the experimental Lab; relevant research, decisions and lab
  experiments remain linked sources under their own authority.
- Studio uses `/dev` as the canonical route for the complete `Development` section;
  the longer `/development` prefix and compatibility aliases are absent. Its Area
  hierarchy starts at `/dev/areas`, with `/dev/areas/multiplayer` and
  `/dev/areas/multiplayer/scenarios` as the first overview routes. The completed
  build and exact evidence claim are recorded in the
  [`studio-development-areas` plan](../../plans/20260818-133015-studio-development-areas/plan.md)
  and [local-play evidence](../evidence/local-play.md#source-backed-studio-development-areas--2026-08-18).
- The fixed-shape current-truth workbooks and their distinct Area workspace were
  delivered by the
  [`area-current-truth` plan](../../plans/20260818-153439-area-current-truth/plan.md).
  `Not yet chosen` and `Research needed` remain Area-owned durable landscape and
  also appear under `/dev/open`; they never become current priority, plan state or
  tasks until Work selects them. Exact projection and responsive-layout proof lives
  in [local-play evidence](../evidence/local-play.md#current-truth-studio-area-workbooks--2026-08-18).
- `Live` is connection-scoped. It browses one selected World's accepted records and
  storage shape read-only, with User, Character, Place, Entity, Property, Trait,
  investigation and Activity records presented as related domain views; generic
  bounded table rows remain an operator inspection surface, not game meaning.
- Governed records and model folders are discovered by convention; compiled and
  PostgreSQL structure is assembled or introspected. Semantic meaning and volatile
  record fields are authored once in their owning repository source and rendered
  directly. Studio owns no content, catalog allowlist or generated authority.
- Aicadia application logic, source parsing, catalog assembly, database access and
  server behavior are always Rust. No Python or Node application is introduced.
- The Rust local application serves Studio at `/`; `cargo dev` remains the single
  command. The former ledger is absorbed by `Live`, not retained as a second
  browser.
- HTML and one hand-written stylesheet provide presentation; one small JavaScript
  enhancement owns copy, loaded-row filtering, keyboard behavior and the mobile
  disclosure only. Browser code owns no rules, models, schemas, catalog, routing or
  database logic.
- Small project-structure changes are allowed only when they materially improve
  overall clarity and do not create parallel truth.
- The completed three-variant comparison does not remain a product mode. Retain one
  coherent application with Atlas's source-first hierarchy as its base, a dedicated
  model inspector using the Workbench's strongest structural ideas and a much clearer
  Live browser using the Observatory's strongest World-inspection ideas.
- Every navigable source, model, capability, MCP tool, storage object and live
  record receives a stable path URL and a copyable development reference suitable
  for an AI conversation. References identify the owning source or durable record;
  they never copy its truth into browser assets.
- The storage view may explicitly capture a downloadable schema snapshot containing
  the exact current table, column, constraint, foreign-key and index definitions.
  Rust derives it from the connected PostgreSQL schema, fingerprints it and stores
  it nowhere automatically; it is evidence from one moment, never a second schema
  authority or a World-data backup.
- One source-backed Markdown builder brief is available at `/brief` and through
  `cargo brief`; both are generated orientation from the same Rust projection and
  never an authority. Repository orientation remains available without a database.

## Current boundaries

The current game has one persistent World. Its concrete `World` type is a behavior
seam around one PostgreSQL pool, `get_world` returns constant name `Aicadia`, and no
durable `WorldId`, `world` table or `world_id` scope exists. Studio shows the one
connected local World and cannot present multiple Worlds as delivered behavior or
choose their storage topology.

The browser is a loopback, read-only local development surface served by the Rust
application. Agent conversation remains the only gameplay interface. Studio never
mutates the World, publishes a player capability, invokes a model or becomes
fallback authority for a player Agent.

The documentation architecture must distinguish the Studio projection from game
runtime meaning even though both are served by one local Rust process: `World`, game
HTTP and MCP remain independent of concept, plan, research, lab and evidence
content.

## Completed comparison question

Which information hierarchy makes it easiest to move between:

- the meaning and structure of Entity, Property and Trait;
- the exact published MCP-tool surface;
- accepted rules versus exploration, experiments and evidence; and
- the one selected local World's actual records and storage shape?

The completed comparison plan is
`dev/plans/20260817-140952-aicadia-studio-prototype/plan.md`. It compares three
structurally different layouts on the existing `/` route over one canonical
Rust-produced, read-only projection.

## Built comparison and selected composition

- **Atlas** is reference-first: repository navigation, rendered owning source and
  provenance dominate; compact model and MCP indexes keep structural context beside
  the document.
- **Workbench** is model-first: current model sections form a selectable canvas,
  live PostgreSQL foreign keys expose their realized relations and the exact MCP
  catalog remains visible as a systems shelf.
- **Observatory** is World-first: one honest local-World snapshot, bounded role/data
  views and Entity state inspection dominate; current and exploratory sources sit
  in distinct lanes under `Game`.

All three consume the same Rust catalog and World reads and proved that source-backed
inspection is practical without a generated documentation authority. Observed use
selected Atlas as the clearest foundation, rejected a permanent layout selector and
retained Workbench and Observatory only as design input for the model and Live
sections. The unified implementation plan is
`dev/plans/20260817-152535-unified-aicadia-studio/plan.md`.

## Accepted direction — convention-driven development environment

On 2026-08-17 the User found the unified Studio still insufficient for stepping
back: the rules player Agents receive, the models and their relations, decided
versus open choices, work in progress, experiments and the live World were not yet
visible together, and maintaining Studio could not become a separate chore beside
maintaining the repository. The accepted plan
`dev/plans/20260817-161615-studio-development-environment/plan.md` resolves that
pressure with convention-driven discovery, one home per volatile field, shared
projection/lint parsers, one folder per model, server-rendered path routes, the four
primary sections and the shared Markdown builder brief.

## Confirmed development-environment choices

- 2026-08-17, D1: Studio pages are Rust server-rendered HTML with one stable path
  route per resource; the superseded JSON application surface and browser
  route/state machine are retired and browser code is one small enhancement script.
- 2026-08-17, D2: primary navigation is `Overview · Game · Development · Live`,
  mirroring the runtime side, development side and connection of the documentation
  constitution and replacing the earlier primary-navigation direction.
- 2026-08-17, D3: the current game contract has one folder per model under
  `game/docs/model/<model>/` with `README.md` as the contract entry (nine folders:
  world, user, entity, character, place, activity, property, trait,
  investigation-attempt); `game/docs/domain.md` remains as the overview and
  cross-model rules; a new model must receive its own folder.
- 2026-08-17, D4: volatile record fields (status, standing, verdict, track status)
  live only in each record's front matter; concept, research, evidence and lab index
  READMEs become link lists with stable navigation text; Studio and the builder
  brief are the status views.
- 2026-08-17, D5: the backlog horizon table is the single home for backlog order
  and state; item files carry no status line.
- 2026-08-17, D6: Live shows indexed Place and Character chronicles and one global
  newest-first World chronicle of at most 100 Activities, explicitly labeled as a
  local-development sort of the Activity table and never a game read; no game index
  is added for Studio.
- 2026-08-17, D7 corrected after read-plan review: Live keeps only operator reads
  whose input work is bounded by an existing primary key or leading index, plus the
  explicitly accepted global World-chronicle sort in D6. Unindexed reverse holder,
  establishing-Activity, reverse-voided and global `request_id` probes are absent;
  operation filtering applies only to loaded rows; tables without a primary key
  fail closed; migration success is resolved inside a fixed newest-row window. No
  game index is added. The former Studio-only loopback `GET /api/entity` and
  `GET /api/entity/{entity_id}` reads are absent from the game HTTP surface; MCP is
  unchanged.
- 2026-08-17, D8: one Markdown builder brief is rendered by the same Rust
  projection as `/brief` in Studio and as `cargo brief` in the terminal (repository
  content without a database; live summary when reachable); it contains pointers and
  current fields only and is never an authority; the build skill points to it as a
  first orientation read.
- 2026-08-17, D9: Studio shows no working-tree/git panel and spawns no external
  process; working-tree state remains a terminal concern.
- 2026-08-17, D10: the documentation lint (home mapping, role headers, front
  matter, live-scope links and anchors, model-to-migration table completeness,
  capability document coverage) is one Rust function that fails `cargo test` and is
  shown as clickable warnings on the Studio Overview and in the brief.
- 2026-08-17, D11a refined after built-page review: Studio keeps one fixed primary
  navigation and a complete stable tree, but long catalogs are disclosed behind
  visible section hubs and open around the current page. Mobile has one disclosure
  for primary navigation, Jump and the local tree. Breadcrumbs, type/status,
  provenance, Related panels and copy actions appear only when they add information;
  Jump resolves models, capabilities and terms to their specialized canonical page.
- 2026-08-17, D11b refined after built-page review: the light interface remains calm
  and typography-first, now with AA contrast for every meaningful label. Sans owns
  application hierarchy, serif authoritative long prose and mono ids, routes,
  paths and code; secondary metadata, raw payloads, facets and task detail use
  progressive disclosure rather than permanent chrome or nested cards.
- 2026-08-17: Studio styling is one hand-written CSS design system (tokens plus
  component classes) with no Tailwind, CSS build tool, generated stylesheet or CDN.
- 2026-08-17: canonical navigation follows the implemented one-home structure:
  tools use `/game/tool/<name>`, repository-backed Development detail remains at
  `/doc/<repository path>`, Live Activity query state is paging only and table rows
  stay on `/live/storage/<table>?after`. No alias or compatibility route is kept.

## Accepted repository topology

- 2026-08-17: `game/`, `dev/` and `studio/` are the three canonical repository
  roots. The root contains only the Cargo workspace and ecosystem entry files that
  must be discovered there.
- `game/` owns the World, wire protocol, HTTP, player MCP, Agent contract,
  migrations, runtime contracts, game tests and player adapter. `studio/` owns the
  read-only development application and the combined local binary. `dev/` owns
  plans, backlog, skills, concept/research/evidence records, lab and playtests.
- The player MCP is compiled only by the game package and served at `/mcp`. A future
  development MCP must be a separately configured process under `dev/`; it may not
  join the player catalog or be started by the game server.
- `dev/skills/build-aicadia` is the canonical skill source. Root
  `.agents/skills/build-aicadia` and `.claude/skills/build-aicadia` are relative
  discovery symlinks for their respective clients and contain no duplicated truth.
- Current files and retained history use the new paths directly. No compatibility
  aliases preserve the former project layout.

## Built repository topology

The accepted three-root topology is now implemented. The root Cargo workspace has
exactly the `game`, `dev` and `studio` members; player MCP code and published text
live exclusively under `game/`; development records, runners, private playtest
state and the canonical skill live under `dev/`; and Studio source, presentation,
tests and launcher live under `studio/`. Root `.agents` and `.claude` contain only
client discovery symlinks. A future development MCP remains a separate, unbuilt
capability: this reorganization created its ownership boundary but no placeholder,
server or tool.

## Open decisions

- Whether a future durable World identity scopes one shared database or selects
  isolated World deployments/databases.
- Whether later evidence needs retained schema-snapshot comparison or diff tooling;
  the selected first capture is explicit, downloadable and non-persistent.
