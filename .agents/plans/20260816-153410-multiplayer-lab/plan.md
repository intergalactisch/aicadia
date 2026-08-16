---
status: active
created_at: "2026-08-16T15:34:10+02:00"
updated_at: "2026-08-16T21:13:21+02:00"
accepted_at: "2026-08-16T16:03:48+02:00"
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
| `.agents/plans/20260816-153410-multiplayer-lab/plan.md` | Active accepted plan. | Track accepted workflow and dependency state. | It does not govern game behavior. |
| `docs/README.md` | Defines general `lab/` as a development-side experimental home. | Preserve its separation from decision, research, runtime and evidence authorities. | Current truth remains in existing owning authorities. |
| `lab/README.md` | Defines the cross-topic lab charter and status vocabulary. | Keep decision ownership outside the lab as experiments are added. | The home can later contain tracks beyond multiplayer without changing its authority. |
| `lab/multiplayer/README.md` | Indexes the current multiplayer experiment track. | Add only earned experiments and bounded verdicts. | Rough code is allowed; each experiment has a bounded question and verdict. |
| `lab/multiplayer/<NN>-<slug>/` | Contains retained observation and subject-conflict experiments. | Add only decision-earned experiments and a concise record per experiment. | No runtime imports, production dependency or secret material. |
| `docs/concept/concurrency-and-world-dynamics.md` | Active exploration record. | Update confirmed direction and remaining frontier after material answers or findings. | It cannot override `docs/game/`. |
| `docs/concept/log/2026-08.md` | Records current multiplayer exploration. | Record crystallized accepted, rejected, deferred or corrected choices. | One choice is written fully in one owning authority. |
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
| T3 | in_progress | T2 | no | Resolve subject-scoped transaction, Agent-authored causality and the activation boundary for collective intent assembly; run focused semantic labs only if earned. | Active concept/log, `lab/multiplayer/02-*`, `lab/multiplayer/03-*`, `lab/multiplayer/04-*`, intent/deliberation research | Q5/Q6 select operation-scoped freshness and exact Property coordination; research defines the bounded Agent-package/strict-World seam, while activation and eligibility remain open. |
| T4 | pending | T3 | no | Resolve the required live board/resource-interest contract, recovery and Agent-knowledge promises; run direct host/MCP smokes only if earned. | Active concept/log, optional next numbered lab | Q7 permits per-resource coalescing; every supported active host must receive prompt live change signals and recover bounded authoritative truth after pressure, loss or reconnect without automatic Agent invocation. |
| T5 | pending | T4 | no | Resolve overload, fairness and honest scale evidence; run bounded hotspot simulation only if earned. | Active concept/log, optional next numbered lab | Hot-subject failure is isolated and every scale claim is explicitly bounded. |
| T6 | pending | T5 | no | Produce and present a decision-complete production build plan. | Future production plan, affected planning/docs surfaces | Terry review, no material open production question, explicit User acceptance requested separately. |

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

### T3 — Resolve transactions and causality

**Objective:** Exact dependency and conflict rules permit independent same-Place
actions while serializing or rejecting truly conflicting hot-Entity actions.

**Actions:**

1. Grill dependency declaration, expected state/absence, idempotency and atomic
   consequence choices one at a time.
2. If needed, model simultaneous stone, door and vegetation actions in memory before
   selecting the production contract.
3. Record which delayed consequence, if any, earns a domain-specific later action.

**Invariants:**

- No exclusive semantic Place-wide mutation lock or revision, global counter or
  blockchain consensus; compatible PostgreSQL integrity locks may still protect a
  referenced Place key.
- Accepted current state and its one Activity commit atomically.
- Retries do not duplicate accepted effects or Activity.

**Evidence:**

- The same fixtures produce deterministic accepted, stale, conflict and replay
  outcomes with the expected number of Activities.

**Stop conditions:**

- Stop if the candidate requires a generic event abstraction currently deferred by
  `docs/game/deferred.md`.

### T4 — Resolve delivery and Agent knowledge

**Objective:** One lightweight cross-host board/resource contract provides required
live multiplayer awareness and remains correct under loss, duplication, delay and
reconnect, without mistaking transport for World authority or Agent knowledge.

**Actions:**

1. Apply Q7's accepted prompt-live, per-resource-coalescible delivery guarantee.
2. Grill and compare global-board, Place discovery and exact Entity focus as views over
   one authoritative-read/required-live-interest substrate, not separate systems.
3. Prove how Claude Code, ChatGPT desktop, ChatGPT web and later hosts can maintain
   that live contract through native MCP listen or a semantically equivalent adapter.
4. Run a token-free Rust delivery simulation and a focused PostgreSQL listener lab
   only when their measured behavior can decide the scope or propagation shape.
5. Only if protocol or Agent comprehension remains uncertain, run the smallest direct
   host/MCP smoke with explicit pre-announced call and token bounds, authoritative
   readback and verified cleanup.
6. Record every real and simulated seam separately and limit deterministic, database,
   MCP, host and Agent verdicts to the exact implementation each layer exercises.

**Invariants:**

- Hints may coalesce or disappear; authoritative refetch establishes truth.
- A global logical board never requires one global invalidation, revision or cursor.
- No MCP notification or server event triggers an Agent model call.
- Commit ordering is not inferred from `(occurred_at, id)` without evidence.

**Evidence:**

- A client that misses or duplicates a hint converges through the accepted bounded
  read contract; any model claim is limited to the exact direct smoke.

**Stop conditions:**

- Stop before any unannounced model call, external persistent side effect or permanent
  runner/harness.

### T5 — Resolve overload and scale evidence

**Objective:** The accepted design states what happens when one Entity is extremely
hot, how quiet subjects remain usable and which bounded proof supports its first
production claim.

**Actions:**

1. Grill fairness, admission, backpressure, retry and degraded-observation behavior.
2. If needed, simulate one hot subject and many independent subjects locally using
   fixed bounded workloads.
3. Separate semantic correctness, local throughput, topology projection and actual
   production capacity claims.

**Invariants:**

- Overload is explicit and bounded; it never spreads via an unbounded queue.
- A hot Entity may serialize, but unrelated Entities and Places do not share its lock.
- No local benchmark is extrapolated into an unsupported million-user claim.

**Evidence:**

- The lab verdict reports workload, limits, results and non-claims; the production
  plan names the next stronger gate rather than pretending it has passed.

**Stop conditions:**

- Stop if meaningful evidence would require production infrastructure or material
  external spend not covered by a newly accepted plan.

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
   verdict; each documentation edit passes focused search/review.
2. **Contract:** Candidate semantics satisfy current World/Activity invariants and
   preserve HTTP/MCP parity requirements without prematurely changing them.
3. **Outcome:** The final production plan demonstrates, with concrete scenarios, how
   independent same-Place actions coexist and how a relevant stone-drop change can
   reach an attentive host and later Agent turn without becoming false authority.
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
