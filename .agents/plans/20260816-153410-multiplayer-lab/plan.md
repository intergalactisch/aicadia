---
status: draft
created_at: "2026-08-16T15:34:10+02:00"
updated_at: "2026-08-17T08:57:09+02:00"
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

Root and User resolve Aicadia's smallest powerful multiplayer foundation one material
question at a time. When reasoning alone cannot settle a factual risk, a small
retained experiment in `lab/multiplayer/` tests that risk. The result is an explicitly
accepted set of product decisions and a separate, implementation-ready production
plan for one crowd-safe transaction-and-observation slice.

The player value is a persistent World in which many Characters may act at one Place
without globally locking it, while nearby Characters can obtain a bounded, grounded
account of relevant change without hidden Agent calls. The exact final evidence claim
is deliberately not fixed yet: the grill and experiments must determine which first
slice is both valuable and honestly provable.

## Non-goals

- No production code, schema, migration, API, MCP capability or operational system is
  implemented under this plan.
- No claim that a local experiment proves million-user capacity.
- No generic event engine, event sourcing, global ledger, broker, CDC pipeline,
  actor framework or per-recipient World records are introduced speculatively.
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
| `docs/research/realtime-agent-subscription-transports.md` | Explicit-turn bounded MCP reads are the portable cross-host floor; MCP listen, raw SSE, WebSocket, webhook and vendor channels do not prove uniform host surfacing or model reaction. | Make pull/refetch sufficient for play and treat push only as a host-proven latency accelerator. |
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

## Alignment

### Strategic

This work addresses the next concrete risk behind Aicadia's shared-world promise:
multiple Characters must be able to discover and enrich the same World without a
single Character freezing a Place or a popular subject collapsing correctness. The
lab keeps the initial system small while forcing semantic choices before costly
infrastructure. The next risk after this plan is production implementation of the
first accepted slice, not further open-ended architecture exploration.

### Tactical

The decision workflow follows five frontiers in order:

1. presence, attention and what a Character may fictionally perceive;
2. transaction dependencies, conflicts and same-Place parallelism;
3. immediate and delayed consequences and their Activity history;
4. catch-up, subscription hints, disconnects and authoritative refetch;
5. overload, fairness and the smallest production slice worth proving.

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
- Keep Root as the default executor; use subagents only when the User explicitly
  requests them for the current execution or current controlling instructions require
  them. Historical broad permission does not itself activate delegation; Root always
  retains integration and the final evidence claim.
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
- Pause the grill with Q8 undecided and compare all credible interest strategies in
  the retained lab — User requested real PostgreSQL and real MCP seams for every
  variant rather than selecting from explanation. The comparison includes global
  firehose, Place, exact-only, flat hybrid and structural World/area/Place/exact
  hybrid forms under one fixture and metric set.
- Keep a genuine World-scope change possible from the core without designing its
  caller — User expects a later admin to use this power but explicitly deferred all
  roles, rights and admission. The lab injects the change at its experimental World
  seam and tests only storage and delivery, exposing no player or admin tool.
- Close the two concrete interest-lab risks before returning to simultaneous Entity
  mutations — User explicitly accepted the proposed small follow-up first. It tests
  hot-resource versus quiet-resource router isolation and fatal PostgreSQL-listener
  recovery; only after that does T3 resume with concurrent requests against one
  Entity.
- Test one direct concurrent Entity-request kernel before proposal rounds, voting or
  cooldown — User accepted this next step on 2026-08-17. Agents own exact writes and
  causal dependencies; World owns only bounded structural validation, exact-slot
  coordination, idempotency, atomic current state plus Activity and explicit
  `accepted`, `conflict` or `busy` outcomes. Real PostgreSQL and rmcp seams are
  required; no production behavior is selected by the lab.
- Bind an ordinary uncontested Entity request to the whole coherent Entity snapshot,
  not to an Agent-selected subset of Properties — User selected Q9 option A on
  2026-08-17. The Agent interprets current Entity content but does not construct a
  semantic dependency list. If the Entity changes before settlement, the old basis
  is stale. The User further directed that genuinely concurrent requests should not
  yet be reduced to an arbitrary first-writer winner; deterministic detection and
  collective settlement are the next decision frontier.
