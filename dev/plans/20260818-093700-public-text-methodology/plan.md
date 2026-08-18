---
status: complete
created_at: "2026-08-18T09:37:00+02:00"
updated_at: "2026-08-18T11:05:00+02:00"
accepted_at: "2026-08-18T09:45:00+02:00"
completed_at: "2026-08-18T11:05:00+02:00"
backlog_item: dev/backlog/items/public-text-methodology.md
---

# Public-facing text methodology and Agent-text rewrite

> **Role / side:** proportional build plan / development side.
> **Authority:** owns this build's accepted outcome, task graph, invariants and evidence claim.
> **Excludes:** current product truth and reusable build rules; see `game/docs/` and `AGENTS.md`.

## Outcome

Aicadia gets one written method for every **public-facing text** — text the
repository publishes verbatim to a party outside the repository: the
`server/discover` play contract, the fifteen tool descriptions and the JSON-Schema
`description` fields inside the published catalog. The method lives in
`dev/docs/methodology/public-text.md`, is pointed to by one compact `AGENTS.md`
rule, and is applied once, completely, to every text an Agent receives today.

After completion every Agent — strong or weak, in any conforming host — receives a
contract that states each rule exactly once, in plain imperative English, with the
play loop first, without numeric bounds that the schema already enforces, without
development jargon, and at roughly half of today's size. **No rule is added,
dropped, widened or weakened**: a numbered rule inventory maps every current
sentence to exactly one destination, and an independent parity review gates it.
Sentences that give the Agent nothing to do (World-implementation facts) move to
`game/docs/agent.md`, which is never published.

The exact evidence claim: (1) the inventory shows zero lost, zero added and zero
weakened rules; (2) `cargo test`, `cargo clippy` and both playtest regression suites
are green against the deliberately regenerated catalog fixture and the redesigned
pins; (3) a live server serves `server/discover.instructions` byte-equal to the
assembled sections and `tools/list` byte-equal to the fixture; (4) the published
word count of instructions and descriptions is recorded before and after; and (5)
one bounded comprehension probe (T7) records, per model class, how many of a fixed
question set each model answers correctly from the old and from the new texts.
Claim (5) is limited to the exact models, prompts and questions exercised.

## Non-goals

- No semantic change to any rule, capability, bound, error, workflow or retry
  behavior; no new or removed capability; catalog names, schema structure,
  constraints and annotations untouched (only `description` strings change).
- No change to `World`, HTTP semantics, storage or migrations.
- No test or lint that enforces word budgets, layers or style. Budgets are review
  guidance in the methodology, never a build gate (User decision 2026-08-18).
- No model-specific text, no provider branch, no "weakest supported model" target.
  The method writes for clarity and low context cost because any model may be
  reading; it does not tune for one.
- No Dutch or localized text; Agents translate for their Users.
- No paid live playtest beyond the bounded probe in T7; no background token spend.
- No change to `studio/` beyond registering the new documentation home in the
  existing home map (`studio/src/home.rs`) so the current lint keeps passing.

## Evidence and authorities

