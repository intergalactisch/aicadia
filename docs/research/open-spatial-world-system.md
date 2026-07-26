# Open spatial world systems

Date: 2026-07-26

Status: research and recommendation, not concept direction

Follow-up: `stable-identity-and-sparse-location.md` replaces “one coarse containing
place” with a less restrictive current-location contract. Every physical entity
needs one current location row, but exact geometry may exist without a containing
place and no geographic hierarchy level is mandatory. This later contract is now
confirmed concept direction.

## Question

How do open-source city-building games, large world engines and geographic
information systems store, change and query houses, districts, cities and entire
worlds?

The concrete Aicadia question is:

> Is a house simply located in a district or other place containing several houses,
> or should every house receive its own exact world geometry?

## Short answer

The established systems use three different patterns because they solve three
different problems:

1. **City-building games use a fixed cell or tile as simulation truth.** A house
   occupies one or more cells. A town is a separate simulation object whose current
   influence or size is usually calculated from cells, roads and buildings.
2. **Geographic systems use a stable feature identity plus geometry, attributes and
   provenance.** A building, district or forest remains the same feature when its
   geometry is corrected or becomes more detailed.
3. **Large map viewers derive spatial tiles and levels of detail for delivery.** The
   tile is an index and payload boundary, not the identity of the building or city.

Aicadia should not copy one pattern completely. The useful combination is:

- stable `entity` and `place` identity from geographic systems;
- one coarse containing place when exact geometry is not established;
- optional, versioned geometry from geographic systems;
- a route graph for reachability;
- PostGIS indexes for exact spatial queries; and
- derived map tile or spatial cell indexes only when measured load requires them.

The scene log and accepted claims remain the source. A storage chunk, H3 cell or map
tile never becomes canon and never becomes a `place`.

## Comparison

| System | Authoritative spatial unit | How a larger place exists | How change works | Scale mechanism |
|---|---|---|---|---|
| OpenTTD | Dense 2D tile | Separate town object plus buildings, roads and calculated radii | Simulation tick adds, replaces or removes tile content | Compact tile array and cached town data |
| OpenRCT2 | Tile coordinate containing stacked tile elements | Park, ride and ownership data refer to map elements | Commands alter elements at coordinates and heights | Fixed dense map and compact elements |
| Luanti | 3D node stored in a `MapBlock` | Game meaning is supplied by nodes, objects and mod data | Changed blocks are persisted | Load and save 16×16×16 blocks; generate larger map chunks |
| Cataclysm: DDA | Detailed submap beneath a coarse overmap tile | Overmap records city, forest, river and road structure | Detailed submap is generated or loaded when approached | Coarse overmap, persistent submap and small active map |
| OpenStreetMap | Versioned node, way or relation | Boundary relation and tags describe a place | Upload creates a new element version and changeset | Spatial database, regional extracts and rendered tiles |
| Overture Maps | Stable feature ID with geometry and source data | Division identity, hierarchy and division area are separate | New dataset release advances feature versions | Columnar global dataset, stable registry and spatial partitioning |
| CityGML | Stable semantic city object | Explicit semantic relationships and alternate geometries | Object and model versions represent change through time | Consumer-selected level of detail |
| GRASS GIS / PostGIS | Vector feature, geometry, attributes and topology | Areas and relationships are queried or explicitly stored | Transactional feature edits | Spatial and topology indexes |
| H3 | Derived hierarchical hexagonal cell | Parent and child cells aggregate indexed geometry | Recalculate cell membership from current geometry | Fixed global hierarchy at multiple resolutions |
| 3D Tiles | Derived quadtree or octree payload | Parent tile bounds child tile content | Regenerate affected tile content | Random access, sparse subtrees and hierarchical level of detail |

## What open-source city-building and world games do

### OpenTTD: a town grows by changing tiles, not by editing a city polygon

OpenTTD keeps a dense map array. Each tile stores compact type-specific values such
as terrain or structure type, ownership and indices referring to towns, industries
or stations.

A town is a separate object with a centre tile and simulation state. Houses are
placed on concrete tiles for a particular town. Town growth follows roads, places
new houses on empty tiles and may demolish a building before replacing it. The
source also calculates cached town-zone radii from the current house state.

Concrete consequence:

```text
tile (214, 83)
  type: house
  town: Alden
```

OpenTTD does not need one exact, historically meaningful Alden boundary polygon. It
needs to decide quickly:

- whether a house can be built on this tile;
- which town owns or influences the tile;
- which road the next growth attempt follows; and
- which nearby stations affect the town.

Useful for Aicadia:

- keep high-volume placement changes independent from a city row;
- calculate expensive query aids as projections;
- do not rewrite a city entity for every new house.

Not suitable for Aicadia:

- a house must have an exact tile even when the story does not establish one;
- a simulation tick authors town growth;
- a radius based on house state is sufficient game mechanics but not a truthful
  fictional border.

### OpenRCT2: one horizontal coordinate can contain several vertical objects

OpenRCT2 stores several `TileElement` values at the same tile coordinate. Each element
has a type, base height and clearance height. Surface, path, track and scenery can
therefore occupy the same horizontal coordinate at different heights.

Useful for Aicadia:

- `(x, y)` alone cannot describe a bridge over a road, a room above another room or
  a cave beneath a forest;
- topology and containment remain necessary even if a 2D map exists.

Not suitable for Aicadia:

- compact fixed-map element encoding assumes a known game ontology and bounded map;
- Aicadia cannot reserve technical element slots for every kind of future fiction.

### Luanti: blocks are persistence units, not named places

Luanti divides its voxel world into `MapBlock` values of 16×16×16 nodes. A larger
map chunk contains 5×5×5 map blocks for generation. A saved map block includes node
data, metadata, objects and a timestamp. The map database can use SQLite or
PostgreSQL.

The important separation is:

```text
MapBlock coordinate  -> where data is loaded and saved
node/object meaning   -> what exists in the game
```

Useful for Aicadia:

- fetch and update only the relevant spatial region;
- a large world does not need to be resident in memory;
- operational storage partitioning may change without changing world meaning.

Not suitable for Aicadia:

- a `MapBlock` is exact voxel storage, while much Aicadia geography begins
  intentionally imprecise;
- the block must never leak into the place hierarchy or canon API.

### Cataclysm: DDA: coarse world structure and local detail are different layers

Cataclysm: DDA has:

- an overmap for large-scale cities, forests, rivers and roads;
- submaps containing detailed terrain and objects;
- a small active map around the player; and
- lazy generation or loading when the player approaches a new area.

A house may first exist at overmap scale as a house terrain type. Its detailed
24×24-tile interior is chosen and generated when the area is approached.

Useful for Aicadia:

- distant queries need less detail than a scene inside one house;
- a briefing can return a place path and selected nearby facts without loading every
  room or object;
- coarse placement can become exact later.

Important difference:

Cataclysm generates missing detail from deterministic game content. Aicadia may not
silently generate semantic facts server-side. Missing exact detail remains unknown
until an accepted scene establishes it.

## What geographic world systems do

### OpenStreetMap: stable element identity, exact geometry and full edit history

OpenStreetMap uses three structural element types:

- node for a point;
- way for a line or simple area; and
- relation for a route, boundary, multipolygon or other relationship.

Each stored element has an ID, version, timestamp, changeset and visibility state.
The server rejects an edit based on a stale version. The full editing history remains
available.

A building normally has its own outline. A city boundary is another feature, often a
boundary relation. Point-in-polygon queries can determine that the building lies
inside the city; the city ID is not necessarily copied onto the building.

Useful for Aicadia:

- entity identity survives geometry edits;
- exact geometry has provenance and optimistic concurrency;
- deletion from the current map does not erase history;
- a building and the administrative boundary containing it are separate features.

Important difference:

OpenStreetMap is a surveyed map. It generally expects geometry. Aicadia must also
represent “Mara's house is in Red Courtyard; its exact point is not established.”

### Overture Maps: place identity, hierarchy and area geometry are explicitly separate

Overture is unusually close to the current Aicadia question:

- `division` represents a country, city, town or neighbourhood;
- the division carries a representative `Point`;
- `division_area` separately carries the `Polygon` or `MultiPolygon` belonging to
  that division;
- `parent_division_id` provides a default parent;
- `hierarchies` can represent more than one political perspective; and
- a stable GERS ID is intended to survive dataset releases.

Overture also models a building part with its own footprint and a direct reference to
its parent building.

Concrete lesson:

```text
city identity     != city representative point
city identity     != city area polygon
building identity != building-part footprint
default hierarchy != every possible hierarchy
```

Useful for Aicadia:

- keep a place's durable identity separate from its spatial representations;
- a fast default containment path can coexist with other accepted relationships;
- a point and an area answer different questions;
- geometry and source provenance may advance without replacing identity.

This research weakens one overly broad interpretation of “one containing place.”
One operational parent is useful for a default physical query path, but it cannot be
the only world truth. A house can be physically inside Alden, governed by another
authority and claimed by two communities. Those extra relationships remain claims;
they do not all become parents in the default path.

### CityGML: one semantic object may have several geometries and versions

CityGML separates the semantic identity of a building, road, vegetation object, room
or other city object from its geometry. One object may have several spatial
representations at different levels of detail. The versioning model can represent
construction, demolition, extension and other change through time.

Useful for Aicadia:

- a house known only by its containing courtyard can later gain a point, footprint,
  rooms and entrances without becoming a different house;
- a map viewer and an LLM briefing do not need the same geometry detail;
- geometry is a property of an object, not the object's identity.

Not suitable for Aicadia:

- CityGML's fixed urban object taxonomy is much larger than Aicadia's structural
  core;
- Aicadia should express urban kinds and properties as entity plus claim rather than
  add a table for each CityGML class.

### GRASS GIS and PostGIS: geometry, attributes, topology and indexes are separate

GRASS stores vector geometry separately from attributes, topology and spatial or
category indexes. Shared boundaries can be topological objects rather than duplicated
polygon edges.

PostGIS stores standard geometry and uses a GiST spatial index. An indexed spatial
predicate first narrows candidates through bounding boxes and then performs the exact
geometry test.

Useful for Aicadia:

- geometry does not need to carry every semantic property;
- a road connection or shared boundary is not the same as geometric proximity;
- exact spatial predicates should operate on the current projection, not replay the
  entire scene log;
- PostGIS already supplies the conventional data structure needed for points, lines,
  polygons and nearby queries.

No separate graph database or world engine is required for these queries.

### QGIS and GeoServer: editor and delivery format are not source truth

QGIS loads vector layers through data providers such as GeoPackage and PostGIS. Its
editing buffer can commit or roll back changes to the underlying provider. QGIS does
not require one proprietary world-state model.

GeoServer states the corresponding map-delivery distinction explicitly: a vector
tile is an output format, not a data source.

Useful for Aicadia:

- the web map is a consumer of the same core API and projections;
- styling, clustering and tile clipping do not alter canon;
- changing the map renderer does not migrate world identity.

## How massive map delivery scales

### H3: useful derived index, unsuitable canon

H3 converts latitude and longitude into a hierarchical hexagonal cell ID. Parent and
child cell relationships make regional aggregation and neighbour searches cheap.
Logical containment in the index is exact, while geometric containment across
resolutions is approximate.

Possible later Aicadia use:

```text
current geometry -> calculated H3 cell -> candidate region query
```

Rejected use:

```text
H3 cell -> canonical district or fictional place identity
```

A boundary can cross many cells, and changing H3 resolution changes the index. It is
therefore a query aid, not a fictional fact.

### 3D Tiles: spatial tree and level of detail are delivery concerns

The OGC 3D Tiles standard supports quadtree and octree subdivision, sparse
availability, random tile access and hierarchical level of detail. A viewer requests
only the content needed for the current camera and screen-space error.

Possible later Aicadia use:

- serve a distant city as one simplified shape;
- load house and tree geometry only at local zoom;
- rebuild only the output tiles affected by a geometry change.

Rejected use:

- make the 3D tile own the entity;
- duplicate canon when an entity crosses a tile boundary;
- change entity identity when the level of detail changes.

## Direct answer: house, district and shared location

The systems produce three concrete answers.

### City-builder answer

```text
house -> exact tile
tile -> town ownership or influence
town size -> calculated from current simulated map
```

This is fast and deterministic, but it requires exact placement and an active
simulator.

### GIS answer

```text
house -> stable feature + exact footprint
district -> stable feature + exact boundary
house inside district -> spatial query, explicit relationship or both
```

This is precise and map-friendly, but it assumes surveyed geometry.

### Recommended Aicadia answer

When the accepted scene only establishes the courtyard:

```text
Mara's house
  containing_place: Red Courtyard
  exact_geometry: not established
```

When a later accepted scene maps it:

```text
Mara's house
  containing_place: Red Courtyard
  geometry: POLYGON(...)
  geometry_source: accepted scene
```

For the hierarchy:

```text
world
└── Alden
    └── Weaver District
        └── Red Courtyard
            ├── Mara's house
            ├── Iven's house
            └── shared well
```

`Red Courtyard` earns a `place` identity because characters can refer to it, travel
to it and query what is there. The server does not manufacture a named place for
every cluster of houses.

The house is spatially located in the courtyard. A kitchen is structurally part of
the house. Those are different claims.

## How a city grows in the three models

### In OpenTTD

The simulator adds a house tile or extends a road. Cached town state changes. The
visual and mechanical town becomes larger without a separately authored polygon.

### In a geographic database

A mapper edits the city boundary or adds building features. Both changes receive
their own feature versions and provenance. Adding a house does not necessarily move
the legal city boundary.

### In Aicadia

An accepted scene adding one house records:

```text
new house entity
house located in district
optional house geometry
```

It does not automatically record:

```text
district annexed surrounding land
city boundary expanded
population caused a larger radius
```

If the fiction establishes an expanded district boundary, that accepted package also
contains a new geometry claim superseding the previous district extent. A world map
may separately calculate a labelled built-area outline for display, but that result
is not the district and does not earn a core `place_coverage` table.

## Proposed source, projection and delivery separation

This is a research recommendation, not accepted schema.

### Accepted source

- immutable `scene`;
- stable `entity` and `place` identity;
- accepted location, geometry, composition and boundary claims;
- source scene, author and effective time;
- accepted `place_edge` changes for reachability.

### Current projection

- current containing place for a physical entity;
- one default containment path for fast ancestor and subtree queries;
- optional current PostGIS geometry and source claim;
- current place extent and source claim;
- current route graph;
- spatial index supplied by PostGIS.

Additional political, disputed or overlapping relationships remain claims. They do
not have to fit one tree.

### Delivery

- map viewport response;
- vector tile or 3D tile;
- simplified geometry for a requested zoom;
- optional H3 or other spatial-cell key if measured query load later earns it;
- LLM briefing containing place path and only relevant exact geometry.

Every delivery result is disposable and rebuildable.

## `5jaar`: what survived after five years

### Ordinary use

Most incidental houses, trees and rooms have a containing place but no invented
coordinate. Frequently visited or mapped objects have exact geometry. A district
briefing reads its indexed containment path; a map viewport reads PostGIS geometry;
a local navigation request also reads `place_edge`.

The complete world is never loaded for one agent. The API first selects one place
subtree or map window, then adds directly connected claims and recent scenes.

### Emergent use

- A district known socially before it has a mapped border gradually gains surveyed
  geometry.
- Two communities use different parent hierarchies for the same disputed market.
- A moving ship contains rooms whose global position follows the ship without
  rewriting every room.
- Old maps remain historically queryable because geometry claims retain their source
  scenes.
- Map renderers change tile format without changing any fictional identity.

### Failures removed

#### Copying a city-builder grid into canon

Agents were forced to invent coordinates for prose that only established a district.
Infrastructure cells leaked into briefings as fictional places. Cells were retained
only as indexes.

#### Running city growth ticks

A background simulator created roads and houses nobody authored and could not supply
accepted semantic claims. World change returned to explicit player or administrator
packages.

#### Treating polygon containment as the only relationship

A changed boundary silently moved houses between communities. Physical placement,
structural composition, governance and affiliation became separate claims.

#### Treating one hierarchy as all truth

Embassies, disputed borders, mobile communities and overlapping districts did not
fit. One default operational path survived; additional relationships remained in the
claim graph.

#### Making a map tile own an entity

Entities crossing tile edges were duplicated and changed identity between zoom
levels. Tiles became disposable delivery payloads.

#### Keeping a permanent calculated city hull

A remote house distorted the hull and a changed algorithm changed apparent history.
Any derived built-area shape became an explicitly labelled query or cache.

### Technical pressure that survived

- Postgres and PostGIS remained sufficient.
- Stable entity and place identity mattered more than coordinate format.
- Accepted geometry needed provenance and history.
- Place-path reads and exact geometry reads served different requests.
- Default containment needed an indexable projection.
- Route topology remained separate from geometric distance.
- Map output eventually needed tiling and geometry simplification.
- No spatial partition became a world entity.

### What did not survive

