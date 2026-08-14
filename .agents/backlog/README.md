# Aicadia development backlog

This directory keeps the forward development route visible to every builder. It is
not a game contract, decision log or research archive:

- `docs/game/` defines the accepted contract and explicitly separates published
  implementation-pending targets from executable delivery.
- `docs/concept/log/log.md` records why material choices changed.
- `docs/research/` preserves sourced findings.
- this backlog orders concrete outcomes and records their current delivery boundary.

The living [player capability map](capability-map.md) shows what Users can do now,
what is selected or queued next and which concepts remain exploratory. It links to
the authorities below rather than replacing them.

If a backlog item conflicts with an authority, the authority wins and the backlog
must be corrected in the same change.

## Ordered horizon

| Horizon | Item | State | Concrete outcome |
| --- | --- | --- | --- |
| Done | [Current immersive Agent play contract](items/current-agent-play-contract.md) | Done | Every conforming Agent keeps the complete conversation inside Aicadia, renders internal World records as concrete in-world facts and fails closed without its required MCP authority. |
| Done | [Local Agent play and World ledger](items/local-agent-play-ledger.md) | Done | One hidden development User can return to the same local World, play only through their Agent and inspect accepted Entity and Activity/prose data in a read-only browser ledger. |
| Done | [Agent world-entry handoff](items/agent-world-entry-handoff.md) | Done | A clean-room Agent can understand and execute World entry and Activity reads from the published MCP contract; two Agents prove shared Place state. |
| Done | [World entry with activity history](items/world-entry-history.md) | Done | A Character may remain unplaced, later enter the shared World, and receive a durable queryable history from the first accepted game actions onward. |
| Done | [Agent-mediated World action](items/agent-mediated-world-action.md) | Done | A User selects and steers one of three Agent proposals; the Agent submits one readable and structured action package that World accepts or rejects atomically. |
| Done | [Character-grounded interaction and knowledge](items/character-grounded-interaction-knowledge.md) | Done | One concrete co-present interaction produces directional Entity history and Character-specific knowledge without global World access, authored responses or control-identity leakage. |
| Done | [Local Entity Property state](items/character-property-state.md) | Done | All Entity creation routes establish initial Properties; bounded local Actions and Interactions change typed Entity-owned state atomically with current and Activity-backed reads. |
| Done | [Trait live validation](items/entity-trait-development.md) | Done | Delivery history and current status: see [Trait evidence](../../docs/evidence/trait.md). |
| Later | First investigation roll | Queued | World admits an Agent request, derives Character and Place, rolls first and returns a retry-stable zero or volatile positive result with neutral context. |
| Later | First discovery commit | Queued | One Agent-authored candidate becomes one validated concrete shared result plus provenance and activity history. |
| Later | Place-neighborhood context | Queued | An Agent can inspect a bounded set of explicit containing and adjacent Places through composable typed reads, without geometry or a monolithic context response. |
| Later | Place expansion and movement | Queued | Discoveries establish further Places and connections; Characters move through validated transitions while past locations remain queryable. |
| Later | Rich domain change | Queued | Flora, fauna, materials, partial boundaries and temporal processes gain concrete models only when selected gameplay needs them. |

A `Now` row is the one selected current edge. When no row is `Now`, the next edge has
not been accepted yet. `Next` and `Later` preserve sequence and dependency context;
they do not authorize implementation or promise a specific schema.

## Item states

- `Proposed`: concrete enough to review, but not accepted in `docs/game/`.
- `Queued`: ordered future outcome whose concrete contract deliberately remains open.
- `Ready`: accepted contract and boundaries exist; implementation may start.
- `Active`: currently being built; at most one item may have this state.
- `Blocked`: a named external decision or dependency prevents meaningful progress.
- `Done`: the stated outcome and evidence are complete.
- `Dropped`: deliberately removed or superseded, with the reason in the concept log.

## Maintenance contract

1. Create a detailed item only for `Now`, a genuinely near next outcome, or a
   concrete blocker. Keep unselected ideas as short horizon rows.
2. Give every item one outcome, its player or World value, accepted facts, open
   choices, explicit non-goals, dependencies and observable completion evidence.
3. Change the item in place when evidence sharpens the same outcome. If the outcome
   changes, record the reason in the concept log and replace or supersede the item.
4. Record progress as current state, not a diary. Git and the concept log retain the
   chronology; the item tells the next builder what remains true now.
5. A completed item links its contract, implementation and verification. Completion
   never follows from documentation alone.
6. Do not add point scores, estimates, owners-by-name, speculative implementation
   tasks or an item per idea. Order follows Terry's game-value gate.
