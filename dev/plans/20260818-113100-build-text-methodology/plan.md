---
status: complete
created_at: "2026-08-18T11:31:00+02:00"
updated_at: "2026-08-18T14:05:00+02:00"
accepted_at: "2026-08-18T11:58:00+02:00"
completed_at: "2026-08-18T14:05:00+02:00"
backlog_item: dev/backlog/items/build-text-methodology.md
---

# Build-facing text methodology and capability-contract pass

> **Role / side:** proportional build plan / development side.
> **Authority:** owns this build's accepted outcome, task graph, invariants and evidence claim.
> **Excludes:** current product truth and reusable build rules; see `game/docs/` and `AGENTS.md`.

## Outcome

Aicadia gets one written method for **build-facing text** — the documentation a
building Agent (Codex, Claude Code or any other model reading the repository) must
understand before it changes code, schema or contract. The method lives in
`dev/docs/methodology/build-text.md` next to `public-text.md`, is bound by the
documentation constitution and one sentence in `AGENTS.md`, and is applied once,
completely, to the fifteen capability contracts in `game/docs/capability/` as its
first application.

After completion a building Agent that opens any capability contract can tell,
from the file alone and in plain English: what this file owns, what is deliberately
not here and which file owns that instead, and — for every fact the file depends on
elsewhere — what the fact is, in which relation it stands to this file and where it
lives, without opening the target to find out whether it must. `game/docs/README.md`
states, per kind of change, which files to read in which order. Every reference is a
sentence, never a bare link; every file has one reason to exist that the file
states; nothing is said in two places unless one is a declared derivation of the
other.

One small companion change to published text rides along (User decision
2026-08-18): each of the fifteen tool descriptions is laid out as blocks — the
template label on its own line, its text on the next line, one blank line between
blocks — so the same bytes read well for a person in any host that shows
descriptions as plain text. Words do not change; only whitespace does.

The exact evidence claim: (1) the method record exists, is indexed, and the
constitution, `AGENTS.md` and `dev/CONTEXT.md` point to or name it once each;
(2) all fifteen capability contracts follow the fixed shape, their headers name the
sibling authorities by path and relation, no cross-cutting Agent-conduct sentence
remains in them, and a removal inventory shows that every removed sentence has one
named owner elsewhere — no capability-owned fact is lost; (3) `cargo test
--workspace` is green, so the existing documentation lint (role headers, links,
anchors, index completeness) accepts every touched file; (4) the reading-path table
in `game/docs/README.md` cites only runtime-side paths; (5) one bounded builder
wayfinding probe records, per change task, whether a small model names the intended
files and the one file to edit from the old and from the new texts; (6) the
fifteen descriptions differ from today's bytes only in whitespace — every
whitespace-collapsed description is byte-equal to its previous collapsed form —
and the catalog fixture is regenerated once. Claim (5) is limited to the exact
model, tasks and prompts exercised.

## Non-goals

- No change to any capability's semantics, input, validation, result, Activity
  footprint, annotations or errors; no change to `World`, HTTP, MCP, storage or
  migrations. Published Agent text changes in whitespace only (T6); no word of the
  play contract, a description or a schema description changes, and no rule is
  added, dropped or weakened.
- No test or lint that enforces document shape, pointer form, relation vocabulary
  or duplication. These are review guidance in the method (User decision
  2026-08-18, same as for public text). The existing lint stays exactly as it is;
  the role-header labels `Role / side`, `Authority` and `Excludes` are kept because
  the lint parses them — the method changes what the lines say, not their labels.
- No repository-wide header or pointer rewrite now. Model contracts, concern
  documents and development records adopt the sharpened form when they are next
  edited for another reason; only the fifteen capability contracts and the files
  this plan touches anyway are rewritten here.
- No new Studio page or projection; the side-by-side capability view (contract ·
  published description · schema) is noted as a later option, not built.
