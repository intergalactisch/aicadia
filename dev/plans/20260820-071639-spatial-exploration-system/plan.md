---
status: draft
created_at: "2026-08-20T07:16:39+02:00"
updated_at: "2026-08-20T07:35:50+02:00"
accepted_at: null
completed_at: null
---

# Build the first spatial exploration slice without losing the complete system

> **Role / side:** draft staged spatial delivery plan / development side.
> **Authority:** owns the first production slice's proposed outcome, task graph, evidence claim and the dependency gates that preserve the complete spatial direction.
> **Excludes:** accepted current gameplay, spatial concept truth, experiment verdicts and later-slice implementation authority; those remain in `game/docs/`, the active spatial synthesis, retained labs and separately accepted later plans.

## Outcome

Deliver Aicadia's first complete large-world exploration loop through `World`,
PostgreSQL, HTTP and MCP:

1. an entered Character and every deliberately established Place have one direct,
   exact whole-centimetre Position from the permanent World origin;
2. the Character's Agent can inspect one eligible, bounded map window containing
   deliberately established Places and their explicit Connections;
3. after a successful eligible discovery and explicit User confirmation, the Agent
   can either establish a new destination Place with its direct Position, initial
   Entity state and one named Connection from the current Place, or reuse an existing
   eligible Place and establish only the Connection;
4. discovery never moves the Character and an already established equivalent first
   Connection produces neither a duplicate nor another Activity; and
5. a later explicit Movement traverses all or a selected exact part of that
   Connection, changes only the Character's ordinary Position and optional current
   Place context and appends one attributable Activity atomically.

The final evidence must demonstrate the complete player outcome with two Users:
one Agent reads map context, discovers B while remaining at A, another eligible
Character can observe the accepted spatial truth, and the first Character later
stops once at an unnamed intermediate Position and then enters B. Lost-response
retry, concurrent equivalent discovery, concurrent independent travellers and a
Connection revision conflict must leave exact current state and Activity.

This plan deliberately implements only the first playable slice. Its complete-system
map below preserves every spatial and adjacent direction discussed in the grill so
later work cannot mistake this slice for the whole design.

## Non-goals

- No Entity-relative Position, moving carrier, rotation, Orientation, surface, part
  or internal-point reference in this slice.
- No Area geometry, inferred crossing, terrain mechanics, collision, travel cost,
  duration, timer, journey, completed trace or background movement.
- No Route record. An Agent may reason over Connections without saving that plan.
- No open Relation capability, Relation-driven execution or special bomb-control
  authority.
- No generic containment, inventory, holding, wearing, attachment, ownership or
  World-enforced hidden-information model.
- No server interpretation of descriptions, Properties, Traits or Connection text
  as coordinates, adjacency, access, travel mode or causality.
- No Position-specific privacy, universal visibility flag, global Place graph,
  global map revision, shared point row, region lock or graph database.
- No named `Chaos` mechanic or default surprising outcome.
- No production import or copy from the retained Position lab.
- No paid or background Agent call; a direct Agent comprehension smoke requires
  separate explicit authority if deterministic parity leaves that question open.

## Complete spatial delivery map

This table is dependency order, not implementation authorization. Every row after
S1 requires its own current contract, proportional plan, User acceptance and proof
gate before code. A later plan may narrow a row but may not silently pull it into an
earlier slice.

