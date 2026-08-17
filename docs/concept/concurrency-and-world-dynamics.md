---
status: active
---

# Mass concurrency and living World direction

> **Role / side:** active concept exploration of crowd concurrency, sparse World change and scale progression / development side.
> **Authority:** current rationale, candidate direction and open decisions for a large living Aicadia World.
> **Excludes:** accepted game behavior, implementation contracts, delivery claims and sourced technical findings; see `docs/game/`, `docs/evidence/` and `docs/research/`.

No candidate direction below is accepted game behavior.

## Question

How can Aicadia become a lively, rich and dynamic shared World for millions of
Users while remaining deterministic, locality-bounded and correct when a crowd
observes or acts on the same subject?

The phrase “support one million Users” is incomplete. It must eventually name at
least the connected-User count, read and mutation rates, burst window, geographic
distribution, hot-subject skew, latency objective, accepted rejection behavior,
storage horizon and recovery objective. This record therefore separates semantic
contention from total platform throughput.

## Confirmed current boundaries

These facts are already governed by `AGENTS.md` and `docs/game/`; this record does
not change them:

- World is deterministic and strict. It never invokes an Agent, spends User tokens
  or runs an LLM.
- One `World` interface owns behavior; HTTP and MCP are stateless adapters over the
  same contract; PostgreSQL is the authoritative store.
- Accepted state-changing game actions write their current state and immutable,
  attributable Activity in one transaction.
- Correctness state is database-backed. Process affinity, process-local locks and a
  global World revision, counter or lock are forbidden.
- Current contextual writers serialize through one Place row and advance one
  `place.latest_activity_id`. This is correct for the MVP but makes one busy Place
  one mutation lane.
- The current World has at most one entry Place and no movement, additional Places,
  clocks, background simulation, generic event/rule engine or server intelligence.
- Current request identity and fingerprints make confirmed mutation retries safe.
- Scores, ranks, levels, points and currencies are absent by design.

No current evidence makes a production-throughput, hosted availability or
million-User claim. Several current queries return bounded pages but are not yet
proven to perform bounded work for arbitrarily large Place occupancy or history.

## Confirmed User direction

On 2026-08-16 the User sharpened the product direction:

- a Place may ground shared context, locality and discovery, but should not become
  the universal lock that prevents independent Users from discovering, enriching or
  acting within it;
- concurrency design must invite exploration and World enrichment rather than make
  safe progress depend on an ever-stale whole-Place snapshot;
- the intended game must remain genuinely playable when millions of people use the
  same persistent World at the same time, including deliberate hot-Place and
  hot-Entity scenarios; and
- the next substantial plan should concentrate on multiplayer, concurrency and high
  World dynamism: many Characters may share one Place, active hosts should be able
  to notice relevant local change quickly and a shared occurrence such as a large
  stone falling should become grounded context for eligible Characters; and
- scale mechanisms must serve excellent game design. They may not preserve a weak
  mechanic merely because it is easy to serialize or partition.

The User also accepted the development workflow for resolving this frontier:

- `lab/` is a lasting general experimental workbench, with multiplayer as its first
  current track under `lab/multiplayer/`;
- lab code may remain deliberately rough and is retained with an explicit verdict
  and status, but it is never automatically promoted into production;
- the grill asks exactly one material question per User turn; and
- a small in-memory, Postgres, MCP or Agent experiment is run only when its bounded
  observation can change a pending decision. Agent calls remain explicit and never
  become background World behavior;
- one lab question may compare multiple credible setups or settings when their
  difference can change the pending decision, but comparison is never mandatory and
  an uninformative parameter sweep is rejected as ceremony;
- every grill question includes Root's argued preference, passed through Terry's
  player-value, contract, domain, smallest-safe-slice and exact-evidence gates; and
- Root may use bounded subagents across the complete plan for independent research,
  adversarial critique and experiments, while retaining integration and judgment.

Repository authorities, not conversational memory, carry this grill forward. After
every answer, the complete current direction and remaining branch are updated here
and the material choice is appended to `docs/concept/log/`. New sourced findings and
their citations live in `docs/research/`; only later accepted current game behavior
lives in `docs/game/`. `lab/` remains separate: it contains experimental realizations
and bounded verdicts that link back to a decision, never the decision itself.

The User further required this repository-memory and experiment-separation method to
become reusable cross-task policy in `AGENTS.md` and the relevant build, grilling and
prototype skills. That plan expansion is accepted and complete: the build
constitution now requires repository-backed resumption and authority separation;
build-aicadia performs per-answer recording; grilling honors explicit round size;
and prototype permits a project-designated retained lab without direct production
promotion. Three syntax validations and independent forward-tests passed before the
multiplayer grill resumed.

The User subsequently chose Rust as the default implementation medium for technical
Aicadia labs because the runtime is Rust and the useful outputs are transferable
invariants, tests and measurements. HTML/JavaScript remains available only when
human visual or semantic interaction is itself the evidence. The first observation
fixture is migrated rather than duplicated; lab code still cannot be copied or
promoted directly into production.

## Confirmed Q1 direction — persistent placement, dormant perception

A Character that is still held by its player remains canonically placed when the
User or host disconnects. Disconnecting does not remove, teleport or protect it by
itself. Presence alone also creates no personal sensory memory: without an explicit
active-attention boundary, the Character cannot later claim it directly saw or
heard an ambient occurrence. On return it may discover current state, public traces
and applicable personal consequences through later accepted bounded reads.

“Player-controlled” is working design language here, deliberately distinct from
“active attention.” It does not introduce a current status field. Whether an
offline Character is a valid target remains mechanic-specific; mere co-location
grants neither blanket permission nor blanket immunity.

If a player later adopts another Character and abandons the old one, the old
Character keeps its durable World identity and may continue through a future NPC
lifecycle instead of disappearing. The abandonment transition, ownership/control,
NPC decision source, activity cadence and safety rules remain open. This direction
does not authorize a server LLM, a background Agent call or generic autonomous
simulation.

## Confirmed Q2 direction — explicit subscribed attention

For a player-controlled Character, active attention begins only after its
authenticated host obtains an authoritative baseline for the current Character and
Place and explicitly subscribes to observation there. It ends on unsubscribe,
connection loss, Character switch or Place departure. Merely being persistently
placed or having an idle authenticated process does not create attention.

While subscribed, the host may receive coalescible change hints, perform bounded
authoritative reads and buffer relevant context for the next explicit User-owned
Agent turn. The subscription creates no durable World `attention` state, no personal
copy of each Activity and no LLM call. An always-connected host may keep listening,
analogous to an online MMO client, but transport availability remains distinct from
World truth and actual Agent knowledge.

## Confirmed Q3 direction — public Place history is not personal memory

A Character that arrives later at the exact Place may retrieve a bounded public
occurrence such as the stone fall through local Place history. It may learn “the
stone fell here,” but its Agent may not present “you saw or heard the stone fall.”
Current state and physical traces remain separately authoritative; the historical
Activity is learned public history, not reconstructed perception.

