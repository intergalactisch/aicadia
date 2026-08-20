---
status: active
created_at: "2026-08-20T07:16:39+02:00"
updated_at: "2026-08-20T15:53:34+02:00"
accepted_at: "2026-08-20T14:51:58+02:00"
completed_at: null
---

# Build the first spatial exploration slice without losing the complete system

> **Role / side:** draft staged spatial delivery plan / development side.
> **Authority:** owns S1's proposed player outcome, exact technical contract, dependency-ordered tasks and evidence claim until explicit User acceptance.
> **Excludes:** accepted current gameplay, spatial concept truth, experiment verdicts and later-slice implementation authority; those remain in `game/docs/`, the active spatial synthesis, retained labs and separately accepted later plans.

## Outcome

Deliver Aicadia's first complete large-world exploration loop through `World`,
PostgreSQL, HTTP and MCP:

1. every deliberately established Place, every entered Character and every spatially
   established Entity has one direct exact whole-centimetre Position from permanent
   World origin;
2. an entered Character's Agent can inspect one bounded coordinate window of
   ordinary shared Places, page the Connections touching one selected Place and read
   one selected Connection's complete bounded course;
3. after one positive Investigation and explicit User confirmation, the Agent can
   discover an Entity at the Character's exact Position or establish a new or reused
   destination Place plus one newly identified Connection without moving;
4. if connected-Place discovery begins without a current Place, the same confirmed
   transaction deliberately creates or explicitly reuses a genuine origin Place at
   the Character's unchanged Position and makes it current;
5. a later confirmed `move_character` traverses an unshaped Connection completely or
   a shaped Connection completely or partially, changes only that Character's
   Position and optional current Place and writes exact Activity atomically; and
6. exact retries return their accepted result, while independently confirmed
   Connections remain distinct even when their endpoints, direction, text and course
   are equal.

The final deterministic evidence must demonstrate this with two Users. User one
reads A, discovers B while remaining at A, stops once at an unnamed intermediate
Position and then arrives at B. User two independently reads B and its Connection as
shared geography. Lost-response retry, two concurrent equal-looking Connection
establishments, stale Character Position, injected rollback and independent
travellers must leave exact current state and Activity.

This plan implements only S1. The delivery map below preserves the complete spatial
and adjacent direction so a later builder cannot mistake the first playable slice
for the whole system.

## Non-goals

- No Entity-relative Position, moving carrier, rotation, Orientation, surface, part
  or internal-point reference.
- No Area geometry, inferred crossing, terrain mechanics, collision, travel cost,
  duration, timer, journey, completed trace or background movement.
- No Route record. An Agent may reason over several Connections without persisting a
  travel plan.
- No open Relation capability, Relation-driven execution or special bomb-control
  authority.
- No generic containment, inventory, holding, wearing, attachment, ownership or
  World-enforced hidden-information model.
- No Observation, Knowledge, recognition, private memory or automatic read receipt.
  Their accepted exploration direction remains preserved for a later slice.
- No three-Place or loose-Position entry choice. S1 only positions the current single
  entry flow; distributed entry remains a separate player outcome.
- No Connection update, deletion, retirement or version lineage. An S1 Connection is
  immutable after creation; later development must earn its own operation and plan.
- No Place Position update or generic Position CRUD. S1 Position changes occur only
  through the concrete creation, entry, Discovery and Character Movement operations
  named here.
- No server interpretation of descriptions, Properties, Traits or Connection text
  as coordinates, adjacency, access, travel mode, visibility or causality.
- No Position-specific privacy, global Place graph, global map revision, shared point
  row, endpoint-pair uniqueness, region lock, graph database or PostGIS dependency.
- No named `Chaos` mechanic or default surprising outcome.
- No production import or copy from the retained Position lab.
- No paid or background Agent call. A direct Agent comprehension smoke needs separate
  explicit authority only if deterministic protocol evidence leaves that risk open.

## Complete spatial delivery map

This table is dependency order, not implementation authorization. Every row after
S1 requires a current contract, proportional plan, explicit User acceptance and
proof gate.

| Slice | Player or World outcome | Added current truth | Gate before the next slice |
| --- | --- | --- | --- |
| S0 — evidence base | Judge the candidate before production. | No game truth; retained model-pressure and PostgreSQL Position labs only. | Completed bounded Position lineage, rollback, retry and concurrency observations remain experimental. |
| S1 — direct exploration | Read shared Place geography, discover B, establish a Connection, remain at A, then move fully or partially. | Direct Position, deliberate positioned Place, immutable named Connection with optional bounded course, map reads and Character Movement. | This plan's World, PostgreSQL, HTTP/MCP, concurrency and two-User evidence. |
| S2 — distributed entry | Introduce a Character at one of three explained Places or a World-selected loose Position and then let play begin. | Entry candidate selection and one retry-stable initial Position; no participation state. | Eligible-Place selection, random-anchor bounds, no hot row and replay evidence. |
| S3 — relative worlds | Put a birdhouse on a table or cabin on a moving ship and retrieve stored basis plus resolved point. | Entity-relative Position, bounded resolution, re-reference and moving-Place behavior without descendant writes. | Production cycle/lock proof, hot-carrier capacity and no-stale-false-negative map-index proof. |
| S4 — open meaning | Retain “sleeps under” or “button activates bomb” meaning without granting mechanics. | Stable directed Relation with free name and description and bounded endpoint reads. | Guessed-id, endpoint, cursor, authority and privacy proof. |
| S5 — positive coverage | Show a forest, city or cave's known coverage and which covered portions a Connection crosses. | Place-keyed Area, exact positive 3D coverage and derived revision-specific course intersections. | Geometry arithmetic, overlap, disconnected coverage, pagination and freshness proof. |
| S6 — situated memory | Let a Character privately remember several explicit encounters and retrieve their chronology. | Immutable Character-owned Observation occurrences and bounded `list_observation`; Knowledge remains separate and parked. | Target existence, private reads, hot-subject isolation, retry and Agent-conduct evidence. |
| S7 — private arrangement | Carry something in a coat or inventory without leaking its existence or allowing ineligible removal. | Only the concrete inventory, holding, wearing or attachment mechanics earned by those scenes. | Dedicated privacy/authorization grill and adversarial modified-Agent evidence. |
| S8 — earned extensions | Save a Green Route, make terrain mechanical, add timed travel, protected remote control or opt into unforeseen results only when gameplay demands each one. | One separately earned mechanic at a time. | One concrete player scene, accepted term and separate plan per mechanic. |