| Slice | Player or World outcome | Added current truth | Gate before the next slice |
| --- | --- | --- | --- |
| S0 — evidence base | The candidate can be judged before production. | No game truth; retained model-pressure and PostgreSQL Position labs only. | Completed: bounded Position lineage, cycle race, atomic rollback and carrier conflict observations are kept. |
| S1 — direct exploration | Read an eligible map, discover or reuse B, establish a Connection, remain at A, then move fully or partially. | Direct Position, deliberate positioned Place, stable named Connection with optional bounded ordered course, map window and Movement. | This plan's deterministic World, PostgreSQL, HTTP/MCP parity, concurrency and two-User outcome evidence. |
| S2 — relative worlds | Put a birdhouse on a table or a cabin on a moving ship and retrieve both stored basis and resolved point. | Entity-relative Position, bounded resolution, re-reference and moving-Place behavior without descendant writes. | Production lock/revision proof, hot-carrier capacity and no-stale-false-negative map-index proof. |
| S3 — open meaning | Let Agents retain “sleeps under” or “button activates bomb” meaning without granting mechanics. | Stable directed Relation with free name and description, bounded endpoint reads and ordinary Action citation. | Guessed-id, endpoint, count, cursor, error and timing privacy proof plus ordinary authority proof. |
| S4 — positive coverage | Show a forest, city or cave's known coverage and which covered portions a shaped Connection crosses. | Place-keyed Area, exact positive 3D coverage and derived revision-specific course intersections. | Geometry arithmetic, overlaps, disconnected coverage, pagination and boundary-change freshness proof. |
| S5 — private arrangement | Carry something in a coat or inventory without leaking its existence or allowing an ineligible removal. | Only the concrete mechanics earned by the private-inventory, holding, wearing or attachment scenarios; no universal containment flag. | Dedicated privacy/authorization grill and adversarial read/write evidence against modified Agents. |
| S6 — earned play extensions | Save a Green Route, make terrain mechanical, add timed travel, protected remote control or opt into unforeseen results only when gameplay demands each one. | One separately earned mechanic at a time; `Route`, terrain effects, protected control, surfaces and surprising-result invocation remain independent. | One concrete player scene, accepted term and separate plan per mechanic. |

### Scenario coverage ledger

| Scenario | Owning slice | Preserved result or open boundary |
| --- | --- | --- |
| SP01 — A to B | S1 | Discovery and Movement are separate; partial stop is ordinary Position. |
| SP02 — object in a coat | S5 | Exact movement, knowledge, access and privacy remain deliberately unresolved; no generic containment is assumed. |
| SP03 — distant button and bomb | S3, then S6 only if earned | Relation may ground Agent meaning; ordinary confirmed Action remains authority and spatial distance is irrelevant. |
| SP04 — cup two centimetres above table | S2, optional S6 surprise | Relative Position supplies exact mechanics, Position description narrates the current scene and the persistent floating characterization remains a Trait. |
| SP05 — dog under bridge | S3 | Relation preserves open authored meaning; Position remains separate and World derives no `under` predicate without geometry. |
| SP06 — one hundred metres from hotel | S1 | Distance is calculated from exact current points and narrated by Agent; no journey record is required. |
| SP07 — birdhouse on table in village | S2 | Nested relative grounding works without universal containment or surface targeting. |
| SP08 — forest edge into heath | S4 | Connection course plus positive Areas can prove covered portions; unknown remainder stays unknown. |
| SP09 — cabin and cargo on ship | S2 | Carrier move writes one canonical Position; interior work remains independent when it needs no external point. |
| SP10 — hidden sword in backpack | S5 | Memory, current truth, existence privacy and removal authority require a dedicated mechanic. |
| SP11 — A connects to B, not C | S1 | Connection is explicit direct topology; absence is not inferred from coordinate distance and Route remains later. |
| SP12 — many Characters at one point | S1 | No shared point owner or count; reads page and writes stay per Character. |
| SP13 — city, forest and waterfall on map | S1 then S4 | Deliberate Place plus Position makes map candidates; optional Area later adds positive coverage. |
| SP14 — cursed glasses hidden from possessor | S5 | Trait can guide a conforming Agent but confidentiality needs deterministic World state. |
| SP15 — impossible staircase | S1 | Connection topology may loop or contradict geography; Position references never cycle. |

## Evidence and authorities

