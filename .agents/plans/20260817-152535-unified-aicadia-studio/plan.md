---
status: complete
created_at: "2026-08-17T15:25:35+02:00"
updated_at: "2026-08-17T16:00:53+02:00"
accepted_at: "2026-08-17T15:29:42+02:00"
completed_at: "2026-08-17T16:00:53+02:00"
---

# One production-shaped Aicadia Studio with navigable Game and Live truth

> **Role / side:** proportional Aicadia Studio build plan / development side.
> **Authority:** owns the bounded execution state for replacing the comparison UI with one source-backed development environment and explicit storage-schema capture.
> **Excludes:** game behavior, a World-data backup, durable schema-snapshot history and future multiple-World identity; see `docs/game/`, `docs/concept/aicadia-studio.md` and later accepted work.

## Outcome

The developer runs the unchanged `cargo dev` command and uses one coherent, English
Aicadia Studio instead of choosing among prototypes. `Game` gives direct, status-
aware navigation through current sources, all current domain models and persistence
seams, and
their realized storage relations, exact MCP tools and development state. `Live`
gives a clear, bounded path from one connected World to Entity, Character, Place,
Activity and storage details. Every supported resource has a reload-safe deep link
and a copyable reference that identifies its owning source or durable id for an AI
conversation.

Rust remains the application layer. It projects source headings, semantic model
sections, the exact compiled MCP catalog and current PostgreSQL structure. An
explicit GET download captures a deterministic JSON storage-schema snapshot with
capture time, latest applied migration, tables, columns, keys, constraints,
foreign-key edges, indexes and a SHA-256 structural fingerprint. It includes no
World rows and writes no database, repository or server state.

The exact evidence claim is: one responsive and keyboard-usable Studio replaces all
variant controls; its deep links and copied references reopen the requested Game or
Live resource; model views combine owning Markdown meaning with current introspected
fields and relations without browser-authored duplicates; Live cross-links resolve
to bounded current reads; schema capture is complete for the bounded public
application schema, reproducible in structure and non-persistent; existing game,
HTTP, MCP, Agent, schema and local-launch behavior remain unchanged.

## Non-goals

- No World-row snapshot, database backup, restore, retained snapshot catalog,
  automatic commit, schema diff or migration generator.
- No game schema or migration, World mutation, new public HTTP/MCP capability,
  Agent-text change, browser gameplay, model invocation or authentication.
- No durable World id, fake World selector or multiple-World storage decision.
- No unrestricted SQL console, repository browser, source editor or code-development
  surface.
- No unindexed global World-data search, exact total row counts, background polling
  or unbounded list. Loaded-list filtering and direct durable-id navigation may be
  provided, but a true search capability requires separately earned indexes.
- No frontend framework, Node/Python application, external runtime or second server
  process.

## Evidence and authorities

| Evidence or authority | Current fact | Consequence for this build |
| --- | --- | --- |
| User choice on 2026-08-17 | Atlas is the preferred base; a permanent prototype selector is unexpected; Game, model, Live, references, navigation, state and accessibility must become one development environment. | Remove the comparison hierarchy and implement one deliberate information architecture rather than selecting a cosmetic variant. |
| `docs/concept/aicadia-studio.md` | Studio is English, Rust-owned, read-only, split into `Game` and `Live`, and projects one home per truth. | Retain those boundaries and record the selected composition there. |
| `docs/game/local-play.md` | `/` is the same-process loopback Studio; Studio is GET-only, bounded and not a player authority. | Preserve the launcher and game boundary while updating only the supported Studio presentation/projection contract. |
| `src/studio/catalog.rs` | Fourteen manually selected sources, six model sections and the exact compiled MCP catalog feed the comparison UI. | Keep authored content in its owning homes; strengthen only structured heading, model-to-storage and reference metadata. |
| `src/studio/live.rs` | Current Entity/Character/Place reads are bounded; storage currently exposes fourteen named tables, columns and single-column FK edges. | Preserve row bounds; deepen structural introspection and reuse it for model relations and explicit schema capture. |
| `web/index.html`, `web/studio.css`, `web/studio.js` | Three duplicated layouts share one in-memory read model; `variant` controls and micro-sized, low-contrast presentation remain. | Replace the duplicated markup/state with one accessible app shell and route-aware render tree. |
| `docs/game/storage.md` and `migration/**` | PostgreSQL migrations and the live database own realized storage; current indexes and constraints are behavior-relevant. | Snapshot the connected schema mechanically; never manually restate its fields or make the export authoritative. |
| `.agents/backlog/items/local-agent-play-ledger.md` | Read-only Studio is delivered development support, while the next game edge remains unselected. | This User-selected follow-up improves the environment that exposes current and future game work without silently selecting a new game capability. |

