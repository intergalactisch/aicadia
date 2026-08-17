---
status: pending
era: August Activity-Property-Trait
---

# Spatial models for a very large persistent game world

> **Role / side:** sourced research report / development side.
> **Authority:** records this report's question, sources, findings and implications.
> **Excludes:** product decisions and current implementation contracts; see `docs/game/`.

Date: 2026-08-08

Status: research and recommendation for discussion; not accepted game behavior or
implementation direction

## Evidence labels

- **External fact** reports what a cited system or specification actually does.
- **Inference** translates several external facts into a consequence for Aicadia.
- **Recommendation** proposes a choice for the current grill. It is not accepted
  until the user confirms it and `docs/game/` records it.
- **Open question** identifies a choice the evidence does not settle.

## Question

For Aicadia, an immense persistent shared-world discovery and settlement game, what
spatial model should distinguish:

- world coordinates;
- bounded semantic `Place` records;
- containment and gameplay locality;
- visibility and interest-management partitions;
- travel and connectivity; and
- current `Character` and `Entity` location?

The immediate behavior under discussion is deliberately smaller:

> Character A is at a Place, introduces one Entity there, and Character B at the
> exact same Place can read that Entity.

Movement is not part of this slice. The research must therefore explain the scale
path without forcing coordinates, boundary geometry, route topology or storage
shards into the first implementation.

## Aicadia constraints

The current executable contract has one `World`, durable `User` records and shared
`Entity` records. It explicitly defers `Character`, location and movement. The grill
has provisionally selected one durable `Character` per `User`; this report does not
accept that choice by itself.

Other always-on constraints are material:

- Postgres and one `World` interface remain the infrastructure boundary.
- HTTP and MCP must expose the same player-facing semantics.
- The server validates deterministically and performs no inference.
- There are no fixed scores, levels or currencies.
- Aicadia-owned names are singular.
- Storage partitions must not become fictional world concepts accidentally.
- Research may recommend a contract but may not install it as current truth.

## Short answer

**Inference:** comparable systems consistently separate at least two of the following
concerns, and the most useful systems separate all of them:

1. a stable semantic identity for the place players refer to;
2. a coordinate or geometry describing exact physical extent;
3. a coarse cell, block, region or bubble used to load and query data; and
4. a graph or movement rule describing what can be reached.

**Recommendation:** Aicadia should make `Place` a semantic role of an existing
`Entity`, identified by the same UUID. The current local-discovery scope should use
an explicit `place_id`, not a polygon calculation and not a storage-cell id. Exact
geometry, containment paths, observation rules, route edges and interest cells
should remain separate optional layers which earn their way in through concrete
behavior.

For the first slice, no coordinate, boundary, size, kind hierarchy, parent Place,
route or spatial extension is needed. Exact equality of two stored Place IDs is the
complete inclusion rule for the proposed Place-scoped discovery read. It does not
claim that the Characters can see, hear or interact with everything sharing that
Place. That is small without closing the path to a vast world.

## Comparison

