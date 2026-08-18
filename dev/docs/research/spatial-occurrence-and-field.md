---
status: historical
era: July scene-claim
---

> **Era:** July scene-claim research; its scene/claim vocabulary predates the 2026-08-07 game reframe.

# Spatial occurrence and field

> **Role / side:** sourced research report / development side.
> **Authority:** records this report's question, sources, findings and implications.
> **Excludes:** product decisions and current implementation contracts; see `game/docs/`.

Date: 2026-07-26

Status: research; emergent nature/material direction confirmed, revised location
recommendation not confirmed

## Question

Does smoke, water or another diffuse physical phenomenon need its own entity and
`entity_location`?

This challenges the confirmed wording:

> Every current physical entity has one `entity_location` row.

The term “physical entity” is not mechanically precise enough. `water` may mean a
material, water inside a bucket, one lake, temporary floodwater or water depth varying
across an entire valley. `smoke` may mean a material type, an observation from far
away, one identifiable plume or a concentration field.

## Short answer

Distinguish three cases without adding new structural tables:

| Case | Example | Source representation |
|---|---|---|
| Type or material | water, smoke | Entity and claims; no location |
| Condition on something located | smoke at the mill, water in a cellar | Time-bounded claim on a located entity or place |
| Discrete spatial occurrence | this plume, this lake, this flood | Entity plus claims and one `entity_location` row |

A fourth case may appear later:

| Case | Example | Representation |
|---|---|---|
| Value varying over space/time | smoke concentration, water depth | Accepted sample/zone claims; optional derived coverage only after a concrete consumer earns it |

The revised invariant should be:

> Every entity represented as one discrete current spatial occurrence has one current
> `entity_location` row.

Not:

> Every entity whose meaning is physical has one current location.

No `is_physical` field and no fixed list of physical kinds is required.

`Type or material` in this table describes how an entity is referenced, not a
server-shipped classification. A material, species or natural phenomenon is an
ordinary world entity introduced by an accepted source package. Its immutable id
allows later scenes and occurrences to reference it; its meaning remains the
time-versioned set of sourced claims about it. The server contains no fixed smoke,
water or material catalogue.

## Smoke

### Smoke as a type or material

The general concept `smoke` may be an entity introduced in play and then used by many
claims:

```text
entity
  id: <smoke type id>
```

Its name, classification and observed properties are claims. It may participate in
`subtype-of` and be referenced as the object of `instance-of`; those id-based claims
make it queryable as a kind. There is no source `kind` string on the entity. It has
no location merely because it is referenced as a reusable category. It is comparable
to `timber`, `water` or an invented plant species: a reusable world referent rather
than one occurrence.

A particular plume may reference this entity through `instance-of`; another material
may reference it through `subtype-of`; a fire, plant or container may reference it
through a free predicate. Those relationships and properties may change as new
scenes add or supersede claims. The referenced entity id does not change.

### Smoke observed at a known place

Scene:

> Smoke hung across the mill yard for ten minutes.

Minimum source:

```text
claim
  subject_id: <mill yard place id>
  predicate_id: <has atmospheric condition predicate id>
  object_id: <smoke type id>
  effective_from: <scene time>
  effective_until: <scene time + established duration>
  scene_id: <source scene id>
```

The time field names are illustrative; the exact accepted-claim time contract remains
open.

The mill yard already supplies spatial context. No smoke entity or duplicate
`entity_location` is needed.

If the duration is unknown, the claim does not invent an end time. A later accepted
scene may supersede it by establishing that the smoke cleared.

### Smoke seen somewhere on the horizon

Scene:

> Mara saw a dark band of smoke beyond the northern ridge.

The observation establishes:

- the observer and scene location;
- the observed phenomenon;
- perhaps a direction; and
- the time of observation.

It does not establish an exact plume geometry or source location. The accepted claim
must not create a fake point beyond the ridge.

Minimum source:

```text
claim
  subject_id: <Mara id>
  predicate_id: <observed predicate id>
  object_id: <smoke type id>
  scene_id: <source scene id>
```

If the direction is accepted too, it needs a separate typed claim using the accepted
claim vocabulary; it is not an unstructured field on this claim. The exact predicate
and literal representation remain open. The important boundary is that an
observation location is not silently reused as the phenomenon location.

### One identifiable smoke plume

A separate entity earns its place when later scenes need to:

- cite the same plume;
- track where it moved;
- state what produced it;
- attach a changing polygon;
- establish who could see it; or
- preserve its history after it dispersed.

Example:

```text
entity
  id: <plume id>

claim
  subject_id: <plume id>
  predicate_id: <instance of predicate id>
  object_id: <smoke type id>

entity_location
  entity_id: <plume id>
  geometry: MULTIPOLYGON(...)
  source_geometry_claim_id: <claim id>
```

The plume geometry may be superseded by later accepted geometry claims. The server
does not calculate fluid motion or advance the plume in the background.

## Water

### Water as material

The reusable `water` entity is discovered and described in the world rather than
preloaded by the server. It has no location merely because other entities reference
it as a material category.

```text
bucket contains material water
well contains material water
roof covered by water
```

The bucket, well or roof provides the location. Creating a new water entity for every
container would add identity without a query or narrative need.

### A lake, river or spring

A lake or river is a discrete identifiable feature. It may have:

- a stable entity ID;
- a name claim;
- a `place` role if scenes or routes use it;
- point, line or polygon geometry;
- changing water-level or extent claims; and
- route edges where travel mechanics require them.

CityGML makes the same useful distinction: a `WaterBody` represents a significant,
permanent or semi-permanent accumulation such as a lake, river, canal or basin. It
models the feature and its geometry, but does not attempt to inherit fluid-flow
simulation.

### Water inside a house

Scene:

> Rainwater covered the kitchen floor.

If no continuing flood object is needed:

```text
kitchen has condition standing water
effective_from: <scene time>
```

This time field is also illustrative rather than a decided schema column.

If characters later track one flood, its source, extent and retreat:

```text
entity
  id: <flood occurrence id>

claim
  subject_id: <flood occurrence id>
  predicate_id: <instance of predicate id>
  object_id: <floodwater kind id>

entity_location
  entity_id: <flood occurrence id>
  place_id: <house id>
  geometry: POLYGON(...) optional
```

The choice depends on identity and future citation, not on how many litres exist.

### Water depth varying across a floodplain

One polygon and one value may be false. Depth can differ at every position and time.
Established geospatial systems represent this as a coverage: a function returning a
property value for positions in a spatial or spatiotemporal domain.

Aicadia does not need a coverage table in the core.

Source facts can remain ordinary accepted claims:

```text
at sample area A, water depth was 0.2 m at time T
at sample area B, water depth was 0.8 m at time T
```

If a later map, simulation or validator needs a continuous flood-depth surface, a
projector may derive a labelled coverage from those claims. The derivation method and
source claims must accompany the result. It is not canon and may be rebuilt.

## Established-system distinction

### Discrete feature

Geographic feature models describe identifiable objects such as a road, lake or
plume with properties and geometry.

This is object-centric:

```text
which object?
what is it?
where is it?
how did it change?
```

### Observation

OGC Observations and Measurements separates:

- feature of interest;
- observed property;
- result;
- observation procedure; and
- phenomenon time.

It explicitly notes that an observation does not inherently own one location. The
relevant location may belong to the observed feature, sampling procedure or observer.

Useful for Aicadia:

- seeing smoke does not establish the smoke source point;
- a claim retains scene, observer and time provenance;
- later observations may disagree without moving one fake smoke entity.

Aicadia does not need a separate `observation` structural table: an accepted scene
and its claims already provide actor, time and provenance.

### Coverage

OGC coverages model homogeneous values varying over space or time, including imagery,
climate values, pollutant concentration and ocean data.

