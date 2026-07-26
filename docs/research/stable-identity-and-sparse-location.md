# Stable identity and sparse location

Date: 2026-07-26

Status: research; stable identity, name claims and current-location invariant
confirmed

Decision after research: every persisted world reference uses an immutable opaque ID.
Names, former names and aliases are versioned claims rather than identity or
reference keys. Every current physical entity has one `entity_location` projection
row containing at least a place ID, geometry or active route; no containing place or
geographic hierarchy level is mandatory. UUIDv7, package-local draft references,
exact field-combination validation and derived-coverage behavior remain open.

Follow-up: `spatial-occurrence-and-field.md` shows that “physical entity” is still
too broad. The location-row shape remains confirmed, but research now recommends
applying it to a discrete current spatial occurrence. Materials, types and conditions
may receive spatial context through their located subject and source scene instead.
This wording correction is not yet confirmed.

## Question

How should Aicadia identify and spatially place entities when:

- names can change or be reused;
- millions of agents create entities concurrently;
- a scene can introduce several related entities atomically;
- a village may contain one house and no district;
- exact geometry may be known without a complete place hierarchy; and
- imprecise location must remain valid without invented coordinates or container
  places?

This report challenges two earlier research recommendations:

1. an entity `name` column is too close to identity; and
2. requiring one containing place for every physical entity is too strict.

## Short answer

Use four independent concepts:

| Concept | Stored meaning |
|---|---|
| stable ID | Immutable opaque identity used by every stored reference |
| name claim | A sourced, time-versioned statement that an entity is called something |
| current location | A rebuildable row containing a place ID, geometry or active route |
| place relation | An accepted relation between actual places, with no required levels |

The recommended invariant is:

> Every current physical entity has one current `entity_location` projection row.
> That row must contain a `place_id`, geometry or active `place_edge_id`. It does not
> have to contain all three.

This replaces:

> Every current physical entity must have one containing place.

Consequently:

- a one-house village needs no district;
- a house with exact geometry needs no fabricated block or neighbourhood;
- a house known only to be in a valley needs no fabricated coordinate;
- a travelling character can be on a route rather than inside a place;
- a name change never changes identity; and
- a duplicate name never creates a reference collision.

## Stable identity

### Existing entities are referenced only by ID

Every persisted relationship uses opaque IDs:

```text
claim
  subject_id: 019b...
  predicate_id: 019a...
  object_id: 0198...
```

Never:

```text
subject: "Mara's house"
predicate: "located in"
object: "Moss End"
```

Human names may be supplied for display or search, but the server resolves and
returns candidate IDs before a package is accepted. A write containing an unresolved
name string is rejected.

Concrete flow:

1. The agent searches for `Moss End`.
2. The API returns candidate entities with ID, current name, kind, location and
   short description.
3. The agent selects the intended ID.
4. The submitted claim uses that ID.
5. A later rename does not affect the stored claim.

### UUIDv7 is the simplest current candidate

PostgreSQL 18 has a native `uuid` type and native UUIDv4 and UUIDv7 generation.
RFC 9562 defines UUIDv7 as a 128-bit UUID with a time-ordered Unix timestamp prefix
and random data for uniqueness.

Candidate:

```text
entity.id       uuid
scene.id        uuid
claim.id        uuid
predicate.id    uuid
place_edge.id   uuid
```

Why UUIDv7 rather than an integer exposed to clients:

- independent writers do not need a shared sequence before constructing a package;
- IDs remain unique across imports, replicas and tools;
- one ID type keeps the model flat;
- time ordering improves database index locality compared with fully random UUIDv4;
  and
- the ID remains meaningless to the fiction.

The UUID timestamp is not world time and not canonical event order. `accepted_at`
and scene-log order remain authoritative.

This is still an implementation candidate. The conceptual requirement is an opaque,
immutable ID; not specifically UUIDv7.

### New entities inside one package use local references

A scene may introduce a house and tree and relate them before either has a permanent
ID:

```text
new_entity:
  local_ref: house
  kind: house

new_entity:
  local_ref: oak
  kind: tree

claim:
  subject_ref: house
  predicate_id: <built_around predicate UUID>
  object_ref: oak
```

`local_ref` exists only in the private draft. Before the complete package is shown
for human confirmation, the MCP adapter or deterministic draft validator:

1. allocates permanent IDs;
2. resolves every local reference;
3. validates the resulting graph; and
4. produces the final package containing only permanent IDs.

The human confirms that resolved package. The accepted write path stores exactly
what was confirmed; it does not replace identifiers after confirmation.

This is a conventional pattern. OpenStreetMap change files use temporary negative
IDs which the server maps to permanent IDs. JSON:API defines `lid` for linking new
resources locally inside one document.

The human and agent do not manually create or transcribe UUIDs.

## Names are sourced world-state, not identity

### Why an entity `name` column is insufficient

One mutable `entity.name` field cannot represent:

- a village renamed after a flood;
- an old name still used by some characters;
- two villages with the same name;
- an unnamed house;
- a formal name and common nickname;
- a disputed name; or
- the scene that introduced or replaced a name.

Wikidata demonstrates the basic identity split: an item has a stable Q identifier,
while labels and aliases are editable and need not be unique.

### Recommended source form

A name is an ordinary provenance-carrying claim:

```text
claim
  subject_id: <place UUID>
  predicate_id: <current_name predicate UUID>
  object_value: "Moss End"
  scene_id: <source scene UUID>
```

A rename adds a new claim which supersedes the previous current-name claim. The old
claim remains in history and may remain searchable as a former name.

An alias is another name claim with a different predicate or explicit role. The
exact predicate vocabulary remains a later choice.

### Recommended query projection

A rebuildable projection may expose:

```text
entity_label
  entity_id
  display_name
  source_claim_id
```

Search may additionally index former names and aliases. `display_name` is a current
read convenience, never an identity key and never a source field.

Duplicate display names are allowed. A search response disambiguates them:

```text
Moss End — village in North Valley — 019b...
Moss End — abandoned orchard near Alden — 019c...
```

This also means the existing open question about name uniqueness need not control
referential integrity. A narrative naming rule may still discourage confusing
duplicates, but the database does not depend on uniqueness.

## No fixed geographic levels

### A place hierarchy is sparse and variable-depth

The database does not require:

```text
world -> region -> city -> district -> village -> block -> house
```

It stores only relationships the world has established:

```text
North Valley -> Moss End -> lone house
```

or:

```text
North Valley -> lone house
```

or, where exact geometry is known:

```text
lone house -> POLYGON(...)
```

`district`, `village`, `courtyard`, `forest` and `house` remain emergent kinds. None
is a required hierarchy level.

### A one-house village

Suppose one accepted scene establishes:

> Moss End is a village consisting of one house beside a spring.

The minimum useful entities are:

```text
place UUID A
  current name: Moss End
  kind: village

entity/place UUID B
  kind: house

entity UUID C
  kind: spring
```

Possible claims:

```text
house B located in village A
spring C located in village A
village A located in North Valley
```

There is no district. The village does not need several houses to qualify as a place.
It exists because the fiction identifies and refers to it as one place.

If the story only establishes one isolated house in North Valley, there is no reason
to create a village either:

```text
house B located in North Valley
```

If the house has exact geometry:

```text
house B geometry POLYGON(...)
```

The system must not create an unnamed district merely to fill a hierarchy slot.

### When a house is also a place

`entity` identifies the thing. The structural `place` role is needed only when the
thing can anchor scenes, locations or route edges.

Examples:

- a decorative model house is only an entity;
- an inhabited house which characters enter may also be a place;
- a room may be a place when scenes or movement distinguish it;
- an incidental cupboard does not become a place merely because it is inside a
  room.

The same UUID should identify the house as an entity and as a place. Do not create a
second “house location” entity solely for navigation.

## Current physical location

### One projection row, optional components

Candidate projection:

```text
entity_location
  entity_id
  place_id
  geometry
  place_edge_id
  source_location_claim_id
  source_geometry_claim_id
  source_journey_scene_id
```

All IDs are UUIDs. The nullable columns express known precision directly.

Valid static forms:

```text
place_id set, geometry null
  => exact geometry is not established

place_id null, geometry set
  => exact geometry is established; no semantic containing place is claimed

place_id set, geometry set
  => both semantic placement and exact geometry are established
```

Valid travel form:

```text
place_edge_id set
  => the entity is travelling; current geometry may be derived for the query
```

Invalid form:

```text
place_id null
geometry null
place_edge_id null
  => a current physical entity has no usable location
```

No stored `location_mode` status is necessary. The populated columns state the
technical fact.

### Exact geometry and place membership may disagree

A house polygon can fall outside the currently accepted village extent while an
accepted claim says inhabitants consider it part of the village.

The server does not silently rewrite either statement. It can return:

```text
accepted place relation: Moss End
current geometric coverage: outside current Moss End extent
```

PostGIS `ST_Covers` can calculate the mismatch using a spatial index. The applicable
predicate or rule decides whether the mismatch is rejected, warned about or allowed.

### Covering places can be derived

For an entity with exact geometry but no `place_id`, the map query may return every
current place extent covering it. Those are calculated results, not accepted
`located_in` claims.

This preserves the difference between:

- physical inclusion according to current polygons; and
- an authored social, administrative or named-place relationship.

## Why this is more scalable

### Write path

Creating one isolated house touches:

- one entity;
- its source claims;
- one current location row; and
- only directly affected search and map projections.

It does not create or update:

- a district;
- a block;
- every ancestor;
- a calculated city hull; or
- a storage cell in canon.

### Read path

Three ordinary indexes cover the main queries:

```text
B-tree entity_location(place_id)
GiST   entity_location(geometry)
B-tree entity_location(place_edge_id)
```

