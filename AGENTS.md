# Aicadia

> **Role / side:** always-loaded build constitution / development side.
> **Authority:** governs compact cross-task build rules.
> **Excludes:** volatile game scope and delivery status; see `docs/game/` and `docs/evidence/`.

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

### Plan Before Build

Before a non-trivial build changes code, schema, executable behavior, authoritative
docs or operations, create one proportional plan at
`.agents/plans/<YYYYMMDD-HHMMSS>-<slug>/plan.md`. Align the strategic player or
World outcome, tactical smallest complete slice, technical seams and exact evidence;
keep material questions in `draft` and grill until resolved. Present the plan and
wait for explicit User acceptance before implementation. Keep accepted plans as
current dependency-ordered execution state, detailed enough for bounded Agent tasks;
if the outcome, public contract, irreversible state, material cost or evidence claim
changes, stop, revise and regain acceptance. A micro-change may skip the artifact and
acceptance only when its outcome is unambiguous; the edit is local, reversible and
merely restores or preserves accepted behavior; it introduces no product, domain or
architecture choice; it touches no schema, migration, public contract,
ownership/history semantics, auth, security, privacy, irreversible or external
operation, material cost or token spend; and one focused check proves it. Line count
alone never qualifies. State its surface and check before editing; if discovery
breaks any condition, stop and plan. A small planned build may use one task.
Read-only explanation, status, orientation and diagnosis are not builds.

### KISS — Keep It Simple, Stupid

Build the smallest system that satisfies the confirmed behavior now. When two
designs both work, choose the one with fewer concepts, tables, states, branches and
moving parts. Add complexity only after a concrete current scenario shows why the
smaller design fails; predicted future scale alone never justifies prebuilt
infrastructure, although no design may preclude it (see Built For Massive
Concurrency).

### Built For Massive Concurrency

Design and implement every component as if millions of Characters act concurrently
in one shared World and any World instance may serve any request. Scope every lock,
transaction, revision, query and admission rule to the smallest subject it
protects—User, Character, Place, Entity or attempt—and keep every read bounded.
Never introduce process-local correctness state, server affinity, a global lock,
revision or counter, or a hot shared row every player must touch. This does not
license prebuilding scale infrastructure: keep KISS, but reject any design whose
correctness or contention would break at scale even though it works locally.

### Current Means Current

Implement only the current accepted product contract and current open standards. Do
not add or retain legacy modes, compatibility paths, deprecated fallbacks, dormant
feature flags, `allow(dead_code)` exceptions or dead code for hypothetical consumers.
Remove superseded implementation, tests, fixtures, configuration and current
documentation together. Preserve historical records as history. Support client
diversity through semantic capabilities and standards, never through provider,
model, tool or client allowlists. An exception requires one concrete current consumer
and explicit User acceptance.

### One Subject, One Identity

One durable World subject has one stable identity. A concrete role of an Entity uses
`entity_id` as its foreign-key identity and never receives a surrogate role id until
accepted behavior requires an independent lifecycle. Name every foreign key for its
meaning, such as `owner_user_id`, and add an index only for a current lookup,
ordering or uniqueness rule.

### The MVP Is The Filter

The complete current MVP contract—including domain shape, capability surface,
provisioning and operator boundaries, and explicit deferrals—lives only in
`docs/game/`. Code may not exceed that contract until a next player or World behavior
is explicitly accepted there. Next-work selection is not limited to the current
surface: once its value is sufficiently proved, choose the next missing game
outcome, decide its concrete contract, update `docs/game/`, then implement it.

### One Home Per Truth

`docs/README.md` is the binding placement constitution. Put each current fact in
exactly one owning authority and link to it from every other surface; never maintain
the same truth in parallel homes. Follow its side, evidence, reference, sweep,
role-header and bounded-size rules.

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
`character`, `entity`, `place` and `activity`. Existing exploration history may
retain old wording. Product copy may be layered on later and never shapes the core
model.

### Player Conversation Stays In The World

A conforming player conversation renders internal game and server structure as
grounded facts about named people, locations, things, events and current
affordances. It never exposes internal type names, fields, absent-value syntax,
identifiers, protocol work or delivery progress, even in response to a mechanics
question. Implementation inspection belongs in a separate development context.
Aicadia MCP is the sole live-game authority for an Agent; if it is unavailable,
player interaction fails closed before mutation instead of falling back to source,
HTTP, a database, tools, logs or remembered state. This presentation rule does not
rename or weaken the precise internal vocabulary and deterministic World contract.

### Boring Infrastructure

Postgres. One `World` interface owns game behaviour; HTTP and MCP are thin adapters
over that same interface. One local browser page may expose a read-only World,
Entity and Activity/prose ledger over existing reads. Browser gameplay, general
web-app scope, authentication and OAuth remain deferred. No microservices, no graph
database, no queues-of-queues. Choose the conventional option until the world itself
proves a need.

### Agent Capability Parity

Every player-facing capability ships in the same change through the `World`
interface, HTTP API and MCP, with one semantic input, output and error contract, a
complete Agent-facing tool description and a capability-parity test. The published
catalog must be complete, actions are validated deterministically by `World`, and a
capability is never UI-only. Provisioning, administration and operational controls
are never Agent tools.

### Direct MCP Smoke Tests

For a bounded question about whether an Agent understands one MCP capability, start
with the smallest direct end-to-end smoke: one pinned Agent call when sufficient,
one owned disposable World, the exact MCP operation under test, independent
authoritative readback and verified cleanup. Do not build a permanent runner,
multi-phase harness or broad fake matrix unless repeated evidence or one concrete
risk requires it. Prompt, output schema and validator must demand the same observable
result; a controller mismatch is inconclusive and may never be blamed on the model.

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
the current period file of `docs/concept/log/` (per its README); omit
shell-command trivia. During an unfinished grill or
design session, maintain one active concept record that separates confirmed
direction from open decisions instead of waiting for implementation or final
agreement. In the same change, update, correct or remove every affected authority so
known-stale documentation is never left behind: current behavior and implementation
in `docs/game/`, sourced research plus its index in `docs/research/`, canonical
vocabulary in `CONTEXT.md`, delivery and evidence history plus evidence-machinery
operation contracts in `docs/evidence/`, development history in the concept log,
and placement roles and boundaries in `docs/README.md`. Write the full choice once
in its authority and link to it elsewhere. `docs/concept/` cannot override
`docs/game/` or this file. Amend this file only for an explicitly accepted, compact
rule that should govern work across tasks.

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

- Documentation constitution: `docs/README.md`
- Current build contract: `docs/game/`
- Delivery and evidence history: `docs/evidence/`
- Exploration history: `docs/concept/`
- Research: `docs/research/`
