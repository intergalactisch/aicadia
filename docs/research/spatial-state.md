> **Era:** July scene-claim research; its scene/claim vocabulary predates the 2026-08-07 game reframe.

# Spatial state and discovery during travel

Date: 2026-07-26

Status: research and `5jaar` recommendation, not concept direction

Follow-up: [Mutable place geometry](mutable-place-geometry.md) refines this report's
single-geometry recommendation. A place may have several time-versioned authored
extents plus a separately calculated coverage; one point or polygon is insufficient.

Confirmed later: every current physical entity has one rebuildable
`entity_location` row containing at least a `place_id`, geometry or active
`place_edge_id`. No containing place or fixed geographic level is mandatory. The
travel and discovery details in this report remain research recommendations.

## Question

How should Aicadia store a character's position while travelling between two places,
and what exactly happens when that character discovers a new place such as a cave
halfway along the route?

The result must support:

- deterministic travel and map plotting without a server LLM;
- immutable history and replay;
- new places discovered at neither endpoint;
- millions of characters travelling concurrently;
- later routing to the discovered place; and
- the player's choice to stop or continue.

## Existing constraints

- `place_edge` connects two places and carries travel time.
- A character cannot be at the origin and destination at the same time.
- The accepted scene package is the immutable source of truth.
- Current character location and the map are rebuildable projections.
- A solo discovery has `verification_status = unverified` until another character
  independently encounters it.
- A new fact must connect to existing canon.
- The server may calculate and validate, but it may not invent semantic content.

## Concrete example

The route from Alden to Brin takes ten hours. Iven leaves Alden at 10:00 and is due in
Brin at 20:00.

At 14:12, 42% of the journey time has elapsed. Iven's current location is:

```text
place_edge_id: alden-brin
position: 0.42
```

`position` is always measured in the edge's stored direction:

- `0.0` = the edge's `from_place`;
- `1.0` = the edge's `to_place`.

If the edge has a map line, the server derives the map point at `0.42`. It does not ask
the agent for a longitude or latitude.

Iven confirms and submits the discovery package. After the server accepts it, the
package introduces:

```text
place: cave-mouth
verification_status: unverified
place_edge_id: alden-brin
position: 0.42
created_in_scene_id: discovery-scene
```

The prose and semantic claims—what the opening looks like, what Iven observed, and
what name they used—come from the agent. The route position comes from the server's
known transit state at the accepted scene time.

## Models considered

### Copy coordinates onto every record

The character, scene, cave, claims and route event would each store an `x/y` or
longitude/latitude.

Failure:

- the copies can disagree;
- moving a character creates repeated coordinate writes;
- historical queries must decide which copy was authoritative;
- abstract entities receive meaningless location fields.

This rejects duplicated coordinate columns, not a common spatial projection. A
single indexed point for each currently physical thing is a different model and is
examined below.

### Keep the character at the origin until arrival

Failure:

- a travel scene appears in the wrong place;
- the cave can only attach to an endpoint;
- nearby queries and encounters are false for the entire journey.

### Split the route every time something is discovered

Discovering the cave would replace Alden–Brin with Alden–Cave and Cave–Brin.

This is workable at small scale but poor as the source model:

- every landmark rewrites current route topology;
- journeys already using the old edge become difficult to interpret;
- a popular road can fragment into thousands of stored edges;
- replay must reconstruct which edge version each traveller used.

The current routing projection may still expose derived segments, but source history
should retain the original route plus the new anchored place.

### Store a position along the route

The route remains one versioned line. Characters, scenes and discovered places may
refer to it with a number from `0.0` to `1.0`.

This is the standard GIS pattern called **linear referencing**. PostGIS can interpolate
a point along a `LINESTRING` from a fractional position, locate a point's fraction on
a line and extract sections of a line.

Fit for Aicadia:

- one location value works for a traveller, a travel scene and a cave mouth;
- the server calculates it without semantic intelligence;
- the cave remains attached if the map is rendered at a different zoom;
- existing journeys keep referencing the same base edge;
- local map and nearby queries are indexable in Postgres/PostGIS.

## Reassessment: one world point plus source context

The earlier recommendation made `at_place(place_id)` and
`on_edge(place_edge_id, position)` the two public location forms. That preserves
source meaning, but it is not the best universal query form.

A point and bounded-place model is stronger for static spatial reads:

```text
place.geometry       geometry(Geometry, world_srid)
place_edge.geometry  geometry(LineString, world_srid)
current_position     geometry(Point, world_srid)
```

- A place with only an approximate location starts with a `Point`.
- A place with a known boundary uses a `Polygon` or `MultiPolygon`.
- A route uses a `LineString`.
- A character, physical object or spatial scene can be exposed as one `Point`.

PostGIS can use a spatial index for point-in-boundary queries through `ST_Covers`.
Its nearest-neighbour operator can return nearby indexed static geometries without
scanning the whole world. This is a better answer to "what static thing is near
`(x, y)`?" than first resolving a place id or route id.