A query uses the index matching the information actually available. A place subtree
projection remains useful where explicit place relations exist, but entities with
exact geometry do not need a fabricated parent merely to enter that tree.

### Rename path

Renaming one village:

1. appends one name claim;
2. supersedes the previous current-name claim;
3. updates one label/search projection; and
4. changes no foreign key or relationship.

### Concurrent creation

Separate scene packages create independent UUIDs and append independently. A package
which names an existing entity must use its ID and current supporting claim. Duplicate
human names do not create write conflicts.

## Failure modes avoided

### Name used as foreign key

A rename either breaks references or requires a mass update. Reused names point to
the wrong entity.

Avoided by immutable IDs.

### Mandatory district level

One-house villages gain meaningless district records. Agents invent names and
boundaries solely to satisfy the schema.

Avoided by variable-depth place relations.

### Mandatory containing place

An exactly mapped object still requires a fabricated semantic parent. Disputed
boundaries force an arbitrary accepted relationship.

Avoided by allowing geometry without `place_id`.

### Geometry as identity

Correcting a footprint appears to create a new house. Splitting or simplifying a
polygon breaks citations.

Avoided by stable entity identity.

### One mutable name column

Former names, aliases and disputes are lost or duplicated in ad-hoc fields.

Avoided by sourced name claims plus a current label projection.

### UUIDs shown as interaction ceremony

Humans and LLMs are asked to copy opaque strings manually and make mistakes.

Avoided by query results, MCP-selected IDs and package-local references.

## `5jaar`: what survived

After five years:

- every persisted link resolves by immutable ID;
- names change freely without graph rewrites;
- unnamed and duplicate-named entities are normal;
- most place paths are short and uneven;
- exact map geometry exists only where established;
- some geometry-only entities have no accepted containing place;
- one current location row serves coarse, exact and travel queries;
- PostGIS handles spatial candidates;
- place relations carry social and narrative meaning rather than storage structure;
  and
- agents almost never display or type a UUID, although every MCP result carries one.

Removed during those five years:

- unique-name foreign keys;
- required `region`, `district`, `block` or address layers;
- a separate location entity for every house;
- a mandatory parent place for exact geometry;
- location precision statuses duplicating nullable facts;
- global hierarchy repair jobs after every boundary edit; and
- map tile or spatial cell IDs in canon.

## Backcast to now

### Recommended choices for discussion

1. Every persisted world reference uses an immutable opaque ID.
2. UUIDv7 is the current implementation candidate.
3. Existing entities are never referenced by name on the accepted write path.
4. Names, former names and aliases are accepted claims.
5. `entity_label` is a rebuildable display/search projection.
6. New entities may use package-local references during drafting; the confirmed
   package contains permanent IDs.
7. No geographic hierarchy level is required.
8. Every current physical entity has one `entity_location` row containing at least a
   place ID, geometry or active route.
9. A containing place is optional when exact geometry exists.
10. Derived polygon coverage never silently creates an accepted place relationship.

Items 1, 3, 4, 7, 8 and 9 are confirmed concept choices. UUIDv7, package-local draft
references, the exact label projection and derived-coverage behavior remain
recommendations.

### Smallest test fixture

1. Create two places both currently called Moss End.
2. Query them by name and require the caller to select one returned UUID.
3. Rename one place and prove every old claim still resolves.
4. Create a one-house village with no district.
5. Create an isolated exact house polygon with no containing-place claim.
6. Query which current extents cover the isolated house.
7. Add a semantic place claim that disagrees with polygon coverage.
8. Create a house and tree in one package through local references.
9. Replay the log and reproduce every relationship and current projection from the
   permanent IDs in the accepted package.

## Decision exposed by the research

Should Aicadia make immutable opaque IDs the only accepted reference mechanism, while
names become versioned claims and place hierarchy levels remain entirely optional?

Decision: immutable IDs, name claims, the absence of required hierarchy levels and
the revised current-location contract are confirmed. The concrete ID format,
package-local draft references and remaining projection details are open.

## Sources

- [RFC 9562 UUIDv7](https://www.rfc-editor.org/rfc/rfc9562.html#name-uuid-version-7)
- [PostgreSQL UUID type](https://www.postgresql.org/docs/current/datatype-uuid.html)
- [PostgreSQL UUID functions](https://www.postgresql.org/docs/current/functions-uuid.html)
- [Wikidata items and stable Q identifiers](https://www.wikidata.org/wiki/Help:Items)
- [Wikidata labels](https://www.wikidata.org/wiki/Help:Label)
- [Wikidata aliases](https://www.wikidata.org/wiki/Help:Aliases)
- [OpenStreetMap temporary and permanent IDs](https://wiki.openstreetmap.org/wiki/Element)
- [JSON:API local identifiers](https://jsonapi.org/format/#document-resource-object-identification)
- [Overture division hierarchy](https://docs.overturemaps.org/schema/reference/divisions/division/)
- [PostGIS spatial coverage](https://postgis.net/docs/ST_Covers.html)