The occurrence is stored once and queried through a bounded local lens. No personal
observation receipt or recipient copy is created. Private or explicitly targeted
Interactions remain excluded from unrelated Characters, matching the current local
Activity contract. A later mechanic must make public occurrence meaning explicit;
actor-written prose, arbitrary Properties or Traits never grant visibility. Showing
only current aftermath was rejected because it would erase other players' durable
contribution from ordinary discovery; treating all local history as personal memory
was rejected as false omniscience.

## Confirmed Q4 direction — World owns eligibility, Agent owns presentation

The User challenged typed `visible` and `audible` metadata as likely overkill and
proposed comparing the Activity and Character locations, then letting the Agent
decide how it was seen or heard. Independent sensory and gameplay critiques supported
the simplification but rejected Agent-owned information eligibility. The User then
accepted the revised hybrid under Terry and explicit ownership boundaries.

Not literally every current subject is placed: an ordinary Entity or Character may
be unplaced and some Activities have no context Place. For the proposed first live
slice, however, the relevant public Action and subscribed Character both have one
World-derived exact Place. Equality is therefore a sufficient first candidate scope,
provided World filters the occurrence before any payload reaches the Agent.

The revised smallest candidate is:

1. the concrete operation contract determines that one accepted occurrence is
   `public local`; prose, Properties and Traits cannot change that audience;
2. World derives the subscribed Character and authorizes only an occurrence at that
   exact Place;
3. a private or targeted Interaction is filtered before delivery, so an unrelated
   Agent never receives the secret;
4. the authorized Agent may naturally present the stone as seen, heard or otherwise
   noticed, but that wording creates no World state, mechanical reach or durable
   personal perception; and
5. hidden Actions, whispers, adjacent-Place sound, containment, occlusion and sensory
   capabilities remain unsupported until one concrete mechanic earns its own bounded
   audience or propagation rule.

`Public local` is only a working policy label in this exploration, not a proposed
table, universal Activity field or canonical operation type.

| Owner | Exact responsibility in the first slice |
| --- | --- |
| Concrete operation contract | declares whether its accepted occurrence is public and local; prose cannot alter this |
| World | derives Character, active attention and exact Place, then authorizes before returning content |
| Activity | stores the occurrence and its attributable local history exactly once |
| Host transport | holds subscription, coalescible hints and bounded temporary context; none is World truth |
| Agent | chooses natural sight, sound or other presentation only after authorization; creates no mechanic or knowledge state |
| User | explicitly initiates every Agent turn and token spend |
| Future concrete mechanic | owns any later hidden audience, whisper, propagation, occlusion or sensory-capability exception |

This is smaller than channel metadata on every Activity and scales as one public
Place-interest cohort. Pure location-only Agent authority remains rejected: once a
hidden theft or whisper is sent to a client, privacy is already lost, and different
models or prompts would produce mechanically inconsistent audiences. Typed sensory
channels in the first slice are rejected as unearned complexity; a concrete mechanic
may earn one later. No new canonical glossary term is introduced. Existing sourced
support and limitations remain in
[`multiplayer-concurrency-and-world-observation.md`](../research/multiplayer-concurrency-and-world-observation.md).

### Bounded lab evidence after Q4

The retained
[`observation-ownership` experiment](../../lab/multiplayer/01-observation-ownership/README.md)
implemented the Q1–Q4 ownership split as one dependency-free in-memory state model.
All eight fixed scenarios passed: public same-Place Action, duplicate and lost hint,
disconnect, later arrival through public history, private Interaction filtering,
Character switch and Place departure. The model needed no durable attention,
personal observation or sensory-channel rows, and Agent state changed only on its
explicit User-turn command. The experiment verdict is `supported` and its artifact
is `kept`.

This is evidence of semantic representability only. Host hints, refetch and Agent
presentation were simulations: no real MCP subscription, player Agent, LLM,
Postgres query, gateway or concurrent load was exercised. Its toy sequence expressly
does not decide lossless ordering or catch-up. A real Agent smoke can later test
whether already-authorized live context and public history are interpreted
correctly; it cannot prove World authorization, privacy or delivery correctness.

### Current real-Agent/MCP feasibility boundary

A read-only runtime audit found no honest live-subscription seam yet. The current
Rust MCP server exposes stateless tools and no subscribe/listen or notification
capability; active attention, observation replay, durable Agent sessions and
notifications remain explicitly deferred in `docs/game/`. Accepting an SSE response
format is transport framing, not a live observation contract.

The current production seams can already support a narrower future smoke: real
Postgres, World, HTTP and MCP can show a later Character a public Place Action while
filtering a private Interaction from an unrelated Character, and one explicit pinned
Agent call can test its presentation. That would prove only the exact history,
privacy-query and Agent-interpretation fixture. A three-call live-versus-history
presentation smoke becomes honest only after the host-attention and catch-up promise
is decided; until a real subscription exists, attention, hint and host buffer must be
reported as simulated. Existing owned disposable-database and Agent-runner machinery
can supply isolation, authoritative readback and verified cleanup, but its current
four-call Property scenario is not reused as if it tested multiplayer.

This confirms the desired outcome and rejects a whole Place as the intended final
conflict unit. It does not yet choose the exact dependency token, admission rule,
discovery law, movement model or operational partition; those remain grill
decisions before a build contract can change.

## The semantic limit

Different crowd intents are different consistency problems:

| Intent | Candidate scale behavior | Irreducible boundary |
| --- | --- | --- |
| Observe one Entity | Serve a bounded authoritative projection; accelerate with non-authoritative cache or stale-tolerant replica only when the contract permits it | A cache may never decide World truth |
| Append Activity without changing target state | Accept independent appends without an exclusive Place-wide state lock, subject to admission and storage capacity | Every accepted append still needs identity, provenance and durable history |
| Change independent Entity state | Lock or compare only the submitted current pointers and placement dependencies in stable order | The dependency set must be explicit and bounded |
| Change the same current Property or Trait | Serialize one winner/order or reject a conflict | Contradictory canonical values cannot all be current at once |
| Reveal one shared unknown | Give the transition a stable identity and let one atomic result win | Concurrent observers must converge on the same accepted fact |
| Make one occurrence available to a million active Characters | Store once, fan out coalescible refresh hints and recover through bounded authorized reads | One million recipients still require roughly one million network deliveries; background Agent activation is forbidden |

No sharding scheme, CRDT or cache can make one million contradictory changes to one
canonical current fact simultaneously true. “Fully handles the burst” must mean
that World accepts, rejects or asks for a later attempt within explicit bounds,
without duplicate facts, split current state, unbounded waits or retry collapse.

### Current Rust mutation pressure

The current implementation makes the scale blocker concrete. Both `submit_action`
and `submit_interaction` lock the requesting User and then the current Place, compare
one `expected_place_revision`, and advance `place.latest_activity_id` after every
accepted contextual mutation. The test suite deliberately proves that every
Place-relevant writer waits for that same row. This is coherent MVP freshness, but a
crowd painting different doors, introducing different stones or appending unrelated
Interactions at one Place still becomes one serial mutation lane.

A full local Rust/Postgres baseline run made that boundary executable rather than
theoretical: 152 tests passed, one deliberately ignored server test remained
ignored, and the concurrency cases proved both one winner from one Place revision
and waiting behind the shared Place row. This is evidence for the current
implementation and fixture only, not a throughput or production-scale result.

