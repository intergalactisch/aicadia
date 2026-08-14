> **Era:** July scene-claim research; its scene/claim vocabulary predates the 2026-08-07 game reframe.

# Mutable place geometry in a persistent world

> **Role / side:** sourced research report / development side.
> **Authority:** records this report's question, sources, findings and implications.
> **Excludes:** product decisions and current implementation contracts; see `docs/game/`.

Date: 2026-07-26

Status: research and `5jaar` recommendation, not concept direction

Follow-up: [Hierarchical spatial placement](archive/hierarchical-spatial-model.md) challenges
the dedicated `place_coverage` candidate. Its revised recommendation is one containing
place per physical entity, optional exact geometry, and no core coverage table. A
derived hull or union may exist later as a labelled map query or cache.

## Question

How can Aicadia create, change, query and understand places whose spatial extent
changes over time?

The model must handle:

- a settlement growing, shrinking or becoming denser;
- a forest, lake, island or road changing shape;
- several meaningful boundaries for the same place;
- a place splitting, merging, moving, being absorbed or ceasing to exist;
- exact current and historical map queries;
- simultaneous changes proposed by millions of connected agents;
- deterministic server validation without server-side LLM inference; and
- the append-only scene log remaining the source of truth.

## The correction to the previous spatial model

A `Point` is sufficient for a current position or approximate map anchor. It is not
the complete spatial state of a place.

Giving `place` one mutable `geometry` column would still fail:

- overwriting the polygon would erase the old city boundary;
- one polygon cannot simultaneously mean built-up area, city wall and claimed
  jurisdiction;
- deleting the row when a place disappears would break historical scenes and
  provenance;
- a population number cannot determine where new area appears; and
- a derived footprint and an authored world fact would become indistinguishable.

The place identity, its authored spatial statements and its calculated map coverage
must be separate.

## Concrete terminology used in this report

These are research terms, not accepted schema:

| Term | Exact meaning |
|---|---|
| `place` | Stable identity of a named or referable place |
| `world_point` | One exact point in the fictional world's coordinate system |
| `place_extent` | A time-bounded, provenance-carrying geometry that asserts one spatial meaning for a place |
| `place_coverage` | A rebuildable geometry calculated from current spatial children; not an authored claim |
| `place_edge` | A connection used for reachability and travel time |

`place_extent` and `place_coverage` deliberately differ. A city can have:

- a calculated coverage around accepted houses and roads;
- an authored wall boundary;
- an authored claimed jurisdiction;
- a district boundary used by its inhabitants; and
- a representative point used for a map label.

Those geometries may overlap without being equal.

## Why population cannot directly create a boundary

Suppose Alden has 400 inhabitants and later has 700.

The number does not say whether:

- existing houses became more crowded;
- taller or shared buildings appeared;
- houses were built north along the road;
- a harbour district appeared to the west;
- two nearby settlements became one; or
- the count is only an in-world estimate.

An algorithm such as `radius = population * constant` would silently invent both
shape and direction. It would also turn population into a mechanical counter, which
conflicts with Aicadia's no-score rule.

Spatial growth needs spatial causes. If accepted scenes add houses, paths and a
market outside Alden's previous footprint, a deterministic projection can expand
the calculated coverage around those accepted objects. If inhabitants increase
inside the same structures, the calculated coverage does not expand.

## What established systems do

### OpenStreetMap: stable identity, point and boundary are separate

OpenStreetMap has stable element ids, incrementing versions, changesets and full
element history. A deleted element retains its id and history; the current version
is marked not visible. The server rejects an edit based on a stale element version.

OpenStreetMap also distinguishes a settlement's place node from its area or boundary.
The representative node can remain at a commonly understood centre while the city
boundary changes. Complex boundaries use relations assembled from outer and inner
ways, including enclaves and disjoint parts.

Useful for Aicadia:

- a place id must outlive every shape version;
- approximate centre and spatial extent are different data;
- deletion from the current map must not erase history;
- concurrent editors need a base version;
- complex places require `MultiPolygon` and holes; and
- shared borders are possible, but reusable boundary-segment topology is more
  complexity than Aicadia v1 has earned.

Important difference: OpenStreetMap records observations about the external world.
Aicadia records authored fictional developments with scene provenance.

### GeoGig: commit whole spatial versions and retain old geometry

GeoGig applies version-control concepts to geospatial features. A geometry edit
creates a new feature version while the previous version remains available. A commit
groups related spatial edits, history is queryable and diffs can show changed
features or changed bounds.

Useful for Aicadia:

- an accepted scene already provides the correct commit boundary;
- store a complete accepted geometry version rather than an imperative sequence of
  vertex edits;
- link every geometry version to the source scene;
- compute map differences from two versions; and
- v1 does not need GeoGig, branches or distributed merge machinery. The same
  semantics fit the existing append-only Postgres model.

### CityGML 3.0: city objects have versions and effective time

The OGC CityGML 3.0 model supports multiple versions of city features and explicit
version transitions. It distinguishes slower changes such as construction and
demolition from high-frequency dynamic properties. Its time model distinguishes the
evolution of the represented city from the evolution of the database model.

Useful for Aicadia:

- a spatial feature has both world-effective time and chronicle acceptance time;
- construction, demolition and boundary change are version transitions;
- current and historical city models are first-class queries; and
- Aicadia should borrow the time distinction without adopting CityGML's extensive
  city ontology.

For Aicadia the two times already arise naturally:

```text
effective_at = when the change is true in the fictional world
accepted_at  = when the server accepted the source package
```

### PostGIS: geometry validation, spatial difference and current indexes

PostGIS supplies the mechanical operations Aicadia needs:

- `ST_IsValid` rejects malformed polygons;
- `ST_Covers` tests whether a point lies inside or on a boundary;
- `ST_Union` combines current spatial components;
- `ST_Difference` calculates added or removed area between versions;
- `ST_Area` measures a polygon for validation and operational inspection; and
- `ST_Subdivide` can split a very complex polygon into indexed projection pieces.

Useful for Aicadia:

- use one planar coordinate system and typed PostGIS geometry;
- validate every submitted geometry deterministically;
- build a GiST index over current geometry projections;
- calculate exact deltas instead of asking an LLM what changed; and
- impose a vertex and payload limit so a scene cannot submit a geometry bomb.

PostGIS supplies spatial operations, not temporal history. Aicadia's source log and
projector supply that history.

### MobilityDB: movement is geometry plus time, often constrained to a network

MobilityDB adds temporal point types and spatiotemporal indexes to PostgreSQL and
PostGIS. It also models temporal network points for objects that move along embedded
networks rather than freely through space.

Useful for Aicadia:

- route, position and time are a proven representation for travellers;
- changing place boundaries and moving characters are different workloads;
- a traveller's point should not require a recurring write;
- MobilityDB is research evidence, not a dependency recommendation. Plain route and
  time rows remain sufficient until actual query measurements prove otherwise.

### Unreal World Partition and Roblox streaming: partition reads, not meaning

Unreal World Partition stores one persistent level while dividing it into grid cells
that load near a streaming source. Roblox similarly streams nearby instances instead
of sending the complete world to every client.

Useful for Aicadia:

- no agent should receive the whole world;
- spatial cells or index pages can bound storage and briefing reads;
- a storage cell is not a fictional place; a city may cross many cells and several
  places may overlap one cell; and
- PostGIS indexing and local API queries should come before introducing an explicit
  world-partition service.

### SLEUTH urban growth: automatic growth always embeds a world model

SLEUTH is a cellular-automata urban-growth model. It uses spatial layers such as
existing urban area, roads, slope, excluded land and land cover, plus calibrated
transition rules, to predict new urban cells.

Useful for Aicadia:

- automatic city growth is not a neutral database operation;
- it requires assumptions about terrain, roads, policy, land use and growth
  behaviour;
