# Current immersive Agent play contract

> **Role / side:** forward-planning item / development side.
> **Authority:** records this outcome's backlog state, dependencies and completion pointers.
> **Excludes:** current product contracts, decision rationale and detailed delivery evidence; see `docs/game/`, `docs/concept/log/` and `docs/evidence/`.

Status: Done

## Outcome

Every conforming interactive Agent host receives one current, provider- and
model-neutral Aicadia player contract plus the complete current tool catalog. The
Agent keeps the entire conversation in player mode, renders internal World structure
as concrete facts about named people, locations, things and events, and fails closed
without Aicadia MCP rather than substituting repository, source, HTTP, database or
remembered state. Typed World state and structured consequences remain authoritative
and visible to every User at the same location.

The completed accepted build plan is
`.agents/plans/20260812-161148-current-agent-play-contract/plan.md`. Current behavior
is governed by [Agent play contract](../../../docs/game/agent.md); the protocol
evidence and limitations are recorded in
[Current MCP Agent guidance](../../../docs/research/current-mcp-agent-guidance.md).

## Confirmed direction

- support only stateless MCP `2026-07-28`; remove Aicadia-owned older session modes,
  compatibility branches, tests and stale current documentation;
- publish one global Agent contract and one complete aligned set of tool descriptions from one
  runtime module;
- use open semantic capabilities, never provider, model, client or tool allowlists;
- keep all player-facing language grounded, inviting and free of internal record
  categories, field names, absent-value syntax, transport, identifier, revision,
  commit and retry narration;
- answer questions about how something works through its concrete situation and
  currently available affordances; implementation inspection belongs in a separate
  development conversation;
- use Aicadia MCP as the sole live-game authority and stop without mutation when it
  is unavailable;
- start local Codex play through an isolated required-MCP adapter with an empty
  workspace and transient home/configuration outside the development repository;
- let the Agent reason and formulate freely while World validates typed identities,
  current state, supported consequences, idempotency, freshness and atomic history;
- do not inspect, store, score or lint private conversation and do not pretend to
  prove private human confirmation server-side; and
- add no prompt database, MCP prompt/resource/tool, schema, migration, auth,
  server-side inference or automatic model-token spend.

## Dependencies

- completed then-current World/HTTP/MCP contract;
- completed Agent-mediated action and cross-User observer evidence;
- pinned current `rmcp` implementation with MCP `2026-07-28` support.

## Completion evidence

Delivery history and current status: see [Agent contract evidence](../../../docs/evidence/README.md#agent-contract-delivery).
