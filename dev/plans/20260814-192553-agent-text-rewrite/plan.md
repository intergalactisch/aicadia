---
status: complete
created_at: "2026-08-14T19:25:53+02:00"
updated_at: "2026-08-14T20:21:52+02:00"
accepted_at: "2026-08-14T19:31:00+02:00"
completed_at: "2026-08-14T20:21:52+02:00"
---

# Agent play-text rewrite: clarity, structure and single delivery

## Outcome

Every text an Agent receives to play Aicadia — the published play contract
(`game/mcp/agent/instruction.md`, 2,278 words of wall-text under ALL-CAPS
headings) and the thirteen tool descriptions (`game/mcp/agent/tool/*.md`) —
is rewritten for comprehension: natural language, real Markdown structure,
short sentences, consistent `dev/CONTEXT.md` vocabulary, and a fixed per-tool
template, under one explicit editorial rule set (below). The contract source
becomes one small file per section, assembled at compile time. The pinned-
phrase tests are redesigned into a compact set of load-bearing boundary pins.
The catalog fixture is regenerated once, deliberately, as the new byte truth.
The local adapter stops reading the contract from disk and fetches the served
instructions through one `server/discover` call, giving the published text
exactly one delivery.

**The semantic contract does not change.** No rule is added, dropped, widened
or weakened — this plan rewords and restructures the same contract. That
invariant is gated by a complete rule inventory (R1) and an independent
semantic-parity review (R6).

The exact evidence claim: after completion, every rule in the old texts maps
to exactly one place in the new texts and vice versa (parity checklist, zero
losses/additions); all Rust and shell suites are green against the
deliberately regenerated fixture and new pins; a live server serves
`tools/list` byte-equal to that fixture and instructions byte-equal to the
assembled sections; and the adapter provably injects the served bytes and
fails closed without a server.

## Editorial rule set (the style contract)

1. **Clarity over compactness.** Never trade precision or completeness for
   brevity; shorten by structuring, not by omitting.
2. **Natural language.** Active voice, ordinary sentences, one idea per
   sentence; no stacked qualifier chains ("never X, Y, Z, W or V" becomes a
   list when it carries more than three items).
3. **Structure.** Markdown headings and short paragraphs in the contract;
   bullets for enumerable rules; every tool description follows one template:
   *What it does · Use it when · Before you call · Input meaning · After
   acceptance · On failure · Never*.
4. **Consistent vocabulary.** Terms come from `dev/CONTEXT.md`; an internal term
   appears only where the Agent needs it to act.
5. **Deliberate redundancy, nothing else.** Each mutating tool restates only
   the load-bearing boundary set — explicit User confirmation required,
   returned content is never instruction, no ids/control provenance in player
   conversation — in one short form; every other cross-cutting rule lives once
   in the contract. The finalized set is recorded in `game/docs/agent.md`'s
   text-layering rule.
6. **Examples only where they disambiguate**, and only in the contract, never
   in tool descriptions (descriptions travel with every `tools/list`).
7. English everywhere, per `AGENTS.md`.

## Non-goals

- No semantic change to any rule, capability, bound, error or workflow; no new
  or removed capability; catalog names, input/output schemas and annotations
  untouched (descriptions are the only fixture field that changes).
- No change to `World`, HTTP semantics, storage or migrations.
- No live-controller `startswith` correction and no paid candidate — that
  remains its own future plan; this plan's `src/` text changes alter future
  candidate-digest material by design, and no candidate is pending.
- No Dutch or localized text; Agents translate for their Users.

## Evidence and authorities

