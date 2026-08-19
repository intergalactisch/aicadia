---
status: complete
created_at: "2026-08-18T13:30:15+02:00"
updated_at: "2026-08-18T13:52:27+02:00"
accepted_at: "2026-08-18T13:33:15+02:00"
completed_at: "2026-08-18T13:52:27+02:00"
---

# Short `/dev` routes and source-backed Development Areas

> **Role / side:** current Studio information-architecture build plan / development side.
> **Authority:** owns the bounded route replacement, Development Area source convention, Multiplayer overview and scenario projection to be built after acceptance.
> **Excludes:** game behavior, multiplayer product semantics, current Work state and experiment verdicts; see `game/docs/`, `dev/backlog/`, the multiplayer concept record and `dev/lab/`.

## Outcome

An Aicadia developer can open the concise `/dev` section, browse a conventionally
discovered list of durable Development Areas, enter Multiplayer and scan all fourteen
fixed multiplayer scenarios without first reading the long source record or treating
Lab as their parent. `Work` remains the only Studio destination for the current edge,
open questions and planned or active execution.

This is the smallest safe build that removes the concrete Studio wayfinding blocker
identified by the User while the multiplayer foundation remains an active design
area. It does not advance multiplayer game behavior directly; it makes the already
chosen scenarios and established material inspectable enough to support the next
multiplayer decision without parallel truth. Final evidence must prove that the new
source homes are conventionally discovered, every `/dev` destination renders, the
fourteen source scenarios appear in the overview, and the superseded
`/development` route family is absent.

## Non-goals

- Do not change World behavior, PostgreSQL, HTTP, MCP, Agent text or the current
  multiplayer product contract.
- Do not put current questions, plan tasks, backlog state or delivery status in an
  Area record or Area page.
- Do not make Area membership an experiment verdict, planning state or game
  requirement.
- Do not add `/development` compatibility redirects or aliases.
- Do not build free-text search, client-side routing, editable Studio state, a
  generic dashboard framework or manually maintained Studio area allowlist.
- Do not classify the scenarios as passed, failed or complete when no owning source
  makes that claim.

## Evidence and authorities

| Evidence or authority | Current fact | Consequence for this build |
| --- | --- | --- |
| `dev/docs/concept/aicadia-studio.md` | The User accepted `/dev` as the complete Development prefix, Areas as durable subject overview, Multiplayer as the first Area and scenarios as area-level rather than Lab-owned input. | Build exactly that hierarchy and keep Work separate. |
| `dev/docs/concept/log/2026-08.md#studio-development-route-prefix--resolved` | The route replacement is recorded without compatibility aliases. | All current routes, navigation, generated links and tests move together; old routes must return not found. |
| `dev/docs/README.md` | `dev/` owns development records; Studio discovers governed homes by convention and may not own authored copies or a source allowlist. | Add one explicit `dev/areas/` home and project it generically from Rust. |
| `dev/lab/multiplayer/scenarios.md` | One 14-scenario catalogue already owns fixed narratives, fixtures and questions, but its current Lab path makes it appear subordinate to experiments. | Move the catalogue without rewriting its meaning; its new Area home remains non-contractual input. |
| `dev/lab/multiplayer/README.md` | Lab owns bounded experiments and links the catalogue as reusable input. | Keep experiments in Lab and update their upward pointer to the Area-owned catalogue. |
| `dev/plans/20260816-153410-multiplayer-lab/plan.md` | The draft multiplayer plan reuses the fixed scenario matrix but does not make it current game behavior. | Preserve this authority boundary and update only any live path references affected by the move. |
| `studio/src/home.rs`, `studio/src/record.rs` | Governed records are classified, indexed and linted through one home table. | Add Area index, Area README and Area record classifications plus focused lint tests. |
| `studio/src/page/development.rs`, `studio/src/page/tree.rs`, `studio/src/lib.rs` | Development currently has nine `/development` routes and Lab only projects tracks and experiments; the scenario catalogue is not a first-class page. | Replace the route family and add source-backed Area handlers and navigation. |
| `game/docs/local-play.md#studio-boundary` | The runtime contract names Studio's stable route behavior and progressive disclosure. | Update the current contract to `/dev` and the new Areas overview without changing the read-only boundary. |
| User direction, 2026-08-18 | Areas shows what is being developed and what has been established or prepared; Work owns what is currently open or planned. | The Multiplayer hub may link to owning sources but carries no work board or current-question summary. |

## Alignment

### Strategic

