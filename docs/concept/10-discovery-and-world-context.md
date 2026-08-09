# Discovery and World context

> **Design record; grill paused; not an implementation contract.** This document
> records the direction confirmed during the August 2026 grill. It supersedes conflicting
> discovery exploration elsewhere in `docs/concept/`, but it does not expand the
> executable MVP. Only `docs/game/` can do that. Open decisions are named explicitly
> so later work does not silently complete the design.

## Intended game outcome

A User experiences one persistent shared World through their own Agent and
Character. The Agent can investigate the Character's surroundings and help the
World acquire new Places, materials, flora, fauna and future kinds of concrete
state. The Agent supplies intelligence and the World remains the authoritative,
deterministic rules processor. The User has no mechanical discovery power.

Discovery is not limited to geography and is not guaranteed on every investigation.
One accepted investigation may result in zero, one or several opportunities for new
shared World state.

## Confirmed responsibility split

| Actor | Confirmed responsibility | Must not do |
|---|---|---|
| User | Start and participate in the Agent interaction through which the game is experienced | Select the mechanical investigation focus, influence the roll, or declare, authenticate or confirm a discovery |
| Agent | Read current state, intelligently select an investigation from that state, request it and formulate smart candidate content after a positive roll result | Choose the Character, Place, roll, seed, odds or number of discovery opportunities |
| Character | Supply the durable personal game context for the investigation; it may have no established Place yet | Reset its discovery context by changing Agent, connection or session |
| World | Derive authoritative context, validate calls, resolve the chance table first, keep delivery retries stable and commit allowed shared state | Run an LLM, infer semantic content or trigger an Agent in the background |

The Agent is not a durable game actor. A different Agent or a restarted conversation
for the same Character does not create a fresh identity or private World.

## Confirmed interaction direction

The direction established so far is:

1. At the start of its work, the Agent reads a compact World overview and the
   Character's separate current context. These reads may happen in parallel.
2. The Agent reasons over that state while interacting with the User and selects an
   investigation. Ordinary conversation may advise the Agent, but never binds its
   choice. No User-supplied field selects the mechanical target or focus, changes
   probability or guarantees that an investigation occurs.
3. The Agent asks the World to begin the selected investigation. It does not submit
   a Character id or Place id; the World derives the Character and, when one has
   been established, its current Place from trusted User context.
4. The World validates the request and resolves a server-authoritative chance table
   before the Agent authors any discovery. A delivery retry of the same attempt
   returns the same roll result rather than consuming another roll.
5. The result is zero or a positive, volatile roll result bound to the
   current World-state. It exposes generic limits, applicable current meta-state and
   changes, but no storage helper or semantic discovery direction. With zero, the
   investigation ends without new World state.
6. The response gives a rich qualitative overview plus typed stable references. The
   Agent may use published read capabilities to inspect referenced World state more
   deeply before deciding what to create.
7. The Agent formulates one structured candidate World change within the roll
   result's limits and submits it to the World.
8. The World revalidates the roll token against current state, validates the
   candidate's deterministic domain rules and commits accepted concrete state to the
   one shared World.

Steps 3 through 8 express the authority boundary, not a settled API. The exact
representation, validity and completion rules for a positive roll result remain
open.

For example, “I wonder whether anything left tracks beside the river” may inspire
the Agent to investigate tracks, investigate something else or do nothing. It may
not map directly to a server focus, create an eligible attempt, improve a chance or
force a tracks-related result.

## Volatile roll result

The earlier helper-shaped `World-change opportunity` direction is corrected. A roll
must not return `introduce_entity`, `add_relation` or another persistence operation
as if that were the gameplay result. Such helpers leak storage structure into the
Agent interface and make the World prescribe what kind of discovery occurred.

A positive roll instead returns a volatile roll result: a server-authoritative
outcome, generic limits and rich meta-context captured from the current World-state.
It is not a session and is not stored as a durable Character opportunity. The Agent
uses it while relevant, performs any needed read-only inspection and submits a
structured candidate World change. The candidate must carry enough explicit
structure for deterministic validation; the World performs no semantic inference.

Only a completed attempt, its provenance and its accepted concrete domain results
become durable. There remains no generic Discovery table or Entity and no pending
Agent session. The exact interface that carries the roll result and candidate is
open, as is the rule for deciding which intervening World changes make a roll result
stale.

### Ownership direction

Ownership here means authoritative responsibility, not legal or player ownership.
The existing architecture already places the public game-behavior seam at `World`;
a separate public `Investigation` interface would create competing ownership.