## Alignment

### Strategic

Studio lowers the concrete risk that accepted game behavior, unsettled exploration,
actual model structure and current World state become impossible to see together as
Aicadia grows. The developer can identify the next game-development edge and cite
the exact source or record without maintaining parallel documentation. This changes
no player or World behavior; the following game risk remains whichever concrete
capability the User next selects from the backlog horizon.

### Tactical

The actor is the local developer. On initial load, explicit navigation, refresh or
schema download, Studio reads existing owning files, compiled tool metadata and
bounded PostgreSQL state. It renders one current view, records navigation in the
URL, copies a reference only after a conscious click and downloads a schema artifact
only after a conscious click. No accepted World state changes, so no Activity
footprint applies. Missing source, partial World data, absent Character context,
unknown ids, truncated Entity state, snapshot bounds and clipboard failure remain
explicit visible states rather than silent fallbacks.

### Technical

- `World` behavior, migrations, mutation transactions, idempotency and locks: not
  changed. Existing World-backed reads and Studio-only read SQL remain separated.
- PostgreSQL: bounded catalog queries read ordinary `public` tables while excluding
  SQLx internals. Explicit hard caps protect table, column, constraint and index
  cardinality. No row count or row scan is introduced; millions of World records and
  one hot Entity do not alter schema-query work.
- Catalog: Rust returns stable source/model ids, structured headings and small model-
  to-owning-section/table metadata. Semantic content, tool definitions, columns,
  keys and relationships still come from their authorities.
- Snapshot: one shared Rust structural reader returns both the Live storage model
  and a pretty JSON download. SHA-256 is computed over deterministic structure only;
  capture time is excluded from the fingerprint.
- Browser: one HTML/CSS/JavaScript presentation consumes the Rust projection, uses
  history-aware query parameters and contains no rule, field, relation or tool copy.
- HTTP/MCP parity: not applicable because Studio endpoints are operator-only and
  remain absent from game OpenAPI and MCP. Existing catalogs must remain byte-for-
  byte unchanged.
- Scale: every World list retains keyset pagination and hard limits. Schema reads are
  metadata-bounded and independent of row volume; no global correctness state, lock,
  revision, counter or hot row is added.

## Decisions, assumptions and open questions

### Confirmed decisions

- One retained Studio replaces Atlas, Workbench and Observatory modes; Atlas is the
  navigational foundation — User choice recorded in
  `docs/concept/aicadia-studio.md` and the August concept log.
- `Game` owns presentation of source-backed current truth, models, tools and
  development state; `Live` owns presentation of one connected World and its
  storage shape — existing Studio direction.
- Domain-model meaning comes from owning `docs/game/domain.md` sections; model
  fields, constraints and relations come from current PostgreSQL introspection;
  mapping a model to its owning section and tables is navigation metadata, not a
  copied model definition.
- Supported resources get stable deep links and copyable references containing the
  human label, owning path or durable id and local Studio URL — User choice.
- Schema capture is explicit, read-only, downloadable, fingerprinted and
  non-persistent; it captures structure rather than World rows — User request plus
  one-home-per-truth boundary.

### Reversible assumptions

- Existing `/` plus query parameters remains the canonical deep-link surface, for
  example `?section=game&view=models&model=entity` and
  `?section=live&view=entity&entity=<uuid>`. This avoids a new routing framework and
  can later move to path routes without changing resource identity.
- Game navigation uses `Overview`, `Models`, `MCP tools`, `Sources` and `Development`;
  Live uses `Overview`, `Entities`, `Characters`, `Places`, `Activity` and `Storage`.
  Browser use at desktop and 390px will verify that labels and grouping remain clear.
