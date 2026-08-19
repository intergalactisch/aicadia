---
status: complete
created_at: "2026-08-18T15:34:39+02:00"
updated_at: "2026-08-18T16:02:31+02:00"
accepted_at: "2026-08-18T15:38:35+02:00"
completed_at: "2026-08-18T16:02:31+02:00"
---

# Current-truth Development Area workbooks

> **Role / side:** current Development Area source-and-Studio build plan / development side.
> **Authority:** owns the bounded authority migration, six Area workbook shapes, structured Studio projection and exact evidence to be built after acceptance.
> **Excludes:** game behavior, multiplayer or spatial product decisions, current Work selection and research findings; those remain in `game/docs/`, the owning Area after migration, `dev/backlog/` and plans, and `dev/docs/research/`.

## Outcome

An Aicadia developer opens `/dev/areas`, sees the six flat development subjects and
can enter any one of them to understand the current development truth without first
reading a long exploration history. Every Area page makes these distinctions
immediately visible: what the Area means, what it is, what it is not, what has been
chosen, what has been rejected, what is not yet chosen, what still needs research,
which components and concepts it contains and what the current technical model is.

The source of that view is one living work document at
`dev/areas/<area>/README.md`. The build deliberately changes the earlier Area
contract: an Area is no longer only prepared-material overview. It becomes the one
home for the current development synthesis of its subject. Exact executable behavior
still lives only in `game/docs/`; a research report owns sourced findings, a Lab owns
its bounded verdict, Evidence owns delivery claims and Work owns which question is
selected plus its plan and execution state. The final evidence must prove all six
Area sources follow the fixed shape, old current-synthesis duplication is removed or
narrowed to historical rationale, Studio renders every distinction from source and
the Work boundary remains intact.

## Non-goals

- Do not change World behavior, PostgreSQL, migrations, HTTP, MCP, tool text, Agent
  conduct or the accepted game contract.
- Do not make Studio editable or store state in the browser, database or generated
  assets.
- Do not add an `Exploration` Area, nested Area hierarchy, tags, search, ownership
  graph or manual Rust allowlist of Area names.
- Do not copy exact schema fields, capability rules, research findings, lab verdicts,
  evidence claims, backlog order, plan tasks or current execution status into an
  Area workbook; point to their owning sources and add only the Area-owned synthesis.
- Do not treat `This is not`, `Rejected` and `Not yet chosen` as synonyms. A boundary,
  a deliberate decision and an unresolved choice must remain visibly different.
- Do not rewrite the append-only concept log or frozen history. Retained concept
  records may preserve rationale and superseded exploration after their current
  synthesis moves.
- Do not introduce a dashboard framework, client-side tabs, decorative metrics,
  icons or a new design language. Use the existing source-first Studio composition.

## Evidence and authorities

| Evidence or authority | Current fact | Consequence for this build |
| --- | --- | --- |
| User direction, 2026-08-18 | The User accepted `Multiplayer`, `Place`, `Movement`, `Discovery`, `Agent Play` and `World Change` as a flat set, allows overlap and requires each Area to show current truth including positive/negative boundary, decisions, gaps, concepts and technical model. | Build exactly these six sources and make their distinctions first-class in Studio. |
| `dev/docs/concept/aicadia-studio.md` | Studio is convention-driven and source-backed; Area direction now names the six flat workbooks and their authority split. | Area content is authored once in `dev/areas/`; Rust parses and renders it without owning semantic copy. |
| `dev/docs/README.md` | The home table currently says Areas own only established/prepared overview and exclude open questions and accepted game behavior. | Redefine the Area home to own current development synthesis while preserving the other homes' exact responsibilities. |
| `dev/areas/README.md`, `dev/areas/multiplayer/README.md` | One Area exists and its README is a short pointer page; no fixed current-truth shape exists. | Expand the index to six entries and adopt one validated workbook shape for every Area. |
| `dev/docs/concept/concurrency-and-world-dynamics.md` | A 1,170-line active record mixes confirmed Multiplayer direction, candidates, open catalogue, rationale and superseded exploration. | Inventory and move only current synthesis to Multiplayer; retain rationale/history without leaving two live homes. |
| `dev/docs/concept/spatial.md` | One live record currently combines Place and Movement direction and the active spatial frontier. | Split the current synthesis between Place and Movement while preserving shared rationale as retained source and allowing both Areas to link it. |
| `dev/docs/concept/discovery.md` | A 447-line live record mixes delivered direction, corrections, prototype history and research basis. | Move current Discovery synthesis to its Area and keep the old record as retained rationale/history. |
| `game/docs/`, `dev/docs/research/`, `dev/lab/`, `dev/docs/evidence/` | Exact runtime contracts, sourced findings, experimental observations and delivery claims already have distinct owners. | Area technical-model and source sections use explicit pointers and status layers; they never restate exact owned facts. |
| `studio/src/page/area.rs` | The current generic Area page renders a lede, prepared records and related links; only the scenario catalogue has structured parsing. | Add one fail-closed `AreaWorkbook` parser and source-derived overview/detail compositions while preserving the scenario page. |
| `studio/src/plan.rs` | Global open projection reads `Open …` headings from live concepts, plans and backlog items, not Areas. | Include `Not yet chosen` and `Research needed` from Area workbooks after current synthesis moves; Work still owns priority and task state. |
| Local `design` skill and applicable guidelines | The existing app must use clear labels, light surface separation, responsive one-column collapse, accessible navigation and plain text status rather than icon-only or color-only meaning. | Use a boundary split, decision-state groups, source tables/lists and progressive disclosure with existing HTML/CSS; verify desktop and mobile. |
| `game/docs/local-play.md` and `dev/docs/evidence/local-play.md` | The current contract/evidence describe Areas as prepared overview only. | Update them only after the changed authority and interface are delivered and verified. |