| Concern | Owner |
|---|---|
| Starting and paying for an Agent interaction | User |
| Reasoning over context and authoring a candidate | Agent |
| Durable personal context | Character as the subject; World authorizes access from User context |
| Roll, meta-view, roll-result freshness and acceptance | World |
| Entity, Place, relationship and future result invariants | Their concrete domain behavior inside World |
| Atomic validation and commit across accepted results | World |
| Accepted persistent state | The shared World, never the User or Agent |
| HTTP and MCP encoding, errors and protocol state | Thin transport adapters |

The volatile roll result remains World-issued authority temporarily carried by the
Agent; the Agent does not own it and cannot modify its scope. A candidate remains
Agent-authored workshop material until World accepts it. Investigation may later
earn an internal module for locality and testing, but it does not earn a second
public seam or ownership of every concrete domain record merely because its
implementation becomes complex.

### High-velocity roll transport — accepted direction

The accepted base needs no Agent session, server affinity or durable pending-roll
record:

1. A roll reads a bounded dependency set and returns the rich meta-view plus an
   opaque, server-authenticated `roll_token`.
2. The token binds its attempt identity, derived Character, result and generic
   limits, contract version and opaque fingerprints of only the relevant state.
3. The Agent may make ordinary read-only World queries from any connection or server
   instance and later submit the token with its structured candidate.
4. Any World implementation instance can authenticate the token, load only its
   bounded dependencies and compare their current fingerprints.
5. A changed relevant dependency makes the token stale; unrelated World changes do
   not. A global World revision is never consulted.
6. If current, one Postgres transaction revalidates concrete domain invariants,
   records the attempt id uniquely and commits every accepted result. A retry returns
   that same committed result rather than writing twice.

The token and fingerprints are technical concurrency evidence, not gameplay
counters, scores or Agent-visible progress. Caches may accelerate reads but never
determine correctness. Process-local roll state remains rejected because it
requires server affinity and fails across restarts or horizontal scaling.

Several scope-bound roll tokens may coexist for one Character. They belong to their
attempts, never to conversations: conversation identity is neither World state nor
an authority source. Operational admission limits aggregate across the Character,
so opening more conversations never increases roll throughput.

Concurrency is optimistic and dependency-scoped. When an accepted candidate changes
state, every token whose bounded dependencies overlap that change becomes stale;
tokens based only on unrelated state remain usable. A Character-global
`current_roll_id` and its one-roll-at-a-time behavior are superseded because they
would let one conversation silently invalidate unrelated work in another. No
replacement marker is stored merely to serialize the Character.

### Meta-state handoff

The World returns the roll outcome and structural envelope together with trustworthy,
scope-bounded meta-state and changes in that state. It does not interpret those
facts into a semantic discovery direction. For example:

```text
result: positive
roll_token: <opaque state-bound value>
scope: current_character_context
limits:
  result_count: one
meta_state:
  current: <established state relevant to this scope>
  changes: <applicable established changes>
constraints:
  - reference established World state
  - preserve source provenance
```

The Agent analyses this information using World context, heuristics, coherence and
fun value, then decides what the discovery means. It might create flora, fauna, a
material, a route or a future subject within the generic limits and deterministic
domain rules. The World never emits `local_flora`, “the World needs an animal” or an
equivalent semantic recommendation and never invents a name, appearance, behavior or
story.

“Interesting”, “valuable” and “fun” are Agent judgments, not a free-form server
judgment or universal `fun_score`. The server returns both authoritative facts and
rich neutral signals derived deterministically from them. A signal may describe what
changed, its affected scope, explicit relationships, before and after state,
world-effective time, provenance and the mechanical rule that included it. It never
ranks signals, assigns value or recommends content.

The quality bar is deliberately high: the payload must preserve enough qualitative
World detail and structural context for an Agent to form an insightful judgment. A
bare code such as `relation_changed` is insufficient when the server can also return
the affected Entity ids, relationship, established descriptions, source references
and exact change. “Insightful” comes from connected evidence and change context, not
from server-authored interpretation.

### Agent drill-down

The immediate meta payload is a rich, coherent overview rather than the full
potentially immense context. Every stable reference it returns is typed and must be
resolvable through a published Agent-facing read capability. A reference without a
read path is invalid output.

The Agent may follow those references to inspect exact current state, relationships,
change detail and provenance before authoring a candidate. Those reads are World
inspection, not a new investigation: they consume no chance roll, grant no
opportunity and change no state. Capability availability is part of the discovery
feature itself; every required read ships through the `World` interface, HTTP and
MCP with one semantic contract and appears in the complete Agent capability catalog.