| Evidence or authority | Current fact | Consequence for this build |
| --- | --- | --- |
| User direction in the completed spatial grill | Discovery establishes after confirmation and never forces entry; the User now wants one plan that preserves the whole spatial system. | S1 must be complete and the plan must retain explicit later gates rather than pretending S1 is all of spatial. |
| `dev/docs/concept/spatial-five-year-backcast.md#technical-synthesis-after-the-completed-grill` | Separate sparse Position, Place, Area, Connection and Relation truths plus per-truth revisions and atomic Activity are the current recommendation. | S1 implements only Position, Place, Connection and Movement; it may not add a generic spatial or relation layer. |
| `dev/CONTEXT.md` | Position, Place, Area, Connection and Relation already have distinct canonical meanings. | No new umbrella domain term, Location, Transform, Link, edge or containment vocabulary enters code or public contract. |
| Place, Movement, Multiplayer, Agent Play and World Change Areas | They record chosen scenario behavior, rejected conflations and remaining privacy/geometry questions. | Contract design must trace every S1 rule back to these Areas and leave later unknowns absent. |
| `dev/areas/place/scenarios.md` | SP01–SP15 are the fixed cross-system pressure catalogue. | Each production slice owns named scenario rows and later plans can audit what remains open. |
| `dev/lab/spatial/02-postgres-position-lineage/README.md` | The scratch candidate survived eight fixed local PostgreSQL 17 cases, but no production seam. | Redesign the production implementation and rerun the exact race, rollback and lock claims against real migrations. |
| `game/docs/` and current migrations | Current behavior has Place and optional Character current Place but no Position, Connection, map or Movement model/capability. | S1 deliberately evolves the runtime contract before code and backfills existing entry state without pretending spatial already ships. |
| `dev/backlog/README.md` | Spatial outcomes are queued and no current edge is selected. | Accepting this complete plan selects S1 as the one current edge; later rows remain queued. |

## Alignment

### Strategic

S1 turns the completed local discovery loop into recognizable large-world
exploration: the User can see a bounded world around the Character, add a real place
to that world without teleporting there and later choose how far to travel. It also
creates the durable seams that settlement, flora, fauna, cities, moving ships and
other players will later compose with.

The highest following risk is no longer whether one relative chain can work in a
scratch database. It is whether the production World can expose useful exploration
without global knowledge, duplicate topology, broad Place contention or a public
operation that fuses discovery and movement. S1 must retire that risk end to end.

### Tactical

The smallest complete scene is one existing entry Place A, two entered Characters,
one candidate or existing destination B, one directed walking Connection and one
Character Movement. The first Agent receives current Character Position and a
bounded eligible map response, discusses the opportunity with its User and submits
only after confirmation.

A new-destination acceptance atomically creates B's Entity, direct Position, Place
role, optional initial Property/Trait state, one Connection and one Activity with
exact roles and dependency revisions. Reuse acceptance creates only the missing
Connection and one Activity. If the equivalent first Connection already exists,
the opportunity is retained or returned as current truth without a new Activity.
The Character remains at A in every discovery result.

Movement is a second confirmed mutation. A partial move validates the named
Connection revision, direction, segment and exact target point, writes Character
Position, clears current Place and appends Activity. A complete arrival writes the
endpoint Position, sets current Place to B and appends Activity. No journey state is
created. Another traveller reading the same Connection writes only its own Position.

### Technical

The leading S1 storage shape is conventional PostgreSQL and remains subject to the
open decisions below:

- an immutable/current Position table family keyed by `entity_id`, direct World
  coordinates only in S1, with whole-centimetre signed integers and one optional
  current description;
- existing `place.entity_id` requires a Position and stores no duplicate coordinates,
  name or description;
- a stable Connection identity with immutable endpoint Places, immutable versions
  for direction, name, description and optional course description, plus bounded
  ordered whole-centimetre points for the same version;
- operation-specific immutable Activity participation and dependency rows, never a
  universal payload or event-sourced reconstruction;
- direct Position indexes supporting a bounded axis-aligned coordinate window and
  stable cursor without PostGIS, Area or a global map revision; and
- exact subject coordination: the changed Character or new Place, the selected
  Connection and its endpoint/revision dependencies only.

One `World` interface owns normalization, identity, authority, freshness,
idempotency, geometry arithmetic, transaction boundaries and errors. HTTP and MCP
remain thin semantic adapters. Every new capability ships through World, HTTP, MCP,
tool description, schema description, generated catalog and parity tests together.
World never parses text or invokes an Agent.

At million-Character pressure:

- a quiet Position change touches no region, World row or Place-wide revision;
- a hot unnamed point owns no row or count, while bounded map and current-Place reads
  return a stable page rather than all occupants;
- travellers on a hot Connection share its short revision dependency but lock and
  update only their own Character Position;
- editing that Connection is the honest conflict and rejects stale Movement;
- concurrent equivalent discovery coordinates only the exact first alternative,
  never the Place graph; and