There is one scale boundary: an exact point for a travelling character changes with
time. Keeping a spatial index exact would require recurring writes for every
traveller. With millions of concurrent journeys, that recreates a global movement
ticker. A travelling point must therefore be calculated from route plus time when it
is requested.

The technical type should be a PostGIS `geometry`, not two independent numeric
columns named `x` and `y`. The geometry carries its coordinate reference and lets the
database index and validate it. Because Aicadia is fictional, a planar world
coordinate system is a better initial fit than longitude/latitude on an Earth
ellipsoid.

### Why the point is not enough as source truth

The same point does not explain how the character got there or what movement remains.
At 14:12 these two states can share exactly the same point:

1. Iven stopped at the cave and is now located there.
2. Iven is passing the cave on the way to Brin and will arrive at 20:00.

A bare point also cannot answer:

- which accepted departure caused the movement;
- which route and direction are in use;
- whether arrival is still due;
- how to replay the position at 13:00;
- whether two close points are reachable without crossing a mountain;
- whether a character is inside a cave or on the surface above it.

Therefore the scalable model separates two jobs:

1. **Source context** records the accepted cause: place, route, departure time,
   arrival time and source scene.
2. **Spatial read result** exposes the calculated `Point` used by map and boundary
   queries.

This is not coordinate duplication. The source context is historical meaning; the
point is a rebuildable query value. Static points may be indexed in a projection;
continuously changing points are calculated.

### Concrete read result

At query time the API can return:

```text
character_id: iven
world_point: POINT(4182 905)
calculated_at: 14:12
place_id: null
place_edge_id: alden-brin
source_scene_id: departure-scene
```

After Iven stops at the cave:

```text
character_id: iven
world_point: POINT(4182 905)
calculated_at: 14:13
place_id: cave-mouth
place_edge_id: null
source_scene_id: discovery-scene
```

The identical point answers spatial queries. The mutually exclusive place or edge
reference explains the current state and supports replay.

### Nearby queries with millions of travellers

For stationary characters, places, objects and past scenes, the server uses their
indexed point or boundary directly.

For travelling characters, the query proceeds locally:

1. Use the spatial index on `place_edge.geometry` to find routes intersecting the
   requested area.
2. Use `place_edge_id`, `departed_at` and `arrives_at` indexes to find active journeys
   on those routes.
3. Calculate each candidate's point from route and time.
4. Keep only candidates whose calculated point is inside the requested distance.

This avoids updating a million character points every minute. The API still returns
the same `world_point` shape for stationary and travelling characters; only the
query plan differs.

### Place boundaries

When a place has a polygon boundary, the server can determine whether the current
point lies inside it:

```text
ST_Covers(place.geometry, current_position)
```

The result is a spatial candidate, not automatically a semantic claim. Boundaries can
overlap: a cottage can be inside a village, which is inside a valley. A cave directly
under a road exposes the limitation of two dimensions: both can cover the same
`(x, y)` while remaining different reachable places. The accepted `located-at` claim
and `place_edge` graph still determine semantic location and reachability.

Consequently, a new `location` table would duplicate the existing structural
`place`. The concrete model can give the `place` projection a geometry and reserve
the word `position` or `world_point` for a coordinate.

### Write authority

An agent may describe where something is relative to accepted canon, but an
untrusted agent-supplied point cannot by itself authorize movement or settlement.
Otherwise a player can teleport by submitting a distant point or create a city-sized
polygon in one scene.

For a discovery during travel, the server calculates the point from the accepted
route and current time. The accepted discovery package can retain that calculated
point as the discovered place's stable map anchor and retain the route reference as
provenance and connectivity. Redrawing the route later then does not move the cave.

## Recommended source and projection split

### Immutable source

The accepted departure package records:

```text
character_id
from_place_id
to_place_id
place_edge_id
departed_at
arrives_at
```

The accepted discovery package records the new place, its calculated world point,
`place_edge_id` and route position. No background process writes movement rows every
minute.

### Current location projection

At query time:

```text
now < departed_at
  => at_place(from_place_id)

departed_at <= now < arrives_at
  => on_edge(place_edge_id, calculated_position)

now >= arrives_at
  => at_place(to_place_id)
```

The simplest calculation uses elapsed-time fraction:

```text
journey_progress =
  (now - departed_at) /
  (arrives_at - departed_at)

position =
  journey_progress
  when travelling from the edge's from_place to its to_place

position =
  1 - journey_progress
  when travelling in the opposite direction
```

This assumes travel progresses uniformly along the stored route. A measured route may
later encode non-uniform travel time, but v1 does not need that until terrain produces
a real counterexample.

The result of this calculation is exposed as `current_position`, a PostGIS `Point`.
The place or edge reference remains attached as state context; consumers do not have
to choose between coordinate queries and route-aware queries.

### Map projection

A mapped route may have a `LINESTRING`. The plotted point is:

```text
ST_LineInterpolatePoint(route_geometry, position)
```