### Scenario coverage ledger

| Scenario | Owning slice | Preserved result or boundary |
| --- | --- | --- |
| SP01 — A to B | S1 | Discovery and Movement are separate; partial stop is ordinary Position. |
| SP02 — object in a coat | S7 | Exact placement, access and privacy remain unresolved; no generic containment is assumed. |
| SP03 — distant button and bomb | S4, then S8 if earned | Relation may ground Agent meaning; confirmed Action retains authority and distance is irrelevant. |
| SP04 — cup two centimetres above table | S3, optional S8 surprise | Relative Position supplies mechanics, Position description narrates the scene and persistent floating remains a Trait. |
| SP05 — dog under bridge | S4 | Relation may preserve authored meaning; World derives no `under` predicate in S1. |
| SP06 — one hundred metres from hotel | S1 | Agent calculates distance from exact points; no journey record is required. |
| SP07 — birdhouse on table in village | S3 | Nested relative grounding works without universal containment or surface targeting. |
| SP08 — forest edge into heath | S5 | Course plus positive Areas can prove covered portions; unknown remainder stays unknown. |
| SP09 — cabin and cargo on ship | S3 | Carrier move changes one canonical Position; local work remains independent. |
| SP10 — hidden sword in backpack | S7 | Current truth, existence privacy and removal authority need a dedicated mechanic. |
| SP11 — A connects to B, not C | S1 | Connection is explicit direct topology; absence is not inferred from distance. |
| SP12 — many Characters at one point | S1 | No shared point owner or count; reads page and writes stay per Character. |
| SP13 — city, forest and waterfall on map | S1 then S5 | Place plus Position supplies a map point; Area later adds positive coverage. |
| SP14 — cursed glasses hidden from possessor | S7 | Trait may guide a conforming Agent; confidentiality needs World-enforced state. |
| SP15 — impossible staircase | S1 | Multi-Connection topology may cycle or contradict geography; Position references never cycle. |

## Evidence and authorities

| Evidence or authority | Current fact | Consequence for S1 |
| --- | --- | --- |
| Completed spatial grill and active record | Product branches through Position, Place, Connection, course, Movement, map visibility, loose discovery and parallel establishment are resolved. | No further product question blocks a complete technical proposal; User acceptance of this plan remains required. |
| `dev/docs/concept/spatial-five-year-backcast.md#technical-synthesis-after-the-completed-grill` | Separate Position, Place, Connection, Relation and Area truths with subject-local revisions are recommended. | S1 implements only the direct Position, Place, immutable Connection and Movement subset and may deliberately be smaller than the later candidate. |
| `dev/CONTEXT.md` | Position, Place, Area, Connection, Relation, Movement and Observation already have distinct canonical meanings. | No new umbrella domain term, Location, Transform, Link, edge or containment vocabulary enters the contract. |
| Place, Discovery, Movement, Agent Play and Multiplayer Areas | They own accepted scenario behavior and remaining later boundaries. | Every S1 contract rule traces to those Areas; deferred behavior remains absent. |
| `dev/areas/place/scenarios.md` | SP01–SP15 are the fixed pressure catalogue. | The ledger must still expose every scenario owner after delivery. |
| `dev/lab/spatial/02-postgres-position-lineage/README.md` | The scratch candidate survived eight fixed PostgreSQL cases but exercised no production seam. | Rebuild against production tables and repeat only the claims S1 actually uses; never import or promote lab code. |
| Current `game/docs/`, migrations and runtime | Place and optional current Place exist; Position, Connection, map and Movement do not. Investigation currently finds one Entity at current Place. | T1 must evolve the current contract before code, preserve current Entity discovery and backfill only facts the repository can establish exactly. |
| `dev/backlog/README.md` | Spatial outcomes remain queued and no current edge is selected. | Explicit acceptance selects S1 as the sole current edge; later slices remain queued. |

## Plan review verdict

The technical review found the prior draft complete in direction but too broad in
implementation. This revision makes four KISS corrections without losing accepted
truth:

1. Observation and `list_observation` move from S1 to S6 because neither is required
   to prove shared geography, discovery or Movement.
