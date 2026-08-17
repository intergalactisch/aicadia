---
status: draft
created_at: "2026-08-16T15:34:10+02:00"
updated_at: "2026-08-17T11:47:38+02:00"
accepted_at: null
completed_at: null
---

# Grill- and experiment-grounded multiplayer foundation

> **Role / side:** current multiplayer decision-lab execution plan / development side.
> **Authority:** owns the bounded grill, lab experiments and synthesis needed to
> produce a decision-ready production build plan.
> **Excludes:** current game behavior, final multiplayer product semantics and
> production implementation; those remain governed by `docs/game/` and a later
> explicitly accepted production plan.

## Outcome

> **Planning reset, 2026-08-17:** the User rejected the mechanism-first
> Affordance/causal-runtime decomposition as too abstract and premature. T3–T6 are
> not executable or decision-ready. First resolve one flat game scene with the
> existing game vocabulary; then rewrite this draft before requesting acceptance.

Root and User resolve one smallest powerful **World change substrate** through which
every Entity creation or mutation is submitted, deterministically settled, recorded
as one Activity and exposed as dirty authoritative resources to active Agents. A
proposal or collective round may assemble the same change package when an explicit
authority requires it, but cannot become a parallel state or truth system.

When reasoning alone cannot settle a factual risk, one retained experiment in
`lab/multiplayer/` tests it. The first new experiment must prove or refute that one
bounded semantic kernel handles ordinary creation, Property/Trait change, compatible
and conflicting same-Entity work, nonexistent targets, explicit multi-Entity/Place
consequences, one scoped World effect, retry, Activity and dirty-resource derivation.
Only then may real PostgreSQL and MCP seams be tested.

The player value is one coherent persistent World in which a single Agent, a crowded
Place and a World-wide contextual change use the same intelligible change contract;
active Agents promptly learn that relevant authoritative context is stale without
hidden calls. The result is an accepted set of product decisions and a separate,
implementation-ready production plan for the smallest complete unified slice.

## Non-goals

- No production code, schema, migration, API, MCP capability or operational system is
  implemented under this plan.
- No claim that a local experiment proves million-user capacity.
- No generic event engine, event sourcing, global ledger, broker, CDC pipeline,
  actor framework or per-recipient World records are introduced speculatively.
- No independent Entity-mutation, listener, proposal, consensus or scoped-effect
  truth systems: components may differ operationally, but every accepted consequence
  resolves to the same change package, current state and Activity.
- No Place-wide exclusivity merely because one Character is interacting there.
- No server-side LLM call, background Agent invocation or unconscious token spend.
- No experiment is promoted directly into production code.
- No generic personal skill receives Aicadia-specific paths, authority names or lab
  status vocabulary.
- No unbounded experimentation, performance theatre or provider benchmark detached
  from a pending decision.

## Evidence and authorities