V1 can begin with a straight line between endpoint map points. Adding a curved line
later improves rendering without changing the traveller or discovery contract.

### Current route graph

Anchored places do not require destructive source-edge splits. For pathfinding, the
projection sorts current places on an edge by `position`:

```text
Alden (0.0)
→ cave mouth (0.42)
→ old bridge (0.71)
→ Brin (1.0)
```

It exposes adjacent derived segments and allocates the base travel time
proportionally. The base edge, accepted journeys and discovery history remain intact.

If a later discovered place has a real side path, that path becomes an ordinary new
`place_edge`. V1 can keep a cave mouth on the route point and make the cave interior a
child place that inherits the mouth's map point.

## The stop-or-continue choice

Discovery does not imply that the character stopped travelling. The final package must
state one of two player-authored results:

### Continue

- The cave mouth is created at the current edge position.
- The existing journey and arrival time remain unchanged.
- The character is no longer at the cave after the scene.

### Stop

- The journey ends at the discovery position.
- The character becomes `located-at` the new `unverified` place.
- Reaching Brin later requires a new accepted departure package.

The server must never choose between these. Otherwise discovering something would
silently author the character's next action.

## Witnessing and discovery status

The first scene creates an `unverified` place. Passing its route position later does
not automatically change it to `verified`:

- the server may include the nearby `unverified` place in the traveller's briefing;
- the traveller's agent and human may consciously witness or cite it;
- only that accepted gesture or scene supplies independent attention.

This prevents travel timers from creating unattended canon.

## `5jaar`: the world after five years

### Ordinary use

Long routes contain many player-discovered places: shrines, cave mouths, springs,
viewpoints and branch paths. A returning player can open the map, replay their old
journey and see exactly where each discovery occurred. Current travellers move along
the same route without recurring movement writes.

Agents ask spatially useful questions:

- What `unverified` places will I pass before arrival?
- Which travellers are near my current route position?
- How long from the east endpoint to this cave?
- Which scenes occurred between positions `0.3` and `0.5`?

The answer is deterministic because every result has a world point derived from
accepted place or route context and time.

### Failures discarded during those five years

- **Agent-authored raw coordinates:** different models placed connected locations in
  incompatible positions and occasionally teleported their owners.
- **Coordinates copied everywhere:** current and historical map views disagreed.
- **Endpoint-only travel:** every discovery clustered at towns and travel felt absent.
- **Source-edge splitting:** popular roads accumulated unstable topology and existing
  journey references became hard to replay.
- **Automatic witnessing by passers-by:** background timers promoted discoveries
  without any conscious player act.
- **Discovery always stops travel:** the system silently made a character choice.
- **Discovery never stops travel:** players could not meaningfully remain at something
  they had found.

### Operational pressure

The hot data is active journey context, not a stream of movement events. Position is
calculated on read or when a relevant scene arrives. Static geometries and route
lines are spatially indexed; a nearby-traveller query first selects local routes and
then calculates only their active travellers. Even with millions of travellers, the
server does no per-character heartbeat work.

## Backcast to now

### Recommended decisions for discussion

1. Every current physical position is returned as one PostGIS
   `geometry(Point, world_srid)`.
2. A `place` projection carries a point or boundary geometry; no separate `location`
   domain table is needed.
3. Static points are spatially indexed. Travelling points are calculated from indexed
   route and time context, so no movement ticker is needed.
4. The accepted source retains place or route context, time and provenance. A point
   does not replace these fields.
5. A travel scene uses the server-derived route point at acceptance time.
6. A newly discovered place stores the calculated world point plus its route
   connection; it does not accept an arbitrary point as proof of movement.
7. The discovery package explicitly chooses `continue` or `stop`.
8. The base route remains immutable; current pathfinding segments are a rebuildable
   projection.

These recommendations are not yet concept choices.

### Smallest experiment

Create one route and run these exact events:

1. Iven departs Alden for Brin.
2. Query at 42% elapsed time.
3. Iven discovers a cave and chooses `continue`.
4. A second traveller passes the cave but does not witness it.
5. Replay the world before and after the discovery.
6. Repeat with `stop`.

The experiment succeeds only if the current map, historical replay, arrival state and
verification status are deterministic without a movement ticker.

## Sources

- [PostGIS reference: linear referencing](https://postgis.net/docs/reference.html#Linear_Referencing)
- [PostGIS: ST_LineInterpolatePoint](https://postgis.net/docs/ST_LineInterpolatePoint.html)
- [PostGIS workshop: Linear Referencing](https://postgis.net/workshops/postgis-intro/linear_referencing.html)
- [PostGIS: ST_LineSubstring](https://postgis.net/docs/ST_LineSubstring.html)
- [PostGIS: geometry type for planar coordinate systems](https://postgis.net/docs/en/geometry.html)
- [PostGIS: indexed boundary test with ST_Covers](https://postgis.net/docs/ST_Covers.html)
- [PostGIS workshop: index-assisted nearest-neighbour search](https://postgis.net/workshops/postgis-intro/knn.html)