2. Three-Place or loose-Position `enter_world` moves to S2 because its eligibility
   and random-anchor behavior are a separate player outcome. S1 still gives the
   current single-entry flow exact Position.
3. Connection development and version lineage are absent. S1 creates immutable
   Connections; stale Movement is proved against Character Position, not an editor
   operation that does not exist.
4. One broad map response becomes three conventional bounded reads: `list_place`,
   `list_connection` and `get_connection`. A list never hydrates up to 12,800 course
   points merely because one page contains many alternatives.

The review also corrects the obsolete race claim. Concurrent equal-looking new
Connections may both commit; endpoint equality is not a conflict or deduplication
key. Only same-request replay deduplicates automatically.

## Alignment

### Strategic

S1 turns the delivered local discovery loop into recognizable large-world
exploration. A User can inspect a sparse shared World, add a real destination without
teleporting and later choose how far to travel. It establishes durable seams for
settlement, flora, fauna, moving ships and other Characters without prebuilding those
mechanics.

The highest current risk is whether production World can support that scene without
global knowledge, adding broad Place contention, unbounded course hydration or a
mutation that fuses discovery with travel. S1 retires that risk for its new spatial
operations; the delivered older exact-Place mutation pointer remains an explicit
non-claim.

### Tactical

The smallest complete scene uses current entry Place A, two entered Characters, one
new or existing destination B, one newly established A→B Connection and one moving
Character. The first Agent reads bounded Place context, discusses the result with its
User and submits a positive Investigation result only after confirmation.

New destination acceptance creates B's Entity, direct Position, Place role, optional
initial Property/Trait state, one Connection and one Activity. Existing destination
acceptance leaves B unchanged and creates the new Connection. If no current Place
exists, the same package first creates or explicitly selects origin A at the
Character's exact Position and makes it current. The Character Position never changes
in Discovery.

If the User chooses a returned existing Connection instead, there is no shared-state
mutation to submit: no Activity is written and the positive opportunity remains.
Choosing a new alternative always creates a new Connection, even when another one
looks equivalent or commits concurrently.

Movement is a second confirmed mutation. A partial move writes one new Character
Position, clears current Place and appends Activity. Complete arrival writes the
endpoint Position, sets B current and appends Activity. No journey, occupancy or
Connection progress is stored. Another traveller reads the same immutable Connection
and writes only its own Position.

## Resolved S1 contract proposed for acceptance

No blocking product or technical choice remains below. These exact choices become
current only if the User explicitly accepts this plan.

### Public capabilities

| Capability | Exact S1 consequence | Bound and eligibility |
| --- | --- | --- |
| existing `get_character` | Returns the controlled Character's complete current direct Position or its absence, plus current Place. | One controlled Character; no separate Position lookup. |
| existing `list_entity_at_current_place` | Includes complete Position for every Entity already selected by that bounded read. | Existing page bound; no Position-specific fan-out or redaction. |
| new `list_place` | Returns Place Entity summary and current Position for one inclusive axis-aligned World box. | Controlled entered Character, opaque box-bound cursor, default 25 and maximum 100; no count. |
| new `list_connection` | Returns bounded summaries of Connections incident to one exact Place: stable id, endpoints, direction, name, description and whether a course exists. | Exact Place anchor, opaque cursor, default 25 and maximum 100; no course points or count. |
| new `get_connection` | Returns one selected incident Connection including its complete optional course and shape description. | Exact Place anchor plus Connection id; wrong or non-incident ids share `not_found` semantics. |
| evolved `start_investigation` | Agent selects mechanical `kind: entity_at_position` or `connected_place`; World retains its current roll, retry, hourly and live-positive rules and grants at most one result. | Controlled entered Character with current Position; attempt binds exact Position revision and optional current Place. |
| evolved `submit_discovery` | Tagged result matching the attempt kind either finds one Entity at the bound Position or establishes the complete origin/destination/Connection package. | Existing positive attempt, exact retry id, confirmation prose and fresh bound Position/current Place. |
| new `move_character` | Moves the controlled Character completely or partially over one exact Connection and appends Activity. | One Character, one Connection, expected Position revision, exact direction/target and request id. |

HTTP uses `GET /api/place`, `GET /api/place/{place_id}/connection`,
`GET /api/place/{place_id}/connection/{connection_id}` and
`POST /api/character/movement`. MCP uses the capability names in the table. Existing
Investigation and Discovery routes and names remain; no compatibility variant or
second spatial-discovery workflow is retained.

The semantic error contract distinguishes invalid Position/window/course input,
stale spatial context, unavailable or non-incident Connection, disallowed direction,
off-course Movement, retry conflict and retryable bounded database contention. HTTP
and MCP expose identical meanings while guessed ids do not reveal which eligibility
check failed.

### Investigation and Discovery

- The Agent, not the User or World prose parser, chooses the typed Investigation kind
  from current authoritative context. User conversation may advise that choice but
  does not alter chance, seed or result.
- `entity_at_position` preserves the delivered natural-find behavior. The new Entity
  receives a direct Position equal to the attempt's exact Character Position. When a
  current Place exists, existing `entity_location` also associates it with that Place;
  otherwise it remains a positioned Entity between Places.
- `connected_place` permits one new or existing destination. A new destination
  includes Entity name, description, existing bounded Property/Trait input and exact
  direct Position. An existing destination names one returned Place and changes none
  of its current state.