`Inspection` is a working distinction for these read-only drill-downs, not yet a
canonical operation name.

Inspection permits broad World queries rather than only opening references or
walking one relationship at a time. “Broad” describes what the Agent may research,
not a programmable query language. The interface remains flat and typed: it exposes
an allow-listed set of targets, filters, comparison operations, ordering and cursor
pagination. Identical state and identical input produce identical matches in a
stable order, and each result can state which explicit predicates it matched.

SQL, Cypher, GraphQL, recursive graph patterns, free-text query interpretation and
semantic or vector search are outside this direction. The Agent composes deeper
research by making further ordinary bounded calls.

Each query explicitly returns exactly one result kind, such as Entity, relationship,
change or provenance. Filters may refer to exposed fields and relationships, but a
response never mixes kinds or constructs joined result shapes. The Agent combines
typed results across calls. Every kind of established shared World state that an
Agent is allowed to know must have an explicit read and query path in the MCP tool
catalog; the same semantic capability exists through `World` and HTTP. Exact
operation names remain open.

“Fully query the World” means complete composability over Agent-visible state, not
one omnipotent query or one enormous response. All established shared World state,
including its history, relationships, provenance and neutral meta-signals, is
Agent-visible. Random seeds, hidden chance mechanics, authentication and rate-limit
state, uncommitted Agent candidates and other operational internals are not World
state and are not queryable through game capabilities.

## World and Character context

The two initial reads serve different scopes:

- The World overview is User-independent. It contains World identity, neutral time
  metadata and only explicitly World-wide current state. It is not a complete World
  snapshot, local feed or server-written AI summary.
- The Character context is player-specific. Its minimum direction is the Character
  and its spatial-presence state. When a current Place has been established, it also
  contains that most-specific Place and established state directly attached to the
  exact Place. It does not automatically expand through containment, proximity,
  visibility or technical relevance.

The World derives the Character and any established current Place from User request
context. Agent input cannot override either one. Absence of a Place is a valid state,
not an Agent-supplied value or lookup failure. The exact operation names and response
fields are not decided.

Agent queryability has two explicit scopes:

- established shared World state, composed through the typed per-result-kind query
  capabilities; and
- the current Character's complete personal state, through context-required read and
  query capabilities that derive the Character from the User request context.

Personal state is not forced into the shared World projection merely to make it
queryable. The Agent may analyse both scopes together, while the server continues to
authorize them separately. An Agent can query the complete personal state of its own
Character. It cannot query another Character's personal state; it sees that
Character only through established shared World state.

## Shared and personal scope

The Character is the durable personal discovery context, rather than the User
record, Agent, transport connection or conversation. A discovery roll result may
therefore differ between two Characters in the same Place.

Accepted results are nevertheless shared World state. There are no private World
copies or private discoveries. A result created through one Character can affect
what every later Character encounters.

The effects of multiple Characters investigating the same opportunity, alternate
Characters and already-existing shared results are still open.

The User cannot transfer or manufacture discovery authority through wording. All
mechanical authority remains divided between the server-owned World rules and the
intelligence of the connected Agent. “LLM” in this direction always means that
Agent; the World never hosts or invokes an LLM.

## Place and spatial direction

- The World is semantically unbounded and technically sparse. Only established
  state needs storage or simulation.
- A Place is an Entity role and shares that Entity's stable id. It is not a second
  identity for the same referent.
- Place identity, geometry, containment, travel, visibility and engine partitioning
  are separate concerns.
- A Place may have a known anchor while its boundary is incomplete or entirely
  unknown. The server does not own a hidden, already-complete geography.
- Two Characters may encounter disconnected parts of what later proves to be the
  same Place. The World does not merge them through omniscience; later evidence can
  append an identity relationship without rewriting the original encounters.
- Exact Place equality may define the narrow minimum scope for one Character-context
  read. It is not a universal rule for visibility, observation or co-presence.

Coordinates, axes, geometry types, boundary storage, parent Places, travel routes
and spatial cells remain deliberately undecided. They must be introduced by a
concrete behavior that needs them rather than by predicted scale alone.

## Discovery state and provenance

`Discovery` is the gameplay outcome perceived by the User, not a universal Entity,
table or JSON record. An investigation persists its normal action/provenance history
and whatever concrete World state actually resulted: for example an Entity, Place
role, relationship, boundary observation or classification.

There is likewise no universal `Observation` object merely to hold every possible
finding. Material, flora, fauna, Place geometry and later subjects retain their own
domain state while sharing a generic investigation flow and provenance boundary.