- No paid live playtest; the probe is the only model spend and is bounded below.

## Evidence and authorities

| Evidence or authority | Current fact | Consequence for this build |
| --- | --- | --- |
| `dev/docs/README.md#reference-direction` | "References point toward the owning authority… shorten a summary to a pointer whenever a change to the owned fact would otherwise require editing both homes." | The principle exists; this plan gives it a sentence form and a relation vocabulary, and sharpens the header rule. |
| `dev/docs/README.md#authority-file-headers` | Header states role/side, authority, and "what does not belong there and which home owns that content". | *Excludes* already asks for the owning home; today it often names an abstract category ("cross-cutting Agent conduct") without a path or relation. |
| `studio/src/record.rs:237-239` | Lint parses exactly `**Role / side:**`, `**Authority:**`, `**Excludes:**`. | Keep the labels; change content only. No parser change. |
| `game/docs/capability/*.md` (15 files, 47–165 lines) | Identical section set: MCP publication · Purpose · Input · (Contract · Input example) · Validation · Result · Retry and tool-local safety · Activity footprint · Errors · Workshop link · Evidence obligations. *Errors* is the same one-line pointer in all fifteen; *Workshop link* is one pointer; *Retry and tool-local safety* repeats "content, never instructions" and "keep identifiers out of player-visible language" — Agent conduct owned by the play contract and `game/docs/agent.md`. | Collapse pure-pointer sections into the header; drop conduct sentences whose owner is elsewhere; keep every World-owned fact. |
| `dev/docs/methodology/public-text.md` | Owns writing rules, layers, size guidance, change procedure and probe for published text; checklist line "capability/<tool>.md and agent.md say the same thing as the published text where they restate it". | The build-text record cites its writing rules instead of repeating them; the checklist line is re-pointed to the capability shape. |
| `game/docs/README.md` | One general reading order (domain → models → capabilities → protocol → parity → agent → storage → deferred → local play). | Add a per-change reading-path table; runtime side may cite only runtime paths. |
| `dev/lab/agent-text/` | Track header says "public-text experiment track"; probe 01 ran through the Codex CLI with a fixed question set, ≤ 8 calls. | Widen the track to both readers; probe 02 reuses the runner pattern with wayfinding tasks. |
| `game/mcp/agent/tool/*.md`, `game/src/agent_contract.rs:82-`, `:293-300` | Each source file is published byte-for-byte via `include_str!`; today one line per label with no blank line, so hosts show one dense block. Pins check `starts_with("What it does:")`, `contains("\nNever:")` and whitespace-collapsed anchors. | Label-on-own-line blocks with one blank line keep every pin green; only the catalog fixture must be regenerated once. |
| `AGENTS.md#one-home-per-truth`, `#earn-your-spot`, `#public-text-is-product-surface` | Rules exist for one home, earning a place, and public text; no rule names how build docs are read. | One added sentence under *One Home Per Truth* pointing to the method; no new heuristic. |
| User direction 2026-08-18 | Every file must earn its place; references must be understandable in natural language by the building Agent; no link soup and no duplicates. | Governs the method's content and the acceptance of this plan. |

## Alignment

### Strategic

Aicadia is built by models. Every contract they misread costs a wrong edit, a second
home for a fact, or a duplicated rule that drifts. A building Agent that can tell
from any file what it owns, what it excludes and where the rest lives edits the
right file the first time and never has to copy a fact to keep it in view. This
plan does for the build side what the public-text plan did for the play side; the
next capability's contract is written right the first time. The concrete risk after
it: model and concern documents still carry the older header and pointer style
until touched, so a builder meets two styles for a while.

### Tactical

The smallest complete slice: one method record; the constitution's reference and
header rules sharpened; five relation words defined; the fixed capability shape
defined and applied to all fifteen contracts; a per-change reading-path table; one
bounded probe. Excluded: any change of meaning; the wider rewrite of other document
kinds; lint. Terry gate: not applicable — no runtime state, contention or cost is
touched.