- With a current Place, that Place is the origin. Without one, the confirmed input
  either supplies one new origin Entity/Place at the exact Character Position or
  selects an existing returned Place whose Position equals it exactly. Coordinate
  equality never selects the origin automatically.
- Every accepted connected-Place result creates one new immutable Connection. The
  Connection input supplies required name and description, forward A→B direction,
  optional reverse permission and either no course or one bounded course.
- A pre-existing Connection is reused conversationally after `list_connection` or
  `get_connection`; no no-op `submit_discovery` variant exists merely to record reuse.
- The existing Place-based discovery-pressure window is used when current Place was
  present at Investigation start. A loose Position has no truthful aggregation owner,
  so its local discovery count is zero; the existing per-User hourly and live-positive
  limits still bound abuse. After connected discovery creates an origin Place, later
  attempts use that Place normally.
- The attempt stores its chosen kind, Character identity, exact Position revision and
  nullable current Place. Settlement rejects any changed Position or current Place.
  Retry returns the stored roll and never rerolls.
- A nearby Place read remains Agent conduct before confirmation, not a server session
  or read receipt. World validates the structured package and never proves which
  earlier reads the Agent performed.

### Position persistence and backfill

S1 uses the repository's immutable-version/current-pointer pattern and deliberately
adds no dormant reference column:

```text
position_version(
  entity_id, activity_id, previous_activity_id?,
  x_cm, y_cm, z_cm, description?
)
position(entity_id, current_activity_id)
```

`(entity_id, activity_id)` identifies one immutable version; `position.entity_id` is
the Position's only subject identity. A composite foreign key proves that the current
pointer names the same Entity. One root and at most one successor per previous version
preserve linear history. Every version references the Activity that established it.
There is no Position id, mutable coordinate row, tombstone or removal operation.

`place.entity_id` gains a required foreign key to `position.entity_id` and stores no
coordinate, Entity name or description. `character.current_place_entity_id` and
`entity_location` remain explicit local context; coordinate equality never derives
either association.

Migration `0011_spatial_exploration.sql` backfills only exact current facts:

1. the existing entry Place receives `(0, 0, 0)` under its original
   `create_entry_place` Activity;
2. each entered Character receives that Place point under its own latest exact
   `enter_world` Activity;
3. each `entity_location` Entity receives its Place point under the Activity that
   established that exact Entity/location result; and
4. an old Entity without exact spatial establishment remains without Position.

The migration fails rather than guessing when an expected Place, entered Character or
located Entity lacks one unambiguous establishing Activity. It creates no synthetic
Activity and rewrites no historical timestamp. Current `create_entry_place`,
`enter_world`, situated Entity creation, Interaction results and both Discovery kinds
then write Position in their existing atomic transaction.

### Connection persistence

S1 stores one immutable row per travel alternative and a child point table:

```text
connection(
  id,
  source_place_entity_id,
  destination_place_entity_id,
  source_position_activity_id,
  destination_position_activity_id,
  allows_reverse,
  name,
  description,
  shape_description?,
  created_by_activity_id
)
connection_point(connection_id, ordinal, x_cm, y_cm, z_cm)
```

Source and destination are distinct existing positioned Places. Source→destination
is always allowed; `allows_reverse` explicitly admits the other direction. There is
no unique endpoint, direction, name, description or geometry constraint. Connection
id is the only identity and exact request replay is the only automatic creation
deduplication. The two composite Place/Position revision references preserve the
exact endpoint points against which an optional course was admitted; they are
dependencies, not a second coordinate copy.

No points means unshaped direct endpoint travel. A shaped Connection has 2 through
128 contiguous points ordered source→destination. First and last equal the exact
endpoint points named by the Position revisions used at creation, consecutive points
differ and non-adjacent segments may not intersect in S1. This keeps progress
derivable from ordinary Position without storing journey or segment membership.
World validates the course with checked integer arithmetic; database constraints and
one affected-row constraint trigger reject invalid count or ordering.

Because no S1 operation edits a Connection or Place Position, neither receives a
speculative version family or editor lock. A later accepted Connection/Place
development behavior must define freshness, history and existing-course consequences
before adding them.

### Rebuildable map index and bounded reads

Canonical Position remains in the version tables. A small synchronous
`place_map_index(place_entity_id, position_activity_id, x_cm, y_cm, z_cm)` exists only
as a rebuildable candidate projection so a million Character Position versions at a
hot point cannot pollute Place selection. It has no domain identity, authorship,
Activity or API and never decides Place truth. Every result exact-rechecks the Place's
current Position before hydration.

Three covering B-trees on the same projection lead respectively with X, Y and Z:
`(x_cm, y_cm, z_cm, place_entity_id)`,
`(y_cm, z_cm, x_cm, place_entity_id)` and
`(z_cm, x_cm, y_cm, place_entity_id)`, each including
`position_activity_id`. PostgreSQL may choose the useful leading axis; `list_place`
still orders and continues canonically on `(x_cm, y_cm, z_cm, place_entity_id)`.
Its opaque cursor binds the exact window and last tuple; changing the window
invalidates the cursor. Concurrent inserts before a cursor may appear only after a
fresh query, so pagination promises stable bounded progress, not a cross-request
snapshot or exact enumeration.

`list_connection` uses separate `(source_place_entity_id, id)` and
`(destination_place_entity_id, id)` indexes and stable id continuation. It hydrates
at most two endpoint summaries per selected row without per-row queries.
`get_connection` alone hydrates course points. No list returns exact total count.

