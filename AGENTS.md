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
Heuristics below to substantial work even when the handle is not repeated. Ask five
questions: what is the highest-value current game-development edge; does this work
advance it by fulfilling or deliberately evolving the current build contract; do
actor, action, state, ownership and nomenclature agree; is this the smallest safe
build needed now; and does the evidence prove exactly the claim being made? Terry
does not require speculative completeness or perfection. Development may remain
deliberately incomplete when its current boundary, evidence claim and next concrete
risk are explicit; `docs/game/` remains the current truth.

## Build Heuristics

### Game Progress First

Choose next work by the highest-leverage concrete advance toward Aicadia becoming a
compelling shared-world discovery and settlement game. Identify the valuable player
or world outcome first, then use KISS to select its smallest safe slice. Tiny
validation, cleanup, plumbing or documentation work does not win merely because it
is bounded; it may be next only when it unlocks a meaningful game capability or
retires a concrete blocker or risk on the selected edge. A working slice is not a
reason to keep polishing it: when its contract no longer contains the highest-value
advance, decide the next concrete game behavior, update `docs/game/`, then build it.

### KISS — Keep It Simple, Stupid

Build the smallest system that satisfies the confirmed behavior now. When two
designs both work, choose the one with fewer concepts, tables, states, branches and
moving parts. Add complexity only after a concrete current scenario shows why the
smaller design fails; predicted future scale alone is not enough.

### One Subject, One Identity

One durable World subject has one stable identity. A concrete role of an Entity uses
`entity_id` as its foreign-key identity and never receives a surrogate role id until
accepted behavior requires an independent lifecycle. Name every foreign key for its
meaning, such as `owner_user_id`, and add an index only for a current lookup,
ordering or uniqueness rule.

### The MVP Is The Filter

The current executable MVP contains exactly one `World`, durable `User` records,
shared `Entity` records and at most one owned `Character` Entity role per User. Its
complete use-case surface is `get_world`, `create_user`, `get_user`, `get_character`,
`create_character`, `list_entity`, `get_entity` and `create_entity`. Code may not
exceed this surface until a next player or World behavior is explicitly accepted in
`docs/game/`.
Next-work selection is not limited to this surface: once its value is sufficiently
proved, choose the next missing game outcome, decide its concrete contract, then
implement it. The seven current player-facing capabilities omit `create_user` and
ship through both HTTP and MCP. Authentication, OAuth and every other game capability
remain deferred until deliberately selected.

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
data it represents. The current implementation vocabulary is `world`, `user`,
`character` and `entity`. Existing exploration history may retain old wording.
Product copy may be layered on later and never shapes the core model.

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

`claim`, `world_event`, `rule`, event sourcing and every related status
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

### Every Choice Leaves A Trail

Every Aicadia choice that changes or constrains product direction, domain model,
behavior, architecture, implementation, evidence or operation is incomplete until
recorded when it crystallizes. Record accepted, rejected, deferred, corrected and
superseded choices with their material reason, current status and affected scope in
`docs/concept/log/log.md`; omit shell-command trivia. During an unfinished grill or
design session, maintain one active concept record that separates confirmed
direction from open decisions instead of waiting for implementation or final
agreement. In the same change, update, correct or remove every affected authority so
known-stale documentation is never left behind: current behavior and implementation
in `docs/game/`, sourced research plus its index in `docs/research/`, canonical
vocabulary in `CONTEXT.md`, and development history in the concept log. Write the
full choice once in its authority and link to it elsewhere. `docs/concept/` cannot
override `docs/game/` or this file. Amend this file only for an explicitly accepted,
compact rule that should govern work across tasks.

### Every World Action Leaves History

Every accepted state-changing game action defines and stores a durable, queryable
historical footprint in the same transaction as its current-state change. It must
remain possible to establish who acted, what was accepted, when and where it
happened, and which Characters, Places and other Entities were involved. Record
stable ids and explicit roles rather than inferring history later from prose. Reads,
rejected requests, transport traffic and private Agent reasoning are not World
history. This requirement does not authorize event sourcing or a universal payload;
current state and each concrete domain result keep their own simplest model.

### The Backlog Is Forward State

`.agents/backlog/README.md` orders the current development edge and later concrete
game outcomes. It is planning state, never an implementation contract and never a
replacement for `docs/game/` or the concept log. Keep at most one item active. Update
an item's current scope, status, dependencies and completion evidence when material
insight or implementation changes it; record the underlying product or architecture
choice once in its proper authority and link to it. Do not turn every idea into an
item, retain stale plans as if still current, or use scores and estimates as a proxy
for game value.

### Research Leaves A Trail

A research unit is complete only when its question, findings, sources and
implications for Aicadia are recorded. Research informs a choice but does not make
one; acceptance still requires updating the relevant current authority.

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
