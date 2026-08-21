---
status: pending
era: August Activity-Property-Trait
---

# Three blank-slate Multiplayer mechanics

> **Role / side:** bounded Multiplayer design research report / development side.
> **Authority:** records three independently generated universal mechanic proposals,
> their common findings, differences, scenario pressure and evidence boundary.
> **Excludes:** accepted Aicadia behavior, canonical terminology and implementation;
> those remain in `game/docs/`, `dev/CONTEXT.md` and an accepted plan.

Date: 2026-08-20

Status: research. Every named mechanism and term in this report is an unaccepted
working proposal. The report does not choose a Multiplayer foundation.

## Question

What universal Multiplayer game mechanic emerges when three designers start from the
fourteen fixed scenarios and only these premises?

- Aicadia is played through bring-your-own AI Agents over MCP.
- The system should exploit Agent intelligence and creativity.
- World is deliberately not intelligent. It may provide powerful purpose-built tools
  and may structurally validate, resolve and store Agent outputs, but it may not make
  semantic judgments.

The exercise deliberately asked for resets rather than variations on prior Aicadia
work. Each designer had to produce one complete mechanic covering player experience,
Agent and MCP use, current state, history, authority, concurrency, hot subjects,
fairness, abuse, S01-S14 and a smallest falsifying experiment.

## Method and evidence boundary

The only repository source supplied to the design runs was the
[Multiplayer scenario catalogue](../../areas/multiplayer/scenarios.md). They were
explicitly prohibited from reading other repository files, earlier proposals, the
conversation or the web.

Two fresh `gpt-5.6-sol` high-reasoning child runs received empty conversation forks.
A hard platform thread limit prevented creation of a third empty-fork child. The
third result came from a previously created but content-unused `gpt-5.6-sol`
high-reasoning design task, restarted with the same source restriction and without
either completed proposal. This is a weaker isolation guarantee than a new thread
and is recorded rather than hidden.

The outputs are design hypotheses, not sourced external facts and not evidence that
BYO hosts, models, PostgreSQL or players can perform the proposed flows. Agreement
between the runs is useful convergence evidence only; it does not make a choice true.

## Proposal A: provable imagination

`Provable imagination`, `World move`, `imprint`, `echo` and `ensemble` are translated
working labels for the first proposal, not accepted Aicadia terminology.

### Core mechanic and GX

An Agent may imagine anything, but World changes only through one bounded package
containing the exact intended state and the exact structural facts on which it
depends. The Agent authors meaning, causal consequences and presentation. World
checks identity, authority, exact facts, explicit relations, bounds, idempotency and
atomicity, then stores precisely that result and one attributable history record.

A normal action has no collaboration phase: read, Agent reasoning, one package and
one transaction. Other players receive at most a disposable dirty-resource hint.
Their Agent, only when explicitly invoked, reads authoritative state and decides what
its Character could notice or later discover.

### Concurrency shape

Current facts are individually addressable and versioned, conceptually as:

```text
(subject, facet, key) -> value, version
```

The package names both writes and exact read dependencies. Disjoint facts can commit
in parallel. Placement and other exclusive facts retain one exclusive conflict
cell. Expected absence also has an addressable guard, so concurrent creation need
not lock an Entity or Place wholesale.

One transaction checks request identity, resolves package-local references, locks
the bounded read/write cells in stable order, rechecks authority and dependencies,
and writes current state, one history record, request receipt and dirty resources.
There is no mutable Place-wide latest-activity row or global revision.

### Collective work

An explicitly communal capability may open a fixed-deadline bounded ensemble. A
deterministically sampled set of explicitly invoked Agents can submit complete
candidates, optionally critique a bounded selection and ratify one exact candidate.
World counts only structurally valid ratification and never combines content. The
winning Agent-authored package goes through the same ordinary transaction. No valid
ratification means no change; a changed dependency means a separately legible stale
final package.