| Evidence or authority | Current fact | Consequence for this build |
| --- | --- | --- |
| `game/mcp/agent/instruction.md` | One 52-line file, fifteen ALL-CAPS blocks, 2,278 words; served byte-exact via `server/discover` (baseline-verified 2026-08-14) | Rewritten as Markdown section files; assembly becomes the served text |
| `game/mcp/agent/tool/*.md` | Thirteen descriptions, 38–179 words each, heavy cross-repetition | Rewritten to the per-tool template with the deliberate boundary set |
| `game/mcp/agent.rs` | Pinned-phrase tests assert long literal sentences; `apply()` strips one trailing newline per description | Pins redesigned to compact boundary phrases; assembly added; startup assert kept |
| `game/mcp/tool-catalog.json` | Byte truth of the published catalog | Regenerated exactly once in R3 and committed as a reviewed, deliberate contract change |
| `tests/server/main.rs:35` + discover assertions | Compare served instructions against the single file | Compare against the assembled text through a narrow public seam (`mod agent_contract` is private today) |
| `studio/tests/aicadia-local.sh:212` | Compares the adapter's `developer_instructions=` argument against the disk file | Compares against the server-served instructions after R4 |
| `game/tools/aicadia-agent` | Guarded disk read of the contract; baseline green | One `server/discover` fetch with the exact stateless headers/`_meta` shape; `jq -ec '.result.instructions'` (never decode-and-re-encode); fail-closed at the same early position |
| `game/docs/agent.md` text-layering rule | Names today's deliberate per-tool redundancies | Updated in the same change to the finalized boundary set |
| `game/docs/capability/*.md` | Each carries an "MCP publication" annotation summary | Any restated description wording is aligned in R5 |
| `dev/playtest/trait/run` predicate | Candidate material now includes `*.md` under `src/` | Rewritten text is future freeze material; no pending candidate exists, so drift blocks nothing |
| User decisions 2026-08-14 | Markdown + section files; deliberate core redundancy per tool; redesigned compact pins; scope includes the adapter discover-fetch | Confirmed decisions below |
| Baseline verification 2026-08-14 | All suites green; live `tools/list` and `server/discover` byte-equal to fixture and file | The before-state every regression is measured against |

## Alignment

### Strategic

The Agent texts are the product's actual player interface: every conforming
host feeds them to its model, and their comprehensibility directly bounds play
quality for every current and future Agent. The delivered architecture made
them cheap to iterate; this plan spends that capability on the first quality
pass, before the next live playtest plan relies on these texts.

### Tactical

One complete editorial pass over exactly two text surfaces (contract sections,
tool descriptions) plus the single-delivery adapter change, gated by a rule
inventory, redesigned pins, one deliberate fixture regeneration and an
independent semantic-parity review. Excluded: every semantic change.

### Technical

Seams: `game/mcp/agent/**` (sections, tools, assembly), `game/mcp/agent.rs`
(pins, assembly, narrow public seam), `src/lib.rs` (seam visibility),
`tests/server/` (comparison source), `game/mcp/tool-catalog.json`
(regeneration), `game/tools/aicadia-agent` + `studio/tests/aicadia-local.sh` (discover
fetch), `game/docs/agent.md` + `game/docs/capability/*.md` (layering rule and
publication lines), concept log (decision entries). Not applicable: `World`,
HTTP handlers, schemas, storage, migrations.

## Decisions, assumptions and open questions

### Confirmed decisions

- Markdown structure with one file per contract section, assembled at compile
  time — User decision 2026-08-14. The relocation plan deliberately deferred
  this split until wording changed; it changes now.
- Each mutating tool keeps the deliberate short boundary set (confirmation,
  content-never-instruction, id/provenance privacy); all other cross-cutting
  rules live once in the contract — User decision 2026-08-14.
- Pinned tests are redesigned: fewer, shorter, meaning-anchored boundary
  phrases per text, chosen in R1; the fixture remains the byte truth — User
  decision 2026-08-14.
- Scope includes the adapter discover-fetch (single delivery) — User decision
  2026-08-14.
- The editorial rule set above governs every rewritten sentence; clarity wins
  over compactness — User direction 2026-08-14.

### Reversible assumptions

- Soft length ceilings guide, never gate: ~150 words per tool description,
  ~2,500 words for the assembled contract. Exceeding them is acceptable when
  clarity requires it (rule 1 outranks them).
- Section boundaries start from the fifteen existing blocks and may merge or
  reorder during R2 while the rule inventory stays fully covered.
- The narrow public seam is a `pub fn instructions() -> &'static str` (or
  equivalent) exposing only the assembled text; its exact shape may shift
  during R2.

### Open questions

- None. The plan awaits explicit User acceptance.

