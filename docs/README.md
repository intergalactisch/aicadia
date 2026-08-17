# Documentation constitution

> **Role / side:** binding placement constitution for repository truth / development side.
> **Authority:** decides where each kind of current truth lives and how other homes refer to it.
> **Excludes:** product, implementation, research, planning and evidence content; those belong to the owning homes below.

This constitution places content; it does not restate that content. Its two sides are
roles, not directories. `tools/` and `tests/` contain files from both sides, so the
home table—not the directory tree—decides placement. `World` remains reserved for
the game domain and is never a name for either documentation side.

## Runtime side — the running product

The runtime side is what builds, runs, exposes, stores or directly verifies the
product. It includes `Cargo.toml`, `Cargo.lock`, `src/` except the local
development projection in `src/studio/` (and including `src/agent_contract/`),
`migration/`, runtime application tests and fixtures under `tests/`, `docs/game/`,
`tools/aicadia-local` and `tools/aicadia-agent`.

Runtime implementation fulfills the current contract in `docs/game/`; it does not
derive meaning from planning, exploration or delivery history.

## Development side — development of the product

The development side governs how Aicadia is understood, researched, chosen, planned
and built. It includes `docs/README.md`, `AGENTS.md`, `CLAUDE.md`, `CONTEXT.md`,
`docs/concept/` with its log and archive, `docs/research/` with its archive,
`.agents/backlog/`, `.agents/plans/`, `.agents/skills/`, `lab/`,
`src/studio/`, `web/`, their development-surface tests, `tools/*-playtest` and
`tests/*-playtest.sh`.

Development material may explain or cite the running product, but it never becomes a
second source for the current game contract or executable behavior.

## Evidence bridge

`docs/evidence/` bridges the sides. It owns development and delivery history about
the product, including operation contracts for the machinery that produces that
evidence. It never owns game-contract rules. Runtime-side documents may point to it
only through the static delivery-status pointer defined below.

## Home table

“Authority” names the owning repository source, not a person or team.

