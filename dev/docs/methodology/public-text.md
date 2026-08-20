# Public-facing text

> **Role / side:** working method for text published outside the repository / development side.
> **Authority:** how public-facing text is written, changed and checked.
> **Excludes:** the published texts themselves and their layering contract; see `game/mcp/agent/` and `game/docs/agent.md#instruction-layering`.

## What counts as public-facing text

Public-facing text is any text the repository publishes verbatim to a party
outside the repository. Today that is:

| Text | Source | Delivered through |
| --- | --- | --- |
| Play contract | `game/mcp/agent/instruction/*.md`, assembled by `game/src/agent_contract.rs` | `server/discover.instructions` |
| Tool descriptions | `game/mcp/agent/tool/*.md` | `tools/list` `description` |
| Schema descriptions | doc comments on `game/src/wire/*.rs` types | `tools/list` `inputSchema` / `outputSchema` and OpenAPI |

Everything else — `game/docs/`, `dev/`, Studio pages — is read by builders, not
by playing Agents, and is not public-facing text.

## Why it is written differently

Every conforming host feeds these texts to its model before the first player word,
in every conversation, and the model may be strong or weak. The texts therefore
have two costs at once: context tokens for every reader, and comprehension for the
reader least able to reconstruct meaning from long prose. Both fall when a rule is
stated once, plainly, where the reader needs it. Nothing here tunes for one model;
it writes so that any model can act on the text.

## Layers: one home per rule

The published texts and the document behind them form four layers. A rule lives in
exactly one of them; `game/docs/agent.md#instruction-layering` is the
runtime contract that names the layers and the one bounded restatement set.

| Layer | Carries | Never carries |
| --- | --- | --- |
| **L0 schema** | field meaning in one short clause; every numeric bound, enum, format and required field as a constraint | workflow, cross-tool rules, development words |
| **L1 tool description** | this tool's local contract in the fixed template: *What it does · Use it when · Before you call · Input meaning · After acceptance / After the call · On failure · Never*; plus the four restated boundaries in one clause each. Laid out as blocks: the label on its own line, its text on the next, one blank line between blocks, no Markdown headings — plain text reads well in every host | the play loop, another tool's rules, bounds the schema already enforces, examples |
| **L2 play contract** | what spans tools: role, authority, the play loop, what exists, Properties, Traits, knowledge, targets, storytelling, entry, Actions, Interactions, investigation, recovery | per-tool preconditions, schema-owned bounds, implementation facts |
| **L3 `game/docs/agent.md`** | rationale, host conduct, implementation facts an Agent cannot act on | anything the Agent needs in order to act |

A sentence that gives the Agent nothing to do belongs in L3 and is not published.
The one deliberate overlap: the loop names the reads once in general, and each
mutating description names its own exact read set, because a host may invoke a
tool without ever loading the contract.

## Writing rules

1. **One rule per line.** Imperative, one action, about twenty words or fewer.
   Bullets and numbered steps are welcome; prose paragraphs are for framing only.
2. **Loop first.** The play loop (read · three proposals · preview · confirm ·
   submit once · tell only what was accepted · retry only an uncertain delivery)
   is stated once, early, and never repeated per section or per tool.
3. **Positive first.** Say what to do. Use *never* for a boundary the reader would
   otherwise cross, not as a defensive list.
4. **Concrete over abstract.** Name the tool, the field, the error code. Prefer
   "call `get_character` first" to "ground through authoritative reads".
5. **Plain words.** A `dev/CONTEXT.md` term appears only where the reader must act
   on it; otherwise use ordinary words. Development vocabulary — slice, lineage,
   provenance, canonical, referent — does not reach an Agent unless it is the only
   exact word.
6. **Bounds live in the schema.** A number that a constraint enforces (`0–100`,
   `1–4,000`, page sizes) leaves the prose. Prose keeps only what the schema cannot
   say: uniqueness, "at least one item", "starts with a letter", English.
7. **No examples in descriptions.** Descriptions travel with every `tools/list`.
   The contract may carry one short example where it disambiguates.