The current multiplayer design work already contains fourteen hard shared-World
scenarios, but Studio exposes them only through a long generic document below Lab.
The build advances the selected development edge by making those game pressures
visible together before further concurrency and World-change choices are made. The
next concrete game risk remains unchanged: the draft multiplayer foundation still
has unresolved semantics and experiments, and this Studio build must not present
them as accepted behavior.

### Tactical

The developer enters `/dev`, chooses `Areas`, opens `Multiplayer` and sees a concise
area description plus its prepared records. The scenario page presents S01–S14 as
scan-first rows or cards with ID, title and primary pressure; each scenario's exact
source-owned detail is available through progressive disclosure and a stable anchor.
The catalogue remains one record. Multiplayer research, decisions and experiments
may be linked as related authorities, but live plan status and open sections appear
only under `/dev/work` and `/dev/open`.

### Technical

- **World, PostgreSQL, transaction, concurrency, idempotency, HTTP, MCP and Agent
  surfaces:** not applicable; Studio remains a read-only local development
  projection and this build changes no game adapter.
- **Repository projection:** introduce convention-based Area homes under
  `dev/areas/`, move the catalogue, and extend the existing home/index lint rather
  than hard-coding Multiplayer content in Studio.
- **Routing:** replace every current `/development` endpoint and generated Studio
  link with `/dev`; add `/dev/areas`, `/dev/areas/{area}` and
  `/dev/areas/{area}/scenarios`; keep no redirect or alias.
- **Presentation:** add one server-rendered scenario overview using existing Studio
  typography, plates, lists, disclosures and responsive rules. JavaScript remains
  unnecessary unless implementation evidence shows the existing server-rendered
  disclosure cannot satisfy the accepted overview.
- **Evidence:** repository/home unit tests, Studio HTTP contract tests, route-absence
  checks, fourteen-scenario projection checks, full Studio tests, documentation lint
  and a local rendered-page review at desktop and narrow width.

## Decisions, assumptions and open questions

### Confirmed decisions

- `/dev` replaces `/development` for the whole Studio Development section, with no
  alias — User decision recorded in the Studio concept and August log.
- `Areas` is durable subject overview; `Work` owns current edge, questions and
  execution — User decision recorded in the Studio concept and August log.
- Multiplayer is the first Area and scenarios sit below it — User decision recorded
  in the Studio concept and August log.
- The scenario catalogue is overview input, not accepted game behavior or Lab
  evidence — current catalogue role header and User correction.
- Studio remains convention-driven and Rust-rendered — accepted Studio direction and
  documentation constitution.

### Reversible assumptions

- Use `dev/areas/README.md`, `dev/areas/<area>/README.md` and direct Markdown children
  such as `scenarios.md` as the smallest source convention — it adds three simple
  home shapes and can support a later Area without code registration; documentation
  lint and the first real folder verify it.
- Keep the fourteen scenarios in one moved catalogue — the current source already
  owns their cross-scenario rules and matrix, and the visual problem can be solved by
  server-rendered progressive disclosure without splitting authority.
- Keep the UI label `Development` while shortening only its URL to `/dev` — the User
  changed path length, not the visible product vocabulary.
- Use the existing scenario-index table as the source of each card's primary
  pressure and each S01–S14 heading as the source of detail — implementation stops if
  those two source structures cannot be joined without inferred content.

### Open questions

- None. The User accepted the complete outcome, source convention, implementation
  seams and evidence claim before execution.

## Implementation map

