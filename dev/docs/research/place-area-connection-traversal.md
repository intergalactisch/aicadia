---
status: pending
era: August 2026 spatial foundation grill
---

# Place extent, spatial inclusion and connection traversal

> **Role / side:** sourced spatial-model research report / development side.
> **Authority:** records this report's question, primary-source findings, inferences
> and unresolved implications.
> **Excludes:** product choices, negotiated Aicadia vocabulary and current
> implementation contracts; those belong in `dev/docs/concept/`, `dev/CONTEXT.md`
> and `game/docs/` respectively.

Controlled: 2026-08-19

Standing: research only; no candidate structure or name below is accepted Aicadia
behavior, vocabulary, schema or API.

## Question

Which separate spatial facts describe:

1. the extent or shape associated with a Place;
2. whether Places include, touch or overlap one another; and
3. the ordered landscape traversed along one direct Connection?

Can Connection endpoints answer the third question alone? If not, how do an
explicit path line or corridor with derived area intersections and explicitly
authored traversal intervals compare under mutable boundaries, overlap, portals,
concurrency and million-player reads?

## Evidence language

- **External fact** is directly supported by an official specification,
  first-party schema or project-owned source.
- **Inference** translates those facts into a constraint on this design question;
  it is not a product choice.
- **Illustration** is concrete but non-canonical and proposes no schema or API.
- **Open decision** is a choice the evidence cannot make for Aicadia.

## Short answer: three distinct truths

| Question | Minimum truth that can answer it | What it cannot answer alone |
| --- | --- | --- |
| What space does this Place cover? | A geometry in a declared coordinate frame and dimension | Parentage, traversal permission or one primary Place among overlaps |
| Which Places include or overlap? | An explicit hierarchy/relation, or a named predicate over exact geometry revisions | The order in which a traveller encounters them |
| What lies along this Connection? | An oriented path/corridor plus an intersection rule, or authored ordered intervals | Endpoint IDs contain no intervening course or distance |

**Inference.** A directed endpoint fact `A -> B` proves at most that an accepted
topology permits a direct transition. Infinitely many physical paths—and a portal
with no intervening physical path—share those endpoints. Endpoint topology therefore
cannot determine whether a traveller crosses a forest, heath, river or dunes.

**Inference.** A geometry-derived answer is not a unique list of areas. It is an
ordered multiset of intervals: one area can be entered repeatedly, several areas can
apply over the same interval, and a boundary touch may or may not count.

## Primary-source findings

### Identity, representative point, extent and hierarchy are separable