8. **Stable structure.** Same headings, same order, same template every time.
   Predictable structure helps weaker readers and prompt caching alike. A
   description is blocks of plain text (label line, text line, blank line), never
   Markdown headings: hosts show descriptions as plain text.
9. **English everywhere.** Agents translate for their Users (`AGENTS.md`).

## Size guidance

Guidance for review, not a gate. Recorded before and after each change so drift is
visible:

| Text | Guidance | 2026-08-18 before | 2026-08-18 after |
| --- | --- | --- | --- |
| Play contract | ≈ 1,400–1,600 words | 2,833 words, 19.5k chars, 16 sections | 2,387 words, 15.9k chars, 15 sections |
| One tool description | ≤ ~100 words | 46–258 words, 2,245 total, 15.3k chars | 42–224 words, 1,756 total, 11.6k chars (11.8k after the block layout of the same day) |
| One schema description | ≤ ~15 words | up to 60 words; 24.2k chars over 323 fields | at most 13 words; 10.0k chars over 323 fields |

Later measurements preserve this baseline:

| Date and reason | Play contract (L2) | Tool descriptions (L1) |
| --- | --- | --- |
| 2026-08-20, spatial S1 publication | 2,493 words, 16,683 chars, 15 sections | 42–195 words, 2,047 total, 13,869 chars across 19 tools |

The first pass under this method landed above the contract guidance: semantic
parity with the previous texts (about 150 distinct rules) bounds how far the
contract can shrink without dropping a rule. The larger gains were in the schema
descriptions (−59 %) and descriptions (−24 %); the contract lost 16 % of its words
and all of its repetition.

When clarity needs more words, take them and record the result; do not compress
meaning to hit a number.

## Change procedure

Any change to public-facing text, however small:

1. **Inventory.** List every rule the change touches with its current source and
   its one destination layer. For a rewrite, inventory every sentence.
2. **Place.** Write each rule once at its destination. Mark duplicates, moved
   implementation facts and bounds that return to the schema.
3. **Parity.** Compare old and new against the inventory: zero rules lost, added or
   weakened. Semantic change is a contract change and needs its own accepted plan.
4. **Publish once.** Regenerate `game/mcp/tool-catalog.json` through the ignored
   test named in `game/docs/protocol.md#mcp-publication-invariants`; review the
   diff; only `description` strings may differ from a text-only change.
5. **Pin the boundaries.** The tests in `game/src/agent_contract.rs` pin short
   meaning anchors, one per non-negotiable boundary, never whole sentences.
6. **Probe when wording moves materially.** Run the comprehension probe below and
   record the result beside the previous one.
7. **Record.** Sizes in the table above; the choice in the concept log; the
   layering contract in `game/docs/agent.md` when a layer boundary moved.

## Comprehension probe

A bounded check that models understand the texts, kept in `dev/lab/agent-text/`.
It is evidence within its stated scope, never proof of universal compliance.

- A fixed set of yes/no questions with expected answers, each tied to an inventory
  rule (for example: "May you call `submit_action` before the User confirms the
  whole package?").
- One prompt per run: the served contract, the published catalog and the questions;
  the model answers in a fixed short format.
- Two model classes reachable from this machine through the Codex CLI already used
  by the runners; old and new texts; at most eight model calls per probe.
- Announced before it runs; model identity, token use, prompts, raw answers and the
  score are recorded in the experiment; the verdict is `supported`, `refuted` or
  `inconclusive` for these questions and models only.
- No World, server or database is involved; the probe never runs in the background.

## Checklist for a new tool or section

- [ ] The tool has one description in the fixed template with only the labels that
      apply and only the four restated boundaries.
- [ ] Every cross-tool rule the tool introduces is written once in the contract
      section that owns that topic; nothing new is repeated in the description.
- [ ] Every bound is a schema constraint; schema descriptions are one short clause.
- [ ] Development words and examples are absent from L0 and L1.
- [ ] The description restates only what its capability contract owns; the
      contract follows the capability shape in [build-text](build-text.md#capability-contract),
      and `game/docs/agent.md` agrees with the play contract where it restates it.
- [ ] Inventory rows, pin anchors, fixture regeneration, size table and probe are
      done per the change procedure.
