# Local Agent play and World ledger

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
[Agent interface](../../../docs/game/agent-interface.md) and
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

## Delivered boundary

The delivered slice is one non-destructive launcher, one private ignored profile,
one self-contained same-origin ledger, revised Character-onboarding Agent
instructions and persistence/browser evidence. The first real User conversation is
the next qualitative experiment; deterministic completion evidence proves readiness
and persistence, not model output quality. Public hosting remains outside this local
slice and requires a separately accepted read-only exposure and deployment boundary.

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

- the lifecycle suite provisions one User and reuses that same UUID after restart;
  missing-profile, concurrent-launch, corrupt/stale-profile and occupied-port cases
  all fail closed without a second User or listener;
- disposable integration state retained the identical User, Character, Place, three
  Entity records, four Activity records and canonical prose across restart, both in
  canonical HTTP JSON and a fresh browser page;
- browser network and source boundaries are GET-only and contain no gameplay input;
- MCP initialization and the exact catalog publish the Character workshop while
  retaining the existing action workshop;
- desktop, mobile, pagination, keyboard disclosure and pre-Character browser checks
  pass without exposed User UUID, viewport overflow or browser errors;
- Bash syntax and lifecycle, formatting, strict Clippy, all 59 Rust tests,
  `git diff --check` and independent risky-seam review pass; and
- current contract, concept log, backlog item and accepted plan agree. The explicitly
  disposable evidence database, state directory, listener and browser tabs were
  removed after ownership checks; unrelated working-tree changes were preserved.