Accepted source evidence is append-only. Later identification, contradiction,
refinement or merging adds evidence and relationships; it does not mutate the
original uncertain encounter into something it did not state.

The exact generic name and storage shape for the investigation action and its source
history are still open.

## Investigation rolls

Every eligible new investigation uses an independent, context-dependent chance
table. Its roll can grant zero, one or several discovery opportunities.

The following are explicitly absent:

- no chance increase after elapsed time;
- no chance increase after consecutive empty investigations;
- no soft or hard pity;
- no entropy accumulator or shuffle bag;
- no transferable or hidden accumulated luck.

Prior outcomes therefore do not improve later odds. Fresh accepted attempts are
independent. A technical retry is not fresh and must reproduce the committed roll
result. Operational rate limiting protects the service from excessive calls but
does not alter the chance table or masquerade as a fairness mechanic.

An Agent may try another investigation without first having discovered something.
It sends one explicit investigation request with an idempotency key. Before any
gameplay roll, World derives the Character from trusted request context and applies
one operational admission policy across all of that Character's conversations. If
admitted, World creates the attempt identity and performs a fresh independent roll.
The same idempotency key is a delivery retry of that attempt; a rejected request
creates no attempt, performs no roll and changes no World state.

User conversation, free text and asserted effort never establish eligibility. The
operational limiter protects request throughput only: it does not alter odds,
accumulate luck or become Agent-visible gameplay state. Its concrete infrastructure
and thresholds are deferred until the investigation capability is selected for
implementation.

## Time and change

Time currently stores and links facts; it does not run the World:

- one neutral, continuous time axis follows real elapsed time;
- timestamps can order attempts and World changes and may later support historical
  reads;
- there is no `world_clock` object, calendar, tick service, scheduler, periodic
  mutation, season system or day/night system;
- waiting does not improve discovery odds or create another discovery;
- the server never starts an Agent or spends User tokens because time passed.

Future concrete behavior may earn a narrow temporal rule. A general simulation
layer is not part of this direction.

## Deliberately absent now

- a generic Discovery table or Entity;
- a universal Observation table;
- caller-supplied Character or Place identity;
- server-side LLM inference;
- background Agent work or durable Agent sessions;
- durable or stockpiled pending discovery opportunities;
- private per-Character Worlds;
- scores, levels, progress meters, pity counters or discovery currency;
- complete pre-generated geography;
- universal locality or visibility rules;
- periodic World mutation merely to keep the World changing;
- programmable database or graph query languages exposed to Agents.

## Open design frontier

The paused grill leaves five decisions, in dependency order:

1. how the chance table derives eligible outcomes and weights from Character and
   World context without an LLM or a finite pre-authored World;
2. how zero, one and several results compose and whether one accepted result
   may exclude another;
3. how duplicate, contradictory or already-known candidate content resolves against
   shared World state;
4. the exact concrete records, provenance and atomic transaction created when an
   Agent completes a positive roll result;
5. which smallest complete discovery behavior earns promotion into `docs/game/`
   and implementation through World, HTTP and MCP together.

These decisions are deliberately deferred rather than silently assumed. They do not
justify continuing the grill before building an already-confirmed prerequisite.
Each is reopened only when the next smallest end-to-end game behavior actually
depends on it; no investigation code enters the executable MVP before its required
behavior is accepted in `docs/game/`.

## Prototype under evaluation

The self-contained [discovery roll lab](discovery-roll-prototype.html) tests one
question: does optimistic, scope-bound parallelism remain understandable and fair
when one Character investigates through multiple conversations while shared World
state changes?

The in-memory lab exposes every relevant state after each action and includes four
guided cases: unrelated scopes both commit, overlapping scopes conflict, a delivery
retry remains one attempt, and Character-wide operational admission rejects every
conversation before a roll. Its result-count selector deliberately forces zero, one
or two results so each branch can be inspected; it is not proposed Agent input or a
chance-table decision. The fixed Character, scopes, candidates, fingerprints and
accepted packages are likewise test fixtures, not production nomenclature, schema
or content.

The prototype has no persistence, authentication, cryptography or production
interface. It changes neither `docs/game/` nor the executable MVP. A game decision
is earned only after hands-on evaluation produces a verdict.

## Research basis

- [Persistent-game spatial models](../research/persistent-game-spatial-model.md)
- [Locality, co-presence and observation](../research/locality-co-presence-and-observation.md)
- [World time and sparse simulation](../research/world-time-and-sparse-simulation.md)
- [Stochastic discovery and bad-luck protection](../research/stochastic-discovery-and-bad-luck-protection.md)
