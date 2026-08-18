# Shared conventions for T1 (projection) and T2 (documentation)

This fragment fixes the exact conventions that the Rust repository projection
parses and that the documentation restructure adopts. Both tasks follow it
literally; T2 also transcribes the durable parts into `docs/README.md` (record
metadata section, model folder row, lint section). Where this fragment and the plan
differ, the plan wins and this fragment is corrected.

## 1. Governed roots and homes

A *governed root* is a directory whose Markdown files Studio projects. Every `.md`
file under a governed root must match exactly one home below; an unmatched file is a
lint error. Paths are repository-relative. `frozen` homes are rendered but excluded
from link/anchor lint and metadata lint (sweep-scope rule in `docs/README.md`).

| Home id | Side | Match | Frozen | Role header | Front matter |
| --- | --- | --- | --- | --- | --- |
| `constitution` | development | `docs/README.md` | no | required | none |
| `build-constitution` | development | `AGENTS.md` | no | required | none |
| `entry-pointer` | development | `CLAUDE.md` | no | required | none |
| `vocabulary` | development | `dev/CONTEXT.md` | no | required | none |
| `game-index` | runtime | `game/docs/README.md` | no | required | none |
| `model` | runtime | `game/docs/model/<model>/README.md` | no | required | `model` vocabulary |
| `model-concern` | runtime | `game/docs/model/<model>/<other>.md` | no | required | none |
| `capability` | runtime | `game/docs/capability/<name>.md` | no | required | none |
| `game-contract` | runtime | `game/docs/<other>.md` | no | required | none |
| `concept-index` | development | `dev/docs/concept/README.md` | no | required | none |
| `concept-record` | development | `dev/docs/concept/<other>.md` | no | required | `concept` vocabulary |
| `concept-log-index` | development | `dev/docs/concept/log/README.md` | no | required | none |
| `decision-register` | development | `dev/docs/concept/log/<YYYY-MM>.md` | yes (links) | required | none |
| `concept-archive` | development | `dev/docs/concept/archive/**/*.md` | yes | not required | none |
| `research-index` | development | `dev/docs/research/README.md` | no | required | none |
| `research-report` | development | `dev/docs/research/<other>.md` | no | required | `research` vocabulary |
| `research-archive` | development | `dev/docs/research/archive/**/*.md` | yes | not required | none |
| `evidence-index` | bridge | `dev/docs/evidence/README.md` | no | required | none |
| `evidence-slice` | bridge | `dev/docs/evidence/<other>.md` | no | required | `evidence` vocabulary |
| `runner-contract` | bridge | `dev/docs/evidence/runner/<name>.md` | no | required | none |
| `backlog-index` | development | `dev/backlog/README.md` | no | required | none |
| `capability-map` | development | `dev/backlog/capability-map.md` | no | required | none |
| `backlog-item` | development | `dev/backlog/items/<name>.md` | no | required | none (state lives in the horizon table) |
| `plan` | development | `dev/plans/<dir>/plan.md` | yes when `status` is `complete`, `superseded` or `dropped` | required unless frozen | `plan` vocabulary |
| `plan-fragment` | development | `dev/plans/<dir>/**/<other>.md` | inherits its plan | not required | none |
| `skill` | development | `dev/skills/<name>/SKILL.md` | no | required | skill front matter (`name`, `description`) — not validated beyond presence |
| `skill-asset` | development | `dev/skills/<name>/**/<other>.md` | no | not required | not validated (templates contain placeholders) |
| `lab-index` | development | `lab/README.md` | no | required | none |
| `lab-track` | development | `lab/<track>/README.md` | no | required | `lab-track` vocabulary |
| `lab-experiment` | development | `lab/<track>/<experiment>/README.md` | no | required | `lab-experiment` vocabulary |
| `lab-record` | development | `lab/<track>/<other>.md` and `lab/<track>/<experiment>/<other>.md` | no | required | none |
| `agent-contract-index` | runtime | `game/mcp/agent/README.md` | no | required | none |
| `agent-instruction` | runtime | `game/mcp/agent/instruction/<nn>-<name>.md` | no | not required (published bytes) | none |
| `agent-tool-text` | runtime | `game/mcp/agent/tool/<name>.md` | no | not required (published bytes) | none |

Non-Markdown governed sources: `migration/<nnnn>_<name>.sql` (home `migration`,
listed and rendered as SQL); the compiled tool catalog, assembled instructions and
HTTP route table (home `compiled`, from Rust functions). Nothing else on disk is
read by Studio. Directories named `target` are never scanned.