- running such a model in the core server would ship an institution and make the
  server an author;
- an admin's consciously invoked agent may use a growth model as private analysis,
  but only an explicitly confirmed source package can alter canon.

## Recommended source model for discussion

### Stable place identity

The `place` row remains permanently addressable. Its identity is not its latest
polygon.

```text
place
  id
  entity_id
  created_in_scene_id
```

A map query may omit a place that no longer currently exists, but a historical scene,
claim or URL can still resolve its id.

### Geometry-valued claims

The core `claim` value vocabulary can include PostGIS geometry:

```text
claim
  id
  subject_id
  predicate_id
  object_geometry
  effective_at
  supersedes_claim_id
  scene_id
```

Examples of predicates the world might develop:

```text
built-area
enclosed-by-wall
claimed-jurisdiction
flooded-area
forest-cover
```

These examples are not a fixed enum. They are ordinary predicate rows, because the
world may invent spatial meanings not known today.

The source claim stores the complete geometry version. It never updates an earlier
claim. The new claim names the earlier claim it supersedes when they compete for the
same spatial meaning.

### Current and historical `place_extent` projection

A projector turns current geometry-valued claims into query rows:

```text
place_extent
  place_id
  predicate_id
  geometry
  effective_from
  effective_to
  source_claim_id
```

`effective_to` may be stored in this rebuildable projection even though no source
claim is mutated. Replay derives it from the next superseding claim.

Queries become deterministic:

```text
current extent:
  effective_from <= now
  and (effective_to is null or effective_to > now)

extent at time T:
  effective_from <= T
  and (effective_to is null or effective_to > T)
```

There may be several current rows for one place because different predicates answer
different questions.

### Calculated `place_coverage`

The server may calculate a non-canon coverage from current spatial children:

```text
place_coverage
  place_id
  geometry
  projected_through_scene_id
```

Concrete example:

1. Alden currently contains 18 spatially anchored buildings and three paths.
2. An accepted scene adds a house just beyond the northern coverage.
3. The projector updates only Alden and its containing places.
4. `place_coverage.geometry` expands around the new house.
5. No authored boundary claim is created.
6. A historical query before the house replays the smaller coverage.

The exact coverage algorithm remains a choice. Candidates include unioning building
footprints or buffering accepted points and paths before union. Whatever algorithm is
chosen must be deterministic, versioned in code and described as a projection. Its
output must never masquerade as something the inhabitants declared.

### Representative point

A place may retain one `world_point` used for approximate queries and map labelling.
It does not have to be the polygon centroid. OpenStreetMap's city-centre practice
shows why a cultural centre can remain useful when a city grows asymmetrically.

## Exact change cases

### Growth

Old built-area geometry:

```text
A
```

New accepted built-area geometry:

```text
B
```

The added area is:

```text
ST_Difference(B, A)
```

The new claim supersedes the old built-area claim. The old geometry remains
historically queryable.

### Shrink

The removed area is:

```text
ST_Difference(A, B)
```

The source package must describe and cite what caused the shrink. The server only
validates the geometry, time, provenance and protected world rules.

### Density increase without growth

New inhabitants occupy current structures. No geometry-valued claim changes and
`place_coverage` remains equal. This is why population cannot directly control
boundary size.

### Split

Alden becomes North Alden and Harbour Alden.

- The original `place` id remains historical.
- Two new place ids are introduced.
- Claims record the relationship from the old place to the new places.
- Each new place receives its own extent or calculated coverage.
- The server must not decide whether this is a split, rename or political change from
  polygon shape alone; the submitting agent supplies those claims.

### Merge or absorption

Two villages become one city.

- The accepted package states which identity, if any, continues.
- Old place ids remain resolvable.
- New containment and identity claims cite the old places.
- Geometry union alone does not decide cultural identity.

### Movement

A seasonal camp moves.

- A new geometry version may be disjoint from the old geometry.
- The place id continues only if the accepted package claims it is the same camp.
- The server validates travel and time but does not infer identity from proximity.