- every read first selects eligible ids in bounded form, then hydrates Position and
  Connection state without guessed-id or count leakage beyond the accepted S1 rules.

## Decisions, assumptions and open questions

### Confirmed decisions

- Keep one truth per domain concept and no universal spatial/Relation table — active
  technical synthesis and canonical vocabulary.
- Position has Entity identity, exact `x`/`y`/`z` whole centimetres, permanent World
  origin and at most one optional description — completed grill.
- S1 uses only direct Position; relative Position is a complete later slice even
  though its database evolution must remain possible — Terry order.
- Place is a deliberate role of a positioned Entity; Position never promotes it
  automatically — Place Area.
- Connection is a stable non-Entity direct travel alternative with explicit
  direction, name, description and optional ordered exact course; it is not a Route,
  Relation or physical road Entity — Place and Movement Areas.
- Proximity, Area overlap and prose never create or merge Place or Connection —
  Place Area.
- A map window is Character-scoped: it never reveals every established Place merely
  because its Position falls inside the coordinates. The current Place and Places
  the Character has structurally learned through accepted play are the eligible
  candidates; the exact smallest current knowledge representation remains open —
  User direction, 2026-08-20.
- S1 stores sparse current Character-owned Knowledge for each exact Place made
  knowable by an accepted act; the same Discovery also makes its exact Connection
  knowable. Reads create no receipt and Activity retains establishment time.
  Whether those two current subject families share one polymorphic row shape or use
  separate typed associations is the next open design choice — User choice A and
  corrected Laravel/PostgreSQL feasibility, 2026-08-20.
- Discovery may create a Place at any technically representable distance or reuse an
  existing eligible Place; neither result moves the Character — Discovery and
  Movement Areas.
- Movement may stop on an exact course point or arrive, writes ordinary Position and
  Activity and has no durable journey — Movement Area.
- Connection loops and geographically impossible topology are artistically valid;
  Position-reference cycles remain invalid — SP15.
- Every accepted mutation is idempotent and appends exact Activity in the same
  transaction; unrelated subjects share no correctness lock — global build rules.

### Reversible assumptions

- S1 map geometry is one direct axis-aligned World-coordinate window over direct
  Place points; Area intersection waits for S4. This avoids PostGIS and cannot change
  the chosen Place or Position meaning.
- Existing entry Place A is backfilled at World origin and each Character currently
  at A receives the same direct World point. The migration fixture must prove this
  against existing data before adoption.
- S1 has exactly one walking alternative in its player scene and therefore no travel
  mode field. Stable Connection identity and versioning still allow later parallel
  alternatives after a separately accepted behavior.
- S1 course membership uses exact checked integer segment arithmetic. The selected
  coordinate and course limits remain technical representability/admission bounds,
  never gameplay distance limits.
- Studio needs only the smallest read-only projection required to inspect the new
  current rows and Activity; it receives no gameplay action.

### Open questions blocking activation

1. **Knowledge scope and storage type.** Player consequence: A has been chosen—one
   sparse durable association remembers that Character M knows Place B, and the
   accepted Discovery already makes its exact Connection knowable too. Should these
   two current subjects share one Knowledge shape, or receive separate typed rows?
   Knowledge of a subject means only that its identity is eligible for a typed,
   authorized current read; it never means every fact about that subject is known.
   Technical consequence: a Laravel-style composite primary key
   `(character_entity_id, subject_type, subject_id)` can efficiently enforce bounded
   Character lookup and uniqueness, but one PostgreSQL foreign key cannot choose a
   target table from `subject_type`. `user_id` still assigns in-World Knowledge to
   control provenance and `seen_at` still confuses current eligibility with
   Observation history. See the
   [primary-source storage research](../../docs/research/polymorphic-character-knowledge-storage.md).
   **Preference:** now prefer one polymorphic row shape limited to the two current
   stable aliases `place` and `connection`, with the Character-leading natural key,
   establishing Activity and deterministic typed World validation in the same
   transaction. Add no surrogate Knowledge id, User owner, read receipt, arbitrary
   type string, Entity or Relation target. This earns one table from two present
   consumers while leaving native target-FK strictness as the exact choice to grill.