The lower state machinery is already narrower: Property and Trait current pointers
are locked in stable subject order, and request id plus fingerprint owns retry
identity. That means the Place lock currently protects more than the actual changed
state requires. A second pressure point is locality validation for Trait changes,
which can enumerate all local Entity ids; a full crowded-Place scan is not made safe
merely because the final mutation list is bounded.

The next contract must keep four responsibilities distinct:

1. the concrete operation defines the bounded typed facts on which its meaning may
   depend;
2. the Agent supplies the expected values, absence or opaque versions it actually
   read, without inventing dependency kinds in prose;
3. World always revalidates ownership, actor placement, target locality, eligibility
   and bounds, even when the Agent omitted them; and
4. PostgreSQL locks only the rows needed to make those checks and writes atomic,
   while accepted request identity plus a normalized fingerprint owns replay.

The candidate unbundling for the next grill is therefore:

- User lock continues to serialize one User's mutation admission and request retry;
- the operation contract determines the allowed dependency shape and the submitted
  intent carries only the current typed facts and expected absences on which its
  meaning depends;
- World independently revalidates actor placement, target locality and permission at
  commit time;
- all bounded read and write dependencies—not merely changed rows—are checked, while
  actual Property, Trait, Character-placement or other concrete subject rows
  serialize in stable order; and
- read-orientation and pagination tokens stop doubling as a universal Place mutation
  revision.

Removing only `expected_place_revision` would not achieve this: advancing
`place.latest_activity_id` would still make every accepted writer touch the hot Place
row. Expected absence also has no existing Property row to lock. A conservative
first PostgreSQL shape may therefore lock each affected Entity in stable order,
recheck the exact present-or-absent Property facts, then update current state and
Activity atomically. That can temporarily serialize different Properties of one
Entity without falsely conflicting them; a persistent generic dependency or slot
framework is not earned.

The retained dependency-free Rust fixture in
`lab/multiplayer/02-subject-conflict` passed ten fixed interleavings: independent
same-Place subjects and different Properties could both succeed; the same current or
expected-absent Property produced one winner; moved-actor, replay and rejected-request
semantics kept exact Activity counts. This supports the candidate semantics only.
Its World, locks, transactions and persistence are simulations, so PostgreSQL race,
atomicity, crash and latency claims remain open.

The real-PostgreSQL scratch follow-up in
`lab/multiplayer/03-postgres-subject-conflict` deliberately refuted a stronger
claim. Existing Door Property work bypassed a held Place lock and a hot Stone lock
did not block the Door; same-current and expected-absence contenders each produced
one accepted history path; retry, fingerprint conflict and rollback remained atomic
inside the fixture. But introducing an Entity with an honest placement foreign key
took a `KEY SHARE` lock on its referenced Place and was boundedly blocked by an old
`FOR UPDATE` Place writer. Two normal introductions without that exclusive writer
did proceed together because their integrity locks were compatible. Therefore
“Place is not the semantic conflict unit” does **not** mean “PostgreSQL never touches
the Place row.” A production transition must remove or sequence every old exclusive
Place writer before claiming same-Place introduction independence. Five SQLx tests
and the separate disposable-database cleanup audit passed; the reduced schema still
proves no production contract, throughput, migration or scale claim.

The User accepted **operation-scoped freshness** for multiplayer Q5. Ivo dropping a
Stone and Nia opening an unrelated Door may both succeed. If Ivo's concrete
roll-through-door operation read that Door as open, a concurrent close makes Ivo's
request stale even though it only writes the Stone. A World-owned occupancy rule can
likewise introduce one explicit shared dependency. The concrete operation owns the
bounded typed dependency shape, the Agent returns expected typed facts it actually
read, and World adds its mandatory ownership, placement, locality, eligibility and
bounds checks. Whole-Place freshness was rejected because it creates false
conflicts; write-set-only checking was rejected because it misses causal reads.
This accepted product direction still selects no dependency token, schema, lock
strategy or API. Multi-Entity atomicity, Activity-only Interaction, consequence
timing and hot-row overload require later choices and real-seam evidence.

### Comparative PostgreSQL evidence after Q5

The User clarified that a lab may compare multiple credible setups or settings when
their difference can alter the pending decision, without making variant production a
mandatory ritual. The retained
`lab/multiplayer/04-postgres-conflict-strategies` fixture therefore held one
operation-scoped dependency/writer model constant and compared Place, Entity,
hybrid current-row/Entity-fallback and exact Property-slot coordinators under
`READ COMMITTED`, plus one bounded `SERIALIZABLE` write-skew challenger.

Ten focused SQLx/Postgres tests produced a material correction:

- Place, Entity, hybrid and exact-slot strategies all produced one accepted request,
  one dependency conflict and one Activity for basic same-current, same-absence and
  one explicit bidirectional Stone-clear/Door-open invariant;
- Place coordination had the widest false-contention radius and Entity coordination
  remained simple but Entity-wide;
- hybrid exact-current plus Entity fallback looked narrow in simple cases, but two
  semantically independent requests mixing `Absent` and `Current` dependencies
  formed a cross-class cycle through later history foreign-key locks, so at least one
  bounded request aborted;
- one exact `(Entity, Property key)` slot with a nullable current Activity gave
  present and absent facts the same stable lock class and total order; it accepted
  that same mixed pair with two Activities and kept a held `Stone.state` slot from
  blocking `Stone.color` or the Door; and
- `SERIALIZABLE` SSI reduced one controlled bidirectional write-skew pair to one
  commit and one `40001`, but absence, retries, abort storms and quiet-subject pool
  isolation were not tested.

Experiment 04 verdict is `refuted` for the initial hybrid-is-smallest hypothesis.
The exact Property slot was the strongest explicit coordinator within this scratch
matrix. After its game and million-User consequences were restated plainly, the User
accepted Q6 option B: independent facts of one Entity may change concurrently, while
two changes to the same exact fact share one canonical lane. This earns an internal
stable `(entity_id, property_key)` coordination identity, including a representation
for expected absence. One exact hot fact still requires bounded `accepted`,
`conflict` or `busy` outcomes; the choice does not claim million-write throughput or
select a production schema or API.

The first formulation of that decision was too implementation-led. The User required
all later options to pair an understandable game and million-User consequence with
the precise technical consequence. Q6 was accepted only after being restated in
those two layers. This communication requirement remains a compact
Terry/build/grilling rule, not merely a conversational preference.

### Open follow-up: cross-Property actions, listeners and cooldown

The User's concrete Tree case tests the boundary of exact slots: one Agent wants to
paint it blue while another wants to chop it. The first candidate explanation gave
World too much semantic knowledge. Current Aicadia deliberately has no Entity types,
Property catalogue or inference: World cannot know that `tree`, `standing`, `felled`
or a prose description means destruction. The Agent authors the exact consequence
and understands its fictional meaning; the User confirms it; World can validate only
the submitted bounded state, exact versions and dependencies plus structural
ownership, placement, locality, authority and history rules it owns.