| Evidence or authority | Current fact | Consequence for this build |
| --- | --- | --- |
| `AGENTS.md` — Plan Before Build | Consequential docs, architecture and executable behavior require an accepted plan. | This plan stays `draft` until User acceptance; production requires its own later accepted plan. |
| `AGENTS.md` — Built For Massive Concurrency | Correctness state, reads, locks and revisions must be subject-scoped and bounded. | Every candidate is tested for hot-subject behavior and absence of global coordination. |
| `AGENTS.md` — Dumb And Strict Server / No Unconscious Token Burn | The server validates deterministically and never invokes Agents. | A subscription may wake a host, but never means an LLM has processed an event. |
| `AGENTS.md` — Every World Action Leaves History | An accepted mutation writes current state and one durable, queryable footprint atomically. | Experiments distinguish authoritative Activity from disposable delivery. |
| `docs/game/domain.md`, `docs/game/protocol.md`, `docs/game/deferred.md` | Current World behavior and explicit deferrals remain authoritative. | Lab findings cannot silently expand the game contract. |
| `docs/research/massively-concurrent-dynamic-world.md` | Current research separates bounded mutation, idempotency, overload and partitioning concerns. | Grill questions must turn those constraints into concrete player semantics. |
| `docs/research/multiplayer-concurrency-and-world-observation.md` | MMO and MCP evidence supports one Activity, typed bounded observation, disposable wake-up and authoritative refetch; dense hotspots retain hard ceilings. | The plan tests the smallest transaction/observation kernel instead of promising magical fanout. |
| `docs/research/mcp-subscriptions-and-collective-agent-intents.md` | MCP resource subscriptions give already-live hosts lossy invalidation plus refetch, not replay, Agent invocation or governance; current Aicadia exposes no resources. | Treat a stable local proposal-board resource as a later protocol experiment, never as World truth or automatic participation. |
| `docs/research/multi-agent-deliberation-and-consensus.md` | Bounded diverse drafts may improve some tasks, while wrong convergence, correlated errors, problem drift, adversarial persuasion, prompt infection and quadratic all-to-all context remain real. | Compare sealed independent packages with at most one critique round; never make conversational consensus or eloquence authoritative. |
| `docs/research/agent-authored-world-intents.md` | Intelligent clients can declare exact targets/dependencies while authority validates structure; multi-Place semantic completeness cannot be proved without a generic predicate, capability or ratification. | Require each claimed scope to have an explicit standing and test A/B/C omission without mandatory Entity Properties. |
| `docs/research/unified-world-change-system.md` | Transaction-data, conflict-range, watch/refetch, MCP, PostgreSQL and subject-routing evidence supports one bounded package that produces state, Activity and dirty resources; huge contextual change can be one scoped effect while literal million-Entity rewrite cannot become free. | Test one semantic kernel before separately choosing transaction, listener or collective mechanisms; keep transport replaceable and truth singular. |
| `docs/research/realtime-agent-subscription-transports.md` | Explicit-turn bounded MCP reads are the portable recovery floor; MCP listen, raw SSE, WebSocket, webhook and vendor channels do not prove uniform host surfacing or model reaction. | Require authoritative baseline/refetch plus a separately proven live-interest path for every fully supported active host; never equate delivery with model invocation. |
| `docs/research/postgres-change-propagation-and-fanout.md` | Agents must never listen to PostgreSQL directly; compact lossy post-commit hints can reach a bounded gateway fleet while authoritative reads recover every gap. | Test `NOTIFY` plus one listener per gateway before earning an outbox, CDC or broker, and never create per-recipient durable delivery state. |
| `docs/research/entity-place-interest-subscriptions-at-scale.md` | A global board, Place interest and exact Entity watches have different discovery and fan-out costs; the strongest small candidate is bounded World-board pull plus Place discovery and Agent-selected exact focus. | Compare scopes over one read/listen substrate and reject a live globally invalidated board or hot global revision. |
| `docs/concept/concurrency-and-world-dynamics.md` | An active exploration record already separates confirmed direction from open choices. | Each resolved grill answer updates this single record rather than creating parallel truth. |
| User direction, 2026-08-16 | Grill questions are asked exactly one per turn; small targeted Agent/MCP experiments are permitted; the retained home is `lab/multiplayer`. | The workflow alternates decisions and earned experiments and never dumps a question frontier on the User. |
| User clarification, 2026-08-16 | The lab is worth preserving; “throw away code” means highly experimental code that need not be production-perfect. | Lab artifacts are retained by default with visible status, but have no production authority or promotion path. |
| User methodology direction, 2026-08-16 | Repository docs and logs must carry every material grill/research choice; lab artifacts remain separate experimental realizations. | Encode the compact rule in `AGENTS.md` and the reusable workflow in only the relevant skills before continuing the grill. |
| Read-only methodology audit, 2026-08-16 | `grilling` mandates the whole frontier, `prototype` mandates off-main capture/direct folding and `build-aicadia` does not yet require rereading the repository trail before each next question. | Correct the conflicting generic defaults without copying Aicadia's placement map into global skills. |
| Local Rust/Postgres baseline, 2026-08-16 | 152 tests pass and one deliberate server test remains ignored; current concurrency tests prove contextual writers wait on the shared Place lock. | The current Place-wide mutation lane is executable evidence, not merely an inferred risk or a throughput result. |
| `lab/multiplayer/02-subject-conflict` | Ten dependency-free Rust interleavings preserve exact Activity and retry outcomes without a Place revision. | Operation-scoped freshness is semantically representable; actual PostgreSQL concurrency and the product contract still need separate evidence and choice. |
| `lab/multiplayer/03-postgres-subject-conflict` | Five real SQLx/Postgres tests support exact Entity/current/absence isolation but refute total Place-row independence: an honest placement foreign key takes `KEY SHARE` and conflicts with an old `FOR UPDATE` Place writer. | Keep gameplay conflict scope distinct from integrity lock modes and require production migration sequencing before claiming independent introductions. |
| `lab/multiplayer/04-postgres-conflict-strategies` | Ten focused Postgres tests refute hybrid current-row/Entity-fallback ordering for mixed present/absent requests; exact Property slots pass the exercised matrix; one `SERIALIZABLE` case remains partial. | Grill whether exact hot-Entity parallelism earns a persistent absent-capable slot before selecting dependency representation or production transactions. |
| MCP/runtime feasibility audit, 2026-08-16 | Current Aicadia MCP is stateless and exposes no listen, subscribe or notification capability. | Do not relabel a simulated host hint as a live MCP test; defer the three-call presentation smoke until the catch-up contract is chosen. |
| User intelligence-ownership correction, 2026-08-16 | World must stay semantically dumb; Agents own meaning and bounded call assembly, including claimed causal and multi-Place scope, and may supply collective judgment. | Research MCP transport, Agent deliberation and structurally verifiable intent scope before choosing a proposal/consensus kernel; never add mandatory pseudo-physics Properties or server inference. |
| User cross-host and board direction, 2026-08-16 | The same game must be reachable from Claude Code, ChatGPT desktop, ChatGPT web and later Agentic apps; one global starting board is a first-class perspective alongside exact and local interest. | Choose the portable semantic promise before transport, compare all credible scopes and do not assume one host's push extension is universal. |
| User unification correction, 2026-08-17 | Prior experiments assumed several systems; all Entity creation/change, concurrency, scoped change and live Agent awareness must instead share one core system, with new tools permitted where necessary. | Return this plan to `draft`, supersede the listener-scope-first route and prove the unified package/resource invariant before more mechanism-specific labs. |
| User compact-reach and sensory correction, 2026-08-17 | A large occurrence may use one bounded World-evaluated reach without enumerating every matched house or inventing a district/Area; in-reach receiving Agents interpret Agent-authored hearing conditions against their own Character Property/Trait context. | Model one compact structural reach and receiving-Agent no-op semantics; do not make World infer hearing or materialize occurrence rows per Place/Character. |
| User causal-propagation correction, 2026-08-17 | Arbitrary Agent-authored Relations cannot drive deterministic triggers, and one initiating Agent may not be forced to author every receiving-side outcome as a final Effect. | Do not select a replacement mechanism yet; resolve ownership of immediate and later consequences in one concrete game scene first. |
| User flattening reset, 2026-08-17 | The Affordance/causal-runtime proposal was too abstract, too broad and insufficiently collaborative. | Withdraw those candidate mechanics, keep this plan non-executable and ask exactly one concrete gameplay question at a time before rewriting it. |