- Pursue one universal globally persistent participation system rather than further
  mutation-path alternatives — User directed this after Q9. Entering a Place gives
  an active Agent a Place registration through which it follows nearby change;
  attempting an Entity change registers one bounded intent in that Place's shared
  coordination surface; concurrent intents settle within a fixed timeframe through
  Agent-owned consensus or voting before one World commit. A bounded trigger web
  carries declared effects to other Entity/Place scopes. MMO “instance” ideas may
  inform local coordination and routing, but never copy or fork Aicadia's World.
- Permit T3D to spend at most three pinned Agent process calls with zero retries and
  no enforceable token ceiling, only after T3A and T3C pass and Root announces the
  calls immediately before execution — User explicitly accepted the revised plan
  and this spend bound on 2026-08-17. A failed or malformed call remains evidence;
  it never earns a retry or a fourth call.

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
- The direct load fixture compares `NOWAIT` with one short transaction-local
  `lock_timeout`; these are reversible lab settings and neither becomes a production
  admission policy without the measured result and a later product decision.
- A fixed Tree has an actual `condition` Property only because that concrete fixture
  needs it; the experiment creates no universal lifecycle key or Entity ontology.

### Open questions

- When a player abandons a Character, what exact control transition and later
  deterministic or Agent-owned NPC behavior lets it continue without a server LLM?
- How is the accepted whole-Entity basis represented without making the Agent select
  Properties or introducing one globally shared revision outside that Entity?
- Which consequences must be atomic with the action, and which may become later
  domain-specific actions with causal references?
- What bounded recent Activity window and reconnect response best implement Q7's
  accepted current-state-plus-coalescible-live promise?
- Which global, Place, exact-only, flat-hybrid or structural-scope-hybrid interest
  strategy gives the best game coverage and bounded fan-out under the shared lab
  matrix, and does the structural lab candidate earn acceptance as current product
  behavior despite its still-unproved hosted-client and current-Gateway integration
  seams?
- Which later caller may invoke a World-scope change remains outside this experiment;
  the core must preserve the capability without prebuilding roles or rights.
- Are the coarse Entity/Place resources that should trigger a refetch distinct from
  the exact facts whose changed versions must reject an Action at commit?
- What overload behavior is fair and legible when one Entity becomes hot while quiet
  subjects remain available?
- What exact bounded rule makes two requests “concurrent” for one Entity: a window on
  every first request, only overlap observed before the first commit, or an adaptive
  contention mode after an initial collision? The rule must not require World to
  understand either request and must keep uncontested interaction fast.
- Is active Place registration transient delivery reachability layered on durable
  Character placement, or authoritative durable membership? This must be resolved
  before eligibility, trigger propagation or settlement can be specified without
  recipient rows, heartbeat write amplification or hidden offline participation.
- What bounded, versioned structural links form the trigger web, and how does an
  Agent declare an affected traversal without World inferring semantics or allowing
  an unbounded graph walk?