| Evidence or authority | Current fact | Consequence for this build |
| --- | --- | --- |
| `game/mcp/agent/instruction/00..15.md` | 16 sections, 2,833 words / 19.5k chars, assembled by `game/src/agent_contract.rs` and served through `server/discover` | Rewritten and restructured; the assembly list and pins change with it |
| `game/mcp/agent/tool/*.md` | 15 descriptions, 2,245 words / 15.4k chars; each restates the three-proposal loop, numeric bounds and several cross-cutting rules | Rewritten to the fixed template with only the bounded restatement set |
| `game/mcp/tool-catalog.json` | 142k bytes; 323 schema `description` fields (111 unique, 24k chars); output schemas alone 53k chars; schema already carries `minLength`/`maxLength` for every bound the prose repeats | Schema descriptions shortened at their doc-comment source; fixture regenerated once through the ignored test named in `game/docs/protocol.md#mcp-publication-invariants` |
| `game/src/wire/input.rs`, `output.rs`, `investigation.rs` | Doc comments are the source of every schema `description` (e.g. "The one closed first-slice consequence.") | Doc-comment-only edits; no type, field or constraint change |
| `game/src/agent_contract.rs` tests | Pins are long literal sentences and section headings | Redesigned into short meaning anchors chosen in T2; still prove every non-negotiable boundary is present |
| `game/docs/agent.md#instruction-layering` | Names the per-tool template and the four deliberately restated boundaries; other repetition "is a defect" | Becomes the runtime-side layering contract (L0 schema · L1 description · L2 instructions · L3 this document); receives the moved implementation facts |
| `dev/plans/20260814-192553-agent-text-rewrite/` | Prior rewrite chose "clarity over compactness", no budget; its inventory technique (R1) proved parity | Same inventory technique reused; its editorial rule set is superseded by the methodology |
| `dev/docs/README.md` home table, `studio/src/home.rs` | Every Markdown file under `dev/docs/` must match exactly one registered home | New home `dev/docs/methodology/` registered in both |
| `AGENTS.md` | No rule addresses published text as product surface | One compact rule added, pointing to the methodology |
| `dev/docs/research/current-mcp-agent-guidance.md` | Hosts inject instructions differently; a caller may skip discovery; comprehension varies by model | Motivates layer discipline and the bounded per-tool restatement set |
| `dev/playtest/trait/schema/live-candidate.sha256` | Rewritten text is future candidate material for a paid live run | No candidate is pending; a later live run re-audits its digest as usual |
| User decisions 2026-08-18 | No test ceremony for editorial rules; probe included; no target model; natural language, bullets welcome; `AGENTS.md` rule + methodology file | Confirmed decisions below |

## Alignment

### Strategic

The published texts are the actual player interface: every conforming host feeds
them to its model before the first player word. Today an Agent pays roughly 14k
tokens (27k where a host forwards output schemas) for a contract that repeats
itself three times over and speaks development jargon. Halving that while keeping
every rule makes play cheaper and more reliable for every present and future
Agent, and gives every later capability a method so its text starts right.

### Tactical

One complete pass over exactly three published surfaces (instruction sections,
tool descriptions, schema descriptions), plus one method document, one `AGENTS.md`
rule and one bounded comprehension probe. Excluded: every semantic change.

### Technical

Seams: `game/mcp/agent/**` (sections, tools), `game/src/agent_contract.rs`
(assembly list, pins), `game/src/wire/*.rs` (doc comments only),
`game/mcp/tool-catalog.json` (one regeneration), `game/tests/server/` (fixture and
discover comparisons keep working unchanged), `game/docs/agent.md` +
`game/docs/capability/*.md` (layering contract, publication lines),
`dev/docs/methodology/` (new home), `dev/docs/README.md` + `studio/src/home.rs` +
`studio/tests/studio/lint.rs` (home registration), `AGENTS.md`, `dev/CONTEXT.md`
(term `public-facing text`), `dev/lab/agent-text/` (probe), backlog + concept log.
Not applicable: `World`, HTTP handlers, schema structure, storage, migrations.

## Decisions, assumptions and open questions

### Confirmed decisions

- A method document, not tests, governs public-facing text; budgets are review
  guidance — User decision 2026-08-18 ("ceremony that will not help").
- The bounded comprehension probe is in scope, explicit and token-bounded — User
  decision 2026-08-18.
- The method is model-agnostic: it states that any model may read the text and
  therefore demands plain, clear, low-cost writing; natural language with bullets
  is welcome — User decision 2026-08-18.
- `AGENTS.md` gains one compact rule for public-facing texts that points to
  `dev/docs/methodology/public-text.md` — User decision 2026-08-18.