### Technical

Documentation and lab seams only. `World`, PostgreSQL, transactions, HTTP/MCP,
tool descriptions, catalog fixture and pins: not applicable. Tests: only the
existing documentation lint in `studio/tests/studio/` runs against the touched
files. Operations: none.

## Decisions, assumptions and open questions

### Confirmed decisions

- **Two method records, not one.** `build-text.md` owns what differs for a reader
  who edits the repository (relation vocabulary, pointer sentence, wayfinding
  header, document shapes, reading paths, builder probe); sentence-level writing
  rules stay owned by `public-text.md#writing-rules` and are cited, not copied —
  the record's first pointer demonstrates the form. Reason: one home per rule,
  `public-text.md` is already near the size guidance, and renaming it would churn
  `AGENTS.md`, the log, the lab track and the completed plan for no meaning gain.
- **Header labels stay; header content changes.** *Authority* says in plain words
  what the file owns; *Excludes* names each excluded topic with its relation and
  the owning path or anchor. Reason: the lint parses the labels; the User rejects
  new lint or ceremony; the wayfinding value is in the sentence, not the label.
- **Five relation words** — *defined in*, *constrained by*, *published as*,
  *narrowed here*, *recorded in* — owned by the method record; the constitution
  requires a pointer to state fact, relation and path in one sentence. Reason: a
  builder must know without opening the target whether it must; a small fixed set
  reads as natural language and stays consistent.
- **Three link kinds, three places.** Body text links only upward, once per
  section, at the point of use; indexes link downward and completely; sideways
  links live only in the header. No "see also", no back-links, no linking of
  every term on every mention. Reason: this is the anti-link-soup rule the User
  asked for, and the lint's index-completeness check already covers downward links.
- **Capability contracts are the first and only application in this plan.**
  Reason: the User's example; fifteen files of one shape; the change is mechanical
  once the shape is written; other kinds follow on touch.
- **Description layout: label on its own line, text below, one blank line between
  blocks; no Markdown headings.** Reason: most hosts render descriptions as plain
  text, so blank lines help a person and a model alike while `#` shows as literal
  noise; the layout costs about one token per block; the User chose this variant
  over inline labels for readability. Recorded in `public-text.md` as part of the
  L1 template.
- **Reading paths live in `game/docs/README.md`**, runtime paths only; the
  procedures a change also needs (public-text parity, planning) are reached through
  `AGENTS.md`, which already points to the methodology index. Reason: the
  constitution's reference direction; the table is reading order, which that index
  owns.

### Reversible assumptions

- Proposed capability shape (T1 fixes it; the inventory in T3 checks it against
  the fifteen files): header (*Authority* = the World-side contract of `<tool>`;
  *Excludes* = Agent wording — *published as* `game/mcp/agent/tool/<tool>.md`;
  workshop or read conduct — *defined in* `game/docs/agent.md#<anchor>`; shared
  value rules — *defined in* `domain.md#shared-value-validation`; canonical error
  codes — *defined in* `protocol.md#canonical-errors`), then body sections
  **Purpose · Input · Validation · Result · Activity footprint · Annotations and
  retry class · Evidence obligations**, with optional **Contract** and **Input
  example** for the four package-shaped capabilities. Sections *Errors* and
  *Workshop link* (pure pointers) collapse into the header; the conduct sentences
  of *Retry and tool-local safety* are removed and its idempotency class stays
  under *Annotations and retry class*. Cannot change the contract: every removed
  sentence has an owner listed in the inventory. Checked in T3 and the ladder.
- The probe uses the Codex CLI already used by the runners, one small model class
  at low effort, six change tasks, old and new trees, at most twelve calls, run
  through a temporary read-only git worktree for the old tree. If a second model
  class is wanted it is a separate announced budget.