- Once registration and trigger reach are fixed, what one bounded settlement state
  machine handles one or thousands of intents without all-to-all Agent discussion,
  automatic token spend, Sybil authority or a hot global counter?
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
| `.agents/plans/20260816-153410-multiplayer-lab/plan.md` | Draft after the exact-fact fixture exposed the whole-Entity and participation-system decision. | Grill registration, trigger reach and settlement, then revise and request acceptance. | It does not govern game behavior. |
| `docs/README.md` | Defines general `lab/` as a development-side experimental home. | Preserve its separation from decision, research, runtime and evidence authorities. | Current truth remains in existing owning authorities. |
| `lab/README.md` | Defines the cross-topic lab charter and status vocabulary. | Keep decision ownership outside the lab as experiments are added. | The home can later contain tracks beyond multiplayer without changing its authority. |
| `lab/multiplayer/README.md` | Indexes the current multiplayer experiment track. | Add only earned experiments and bounded verdicts. | Rough code is allowed; each experiment has a bounded question and verdict. |
| `lab/multiplayer/<NN>-<slug>/` | Contains retained observation and subject-conflict experiments. | Add only decision-earned experiments and a concise record per experiment. | No runtime imports, production dependency or secret material. |
| `lab/multiplayer/05-postgres-mcp-interest-strategies/` | Retained kept experiment with an overall `inconclusive` verdict; structural interest is the only supported direct-tier candidate. | Preserve the real-seam, synthetic and focused follow-up evidence for the next product choice. | No production imports or schema; one shared implementation and fixture hold every variant comparable; actual and synthetic scale evidence stay distinct. |
| `lab/multiplayer/06-concurrent-entity-requests/` | Retained active exact-fact comparison: thirteen PostgreSQL tests pass, but the overall verdict is `inconclusive`. | Preserve its bounded evidence; do not start its superseded load, MCP or Agent tiers until the universal participation plan is redesigned and accepted. | No production import, Entity ontology, automatic Agent invocation or foundation claim. |
| `docs/concept/concurrency-and-world-dynamics.md` | Active exploration record. | Update confirmed direction and remaining frontier after material answers or findings. | It cannot override `docs/game/`. |
| `docs/concept/log/2026-08.md` | Records current multiplayer exploration. | Record crystallized accepted, rejected, deferred or corrected choices. | One choice is written fully in one owning authority. |
| `AGENTS.md` | Repository memory, retained-lab separation and exact evidence claims are current build rules. | No T3 change. | Keep volatile multiplayer choices out of the always-loaded constitution. |
| `.agents/skills/build-aicadia/SKILL.md` | Current workflow requires repository reread, plan acceptance and separated real/simulated evidence. | Apply it; do not modify it for T3. | Project placement remains owned by `docs/README.md` and `AGENTS.md`. |
| `/Users/sanderjansma/.agents/skills/grilling/SKILL.md` | Supports explicit one-question cadence and project-owned trails. | No T3 change. | Stay project-generic and keep User decisions distinct from facts. |
| `/Users/sanderjansma/.agents/skills/prototype/SKILL.md` | Supports retained project-language labs without direct promotion. | Apply its technical-concurrency branch; do not modify it for T3. | A retained prototype remains unsupported evidence and never owns a choice. |
| Future `.agents/plans/<timestamp>-multiplayer-foundation/plan.md` | Absent. | Synthesize the accepted production outcome, tasks and exact evidence claim. | It remains draft until separately accepted by User. |

## Execution contract

Root owns the question order, scope, experiment gates, plan state, documentation
alignment and final evidence claim. Root executes tasks directly unless the User
explicitly requests subagents for the current execution or current controlling
instructions require them. Any authorized subagent must read this plan, receive one
dependency-ready task and owned surface, and return raw evidence for Root review.
Parallel-safe labels express technical independence only; they do not authorize
delegation and never replace the one-at-a-time User grill.

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
| T3A | blocked | T2, T5 | no | Determine whether the passing exact-fact concurrent Entity-request kernel is the foundation or only one high-concurrency variant after the default snapshot-basis grill is resolved. | `lab/multiplayer/06-concurrent-entity-requests/{Cargo.toml,Cargo.lock,migration/,src/world.rs,tests/semantic_matrix.rs}` | The exact-fact matrix is retained; no broader foundation verdict or further tier begins until the reopened semantic-completeness choice is accepted. |
| T3B | pending | T3A | no | Compare `NOWAIT` and short `lock_timeout` admission under hot-fact, same-Entity/different-fact and quiet-Entity workloads. | `lab/multiplayer/06-concurrent-entity-requests/src/load.rs`, `tests/load_matrix.rs` | Exact result/Activity counts, held-lock isolation, query plans and descriptive latency/pool metrics share one fixed workload. |
| T3C | pending | T3A | no | Cross the accepted mutation and observation shape through real rmcp tools, resource listen, transactional PostgreSQL hint and authoritative read. | `lab/multiplayer/06-concurrent-entity-requests/src/mcp.rs`, `src/gateway.rs`, `tests/mcp_chain.rs` | Multiple real rmcp clients submit concurrent packages; coalesced updates and baseline read converge on committed state and Activity. |
| T3D | pending | T3A, T3C | no | Test whether real User-owned Agents author three semantically distinct bounded Tree dependency packages through the lab MCP read surface. | Bounded disposable Agent workspaces/evidence under experiment 06; no production or private prompt surface | At most three pinned calls, zero retries; validated packages and independent World readback separate comprehension from correctness. |
| T3E | pending | T3B, T3C, T3D | no | Adversarially review, record and index experiment 06 without selecting production behavior. | Experiment README, lab index, active concept/log, this plan | Verdict, real/simulated seams, falsifiers, exact metrics, cleanup and all nonclaims agree. |
| T4 | completed | T2 | no | Compare required live resource-interest strategies through one real PostgreSQL and MCP lab before asking Q8 again. | `lab/multiplayer/05-postgres-mcp-interest-strategies`, lab index, active concept/log, this plan | Five variants ran through the same real 32-client/two-gateway/100-commit seam; structural alone had no stipulated coverage miss or noise, while the separate million-record mapper and all nonclaims remain explicit. |
| T5 | completed | T4 | no | Close the small interest follow-up: isolate quiet resources from maximum local hot fan-out and prove fatal PostgreSQL-listener recovery through reconnect plus baseline. | `lab/multiplayer/05-postgres-mcp-interest-strategies`, lab index, active concept/log, this plan | Equal-capacity 1/8/64-stripe variants preserved exact bounded routing; the 64-stripe candidate crossed real rmcp, while real fatal PgListener loss ended the stream and replacement listen-then-baseline recovered current state plus the missed Activity. |
| T6 | pending | T3E, T5 | no | Produce and present a decision-complete multiplayer build plan. | Future production plan, affected planning/docs surfaces | Terry review, no material open production question, explicit User acceptance requested separately. |

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