## Alignment

### Strategic

This work addresses the next concrete risk behind Aicadia's shared-world promise:
multiple Characters must be able to discover and enrich the same World without a
single Character freezing a Place or a popular subject collapsing correctness. The
lab keeps the initial system small while forcing semantic choices before costly
infrastructure. The next risk after this plan is production implementation of the
first accepted slice, not further open-ended architecture exploration.

### Tactical

The completed presence, attention and exact-fact work remains evidence. The revised
decision workflow follows four frontiers in order:

1. one unified Agent-authored change package and deterministic World settlement;
2. current state, one Activity and mechanically derived dirty-resource views from
   that same settlement, including a bounded scoped World effect;
3. real PostgreSQL conflict/atomicity and post-commit delivery under hot-subject,
   loss, reconnect and slow-consumer pressure; and
4. the smallest production tool surface and scale claim worth proving.

Root asks exactly one grill question per User turn and gives a concrete, reasoned
recommendation before asking the User to decide. Every recommendation passes Terry:
name the highest-value player or World outcome, check it against the current contract
and actor/action/state/ownership vocabulary, select the smallest safe system and name
the evidence that could prove or refute it. An experiment runs only when a specific
unresolved fact can materially alter the next decision. Each experiment records its
question, hypothesis, fixture, scope limit, result, falsifier, verdict and status.
The User decides product meaning; an experiment supplies evidence only.

Repository records, not conversation memory, carry the design forward. Before Root
asks the next grill question, the full current choice and open branch are updated in
the active concept record and the material decision is appended to the period log.
Sourced factual work lives in `docs/research/` and its index; only later accepted
current behavior enters `docs/game/`. Lab artifacts link to those authorities but do
not own or restate the decisions they test.

### Technical

`lab/` is Aicadia's general development-side, non-authoritative experimental home;
multiplayer is only its first current track. `lab/README.md` defines the shared rules,
while `lab/multiplayer/README.md` indexes retained multiplayer experiments. Default
experiments are token-free and in-memory. A disposable database or existing World
may be used only for one bounded question with verified isolation and cleanup. A
direct Agent/MCP smoke may use the smallest number of calls needed—normally one,
never more than three without revising this plan—and Root states the exact call and
spend boundary in commentary before it runs.

Experiments may be rough, but must be legible and reproducible enough to support
their verdict. Retention does not make them supported software. Positive logic is
redesigned against production invariants later; files are never copied or promoted
as the production implementation. No secrets, private prompts or uncontrolled model
output are retained.

Aicadia technical lab experiments default to standalone Rust crates because the
runtime is Rust and concurrency, ordering and integration results should exercise
the closest honest implementation medium. Interactive HTML/JavaScript is reserved
for a question whose evidence is human visual or semantic inspection. Keep one
canonical implementation per experiment: a viewer may consume recorded output but
must not duplicate its state machine.

The new semantic lab must have exactly one package type and one settlement function.
Creation, current-state change, scoped effect, Activity and dirty resources may use
concrete typed variants, but never independent commit paths. A later PostgreSQL,
MCP, `NOTIFY`, CDC or subject-router fixture is an implementation of this same
change-resource contract and cannot redefine game truth.

## Decisions, assumptions and open questions

### Confirmed decisions

- Ask one grill question per turn — User explicitly rejected batch grilling.
- Preserve `lab/multiplayer` as a development research workbench — User selected the
  path and clarified retention.
- Define `lab/` broadly enough for later experimental tracks — User clarified that
  multiplayer is the current use, not the eventual full scope of the lab.
- Treat code inside the lab as highly experimental — it may remain rough and has no
  production authority.
- Use an experiment only to resolve a concrete factual uncertainty — product meaning
  remains a User decision.
- Give Root's Terry-grounded preference with every grill question before the User
  chooses — User explicitly required an argued recommendation, not neutral options.
- Permit bounded subagent use across every plan task — User explicitly authorized
  delegation for research, critique and experiments; Root retains integration and
  the final evidence claim.
- Keep Agent effort explicit and User-owned — a realtime hint never triggers an LLM.
- Keep authoritative state and Activity separate from delivery hints and Agent
  knowledge — supported by current research and the dumb-server constitution.
- Keep a non-abandoned player-controlled Character persistently placed across User
  disconnects, but grant no personal sight or hearing without active attention —
  User accepted the Terry recommendation for Q1.
- Preserve an abandoned Character's World identity and permit a later NPC lifecycle
  instead of making abandonment erase it — User direction; exact transition and NPC
  behavior remain undecided and authorize no current background simulation.
- Define active attention through an explicit current-Character observation
  subscription after an authoritative baseline, ending on unsubscribe, disconnect,
  Character switch or Place departure — User accepted the Terry recommendation for
  Q2; hints and bounded host buffering never invoke an LLM or create World attention
  state.
- Let a later Character at the exact Place retrieve a bounded public occurrence as
  Place history while denying any claim that it personally saw or heard it — User
  accepted the Terry recommendation for Q3; private or targeted Interactions remain
  excluded and public meaning is never inferred from prose.