Match precedence: the most specific pattern wins (`README.md` before `<other>.md`;
`log/`, `archive/`, `model/`, `capability/`, `runner/`, `items/` before their parent
`<other>` patterns). `<other>` never matches `README.md`.

## 2. Front matter grammar

Front matter is optional unless a home vocabulary requires it. When present it is:

- line 1 exactly `---`;
- one or more lines `key: value`;
- a closing line exactly `---`;
- `key` matches `^[a-z][a-z0-9_]*$`;
- `value` is either a scalar (unquoted text to end of line, or a double-quoted
  string without escapes) or an inline list `[item, item, …]` of unquoted scalars
  (items trimmed; empty list `[]` allowed);
- no nesting, no block lists, no comments, no duplicate keys, no blank lines
  inside; a `null` scalar is the literal `null`.

The parser is strict: any deviation is a lint error naming the file and line.
Unknown keys for a home with a vocabulary are lint errors; a home without a
vocabulary tolerates any front matter but Studio only displays it.

## 3. Per-home vocabularies

| Home | Key | Required | Values |
| --- | --- | --- | --- |
| `model` | `storage_table` | yes | list of `public` table names; may be `[]` for a seam |
| `model` | `kind` | no | one of `subject`, `role`, `seam`, `history`, `state`, `provenance`, `participant` |
| `concept` | `status` | yes | one of `active`, `live`, `retained`, `throwaway` |
| `research` | `status` | yes | one of `pending`, `load-bearing`, `historical`, `superseded` |
| `research` | `era` | yes | free scalar (current values: `July scene-claim`, `August Activity-Property-Trait`) |
| `evidence` | `status` | yes | free one-line scalar |
| `lab-track` | `status` | yes | one of `active`, `kept`, `superseded`, `discarded` |
| `lab-experiment` | `question` | yes | free scalar |
| `lab-experiment` | `verdict` | yes | one of `supported`, `refuted`, `inconclusive`, `pending` |
| `lab-experiment` | `status` | yes | one of `active`, `kept`, `superseded`, `discarded` |
| `lab-experiment` | `real_seam` | yes | list (may be `[]`) |
| `lab-experiment` | `simulated_seam` | yes | list (may be `[]`) |
| `lab-experiment` | `informs` | yes | repository path from the repository root (never relative to the record's own directory), optionally with `#anchor`; resolved and lint-checked like a link |
| `plan` | `status` | yes | one of `draft`, `active`, `complete`, `superseded`, `dropped` |
| `plan` | `created_at`, `updated_at` | yes | quoted RFC 3339 timestamp |
| `plan` | `accepted_at`, `completed_at` | yes | quoted RFC 3339 timestamp or `null` |
| `plan` | `backlog_item` | no | repository path from the repository root; resolved and lint-checked like a link |
| `plan` | `dropped_at` | no | quoted RFC 3339 timestamp (frozen dropped plans) |
| `plan` | `dropped_reason` | no | free scalar (frozen dropped plans) |

Concept status meanings (from today's index): `active` = active exploration,
`live` = live exploration, `retained` = retained rationale/heuristics,
`throwaway` = throwaway prototype record.

## 4. Role header

An authority file's role header is the first blockquote after the title (or after
the archive banner / front matter): two or three lines beginning
`> **Role / side:**`, `> **Authority:**`, `> **Excludes:**`. The projection parses
`role` and `side` from the first line (text before ` / ` and the trailing
`<side> side`), `authority` and `excludes` verbatim. Required per the table above.

## 5. Headings, anchors and links

- Heading ids follow GitHub's algorithm so every link works both on GitHub and in
  Studio: lower-case the heading text; remove every character that is not a letter,
  digit, space, hyphen or underscore (so `—`, backticks and punctuation vanish and
  runs of hyphens are kept, e.g. `T3 — Prove …` → `t3--prove-…`); replace each space
  with `-`; suffix duplicates `-1`, `-2` … in document order; a heading with no
  remaining character gets the id `section`. Only ATX headings (`#`…`######`) count.
  (This supersedes the earlier "collapse to one hyphen" rule.)
- A Markdown link is any `[text](target)`; targets starting with a scheme
  (`http:`, `https:`, `mailto:`) are external and ignored; `#anchor` targets refer
  to the same file; relative targets resolve against the file's directory and are
  normalized; targets are split at the first `#`.
- Link lint (non-frozen homes only): the target file must exist and, when an anchor
  is given, the target must contain that heading id. Autolinks and reference-style
  links are not used in this repository and are not parsed.
- Backlinks: for every non-frozen record, the set of non-frozen records whose links
  resolve to it (with the anchor when present).

## 6. Decision register (concept log)

A monthly file `dev/docs/concept/log/YYYY-MM.md` is parsed as:

- `# YYYY-MM-DD` — a date section (H1 after the title/header block);
- `## <topic>` — a topic under the current date; anchor `YYYY-MM-DD/<topic-id>`
  where `<topic-id>` is the heading id;
- entries are top-level list items under a topic (`- ` at column 0; continuation
  lines indented). Two shapes:
  - bullet-tag: `- <tag>[ / <qualifier>]: <text>` — `tag` is the lowercase phrase
    before the first `:`; a ` / ` splits tag and qualifier; `→` inside the tag is
    kept literally;
  - bold-status: consecutive items `- **Status:** …`, `- **Choice:** …`,
    `- **Reason:** …`, `- **Evidence:** …`, `- **Affected scope:** …`,
    `- **Schema tooling:** …` under one topic form one entry whose tag is the
    lower-cased first word of `Status` (`accepted`, `completed`, `selected`, …) and
    whose text is the concatenation;
  - `### <sub-heading>` under a topic starts a sub-topic; its items belong to that
    sub-topic (anchor `YYYY-MM-DD/<topic-id>/<sub-id>`).
- Every entry has: date, topic, optional sub-topic, tag, qualifier, text (Markdown),
  ordinal within topic, links found in its text.

Tag vocabulary is not validated against `log/README.md` (history varies); Studio
facets show tags as found.

## 7. Plans, task graphs and backlog

- Plan front matter per §3. The task graph is the first Markdown table whose header
  row is exactly `ID | State | Depends | Parallel-safe | Objective | Owned surfaces |
  Evidence`. Task states are `pending`, `in_progress`, `completed`, `blocked`.
- The plan's "Open questions" section is the H3 `### Open questions`; its content
  (until the next heading of level ≤ 3) is the open text.
- Backlog horizon: the first table in `dev/backlog/README.md` whose header row
  is `Horizon | Item | State | Concrete outcome`; `Item` may contain one link to an
  item file. Item states are `Proposed`, `Queued`, `Ready`, `Active`, `Blocked`,
  `Done`, `Dropped`; horizons are `Now`, `Next`, `Later`, `Done`.
- "Open …" aggregation: every heading (H2/H3) whose text starts with `Open` in a
  non-frozen `concept-record`, `plan` with status `draft` or `active`, or
  `backlog-item`; the section body until the next heading of the same or higher
  level.

## 8. Model folders

- `game/docs/model/<model>/README.md` — title `# <Model name>`, role header, front
  matter per §3, then the contract. Additional concern files in the same folder are
  `model-concern` pages of that model, ordered by filename after `README.md`.
- Model id = folder name (kebab-case). Display name = README title.
- Vocabulary join: the `dev/CONTEXT.md` term whose bold name, lower-cased and
  kebab-cased, equals the model id (`Investigation attempt` → `investigation-attempt`,
  `World` → `world`); absent term is allowed.
- Storage join: `storage_table` names must exist among `CREATE TABLE` names in
  `migration/*.sql` (after renames such as `app_user` → `user`, the final name);
  every migration table except `_sqlx_migrations` must be claimed by exactly one
  model. Lint enforces both.
- Capability join: capabilities that link to the model folder (backlinks) plus the
  compiled catalog name match is not attempted; backlinks only.

## 9. Index READMEs after T2

Index READMEs (`concept-index`, `research-index`, `evidence-index`, `lab-index`,
`lab-track`) keep their role header and prose, and list records as Markdown links
with stable navigation text (title, question or theme). They carry no status,
standing, verdict, era or state columns. The backlog index keeps its horizon table
(D5). Studio renders every index home from the records, not from the README list;
lint checks that every record in the home is linked from its index.

## 10. Rendering rules shared by pages

- Raw HTML in Markdown is escaped (existing behavior).
- Internal links are rewritten to Studio routes (`/doc/<path>#<anchor>` or the
  home's dedicated route when one exists).
- Every record page shows: breadcrumb (section › home › record), title, role
  header panel, front matter panel, outline, content, "referenced by" panel,
  copyable reference `[Aicadia Studio · <title>](<url>) — <path or id>`.