### Place ceases to exist

This is not a hard delete.

Examples differ:

- a lake dries up but the basin and old name remain;
- an island submerges;
- a village is abandoned and becomes ruins;
- a district is absorbed and loses its separate identity;
- a forest is completely cleared.

The source package records the concrete event and claims. Current projections stop
returning an applicable extent after its effective end. Historical queries continue
to return the place and its former geometry.

No single technical status should pretend these cases mean the same thing. Terms
such as abandoned, submerged, absorbed and cleared remain claims.

A minimal mechanical existence claim can close the current spatial state without
encoding the reason:

```text
subject_id: alden
predicate: exists
object_boolean: false
effective_at: <world time>
supersedes_claim_id: alden-exists-true
```

The projector ends Alden's current extents at that world time. The reason—abandoned,
submerged, absorbed or cleared—is supplied by separate ordinary claims in the same
source package. Existing child structures are not deleted.

If a later package establishes `exists = true` for the same place identity, the old
geometry does not silently reactivate. That package must also establish a new
applicable extent or new spatial children. A minimal `exists_at(time)` query can then
be derived without losing the historical place.

### Reappearance

If a settlement is rebuilt on old ruins, the human and agent must state whether this
is the same place returning or a new place at the old location. The server cannot
derive identity from coordinate equality.

## Write validation

The server can enforce the following without interpreting prose:

1. The geometry uses the one world coordinate system.
2. The geometry type is allowed for the predicate.
3. `ST_IsValid(geometry)` returns true.
4. Vertex count and encoded payload are below fixed operational limits.
5. The package names the current source claim it intends to supersede.
6. That source claim is still current; otherwise the package is stale and must be
   rebuilt against the new extent.
7. Referenced places, edges and supporting source claims exist.
8. The submitting character could plausibly observe or affect the cited area.
9. A normal scene cannot supersede a claim with
   `ordinary_scene_can_supersede = false`.

The server cannot determine whether prose describing three houses morally or
narratively justifies a city boundary. The agent must produce the correct geometry
and claims; a later challenge can contest semantic overreach.

For mechanically derived coverage, stronger checks are possible: new coverage can
only arise from accepted spatial children. An agent cannot directly write
`place_coverage`.

## Concurrent change

At intended scale, two accepted packages may attempt to change the same extent.

Each extent-changing package must name the `source_claim_id` it read:

```text
based_on_claim_id: alden-built-area-v17
```

If v18 was accepted first, the second package is rejected as stale. Its agent must
query v18 and produce a new complete package. This is the same optimistic concurrency
lesson used by OpenStreetMap element versions.

The server should not geometrically merge two independently authored boundaries.
That would create canon neither human confirmed.

Derived `place_coverage` has no author conflict. The projector deterministically
incorporates both accepted child changes in scene-log order.

## Query contract for agents

The agent needs semantic and spatial results together.

An `observe place` response at a requested time can include:

```text
place_id
name
exists_at
representative_world_point
current_extent [
  predicate,
  bounding_box,
  simplified_geometry,
  source_claim_id,
  source_scene_id,
  effective_from
]
calculated_coverage
contained_place
adjacent_place_edge
recent_spatial_change
```

The normal briefing should not contain full high-resolution polygons. It can return
bounding boxes, simplified geometry and named anchors. A deliberate map query can
return full GeoJSON.

Useful deterministic queries:

- which current extents cover this point?
- what covered this point at world time T?
- what area was added or removed between two accepted versions?
- which places ceased to have a current extent in this region?
- which source scenes caused this map change?
- which accepted objects currently determine this calculated coverage?

This is how an agent can understand the world without the server interpreting prose.
The server returns exact structured state and provenance; the consciously invoked
agent interprets it for its human.

## Scale model

### Current reads

Keep current `place_extent`, `place_coverage`, place points and route lines in
rebuildable PostGIS projections with GiST indexes. Local map queries touch only
geometries intersecting the requested bounding box.

### Historical reads