- The `dev/lab/agent-text/` track is widened to both readers rather than opening a
  second track; if the track index grows past two readers' worth, split later.

### Open questions

None material. The User has directed the outcome, the natural-language requirement
and the no-lint boundary; the two structural choices above (two records; header
labels kept) are stated so the User can flip them before acceptance.

## Implementation map

| Surface | Current state | Intended change | Invariants |
| --- | --- | --- | --- |
| `dev/docs/methodology/build-text.md` | absent | New record: what counts as build-facing text; why it is written differently (a model reads it at the moment of change); relation vocabulary; pointer sentence form; wayfinding header; the three link kinds; document shape principle and the capability shape; reading paths (pointer to `game/docs/README.md`); adopt-on-touch rule; change procedure; builder probe; checklist for a new capability contract | Cites `public-text.md#writing-rules`; owns no product truth |
| `dev/docs/methodology/README.md` | lists `public-text.md` | Add `build-text.md` link line | Index links only |
| `dev/docs/README.md#reference-direction`, `#authority-file-headers` | principle stated; header asks for owning home | Pointer = fact + relation + path in one sentence (method in `build-text.md`); *Excludes* names topic + relation + path; body links upward at point of use, indexes downward, sideways only in header | Labels, lint, sweep scope and home table unchanged |
| `dev/CONTEXT.md` | has *Public-facing text* | Add *Build-facing text* term after it | Terminology only |
| `AGENTS.md#one-home-per-truth` | five lines | One added sentence: write pointers and role headers by `dev/docs/methodology/build-text.md` | No new heuristic; Reference Docs unchanged |
| `game/docs/README.md` | reading order + indexes | New section *Reading paths by change* — one table row per change kind (capability semantics · model or storage · protocol or wire · Agent-facing text · local operation · Studio projection), runtime paths only | Index stays index; no restated contract |
| `game/docs/capability/*.md` (15) | eleven-section shape, abstract *Excludes*, pointer sections, conduct sentences | Header and body to the fixed shape; removal inventory in this plan directory | Every World-owned fact, example and evidence obligation kept verbatim or tightened without meaning change |
| `dev/docs/methodology/public-text.md` checklist | line points capability/agent docs at "say the same thing" | Re-point to the capability shape in `build-text.md`; one line | No other change |
| `dev/lab/agent-text/README.md`, `02-builder-wayfinding-probe/` | track header names public text only; one experiment | Widen header/intro to both readers; add experiment 02 (tasks, prompt, run, results, verdict) with front matter per the constitution | Bounded, announced, no background spend |
| `game/mcp/agent/tool/*.md` (15) | one line per label, no blank line | Label on its own line, text on the next, one blank line between blocks; no word changes | Whitespace-collapsed text byte-equal to before; pins green |
| `game/mcp/tool-catalog.json` | fixture with today's description bytes | Regenerated once; only `description` strings differ, and only in whitespace | Names, schemas, annotations untouched |
| `dev/docs/methodology/public-text.md#layers-one-home-per-rule` (L1 row) and `#writing-rules` | template labels listed; no layout rule | One clause: blocks — label line, text line, blank line between; no headings | No other change |
| Concept log, backlog item and row | — | *planned* now, *completed* at the end; `Now` row `Proposed` → `Active` → `Done` | Log append-only |

## Execution contract

Root owns outcome, scope, plan state, integration and the final evidence claim. A
delegated Agent receives this plan path and one dependency-ready task id, re-reads
the live repository, changes only its owned surfaces, runs focused evidence and
returns raw results. Delegation is optional. T3 may be split by capability file
between Agents once T1 has fixed the shape; write surfaces then do not overlap.

## Task graph

Allowed states are `pending`, `in_progress`, `completed` and `blocked`.