This yields a hard boundary. World can recognize a future typed structural operation
such as movement or retirement because its contract defines that effect, but it
cannot safely trust an Agent-supplied `destructive` label or infer one from arbitrary
Property content. An Agent may explicitly declare that painting depended on the
Tree's current form and World can revalidate that version; World cannot prove that
the Agent named every semantically relevant fact. Omitting such a dependency may
produce an odd but still historically attributable state unless an explicit
structural or communal authority rule forbids it.

An Entity-scoped listener can announce a stored proposed or accepted change and let
an explicitly invoked Agent refetch; it cannot resolve meaning, prove interest or
grant authority. A generic reactable-action protocol could let an Agent voluntarily
submit a bounded proposed consequence with a deadline, while explicitly protected
communal facts could require their already-selected governance path. Whether that
voluntary protocol is worthwhile, who may require it, which structural actions earn
mandatory reaction and whether stabilization is mechanic-owned remain open.

### Confirmed intelligence ownership and property-agnostic calls

The User sharpened the boundary into a cross-task build rule: World must always stay
semantically dumb, while explicitly invoked Agents own understanding and correct call
assembly. If a decision needs intelligence, the game should be able to place that
judgment with one Agent or a bounded collective of eligible nearby Agents; World
must not grow an ontology or model judgment to imitate them. The constitution,
build-aicadia workflow and canonical World/Agent vocabulary now record this split.

This explicitly rejects ceremonial pseudo-physics. Not every Tree has `form`,
`exists`, `shape`, `state` or any other mandatory lifecycle Property, and exact-slot
coordination does not create such keys. An Agent inspects the actual Entity and names
only the real or expected-absent facts its intended call uses. World validates the
submitted identities, exact versions and bounds; it neither invents missing
Properties nor proves that the Agent's semantic dependency set is complete.

The explosion example generalizes the same ownership. An Agent may reason that one
occurrence affects Places A and B and submit one bounded structured consequence that
names them. World does not calculate a blast radius from prose. It may verify only
facts it structurally owns, such as the actor's authority, explicit spatial
relationships, permitted locality, target existence, version freshness, maximum
scope and atomic history. Which evidence makes a cross-Place scope admissible,
whether the consequence is one atomic call or several causal local calls, and how
nearby Agents collectively challenge or complete it remain research and grill
questions.

The broader candidate is therefore not merely voting on a predefined communal
Property. It is a bounded Agent deliberation and intent-assembly surface: World can
store a proposal or accepted contribution, announce a change hint to eligible active
hosts, let explicitly invoked Agents read and contribute, and deterministically
settle only the validated structured result. This may become a core concurrency and
gameplay mechanic. It does not yet supersede the confirmed rule that traffic alone
cannot grant collective authority.

#### Primary-source synthesis: collective intent assembly

Three targeted reports now constrain this candidate:

- [MCP subscriptions and collective Agent intents](../research/mcp-subscriptions-and-collective-agent-intents.md)
  establishes that MCP `2026-07-28` can send an opt-in, content-free resource
  invalidation to an already-alive host. It cannot wake a stopped host, invoke an
  LLM, replay missed changes or implement governance. A stable Place/Entity board
  index plus authoritative refetch is a better fit than dynamic event payloads;
- [Multi-Agent deliberation and consensus](../research/multi-agent-deliberation-and-consensus.md)
  finds task-dependent benefits from diverse initial drafts but also wrong
  convergence, correlated errors, problem drift, adversarial persuasion, prompt
  infection and quadratic all-to-all context. Agreement is not truth. The strongest
  testable process is sealed independent drafts, at most one bounded critique or
  amendment round, immutable final packages and one fixed selection rule; and
- [Agent-authored bounded World intents](../research/agent-authored-world-intents.md)
  shows that client-authored targets and exact optimistic preconditions can coexist
  with a strict authority. A claimed multi-Place scope must be structurally
  witnessed, capability-authorized, collectively ratified or rejected/narrowed.
  World can often prove inclusion in a bounded envelope but cannot prove semantic
  completeness, such as whether adjacent Place C was wrongly omitted from an
  explosion affecting A and B.

`Consensus` is therefore too ambiguous as the working game term: conversational
agreement, voting, distributed consensus and an atomic World settlement make four
different claims. The current working phrase is **collective intent assembly**:
eligible Agents independently interpret World facts, challenge or synthesize exact
candidate requests and finish with one immutable versioned consequence package;
World applies a predeclared deterministic selection rule, then revalidates and
settles that package or nothing.

The smallest research-backed candidate flow is:

1. one Agent reads bounded actual World facts and opens a versioned proposal with an
   exact claimed subject/Place scope, dependencies, writes and bounded rationale;
2. eligible Agents submit sealed independent complete packages or abstain before
   seeing peers' content;
3. after reveal, each gets at most one bounded critique or replacement-package turn;
4. one fixed rule selects an immutable package or no outcome; no Agent or LLM judges
   argument quality for World;
5. World checks authenticated identity, eligibility, structural scope witness or
   authority, item/byte/time limits, exact current dependencies and every write;
6. one short exact-slot transaction commits current state plus Activity, or reports
   stale/conflict/busy; no lock is held during Agent reasoning; and
7. a mechanic-specific exact-result cooldown may follow only an accepted settlement.

Four scopes must remain separate: the Agent's claimed causal scope, World's
structurally admissible envelope, the eligible participant set and the active
notification-interest set. An Agent cannot self-select only friendly voters; a broad
listener set cannot make its members semantically affected. MCP transports changes
to a board but does not own any of these scopes.

Activation, eligibility, bounded participant selection, the fixed settlement rule,
whether per-decision ballots conflict with `No Score Anywhere`, hostile-text history,
stale-result behavior and cooldown scope remain product decisions. Current Aicadia
MCP exposes only tools and cannot yet run the researched resource/subscription flow.

#### Core problem: massively concurrent Agent-authored World change

The User corrected the problem hierarchy again. Canonical authorship, authority,
listeners, voting, exact slots, cooldown and settlement are important downstream
questions, but none is the root problem.

> Aicadia is one persistent multiplayer World with millions of Users and millions of
> Entities. Through their Agents, Users can concurrently discover, create, enrich and
> manipulate Entities and linked Entities such as Places and areas. World state may
> therefore change at large scale at any moment while every participant must continue
> to encounter one coherent shared World.

The solution must keep semantic intelligence and correct consequence assembly in
Agents while World stays dumb, deterministic and authoritative over bounded state
transitions. It must remain performant both when work is spread across millions of
subjects and when a crowd converges on the same subject. It may not require a fixed
Entity ontology or ceremonial Properties. Agents may decide that chopping a Tree is
more consequential than painting it, or that both consequences validly compose into
a felled blue Tree; World must not infer that meaning.

All current mechanisms are now treated as hypotheses for pieces of this one problem:
exact-fact coordination, dynamic dependencies, multi-Entity and multi-Place calls,
Agent deliberation, subscriptions, deterministic settlement, overload behavior and
cooldown. Work proceeds step by step, with one concrete decision or bounded lab
experiment at a time. No single downstream issue is promoted to “the core” before
the complete concurrent World-change problem has been decomposed.

#### Confirmed first decomposition: action-local interest and declared effect scope

