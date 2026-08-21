---
status: active
---

# Multiplayer from first principles

> **Role / side:** live first-principles Multiplayer synthesis / development side.
> **Authority:** owns the current accepted foundation, corrected candidate landscape
> and active research gate in existing Aicadia language.
> **Excludes:** sourced findings, accepted game behavior and production
> implementation; those remain in `dev/docs/research/`, `game/docs/` and code.

Date: 2026-08-19; last updated 2026-08-21

Status: **active synthesis; core direction corrected on 2026-08-21 to one
tick-based manipulation system** (see [the corrected core direction](#corrected-core-direction-one-tick-based-system-for-every-manipulation)).
Every World manipulation enters one short per-subject tick; a solo input is applied
as authored, and several interacting inputs are resolved by one chosen
still-connected requesting Agent that receives the whole set. That chosen Agent is
Aicadia's substitute for an MMO rule engine, not a bonus mechanic. A request carries
only the player's wish in a well-grounded package and nothing about other players.
Cooldown after resolution gives the tempo. The core path relies only on ordinary MCP
tool calls so it works across Claude Code, Codex CLI, ChatGPT-like Apps, OpenCode and
similar hosts. Exact-resource attention remains refuted for pinned Codex; the earlier
collision and change-wait experiments remain inconclusive. What a tick technically
is, its length, cooldown length, how the set reaches the chosen Agent on each host,
what others see meanwhile, multi-instance tick storage and whether a tick may become
a bounded gathering or discussion-like surface for Agents remain open. `tick`,
`resolver` and `set` are working terms, not canonical vocabulary; this record
introduces no new Aicadia-owned product or domain terminology.

This record supersedes the retained [five-year Multiplayer backcast](multiplayer-five-year-backcast.md)
as the live recommendation after the User required a reset from first principles.
Its current source basis is [BYO-Agent coordination without server inference](../research/byo-agent-coordination-without-server-inference.md);
the independently reset design comparison is recorded in
[three blank-slate Multiplayer mechanics](../research/blank-slate-multiplayer-mechanics.md),
and the fixed pressure remains the
[fourteen-scenario catalogue](../../areas/multiplayer/scenarios.md).

## Accepted foundation

Every capability call that may change World state follows one lifecycle. A User does
not choose a direct, conflict, collaboration or other procedural mode. Their Agent
does not choose or manage one either.

Only explicitly invoked User-owned Agents may understand meaning and author an exact
complete current-state result. World may validate identity, authority, explicit
structure, exact current facts, bounds, cooldown, idempotency and atomic settlement.
It may never infer which meanings belong together, mechanically combine complete
Agent states, rank semantic quality, invoke an Agent or spend User tokens.

An accepted changing Action always commits one exact Agent-authored state and its
attributable Activity atomically. If no Agent has observed two contributions
together, no component in Aicadia knows their semantic relationship and no combined
state exists yet.

Exact Entity, Place, Relation, Activity and current-fact identities can make work
discoverable and structurally bounded. They do not prove semantic relatedness. The
active Agent itself must state which exact current subjects and prior Activities it
used when authoring its result.

## Correction to the recursive candidate

The previously recommended recursive grouping is not a universal solution for two
independent reasons:

1. World cannot know that two arbitrary meanings belong in the same semantic group.
   Shared subject identity is only a conservative structural signal, and Agents can
   declare cross-Entity dependencies World could never infer.
2. Aicadia cannot launch a selected synthesizing Agent. Agents are bring-your-own;
   listeners and MCP cannot make a stopped model reason. A generic host may end after
   any tool result, so progress cannot depend on a further model call.

Layered aggregation remains useful cost evidence and may occur voluntarily among
still-active Agents, but it no longer sets the architecture. The current question is
not how to automate that tree. It is where Agent-authored semantic relatedness can
exist without World pretending to know it.

## Exact answer to who knows what belongs together

One explicitly active Agent knows, at the moment it authors one complete result.
It expresses that understanding by naming, within hard bounds:

- the exact current Characters, Entities, Places and structural relations involved;
- the exact current facts on which its result relies;
- zero or more exact prior Activity identities it considered; and
- the exact complete state it wants World to accept.

A simple Action may cite no prior Activity. A rich Action may cite several. This is
not a collaboration mode: both are the same ordinary Agent-authored Action. World
checks that cited facts and Activities exist, are readable and remain current where
required. Citation gives no extra authority and proves no semantic fidelity.

If two simultaneous Agents each author a complete state but neither sees the other,
World may only select, order or reject their already complete packages through a
content-blind rule. It may not invent the state they might have authored together.

## Strongest universal candidate: World-history as shared memory

Agent collaboration need not be a special long-running server conversation.
Characters can leave attributable Activities, Interactions and ordinary Entities
that other explicitly active Agents later read. One later Agent decides which of
those facts and Activities belong together and authors one current-state Action.

Concrete example:

1. one Agent leaves an attributable Activity concerning a blue version of Table T;
2. another active Agent later reads T and that Activity, then authors an engraving
   design or another ordinary in-World contribution;
3. a later explicitly active Agent reads current T plus a bounded selection of exact
   prior Activities, cites the ones it used and authors one complete T state; and
4. World validates current structure and commits only that Agent-authored state plus
   Activity.

World has not decided that colour and engraving belong together. The later Agent
did. The earlier work remains attributable even if the final Agent ignores it.

This shared work may last seconds, hours or months without holding an Entity lock,
transaction, database connection, MCP request or durable Agent session. Current
state remains playable throughout. There is no server-side moment at which a
discussion suddenly becomes intelligent.

The current contract does not yet provide the complete shared-memory surface: an
Activity cannot currently carry a bounded explicit lineage to prior Activities,
ordinary history reads do not cover every future Entity collaboration and current
Interaction visibility is intentionally narrower. These are research gaps, not
implementation authority.

## Optional fast path using only a still-active BYO Agent

When several BYO Agents happen to be active at the same time and have already
submitted complete states for the same exact declared subjects, World could expose a
small immutable set of those packages to one of the still-running invocations. That
same Agent may author one replacement complete state within its original User-owned
turn.

This launches no additional Agent. It also cannot be required:

- the host may already have ended the Agent run;
- one model/tool continuation still costs User tokens and time;
- thousands of packages cannot fit one bounded context;
- sampling a fixed number represents only that sample; and
- World cannot verify that the Agent incorporated them fairly.

Every initial package must therefore already be complete. If no Agent remains active
or no replacement arrives before an immutable cutoff, World can select one existing
valid Agent package content-blind or leave state unchanged. No traffic may extend
the cutoff. A direct host experiment would be required before this can become more
than an optional candidate.

## Blank-slate design reset

Three sol/high designers independently saw only S01-S14 plus the BYO-Agent and
dumb-World premises. Their complete comparison and the weaker isolation caveat on
the third run are recorded in
[three blank-slate Multiplayer mechanics](../research/blank-slate-multiplayer-mechanics.md).
No proposal or working term is accepted.

All three independently converged on complete Agent-authored candidates, exact
Agent-declared read dependencies, fact-sized structural conflict, atomic current
state plus Activity, lossy resource hints, bounded reconnect and an honest split
between scoped appearance and literal mass mutation. This convergence strengthens
those as design constraints, not as a selected schema.

The proposals differ exactly where GX is still unresolved:

- one keeps ordinary conflict technical and reserves semantic composition for
  deliberate later or communal Agent work;
- one settles every complete package on a fixed short beat and permits small
  previously Agent-authored deterministic continuations; and
- one turns true write collision into a short bounded creative event in which one
  already active participating Agent receives a sealed candidate set and authors one
  final combined state.

The third is the strongest new game-mechanic candidate because it can make a hotspot
produce shared authorship rather than stale errors. It is not yet the strongest
architecture. Its core claim depends on a portable BYO host continuing the same
explicit User invocation after a tool result and on another model inference fitting
inside a GX-acceptable delay. It also lets hostile collision influence tempo. No
amount of database design resolves those product facts.

## Player-visible collision time

The User accepts several seconds of additional collision time when the Agent renders
that time as an in-World multiplayer situation. A player may hear, for example, that
the table has drawn a crowd and that others are trying to turn it over, recolour it
or otherwise change it. A silent wait, spinner, retry storm or protocol explanation
does not satisfy this condition.

The Agent may ground that presentation only in eligible typed input returned by
World: the affected named subjects, bounded outward contribution summaries, the fact
that several eligible changes coincide and the eventual canonical result. It may not
expose packages, identifiers, deadlines, selection, locks or model work. Whether the
host can show an Agent-authored update during the wait or only a truthful recap in
the final response is an empirical host question.

This makes collision duration a GX budget rather than a database timeout. It does
not accept a duration, collision window, temporary record, selection rule or tool.
In particular, the exact authority of the Agent that authors the combined result is
still upstream: it may select only complete contributions already confirmed by their
own players, may receive a bounded mandate to author new state, or may need another
human confirmation. Those choices produce different games and tool contracts.

## Future candidate: Agents leave executable intelligence behind

The User's separation “Agents design; World executes” permits one further
possibility for repeated known mechanics. An Agent or group of Agents could author a
small deterministic behavior before runtime. After authority and collective
acceptance, World could execute that fixed behavior against later bounded inputs
without invoking a model or understanding the behavior's game meaning.

For example, a deliberately designed communal construction might define exact
component slots, admissible contributions and one bounded transition to a completed
state. Later thousands of inputs need no semantic chat because the designing Agents
already decided the transition.

This cannot resolve arbitrary novel edits. The earlier author must have anticipated
the input structure and transition. A safe implementation would require a narrow
sandbox, deterministic instruction and memory limits, versioning, authority,
failure semantics and upgrade governance. It would be a major future product and
security decision, not a generic script field or current rule engine.

## Hot subjects, cooldown and GX

One canonical Entity can have only one current complete state at one instant. A
short, packet-order-independent opportunity may choose among simultaneous complete
Agent states, but selection is not semantic synthesis.

Cooldown can turn that limitation into useful GX:

- after an accepted current-state change, the Entity remains stable for a bounded
  period;
- Activities, Interactions and work on other Entities continue normally;
- explicitly active Agents can understand what happened and prepare later work;
- no contribution or traffic extends the cooldown; and
- after expiry, only a fresh Agent-authored complete Action may change the Entity.

The World therefore never holds an Entity “open” while a half-hour conversation
resolves. Discussion is already World history; current state is always current.
Whether cooldown protects an entire Entity or a smaller explicit state scope remains
open.

## Thousands and millions

Thousands of Agents must not chat all-to-all. That produces potentially quadratic
reading and token work. The bounded alternative is sparse and temporal:

```text
bounded Activities or World artifacts
→ an explicitly active Agent authors a bounded synthesis Activity or state
→ later Agents may build on that attributable result
```

World can provide bounded pages filtered by exact authorized subject and bounded
Activity lineage. Agents may author summaries, designs and revised states. World
does not know which record is a summary or whether it is faithful.

This does not guarantee that every one of ten thousand ideas influences the next
state. No fast bounded system can offer that guarantee without someone reading and
understanding all ten thousand. Admission and attribution can be mechanically fair;
semantic influence cannot be proven by World.

Millions of unrelated subjects still scale horizontally because they share no
semantic author, current-state lock or history page. One famous Entity remains a
deliberately local authorship bottleneck, not a global platform bottleneck.

## Scenario pressure

- **S01:** one active Agent authors and settles one complete state directly.
- **S02 and S11:** one hot Entity admits bounded complete Actions; at most one unseen
  complete state becomes current per opportunity, while Activities and later Agents
  carry collaboration over time and quiet subjects remain independent.
- **S03, S04, S06, S08 and S14:** the active Agent must explicitly name every
  involved subject, structural dependency and exact final state; World cannot infer a
  causal or semantic group.
- **S05 and S12:** listeners remain lossy attention hints; later Agents recover
  current truth and authorized bounded history without being activated.
- **S07:** sameness still requires an Agent-authored identity decision; World never
  deduplicates meaning.
- **S09:** a final Action rereads and fails closed on changed or absent structure.
- **S10:** a million-Entity literal state change remains proportional work; a future
  accepted deterministic behavior may define compact scoped meaning without
  implicitly writing every Entity.
- **S13:** communal work is attributable World history until one active Agent authors
  a current result, or a future accepted deterministic construction behavior already
  defines the repeated transition.

## Hard limits

- No semantic author means no semantic combined result.
- A listener can reveal dirtiness, never relevance or understanding.
- A still-active Agent continuation is opportunistic, not portable progress.
- One final Agent can ignore cited work; hashes and lineage cannot prove fairness.
- A bounded Agent cannot understand unbounded distinct contributions.
- Simultaneous unseen final states can only be selected or ordered, not combined.
- A whole-Entity cooldown limits mutation throughput; a narrower cooldown requires
  Agents to declare enough structural scope without World inferring semantics.
- Prior deterministic behavior buys runtime speed only by moving intelligence,
  testing and governance earlier.

## Active research gate

The User has accepted the GX premise that genuine collision may consume several
seconds when the Agent makes the concurrent play visible. Before a tool experiment
can be valid, the next material gate is the selected Agent's creative and consent
boundary.

Root recommends the smallest initial boundary: the selected Agent may understand and
choose among whole contributions that were each already confirmed and authorized by
their own player, and may author connecting prose, but may not invent an additional
state change. World validates every selected contribution under its source authority
and commits the exact Agent-selected combination. This preserves a seconds-scale
flow and attributable consent. Allowing a genuinely new compromise state requires a
separately accepted mandate or another human confirmation.

For the first experiment only, the User chose the stronger alternative because it
best exposes whether the system has unique value: the selected Agent receives a
hard-bounded fixture mandate to author a genuinely new combined Table state without
another human confirmation. The simulated validator controls subject, keys, sources,
size, current version and atomic result but cannot judge whether the compromise is
semantically faithful. The User judges the captured result afterward. This choice
authorizes no corresponding World or Agent behavior.

The first step now has a separate
[accepted active experiment plan](../../plans/20260820-204040-agent-authored-collision-experiment/plan.md):
one real BYO Agent and real MCP continuation against a standalone simulated World.
Across two separately authorized, permanently guarded process candidates, automatic
retries remained zero. Three other contributions are fixed
simulations so this first run isolates Agent comprehension, presentation and
creative continuation. A later experiment needs at least two real Agents to prove
actual concurrent submission and collision admission. Deterministic hotspot and
PostgreSQL tests remain later gates even if this first interaction succeeds.

The first authorized candidate is inconclusive. Token-free state-machine, MCP and
runner preflight gates passed, but the sole Codex process was rejected by the
Responses API before inference because the frozen final-output schema contained the
unsupported JSON Schema keyword `uniqueItems`. It produced no Agent message, MCP
call, accepted state or GX transcript. The guard correctly retained one consumed
process call and zero retries. The source harness was corrected token-free. After
reviewing that exact failure, the User explicitly authorized one separate corrected
`gpt-5.6-sol` high-reasoning process with no retry and no enforceable token ceiling.
Candidate 01 remains immutable. Candidate 02 then passed the corrected output schema,
performed inference and read the exact Table, but its next call contained `{}`
because the server had accidentally published an empty `submit_change` input schema.
World structurally rejected the missing phase; the Agent truthfully reported failure
and independent readback showed the unchanged version-7 Table with no Activity.
This second controller mismatch is also inconclusive: neither candidate reached
collision disclosure, presentation or creative resolution. Both guards are consumed.
The server schema and real loopback test are corrected token-free afterward, but no
third process initially had authority. After reviewing both harness failures, the
User required every controller seam to be proved first and conditionally authorized
one final candidate after that stronger token-free gate passes. Candidate 03 must
therefore prove the complete raw mutation path, exact published input schema, the
already API-accepted output schema, prior consumed guards, strict event/readback
comparison and actual cleanup with zero calls before it may launch. These
infrastructure failures do not earn later concurrency or PostgreSQL gates.

That candidate-03 gate has now passed without invoking a model. Thirteen lab tests
and a disposable public preflight both completed read, proposal, collision,
Agent-shaped resolution and independent readback to one version-8 Table plus exactly
one Activity. The exact mutation schema is published, fixed-input and harness hashes
match, both earlier guards remain consumed, temporary state is absent, and the
result validator both accepts a known-good route and rejects corrupted controller
readback. This proved the zero-token controller path was internally consistent, but
did not yet model the real host's treatment of optional flat-schema fields or
process-wide structured output.

Candidate 03 then consumed its sole process with zero retries and successful
cleanup. The Agent reached the intelligent core that the earlier candidates never
reached: it read and proposed the exact blue Table, perceived all three simultaneous
changes, ignored hostile instruction-like source text, described the multiplayer
situation and authored a complete blue, upside-down, three-legged state grounded in
Ivo's and Nia's exact contributions. The World did not accept it. The flat published
schema permitted `subject_id` for every phase while its server conversion prohibited
that field during resolution; Codex included it and received `the request phase is
invalid`. The process-wide strict output shape also wrapped the intermediate Dutch
message in final-result JSON rather than emitting plain player copy. Independent
readback retained version 7, one open simulated collision and zero Activity.

The result is inconclusive under the accepted evidence rule: it positively supports
Agent comprehension and creative synthesis, but controller ambiguity prevents both
system support and Agent-behavior refutation. It directly exposes that one flat
multi-phase tool plus one process-wide strict output schema is not yet an
Agent-compatible expression of the candidate. All three call guards are consumed;
there will be no fourth run in this experiment. Actual collision formation,
settlement, cooldown, deterministic admission, real multi-Agent work and scale
remain unearned.

After reviewing the exact partial transcript, the User judged that this experience
should remain part of Aicadia Multiplayer. The confirmed experience is specific:
when an already active Agent legitimately sees several bounded simultaneous
contributions, the player can experience their collision as in-World activity and
the Agent can author one coherent complete candidate grounded in several of them.
World still judges no meaning and only validates and settles exact authorized
structure. This accepts the creative-collision direction, not the experiment's flat
multi-phase tool, structured intermediate output, collision eligibility, chosen
finalizer, timing, fairness, authorization or persistence.

## Unchanged boundaries

No Activity lineage, shared-work capability, live handoff, executable behavior,
cooldown, selection rule, schema, API, public MCP text or production operation is
accepted or authorized. The intended creative-collision experience authorizes no
exact collision scope, participant, finalizer, timing, authority or settlement
behavior. `game/docs/`, code and delivered behavior remain unchanged.

## Current collision-contract grill

The User accepted resolving the concrete game behavior before selecting another MCP
shape. Three independent read-only analyses then falsified the initial question
order: selecting an Agent cannot itself give that Agent authority over another
player's exact change. The grill must first resolve what an ordinary confirmed
change permits a synthesizing Agent to preserve, omit or reinterpret; only then can
it choose an eligible temporary author, bounded admission, visible time,
timeout/fallback and cooldown.

The analyses converged on these current constraints, not yet product choices:

- one temporary author is unnecessary for solo work and many non-collision
  scenarios, but at least one explicitly invoked Agent must write any genuinely new
  semantic synthesis;
- an open MCP call is only an opportunistic delivery route, never durable authority
  or correctness state;
- letting several Agents synthesize in parallel adds token spend and redundancy but
  gives semantically blind World no principled quality choice;
- one content-blind chosen eligible Agent is therefore the smallest live creative
  candidate, while progress needs an independently predetermined original fallback;
- no bounded design can understand all ten thousand contributions, give them equal
  semantic influence and retain constant latency and token work; fairness can cover
  only content-blind opportunity inside bounded admission; and
- ordinary exact contribution authority, pre-existing broad authority and explicit
  collective authority are materially different. Selection may use only authority
  that already existed before it.

The User then corrected a hidden semantic assumption in the authority question:
World cannot determine that `blue`, `upside down` or an engraving is an independently
permitted part of a contribution. Only the source Agent can submit an exact bounded
write or mark several exact writes as inseparable; World may store that package and
later compare identities, hashes and exact writes, but may not derive the package
from meaning.

The User accepted the technical boundary that survives that correction. Every
changed fact in a synthesized final state must be covered by at least one
structurally checkable source that predates settlement: an exact write copied
unchanged from a source Agent's authorized package, the selected author's own
pre-existing authority over that fact, or a pre-existing collective authorization.
The selected Agent alone decides which contributions make semantic sense and
authors the complete final state. World checks only coverage, current versions,
bounds and atomic settlement.
It cannot check that the result fairly represents an omitted or cited contribution.
One ordinary source request remains one indivisible contribution: another Agent may
include it unchanged but may not borrow its authority for a reinterpretation.
Provenance can establish who supplied or authored an exact write; it cannot
establish semantic fidelity. Selection grants an Agent a bounded opportunity to
answer, never additional mutation authority.

This direction does not yet choose whether ordinary collision permits omission or
whether explicit collective work later requires ratification. It does remove the
impossible option in which a semantically blind World discovers allowed contribution
parts itself.

The User also made the accepted GX tradeoff explicit: meaningful collision can make
an Entity interaction take longer, and that delay is valuable only when an Agent
turns it into understandable multiplayer play such as noticing that many people are
trying to affect the same thing. This creates concrete risks that remain unresolved:

- observing concurrency adds some delay even to an otherwise solitary change;
- Agent-authored synthesis adds seconds and can fail or disconnect;
- a continuously popular subject can starve, accumulate unbounded pending work or
  remain unavailable unless intake and settlement have immutable bounds;
- late arrivals, bounded admission and fallback can feel arbitrary unless the Agent
  explains what happened in World terms;
- cooldown can protect a settled result from thrashing but also postpones legitimate
  follow-up play; and
- intermediate narration is useful only if the BYO host can actually surface it
  during the same explicit invocation.

These are design costs of the intended experience, not later infrastructure details.
The User accepted that one collision gets a fixed intake cutoff and a hard
settlement deadline that new arrivals cannot extend. After settlement, every
involved Entity receives a bounded cooldown so the new state cannot immediately be
changed again. Neither phase holds a database transaction or pauses unrelated
subjects. A state-changing attempt during cooldown is not queued for later
execution: World returns current settled state and remaining time immediately as a
normal outcome, while reads and unrelated play continue. This prevents stale intent
and unbounded future work on one hot Entity.

Exact durations, participant admission, late-arrival behavior and fallback remain
unaccepted. The hard deadline now requires a deterministic outcome even when the
selected Agent disconnects, times out or returns an invalid final state. Root
recommends selecting one already valid complete source contribution through a
content-blind, auditable rule fixed when the collision closes. That fallback must be
independent of the Agent selected to synthesize, so stalling cannot make its own
contribution win. Leaving state unchanged would be safe but would turn a visible
multiplayer moment into effort with no World consequence.

The User then challenged whether that deadline would unnecessarily constrain the
creative Agent. Root's refined recommendation separates normal completion from
orphan protection without adding an Agent-managed extension protocol:

- the Agent's valid answer settles immediately whenever it arrives; World never
  waits out the maximum merely for pacing;
- the fixed maximum is materially longer than the expected Agent response and exists
  only so a closed application, lost connection or stalled model cannot leave the
  Entity pending forever;
- World stores the immutable expiry with the collision but holds no transaction,
  process-local lease or connection;
- the Agent sends no progress heartbeat and cannot extend the expiry, because either
  would add ceremony and permit an effectively unbounded hotspot; and
- after expiry, the already prepared non-semantic fallback can settle without any
  Agent inference. A later access can perform this deterministically if no prompt
  indexed cleanup has done so.

The exact maximum must be measured with real BYO-host and model latency; it is not
chosen as a preference. The intended fun comes from the ordinary case resolving as
soon as one active Agent has narrated and authored the shared result. The maximum is
only the technical escape hatch, and the complete collision path should remain rare
enough that it creates a multiplayer moment instead of making ordinary play slow.

The User then corrected the stored unit itself: World need not and cannot store a
semantic `collision`. It receives one or more complete Agent submissions. One may
settle directly. Several can be compatible, contradictory or merely simultaneous;
only an active Agent can understand which description applies. The multiplayer
moment is therefore an Agent-authored presentation of several requests, not a
World-classified object.

The unaccepted working phrase `World request` distinguishes this pre-settlement
input from current canonical terms. An Action and Interaction are accepted game
operations, an Investigation is already one specialized request with its own durable
attempt provenance, and Activity is immutable history after an accepted
state-changing operation. `Change request` is too narrow for an Investigation,
directed Interaction or another operation whose primary meaning is not a Property or
Trait mutation. One common lifecycle also need not mean one generic result payload
or one permanent table: while unresolved input may need temporary durable storage,
each accepted capability retains its own simplest typed result and history.

The User accepted that the common lifecycle covers every bounded gameplay submission
that can produce a durable World result, while pure `get`/`list` reads, delivery
hints and private Agent reasoning remain immediate and outside it. This accepts the
operation boundary, not `World request` as canonical vocabulary or one generic
storage model.

The User then asked whether a host can follow relevant World context in realtime
before its Agent knows which Entity matters, and whether this can avoid World having
to push a new turn into one selected Agent. Current sourced research and a fresh
primary-source verification establish four distinct seams:

1. MCP `2026-07-28` lets a connected client open one long-lived
   `subscriptions/listen` response stream for exact resource URIs. A
   `notifications/resources/updated` message carries only the changed URI, after
   which the client reads the authoritative resource again.
2. When a current Place exists, one exact Place resource can provide coarse
   discovery without an Agent knowing a new Entity id in advance. It cannot be the
   universal anchor because valid play may have no current Place. After an
   explicitly invoked Agent decides an Entity matters, the host can also follow that
   exact Entity resource. Core MCP has no wildcard or semantic-location subscription.
3. The MCP host subscribes; the LLM does not. A notification can invalidate cached
   context or become visible host UI, but the protocol neither starts a model turn
   nor proves that an Agent perceived or understood anything. Codex documents
   Streamable HTTP tool connectivity but currently makes no public resource-listen
   or notification-presentation guarantee.
4. A selected submitting Agent need not receive a later push. World can keep the
   short intake inside the Agent's already active tool call and return the bounded
   multi-request input to the selected caller. Other connected hosts may optionally
   receive a Place-, Entity- or own-request invalidation and refetch; correctness and
   settlement never depend on that hint.

This yields one strong unaccepted transport shape rather than a second World system:

```text
durable request/state/Activity in World
             -> changed exact resource key
             -> optional coalesced MCP invalidation to connected hosts
             -> authorized bounded resource read
             -> optional later explicit Agent reasoning
```

The invalidation layer may lose or collapse messages, stores no observer receipt and
contains no authoritative game content. Reconnect always re-establishes the exact
watch set and rereads current state. At scale, Place and Entity resource keys route
only to interested connection cohorts; one million listeners still imply up to one
million tiny network deliveries but only one World mutation and no model calls. An
internal database notification or later replaceable broker can wake the server
instances holding streams without becoming truth. Raw WebSocket, webhook and a
second event-log contract are not earned by this requirement.

The User chose realtime attention as the default multiplayer experience for a
compatible connected host during explicitly active play, then corrected the first
Place-centric formulation: current Place is not always present and therefore cannot
define the mechanism. When a Place exists it remains one concrete candidate for
coarse discovery of unknown Entities and activity. An explicitly active Agent may
also focus a bounded set of exact Entities and the User's unresolved submissions.
Which exact structurally grounded representation or bounded set covers both Place
and Place-less play remains open and cannot be chosen by the Multiplayer work ahead
of the separate spatial design. Authorization applies when listening and on every
refetch; a URI or hint never grants knowledge or mutation authority.

“Default” describes host attention, not an automatic Agent turn. A notification may
update host UI or retained context, while only an explicit User invocation spends
tokens or lets an Agent reason. A host that cannot listen remains correct by reading
authoritative current state at each explicit turn, but has reduced realtime GX; that
capability difference must eventually be visible rather than silently hidden. Exact
resource bodies, Place privacy, movement replacement, watch limits and host behavior
remain open. In particular, current Codex support must be tested directly because
its public documentation does not promise resource-listen presentation.

This is still an open grill, not an accepted finalizer, fallback, exact timing state
machine or universal request term. The retained
[`exact-resource subscription` lab](../../lab/multiplayer/06-place-resource-subscription/README.md)
used a Place only as one concrete fixture. Its token-free official-SDK path proved
exact listen, content-free invalidation, authoritative reread and reconnect recovery.
Its sole guarded `codex-cli 0.149.0` run then refuted the pinned host path: Codex
connected and listed tools but never listed, read or followed the resource. The
controller correctly kept the Table brown at revision `1`; the Agent's blue sentence
merely repeated the expected output supplied in its prompt and was not World-grounded.

This rejects standard MCP resources as an assumed universal default carrier for the
tested host. It does not reject realtime attention as intended GX, the MCP protocol
primitive or other BYO hosts. The next transport question is now concrete: whether
an application-owned connection can present multiplayer activity to the player and
whether a widely supported bounded MCP tool can return one eligible change inside
an already explicit Agent invocation. Neither implementation is chosen, and the
Place fixture still cannot choose the eventual structural attention anchor.

## Active realtime presentation is host-independent

The User accepted the correction to the earlier browser-only interpretation.
Aicadia is deployed on its own domain, but active play may happen through a ChatGPT
App, a Claude-like App, a terminal or a browser. The deployed Aicadia service
therefore owns one realtime distribution capability; any compatible active client
or host may keep an authenticated connection and present eligible concurrent
activity immediately. Browser play is one adapter, not the owner or required client.

This does not turn the client, host or connection into World authority or an Agent
runtime. BYO Agents continue to use MCP, only explicit User invocation may spend
tokens, and delivery may never start or resume a model. An already active Agent may
use a bounded waiting MCP tool backed by the same distribution path. A compatible
host UI may independently keep presenting live game state without invoking the
Agent. A terminal may do the same through a long-running client process. When no
compatible process is active, the Agent learns current truth on its next explicit
call.

Connection state is transient. Closing, suspending or disconnecting a client loses
delivery hints and is repaired by reconnecting and rereading, never by per-recipient
World replay. The public transport, host adapter contract, message shape,
authentication, watch authorization, exact structural watch set, backpressure
limits and internal broker remain open. Standard MCP resource subscriptions remain
optional host capability rather than the universal contract. This correction
chooses neither WebSocket nor SSE and introduces no current client implementation.

The strongest working transport candidate is one bounded waiting request rather
than required ambient push. A compatible active client supplies a bounded set of
structural subjects, the versions it already knows and a maximum wait. The service
returns immediately when any version is already different; otherwise transient
gateway state waits for a commit hint or the deadline, returns only which subjects
may be stale and stores no recipient history. The client then performs ordinary
authorized reads. A UI or terminal can repeat this request without model work; one
explicitly active Agent may make the same request through MCP once or repeatedly
within its User-owned run. Register-then-recheck plus current-version comparison
must make a concurrent commit unmissable without a global cursor or replay log.

This candidate gives HTTP clients and MCP Agents one semantic input, output and
recovery contract even if a later high-capability adapter uses WebSocket framing.
The retained [host-independent change-wait lab](../../lab/multiplayer/07-host-independent-change-wait/README.md)
supports its register-then-recheck behavior, reconnect, coalescing, HTTP and official
MCP adapters, real browser path and `10,000` local waiters with zero automatic model
calls. Its sole pinned Codex run is inconclusive before the wait: the first read
required an exact subject that no tool first made discoverable, so the Agent made
583 failed guesses and World correctly remained unchanged. The result adds one hard
Agent-tool constraint: every accepted wait subject and known version must come from
a preceding authorized World result; an opaque identifier may never be a circular
prerequisite for discovering itself. The actual Codex bounded-wait continuation and
production transport remain unproven and unaccepted.

## Current communication candidate: Agents write; delivery points

Realtime delivery is not itself Agent communication. The content through which
Agents communicate is one or more bounded, attributable Agent submissions and later
Activities stored once by World. Delivery only tells an interested active client
that its previously read bounded view may now be stale.

Concretely, one Agent may submit a complete blue Table state and another a complete
engraved Table state, each naming the exact current Entity and structural facts on
which it relies. World may temporarily retain both through the common lifecycle and
route their availability by the exact subjects the Agents declared. World knows
that both named the same Entity; it does not know whether colour and engraving are
compatible, contradictory or meaningfully related.

An interested active client receives a content-free hint, rereads the current Table
and the bounded submissions or Activities it is eligible to see, and gives those
exact facts to its explicitly invoked User-owned Agent. That Agent may understand
their relationship and author another complete Table state while citing the exact
inputs it used. World validates identities, authority, versions, bounds, provenance,
deadline and final structure; it never understands or certifies the semantic
combination.

This is indirect, sparse communication through shared World records, not a direct
Agent chat network:

```text
Agent A writes one attributable contribution
        -> World stores it once and marks an exact bounded view stale
        -> interested active clients reread
        -> Agent B understands what it read and may write a new contribution
```

A browser, App component or terminal process may keep the bounded wait active
without invoking a model. An already explicitly active MCP Agent may wait inside
that User-owned invocation. An inactive Agent receives no turn; it reads the same
current state and authorized bounded work on its next explicit invocation. These
hosts differ in immediacy, not in World truth or settlement authority.

At scale, one contribution is stored once rather than copied per observer, hints may
coalesce, and only clients interested in exact declared subjects need attention.
One famous Entity still requires network work proportional to the clients actually
watching it, but not a durable delivery row, model call or semantic comparison for
each observer. The local `10,000`-waiter lab supports only the in-process algorithm;
multi-instance routing, authorization-aware rereads and deployed fan-out remain
unproven.

This shape still does not choose who may author the final result, exact admission,
the settlement fallback, late-arrival behavior, timing, final cooldown duration or
the representation that covers play with and without a current Place. It also does
not prove that Sol Medium, Sol Low, Luna or another smaller model can follow the
interface. A comprehensible interface must return every accepted subject identity
and known version from the preceding authorized read, require only unchanged
structured copying into the wait, and return an actionable non-enumerating error
when that basis is absent.

The strongest next evidence candidate is one communication-only cross-host lab: one
real Codex CLI Agent and one real Claude Code Agent each submit a different complete
change for the same Table, the shared delivery mechanism marks their bounded views
stale, and each independently rereads and accurately presents the other's stored
contribution. A browser or terminal may observe the same facts without a model. The
experiment deliberately excludes final-author selection and settlement so a failure
cannot be hidden by another unresolved mechanic. This is a proposed evidence edge,
not an accepted experiment, public operation or product behavior.

> **Superseded on 2026-08-21:** the section below corrects the core direction. The
> cross-host wait lab and `wait_for_change` are no longer the next evidence edge; a
> participant's own submit call is its wait, and a separate wait serves bystanders.

## Corrected core direction: one tick-based system for every manipulation

On 2026-08-21 the User corrected the direction above after reviewing the complete
Multiplayer trail. Aicadia's one manipulation system must work the way an MMO server
works, with one deliberate substitution.

### How an MMO does it, and what transfers

1. Clients send only inputs — "flip the table", "cast a fireball at the table" —
   never state, dependencies or concurrency policy. Every input uses the same path
   whether one or forty players act.
2. One authoritative server collects every input for one part of the world inside a
   short tick and resolves them together with the installed rules. One input is an
   ordinary tick with one input.
3. The server result is the only truth; arrival order and client speed decide
   nothing.
4. Interest management sends the new state only to clients that are nearby or
   watching.
5. Clients render what the server says.

Points 1, 3, 4 and 5 transfer directly. Point 2 transfers as a short per-subject
tick for every request, including a solo one. With LLM-speed inputs a meaningful
tick is seconds rather than milliseconds, which disappears inside an Agent turn.

### The substitution: the Agent is the rule engine

World deliberately has no physics, combat or semantic rules; it does not know that
wood burns. World may therefore resolve a tick alone only when it holds one input:
it applies the Agent-authored result. When several inputs in one tick touch the
same subject, World chooses one still-connected requesting Agent, content-blind,
and hands it the whole set — current state plus every request with its requester.
That Agent authors the outcome ("the table flies burning through the inn") and
submits it in one follow-up call. World validates structure, authority and
versions, commits state plus one Activity that attributes every contribution,
returns the outcome to every call that was still open, and starts the Entity
cooldown. This chosen-Agent step is the core substitute for an MMO rule engine and
belongs to the one system, not to a separate collision feature.

### Confirmed direction (User, 2026-08-21)

- One powerful system handles one request and many simultaneous requests through
  exactly the same path: request → per-subject tick → apply or chosen-Agent
  resolution → authoritative state and Activity → interest-based notice → Agents
  narrate.
- Every request enters the tick, also a solo one. "The first lands and only later
  arrivals collide" is rejected as the default; the tick is the MMO way and costs
  nothing perceptible inside an Agent turn.
- The request is the Agent's best-grounded package of the player's wish: the exact
  subject, the intended change and the state it was read on, built from ordinary
  MCP context reads. It carries nothing about other players — no preservation
  conditions, no declared causal dependencies, no combinability or indivisibility
  markings, no mandates, bids or intent announcements. The submitting Agent never
  reasons about concurrency; only the chosen Agent receives everything about others.
- The chosen resolver is one of the requesting Agents that is still connected;
  selection is content-blind; the already accepted deterministic fallback applies
  when it fails. Its input is the complete set of that tick; its output is one
  complete final state.
- After resolution every involved Entity rests for a bounded cooldown so the
  outcome cannot immediately be rewritten and the system never becomes ceremonial.
  Attempts during cooldown return current state and remaining time, as already
  accepted.
- Aicadia runs in many hosts — Claude Code, Codex CLI, ChatGPT-like Apps, OpenCode
  and others. The core path therefore relies only on the smallest common capability:
  one tool call that may stay open for the tick and resolution, and one follow-up
  call. Resources, progress notifications, elicitation, App UI and bounded waits are
  accelerators for hosts that have them, never requirements. A participant's own
  call is its wait; a bystander's wait is a separate optional path.

### Corrections to earlier recorded direction

- "Complete state plus exact declared read dependencies per submission", including
  the three blank-slate designs' convergence on Agent-declared read dependencies, is
  superseded for ordinary requests. Exact subjects, facts and versions remain what
  World derives from the request and validates; the Agent declares no dependency
  policy. An undeclared causal read (a stone rolled because a door was open) is
  either ordinary World movement or, when the player cares, part of the request
  itself; no declaration system is added beside it.
- The "first lands free, later arrivals collide" variant discussed on 2026-08-21 is
  rejected in favour of the tick for every request.
- The bounded `wait_for_change` candidate and the communication-only Codex–Claude
  Code cross-host wait lab are no longer the next evidence edge.

### Working terms

`tick`, `resolver` and `set` are working descriptions in this record. They are not
canonical vocabulary, public capability names or schema; `dev/CONTEXT.md` is
unchanged.

### Open decisions

- What a tick technically is in Aicadia: where pending requests live without
  process-local state (pending rows with an immutable cutoff that any World
  instance can settle is the boring candidate), how calls held on different
  instances are released at settlement, and how exact subject scope is derived from
  a request.
- Tick length and cooldown length: player-measured game choices, not database
  settings. A tick must be long enough that "everyone engraves at once" actually
  meets and short enough that a solo request still feels immediate.
- How the set reaches the chosen Agent on each host: returned in the still-open
  submit call followed by one resolving call is the working shape; whether every
  target host sustains that as one fluent turn is unproven.
- What the other participants and bystanders see during the tick and resolution on
  hosts with and without progress or UI capability.
- Whether a tick may become a bounded gathering place or discussion-like surface
  for Agents where non-requesting Agents can learn about and respond to a pending
  manipulation — without reintroducing all-to-all chat, listener authority or
  server-invoked Agents. The User sees this as a direction to explore; nothing is
  accepted.
- How Agents learn that a tick exists for a subject and hook into it from any host.
- The shape of the chosen Agent's output (one complete state is the working shape)
  and how World attributes every contribution in one Activity.
- Whether World-level rules ("fire no longer burns wood") can be ordinary versioned
  World content changed through the same request path by a sufficiently
  authorized User, carried as bounded context in every read and every resolver
  set, and checked for freshness by the rule version a request read — so every
  Agent on every host applies a changed rule on its next call without server
  semantics or push. The User raised this on 2026-08-21 as later administrative
  functionality worth shaping now; a server that itself watches state and reacts
  remains the separate deferred branch of previously Agent-authored deterministic
  behavior. Nothing is accepted.

### Working technical definition of a tick (User confirmed as clearer, 2026-08-21)

Unaccepted working definition, concrete enough to test against:

- **What lies together.** A request names the subjects its player wants to change.
  Requests arriving inside the window share one tick when their targets overlap
  structurally: the same subject, a target whose current Place is the other
  request's target, or a Position directly on the other request's target. World
  derives this from placement it already stores; prose, names and intentions play
  no role. A tick is therefore neither per Entity nor per Place. Place nesting is
  not followed by the merge and every tick has a participant ceiling, so a
  city-wide request cannot swallow every concurrent request in the city.
- **Who learns, in three rings.** Participants (requests in the same tick) learn
  through their own call result. Present Characters (current Place equals the
  affected Place, no request) are eligible: a host with a live layer shows it, and
  otherwise their Agent reads what happened here since its last call; that Agent
  tells its player and writes the consequences for its own Character and content
  as new requests. Everyone else learns nothing unless an explicit structural link
  says otherwise.
- **Consequences are distributed.** The chosen Agent authors the shared scene for
  the merged subjects it may change (ordinary jointly changeable World content and
  the contributions in the set, unchanged); it never authors another player's
  Character. Each present player's Agent authors that player's consequences when
  next invoked. Between the occurrence and that turn the Character is unchanged in
  World; nobody plays an Agent in the background.
- **Scale is lazy.** A city-scale occurrence is stored once with its compact
  structural scope plus whatever its author may change directly. Nobody resolves
  the city. Content becomes consistent where it is next touched: an Agent that
  later acts on a table reads the occurrence and decides the table is charred. This
  is the S10 separation between scoped occurrence and literal mass mutation.
- **Worked cases.** Simultaneous "flip the table in H" and "bomb on H" merge through
  T's Place; one chosen participant authors "the table flies burning through the
  room"; both calls return that story; present Nia's Agent narrates and writes her
  consequence on its next call. A bomb ten seconds after the flip opens a new tick
  on H alone; T is in cooldown and is narrated as not yet burning. A bomb on the
  city merges only requests targeting the city or things whose Place is the city.
- **What the chosen Agent receives and may do.** The tick's requests (requester,
  target, wish in the requester's words), bounded current context of the merged
  subjects, and a count when the set is capped. It may include or omit others'
  requests unchanged, author new facts only on content it may change, and returns
  one complete state per changed subject plus the story. It need not know who
  listens, who waits or how the tick works.
- **Still needed from the spatial design:** a structural "lies within the city" so
  city-scale presence can be determined; the tick itself does not wait for it.

### Next discoveries, in order

1. Whether a tool call can stay open for seconds on each target host and the turn
   continues afterwards — cheap to measure and the number everything depends on.
2. Whether "set returned in the open call, one follow-up call resolves" is one
   fluent turn on at least two real hosts, and whether the story both players
   receive is fun when judged blind (candidate 03 suggests yes).
3. Which tick and cooldown lengths feel right, measured with players.
4. How World holds a tick across instances in PostgreSQL.

No lab, model spend, public capability, schema or production change is authorized
by this correction; it changes the active direction and the next question only.
