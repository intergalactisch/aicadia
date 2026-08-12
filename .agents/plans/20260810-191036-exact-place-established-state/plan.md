---
status: dropped
created_at: 2026-08-10T19:10:36+02:00
updated_at: 2026-08-11T12:32:07+02:00
accepted_at: null
completed_at: null
dropped_at: 2026-08-11T12:32:07+02:00
dropped_reason: The plan incorrectly made direct create_entity placement part of the player outcome; Exact-Place reading is now a supporting query for the Agent-mediated bundled World action.
---

# Exact-Place established-state read

Planning state: dropped. The Exact-Place read remains a queued supporting capability,
but this plan's combined direct-create/write model must not be implemented.

## Outcome

An Agent whose Character has entered the World can inspect the current shared Entity
state established directly at that Character's exact current Place, without supplying
a User, Character or Place id. Two Characters at the same stored Place receive the
same Place-scoped state; an Entity outside that exact Place does not appear.

“Current Place” means the durable Place referenced by
`character.current_place_entity_id` at the moment of the call. Each contextual read
derives the User from request context, finds the one owned Character and joins that
stored Place. `null` means the Character has not entered; there is no Agent-supplied
selector, coordinate, session location or inference from Activity.

An Agent explicitly requests Entity introduction through `create_entity`; `World`
alone validates, accepts and creates durable state, then returns the Entity or a
deterministic error. The required nullable `place` field contains a Place's Entity
id: `null` requests an unlocated shared referent; an id requests placement at any
existing Place. There is no administrator, private meta path, durable proposal or
human confirmation step.

Final evidence must demonstrate through `World`, HTTP and MCP that a User can
establish an Entity at an existing Place independently of the Character's current
Place, two Characters at that target Place read it, and a Character at another Place
cannot accidentally receive that state.

## Non-goals

- investigation requests, rolls, candidates, discovery commits or claims;
- additional Places, movement, containment, distance, geometry or visibility;
- moving, updating or deleting an established Entity;
- per-observer discovery, secrecy, ownership, scores or authorization expansion;
- designing or building the future choose-one-of-three Character onboarding;
- replacing global `list_entity` or `get_entity`;
- deriving current location later from Activity history.

## Evidence and authorities

| Evidence or authority | Current fact | Consequence for this build |
| --- | --- | --- |
| `docs/game/README.md` Entity listing | `list_entity` is global and has no User context | Add a separate contextual read; preserve the global catalog |
| `docs/game/README.md` Activity | `create_entity` records historical `context_place` | Do not treat history as current Entity location |
| `docs/game/README.md` Character | World already derives nullable current Place from User context | Only the contextual read requires that derived Place; create validates its supplied target independently |
| `migration/0004_world_entry_activity.sql` | `character.current_place_entity_id` is the stored Place FK set by `enter_world` | Use that exact value as the actor and read scope |
| `.agents/backlog/README.md` | Exact-Place established-state read is the first queued game edge | This is now the selected edge, not investigation |
| `docs/research/persistent-game-spatial-model.md` | Exact stored Place equality is the researched first slice | Exclude containment, distance and visibility |
| `docs/concept/10-discovery-and-world-context.md` | Exact operation names and fields remain undecided | Resolve semantics before naming or code |
| User choices in this task | An Agent requests; World autonomously and authoritatively accepts or rejects, creates accepted state and returns it | Keep one synchronous player capability and define its deterministic acceptance predicate |
| User onboarding direction in this task | A future User chooses one of three Character candidates before ordinary play | Define the contextual precondition here; design selection as its own later edge |

## Alignment

### Strategic

World entry currently gives Agents shared placement but not meaningful local state.
This slice makes “being here” useful: Agents can ground later investigation in the
same authoritative surroundings. It advances shared-world discovery without yet
introducing randomness, authored candidates or discovery mechanics. The next risk
after this slice is whether an investigation can safely read and version this local
state before producing a candidate.

### Tactical

The smallest complete slice contains two semantic capabilities:

1. an Agent submits one Entity candidate either unlocated or targeted at an existing
   Place, and World accepts or rejects it synchronously under one deterministic
   command; and
2. a placed Character lists Entity summaries whose stored current Place equals that
   same server-derived Place.

The write stores current location and Activity atomically. The read is idempotent,
paged and writes no history. Exact equality is only this read's inclusion rule, not
a universal co-presence or visibility rule. Behavior for missing Character and
unplaced Character must be explicit and adapter-identical.

### Technical

`World::create_entity` remains authoritative World behavior exposed through thin HTTP
and MCP adapters. An Agent submits
`CreateEntity { name, description, place: Option<EntityId> }`; the Agent never creates
state itself. `World` validates the complete command, assigns Entity identity and
commits accepted state. JSON represents `place` as a Place Entity UUID or `null`, not
a nested selector object.
`null` preserves unlocated creation. A non-null id
must resolve to an existing Place; World then inserts Entity and location and records
Activity atomically. Create derives no target from Character placement. User
provenance comes from the existing request context; whether an actor Character is
required remains an explicit grill choice.

A dedicated optional `entity_location` relation remains keyed by `entity_id` and
indexed for current lookup by database foreign key `place_entity_id`; this avoids a
second ambiguous location field on embedded Character Entities. Activity keeps the
single `create_entity` operation. It links the new Entity as `subject` and, when
located, the target Place as `location`; optional actor Character and `context_place`
continue to describe who acted and where that Character was. The distinct
User-context read joins location to Entity, filters on the derived current Place and
reuses `(introduced_at, entity_id)` pagination. HTTP and MCP remain thin adapters
over the same World types and errors.
The existing complete `Entity` result is preserved because successful acceptance
already confirms the supplied location and the contextual read exposes current Place
membership.

Wire input rejects an absent or malformed `place` field as `invalid_request` with
HTTP `400`. A non-null id that does not identify an existing Place returns
`place_not_found`/`404`; an Entity without a Place role is equally not a Place.
Under the current recommendation create adds no Character-specific error. The
contextual read retains `character_not_found`/`404` and
`character_not_entered`/`409`.

## Decisions, assumptions and open questions

### Confirmed decisions

- This is the selected current game-development edge.
- Agent candidate authorship, User provenance and World authority are distinct: an
  Agent may submit content on a User's behalf, but only World validates, assigns the
  Entity id and accepts durable state. An unaccepted candidate is not a World Entity.
- `create_entity` remains one player-facing Agent capability through World, HTTP and
  MCP. Its name describes a requested World command; only World writes or accepts
  durable state.
- No human administrator, meta Agent, private operator path, confirmation phase or
  durable proposal record is introduced. World responds synchronously with the
  accepted Entity or a deterministic error.
- The contextual read derives User, Character and current Place and accepts none of
  those ids. Consolidated create accepts `place` in addition to Entity content and
  never derives its target from Character placement.
- First-slice inclusion uses exact stored Place equality only.
- Current Entity location is explicit current state, not inferred from Activity.
- `create_character`, not `create_entity`, introduces the current User's Character
  Entity role. Its `current_place` remains nullable until World entry.
- The consolidated create input uses `place`, never `at_current_place`. The field is
  required and its value is either a Place Entity id or `null`; it is not a nested
  Place object. A non-null id may name any existing Place, independent of the User's
  Character's current Place. There is no ownership, discovery or known-Place gate in
  this slice.
- Consolidated `create_entity` keeps returning the complete base `Entity`. It does
  not add a `CreatedEntity` wrapper or add location to base Entity.
- Ordinary Entities may have an optional current Place relation. Absence is valid;
  “most concrete occurrences are placed” is expected game content, not a database
  cardinality rule. This slice creates the relation only together with a new local
  Entity and does not locate or move an existing Entity.
- A User without a Character is valid only during provisioning/onboarding and is not
  playable. The User's Agent may explicitly propose three transient candidates;
  only the selected candidate becomes the User's one durable Character Entity.
  Building that selection flow remains a separate edge.
