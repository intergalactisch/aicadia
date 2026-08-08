# Aicadia

Aicadia is an MMO-like shared-world discovery and settlement game. Human users
connect their own AI agents over MCP to inspect and extend one persistent shared
world. The game server is dumb and strict; all inference spends the user's own
tokens. Keep this file compact — always-on build rules only. The current build
contract lives in `docs/game/`. `docs/concept/` is exploration history: it may inform
discussion but never governs current implementation.

## Terry

`Terry` is Aicadia's proportional game-and-build gate for concept decisions, domain
model, executable behaviour, code, tests, docs and operations. Apply the Build
Heuristics below to substantial work even when the handle is not repeated. Ask four
questions: does this belong in the current game and build contract; do actor, action,
state, ownership and nomenclature agree; is this the smallest safe build needed now;
and does the evidence prove exactly the claim being made? Terry does not require
speculative completeness or perfection. Development may remain deliberately
incomplete when its current boundary, evidence claim and next concrete risk are
explicit; `docs/game/` remains the current truth.

## Build Heuristics

### KISS — Keep It Simple, Stupid

Build the smallest system that satisfies the confirmed behavior now. When two
designs both work, choose the one with fewer concepts, tables, states, branches and
moving parts. Add complexity only after a concrete current scenario shows why the
smaller design fails; predicted future scale alone is not enough.

### The MVP Is The Filter

The current executable MVP contains exactly one `World`, durable `User` records and
shared `Entity` records. Its complete use-case surface is `get_world`, `create_user`,
`get_user`, `list_entity`, `get_entity` and `create_entity`. Every task must directly
decide, implement or verify that surface. The five player-facing capabilities omit
`create_user` and ship through both HTTP and MCP. Authentication, OAuth and every
other game capability remain deferred.

### Singular Domain Names

Aicadia-owned domain names are singular: tables, models, endpoints, MCP tools and
jobs use `user` and `entity`, never `users` or `entities`. Keep required ecosystem
conventions such as Rust's `tests/` directory instead of fighting the toolchain.

### Conventional Operation Names

Use standard CRUD verbs and the shortest unambiguous name. Bare `create`, `get`,
`list`, `update` and `delete` are allowed only inside a resource-specific interface;
otherwise qualify the resource, as in `create_entity`. Never add `update` or `delete`
before current behavior requires it.

### Flat Over Clever

The complexity budget belongs to the world and what gets written in it — never to
the tech. No ceremonies: no speculative abstractions, no patterns-for-patterns, no
frameworks on frameworks. If a fresh agent can't follow a code path in one sitting,
it's too clever.

### Concrete Before Abstract

The world may become abstract; development communication may not. Explain every
proposal with the concrete actor, input, action, stored result and boundary cases.
Do not use a slogan, metaphor or broad principle as if it were a decision. If an
abstract phrase is useful, immediately translate it into specific allowed and rejected
examples that a future builder can implement and test.

### Game And Server Vocabulary

Treat Aicadia as a game under development, never as a literary platform. Every term
in code, schema, API, architecture and current concept direction uses conventional
game-development and server English and states the actor, action, state or stored
data it represents. The current implementation vocabulary is `world`, `user` and
`entity`. Existing exploration history may retain old wording. Product copy may be
layered on later and never shapes the core model.

### Boring Infrastructure

Postgres. One `World` interface owns game behaviour; HTTP and MCP are thin adapters
over that same interface. Authentication, OAuth and a web app are deferred. No
microservices, no graph database, no queues-of-queues. Choose the conventional
option until the world itself proves a need.

### Agent Capability Parity

Every player-facing capability ships in the same change through the `World`
interface, HTTP API and MCP, with one semantic input, output and error contract, a
complete Agent-facing tool description and a capability-parity test. The published
catalog must be complete, actions are validated deterministically by `World`, and a
capability is never UI-only. Provisioning, administration and operational controls
are never Agent tools.

### Deferred Means Absent

`claim`, `world_event`, `character`, `rule`, event sourcing and every related status
or projection are undecided and outside the current MVP. Do not add their tables,
types, fields, interfaces or abstractions until `docs/game/` explicitly introduces
required current behavior.

### Dumb And Strict Server

No LLM calls server-side. Validation is deterministic. Intelligence lives in the
connected agents; the server is the authoritative world-state processor.

### No Unconscious Token Burn

Nothing server-side may ever trigger a user's agent or spend on a user's behalf.
Each explicit call stands alone. There is no durable domain session; any future
connection, authentication or protocol state stays outside the World.

### English Everywhere

Code, schema, API and stored world content are English. Agents translate for their
own users; the server never localizes.

### No Score Anywhere

No counters, ranks, levels, points, or currencies in schema or API. If a feature
seems to need one, the feature is wrong.

### Earn Your Spot

Every file, table, endpoint and abstraction justifies itself now — no "might need
later" code, no dead paths, no placeholder services. Remove what isn't current.

### Documents Earn Their Place

Current executable behavior and implementation decisions live in `docs/game/`.
Update those documents with every accepted behavior change. `docs/concept/` remains
exploration and history and cannot override `docs/game/` or this file. Amend this
file only for compact, always-on repository instructions.

### Research Leaves A Trail

Research is never left only in a conversation. Save it under `docs/research/` with
its question, findings, sources and implications for Aicadia. Research informs a
choice but does not make one: accepted current behavior and implementation decisions
land in `docs/game/`.

### Five-Year Backcast (`5jaar`)

When the user says `5jaar`, inhabit Aicadia after five real years at its intended
scale: describe ordinary use, emergent culture, abuse and failure modes, and the
technical and operational pressures that actually survived. Then walk backward to
the present and identify the smallest decisions and experiments needed now. Keep
future observation, recommendation and user decision distinct; never use the future
story to smuggle in an unchosen concept or speculative infrastructure.

## Reference Docs

- Current build contract: `docs/game/`
- Exploration history: `docs/concept/`
- Research: `docs/research/`
