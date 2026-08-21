---
status: pending
era: August Activity-Property-Trait
---

# BYO-Agent coordination without server inference

> **Role / side:** sourced Multiplayer research report / development side.
> **Authority:** records findings and inferences about semantic relatedness,
> explicitly active BYO Agents, durable shared context and previously authored
> deterministic behavior.
> **Excludes:** accepted Aicadia behavior, canonical terminology and implementation;
> those remain in `game/docs/`, `dev/CONTEXT.md` and an accepted plan.

Date: 2026-08-20

Status: research. Shared World-history, bounded live handoff and Agent-authored
executable behavior remain unaccepted candidates.

## Question

Who can know that several proposed World changes belong together when World has no
semantic intelligence, Aicadia cannot launch an additional Agent because every Agent
is User-owned, and thousands of Agents cannot deliberate all-to-all before one fast
result?

## Hard epistemic boundary

Only an explicitly active Agent can judge that two meanings belong together. World
may observe exact shared Entity, Place, relation, Activity and current-fact
identities, but structural overlap is not semantic relatedness. Two changes to one
Entity may be independent; changes to distant Entities may be causally inseparable.

**Inference.** Relatedness must enter the system as Agent-authored data. A live Agent
can name exact current subjects and prior Activities it considered. A previously
accepted Agent-authored deterministic behavior can declare the exact input and state
scope it executes over. If neither exists, no component can truthfully construct a
combined semantic result.

## MCP cannot supply a missing Agent

MCP resource updates announce that an exact resource may have changed and require a
refetch. They do not invoke a model. Multi-round-trip input can continue one already
active request, but it does not create another User-owned Agent or cross-client
deliberation.
[MCP resources](https://modelcontextprotocol.io/specification/2026-07-28/server/resources),
[MCP multi-round-trip requests](https://modelcontextprotocol.io/specification/2026-07-28/basic/patterns/mrtr)

**Inference.** Aicadia may opportunistically return bounded shared input to an Agent
whose User invocation is still running. Correctness and progress cannot depend on
that Agent remaining active, because generic BYO hosts may finish after any tool
result and World cannot restart them.

## Durable World-history as asynchronous shared memory

An explicitly active Agent can understand earlier World work without a separate
server-side discussion process when ordinary attributable history and World Entities
carry the contribution. It can cite exact earlier Activity identities and exact
current subjects in one later complete Action. World validates existence,
visibility, authority, bounds and freshness; the Agent alone decides relevance and
authors the final state.

**Inference.** This avoids an open database transaction, Agent session, lock or
deadline while discussion develops. Current state remains playable. A discussion,
sketch, design or summary can be ordinary in-World content rather than a universal
technical proposal type. The cost is temporal: one later active Agent, not World,
must eventually read bounded context and author the next exact state.

Thousands of contributions cannot all fit one bounded Agent context. Agents may
author bounded summaries that cite bounded earlier Activities, but World cannot know
that a summary is complete or faithful. Complete representation remains at least
linear in reads, tokens and attributable storage.

## Bounded live handoff within one existing Agent invocation

When several currently active BYO Agents already submitted complete results for the
same exact declared subjects, World can expose a bounded immutable set to one of
those still-running Agent invocations. That Agent may return one replacement complete
state. This launches no new Agent; it is another model/tool step inside the original
User-owned invocation.

**Inference.** Every initial result must remain complete so World can settle one
already Agent-authored package or leave state unchanged if the selected Agent or host
stops. Contributions must be append-only, deadlines immutable and semantic input
hard-bounded. Sampling `K` contributions from thousands bounds finalizer context but
does not synthesize every contribution or guarantee semantic representation.

This is an optional latency optimization, not a portable correctness foundation.
Its feasibility depends on actual BYO host behavior and would require direct host
evidence before acceptance.

## Previously Agent-authored deterministic behavior

Persistent virtual worlds already demonstrate that creator-authored object behavior
can execute server-side in response to events. Second Life documents event-driven
scripts attached to in-World objects and executed on its servers.
[Second Life LSL reference](https://create.secondlife.com/script/lsl-reference/),
[Getting started with LSL](https://wiki.secondlife.com/wiki/Getting_started_with_LSL)

Smart contracts similarly store code and state, then execute their predefined
functions when later transactions arrive. Their value here is the separation between
earlier authored logic and later deterministic execution, not blockchain, stake,
currency or irreversible deployment.
[Ethereum smart contracts](https://ethereum.org/developers/docs/smart-contracts/)

Sandboxed WebAssembly runtimes can bound untrusted execution. Wasmtime documents
deterministic fuel-based interruption: the same execution with the same fuel reaches
the same interruption point, subject to other nondeterminism being excluded.
[Wasmtime interruption](https://docs.wasmtime.dev/examples-interrupting-wasm.html)

**Inference.** For a repeated known mechanic, Agents could author and Users could
ratify a bounded deterministic behavior before runtime. World would later execute it
without understanding its meaning or invoking a model. This fits “Agents design;
World executes,” and can reduce large typed input sets quickly when the installed
behavior itself has a bounded reduction.

It does not solve novel arbitrary semantics. The earlier Agent had to anticipate the
input vocabulary and exact transition. Safe execution also requires a small
deterministic capability surface, authority review, memory and instruction bounds,
versioning, failure semantics and upgrade governance. Arbitrary Agent-authored server
scripts would therefore add major architecture and security cost and remain outside
the current game contract.

## Candidate comparison

| Where semantic intelligence exists | Who authors the result | Latency | Thousand-Agent truth |
| --- | --- | --- | --- |
| Current active Agent reading current state and bounded history | That BYO Agent | one normal Agent turn | only bounded context can be understood |
| Same already-active Agent receiving bounded live contributions | That BYO Agent | one extra model/tool step | sampling is representation, not full synthesis |
| Earlier Agent-authored deterministic behavior | The earlier behavior author; World executes | server-speed after admission | only the behavior's predefined bounded semantics |
| World, listener or database | nobody semantic | fast mechanically | cannot truthfully create a combined meaning |

## Game and scale implications

- Ordinary state change can remain a complete one-Agent Action with short atomic
  settlement.
- Shared work can remain non-blocking World history until one later active Agent
  authors a current-state change from it.
- A hot Entity still permits only bounded current-state mutation throughput; cooldown
  can keep one accepted state stable while history and other Entities continue.
- Simultaneous complete states that no Agent has seen together may only be selected,
  ordered or left unchanged content-blind. World may never merge them.
- An all-to-all room creates potentially quadratic reads and token work. Append-only
  contributions plus bounded Agent-authored summaries make work sparse but cannot
  guarantee equal semantic inclusion.
- Millions of independent subjects scale horizontally; one shared meaning remains a
  local information and authorship bottleneck regardless of database architecture.
- The final accepted Action can cite exact prior Activities as provenance. Citation
  proves what the Agent claimed to consider, not semantic fidelity.

## Aicadia implications

The recursive Agent tree previously recommended is not a universal foundation: it
assumed World could form meaningful groups and that selected Agents could be kept or
made active for additional inference. Both assumptions are invalid under BYO Agent
portability.

The strongest current research direction is to treat World as durable shared memory,
not as a consensus engine. Explicitly active Agents decide relatedness and author
complete current states. A bounded same-invocation handoff may accelerate a hot
moment when supported. Previously Agent-authored deterministic behavior may later
make repeated mechanics fast, but it is a separate high-cost research branch rather
than a current universal rule engine.

No current capability, Activity relation, executable behavior, schema, MCP surface
or production operation is authorized by this research.
