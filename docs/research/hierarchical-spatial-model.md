# Hierarchical spatial placement

Date: 2026-07-26

Status: research and `5jaar` recommendation, not concept direction

Follow-up: `stable-identity-and-sparse-location.md` challenges this report's
requirement that every physical entity have a containing place. The revised
recommendation is one current `entity_location` row containing at least a place ID,
geometry or active route. Exact geometry may therefore stand without a fabricated
parent place, and no district or other hierarchy level is required. This later
current-location contract is now confirmed concept direction.

## Question

Does every house, tree, forest and other physical entity need an exact world
coordinate, or can it be located through a containing place such as a district?

The answer must support:

- millions of agents introducing unpredictable physical entities;
- cities, districts, forests, buildings and rooms nested inside one another;
- exact maps where exactness matters;
- incomplete geography without fake precision;
- moving characters and changing place boundaries;
- current and historical queries;
- limited LLM context; and
- a dumb, deterministic server.

## Short answer

Every current physical entity needs a containing place. Exact geometry is optional.

Concrete examples:

```text
Alden
└── Weaver District
    └── Red Courtyard
        └── Mara's house
            └── kitchen
```

```text
North Forest
└── old lightning-struck oak
```

Mara's house can initially be known only as located in Red Courtyard. The old oak can
initially be known only as located in North Forest. Neither needs a fabricated point
at the parent's centre.

When exact position becomes relevant:

- the house may receive a `Polygon` footprint;
- the oak may receive a `Point`;
- the forest may receive a `MultiPolygon`;
- a road may receive a `LineString`; and
- a district may receive a versioned `Polygon` extent.

Containment answers "where in the world hierarchy?" Geometry answers "where exactly
in coordinate space?" They are related but not interchangeable.

## Terms used in this report

These are candidate technical terms, not accepted schema:

| Term | Exact meaning |
|---|---|
| `located-in` | A physical entity currently has this containing place |
| `part-of` | An entity is a structural component of another entity |
| `geometry` | Optional exact `Point`, `LineString`, `Polygon` or `MultiPolygon` |
| `place_extent` | A versioned geometry-valued claim about one place |
| `containment_path` | Rebuildable ordered path through containing places |
| `derived_extent` | Geometry calculated for a query or map view; not world-state |

`located-in` and `part-of` must not be treated as synonyms:

- a kitchen is `part-of` a house;
- a house is `located-in` a district;
- a tree is `located-in` a forest;
- a character is temporarily `located-in` a house;
- a district may be `part-of` a city if the world has established that structural
  relationship.

The exact predicate names remain a concept choice. The distinction is the finding.

## Models considered

### Exact global geometry for every physical entity

Every house, tree, room and object receives a world point or shape on creation.

Advantages:

- immediate map plotting;
- direct nearby and point-in-polygon queries;
- no spatial ambiguity.

Failures at intended scale:

- agents invent false precision when the fiction only establishes "somewhere in the
  northern district";
- every source package carries coordinate work unrelated to the scene;
- interiors and caves require stacked or local coordinate spaces;
- moving a ship or seasonal camp implies moving every contained object's global
  geometry;
- briefing payloads fill with coordinate data; and
- malformed or inconsistent geometry becomes a constant write-path concern.

This model is appropriate for a surveyed geographic database, not for every
statement in an emergent fictional world.

### Containing place only

Every entity has a parent place, but no entity has coordinates.

Advantages:

- simple hierarchy;
- compact agent context;
- easy place-based briefing and query scope.

Failures:

- no map;
- no deterministic distance within a large district;
- no exact boundary validation;
- no distinction between two houses in the same place;
- no spatial discovery along a route; and
- no way to calculate what lies near a border.

This model is useful as the minimum spatial truth, but insufficient as the complete
model.

### Containing place plus optional geometry

Every current physical entity has a containing place. Geometry is added only when an
accepted package establishes it.

Advantages:

- no entity is spatially orphaned;
- the world may remain approximate without becoming inconsistent;
- important or explored things become more precise over time;
- exact PostGIS queries work for entities that have geometry;
- coarse queries still work through the containing place; and
- changing a district boundary does not require rewriting every contained house.

This is the recommended model for discussion.

## What established systems do

### OpenStreetMap: explicit building geometry, higher area often derived

OpenStreetMap commonly gives a building its own outline. Address attributes may be
stored on the building polygon or a point inside it. Higher address fields such as
city can often be derived from the administrative boundary that covers the building
instead of being copied to every object.

OpenStreetMap also allows a building to start as a simplified group outline and later
be refined or split into individual properties.

Useful for Aicadia:

- one physical object may gain more detailed geometry later;
- do not copy every containing region onto every object;
- exact geometry is valuable when independently established; and
- OSM assumes surveyed absolute geometry, while Aicadia must also support
  intentionally unknown exact position.

### CityGML: semantic object and geometry detail are separate

CityGML models buildings, roads, vegetation, terrain, rooms and other city objects
semantically. The same object can have different geometric representations at
different levels of detail; its semantic identity does not depend on one geometry.
CityJSON likewise supports parent-child city objects and more than one geometry for
an object.

Useful for Aicadia:

- a house remains the same entity when a point becomes a footprint;
- semantic containment and spatial representation are separate concerns;
- different consumers do not need the same detail; and
- Aicadia should borrow progressive detail without adopting CityGML's fixed urban
  ontology.

### IndoorGML: containment, adjacency and connectivity matter more than coordinates

IndoorGML represents rooms and other cell spaces together with adjacency and
connectivity graphs. A door connects rooms for navigation even when their geometry
alone would not explain reachability.

Useful for Aicadia:

- a room can be understood through house membership and door connections;
- reachability is not the same as Euclidean proximity;
- caves, bridges and stacked interiors do not require full 3D geometry in v1; and
- `place_edge` remains necessary beside coordinates.

### OpenUSD and glTF: child objects may use a parent coordinate space

OpenUSD and glTF use parent-child scene hierarchies. Child transforms can be local to
their parent and composed into a world transform. This lets a car or model move
without rewriting the source geometry of every child component.

Useful for Aicadia:

- nested spatial context is a proven large-world pattern;
- a mobile container such as a ship may eventually justify a local coordinate
  system;
- adding transform matrices to every Aicadia place now would be speculative
  complexity; and
- v1 can use containment-only for interiors and global geometry for surveyed outdoor
  features.

### PostgreSQL `ltree`: hierarchy can be a query projection

PostgreSQL's `ltree` type stores and indexes tree paths. Aicadia can materialize one
operational containment path such as:

```text
world.alden.weaver_district.red_courtyard.mara_house
```

The accepted claims remain the source. The path is a rebuildable projection used for
fast subtree queries and briefings.

Useful for Aicadia:

- "everything inside Weaver District" does not require recursive graph traversal on
  every request;
- moving or reclassifying one place rebuilds only the affected subtree; and
- an `ltree` path is operational indexing, never canon text.

## Recommended source and projection split

### Source claims

A house with only coarse placement:

```text
entity:
  id: mara-house
  kind: house

claim:
  subject_id: mara-house
  predicate: located-in
  object_id: red-courtyard
  effective_at: <world time>
  scene_id: <source scene>
```

The source does not invent coordinates.

Later, an accepted surveying, building or discovery scene may add:

```text
claim:
  subject_id: mara-house
  predicate: occupies
  object_geometry: POLYGON(...)
  effective_at: <world time>
  scene_id: <source scene>
```

The exact predicate vocabulary is open. The structural requirement is that the
geometry remains an immutable, provenance-carrying claim.

### Current location projection

A rebuildable projection can expose:

```text
entity_location
  entity_id
  parent_place_id
  geometry
  source_location_claim_id
  source_geometry_claim_id
```