**External fact.** Overture represents a recognized `division` with a Point, a
separate `division_area` with Polygon or MultiPolygon geometry referencing that
division's ID, and a separate shared `division_boundary` line. It distinguishes
land-clipped and territorial variants of the same division's area. [Overture
Divisions guide](https://docs.overturemaps.org/guides/divisions/), [Overture
DivisionArea schema](https://docs.overturemaps.org/schema/reference/divisions/division_area/)

**External fact.** Overture stores administrative hierarchies on division identity,
separate from area geometry, and permits multiple hierarchies for different
perspectives. [Overture Division
schema](https://docs.overturemaps.org/schema/reference/divisions/division/),
[Overture Hierarchy
schema](https://docs.overturemaps.org/schema/reference/divisions/types/hierarchy/)

**Inference.** A Place can retain one stable identity and Position without claiming
a boundary. Adding or changing an extent need not replace that identity. One Place
may also have several purpose-specific extents: for example, inhabited footprint,
walls and jurisdiction are not automatically the same shape.

**Inference.** “C falls in B” can mean an authored hierarchy or current geometric
coverage. Those can legitimately differ. A portal room can belong to a castle while
being geometrically elsewhere; a forest can cross two jurisdictions.

### Inclusion, touching and overlap require an explicit predicate

**External fact.** OGC Simple Features distinguishes Point, LineString, Polygon and
multi-geometries and defines `Intersects`, `Touches`, `Crosses`, `Within`, `Contains`
and `Overlaps` as different topological predicates. GeoSPARQL supports both asserted
topological relations and geometry-derived query functions. [OGC Simple Feature
Access 1.2.1](https://docs.ogc.org/is/06-103r4/06-103r4.pdf), [OGC GeoSPARQL 1.1,
sections 7 and 9](https://docs.ogc.org/is/22-047r1/22-047r1.html)

**External fact.** PostGIS `ST_Contains` excludes a candidate that lies only on the
container boundary, `ST_Covers` includes the boundary, and `ST_Intersects` is true
for any shared point. [`ST_Contains`](https://postgis.net/docs/ST_Contains.html),
[`ST_Covers`](https://postgis.net/docs/ST_Covers.html),
[`ST_Intersects`](https://postgis.net/docs/ST_Intersects.html)

**Inference.** Geometry does not create a Place tree. Nested and overlapping shapes
produce many-to-many matches; a point on a shared edge can be covered by both
adjacent polygons. Choosing one current or primary Place requires a separate game
rule.

**External fact.** PostGIS polygon validity is deliberately checked, not guaranteed
on arbitrary load. Its standard validity and many topology operations remain 2D
even when coordinates contain Z. [PostGIS geometry
validation](https://postgis.net/docs/using_postgis_dbmanagement.html#OGC_Validity),
[`ST_IsValid`](https://postgis.net/docs/ST_IsValid.html)

**Inference.** A bridge above a river and a cave below a forest can overlap in 2D
without sharing traversable space. Existing Z coordinates do not automatically make
2D area predicates truthful for layered or volumetric gameplay.

### Routable topology and travelled shape are separate in mapping and games

**External fact.** Overture transportation segments carry an oriented LineString
centreline, while connector references identify routing decision points and their
normalized positions along that line. Together, connectors and segments provide
topology and shape rather than treating either as sufficient alone. [Overture
segments and connectors](https://docs.overturemaps.org/guides/transportation/segments-and-connectors/),
[Overture Segment
schema](https://docs.overturemaps.org/schema/reference/transportation/segment/)

**External fact.** Overture linear references run from `0.0` at the oriented start
to `1.0` at the end. A property can apply at a point or over a `between: [a, b]`
range without splitting the segment. It warns that self-crossing or near-self-
crossing lines can make closest-point location ambiguous. [Overture linear
referencing](https://docs.overturemaps.org/guides/transportation/linear-referencing/),
[Overture scoping
rules](https://docs.overturemaps.org/guides/transportation/scoping-and-travel-modes/)

**External fact.** Detour pathfinding uses an ordered corridor of navigation-mesh
polygons and can return a vertex whenever the polygon area ID changes. It separately
represents an off-mesh connection using two endpoints and a direction flag. [Detour
query interface](https://github.com/recastnavigation/recastnavigation/blob/main/Detour/Include/DetourNavMeshQuery.h),
[Detour navigation
structures](https://github.com/recastnavigation/recastnavigation/blob/main/Detour/Include/DetourNavMesh.h)

**External fact.** Unreal Navigation Link Proxies connect navigation areas even when
no navigation data exists between them. They can be one-way and support jumps,
drops or separated meshes; visible bridge geometry is not required for the link.
[Epic navigation-mesh modification
guide](https://dev.epicgames.com/documentation/en-us/unreal-engine/overview-of-how-to-modify-the-navigation-mesh-in-unreal-engine)

**Inference.** Drawing a straight line between endpoint Positions silently invents
a course and is false for winding roads, ferries, interiors and portals. A portal's
Connection can truthfully express reachability while having no ordinary landscape
between its endpoints.

## Two candidate models for traversal context

The labels below describe research alternatives only.

### Candidate 1: explicit course, current context derived from extents

**Player/World consequence.** An Agent supplies the actual oriented bends of an
ordinary direct journey. World can mechanically return, for the geometry revisions
read, that the traveller first crosses heath, then forest, then dunes. Moving the
forest boundary changes later current answers without editing the course.

**Technical shape.** A LineString is the centre course. A spatially indexed
`ST_Intersects` join finds candidate area geometries; `ST_Intersection` clips shared
portions; linear referencing orders the resulting components. `ST_Dump` is needed
because one disconnected or holed area can yield several components. [PostGIS
`ST_Intersection`](https://postgis.net/docs/ST_Intersection.html),
[`ST_LineLocatePoint`](https://postgis.net/docs/ST_LineLocatePoint.html),
[`ST_Dump`](https://postgis.net/docs/ST_Dump.html)

**Illustration — incomplete, non-canonical SQL data flow:**

```sql
candidate := area rows where ST_Intersects(area.geometry, course.geometry);
piece     := each line component of ST_Intersection(area.geometry, course.geometry);
start_at  := location of piece start along the oriented course;
end_at    := location of piece end along the oriented course;
result    := order by least(start_at, end_at), stable_tie_breaker;
```

**Corridor variant.** A Polygon corridor can be authored or created by buffering the
centreline. It answers which areas touch the traversable width or exposure envelope,
not the exact line a Character walked. PostGIS `ST_Buffer` creates a 2D result and
ignores Z; Overture similarly keeps centreline and width rules separate. [PostGIS
`ST_Buffer`](https://postgis.net/docs/ST_Buffer.html), [Overture
WidthRule](https://docs.overturemaps.org/schema/reference/transportation/types/width_rule/)

**Failure boundaries.** A corner touch may yield only a Point and need not mean
“walks through.” Holes cause repeat entry; overlap causes simultaneous intervals.
Missing, invalid, differently framed or dimensionally incompatible geometry cannot
produce a deterministic answer. A continuous world-space course is untruthful for
a portal.

### Candidate 2: explicitly authored ordered intervals

**Player/World consequence.** An Agent can state the ordered journey even before the
world has complete area polygons, and can describe a discontinuous or non-Euclidean
transition.

**Illustration — non-canonical:**

```text
Connection A -> B
  [0.00, 0.20): Open Heath
  [0.20, 0.75): Whispering Forest
  [0.75, 1.00]: Dunes
```

**Technical shape.** Normalized intervals follow Overture's established linear-
referencing pattern. A bounded B-tree read can page by an illustrative key such as
`(connection_id, start_fraction, stable_tie_breaker)`. The interval set needs
revision/lifecycle rules, and gaps and overlapping intervals need explicit meaning.

**Failure boundaries.** When both intervals and geometry claim the physical
landscape, they can disagree. A boundary edit does not automatically move authored
intervals. Free prose alone cannot be ordered or validated by World; the Agent must
supply bounded structured ranges. Concurrent writers changing the same popular
Connection still conflict on that Connection's interval-set revision.

### Comparison

| Basis | Boundary change | Overlap and repeat entry | Portal | Read and write pressure |
| --- | --- | --- | --- | --- |
| Endpoints only | No traversal answer exists | Invisible | Expresses reachability, not intervening context | One cheap record; insufficient answer |
| Course plus derived intersections | Current result changes with the exact extent revisions read | Naturally returns concurrent and repeated intervals | No truthful continuous course | Spatial join on read; materializing results creates boundary-change fan-out |
| Corridor plus derived intersections | Same revision dependency | More matches because width counts | Still cannot represent discontinuity | Larger candidate shape; answers contact, not exact footsteps |
| Authored intervals | Stable until explicitly edited; may disagree with later geometry | Must deliberately allow or reject both | Can express no-space or authored stages | Cheap ordered pages; authorship and same-Connection write conflict remain |

Neither candidate is selected here. Combining them also requires a deterministic
authority rule; World cannot read prose to decide whether authored or derived facts
win.

## Mutable boundaries, history and concurrency

**External fact.** Overture increments a feature version when geometry or attributes
change. Its linear referencing avoids splitting a road at every partial property
change. [Overture Divisions data
dictionary](https://docs.overturemaps.org/guides/divisions/#data-dictionary),
[Overture linear
referencing](https://docs.overturemaps.org/guides/transportation/linear-referencing/)

**Inference.** “What would I cross now?” differs from “what did I cross during the
accepted journey?” Recomputing old travel against today's forest boundary rewrites
history. A historical answer therefore needs the accepted interval result or the
exact course and extent revisions used at acceptance. This does not decide how much
history Aicadia stores.

**External fact.** PostgreSQL MVCC gives each statement a consistent snapshot;
ordinary reads do not block writes, and row locks block writers/lockers of the same
rows rather than ordinary readers. [PostgreSQL
MVCC](https://www.postgresql.org/docs/current/mvcc-intro.html), [row-level
locking](https://www.postgresql.org/docs/current/explicit-locking.html#LOCKING-ROWS)

**Inference.** Millions of Characters may read one immutable Connection revision
without locking it. A state-changing action based on a previous read must recheck
the exact course, extent or interval-set revisions it depended on. No global map
lock or revision is required; one would become a shared write hotspot.

**Inference.** Materializing all course/area intersections changes read cost into
write fan-out: moving one large forest boundary may invalidate thousands of
Connections. Deriving on demand avoids those writes but pays the spatial join.
Caches are correct only for the exact input revisions they represent.

## Bounded reads at scale

**External fact.** PostGIS index-aware predicates use a spatial index for a bounding-
box prefilter and then perform the exact test. Large shapes have expensive, broad
bounding boxes; PostGIS recommends `ST_Subdivide` so smaller indexed pieces retain a
reference to the source feature. [PostGIS spatial
indexes](https://postgis.net/documentation/faq/spatial-indexes/), [large-shape
performance](https://postgis.net/documentation/faq/big-objects-performance/),
[`ST_Subdivide`](https://postgis.net/docs/ST_Subdivide.html)

**Inference.** An index bounds candidate search, not result cardinality. One path can
cross millions of tiny areas, and millions of overlapping areas can cover one point.
A read must page by path position and a stable tie-breaker, constrain accepted input
complexity, or deliberately select a smaller relevant class. Silently truncating
matches would make the spatial answer false.

**Inference.** `(connection_id, area_id)` is not a sufficient cursor because the same
area may be entered more than once. Mutable boundaries also mean page two can skip,
repeat or reorder results relative to page one. A cursor must bind to a stable
derived revision/snapshot, report staleness, or promise only a fresh independent
page; that is a product semantic, not just pagination syntax.

## Unresolved implications for Aicadia

The evidence establishes constraints but does not make the choice:

1. Place extent, spatial inclusion and Connection traversal must not be treated as
   one fact.
2. Endpoint topology cannot answer what lies between Places.
3. Geometry-derived traversal needs a real oriented course, compatible valid
   geometries, an explicit boundary/dimension rule and exact revisions.
4. The result must permit repeated and simultaneous intervals.
5. A corridor answers possible width/contact, not necessarily exact footsteps.
6. Authored intervals support incomplete geometry and impossible topology, but can
   duplicate or contradict derived truth.
7. Current traversal and accepted historical traversal need different freshness
   semantics.
8. Indexes and subdivision help candidate work but never justify an unbounded read.

The next concept choice should therefore ask one concrete scenario at a time:

- Which immediate player action needs an extent rather than Position and prose?
- For that action, does “falls within” mean authored hierarchy, geometric coverage,
  or both?
- Must an ordinary Connection describe its course now, or is endpoint reachability
  sufficient until actual travel narration is implemented?
- When traversal narration is required, should ordinary landscape be derived from
  current extents or authored as ordered intervals?
- What exactly should the same read return for a one-way portal?

Answering those questions is concept work. This report does not accept terminology,
schema, capabilities or implementation.