## Alignment

### Strategic

The current highest-leverage design edge is spatial multiplayer, but its current
truth is distributed across a very long concept record, fresh research, scenario
catalogue, backlog and game contracts. Making Area workbooks authoritative gives the
developer a reliable place to understand that edge before choosing or building the
next gameplay scene. The same structure makes Discovery, Agent Play and World Change
navigable without allowing documentation work to masquerade as game progress. The
next concrete game risk remains the choice and contract for the first Place/movement
scene; this build exposes its truthful basis but does not make that choice.

### Tactical

The developer enters `/dev/areas`, scans six areas and sees a short definition plus
source-derived counts for chosen, rejected, unchosen and research-needed statements.
Opening one Area presents `This is` and `This is not` together, then the four current
state groups, components/concepts, a layered technical model and owning sources.
Prepared records such as the Multiplayer scenario catalogue remain directly
navigable. Durable unresolved landscape belongs in the Area; `/dev/work` alone says
which question or outcome is selected and carries plans, tasks and backlog state.

### Technical

- **World, PostgreSQL, transactions, concurrency, idempotency, HTTP, MCP and Agent
  surfaces:** not applicable. This is a repository-source and read-only local Studio
  build; no game adapter or database state changes.
- **Authority migration:** `dev/areas/<area>/README.md` owns one fixed current
  development synthesis. Touched live concept records surrender that synthesis and
  become retained rationale/history or keep only a narrower fact not represented by
  an Area. A move inventory prevents loss.
- **Workbook grammar:** require unique headings in this order: `Meaning`; `Boundary`
  with `This is` and `This is not`; `Decisions` with `Chosen`, `Rejected` and `Not
  yet chosen`; `Research needed`; `Components`; `Technical model` with `Delivered`,
  `Directional` and `Absent`; and `Sources`. Decision and boundary groups use
  explicit bullet items so source/UI parity is deterministic.
- **Projection:** add a Rust `AreaWorkbook` parser beside the scenario parser. It
  joins only governed `development-area` records, rejects missing/duplicate/out-of-
  order headings and emits semantic server-rendered markup. Area names remain folder
  conventions, not a Rust list.
- **Presentation:** retain Studio's existing typography, compass/ink palette,
  authority plate, Related column and responsive shell. Use the lightest separation:
  a two-part boundary band, labelled decision groups, direct prose for components and
  model, and linked prepared material. Meaning is never communicated by color alone;
  mobile collapses to one column without horizontal overflow.
- **Open/work projection:** Area `Not yet chosen` and `Research needed` sections are
  discoverable as durable open landscape. Plans/backlog remain the only current edge,
  order and execution source.
- **Evidence:** parser/unit tests, real-repository shape lint, HTTP source parity,
  route and Work-boundary tests, full Studio validation and real desktop/mobile
  browser review.

## Decisions, assumptions and open questions

### Confirmed decisions