For the first concrete multiplayer case, a Character is at one Place beside a Tree
and its Agent prepares a request to make that Tree blue. Every current World subject
the Agent considers relevant to that preparation must be in a bounded active interest
set—at least the Tree and current Place in this case. Their resource subscriptions
provide content-free change hints so the host can mark its prepared context stale and
refetch. Subscription delivery is never correctness: the request also carries exact
expected current facts or versions, and World revalidates them at commit even when a
hint was lost, delayed or duplicated. No notification starts a new Agent turn.

The Agent also owns explicit effect scope. If its User detonates a bomb and the Agent
reasons that surrounding Places A and B are affected, the submitted bounded call
names A and B plus its exact intended consequences and dependencies. World does not
derive blast radius. It validates only structural availability, admitted relations or
capabilities, hard scope bounds, current versions and write authority. If accepted,
the changed Entity and every affected Place resource become stale for their active
subscribers, who recover through authoritative reads.

Two sets may need different granularity and remain the next decision: a coarse
resource interest set answers which Entity/Place change should prompt a refetch,
while an exact dependency set answers which changed fact must reject the submitted
Action. Equating them would either make every Place change conflict or make the host
miss useful contextual change.

#### Research synthesis — one cross-host resource-interest substrate

The User deliberately stepped back before choosing those sets and required a design
that can work through Claude Code, ChatGPT desktop, ChatGPT web and later Agentic
hosts. Three focused reports now compare the [host and transport boundary](../research/realtime-agent-subscription-transports.md),
the [PostgreSQL-to-gateway path](../research/postgres-change-propagation-and-fanout.md)
and [global, local, exact and hybrid interest shapes](../research/entity-place-interest-subscriptions-at-scale.md).
They establish evidence and candidates, not a current game contract.

The portable technical lower bound is an explicit User turn followed by one ordinary
bounded MCP tool or resource read. Current host documentation does not prove that
every named Agent app can open MCP `subscriptions/listen`, surface an invalidation
and cause the same host behavior. The User explicitly rejected treating that lower
bound as full Aicadia play: live multiplayer awareness is a core game mechanic, not
an optional latency improvement. A host without a proven live-interest path does not
yet provide the complete active game, even if it can still read truth or recover
after reconnect. No notification may itself invoke an LLM under the confirmed
token-ownership rule.

The strongest small candidate remains one semantic resource-interest system, but
active play requires both complementary modes:

- `read(resource)` establishes a bounded authoritative baseline and recovers after
  loss or reconnect;
- `listen([exact authorized resources])`, or one semantically equivalent live host
  adapter, keeps an active session aware of relevant changes, after which `read`
  establishes the resulting truth.

This is not one connection per Tree. One live host can put its current Place and a
small Agent-selected set of exact Entities on one listen stream. Movement replaces
that bounded set after a new authoritative baseline. Disconnect, loss, duplication
or overload always falls back to re-listen plus refetch; no durable subscription or
per-recipient receipt belongs in World.

A single logical **World board** remains a serious discovery perspective: every
Agent can enter through the same bounded read and encounter distant change or
culture. It is a poor first live feed. Invalidating every listener for every World
change creates global wake amplification, reconnect stampedes and pressure toward a
forbidden hot global revision or cursor. The current candidate is therefore a
bounded pull-only World board, a current-Place board for discovering unknown local
change, and exact Entity watches for Agent-selected focus. A private Character
attention resource adds personalized state and is deferred unless a real host or
privacy need earns it.

Inside the server boundary, the smallest unaccepted propagation candidate is one
compact best-effort post-commit PostgreSQL `NOTIFY` dirty hint, one dedicated
listener per bounded gateway process and transient coalescing by resource before
external delivery. The hint carries no prose or state and may be lost; authoritative
reads preserve correctness. An outbox, CDC stream or subject-routed broker is earned
only if measured gateway count, write rate or a future lossless projection contract
defeats that smaller path. A million listeners to one Tree still require roughly a
million outbound deliveries when all are online; the system can share computation,
coalesce hints and avoid automatic Agent calls, but cannot make that network work
disappear.

#### Confirmed Q7 direction — live and bounded under pressure

The User selected coalescible live delivery over lossless per-recipient Activity
delivery. During normal operation an active host should receive each relevant
resource change promptly. When the same resource changes faster than that host can
consume it, the delivery layer may retain one pending stale signal instead of an
unbounded queue. The host then refetches authoritative current state plus bounded
recent Activities. World still stores each accepted Activity once; it does not copy
every Activity to every listener or promise that every intermediate change enters
every Agent context.

This is the scalability boundary, not permission for silent polling. The active
live-interest path remains required, exposes loss/reconnect state and must converge
quickly after coalescing. Exact queue bounds, latency objectives, recent-Activity
window, slow-consumer behavior and internal transport remain unchosen and require
lab evidence. No semantic priority is inferred by World: the delivery layer merges
repeated dirtiness by resource, while the Agent interprets the bounded authoritative
result.

#### Confirmed unification direction — one change substrate

The User rejected continuing as though concurrency coordination, Entity mutation,
Activity, listeners, scoped World change and collective assembly were separate
systems. The target is one lightweight but powerful substrate through which every
Entity creation or change is submitted, settled, recorded and made available to
active Agents. It may have several internal components and may introduce new Agent
tools when required, but those components cannot create parallel identities or
authorities for the same accepted World change.

The new [unified-system research](../research/unified-world-change-system.md) finds
one coherent candidate: an Agent authors one bounded change package; World validates
and atomically settles it once; that transaction writes current state, one Activity
and the exact resource identities made dirty. Resource reads, MCP live hints,
reconnect recovery and any optional collective assembly all point back to that same
package and authoritative state. A proposal round may assemble the package, but may
not become an alternate Entity mutation path or truth system.

This direction also separates huge scoped change from huge materialized rewrite.
“A World-wide condition now makes everything appear blue” can be one structurally
World-scoped effect delivered through the same substrate. “Rewrite the own `color`
Property of every existing Entity” remains intrinsically proportional to Entity
count and cannot be represented as instant atomic work without lying about state and
history. Exact scoped-effect semantics remain an open product decision.

The prior exact-fact, Activity, coalescible-live-interest and Agent-intelligence
choices remain candidate constraints inside this single substrate. The earlier next
question about selecting one listener scope is superseded: the next accepted work
must first prove that create, change, concurrent conflict, scoped effect, Activity
and dirty-resource derivation can share one change kernel without importing a
generic event or rule engine.

#### Confirmed correction — compact structural reach, Agent-owned noticing

The User rejected a design in which an Agent must enumerate every Place covered by
one large effect. If a sound, storm or other occurrence reaches one thousand houses,
its accepted package must be able to name one compact, structurally verifiable
selection instead of one thousand Place ids. The User explicitly rejected adding a
mandatory `district`, containing `Area` or another map hierarchy merely to carry that
selection: those are hypothetical content structures, not an earned World primitive.
The strongest open candidate is one exact Place or one bounded World-evaluated reach
starting from an Entity or Place over explicit current structural relations. The
current World does not yet own those relations or that query surface. `Reach` is a
working term, not canonical vocabulary; arbitrary prose and an unbounded dynamic
query are not authority.

