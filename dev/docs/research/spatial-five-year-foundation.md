---
status: pending
era: August Activity-Property-Trait
---

# Spatial foundation through a five-year backcast

> **Role / side:** sourced spatial-foundation research report / development side.
> **Authority:** records this report's five-year pressures, primary-source findings, scenario implications, falsifiers and non-claims.
> **Excludes:** product choices, accepted vocabulary, current behavior, schema and implementation; those belong in `dev/docs/concept/`, `dev/CONTEXT.md`, `game/docs/`, accepted plans and runtime code.

Date: 2026-08-18

Status: research implication only; no candidate term or data shape in this report is
accepted Aicadia truth.

## Question

If Aicadia is played by millions of people five years from now, which spatial
separations and technical pressures are likely to have survived, and what is the
smallest present-day foundation that would preserve a path to that future without
prebuilding it?

The question is tested against the thirteen fixed spatial scenes in the
[Place scenario catalogue](../../areas/place/scenarios.md), the paper findings in
the [spatial-model pressure test](../../lab/spatial/01-model-pressure/README.md), and
the sourced boundaries already collected in
[Spatial multiplayer foundation](spatial-multiplayer-foundation.md) and
[Entity and Place through a game-framework lens](entity-place-framework-boundary.md).
Those records are inputs, not decisions made again here.

## Method and evidence boundary

This is a `5jaar` backcast, not a forecast presented as fact:

1. inhabit ordinary play after five years at intended scale;
2. identify culture, abuse, failures and technical pressures that survived;
3. walk backward to the smallest decisions and experiments that matter now; and
4. keep external fact, Aicadia inference and User choice visibly separate.

All external claims use primary sources: official engine and database manuals,
project-owned specifications and original systems papers. Sources were opened and
checked on 2026-08-18. Engine runtime transforms, a persistent authoritative World
and a PostgreSQL query model solve different problems; a useful pattern in one is
not proof of a production design in another.

`Local placement`, `resolved world point`, `reference` and `authored spatial
meaning` are descriptive working labels in this report only. They do not introduce
Aicadia-owned terms or authorize tables, fields, endpoints or behavior.

## Five years forward: ordinary Aicadia

The future scenes below are backcast observations. They deliberately combine the
fixed scenarios at a scale that exposes design pressure.

### Ordinary use

- An explorer walks from a discovered Place into unnamed heath, stops anywhere,
  later reaches a hotel and asks for straight-line distance, known travel options
  or a selected journey. Those answers are not assumed to be the same number.
- A village contains player-built tables, cups and birdhouses. Some objects remain
  where placed; others move with a table, Character, wagon or ship. An Agent can
  still state “two centimetres above the table” when the World lacks the geometry
  needed to calculate that statement.
- Ships carry cabins that qualify as Places, active Characters and thousands of
  private and public Entities. Interior play continues without rewriting every
  passenger when the ship moves or conflicting every interior action with motion.
- A Character can inspect a visible coat without learning that a hidden Entity is
  inside it. The holder can retrieve eligible contents through a bounded read.
- A button can affect a distant bomb only through explicit non-spatial authority;
  distance, descriptive prose and guessed identities grant nothing.
- Map Agents retrieve established Places and optional coverage, direct travel
  options and calculated paths without confusing a physical bridge with topology
  or a selected path with the graph from which it was found.

### Emergent culture and fun

- Players name popular journeys, moving settlements and strange physical customs.
  A named journey may become durable game content, while an ordinary shortest path
  remains a calculation over current topology.
- Agents preserve delightful anomalies such as a floating cup or a Character that
  deliberately floats with it. World validates exact submitted consequences but
  does not infer physics or normalize the scene back to realism.
- Multiple authored descriptions coexist around one mechanically coherent scene.
  Different prose does not silently create several competing exact positions.
- Moving carriers become social places. Their interior identity and relationships
  remain stable while their external point changes.

### Abuse and failure modes

- Attackers create deep or cyclic placement references to amplify reads, force
  recursion or make resolution non-terminating.
- A festival puts a million subjects at one point. An unbounded “nearby” response,
  count or lock becomes a denial-of-service primitive even if the spatial index is
  fast.
- A hot carrier is moved repeatedly while thousands of Agents act inside it.
  Designs that lock the carrier for every interior action serialize unrelated play;
  designs that rewrite descendants create an unbounded transaction.