`geometry` may be null. Null means exact coordinate geometry is not established; it
does not mean the entity is nowhere.

The API can derive precision without storing a vague status:

```text
parent_place_id present, geometry absent  => place-level location
Point present                             => point location
LineString present                        => linear geometry
Polygon or MultiPolygon present           => area geometry
```

### Containment path projection

For place-scoped reads:

```text
place_containment
  place_id
  parent_place_id
  containment_path
  source_claim_id
```

The table name is singular. The path can be implemented with `ltree`.

Follow-up comparison with Overture Maps shows why this must be an operational
default path rather than the complete world truth. A physical place can occur in
more than one political or disputed hierarchy. Additional spatial, political and
social relationships remain accepted claims; they do not all become parents in this
projection. See `open-spatial-world-system.md`.

### No required `place_coverage`

`place_coverage` does not earn a core place in this model.

If a map needs an approximate outline around all houses in a district, it may
calculate a `derived_extent` from current child geometries. That result is:

- a query result or rebuildable map cache;
- labelled with the exact derivation method;
- never an authored claim;
- never used to decide that a city annexed land; and
- not required before a measured performance or product need exists.

This applies Terry's "Earn Your Spot": individual geometry plus containment already
stores the world truth. A permanent coverage projection would duplicate it before a
consumer has proved the need.

## Concrete examples

### House inside a district

Minimum accepted state:

```text
mara-house located-in weaver-district
```

This is enough to:

- include the house in a district briefing;
- let a character already in the district visit it through a local scene;
- query its district, city and region ancestors; and
- preserve provenance.

It is not enough to:

- calculate metres from the district gate;
- determine which side of a river it is on;
- draw a building footprint; or
- reject another footprint overlapping it.

Those functions require later geometry.

### Several houses at one shared location

If the houses share a meaningful courtyard, hamlet, block or unnamed site, that site
may be a `place`:

```text
weaver-district
└── red-courtyard
    ├── mara-house
    ├── iven-house
    └── shared-well
```

The containing place earns an entity only when the world refers to it or queries need
it. The server must not manufacture a place row for every geometric cluster.

### Special tree inside a forest

Minimum:

```text
lightning-oak located-in north-forest
```

More precise:

```text
lightning-oak occupies POINT(...)
north-forest occupies MULTIPOLYGON(...)
```

PostGIS may verify that the forest geometry covers the tree point. The semantic
`located-in` claim remains useful because a changing or disputed forest boundary
should not silently rewrite the tree's history.

### Forest growth

The forest's accepted geometry claim receives a new version. Individual trees do not
need new locations merely because the outer forest boundary changed.

A new tree just beyond the old boundary does not automatically expand the forest. It
may be a lone tree. The submitting agent or a later world-steward package must state
and support that the forest itself expanded.

### District growth

An accepted package may:

1. add a new house with exact or district-level placement;
2. establish that it belongs to the district; and
3. if authorised and supported, submit a new district extent that supersedes the old
   extent.

The server validates both geometries and sources atomically. It does not infer
annexation from the house alone.

### Place with no boundary

A place may legitimately have only:

- a representative point;
- a containing place;
- connected `place_edge` rows; or
- no exact geometry yet.

This does not make the world invalid. It states the actual precision of current canon.

## Boundary and containment consistency

When both parent extent and child geometry exist, the server can calculate:

```text
ST_Covers(parent_extent, child_geometry)
```

The result has three possible uses:

1. **Validation** when a predicate explicitly requires physical containment.
2. **Warning** when an authored semantic membership and current geometry disagree.
3. **No action** when overlapping or disputed boundaries are allowed.

The server must not globally assume every `located-in` claim requires polygon
coverage. A houseboat can belong to a harbour community while outside a formal city
boundary; an embassy may be politically related to one place while physically inside
another.

The predicate definition or applicable rule must state which behavior applies.

## Map behavior

At distant zoom:

- return region, city and district place geometry;
- omit individual houses and trees.

