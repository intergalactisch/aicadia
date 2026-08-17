---
status: "Current deterministic host/catalog delivery complete; historical catalog milestones remain bounded history"
---

# Agent contract delivery

> **Role / side:** Agent-contract delivery and evidence history / evidence bridge.
> **Authority:** owns the retained host, catalog and isolation completion record for the Agent play contract.
> **Excludes:** the current Agent-facing contract and published catalog, and launcher/Studio proof; see `docs/game/agent.md`, `docs/game/README.md#capability-catalog` and `local-play.md`.

The following delivery record is retained from the completed Agent-contract build:

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
  tests, the disposable local launcher/adapter suite and diff integrity pass; and
- independent source review found no P0-P3 issue, allowlist, model call, prompt or
  transcript persistence, narrative linter, dead-code suppression, legacy execution
  path or unearned new game surface.

Completion proved contract delivery, host isolation and deterministic World
behavior. It did not prove identical wording or universal instruction-following by
arbitrary LLMs. No live model run was part of this build.

The shared 2026-08-12 launcher, Agent-handoff and catalog proof is retained once in
[local-play evidence](local-play.md).

The later Character-grounded Interaction build evolved this same delivery contract
to twelve player tools: it added `submit_interaction` and removed global
`list_entity`/`get_entity` from MCP while retaining them as loopback Studio Live
HTTP reads. That historical thirteen-tool completion evidence remains accurate for
the build at that time; current truth is in `docs/game/README.md#capability-catalog`.
