---
status: done
horizon: done
updated: 2026-08-10
---

# Agent world-entry handoff

> **Role / side:** forward-planning item / development side.
> **Authority:** records this outcome's backlog state, dependencies and completion pointers.
> **Excludes:** current product contracts, decision rationale and detailed delivery evidence; see `docs/game/`, `docs/concept/log/` and `docs/evidence/`.

## Outcome

A newly connected Agent can understand and execute the complete current World-entry
flow from the published MCP catalog alone. Two isolated Agents can then prove that
their Characters enter the same persistent Place and can read the Activity history
created by those accepted actions.

## Why now

The World-entry and Activity behavior is implemented and adapter-tested, but the live
Agent acceptance proof still covers only shared Entity creation and observation.
Several important meanings also live mainly in repository documentation instead of
the MCP descriptions and output schemas an external Agent actually receives. The
next game capability should not build on an Agent interface whose current flow has
not yet been understood clean-room.

## Current scope

- Make the existing ten MCP tools self-explanatory about unplaced Characters, World
  genesis, entry retry behavior, Place identity and Activity references.
- Publish one deterministic recommended entry flow without adding a new World
  operation, model, table or server decision.
- Extend the disposable two-Agent playtest so Agent A establishes the entry Place
  when needed and enters, while Agent B later enters the same Place.
- Require both Agents to read and explain their resulting personal Activity through
  MCP, while retaining the existing cross-User shared-Entity observation proof.

## Completion evidence

This item is done only when:

1. the generated MCP descriptions and output schemas expose every meaning needed to
   follow the flow without repository access;
2. the exact checked-in catalog fixture and contract documentation agree;
3. the fake-runner suite, Rust suite, formatting, lint and token-free preflight pass;
4. one explicitly authorized paid run with two isolated Agents completes and proves
   entry into the same Place, personal Activity comprehension and shared-Entity
   observation.

## Current boundary

Delivery history and current status: see [World-entry evidence](../../../docs/evidence/world-entry.md).

## Explicit non-goals

- new World behavior, public capabilities, tables or domain states;
- later Places, movement, investigation, discovery or global history;
- server inference, Agent sessions or durable playtest fixtures;
- making expected first-use absence or genesis conflicts disappear from the current
  game error contract.