At local zoom:

- return exact child geometries where available;
- group children with only parent-level location under that place;
- label them as exact position not established; and
- never plot all unknown children at the parent's centre as if they shared a point.

For an LLM briefing:

```text
Mara's house
  located_in: Red Courtyard > Weaver District > Alden
  exact_geometry: not established
  reachable_from: courtyard path
```

This is more useful and cheaper than raw coordinates.

## Movement and mobile containers

### Character

A character has a current containing place or active journey. Exact position is
derived only when needed.

### Ship or seasonal camp

If a ship contains rooms and objects, rewriting every child's global position while
the ship moves is wasteful.

V1 can answer that each child is located in the ship and derive the ship's world
position. A local coordinate system becomes justified only if scenes need exact
within-ship distances or geometry. OpenUSD and glTF show how parent-relative
transforms could solve that later, but they do not justify shipping transform
hierarchies now.

## Concurrency

Two players may add houses to the same district concurrently:

- both source packages append independently;
- both location claims can be accepted if mechanically valid;
- the containment projection processes them in scene-log order; and
- neither package rewrites the district row.

Two packages that directly replace the same district extent must name the extent
claim they read. Only the first remains current; the second is rejected as stale and
must be rebuilt.

This keeps high-volume child creation independent while serialising the rarer direct
boundary edit.

## `5jaar`: Aicadia after five years

### Ordinary use

The world contains millions of houses, trees, rooms, paths, wells and named objects.
Most have a precise containing place. Only the spatial entities that have been
surveyed, travelled around, mapped or mechanically tested carry exact geometry.

A normal agent query for one character returns:

```text
world
> western-valley
> alden
> weaver-district
> red-courtyard
> mara-house
> kitchen
```

The query also returns nearby exact map features where relevant. It does not load
every coordinate in Alden.

Cities grow by adding and changing districts, paths, structures and versioned city
extents. Forests expand through new accepted forest extents. Unmapped interiors remain
topological place graphs until exact geometry matters.

### Emergent culture

Spatial precision becomes part of discovery:

- everyone knows a hermit lives somewhere in North Forest, but no accepted scene has
  fixed the hut's point;
- a district has a commonly used extent but no formal boundary;
- a newly surveyed path makes old travel descriptions more exact without invalidating
  them;
- a house first known only by courtyard later gains rooms and entrances; and
- maps openly distinguish accepted exact geometry from place-level knowledge.

Unknown exact position is usable world-state, not missing database work.

### Failures removed during five years

#### Exact coordinates required for everything

Agents generated plausible-looking but mutually inconsistent geometry for incidental
objects. Scene packages became large and brittle. Exact geometry became optional.

#### Containment only

The map, travel and border cases remained vague forever. Important entities gained
versioned geometry without forcing that cost onto every entity.

#### `place_coverage` as core world-state

A union or hull around child points was mistaken for the district itself. One remote
house stretched an outline across empty land; a different algorithm produced a
different "truth". Derived extents were reduced to labelled map/query results.

#### One relation named `part-of`

Rooms, political membership, physical containment and temporary character location
became indistinguishable. Composition and current location received separate
predicates.

#### Fake inherited points

Thousands of houses without exact geometry were plotted on their district's centre.
Agents treated those coincident points as real. Unknown child geometry stopped
producing fake coordinates.

#### Updating every child when a parent changed

Boundary growth and mobile containers caused write amplification. Child placement
remained relative through containment; exact global geometry was not copied from the
parent.

#### Storage cells exposed as places

Infrastructure partitions leaked into canon. Public queries returned fictional place
paths, never storage-cell ids.

#### Full hierarchy traversal for every briefing

Recursive graph reads became a hotspot. A rebuildable indexed `containment_path`
projection made subtree and ancestor reads conventional Postgres queries.

### Abuse

- Players tried to place a house in a famous district without plausible travel or
  source support.
