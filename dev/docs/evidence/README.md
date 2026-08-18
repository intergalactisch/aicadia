# Delivery and evidence history

> **Role / side:** delivery/evidence navigation index and history / evidence bridge.
> **Authority:** identifies each evidence slice, its proof links and the evidence-machine operation contracts.
> **Excludes:** Each slice's own current status, which lives in its front matter; game-contract rules, concept rationale and planning state; see `game/docs/`, `dev/docs/concept/` and `.agents/`.

Delivery detail lives only under `dev/docs/evidence/`. Every pointer from another home
is static: it names this evidence record without repeating status, results, digests,
candidate ids or audit findings. Each slice carries its own current status in its
front matter.

## Slices

- [World entry](world-entry.md) · [Agent runner contract](runner/agent-playtest.md)
- [Local play and Studio](local-play.md)
- [Character-grounded Action](action.md) · [Agent runner contract](runner/agent-playtest.md)
- [Character-grounded Interaction](interaction.md) · [Agent runner contract](runner/agent-playtest.md)
- [Entity Property](property.md) · [Agent runner contract](runner/agent-playtest.md)
- [Entity Trait](trait.md) · [Trait runner contract](runner/trait-playtest.md)
- [Uniform Entity state packages](entity-state.md)
- [Investigation and discovery](discovery.md) · [Agent runner contract](runner/agent-playtest.md)
- [Agent contract delivery](agent-contract.md)

## Runner contracts

- [Agent playtest runner](runner/agent-playtest.md)
- [Trait playtest runner](runner/trait-playtest.md)

The post-correction combined audit first found one P3 in stale active-planning text.
After that text was corrected without changing evidence, its read-only
`gpt-5.6-sol` high re-audit returned GO with P0–P3 all zero. The audit itself made
zero paid/model calls, playtests, server/database mutations or repository edits.