- Semantic parity is the gate: every current rule keeps exactly one home. A
  sentence that gives the Agent no action (a World-implementation fact) moves to
  `game/docs/agent.md` and is no longer published — User agreement 2026-08-18 on
  example 3 (`04-property.md`, "no control-word denylist").
- The play loop (read · three proposals · preview · confirm · submit once · tell
  only what was accepted · retry rule) is stated once, first, and no longer repeated
  per section or per description — User agreement 2026-08-18 on example 1.
- Numeric bounds already enforced by schema constraints (`0–100`, `1–4,000`,
  `1–120`, page sizes) leave the prose; a description names a bound only where the
  schema cannot express it — User agreement 2026-08-18 on examples 1–3.
- Runtime-side `game/docs/agent.md` states the layering contract itself and never
  points into `dev/`; the methodology (development side) cites it — documentation
  constitution, reference direction.

### Reversible assumptions

- Budget guidance recorded in the methodology: instructions ≈ 1,400–1,600 words;
  one description ≤ ~100 words; one schema description ≤ ~15 words; the guidance
  may be tuned during T3–T5 when clarity requires it and is recorded as the observed
  result, never enforced.
- Section boundaries start from the current sixteen files plus one new loop
  section; they may merge, split or reorder during T3 while the inventory stays
  fully covered.
- The probe uses two model classes reachable from this machine through the Codex
  CLI already used by the runners (one small, one large), the same fixed question
  set (≈ 12–16 yes/no questions with expected answers grounded in the inventory),
  and at most eight model calls in total (2 models × old/new text × ≤ 2 repeats).
  Model identity, token use and prompts are recorded in the experiment.
- The methodology home is `dev/docs/methodology/` with an index `README.md` and
  one record `public-text.md`; registration copies the `research-report` pattern.

### Open questions

- None. The plan awaits explicit User acceptance.

## Implementation map

| Surface | Current state | Intended change | Invariants |
| --- | --- | --- | --- |
| `dev/docs/methodology/README.md`, `public-text.md` | absent | Index plus the method: definition of public-facing text; layers L0–L3 with one-home rule; budget guidance; writing rules; inventory procedure for any change; probe procedure; checklist for a new tool/section | Development side; cites `game/docs/agent.md` for the layering contract, never duplicates it |
| `AGENTS.md` | no rule | One compact heuristic (working title *Public Text Is Product Surface*): published text costs every reader context, is written once per rule, follows the methodology; pointer added under Reference Docs | Compact; no duplicated method content |
| `dev/docs/README.md`, `studio/src/home.rs`, `studio/tests/studio/lint.rs` | no methodology home | Home-table row; `methodology-index` and `methodology-record` homes; id list updated | Existing lint keeps passing; no new lint |
| `dev/CONTEXT.md` | no term | `public-facing text` defined once | Vocabulary only |
| Plan-dir `inventory.md` | absent | Every rule in 16 sections + 15 descriptions + schema descriptions numbered with destination (L0/L1/L2/L3), merge marks and the new pin anchors | Working artifact; parity gate for T8 |
| `game/mcp/agent/instruction/**` | 16 sections | Rewritten per methodology: loop section first; each rule once; plain imperative; no schema-owned bounds; no development jargon | Every inventory rule with destination L2 lands exactly once |
| `game/src/agent_contract.rs` | assembly list + literal pins | Assembly list follows new sections; pins become short anchors from the inventory | Startup completeness assert kept; `instructions()` seam unchanged |
| `game/mcp/agent/tool/*.md` | 15 descriptions | Same template; only the four restated boundaries; no loop, no schema-owned bounds | One file per tool; names unchanged |
| `game/src/wire/*.rs` doc comments | e.g. "first-slice", repeated bound prose | Short plain descriptions; bounds stay as constraints | No type, field, constraint or serde change |
| `game/mcp/tool-catalog.json` | current bytes | Regenerated once via the ignored test; diff reviewed | Only `description` strings differ |
| `game/docs/agent.md` | template + four boundaries | Layering contract L0–L3; receives moved implementation facts; supersedes the 2026-08-14 editorial rule set | Runtime side; no pointer into `dev/` |
| `game/docs/capability/*.md` | publication lines echo descriptions | Aligned where wording is restated | Capability semantics untouched |
| `dev/lab/agent-text/README.md`, `01-comprehension-probe/` | absent | Track index; experiment with question set, expected answers, runner script, recorded answers per model and text version, verdict | Lab contract front matter; explicit token bound; no production dependency |
| `dev/backlog/README.md`, `dev/backlog/items/public-text-methodology.md` | no `Now` row | One `Now` item for this edge; `Done` at completion | Backlog points to authorities |
| Concept log `dev/docs/concept/log/2026-08.md` | — | Entries at planning, acceptance and completion | Append-only |