T1E refuted the one-index candidate and a two-axis correction, then supported the
three-axis set above for the exact dense, cross-axis and rotated million-row
fixtures. This does not mathematically bound every correlated 3D distribution; the
three-second statement budget remains the bounded failure contract and T3 must prove
the production query still matches the retained candidate. The correction changes
only rebuildable indexes. Canonical cells, PostGIS, another Place coordinate truth
or global map state remain prohibited without renewed acceptance.

### Exact Movement

`move_character` supplies Character, Connection, expected Character Position
revision, direction and a tagged target:

- `complete` means the exact allowed opposite endpoint; or
- `partial` supplies origin segment ordinal, target segment ordinal and exact target
  `x`/`y`/`z` on a shaped course.

World locks only the controlled Character coordinator, rechecks its Position and
current Place, reads the immutable Connection, validates allowed progress and writes
one new Position version. Unshaped Connections support complete travel only. A
shaped start at an endpoint must agree with current Place; after a partial stop,
current Place is absent and the unique non-self-intersecting course plus supplied
segment establishes progress. Complete arrival sets the endpoint Place current;
partial travel clears it.

Exact point-on-segment, intersection and progress checks use checked `i128` cross and
dot products. No floating point, stored fraction, current Connection membership,
traveller count, timer or completed trace exists. A second traveller never locks the
first Character. Two requests for the same Character serialize honestly; a stale
expected Position writes nothing.

### Activity and atomicity

The existing `submit_discovery` Activity operation covers both result kinds;
`move_character` is the only new operation. Existing Entity roles remain sufficient:
new subjects use `subject`, origin uses `location` and destination uses
`destination`. No generic spatial payload or new universal role is added.

Two typed history associations preserve exact non-Entity and Position involvement:

```text
activity_position(activity_id, role: origin|result,
                  position_entity_id, position_activity_id)
activity_connection(activity_id, connection_id)
```

Discovery records the Character's exact origin Position, every newly established
Position as a result, origin/destination Entity roles and its new Connection.
Movement records old and new Character Position versions plus the exact Connection.
When a newly created origin Place makes the existing immediate Activity context
foreign key cyclic, `context_place_entity_id` remains absent and the typed origin
Position plus `location` Entity role carry the exact history; no broad deferrable
schema change is needed.

Each mutation follows one stable order: normalize and bound input; find exact retry;
lock User then Character/affected Entity coordinators in UUID order; recheck attempt,
Position and current Place; insert prerequisite Entities; append Activity; insert
Position/Place/Connection state and typed history; consume the Investigation when
applicable; commit once. Rejection or database failure leaves no row behind.

The current `(requested_by_user_id, request_id)` Activity namespace and normalized
SHA-256 fingerprint extend to both new mutation shapes. Same id and same input returns
the original accepted result; changed input conflicts. Independent Users or request
ids never deduplicate by meaning.

The delivered `place.latest_activity_id` remains only where existing local Entity
Action, Interaction and `entity_at_position` contracts already require exact-Place
freshness. Place windows, Connection establishment and Character Movement neither
depend on nor advance that broad pointer. Removing it from the older local-mutation
contract is a separate multiplayer refactor and is not claimed by S1.

### Technical admission bounds

| Input | Exact S1 bound | Reason |
| --- | --- | --- |
| each canonical coordinate | inclusive `-1_000_000_000_000_000` through `+1_000_000_000_000_000` cm | Symmetric ten-billion-kilometre extent while worst-case 3D segment products remain safely inside checked `i128`. |
| Place window | ordered inclusive min/max per axis; each span at most `100_000_000` cm | One query covers up to 1,000 km per axis without imposing any discovery or Movement distance rule. |
| page | default 25, requested 1–100 | Matches current repository pagination and bounds hydration. |
| Connection course | either 0 or 2–128 points | Keeps validation and one selected response bounded. |
| name | existing trimmed 1–120 Unicode scalars | Reuses Entity public-text convention. |
| description, shape description, Position description and Activity prose | absent where optional, otherwise trimmed 1–4,000 Unicode scalars | Reuses current text convention; World never interprets it. |
| Discovery result | exactly one | Preserves current Investigation limit. |
| initial Properties and Traits | existing maximum 100 each | No new content container or bound. |

Coordinate range is representability, not gameplay distance. The Agent may propose B
anywhere inside it. Window span is read admission: the Agent can issue another
bounded window and can inspect a far proposed point directly.

New spatial reads run with a transaction-local 3-second statement budget. New
spatial mutations use the same statement budget and a 500-millisecond lock budget;
timeout is one retryable `temporarily_unavailable` semantic error and the transaction
writes nothing. These are server admission limits, not travel duration or Agent
session state. Exact retries make uncertain delivery safe.

### Five-year concurrency boundary

- One million Characters at one coordinate own one Position current pointer each and
  no shared point row, count or lock.
- A hot Place map page reads a rebuildable candidate range and hydrates at most 100
  Places; it does not enumerate Characters or return a count.
- A hot Connection has many immutable readers and no traveller, progress or editor
  row. Each Movement writes only its Character Position and Activity.
- Two equal-looking Connection creations touch no endpoint-pair coordinator and may
  both commit. Only exact retry identity conflicts.