- The flat first set is Multiplayer, Place, Movement, Discovery, Agent Play and
  World Change; overlap is allowed and Exploration is not a separate first Area —
  User decision recorded in the Studio concept and August log.
- Area workbooks own current development synthesis, including durable unchosen and
  research-needed landscape; Work owns only current selection, planning and
  execution — User decision recorded in the Studio concept and August log.
- Exact runtime behavior, research findings, experimental verdicts and delivery
  evidence keep their existing homes — User direction plus the documentation
  constitution's one-home rule.
- Studio must make positive boundary, negative boundary and each decision state
  visibly distinct — explicit User interface requirement.
- `/dev/areas` remains the canonical route; the User's `/areas/` wording refers to
  the existing Areas section and does not authorize a second root alias.

### Reversible assumptions

- Use one fixed Markdown shape for all Area READMEs — consistent parsing and visual
  comparison justify the constraint; focused source fixtures prove it before content
  migration.
- Use bullet items for boundary and decision groups, while Meaning, Components,
  Technical model and Sources may use prose and tables — this gives deterministic
  status parity without forcing all domain explanation into cards.
- Preserve long concept records as retained rationale/history rather than deleting
  them — their decision trail remains useful, while a role/status change and current
  pointer remove their authority over the migrated synthesis.
- Show source-derived counts on `/dev/areas` only as navigation metadata, never as
  progress, rank, quality or score — labels and content remain primary.
- Keep all Area pages on the generic `/dev/areas/{area}` handler and all prepared
  record routes convention-driven — no new per-Area Rust registration is needed.

### Open questions

- None. The plan exposes the authority migration, fixed workbook shape, six names,
  Work boundary and UI composition for explicit acceptance before implementation.

## Implementation map

| Surface | Current state | Intended change | Invariants |
| --- | --- | --- | --- |
| `dev/areas/README.md` | Lists only Multiplayer and excludes all open questions. | List six flat Areas and define the index as navigation to current development syntheses. | No work priority, plan status or copied area truth. |
| `dev/areas/multiplayer/README.md` | Short prepared-material pointer page. | Adopt the full workbook shape and own current Multiplayer synthesis plus its scenario pointer. | Scenario catalogue remains prepared pressure, not a game contract or verdict. |
| `dev/areas/{place,movement,discovery,agent-play,world-change}/README.md` | Absent. | Create five complete Area workbooks from current owning sources and explicit pointers. | One folder per flat Area; singular English domain naming. |
| `dev/docs/concept/{concurrency-and-world-dynamics,spatial,discovery}.md` | Live/active current synthesis mixed with rationale/history. | Inventory and move current synthesis, then reclassify/narrow these records as retained rationale with explicit Area owners. | No decision or rationale is lost; no two current homes remain. |
| `dev/docs/concept/{entity-state,interaction}.md`, `game/docs/agent.md`, related concept records | Retained rationale or exact runtime owners used by Agent Play and World Change. | Keep owned facts in place and add only required Area-owner pointers when a reader could otherwise edit the wrong home. | Exact capability and model truth remains in `game/docs/`. |
| `dev/docs/README.md`, `dev/docs/methodology/build-text.md` | Area home owns prepared overview; no Area document shape exists. | Redefine Area authority and add the fixed Area workbook shape/pointer rules. | One home per truth and adopt-on-touch continue to govern every source. |
| `studio/src/page/area.rs` | Generic Area list/detail plus structured scenarios. | Parse/render workbook sections, index summaries, boundary split, decision groups, components, technical model, sources and prepared material. | Invalid sources fail closed; all semantic text comes from repository records. |
| `studio/src/plan.rs`, `studio/src/page/development.rs` | Open projection ignores Areas and still describes concepts as the live current synthesis. | Project Area unchosen/research sections and update explanatory copy while Work remains plans/backlog/tasks. | No Area statement becomes plan state or backlog order. |
| `studio/src/page/tree.rs` | Discovers Area folders and records generically. | Preserve convention discovery and verify all six Areas appear without registration. | Navigation order is deterministic and mobile menu remains usable. |
| `studio/web/studio.css` | Area list and scenario-specific responsive rules only. | Add minimum boundary, decision-state, component/model and source presentation rules using existing tokens. | No new framework, icon set, client state or color-only status. |
| `studio/src/home.rs`, `studio/src/record.rs`, `studio/tests/studio/**` | Area records are governed but workbook shape is not linted or tested. | Add shape/parity fixtures, repository lint, six-route coverage, open/work boundary checks and source-count assertions. | Tests derive names and counts from governed sources where possible. |
| `dev/docs/concept/aicadia-studio.md`, `dev/docs/concept/log/2026-08.md` | New User direction recorded; build remains planned. | On accepted execution, keep plan and delivery state current and append one bounded completion entry. | History remains append-only; no game choice is implied. |
| `game/docs/local-play.md`, `dev/docs/evidence/local-play.md` | Describe prepared Area overview only. | Describe and evidence the source-backed current-truth workbooks after delivery. | Claim only repository projection and exercised browser presentation. |