## Implementation map

| Surface | Current state | Intended change | Invariants |
| --- | --- | --- | --- |
| Rule inventory (plan directory) | absent | `inventory.md`: every rule/bound/boundary in the current texts, numbered, with its destination in the new structure; the new pin set per text | R6 gates parity against this inventory; working artifact of this plan |
| `game/mcp/agent/instruction/` | absent (one `instruction.md`) | One Markdown file per section, rewritten per the editorial rules; explicit ordered assembly | Every inventory rule lands exactly once; served text = assembly |
| `game/mcp/agent/instruction.md` | the served contract | Removed after the assembly serves the rewritten text | Removal and seam switch in one task |
| `game/mcp/agent/tool/*.md` | thirteen relocated originals | Rewritten to the per-tool template with the deliberate boundary set | One file per tool; names unchanged |
| `game/mcp/agent.rs` | relocation-era pins; per-tool includes; newline strip | Assembly for sections; redesigned pin tests (compact boundary phrases per text); startup completeness assert kept | Pins cover every non-negotiable boundary named in the inventory |
| `src/lib.rs` | `mod agent_contract;` private | Narrow public seam for the assembled instructions | Nothing else becomes public |
| `tests/server/main.rs` + protocol assertions | include the single file | Compare served instructions against the public seam | Discover test remains byte-exact |
| `game/mcp/tool-catalog.json` | relocation-era bytes | Regenerated once; reviewed as the deliberate contract change | Only `description` fields differ from the previous fixture |
| `game/tools/aicadia-agent` | guarded disk read | One `server/discover` POST (headers `Content-Type`, `Accept: application/json, text/event-stream`, `MCP-Protocol-Version: 2026-07-28`, `Mcp-Method: server/discover`; `params._meta` with protocolVersion/clientInfo/clientCapabilities); `jq -ec '.result.instructions'`; fail-closed when absent or non-string | Injected bytes equal served bytes; no fallback source; no decode round-trip |
| `studio/tests/aicadia-local.sh` | compares against disk file | Compares recorded `developer_instructions=` against the served instructions | Test still proves byte-exact injection and fail-closed startup |
| `game/docs/agent.md` | layering rule names current redundancies | Finalized deliberate boundary set recorded; host paragraph updated for single delivery | Layering rule stays the editorial authority for future text changes |
| `game/docs/local-play.md` | describes disk-read adapter | Describes the discover-fetch delivery | Fail-closed meaning unchanged |
| `game/docs/capability/*.md` | publication lines from relocation | Aligned where they restate description wording | Capability semantics untouched |
| Concept log (current period file) | — | One entry at acceptance, one at completion | Every Choice Leaves A Trail |

## Execution contract

Root owns outcome, scope, plan state, integration and the final evidence
claim. A delegated Agent receives this plan path and one dependency-ready task
id, re-reads the live repository, changes only its owned surfaces, runs
focused evidence and returns raw results. Delegation is optional. R2 and R3
may run in parallel only after R1 fixes the inventory, template and pin set;
they own disjoint files.

## Task graph

| ID | State | Depends | Parallel-safe | Objective | Owned surfaces | Evidence |
| --- | --- | --- | --- | --- | --- | --- |
| R1 | completed | — | no | Rule inventory, section structure, per-tool template, new pin set | plan-dir `inventory.md` | Every current rule numbered with one destination; pin set covers every non-negotiable boundary |
| R2 | completed | R1 | yes | Contract rewrite as section files + assembly + seam + new contract pins | `game/mcp/agent/instruction/**`, `instruction.md` removal, `game/mcp/agent.rs` (instruction side), `src/lib.rs`, `tests/server/` comparison | `cargo test` green; served text = assembly; inventory items for the contract all ticked |
| R3 | completed | R1 | yes | Thirteen tool descriptions rewritten + fixture regeneration + new tool pins | `game/mcp/agent/tool/*.md`, `game/mcp/agent.rs` (tool side), `game/mcp/tool-catalog.json` | `cargo test` green; fixture diff touches only `description` fields; inventory items for tools all ticked |
| R4 | completed | R2 | no | Adapter single delivery via discover | `game/tools/aicadia-agent`, `studio/tests/aicadia-local.sh` | Suite passes; stopped-server run fails closed; injected bytes equal served bytes |
| R5 | completed | R2, R3 | no | Documentation alignment + log entries | `game/docs/agent.md`, `game/docs/local-play.md`, `game/docs/capability/*.md`, concept log | Layering rule matches shipped redundancy; no stale wording |
| R6 | completed | R4, R5 | no | Independent semantic-parity review + validation ladder | read-only + this plan | Zero lost/added/weakened rules against the inventory; ladder green |