- Use exact-Place `public local` as the first live eligibility rule with strict
  ownership: concrete operation and World own audience and authorization, Activity
  owns one occurrence/history, host transport owns subscription/hints/buffer, Agent
  owns sensory presentation only and User owns every invocation — User accepted the
  Terry-corrected hybrid for Q4; typed channels wait for a concrete mechanic.
- Use repository authorities instead of conversational memory — every grill choice
  updates the active concept and concept log before the next question; research and
  current behavior retain their separate homes, while lab owns experiments only.
- Separate deterministic correctness evidence from real Agent/MCP evidence in both
  lab and build methodology — User explicitly confirmed this after the first lab;
  every experiment names its real and simulated seams, and a model smoke proves only
  the behavior it directly observes, never World authorization or privacy by itself.
- Default technical Aicadia labs to Rust and carry forward their invariants, tests
  and measurements rather than copying experimental code — User corrected the first
  lab's implementation medium after clarifying that Python was only the skill
  validator; the existing JavaScript state model is migrated instead of duplicated.
- Make mutation freshness operation-scoped — User accepted multiplayer Q5: a
  concrete operation owns its bounded typed read/write dependency shape, the Agent
  returns expected typed facts it read and World adds mandatory invariants;
  unrelated same-Place change does not conflict, while a changed causal read does.
  Whole-Place freshness and write-set-only checking are rejected.
- Permit—not require—multiple lab setups and settings when the difference can alter
  the pending decision; keep one falsifiable setup when comparison would be ceremony
  or performance theatre — User clarified the retained lab's exploration freedom.
- Present every remaining grill option in two adjacent layers—understandable
  player/World impact and exact technical/scale consequence—and explicitly apply the
  million-User and hot-subject Terry gate — User corrected the first Q6 formulation;
  Q6 was accepted only after the corrected explanation.
- Permit collective Agent proposal or voting rounds only for World facts whose
  concrete operation explicitly grants communal authority — User selected Q6a
  option A; traffic, listener count and contention never activate governance, while
  eligibility, decision rule, `No Score Anywhere` impact and base coordination stay
  open.
- Coordinate concurrent Property work on the exact `(Entity, Property key)` fact,
  including expected absence, rather than the whole Entity — User selected Q6 option
  B; independent facts may progress concurrently, one exact hot fact stays serial
  with bounded overload, and no production schema or throughput claim is implied.
- Keep all semantic intelligence and bounded call assembly in explicitly invoked
  User-owned Agents — User made this a permanent build rule; World validates only
  structural identity, authority, placement/spatial relations, current versions,
  bounds, idempotency and atomic history, while collective semantic judgment must be
  expressible through Agent-facing capabilities rather than server inference.
- Require no ceremonial lifecycle Property on every Entity — a Tree need not have
  `form`, `exists`, `shape` or `state`; Agents name only actual or expected-absent
  facts used by their call and exact-slot coordination remains property-agnostic.
- Treat massively concurrent Agent-authored shared World change as the root problem
  — millions of Users concurrently discover, create, enrich and manipulate millions
  of Entities and linked Places while World remains coherent, bounded and dumb;
  authority, listeners, deliberation, slots, settlement and cooldown are downstream
  hypotheses to decompose and test rather than substitute core problems.
- During Action preparation, keep a bounded Agent-selected interest set of relevant
  Entity/Place resources and always pair it with commit-time authoritative version
  validation — User's first case subscribes to both Tree and current Place; a bomb
  call explicitly names every Agent-inferred affected Place, while World never
  derives semantic blast scope and notifications never invoke Agents.
- Require live resource-interest delivery for full active multiplayer — User rejected
  treating turn-time reads as an equivalent portable game. A bounded authoritative
  read remains mandatory for baseline and recovery, but a supported active host must
  also maintain a native MCP listen path or semantically equivalent live adapter;
  host compatibility may not be purchased by weakening the core game mechanic.
- Coalesce repeated live change signals per resource under pressure — User selected
  multiplayer Q7 option B. Normal operation remains promptly live, while a slow host
  may hold one pending stale signal and refetch current state plus bounded recent
  Activities instead of accumulating an unbounded per-Activity delivery backlog.
  World stores each Activity once and no Agent invocation follows from delivery.
- Use one unified World change substrate for every Entity create or change — User
  rejected the prior several-system decomposition. One bounded Agent-authored package
  must settle once into current state, one Activity and mechanically derived dirty
  resources; live reads/listens and optional collective assembly must refer to that
  same accepted package. New tools are permitted when they preserve this invariant.
- Treat huge contextual World change and literal mass rewrite differently — one
  structurally scoped effect may represent a World-wide condition through the same
  substrate, while millions of independent Entity fact rewrites remain proportional
  bounded work and cannot be presented as an instant atomic change.
- Let one compact structural reach cover many matched Places for occurrence and
  contextual delivery without enumerating or mutating each Place or requiring a
  containing district/Area. Within the
  authorized scope, the receiving Agent—not World—interprets an Agent-authored
  sensory condition against its Character's authoritative Property/Trait context and
  may silently do nothing.

### Reversible assumptions

- Lab artifacts are retained by default as `active`, `kept`, `superseded` or
  `discarded`; status describes evidentiary usefulness, not runtime support.
- A standalone dependency-free Rust crate is the default first technical fixture;
  switch to Postgres, MCP or an Agent only when that real seam is the uncertainty.
- The first three semantic questions should normally precede an experiment, but Root
  may run one earlier when a factual uncertainty otherwise makes the next question
  misleading.
