> **Superseded — August Activity-Property-Trait era:** root `AGENTS.md` and `dev/skills/build-aicadia/SKILL.md` are the current development-governance successors.

# Codex agent graphs as an Aicadia build method

Status: research, checked 2026-08-08; development-method recommendation, not an
accepted build decision

## Question

How can a graph-shaped Codex workflow improve the way Aicadia is researched,
designed, implemented and verified, without introducing an Agent graph or LLM
runtime into the Aicadia product?

This note uses current official Codex guidance, official graph-orchestration
documentation and original multi-Agent failure research. It does not change
`game/docs/`. Aicadia remains one deterministic `World` over PostgreSQL with six
current use cases, thin HTTP and MCP adapters, and no server-side LLM or durable
Agent session.

## Short answer

Use an Agent graph as a **build methodology in Codex**: the main Codex thread owns
the user intent, Terry decisions, task graph, integration and final report. For an
accepted build it delegates every repository write to one bounded designated writer
subagent; every other subagent is read-only. Dependencies determine which nodes may
run; independent nodes may run in parallel; every subagent returns a small structured
handoff; deterministic commands verify the integrated change; separate standards and
specification reviews may trigger one bounded repair loop; and unresolved product
choices or impactful actions stop at a human gate.

Do not install a graph framework or build orchestration infrastructure now. Codex
already supports spawning subagents, waiting for their results and consolidating
them in the main thread. Its official guidance recommends keeping requirements,
decisions and final outputs in that main thread, using subagents for bounded
exploration, tests or triage, and being cautious with parallel write-heavy work.
Subagents also spend more tokens than a comparable single-Agent run.
([Codex subagents](https://learn.chatgpt.com/docs/agent-configuration/subagents.md))

The first version should therefore be a repeatable task shape, not software. If the
same shape succeeds on several real Aicadia tasks and keeps needing the same prompt,
package it as a repo skill. Official Codex guidance likewise says to make one
representative task work before turning the recurring workflow into a skill.
([Codex best practices](https://learn.chatgpt.com/guides/best-practices.md),
[Build skills](https://learn.chatgpt.com/docs/build-skills))

## What “Agent graph” means here

The term is not standardized. It may describe a workflow control graph, a network
of communicating Agents, an observed execution trace, or a knowledge/dependency
graph used as data. Frameworks expose different meanings: LangGraph nodes can be
arbitrary functions over shared state, AutoGen GraphFlow makes Agents the nodes,
Google ADK permits Agents, tools and ordinary code as nodes, and OpenAI's Agents SDK
can express manager and handoff topologies without requiring a graph abstraction.
([LangGraph Graph API](https://docs.langchain.com/oss/python/langgraph/graph-api),
[AutoGen GraphFlow](https://microsoft.github.io/autogen/dev/user-guide/agentchat-user-guide/graph-flow.html),
[Google ADK graph workflows](https://adk.dev/graphs/),
[OpenAI Agent orchestration](https://openai.github.io/openai-agents-python/multi_agent/))

For this proposal, the graph is specifically a **development task dependency
graph**:

- a node is one bounded research, inspection, decision, edit, verification or
  review job;
- an edge means the destination needs the predecessor's accepted output;
- fan-out means independent read-only jobs may run concurrently;
- fan-in means the main thread compares and integrates their structured handoffs;
- a conditional edge is an explicit pass, fail, defer or human-decision route; and
- the only cycle is a bounded `review failure -> repair -> reverify` loop.

This is not a graph database, a new Aicadia domain model, a multi-Agent game server,
or a background process that spends player tokens.

## Why Codex fits this method

### Durable rules remain in `AGENTS.md`

Codex reads `AGENTS.md` before work and layers repository and nested guidance into
its instruction chain. Official guidance recommends keeping it concise and using it
for persistent repository conventions, commands, constraints and verification
expectations; new rules should be added when repeated mistakes prove their value.
Aicadia's current `AGENTS.md` already carries Terry and the always-on build gates,
so the graph should reference those rules rather than copy them into every node.
([Codex `AGENTS.md` guidance](https://learn.chatgpt.com/docs/agent-configuration/agents-md.md),
[Codex best practices](https://learn.chatgpt.com/guides/best-practices.md))

Task-specific state stays in the current prompt and task plan. Accepted behavior
stays in `game/docs/`. Research stays in `dev/docs/research/`. A future skill may encode
the repeated workflow. This prevents the graph prompt or a subagent summary from
becoming a second source of truth.

### The main thread coordinates; one subagent writes

Official Codex guidance says the main Agent should remain focused on requirements,
decisions and final outputs while subagents return summaries of noisy exploration,
test logs or analysis. It recommends parallel Agents first for read-heavy work and
warns that simultaneous code editing creates conflicts and coordination overhead.
([Codex subagents](https://learn.chatgpt.com/docs/agent-configuration/subagents.md))

For Aicadia, tighten that into one rule: **exactly one bounded designated writer
subagent owns every repository write in a run**. The root/main thread coordinates,
decides scope, integrates results and reports, but does not edit. Every other
subagent may inspect files, research primary sources, run non-mutating diagnostics,
analyze test output, or review a stable diff, but remains read-only. For a
research-only run, the researcher may be the designated writer limited to the one
research note and its index entry.

This sacrifices some maximum parallelism, but Aicadia's current codebase and six-use-
case MVP do not justify multiple overlapping implementers. If a future task has
genuinely disjoint worktrees and interfaces, that would be a new evidence-based
decision rather than an exception silently added now.

### Structured handoffs protect the main context

Codex subagents help because they keep intermediate noise out of the main context,
but a vague summary can still omit the decisive constraint. Each delegated node
should receive and return a fixed, small work packet.

Input:

- one question or acceptance claim;
- authoritative files or sources to inspect;
- explicit allowed scope and a no-write instruction;
- required evidence format; and
- the condition that marks the node complete.

Output:

- conclusion in one or two sentences;
- evidence with file locations, commands or primary-source links;
- concrete implication for the parent task;
- uncertainty or disagreement that remains; and
- `pass`, `fail`, `defer` or `decision_required`.

Raw transcripts and large logs remain in the subagent thread unless the main thread
needs a specific excerpt. The parent integrates evidence, not authority: a subagent
recommendation never overrules `AGENTS.md`, `game/docs/`, executable behavior or the
user.

## The proposed bounded build graph

```text
request
  -> main: define goal, context, constraints, done condition
  -> Terry fit gate
       -> outside current contract: stop and defer explicitly
       -> behavior undecided: research fan-out -> main synthesis -> human decision
       -> behavior confirmed: dependency map
  -> optional read-only fan-out
       -> contract/code-path inspection
       -> primary-source research
       -> test or failure reproduction
  -> main fan-in: resolve evidence and approve implementation plan
  -> designated writer subagent: make the smallest complete change
  -> deterministic verification
       -> fail: one repair -> rerun failed and affected checks
       -> pass: parallel read-only standards review + specification review
  -> main fan-in: triage only evidence-backed findings
       -> material finding: one repair -> deterministic reverify
       -> no material finding: finish
  -> human gate for unchosen behavior, scope expansion, destructive or publish action
  -> report changed files, exact evidence, limits and next risk
```

Codex's official best-practices prompt shape maps directly to the first node: goal,
context, constraints and “done when.” Planning first is recommended for complex or
ambiguous tasks; testing, checking and reviewing are separate reliability steps, not
something implied by code generation.
([Codex best practices](https://learn.chatgpt.com/guides/best-practices.md))

### Node and dependency rules

1. The main thread defines the dependency graph before spawning. A node is ready only
   when every required predecessor has an accepted handoff.
2. Spawn the fewest subagents that expose useful concurrency. “One agent per topic”
   is not a goal; each extra Agent must remove a real context, latency or specialization
   bottleneck because it adds token cost.
3. Parallel nodes must be independent. If two tasks need the same evolving result,
   sequence them. Route every required edit through the one designated writer node.
4. Deterministic code decides routes wherever a command can establish the fact:
   compilation, tests, schema/catalog comparison and playtest assertions. An LLM may
   identify a suspected gap but does not certify its own work.
5. Every cycle has a named failed check and a maximum of one repair pass. A second
   failure ends the run with evidence and a new-plan requirement; it does not trigger
   autonomous retries.
6. Fan-in is a main-thread judgment step. Conflicting handoffs are compared against
   primary evidence and current contracts; they are not settled by majority vote.
7. A node stops as soon as its requested claim is proved or disproved. It does not
   expand into adjacent improvements.

### Aicadia routing rules

- **MVP gate:** every task identifies which subset of `get_world`, `create_user`,
  `get_user`, `list_entity`, `get_entity` and `create_entity` it decides, implements
  or verifies. Work unrelated to that surface is deferred.
- **Capability-parity join:** a player-facing behavior cannot finish until `World`,
  HTTP, MCP, Agent-facing descriptions, shared success/error semantics and parity
  tests agree in the same change. `create_user` stays on the provisioning/test path
  and does not fan out to HTTP or MCP.
- **Document gate:** research may inform a proposal but cannot change behavior.
  Unchosen behavior routes to the user; an accepted decision is recorded in
  `game/docs/` by the designated writer as part of the implementation change.
- **Runtime boundary:** no workflow node creates an Aicadia table, queue, server
  Agent, LLM call, autonomous trigger, Agent identity or durable game session. The
  build graph ends when development work ends.
- **Token-spend gate:** the existing real-Agent playtest remains an explicit human
  approval node. Ordinary deterministic checks run without inventing extra ceremony.

### Reviews and repair

After deterministic verification passes, two read-only review nodes may run in
parallel:

- **Standards review:** does the diff obey `AGENTS.md`, Terry, repository conventions
  and documented verification requirements?
- **Specification review:** does the diff implement exactly the requested behavior
  and current `game/docs/` contract, including HTTP/MCP parity where applicable?

The main thread joins both results and discards unsupported or style-only findings.
The same designated writer repairs material findings and reruns the failed evidence
plus checks affected by the repair. This separation reduces self-review blind spots
without letting reviewers become additional writers.

Original research on 1,642 traces from seven multi-Agent systems found distinct
failures in system specification, inter-Agent alignment, and task verification or
termination. Prompt and topology changes helped some cases but did not make the
systems reliably correct. Explicit work packets, deterministic verification and
hard stop conditions address those observed categories; they do not eliminate them.
([Cemri et al., 2025](https://arxiv.org/abs/2503.13657))

## Human gates and stop conditions

A human decision is required when evidence cannot choose without changing product
direction, when the requested scope must materially expand, or before a destructive,
token-spending, commit, push or publication action that was not already authorized.
The graph pauses with the alternatives, concrete consequences and current evidence;
it does not ask the user to adjudicate an internal implementation detail that the
contract or tests already decide.

End the run without claiming completion when any of these holds:

- the Terry gate says the work is outside the current build contract;
- an unchosen behavior blocks a safe implementation;
- a required primary source or local fact remains unavailable;
- deterministic verification still fails after the one repair pass;
- subagent findings conflict and evidence cannot resolve them;
- completing the task needs new authority or destructive scope; or
- the only remaining work is speculative improvement rather than the requested
  acceptance claim.

These are successful control-flow outcomes when reported precisely. A graph should
make “deferred,” “decision required” and “failed evidence” visible instead of
converting them into optimistic completion.

## When this beats one Agent—and when it does not

Use one main Agent without subagents when the task is small, sequential, write-heavy,
or depends on a single evolving code path. That keeps coordination and token cost
lowest.

Use the bounded graph when at least two independent evidence lanes can run at the
same time or when separating roles materially protects context or review quality.
Good Aicadia examples are:

- primary-source research in parallel with read-only current-contract inspection;
- code-path mapping in parallel with reproducing an existing failure;
- standards and specification review of one stable diff; or
- independent inspection of HTTP and MCP adapters before the main thread integrates
  one parity change.

Do not use it merely because a task is important or because more Agents are
available. It wins only when the saved latency, reduced main-context noise or
independent review is worth the extra tokens and handoff risk. Official Codex
guidance makes the same tradeoff explicit: subagents can parallelize and isolate
noisy work, but they consume more tokens and parallel writing raises conflict costs.
([Codex subagents](https://learn.chatgpt.com/docs/agent-configuration/subagents.md))

## Adoption plan without building infrastructure

1. Use the graph manually on one real, bounded Aicadia task through the existing
   Codex task plan and subagent controls. Add no framework, configuration, custom
   Agent or repository file for the method.
2. Record whether it caught a real contract, parity or evidence gap; whether parallel
   work reduced elapsed time; how many extra tokens or handoff corrections it cost;
   and which node added no value.
3. Repeat only on another task with a similar dependency shape. Keep the Terry gate,
   one-writer rule, structured handoff, deterministic verifier and split review only
   if they repeatedly change the outcome.
4. Once the manual workflow is stable across two or three concrete use cases, refine
   the existing `dev/skills/build-aicadia/SKILL.md` with only the graph, work
   packet and stop-condition parts that earned their place. Add scripts only for
   deterministic checks that cannot already be called directly. Skills are the
   official Codex surface for reusable workflows and load detailed instructions only
   when selected; Aicadia does not need a second build skill.
   ([Build skills](https://learn.chatgpt.com/docs/build-skills))
5. Consider a custom Agent profile only after a recurring specialist needs a distinct
   read-only sandbox, model or tool configuration. Consider an external graph runtime
   only if repeated runs prove that Codex's existing spawning, waiting and synthesis
   cannot supply required durable pause/resume or trace replay.

The proposed method is intentionally a graph on the development process, not in the
game. It spends developer-authorized Codex tokens, mutates the repository through one
visible writer, and leaves Aicadia's dumb and strict runtime unchanged.
