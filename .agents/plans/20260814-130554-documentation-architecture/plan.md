---
status: active
created_at: "2026-08-14T13:05:54+02:00"
updated_at: "2026-08-14T16:20:00+02:00"
accepted_at: "2026-08-14T15:00:07+02:00"
completed_at: null
---

# One home per truth: documentation, Agent-text and code architecture

## Outcome

Every kind of repository truth receives exactly one home with an explicit role,
owner and boundary, recorded in one binding constitution at `docs/README.md`
whose top-level structure is the side split: the **runtime side** (the running
product) versus the **development side** (how the product is discovered and
built), with `docs/evidence/` as the bridge. ("Side" is deliberate vocabulary:
`World` already has one precise game meaning and is not reused for
documentation roles.) The monolithic authorities are decomposed into small,
role-pure, iterable units:

- `docs/game/` becomes an index plus per-concern and per-capability documents;
- delivery and evidence narratives collapse from every duplicated live site
  into one `docs/evidence/` home, with only static pointers elsewhere — the
  sweep, not a fixed site count, defines completeness;
- the concept log regains its decision-register role with correct dates and
  bounded per-month files;
- superseded concept and research documents are archived with explicit
  supersession banners, while a still-live-ideas index keeps dormant product
  ideas findable;
- `docs/concept/10-discovery-and-world-context.md` (714 lines, five distinct
  design themes) splits into themed live concept records;
- the thirteen published tool descriptions move from Rust string constants into
  per-tool Markdown files; the play contract stays one whole file, relocated to
  `src/agent_contract/instruction.md`;
- the four code monoliths — `src/world.rs` (5,427 lines), `src/wire.rs`
  (2,156), `tests/world.rs` (7,520), `tests/server.rs` (3,714) — are decomposed
  into concern-named modules by pure moves with no signature, behavior or
  dependency change.

The value is builder-facing but concrete: today one delivery-status change costs
seven edits, stale July-era documents actively mislead any builder or Agent that
opens them, iterating on one tool description means editing a Rust source file,
and every World change navigates a 5,427-line file. After this build, each fact
has one authority, each Agent-facing text is one small Markdown file, each code
concern is one bounded module, and mechanical checks prove nothing
player-facing changed.

The exact evidence claim: after completion the runtime MCP surface is unchanged —
`tests/agent-tool-catalog.json` is byte-identical, the published instructions
are byte-identical (relocated, not edited), the integration-test inventory maps
one-to-one onto the pre-move inventory (same two target names, module prefixes
only, equal counts, all green), all Rust and shell test suites pass — while
every live internal documentation link and anchor resolves, each formerly
duplicated delivery narrative exists in exactly one evidence file, and no
current-authority document or source module exceeds its bounded size except
designated long-form records and frozen runners.

## Non-goals

- No change to game behavior, schema, migrations, World semantics, HTTP/MCP wire
  contracts, the thirteen-capability catalog or any error meaning.
- No rewording of the published play contract or tool descriptions, and no
  sectioning of the play contract file. The readability rewrite (shorter
  sentences, Markdown structure, section files, revised pinned phrases,
  regenerated fixture), the optional adapter discover-fetch and the known
  deferred live-controller `startswith` correction are separate future plans;
  none is a gate of this one.
- No deletion of history. Superseded documents are archived verbatim with a
  banner; log entries are regrouped by real date, never rewritten; frozen
  historical records keep their original links as citations.
- No new backlog game item, no authentication, hosting, tooling framework,
  build script, codegen or new dependency. Text inclusion uses plain
  `include_str!`.
- Code decomposition is pure relocation: no renamed public item, no public
  signature, behavior, SQL or dependency change, no new abstractions. The
  digest-frozen runner scripts (`tools/agent-playtest`, `tools/trait-playtest`
  — except the two named single-line corrections in T7/T10), the single-page
  ledger (`web/index.html`) and `src/server.rs` (1,108 lines, within bounds)
  are deliberately not split.
- No change to completed or superseded `.agents/plans/` history.

## Evidence and authorities