## Execution contract

Root owns outcome, scope, plan state, integration and the final evidence claim. A
delegated Agent receives this plan path and one dependency-ready task id, re-reads
the live repository, changes only its owned surfaces, runs focused evidence and
returns raw results. Delegation is optional. T3, T4 and T5 may run in parallel
after T2 fixes the inventory and anchors; they own disjoint files.

## Task graph

| ID | State | Depends | Parallel-safe | Objective | Owned surfaces | Evidence |
| --- | --- | --- | --- | --- | --- | --- |
| T1 | completed | — | yes | Methodology, `AGENTS.md` rule, home registration, vocabulary | `dev/docs/methodology/**`, `AGENTS.md`, `dev/docs/README.md`, `dev/CONTEXT.md`, `studio/src/home.rs`, `studio/tests/studio/lint.rs` | Studio tests green; methodology complete per its own checklist |
| T2 | completed | — | yes | Rule inventory with destinations and pin anchors | plan-dir `inventory.md` | Every current sentence has one row and one destination |
| T3 | completed | T1, T2 | yes | Instruction sections rewrite + assembly + pins | `game/mcp/agent/instruction/**`, `game/src/agent_contract.rs` (instruction side) | `cargo test` green; every L2 row ticked |
| T4 | completed | T1, T2 | yes | Tool descriptions rewrite + pins | `game/mcp/agent/tool/*.md`, `game/src/agent_contract.rs` (tool side) | `cargo test` green; every L1 row ticked |
| T5 | completed | T1, T2 | yes | Schema descriptions + one fixture regeneration | `game/src/wire/*.rs` doc comments, `game/mcp/tool-catalog.json` | fixture diff touches only `description`; every L0 row ticked |
| T6 | completed | T3, T4, T5 | no | Runtime docs, backlog and log alignment | `game/docs/agent.md`, `game/docs/capability/*.md`, `dev/backlog/**`, concept log | Every L3 row landed; no stale wording |
| T7 | completed | T3, T4, T5 | no | Comprehension probe old vs new, two model classes | `dev/lab/agent-text/**` | Recorded answers and verdict within the token bound |
| T8 | completed | T6, T7 | no | Independent parity review + validation ladder | read-only + this plan | Zero lost/added/weakened rules; ladder green |

## Task details

### T1 — Methodology, `AGENTS.md` rule, home registration, vocabulary

**Objective:** The method exists, is findable from `AGENTS.md`, and the repository
lint accepts its home.

**Actions:**

1. Write `dev/docs/methodology/public-text.md` with: what counts as public-facing
   text and where each kind is sourced; the layer model (L0 schema constraints and
   short field meaning · L1 tool description, fixed template, four restated
   boundaries · L2 global instructions, loop first, cross-tool rules once · L3
   `game/docs/agent.md`, rationale and implementation facts, never published); the
   one-home rule; budget guidance and the recorded current sizes; writing rules
   (one rule per line, imperative, plain words, positive first, concrete tool names,
   no schema-owned bounds in prose, no examples in descriptions, stable headings and
   order, canonical vocabulary only where the reader must act on it); the change
   procedure (inventory rows → destinations → parity check → one fixture
   regeneration → probe when wording changes materially); the probe procedure and
   its token bound; and the checklist for a new tool or section.
