---
status: complete
created_at: "2026-08-18T22:39:02+02:00"
updated_at: "2026-08-18T22:47:34+02:00"
accepted_at: "2026-08-18T22:44:41+02:00"
completed_at: "2026-08-18T22:47:34+02:00"
---

# Establish the spatial scenario catalogue

> **Role / side:** execution plan for one spatial-design documentation slice / development side.
> **Authority:** owns the bounded tasks, state and evidence for establishing the reusable spatial scenario catalogue.
> **Excludes:** accepted game behavior, canonical spatial vocabulary, schema and runtime implementation; those remain in `game/docs/`, `dev/CONTEXT.md` and the runtime surfaces.

## Outcome

A future building Agent can pressure every Position, Place, Area, Connection,
Relation, Movement, visibility and remote-action proposal against one concrete
catalogue before introducing another term or schema. The catalogue preserves fixed
player and World narratives plus open questions; it chooses no solution. Final
evidence must prove that each requested scene and the added edge cases have exactly
one narrative owner and are discoverable from every affected Area.

## Non-goals

- Choosing whether “two centimetres above” is Position, Relation, geometry or a combination.
- Choosing coordinates, relative movement, Route, inventory, geometry, privacy or remote-action mechanics.
- Changing `dev/CONTEXT.md`, `game/docs/`, schema, code, public Agent text, backlog or runtime behavior.
- Creating a new Spatial Area or duplicating full scenarios across existing Areas.

## Evidence and authorities

| Evidence or authority | Current fact | Consequence for this build |
| --- | --- | --- |
| User direction, 2026-08-18 | Concrete scenarios must precede further spatial model choices and may be added to the appropriate Areas. | The catalogue records pressure and questions, never a preferred model. |
| `dev/areas/README.md` | Place owns location, placement and spatial structure at rest; Areas are intentionally overlapping. | One catalogue lives under Place and affected Areas point to it. |
| `dev/areas/multiplayer/scenarios.md` | Multiplayer already uses one fixed scenario catalogue with narratives, fixtures and questions. | Reuse its catalogue discipline without copying its World-change scenarios. |
| `dev/areas/place/README.md` | Position, Place, Area, Connection, open spatial meaning and visibility remain partly unresolved. | The catalogue must pressure each open boundary without settling it. |
| `dev/areas/movement/README.md` | Movement between Places, unnamed Positions and Routes remains open. | Travel and progress scenes must link to Movement. |
| `dev/areas/world-change/README.md` | Remote consequences need explicit bounded structural basis and deterministic settlement. | The remote button scene must separate distance from causality and link to World Change. |
| `dev/docs/concept/spatial.md` | The active frontier has repeatedly overfit a term before testing all examples. | Record scenario-first sequencing and leave Q15b unresolved. |

## Alignment

### Strategic

Spatial structure is a core dependency of large-scale exploration, settlement and
multiplayer. A fixed scenario catalogue prevents a convenient cup-and-table model
from silently failing travel, terrain, inventories, moving carriers, hidden state or
remote Actions. The next risk is not missing schema; it is choosing a spatial truth
that only solves one narrative.

### Tactical

Create one Place-owned scenario catalogue containing the User's six scenes and the
smallest additional hard cases needed to expose nesting, privacy, moving Places,
terrain, topology, concurrent co-location and Place qualification. Every entry has a
fixed narrative, explicit known/unknown facts and open questions. Add only concise
links and scenario-component pointers to affected Areas. No expected outcome,
accepted terminology or implementation shape is added.

### Technical

This is development documentation only. World behavior, PostgreSQL, migrations,
transactions, Activity, HTTP, MCP, public text and operations are not applicable.
Massive-concurrency pressure remains explicit inside the catalogue: each candidate
must address bounded reads, one hot subject, relative-movement fan-out, concurrent
mutation and privacy without the catalogue selecting an implementation.

## Decisions, assumptions and open questions

### Confirmed decisions

- Work scenario-first before resuming the spatial model grill — direct User instruction.
- Preserve the exact requested scenes and add further pressure cases — direct User authorization.
- Keep one narrative home and link from affected Areas — `dev/docs/README.md` one-home rule.
- Keep Q15b and every scenario outcome open — the User explicitly rejected the current explanation as insufficiently clear.

### Reversible assumptions

- `dev/areas/place/scenarios.md` is the narrowest owner because Place owns placement and spatial structure at rest; links make Movement, Agent Play and World Change participation explicit. This can move without changing scenario meaning if the Area constitution later gains a broader owner.
- Scenario ids use `SP01` onward only as catalogue labels, never as domain vocabulary.

### Open questions

- None blocks catalogue creation. Every product and model question exposed by a scenario remains deliberately open for the subsequent one-question-per-turn grill.

## Implementation map