- Future onboarding composes Character creation, any first-World genesis and
  `enter_world` as separate accepted actions inside one guided flow. An interrupted
  unplaced Character is valid but not playable; resuming continues World entry.
- A non-null id that is not an existing Place yields `place_not_found` and HTTP
  `404`. The Exact-Place read requires a placed Character: no Character retains
  `character_not_found`/`404`, and an unplaced Character yields
  `character_not_entered`/`409`. It never converts absent Place context into a
  misleading empty page. Create provenance uses the request's responsible User context;
  whether creation requires that User to have a Character remains open.
- `list_entity_at_current_place` returns the complete derived `place`, paged Entity
  summaries and `next`; HTTP exposes it as GET `/api/place/current/entity`.
- The capability ships through World, HTTP and MCP with one semantic contract.
- Located and unlocated creation remain non-idempotent: every accepted call creates
  one new Entity, even when its semantic input equals an earlier call. This preserves
  current `create_entity` identity semantics and adds no idempotency-key state.
- Activity keeps one `create_entity` operation. Both forms link the new Entity as
  `subject`; located creation also links the target Place as `location`. Actor and
  `context_place` retain their existing meanings, so history distinguishes actor
  location from subject placement even when they differ.
- Reads, rejected writes and Agent reasoning create no Activity.

### Reversible assumptions

- Reuse the current Entity summary ordering and typed cursor inside the confirmed
  Place-scoped page; validate exact scope behavior in World tests.

### Open questions

1. What exactly makes an Entity request “suitable/possible” for World acceptance?
   **Recommendation:** limit the first slice to facts World can check deterministically
   now: valid User context, bounded normalized name and description, `place: null` or
   an existing Place, and successful database constraints. Do not judge semantic
   quality, lore fit, plausibility or duplicate meaning.
2. Does `create_entity` require the requesting User to already have a Character?
   **Recommendation:** no. Placement describes the new Entity and may target any
   existing Place, so a Character prerequisite adds no authority or validation value.

Keep this plan `draft` until these material questions are resolved and the complete plan
is explicitly accepted.

## Implementation map

| Surface | Current state | Intended change | Invariants |
| --- | --- | --- | --- |
| `docs/game/README.md` | Global Entity list; no Entity location | Define accepted local write/read, storage, errors and evidence | Current MVP truth remains explicit; deferred mechanics stay absent |
| `docs/game/agent-interface.md` | Ten capabilities including `create_entity` | Extend create with nullable `place` and add the Exact-Place read | The Agent requests; World accepts and writes; the read derives current Place |
| `migration/0005_*.sql` | No current Entity location | Add only the confirmed location relation, constraints and current lookup index | One subject identity; no inferred history or orphan rows |
| `src/world.rs`, `src/lib.rs` | Global Entity write/read | Add domain types, errors, transaction and exact-Place query | Dumb strict World; atomic state/history; KISS |
| `src/wire.rs` | Existing Entity and cursor wire types | Add semantic inputs/outputs and error mapping | HTTP and MCP share parsing and output |
| `src/server.rs` | Ten HTTP/MCP capabilities | Extend create adapters and add the read adapter | Thin adapters preserve one World contract and complete catalog parity |
| `tests/world.rs` | World persistence and Activity tests | Prove location, exact equality, boundaries, atomicity and pagination | Evidence matches only the accepted contract |
| `tests/server.rs`, `tests/agent-tool-catalog.json` | Ten-capability parity fixture | Prove HTTP/MCP/OpenAPI/catalog parity and context derivation | Published catalog remains complete |
| `.agents/backlog/`, `docs/concept/log/log.md` | Edge queued; grill not yet recorded | Maintain selected scope, decisions and completion evidence | Planning never overrides `docs/game/` |

## Execution contract