2. Write `dev/docs/methodology/README.md` as the home index.
3. Add the `AGENTS.md` heuristic and Reference Docs pointer; add the home-table row
   in `dev/docs/README.md`; register `methodology-index` and `methodology-record`
   in `studio/src/home.rs` and the id list in `studio/tests/studio/lint.rs`; add
   `public-facing text` to `dev/CONTEXT.md`.

**Invariants:** the methodology cites `game/docs/agent.md` for the layering
contract instead of restating it; `AGENTS.md` stays compact.

**Evidence:** `cargo test -p aicadia-studio` (or the workspace Studio test target)
green with the new files present; `rg -n "public-text" AGENTS.md dev/docs/README.md`.

**Stop conditions:** the lint needs more than home registration; the methodology
starts to duplicate `game/docs/agent.md`.

### T2 — Rule inventory with destinations and pin anchors

**Objective:** Every sentence of the current published texts has one numbered row
with source, rule, destination layer/section/tool, merge mark and, where
non-negotiable, its short pin anchor.

**Actions:**

1. Enumerate the 16 sections, 15 descriptions and the 111 unique schema
   descriptions into `inventory.md` (source path, current wording, rule in one
   line, destination, mark: `keep`, `merge-into-loop`, `bound→schema`,
   `fact→L3`, `dup-of #n`).
2. Choose the pin anchors: short phrases that prove a boundary is present, taken
   from the new wording, at most one per non-negotiable boundary.
3. Draft the new section list (files, titles, order) with the loop section first
   after role and authority.

**Invariants:** no row deleted; a `fact→L3` row is only a sentence with no Agent
action; the anchor set covers every boundary the current pins protect.

**Evidence:** row count ≥ number of source sentences; every current pin string maps
to a row.

**Stop conditions:** a sentence's meaning is ambiguous enough that its destination
would change semantics — return to root for a decision.

### T3 — Instruction sections rewrite, assembly and pins

**Objective:** The served contract is the rewritten set of sections.

**Actions:**

1. Create the new section files per T2 (loop section included), remove or renumber
   old ones, and update `INSTRUCTION_SECTION` in `game/src/agent_contract.rs`.
2. Replace the instruction-side pin tests with the T2 anchors.
3. Tick every L2 and merge row in `inventory.md`.

**Invariants:** the `instructions()` seam and the discover comparison in
`game/tests/server/` keep working; only English; each rule once.

**Evidence:** `cargo test` green; `wc -w game/mcp/agent/instruction/*.md` recorded
in the plan.

**Stop conditions:** a rule cannot be placed without changing meaning.

### T4 — Tool descriptions rewrite and pins

**Objective:** Fifteen descriptions follow the template with only the four restated
boundaries and no schema-owned bounds.

**Actions:** rewrite `game/mcp/agent/tool/*.md`; replace tool-side pins with T2
anchors; tick L1 rows.

**Invariants:** template labels only; each file ends with one newline (startup
assert); no loop restatement.

**Evidence:** `cargo test` green (fixture test will fail until T5 regenerates —
run T5 before the final green); word counts recorded.

**Stop conditions:** as T3.

### T5 — Schema descriptions and one fixture regeneration

**Objective:** Schema `description` strings are short and plain; the fixture is the
new byte truth.

**Actions:** edit doc comments in `game/src/wire/input.rs`, `output.rs`,
`investigation.rs`; run the ignored `regenerate_agent_tool_catalog_fixture` test
once after T4 lands; review the diff; tick L0 rows.

**Invariants:** no type, field, constraint, serde or annotation change; only
`description` strings differ in the fixture diff.