- Local concurrency simulation establishes semantic and boundedness properties only;
  it cannot establish production throughput or million-user capacity.

### Open questions

- Which smallest concrete operation catalog proves the unified change package without
  creating a generic event or consequence engine?
- What exact structurally scoped effect can represent “everything is blue” while
  World remains semantically dumb and Entity-owned Properties remain truthful?
- What smallest compact reach grammar covers exact Place and a bounded
  World-evaluated structural selection without a mandatory district/Area, unbounded
  dynamic selector or per-Place occurrence rows?
- Which occurrence-time or read-time Character Property/Trait context must a
  receiving Agent use when interpreting a sensory condition?
- Should Agents receive one union-shaped `submit_change` tool or a few concrete tools
  that compile into the same internal package and settlement engine?
- When a player abandons a Character, what exact control transition and later
  deterministic or Agent-owned NPC behavior lets it continue without a server LLM?
- How does an intent name the exact state it depends upon, including expected absence,
  without turning a Place into the conflict unit?
- Which consequences must be atomic with the action, and which may become later
  domain-specific actions with causal references?
- Does gameplay require lossless ordered occurrence catch-up, or only authoritative
  current state plus bounded recent context?
- What promise does subscription make: coalescible change hint, typed occurrence,
  cursor progression, or some combination?
- Are the coarse Entity/Place resources that should trigger a refetch distinct from
  the exact facts whose changed versions must reject an Action at commit?
- What overload behavior is fair and legible when one Entity becomes hot while quiet
  subjects remain available?
- When operations touch different Properties but share a causal fact, what sequential
  outcomes remain valid and is stabilization a generic cooldown, a concrete mechanic
  state or only a collective-settlement rule?
- Since World cannot infer semantic destruction from user-authored Entity/Property
  content, may an Agent voluntarily declare a reactable proposed Action, and which
  explicit structural or communal authority—if any—can require that path?
- How can eligible nearby Agents assemble or challenge one bounded consequence—such
  as an explosion explicitly affecting Places A and B—through current MCP-style
  calls, hints and authoritative refetch without automatic model invocation,
  unbounded debate or World semantic inference?
- After decomposing the root concurrent World-change problem, where does a
  structurally checkable standing or settlement basis belong among its downstream
  correctness, authorship and gameplay concerns?
- Are short Agent proposal/vote rounds a special authority for explicitly communal
  Properties, and if so who may participate without turning listener count, token
  spend or a million votes into the new correctness bottleneck?
- What smallest production slice and load/failure evidence justify the first scale
  claim?

## Implementation map

| Surface | Current state | Intended change | Invariants |
| --- | --- | --- | --- |
| `.agents/plans/20260816-153410-multiplayer-lab/plan.md` | Draft revision awaiting acceptance after the unified-system correction. | Track the unified package, PostgreSQL, delivery and synthesis dependency state. | It does not govern game behavior. |
| `docs/README.md` | Defines general `lab/` as a development-side experimental home. | Preserve its separation from decision, research, runtime and evidence authorities. | Current truth remains in existing owning authorities. |
| `lab/README.md` | Defines the cross-topic lab charter and status vocabulary. | Keep decision ownership outside the lab as experiments are added. | The home can later contain tracks beyond multiplayer without changing its authority. |
| `lab/multiplayer/README.md` | Indexes the current multiplayer experiment track. | Add only earned experiments and bounded verdicts. | Rough code is allowed; each experiment has a bounded question and verdict. |
| `lab/multiplayer/<NN>-<slug>/` | Contains retained observation and subject-conflict experiments. | Add only decision-earned experiments and a concise record per experiment. | No runtime imports, production dependency or secret material. |
| `docs/concept/concurrency-and-world-dynamics.md` | Active exploration record. | Update confirmed direction and remaining frontier after material answers or findings. | It cannot override `docs/game/`. |
| `docs/concept/log/2026-08.md` | Records current multiplayer exploration. | Record crystallized accepted, rejected, deferred or corrected choices. | One choice is written fully in one owning authority. |
| `docs/research/unified-world-change-system.md` | Primary-source synthesis for one change-resource substrate. | Constrain the unified semantic, PostgreSQL and delivery experiments without selecting current behavior. | Research does not establish a product decision. |
| `AGENTS.md` | Requires a trail but does not yet say that repository state replaces conversation memory between grill questions or place retained lab evidence. | Add one compact cross-task repository-memory and lab-authority rule under `Every Choice Leaves A Trail`. | Keep volatile multiplayer choices out of the always-loaded constitution. |
| `.agents/skills/build-aicadia/SKILL.md` | Records choices but has a stale log destination and no explicit reread-before-next-question sequence. | Make repository reread, per-answer recording, User cadence and lab separation procedural. | Project placement remains owned by `docs/README.md` and `AGENTS.md`. |
| `/Users/sanderjansma/.agents/skills/grilling/SKILL.md` | Defaults unconditionally to the whole frontier and unconditional factual delegation. | Respect explicit round-size overrides and project-owned repository trails; guard delegation by current authority. | Stay project-generic and keep User decisions distinct from facts. |
| `/Users/sanderjansma/.agents/skills/prototype/SKILL.md` | Treats throwaway as mandatory off-main disposal and says to fold a validated decision into real code. | Define throwaway as experimental quality/no direct promotion and allow a project-designated retained lab. | A retained prototype remains unsupported evidence and never owns a choice. |
| Future `.agents/plans/<timestamp>-multiplayer-foundation/plan.md` | Absent. | Synthesize the accepted production outcome, tasks and exact evidence claim. | It remains draft until separately accepted by User. |

