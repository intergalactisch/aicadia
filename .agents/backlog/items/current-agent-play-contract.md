# Current immersive Agent play contract

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
is governed by [Agent interface](../../../docs/game/agent-interface.md); the protocol
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

- current discovery advertises only `2026-07-28`, one exact permanent player
  contract and the complete catalog without a transport session;
- the global contract treats every World value as potentially player-authored game
  data rather than instructions, without a content enum, scanner, allowlist, linter
  or model call;
- exact catalog, HTTP/MCP parity, strict invalid-state, unsupported consequence,
  retry/freshness, atomic Activity and cross-User observer tests pass unchanged in
  meaning;
- a fake local Agent process proves the adapter starts with an empty external
  workspace and isolated home/configuration, inherits only a transient authentication
  copy, supplies current required MCP, exact player instructions and stable User
  context, then removes only its owned temporary root;
- formatter, Clippy across all targets and features with warnings denied, all Rust
  tests, the disposable local launcher/adapter suite and diff integrity pass;
- independent source review finds no P0-P3 issue, allowlist, model call, prompt or
  transcript persistence, narrative linter, dead-code suppression, legacy execution
  path or unearned new game surface.

Completion proves contract delivery, host isolation and deterministic World
behavior. It cannot prove identical wording or universal instruction-following by
arbitrary LLMs. No live model run is part of this build.

Completed on 2026-08-12. Current discovery and catalog tests passed for exactly the
thirteen unchanged capability schemas and annotations. Formatter, all-target and
all-feature Clippy with warnings denied, all 58 Rust tests, the disposable local
launcher/adapter lifecycle and diff integrity passed. The lifecycle preserved one
stable User, started fake Codex in an empty external workspace with isolated home
and configuration, injected the exact player contract, required current Aicadia MCP,
removed its owned root and reported `codex_invoked=false`. Independent review found
no P0-P3 issue. No test database, temporary player root, new listener or process
remained; the pre-existing server on port 3000 was deliberately untouched.

This proves deterministic delivery and boundaries, not arbitrary model wording. The
next real conversation is a separate explicit User action.

The later Character-grounded Interaction build evolved this same delivery contract
to twelve player tools: it added `submit_interaction` and removed global
`list_entity`/`get_entity` from MCP while retaining them as loopback operator-ledger
HTTP reads. The historical thirteen-tool completion evidence above remains accurate
for the build at that time; current truth is in `docs/game/agent-interface.md`.