- A loose Investigation uses the existing User admission coordinator, its Character
  Position revision and no synthetic region counter.
- A quiet Character, Place or Connection is never blocked by contention on an
  unrelated new spatial subject; transaction-local budgets fail the exact request
  instead of starving the service. The older exact-Place mutation pointer remains an
  explicit non-claim rather than being hidden inside this spatial slice.
- Any future partition, spatial cell, carrier envelope or PostGIS projection remains
  rebuildable operation state and may never become Position, Place or Connection
  identity.

## Implementation map

| Surface | Current state | Intended S1 change | Invariants |
| --- | --- | --- | --- |
| `game/docs/model/` | Entity, Character, Place and Activity only. | Add direct Position and immutable Connection contracts; evolve Place, Character and Activity. | One Entity identity; no S2–S8 concept enters current contract. |
| `game/docs/capability/`, protocol and parity | No map, spatial Discovery or Movement capability. | Publish the exact capability table, errors, confirmation and deferred scope above. | World/HTTP/MCP semantic parity and one publication owner per rule. |
| `game/docs/storage.md`, `game/migration/0011_spatial_exploration.sql` | No Position, Connection or spatial projection. | Add exact tables, indexes, triggers and evidence-backed backfill. | No generic spatial table, endpoint uniqueness or global coordinator. |
| `game/src/world/model.rs`, `world/read.rs`, `world/mutation.rs`, `world/activity.rs`, `world/investigation/` | Place-local placement and fixed Entity-at-current-Place Investigation. | Add deep Position, Connection, map and Movement seams and evolve Investigation grounding/kinds. | Checked arithmetic, stable lock order, no prose inference. |
| `game/src/wire/`, `game/src/server/http.rs`, `game/src/server/mcp.rs` | Existing thin adapters and error taxonomy. | Add exact inputs, outputs, routes/tools and mappings. | Bounded schemas, User context and identical semantic errors. |
| `game/mcp/agent/`, `game/src/agent_contract.rs`, generated catalog | Existing play loop and fifteen tools. | Teach the S1 loop briefly, inventory public text and regenerate once. | No internal structure in player conversation; context cost proportional. |
| `game/tests/world/` | No production spatial proof. | Prove migration, lineage, map bounds, Investigation, Discovery, Movement, retry, rollback and races. | Claims limited to production World and disposable PostgreSQL exercised. |
| `game/tests/server/` | Fifteen-capability HTTP/MCP parity. | Prove wire parity, errors, catalog and two-User S1 flow. | No adapter-only behavior or semantic divergence. |
| `dev/docs/evidence/` | No delivered spatial slice. | Record exact deterministic evidence and non-claims after implementation. | Lab evidence is not promoted. |
| Areas, synthesis, backlog and concept log | Full direction and forward state exist. | Update selected edge and final result only when their state changes. | S1 plan never becomes duplicate product truth. |

Studio receives no mandatory S1 change. Authoritative protocol and database readback
are sufficient verification; a map UI must earn its own player or operator outcome.

## Execution contract

Root owns outcome, plan state, integration and final evidence. A delegated high-
reasoning implementation Agent may receive this accepted plan path and exactly one
dependency-ready task id, re-read live authorities, change only owned surfaces, run
focused evidence and return raw results. No delegated task edits the plan unless
plan maintenance is its assignment.

No S1 code, schema, `game/docs` contract or public text may change while `status` is
`draft`. Explicit acceptance activates S1 only. S2–S8 remain preservation rows and
each requires a separately accepted plan.

## Task graph

Allowed states are `pending`, `in_progress`, `completed` and `blocked`.

| ID | State | Depends | Parallel-safe | Objective | Owned surfaces | Evidence |
| --- | --- | --- | --- | --- | --- | --- |
| T1 | completed | — | no | Publish the accepted S1 runtime contract and exact migration design. | `game/docs/`, plan file map | Documentation lint and public-contract inventory. |
| T1E | completed | T1 | no | Falsify the leading Place-map projection and covering B-tree before production schema work. | `dev/lab/spatial/03-place-map-index/` only | Real PostgreSQL 17 dense/adversarial million-row `EXPLAIN (ANALYZE, BUFFERS)` verdict and cleanup. |
| T2 | in_progress | T1E | no | Persist direct Position, positioned Place, immutable Connection and typed Activity footprint on existing Worlds. | migration, World models/storage, focused tests | Backfill, constraint, lineage and rollback tests on real PostgreSQL. |
| T3 | pending | T2 | no | Ship bounded Place and Connection reads with the proved rebuildable projection. | World reads and tests | Pagination, cursor, hot-point and production query-bound evidence. |
| T4 | pending | T3 | no | Extend Investigation and accept Entity or connected-Place Discovery atomically without Movement. | Investigation, World mutation, Activity and tests | Retry, both kinds, loose origin, rollback and parallel-Connection races. |
| T5 | pending | T4 | no | Move fully or partially over one exact Connection with independent travellers. | World Movement, Activity and tests | Integer geometry, arrival/intermediate, retry, stale Position and lock-bound evidence. |
| T6 | pending | T5 | no | Ship S1 through HTTP, MCP and concise Agent text with full parity. | wire, adapters, MCP sources, catalog and server tests | Schema/error/catalog parity and two-User protocol flow. |
| T7 | pending | T6 | no | Prove the complete outcome, align authorities and record exact evidence/non-claims. | evidence, Areas/synthesis/log/backlog, plan | Validation ladder, focused review and clean builder brief. |