Index extent history by `(place_id, effective_from)` and retain
`source_claim_id`. A world-at-time query first restricts by time and bounding box.

### Change processing

A scene changes only directly affected spatial projections and their containment
ancestors. Adding one house to Alden does not recalculate every settlement.

Complex current polygons may be subdivided in a projection for faster point-in-area
queries. The source claim still retains the accepted complete geometry.

### Spatial partition

Grid cells may later partition indexes, caches or deployment work. They never become
canon and never define place identity. Start with one Postgres/PostGIS database until
measurements prove a need for physical partitioning.

## `5jaar`: Aicadia after five years

### Ordinary use

The first valley is now one small part of a large mapped world. Places have long
spatial biographies:

- Alden's original cluster of houses is visible on the year-one map;
- its northern road development appears in year two;
- the old wall still has its own authored geometry inside the larger built coverage;
- Harbour Alden is a named district with a disputed edge;
- a nearby village retains its own historical identity after being absorbed; and
- the dried lake is absent from the current water map but present in historical
  scenes, old travel routes and local stories.

An agent returning after a year asks for changed extents around its character. The
server returns exact geometry deltas and source scenes. The agent explains which
district grew, which path was cut off and which known place no longer currently
exists.

### Emergent culture

Old boundaries become story material:

- people say "inside the first wall" although the city now extends far beyond it;
- two neighbourhoods use different extents for the same district name;
- ruins remain socially important after their original settlement ceased;
- absorbed villages retain customs and place names; and
- routes follow vanished rivers whose historical geometries remain queryable.

The map does not merely illustrate the story. Its version history creates story.

### Abuse and failures encountered

#### Population-radius growth

Players created resident accounts or invented crowds to expand territory. Dense
vertical settlements were incorrectly drawn as large circles. Rivers, mountains and
other settlements were swallowed because a scalar population supplied no direction.
The mechanism was removed.

#### One boundary per place

The wall, built-up area and claimed jurisdiction repeatedly overwrote one another.
Agents could not answer what "inside Alden" meant. The model was replaced by
predicate-specific extents.

#### Agent-authored arbitrary polygons

Some agents submitted enormous or highly detailed polygons to seize attention,
surround other places or exhaust geometry operations. Geometry validity, size limits,
base-version checks and source support became mandatory.

#### Deleting vanished places

Historical scenes lost referents, old routes broke and map replay became impossible.
Current absence was separated from historical identity.

#### Canonising calculated coverage

When an implementation treated a buffered building union as an authored boundary,
players argued with an algorithm nobody in the world had established. Calculated
coverage was relabelled as projection and kept separate from spatial claims.

#### Global geometry rebuilds

Recalculating every region after every spatial scene produced growing write latency.
Projectors changed only affected places and containment ancestors.

#### Grid cells becoming world concepts

Early APIs exposed storage-cell ids. Agents began treating arbitrary cell edges as
real borders. Cell ids were removed from public world responses.

#### Full geometry in every briefing

Large GeoJSON payloads consumed context without improving narration. Briefings moved
to bounding boxes, simplified geometry and named change summaries; full geometry
became an explicit map query.

#### Surface-only coordinates

Caves, bridges and stacked interiors shared two-dimensional coordinates. The route
and containment graph remained necessary to distinguish reachable place from map
overlap. A full 3D model was not added merely to solve topology.

### Technical and operational pressure that survived

- Stable place identity separate from spatial version.
- Two times: world-effective and server-accepted.
- Geometry-valued claims with source provenance.
- Current PostGIS projections for fast local reads.
- Calculated coverage separate from authored extent.
- Optimistic concurrency for direct extent changes.
- Exact geometry validation and complexity limits.
- Local projection updates rather than global ticks.
- Historical map queries treated as a core product, not an audit afterthought.

### What was not needed

- A graph database.
- A geospatial microservice.
- Server-side LLM interpretation.
- Continuous population simulation.
- A cellular-automata growth engine in canon.
- A fixed square world ontology.
- GeoGig or CityGML as runtime dependencies.

