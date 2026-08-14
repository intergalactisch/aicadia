# Local Agent play and World ledger

> **Role / side:** forward-planning item / development side.
> **Authority:** records this outcome's backlog state, dependencies and completion pointers.
> **Excludes:** current product contracts, decision rationale and detailed delivery evidence; see `docs/game/`, `docs/concept/log/` and `docs/evidence/`.

Status: Done

## Outcome

A developer-User can start and return to one persistent local World with one stable
hidden development User, conduct Character onboarding and all gameplay conversation
only in their own Agent, and inspect accepted World state in one read-only browser
ledger. Server restart preserves the same Character, placement, Entities, Activity
and prose.

The completed accepted plan is
`.agents/plans/20260812-091744-agent-only-local-play-ledger/plan.md`. The governing
contract is [the current build contract](../../../docs/game/README.md),
[Agent play contract](../../../docs/game/agent.md) and
[Local play](../../../docs/game/local-play.md).

## Confirmed direction

- the Agent is the only conversational interface;
- the browser has no chat, proposal, confirmation, mutation or model surface;
- the development User is local connection context, not a visible account;
- first-time Character onboarding privately offers exactly three candidates, accepts
  steering, previews the exact final Character and waits for confirmation before the
  existing `create_character` call;
- the first browser slice shows only World connection, shared Entity records and the
  local Character's accepted Activity/prose when available;
- Character and Place may appear as typed references in Activity, but receive no
  dedicated panel, page or map;
- the local database and User survive normal stop/restart;
- existing World, PostgreSQL, HTTP and then-current MCP behavior is reused without a
  new game capability;
- no build or validation step automatically spends model tokens.

## Current boundary

Delivery history and current status: see [Local-play evidence](../../../docs/evidence/local-play.md).

## Dependencies

- completed Agent-mediated World action and its then-current World/HTTP/MCP
  contract;
- local PostgreSQL and the existing server/provisioning binaries;
- the existing project-scoped Codex MCP configuration.

## Non-goals

- authentication, multiple Users/profiles or account UI;
- web conversation or any web mutation;
- dedicated User, Character or Place dashboards;
- new World behavior, schema, game-data endpoint or MCP tool; `/` only serves the
  read-only ledger document;
- discovery, movement, Place neighborhoods or later settlement behavior;
- automatic Agent launch or model-token spend.

## Completion evidence

The static evidence pointer above owns this record.