| Evidence or authority | Current fact | Consequence for this build |
| --- | --- | --- |
| `docs/game/README.md` (~1,000 lines) | Mixes contract, schema, validation, required evidence and ~40% delivery-status narrative | Decompose into index + role-pure files; delivery narrative moves to `docs/evidence/` |
| `docs/game/agent-interface.md` (~800 lines) | Duplicates capability semantics against README; carries its own delivery-status block | Absorbed into `protocol.md`, `agent.md` and per-capability files |
| Trait delivery narrative | Repeated near-identically across at least seven live sites (game README, agent-interface, trait-playtest, agent-playtest, capability-map, the backlog README horizon row, the backlog item, concept 11, log) — the working tree keeps evolving, so T2's sweep, not a fixed count, defines the site list — now spanning three completed-as-failed paid candidates: `candidate-MmwRmcBv`, `candidate-ydttdFfc` and `candidate-63hjH4HW` | Written once in `docs/evidence/trait.md`; every other live site becomes one static pointer |
| `.agents/plans/20260814-111749-trait-live-validation/plan.md` | `status: complete` (accepted, spend-authorized and closed 2026-08-14); T5 review returned GO for the completed-as-failed claim; the sole known P1 (live `startswith("Pip ")` validator drift) is explicitly deferred; the log designates this architecture plan as the next chosen work with no active edge | This plan is rebased on that end state: no pending candidate exists, no other plan will freeze a digest, and the controller correction is a future plan, not a dependency here |
| `docs/concept/00–09` | Pre-2026-08-07 generation; scene/claim model, persona dealing, MCP verb sketch all contradict the current system with no marker | Archive verbatim under `docs/concept/archive/` with banners; still-live ideas stay indexed |
| `docs/concept/log/log.md` (~2,180 lines) | Most entries sit under one stale `# 2026-08-10` heading covering work through 2026-08-14; the trait-live-validation opening and closure ARE logged in the working tree | Split into per-month files with restored real dates; future delivery detail lives in evidence |
| `docs/research/README.md` | Index has no status column; two files are header-superseded; `ai-agent-graphs.md` describes a six-operation MVP that reads as current scope | Status/era columns; archive the three misleading files |
| `src/agent_contract.rs:5-29` | Thirteen descriptions as Rust consts without trailing newlines (fixture check: zero descriptions end with `\n`) | Per-tool `.md` files each end with `\n`; `apply()` removes exactly one final newline via `strip_suffix('\n')` with an assert (never `trim_end`, which would silently eat meaningful trailing whitespace) |
| `src/agent-play-contract.txt` | One 52-line standalone text, already editable without touching Rust; published via `get_info()` (`src/server.rs:1072`) and injected by the adapter | Relocated whole to `src/agent_contract/instruction.md`; not split, not reworded |
| `tests/server.rs:27,619,670` | `include_str!("../src/agent-play-contract.txt")` plus pinned discover/instruction assertions | T7 updates the include path in the same change |
| `tests/aicadia-local.sh:212` | Compares the recorded `developer_instructions=` argument against `jq -Rs . src/agent-play-contract.txt` | T7 updates the path in the same change |
| `tools/aicadia-agent:8,51-52,111-129` | Reads the contract from a hardcoded path behind an early exists/not-symlink guard and injects it as `developer_instructions` | T7 updates `CONTRACT=` to the new path; the guard and injection stay byte-identical |
| `tools/trait-playtest:1348` digest material | Candidate material is `find migration src -name '*.sql' -o -name '*.rs'` — Rust and SQL only; the contract text is not digest material today | T7 extends the predicate to `*.md`/`*.txt` under `src/` so published Agent text stays freeze material for any FUTURE live plan; with the validation plan closed and no candidate pending, digest drift from this plan blocks nothing |
| `tests/trait-playtest.sh:167` | Digest-drift injection appends to `src/world.rs` specifically | T10 repoints it at a real post-split World source or the check goes silently vacuous |
| Cargo target discovery | `tests/world.rs` and `tests/server.rs` are auto-discovered targets named `world` and `server`; Cargo also auto-discovers `tests/<dir>/main.rs` as the same-named target — no `[[test]]` sections exist or are needed (`Cargo.toml` has none) | T10 keeps the two target names; its first action proves discovery before anything moves |
| `tests/world.rs` / `tests/server.rs` | 17 + 2 `include_str!` calls whose relative depth changes on a directory-crate move; listed test names gain module prefixes after the split, so a byte-identical `--list` is impossible by construction | T10 rewrites the include paths and proves a one-to-one name mapping (module prefix only) with equal counts instead of list identity |
| `docs/game/**` → `.agents/plans/**` links | Six A→development authority links exist today (`README.md:19,69`, `agent-interface.md:8,53`, `agent-playtest.md:18`, `trait-playtest.md:150`); the two plan directories involved are still untracked — already broken on a fresh clone | Present defect; T2/T3 remove them, `docs/evidence/trait.md` becomes the sole plan-pointer holder |
| Research links | Six links to `agent-interface.md`, several anchor-level, across five research files | Recorded in the move map; repointed in T9's single cross-tree link pass (no task edits another task's tree) |
| `.agents/skills/build-aicadia/SKILL.md` | 75 lines restating AGENTS.md rules; references `docs/concept/log/log.md` and doc paths this plan moves | Owned by T1 (side map) and T9 (path alignment; broader duplication flagged, not resolved) |
| Historical records' links | The log and past plans link to files this plan moves; their entries are frozen history; this plan and its move-map fragments contain old tokens BY DESIGN | Constitution splits sweep scope: frozen history is exempt from everything; draft/active plans stay inside link-and-anchor checks but are, with the move-map fragments, exempt from token/duplication scans as explicit planning citations — otherwise T9 would be unsatisfiable by construction |
| `AGENTS.md` "The MVP Is The Filter" | Enumerates the thirteen capability names and MVP shape, duplicating `docs/game/` | Slim to the rule plus a pointer; contract state lives only in `docs/game/` |
| Working tree | 15 modified files plus two untracked plan directories (trait-live-validation closeout and this plan); line figures were measured earlier and shift as the tree moves | Precondition: the User settles/commits ALL in-flight work before T1 begins — T1 itself appends to the already-modified log |
| User decisions this session | Markdown sources; no monolithic files; per-capability files; `docs/evidence/` as own home; `AGENTS.md` stays one strictly formatted file; contract stays one file (former T8 discover-fetch dropped); code decomposition stays in this plan | Confirmed decisions below |
| Independent reviews 2026-08-14 | First review: six P1 / eight P2 / six P3, all folded in. Second review (User-commissioned): five P1 + six corrections; adopted except the runner-doc relocation (rejected: it would create a fourth home for two files; the boundary wording is fixed instead) and the claim that directory test crates need explicit Cargo targets (incorrect; auto-discovery covers `tests/*/main.rs`, proven in T10 action 1) | This plan is the rebased result |

Governing authorities: `AGENTS.md` (build rules, amended by T1), `docs/game/`
(content redistributed, semantics unchanged), `docs/concept/log/log.md` (records
this choice; its closing entry already designates this plan as next). No
backlog game edge is claimed; see Strategic alignment.

## Alignment

### Strategic

This is documentation, text and code architecture work, not a player
capability. It qualifies under Game Progress First because it retires a
concrete standing risk on every future edge: the same delivery fact is
maintained in seven places, stale authorities (concept 00–09,
`ai-agent-graphs.md`) mislead builders and build Agents, iterating the
Agent-facing texts — the product's actual player interface — requires editing
Rust, and every World change navigates multi-thousand-line files. The Trait
edge is closed with no active edge and the log names this plan as the chosen
next work; the following edge (controller correction and live proof, or the
investigation roll) inherits a cheaper, truthful, navigable system.

### Tactical

The smallest complete slice is the full home-by-home relocation of
documentation, Agent-text sources and code modules with zero content invention
and zero behavior change: one constitution, one evidence home, one contract
decomposition, one concept/log/research cleanup, one code-side text split, one
code-module decomposition by pure moves, one cross-tree link pass. Excluded:
any prose rewrite, any behavior change, any new mechanism beyond
`include_str!`.

### Technical

Seams: documentation tree (`docs/`), root rule files (`AGENTS.md`,
`CLAUDE.md`), `src/agent_contract.rs` plus new `src/agent_contract/` text
sources, `src/world.rs` → `src/world/`, `src/wire.rs` → `src/wire/`,
`tests/world.rs` and `tests/server.rs` → directory test crates,
`tools/aicadia-agent` (path only), two single-line runner-script corrections,
and one cross-tree link pass driven by the per-task move-map fragments. Not
applicable:
`World` semantics, PostgreSQL, migrations, transactions, HTTP/MCP handlers,
schemas, catalog content.

## Decisions, assumptions and open questions

### Confirmed decisions

- Markdown (`.md`) for all relocated Agent-text sources — User decision
  2026-08-14.
- No monolithic authority files: current-truth documents stay bounded
  (guideline ≤ ~400 lines) and source modules ≤ ~1,500 lines; designated
  long-form records (active concept records, research reports, archived
  history, plans, per-month log files), digest-frozen runner scripts and the
  single-page ledger are exempt — User decision 2026-08-14.