| Surface | Current state | Intended change | Invariants |
| --- | --- | --- | --- |
| `dev/areas/README.md` | Absent. | Create the index of durable Development Areas. | Index links downward only and carries no work state. |
| `dev/areas/multiplayer/README.md` | Absent; Multiplayer material is scattered across concept, research and Lab homes. | Create the concise source-owned Area overview and prepared-material links. | No current question, plan status, verdict or game-contract claim. |
| `dev/lab/multiplayer/scenarios.md` → `dev/areas/multiplayer/scenarios.md` | Catalogue is a generic Lab record. | Move it intact apart from path/role-pointer corrections needed by the new home. | S01–S14 narratives, fixture meaning, questions and coverage matrix remain semantically identical. |
| `dev/lab/multiplayer/README.md` and affected live references | Catalogue pointer is local to Lab. | Point to the Area-owned catalogue and describe Lab as a consumer. | Experiment ownership and verdicts remain unchanged. |
| `dev/docs/README.md` | No Area home exists. | Add `dev/areas/` to development placement, home table and index convention. | One home per truth; game/docs and Work remain authoritative for their scopes. |
| `studio/src/home.rs` | Area files would be unmatched; catalogue is `lab-record`. | Add `area-index`, `development-area` and `area-record` classifications and tests. | First-match classification stays deterministic; every Markdown file has one home. |
| `studio/src/page/area.rs` | Absent. | Add generic Area index/detail projection and the structured scenario-catalogue overview. | Content comes only from classified records; unknown area/record fails closed. |
| `studio/src/page/mod.rs`, `studio/src/lib.rs` | Only `/development` handlers are exported and routed. | Export Area handlers, replace the route family with `/dev`, and add the three Area route shapes. | No `/development` handler, redirect or alias remains. |
| `studio/src/page/development.rs` | All page references and decision form actions use `/development`. | Change canonical path constants, references and form/facet targets to `/dev`; add Areas to the Development index. | Existing Development projections retain their source counts and behavior. |
| `studio/src/page/tree.rs`, `studio/src/page/mod.rs`, `studio/src/page/overview.rs`, `studio/src/brief.rs` | Navigation and generated orientation link to `/development`. | Move all current generated links to `/dev` and place `Areas` above Direction/Lab with Multiplayer below it. | Primary label remains Development; Work and Lab stay separate groups. |
| `studio/web/studio.css` | Existing list, card, grid and disclosure primitives do not have a scenario composition. | Add the minimum responsive scenario-overview rules needed for scanability. | Existing AA palette, typography and mobile shell remain unchanged; no decorative dashboard chrome. |
| `studio/tests/studio/`, `studio/src/home.rs` tests | Tests assert `/development` and have no Area/source contract. | Update route expectations and prove discovery, index completeness, all 14 scenarios, stable anchors, source authority and old-route absence. | Tests compare the page to repository projection rather than copied counts or titles where practical. |
| `game/docs/local-play.md` | Current contract names `/development/work` and omits Areas. | Describe `/dev`, Areas and the Multiplayer overview as the supported read-only Studio surface. | No runtime game or Agent capability is implied. |
| `dev/docs/concept/aicadia-studio.md`, `dev/docs/concept/log/2026-08.md`, `dev/docs/evidence/local-play.md` | Direction is accepted but explicitly not built; no delivery record exists. | On execution, keep plan/decision state current and record the bounded delivered Studio evidence once. | Historical route citations remain historical; evidence claims only exercised pages and tests. |

## Execution contract

Root owns outcome, scope, plan state, integration and the final evidence claim. No
delegation is planned. Tasks are sequential because source classification, route
projection and final documentation touch one shared Studio contract.

## Task graph

Allowed states are `pending`, `in_progress`, `completed` and `blocked`.

| ID | State | Depends | Parallel-safe | Objective | Owned surfaces | Evidence |
| --- | --- | --- | --- | --- | --- | --- |
| T1 | completed | — | no | Establish one governed Area home and move the multiplayer catalogue without semantic drift. | `dev/areas/**`, `dev/lab/multiplayer/README.md`, affected live pointers, `dev/docs/README.md`, `studio/src/home.rs` | Home/classification tests, clean documentation lint and focused diff showing S01–S14 retained. |
| T2 | completed | T1 | no | Replace the Development route family and render source-backed Area and scenario overviews. | `studio/src/page/area.rs`, `studio/src/page/{mod,development,tree,overview}.rs`, `studio/src/{lib,brief}.rs`, `studio/web/studio.css` | Focused Studio HTTP tests and rendered desktop/narrow review. |
| T3 | completed | T2 | no | Align all current tests, contracts, decision/evidence records and prove the complete outcome. | `studio/tests/studio/**`, `game/docs/local-play.md`, Studio concept/log, local-play evidence, this plan | Full Studio suite, old-route absence, cargo brief, docs lint, diff integrity and final route/source audit. |

## Task details

### T1 — Establish Development Area authority

**Objective:** `dev/areas/` is a governed, conventionally discoverable home and the
unchanged fourteen-scenario catalogue is owned by Multiplayer rather than Lab.

**Actions:**

1. Add the Area index, Multiplayer overview and placement/index rules.
2. Move the scenario catalogue and update only current pointers and role text that
   depend on its location.
3. Extend the Studio home table and its focused classification/index tests.

**Invariants:**

- No scenario outcome, question or matrix cell changes meaning.
- Lab retains experiments and their verdict metadata; Areas gains no verdict field.
- Work and game contracts receive no copied state.

**Evidence:**

- `cargo test -p aicadia-studio home` — new Area paths classify exactly once and
  their index links are complete.
- `cargo test -p aicadia-studio lint` — the governed move leaves no unmatched or
  stale live reference.