Root owns grill sequencing, plan state, integration and the final evidence claim.
After acceptance, a delegated Agent may receive this plan path plus exactly one
dependency-ready task. It must re-read live files, change only owned surfaces, run
focused evidence and return raw results. Tasks remain sequential unless root proves
their write surfaces independent; delegation is optional.

## Task graph

| ID | State | Depends | Parallel-safe | Objective | Owned surfaces | Evidence |
| --- | --- | --- | --- | --- | --- | --- |
| T1 | pending | — | no | Install the accepted behavior contract and current edge | `docs/game/*`, backlog item, concept log | contract contains every confirmed actor/action/state/error/history boundary |
| T2 | pending | T1 | no | Implement persistent exact-Place state and World behavior | migration, `src/world.rs`, `src/lib.rs`, `tests/world.rs` | focused World tests prove transaction, exact equality and boundary errors |
| T3 | pending | T2 | no | Ship HTTP/MCP parity and Agent contract | `src/wire.rs`, `src/server.rs`, `tests/server.rs`, catalog fixture | both adapters and complete catalog expose identical semantics |
| T4 | pending | T3 | no | Prove the outcome and close authorities | all changed files, backlog, plan | full tests, formatting, strict lint, focused scenario and diff review pass |

## Task details

### T1 — Accepted contract

First finish the active grill and obtain explicit acceptance of this complete plan.
Then record the final contract in `docs/game/`, link the backlog item and amend the
concept record. Stop if the deterministic acceptance predicate or Character
prerequisite remains open.

### T2 — Persistence and World

Add the confirmed current-location schema and only its earned index. Extend the
existing create mutation so `place: null` creates no location and a supplied id is
validated as a Place, while Entity, optional location and Activity commit or roll
back together under the accepted Character rule. Implement the contextual
read with exact Place equality and typed keyset pagination. Prove creation targeting a
Place other than actor context, the accepted Character boundary, missing Place,
missing/unplaced read context, same-Place inclusion, non-local exclusion, persistence,
concurrency and rollback.

Stop if implementation requires movement, existing-Entity relocation, visibility,
containment, generic claims or a second identity for Entity or Place.

### T3 — Adapters and catalog

Expose the extended create capability and new contextual read through HTTP and MCP
with shared wire types, errors and schemas. Publish complete descriptions that
explain nullable `place`, independent target validation, actor versus target context,
server-derived read scope, retry behavior and exclusions. Extend parity tests and the
exact catalog fixture in the same change.

### T4 — Outcome evidence

Run focused and full Rust evidence, formatting and strict lint. Demonstrate a User
targeting an existing Place independently of Character placement, two Characters at
that Place sharing the Entity result and a boundary Character not receiving it.
Align `docs/game/`, concept log and backlog with exact evidence, preserve unrelated
changes and close this plan. A paid Agent run is not assumed; request explicit
approval only if it becomes necessary for a claim.

## Validation ladder

1. **Storage/World:** migration and World tests prove one current exact Place per
   local Entity, any-existing-Place targeting, actor/target history separation,
   atomic Activity, exact equality, ordering and boundary errors.
2. **Contract parity:** HTTP and MCP tests prove identical inputs, outputs, errors,
   operation catalog and generated schemas.
3. **Outcome:** two same-Place Characters observe the same locally established
   Entity; an invalid context cannot leak or fabricate Place state.
4. **Repository:** `cargo fmt --check`, `cargo clippy --all-targets --all-features
   -- -D warnings`, `cargo test`, catalog fixture review and `git diff --check` pass.

## Change control

Refine paths, internal helper placement and stronger evidence in place once the plan
is accepted. Return to `draft`, grill and request re-acceptance if the local/global
Entity distinction, public operations, location cardinality, context errors,
history semantics or evidence claim changes.

## Completion conditions

- every open material question is answered and the plan explicitly accepted;
- T1–T4 and the validation ladder pass;
- the exact same-Place player outcome is demonstrated through World, HTTP and MCP;
- global Entity behavior and every listed non-goal remain intact;
- current contract, concept record, backlog and plan agree with no stale authority.
