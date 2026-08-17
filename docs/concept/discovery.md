---
status: live
---

# Discovery and investigation

> **Role / side:** live concept exploration of investigation and stochastic discovery rolls / development side.
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
shared World state; the first accepted slice allows exactly one.

The discovery rule accepted on 2026-08-15 gives the roll its purpose: what already
existed in the World without anyone making it—plants, tracks, ore, springs, ruin
fragments and later new Places—enters the World only through an admitted
investigation and roll. What a Character makes, brings or places remains ordinary
confirmed Action introduction. World cannot enforce the distinction; the Agent
contract states it and deterministic Agent evidence exercises it.

## Confirmed responsibility split

| Actor | Confirmed responsibility | Must not do |
|---|---|---|
| User | Start and participate in the Agent interaction through which the game is experienced; confirm the complete previewed find before it is submitted, exactly as for every other World-changing package | Select the mechanical investigation focus, influence the roll, or declare or authenticate that something was found |
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
5. The result is zero or positive. The accepted revision uses a retry-stable
   response containing only the stored outcome, opaque attempt authority and
   immutable generic limits, with no mutable World-state snapshot or semantic
   discovery direction. With zero, the investigation ends without new World state.
6. After a positive the Agent uses the existing bounded exact-Place, Entity-state
   and Activity reads to re-ground current context before deciding what to create.
7. The Agent formulates one structured candidate World change within the roll
   result's limits, previews it completely in the User's language and, only after
   explicit User confirmation, submits it to the World. Starting an investigation
   needs no confirmation: it is admission plus a World roll, not a player-visible
   World change.
8. The World verifies the attempt authority against current state, validates the
   candidate's deterministic domain rules and commits accepted concrete state to the
   one shared World.

Steps 3 through 8 express the authority boundary. The [current game
contract](../game/README.md) owns their concrete executable representation; this
record retains why that boundary was selected.

For example, “I wonder whether anything left tracks beside the river” may inspire
the Agent to investigate tracks, investigate something else or do nothing. It may
not map directly to a server focus, create an eligible attempt, improve a chance or
force a tracks-related result.

## Stable roll result

The earlier helper-shaped `World-change opportunity` direction is corrected. A roll
must not return `introduce_entity`, `add_relation` or another persistence operation
as if that were the gameplay result. Such helpers leak storage structure into the
Agent interface and make the World prescribe what kind of discovery occurred.

A positive roll instead returns a stable roll result: a server-authoritative
outcome, opaque attempt authority and generic limits. T1 review reopened and
rejected the earlier mutable meta-context payload as incoherent with byte-identical
retries, the deliberately minimal attempt row and bounded current-state reads. The
accepted revision returns no snapshot; the Agent performs current read-only inspection
and submits a structured candidate World change. The candidate must carry enough
explicit structure for deterministic validation; World performs no semantic
inference.

Since 2026-08-15 the attempt itself is durable technical provenance—one
`investigation_attempt` row holding retry identity, outcome and its consuming
Activity—so retries, admission and one-time consumption are exact across restarts and
instances. That row is not a pending Agent session, is never listed to play and
becomes World history only through the accepted find's Activity. There remains no
generic Discovery table or Entity. In the first slice a positive attempt becomes
unusable only when it is consumed, voided by the hoarding rule or the Character is no
longer at its Place; unrelated World changes never stale it.

### Ownership direction

Ownership here means authoritative responsibility, not legal or player ownership.
The existing architecture already places the public game-behavior seam at `World`;
a separate public `Investigation` interface would create competing ownership.

| Concern | Owner |
|---|---|
| Starting and paying for an Agent interaction | User |
| Reasoning over context and authoring a candidate | Agent |
| Durable personal context | Character as the subject; World authorizes access from User context |
| Roll, retry-stable result authority and acceptance | World |
| Entity, Place, relationship and future result invariants | Their concrete domain behavior inside World |
| Atomic validation and commit across accepted results | World |
| Accepted persistent state | The shared World, never the User or Agent |
| HTTP and MCP encoding, errors and protocol state | Thin transport adapters |

