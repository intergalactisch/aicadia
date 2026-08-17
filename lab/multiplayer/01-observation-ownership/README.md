---
question: Can Q1–Q4 remain one World occurrence with transient host attention and Agent presentation only after World authorization?
verdict: supported
status: kept
real_seam: [Rust compiler, test runner, committed in-memory state machine in src/lib.rs]
simulated_seam: [World, host, hint delivery, Agent state]
informs: .agents/plans/20260816-153410-multiplayer-lab/plan.md
---

# Observation ownership experiment

> **Role / side:** retained observation-ownership experiment / development side.
> **Authority:** records this fixture, its bounded observations, verdict and artifact status.
> **Excludes:** current game behavior, production ordering, delivery guarantees, scale proof and implementation design.

## Pending decision

Before the grill moves from accepted Q1–Q4 into catch-up guarantees, determine
whether their ownership split is internally representable without silently adding
durable observer state or Agent-owned access decisions.

This experiment informs the next observation and delivery question in the accepted
[`multiplayer-lab` plan](../../../.agents/plans/20260816-153410-multiplayer-lab/plan.md)
and links to the owning active exploration in
[`concurrency-and-world-dynamics.md`](../../../docs/concept/concurrency-and-world-dynamics.md).
It does not make either decision.

## Question

Can accepted Q1–Q4 be represented as bounded in-memory state with one World Activity
per occurrence, no durable attention, observation or sensory-channel rows,
World-owned operation audience plus exact-Place eligibility, host-owned
subscription, hints and buffer, and Agent knowledge only on an explicit User turn?

## Falsifiable hypothesis

One fixed host can complete every scenario test without:

- adding an Activity or observer copy per eligible Character;
- storing subscription, attention, personal perception or sensory channels inside
  World state;
- exposing a private `submit_interaction` to a same-Place bystander;
- trusting a hint as occurrence content; or
- incrementing Agent invocation state before an explicit User turn.

The hypothesis is false if any required outcome needs one of those behaviors, if a
lost or duplicate hint changes World truth, or if later public history must be
misrepresented as personal witnessing.

## Fixture

Run the standalone dependency-free Rust crate from the repository root:

```sh
cargo test --manifest-path lab/multiplayer/01-observation-ownership/Cargo.toml
```

[`src/lib.rs`](src/lib.rs) contains the one canonical in-memory transition model and
eight focused tests. Each failed assertion prints the full relevant fixture state.
The crate uses Rust edition 2024, is not publishable and is not part of the
production Cargo workspace.

The fixed fixture contains:

- two Places: Old Quarry and Quiet Grove;
- three Characters: Mara, Ivo and Nia;
- one Great Stone with `standing` or `fallen` current state;
- one host controlling either Ivo or Nia;
- `submit_action`, whose concrete contract is public at the exact Place;
- `submit_interaction`, whose concrete contract is limited to named participants at
  the exact Place; and
- a host buffer capped at three authorized context items.

Publicness is derived from the concrete operation contract. An Activity contains
operation, actor, Place and the participant roles needed by a private Interaction;
it has no generic user-authored audience or sensory-channel field.

Character placement changes are explicit fixture setup controls, not modeled
movement Actions. The integer `fixture_sequence` makes the tests legible only. It
makes no production ordering, cursor or lossless-catch-up recommendation.

## Exact bounds

- Token and model calls: zero.
- Persistence, database, network, MCP and external services: none.
- Runtime: one local Rust test process; each test owns a fresh in-memory fixture.
- Actors: exactly Mara, Ivo, Nia and one host.
- Places: exactly Old Quarry and Quiet Grove.
- Occurrences: at most one public stone `submit_action` and one private
  Mara-to-Ivo `submit_interaction`; repeated fixture submissions are rejected.
- Context bound: at most three host-buffer and three public-history items per read.
- Simulated Agent knowledge: at most six context items.
- Connection model: one connected or disconnected fixture host; no socket or gateway.
- Workload and duration: eight deterministic scenario tests; no concurrency or
  throughput load.

## Real and simulated seams