## Execution contract

Root owns outcome, authority migration, content integrity, plan state, Studio
integration and the final evidence claim. No delegation is planned. Tasks are
dependency-ordered because Area content, concept ownership, parser grammar and final
evidence touch one source/projection contract. Preserve unrelated spatial research
and concurrent user changes; stop rather than overwrite an actively changed source.

## Task graph

Allowed states are `pending`, `in_progress`, `completed` and `blocked`.

| ID | State | Depends | Parallel-safe | Objective | Owned surfaces | Evidence |
| --- | --- | --- | --- | --- | --- | --- |
| T1 | completed | — | no | Establish the Area workbook authority and grammar without duplicating another home. | `dev/docs/README.md`, `dev/docs/methodology/build-text.md`, `studio/src/page/area.rs` parser fixtures | Documentation lint, fixed-shape parser tests and source ownership review. |
| T2 | completed | T1 | no | Author six current-truth workbooks and migrate overlapping live concept synthesis with a lossless inventory. | `dev/areas/**`, touched `dev/docs/concept/**`, required source pointers | Six real workbooks parse; current/rejected/open/research/model inventory review; no duplicate live owner. |
| T3 | completed | T2 | no | Render source-backed Area index/detail workspaces and include their durable gaps in open projection. | `studio/src/page/area.rs`, `studio/src/plan.rs`, `studio/src/page/{development,tree}.rs`, `studio/web/studio.css` | Unit and HTTP parity tests; all six routes; invalid-source failure; Work boundary. |
| T4 | completed | T3 | no | Align current contracts, decisions and evidence and prove the complete responsive result. | `studio/tests/studio/**`, `game/docs/local-play.md`, Studio concept/log, local-play evidence, this plan | Full Studio suite, Clippy, fmt, cargo brief, docs lint, desktop/mobile browser review and diff audit. |

## Task details

### T1 — Establish the Area workbook contract

**Objective:** one documented and executable source shape defines current Area
synthesis without pretending that its durable gaps are current Work.

**Actions:**

1. Redefine the Area home and add its exact document shape and pointer obligations.
2. Add a parser fixture covering every required section and state distinction.
3. Make malformed, duplicate or out-of-order source fail with a named error. T2
   adds the six index entries atomically with their complete workbooks.

**Invariants:**

- Area owns synthesis only; exact game rules and external findings are pointers.
- `Rejected`, `Not yet chosen`, `Research needed` and `This is not` stay distinct.
- No Area name is hard-coded in Rust.

**Evidence:**

- Focused Area parser tests — the valid fixed shape parses and each malformed shape
  fails closed with its source distinction named.
- Documentation lint and manual change-coupling review — every current fact has one
  editable owner.

**Stop conditions:**

- Stop if useful Area truth requires copying exact runtime contracts or research
  findings instead of owning a higher-level synthesis.

### T2 — Build the six current-truth workbooks

**Objective:** every accepted Area route has one useful source whose current meaning,
boundaries, decision states, research needs, components and model agree with existing
authorities.

**Actions:**

1. Author and index Multiplayer, Place, Movement, Discovery, Agent Play and World
   Change in the fixed shape, reusing the scenario catalogue only as Multiplayer
   prepared material.
2. Inventory each current statement moved from Multiplayer, spatial and Discovery
   concept records; keep, move or replace it with one explicit owner pointer.
3. Reclassify or narrow the old concept records so they preserve rationale/history
   without remaining a competing current synthesis.

**Invariants:**

- Delivered, Directional and Absent technical model layers never blur.
- Overlap between Area lenses uses shared pointers, not copied facts.
- Concurrent spatial research/user changes remain intact.

**Evidence:**