The stable roll result remains World-issued authority temporarily carried by the
Agent; the Agent does not own it and cannot modify its scope. A candidate remains
Agent-authored workshop material until World accepts it. Investigation may later
earn an internal module for locality and testing, but it does not earn a second
public seam or ownership of every concrete domain record merely because its
implementation becomes complex.

For the selected first complete loop, the User has now made that locality constraint
explicit: discovery must not be appended to a monolithic World or server file. The
accepted build direction is one private deep investigation Module behind the existing
`World` Interface, with distinct ownership for typed chance resolution, attempt/
retry state, stable result authority and atomic concrete commit. Existing Entity,
Property and Activity behavior keeps ownership of its own invariants; investigation
composes those rules instead of copying them or calling an ordinary public mutation
as a shortcut. HTTP and MCP remain separate thin Adapters. A private Interface is
introduced only when two real Adapters exist—for example production randomness and
a deterministic test resolver—and never solely to make the file tree look layered.
The exact file map remains owned by the accepted plan.

### High-velocity roll transport — superseded on 2026-08-15

The signed, client-carried `roll_token` below was the accepted transport until the
resumed grill resolved Q5. It is superseded by one durable `investigation_attempt`
row whose unguessable id, bound to the User context, is the opaque commit authority.
Material reason: retry identity, Character-wide admission and one-time consumption
already require durable per-attempt state, so a signed token would be a second truth
that adds secret provisioning and rotation without gain. Restart and multi-instance
safety follow from PostgreSQL alone; the token direction's optimistic, scope-bound
parallelism is preserved. The original direction is kept here as history:

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

### Post-roll grounding — reopened on 2026-08-15

Independent T1 review proved that the earlier positive payload could not satisfy its
own retry and scale invariants. Place state is paginated and may grow without a
lifetime cap; recent Activity and current state change after issue; the attempt row
stores neither a snapshot nor an as-of revision. Recomputing later would break
byte-identical retry semantics, while persisting the response would duplicate World
truth and create unearned snapshot machinery.

The User reaccepted this stable response on 2026-08-15:

```text
outcome: positive
attempt_id: <opaque attempt authority>
limit:
  result_count: 1
  kind: entity_at_current_place
```

After `positive`, the Agent re-reads the current exact Place, selected Entity state
and recent exact-Place Activity through existing paginated capabilities. The attempt
remains bound only to its original Place identity; unrelated same-Place changes do
not stale it. The richer broad-inspection payload remains later exploration, not
part of this slice.

The Agent analyses the current read results using World context, heuristics, coherence and
fun value, then decides what the discovery means. It might establish flora, fauna, a
material, tracks or a ruin fragment as one ordinary Entity within the generic limits
and deterministic domain rules. The World never emits `local_flora`, “the World needs an animal” or an
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

The first slice uses Activity operation `submit_discovery` and one private
`investigation_attempt` row with the consuming Activity link as provenance. Broader
future discovery kinds still keep their own concrete domain state rather than
earning a generic Discovery relation.

## Investigation rolls

Every eligible new investigation uses an independent, context-dependent chance
table. Its roll can grant zero, one or several discovery opportunities; the first
slice grants zero or one.

The first accepted context is recent discovery density at the exact Place: the
number `n` of accepted finds among the last `W` Activities at that Place. Chance is
one owned component with typed parameters and a pure probability
`p = p_min + (p_max − p_min) · 2^(−n / h)` (accepted defaults `p_max = 1/2`,
`p_min = 1/10`, `h = 6`, `W = 48`). Because the signal is a bounded window over the
Place's own history, a well-searched Place recovers only as ordinary play continues;
no clock, counter column or World-wide aggregate is consulted, and the read stays
bounded with millions of concurrent Characters. Own misses and elapsed Character
time are deliberately not signals.

A zero writes no Activity and never moves the exact-Place pointer; only the internal
attempt row exists for retry identity and admission. The Agent renders it in-world as
one honest unsuccessful search.

The following are explicitly absent:

- no chance increase after elapsed time;
- no chance increase after consecutive empty investigations;
- no soft or hard pity;
- no entropy accumulator or shuffle bag;
- no transferable or hidden accumulated luck.

Prior outcomes therefore do not improve later odds. Fresh accepted attempts are
independent. A technical retry is not fresh and must reproduce the stored outcome,
attempt id and immutable limits. Operational rate limiting protects the service from excessive calls but
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
accumulate luck or become Agent-visible gameplay state. Its accepted first form is
two bounded per-User rules over the durable attempt rows, applied under the existing
User lock: at most `A` admitted attempts per rolling hour (default 12) and at most
`P` unconsumed positive attempts (default 3). The accepted revision defines one database
timestamp for the inclusive rolling-hour boundary and stored attempt time. Only a
newly inserted positive beyond the cap voids the oldest prior live positive with
`id <> new_attempt_id`, ordered by `(created_at, id)`; the schema rejects self-void
provenance and a zero never voids one. A lost attempt id therefore cannot
deadlock a Character and no listable stock of pending finds exists. Thresholds are
hidden from play and never inputs to odds.


## Deliberately absent now

- a generic Discovery table or Entity;
- a universal Observation table;
- caller-supplied Character or Place identity;
- server-side LLM inference;
- background Agent work or durable Agent sessions;
- durable or stockpiled pending discovery opportunities (a bounded number of
  unconsumed positive attempts is technical provenance, not a listable stock);
- signed roll tokens, HMAC keys, secret rotation or launcher key configuration;
- private per-Character Worlds;
- scores, levels, progress meters, pity counters or discovery currency;
- complete pre-generated geography;
- universal locality or visibility rules;
- universal player access to shared World records, global Entity catalogs or
  absolute World aggregates;
- automatic exposure of which beings are controlled by Users;
- periodic World mutation merely to keep the World changing;
- programmable database or graph query languages exposed to Agents.


## Selected complete loop

On 2026-08-14 the User selected one complete investigation-and-discovery loop as the
next planning edge. This supersedes treating the first roll and first discovery
commit as independently shippable backlog outcomes: a roll that changes no future
play is plumbing, while a mutation without World-first uncertainty is not
discovery. The User then reaffirmed that chance resolution is essential to an
understandable and powerful discovery mechanism: the Agent asks to investigate; after
admitting the exact situation, World performs the roll as an internal authoritative
step. There is no separate caller-controlled roll button and no input for a seed,
odds, result count or retry count.

Discovery is not `create_entity` or `submit_action.introduce_entity` under another
name: it uniquely requires World admission and chance resolution before Agent
authorship plus attempt/result provenance, and the discovery rule above states which
things must be found rather than made. The concurrency choice preserves several
independent conversations without making conversation identity part of World or
allowing conversation count to increase throughput.

Delivery history and current status: see [discovery
evidence](../evidence/discovery.md).

## Resolved design frontier after T1 correction

The grill resolved the main domain choices, then T1 review caught one incoherent
combination: mutable Place context could not be both bounded and byte-identical on
retry without adding snapshot storage. The accepted correction kept the attempt
minimal and moved current context back to the existing scoped reads. It also kept
operational admission separate from chance, made the start free of User
confirmation, retained confirmation for the authored find, and rejected a generic
Discovery object or signed authority token.

Those material reasons and the correction trail live in the [concept
log](log/2026-08.md). The [game contract](../game/README.md) owns all current fields,
limits, errors, ordering and adapter behavior; the [completed build
plan](../../.agents/plans/20260814-204007-first-investigation-discovery-loop/plan.md)
retains the dependency-ordered execution boundary.

## Prototype under evaluation

### Lab framing — superseded on 2026-08-15

The lab's scope-bound parallelism, its forced zero/one/two result selector and its
"a decision is earned only after a verdict" framing are superseded by the delivered
contract: an investigation binds no revision or scope, a positive attempt permits
exactly one result, and the loop is decided and built rather than awaiting hands-on
evaluation. The [game contract](../game/README.md) owns the current rules. The
original evaluation text is kept here as history:

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