2. **Discovery admission.** Player consequence: must spatial expansion consume a
   successful current Investigation attempt, or may an Agent propose a new Place
   after ordinary exploration discussion? Technical consequence: this decides
   whether the current `submit_discovery` transaction evolves or a separate confirmed
   operation owns Place expansion. **Preference:** evolve the successful Discovery
   path so finding and confirming one opportunity remain coherent, while movement
   stays separate.
3. **First Connection deduplication.** Player consequence: when another Agent already
   established an A→C alternative, should the opportunity reuse that exact
   Connection or may it still establish a different named alternative? Technical
   consequence: parallel Connections are accepted long-term, so endpoint uniqueness
   cannot masquerade as permanent identity. **Preference:** S1 explicitly reuses a
   selected eligible Connection when supplied; only an exact idempotency/concurrent
   creation key deduplicates a new alternative.
4. **Public capability boundaries and names.** Player consequence: which small tool
   set lets an Agent read map context, confirm expansion and move without exposing
   internal rows? Technical consequence: names fix World methods, HTTP routes, MCP
   tools and published context cost. **Preference:** one bounded map read, the
   existing Discovery confirmation evolved for expansion, and one conventional
   Movement operation; do not publish raw Position or Connection CRUD.
5. **S1 observation after departure and arrival.** Player consequence: who sees a
   Character at an unnamed intermediate Position or at B, and through which bounded
   read? Technical consequence: current Place reads cannot select a Character with
   no current Place, while a global Position lookup is rejected. **Preference:** the
   moving Character always receives itself; other Characters need an explicit
   eligible map/local observation path, not guessed-id Position access.
6. **History vocabulary.** Player consequence: Activity clearly says a Place was
   discovered/connected or a Character moved. Technical consequence: exact new
   Activity operations, Entity roles and Position/Connection dependency rows must be
   fixed without a generic spatial payload. **Preference:** distinct expansion and
   Movement operations with typed roles; reuse existing `subject`, `destination` and
   `location` only where their current meanings remain exact.
7. **Representability and request bounds.** Player consequence: no arbitrary gameplay
   travel cap, but malformed or enormous windows/courses fail clearly. Technical
   consequence: choose coordinate bound, page size, window volume, course-point
   count, text limits and statement/lock budgets before publishing schemas.
   **Preference:** conservative server constants proven with checked arithmetic and
   cursors; no unit enum, floating point or distance rule.

The plan remains `draft` until these questions are resolved in the active spatial
record and current log. Later-slice questions do not block S1 because their concepts
are explicitly absent here and require child plans.

## Implementation map

| Surface | Current state | Intended S1 change | Invariants |
| --- | --- | --- | --- |
| `game/docs/model/` | Entity, Character, Place and Activity only; Place has no Position contract. | Add Position and Connection contracts; evolve Place, Character and Activity only for accepted S1 state. | Entity identity remains singular; no future-slice concept enters current contract. |
| `game/docs/capability/`, protocol, parity and Agent contract | No map, spatial expansion or Movement public capability. | Add only the resolved bounded public operations and exact errors, Activity and parity rules. | World/HTTP/MCP semantic parity; public text written once per owning layer. |
| `game/docs/storage.md`, `game/migration/0011_*.sql` | No Position or Connection tables. | Add immutable/current S1 storage, backfill existing entry/current Character state, exact constraints and current indexes. | Migration succeeds on representative existing data; no generic spatial table or global coordinator. |
| `game/src/world/` | Place-local placement and broad Place revision; no coordinate or topology module. | Add deep Position/Connection/map/Movement seams and operation-specific transactions. | One World interface; checked arithmetic; stable lock order; no prose inference. |
| `game/src/wire/`, `game/src/server/` | Existing HTTP/MCP adapters and error taxonomy. | Add the chosen inputs, outputs, routes/tools and thin mappings. | Bounded schemas; user context; identical semantic errors. |
| `game/mcp/agent/`, `game/src/agent_contract.rs`, catalog | Existing published play loop and fifteen tools. | Teach the selected spatial loop briefly and publish each new tool once; regenerate catalog once. | No internal model/field exposition in player conversation; context cost stays proportional. |
| `game/tests/world/` | No production spatial state or concurrency proof. | Prove schema, backfill, arithmetic, authority, idempotency, exact Activity, discovery races, Movement conflicts and hot-subject isolation. | Claims limited to production World and disposable PostgreSQL fixture exercised. |
| `game/tests/server/` | Fifteen-capability HTTP/MCP parity. | Prove wire parity, bounds, errors, catalog completeness and the two-User S1 flow through both adapters. | No adapter-only capability or semantic divergence. |
| `studio/` | Read-only current World/Entity/Activity projection. | Add only the minimum bounded inspection of current Position/Connection state if required for local verification. | Studio remains read-only and is not an Agent capability. |
| `dev/docs/evidence/` | No delivered spatial slice. | Record deterministic delivery evidence and exact non-claims after implementation. | Lab evidence is not promoted; production evidence names real and absent seams. |
| Areas, synthesis, backlog and concept log | Full direction and forward state exist. | Update only changed current direction, selected edge and final bounded delivery result. | Full system remains in its owning records; S1 plan never becomes duplicate product truth. |