- The persistent model map initially covers the actual current `World` seam, User,
  Entity, Character, Place, Activity, Investigation attempt, Property and Trait.
  Rust metadata may name the associated tables but never repeats columns or FK
  definitions.
- One high-contrast light theme is sufficient for this slice. Dark mode remains a
  separate choice; system/browser forced-color and reduced-motion behavior must not
  break access.

### Open questions

- None changes the accepted outcome or evidence claim. Retained snapshot history,
  schema diffing, indexed World-data search and durable multiple-World selection are
  explicitly later decisions.

## Implementation map

| Surface | Current state | Intended change | Invariants |
| --- | --- | --- | --- |
| `src/studio/catalog.rs` | Flat heading strings, six model excerpts and selected source metadata. | Return stable heading references and complete current model navigation metadata tied to owning sections and storage tables. | Markdown and compiled MCP remain the content authorities; raw HTML stays escaped. |
| `src/studio/live.rs` | Bounded World reads plus table/column/FK introspection over a static table list. | Extract one bounded schema reader covering public application tables, rich columns, keys, checks, FKs and indexes; compute deterministic structure/fingerprint; expose model-ready relations and exact bounded Activity detail for Live references. | No World rows in schema output; no writes; list/detail bounds remain. |
| `src/studio/mod.rs` | Root plus catalog/live GET routes and a three-variant HTML assertion. | Add explicit schema-snapshot download route and assert one application shell with no prototype picker. | Same loopback Rust process; no OpenAPI/MCP exposure. |
| `web/index.html` | Three full page variants and floating prototype selector. | One semantic app shell with skip link, header, primary Game/Live navigation, responsive secondary navigation, main content and accessible status/toast regions. | Presentation only; no authored catalog truth. |
| `web/studio.css` | Variant-specific visual systems, tiny text and inconsistent contrast/focus. | One restrained Atlas-derived design system with readable type, WCAG AA color/focus states, sparse surfaces, responsive single-column/mobile navigation and usable data tables. | No external UI runtime; motion respects reduced-motion; touch targets remain usable. |
| `web/studio.js` | Variant state, partial source links and duplicated mounts. | One route-aware state/render tree, back/forward support, source heading links, model/storage relations, Live cross-links, loaded-data filtering/direct id navigation, copy-link/reference actions and explicit snapshot download. | GET-only; bounded reads; clipboard/download only after User action; failure states visible. |
| `docs/game/local-play.md` | Current contract describes three shareable comparison variants and allowlisted storage metadata. | Describe the single UI, stable references and bounded explicit schema capture. | Studio remains read-only local development context and never gameplay authority. |
| `docs/README.md`, `docs/concept/aicadia-studio.md`, August concept log | Placement and exploration still name prototype switching/final choice as open. | Align placement and selected direction; preserve completed comparison as history. | Each fact has one home; old completed plan/evidence remains historical. |
| `docs/evidence/local-play.md`, Studio tests and launcher suite | Prototype completion evidence covers three variants. | Append bounded completion evidence and update assertions for the unified UI, references and schema artifact. | Do not rewrite historical evidence; existing launcher/game behavior still passes. |

## Execution contract

Root owns outcome, scope, plan state, integration and the final evidence claim. No
delegation is planned. Tasks run sequentially because the Rust response shape and
single browser render tree are coupled.

## Task graph

Allowed states are `pending`, `in_progress`, `completed` and `blocked`.