World owns only structural result membership, authorization, bounds and freshness.
The submitting Agent owns why that reach is meaningful. One accepted occurrence over
that reach is stored once and makes its shared resource dirty; it does not create an
occurrence, Activity or delivery row for every matched house or Character. Active
hosts whose authorized interest intersects that reach may receive one coalescible
stale hint and refetch. A compact reach is not a free materialized rewrite: if the intended result
changes the own state of one thousand houses or their contents, those independent
mutations remain proportional bounded work.

The User also selected Agent-owned sensory interpretation inside the already
authorized structural reach. An originating Agent may state, for example, “this
music is noticeable only by Characters that can hear.” World does not interpret
`hearing`, search Trait prose or evaluate a Property as a sensory rule. A receiving
Agent inside the admitted scope receives the occurrence together with the
authoritative Character Property/Trait context it needs, decides whether its own
Character noticed it and may silently do nothing when the Character is deaf. No
World mutation, response or explicit player-facing denial is required. This does not
weaken World-owned privacy: private or targeted content is still filtered before an
unrelated Agent receives it, and an Agent outside the structural scope does not gain
the occurrence merely by claiming it can hear.

The exact representation of compact reach, how its result is versioned, and which
historical Character state a later read should use remain open experiment and
product questions. The ownership direction is fixed: World filters structural and
private eligibility; Agent-authored meaning plus receiving Agent interpretation owns
fictional sight, hearing and other noticing.

#### Confirmed reset — flat game model before mechanisms

The User rejected the proposed `Affordance`, causal-link and mechanical-runtime
direction as too large, abstract and premature. That direction is withdrawn from
the current exploration. It supplies no candidate architecture, canonical term or
next experiment.

Exploration returns to the existing game nouns: World game-state, Entities,
Relations, Actions, Interactions and Consequences. `Relation` and `Consequence` are
still unresolved working words, not accepted implementation concepts. Before any
propagation, subscription or concurrency mechanism is designed, one smallest scene
must make the following concrete: who acts, what state exists beforehand, what the
Agent submits, what changes immediately, what another Agent may still influence and
what final state and history World retains.

Root asks exactly one concrete gameplay question at a time and updates this record
before advancing. No replacement architecture is implied by this reset.

### Confirmed scope: collective settlement is an explicit special authority

The User proposed turning extreme contention into gameplay: interested Agents could
submit desired changes, exchange short arguments, vote or form a compromise within a
bounded window, after which World applies one result and briefly protects that result
from another change. This is promising for deliberately shared meaning such as a
settlement name, monument or communal inscription. It is a poor universal rule for
physical causality: a falling Stone, opening Door or picked-up Entity cannot wait for
a parliament without making the World slow and incoherent.

The User selected this as an optional authority layer above, never a
replacement for, deterministic Property coordination. Agents may create proposals,
rationales and compromises, but user-controlled LLMs cannot decide eligibility,
correctness or settlement. World would have to own one round for an exact base
Property Activity, fixed phases and database-clock deadlines, bounded typed
proposals, one explicitly submitted vote per eligible User, a deterministic
status-quo-on-tie rule, atomic result plus Activity, idempotent settlement and a
cooldown only after an accepted change. A subscription can announce that the round
changed; it cannot prove interest, wake an Agent or spend a User's tokens.

This mechanism does not make one million participants free. Listener counts are
racy and gameable, one shared tally becomes hot, all-to-all Agent discussion is
quadratic, and one million authoritative votes require one million authenticated
inputs plus aggregation, history, Sybil resistance and overload control. The small
candidate must either bound decision-makers and proposals, restrict authority to
explicit owners/local governors, or later earn dedicated distributed aggregation.
Millions may observe through coalesced hints without receiving rows of their own.
Collective settlement may therefore apply only to World facts whose concrete
operation explicitly defines them as communal, such as a settlement name, monument
or shared creation. Traffic, listener count and conflict frequency can never grant
that authority or open a round for an ordinary Property. Who is eligible, which
bounded decision rule applies and whether voting is compatible with or deliberately
evolves `No Score Anywhere` remain open; Q6 now supplies exact Property coordination
underneath any later collective settlement.

## Candidate multiplayer observation separation

The new multiplayer research supports four separate meanings. Collapsing any two
would either weaken game truth or create pathological fan-out:

| Plane | Concrete responsibility | Durable authority |
| --- | --- | --- |
| World consequence | Validate and atomically apply the Agent's exact explicit consequence | exact affected state plus one Activity in one PostgreSQL transaction |
| Observation eligibility | Decide whether an active Character may retrieve the public local occurrence | concrete operation audience plus World-derived active attention and exact Place; no personal observation row in the first slice |
| Realtime delivery | Tell an active host that relevant context may have changed | no World authority; a lossy, duplicate-tolerant and coalescible hint |
| LLM consumption | Let the Character's Agent reason about the occurrence | only inside a new explicit User-owned turn and token spend |

For the large-stone example, the candidate flow is concrete:

1. the Agent submits one explicit typed Action grounded in current stone and
   Character state; World never derives mechanics from prose;
2. World locks or compares only the actual stone/state dependencies, accepts one
   canonical consequence and appends one Activity;
3. for the first slice, the concrete Action contract makes the occurrence public at
   the exact Place; World authorizes it before delivery and the Agent chooses only
   natural sensory presentation. Typed visible/audible channels wait until one
   concrete mechanic earns them;
4. after commit, a rebuildable publisher emits an opaque Activity/scope change hint;
5. delivery gateways share candidate work, coalesce pressure and fan the hint out
   to active interested hosts without writing one recipient row per Character;
6. the host performs a bounded authorized World read, so a forged, duplicated,
   reordered or lost hint cannot leak or change World truth;
7. a late or reconnecting host reads current state first and only then a bounded
   relevant Activity delta or recap; and
8. the host gives that material to the LLM only on the next explicit User turn.

This makes “immediately available to an active host” a plausible transport claim.
It deliberately does not claim “every LLM immediately knows”: that requires a new
model call and would violate the no-background-token boundary.

Lossless catch-up also needs an explicit decision. The current newest-first
`(occurred_at, id)` history cursor is a stable presentation cursor, not a guaranteed
commit-ordered forward watermark once independent same-Place transactions may
commit out of order. A future delivery watermark may be partition-local transport
or projection state; it may never become a global World revision or semantic Place
identity. Exact current state plus bounded recent context may be sufficient if the
game does not require every intermediate ambient occurrence to be recovered.

### Candidate later catch-up direction — World lossless, attention bounded

The strongest current Terry recommendation is not “drop events” versus “keep
events.” World truth and Character attention have different obligations:

- every accepted consequence and its one Activity remain durable and queryable;
- an authoritative refetch returns exact bounded subject/Character state, never the
  whole crowded Place;
- public-local history and Character-involvement history are separate bounded,
  paginated authorization lenses over that same stored history; involvement follows
  only concrete operation roles and never means personal perception;
- hints may coalesce or disappear, and an overrun resets to a fresh baseline instead
  of growing an unbounded replay queue;
- a missed ambient occurrence remains discoverable through explicit public history,
  but is not silently turned into personal perception; and