## Execution contract

Root owns the question order, scope, experiment gates, plan state, documentation
alignment and final evidence claim. The User authorizes subagents across every task.
Root may delegate a bounded dependency-ready research, adversarial critique or
experiment task whenever it improves decision quality or latency; delegation is not
ceremony and does not replace Root judgment. Each subagent must read this plan, work
only in its assigned surface and return raw evidence. Root reviews and integrates
every result. Parallel work is allowed only for independent factual questions and
never replaces the one-at-a-time User grill.

Every experiment record must contain:

- the pending decision and one falsifiable question;
- the smallest fixture and explicit exclusions;
- time, request, connection and optional token/cost bounds;
- the observed result and what would falsify it;
- a verdict of `supported`, `refuted` or `inconclusive`;
- an artifact status of `active`, `kept`, `superseded` or `discarded`;
- the exact downstream question or plan change it informs.

## Task graph

Allowed states are `pending`, `in_progress`, `completed` and `blocked`.

| ID | State | Depends | Parallel-safe | Objective | Owned surfaces | Evidence |
| --- | --- | --- | --- | --- | --- | --- |
| T1 | completed | — | no | Establish the general retained lab home and its first multiplayer track. | `docs/README.md`, `lab/README.md`, `lab/multiplayer/README.md`, active concept/log | Placement review and `git diff --check`. |
| T1M | completed | T1 | no | Make repository-backed decision memory, retained experimental separation, honest evidence layers and project-language technical labs reusable build methodology. | `AGENTS.md`, `build-aicadia`, global `grilling`, global `prototype`, `lab/README.md`, concept log | Both changed skills validate; project-language and no-direct-promotion wording is consistent across the full referenced procedure. |
| T2 | completed | T1, T1M | no | Resolve presence, attention and sensory semantics one question per turn; keep the earned observation lab in the accepted technical medium. | Active concept/log, `lab/multiplayer/01-observation-ownership` | Q1–Q4 are accepted; one dependency-free Rust fixture passes all eight ownership scenarios with no real-seam overclaim. |
| T3 | pending | T2 | no | Prove or refute one unified semantic change kernel across create, change, concurrency, scoped effect, Activity and dirty-resource scenarios. | Active concept/log, `docs/research/unified-world-change-system.md`, `lab/multiplayer/05-unified-change-kernel` | One package type and settlement function pass the fixed scenario matrix with exact accepted/rejected outcomes, Activity counts and dirty resources. |
| T4 | pending | T3 | no | Prove the unified package's exact PostgreSQL atomicity, hot-subject isolation and bounded overload behavior. | Active concept/log, `lab/multiplayer/06-postgres-unified-change` | Real scratch Postgres preserves temp-reference resolution, exact conflict coordinates, Activity/resource edges and quiet-subject availability under a hot control. |
| T5 | pending | T4 | no | Prove post-commit dirty-resource routing, coalescing, refetch and supported-host delivery without creating another truth system. | Active concept/log, `lab/multiplayer/07-change-delivery`, optional bounded MCP/host smoke | Loss, duplication, reconnect and slow consumers converge through authoritative resources; every real/simulated seam and scale non-claim is explicit. |
| T6 | pending | T5 | no | Resolve the public tool/scope-effect boundary and produce a decision-complete unified production build plan. | Future production plan, affected planning/docs surfaces | Terry review, complete scenario mapping, explicit bounds and separate User acceptance requested before production. |

## Task details

### T1 — Establish the retained lab and multiplayer track

**Objective:** `lab/` has one explicit general non-authoritative role and
`lab/multiplayer` has a durable index for bounded multiplayer evidence.

**Actions:**

1. Add the general `lab/` placement rule to `docs/README.md` without moving current
   truth.
2. Create `lab/README.md` with the shared experiment contract, status vocabulary and
   promotion prohibition.
3. Create `lab/multiplayer/README.md` as the first track with its initial empty index.
4. Record the User's scope and retention clarifications in the active concept/log
   surfaces.

**Invariants:**

- Lab code is never imported by production or presented as supported behavior.
- Retained does not mean polished, authoritative or production-ready.
- Existing multiplayer research and unrelated worktree edits remain intact.

**Evidence:**

- `rg -n "lab/multiplayer|non-authoritative|superseded" docs/README.md lab/README.md lab/multiplayer/README.md docs/concept` — the placement and choice are traceable.
- `git diff --check` — edited tracked files contain no whitespace errors.

**Stop conditions:**

- Stop if the documentation constitution has a conflicting existing home or if
  establishing the lab would change runtime/package discovery.

### T1M — Make repository memory reusable methodology

**Objective:** Future Aicadia grills and research-led decisions resume from their
repository trail, record every material answer before the next question and keep
retained experiments separate from choices and production.

**Actions:**

1. Strengthen `AGENTS.md` under `Every Choice Leaves A Trail`: reread the owning
   repository trail on resume; update active concept and period log before the next
   material grill question; keep concept, log, research, game and lab roles distinct.
2. Make `build-aicadia` execute that sequence, fix the current period-log destination
   and respect an explicit one-question-per-turn User cadence.
3. Make global `grilling` preserve its whole-frontier default while allowing an
   explicit one-question round, project-designated durable trails and authorized-only
   delegation.
4. Make global `prototype` treat throwaway as experimental quality and no direct
   promotion, while allowing a project-designated retained lab with explicit status.
