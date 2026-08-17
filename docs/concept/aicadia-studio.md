# Aicadia Studio direction

> **Role / side:** active development-interface exploration / development side.
> **Authority:** owns the confirmed Aicadia Studio direction, rationale and open design decisions.
> **Excludes:** current game behavior, delivery evidence and final production implementation; see `docs/game/`, `docs/evidence/` and an accepted build plan.

Status: selected direction delivered; a convention-driven development-environment
extension is proposed and under grill.

## Question

How can the developer step back from implementation and inspect Aicadia's accepted
game meaning, models, tools, live World state, unresolved questions, experiments and
evidence in one clear interface without creating a second documentation authority?

## Confirmed direction

- The application is named **Aicadia Studio** and its interface is English.
- `Game` and `Live` are its two primary sections (superseded on 2026-08-17 by D2
  below: `Overview · Game · Development · Live`).
- `Game` is repository/build-scoped. It projects accepted documentation, domain
  meaning, structural models, MCP tools, rules, open questions, experiments,
  evidence and decision history from their existing owning homes.
- `Live` is connection-scoped. It browses one selected World's accepted records and
  storage shape read-only, with Entity, Character, Place, Property, Trait and
  Activity presented as related domain views rather than unrelated table dumps.
- Mechanically available structure is generated or introspected; semantic meaning
  is authored once in its owning Markdown source and rendered directly.
- Aicadia application logic, source parsing, catalog assembly, database access and
  server behavior are always Rust. No Python or Node application is introduced.
- The existing Rust local application and its `/` ledger are rebuilt into Studio;
  `cargo dev` remains the single command. The ledger is absorbed by `Live`, not
  retained as a second browser.
- Existing HTML/CSS/JavaScript remains only browser presentation, as in the current
  application. It owns no rules, models, schemas, catalog or database logic.
- Small project-structure changes are allowed only when they materially improve
  overall clarity and do not create parallel truth.
- The completed three-variant comparison does not remain a product mode. Retain one
  coherent application with Atlas's source-first hierarchy as its base, a dedicated
  model inspector using the Workbench's strongest structural ideas and a much clearer
  Live browser using the Observatory's strongest World-inspection ideas.
- Every navigable source, model, MCP tool, storage object and live record receives a
  stable deep link and a copyable development reference suitable for an AI
  conversation. References identify the owning source or durable record; they never
  copy its truth into browser assets.
- The storage view may explicitly capture a downloadable schema snapshot containing
  the exact current table, column, constraint, foreign-key and index definitions.
  Rust derives it from the connected PostgreSQL schema, fingerprints it and stores
  it nowhere automatically; it is evidence from one moment, never a second schema
  authority or a World-data backup.
- A Rust static generator may be earned later only when one concrete source cannot
  be projected clearly at runtime. Generated output remains rebuildable and never
  authoritative.

## Current boundaries

The current game has one persistent World. Its concrete `World` type is a behavior
seam around one PostgreSQL pool, `get_world` returns constant name `Aicadia`, and no
durable `WorldId`, `world` table or `world_id` scope exists. Studio may show one
selected local connection and a future-compatible selector shape, but cannot present
multiple Worlds as delivered behavior or choose their storage topology.

The existing browser is a loopback, read-only local development surface served by
the Rust application. Studio may expand its out-of-world inspection and project
development documentation, but Agent conversation remains the only gameplay
interface. Studio never mutates the World, publishes a player capability, invokes a
model or becomes fallback authority for a player Agent.

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
`.agents/plans/20260817-140952-aicadia-studio-prototype/plan.md`. It compares three
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
`.agents/plans/20260817-152535-unified-aicadia-studio/plan.md`.

## Proposed next direction — convention-driven development environment

On 2026-08-17 the User found the unified Studio still insufficient for stepping
back: the rules player Agents receive, the models and their relations, decided
versus open choices, work in progress, experiments and the live World are not yet
visible together, and maintaining Studio must never become a separate chore beside
maintaining the repository. The draft plan
`.agents/plans/20260817-161615-studio-development-environment/plan.md` proposes
that Studio owns no content and no allowlist: it discovers governed repository roots
by convention, reads record metadata from front matter that has exactly one home,
joins compiled catalogs and the connected schema, runs the same parsers inside
`cargo test`, and renders one path-routed `Overview · Game · Development · Live`
environment plus a Markdown builder brief. It also proposes one folder per model
under `docs/game/model/<model>/`. Nothing below is accepted until that plan is.

## Confirmed direction under the development-environment grill

- 2026-08-17, D1: Studio pages are Rust server-rendered HTML with one stable path
  route per resource; the JSON API and browser route/state machine are retired once
  replaced and browser code is one small enhancement script.
- 2026-08-17, D2: primary navigation becomes `Overview · Game · Development · Live`,
  mirroring the runtime side, development side and connection of the documentation
  constitution; this supersedes `Game` and `Live` as the only two primary sections.
- 2026-08-17, D3: the current game contract gets one folder per model under
  `docs/game/model/<model>/` with `README.md` as the contract entry (nine folders:
  world, user, entity, character, place, activity, property, trait,
  investigation-attempt); `docs/game/domain.md` remains as the overview and
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
- 2026-08-17, D7: Live expands to bounded operator-only reads of Users,
  investigation attempts, Property keys, Trait lineages, full Entity history, an id
  resolver and a generic bounded row viewer over every introspected `public` table;
  the Studio-only loopback `GET /api/entity` and `GET /api/entity/{entity_id}` leave
  the game HTTP surface once Studio owns its reads; MCP is unchanged.
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
- 2026-08-17, D11a: Studio navigation is one fixed primary navigation, a complete
  and stable per-section tree sidebar, a breadcrumb/context strip on every page, a
  server-side jump box over every known resource (not a search engine) and a fixed
  Related column on detail pages.
- 2026-08-17, D11b: the visual direction is light mode only, calm and clean,
  typography first, color reserved for status, with equally careful presentation of
  prose records and data; one design is made directly (no mockup variants) and
  reviewed on the built pages.
- 2026-08-17: Studio styling is one hand-written CSS design system (tokens plus
  component classes) with no Tailwind, CSS build tool, generated stylesheet or CDN.

## Open decisions

- Whether later Studio production uses live Rust source parsing, a Rust-generated
  rebuildable catalog or a deliberately small combination. The draft plan proposes
  runtime parsing plus `cargo test` lint and no generated authority.
- Which minimal Markdown metadata is worth standardizing after real navigation gaps
  are observed. The draft plan proposes a fixed per-home front-matter vocabulary.
- Whether a future durable World identity scopes one shared database or selects
  isolated World deployments/databases.
- Whether later evidence needs retained schema-snapshot comparison or diff tooling;
  the selected first capture is explicit, downloadable and non-persistent.
- The concrete Studio design system, made directly under plan T4.0 and reviewed
  by the User on the built pages.