| Home | Side | Role | Owner / authority | Contains | Never contains | Update trigger |
| --- | --- | --- | --- | --- | --- | --- |
| `docs/README.md` | Development | Placement constitution | This file | Repository truth placement, reference direction, sweep and size rules | The truths being placed | A home, boundary or cross-home rule changes |
| `AGENTS.md` | Development | Always-loaded build constitution | `AGENTS.md` | Compact cross-task build rules | Volatile game scope, delivery status or duplicated authority content | An explicitly accepted rule must govern work across tasks |
| `CLAUDE.md` | Development | Thin Agent entry pointer | `CLAUDE.md` | Pointers to governing repository context | Independent build rules or product facts | The governing entry points change |
| `CONTEXT.md` | Development | Canonical vocabulary | `CONTEXT.md` | Current project and domain terminology | Full behavior contracts, decision history or delivery status | Canonical terminology changes |
| `docs/concept/` | Development | Live product exploration | Each live concept record | Rationale, open decisions and still-live ideas | Current implementation contract or delivery bookkeeping | Exploration changes or a concept choice crystallizes |
| `docs/concept/log/` | Development | Append-only decision register | Period log files and their index | Accepted, rejected, deferred, corrected and superseded choices | Current contract prose or repeated delivery narratives | A material choice changes or constrains direction |
| `docs/concept/archive/` | Development | Frozen concept history | Each archived record | Superseded concept generations with archive banners | Current authority or silently revived decisions | A concept record is superseded and archived |
| `docs/research/` | Development | Live sourced research | Each report and `docs/research/README.md` | Questions, sources, findings, standing and implications | Product decisions or implementation contracts | Research begins, changes standing or completes |
| `docs/research/archive/` | Development | Frozen research history | Each archived report | Superseded or historical research with banners | Current research standing or product authority | A report is superseded and archived |
| `.agents/backlog/` | Development | Ordered forward planning state | `.agents/backlog/README.md` and the current item | At most one active edge, later concrete outcomes and dependencies | Current product contract or duplicated decision rationale | Edge order, state, dependency or completion evidence changes |
| `.agents/plans/` | Development | Proportional build execution state | Each plan | Accepted outcome, task graph, invariants and exact evidence claim | Current product truth or reusable build rules | A consequential build is planned or its accepted execution state changes |
| `.agents/skills/` | Development | Reusable Agent workflow | Each skill's `SKILL.md` | Skill-specific procedure and routing | Project contract or duplicated global build rules | A reusable workflow changes |
| `lab/` | Development | Retained experimental workbench | `lab/README.md` and each track index | Small decision-oriented experiments, rough reproducible artifacts, bounded observations and verdicts | Current product truth, production dependencies, sourced research authority, secrets or proof beyond the experiment's stated scope | An experiment, verdict, artifact status or track boundary changes |
| `tools/agent-playtest`, `tools/trait-playtest` | Development | Evidence-producing runners | The runner and its evidence operation contract in `docs/evidence/runner/` | Executable playtest orchestration | Game-contract rules or delivery-status narrative | Runner behavior or its operation contract changes |
| `tests/agent-playtest.sh`, `tests/trait-playtest.sh` | Development | Runner regression suites | The matching test script | Token-free checks of evidence machinery | Runtime game tests or delivery narratives | Evidence machinery behavior changes |
| `tests/studio/` | Development | Studio and documentation-lint tests | The named test crate | Executable checks of the repository projection, its pages and the documentation lint below | Runtime game tests or delivery narratives | The Studio projection, its pages or the record conventions change |
| `docs/evidence/` | Bridge | Delivery and evidence history | Its index, per-slice records and `runner/` operation contracts | Status, runs, audits, digests, proof links and evidence-machine operations | Game-contract rules, planning state or concept rationale | Evidence is produced, reviewed, corrected or superseded |
| `docs/game/` | Runtime | Current product contract | `docs/game/README.md` and its concern/capability documents | Accepted domain, behavior, storage, protocol, Agent and deferral contracts | Delivery status, exploration history or build planning | Accepted current product behavior or implementation changes |
| `docs/game/model/<model>/` | Runtime | Model contract for one durable subject, role, seam or state | The folder's `README.md` | That model's contract text, its front matter and its further concern files | Delivery status, copied storage definitions or another model's contract | The model's accepted contract changes |
| `Cargo.toml`, `Cargo.lock` | Runtime | Rust build and dependency manifest | The manifest and lockfile | Current package, target and dependency resolution | Product requirements, rationale or delivery history | Build topology or a current dependency changes |
| `src/` except `src/agent_contract/` and `src/studio/` | Runtime | Executable game-server implementation | Rust source and migrations' consumers | Current deterministic game, protocol and server behavior | Planning, research or delivery narratives | Accepted executable behavior or implementation changes |
| `src/agent_contract/` | Runtime | Published Agent-facing text source | Its instruction and per-tool source files | Bytes published to connected Agents | Workshop history, delivery status or internal planning | The accepted Agent-facing contract changes |
| `src/studio/` | Development | Local read-only development projection | Its Rust modules plus the local-operation boundary in `docs/game/local-play.md` | Discovery of the governed roots in this table by convention, exact runtime-catalog projection, bounded operator-only World/public-schema reads and the Rust-rendered Studio pages | Game behavior, Agent capabilities, a source allowlist, authored copies of projected truth or delivery narrative | A governed root, record convention or supported Studio projection changes |
| `migration/` | Runtime | PostgreSQL schema evolution | Ordered migration files | Current durable schema transitions | Application behavior, planning or evidence narrative | The accepted storage contract changes |
| `tests/world*`, `tests/server*` | Runtime | Application integration tests | The named test crates | Executable proof of World and adapter behavior | Build-process policy or delivery history | Runtime behavior or its evidence obligation changes |
| `tests/aicadia-local.sh`, `tests/agent-tool-catalog.json` | Runtime | Local-operation and published-catalog fixtures | The named fixture or script | Exact launcher and Agent catalog assertions | Product rationale or evidence-run narratives | Runtime operation or published catalog bytes change |
| `tools/aicadia-local`, `tools/aicadia-agent` | Runtime | Local product operation | The named tool plus its contract in `docs/game/` | Launch and connection behavior | Playtest orchestration, planning or delivery status | Accepted local operation changes |
| `web/` | Development | Aicadia Studio browser presentation | Browser assets plus the local-operation boundary in `docs/game/local-play.md` | One hand-written stylesheet and one small enhancement script over the Rust-rendered Studio pages | Canonical rules, models, tool catalogs, schema logic, page routing or client state machines, browser gameplay or delivery history | Supported Studio presentation changes |

## Reference direction

Development-side homes may cite runtime-side authorities. A runtime-side home never
depends on the development side for its meaning. The only permitted pointer from a
runtime-side document away from runtime authority is a static delivery-status
pointer into `docs/evidence/`; it carries no status itself.

References point toward the owning authority. A summary must not become a second
authority: shorten it to a pointer whenever a change to the owned fact would
otherwise require editing both homes.

## Sweep scope

Frozen history—concept-log entries, `complete`, `superseded` or `dropped` plans and archived
documents—keeps its original links as citations and is excluded from link, anchor,
duplication, status and old-token sweeps.

Draft and active plans remain inside link-and-anchor checks. Duplication, status and
old-token scans exclude all of `.agents/plans/**`, including move-map fragments,
because plans cite the states and paths they change. Move-map fragments are checked
separately: every old source has exactly one destination and every destination
exists.

## Delivery-status pointers

Outside `docs/evidence/`, delivery status is represented only by a static pointer of
the form “Delivery history and current status: see `docs/evidence/<slice>.md`.” The
pointer never repeats a run result, current status, digest, candidate id or audit
finding. Planning-state labels such as `Queued`, `Active` and `Done` belong to the
backlog and are not delivery status.