- no generic importance score, server summary, per-Character ambient receipt or
  automatic Agent call decides what matters.

This avoids letting 10,000 ambient Actions displace exact current Character state or
a concrete involvement consequence, and it does not force every Agent to process
grief-generated noise. The same Activity may appear in more than one authorized
bounded lens and is deduplicated by identity; it is still stored once. A future
concrete combat, whisper, watch or sensory mechanic may earn its own stronger bounded
read or detection contract; no generic mechanic-owned lens is introduced now.

Private Interaction never causes a public Place hint, including a content-free hint
whose timing would still leak that something happened. Reconnect starts from a new
authoritative Character/Place baseline; old ambient buffers do not prove continuous
perception. Multiple hosts remain independent transport consumers and may hold
different context without changing World mechanics.

The unresolved game choice is real: under this model, an actively subscribed
Character can miss an intermediate purely ambient occurrence during overrun. That is
bounded attention rather than lost World history. Rust can prove memory bounds,
filtering and convergence for fixed lenses; only the User can decide whether this
trade feels like presence rather than unfair blindness.

## Candidate north-star direction

The following is a recommendation for discussion, not an accepted contract.

1. **Conflict follows causality.** A request names the smallest set of placement,
   Property, Trait, topology or attempt facts whose change can invalidate it.
   Unrelated Activity at the same Place does not automatically conflict.
2. **Semantic Place is not a shard.** Place identity, history, containment,
   adjacency, geometry and an operational cell or database partition remain
   separate. Repartitioning never renames a Place or changes player knowledge.
3. **Work follows touch.** Inactive World areas receive no universal heartbeat.
   Time-dependent state is introduced one concrete mechanic at a time as a pure
   timestamp-derived view, bounded mechanic-specific catch-up or explicitly active
   local simulation.
4. **The World has momentum, not autonomy.** New creative meaning comes from
   explicitly invoked player Agents or an explicitly invoked, attributable human
   steward. Time may derive or complete only an already accepted deterministic
   possibility; it does not invent prose, weather, intentions or culture.
5. **History stays durable; attention stays bounded.** Activity remains immutable,
   while Character-facing reads select a bounded, authorized lens. A global feed or
   recursive “everything nearby” response never becomes the scale mechanism.
6. **An occurrence is stored once; wake-ups remain disposable.** Active delivery
   may share interest cohorts and fan-out trees, but every client re-reads
   authorized World truth and no delivery path invokes an Agent.
7. **Hotspots receive admission, not denial.** Popular subjects remain real shared
   subjects. World protects quiet work through bounded lock waits, connection
   budgets and target/context admission rather than silently cloning the subject or
   hiding players in technical instances.
8. **Scale is earned by gates.** Begin with one PostgreSQL authority and stateless
   instances. Add indexes, partitioning, replicas, caches or operational shards only
   after a measured workload names the bottleneck and a preservation test pins the
   World invariants.

## Candidate capability roadmap

### Phase 0 — make scale claims concrete

Define a conflict/admission matrix for observe, append, mutate, reveal and fan-out.
Measure the current hot-Place and adversarial-read baselines in a disposable World.
The first evidence must compare a hot Place with independent Places and prove
whether a small page performs work proportional to its page rather than total
occupancy or history. Include one stone-like public local occurrence, duplicate and
missing delivery hints, reconnect recovery and a quiet-Place control.

This phase earns its place because the current Place-wide lane and workload-unbounded
queries are concrete blockers. It does not authorize production infrastructure or a
million-User claim.

### Phase 1 — broaden the playable World

Deliver bounded Place-neighborhood context, establish additional Places and explicit
connections, and move Characters through validated transitions with Activity. This
is the strongest candidate next game edge: it creates an actual frontier and makes
locality a real player choice instead of forcing everyone into one entry Place.

Movement design must first preserve canonical retry reconstruction for earlier
Actions and discoveries after an Entity or Character has moved.

### Phase 2 — make crowds observable, safe and legible

Decide private attention control before Interaction reach expands. Separate
Activity-only Interaction from actual target-state mutation, replace Place-wide
freshness where a smaller dependency set is truthful, validate only submitted local
subjects and add explicit bounded admission and overload semantics. Define one
bounded observation lens over one stored Activity; only afterwards test disposable
realtime wake-ups and one concrete typed sensory channel.

Evidence must include a hot-target burst plus a quiet control Place. Success is
bounded accept/reject latency, no pool starvation, no duplicate Activity and no
lost or split state—not acceptance of every incompatible intent.

### Phase 3 — make discovery produce novelty rather than volume

Decide when an investigation finds an existing shared subject versus materializes a
new one, which context change earns another opportunity and how unknown remains
distinct from empty. Repeated identical context must not create World content
linearly forever merely because many Users can call the mechanic.

Evidence combines deterministic load with a blind human quality sample and a
returning-Character orientation test; raw Entity count is not a quality proxy.

### Phase 4 — add one temporal affordance

Choose one concrete player-visible opening, deadline or standing process. Define its
clock, rule version, restart behavior, missed-stage semantics and exact history
footprint. Prefer a pure on-demand derivation when intermediate stages cause no
independent World facts.

The test crosses restart, long silence, clock correction and concurrent observation
without scanning all sleeping subjects or rerolling from observation order.

### Phase 5 — add one living process

Choose exactly one flora, fauna or material experience. Represent only the semantic
subjects and qualitative states the gameplay needs. Ten million potential subjects
must cost approximately nothing until the bounded touched set matters.

No generic ecology engine, one-Entity-per-particle model, quantity grind or hidden
progress counter is implied.

### Phase 6 — support collective settlement and causal ripples

Model a structure through independently attributable parts or relations where that
matches the game, then make completion an explicit supported World action rather
than an incrementing progress bar. Let distant consequences reach a Character only
through bounded, sourced carriers; there is no global news feed.

### Phase 7 — harden hosted operation

Before public scale, add real authentication, safety boundaries, overload control,
telemetry, backup/restore drills, failure injection and online migration discipline.
Only measured table size, maintenance pressure, read load or primary write capacity
may earn PostgreSQL partitioning, replicas, caching or operational sharding.

## Open decision catalogue

### Scale claim and admission

1. Is the target one million registered, connected, concurrently active or
   simultaneously mutating Users?
2. What read/mutation rate and burst window must the first public claim cover?
3. Must every structurally valid crowd intent eventually succeed, or may bounded
   admission reject it explicitly?
4. Is fairness scoped to User, Character, target Entity, Place or intent class?
5. What player-facing outcome represents “busy” without pretending the action
   happened?
6. Which latency, availability, RPO and RTO objectives define success?

### Hot Entities and Places

7. Does “interact” mean observe, append outward behavior, change actor state, change
   target state or receive a target-authored response?
8. Which of those intents need one player-visible total order?
9. May Activity-only Interaction use narrower freshness than target-state change?
10. May a popular Character privately refuse or suppress repeated targeting?
11. May another Character's Property or Trait change without separate target
    consent?
12. How does a Character read meaningful history after being targeted a million
    times without an attention-denial attack?
13. When is a crowded semantic Place deliberately split into actual game Places,
    and when must it remain one Place despite operational pressure?