- Hidden Entities are probed through guessed IDs, result counts, pagination gaps,
  timing differences, topology traversal and “why unavailable?” errors.
- Huge Areas, adversarial geometry, path searches and very wide direct topology
  exhaust CPU or memory unless inputs and outputs are bounded.
- Coordinates far from an origin lose useful precision, while mismatched units,
  axes or 2D/3D predicates make “two centimetres” or “under” disagree between
  systems.
- A stale calculated world point is mistaken for canonical state after an ancestor
  moves, causing invalid proximity, visibility or action authorization.

### Pressures that survived

The future World did not survive because one primitive represented every spatial
sentence. It survived because independently changing truths stayed separate:

1. stable subject identity;
2. one exact mechanical placement when established;
3. bounded parent-relative movement when explicitly chosen;
4. a resolved current world point for authorized reads and exact predicates;
5. optional geometry and Area only where gameplay needs them;
6. authored meaning that cannot cause mechanics by prose alone;
7. direct travel topology distinct from calculated or player-selected journeys;
8. authorization and knowledge distinct from spatial relevance; and
9. replaceable indexes and caches distinct from canonical World truth.

This is a backcast implication, not an accepted model.

## Primary-source findings

### 1. Mature scene and ECS systems distinguish authored local state from resolved world state

**External fact.** OpenUSD exposes both a fully combined local-to-parent transform
and a computed local-to-world transform. A prim may reset inherited transform state,
and clients can cache ordered transform operations to reduce value-resolution cost.
[OpenUSD `UsdGeomXformable`](https://openusd.org/release/api/class_usd_geom_xformable.html)

Godot `Node3D` likewise stores a transform relative to its parent by default and
exposes `global_transform`; `top_level` makes the node stop inheriting parent
transforms. Godot notes that global transform changes propagate through children
and batches normal transform updates for performance.
[Godot `Node3D`](https://docs.godotengine.org/en/stable/classes/class_node3d.html)

Unity's ECS transform system stores `LocalTransform`, an optional single `Parent`
and computed `LocalToWorld`. Without a parent, the local transform is relative to
the World origin. Unity warns that `LocalToWorld` can be out of date during part of
the simulation cycle and supplies an explicit computation when a current result is
required.
[Unity Entities transform concepts](https://docs.unity.cn/Packages/com.unity.entities%401.0/manual/transforms-concepts.html)

**Inference for Aicadia.** One stored placement may be relative either to the World
or to one Entity, while an authorized World read returns one resolved current point.
The resolved point need not become a second canonical truth. This gives the Agent a
stable answer shape without forcing every carrier move to rewrite descendants.

The sources do **not** prove that Aicadia Position should include rotation, scale,
arbitrary transform stacks or every engine hierarchy rule. The current Aicadia
question is an exact point; any broader pose remains a separate choice.

### 2. Relative placement is mechanical; it does not explain “inside”, ownership or visibility

**External fact.** Unity says a child moves, rotates and scales with its parent and
allows reparenting either while preserving world-space pose or while preserving
parent-relative pose. Unreal similarly exposes attachment with explicit rules for
location, rotation and scale.
[Unity `Transform.SetParent`](https://docs.unity3d.com/6000.0/Documentation/ScriptReference/Transform.SetParent.html),
[Unreal Attach Actor to Actor](https://dev.epicgames.com/documentation/en-us/unreal-engine/BlueprintAPI/Transformation/AttachActorToActor)

**Inference for Aicadia.** If a candidate Position is stored relative to another
Entity, following that Entity is the mechanical consequence; otherwise “relative”
has no stable operational meaning. A cup that should remain behind uses World-
relative placement even if an Agent describes it as above a table. Re-establishing
the reference must explicitly choose whether the current world point or current
local offset is preserved.

The reference does not prove that the cup is on the table, that a sword is owned by
the coat wearer, that an observer may see it or that it can be removed. Those truths
have different visibility, authority and lifecycle.

### 3. “Two centimetres above” is exact only when its missing spatial basis is exact

**External fact.** Engine local transforms are coordinates relative to a parent's
origin and axes, not natural-language surface constraints. OpenUSD and Godot both
compute local-to-world results from explicit transforms; neither interprets “above
the surface” prose to manufacture the missing point.
[OpenUSD `UsdGeomXformable`](https://openusd.org/release/api/class_usd_geom_xformable.html),
[Godot using 3D transforms](https://docs.godotengine.org/en/stable/tutorials/3d/using_transforms.html)

**Inference for Aicadia.** “Two centimetres above the table” contributes to exact
Position only if the proposal also supplies or addresses enough accepted structure
to select one point: a reference, surface point or equivalent anchor, direction and
remaining coordinates. Without that basis it remains useful authored spatial
meaning, but World cannot use it for collision, proximity, movement or permission.

Free authored input and exact Position can be submitted and read in one operation
without occupying the same canonical field. One conversation or query does not
require one table or lifecycle.

### 4. A carrier move can be one canonical write, but hierarchy traversal is not free

**External fact.** Unity's ECS documentation says transform work is parallelized by
root hierarchy and explicitly warns against large numbers of non-static Entities
under one root. Godot propagates global transform changes through Node3D children.
[Unity Entities using transforms](https://docs.unity.cn/Packages/com.unity.entities%401.2/manual/transforms-using.html),
[Godot `Node3D`](https://docs.godotengine.org/en/stable/classes/class_node3d.html)

PostgreSQL supports recursive graph traversal and explicit cycle detection, but its
manual warns that recursive queries must terminate. Generated columns cannot use a
subquery or reference another row, so PostgreSQL cannot turn a multi-row placement
chain into an automatically generated indexed world point.
[PostgreSQL recursive queries](https://www.postgresql.org/docs/current/queries-with.html#QUERIES-WITH-CYCLE),
[PostgreSQL generated columns](https://www.postgresql.org/docs/current/ddl-generated-columns.html)

**Inference for Aicadia.** Relative placement removes descendant **writes**, not
descendant **work**. A production candidate needs one parent at most, rejected
cycles, a small explicit depth bound and bounded resolution. Interior actions that
depend only on local state should not read or lock the carrier's external Position;
an action whose validity depends on the resolved world point must name and validate
the bounded ancestor dependency.

A future replaceable search projection or two-stage root/local index may be earned
by measured nearby queries. It cannot be smuggled in as a second authoritative
Position, and standard generated columns do not maintain it across ancestor rows.

### 5. Spatial indexes bound candidate work, not result cardinality or semantics

**External fact.** PostGIS GiST indexes use bounding-box candidates for index-aware
spatial predicates. `ST_DWithin` adds an index-usable bounding-box comparison, and
the `<->` operator supports index-assisted nearest-neighbour ordering.
[PostGIS spatial indexes](https://postgis.net/documentation/faq/spatial-indexes/),
[PostGIS `ST_DWithin`](https://postgis.net/docs/ST_DWithin.html),
[PostGIS `<->`](https://postgis.net/docs/geometry_distance_knn.html)

Unreal's Replication Graph builds per-client Actor lists from persistent groupings
such as grid cells, rooms or zones rather than evaluating every Actor against every
connection. It also separates frequently changing Actors from dormant ones.
[Unreal Replication Graph](https://dev.epicgames.com/documentation/en-us/unreal-engine/replication-graph-in-unreal-engine)

**Inference for Aicadia.** Addressed lookup, exact proximity, same-carrier contents,
same-Place play and map discovery are different bounded reads. One global nearby
query is not the public API for all of them. Every list still needs a limit, stable
continuation and overload behavior; a million matching rows remain a million rows
after an index found them efficiently.

Operational cells, partitions, root/local indexes and caches should be replaceable
query accelerators. They must not become Place identity, discovery truth or access
authority.

### 6. Large worlds require an explicit precision contract, not a casual `x/y/z`

**External fact.** Unreal Large World Coordinates moved core spatial types to
64-bit doubles to improve large-scale placement precision. Its Niagara path uses a
tile plus a relative position because GPU-oriented work cannot use doubles in the
same way. Godot documents increasing single-precision error with distance, offers
double-precision large-world builds and notes that origin shifting adds complexity,
especially for multiplayer.
[Unreal Large World Coordinates](https://dev.epicgames.com/documentation/en-us/unreal-engine/large-world-coordinates-in-unreal-engine-5),
[Unreal Large World Coordinates in Niagara](https://dev.epicgames.com/documentation/en-us/unreal-engine/large-world-coordinates-in-niagara-for-unreal-engine),
[Godot large world coordinates](https://docs.godotengine.org/en/stable/tutorials/physics/large_world_coordinates.html)

**Inference for Aicadia.** The stored numeric form must eventually specify units,
axes, dimensionality, precision and overflow bounds. Double precision, fixed-point
or cell-plus-local coordinates are candidates to measure; source precedent alone
does not select one. Parent-relative placement reduces local magnitude but does not
remove the need for a stable World-scale coordinate contract.

Starting with the smallest database-native exact form remains Terry only if a
migration to the measured long-term representation is possible and the accepted
world extent cannot already violate its precision.

### 7. Interest is not privacy, and identity is not authority

**External fact.** Unreal network relevancy selects Actors capable of affecting one
client and can inherit relevancy from ownership; it is a replication optimization,
not a general authorization proof.
[Unreal Actor relevancy](https://dev.epicgames.com/documentation/en-us/unreal-engine/actor-relevancy-and-priority-in-unreal-engine)

PostgreSQL row security can restrict which rows normal queries return or modify and
uses default deny when row security is enabled without an applicable policy.
[PostgreSQL row security](https://www.postgresql.org/docs/current/ddl-rowsecurity.html)

Google's Zanzibar paper reports object-level authorization checks over explicit
relation tuples, causal ordering between permission and content changes, trillions
of ACLs and millions of checks per second. It also reports caching, request
deduplication and specialized work for hot or deeply nested authorization data.
[Zanzibar original paper](https://www.usenix.org/conference/atc19/presentation/pang)

Macaroons demonstrate attenuable bearer credentials with contextual caveats for
delegation. They are evidence that a capability can constrain actor, purpose and
context; they are not evidence that Aicadia needs transferable bearer tokens.
[Macaroons original paper](https://research.google/pubs/macaroons-cookies-with-contextual-caveats-for-decentralized-authorization-in-the-cloud/)

**Inference for Aicadia.** Spatial candidate selection must be followed by one
Character-grounded authorization and knowledge decision before any fact, count,
error detail or continuation token leaves World. Guessing an Entity identity does
not confer current observation or mutation authority. Position, authored meaning,
private carried state and control may need different exposure even when one World
operation composes them.

Authorization structure must not be hidden inside open spatial prose or inferred
from proximity. The distant button needs an explicit action-specific authority
basis; the hidden sword needs explicit read and mutation eligibility.

### 8. Direct topology and a path through it are different state shapes

**External fact.** Godot `AStar3D` stores points and explicit directed or
bidirectional connections. `get_id_path` separately calculates and returns an
ordered path through those connections; changing weights can change the returned
path without changing endpoint positions.
[Godot `AStar3D`](https://docs.godotengine.org/en/stable/classes/class_astar3d.html)

**Inference for Aicadia.** “A has direct travel to B” is durable topology if the
game establishes it. “Travel from A to C via B” can be a calculated answer over
current topology. A later player-named journey may earn durable independent state,
but neither coordinates nor an open Relation should silently grant traversal.

A physical road, door or bridge remains an Entity. It may be named as a dependency
of direct topology without the topology itself becoming that Entity. Remote bomb
causality is neither topology nor a calculated path.

### 9. Concurrency should follow the smallest exact dependency

**External fact.** PostgreSQL MVCC gives each statement a snapshot while row-level
locks block conflicting writers to the same rows rather than every reader. The
manual cautions that `SKIP LOCKED` provides an inconsistent view and is unsuitable
for general-purpose correctness reads.
[PostgreSQL concurrency control](https://www.postgresql.org/docs/current/mvcc.html),
[PostgreSQL explicit locking](https://www.postgresql.org/docs/current/explicit-locking.html)

**Inference for Aicadia.** A move or re-reference should conflict on the positioned
Entity and only the bounded current dependencies needed for its accepted result.
Moving one carrier should not update or lock every descendant. Quiet Entities must
continue while one Place, carrier or coordinate is hot. Queue-oriented lock skipping
cannot make an inconsistent spatial snapshot correct.

Current state and Activity still settle atomically for the concrete action. This
does not imply event sourcing, a global World revision or continuous server-side
simulation.

## Scenario implications

These are pressure-test results for the research-supported direction, not accepted
outcomes.

| Scenario | Research-supported implication | Remaining independent decision |
| --- | --- | --- |
| SP01 · A to B | Character has one exact placement; direct topology may authorize a step; intermediate World-relative points need no Place | movement timing, admission and whether topology is required |
| SP02 · Entity in coat | Relative placement can make the Entity move with coat or Character | private-content discovery, removal and what “in” means |
| SP03 · distant bomb | endpoint Positions remain spatial facts only | explicit remote authority and atomic consequences |
| SP04 · cup above table | exact local point follows table only when complete structural input exists; prose may coexist separately | surface geometry, who can observe, deliberate multi-Entity movement |
| SP05 · dog under bridge | dog and bridge may have exact points; “under” is calculated only from accepted geometry or remains authored | geometry predicate and statement freshness |
| SP06 · 100 m to hotel | calculate straight-line or path distance from an explicit basis; do not store a duplicate changing number as Position | arrival boundary, estimate versus measurement, path choice |
| SP07 · birdhouse on table | one bounded relative chain avoids copied ancestor positions and descendant rewrites | re-reference concurrency and discovery visibility |
| SP08 · forest edge | Character Position remains exact without minting a Place; optional Area/terrain state stays separate | incomplete boundaries and observation rules |
| SP09 · moving ship | ship move changes one canonical placement; cabin/passenger points resolve through bounded references | depth, indexing and which interior actions depend on exterior pose |
| SP10 · hidden sword | an eligible holder may resolve/list it; other Agents receive no fresh fact merely from identity or memory | concrete private-content and investigation mechanic |
| SP11 · A connects to B | store direct directed topology only if gameplay establishes it; calculate a multi-step path | final topology name, access/cost and later durable named journey |
| SP12 · hot point | independent subject placements avoid a shared position owner; indexed reads remain limited and paged | admission, fairness, exact co-location and measured overload behavior |
| SP13 · city/forest/waterfall | Place uses the Entity's same Position; optional Area and map visibility remain separate | which subjects earn Place and which coverage is authoritative |

## Research-supported candidate shape

The sources and scenarios jointly support one strongest **implication** for later
User negotiation. It is not a product choice and its field names are illustrative:

```text
one Entity identity
    └─ zero or one exact mechanical Position
         ├─ reference: World or exactly one Entity
         └─ exact coordinates in that reference

authorized read
    └─ one resolved current World point, or honestly unavailable

separate when earned
    ├─ authored spatial meaning
    ├─ optional Area or geometry
    ├─ direct travel topology
    ├─ private-content and action authority
    └─ remote causality
```

Consequences of this candidate:

- A Place does not need duplicate coordinate columns; it uses the same Position as
  its Entity identity. A Place may be World-relative or, if later accepted, move
  with one Entity such as a ship.
- Relative Position means mechanical following. A merely descriptive “near”, “in”,
  “above” or “under” statement cannot cause following.
- An authorized Agent can ask one World operation for the resolved Position. That
  promise does not expose a hidden Entity to an ineligible Character and does not
  promise a point where exact placement was never established.
- Free Agent-authored spatial text remains possible without making the exact point
  non-indexable or asking World to interpret it.
- Direct topology is not a Route. A path or journey is calculated from topology
  unless later gameplay earns durable named path state.
- The simplest current storage can remain one Position row keyed by `entity_id`;
  reference, numeric representation, orientation, depth and indexes remain choices
  until the necessary game behavior and experiments establish them.

The candidate is stronger than absolute-only Position because it survives moving
carriers without mass writes. It is stronger than one flexible spatial row because
exact placement, authored meaning, privacy and topology do not share lifecycle or
authority. It is stronger than a universal Relation graph because deterministic
mechanics retain small explicit invariants.

## Walk backward to the present

The future does not justify implementing the whole shape now. It identifies a
dependency order for decisions and evidence.

### Decisions that matter before schema

1. Decide whether Position may use either World or exactly one Entity as its
   reference, and whether Entity-relative placement necessarily follows that Entity.
2. Decide whether an authorized read always returns the resolved World point when
   exact Position exists, while hidden or unpositioned state remains unavailable.
3. Keep authored spatial meaning outside Position's exact mechanical fields; decide
   its concrete storage only when a current scene needs durable authored meaning.
4. Keep direct Place topology distinct from Position and calculated journey output;
   negotiate its final name only when traversal is current gameplay.

### Small experiments that can falsify the direction

1. **Resolution fixture:** World-relative cup, table-relative cup, nested birdhouse
   and cabin-on-ship; prove preserve-world versus preserve-local re-reference and
   reject a cycle and over-depth chain.
2. **PostgreSQL query fixture:** compare a bounded recursive read with explicit
   application resolution for the accepted maximum depth; inspect query plans and
   row locks rather than claiming scale from syntax.
3. **Carrier fixture:** move one ship with 1, 1,000 and 100,000 descendants; prove
   canonical write count, addressed-resolution latency and that interior actions do
   not conflict with irrelevant exterior motion.
4. **Nearby fixture:** compare absolute roots, a local carrier interior and the hot
   million-subject point using GiST candidate filtering, authorization before
   hydration, stable limits and overload admission.
5. **Precision fixture:** test the accepted maximum World extent and smallest
   meaningful offset under double, fixed-point and cell-plus-local candidates,
   including conversion through the HTTP/MCP contract.
6. **Privacy fixture:** visible coat/hidden sword, prior memory, guessed identity,
   counts, errors, pagination and timing; prove no unauthorized existence leak.
7. **Topology fixture:** A→B but not C, changing cost and a disabled physical bridge;
   prove direct options and a calculated path remain different outputs.

Each experiment remains in `dev/lab/`, names real and simulated seams and cannot
promote its own winner.

## Falsifiers

The candidate direction should be rejected or revised if evidence shows any of the
following under accepted behavior and bounds:

- an exact Entity-relative point cannot yield one unambiguous current World point;
- cycle/depth enforcement or exact resolution requires a global lock or unbounded
  traversal;
- carrier movement still requires descendant writes for correctness;
- authorized nearby lookup cannot remain bounded without persisting a second
  conflicting Position truth;
- a moving Place loses stable discovery or map identity;
- private child placement cannot be indexed or resolved without exposing existence;
- Agent-facing composition is materially harder than one broad spatial payload
  after both support the same scenes;
- the chosen numeric form cannot preserve accepted extent and precision; or
- direct topology and calculated journeys cannot be separated without duplicating
  authoritative travel truth.

## Non-claims

This report does not:

- accept the candidate shape, its working labels or any new Aicadia vocabulary;
- choose `x/y/z`, dimension, units, axes, precision, orientation, geometry or Area;
- choose a Position reference field, maximum depth, lock order, revision or cache;
- choose Relation storage, inventory, possession, ownership, access or visibility;
- choose the final direct-topology name or introduce Route;
- make Place movable, make every Entity positioned or grant universal Position
  visibility;
- choose PostGIS, RLS, Zanzibar, Macaroons, a graph database, partitions or a
  projection as architecture;
- prove query plans, privacy, Agent comprehension, concurrency, throughput, latency,
  failure recovery or million-User scale; or
- authorize changes to Areas, concept records, `dev/CONTEXT.md`, `game/docs/`, schema,
  API or code.

## Primary-source audit

All sources were checked online on 2026-08-18. “Used for” is the narrow transferred
fact; “does not establish” prevents an engine or paper from becoming Aicadia truth.

| Primary source | Owner/type | Used for | Does not establish |
| --- | --- | --- | --- |
| [OpenUSD `UsdGeomXformable`](https://openusd.org/release/api/class_usd_geom_xformable.html) | Alliance for OpenUSD API | local-to-parent versus computed world transform, reset and cached ops | Aicadia hierarchy, fields or transform breadth |
| [Godot `Node3D`](https://docs.godotengine.org/en/stable/classes/class_node3d.html) | engine docs | parent-relative/default and global transform, propagation | persistent server storage or privacy |
| [Godot transforms](https://docs.godotengine.org/en/stable/tutorials/3d/using_transforms.html) | engine docs | explicit local and global coordinate spaces | prose-to-geometry inference |
| [Unity Entities transform concepts](https://docs.unity.cn/Packages/com.unity.entities%401.0/manual/transforms-concepts.html) | engine ECS docs | `LocalTransform`, one `Parent`, computed `LocalToWorld`, freshness caveat | Aicadia component or update system |
| [Unity Entities using transforms](https://docs.unity.cn/Packages/com.unity.entities%401.2/manual/transforms-using.html) | engine ECS docs | large single-root hierarchy warning | Aicadia's acceptable depth or fan-out |
| [Unity `SetParent`](https://docs.unity3d.com/6000.0/Documentation/ScriptReference/Transform.SetParent.html) | engine API | explicit preserve-world versus preserve-local reparenting | product semantics |
| [Unreal attachment](https://dev.epicgames.com/documentation/en-us/unreal-engine/BlueprintAPI/Transformation/AttachActorToActor) | engine API | parent attachment has explicit transform rules | ownership, contents or authority |
| [Unreal Large World Coordinates](https://dev.epicgames.com/documentation/en-us/unreal-engine/large-world-coordinates-in-unreal-engine-5) and [Niagara LWC](https://dev.epicgames.com/documentation/en-us/unreal-engine/large-world-coordinates-in-niagara-for-unreal-engine) | engine docs | double precision and tile-plus-local implementation pressures | Aicadia numeric representation |
| [Godot large-world coordinates](https://docs.godotengine.org/en/stable/tutorials/physics/large_world_coordinates.html) | engine docs | precision loss, double cost and multiplayer origin-shift complexity | accepted extent or precision |
| [Unreal Replication Graph](https://dev.epicgames.com/documentation/en-us/unreal-engine/replication-graph-in-unreal-engine) | engine networking docs | persistent candidate groups for bounded per-client replication | persistent World truth or million-player proof |
| [Unreal Actor relevancy](https://dev.epicgames.com/documentation/en-us/unreal-engine/actor-relevancy-and-priority-in-unreal-engine) | engine networking docs | replication relevance and owner-based rules | authorization or knowledge |
| [Godot `AStar3D`](https://docs.godotengine.org/en/stable/classes/class_astar3d.html) | engine API | stored points/connections versus calculated ordered path | Aicadia topology name or durable Route |
| [PostGIS spatial indexes](https://postgis.net/documentation/faq/spatial-indexes/), [`ST_DWithin`](https://postgis.net/docs/ST_DWithin.html) and [`<->`](https://postgis.net/docs/geometry_distance_knn.html) | project manuals | index-aware candidate, distance and nearest reads | semantics, authorization or bounded match count |
| [PostgreSQL recursive queries](https://www.postgresql.org/docs/current/queries-with.html#QUERIES-WITH-CYCLE) | database manual | termination and cycle detection for graphs | safe unbounded traversal |
| [PostgreSQL generated columns](https://www.postgresql.org/docs/current/ddl-generated-columns.html) | database manual | generated value cannot reference ancestor rows | projection design |
| [PostgreSQL concurrency](https://www.postgresql.org/docs/current/mvcc.html) and [locking](https://www.postgresql.org/docs/current/explicit-locking.html) | database manual | snapshots, row conflict and limits of lock skipping | Aicadia transaction design |
| [PostgreSQL row security](https://www.postgresql.org/docs/current/ddl-rowsecurity.html) | database manual | row filtering and default deny | World-level game authorization design |
| [Zanzibar](https://www.usenix.org/conference/atc19/presentation/pang) | original Google/USENIX paper | object checks, causal permission consistency and hot/nested scale pressure | universal World relation graph |
| [Macaroons](https://research.google/pubs/macaroons-cookies-with-contextual-caveats-for-decentralized-authorization-in-the-cloud/) | original paper | attenuated contextual capability credentials | need for bearer delegation in Aicadia |

## Research conclusion

**External finding.** OpenUSD, Godot, Unity and Unreal converge on explicit local
placement plus derived world placement; PostgreSQL and PostGIS can support bounded
graph and spatial reads but do not automatically index cross-row derived world
points; large-scale network and authorization systems keep relevance, access and hot
data treatment explicit; pathfinding keeps direct connections distinct from path
output.

**Aicadia implication.** The strongest five-year-safe direction is a small layered
foundation: one exact Position per positioned Entity, optionally relative to exactly
one Entity and therefore mechanically following it; one resolved world point for an
eligible read; free authored spatial meaning outside the exact mechanical fields;
Place using the same Position rather than duplicate coordinates; optional geometry
and Area; direct topology separate from calculated journeys; and authorization,
private contents and remote causality as independently earned mechanics.

**User decision still required.** This research cannot accept that direction. The
first negotiation remains whether Entity-relative Position itself means persistent
following and whether an authorized Position read returns the resolved World point.
Only after that choice do the depth, numeric representation, index and API
experiments become well-posed.