- exact geometry required on every physical entity;
- one universal city boundary derived from inhabitants or houses;
- a global simulation tick;
- an H3 cell, chunk or map tile in canon;
- all world relationships forced into one parent tree;
- a dedicated `place_coverage` world-state table;
- a separate graph database or game engine.

## Backcast to now

### Recommended choices for discussion

1. Keep stable world identity separate from geometry and spatial indexes.
2. Require a containing place for current physical placement, while allowing exact
   geometry to remain unknown.
3. Treat one parent as the default physical query path, not as every possible world
   relationship.
4. Store exact geometry only through accepted, provenance-carrying claims.
5. Keep route topology separate from coordinates and containment.
6. Keep tile, chunk, H3 and level-of-detail data outside canon.
7. Start with PostGIS indexes; add another spatial partition only after a measured
   query requires it.
8. Do not infer city annexation or forest expansion from a new child object.

### Smallest present experiment

Build one replay fixture:

1. Create Alden, Weaver District and Red Courtyard as stable places.
2. Add one house with only Red Courtyard as its containing place.
3. Prove that place briefing and ancestor queries work without a coordinate.
4. Add the house footprint in a later accepted scene.
5. Add a second accepted relationship that does not belong in the physical parent
   path.
6. Replace the district extent without rewriting the house source claim.
7. Query the house before and after the extent change.
8. Produce a disposable map response from the current projection.
9. Prove that deleting that map response loses no world-state.

Do not build H3, vector tile or 3D tile infrastructure for this experiment.

## Decision exposed by the research

Should Aicadia treat stable entity/place identity and accepted claims as world truth,
while every grid cell, chunk, hull and map tile remains a derived query or delivery
artifact?

Recommendation: yes. This preserves incomplete fictional geography, exact mapping
where earned, historical provenance and future scale without putting a game grid or
rendering format into canon.

## Sources

### Open-source games and world engines

- [OpenTTD tile data structure](https://docs.openttd.org/source/d6/dba/classTile)
- [OpenTTD town growth source](https://docs.openttd.org/source/d6/da0/town__cmd_8cpp)
- [OpenTTD town behavior](https://wiki.openttd.org/en/Manual/Towns)
- [OpenRCT2 map structure](https://github.com/OpenRCT2/OpenRCT2/wiki/Maps)
- [Luanti basic map structures](https://docs.luanti.org/for-engine-devs/basic-data-structures/)
- [Luanti database backends](https://docs.luanti.org/for-server-hosts/database-backends/)
- [Cataclysm: DDA map objects](https://docs.cataclysmdda.org/DEVELOPER_FAQ.html)
- [Cataclysm: DDA map generation](https://docs.cataclysmdda.org/JSON/MAPGEN.html)

### Geographic identity and geometry

- [OpenStreetMap element model and history](https://wiki.openstreetmap.org/wiki/Element)
- [OpenStreetMap boundary relation](https://wiki.openstreetmap.org/wiki/Relation%3Aboundary)
- [Overture division](https://docs.overturemaps.org/schema/reference/divisions/division/)
- [Overture division area](https://docs.overturemaps.org/schema/reference/divisions/division_area/)
- [Overture hierarchy](https://docs.overturemaps.org/schema/reference/divisions/types/hierarchy/)
- [Overture GERS](https://docs.overturemaps.org/gers/)
- [Overture building part](https://docs.overturemaps.org/schema/reference/buildings/building_part/)
- [OGC CityGML 3.0 conceptual model](https://docs.ogc.org/is/20-010/20-010.html)
- [GRASS vector data model](https://grass.osgeo.org/programming8/vectorlib.html)
- [GRASS topology](https://grass.osgeo.org/programming8/vlibTopology.html)
- [PostGIS spatial containment](https://postgis.net/docs/ST_Covers.html)
- [PostGIS spatial indexing](https://postgis.net/workshops/en/postgis-intro/indexing.html)

### Editing and delivery

- [QGIS vector editing buffer](https://docs.qgis.org/3.44/en/docs/pyqgis_developer_cookbook/vector.html)
- [GeoServer vector tile output](https://docs.geoserver.org/stable/en/user/extensions/vectortiles/)
- [H3 hierarchical indexing](https://h3geo.org/docs/highlights/indexing/)
- [OGC 3D Tiles implicit tiling](https://docs.ogc.org/cs/22-025r4/22-025r4.html)
