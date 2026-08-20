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
| Later | Place map context | Queued | An Agent can inspect one bounded coordinate window of ordinary established Places selected by resolved Position or known positive Area intersection, with bounded Place, Area and touching-Connection context plus continuation. Base Place geography is shared rather than Character-Knowledge-gated, and the read writes no Knowledge, Observation or Activity; protected and remembered map behavior remains later. Numeric proximity is read context and never automatic identity, uniqueness, merge or Connection; Place neighborhood remains the separate explicit-relationship view from one Place. |
| Later | Place expansion and movement | Queued | After bounded nearby inspection and User discussion, one confirmation discovers new B at any technically valid Agent-supplied absolute whole-centimetre `x`/`y`/`z` Position from permanent World origin `(0, 0, 0)`, with increasing values east/north/up, optional initial Entity state and A→B Connection, or reuses existing C and establishes only A→C; the Character does not move. When it has no current Place, the same preview first establishes a separately identified named origin Place A at its unchanged Position, makes A current and includes A→B; the Character itself never becomes a Place. A pre-existing or concurrently won A→C yields no duplicate or Activity and preserves the losing opportunity. Later Movement may traverse all or part of a shaped Connection and stop at an ordinary Position without journey state. The first slice has no gameplay distance limit; integer range, geographic adjacency, later timed travel, travel modes, mechanical terrain and long-term proliferation handling remain open. |
| Later | Relative Position and moving places | Queued | An Agent may deliberately establish an Entity—including a Place such as a ship's cabin—relative to exactly one Entity so it mechanically follows that reference without descendant rewrites. Its offsets start only at that reference Entity's one Position point; the foundation has no surface, part, internal-point or geometry target. Every read that returns the Position includes its immediate stored basis; bounded resolution also returns the current World point when it reaches the absolute basis, otherwise Actions needing that point fail closed. A bounded current Place read includes each already-returned Entity's complete Position without per-Entity follow-up calls or Position-specific redaction. Position has at most one optional multi-sentence current description and owns no Traits; durable independent characterizations remain Entity Traits. The fixed chain limit of 32 is rejected, Position-reference cycles are invalid and Connection cycles remain valid artistic topology. A relative Position write fails unless bounded validation proves an absolute acyclic chain against revisions still current at commit. Re-referencing supplies a complete new reference and offsets; the Agent accounts for returned durable Traits without World interpretation and may propose bounded surprising results. The [kept PostgreSQL fixture](../lab/spatial/02-postgres-position-lineage/README.md) supports lineage, cycle-race prevention, one canonical carrier move and local-work isolation only inside its scratch boundary; production integration, creative-result confirmation, remaining Entity-selection boundaries, indexing, hot-reference capacity and hosted operation remain open until this behavior becomes the selected game edge. |
| Later | Open Entity Relations | Queued | An Agent can establish and later develop one stable non-Entity Relation from one Entity to another with a free name and description, no server-owned semantic kind and no mechanical authority. Multiple Relations may coexist between the same pair. An eligible Relation may ground Agent understanding of remote causality and be cited as current context, but it never executes or authorizes the resulting exact confirmed Action; Connection, Position, Inventory and Action consequences remain separately owned truth. Reads are bounded by endpoint, direction, cursor and limit. Exact operations, text bounds, dependency revisions, privacy and duplicate presentation remain open until this behavior becomes the selected game edge. |
| Later | Area and Connection traversal context | Queued | [Focused research](../docs/research/place-area-connection-traversal.md), the completed grill and the [current technical synthesis](../docs/concept/spatial-five-year-backcast.md#technical-synthesis-after-the-completed-grill) ground the selected spatial direction and its proof gates. One stable named Connection alternative optionally owns ordered exact World points and one non-mechanical shape description; Area proves positive coverage only and crossings are derived; any positioned Entity may deliberately receive the Place role; bounded coordinate-window reads return selected map context; and Movement may stop on an exact course point without journey state. Remote button→bomb causality remains Agent-understood Relation meaning plus one exact confirmed Action under ordinary authority, not spatial state or executable Relation. The product-choice frontier and technical synthesis are complete; the Position-lineage lab is kept, while exact Connection and Area geometry, privacy, limits, concurrent traversal, public interface and accepted implementation planning remain work, not production schema. |
| Later | World-enforced private information | Queued | The spatial foundation may let a conforming Agent withhold details based on Trait or Position description, but this is not confidentiality because the Agent already received the data. A later dedicated grill must decide concrete private-inventory, Relation, visibility and derived-information behavior that World can enforce deterministically against modified Agents without a universal hidden flag or audience-wide fan-out. |
| Later | Opt-in unforeseen outcomes | Queued | After the spatial foundation, a separate grill designs how a User can deliberately invite an Agent-authored result that is surprising but grounded in current World meaning and submitted as exact bounded state with Activity. It is never default behavior, server-authored randomness, background Agent work or extra cross-User authority. `Chaos` is only the User's working label; its canonical name, invocation, preview and confirmation contract, affected-subject bounds and relationship to concrete Actions remain open. |
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