5. Append the accepted methodology change to the current concept log without
   duplicating the full placement constitution.
6. Require every lab and build evidence claim to name which seams are real versus
   simulated, and keep deterministic authorization/correctness claims separate from
   direct Agent/MCP comprehension claims.

**Invariants:**

- `docs/README.md` remains Aicadia's binding detailed placement authority.
- Global skills contain no Aicadia-specific paths, domain terms or lab statuses.
- Material choices, sourced findings, current contracts and experimental verdicts
  stay in different owning homes.
- Retained prototype code is never imported, copied or promoted into production;
  validated behavior is redesigned under normal build rules.
- An Agent or MCP smoke cannot establish authorization, privacy, delivery or scale
  properties outside the exact real seam it exercises.

**Evidence:**

- `python /Users/sanderjansma/.codex/skills/.system/skill-creator/scripts/quick_validate.py <skill-dir>` for `build-aicadia`, `grilling` and `prototype` — all three skill packages validate.
- `rg -n "conversation memory|exactly one question|project-designated.*lab|never own.*choice|promot" AGENTS.md .agents/skills/build-aicadia/SKILL.md /Users/sanderjansma/.agents/skills/grilling/SKILL.md /Users/sanderjansma/.agents/skills/prototype/SKILL.md` — each required boundary is present in its owning procedure.
- `rg -n "Ask the whole frontier|Fold any validated decision|docs/concept/log/README.md" .agents/skills/build-aicadia/SKILL.md /Users/sanderjansma/.agents/skills/grilling/SKILL.md /Users/sanderjansma/.agents/skills/prototype/SKILL.md` — no obsolete unconditional wording remains.
- `git diff --check` plus focused diff review — repository edits are clean and compact; personal skill edits pass their validators and a trailing-whitespace scan.

**Stop conditions:**

- Stop if the generic skills would need Aicadia-specific placement rules, if skill
  validation fails or if retained prototype wording could imply production support.

### T2 — Resolve perception semantics

**Objective:** A future builder can decide, at event time and read time, whether a
specific Character may know about one concrete stone-drop Activity.

**Actions:**

1. Grill exactly one material question per User turn.
2. Maintain confirmed direction and remaining choices in the active concept record.
3. If reasoning cannot settle late arrival, missed hint or offline-presence behavior,
   build one in-memory observation/catch-up model covering those cases.

**Invariants:**

- `available to host` is never equated with `processed by LLM`.
- One accepted action does not create per-recipient World history rows.

**Evidence:**

- Concrete actor/place/stone scenarios have unambiguous allowed and rejected outcomes.
- Any experiment record satisfies the execution contract and is indexed by the lab README.

**Stop conditions:**

- Stop and return to the User when a gameplay preference—not a technical fact—would
  decide the model.

### T3 — Prove one unified semantic change kernel

**Objective:** One dependency-free Rust model demonstrates that every target
scenario can use one package type and one settlement function without a parallel
create, listener, scope-effect or proposal truth path.

**Actions:**

1. Define one research-only bounded package with stable request identity, actor,
   concrete settlement basis, exact typed expectations, concrete typed mutations,
   optional occurrence/causal references and mechanically derived dirty resources.
2. Exercise one Agent create/enrich, missing target, concurrent blue/three-leg/
   overturned table, same-fact conflict, bomb-plus-table/window consequence, one
   compact occurrence scope covering many Places, receiving-Agent hearing/no-op
   interpretation, and World-scoped blue condition versus rejected unbounded Entity
   rewrite.
3. Verify equal retry, changed fingerprint, stale relevant fact, unrelated change,
   bounded busy and rollback outcomes with exact current state and Activity counts.
4. Grill only product semantics exposed by this one kernel, including whether the
   scoped-effect case is acceptable and which first concrete mutations belong in a
   production slice.

**Invariants:**

- One accepted package produces current state, exactly one Activity and one dirty
  resource set; zero accepts produce none of them.
- Agent reasoning supplies meaning and explicit effect scope; World validates only
  structural authority, existence, bounds, freshness and atomicity.
- A nonexistent target never becomes an implicit create.
- Huge contextual scope is one explicit scoped fact; literal mass rewrite is
  rejected as unbounded in this kernel.
- One compact scope may cover many structurally contained Places without one
  occurrence or dirty row per Place; every independently mutated Entity remains an
  explicit proportional write.
- World authorizes structural scope but never interprets hearing; the receiving
  Agent owns sensory presentation and may produce no response or mutation.

**Evidence:**

- Every fixed scenario has a deterministic allowed/rejected result, final state,
  Activity count and dirty-resource set in one retained Rust fixture.
- The record names all simulated seams and makes no database, MCP or scale claim.

**Stop conditions:**

- Stop if one scenario requires an unchosen product meaning, generic event/rule
  engine, server ontology or second authoritative commit path.

### T4 — Prove PostgreSQL settlement and overload

**Objective:** A scratch PostgreSQL realization preserves the unified kernel's
atomicity and exact conflict scope while a hot subject cannot starve quiet work.

**Actions:**

1. Model stable accepted request identity, temporary Entity resolution, exact current
   and absent coordinates, multi-coordinate stable lock order, current state, one
   Activity and Activity-to-resource edges in one transaction.
2. Run concurrent create, independent table-fact, same-fact, cross-fact dependency,
   explicit multi-Entity consequence and forced-failure interleavings.