## Task details

### R1 — Rule inventory, structure, template and pin set

**Objective:** The complete semantic content of the current texts is
enumerated and every item has one destination; the new pin set is chosen.

**Actions:**

1. Extract every rule, bound, boundary and behavioral instruction from
   `instruction.md` and the thirteen descriptions into numbered inventory rows
   (rule, source location, destination section/tool, deliberate-redundancy
   flag).
2. Fix the section structure of the new contract and the per-tool template
   order.
3. Choose the new pin set: per text, the short boundary phrases whose absence
   must fail the build; map each old pinned phrase to its successor or to an
   inventory row explaining why a pin is no longer separately needed.

**Invariants:**

- No inventory row may be marked "drop"; this plan rewords, it never removes.

**Evidence:**

- Inventory reviewed at root: complete, numbered, destination per row; pin map
  complete.

**Stop conditions:**

- Stop if any current sentence turns out to be semantically ambiguous such
  that rewording requires an interpretation choice — that choice goes to the
  User before R2/R3 start.

### R2 — Contract rewrite

**Objective:** The play contract is served as the assembled, rewritten
Markdown sections.

**Actions:**

1. Write `game/mcp/agent/instruction/<nn>-<section>.md` per R1's
   structure, rewriting per the editorial rules; assemble in
   `agent_contract.rs` via an explicit ordered `include_str!` list.
2. Expose the assembled text through the narrow public seam; repoint the
   `tests/server/` comparisons; remove `instruction.md` in the same change.
3. Replace the contract-side pinned tests with R1's new pin set.

**Invariants:**

- Every contract inventory row lands exactly once; assembly is the only
  served form; no build script or new dependency.

**Evidence:**

- `cargo test` green; a live `server/discover` returns exactly the assembled
  text; contract inventory rows ticked.

**Stop conditions:**

- Stop on any rewording that cannot preserve a rule's exact meaning — return
  to the User with the sentence pair.

### R3 — Tool description rewrite

**Objective:** Thirteen descriptions follow the template, carry the deliberate
boundary set and nothing else cross-cutting, and the fixture is regenerated.

**Actions:**

1. Rewrite each `tool/*.md` per the template and editorial rules; mutating
   tools carry the short boundary set; read tools carry only what is local.
2. Replace the tool-side pinned tests with R1's set; regenerate
   `game/mcp/tool-catalog.json` once and review its diff: only
   `description` fields change.

**Invariants:**

- Schemas, annotations, names and order byte-identical in the fixture diff.

**Evidence:**

- `cargo test` green; fixture diff inspection; tool inventory rows ticked.

**Stop conditions:**

- Same interpretation-choice rule as R2.

### R4 — Adapter single delivery

**Objective:** `game/tools/aicadia-agent` injects exactly the served instructions,
fetched live, fail-closed.

**Actions:**

1. Replace the disk read with the `server/discover` POST described in the
   implementation map; extract with `jq -ec '.result.instructions'` and fail
   closed on absence, error or non-string — at the same early position as
   today's file guard.
2. Repoint `studio/tests/aicadia-local.sh` to compare against the served
   instructions.

**Invariants:**

- No fallback source; no decode/re-encode round-trip; byte-exact injection.

**Evidence:**

- `studio/tests/aicadia-local.sh` green; manual stopped-server run fails closed with
  a clear message.

**Stop conditions:**

- Stop if byte-exact extraction proves impossible in shell — bring the seam
  back to root rather than approximating.

### R5 — Documentation alignment

**Objective:** The game docs describe the shipped texts and delivery.