- Agents submitted exact polygons overlapping existing houses.
- A direct district boundary edit attempted to absorb many unrelated places.
- Deep chains of meaningless container places attempted to inflate context.
- A high-detail geometry payload attempted to exhaust validation.

Surviving controls:

- location and geometry claims cite accepted canon;
- travel and connectivity remain deterministic;
- exact geometry is validated and size-limited;
- containment depth is operationally bounded;
- new container places must earn a name or recurring world reference;
- current direct extent edits use optimistic concurrency; and
- semantic overreach remains challengeable after acceptance.

### Technical pressure that survived

- One current containing place per physical entity for operational queries.
- Optional, versioned geometry for exact spatial facts.
- Separate composition and location relations.
- `ltree` containment projection.
- PostGIS geometry projection and index.
- Route graph for reachability.
- Progressive spatial detail.
- Current and historical queries.
- Full geometry only on deliberate map requests.

### What did not survive

- A required point for every entity.
- A dedicated `place_coverage` world-state concept.
- A fixed district or address schema.
- Parent-relative transform matrices for every place.
- A 3D world engine.
- Server-generated semantic places.

## Backcast to now

### Recommended decisions for discussion

1. Every current physical entity has one operational containing place.
2. Exact geometry is optional and provenance-carrying.
3. A place may itself be contained by another place.
4. Composition and spatial location are different relations.
5. Direct place boundaries are versioned geometry claims.
6. A derived hull or union is a labelled query result or cache, not canon and not a
   required core table.
7. The containment path is a rebuildable projection.
8. Route connectivity remains separate from geometry.

These recommendations are not yet concept direction.

### Smallest experiment

Create and replay this exact fixture:

1. Create Alden and Weaver District with versioned extents.
2. Create Red Courtyard inside Weaver District.
3. Add Mara's house with only `located-in = Red Courtyard`.
4. Query the house's full ancestor path.
5. Add a polygon footprint for the same house in a later scene.
6. Add a tree with only `located-in = North Forest`.
7. Add an exact point for the tree later.
8. Expand the forest extent without changing the tree claim.
9. Add a house whose geometry falls outside its stated district and exercise the
   predicate-specific validation response.
10. Query the world before and after every precision and boundary change.

Success means:

- no fake point is created;
- the identity remains stable as precision increases;
- every current and historical query is replayable;
- a parent boundary change does not rewrite every child; and
- no `place_coverage` table is needed.

## Decision exposed by the research

Should every current physical entity have one containing place while exact geometry
remains optional?

Recommendation: yes. It provides complete coarse placement, progressive exactness and
scalable hierarchy without inventing coordinates or a second coverage truth.

## Sources

- [OpenStreetMap building model](https://wiki.openstreetmap.org/wiki/Buildings)
- [OpenStreetMap building relation](https://wiki.openstreetmap.org/wiki/Tag:type%3Dbuilding)
- [OpenStreetMap address placement](https://wiki.openstreetmap.org/wiki/Key:addr:*)
- [OGC CityGML 3.0 conceptual model](https://docs.ogc.org/is/20-010/20-010.html)
- [OGC CityGML 3.0 user guide](https://docs.ogc.org/guides/20-066.html)
- [OGC CityJSON standard](https://docs.ogc.org/cs/20-072r2/20-072r2.html)
- [OGC IndoorGML 2.0 conceptual model](https://docs.ogc.org/is/22-045r5/22-045r5.html)
- [OpenUSD geometry and transform hierarchy](https://openusd.org/release/api/usd_geom_page_front.html)
- [glTF node hierarchy and local transforms](https://registry.khronos.org/glTF/specs/2.0/glTF-2.0.html)
- [PostgreSQL `ltree`](https://www.postgresql.org/docs/17/ltree.html)
- [PostGIS spatial containment](https://postgis.net/docs/ST_Covers.html)
- [PostGIS derived concave hull](https://postgis.net/docs/ST_ConcaveHull.html)