## Authority-file headers

Every authority file starts with a two- or three-line role header that states:

1. its role and side;
2. what it is authoritative for; and
3. what does not belong there and which home owns that content.

Indexes and frozen records may place the header immediately below their title or
archive banner. A header identifies ownership; it does not summarize the owned
content.

## Record metadata

A record's volatile fields live in that record's own front matter, never in an index.
Front matter is optional unless the home below requires it. When present it is:

- line 1 exactly `---`;
- one or more lines `key: value`;
- a closing line exactly `---`;
- `key` matches `^[a-z][a-z0-9_]*$`;
- `value` is either a scalar (unquoted text to end of line, or a double-quoted
  string without escapes) or an inline list `[item, item, …]` of unquoted scalars
  (items trimmed; the empty list `[]` is allowed);
- no nesting, no block lists, no comments, no duplicate keys and no blank lines
  inside; a `null` scalar is the literal `null`.

Deviation from that grammar, a missing required key or an unknown key for a home
with a vocabulary is a lint error naming the file and line. A home without a
vocabulary tolerates any front matter.

| Home | Key | Required | Values |
| --- | --- | --- | --- |
| `docs/game/model/<model>/README.md` | `storage_table` | yes | list of `public` table names; may be `[]` for a seam |
| `docs/game/model/<model>/README.md` | `kind` | no | one of `subject`, `role`, `seam`, `history`, `state`, `provenance`, `participant` |
| `docs/concept/<record>.md` | `status` | yes | one of `active`, `live`, `retained`, `throwaway` |
| `docs/research/<report>.md` | `status` | yes | one of `pending`, `load-bearing`, `historical`, `superseded` |
| `docs/research/<report>.md` | `era` | yes | free scalar |
| `docs/evidence/<slice>.md` | `status` | yes | free one-line scalar |
| `lab/<track>/README.md` | `status` | yes | one of `active`, `kept`, `superseded`, `discarded` |
| `lab/<track>/<experiment>/README.md` | `question` | yes | free scalar |
| `lab/<track>/<experiment>/README.md` | `verdict` | yes | one of `supported`, `refuted`, `inconclusive`, `pending` |
| `lab/<track>/<experiment>/README.md` | `status` | yes | one of `active`, `kept`, `superseded`, `discarded` |
| `lab/<track>/<experiment>/README.md` | `real_seam`, `simulated_seam` | yes | list; may be `[]` |
| `lab/<track>/<experiment>/README.md` | `informs` | yes | repository path from the repository root, optionally with `#anchor`; lint-checked like a link |
| `.agents/plans/<dir>/plan.md` | `status` | yes | one of `draft`, `active`, `complete`, `superseded`, `dropped` |
| `.agents/plans/<dir>/plan.md` | `created_at`, `updated_at` | yes | quoted RFC 3339 timestamp |
| `.agents/plans/<dir>/plan.md` | `accepted_at`, `completed_at` | yes | quoted RFC 3339 timestamp or `null` |
| `.agents/plans/<dir>/plan.md` | `backlog_item` | no | repository path from the repository root; lint-checked like a link |
| `.agents/plans/<dir>/plan.md` | `dropped_at`, `dropped_reason` | no | quoted RFC 3339 timestamp and free scalar on frozen dropped plans |

Index READMEs list their records as Markdown links with stable navigation text—title,
question or theme—and nothing else. They carry no status, standing, verdict, era or
state column; those fields belong to the record. The single exception is the backlog
horizon table in `.agents/backlog/README.md`, which remains the one home of backlog
order and item state, so backlog item files carry no status line.

## Documentation lint

The sweeps described in “Sweep scope”, plus home mapping, role headers, front
matter, model-to-migration table completeness and index completeness, execute as
Rust tests over the Studio projection: `cargo test` fails and names the offending
path, and Studio shows the same findings as warnings. Within the sweep scope the
lint checks that

- every Markdown file under a governed root matches exactly one home;
- every non-frozen authority file has its role header;
- every record's front matter parses and validates against its home vocabulary;
- every relative link and `#anchor` in a non-frozen record resolves to an existing
  file and heading, where heading anchors follow GitHub's algorithm (lower-case,
  punctuation removed, spaces to `-`, duplicates suffixed `-1`, `-2` …) so links
  work on GitHub and in Studio alike;
- every `CREATE TABLE` in `migration/` except `_sqlx_migrations` is claimed by
  exactly one model folder, and every claimed table exists; and
- every record in a home is linked from that home's index.

## Bounded size

Current-authority documents should remain at or below roughly 400 lines. Source
modules should remain at or below roughly 1,500 lines. Split by one clear concern
when a file crosses its bound; do not compress meaning or invent abstractions merely
to hit a count.

The named exemptions are designated long-form records (active concept records,
research reports, per-month log files, plans and archives) and digest-frozen runner
scripts. An exemption permits necessary length, not mixed roles or duplicated
truth.
