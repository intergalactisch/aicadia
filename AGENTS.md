# Aicadia

Aicadia is a persistent, shared fictional world kept by a dumb-but-strict chronicle
server; all intelligence comes from players' own AI agents connecting over MCP and
spending their owners' tokens. Keep this file compact — always-on build rules only.
The product concept lives in `docs/concept/` (a concept log: exploration, not
specification, still being discovered).

## Terry

`Terry` is the user's short handle for the full Aicadia build standard — the same
handle the user uses in the Scout project. When the user says "Terry", apply
everything under Build Heuristics below. Treat it as the default for all substantial
work even when the name isn't repeated.

## Build Heuristics

### KISS — Keep It Simple, Stupid

Build the smallest system that satisfies the confirmed behavior now. When two
designs both work, choose the one with fewer concepts, tables, states, branches and
moving parts. Add complexity only after a concrete current scenario shows why the
smaller design fails; predicted future scale alone is not enough.

### Singular Names, Always

Every technical name is singular: tables, models, files, folders, endpoints, MCP
tools, jobs, events. `scene`, `entity`, `claim`, `place_edge`, `rule` — never
`scenes`, never `entities`. No exceptions.

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

### Literal Technical Vocabulary

Schema, API, status and job names use conventional operational English that states
exactly what the system stores or does. Never use metaphorical world or presentation
words such as `pencil`, `ink`, `stone` or `whisper` as technical identifiers. Define
the transition for every stored status. Product wording, if any, is a later
presentation-layer decision.

### Boring Infrastructure

Postgres. One core API; MCP as a thin adapter on top; a web app reading the same
API. No microservices, no graph database, no queues-of-queues. Choose the
conventional option until the world itself proves a need.

### The Scene Log Is The Truth

Append-only. An accepted scene is one immutable source package: prose, the submitting
agent's structured claims and provenance. The server validates and places that
package without interpreting the prose again. Everything current (entity, claim,
map, dossier, catch-up) is a projection that can be rebuilt by replaying the log.
Never mutate history.

### No Universal Truth Status

Package acceptance, claim provenance and current-state selection are separate.
There is no universal claim evidence status. Every accepted claim is immediately
queryable; a current projection may use it only when that projection's deterministic
contract permits it. Each contract defines its key, authority, effective time and
replacement behavior, and every projected row retains its source claim id. Later
citations, observations, contradictions and replacements append new provenance; they
never promote or mutate the original claim.

### Realtime, Per Event

Process every scene and gesture on arrival: accept the agent-authored claims → update
projections → route ripples. Idempotent per event. No global batch moments and no
scheduled world ceremonies — the world moves before your eyes, and shipped
institutions are forbidden by concept principle 8.

### Statements, Not Modules

Fixed structural types only: `scene`, `entity`, `claim`, `place`, `character`,
`rule` (plus `account`, which lives outside the world). Every emergent kind is an
ordinary entity defined and related by claims, never a fixed schema value. Never add
a domain table (`job`, `house`, `economy`) — extend the claim vocabulary instead.
`subtype-of(A, B)` strictly means every A is also a B; multiple direct parent kinds
are allowed. Classification queries may traverse direct `subtype-of` claims but
never append the resulting indirect paths as claims; derived results retain depth
and every source claim id. Newly accepted classifications are immediately queryable.
Every returned edge and calculated path retains its source claim ids and provenance;
traversal never upgrades the source claims or turns a calculated path into source
truth.

### Everything Must Be Expressible

If something true in the world can't be stored as entity + claim, the model is
wrong: extend the core, never reject the fiction. Test sentence: "she lives in a
timber house with a reed roof and works as a ferryman" must fit without a schema
change.

### Rules Are Data

The leefregels live in the `rule` table, versioned, append-only. Every mechanical
validator declares the rule slug it enforces; every rejection cites its rule. Kind
definitions are descriptive and queryable and never become validators implicitly.

### Dumb And Strict Server

No LLM calls server-side. Validation is deterministic. Intelligence lives in the
connected agents; the server is the chronicle-keeper.

### No Unconscious Token Burn

Nothing server-side may ever trigger a user's agent or spend on a user's behalf.
A turn is a session the human starts.

### One Public Commit

Everything before canon is a private, reversible workshop between a person and their
agent. Every canon scene requires one explicit human confirmation of the complete
public source package. An agent may propose, draft and revise without confirmation;
it may never silently spend the scene by publishing.

### English Everywhere

Code, schema, API and canon are English. Agents translate for their own users; the
server never localizes.

### No Score Anywhere

No counters, ranks, levels, points, or currencies in schema or API. If a feature
seems to need one, the feature is wrong
(see `docs/concept/05-influence-and-retention.md`, anti-patterns).

### Earn Your Spot

Every file, table, endpoint and abstraction justifies itself now — no "might need
later" code, no dead paths, no placeholder services. Remove what isn't current.

### Documents Earn Their Place

Running concept development lands in `docs/concept/log/log.md` — one running log,
one line per development, grouped by date: what was added, chosen, discussed, where
we stand. A separate document is created or amended only when it pins something down
with standing value. Never scatter a session's thinking across many document edits —
log first, pin sparingly.

### Research Leaves A Trail

Research is never left only in a conversation. Save it under `docs/research/` with
its question, findings, sources and implications for Aicadia. Research informs a
choice but does not make one: confirmed concept direction still lands in the concept
log and, when it has standing value, the relevant concept document.

### Five-Year Backcast (`5jaar`)

When the user says `5jaar`, inhabit Aicadia after five real years at its intended
scale: describe ordinary use, emergent culture, abuse and failure modes, and the
technical and operational pressures that actually survived. Then walk backward to
the present and identify the smallest decisions and experiments needed now. Keep
future observation, recommendation and user decision distinct; never use the future
story to smuggle in an unchosen concept or speculative infrastructure.

## Reference Docs

- Concept log index: `docs/concept/README.md`
- Server shape (identity, briefing, ontology, storage): `docs/concept/08-server-shape.md`
- World rules (leefregels): `docs/concept/01-world-rules.md`
- Influence, retention and anti-patterns: `docs/concept/05-influence-and-retention.md`
