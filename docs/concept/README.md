# Concept guide

> **Role / side:** navigation index for live exploration and frozen concept history / development side.
> **Authority:** identifies each concept record's theme and exploration status.
> **Excludes:** Current implementation and game behavior, which belong in `docs/game/`; delivery evidence, which belongs in `docs/evidence/`.

Concept records explain rationale and preserve open exploration. They cannot override
the [current game contract](../game/README.md). A concept becomes current behavior
only when the owning `docs/game/` contract explicitly accepts it.

## Live themed records

| Record | Status | Theme |
| --- | --- | --- |
| [Discovery and investigation](discovery.md) | live exploration | Investigation, rolls, volatile transport, meta-state handoff, drill-down, provenance, open frontier and prototype |
| [Character-grounded knowledge](knowledge.md) | live exploration | Knowledge paths, identity ambiguity, World/Character context, shared/personal scope and retained frontier |
| [Interaction participation rationale](interaction.md) | retained rationale | Asymmetric participation; delivered behavior is a pointer to `docs/game/` |
| [Spatial direction](spatial.md) | live exploration | Place identity, sparse World and deliberately undecided geometry |
| [Time and change](time.md) | live exploration | Continuous time and the no-background-simulation boundary |
| [Tabletop-derived play heuristics](tabletop.md) | retained heuristics | Cross-cutting play guidance and non-import boundary |
| [Entity state rationale](entity-state.md) | retained rationale | Property/Trait distinction and uniform authority boundary; delivery uses its static evidence pointer |
| [Aicadia Studio](aicadia-studio.md) | active exploration | Source-backed `Game`/`Live` development interface, model/tool projection and World inspection |
| [Discovery roll lab](discovery-roll-prototype.html) | throwaway prototype | Optimistic scope-bound roll concurrency; never a contract |
| [Development log](log/README.md) | history | Accepted, rejected, deferred, corrected and superseded choices |

## Frozen July 2026 generation

The [archive index](archive/README.md) preserves documents 00–09 byte-for-byte below
their archive banners. Their scene/claim vocabulary predates the 2026-08-07 game
reframe. The development log records supersession; archived ideas are not current
implementation authority.

## Still-live ideas inside the archive

Archival status does not reject these still-unbuilt product ideas:

- [World steward](archive/03-time-and-turns.md#the-world-steward--user-direction-2026-07-26) — an administrator consciously invokes bounded proposals rather than background simulation.
- [Ripples and catch-up](archive/03-time-and-turns.md#realtime-not-turns--user-direction-2026-07-25) — causal information arriving when a Character returns, without a daily ceremony.
- [Naming economy](archive/02-canon-model.md#naming-economy--idea-from-the-debate-possibly-the-highest-leverage-rule) — ration costly new names while citation remains cheap.
- [Player-sealed envelopes](archive/05-influence-and-retention.md#mystery-supply--idea-from-the-debate) — player-authored conditional secrets as mystery supply.
- [Safe tension sources](archive/05-influence-and-retention.md#safe-tension-sources-ranked--from-the-debate) — pressure without scores, compulsory crisis or stolen volition.
- [Anti-patterns](archive/05-influence-and-retention.md#anti-patterns--never-build) — explicit rejected product shapes that remain useful guardrails.