| ID | State | Depends | Parallel-safe | Objective | Owned surfaces | Evidence |
| --- | --- | --- | --- | --- | --- | --- |
| T1 | completed | — | no | Produce the exact structured Game/model catalog and bounded fingerprinted PostgreSQL schema snapshot in Rust. | `src/studio/**`, focused Rust tests | Nine model/seam projections, stable source headings, bounded Activity detail and live PostgreSQL capture returned 14 tables, 61 columns, 41 constraints, 32 ordered FKs and 37 indexes with a stable structural fingerprint. |
| T2 | completed | T1 | no | Replace three variants with one accessible, deeply linked Game/Live Studio over the T1 projection. | `web/**`, `src/studio/mod.rs` asset assertions | In-app browser inspection at desktop and 390 px proved responsive Game/Live navigation, source/model/table/Entity/Activity deep links, copy references, model relations, Live cross-links, refresh state, schema download affordance, zero horizontal overflow and no warning/error console diagnostics. |
| T3 | completed | T2 | no | Align current contracts and run the full no-regression/evidence ladder. | `docs/game/local-play.md`, `docs/README.md`, `docs/concept/aicadia-studio.md`, August concept log, `docs/evidence/local-play.md`, affected tests/plan | Current authorities describe one Studio; strict Clippy/formatting, 161 executed Rust tests, JavaScript/Bash syntax, disposable launcher lifecycle, GET-only/catalog boundaries, browser matrix and diff integrity passed. |

## Task details

### T1 — Structured catalog and schema capture

**Objective:** Rust provides every datum needed for one non-duplicative model and
storage inspector plus one explicit schema artifact.

**Actions:**

1. Give source headings stable ids/levels and extend model navigation to all current
   persistent domain subjects using owning-section and table-name metadata only.
2. Replace the static-table storage projection with one reusable, bounded public-
   application-schema reader that excludes SQLx internals and groups exact columns,
   primary/unique/check constraints, ordered FKs and indexes deterministically.
3. Add capture metadata, latest applied migration and a SHA-256 fingerprint over the
   deterministic structural payload; serve the same structure to Live and as an
   explicit pretty JSON download.
4. Add one bounded operator-only Activity detail read so an Activity id in Live or
   Entity provenance can resolve and reopen without scanning history.
5. Add focused tests for model coverage, safe Markdown rendering, schema ordering,
   composite relation shape, bounds, fingerprint stability and response headers.

**Invariants:**

- No table field, constraint, relation or tool description is manually copied into
  browser code or documentation.
- Public application-schema discovery returns metadata only and fails explicitly at
  its hard cap; it never scans World rows.
- Existing game endpoints, MCP catalog and migrations do not change.

**Evidence:**

- `cargo test studio --all-features` — focused catalog, schema and route behavior.
- Inspection of schema JSON against the disposable migrated database — exact current
  table, column, composite constraint, FK and index examples plus stable fingerprint.

**Stop conditions:**

- Stop if accurate introspection requires a migration, database write, unrestricted
  row access or a second authored schema model.

### T2 — One navigable and accessible Studio

**Objective:** The developer can traverse and cite current Game and Live resources
without understanding prototype modes or losing context.

**Actions:**

1. Rebuild the three HTML hierarchies into one semantic shell with a stable header,
   Game/Live switch, context navigation, current connection/read timestamp and
   responsive mobile disclosure.
2. Implement Game overview, model, MCP-tool, source and development-state views.
   Render source outlines and internal links; combine semantic model text with exact
   storage columns and accessible relation rows.
3. Implement Live overview, Entity, Character, Place, Activity and Storage views
   with keyset paging, related-record navigation, explicit truncation/read states,
   schema-table detail and snapshot download.
4. Define URL serialization/parsing, `popstate`, focus restoration and stable resource
   references. Add copy-reference/copy-link affordances with clipboard fallback and
   visible `aria-live` feedback.
5. Replace the three visual systems with one readable Atlas-derived palette and type
   scale; verify focus, selected/current states, contrast, overflow, reduced motion,
   touch targets and mobile navigation.

**Invariants:**

- Browser assets contain presentation labels and resource ids only, never copied
  game rules, fields, relationships or tool definitions.
- Every list remains bounded; filtering is honest about loaded data and direct id
  lookup does not imply global search.
- A reload or browser back/forward action restores the same supported resource.
- Studio remains entirely read-only with no background polling or Agent invocation.

**Evidence:**

- Browser inspection at 1440px and 390px — every view, mobile navigation, keyboard
  path, visible focus, selected/current states, data overflow and empty/error states.
- Direct-load matrix for one source heading, model, tool, Entity and storage table —
  URL restores the exact resource and copied Markdown reference contains its local
  URL plus owning path or durable id.