- One capability file per player capability (13 files, fixed template).
- `docs/evidence/` is its own top-level documentation home — product history is
  not planning state (`.agents/`) and not current contract (`docs/game/`). Its
  boundary is precise: it contains delivery/evidence history AND the operation
  contracts of the machinery that produces evidence (`runner/`); it never
  contains game-contract rules.
- Outside `docs/evidence/`, delivery status appears only as a STATIC pointer
  ("delivery history and current status: see `docs/evidence/<slice>.md`") that
  never restates the current status, so a status change touches exactly one
  file. A backlog item's own planning state (`Done`, `Queued`, …) is a
  different fact and remains in the backlog.
- `AGENTS.md` remains one file: at 262 lines it is not a monolith, it is the
  always-loaded build context, and splitting it would break automatic loading.
  Each heuristic keeps one strictly formatted section; volatile contract state
  moves out.
- The concept log becomes per-month files under `docs/concept/log/`; entries
  are regrouped under their real dates without rewording.
- Runtime Agent-text delivery is unchanged: `server/discover` instructions plus
  `tools/list` descriptions, `ttlMs: 0`, protocol `2026-07-28` only.
- The constitution's split is named **runtime side / development side** —
  `World` is reserved domain vocabulary and is not reused for documentation
  roles (second review, adopted).
- Sweep scope is split by check kind, because this plan and its move-map
  fragments legitimately contain old tokens as planning citations and would
  otherwise make T9 unsatisfiable by construction (fourth review, adopted):
  LINK-AND-ANCHOR checks include draft/active plans (their actual links must
  resolve) and exclude only frozen history (log entries, completed/superseded
  plans, archives); DUPLICATION, STATUS and OLD-TOKEN scans additionally
  exclude all of `.agents/plans/**` and the move-map fragments as explicit
  planning citations; the move-map fragments are validated separately — every
  old source has exactly one destination and every destination exists;
  outside plans, the log, archives and the fragments, every old token must be
  gone.
- Each task updates references only inside its own owned tree and records
  every rename and heading move in its OWN move-map fragment
  (`.agents/plans/20260814-130554-documentation-architecture/move-map/<task>.md`,
  working artifacts of this plan) — no two tasks share a write surface, so
  parallel-safe stays true. T9 concatenates the fragments and executes the
  single cross-tree link pass. This removes all cross-task ownership overlap
  (second and third review, adopted).
- Resolved former OQ2, rebased 2026-08-14: the Trait live-validation plan is
  COMPLETE and closed (third candidate `candidate-63hjH4HW`,
  completed-as-failed on the deferred `startswith` validator drift; no retry
  or authorization outstanding). No plan currently exists that will freeze a
  digest, so digest drift caused by this plan blocks nothing; T7 still extends
  the candidate-material predicate to `*.md`/`*.txt` under `src/` so any
  future live plan freezes the published Agent text too.
- Resolved OQ3 — User decision 2026-08-14, following the first review: the
  play contract stays ONE file, relocated to
  `src/agent_contract/instruction.md`; only the thirteen tool descriptions
  split into `tool/*.md`. The adapter keeps its simple guarded file read at the
  new path. The former T8 (discover-fetch) is removed; its id is retired.
- Resolved OQ4 — User decision 2026-08-14: code decomposition (T10) stays in
  this plan, with both reviews' findings folded in.

### Reversible assumptions

- The capability-file template (Purpose / Input / Validation / Result / Errors /
  Workshop / Evidence) may gain or drop a section during T3 while every
  existing fact keeps exactly one home.
- Trailing-newline handling for `include_str!` tool sources is an
  implementation detail confined to `apply()` (`strip_suffix('\n')` + assert)
  and must never change published bytes; checked by the unchanged fixture and
  the `jq` newline probe.
- Module boundaries inside `src/world/` (e.g. whether `read.rs`/`mutation.rs`
  or finer) may shift during T10 while the pure-move rule holds.

### Open questions

- None. The plan is final and awaits explicit User acceptance.

## Implementation map