This makes collective work a deliberate authority condition, not an automatic
response to traffic. Its cost is that ordinary concurrent creative changes are not
semantically merged unless an Agent later authors a new state or a communal
capability was already active.

### Scale, effects and recovery

- One hot exact fact has finite local throughput and fast bounded overload; unrelated
  facts, Entities and Places continue.
- Place membership and history indexes are segmented rather than advanced through a
  shared Place row.
- A World- or Place-scoped appearance can be one separately versioned scope effect.
  A literal rewrite of millions of Entity properties remains proportional batch work.
- Dirty hints are coalescible and lossy. Reconnect reads a bounded current baseline
  plus recent history and reports an honest gap when older context is unavailable.
- Occurrence-time placement and Character fact history can support later Agent
  judgment without one durable delivery row per observer.

### Primary risk

This is the cleanest technical foundation, but its ordinary conflict experience can
still collapse into stale results, admission failure or later repair. It does not by
itself turn simultaneous creative collision into fun.

## Proposal B: the World score

`World score`, `measure`, `beat`, `choir` and `continuation` are translated working
labels for the second proposal, not accepted Aicadia terminology.

### Core mechanic and GX

Every Agent writes one complete bounded measure. World settles measures on fixed
short beats, proposed at 100 milliseconds. Compatible fact writes pass together;
conflicting fact writes are evaluated in an auditable, packet-order-independent
lottery order. Every measure is all-or-nothing and produces one history record.

The fixed rhythm makes contention predictable instead of allowing request timing or
an unbounded queue to decide it. Under extreme pressure, a per-principal admission
ticket places attempts in a bounded future beat. Quiet facts never enter the hot
fact's schedule.

### Pre-authored consequences

An Agent may include a small finite directed acyclic graph of complete future change
packages. Every later node was therefore semantically authored during the explicit
User invocation. World may execute a node at its declared time or predecessor result
without invoking an Agent, but rechecks its exact authority and current facts.

World never traverses arbitrary World relations. The executable graph is separately
bounded by depth, subjects and identities, rejects cycles and uses idempotency so a
duplicate hint cannot execute anything twice. This directly addresses delayed remote
effects and causal chains, but risks becoming a hidden rule engine if allowed to grow.

### Collective work

A communal Entity may open a bounded choir with a base state, eligibility, deadlines,
maximum participants and a predetermined ratification rule. Explicitly invoked Agents
commit sealed proposals, reveal them, critique a bounded set and may author one full
combined measure. Eligible Agents sign exact package hashes; World chooses a package
only from valid structural quorums and never judges meaning.

The final package uses the same beat and transaction path as an ordinary measure.
`No quorum`, `stale after collective authoring` and `lost settlement conflict` remain
different outcomes.

### Scale, effects and recovery

The proposal converges with A on exact fact cells, stable lock ordering, atomic
current state and history, scoped effects, lossy resource hints, bounded recovery and
no global revision, counter or Place integrity row. Its distinct additions are the
fixed 100-millisecond settlement rhythm and already Agent-authored executable
continuations.

### Primary risk

The beat may feel like arbitrary lottery loss, and deterministic continuation may
quietly reintroduce exactly the resident behavior engine Aicadia is trying to avoid.
The communal choir also depends on enough Users keeping their Agents explicitly
active across multiple phases.

## Proposal C: World montage

`World montage`, `take`, `cut`, `editor` and `write lane` are translated working
labels for the third proposal, not accepted Aicadia terminology.

### Core mechanic and GX

Every change starts as one complete Agent-authored take. A take that sees no
structural collision after a proposed 120-millisecond breath becomes current
directly. Overlapping writes open one fixed, approximately 1.2-second montage. World
admits at most sixteen sealed takes through content-independent sampling and selects
one of those still-active BYO Agents as temporary editor.

