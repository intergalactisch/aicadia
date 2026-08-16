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
product. It includes `Cargo.toml`, `Cargo.lock`, `src/` (including
`src/agent_contract/`), `migration/`, application test crates and fixtures under
`tests/`, `docs/game/`, `tools/aicadia-local`, `tools/aicadia-agent` and `web/`.

Runtime implementation fulfills the current contract in `docs/game/`; it does not
derive meaning from planning, exploration or delivery history.

## Development side — development of the product

The development side governs how Aicadia is understood, researched, chosen, planned
and built. It includes `docs/README.md`, `AGENTS.md`, `CLAUDE.md`, `CONTEXT.md`,
`docs/concept/` with its log and archive, `docs/research/` with its archive,
`.agents/backlog/`, `.agents/plans/`, `.agents/skills/`, `lab/`,
`tools/*-playtest` and `tests/*-playtest.sh`.

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
| `docs/evidence/` | Bridge | Delivery and evidence history | Its index, per-slice records and `runner/` operation contracts | Status, runs, audits, digests, proof links and evidence-machine operations | Game-contract rules, planning state or concept rationale | Evidence is produced, reviewed, corrected or superseded |
| `docs/game/` | Runtime | Current product contract | `docs/game/README.md` and its concern/capability documents | Accepted domain, behavior, storage, protocol, Agent and deferral contracts | Delivery status, exploration history or build planning | Accepted current product behavior or implementation changes |
| `Cargo.toml`, `Cargo.lock` | Runtime | Rust build and dependency manifest | The manifest and lockfile | Current package, target and dependency resolution | Product requirements, rationale or delivery history | Build topology or a current dependency changes |
| `src/` except `src/agent_contract/` | Runtime | Executable server implementation | Rust source and migrations' consumers | Current deterministic game, protocol and server behavior | Planning, research or delivery narratives | Accepted executable behavior or implementation changes |
| `src/agent_contract/` | Runtime | Published Agent-facing text source | Its instruction and per-tool source files | Bytes published to connected Agents | Workshop history, delivery status or internal planning | The accepted Agent-facing contract changes |
| `migration/` | Runtime | PostgreSQL schema evolution | Ordered migration files | Current durable schema transitions | Application behavior, planning or evidence narrative | The accepted storage contract changes |
| `tests/world*`, `tests/server*` | Runtime | Application integration tests | The named test crates | Executable proof of World and adapter behavior | Build-process policy or delivery history | Runtime behavior or its evidence obligation changes |
| `tests/aicadia-local.sh`, `tests/agent-tool-catalog.json` | Runtime | Local-operation and published-catalog fixtures | The named fixture or script | Exact launcher and Agent catalog assertions | Product rationale or evidence-run narratives | Runtime operation or published catalog bytes change |
| `tools/aicadia-local`, `tools/aicadia-agent` | Runtime | Local product operation | The named tool plus its contract in `docs/game/` | Launch and connection behavior | Playtest orchestration, planning or delivery status | Accepted local operation changes |
| `web/` | Runtime | Read-only local ledger | `web/index.html` plus its contract in `docs/game/` | The accepted local ledger implementation | Browser gameplay, planning or evidence history | Accepted ledger behavior changes |

## Reference direction

Development-side homes may cite runtime-side authorities. A runtime-side home never
depends on the development side for its meaning. The only permitted pointer from a
runtime-side document away from runtime authority is a static delivery-status
pointer into `docs/evidence/`; it carries no status itself.

References point toward the owning authority. A summary must not become a second
authority: shorten it to a pointer whenever a change to the owned fact would
otherwise require editing both homes.

## Sweep scope

Frozen history—concept-log entries, `COMPLETED` or `SUPERSEDED` plans and archived
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

## Bounded size

Current-authority documents should remain at or below roughly 400 lines. Source
modules should remain at or below roughly 1,500 lines. Split by one clear concern
when a file crosses its bound; do not compress meaning or invent abstractions merely
to hit a count.

The named exemptions are designated long-form records (active concept records,
research reports, per-month log files, plans and archives), digest-frozen runner
scripts and the single-page ledger. An exemption permits necessary length, not mixed
roles or duplicated truth.