| ID | State | Depends | Parallel-safe | Objective | Owned surfaces | Evidence |
| --- | --- | --- | --- | --- | --- | --- |
| T1 | completed | — | no | Method record written and bound: relation vocabulary, pointer sentence, wayfinding header, three link kinds, capability shape, adopt-on-touch, probe procedure, checklist; constitution, `AGENTS.md`, vocabulary and indexes updated | `dev/docs/methodology/build-text.md`, `dev/docs/methodology/README.md`, `dev/docs/README.md` (two sections), `dev/CONTEXT.md`, `AGENTS.md#one-home-per-truth`, `dev/docs/methodology/public-text.md` (checklist line) | Lint green; every pointer in the new record itself follows the form it prescribes |
| T2 | completed | T1 | with T3 | Reading-path table in the game-contract index | `game/docs/README.md` | Lint green; each row cites only runtime paths that exist |
| T3 | completed | T1 | with T2 | All fifteen capability contracts in the fixed shape with wayfinding headers; removal inventory complete | `game/docs/capability/*.md`, `dev/plans/20260818-113100-build-text-methodology/inventory.md` | Inventory: every removed sentence → owner path; no World-owned fact lost; lint green; line count before/after per file |
| T4 | completed | T2, T3 | no | Builder wayfinding probe run and recorded, old versus new, bounded | `dev/lab/agent-text/README.md`, `dev/lab/agent-text/02-builder-wayfinding-probe/**` | ≤ 12 model calls; per-task score recorded; verdict limited to model, tasks and prompts |
| T6 | completed | — | with T1–T3 | Fifteen descriptions laid out as blocks; fixture regenerated once; layout rule in the public-text method | `game/mcp/agent/tool/*.md`, `game/mcp/tool-catalog.json`, `dev/docs/methodology/public-text.md` (L1 template clause) | Collapsed-whitespace diff empty for all fifteen; `cargo test -p aicadia-game` pins green; fixture diff shows only `description` whitespace |
| T5 | completed | T4, T6 | no | Choice and completion recorded; backlog and log aligned | `dev/docs/concept/log/2026-08.md`, `dev/backlog/README.md`, `dev/backlog/items/build-text-methodology.md`, this plan | Lint green; plan `complete` |

## Task details

### T1 — Method record and bindings

**Objective:** `dev/docs/methodology/build-text.md` exists and every surface that
must name it does so once.

**Actions:**

1. Write the record with the sections listed in the implementation map; the first
   pointer in it (to `public-text.md#writing-rules`) is written in the prescribed
   form and serves as the worked example.
2. Define the five relation words with one line each and one example each; state
   the pointer form `<fact> — <relation> in [<owned thing>](<path#anchor>); <what this file
   adds or does not add>`.
3. State the header rule and the three link kinds; state the document-shape
   principle (every section owned or pointer-only; a section outside the shape is
   a new fact needing an owner or a copy) and the capability shape from the
   reversible assumption above.
4. State the adopt-on-touch rule for other document kinds and the checklist for a
   new capability contract.
5. Sharpen `dev/docs/README.md#reference-direction` and `#authority-file-headers`
   (pointer sentence, header content, three link kinds) with one pointer to the
   method; add the `AGENTS.md` sentence, the `dev/CONTEXT.md` term, the index line
   and the re-pointed `public-text.md` checklist line.

**Invariants:** header labels, lint, home table and sweep scope unchanged; no
product truth in the record; `AGENTS.md` gains one sentence, no heuristic.

**Evidence:** `cargo test --workspace -q` green (documentation lint); a read of the
record confirms every pointer in it uses one of the five relations.

**Stop conditions:** stop if the sharpened constitution rule would require a lint
or parser change, or if the capability shape cannot hold a World-owned fact of one
of the fifteen files without a new section.

### T2 — Reading paths by change

**Objective:** `game/docs/README.md` states, per kind of change, which runtime
files to read in which order.

**Actions:**

1. Add *Reading paths by change* with one row each for: capability semantics;
   model or storage; protocol or wire; Agent-facing text; local operation; Studio
   projection — path list in reading order, no prose beyond one clause per row.