**Actions:**

1. Record the finalized deliberate-redundancy set in `game/docs/agent.md` and
   update its host-delivery paragraph; update `game/docs/local-play.md`.
2. Align capability files where they restate description wording.
3. Concept-log entries at acceptance and completion (current period file).

**Invariants:**

- No doc gains contract authority it did not have; evidence home untouched.

**Evidence:**

- Grep: no stale pre-rewrite phrasing in live game docs.

**Stop conditions:**

- Stop if alignment would require changing a capability contract — that is a
  semantic change and out of scope.

### R6 — Independent parity review and ladder

**Objective:** Independent confirmation that meaning is unchanged and the
system is green end to end.

**Actions:**

1. An independent reviewer (fresh context) walks the R1 inventory against the
   new texts: every rule present, none added, none weakened; readability
   judged against the editorial rule set.
2. Run the full validation ladder.

**Invariants:**

- Findings are fixed and re-reviewed, or the plan stops in draft for the User.

**Evidence:**

- Review verdict recorded in this plan; ladder results recorded.

**Stop conditions:**

- Any parity finding the inventory cannot resolve — User decision.

## Validation ladder

1. **Focused:** inventory tick-off per task; fixture diff limited to
   `description` fields; pin tests green.
2. **Contract:** `cargo test --all-targets` green; `studio/tests/aicadia-local.sh`,
   `dev/tests/agent-playtest.sh`, `dev/tests/trait-playtest.sh` green; live
   `tools/list` byte-equal to the regenerated fixture; live `server/discover`
   byte-equal to the assembly; adapter injection byte-equal to the served
   text.
3. **Outcome:** independent parity review returns zero lost/added/weakened
   rules; a fresh reading of any single tool description answers what/when/
   before/after/never without consulting another text; editing one section or
   one tool file touches exactly one text file.
4. **Integrity:** `git diff --check`; concept log records acceptance and
   completion; evidence home untouched; no unrelated change.

## Completion record (2026-08-14)

Independent R6 review: first pass NO-GO with 3 P1 / 11 P2 / advisory P3;
every finding was applied (including the deliberate P2-5 sign-off: the tool
set now uniformly says "null" for the nullable wire fields, matching
`game/src/wire/output.rs`). Focused re-review returned **GO**: semantic parity
complete, no rule lost, added or weakened; bounds, error identifiers and
catalog order mechanically verified identical. All four advisory items were
then also adopted (create_entity wording, enter_world section order, "remote"
restored in the target-id prohibition, and the pin net re-extended with
`non-executable`, the preview-lifecycle phrase, the target-authored boundary
and the no-background enumeration anchor).

Validation ladder: `cargo test --all-targets` green (18+2+13+75, identical
inventory); fixture regenerated — final committed diff touches exactly the 13
`description` fields; `studio/tests/aicadia-local.sh` green including the new
served-instructions comparison; `dev/tests/agent-playtest.sh` green;
`dev/tests/trait-playtest.sh` green for its advertised token-free contract (no
DATABASE_URL). With an admin DATABASE_URL its final regression test
fails-closed at the candidate digest-drift gate — pre-existing since the
documentation-architecture plan changed `src/` and deliberate per resolved
OQ2: the frozen digest `6649…` is stale by design until a future live plan
freezes a fresh one; one runner catalog pin
(`dev/playtest/trait/run:100`) was updated to the new description phrasing in
this plan. Live end-to-end: `server/discover` serves exactly the assembled
sections and the adapter injects exactly those served bytes, fail-closed
without a server.

## Change control

Refine wording, section boundaries, pin choices and task order freely inside
the accepted outcome. Stop, return to draft and re-request acceptance if any
change would alter a rule's meaning, a capability, a schema, an error, the
World surface or the delivery guarantees — semantic parity is this plan's
outcome, not a preference.

## Completion conditions

- every task `completed`; the validation ladder passes;
- the parity review confirms zero semantic drift against the R1 inventory;
- the editorial rule set is demonstrably applied (R6 readability judgment);
- the single-delivery guarantee is proven (served = assembled = injected);
- `status: complete` and `completed_at` recorded only after these conditions.
