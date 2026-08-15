# Delivery and evidence history

> **Role / side:** delivery/evidence index and history / evidence bridge.
> **Authority:** owns current delivery status, retained runs, audits and proof links.
> **Excludes:** game-contract rules, concept rationale and planning state; see `docs/game/`, `docs/concept/` and `.agents/`.

Delivery detail lives only under `docs/evidence/`. Every pointer from another home
is static: it names this evidence record without repeating status, results, digests,
candidate ids or audit findings.

| Slice | Current status | Proof |
| --- | --- | --- |
| World entry | Complete, including retained live proof | [World-entry evidence](world-entry.md) · [Agent runner contract](runner/agent-playtest.md) |
| Local play and ledger | Deterministic launcher, handoff, persistence and browser proof complete | [Local-play evidence](local-play.md) |
| Character-grounded Action | Deterministic delivery and bounded live proof complete | [Action evidence](action.md) · [Agent runner contract](runner/agent-playtest.md) |
| Character-grounded Interaction | Deterministic evidence complete; no paid-model Interaction claim | [Interaction evidence](interaction.md) · [Agent runner contract](runner/agent-playtest.md) |
| Entity Property | Deterministic evidence complete; the fresh paid flow reached exact World, HTTP and independent-observer content but terminal-failed on a valid optional observer limit; current controller corrected and pinned token-free, no retry authority | [Property evidence](property.md) · [Agent runner contract](runner/agent-playtest.md) |
| Entity Trait | Deterministic delivery and complete paid establish/develop/independent-observer validation succeed; exact seven-call candidate, lineage, both HTTP gates and ownership-safe cleanup passed | [Trait evidence](trait.md) · [Trait runner contract](runner/trait-playtest.md) |
| Uniform Entity state packages | Deterministic creation and combined Action delivery complete across World, HTTP and MCP; no new paid/model call | [Uniform Entity-state evidence](entity-state.md) |
| Investigation and discovery | Deterministic World/HTTP/MCP delivery, real restart-stable retry and executed token-free Agent orchestration complete; no paid/model discovery call | [Discovery evidence](discovery.md) · [Agent runner contract](runner/agent-playtest.md) |
| Agent contract delivery | Current deterministic host/catalog delivery complete; historical catalog milestones remain bounded history | [Agent contract delivery](#agent-contract-delivery) |

The post-correction combined audit first found one P3 in stale active-planning text.
After that text was corrected without changing evidence, its read-only
`gpt-5.6-sol` high re-audit returned GO with P0–P3 all zero. The audit itself made
zero paid/model calls, playtests, server/database mutations or repository edits.

## Agent contract delivery

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
`list_entity`/`get_entity` from MCP while retaining them as loopback operator-ledger
HTTP reads. That historical thirteen-tool completion evidence remains accurate for
the build at that time; current truth is in `docs/game/README.md#capability-catalog`.