## Task details

### T1 — Publish the S1 contract

**Objective:** Every S1 actor, action, state, authority, error, bound and Activity
role has one accepted runtime home before schema or code.

**Actions:**

1. Add Position and Connection model contracts and evolve Place, Character,
   Investigation, Discovery, Movement, Activity, storage, protocol, Agent play and
   deferred scope once each.
2. Freeze the capability table, tagged inputs, response pages, semantic errors and
   HTTP/MCP names exactly as accepted above.
3. Publish the backfill, tables, constraints, indexes, lock order, idempotency and
   timeout semantics before migration work.

**Evidence:** documentation lint and a focused public-text inventory showing every
new rule's one owner and every generated consumer.

**Stop:** return the plan to `draft` if the contract needs Observation, entry-choice,
Area, Relation, privacy, Route, terrain, Connection editing or another new concept.

**T1 review record:** focused documentation lint and `git diff --check` pass. The
public inventory contains 19 capability contracts and 15 published tool-text
sources; exactly `get_connection`, `list_connection`, `list_place` and
`move_character` are absent from publication until T6. The broad Studio lint is
therefore deliberately 4/5 green at this intermediate checkpoint: only exact
capability/tool-text parity fails. T1 neither publishes placeholder tool text nor
claims adapter delivery; T6 must close this named gap and restore the full lint.

### T1E — Falsify the Place-map index before production

**Objective:** Determine whether the accepted rebuildable `place_map_index` plus one
covering `(x_cm, y_cm, z_cm, place_entity_id)` B-tree keeps the exact S1 window read
bounded under five-year-scale dense and adversarial distributions.

**Real seams:** local PostgreSQL 17 query planner, table/index storage, one million
synthetic Place projection rows, keyset query and `EXPLAIN (ANALYZE, BUFFERS)`.

**Simulated or absent seams:** production migration and World schema, authorization,
HTTP/MCP, Position exact-recheck, concurrent production traffic, failover and hosted
latency. The fixture proves no behavior outside its SQL candidate.

**Actions:**

1. Create a standalone Rust/SQLx lab under `dev/lab/spatial/03-place-map-index/`
   without importing production or prior lab code.
2. Load exactly one million rows for a dense same-point case and an adversarial case
   where the leading-axis range is broad but other axes exclude almost every row.
3. Execute first and continued 100-row pages using the exact accepted ordering and
   window predicates; capture plans, actual rows, rows removed, buffers and time.
4. Drop the disposable database through the repository's ownership-safe cleanup
   pattern and record the exact supported or refuted verdict and non-claims.

**Evidence:** no sequential scan; dense first/continued pages return at most 100 with
stable continuation; the adversarial empty or sparse page does not inspect work
proportional to one million candidates. Raw plans and cleanup result remain in the
lab record.

**T1E review record:** PostgreSQL 17.8 ran three independent disposable fixtures of
exactly 1,000,000 rows. The accepted X-leading index and then X+Y were refuted by
cross-axis and rotated first pages that did work proportional to the fixture. The
final X+Y+Z set returned dense first/continued pages through 4–5 shared blocks and
cross/rotated first/continued pages through 5 shared blocks, with no sequential scan.
The retained suite is 3/3 green; the independent cleanup audit found zero leaks.
This supports only the three fixed distributions, not every correlated 3D workload,
production World behavior, throughput or hosted latency.

**Stop resolution:** the smallest tested rebuildable PostgreSQL-only correction is
the three-axis covering set now adopted by T2. Any need for canonical spatial cells,
PostGIS, changed window/public semantics or another World truth still returns the
plan to draft for User re-acceptance.

### T2 — Build direct Position and immutable Connection

**Objective:** Existing and new S1 spatial subjects have exact current Position and
immutable Connection state with queryable history and no Place-wide spatial revision.

**Actions:**

1. Add `0011_spatial_exploration.sql` with deterministic fail-closed backfill,
   constraints, triggers, typed Activity associations and map projection.
2. Implement deep Position and Connection storage/read modules behind `World`.
3. Evolve current entry and situated Entity-producing operations to write Position
   atomically and keep `entity_location`/current Place explicit.
4. Prove coordinate bounds, linear Position lineage, current-pointer integrity,
   endpoint validity, course ordering and projection rebuild on real PostgreSQL.

**Evidence:** focused `aicadia-game` World tests cover legacy fixture variants,
backfill refusal, migration success, constraints, injected rollback and immutable
readback.

**Stop:** do not add a Position id, duplicate Place-owned coordinates, relative
reference field, Connection version/update, global revision or lab-code import.

### T3 — Build bounded shared spatial reads

**Objective:** An Agent can page exact ordinary Place and Connection context without
Knowledge gating, unrelated hydration or course explosion.

**Actions:**

1. Implement `list_place` candidate selection, exact Position recheck, opaque
   box-bound cursor and maximum page.
2. Implement incident `list_connection` summaries and anchored `get_connection` with
   one bounded course.
3. Prove dense hot-point, sparse adversarial, concurrent insert and cursor semantics.

**Evidence:** functional pagination and authorization tests plus T1E's retained
index verdict show no unbounded hydration, no count and bounded production query
count for each public operation.