3. Run a fixed hot-Property workload beside many quiet Properties with bounded pool,
   statement, lock-wait and retry limits; include one hot Place observation-index
   control without reintroducing a Place mutation lock.
4. Capture outcomes, p50/p95/p99, pool and lock waits, rows/buffers, WAL, dead tuples
   and zero-leak cleanup. Compare one alternative only when it can change the pending
   decision.

**Invariants:**

- A hot exact fact may serialize or return busy; unrelated facts do not share its
  semantic lock or unbounded wait queue.
- No global or Place-wide revision/counter is introduced.
- Current state, Activity, request result and resource edges commit or roll back
  together.
- No local throughput result becomes a hosted or million-Agent claim.

**Evidence:**

- Real SQLx/PostgreSQL tests and a bounded skewed workload establish only the
  exercised scratch schema, lock strategy and host.

**Stop conditions:**

- Stop before production migration, external infrastructure, a new database or a
  benchmark whose material cost has not been accepted.

### T5 — Prove unified dirty-resource delivery

**Objective:** The same Activity/resource edges produced by settlement drive prompt,
coalescible live awareness and authoritative recovery without becoming another truth
or per-recipient history system.

**Actions:**

1. Commit one opaque post-commit change identity, let bounded gateway listeners fetch
   its exact resource set and coalesce repeated dirtiness by URI.
2. Inject loss, duplication, delay, disconnect, slow consumer, gateway restart and a
   World-scope invalidation; verify listen-then-baseline and refetch recovery.
3. Measure PostgreSQL `NOTIFY` listener/write amplification and queue behavior. Compare
   one Core NATS subject-router variant only if the measured gateway shape can change
   the decision; never add JetStream or another durable history by default.
4. After the resource contract is fixed, prove the smallest native MCP
   `subscriptions/listen` or semantically equivalent host-adapter path for each
   supported host. Announce exact model calls and token bounds before any direct Agent
   smoke.
5. Keep host receipt, authorized refetch and actual LLM consumption as separate
   evidence claims.

**Invariants:**

- Hints may coalesce or disappear; bounded authoritative refetch establishes truth.
- No notification invokes an Agent, grants authority or leaks an unauthorized
  resource.
- One World-scope change avoids million-row mutation but does not pretend its roughly
  one-delivery-per-live-host network cost disappears.
- No global revision, cursor, durable subscription or recipient row is introduced.

**Evidence:**

- Every failure converges in the retained transport fixture; each PostgreSQL, NATS,
  MCP, host and Agent claim is limited to the exact real seam exercised.

**Stop conditions:**

- Stop before any unannounced model call, external persistent side effect, permanent
  runner, speculative broker or unsupported cross-host promise.

### T6 — Resolve the surface and synthesize the production plan

**Objective:** A separate draft plan defines the smallest complete unified slice,
including exact player behavior, scoped-effect boundary, public Agent tools, World
contract, implementation seams and honest evidence.

**Actions:**

1. Grill one question per turn to select the first concrete operation catalog,
   structurally scoped-effect semantics, settlement authority, public tool shape and
   exact overload/live bounds exposed by T3–T5.
2. Decide whether Agents receive one union-shaped `submit_change` or a few concrete
   capabilities that compile into the same internal package; require HTTP/MCP parity
   and remove superseded mutation paths in the eventual production change.
3. Reconcile accepted, rejected and deferred choices with `docs/game/`, research,
   concept/log and backlog authorities.
4. Create a production plan with dependency-ready World, PostgreSQL, transaction,
   Activity/resource, HTTP/MCP, test, documentation and operation tasks.
5. Present that plan and wait for explicit User acceptance before production work.

**Invariants:**

- One internal mutation path remains the architectural invariant even if concrete
  public tools are plural.
- Only accepted current behavior enters `docs/game/` and implementation scope.
- Experimental artifacts remain evidence and are never production dependencies.

**Evidence:**

- Terry's five questions and every User scenario have concrete answers; no material
  product or technical question is hidden in the task graph.

**Stop conditions:**

- Keep the production plan `draft` and resume the grill if any answer can change its
  actor, action, state, ownership, contract, cost or evidence claim.

## Validation ladder

1. **Focused:** Each experiment reproduces its exact fixture and records a bounded
   verdict; each documentation edit passes focused search/review.
2. **Contract:** Candidate semantics satisfy current World/Activity invariants and
   preserve HTTP/MCP parity requirements without prematurely changing them.
3. **Outcome:** The final production plan demonstrates, with every User scenario,
   how one package creates or changes Entities, resolves same-Entity concurrency,
   rejects missing targets, settles a bounded bomb consequence, represents or rejects
   mass change honestly and reaches active hosts through the same dirty resources.
4. **Integrity:** `git diff --check`, focused diff review, explicit review of retained
   lab status and confirmation that unrelated User changes remain intact.

## Change control

Refine question order, lab paths and stronger evidence in place while this accepted
workflow remains unchanged. Stop, set `status: draft`, revise and request explicit
re-acceptance if new evidence changes lab authority, permits automatic Agent spend,
adds material external cost, broadens from research to production implementation or
changes the promised final artifact.

## Completion conditions

- T1, T1M and T2–T6 are `completed` and the validation ladder passes;
- the one-at-a-time grill has resolved every material production choice;
- retained lab artifacts are indexed with explicit verdict and status;
- a separate decision-complete production plan has been presented for acceptance;
- current research, concept choices, vocabulary and backlog are aligned without
  claiming lab code as game behavior;
- `status: complete` and `completed_at` are recorded only after these conditions.