**Evidence:** `git diff --stat game/mcp/tool-catalog.json`; a `jq` comparison
showing every non-`description` value unchanged; `cargo test` green.

**Stop conditions:** the diff shows any non-description change.

### T6 — Runtime docs, backlog and log alignment

**Objective:** No stale authority remains.

**Actions:** update `game/docs/agent.md#instruction-layering` to the L0–L3
layering contract and receive every `fact→L3` row; align
`game/docs/capability/*.md` publication lines; add the backlog `Now` item and row;
append log entries.

**Invariants:** runtime side never points into `dev/`; capability semantics
untouched.

**Evidence:** every `fact→L3` row ticked; Studio lint green; `rg` for removed
wording finds no stale copy.

### T7 — Comprehension probe, old versus new, two model classes

**Objective:** Recorded, bounded evidence of whether the new texts are understood
at least as well as the old ones by two model classes.

**Actions:**

1. Create `dev/lab/agent-text/README.md` (track) and
   `01-comprehension-probe/README.md` with the lab front matter (question, seams,
   token bound, verdict placeholder) and `questions.md`: ≈ 12–16 yes/no questions
   with expected answers and their inventory rows (e.g. "May you call
   `submit_action` before the User confirms the whole package?").
2. Add a small script that assembles a prompt from a given instructions text and
   catalog file plus the questions, calls one named model once through the Codex
   CLI, and stores the raw answers; the old texts come from
   `git show <accepted-commit>:<path>`.
3. Announce, then run: 2 models × old/new, at most eight calls; record model ids,
   token use, answers and score.
4. Write the verdict (`supported` / `refuted` / `inconclusive`) and what would
   falsify it; set artifact status `kept`.

**Invariants:** no World, server or database involved; no background spend; the
probe proves comprehension of these questions only.

**Evidence:** the recorded answer files and the verdict in the experiment README.

**Stop conditions:** the CLI or models are unavailable — record `inconclusive`
with the reason and continue T8; the new text scores clearly lower — return to
root before T8, revise wording, re-run within the remaining bound.

### T8 — Independent parity review and validation ladder

**Objective:** Independent confirmation of zero lost/added/weakened rules and a
green ladder.

**Actions:** a fresh read-only review compares old texts, `inventory.md` and new
texts; run the ladder; record results in the plan; set `complete`.

**Evidence:** review report with zero findings or resolved findings; ladder output.

## Execution record

- T1: `dev/docs/methodology/README.md` + `public-text.md`; `AGENTS.md` heuristic
  *Public Text Is Product Surface* + Reference Docs pointer; home-table row;
  `methodology-index`/`methodology-record` homes registered in `studio/src/home.rs`,
  labels and the Rules tree in Studio; lint id list; `public-facing text` in
  `dev/CONTEXT.md`. Studio suite green.
- T2: `inventory.md` — every old sentence rowed (A–X, C, D) with destination and
  mark; anchors in section E.
- T3: 15 sections (16 → 15; loop section `03-loop.md` first after role and
  authority): 2,833 → 2,387 words, 19.5k → 15.9k chars. `agent_contract.rs`
  assembly list and anchor pins (whitespace-collapsed matching); duplicated
  instruction pins removed from `game/tests/server/protocol.rs`. Sizes recorded in
  the methodology; the contract landed above the size guidance because parity
  with ≈ 150 distinct rules bounds compression.
- T4: 15 descriptions: 2,245 → 1,756 words; template kept; only the four
  boundaries restated; no loop, no schema-owned bounds.
- T5: doc comments in `game/src/wire/{input,output,investigation}.rs`; schema
  descriptions 24.2k → 10.0k chars, longest 13 words; fixture regenerated (three
  times in total during T4/T5/T8 wording fixes; final diff touches only
  `description` strings — verified by structural comparison). `dev/playtest/trait/run`
  catalog-phrase pins repointed to the new wording; both token-free playtest suites
  green.
- T6: `game/docs/agent.md` → *Instruction layering* (four layers, loop once, bounded
  restatement set, moved facts K5 and normalization); `game/mcp/agent/README.md`;
  Studio outline tests (15 sections); research link repointed; backlog `Now` item;
  evidence paragraph in `dev/docs/evidence/agent-contract.md`; log entries.
- T7: `dev/lab/agent-text/01-comprehension-probe/` — seven Codex calls; final texts
  16/16 on gpt-5.4-mini (low) and gpt-5.6-sol (medium); one interim miss (Q4)
  fixed by splitting a stacked negative list into a positive-first bullet
  (K6/O6, one home). Verdict `supported`, status `kept`.
- T8: independent read-only parity review (fresh agent, HEAD vs working tree
  against the inventory): no boundary lost, all anchors present, schema
  constraints unchanged, no orphan old sentence; four low-severity findings,
  all resolved — (1) preview names the Property type again (L4/AC2); (2) loop
  step 1 is scoped to Actions, Interactions and investigations, and creation/
  entry start from `get_character` alone (O1/E1); (3) "never invent what a
  Character knows or noticed" restored in 07 (O4); (4) 04-world names the
  complete set of writing calls without "only". Two unsure points also taken:
  "current Properties and Traits" (K3) and `enter_world` restates the
  no-background boundary. Probe call 7 (mini, final bytes) 16/16. Ladder below.

## Validation result (2026-08-18)

1. Focused: `cargo test --workspace` green (14 + 50 + 51 + 67 + 81 + 2 tests, one
   ignored regeneration test), `cargo clippy --workspace --all-targets
   --all-features -D warnings` clean, `cargo fmt --check` clean;
   `studio/tests/aicadia-local.sh`, `dev/tests/agent-playtest.sh` and
   `dev/tests/trait-playtest.sh` exit 0.
2. Contract: live `cargo dev` on port 3477 served `server/discover.instructions`
   byte-equal to the assembled sections (15,908 bytes) and `tools/list` equal to
   the regenerated fixture (15 tools); structural comparison of HEAD vs new fixture:
   only `description` strings differ.
3. Outcome: sizes recorded in the methodology; inventory fully mapped and reviewed;
   probe verdict `supported` (seven calls; both models 16/16 on the rewritten
   texts; call 7 on the final bytes).
4. Integrity: `git diff --check` clean; the other session's uncommitted local-play
   changes are untouched; authorities aligned (`game/docs/agent.md`, methodology,
   `AGENTS.md`, `dev/CONTEXT.md`, backlog, evidence, log).

## Validation ladder

1. **Focused:** `cargo test` (pins, fixture, discover comparison), Studio tests,
   `dev/tests/agent-playtest.sh` and `dev/tests/trait-playtest.sh` token-free
   suites.
2. **Contract:** live `cargo dev` server: `server/discover.instructions` byte-equal
   to `agent_contract::instructions()`; `tools/list` byte-equal to the fixture;
   fixture diff limited to `description` strings.
3. **Outcome:** before/after word and character counts per surface recorded; T7
   probe verdict recorded; inventory fully ticked; parity review clean.
4. **Integrity:** `git diff --check`, focused diff review, unrelated user changes
   preserved (including the currently uncommitted local-play edits), all governing
   authorities aligned.

## Change control

Refine paths, section boundaries, budgets and evidence in place while the outcome
and semantic parity stay unchanged. Stop, set `status: draft`, revise and request
re-acceptance if any rule must change meaning, if the probe bound must grow, or if
the fixture diff needs a non-description change.

## Completion conditions

- every task `completed`, ladder green, evidence claim demonstrated;
- `game/docs/agent.md`, methodology, `AGENTS.md`, vocabulary, backlog and log
  aligned;
- no known-stale authority, open material question or unrelated change remains;
- `status: complete` and `completed_at` recorded only after these conditions.
