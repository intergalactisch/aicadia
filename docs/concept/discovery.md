# Discovery and investigation

> **Role / side:** live concept exploration of investigation and volatile discovery rolls / development side.
> **Authority:** current discovery rationale, responsibility boundaries, roll transport, provenance, frontier decisions and prototype
> **Excludes:** Executable behavior, delivery evidence and the separate knowledge, interaction, spatial, time and tabletop themes.

This record preserves the August 2026 discovery direction. It cannot expand the executable MVP; only [the current game contract](../game/README.md) can do that.

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

The Agent may follow those references only when they are inside the current
Character's authorized knowledge scope. It can then inspect the exact known state,
relationships, change detail and provenance before authoring a candidate. Those
reads are World inspection, not a new investigation: they consume no chance roll,
grant no opportunity and change no state. Capability availability is part of the
discovery feature itself; every required read ships through the `World` interface,
HTTP and MCP with one semantic contract and appears in the complete Agent capability
catalog.

`Inspection` is a working distinction for these read-only drill-downs, not yet a
canonical operation name.

Inspection permits broad queries across what this Character can know rather than
only opening one reference or walking one relationship at a time. “Broad” describes
composition inside that epistemic boundary, never global World access. The interface
remains flat and typed: it exposes an allow-listed set of targets, filters,
comparison operations, ordering and cursor pagination. Identical authorized state
and identical input produce identical matches in a stable order, and each result can
state which explicit predicates it matched.

SQL, Cypher, GraphQL, recursive graph patterns, free-text query interpretation and
semantic or vector search are outside this direction. The Agent composes deeper
research by making further ordinary bounded calls.

Each query explicitly returns exactly one result kind, such as Entity, relationship,
change or provenance. Filters may refer to exposed fields and relationships, but a
response never mixes kinds or constructs joined result shapes. The Agent combines
typed results across calls. Every kind of Character-knowable World state must have
an explicit read and query path in the MCP tool catalog; the same semantic capability
exists through `World` and HTTP. Exact operation names remain open.

“Fully inspect” means complete composability over what this Character can currently
know, not one omnipotent query, one enormous response or access to every accepted
record. Shared persistence does not imply universal visibility. Random seeds,
hidden chance mechanics, authentication and rate-limit state, uncommitted Agent
candidates, User-control provenance and other operational internals remain outside
player game queries.


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
- universal player access to shared World records, global Entity catalogs or
  absolute World aggregates;
- automatic exposure of which beings are controlled by Users;
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