The Rust compiler, test runner and transitions in [`src/lib.rs`](src/lib.rs) are
real: the assertions execute the exact in-memory state machine committed in this
experiment. The `World`, host, hint delivery and Agent state inside that machine are
all simulations. PostgreSQL, transactions, HTTP, MCP, authentication, gateways,
networks, an LLM and production Aicadia code are absent.

The verdict therefore covers only the simulated fixture's representability and
ownership invariants. It is neither production-World correctness evidence nor a
direct Agent/MCP smoke.

## Deliberate exclusions

The experiment does not model or prove PostgreSQL transactions, commit ordering,
MCP behavior, authentication, multiple simultaneous connections, fan-out topology,
movement history, reconnect persistence, event-time Place intervals, hidden Actions,
whispers, adjacent-Place sound, occlusion, sensory capabilities, NPC behavior,
overload, fairness or production recovery.

It also does not decide whether an attentive host must recover every occurrence or
only exact current state plus bounded recent context. Manual refetch uses the toy
fixture sequence solely to make loss and duplication inspectable.

## Observations

The eight Rust tests produce these bounded observations:

1. Baseline then subscription, one public stone Action, refetch and one explicit
   User turn leave one World Activity and one Agent knowledge item. The hint alone
   never changes Agent state.
2. Two simulated delivery attempts coalesce to one opaque dirty-Place hint. One
   authorized refetch still returns one occurrence; no duplicate Activity or Agent
   item is created.
3. A deliberately lost hint leaves World current state and Activity intact. A manual
   authoritative refetch can still return the occurrence while attention remains
   active.
4. Disconnect before the Action ends attention. Reconnect and a new baseline cannot
   fabricate live context; a separate bounded public-history read labels the result
   as history and explicitly denies personal witnessing.
5. Ivo can arrive later and learn the public stone occurrence from Place history
   without any realtime delivery attempt or personal perception record.
6. Nia can be actively subscribed at the same Place while Mara privately interacts
   with Ivo. The World stores one Interaction Activity, but the participant-only
   operation contract yields no hint or content for Nia.
7. Character switch and Place departure each remove host attention. Mere same-Place
   placement after a switch does not reactivate the old subscription.
8. The fixture's deterministic Agent-state formatter adds natural sensory wording
   only for already-authorized active context, and this simulated presentation adds
   no World mechanic or sensory state.

Across all scenarios, World state contains current subjects and one shared Activity
list only. Subscription, opaque hints and bounded context stay on the host, while
Agent invocation and derived presentation stay in Agent state.

## Falsifier

Change the verdict to `refuted` if a reproducible required Q1–Q4 case shows that:

- a bystander can receive the private Interaction through a hint, refetch or Agent
  turn;
- a disconnected, switched or departed Character receives live context from an old
  subscription;
- Agent knowledge changes without the explicit User-turn command;
- delivery loss or duplication creates, removes or duplicates World Activity; or
- the fixture cannot keep later public history distinct from attentive local context
  without a durable personal observation row.

Change it to `inconclusive` if a remaining question depends on a real PostgreSQL,
MCP, authentication or multi-connection seam absent from this fixture.

## Verdict and artifact status

**Verdict: `supported`.** The accepted Q1–Q4 ownership split is internally
representable for this fixed fixture without durable observer or sensory state and
without Agent-owned authorization. Every required scenario reaches its intended
bounded end state.

**Artifact status: `kept`.** The crate remains a rough, non-authoritative logic aid
because it makes the ownership boundaries and falsifiers executable in Aicadia's
implementation language. It must not be imported, copied or promoted into
production.

## Limitations and non-claims

This supports only semantic representability in one deterministic simulated
in-memory model. It does not prove the chosen semantics are the best game design,
that any transport will deliver them, that a cursor is lossless, or that the
production World is correct. It makes no million-User, concurrency, latency,
durability, PostgreSQL, MCP, Agent-comprehension, security or operational claim.

## Downstream implication

No additional durable attention, personal observation or sensory-channel model is
needed merely to continue the grill. After the current transaction-conflict branch,
the later observation decision can stay focused on the actual catch-up promise:
whether active play requires every accepted ambient occurrence or authoritative
current state plus bounded recent context. A later real-seam experiment is earned
only if that decision depends on PostgreSQL ordering, protocol notification behavior
or reconnect recovery rather than game preference.