**Invariants:** runtime paths only; index restates no contract.

**Evidence:** lint green; every path exists.

**Stop conditions:** stop if a row genuinely needs a development-side path — then
that path belongs in the method record's reading-path section instead.

### T3 — Capability contracts in the fixed shape

**Objective:** all fifteen files follow the shape; the inventory proves no loss.

**Actions:**

1. Record every sentence to be removed or moved per file in `inventory.md` with its
   owner path (`game/docs/agent.md#…`, `protocol.md#canonical-errors`, the tool
   description, or "kept — reworded").
2. Rewrite header (*Authority*, *Excludes* with relations and paths) and body
   sections; keep Contract and Input example where they exist; keep every
   World-owned fact, atomicity and concurrency guarantee, Activity footprint,
   annotation class and evidence obligation.
3. Record line count before and after per file in the inventory.

**Invariants:** no meaning change; the four package-shaped capabilities keep their
examples; anchors referenced from active records still resolve (none exist today
outside plans).

**Evidence:** inventory complete; lint green; `git diff --stat game/docs/capability/`.

**Stop conditions:** stop if a sentence has no owner anywhere else and is not a
World-owned fact — that is a rule without a home and needs the User.

### T4 — Builder wayfinding probe

**Objective:** a bounded record of whether a small model finds the intended files
better from the new texts than from the old.

**Actions:**

1. Announce the spend; fix six change tasks (for example: change the maximum name
   length; add an optional field to `create_character`; change the Activity
   footprint of `submit_interaction`; change the wording an Agent uses for
   `enter_world`; add a canonical error code; change the launcher's port choice),
   each with the intended read path and the one file to edit, derived from T2.
2. Prompt: repository is available read-only; list files opened in order and the
   file you would edit; do not edit. Run against a temporary worktree of the
   pre-change tree and against the working tree; one small model class, low
   effort; at most twelve calls.
3. Record model identity, prompts, raw answers, per-task match and the verdict
   with real and simulated seams in the experiment README; widen the track header.

**Invariants:** no background spend; no World, server or database; verdict limited
to the exact model, tasks and prompts.

**Evidence:** experiment README with front matter valid under the lint; call count
≤ 12.

**Stop conditions:** stop if the Codex CLI cannot run read-only against a worktree;
record `inconclusive` rather than widening the budget.

### T6 — Description block layout

**Objective:** every tool description reads as labelled blocks in any plain-text
host, with no word changed.

**Actions:**

1. Rewrite each of the fifteen source files: label followed by a newline, its text
   on the next line, one blank line between blocks; the same labels in the same
   order as today.
2. Regenerate `game/mcp/tool-catalog.json` once through the existing generation
   path; confirm only `description` strings differ.
3. Add the layout clause to the L1 template in `public-text.md`.

**Invariants:** the whitespace-collapsed text of every description is byte-equal
to before; no word, label, order or boundary changes; names, schemas and
annotations untouched.

**Evidence:** a script comparing `split_whitespace().join(" ")` of old versus new
for all fifteen prints no difference; `cargo test -p aicadia-game -q` (pins) green;
`git diff game/mcp/tool-catalog.json` shows only description whitespace.

**Stop conditions:** stop if a pin or the fixture test depends on the single-line
form in a way that would need a semantic test change.

### T5 — Record and close

**Objective:** the choice and completion are recorded once; backlog and log agree.

**Actions:** append the *completed* log entry; set the backlog row `Done` and the
item's completion evidence; set this plan `complete`.

**Evidence:** lint green; `git diff --check`.

## Execution record

- T1: `dev/docs/methodology/build-text.md` written (212 lines); index line;
  constitution paragraphs under *Reference direction* and *Authority-file
  headers*; *Build-facing text* term; one sentence in `AGENTS.md#one-home-per-truth`;
  `public-text.md` checklist line re-pointed to the capability shape.
- T2: *Reading paths by change* table (five rows, runtime paths only) in
  `game/docs/README.md`; the Studio row was deliberately left out per the stop
  condition and the table says where development-side surfaces are named.
- T3: fifteen contracts rewritten to the shape (1,113 → 871 lines; words level at
  5,249 → 5,371 because pointers now carry relation and "adds only" clauses);
  removal inventory in `inventory.md`; independent Opus parity review: no
  World-owned fact lost, nine findings all resolved (see the inventory).
- T6: fifteen descriptions laid out as label blocks; whitespace-collapsed text
  byte-equal to before for all fifteen; catalog regenerated once through the
  ignored fixture test with `DATABASE_URL=postgres://localhost:5433/postgres`;
  structural comparison against a fixture regenerated from the old sources: equal
  except description whitespace; L1 layout clause and size note in `public-text.md`.
- T4: probe `dev/lab/agent-text/02-builder-wayfinding-probe/`, twelve Codex calls
  (gpt-5.4-mini · low; six tasks × old and new tree): old 4/6 owners found, mean
  recall 0.72, 45 files opened; new 6/6, 0.85, 43 files; verdict `supported`.
- T5: this record, the log, the backlog.

## Validation result (2026-08-18)

1. Focused: `cargo test --workspace` green with the local database
   (2 + 50 + 14 + 81 + 67 + 51 tests, one ignored regeneration test);
   `cargo clippy --workspace --all-targets --all-features -D warnings` clean;
   `cargo fmt --check` clean; `studio/tests/aicadia-local.sh`,
   `dev/tests/agent-playtest.sh` and `dev/tests/trait-playtest.sh` exit 0.
2. Contract: collapsed-whitespace description diff empty for all fifteen; the
   pins in `agent_contract.rs` green unchanged; inventory complete with owners and
   confirmed by the review; only the five relation words occur in the fifteen
   contracts (35 *defined in*, 18 *constrained by*, 15 *published as*); no conduct
   phrase (`rg` for the removed phrases) remains in `game/docs/capability/`.
3. Outcome: the header of `create_character.md` and of `submit_action.md` answers
   "what is here, what is not, where is the rest" in three lines with three
   resolving links; the probe's two old misses (edit code before contract; stop at
   the never-published `agent.md`) both resolved on the new texts.