- Browser network/console inspection — GET-only Studio traffic, successful JSON
  download and no warning/error output.

**Stop conditions:**

- Stop if deep-link identity requires durable multi-World identity, unbounded search,
  browser-owned domain truth or a new frontend runtime.

### T3 — Contract alignment and final evidence

**Objective:** The repository describes and proves exactly the unified read-only
Studio, with the comparison retained only as history.

**Actions:**

1. Update the current local-operation and placement contracts, selected Studio
   concept and backlog pointer where materially affected; append rather than rewrite
   completed comparison evidence/history.
2. Replace prototype-specific assertions with unified-shell, reference, schema and
   GET-only boundary assertions while preserving the complete launcher lifecycle.
3. Run the validation ladder, inspect the complete focused diff and record only the
   evidence actually demonstrated.

**Invariants:**

- Historical completed plans and evidence remain historical.
- No current authority still describes selectable layout variants after completion.
- Unrelated existing working-tree changes are preserved.

**Evidence:**

- `cargo fmt --all -- --check`.
- `cargo clippy --all-targets --all-features -- -D warnings`.
- `DATABASE_URL=postgres://localhost:5433/postgres cargo test --all-targets --all-features`.
- `bash -n tools/aicadia-local tests/aicadia-local.sh` and the disposable launcher
  lifecycle test.
- `git diff --check`, targeted stale-token scan and focused diff review.

**Stop conditions:**

- Stop and return the plan to `draft` if implementation changes game behavior,
  schema, public capability, durable identity, snapshot persistence or the evidence
  claim.

## Validation ladder

1. **Focused:** Rust catalog/schema/route tests plus browser direct-load, reference,
   navigation and snapshot-download checks.
2. **Contract:** all-target/all-feature Rust tests, strict Clippy, formatting,
   GET-only/MCP/OpenAPI boundary checks and the disposable local launcher lifecycle.
3. **Outcome:** from one fresh `cargo dev`, navigate Game and Live at desktop/mobile,
   open a source heading/model/tool/Entity/table through a copied deep link, inspect
   exact model/storage relations and download a fingerprinted schema JSON containing
   no World rows.
4. **Integrity:** `git diff --check`, focused diff review, stale variant-language scan
   outside frozen history and confirmation that unrelated user changes and all
   governing authorities remain intact.

## Change control

Refine paths, response fields, task order, navigation labels and stronger evidence in
place while the accepted single-Studio, read-only, non-duplicative outcome remains
unchanged. Stop implementation, keep or return `status: draft`, revise and request
explicit re-acceptance for any game/public contract change, migration, World-data
snapshot, persistent snapshot history, authentication, durable World identity,
unbounded search, external service, token spend or materially different evidence
claim.

## Completion conditions

- T1–T3 are `completed` and the validation ladder passes;
- one fixed Studio replaces every prototype chooser and stale current variant claim;
- Game, model, MCP, development-state, Live and storage navigation is clear on
  desktop and mobile, with supported deep links and copyable references;
- semantic meaning, compiled tools and PostgreSQL structure each remain sourced from
  their one authority;
- explicit schema capture is bounded, fingerprinted, downloadable, contains no World
  rows and persists nothing automatically;
- current operation, concept direction, placement, evidence and tests agree;
- no material open question, known stale authority or accidental unrelated change
  remains; and
- `status: complete` and `completed_at` are recorded only after those conditions.

## Completion evidence

Completed on 2026-08-17. The live `aicadia_local` capture returned 14 tables, 61
columns, 41 non-foreign-key constraints, 32 ordered foreign keys and 37 indexes with
a stable structural fingerprint. Desktop and 390 px in-app browser inspection
covered Game/Live navigation, direct source/model/Entity/Activity/storage links,
copyable references, mobile disclosure, zero horizontal overflow and an empty
warning/error console. Strict Clippy, Rust formatting, JavaScript/Bash syntax,
`git diff --check`, all 161 executed Rust tests and the complete disposable launcher
lifecycle passed; the accepted-contract fixture generator remained explicitly
ignored. Detailed claims and boundaries remain in `docs/evidence/local-play.md`.
