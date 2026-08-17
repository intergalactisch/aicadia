# Local Agent play and read-only Studio

> **Role / side:** forward-planning item / development side.
> **Authority:** records this outcome's backlog state, dependencies and completion pointers.
> **Excludes:** current product contracts, decision rationale and detailed delivery evidence; see `docs/game/`, `docs/concept/log/` and `docs/evidence/`.

## Outcome

A developer-User can start and return to one persistent local World with one stable
hidden development User, conduct Character onboarding and all gameplay conversation
only in their own Agent, and inspect accepted game sources plus bounded World state
in Aicadia Studio. Server restart preserves the same Character, placement, Entities,
Activity and prose.

The original local-play plan is
`.agents/plans/20260812-091744-agent-only-local-play-ledger/plan.md`; its ledger
surface was absorbed by the accepted Studio plan
`.agents/plans/20260817-140952-aicadia-studio-prototype/plan.md` and unified through
`.agents/plans/20260817-152535-unified-aicadia-studio/plan.md`. The governing
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
- Studio `Game` projects owning repository sources and the exact compiled MCP
  catalog without copying their content into browser assets;
- Studio `Live` absorbs the first ledger slice and adds bounded Character-role,
  Place-role, Entity-state, Activity-detail and public-schema views;
- one unified interface provides reload-safe source, model, tool, record and storage
  links plus copyable AI-conversation references and an explicit non-persistent
  schema-snapshot download;
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
- new World behavior, schema, game-data endpoint or MCP tool; `/` only serves the
  read-only development Studio;
- discovery, movement, Place neighborhoods or later settlement behavior;
- automatic Agent launch or model-token spend.

## Completion evidence

The static evidence pointer above owns this record.