## Execution contract

Root owns outcome, open-question resolution, plan state, integration and the final
evidence claim. A delegated Agent receives this plan path and one dependency-ready
task id, re-reads live authorities, changes only its owned surfaces, runs focused
evidence and returns raw results. No delegated task edits this plan unless plan
maintenance is its explicit assignment.

No S1 code, schema, `game/docs` contract or public text may change while this plan is
`draft`. Activating this plan authorizes S1 only. S2–S6 remain a preservation map and
each requires a separately accepted plan.

## Task graph

Allowed states are `pending`, `in_progress`, `completed` and `blocked`.

| ID | State | Depends | Parallel-safe | Objective | Owned surfaces | Evidence |
| --- | --- | --- | --- | --- | --- | --- |
| T1 | pending | — | no | Publish the resolved S1 runtime contract and exact migration design. | `game/docs/`, plan file map | Documentation lint and contract inventory/parity audit. |
| T2 | pending | T1 | no | Persist and read direct Position, positioned Place and versioned Connection safely on existing Worlds. | migration, World models and storage tests | Schema/backfill/constraint/lineage tests on real PostgreSQL. |
| T3 | pending | T2 | no | Return the accepted eligible bounded map and Connection context without unbounded or unauthorized hydration. | World reads, wire-independent models and World tests | Pagination, eligibility, hot-point and query-bound tests. |
| T4 | pending | T3 | no | Accept new-or-reused Place expansion plus Connection atomically while Character remains at A. | World mutation, Activity and World tests | Retry, failure rollback, reuse and synchronized equivalent-discovery race tests. |
| T5 | pending | T4 | no | Move fully or partially over one exact Connection with independent travellers and stale-edit rejection. | World Movement, Activity and World tests | Segment arithmetic, arrival/intermediate, retry and concurrency tests. |
| T6 | pending | T5 | no | Ship S1 through HTTP, MCP and concise Agent text with complete semantic parity. | wire, server adapters, MCP sources, catalog and server tests | Adapter parity, schema/error/catalog and two-User protocol tests. |
| T7 | pending | T6 | no | Prove the complete S1 outcome, align authorities and record exact evidence/non-claims. | evidence, Areas/synthesis/log/backlog, plan | Validation ladder, focused scope review and clean builder brief. |

## Task details

### T1 — Resolve and publish the S1 contract

**Objective:** Every S1 actor, action, state, authority, error, bound and Activity
role has one accepted runtime home before schema or code.

**Actions:**

1. Incorporate the seven resolved open questions and freeze public capability
   boundaries, input/output shape and error taxonomy.
2. Add Position and Connection model contracts and evolve Place, Character, Activity,
   storage, protocol, parity, Agent play and deferred scope once each.
3. Design the backfill, exact indexes, lock order, idempotency key and typed
   dependencies against current production tables.

**Invariants:**

- No later-slice record or dormant code path enters the S1 contract.
- Every new term already exists in canonical vocabulary or returns to negotiation.
- The contract describes player consequences before storage representation.

**Evidence:**

- `cargo test -p aicadia-studio --test studio the_documentation_lint_is_clean` — all
  current authorities, links and model storage claims are coherent.
- Focused public-text inventory — every S1 rule has exactly one future publication
  owner before any generated catalog change.

