# Build-facing text

> **Role / side:** working method for the documents a building Agent reads / development side.
> **Authority:** how a document earns its place, states what it owns and points to what it does not, so that a model editing the repository reads it correctly from the file alone.
> **Excludes:** sentence-level writing rules — defined in [public-text writing rules](public-text.md#writing-rules); which home owns which truth — defined in the [documentation constitution](../README.md#home-table); the reading order per kind of change — defined in [reading paths by change](../../../game/docs/README.md#reading-paths-by-change).

## What counts as build-facing text

Build-facing text is every Markdown document a building Agent — Codex, Claude
Code or any other model that edits this repository — reads before it changes code,
schema, contract or records. Today that is `AGENTS.md`, `CLAUDE.md`,
`dev/CONTEXT.md`, everything under `game/docs/` and `dev/docs/`, the backlog, the
plans, the lab and the skill files. It is never published to a playing Agent; the
texts that are, and how they are written, are the subject of
[public-facing text](public-text.md).

## Why it is written differently

The reader is a model at the moment of a change. It has one file open, a task in
mind and a limited context; it must decide from what it sees whether this is the
file to edit, whether another file owns the fact it needs, and whether it must open
that file to find out. Every time the text leaves that decision to a bare link, the
model either opens everything — and pays for it — or edits here and creates a
second home. Build-facing text is therefore written so that each document says, in
plain sentences, what it owns, what it does not and where that lives, and so that
every reference tells the reader what it will find before it follows the link.

The sentence-level rules are the same as for public text — one rule per line,
positive first, concrete over abstract, plain words, stable structure, English —
*defined in* [public-text writing rules](public-text.md#writing-rules); this record
adds only what a reader who edits the repository needs beyond them.

## Every file earns its place

A document earns its place when its own text answers three questions:

1. **Who reads it, and when?** One reader at one moment — a builder changing a
   capability, a builder placing a new record, a reviewer checking evidence.
2. **Which fact lives only here?** At least one. A file whose every sentence is
   also true somewhere else is a copy, not a home.
3. **What makes it change?** One kind of event, stated as the home table states an
   update trigger.

The role header (below) is where a file answers the first two; the constitution's
home table answers the third for its home. A file that cannot answer question 2
folds into its parent or its sibling; a section that cannot answer it is removed
in favour of a pointer.

## One owner per fact, tested by change coupling

The test is not "do these two passages look alike" but: *if this fact changed,
would two files have to change?* If yes, one of them is the owner and the other
becomes a pointer — or the second is a **declared derivation**: text generated
from the owner, or text kept equal under a parity procedure, and said to be so.
Today's declared derivations: each tool description restates its capability
contract for the playing Agent and is kept equal by the public-text parity
procedure, and `game/mcp/tool-catalog.json` is generated from its sources.
Nothing else may restate an owned fact.

## The pointer sentence

A reference is a sentence, never a bare link. It names the fact, the relation the
fact stands in to this file, the owner, and what this file adds or does not add:

`<fact> — <relation> in [<owned thing>](<path#anchor>); <what this file adds or does not add>`

For example: *Name and description follow the World-wide value rules (length,
trimming, U+0000) — constrained by [shared value validation](../../../game/docs/domain.md#shared-value-validation);
this capability adds nothing to them.* The reader now knows the fact exists, where
it lives, and that nothing about lengths is decided or to be edited here.

The relation is one of five words. Using the same five everywhere lets a reader
parse a reference without thinking about it:

| Relation | Meaning for the builder | Example |
| --- | --- | --- |
| **defined in** | that file is the owner; change the fact there and nowhere else | canonical error codes — defined in `protocol.md#canonical-errors` |
| **constrained by** | a shared rule of another file also holds here; this file adds only its local part | initial Property items — constrained by the Property model contract |
| **published as** | this file is the source; that surface is a derivation kept equal by generation or parity | the capability contract — published as its tool description |
| **narrowed here** | the general case lives elsewhere; this file adds a stricter local rule | Activity roles — narrowed here to one `subject` |
| **recorded in** | history or evidence; read for context, never for current behaviour | the delivery run — recorded in `dev/docs/evidence/…` |

The link text names the owned thing (a section, a model, a file), not "here" or
"this document". One pointer per fact per section; a fact used twice in one section
is linked once, at first use.

## The wayfinding header

Every authority file keeps the three labels the constitution requires and the lint
parses — `Role / side`, `Authority`, `Excludes` — and fills them so that a reader
who has just opened the file knows within three lines whether it is in the right
place and, if not, where to go:

- **Role / side** stays a short label: what kind of document, which side.
- **Authority** says in plain words what the file owns — the facts a reader will
  find nowhere else — not a category name.
- **Excludes** lists each topic a reader might expect here but will not find, each
  with its relation and a Markdown link to the owner: `<topic> — <relation> in
  [<owner>](<path#anchor>)`. Because it is a link, the existing lint checks that
  the owner exists; no new lint is needed.

The header of this record is an example. A header identifies ownership and routes
the reader; it never summarizes the owned content.

## Three kinds of link, three places

Link soup comes from "see also" lists, back-links and linking every term on every
mention. Build-facing text allows exactly three kinds of link, each in one place:

| Kind | Where | Form |
| --- | --- | --- |
| **Upward** — a fact this text depends on | in the body, at the point of use, once per section | the pointer sentence |
| **Downward** — an index to its records | in an index or `README.md` only, complete | a list of links with stable navigation text |
| **Sideways** — the sibling that owns an excluded topic | in the role header only | the *Excludes* line |

Nothing else: no "see also" section, no link back to the index, no link on a
domain term the reader is not asked to act on. Reading order per kind of change
lives once, in the game-contract index — *defined in*
[reading paths by change](../../../game/docs/README.md#reading-paths-by-change).

## Document shape

Each kind of document has a fixed shape: a short list of sections in a fixed order,
each section either **owned** (it carries facts found nowhere else) or **pointer
only** (one pointer sentence). A section that is not in the shape is one of two
things — a new fact that needs an owner, or a copy — and is treated as a smell
until it is placed. Same headings, same order, every time.

### Capability contract

The shape of `game/docs/capability/<tool>.md`.

Header: *Role / side* — one capability contract / runtime side. *Authority* — the
World-side contract of `<tool>`: input, local validation, result, Activity
footprint, annotation class and evidence obligation. *Excludes* — Agent wording —
published as `game/mcp/agent/tool/<tool>.md`; workshop and player conduct —
defined in the owning `game/docs/agent.md` section; canonical error codes and
transport mapping — defined in `game/docs/protocol.md#canonical-errors`.

| Section | Status | Carries |
| --- | --- | --- |
| Purpose | owned | one or two sentences: what World does when this capability is accepted |
| Input | owned | the input shape and the World, HTTP and MCP calls |
| Contract | optional, owned | the structured package rules for package-shaped capabilities |
| Input example(s) | optional, owned | one or two examples where the package needs them |
| Validation | owned | the local rules; shared rules appear only as *constrained by* pointers |
| Result | owned | accepted state, atomicity and concurrency guarantees, what absence means |
| Activity footprint | owned | which Activity, which roles; general Activity semantics as a pointer |
| Annotations and retry class | owned | additive / modifying / read-only, idempotency by what; the retry conduct itself is Agent text and is not restated |
| Evidence obligations | owned | what World, HTTP and MCP must each prove |

Absent by design: an *Errors* section (pure pointer — the header names the owner),
a *Workshop link* section (the header names the owning conduct section), and any
sentence about how an Agent speaks to a player (owned by the play contract and
`game/docs/agent.md`).

### Other kinds

Model contracts, concern documents (`domain.md`, `protocol.md`, `storage.md`,
`agent.md`, …), development records and indexes adopt the wayfinding header and the
pointer sentence **when they are next edited for another reason**; no sweep rewrites
them at once. When a second document kind gets a fixed shape, it is added here as a
subsection like the one above.

### Development Area workbook

The shape of `dev/areas/<area>/README.md`. The workbook owns current development
synthesis for one flat subject lens. It may link the same underlying source as
another Area, but never copies a fact owned by a runtime contract, research report,
Lab, evidence record, backlog item or plan.

| Section | Status | Carries |
| --- | --- | --- |
| Meaning | owned | a short explanation of the subject and why it matters to Aicadia |
| Boundary → This is / This is not | owned | positive and negative scope; neither list is a decision status |
| Decisions → Chosen / Rejected / Not yet chosen | owned | current directional disposition, with rejection distinct from absence or an unresolved choice |
| Research needed | owned | durable questions that need sourced facts or an experiment; Work separately selects what is active |
| Components | owned synthesis | the concepts and parts that make up the Area, with pointers where another source owns their exact meaning |
| Technical model → Delivered / Directional / Absent | owned synthesis and pointers | executable structure by pointer, current unbuilt direction, and deliberately absent or unselected structure without presenting one as another |
| Sources | pointer only | the owning contracts, rationale, research, Lab and evidence records a builder may need next |

Every heading is present exactly once and in this order. `This is`, `This is not`,
the three decision states and `Research needed` use explicit bullet items so Studio
can preserve their distinctions without interpreting prose. A section with no items
says `None.` rather than inventing a placeholder fact. The Area README never carries
the selected current question, plan status, task state or backlog order; those are
defined in Work sources.

## Change procedure

Any change to build-facing text:

1. **Find the owner.** Before writing a fact, find the file whose *Authority* line
   claims it. Write it there. If no file claims it, the fact needs a home in the
   constitution's home table first.
2. **Point, do not restate.** Where another file needs the fact, write one pointer
   sentence with one of the five relations at the point of use.
3. **Route from the header.** If a reader might expect the fact in a file that
   does not own it, add the topic to that file's *Excludes* line with its owner.
4. **Inventory a removal.** When sentences leave a file, list each with its owner
   path (or "kept — reworded") in the plan; nothing owned may be lost.
5. **Lint.** `cargo test -p aicadia-studio --test studio lint` checks headers,
   links, anchors, front matter and index completeness; it does not check shape,
   relation words or duplication — those are review.
6. **Adopt on touch.** Bring the touched file's header and pointers to this form in
   the same change when they are not there yet.

## Builder wayfinding probe

A bounded check that a model finds the right files from the texts, kept in
`dev/lab/agent-text/`. It is evidence within its stated scope, never proof.

- A fixed set of change tasks ("change the maximum name length"; "add an optional
  field to `create_character`"), each with the intended read path and the one file
  to edit, derived from the reading-path table.
- One prompt per task: the repository is available read-only; list the files you
  would open, in order, and the file you would edit; do not edit.
- One small model class at low effort through the Codex CLI the runners use; old
  and new texts through a temporary read-only worktree; at most twelve calls per
  probe.
- Announced before it runs; model identity, prompts, raw answers and the per-task
  match are recorded; the verdict is `supported`, `refuted` or `inconclusive` for
  these tasks and this model only. No World, server or database; never in the
  background.

## Checklist for a new capability contract

- [ ] The header's *Authority* names the World-side facts this file owns and the
      *Excludes* line links the tool description, the owning conduct section and
      the canonical errors.
- [ ] The body has exactly the sections of the capability shape, in order, with
      *Contract* and *Input example(s)* only for a package-shaped capability.
- [ ] Every shared rule appears as one *constrained by* pointer at its point of use;
      no length, format or error code is restated.
- [ ] No sentence tells an Agent how to speak, preview, confirm or retry.
- [ ] The tool description is written by the public-text method and says nothing
      the contract does not own; the two are kept equal by parity review.
- [ ] `game/docs/README.md` lists the capability in the catalog and its reading
      path still holds.