| System | Authoritative primitive | Scale or partition | Boundary semantics | Stored live location | Lesson for Aicadia |
|---|---|---|---|---|---|
| EVE Online | A universe of regions, constellations, solar systems and celestial objects with stable IDs and positions | Regions and constellations organize systems; coordinates exist at cluster scale and separately relative to a solar-system origin | Region/constellation/system membership is ID-based rather than published as gameplay polygons; route connectivity is an explicit stargate graph and need not follow geometric proximity | The official character-location operation reports a solar system and, when applicable, a station or structure rather than requiring a continuously exposed exact point | Coarse semantic location, exact coordinates and reachability are separate concerns. [Map data](https://developers.eveonline.com/docs/guides/map-data/), [route calculation](https://developers.eveonline.com/docs/guides/route-calculation/), [ESI API explorer](https://developers.eveonline.com/api-explorer#/operations/GetCharactersCharacterIdLocation) |
| Second Life | A named Region plus objects and avatars at Region-relative coordinates; parcels are separate land records | A normal Region is 256 m by 256 m and is hosted by a simulator; global position can be composed from the Region corner and Region-relative position | Parcels are made from 4 m by 4 m land units, can be non-contiguous, and stay within one Region; parcel identity, access and visibility rules are distinct from the Region's simulator boundary | `llGetPos` returns Region coordinates; `llGetRegionCorner` returns the Region's global south-west corner; object-detail queries are primarily Region-scoped | Coordinate frames, simulation ownership and semantic/access areas may overlap without being the same concept. [Land model](https://wiki.secondlife.com/wiki/Land), [`llGetPos`](https://create.secondlife.com/script/lsl-reference/functions/llgetpos/), [`llGetRegionCorner`](https://create.secondlife.com/script/lua-reference/functions/llgetregioncorner/), [`llGetObjectDetails`](https://create.secondlife.com/script/lsl-reference/functions/llgetobjectdetails/) |
| Luanti | Integer-positioned nodes; a `MapBlock` stores 16×16×16 nodes and static objects | A map chunk groups 5×5×5 MapBlocks for generation; the default world is a roughly 60,000-node cube | The engine block is a persistence, generation and transfer unit, not a built-in named settlement or room boundary | Players and objects carry positions in the voxel world; map data is loaded and persisted by block | A chunk can be operationally essential without becoming a player-visible Place. [Basic data structures](https://docs.luanti.org/for-engine-devs/basic-data-structures/), [engine structure](https://docs.luanti.org/for-engine-devs/structure/), [world boundaries](https://docs.luanti.org/for-players/world-boundaries/) |
| Cataclysm: DDA | Absolute map-square coordinates plus explicit coordinate scales: submap, overmap terrain and overmap | One submap is 12×12 map squares; one overmap terrain is 2×2 submaps; one overmap is 180×180 overmap terrains; only a local reality bubble is active at full detail | An overmap terrain names coarse content; a multi-tile `overmap_special` groups several terrain tiles into one conceptual feature. Fixed grid boundaries remain generation primitives | Code converts between absolute and local coordinates; detailed submaps load or save as they cross the reality bubble | Coarse world description, detailed local state and active interest scope can be separate resolutions, but fixed engine levels need not become Aicadia domain levels. [Coordinate systems](https://docs.cataclysmdda.org/c%2B%2B/POINTS_COORDINATES.html), [overmap model](https://docs.cataclysmdda.org/JSON/OVERMAP.html), [map generation](https://docs.cataclysmdda.org/JSON/MAPGEN.html) |
| OpenTTD | A dense rectangular map of `TileIndex` records plus separate Town objects | Each map dimension is 64 through 4096 tiles; tiles carry compact type-specific state | A Town has a centre tile and cached concentric zone radii calculated from house count; it is not represented by one authored polygon | Vehicles and structures use tile-based positions; Town proximity and zone are calculated from the Town centre and tiles | Simulation grids are excellent when every action is cell-based, but a derived influence radius should not be mistaken for a semantic settlement boundary. [Map types](https://docs.openttd.org/source/de/dc0/map__type_8h), [Town functions](https://docs.openttd.org/source/d7/d3c/town_8h), [house zones](https://docs.openttd.org/source/d3/de8/house_8h_source) |
| Overture Maps | Stable feature IDs; point `place` records; separate `division`, `division_area` and `division_boundary` features | Global columnar data is filtered by bounding boxes; hierarchy is explicit data rather than a storage level | A Division can participate in multiple hierarchies; areas are Polygon/MultiPolygon records and shared boundaries are LineString/MultiLineString records. Disputed perspectives are represented explicitly | Place and Division identity retain representative Point geometry while area and boundary records refer back by ID | Stable identity, representative point, area, boundary and hierarchy do not have to be one row or one truth. [Places guide](https://docs.overturemaps.org/guides/places/), [Divisions guide](https://docs.overturemaps.org/guides/divisions/), [Division schema](https://docs.overturemaps.org/schema/reference/divisions/division/), [Division boundary schema](https://docs.overturemaps.org/schema/reference/divisions/division_boundary/) |
| PostGIS and H3 | PostGIS stores planar feature geometry with an SRID; H3 assigns hierarchical indexes to spherical grid cells | GiST provides bounding-box-assisted spatial search; H3 offers resolutions 0–15 and parent/child cell operations | `ST_Covers` includes an area's boundary; H3 has exact logical hierarchy but only approximate geometric parent-child containment across resolutions | Neither system defines game presence: applications store source location and use geometry/cells to query it | Geometry and cell indexes are query machinery. A precise source Place relation still needs an application rule. [PostGIS geometry](https://postgis.net/docs/en/geometry.html), [`ST_Covers`](https://postgis.net/docs/ST_Covers.html), [PostGIS spatial indexes](https://postgis.net/documentation/faq/spatial-indexes/), [H3 introduction](https://h3geo.org/docs/), [H3 indexing](https://h3geo.org/docs/highlights/indexing/) |

The Luanti statement that a MapBlock is not a named Place is an **inference** from
the official engine structures: the engine defines nodes, blocks, objects and
metadata while game or mod code supplies meaning. The other comparison cells report
directly documented structures or operations.

## Findings

### 1. Coordinate choice

**External fact:** EVE uses one cluster coordinate system for regions,
constellations and systems, and a second system relative to the centre of a solar
system. Both use metres, but their axis conventions differ. Second Life likewise
combines a global Region corner with Region-relative object positions. These are two
examples of a large world retaining the coordinate frame as part of the location
meaning rather than treating a naked `(x, y, z)` as universal. [EVE map
data](https://developers.eveonline.com/docs/guides/map-data/), [Second Life
`llGetPos`](https://create.secondlife.com/script/lsl-reference/functions/llgetpos/),
[Second Life
`llGetRegionCorner`](https://create.secondlife.com/script/lua-reference/functions/llgetregioncorner/)

**External fact:** PostGIS `geometry` represents features in a planar coordinate
system, and every operation uses the units of its Spatial Reference System. PostGIS
recommends choosing one stored SRID so indexes remain useful. Its point constructors
use floating-point coordinates; current PostGIS documentation states that coordinate
storage has about fifteen significant decimal digits. [PostGIS geometry
type](https://postgis.net/docs/en/geometry.html), [PostGIS projection
guidance](https://postgis.net/workshops/postgis-intro/projection.html), [PostGIS
manual](https://postgis.net/docs/postgis-en.html)

**Inference:** “Immense” does not require coordinates in the first slice. Prematurely
choosing a flat infinite plane, a sphere, a finite map, separate planets or nested
interiors would settle world topology through a database type. The same-Place use
case needs identity equality only.

**Recommendation:** when exact mapping first becomes necessary, use a declared
fictional planar `world_srid` and PostGIS `geometry`, with a documented unit, origin,
axis direction and valid bounds. Never store coordinate numbers without a frame or
SRID. Add local frames only after a concrete interior, moving container, planet or
precision case proves that one global frame fails.

**Open question:** is Aicadia's physical World ultimately one plane, a wrapped or
spherical surface, several disconnected spaces, or a graph of partly mapped spaces?
Nothing in the first local-discovery slice should answer this accidentally.

### 2. Semantic Places and engine partitions are different

**External fact:** Second Life's Region is a simulator-hosted square, while a parcel
is a separately identified land area within that Region and may be non-contiguous.
Luanti's MapBlock is a persistence and transfer unit. Cataclysm's reality bubble is
an active-detail window, while overmap terrains and specials describe coarse world
content. [Second Life land
model](https://wiki.secondlife.com/wiki/Land), [Luanti engine
structure](https://docs.luanti.org/for-engine-devs/structure/), [Cataclysm coordinate
systems](https://docs.cataclysmdda.org/c%2B%2B/POINTS_COORDINATES.html), [Cataclysm
overmap](https://docs.cataclysmdda.org/JSON/OVERMAP.html)

**Inference:** a Place answers “which world area are these actors talking about?” An
interest partition answers “which rows should this server process or send now?”
Those questions can happen to share a key in a particular engine, but Aicadia should
not make that coincidence part of canon.

**Recommendation:** never expose `map_block`, `chunk`, `shard`, `H3` or another cell
identifier as `place_id`. If scale later requires partitions, derive them from
geometry or assign them operationally and keep them rebuildable. Moving an interest
boundary must not rename or move a fictional Place.

### 3. Boundary representation and edge semantics

**External fact:** common spatial feature models support Point, LineString, Polygon,
MultiPolygon and geometries with holes. Overture uses a representative Point for a
Division, a separate Polygon or MultiPolygon for its area, and separate line
features for shared boundaries. [Overture Divisions
guide](https://docs.overturemaps.org/guides/divisions/), [OGC Simple Features
1.1](https://docs.ogc.org/is/99-050/99-050.pdf)

**External fact:** `ST_Covers(area, point)` returns true for both the interior and the
boundary, unlike `ST_Contains`, which does not consider a geometry to contain its
entire boundary. It uses available spatial indexes as a prefilter and warns that
invalid geometries produce unexpected results. [PostGIS
`ST_Covers`](https://postgis.net/docs/ST_Covers.html)

**Inference:** geometry alone cannot guarantee one unique Place. If two adjacent
polygons share an edge and boundary-inclusive `ST_Covers` is used, the same point can
be covered by both. Nested and overlapping Places create the same multiplicity away
from an edge. A strict server therefore needs an explicit gameplay rule in addition
to a geometric predicate.

**Recommendation:** the first slice defines its discovery-list scope by exact
`place_id` equality, so that read has no geometric edge case. When geometry arrives:

- use `ST_Covers` when a specific rule says a boundary point counts as inside;
- treat returned covering Places as spatial candidates, not an automatic canonical
  current Place;
- validate geometry before using it;
- require a named extent meaning where one Place has several boundaries; and
- keep one explicit operational `current_place_id` for Place-scoped actions even
  when several extents cover the same point.

### 4. Arbitrary size, holes, nesting and overlap

**External fact:** Second Life parcels range from one 4 m by 4 m land unit to a full
Region and may contain disconnected pieces. Overture uses MultiPolygon areas and
allows a Division to participate in more than one hierarchy. H3 cells, by contrast,
are fixed grid units at defined resolutions rather than arbitrary semantic extents.
[Second Life land](https://wiki.secondlife.com/wiki/Land), [Overture Division
schema](https://docs.overturemaps.org/schema/reference/divisions/division/), [H3 cell
model](https://h3geo.org/docs/library/index/cell/)

**Inference:** minimum and maximum Place size, one required parent, and a fixed
`world → region → district → room` depth are not general truths. A forest can contain
a cottage while overlapping a jurisdiction; a cave can share surface coordinates
with a road; an archipelago or parcel can be disconnected.

**Recommendation:** do not store `size` as source truth. If an accepted extent later
exists, calculate area from the selected geometry and its coordinate units. Permit
Point, LineString, Polygon and MultiPolygon as different precisions or physical
forms; permit holes, nesting and overlap. Store only explicit containment relations
the game establishes, with no mandatory levels. For the proposed Place-scoped
discovery read, every Character has one most-specific current Place at a time. What
that Place authorizes or exposes remains action-specific.

**Open question:** does the future game allow a Character to be simultaneously in
several gameplay Places, or will one current Place remain primary while other
covering or social Places are context? The first slice should use one primary Place
and leave this open.

### 5. Place types and kinds

**External fact:** Cataclysm's `overmap_terrain` has a fixed content ID and an
`overmap_location` can group several terrain values. Overture has finite Division
subtypes and a separate Place taxonomy which can include primary and alternate
categories. These classifications exist because those systems already have concrete
generation, search or cartographic consumers. [Cataclysm
overmap](https://docs.cataclysmdda.org/JSON/OVERMAP.html), [Overture Place
taxonomy](https://docs.overturemaps.org/schema/reference/places/types/taxonomy/),
[Overture Divisions guide](https://docs.overturemaps.org/guides/divisions/)

**Inference:** a `place_type` enum such as `region`, `city`, `district`, `building`,
`room` would create a required geographic ladder and would make fictional vocabulary
server-owned before any behavior uses it.

**Recommendation:** the first slice needs only the structural Place role: this Entity
can anchor a Character or located Entity. It needs no `kind`, `type`, `level`, size
class or hierarchy depth. Later world-authored kinds can use the classification
direction already researched in [Kind classification](kind-classification.md). Add a
closed mechanic-specific type only if deterministic server behavior actually differs
by that type.

An extent's **purpose** is a different concern from a Place's **kind**. “Built-up
footprint”, “city wall” and “claimed jurisdiction” can be three extents of one city;
none says whether that city is a village, capital or ruin.

### 6. Huge-scale indexing and partitioning

**External fact:** Luanti persists 16×16×16 MapBlocks and generates larger chunks;
Cataclysm loads and saves 12×12 submaps around a limited reality bubble; PostGIS GiST
indexes use bounding boxes to prefilter spatial predicates; H3 assigns compact
hierarchical cell indexes. [Luanti basic data
structures](https://docs.luanti.org/for-engine-devs/basic-data-structures/),
[Cataclysm coordinates](https://docs.cataclysmdda.org/c%2B%2B/POINTS_COORDINATES.html),
[PostGIS spatial indexes](https://postgis.net/documentation/faq/spatial-indexes/), [H3
indexing](https://h3geo.org/docs/highlights/indexing/)

**External fact:** H3 explicitly distinguishes exact logical parentage in the index
from approximate geometric containment across resolutions, and recommends an exact
point-in-polygon check when boundaries matter. [H3
introduction](https://h3geo.org/docs/), [H3
indexing](https://h3geo.org/docs/highlights/indexing/)

**Inference:** the correct first index for same-Place discovery is an ordinary B-tree
on `place_id`. It is exact, cheap and proportional to the current query. A spatial
index is useful only after exact geometry is stored; a fixed-cell key is useful only
after query or distribution pressure makes it measurable.

**Recommendation:** scale in this order:

1. B-tree exact Place membership with keyset pagination.
2. GiST geometry index when nearby, extent or map-window queries exist.
3. A derived coarse cell or bounding-box column when profiling proves it reduces
   candidates beyond GiST and Place scope.
4. Operational partitioning by region/cell/range only when one Postgres layout has a
   measured limit.

At every stage, `place_id` remains semantic source state; the index and partition
keys remain infrastructure.

### 7. Place-scoped discovery query

**External fact:** EVE's public route documentation treats solar systems as graph
nodes and stargates as edges. Second Life's object-detail call looks primarily in the
current Region, with a narrow adjacent-region exception for avatars. Both are
examples of bounded lookup based on current operational context rather than a global
scan. [EVE route
calculation](https://developers.eveonline.com/docs/guides/route-calculation/), [Second
Life `llGetObjectDetails`](https://create.secondlife.com/script/lsl-reference/functions/llgetobjectdetails/)

**Recommendation:** for the first slice, resolve the request's Character from User
request context and infer the Place. The Agent does not submit `character_id` or
`place_id`. This scopes the operation but does not authenticate the caller; the
current transport assertion remains untrusted. Local listing is exact equality:

```sql
SELECT e.id, e.name, e.description, e.introduced_by_user_id, e.introduced_at
FROM character AS c
JOIN entity_location AS el
  ON el.place_id = c.current_place_id
JOIN entity AS e
  ON e.id = el.entity_id
WHERE c.user_id = $1
ORDER BY e.introduced_at DESC, e.id DESC;
```

Creating the locally discovered Entity happens in one transaction:

1. resolve the request User's one Character;
2. read that Character's `current_place_id`;
3. insert the Entity;
4. insert its `entity_location` with the resolved Place; and
5. return the Entity with its current Place context.

The Entity appears in Character B's Place-scoped discovery read exactly when B's
resolved `current_place_id` equals the stored `entity_location.place_id`. No
polygon, distance or ancestor traversal participates. Sensory visibility is a
different behavior and remains undecided.

### 8. Movement and travel are separate from location

**External fact:** EVE route planning uses an explicit system/stargate graph and
warns that geometric closeness can mislead routing because “pocket” space can be near
in 3D but far through gates. Cataclysm also stores explicit overmap connections for
roads and multi-tile specials. [EVE route
calculation](https://developers.eveonline.com/docs/guides/route-calculation/),
[Cataclysm overmap](https://docs.cataclysmdda.org/JSON/OVERMAP.html)

**Inference:** an overlap, shared boundary or short coordinate distance does not
prove that travel is allowed. Conversely, a portal or stargate can connect distant
Places.

**Recommendation:** keep future `place_edge` or equivalent connectivity separate
from Place geometry. Do not add it to the first slice. Until movement is accepted,
`current_place_id` is provisioned state with no player-facing mutation operation.

### 9. Mutable geometry

**External fact:** Overture separates a Division identity from its area and boundary
features and increments feature versions as data changes. PostGIS supports geometry
validation and version-independent spatial indexing but does not define application
history semantics. [Overture Divisions
guide](https://docs.overturemaps.org/guides/divisions/), [Overture Division
schema](https://docs.overturemaps.org/schema/reference/divisions/division/), [PostGIS
reference](https://postgis.net/docs/reference.html)

**Inference:** changing a Place extent must not change its identity, rewrite every
contained Entity or silently change its travel graph. A forest growing around a
house and a house moving into a forest are different state changes.

**Recommendation:** geometry remains absent now. When mutable extents become current
behavior, add provenance-carrying extent versions with explicit meaning and effective
time rather than one silently overwritten Place polygon. Current geometry may be a
projection over those versions. Do not add event sourcing or a general claim system
merely to obtain this property; the exact history mechanism must fit the build
contract at that time.

## Reconciliation with existing Aicadia research

This report narrows earlier spatial research to the next concrete game edge. It does
not replace the still-open parts of those reports.

- [Spatial state](spatial-state.md) found that a world point and its semantic
  Place/route context answer different questions. Its travel model remains deferred.
  This report takes the smallest compatible step: Place context without a point or
  route.
- [Mutable place geometry](mutable-place-geometry.md) corrected the idea that one
  mutable geometry could represent every meaning and time of one Place. This report
  therefore does not put a `geometry` column on the first `place` role or invent a
  single `size`.
- [Hierarchical spatial placement](archive/hierarchical-spatial-model.md) distinguishes
  containment from exact geometry and warns against exposing storage cells as
  Places. This report adopts that distinction but follows the later relaxation below
  instead of requiring a containing Place for every possible Entity.
- [Open spatial world systems](archive/open-spatial-world-system.md) already compared
  city-builder grids, geographic identity and delivery indexes. Its central finding
  remains: grids and map tiles are not canon. This report adds MMO location examples
  and turns that finding into a first-slice schema.
- [Stable identity and sparse location](stable-identity-and-sparse-location.md) is
  the most specific later correction: the same UUID identifies an Entity which also
  has a Place role, hierarchy levels are optional, and a current discrete location
  may be Place-level, geometry-level or route-level. This report selects the
  Place-level form only for the first local-discovery behavior; it does not require
  every Entity in the World to have a Place.
- [Spatial occurrence and field](spatial-occurrence-and-field.md) further narrows
  location to discrete spatial occurrences rather than abstract materials or
  conditions. This is why `entity_location` is optional: an abstract Entity such as
  a material kind need not appear in a local Place list.
- [Locality, co-presence and
  observation](locality-co-presence-and-observation.md) shows that comparable
  systems define locality per action and compose direct location, distance,
  access, observation and technical relevance. This report's exact Place equality
  is therefore only the proposed discovery-read scope, not a universal visibility
  law.

One tension is explicit. [Hierarchical spatial
placement](archive/hierarchical-spatial-model.md) originally recommends a containing Place
for every physical Entity; [Stable identity and sparse
location](stable-identity-and-sparse-location.md) later permits exact geometry
without a Place, and [Spatial occurrence and
field](spatial-occurrence-and-field.md) replaces “physical” with “discrete spatial
occurrence.” This report follows the later, narrower position. The first slice still
requires a Place for the specific Character and discovered Entity because a
same-Place discovery read is the behavior being considered.

## Concrete Aicadia recommendation for discussion

### Domain distinctions

| Concept | Meaning | Authoritative in the first slice? |
|---|---|---|
| `Place` | An Entity which can anchor a Place-scoped game action | Yes |
| `current_place_id` | The one operational Place occupied by a Character | Yes |
| `entity_location.place_id` | The Place containing one discrete local Entity at current known precision | Yes, for locally introduced Entities |
| world geometry | Optional exact point, line or extent in a declared coordinate frame | No; absent |
| Place containment | An established relation between Places | No; deferred |
| Place extent | One meaning- and time-specific geometry for a Place | No; deferred |
| travel edge | A reachable connection between Places | No; deferred |
| interest cell or shard | An operational query/hosting partition | No; derived later |
| map tile or level of detail | A delivery representation | No; derived later |

### Smallest current schema

This is a discussion candidate, not a migration plan:

```text
place
  entity_id uuid primary key references entity(id)

character
  id uuid primary key
  user_id uuid not null unique references user(id)
  current_place_id uuid not null references place(entity_id)

entity_location
  entity_id uuid primary key references entity(id)
  place_id uuid not null references place(entity_id)
```

Properties deliberately absent:

```text
place.type
place.level
place.size
place.geometry
place.parent_id
place.cell_id
character.coordinate
entity.coordinate
route or movement state
```

`place.entity_id` means an inhabited house can remain one Entity and also anchor
Characters. It does not create a second Place identity or copy the Entity name and
description.

The exact creation and provisioning path for the initial Place and one Character per
User remains part of the game-contract decision. It is not automatically an Agent
tool.

### Deterministic invariants

1. One `User` has exactly one `Character` in the selected first-slice model.
2. Every Character has exactly one existing `current_place_id`.
3. A Place is identified by the same UUID as its underlying Entity.
4. A locally introduced discrete Entity has exactly one current
   `entity_location`.
5. The server copies the actor Character's current Place into the new location; the
   caller cannot nominate another Place.
6. Two Characters share the proposed discovery-read scope if and only if their
   stored `current_place_id` values are equal.
7. A located Entity appears in a Character's Place-scoped discovery read if and
   only if its stored `place_id` equals that Character's stored
   `current_place_id`.
8. Entity or Place names never participate in equality.
9. An Entity without `entity_location` is not a local discrete occurrence and does
   not appear in this local query.
10. No geometry, inherited ancestor, distance, partition cell or route is consulted.

These invariants prove only the proposed same-Place discovery slice. They do not
define co-presence for every action, line of sight, secrecy, ownership, discovery
per observer, range, movement or parent-Place observation.

### Allowed examples

- Character A and Character B both store `current_place_id = moss-end`. A introduces
  a particular spring. The server stores the spring's location as `moss-end`; B's
  local list returns it.
- A house Entity receives a Place role using the same UUID. Characters can be at the
  house without creating a second “house location” Entity.
- An abstract Entity named `amberwood` has no `entity_location`. It remains globally
  addressable by ID but is absent from a Place-local occurrence list.
- Two different Places both named “Moss End” remain different because their IDs
  differ.

### Rejected examples

- Character A is at `moss-end` but submits `place_id = distant-port` while creating a
  local Entity.
- A server infers that two Places with the same name are the same Place.
- A point happens to lie on two polygons, so the server silently chooses one as the
  current Place.
- An H3, chunk, map-block or database-partition key is exposed as the Place's stable
  identity.
- Every Entity is forced to have a location even when it represents a material,
  species, idea or other reusable concept.
- An Entity in a nested room automatically appears to a Character in the containing
  district; descendant expansion is not part of this exact same-Place read.
- Introducing a local Entity implicitly creates a route, claim, settlement boundary
  or ownership right.

### Deferred capabilities

- movement and changing `current_place_id`;
- routes, travel time, portals and reachability;
- world topology, coordinate frame, origin, units and bounds;
- Place extent, boundary editing, geometry versions and area calculation;
- Point, LineString, Polygon or MultiPolygon Entity geometry;
- Place containment, ancestors and descendant-expanded discovery reads;
- overlapping spatial context and boundary tie-breaking;
- line of sight, range, occlusion and per-Character discovery state;
- Place kind or classification mechanics;
- map tiles, H3/grid cells, shards and simulator ownership;
- automatic generation, simulation or server-created semantic content; and
- history machinery, event sourcing, general claims and projections not already in
  the current contract.

### Scale-evolution path

1. **Exact Place scope:** B-tree index `entity_location(place_id)` and existing
   keyset pagination. This supports the first slice and many Places with sparse
   contents.
2. **Nested Place context:** add explicit Place-to-Place relations and a rebuildable
   traversal or path only when a concrete briefing/query needs ancestors or
   descendants.
3. **Exact mapping:** add optional PostGIS geometry with an explicit fictional SRID
   when distance, boundaries, maps or within-Place distinction becomes current
   behavior.
4. **Travel:** add explicit connectivity independently of geometry. A route may link
   distant Places and may omit adjacent ones.
5. **Local interest inside a large Place:** when one Place contains enough current
   Entities to make equality scope too broad, query its indexed geometry by distance
   or map window.
6. **Operational partitioning:** derive cell keys, ranges or shards from current
   geometry and measured load. Keep those keys out of accepted world identity.
7. **Multiple coordinate spaces:** add an explicit frame or `space_id` only when a
   sphere, disconnected world, moving container or precision limit provides a real
   counterexample to one planar World frame.

Each step leaves Place identity stable and adds one separately testable capability.

### Tradeoffs and alternatives

#### Recommended: explicit Place membership first

Advantages:

- proves the selected multiplayer outcome with three small domain records;
- requires no invented map topology or boundary precision;
- scales through an ordinary indexed equality query;
- prevents a caller from acting remotely by supplying coordinates or another Place;
- permits exact geometry later without identity migration; and
- keeps abstract Entities unlocated.

Costs:

- two Characters at different nested Places do not share this discovery scope until
  a later containment rule exists;
- a very large Place can be an overly broad discovery scope; and
- no map or distance query is possible yet.

#### Alternative: require an exact polygon for every Place now

This permits point-in-polygon validation immediately but forces world topology,
coordinate units, boundary provenance, overlap policy and edge semantics into a
slice which uses none of them. It also creates false precision for unmapped Places.
Not recommended.

#### Alternative: use fixed cells as Places

This makes partitioning and locality cheap, as in voxel and tile engines, but leaks
the engine grid into world vocabulary and cannot naturally represent overlapping,
nested, disputed or non-contiguous Places. Not recommended for Aicadia's semantic
world.

#### Alternative: store only coordinates and derive Place

This supports nearby queries but makes Place membership ambiguous at borders and
overlaps, requires every occurrence to have exact geometry, and cannot represent
semantic membership which disagrees with a current polygon. Not recommended as the
source model.

#### Alternative: make every Character an Entity and reuse one location table

This can reduce one special location field, but it also means `Entity` queries and
creation semantics must distinguish Characters immediately. The current contract
explicitly defines User and Entity but has not decided whether Character is an
Entity. Keep this as an open domain question rather than smuggling it through the
spatial schema.

## Open questions for the grill

The research supports asking these in order, one at a time:

1. Is a `Place` a role of an `Entity` using the same UUID, rather than a separate
   identity?
2. Is exact stored `place_id` equality the complete inclusion rule for the first
   Place-scoped discovery read, without making it a universal co-presence or
   visibility rule?
3. Does the server infer the actor Character and current Place from User context, so
   no Agent supplies either ID when introducing a local Entity?
4. May global abstract Entities continue to exist without `entity_location`, while
   a newly discovered local occurrence receives one?
5. How is the initial Place and one Character per User provisioned without turning
   administration into an Agent capability?
6. Does the first slice alter `list_entity`, add a current-Place filter, or introduce
   a separate operation? The semantic query is decided before its public name.
7. Only when exact mapping becomes the selected edge: what is the World topology,
   coordinate frame, unit, origin and boundary policy?

## Primary source audit

Every factual comparison above is supported inline by an owning source:

- CCP Games: [Map Data](https://developers.eveonline.com/docs/guides/map-data/),
  [Route Calculation](https://developers.eveonline.com/docs/guides/route-calculation/),
  [ESI API Explorer](https://developers.eveonline.com/api-explorer)
- Linden Research: [Land](https://wiki.secondlife.com/wiki/Land),
  [`llGetPos`](https://create.secondlife.com/script/lsl-reference/functions/llgetpos/),
  [`llGetRegionCorner`](https://create.secondlife.com/script/lua-reference/functions/llgetregioncorner/),
  [`llGetObjectDetails`](https://create.secondlife.com/script/lsl-reference/functions/llgetobjectdetails/)
- Luanti project: [Basic data
  structures](https://docs.luanti.org/for-engine-devs/basic-data-structures/), [Engine
  structure](https://docs.luanti.org/for-engine-devs/structure/), [World
  boundaries](https://docs.luanti.org/for-players/world-boundaries/)
- Cataclysm: DDA project: [Point and coordinate
  systems](https://docs.cataclysmdda.org/c%2B%2B/POINTS_COORDINATES.html), [Overmap
  model](https://docs.cataclysmdda.org/JSON/OVERMAP.html), [Map
  generation](https://docs.cataclysmdda.org/JSON/MAPGEN.html)
- OpenTTD project source documentation: [Map
  types](https://docs.openttd.org/source/de/dc0/map__type_8h), [Town
  functions](https://docs.openttd.org/source/d7/d3c/town_8h), [House
  zones](https://docs.openttd.org/source/d3/de8/house_8h_source)
- Overture Maps Foundation: [Places
  guide](https://docs.overturemaps.org/guides/places/), [Divisions
  guide](https://docs.overturemaps.org/guides/divisions/), [Division
  schema](https://docs.overturemaps.org/schema/reference/divisions/division/),
  [Division boundary
  schema](https://docs.overturemaps.org/schema/reference/divisions/division_boundary/)
- PostGIS project: [`geometry`](https://postgis.net/docs/en/geometry.html),
  [`ST_Covers`](https://postgis.net/docs/ST_Covers.html), [spatial-index
  guidance](https://postgis.net/documentation/faq/spatial-indexes/), [projection
  guidance](https://postgis.net/workshops/postgis-intro/projection.html)
- H3 project: [Introduction](https://h3geo.org/docs/), [cell
  model](https://h3geo.org/docs/library/index/cell/),
  [indexing](https://h3geo.org/docs/highlights/indexing/)
- Open Geospatial Consortium: [Simple Features
  1.1](https://docs.ogc.org/is/99-050/99-050.pdf)