**Stop conditions:**

- Stop and return the plan to `draft` if the resolved contract needs Area, Relation,
  privacy, Route, terrain mechanics or another new domain concept.

### T2 — Build the direct Position and Connection foundation

**Objective:** Existing and new S1 subjects have exact current direct Position and
Connection state with immutable history and no Place-wide spatial revision.

**Actions:**

1. Add the accepted migration, including deterministic backfill and rollback-safe
   constraints for current production data.
2. Implement deep Position and Connection storage/read modules behind World.
3. Prove direct coordinate bounds, immutable lineage, current pointers, endpoint
   validity, course ordering and exact indexes on real PostgreSQL.

**Invariants:**

- Place copies no Entity or Position fields.
- Position and Connection revisions never overload `place.latest_activity_id`.
- Quiet writes touch only their exact coordinators and Activity rows.

**Evidence:**

- `cargo test -p aicadia-game --test world` — focused schema, migration, backfill,
  constraint and storage cases pass against disposable PostgreSQL.
- A synchronized production A↔B candidate-reference test is required only if S1
  stores a reference field; otherwise relative code is absent and that lab claim is
  not borrowed.

**Stop conditions:**

- Stop before adding a global revision, Position id, duplicate Place coordinates,
  dormant relative behavior or lab-code import.

### T3 — Build bounded eligible map context

**Objective:** An Agent can obtain exactly the accepted Character-eligible Place and
Connection page for one bounded direct-coordinate window.

**Actions:**

1. Implement eligibility-first candidate selection, stable cursor and maximum page.
2. Hydrate selected Place, Position and touching Connection state in bounded query
   work without counts or per-row follow-up queries.
3. Cover hot points, boundary coordinates, concurrent writes and cursor stability.

**Invariants:**

- Numeric proximity is read context, never Place identity, merge or Connection.
- A guessed Entity/Place/Connection id cannot bypass the chosen eligibility rule.
- No window produces an unbounded response or exact total count.

**Evidence:**

- Focused World map tests compare one result with the maximum page and assert bounded
  query count, stable continuation and no unauthorized hydration.

**Stop conditions:**

- Stop if eligibility requires World to infer visibility from prose or prebuild the
  later private-information system.

### T4 — Build confirmed spatial expansion

**Objective:** One successful eligible discovery confirmation creates or reuses the
destination and exact Connection without moving the Character.

**Actions:**

1. Implement new-Place and existing-Place variants with one normalized fingerprint.
2. Validate current Character, source Place/Position, destination eligibility,
   coordinates, Connection input, expected revisions and bounds before atomic write.
3. Store Entity, optional initial Properties/Traits, Position, Place, Connection,
   Activity and typed dependencies in one transaction.
4. Prove exact retry, rejected failure, existing reuse and synchronized equivalent
   proposals with complete state/history readback.

**Invariants:**

- Character Position/current Place never changes in expansion.
- Losing or no-op equivalent proposals write no Activity or orphan row.
- Distance, text and nearby candidates never decide admission semantically.

**Evidence:**

- Focused production World tests prove both variants, lost-response replay, injected
  rollback and the exact concurrent first-alternative contract.

**Stop conditions:**

- Stop if correctness needs endpoint-wide permanent uniqueness that contradicts
  later parallel Connections or any global Place/Connection lock.

### T5 — Build complete and partial Movement

**Objective:** A Character can traverse the accepted direction to B or stop at one
exact course point while other travellers remain independent.

**Actions:**

1. Implement checked point-on-segment and direction validation against exact current
   Connection and Character Position revisions.
2. Set ordinary Position plus cleared/intermediate or destination current Place and
   one Activity atomically.
3. Prove retry, stale endpoint/Connection/Character input, no-op rules, partial then
   complete travel and concurrent independent travellers.

**Invariants:**

- No journey, timer, trace, Connection occupancy or traveller count is stored.
- Movement changes only exact named subjects and never rewrites the Connection.
- Another traveller on the same course is not a conflict.

**Evidence:**

- Focused World Movement tests prove exact arithmetic, state, Activity and conflict
  isolation, including a deliberately held Connection editor.

**Stop conditions:**

- Stop if S1 requires duration, terrain cost, multiple travel modes or an inferred
  destination.