14. Can a popular Entity be composed of independent meaningful parts, or is one
    canonical subject essential?

### Freshness and consistency

15. Which exact placement and state pointers make up the dependency set for each
    current capability?
16. How does a request depend on the absence of a Property, Trait, connection or
    occupant without an Entity-wide hot revision?
17. Which stale reads are safe for orientation and which must come from the writer?
18. Does Activity display order need to equal commit order, causal order or only one
    stable presentation order?
19. How are multi-Entity locks ordered when future operations cross a Place or cell?
20. What retry budget and backoff behavior prevents retry storms without another
    Agent reasoning cycle?

### Spatial growth

21. Which relation first earns current behavior: adjacency or containment?
22. What does direct presence mean independently from visibility, sound and travel?
23. Who establishes a new Place and connection, and through which discovery or
    Action authority?
24. What immutable facts make a movement retry reconstructible after later movement?
25. Which operational partition key can change without semantic history changing?
26. Which future cross-partition action truly must be atomic?
27. How do overlaps, unknown boundaries, portals and moving containers affect
    locality without requiring a universal geometry now?

### Discovery and unknown space

28. When is a find an existing shared Entity and when is it new materialization?
29. What stable transition identity lets concurrent explorers converge on one
    revealed unknown?
30. Which Character, Place or accepted-history change earns another meaningful
    attempt?
31. How is semantic duplication handled when World deliberately performs no
    language inference?
32. How does a returning Character receive a few meaningful changes instead of an
    unbounded page crawl?
33. Does discovery pressure need per-Place or per-opportunity admission in addition
    to current per-User admission?

### Time and sparse change

34. Does Aicadia need a fictional calendar, or are acceptance and elapsed times
    enough for the first mechanic?
35. What activates a process: read, Character presence, Action or operator work?
36. Is its current state pure derivation, bounded catch-up or stored mutation?
37. Which intermediate stages matter enough to leave independent history?
38. How is randomness keyed or stored so restart and visit order never reroll it?
39. What happens after six months of silence, clock correction or a changed rule
    version?
40. Which offline changes would create punishment or FOMO and should therefore be
    rejected?

### Ecology, resources and settlement

41. Which one flora, fauna or material interaction has the highest current game
    value?
42. Is its subject an Entity, occurrence, qualitative coverage or derived context?
43. Can qualitative stages replace a numerical resource meter?
44. What makes harvesting meaningful without inventory grind, currency or a score?
45. Is a collective contribution a state change on one structure, an independent
    part or an explicit relation?
46. Who may declare a construction complete, repair it, sabotage it or abandon it?
47. What bounded historical support makes settlement status meaningful without a
    rank or population counter?

### Knowledge and culture

48. Which source may this Character know now, and what causal path delivered it?
49. How are current presence and remembered observation kept separate?
50. What provenance and information loss does a distant ripple retain?
51. May World select bounded sources while the explicitly invoked Agent privately
    summarizes them?
52. How do contested and contradictory facts remain grounded without an omniscient
    semantic resolver?
53. Which traditions or institutions emerge from repeated concrete relations and
    Activity rather than a server-awarded status bit?

### Safety, moderation and abuse

54. How can immutable Activity coexist with reduced public visibility of harmful
    content?
55. Is a moderation marker World history, private operational state or both through
    separate records?
56. Who can moderate, appeal or restore, and how is each operator action audited?
57. Which target and context limits resist brigading and Sybil multiplication?
58. How are impersonation, name flooding, harassment and semantically abusive prose
    handled when deterministic validation cannot judge meaning?
59. Which errors must remain neutral to avoid leaking identity, role or location?
60. When does input cardinality or query shape itself become an abuse vector?

### Operation and recovery

61. Which lock wait, queue depth, revision-conflict, deadlock, retry and admission
    metrics are required per conflict subject?
62. How is an operational request count kept entirely separate from in-game score?
63. Can every bounded read prove rows and buffers touched independent of total
    occupancy/history?
64. What table, WAL, index, vacuum and restore growth follows each accepted Activity?
65. Does crash at every write boundary leave exactly zero or one complete result?
66. Does uncertain-delivery retry after failover reconstruct the original result?
67. Which replicas and caches may serve which consistency class?
68. When may logical change feeds wake projections without becoming authority or a
    durable queue?
69. What measured threshold earns native partitioning or operational sharding?
70. How is repartitioning verified not to change Place identity, visibility,
    Activity or current-state answers?

### Observation and realtime multiplayer

71. Is a placed Character fictionally present while its User is offline, or can
    direct perception exist only inside an explicit active attention window?
72. May a Character that arrives later learn a Place's public chronicle while still
    being unable to claim it personally saw or heard the event?
73. Does the first observation slice mean merely “locally knowable”, or must it
    already distinguish visible and audible channels?
74. Which concrete mechanic owns a sensory signal, and how does World validate it
    without prose inference or arbitrary player-selected audience?
75. Does event-time co-presence require durable movement intervals, or is Place
    memory sufficient for the first capability?
76. Must an active observer recover every accepted ambient occurrence without a
    gap, or is exact current state plus bounded recent causal context sufficient?
77. Which occurrences may have their realtime hints coalesced, delayed or dropped
    under pressure without damaging play?
78. What does a host do after a reconnect, cursor expiry or backlog larger than the
    Agent context budget?
79. Which MCP clients can actually consume opt-in change notifications, and what is
    the portable polling/refetch fallback when they cannot?
80. When does “this Character personally perceived this Activity” become valuable
    enough to justify personal observation state and its crowd-sized write cost?
81. How are active subscriptions grouped and routed without making process-local
    membership a correctness fact or leaking a private audience?
82. Which latency and loss claims apply separately to World commit, host wake-up,
    authorized refetch and eventual LLM consumption?

## No-go directions

- universal World ticks, per-Entity heartbeats or replay of every missed second;
- server-generated prose, a hidden AI director, background Agent activation or
  automatic token spend;
- global World locks, revisions, counters, presence lists or feeds;
- semantic Place identity derived from a shard, cell, process or host;
- last-write-wins or CRDT machinery used to decide contradictory game meaning;
- caches, replicas, notifications or change feeds as correctness authority;
- claiming an LLM knows an occurrence merely because a host received a notification;
- personal observation or durable broker-consumer rows for every possible witness;
- an unbounded queue used to claim a hot subject has been handled;
- event sourcing, `world_event`, `rule`, `claim` or a generic simulation framework
  before one concrete accepted capability earns it;
- one Entity per particle, plant leaf, voxel, raindrop or other unobservable detail;
- popularity, progress, resource, relationship or settlement scores under renamed
  fields; and
- infrastructure built ahead of the measured capability bottleneck it solves.

## Immediate decision gate

The next gate is not acceptance of a broad change kernel, propagation mechanism or
experiment route. Root and User first resolve one small game scene using only the
current game vocabulary: actor, initial game-state, Action or Interaction, immediate
Consequence, any later Agent response, resulting state and retained history.

Only after that scene is unambiguous may the draft lab plan be rewritten around the
smallest system it actually requires. `docs/game/`, `CONTEXT.md`, code and production
behavior remain unchanged.