4. Integrity: `git diff --check` clean; the uncommitted public-text and local-play
   changes from other sessions untouched; authorities aligned (`AGENTS.md`,
   constitution, vocabulary, methodology index, `game/docs/README.md`, lab track,
   backlog, log).

## Validation ladder

1. **Focused:** `cargo test --workspace -q` after T1, T2, T3 (documentation lint).
2. **Contract:** collapsed-whitespace description diff empty (T6); inventory complete with owners; `rg` for the removed conduct
   phrases across `game/docs/capability/` returns nothing; every relation word used
   is one of the five.
3. **Outcome:** a fresh read of one simple and one package-shaped capability file
   answers "what is here, what is not, where is the rest" from the header alone;
   probe recorded.
4. **Integrity:** `git diff --check`; focused diff review; the uncommitted
   public-text and local-play changes from other sessions remain intact.

## Change control

Refine paths, task order and stronger evidence in place while the accepted outcome
and contract remain unchanged. Stop implementation, set `status: draft`, revise and
request explicit re-acceptance when new evidence changes the outcome, public
behavior, domain meaning, non-goals, irreversible state, external authority,
material cost or evidence claim.

## Completion conditions

- every required task is `completed` and the validation ladder passes;
- the exact strategic outcome and evidence claim are demonstrated;
- current behavior, concept choices, vocabulary and backlog are aligned;
- no known-stale authority, material open question or accidental unrelated change
  remains;
- `status: complete` and `completed_at` are recorded only after these conditions.