### T3A — Build the deterministic concurrent-request kernel

> **Current planning status:** the detailed T3A–T3E contract below is retained as the
> previously accepted exact-fact branch. Its T3A fixture ran; Q9 then refuted its
> unstated claim to be the default foundation. While this plan is `draft`, none of
> these remaining tasks is executable. Registration, trigger reach and the universal
> settlement lifecycle must replace or deliberately preserve each task before new
> acceptance.

**Objective:** One standalone Rust lab proves that Agent-authored exact writes and
dependencies can settle simultaneous requests against one Entity without making the
Entity or Place the universal conflict unit.

**Actions:**

1. Create `lab/multiplayer/06-concurrent-entity-requests/` with one scratch migration
   and one `World` seam. Store bounded Character placement, Entity, exact Property
   slot/current/history, accepted request identity and Activity state.
2. Define one bounded request package: request id and fingerprint; actor placement
   expectation; at most sixteen sorted `Current` or `Absent` exact dependencies; at
   most sixteen exact Property writes; 1–64-byte lower-snake-case keys; typed values
   with at most 4,096 stored bytes; at most 128 operation bytes; and at most 64 KiB
   serialized input. Agents choose the dependency set; World derives no semantic
   dependency from names, prose or values.
3. In one short transaction, establish the request-id coordinator, materialize and
   lock exact slots in canonical `(entity_id, property_key)` order, revalidate every
   submitted and mandatory structural dependency, apply all writes, append history
   and exactly one Activity, then persist the canonical accepted result. Rejection
   rolls the coordinator row back; equal accepted retries replay; changed
   fingerprints conflict.
4. Exercise the concrete Tree matrix with two actual Properties, `color` and
   `condition`: blue versus red on `color`; blue versus felled without a cross-fact
   dependency; blue-only-while-standing versus felled in both commit orders; two
   expected-absent writes; different Entities; moved actor; equal/changed retry; and
   injected failure after Activity insertion.
5. Prove rejected, stale and busy expected-absence requests leave no slot, accepted
   request or Activity residue. Report the persistent slot cost of accepted unique
   absent dependencies explicitly; do not relabel a per-request bound as a lifetime
   Entity-cardinality solution.
6. Add a 100,000-row adversarial history/slot fixture and `EXPLAIN (ANALYZE, BUFFERS)`
   gates proving each exact read and lock follows its bounded key/index path rather
   than scanning Entity or World history.

**Invariants:**

- The fixture Tree's `condition` is concrete content, not a mandatory lifecycle key;
  another Entity needs no `condition`, `form`, `shape`, `exists` or ontology.
- No exclusive Place/Entity mutation lock, global revision/counter, last-write-wins,
  CRDT or semantic server inference enters the kernel.
- All dependencies and writes are known and bounded before locking; one request
  cannot expand its lock set inside the transaction.
- One newly accepted request yields one complete current-state result and one
  Activity or nothing; retries never duplicate either.

**Evidence:**

- `DATABASE_URL=postgres://localhost/postgres cargo test --locked --manifest-path lab/multiplayer/06-concurrent-entity-requests/Cargo.toml --test semantic_matrix -- --test-threads=1` — every real concurrent interleaving, rollback and idempotency assertion has exact state and Activity counts.
- Retained adversarial query plans show work proportional to the package bounds and
  page limit, independent of 100,000 unrelated rows.

**Stop conditions:**