| Surface | Current state | Intended change | Invariants |
| --- | --- | --- | --- |
| `docs/README.md` | absent | New constitution structured as the side split — runtime side (src, migration, operations tooling, `docs/game/`) and development side (build rules, concept, research, planning, playtest machinery) with `docs/evidence/` as the bridge — then the home table, the one-way reference rule, the frozen-history exemption (completed/superseded plans only), role-header convention and bounded-size rule; states explicitly that sides are roles, not directories | Constitution defines placement; it never restates content; it never reuses `World` for documentation roles |
| `AGENTS.md` | Duplicates MVP surface; no constitution anchor | Slim "The MVP Is The Filter" to rule + pointer; add one compact "One Home Per Truth" rule; extend "Every Choice Leaves A Trail" authority list and Reference Docs with `docs/evidence/` and the constitution | All other heuristics byte-unchanged; file stays compact |
| `CLAUDE.md` | Points to AGENTS.md and concept | Add one line pointing to `docs/README.md` | Stays a thin pointer file |
| `docs/evidence/` | absent (content scattered across 7 sites) | `README.md` index; `world-entry.md`, `action.md`, `interaction.md`, `property.md`, `trait.md` (all three paid Trait candidates, digests, audits, closure); `runner/agent-playtest.md`, `runner/trait-playtest.md` | Each narrative appears once; other sites carry one static pointer; no game-contract rules live here |
| `docs/game/README.md` | ~1,000-line monolith | Short index: authority statement, home map, reading order | Every removed fact lands in exactly one new home |
| `docs/game/domain.md` | part of README | Current domain model: User, Entity, Character, Place, Property, Trait, Activity semantics and role table | Semantics verbatim-preserved |
| `docs/game/protocol.md` | split across README/agent-interface | Request context, MCP revision/statelessness, wire conventions and shared shapes, pagination/cursors, canonical error table, delivery identity and Place freshness, HTTP statuses, origin rules | One cross-cutting home; capability files link instead of repeating |
| `docs/game/agent.md` | part of agent-interface | Host conformance, instruction hierarchy, player-mode communication, three workshop flows, and the text-layering rule (what lives in global instructions vs per-tool descriptions, and which redundancy is deliberate) | Layering rule governs the follow-up rewrite |
| `docs/game/capability/*.md` | absent | 13 files, fixed template: Purpose, Input, Validation, Result and Activity footprint, Errors, Workshop pointer, Evidence obligations + static evidence pointer | Names match the catalog exactly; no capability described elsewhere |
| `docs/game/storage.md` | part of README | PostgreSQL relations, indexes, locking, migration boundaries | Schema description matches migrations |
| `docs/game/deferred.md` | wall at README end | The explicit deferral list as an extensible file | Deferred means absent, per AGENTS.md |
| `docs/game/local-play.md` | current | Update links and the adapter paragraph (new contract path) | Role unchanged |
| `docs/game/agent-interface.md`, `docs/game/agent-playtest.md`, `docs/game/trait-playtest.md` | monoliths / evidence mix | Deleted after full absorption into `protocol.md`/`agent.md`/`capability/` and `docs/evidence/` | `git log` retains history; every removal lands in the move map |
| `docs/concept/README.md` | 36-line table | Guide: two generations, per-doc status, archive pointer, and a still-live-ideas index distinguishing superseded model choices from dormant-but-alive product ideas (steward, ripples/catch-up, naming economy, sealed envelopes, tension sources, anti-patterns) | Concept never governs implementation |
| `docs/concept/10-discovery-and-world-context.md` | 714-line record spanning five themes | Split into themed live records: `discovery.md`, `knowledge.md`, `interaction.md`, `spatial.md`, `time.md`, `tabletop.md`; delivered Interaction content becomes rationale + pointer to `docs/game/` | Every open decision and frontier item keeps exactly one home; the prototype HTML stays beside `discovery.md`; all renames land in the move map |
| `docs/concept/11-entity-traits-and-change.md` | duplicates delivery saga | Trimmed to `entity-state.md`: unique concept rationale kept; delivery detail becomes one static pointer | Concept rationale preserved; rename in the move map |
| `docs/concept/log/` | one ~2,180-line file | `README.md` (from the current preamble) + `2026-07.md`, `2026-08.md`; real-date headings restored inside the August block | Entries regrouped, never reworded; append-only discipline stated in the index |
| `docs/research/README.md` | question-only index | Add Status (load-bearing / pending / historical / superseded) and Era columns; archived files keep an index row pointing into `archive/` — the README is the one index, no separate archive index exists | Research informs, never decides |
| `docs/research/archive/` | absent | `open-spatial-world-system.md`, `hierarchical-spatial-model.md`, `ai-agent-graphs.md` moved with banners; July-era vocabulary banner added to remaining July files | Files verbatim below the banner; moves land in the move map |
| `src/agent_contract.rs` | consts + include of one txt | Per-tool `include_str!` table; `apply()` removes exactly one final newline per description via `strip_suffix('\n')` + assert; instruction include repointed to `instruction.md`; all existing tests kept | Published bytes identical; startup assert kept; no description ends with `\n` |
| `src/agent_contract/instruction.md` | absent (`src/agent-play-contract.txt` today) | `git mv`, byte-identical; inbound paths updated in `agent_contract.rs`, `tests/server.rs:27`, `tests/aicadia-local.sh:212`, `tools/aicadia-agent` (`CONTRACT=`, guard kept at the same early position) | Bytes unchanged; every consumer repointed in the same task |
| `src/agent_contract/tool/*.md` | absent | Thirteen description files, byte-exact | `tests/agent-tool-catalog.json` unchanged |
| `src/agent_contract/README.md` | absent | States the bytes are published verbatim to every Agent and formatting is load-bearing; "fixing the Markdown" is a contract change requiring the rewrite plan | Guards against later markdown-fixing that would break pinned phrases |
| `tools/trait-playtest:1348` | candidate material `*.sql`/`*.rs` only | Predicate extended with `*.md`/`*.txt` under `src/` | Published Agent text becomes freeze material for future live plans |
| `src/world.rs` | 5,427-line monolith: `World` impl (16 operations), private Property/Trait persistence machinery, domain types, errors | Directory module `src/world/`: `model.rs` (ids + domain types), `error.rs`, `property.rs`, `entity_trait.rs` (`trait` is a Rust keyword), `activity.rs`, operation impl blocks grouped by concern (`read.rs`, `mutation.rs`); `mod.rs` keeps `World` and re-exports | Public seam `World` unchanged; `lib.rs` re-exports identical; pure moves |
| `src/wire.rs` | 2,156 lines of ~30 input/output types | Directory module `src/wire/` split by concern with unchanged re-exports and every serde/schemars/utoipa attribute identical | Serialized JSON shapes byte-identical; OpenAPI/MCP schemas unchanged |
| `tests/world.rs` | auto-discovered target `world`; 7,520 lines | Directory test crate `tests/world/main.rs` + per-concern modules — same auto-discovered target name; 17 `include_str!` paths rewritten | One-to-one test mapping (module prefix only), equal count, all green |
| `tests/server.rs` | auto-discovered target `server`; 3,714 lines | Directory test crate `tests/server/main.rs` + per-concern modules; 2 `include_str!` paths rewritten | One-to-one test mapping, equal count, all green |
| `tests/trait-playtest.sh:167` | drift injection appends to `src/world.rs` | Repointed at a real post-split World source (e.g. `src/world/mutation.rs`) | The drift check keeps proving World-source coverage |
| `.agents/backlog/capability-map.md` | stale delivery bullets | Trait bullet and delivery block become one static evidence pointer | Map stays a living index, not an authority |
| `.agents/backlog/items/entity-trait-development.md` | delivery narrative | Item keeps its own planning state (`Done`) plus one static evidence pointer | Item states outcome, not saga |
| `.agents/skills/build-aicadia/SKILL.md` | references paths this plan moves | Paths repointed via the move map in T9 (`:14-18,63,69`); broader AGENTS.md duplication flagged as follow-up candidate, not resolved here | Skill keeps working during and after the move |
| Move map (plan directory) | absent | Directory `move-map/` with one fragment per task (`t2.md` … `t10.md`): every old path/anchor → new path/heading, written only by its owning task | T9 concatenates the fragments; its cross-tree pass is driven exclusively by them and searches old path TOKENS as plain text, not only Markdown links; the fragments are validated separately (one destination per old source, every destination exists) and are, like all of `.agents/plans/**`, exempt from the token scan |

## Execution contract

Root owns outcome, scope, plan state, integration and the final evidence claim. A
delegated Agent receives this plan path and one dependency-ready task id, re-reads
the live repository, changes only its owned surfaces, runs focused evidence and
returns raw results. Delegation is optional. Run tasks in parallel only when the
table marks them safe, write surfaces do not overlap and results verify
independently.

