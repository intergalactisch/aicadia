---
status: done
horizon: done
updated: 2026-08-10
---

# Agent world-entry handoff

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

Complete. Generated MCP descriptions, server instructions and output schemas carry
the entry and Activity meanings; the checked-in ten-tool fixture pins them. The
expanded runner and fake integration suite prove its exact two-Agent orchestration,
expected genesis-error handling, authoritative post-run state validation,
fail-closed evidence and cleanup paths.

`cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, all 38 Rust tests,
the Bash syntax checks, `bash tests/agent-playtest.sh`, `git diff --check` and the real
token-free Codex/PostgreSQL preflight passed on 2026-08-09. Explicitly authorized
live run `run-9TOG5yrJ` passed on 2026-08-10: two distinct Characters entered the
same entry Place, both personal Activity proofs matched authoritative HTTP state,
Agent B observed Agent A's exact shared Entity, and the disposable World was dropped.

## Explicit non-goals

- new World behavior, public capabilities, tables or domain states;
- later Places, movement, investigation, discovery or global history;
- server inference, Agent sessions or durable playtest fixtures;
- making expected first-use absence or genesis conflicts disappear from the current
  game error contract.