- Stop if correctness requires a generic event/proposal abstraction, a semantic
  Entity taxonomy, unbounded dependency discovery, a Place/Entity-wide coordinator
  or persistent rejected-attempt history.

### T3B — Compare bounded hot-subject admission

**Objective:** The lab distinguishes unavoidable serialization on one exact fact
from accidental pool or lock contention that delays different facts and quiet
Entities.

**Actions:**

1. Hold the kernel and fixed workload constant while comparing two transaction-local
   policies only: exact-slot `FOR UPDATE NOWAIT`, and exact-slot `FOR UPDATE` with a
   fixed 10 ms `lock_timeout`. `SKIP LOCKED` is excluded because canonical state may
   not be silently skipped.
2. Run seven isolated repeats through two independent `World` instances/pools against
   one database, each with a maximum sixteen connections and a controller-wide
   maximum of sixty-four in-flight requests: 4,096 attempts on one hot Tree fact;
   4,096 attempts distributed across sixty-four facts of that same Tree; and 4,096
   quiet requests distributed across independent Entities. Use the same fixed pool,
   task and request bounds for both policies.
3. Add deterministic held-lock tests proving a quiet Entity and a disjoint exact fact
   complete before the hot lock is released. Bound every hot attempt to
   `accepted`, `conflict` or `busy` inside a fixed outer timeout; `busy` writes no
   Activity and retry advice is bounded data, never an automatic retry loop.
4. Record accepted/conflict/busy/replay counts, exact Activities, pool-acquire and
   transaction p50/p95/p99/max, lock errors, quiet-control versus hot-load latency,
   database statements, connections and final pending work. Timing is descriptive;
   correctness and structural isolation gate any candidate preference.

**Invariants:**

- Both policies receive identical data, pools, workloads and result definitions.
- No process-local state decides correctness or requires gateway/server affinity.
- One million requests to one exact fact are acknowledged as one canonical lane;
  scalability means bounded shed/conflict behavior and quiet isolation, not one
  million simultaneous accepted truths.
- The fixture makes no million-user throughput or hosted-capacity claim.

**Evidence:**

- `cargo test --release --locked --manifest-path lab/multiplayer/06-concurrent-entity-requests/Cargo.toml --test load_matrix -- --ignored --nocapture --test-threads=1` — fixed workload, exact counts and descriptive comparison are emitted.
- Focused held-lock tests deterministically prove disjoint progress without relying
  on noisy percentile thresholds.

**Stop conditions:**

- Stop if the comparison changes semantics between policies, needs an unbounded
  retry/queue, exhausts machine resources outside the accepted fixed bounds or turns
  local timing into a production scale claim.

### T3C — Cross the kernel through real MCP and live interest

**Objective:** Multiple real rmcp clients can submit concurrent packages while
interested hosts receive only coalescible stale hints and converge through an
authoritative Entity read.

**Actions:**

1. Add the smallest test-only rmcp `2026-07-28` server over loopback Streamable HTTP:
   one bounded submit tool, exact Entity resource listen/read and the same semantic
   input/result contract as the lab `World`.
2. Enqueue one compact `pg_notify` naming the changed Entity resource inside the
   mutation transaction so PostgreSQL exposes it only after commit; one real
   `PgListener` routes the lossy hint to bounded transient subscribers. Never store a
   recipient row or call an Agent.
3. Use four rmcp mutation clients and eight subscribed clients for the Tree matrix.
   Assert exact tool outcomes, coalescing under repeated hot change, authoritative
   current state plus bounded recent Activity, disconnect/relisten/baseline recovery
   and zero pending teardown state.
4. Keep this implementation independent from experiment 05; reuse its proven
   invariants and scenarios, never its code as a production or lab dependency.

**Invariants:**

- MCP, PostgreSQL hints and resources are transport/read seams, never mutation
  authority, delivery replay or Agent knowledge.
- Listen happens before the recovery baseline; notification loss cannot lose
  accepted current state or Activity.
- Tool/resource payloads, subscriptions, pending keys and response bytes are hard
  bounded.

**Evidence:**

- `DATABASE_URL=postgres://localhost/postgres cargo test --locked --manifest-path lab/multiplayer/06-concurrent-entity-requests/Cargo.toml --test mcp_chain -- --test-threads=1` — real SQLx commit, `pg_notify`, `PgListener`, rmcp submit/listen/update/read and cleanup form one asserted chain.

**Stop conditions:**