Their useful semantics fit in the scene log, claims and PostGIS projections.

## Backcast to now

### Smallest decisions needed

1. A `place` id is permanent and distinct from all of its geometry versions.
2. Authored spatial meanings are geometry-valued claims, not one mutable place
   polygon.
3. Calculated physical coverage is a projection and never a canon claim.
4. Population does not directly mutate geometry.
5. Current absence never deletes historical identity.
6. Direct extent changes name the version they are based on.
7. Current and `as_of` map queries must work from the first spatial vertical slice.

These are recommendations for discussion, not accepted concept direction.

### Smallest experiment

Build one deterministic replay fixture:

1. Create Alden with one representative point.
2. Add three spatially anchored houses and derive initial coverage.
3. Add two houses along the northern road and derive expanded coverage.
4. Add inhabitants inside existing houses and prove coverage does not change.
5. Accept a wall-boundary claim different from calculated coverage.
6. Remove the northern houses through accepted events and derive smaller coverage.
7. End Alden's current settlement extent without deleting its place id.
8. Query current state and every prior step by world time.
9. Submit two concurrent wall edits based on the same source claim and prove only
   the first is accepted.
10. Replay from the empty database and compare every current geometry byte-for-byte.

The experiment succeeds only if the server uses no LLM, no movement or population
tick, no destructive history mutation and no global geometry rebuild.

## Revised decision exposed by follow-up research

The initial decision question distinguished:

1. calculated `place_coverage`, derived from accepted spatial objects; and
2. authored `place_extent`, stored as versioned geometry-valued claims?

The follow-up research found that a dedicated coverage concept does not yet earn its
place. Individual location claims, optional geometry and versioned `place_extent`
already store the world. A derived hull or union can be calculated for a map without
becoming source or required projection state.

Revised recommendation: keep versioned authored extents; do not add
`place_coverage` to the core model.

## Sources

- [OpenStreetMap element identity, version and history](https://wiki.openstreetmap.org/wiki/Element)
- [OpenStreetMap place node and area](https://wiki.openstreetmap.org/wiki/Key:place)
- [OpenStreetMap boundary relation](https://wiki.openstreetmap.org/wiki/Relation:boundary)
- [OpenStreetMap multipolygon relation](https://wiki.openstreetmap.org/wiki/Relation:multipolygon)
- [OpenStreetMap attic data and historical queries](https://wiki.openstreetmap.org/wiki/Attic_data)
- [GeoGig introduction and spatial version model](https://geogig.org/docs/start/introduction.html)
- [GeoGig geometry diff](https://geogig.org/manpages/diff.html)
- [OGC CityGML 3.0 conceptual model](https://docs.ogc.org/is/20-010/20-010.pdf)
- [OGC CityGML 3.0 versioning](https://docs.ogc.org/is/21-006r2/21-006r2.html#versioning)
- [PostGIS geometry reference](https://postgis.net/docs/reference.html)
- [PostGIS geometry validation](https://postgis.net/docs/ST_IsValid.html)
- [PostGIS spatial difference](https://postgis.net/docs/ST_Difference.html)
- [PostGIS spatial union](https://postgis.net/docs/ST_Union.html)
- [PostGIS geometry subdivision](https://postgis.net/docs/ST_Subdivide.html)
- [MobilityDB temporal types](https://docs.mobilitydb.com/MobilityDB/develop/ch03.html)
- [MobilityDB temporal network points](https://docs.mobilitydb.com/MobilityDB/develop/ch06.html)
- [MobilityDB temporal indexing](https://docs.mobilitydb.com/MobilityDB/master/ch05s17.html)
- [Unreal Engine World Partition](https://dev.epicgames.com/documentation/en-us/unreal-engine/world-partition-in-unreal-engine)
- [Roblox instance streaming](https://create.roblox.com/docs/workspace/streaming)
- [USGS: SLEUTH cellular urban-growth model](https://pubs.usgs.gov/publication/70033780)
