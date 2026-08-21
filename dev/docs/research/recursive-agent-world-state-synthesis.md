---
status: superseded
era: August Activity-Property-Trait
---

# Recursive Agent synthesis of one World state

> **Role / side:** sourced Multiplayer research report / development side.
> **Authority:** records findings about bounded hierarchical Agent aggregation,
> game-theoretic selection and current protocol limits.
> **Excludes:** accepted Aicadia behavior, canonical terminology and implementation;
> those remain in `game/docs/`, `dev/CONTEXT.md` and an accepted plan.

Date: 2026-08-20

Status: research. The recursive protocol, selection rules, controller and technical
names below are unaccepted candidates.

> **Superseded direction:** retained because it establishes the cost and hard limits
> of layered Agent aggregation. It is no longer the active technical candidate after
> the User established that World cannot know semantic grouping and Aicadia cannot
> launch or depend on additional BYO Agent inference. See
> [BYO-Agent coordination without server inference](byo-agent-coordination-without-server-inference.md).

## Question

Can one universal Agent operation turn one or thousands of concurrent complete
Agent-authored World outcomes into exactly one Agent-authored final state, while
World understands no semantics and only bounds, routes, validates and commits?

The User requires that an Agent never choose a direct, conflict, collaboration or
other procedural mode. Only Agents may determine the semantically coherent final
state of a large group of changes. Application tooling may orchestrate their work;
World may not synthesize meaning.

## Findings

### Layered Agent aggregation is technically plausible

Mixture-of-Agents demonstrates a layered architecture in which Agents consume the
outputs of the prior layer and synthesize a next response. It reports improved
benchmark quality, but does not prove adversarial multiplayer fairness, faithful
preservation of every input, exact structured-state validity or bounded latency.
[Mixture-of-Agents](https://arxiv.org/abs/2406.04692)

Research on sparse multi-Agent debate reports that sparse communication topologies
can match or outperform all-to-all communication at lower cost. This supports small
bounded groups rather than every Agent reading every other Agent, but does not select
an Aicadia topology or prove semantic consensus.
[Sparse communication topology](https://arxiv.org/abs/2406.11776)

**Inference.** A recursive fan-in tree can keep every Agent context bounded. At each
node an Agent receives current World facts plus a small number of complete child
packages and returns one complete parent package. A one-package request is the same
operation at depth zero.

### More debate is not a truth proof

Recent work separating debate from voting reports that debate alone does not improve
expected correctness in its theoretical model and that simple ensembling accounts
for much of the measured gain. Another study identifies debate hacking in
competitive protocols and reports better error-detection performance after changing
the interaction to collaborative critique. These results disagree on universal
benefit but agree that topology and incentives materially affect output quality.
[Debate or Vote](https://arxiv.org/abs/2508.17536),
[Collaborative multi-Agent debate](https://arxiv.org/abs/2510.20963)

**Inference.** Aicadia cannot treat a number of agreeing Agents, a debate transcript
or a parent package as proof of semantic correctness. World can verify normalized
bytes, provenance and eligible authorship only. The final state necessarily grants
bounded editorial power to selected Agent outputs.

### Sortition can bound participation, not solve meaning

Algorand's Byzantine-agreement research uses cryptographic sortition to select small
committees from a large population and lets members prove selection. Its correctness
depends on a blockchain threat model and weighted honest participation that Aicadia
does not possess.
[Algorand: Scaling Byzantine Agreements](https://eprint.iacr.org/2017/454.pdf)

**Inference.** Aicadia may borrow auditable unpredictable assignment so request
speed and operator choice do not select synthesizing Agents. It may not import stake,
majority weight or the claim that a committee certificate proves a good World
outcome. Sybil resistance and the eligible identity remain separate game choices.

### Current MCP cannot create the collective by itself

MCP resource subscriptions announce that a resource changed and require a refetch;
they do not invoke a model. Multi-round-trip requests can ask one already active
client for more input, but they do not create cross-client Agent communication or
authorize background token use.
[MCP resources](https://modelcontextprotocol.io/specification/2026-07-28/server/resources),
[MCP multi-round-trip requests](https://modelcontextprotocol.io/specification/2026-07-28/basic/patterns/mrtr)

**Inference.** Recursive synthesis needs User-side Aicadia orchestration. One
explicit User action would have to authorize a hard maximum of follow-up model calls,
tokens and elapsed time while the action remains active. A listener remains an
optional attention hint, never the execution mechanism.

## Candidate universal Agent operation

The same model instruction can serve every level:

```text
Given bounded current World facts and one or more complete Agent-authored packages,
return exactly one complete final World-state package, including a no-change result
when appropriate.
```

World, not the model, handles commitments, reveal deadlines, content-blind random
group assignment and package canonicalization. The User-side application handles
the still-authorized model and tool calls. The model chooses no mode or role. Keeping
one child unchanged is represented by returning that exact package; combining
children produces a new exact package.

World may validate only:

- stable request and Character identity;
- one effective participation per accepted fairness identity;
- fixed admission and round deadlines;
- exact package hashes and declared child provenance;
- subject and payload bounds;
- structural authority and scope;
- exact current facts, cooldown and stable transaction order; and
- atomic current state, Activity and canonical request results.

It cannot validate that a parent faithfully represents its children, that a selected
Agent is wise or neutral, or that an omitted semantic dependency should have been
present.

## Scale arithmetic

With fan-in sixteen, ten thousand leaf packages require approximately four sequential
synthesis levels and about 669 internal Agent outputs:

```text
10,000 -> 625 -> 40 -> 3 -> 1
```

Groups at one level run in parallel. Depth therefore grows logarithmically, while
total Agent work and source information remain at least linear. At current model
latencies and heterogeneous hosts, a robust ten-thousand-Agent synthesis is more
plausibly measured in tens of seconds or minutes than milliseconds. Identical exact
packages may collapse by hash, but distinct meaning cannot be compressed for free.

A bounded constant-time alternative must sample only some proposals. That gives
representation by sortition, not synthesis of all proposed changes.

## Game-theoretic pressures

- Sealed commitments before an auditable grouping seed is revealed reduce copying,
  adaptive grouping and network front-running.
- One effective leaf per accepted User or Character boundary prevents request spam
  inside one opportunity but does not solve multi-account Sybils.
- Random group assignment distributes editorial power but cannot make model quality
  or collusion irrelevant.
- Multiple parallel outputs preserve alternatives but require another Agent-authored
  synthesis or a content-blind final fallback.
- Exact-hash agreement is strong integrity evidence but weak semantic evidence;
  equivalent meanings often have different representations.
- Requiring every participant to authorize one exact package gives each participant
  a denial-of-service veto.
- Majority and listener count reward popularity and Sybils and do not establish
  semantic correctness.
- Any fixed deadline needs a final fallback: one already Agent-authored valid package
  or no change. World may never manufacture a compromise.

## Aicadia implications

The recursive operation is the first candidate in this exploration that simultaneously
keeps one Agent interface, lets complexity grow automatically, makes Agents the only
semantic authors and leaves World dumb and strict. It is not lightweight inference:
it shifts semantic cost from one unbounded Agent context to bounded parallel User-owned
Agent work.

The upstream product gate is whether one explicit User action may pre-authorize a
hard-bounded series of follow-up model calls by the already active Agent. Without
that authority, automatic cross-Agent synthesis is impossible under the current
no-background-token rule. If accepted, later questions must choose admission,
sortition identity, maximum depth, subject stability, Activity attribution and the
fallback when Agents do not converge.

No current capability, schema, MCP surface or operation is authorized by this
research.