- Real-repository Area parser and index lint — exactly six complete workbooks are
  discoverable and every source link resolves.
- Focused migration inventory/diff review — no confirmed, rejected, open or research
  item is silently lost and no old file still claims the migrated current truth.

**Stop conditions:**

- Stop if an Area requires a new product/domain decision, or if a source under
  concurrent change cannot be migrated without overwriting unrelated work.

### T3 — Render the Area workspaces

**Objective:** `/dev/areas` and every Area detail make current truth visually
scannable and remain usable on narrow screens.

**Actions:**

1. Render source-derived Area summaries with clearly labelled decision metadata.
2. Compose detail pages with adjacent `This is`/`This is not`, distinct Chosen/
   Rejected/Not yet chosen/Research needed groups, and readable Components,
   Technical model, Sources and prepared material.
3. Add Area gaps to the global open projection while keeping current edge, plan and
   tasks under Work.

**Invariants:**

- Text labels carry meaning independently of color.
- No semantic content or Area registry enters CSS, JavaScript or Rust constants.
- Existing scenario, Game, Live, Work and mobile navigation behavior remains intact.

**Evidence:**

- Focused unit/HTTP tests — source counts, headings, routes, deep links and open/work
  ownership agree; malformed source returns an honest error.
- Static accessibility/markup review — semantic headings/lists, no icon-only state,
  keyboard-native links/disclosures and valid responsive structure.

**Stop conditions:**

- Stop if scanability requires client-side state or a generic dashboard abstraction.

### T4 — Prove and record delivery

**Objective:** current documentation and executable Studio evidence agree on the new
Area authority and interface without claiming a game capability.

**Actions:**

1. Update current local-play contract, Studio direction, decision log, delivery
   evidence and plan state at the earned moment.
2. Run focused and complete validation, then review real pages at desktop and narrow
   mobile width using the in-app browser.
3. Audit the exact diff, old concept authorities and unrelated work before marking
   the plan complete.

**Invariants:**

- Evidence claims only source projection, routing, visual distinctions and
  responsive behavior.
- No World, database, capability, Agent or multiplayer-correctness claim is added.

**Evidence:**

- `cargo fmt --all -- --check` and strict all-target Studio Clippy.
- `DATABASE_URL=… cargo test -p aicadia-studio` — full Studio unit/integration suite.
- `cargo brief`, documentation lint and `git diff --check`.
- Real `/dev/areas` plus representative Multiplayer, Place, Movement and World
  Change pages at desktop and approximately 390 px mobile — all required distinctions
  visible, no horizontal overflow and usable navigation.

**Stop conditions:**

- Stop if evidence reveals a stale authority, hidden overflow or UI distinction
  carried only by color.

## Validation ladder

1. **Focused:** Area grammar fixtures, real six-source parse, concept-move inventory,
   open-section projection and HTTP route/source parity.
2. **Contract:** complete Studio suite, documentation lint, generated brief and
   verification that Work alone owns current selection/tasks while game/docs alone
   owns executable behavior.
3. **Outcome:** real browser review proves a developer can distinguish what each
   Area is/is not, chosen/rejected/open/research states, components and model at
   desktop and mobile without reading the long retained concept history.
4. **Integrity:** `git diff --check`, focused ownership/diff review and confirmation
   that unrelated spatial research and all pre-existing user changes remain intact.

## Change control

Refine parser structure, CSS composition, source wording and stronger evidence in
place while the accepted six Areas, workbook authority, fixed distinctions, Work
boundary and evidence claim remain unchanged. Stop implementation, keep or return
`status: draft`, revise and request explicit re-acceptance when new evidence changes
an Area name, authority home, source shape meaning, public route, game contract,
external side effect or final claim.

## Completion conditions

- every required task is `completed` and the validation ladder passes;
- exactly six flat Area workbooks own their current development synthesis and every
  overlapping live concept record has been narrowed or retained without duplication;
- `/dev/areas` and every Area detail render the fixed distinctions from source,
  while existing scenario routes remain intact;
- durable unchosen/research landscape is visible from its Area and global open
  projection, while Work alone owns selected priority, plans and execution;
- exact runtime, research, Lab and Evidence facts remain in their existing homes;
- current contract, Studio direction, generated brief and delivery evidence agree;
- no known-stale authority, material open question or accidental unrelated change
  remains;
- `status: complete` and `completed_at` are recorded only after these conditions.