**Stop:** stop if the production query no longer matches the proved T1E shape; do not
silently add canonical cells, PostGIS, Place coordinates or a global map revision.

### T4 — Build Position-grounded Investigation and Discovery

**Objective:** One positive Agent-selected result kind commits an Entity find or the
complete connected-Place package without moving the Character.

**Actions:**

1. Bind each attempt to kind, exact Character Position revision and optional current
   Place while retaining current chance, retry and User admission rules.
2. Implement `entity_at_position` with optional current Place association.
3. Implement new/existing destination and new/existing loose-origin variants, each
   creating exactly one newly identified Connection.
4. Extend normalized fingerprints, retry reconstruction, Activity roles and typed
   history for every variant.
5. Prove exact retry, changed-input conflict, rejection, injected rollback, stale
   grounding and two concurrent equal-looking Connections that both commit.

**Evidence:** focused production World tests read back complete state, attempts and
Activity after every variant and race.

**Stop:** no endpoint-pair deduplication, coordinate merge, map-read receipt, direct
spatial proposal outside Investigation or Discovery-triggered Movement.

### T5 — Build complete and partial Character Movement

**Objective:** A Character traverses one allowed Connection completely or stops at
one exact course point while other travellers remain independent.

**Actions:**

1. Implement checked course creation and Movement arithmetic with exact direction,
   segment and progress validation.
2. Write new Character Position, optional current Place, typed history and Activity
   in one request-idempotent transaction.
3. Prove unshaped completion, shaped forward/reverse completion, partial then complete
   travel, off-course and non-progress rejection, exact retry and stale Position.
4. Hold one Character lock deliberately to prove the 500 ms retryable bound while a
   second Character on the same Connection still commits.

**Evidence:** focused World tests prove exact coordinates, Activity, current Place,
rollback, subject isolation and timeout classification.

**Stop:** no journey, timer, trace, Connection occupancy, editor, traveller count,
terrain cost or inferred destination.

### T6 — Publish protocol parity and Agent guidance

**Objective:** A conforming Agent can complete S1 through identical World, HTTP and
MCP semantics without exposing implementation structure in player conversation.

**Actions:**

1. Add exact bounded wire types, HTTP routes and MCP tools.
2. Inventory public text, add the smallest play-loop/tool guidance and regenerate the
   catalog once.
3. Prove every success, retry, bound and semantic error through HTTP and MCP.
4. Run the two-User adapter flow with independent authoritative readback.

**Evidence:** `cargo test -p aicadia-game --test server` plus focused parity and
catalog tests cover all new and evolved capabilities.

**Stop:** repeat the inventory if one rule, bound, capability or error diverges
between World, HTTP, MCP and generated catalog.

### T7 — Demonstrate and record S1

**Objective:** The promised exploration loop is reproducible, authorities agree and
the next spatial risk remains explicit without beginning S2.

**Actions:**

1. Run the full validation ladder and inspect exact current/history state after the
   two-User flow and concurrency cases.
2. Record production evidence with real and absent seams.
3. Align Areas, active synthesis, concept log, backlog and this plan once each.
4. Select no next edge automatically; report which game outcome now has highest
   value without authorizing it.

**Evidence:** workspace tests, documentation lint, public catalog parity,
`git diff --check`, focused scope review and `cargo brief` all pass.

**Stop:** do not claim production throughput, privacy, Agent comprehension, Area,
relative Position or Connection editing that the evidence did not exercise.

## Validation ladder

1. **Focused:** each task's real migration, storage, arithmetic, query, race, adapter
   and documentation checks pass.
2. **Contract:** Position, Place, Connection, Investigation, Discovery, Movement and
   Activity agree across `game/docs`, PostgreSQL, World, HTTP, MCP, Agent text and the
   generated catalog; S2–S8 concepts remain absent.
3. **Outcome:** two Users reproduce bounded map read → confirmed B discovery while
   remaining at A → independent shared read → partial Movement → arrival at B, with
   retry, parallel Connection, rollback, stale Position and independent traveller
   evidence.
4. **Scale:** dense hot point, sparse adversarial window, hot immutable Connection and
   unrelated quiet Character evidence demonstrate bounded work without global state.
5. **Integrity:** `cargo test --workspace`, Studio documentation lint,
   `git diff --check`, focused diff review and authority audit pass.

## Change control

File paths, helper boundaries and stronger evidence may be refined while S1's
accepted player outcome, public semantics, persistence truths, bounds and exclusions
remain unchanged. Stop implementation, set `status: draft`, revise and request
explicit re-acceptance if evidence changes Investigation kind/admission, Position or
Connection identity, map visibility, Movement meaning, public operations, Activity
history, numeric limits, irreversible data, external authority, material cost or the
evidence claim.

The complete delivery map may be corrected when a later accepted choice changes its
own slice. It never authorizes pulling S2–S8 into S1.

## Completion conditions

- this complete technical contract has explicit User acceptance before T1;
- T1, T1E and T2–T7 are `completed` and the validation ladder passes;
- the S1 player outcome is demonstrated through World, PostgreSQL, HTTP and MCP;
- current contract, vocabulary, Areas, synthesis, backlog, public text and evidence
  agree without duplicating one another;
- the scenario ledger still shows where SP01–SP15 continue after S1;
- no known-stale authority, material open S1 choice or accidental later-slice
  implementation remains; and
- `status: complete` and `completed_at` are recorded only after all conditions hold.