The editor receives the bounded immutable candidate set inside the same explicit
User-owned Agent run. It selects whole, pre-authorized bundles and writes one exact
final package and final prose. World does not combine, rank or interpret any meaning.
The editor may not invent writes outside the submitted bundles or a narrowly granted
editor mandate.

This reframes a hotspot as a game event: simultaneous blue, engraving and leg ideas
can become one attributable cut rather than a stack of stale errors. The losing
Agents may still have contributed to current state.

### Who knows what belongs together

- Each source Agent declares which writes within its own take are indivisible and
  whether they may be combined with other takes.
- World groups only exact structural write overlap, never semantic similarity.
- The selected editor-Agent is the only semantic author of the final combined state.

Thus World does know when writes technically contend, but only an Agent judges
whether their meanings form a coherent result.

### MCP flow and fallback

The proposed single entry tool may return direct acceptance, an editor turn, waiting,
bounded crowding or structural rejection. Only a selected editor may submit the final
selection and prose. The call is long-polled so an editor turn can continue within
the explicit invocation rather than a server-triggered Agent run.

Every take contains a complete standalone fallback. If the editor does not finish by
the immutable deadline, World may settle that already Agent-authored fallback if it
is still valid. No incoming traffic extends the deadline. A late host can read the
canonical montage result by its stable identity.

### Hot-subject fairness

- One effective take per durable User or explicit authority seat per montage.
- Content is sealed before editor selection.
- Bottom-K sampling is derived from World randomness and identity, not prose, nonce
  grinding or network arrival.
- The same editor cannot immediately edit the next conflict on that lane while other
  eligible Agents are present.
- A persistently hot lane remains on a fixed montage rhythm; it does not slow any
  disjoint lane.
- Collision components, multi-subject packages and candidate storage have hard
  bounds. A package that would join two already-full montages waits for a later one.

### Scale, effects and recovery

The final transaction again locks exact write lanes in stable order, stores current
state and one Activity atomically, keeps Activity indexes append-only and emits only
resource dirty keys. Scoped appearance and literal mass mutation remain distinct.
Lossy hints and authoritative reconnect are the same separation as in A and B.

### Primary risk

The creative promise depends on a BYO host actually continuing after the tool result
and on another model inference being fast enough for good GX. A 900-millisecond editor
deadline is not credible until measured with real hosts and heterogeneous models.
Longer deadlines make a hot Entity feel slow; fallback weakens the collaborative
promise. Hostile collision can also force ordinary play onto the montage rhythm.

## Scenario comparison

All three proposals provide a plausible structural answer to every catalogue case;
the table emphasizes where they differ.

| Scenario | Proposal A | Proposal B | Proposal C |
| --- | --- | --- | --- |
| S01 | Direct exact-fact atomic package | Same package on next fixed beat | Solo take after a short collision breath |
| S02 | Fact-local concurrency; communal work is deliberate | Fact-local beat lottery plus optional choir | Bounded live montage may combine colliding takes |
| S03 | One atomic remote package or later explicit Action | Atomic package or pre-authored continuation | Atomic take; later consequence needs new take |
| S04 | Agent names all mutations and structural occurrence scope | Same, optionally split through authored nodes | Same inside one take or bounded causal graph |
| S05 | Receiving Agent interprets occurrence-time own facts | Same | Same, with source-authored perception premise |
| S06 | Exact dependencies decide composition and staleness | Same, ordered by one beat | Editor may combine compatible bundles; dependencies still gate |
| S07 | Concrete identity or explicit communal materialization | Concrete identity or materialization key | Concrete identity or shared materialization lane |
| S08 | Package-local references, one atomic graph | Same | Same inside an indivisible take bundle |
| S09 | Exact absence and freshness fail closed | Same | Same before final cut |
| S10 | Separate scoped appearance effect | Same | Same through its own write lane |
| S11 | Local fact admission and segmented indexes | Per-fact beat and overload horizon | Per-lane montage and bounded collision components |
| S12 | Lossy hints; baseline plus bounded history | Same | Same, plus stable montage result lookup |
| S13 | Sampled proposal, critique and ratification round | Multi-phase sealed choir and quorum | Common mandate plus one live editor cut |
| S14 | Closed bounded package; later effects need explicit calls | A bounded pre-authored executable DAG | Bounded causal graph in take; later calls otherwise |