- Stop if the seam needs production code, one database connection or durable row per
  subscriber, notification replay, automatic model invocation or an unbounded
  delivery queue.

### T3D — Run the bounded real-Agent dependency smoke

**Objective:** Real User-owned Agents can read the lab Entity and author three
different exact request packages whose dependencies match explicit User meaning.

**Actions:**

1. Only after T3A and T3C pass, announce and run at most three pinned Agent process
   calls, zero retries, isolated workspaces and one disposable owned lab database.
   Record model identity, actual usage when available and the absence of an
   enforceable token ceiling.
2. Each Agent may use only the lab MCP read surface and a strict output schema; no
   web, source, shell, database or fallback authority. The three instructions are:
   make the Tree blue only while standing; fell the Tree; and make it blue regardless
   of whether it is felled.
3. Validate the resulting packages independently. The first must depend on current
   `condition=standing` and write `color=blue`; the second must write the concrete
   `condition=felled` consequence while depending on its current standing value; the
   third must write blue without claiming the standing dependency. A deterministic
   controller submits the valid packages through the lab World/MCP seam and performs
   authoritative readback.

**Invariants:**

- Agent output can support comprehension and package-authorship claims only; it
  cannot prove transaction correctness, authorization, fairness or scale.
- No notification triggers an Agent and no failed call is retried or replaced.
- Model prose and private reasoning are not World history; only an accepted bounded
  request produces Activity.

**Evidence:**

- One retained manifest records the three-call maximum, actual calls, tool trace,
  strict package validation, authoritative outcomes and verified database/process/
  workspace cleanup.

**Stop conditions:**

- Stop before the first call unless this revised plan is explicitly accepted. Stop
  the tier as `inconclusive` on unavailable model/auth, schema failure, tool escape,
  missing cleanup ownership or any need for a retry/fourth call.

### T3E — Review and record the bounded verdict

**Objective:** Experiment 06 remains a reproducible lab answer with no borrowed
production, host, Agent or scale claim.

**Actions:**

1. Adversarially review lock ordering, expected absence, request-id races, rollback,
   pool isolation, query boundedness, MCP dirty-state races, cleanup and metric
   definitions against the live files and raw runs.
2. Record question, bounds, real/simulated seams, observations, falsifiers, verdict
   and `kept`/`discarded` status in the experiment README; index it in
   `lab/multiplayer/README.md` and update current concept/log plus this plan.
3. State the next product question explicitly: whether direct request evidence is
   sufficient or one separately authorized collective proposal/cooldown experiment
   is now earned. Do not answer it by benchmark score.

**Invariants:**

- Lab code is never imported, copied or promoted into production.
- `docs/game/`, runtime, backlog and capability surfaces remain unchanged because no
  production behavior is selected.
- Independent exact-fact settlement remains distinct from optional collective
  authority, discussion and cooldown.

**Evidence:**

- `cargo fmt --check`, `cargo check --tests --locked`, `cargo clippy --tests --locked -- -D warnings`, focused/debug tests, the isolated release matrix, exact SQLx cleanup audit, `git diff --check` and focused review all pass.

**Stop conditions:**

- Stop and return the plan to `draft` if evidence requires production changes,
  changes Agent/World ownership, broadens into proposal/voting/cooldown state or
  alters the accepted call/cost bound.

### T4 — Resolve delivery and Agent knowledge

**Objective:** One lightweight cross-host board/resource contract provides required
live multiplayer awareness and remains correct under loss, duplication, delay and
reconnect, without mistaking transport for World authority or Agent knowledge.

**Actions:**

1. Apply Q7's accepted prompt-live, per-resource-coalescible delivery guarantee.
2. Build one standalone Rust 2024 crate with a scratch migration, SQLx `0.8.6`, real
   transactions plus one Activity, post-commit `pg_notify`, a dedicated `PgListener`,
   `rmcp` `3.1.1` server/client over loopback Streamable HTTP, exact subscription
   acknowledgments, resource-update notifications and authoritative resource reads.
3. Hold implementation and workload constant while comparing five routing forms:
   global firehose, Place board, exact Entity only, Place plus exact Entity, and a
   declared World/area/Place scope chain plus exact Entities. The World-scope case is
   injected directly by the fixture; caller authority is deliberately absent.