This is property-centric:

```text
what value applies at this position and time?
```

Useful for Aicadia only after the world needs dense environmental values. It should
begin as a projection over accepted claims, not as a shipped environmental
simulation.

## Concrete storage rule

### No new structural type

Do not add:

- `smoke` table;
- `water` table;
- `weather` table;
- `field` table;
- fluid-simulation service; or
- mandatory raster storage.

Use existing structural types:

- `entity` for a reusable type or discrete occurrence;
- `claim` for material, condition, observation, extent and change;
- `place` when the occurrence can anchor scenes or travel;
- `scene` for source time and provenance; and
- `entity_location` as a current projection for discrete spatial occurrences.

### When an occurrence entity earns an ID

Create a separate occurrence entity only when at least one accepted package needs to:

- cite it independently of its container or observer;
- give it its own changing location or extent;
- preserve its individual history;
- connect it to causes or effects; or
- use it as a scene or route anchor.

Otherwise attach the condition or material claim to the already located subject.

This is not a server-side semantic test. The submitting agent makes the distinction
in the structured package; the server validates references, time, geometry and
location mechanically.

## Query behavior

Query:

> What smoke is near Mara?

The current projection may return:

1. smoke-condition claims whose located subject is near Mara;
2. discrete smoke-plume entities whose `entity_location` is near Mara; and
3. recent observation claims made by Mara or nearby characters.

The API keeps their provenance and does not collapse them into one assumed plume.

Query:

> Where is water?

This is too broad. A purposeful query must state which result it wants:

- water-body features;
- entities containing water;
- current standing-water or flood claims;
- exact flood occurrences; or
- derived depth values at a requested geometry and time.

One global search for every claim referencing the water type would be unbounded and
not useful for a scene briefing.

## Scale and failure modes

### Entity per molecule, cloud fragment or wet tile

This creates huge identity churn and makes ordinary conditions expensive.

Avoided by attaching material and condition claims to existing located subjects.

### One location for the abstract material

The reusable `water` or `smoke` type appears to move whenever a new occurrence is
written.

Avoided by keeping type identity non-spatial.

### One plume entity for every observation

Different observers create duplicates for the same smoke, while a single observer
may incorrectly merge several plumes.

Avoided by storing observations first and creating a shared occurrence only when an
accepted package establishes that identity.

### Dense coverage in the core

Every environmental sentence becomes a raster update, interpolation choice and
storage burden.

Avoided by retaining authored samples and zones as claims until a concrete consumer
earns a derived coverage.

### Automatic fluid simulation

Water and smoke change without accepted authorship, consume server resources and
produce facts with no source scene.

Rejected by the dumb-server and no-unconscious-token-burn principles.

## Revised recommendation

Replace:

> Every current physical entity has one `entity_location`.

With:

> Every entity represented as a discrete current spatial occurrence has one
> `entity_location`.

And:

> A type, material or condition does not need its own location. Its spatial context
> may come from the located subject and source scene. A separate occurrence entity is
> created only when the accepted package gives it independent identity.

This is a recommendation, not yet a confirmed concept correction.

## Decision exposed by the research

Should the location invariant apply to discrete spatial occurrences rather than to
everything that could be described as physical?

Recommendation: yes. It handles houses, people, smoke, water and future phenomena
without `is_physical`, fixed kind lists or environmental modules.

## Sources

- [OGC Observations, Measurements and Samples](https://docs.ogc.org/as/20-082r4/20-082r4.html)
- [OGC Coverage Implementation Schema](https://docs.ogc.org/is/09-146r8/09-146r8.html)
- [OGC SensorThings observation model](https://docs.ogc.org/is/15-078r6/15-078r6.html)
- [OGC CityGML WaterBody](https://docs.ogc.org/is/20-010/20-010.html#toc61)
- [PostGIS geometry coverage predicate](https://postgis.net/docs/ST_Covers.html)