## Independent convergence

Despite the reset, all three designs independently converged on these foundations:

1. **Every submitted candidate is already complete.** A server cannot wait for an
   absent semantic author and still guarantee progress.
2. **Agent-authored read dependencies are as important as writes.** World can detect
   stale structure only when the Agent names the exact facts its meaning used.
3. **Concurrency is scoped below the Entity.** Per-property, placement, relation,
   materialization and scoped-effect conflict subjects avoid turning a popular Entity
   or Place into one unnecessary lock.
4. **Current state and attributable history commit together.** Proposal traffic and
   delivery never become a second current truth.
5. **Hints are disposable.** They mark resources dirty; explicit Agent invocation and
   authoritative refetch remain separate.
6. **Scoped appearance differs from literal mass mutation.** One compact contextual
   effect cannot pretend that millions of owned Entity facts were rewritten.
7. **One truly hot fact is necessarily finite.** The goal is fair local degradation
   and quiet-subject isolation, not physically impossible infinite parallel writes.
8. **Collective meaning still needs one exact Agent-authored final package.** World
   may sample, schedule and verify authority but cannot prove semantic quality.

These are stronger research findings than any proposal's working terminology. They
may inform the active concept without selecting cells, lanes, beats or montages as
current architecture.

## Material differences

| Question | Proposal A | Proposal B | Proposal C |
| --- | --- | --- | --- |
| What happens on ordinary collision? | deterministic structural settlement or retry | fixed short beat with auditable order | live bounded Agent editing |
| When does semantic combination happen? | deliberate communal round or later Agent | deliberate choir | automatically when exact writes collide |
| Extra Agent inference on hot path? | no for ordinary play | no for ordinary play | yes, if editor continuation works |
| Novel delayed consequence | later explicit Agent Action | pre-authored finite executable graph | later explicit Agent Action |
| Strongest GX promise | clarity and immediate play | predictable shared rhythm | collision itself becomes creative multiplayer |
| Strongest failure | stale/error ceremony remains | lottery and hidden rule-engine pressure | host continuation, model latency and griefing |

## Aicadia implications

The reset did not reveal a free semantic merge. It did reveal a more promising game
question than “which database conflict policy wins?”:

> Should true same-fact collision sometimes become a short, bounded creative editing
> opportunity for one already active participating Agent?

Proposal C is the only reset that makes collision itself potentially fun and uses the
Agents' comparative advantage in live semantic composition. It is therefore the
strongest new experiment candidate, not yet the strongest architecture. Its central
claim fails unless an actual BYO Agent host can continue the same User invocation,
read several immutable packages, author a structurally valid combined state and
return within a GX-acceptable bound.

Proposal A is the clean control: if live montage fails, it gives the smallest honest
system. Proposal B contributes two separately testable ideas—fixed fair settlement
beats and pre-authored deterministic continuations—but combining either into the
universal foundation would add more machinery than the other proposals require.

The smallest next research step is therefore not a schema or distributed load test.
It is a throwaway direct-Agent interaction test comparing:

1. one unopposed complete take;
2. two compatible and two contradictory sealed takes;
3. one real BYO Agent receiving the bounded set after its first tool call;
4. the Agent returning one exact package or timing out to a pre-authored fallback;
5. human judgment of clarity and fun against a content-blind winner plus retry.

Only if that loop works should a state-machine test pressure atomicity, bottom-K
admission, hot-lane isolation and ten-thousand-offer bounds. No current schema, API,
MCP surface, timer, cooldown or production behavior is authorized by this report.