4. Exercise the same scenarios for every form: selected Tree change, unselected local
   Entity change, new Entity discovery, bomb affecting Places A/B, regional effect,
   one world-wide effect, movement A→B, hot
   Entity burst, hot Place, quiet Place isolation, slow consumer, lost host
   subscription and reconnect/refetch. Fatal PostgreSQL-listener loss was deliberately
   left to and then closed by T5 rather than mixed into this comparison.
5. Use two evidence sizes. The direct integration tier uses one writer, two gateway
   listeners, 32 real MCP subscribers and 100 committed changes per strategy, run
   sequentially with a 30-second bound per strategy. The scale tier streams one
   million synthetic host-interest records and 10,000 fixed skewed changes through
   the same strategy key-selection code, folds the full population into cohort
   counts and retains a bounded executable recipient sample. It does not use one
   million Gateway records and claims no million sockets, database clients or
   supported Users.
6. Capture mutation and notification latency p50/p95/p99, database notification and
   query counts, `pg_notification_queue_usage`, accepted resource filters, raw and
   coalesced outbound hints, peak pending keys, refetch count/rows/bytes, irrelevant
   wake-ups, subscription churn and quiet-subject latency. Timing is descriptive;
   correctness, coverage and boundedness gate every comparison before speed.
7. Record a result matrix and explicit falsifiers in the experiment README, keep each
   real and simulated seam separate, index the active artifact and update the concept
   trail before Q8 resumes.
8. After scope evidence, separately prove how Claude Code, ChatGPT desktop, ChatGPT
   web and later hosts can maintain the chosen live contract; no lab SDK client result
   is relabeled as host support.

**Invariants:**

- Hints may coalesce or disappear; authoritative refetch establishes truth.
- No global logical board requires one globally mutated database row, revision or
  cursor; the firehose comparison may still route a logical Global invalidation.
- A World-scope effect updates one scope resource, not every Place row; different
  per-Place consequences remain unbounded and outside this experiment.
- World-scope caller authority is absent; the experiment may prove the core delivery
  shape but no role, right, admin operation or player permission.
- No MCP notification or server event triggers an Agent model call.
- Commit ordering is not inferred from `(occurred_at, id)` without evidence.

**Evidence:**

- `DATABASE_URL=postgres://localhost/postgres cargo test --manifest-path lab/multiplayer/05-postgres-mcp-interest-strategies/Cargo.toml -- --test-threads=1` — all real database/MCP scenario assertions pass.
- The experiment's release comparison command emits the complete fixed metric matrix
  for five strategies and distinguishes actual connections from the synthetic
  million-interest strategy-key/cohort workload.
- A separate ignored SQLx cleanup audit finds no registered database for the eight
  owned test paths; `cargo fmt --check`, `cargo check --tests` and `git diff --check`
  pass.

**Stop conditions:**

- Stop if a strategy cannot use the same real seam and fixture without changing the
  compared game promise, if the test needs production code/schema, or if an SDK
  client result would be mistaken for Claude/ChatGPT host support.
- Stop before any unannounced model call, remote service, role/right system or
  persistent external side effect.

### T5 — Resolve overload and scale evidence

**Objective:** The interest lab determines whether one maximally hot resource causes
head-of-line delay for unrelated quiet resources and proves that a fatal real
PostgreSQL listener loss becomes a visible reconnect-and-baseline condition rather
than a silently stale live stream.

**Actions:**

1. Extend experiment 05 rather than create a second copy of its World/MCP fixture.
2. Compare one, eight and sixty-four resource-lock stripes through one Rust router
   implementation and identical bounded registration, hot-fan-out and quiet-route
   workloads. Use 4,096 hot recipients, independent quiet resources, concurrent hot
   producers and repeated runs; record quiet p50/p95/p99/max, hot throughput,
   coalescing and exact delivery counts. The router capacity counts exact
   resource-recipient pairs, not the existing Gateway's separate host admission;
   timing remains descriptive.
3. Put the strongest bounded lock setting behind one minimal real rmcp
   `subscriptions/listen` → resource update → authoritative read smoke so the result
   proves protocol integration rather than only a fast in-memory router.
4. Run one real SQLx/PostgreSQL/rmcp failure scenario with a dedicated listener pool:
   establish listen plus baseline, close that pool so `PgListener::recv` fails,
   require the existing MCP subscription to end, commit while no listener exists,
   create a replacement gateway/listen and verify its authoritative baseline contains
   the missed committed Activity and current state.