Reference discipline during execution: a task renames and edits only inside its
owned tree, appends every rename/heading move to its own move-map fragment
(`move-map/<task>.md` — never another task's fragment), and leaves cross-tree
links untouched; T9 concatenates the fragments and performs the one cross-tree
link pass. Links may therefore dangle between tasks; T9 gates completion.

Precondition gate before T1: the User settles/commits the complete in-flight
working tree (Trait closeout edits, the closed trait-live-validation plan
directory and this plan directory) — T1 itself appends to the already-modified
concept log, so relocation diffs must not entangle with delivery diffs.

## Task graph

| ID | State | Depends | Parallel-safe | Objective | Owned surfaces | Evidence |
| --- | --- | --- | --- | --- | --- | --- |
| T1 | pending | — | no | Constitution + root rule alignment | `docs/README.md`, `AGENTS.md`, `CLAUDE.md`, log entry | Constitution exists; AGENTS.md diff limited to the three named sections |
| T2 | pending | T1 | no | Evidence home extraction | `docs/evidence/**`, static-pointer replacements in every live delivery site its sweep finds (known: game README, agent-interface, both playtest docs, capability-map, backlog README horizon row, backlog item, concept 11), `move-map/t2.md` | Each narrative greps to exactly one home; pointers restate no status |
| T3 | pending | T2 | no | Game contract decomposition + anchor map | `docs/game/**`, `move-map/t3.md` | Fact-coverage review; in-tree links resolve; no game doc > 400 lines |
| T4 | pending | T1, T2 | no | Concept archive, doc 10 themed split, doc 11 trim | `docs/concept/**` except `log/`, `move-map/t4.md` | Banners present; archived content verbatim; every doc-10 fact has one themed home |
| T5 | pending | T1 | yes | Log restore + split | `docs/concept/log/**`, `move-map/t5.md` | Heading/entry mapping table complete; entry text unchanged |
| T6 | pending | T1 | yes | Research status index + archive | `docs/research/**` (index/banners/moves only, no link edits), `move-map/t6.md` | Index rows cover every live and archived file; 3 files archived |
| T7 | pending | T1 | yes | Agent-text relocation: tool split + contract move | `src/agent_contract.rs`, `src/agent_contract/**`, `src/agent-play-contract.txt`, `tests/server.rs` (include path), `tests/aicadia-local.sh` (path), `tools/aicadia-agent` (`CONTRACT=`), `tools/trait-playtest` (predicate line), `move-map/t7.md` | `cargo test` green incl. discover assertions; fixture diff empty; no description ends with `\n`; `tests/aicadia-local.sh` passes |
| T10 | pending | T7 | no | Code decomposition by pure moves | `src/world.rs` → `src/world/**`, `src/wire.rs` → `src/wire/**`, `tests/world.rs` → `tests/world/**`, `tests/server.rs` → `tests/server/**`, `src/lib.rs`, `tests/trait-playtest.sh` (drift line), `move-map/t10.md` | One-to-one test mapping with equal counts, all green; fixture diff empty; drift line targets a real file |
| T9 | pending | T3,T4,T5,T6,T7,T10 | no | Cross-tree link pass + integrity sweep + alignment | whole tree, `.agents/backlog/*`, `.agents/skills/build-aicadia/SKILL.md` | Validation ladder passes |

The former T8 (adapter discover-fetch) was removed by resolved OQ3; the id is
retired, not reused.

## Task details

### T1 — Constitution and root rule alignment

**Objective:** `docs/README.md` exists as the binding home constitution; root rule
files anchor it; the architecture decision is logged.

**Actions:**

1. Write `docs/README.md` with the side split as its top-level structure:
   **runtime side — the running product** (`src/` including
   `src/agent_contract/`, `migration/`, app test crates under `tests/`,
   `docs/game/`, `tools/aicadia-local`, `tools/aicadia-agent`, `web/`) and
   **development side — development of the product** (`AGENTS.md`,
   `CLAUDE.md`, `CONTEXT.md`, `docs/concept/` + log + archive,
   `docs/research/` + archive, `.agents/backlog/`, `.agents/plans/`,
   `.agents/skills/`, `tools/*-playtest`, `tests/*-playtest.sh`), with
   `docs/evidence/` as the bridge (development history about the product,
   including the evidence-machinery operation contracts; never game-contract
   rules). The constitution states explicitly that sides are roles, not
   directories — `tools/` and `tests/` contain files of both sides, and the
   home table, not the directory tree, answers placement — and that `World`
   remains reserved domain vocabulary. Then the home table (home, side, role,
   owner-authority, contains, never contains, update trigger); the one-way
   reference rule: the development side may cite the runtime side, the runtime
   side never depends on the development side for meaning, and the only
   permitted runtime→development link is a static pointer into
   `docs/evidence/`; the split sweep scope: frozen history (log entries,
   COMPLETED or SUPERSEDED plans, archived docs) keeps its original links as
   citations and is excluded from every sweep; draft/active plans stay inside
   link-and-anchor checks, while duplication/status/old-token scans exclude
   all of `.agents/plans/**` and move-map fragments as explicit planning
   citations;
   the static-pointer rule for delivery status; the role-header convention
   (2–3 lines at the top of every authority file: role, side, authority, what
   does not belong here and where it goes); the bounded-size rule and its
   named exemptions — current-authority docs ≤ ~400 lines, source modules ≤
   ~1,500 lines; exempt: designated long-form records (research reports,
   per-month log files, plans, archives), digest-frozen runner scripts and the
   single-page ledger.
2. Amend `AGENTS.md` exactly three ways: slim "The MVP Is The Filter" to the
   rule plus a pointer to `docs/game/` for the current surface; add compact
   rule "One Home Per Truth" anchoring `docs/README.md`; extend "Every Choice
   Leaves A Trail" and Reference Docs with `docs/evidence/` and the
   constitution.
3. Add the constitution pointer line to `CLAUDE.md`.
4. Record the accepted architecture choice in the concept log (one entry, with
   reason, both review outcomes and a link to this plan).

**Invariants:**

- Every other AGENTS.md heuristic remains byte-identical.
- The constitution places content; it never duplicates it.

**Evidence:**

- `git diff AGENTS.md` — touches only the three named sections.
- `docs/README.md` review — every existing home has a row; no home lacks a
  "never contains" boundary.

**Stop conditions:**

- Stop if slimming "The MVP Is The Filter" would remove a constraint not present
  in `docs/game/` — that content must first gain its home.

### T2 — Evidence home extraction

**Objective:** Every delivery/evidence narrative exists exactly once under
`docs/evidence/`; all live sites carry one static pointer.

**Actions:**

1. Create `docs/evidence/README.md` (role header; index: slice → current status →
   proof links; the rule that delivery detail lives only here and pointers
   elsewhere never restate status).
2. Create per-slice records by extracting — not rewriting — current text:
   `world-entry.md` (run-9TOG5yrJ), `action.md` (trail-marker slice; runs
   run-G8k1sTRm, run-nvULnvxQ, run-gE8iED5m), `interaction.md` (deterministic
   Pip/Mara evidence summary), `property.md`, `trait.md` (the complete saga:
   candidates `MmwRmcBv`, `ydttdFfc` and `63hjH4HW`, digests, audits, the
   closed live-validation plan pointer and the deferred `startswith` P1).
3. Move runner operation contracts to `docs/evidence/runner/agent-playtest.md`
   and `docs/evidence/runner/trait-playtest.md`, trimming their duplicated
   delivery-status preambles to pointers.
4. Find every live delivery site with the sweep itself — `grep -rn
   "candidate-\|digest\|P0–P3\|model_calls" docs .agents` filtered by the
   frozen-history exemption — and replace each found block or line with one
   STATIC pointer (no restated status). Known sites: `docs/game/README.md`,
   `docs/game/agent-interface.md`, `.agents/backlog/capability-map.md`, the
   `.agents/backlog/README.md` horizon row for entity-trait-development
   (the row keeps its own planning state `Done` plus the pointer),
   `.agents/backlog/items/entity-trait-development.md` and
   `docs/concept/11-entity-traits-and-change.md` — but the sweep, not this
   list, defines completeness. The six `docs/game/**` → `.agents/plans/**`
   authority links disappear here or in T3; `docs/evidence/trait.md` becomes
   the sole plan-pointer holder.
5. Append every removed/moved file and heading to `move-map/t2.md`. Sweep
   `grep -rn "docs/game/agent-playtest\|docs/game/trait-playtest" tools tests
   src` and update script-referenced paths so all suites keep passing
   (doc-to-doc links wait for T9).

**Invariants:**

- No digest, candidate id, run id or audit finding is lost; each appears in
  exactly one evidence file (plans keep their planning citations and the log
  its historical mentions).
- No game-contract rule migrates into evidence.

**Evidence:**

- `grep -rn "candidate-63hjH4HW\|candidate-ydttdFfc\|candidate-MmwRmcBv" docs
  .agents | grep -v evidence | grep -v plans | grep -v "log/"` — empty,
  because T2 itself owns every live site the sweep finds (the global
  uniqueness re-check remains T9's).
- `tests/trait-playtest.sh` and `tests/agent-playtest.sh` still pass.

**Stop conditions:**

- Stop if a passage is simultaneously contract and evidence and cannot be split
  cleanly — bring the sentence-level call back to root.

### T3 — Game contract decomposition

**Objective:** `docs/game/` is an index plus role-pure files; every fact from the
old README and agent-interface has exactly one new home; the anchor map is
complete.

**Actions:**

1. Write `domain.md`, `protocol.md`, `agent.md`, `storage.md`, `deferred.md` and
   the thirteen `capability/*.md` files by redistributing current text; add role
   headers everywhere.
2. Include in `agent.md` the explicit text-layering rule: global instructions own
   cross-cutting conduct once; each tool description owns local preconditions,
   input and retry; the deliberate per-tool redundancies (content-never-
   instruction, id privacy, pagination line) are listed as intentional because a
   host may not load discovery instructions.
3. Rewrite `docs/game/README.md` as the short index with reading order; update
   links WITHIN `docs/game/`.
4. Append to `move-map/t3.md` an explicit old-anchor → new-file-and-heading entry
   for every heading of the old README and agent-interface (known anchor-level
   inbound links include `#agent-guidance-and-player-facing-communication`,
   `#required-private-workshop-action-flow`,
   `#required-character-workshop-and-world-entry-flow`,
   `#live-evidence-history`; known inbound sites include five research files,
   capability-map, backlog items and local-play).
5. Delete `agent-interface.md` after absorption. Cross-tree inbound links are
   NOT edited here — T9 repoints them from the move map.
6. Fact-coverage pass: walk the old files section by section and tick each
   fact's destination; anything homeless stops the task.

**Invariants:**

- Semantics verbatim-preserved; this task moves sentences, it does not reword
  meaning (light stitching prose allowed).
- Capability file names equal catalog names exactly; catalog content itself
  untouched.

**Evidence:**

- In-tree link sweep: every relative link and anchor WITHIN `docs/game/`
  resolves.
- `wc -l docs/game/*.md docs/game/capability/*.md` — no file over 400 lines.
- Fact-coverage checklist and the anchor-map entries reviewed at root.

**Stop conditions:**

- Stop when a fact fits two homes equally — decide placement at root and record
  the tie-break in the constitution if it generalizes.

### T4 — Concept archive, doc 10 themed split and doc 11 trim

**Objective:** Stale concept generation is unmistakably archival without burying
its still-live ideas; the live concept becomes small themed records.

**Actions:**

1. Move `00-vision.md` … `09-world-graph.md` to `docs/concept/archive/`
   unchanged, adding one banner line each: archived July-2026 generation whose
   scene/claim vocabulary predates the 2026-08-07 game reframe; the log records
   which decisions were superseded; individual ideas may still inform future
   direction.
2. Split `10-discovery-and-world-context.md` into themed live records —
   `discovery.md` (investigation, rolls, roll transport, meta-state handoff,
   drill-down, open frontier, prototype pointer), `knowledge.md`
   (Character-grounded knowledge, identity ambiguity, World/Character context,
   shared/personal scope), `interaction.md` (participation model rationale;
   delivered parts become pointer to `docs/game/`), `spatial.md`, `time.md`,
   `tabletop.md` — each with a role header and its own open decisions.
3. Trim doc 11 to `entity-state.md`: keep the Property/Trait domain-distinction
   and uniform-authority rationale; delivery detail stays the static pointer
   T2 placed.
4. Rewrite `docs/concept/README.md` as the guide: live themed records, per-doc
   status, the archive pointer, and the still-live-ideas index (steward,
   ripples/catch-up, naming economy, sealed envelopes, safe-tension sources,
   anti-patterns) so an "archived" stamp cannot bury an unbuilt but living
   product idea.
5. Update links WITHIN `docs/concept/` (including
   `discovery-roll-prototype.html` references) and append every rename and
   heading move to `move-map/t4.md`; cross-tree inbound links wait for T9.

**Invariants:**

- Archived content byte-unchanged below its banner; split content moved, not
  reworded (light stitching prose allowed).
- `docs/concept/` still cannot override `docs/game/` (restate in README).

**Evidence:**

- `diff` of each archived file against `git show HEAD:docs/concept/<name>` —
  banner-only delta.
- Fact-coverage pass over doc 10: every section ticked into exactly one themed
  record; the five open-frontier decisions and the retained-knowledge frontier
  list survive verbatim.

**Stop conditions:**

- Stop if any archived doc contains a still-live decision recorded nowhere else
  — log it first, then archive.

### T5 — Log restore and split

**Objective:** The log is a per-month decision register with truthful dates.

**Actions:**

1. Split `log.md` into `docs/concept/log/2026-07.md` and
   `docs/concept/log/2026-08.md`; the current preamble (title, tag vocabulary,
   separator) becomes `docs/concept/log/README.md` (role header, consolidated
   tag vocabulary including the ad-hoc tags that appeared, append-only rule,
   and the new rule that delivery bookkeeping lives in `docs/evidence/` with
   the log carrying one line + link). Historical entries keep their original
   links as citations per the constitution's exemption.
2. Inside the August file, restore real date headings for the block currently
   under `# 2026-08-10`, using the entries' own internal date references;
   regroup only, never reword. Correct stale stream-heading suffixes (e.g. a
   "review active" heading over completed entries) with a bracketed editorial
   note rather than silent edits.
3. Append the `log/log.md` → period-file mapping to `move-map/t5.md`;
   cross-tree inbound references wait for T9.

**Invariants:**

- Entry count and entry text preserved (headings and grouping only).
- Chronological append-only order within each file.

**Evidence:**

- The judged measure: a heading/entry mapping table — every `##` entry of the
  old file appears exactly once in a period file under a real-date `#` heading;
  entry count and entry text unchanged.

**Stop conditions:**

- Stop if an entry's real date is genuinely underivable — leave it in place
  under a marked "date uncertain" subheading rather than guessing.

### T6 — Research status index and archive

**Objective:** The research index states each file's standing; misleading files
are archived.

**Actions:**

1. Add Status (load-bearing / pending / historical / superseded) and Era
   (July scene-claim / August Activity-Property-Trait) columns to
   `docs/research/README.md`, using the completed research audit. Archived
   files KEEP an index row whose link points into `archive/` — the README is
   the one research index; no separate archive index is created.
2. Move `open-spatial-world-system.md`, `hierarchical-spatial-model.md` and
   `ai-agent-graphs.md` to `docs/research/archive/` with supersession banners
   naming their successors; append the moves to `move-map/t6.md`.
3. Add a one-line era banner to remaining July-era files warning that their
   scene/claim vocabulary predates the game reframe. Link lines inside research
   files are NOT edited here — T9 repoints them from the move-map fragments.

**Invariants:**

- Research content unchanged below banners; the index judges standing, not
  findings.

**Evidence:**

- The README index has one row for every live AND archived research file;
  each archived row's link resolves into `archive/`.

**Stop conditions:**

- Stop if archiving a file would orphan a citation from a live concept record
  or the capability map — keep it live and mark `historical` instead.

### T7 — Agent-text relocation: tool split and contract move

**Objective:** The thirteen tool descriptions live as per-tool Markdown files;
the play contract lives whole at `src/agent_contract/instruction.md`; published
bytes are unchanged.

**Actions:**

1. Create `src/agent_contract/tool/<tool_name>.md` (13), each containing
   today's exact description text; the per-tool table in `agent_contract.rs`
   becomes one `include_str!` per file. Because every file ends with `\n` and
   the current consts do not (fixture check: zero descriptions end with a
   newline), `apply()` removes exactly one final newline per description via
   `strip_suffix('\n')` with an assert that it was present — never `trim_end`,
   which would also strip meaningful trailing whitespace and mask content
   drift.
2. `git mv src/agent-play-contract.txt src/agent_contract/instruction.md`
   (byte-identical) and update the four inbound paths in the same change:
   the `include_str!` in `src/agent_contract.rs`, the include at
   `tests/server.rs:27`, the comparison path at `tests/aicadia-local.sh:212`,
   and `CONTRACT=` in `tools/aicadia-agent` — keeping the existing
   exists/not-symlink guard at the same early position.
3. Add `src/agent_contract/README.md`: these bytes are published verbatim to
   every Agent; formatting is load-bearing (pinned-phrase tests, fixture);
   "fixing the Markdown" is a contract change requiring the rewrite plan.
4. Extend the `tools/trait-playtest` candidate-material predicate
   (`find … -name '*.sql' -o -name '*.rs'`, line ~1348) with `*.md`/`*.txt`
   under `src/`, so published Agent text remains paid-run freeze material for
   any future live plan. Append the moves to `move-map/t7.md`.

**Invariants:**

- `tests/agent-tool-catalog.json` byte-identical; `cargo test` fully green,
  including `tests/server.rs`'s discover/instruction assertions.
- Published description bytes carry no trailing newline; instruction bytes
  unchanged.
- No build script, codegen, assembly join or new dependency.

**Evidence:**

- `git diff --stat tests/agent-tool-catalog.json` empty; `cargo test` green;
  `jq '[.[] | select(.description | endswith("\n"))] | length'
  tests/agent-tool-catalog.json` returns 0; `tests/aicadia-local.sh` passes.

**Stop conditions:**

- Stop if any consumer of the contract text cannot be repointed without a
  behavior change — bring the seam choice back to root.

### T10 — Code decomposition by pure moves

**Objective:** The four code monoliths become concern-named modules with
provably unchanged behavior.

**Actions:**

1. Record the baseline: `cargo test -- --list` output per target (current
   attribute counts: 75 `#[sqlx::test]` in `tests/world.rs`, 13 in
   `tests/server.rs`, 10 unit tests under `src/**`). Then prove Cargo's
   directory-target discovery in a DISPOSABLE temporary package outside this
   repository (scratch `cargo new`, scaffold `tests/foo/main.rs`, confirm the
   auto-discovered target with no `[[test]]` section) — never by letting
   `tests/world.rs` and `tests/world/main.rs` coexist as the same target in
   this repo. Each real conversion is then one atomic move: the old file
   becomes the directory crate in a single change. If discovery fails in the
   scratch proof, that is a stop condition, not a workaround site.
2. Convert `src/world.rs` into directory module `src/world/`: `model.rs` (id
   newtypes and domain types), `error.rs`, `property.rs` (private Property
   normalization/persistence), `entity_trait.rs` (private Trait machinery;
   `trait` is a Rust keyword), `activity.rs` (Activity drafting/hydration),
   and operation `impl World` blocks grouped by concern (e.g. `read.rs`,
   `mutation.rs`), with `mod.rs` holding the `World` struct and re-exports.
   Moves only.
3. Convert `src/wire.rs` into directory module `src/wire/` (`input.rs`,
   `output.rs`, `error.rs` or per-domain), keeping every serde/schemars/utoipa
   attribute and re-export identical.
4. Convert `tests/world.rs` into directory crate `tests/world/main.rs` plus
   per-concern modules, and `tests/server.rs` likewise; test function names
   unchanged (listed names gain module prefixes — that is the expected
   mapping, not drift). Rewrite the nineteen `include_str!` relative paths
   whose depth changes (17 in `tests/world.rs`, 2 in `tests/server.rs`,
   including the catalog fixture, the instruction file and migration SQL
   includes).
5. Keep `src/lib.rs` re-exports byte-compatible for every existing consumer.
6. Repoint the digest-drift injection `tests/trait-playtest.sh:167`
   (`printf … >> "$copy/src/world.rs"`) at a real post-split World source such
   as `$copy/src/world/mutation.rs` — otherwise the check appends to a
   nonexistent-then-new file and goes silently vacuous.
7. Append the four old-path → new-module mappings (`src/world.rs`,
   `src/wire.rs`, `tests/world.rs`, `tests/server.rs`) to `move-map/t10.md` —
   live flat references to these paths exist (e.g.
   `docs/research/character-entity-control-model.md:511`,
   `.agents/backlog/items/world-entry-history.md:145`) and T9 repoints them.

**Invariants:**

- No renamed public item, public signature, SQL string, dependency or behavior
  change; no new abstraction; `src/server.rs`, the runner scripts (beyond the
  one repointed line above) and `web/index.html` untouched.
- Crate-internal visibility widening (`pub(super)`/`pub(crate)`) on today's
  file-private helpers is allowed — it is what makes pure moves possible —
  while the `pub` surface stays unchanged.
- Source-module guideline after the split: no module over ~1,500 lines.

**Evidence:**

- `cargo test --all-targets` green; `cargo test -- --list` before/after
  compared as a one-to-one mapping: same two integration-target names, every
  post-move test name = module prefix + original name, equal counts per
  target, nothing added or lost.
- `git diff --stat tests/agent-tool-catalog.json` empty;
  `tests/trait-playtest.sh` passes AND the drift-injection line provably
  targets an existing post-split file.

**Stop conditions:**

- Stop if any move would require changing a `pub` signature, SQL string or
  observable behavior — that is refactoring, not relocation, and needs its own
  decision.
- Stop if Cargo does not auto-discover a directory test target in action 1.

### T9 — Cross-tree link pass, integrity sweep and alignment

**Objective:** All cross-tree references are repointed from the move map; the
whole tree satisfies the constitution; nothing stale, broken or duplicated
remains.

**Actions:**

1. Validate the `move-map/*.md` fragments themselves: every old source (path
   or anchor) has exactly one destination, and every destination exists on
   disk or as a heading. The fragments and this plan legitimately CONTAIN old
   tokens as planning citations; they are therefore exempt from the token and
   duplication scans below.
2. Concatenate the fragments and execute the single cross-tree link pass:
   repoint every live reference in `docs/`, `.agents/backlog/**`,
   `.agents/skills/build-aicadia/SKILL.md` (`:14-18,63,69`), root files and
   any draft/active plan whose links are meant to resolve. Search every old
   path as a plain-text TOKEN (e.g. `tests/world.rs`, `src/world.rs`,
   `agent-interface.md`, `log/log.md`), not only as a Markdown link — flat
   path mentions in research and backlog prose count.
3. Link-and-anchor sweep: every relative `.md` link and anchor resolves. This
   check INCLUDES draft/active plans; only frozen history (log entries,
   completed/superseded plans, archives) is excluded.
4. Old-token scan: outside `.agents/plans/**`, the log, archives and the
   move-map fragments, no old path token from the fragments survives.
5. Duplication greps: candidate ids, digests, the thirteen-name enumeration
   (allowed only in code, fixture, `docs/game/capability/` filenames and the
   game index), delivery-status phrases — excluding `.agents/plans/**`
   (explicit planning citations), the log and archives.
6. Reference-direction sweep: no runtime-side file (`docs/game/**`, `src/**`,
   `tools/aicadia-local`, `tools/aicadia-agent`, `web/`) links to
   `docs/concept/`, `docs/research/` or `.agents/` as authority; the only
   runtime→development links are static pointers into `docs/evidence/`.
7. Monolith audit: `wc -l` over current-authority docs and source modules
   against the bounded-size rule and its exemptions.
8. Align `.agents/backlog/README.md` wording and capability-map links; flag —
   without resolving here — SKILL.md's broader duplication of AGENTS.md rules
   as a candidate follow-up choice; confirm role headers exist on every
   authority file.
9. Run the full validation ladder.

**Invariants:**

- No unrelated user change touched; frozen plans and archived history
  unmodified except as this plan states.

**Evidence:**

- The validation ladder below, recorded in this plan on completion.

**Stop conditions:**

- Any grep hit that the constitution cannot place — return to root.

## Validation ladder

1. **Focused:** per-task evidence above (greps, diffs, wc, entry counts,
   `jq` newline probe, one-to-one `cargo test -- --list` mapping).
2. **Contract:** `cargo test --all-targets` green; `tests/agent-tool-catalog.json`
   byte-identical; `tests/aicadia-local.sh`, `tests/agent-playtest.sh`,
   `tests/trait-playtest.sh` pass; a live `tools/list` + `server/discover`
   against a locally started server matches the fixture and the relocated
   instructions.
3. **Outcome:** one status change rehearsal — editing the Trait status touches
   exactly one file (`docs/evidence/trait.md`), because every other site holds
   a static pointer; one tool-description iteration rehearsal — editing
   `src/agent_contract/tool/submit_action.md` touches no Rust; one code
   navigation rehearsal — the Property write path is findable from
   `src/world/property.rs` alone; a new reader can answer "where does X live?"
   from `docs/README.md` alone.
4. **Integrity:** `git diff --check`; focused diff review; unrelated user
   changes and all governing authorities intact; the concept log records the
   acceptance and completion.

## Change control

Refine paths, task order and stronger evidence in place while the accepted
outcome and contract remain unchanged. Stop implementation, set `status: draft`,
revise and request explicit re-acceptance when new evidence changes the outcome,
public behavior, domain meaning, non-goals, irreversible state, external
authority, material cost or evidence claim — in particular if any step would
change published Agent-text bytes, catalog content, the public `World` surface
or game semantics.

## Completion conditions

- every required task is `completed` and the validation ladder passes;
- the exact evidence claim is demonstrated: unchanged runtime surface,
  one-to-one test inventory, one home per truth, resolving live links, bounded
  authority files and source modules;
- `AGENTS.md`, `CLAUDE.md`, `CONTEXT.md`, `docs/`, `.agents/backlog/` and the
  concept log are mutually aligned with no known-stale authority;
- `status: complete` and `completed_at` are recorded only after these
  conditions.

Follow-up note (non-binding, not a gate): the natural next plans after this one
are (a) the Agent-text readability rewrite — instruction sectioning, revised
pinned phrases, regenerated fixture, optional adapter discover-fetch — and
(b) the live-controller `startswith` correction with a freshly frozen digest.
Neither is decided here.