### T6 — Publish adapter parity and Agent guidance

**Objective:** A conforming Agent can use every S1 capability through the same World,
HTTP and MCP semantics without learning internal structure.

**Actions:**

1. Add bounded wire inputs/outputs, HTTP routes and MCP tools for the accepted public
   capabilities only.
2. Add concise Agent instructions and tool descriptions through the public-text
   inventory procedure; regenerate the catalog once.
3. Prove HTTP/MCP input, success, retry and error parity plus catalog completeness.

**Invariants:**

- No adapter owns gameplay or publishes raw CRUD.
- Player conversation renders named places, things, movement and opportunity rather
  than internal ids, fields, revisions or delivery status.
- Every public bound stays in schema and every rule stays in one owning text layer.

**Evidence:**

- `cargo test -p aicadia-game --test server` — HTTP/MCP and catalog parity pass.
- Focused two-User adapter scenario independently reads back authoritative World
  state and Activity.

**Stop conditions:**

- Stop and inventory public text again if one capability or error diverges between
  World, HTTP, MCP or generated catalog.

### T7 — Demonstrate and record S1

**Objective:** The promised exploration loop is reproducible, current authorities
agree and the next spatial risk is explicit without starting S2.

**Actions:**

1. Run the full validation ladder and inspect exact current/history state after the
   two-User scenario and concurrency cases.
2. Record production delivery evidence and its exact real/absent seams.
3. Align Areas, active synthesis, concept log, backlog and this plan once each.
4. Select no next edge automatically; report whether S2, Relation, Area or a
   non-spatial game outcome now has highest value.

**Invariants:**

- Lab observations remain lab observations and are not restated as production proof.
- The complete-system map remains visible after S1 and later slices remain absent.
- No known-stale `game/docs`, public text, generated catalog or backlog state remains.

**Evidence:**

- `cargo test --workspace` — current deterministic workspace suite passes.
- `cargo test -p aicadia-studio --test studio the_documentation_lint_is_clean` —
  record placement, model tables, indexes and links are coherent.
- `cargo brief` — exactly one completed S1 result, no accidentally active next edge
  and the full later spatial horizon remain visible.
- `git diff --check` — all planned changes are whitespace-clean.

**Stop conditions:**

- Stop before starting S2 or claiming production scale, privacy, Agent comprehension
  or geometry that the final evidence did not exercise.

## Validation ladder

1. **Focused:** each task's migration, World, arithmetic, query-bound, race, adapter
   and documentation checks pass at its real seam.
2. **Contract:** Position, Place, Connection, Movement and Activity rules agree
   across `game/docs`, PostgreSQL, World, HTTP, MCP, public Agent text and generated
   catalog; all later concepts remain absent.
3. **Outcome:** two Users reproduce map read → confirmed B expansion while staying at
   A → independent observation → partial Movement → arrival at B, with exact retry,
   concurrent equivalent expansion and independent traveller evidence.
4. **Integrity:** `cargo test --workspace`, documentation lint, `git diff --check`,
   focused diff review and confirmation that unrelated User changes and governing
   authorities remain intact.

## Change control

Refine file paths, task order, internal helper boundaries and stronger evidence in
place while S1's accepted player outcome, public semantics and exclusions remain
unchanged. Stop implementation, set `status: draft`, revise and request explicit
re-acceptance if new evidence changes eligibility, discovery admission, Connection
identity, Movement meaning, public operations, Activity roles, irreversible data,
external authority, material cost or the evidence claim.

The complete-system map may be corrected when a later accepted choice changes its
own slice. It never authorizes pulling S2–S6 state into S1.

## Completion conditions

- all seven blocking questions are resolved and recorded before activation;
- T1–T7 are `completed` and the validation ladder passes;
- the exact S1 player outcome and evidence claim are demonstrated through World,
  PostgreSQL, HTTP and MCP;
- current contract, vocabulary, Areas, synthesis, backlog, public text and evidence
  agree without duplicating one another;
- the scenario ledger still shows where SP01–SP15 continue after S1;
- no known-stale authority, material open S1 question or accidental later-slice
  implementation remains; and
- `status: complete` and `completed_at` are recorded only after these conditions.
