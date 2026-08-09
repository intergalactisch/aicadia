# Aicadia development backlog

This directory keeps the forward development route visible to every builder. It is
not a game contract, decision log or research archive:

- `docs/game/` defines accepted executable behavior.
- `docs/concept/log/log.md` records why material choices changed.
- `docs/research/` preserves sourced findings.
- this backlog orders concrete outcomes and records their current delivery boundary.

If a backlog item conflicts with an authority, the authority wins and the backlog
must be corrected in the same change.

## Ordered horizon

| Horizon | Item | State | Concrete outcome |
| --- | --- | --- | --- |
| Now | [World entry with activity history](items/world-entry-history.md) | Proposed | A Character may remain unplaced, later enter the shared World, and receive a durable queryable history from the first accepted game actions onward. |
| Next | Local Character context and history inspection | Queued | An Agent can read the current Character, optional exact Place, local established state and bounded personal history without selecting another Character. |
| Next | First investigation roll | Queued | World admits an Agent request, derives Character and Place, rolls first and returns a retry-stable zero or volatile positive result with neutral context. |
| Later | First discovery commit | Queued | One Agent-authored candidate becomes one validated concrete shared result plus provenance and activity history. |
| Later | Place expansion and movement | Queued | Discoveries establish further Places and connections; Characters move through validated transitions while past locations remain queryable. |
| Later | Rich domain change | Queued | Flora, fauna, materials, partial boundaries and temporal processes gain concrete models only when selected gameplay needs them. |

Only the `Now` row is a candidate for current work. `Next` and `Later` preserve
sequence and dependency context; they do not authorize implementation or promise a
specific schema.

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