5. Keep the World, Activity mapping, subscription bounds and no-Agent/no-token rule
   from experiment 05 unchanged; update only the experiment record, plan and concept
   trail with the bounded observation.
6. After this task completes, resume T3 with a separately planned experiment for
   simultaneous requests against one Entity, as the User explicitly requested.

**Invariants:**

- Overload is explicit and bounded; it never spreads via an unbounded queue.
- One hot resource may retain its unavoidable O(local subscribers) fan-out, but a
  candidate isolation design may not make an unrelated resource share that lock.
- Listener failure must never leave a healthy-looking active subscription that has
  silently stopped receiving hints.
- Reconnect follows listen-then-authoritative-baseline ordering; notification replay
  is neither required nor claimed.
- No production code, host product, Agent, remote service or role/right system enters
  the fixture.
- No local benchmark is extrapolated into an unsupported million-user claim.

**Evidence:**

- Focused correctness tests prove exact delivery/coalescing for every lock setting
  and bounded teardown state.
- An ignored release matrix prints the fixed workload and descriptive lock-wait and
  quiet-latency distributions for one, eight and sixty-four stripes.
- A real SQLx test proves listener failure → MCP stream end → offline commit → new
  listener/listen → authoritative recovery, followed by the explicit cleanup audit.
- A focused real rmcp smoke proves the strongest fixed-fixture lock setting preserves
  the exercised one-client listen/update/coalesce/read success path; it does not
  prove overload, failed-read restoration or Claude/ChatGPT host support.
- `cargo fmt --check`, `cargo check --tests`, `cargo clippy --tests -- -D warnings`,
  focused tests and `git diff --check` pass.

**Stop conditions:**

- Stop if resource sharding requires changing the World or subscription promise, if
  fatal-listener recovery cannot be surfaced through current rmcp semantics, or if
  meaningful evidence would require production infrastructure or external spend.

### T6 — Synthesize the production plan

**Objective:** A separate draft plan defines one smallest complete multiplayer slice
with exact player behavior, World contract, implementation seams and honest evidence.

**Actions:**

1. Reconcile all accepted, rejected and deferred choices with `docs/game/`, research,
   concept/log and backlog authorities.
2. Create a production plan with dependency-ready tasks for World, Postgres,
   transactions, Activity, HTTP/MCP parity, tests, docs and operations as applicable.
3. Present that plan and wait for explicit User acceptance before production work.

**Invariants:**

- Only current accepted behavior enters `docs/game/` and implementation scope.
- Experimental artifacts remain evidence, never production dependencies.

**Evidence:**

- Terry's five questions have concrete answers and no material product or technical
  question remains hidden in the task graph.

**Stop conditions:**

- Keep the production plan `draft` and resume the grill if any answer can change its
  actor, action, state, ownership, contract, cost or evidence claim.

## Validation ladder

1. **Focused:** Each experiment reproduces its exact fixture and records a bounded
   verdict; experiment 06 separately proves semantic, overload, MCP and Agent tiers;
   each documentation edit passes focused search/review.
2. **Contract:** Candidate semantics preserve exact Agent/World ownership, bounded
   request shape, atomic current state plus one Activity, durable idempotency and the
   accepted coalescible-hint/authoritative-read contract without changing
   `docs/game/` or production surfaces.
3. **Outcome:** The Tree scenarios demonstrate that concurrent same-fact requests
   conflict or shed boundedly, different facts can compose, an Agent-declared causal
   dependency changes the valid outcome, quiet Entities remain structurally
   independent and subscribed hosts converge on committed truth.
4. **Integrity:** `git diff --check`, focused diff review, explicit review of retained
   lab status and confirmation that unrelated User changes remain intact.

## Change control

Refine question order, lab paths and stronger evidence in place while this accepted
workflow remains unchanged. Stop, set `status: draft`, revise and request explicit
re-acceptance if new evidence changes lab authority, permits automatic Agent spend,
adds material external cost, broadens from research to production implementation or
changes the promised final artifact.

## Completion conditions

- T1, T1M, T2, T3A–T3E and T4–T6 are `completed` and the validation ladder passes;
- the one-at-a-time grill has resolved every material production choice;
- retained lab artifacts are indexed with explicit verdict and status;
- a separate decision-complete production plan has been presented for acceptance;
- current research, concept choices, vocabulary and backlog are aligned without
  claiming lab code as game behavior;
- `status: complete` and `completed_at` are recorded only after these conditions.