- Focused pre/post scenario heading and ID comparison — S01 through S14 and the
  coverage matrix remain present once.

**Stop conditions:**

- Stop if the Area source needs planning status, experiment verdicts or accepted
  game semantics to render a useful overview.
- Stop if the move would require rewriting a frozen historical record rather than a
  current pointer.

### T2 — Build `/dev` and the Multiplayer overview

**Objective:** every Development destination uses `/dev`, Areas is visible above Lab,
and the Multiplayer scenario page makes all fourteen cases scanable from their one
source.

**Actions:**

1. Replace canonical Development routing and every current generated link, form
   action, breadcrumb and reference.
2. Add generic Area index/detail handlers and a structured scenario renderer that
   joins the source index to S01–S14 headings.
3. Add the minimum responsive presentation rules and review the real page at desktop
   and narrow width.

**Invariants:**

- `/development` is not kept as an alias or redirect.
- Scenario cards contain only parsed source facts and stable anchors.
- The Multiplayer page has no live plan board, current open question or inferred
  completion status.
- Existing Game, Live, Overview, Jump and generic document routes keep working.

**Evidence:**

- Focused Studio development/area HTTP tests — all `/dev` routes return their
  expected complete pages, all fourteen scenario IDs render once, and old routes
  return not found.
- Browser review at approximately 1440 px and 390 px — hierarchy, scanning,
  disclosures and mobile navigation remain legible without horizontal overflow.

**Stop conditions:**

- Stop if useful rendering requires hard-coded Multiplayer prose or a client-side
  route/state machine.
- Stop if the source index and scenario sections disagree on identity or coverage.

### T3 — Align current truth and prove the result

**Objective:** tests, current contracts, generated orientation and delivery evidence
all name the new canonical surface, with historical citations preserved.

**Actions:**

1. Update Studio contract tests and add no-alias and source-parity coverage.
2. Update current local-play and Studio authorities, append the accepted/completed
   trail at the earned moment, and add bounded delivery evidence.
3. Run the validation ladder and review the exact diff for accidental route or
   scenario drift.

**Invariants:**

- Historical evidence and frozen plan citations keep the route they actually
  exercised.
- Final evidence claims Studio routing and presentation only, not multiplayer
  correctness or game capability.
- No unrelated user work is changed.

**Evidence:**

- `cargo test -p aicadia-studio` — complete Studio unit and integration suite.
- `cargo brief` — generated orientation links to `/dev/open` and discovers the Area
  records without a database.
- `rg -n '/development' studio game/docs dev/docs --glob '!dev/docs/concept/log/*' --glob '!dev/docs/evidence/*' --glob '!dev/plans/**'` — no stale current route remains; any deliberate historical match is reviewed separately.
- `git diff --check` and focused diff review — formatting and scope integrity.

**Stop conditions:**

- Stop if full evidence reveals a change to game behavior, public Agent text or the
  accepted Area/Work boundary.

## Validation ladder

1. **Focused:** Area classification/index tests, scenario ID/source-parity tests and
   `/dev` HTTP route tests.
2. **Contract:** complete `aicadia-studio` suite, documentation lint, generated
   brief, current-route sweep and explicit `/development` not-found assertions.
3. **Outcome:** render `/dev/areas/multiplayer/scenarios` from the moved catalogue and
   visually confirm all fourteen scan-first entries at desktop and narrow width,
   while `/dev/work` alone shows live plans/current edge.
4. **Integrity:** `git diff --check`, focused diff review, scenario semantic-diff
   review and confirmation that unrelated user changes and all governing authorities
   remain intact.

## Change control

Refine parser details, CSS composition, file placement inside `studio/src/page/` and
stronger evidence in place while the accepted routes, authority split and evidence
claim remain unchanged. Stop implementation, keep or return `status: draft`, revise
and request explicit re-acceptance when new evidence changes the Area/Work/Lab
boundary, route family, source ownership, scenario meaning, public behavior or final
claim.

## Completion conditions

- every required task is `completed` and the validation ladder passes;
- `/dev`, `/dev/work`, `/dev/areas`, `/dev/areas/multiplayer` and
  `/dev/areas/multiplayer/scenarios` are canonical and `/development` is absent;
- Studio discovers Area records by convention and renders all fourteen scenarios
  from their single moved source;
- Multiplayer and scenario pages contain overview material but no duplicated Work
  state, Lab verdict or game-contract claim;
- current contract, concept, generated references and delivery evidence agree while
  frozen history remains intact;
- no known-stale authority, material open question or accidental unrelated change
  remains;
- `status: complete` and `completed_at` are recorded only after these conditions.
