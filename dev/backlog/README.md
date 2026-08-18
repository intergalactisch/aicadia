# Aicadia development backlog

> **Role / side:** ordered forward-planning index / development side.
> **Authority:** governs the backlog horizon, item states and maintenance rules.
> **Excludes:** current product contracts, decision rationale and detailed delivery evidence; see `game/docs/`, `dev/docs/concept/log/` and `dev/docs/evidence/`.

This directory keeps the forward development route visible to every builder. It is
not a game contract, decision log or research archive:

- `game/docs/` defines the accepted current contract. Active plans and this backlog
  own unfinished execution state; `dev/docs/evidence/` owns delivery results and status.
- `dev/docs/concept/log/README.md` indexes why material choices changed.
- `dev/docs/research/` preserves sourced findings.
- this backlog orders concrete outcomes and records their current planning state.

The living [player capability map](capability-map.md) shows what Users can do now,
what is selected or queued next and which concepts remain exploratory. It links to
the authorities below rather than replacing them.

If a backlog item conflicts with an authority, the authority wins and the backlog
must be corrected in the same change.

## Ordered horizon

| Horizon | Item | State | Concrete outcome |
| --- | --- | --- | --- |
| Done | [Current immersive Agent play contract](items/current-agent-play-contract.md) | Done | Every conforming Agent keeps the complete conversation inside Aicadia, renders internal World records as concrete in-world facts and fails closed without its required MCP authority. |
| Done | [Local Agent play and read-only Studio](items/local-agent-play-ledger.md) | Done | One hidden development User can return to the same local World, play only through their Agent and inspect accepted game sources and bounded World data in Aicadia Studio. |
| Done | [Agent world-entry handoff](items/agent-world-entry-handoff.md) | Done | A clean-room Agent can understand and execute World entry and Activity reads from the published MCP contract; two Agents prove shared Place state. |
| Done | [World entry with activity history](items/world-entry-history.md) | Done | A Character may remain unplaced, later enter the shared World, and receive a durable queryable history from the first accepted game actions onward. |
| Done | [Agent-mediated World action](items/agent-mediated-world-action.md) | Done | A User selects and steers one of three Agent proposals; the Agent submits one readable and structured action package that World accepts or rejects atomically. |
| Done | [Character-grounded interaction and knowledge](items/character-grounded-interaction-knowledge.md) | Done | One concrete co-present interaction produces directional Entity history and Character-specific knowledge without global World access, authored responses or control-identity leakage. |
| Done | [Local Entity Property state](items/character-property-state.md) | Done | All Entity creation routes establish initial Properties; bounded local Actions and Interactions change typed Entity-owned state atomically with current and Activity-backed reads. |
| Done | [Trait live validation](items/entity-trait-development.md) | Done | Delivery history and current status: see [Trait evidence](../../dev/docs/evidence/trait.md). |
| Done | [Current Property and Trait live validation](items/current-property-trait-live-validation.md) | Done | Delivery history and current status: see [Property evidence](../../dev/docs/evidence/property.md) and [Trait evidence](../../dev/docs/evidence/trait.md). |
| Done | [Post-correction Property and Trait live validation](items/post-correction-property-trait-live-validation.md) | Done | Delivery history and current status: see [Property evidence](../../dev/docs/evidence/property.md) and [Trait evidence](../../dev/docs/evidence/trait.md). |
| Done | [Uniform Entity creation and state-change packages](items/uniform-entity-state-packages.md) | Done | Every Entity role can be created with multiple Properties and Traits, and one Action can later change both atomically in one Activity. |
| Done | [Sol-medium combined Entity-state live validation](items/sol-medium-combined-state-validation.md) | Done | One clean-room Sol-medium smoke test grounded through MCP and committed one combined Property/Trait Action; HTTP proved both changes in one Activity. |
| Done | [First complete investigation and discovery loop](items/first-investigation-discovery-loop.md) | Done | Delivery history and current status: see [discovery evidence](../../dev/docs/evidence/discovery.md). |
| Done | [Exact-Place established-state read](items/exact-place-established-state.md) | Dropped | Supporting scope inside Agent-mediated World action instead of a separate outcome; its combined read/write plan was dropped. |
| Done | [Meta-steward Entity acceptance](items/meta-steward-entity-acceptance.md) | Dropped | Rejected privileged administrator path; the corrected direction is recorded in the item. |
| Done | [Public-facing text methodology and Agent-text rewrite](items/public-text-methodology.md) | Done | Every text published verbatim to Agents states each rule once, plainly, loop first and without schema-owned bounds, under one written method that future texts follow; no rule added, dropped or weakened. |
| Done | [Build-facing text methodology and capability-contract pass](items/build-text-methodology.md) | Done | Every document a building Agent reads states what it owns, what is not there and where that lives; every reference is a plain sentence with fact, relation and owning path; the fifteen capability contracts are the first complete application. No capability semantics change. |
| Later | Place-neighborhood context | Queued | An Agent can inspect a bounded set of explicit containing and adjacent Places through composable typed reads, without geometry or a monolithic context response. |
| Later | Place expansion and movement | Queued | Discoveries establish further Places and connections; Characters move through validated transitions while past locations remain queryable. |
| Later | Rich domain change | Queued | Flora, fauna, materials, partial boundaries and temporal processes gain concrete models only when selected gameplay needs them. |

A `Now` row is the one selected current edge. When no row is `Now`, the next edge has
not been accepted yet. `Next` and `Later` preserve sequence and dependency context;
they do not authorize implementation or promise a specific schema. This table is the
single home of backlog order and item state; item files carry no status line.

## Item states

- `Proposed`: concrete enough to review, but not accepted in `game/docs/`.
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