| Surface | Current state | Intended change | Invariants |
| --- | --- | --- | --- |
| `dev/areas/place/scenarios.md` | Absent | Add the one spatial scenario catalogue with use rules, cross-scenario probes and fixed narratives. | No scenario chooses a model, term, schema or expected result. |
| `dev/areas/place/README.md` | Refers to a few examples informally. | Link the catalogue as prepared pressure and identify it as a component. | Place remains owner of synthesis, not Movement decisions. |
| `dev/areas/movement/README.md` | Has no reusable spatial scenario source. | Link the catalogue for movement and progress pressure. | No Route or movement behavior is accepted. |
| `dev/areas/agent-play/README.md` | Has no reusable spatial scenario source. | Link the catalogue for Agent expression, grounding and privacy pressure. | Public Agent contract remains unchanged. |
| `dev/areas/world-change/README.md` | Links only the Multiplayer catalogue. | Link the spatial catalogue for remote and multi-subject spatial pressure. | No generic World-change kernel or remote mechanic is accepted. |
| `dev/docs/concept/spatial.md` | Owns the retained design tree and current unresolved rationale. | Record scenario-first sequencing and the catalogue boundary. | Current choices remain distinct from scenario questions. |
| `dev/docs/concept/log/2026-08.md` | Records Q15b's unresolved state. | Append the accepted scenario-first method and bounded catalogue scope. | History stays append-only; no prior entry is rewritten. |

## Execution contract

Root owns outcome, scope, plan state, integration and the final evidence claim. No
delegation is needed. The task changes only the listed development documents and
stops if catalogue placement would require a new Area or any scenario answer becomes
current truth.

## Task graph

Allowed states are `pending`, `in_progress`, `completed` and `blocked`.

| ID | State | Depends | Parallel-safe | Objective | Owned surfaces | Evidence |
| --- | --- | --- | --- | --- | --- | --- |
| T1 | completed | — | no | Establish one linked, solution-neutral spatial scenario catalogue. | `dev/areas/place/scenarios.md`, four affected Area READMEs, `dev/docs/concept/spatial.md`, `dev/docs/concept/log/2026-08.md` | Focused content audit, Studio documentation lint and `git diff --check`. |

## Task details

### T1 — Write and connect the spatial scenarios

**Objective:** Every requested and added spatial hard case has one fixed narrative,
explicit unknowns and discoverable cross-Area pressure without implying an answer.

**Actions:**

1. Add catalogue use rules and cross-scenario probes for observability, movement,
   concurrent change, bounded reads, hot subjects and exact-versus-authored meaning.
2. Add the six requested narratives plus nesting/settlement, terrain transition,
   moving carrier, hidden inventory, direct topology, co-location and Place-role
   qualification cases.
3. Link the one catalogue from Place, Movement, Agent Play and World Change and
   record the scenario-first sequencing in the concept trail.

**Invariants:**

- Scenario prose uses current vocabulary or marks working words as unaccepted.
- A scenario states questions, never expected behavior or a schema recommendation.
- Relation, Position, movement inheritance, Route, geometry and visibility stay separate open questions.
- Existing Multiplayer scenario narratives remain owned by their catalogue and are linked rather than copied when equivalent.
- Unrelated dirty worktree changes remain untouched.

**Evidence:**

- `rg` audit over the catalogue — every required narrative and pressure dimension is present exactly once.
- `cargo test -p aicadia-studio --test studio the_documentation_lint_is_clean` — governed documents and links remain valid.
- `git diff --check` — no whitespace errors.

**Stop conditions:**

- Stop and return to the User if writing a scenario requires choosing a new domain
  term, expected outcome, schema, authority or privacy contract.

## Validation ladder

1. **Focused:** audit ids, requested narratives, added edge cases, questions and Area links.
2. **Contract:** Studio documentation lint passes; `game/docs/` and public Agent text are untouched.
3. **Outcome:** the next grill question can cite several scenarios and compare candidate models without inventing missing facts.
4. **Integrity:** `git diff --check`, focused diff review and confirmation that unrelated user changes and all governing authorities remain intact.

Completion evidence: the focused audit found thirteen uniquely headed scenarios,
every required narrative and all four Area links; the Studio documentation lint
passed; `git diff --check` reported no error; and `game/docs/`, schema, runtime and
public Agent surfaces remained outside the owned changes.

## Change control

Refine wording, scenario order and stronger evidence in place while the accepted
outcome remains unchanged. Stop implementation, keep `status: draft`, revise and
request explicit re-acceptance when new evidence changes catalogue ownership,
domain meaning, public behavior, scope, authority or the evidence claim.

## Completion conditions

- T1 is `completed` and the validation ladder passes;
- every requested scene and added hard case has one solution-neutral narrative;
- affected Areas link to the same catalogue and no current choice is contradicted;
- no known-stale authority or accidental unrelated change remains; and
- `status: complete` and `completed_at` are recorded only after these conditions.
