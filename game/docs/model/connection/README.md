---
kind: subject
storage_table: []
---

# Connection

> **Role / side:** Connection model contract / runtime side.
> **Authority:** stable direct travel alternatives between two positioned Places, their direction, immutable description and optional exact course.
> **Excludes:** Place and Position identity — defined in [Place](../place/README.md) and [Position](../position/README.md); multi-Connection travel plans, terrain and Area meaning — absent in [deferred scope](../../deferred.md#absent).

A Connection is one stable non-Entity direct travel alternative between two
different positioned Places. World assigns its id. It is neither an Entity nor a
Relation, so it has no Entity role, Property, Trait, ownership or generic semantic
kind.

```rust
struct Connection {
    id: ConnectionId,
    source: ConnectionEndpoint,
    destination: ConnectionEndpoint,
    allows_reverse: bool,
    name: String,
    description: String,
    shape_description: Option<String>,
    course: Vec<ConnectionPoint>,
}
```

Source-to-destination travel is always allowed. `allows_reverse` alone admits the
opposite direction. Name uses the shared 1–120 bound; description and optional shape
description use the shared 1–4,000 bound. Shape description can help an Agent tell
what the course is like but has no mechanical effect.

The endpoint records retain the exact immutable Position revisions used when the
Connection was established. They are dependencies, not copied coordinate truth.
World admits a Connection only when both endpoints are distinct current Places and
their submitted endpoint revisions are current.

No course points means direct complete travel between the two endpoint points. A
shaped Connection contains 2–128 exact World points in source-to-destination order.
Its first and last points equal the exact endpoint points named by the stored
Position revisions, consecutive points differ and non-adjacent segments do not
intersect. World validates points and segments with checked integer arithmetic.

Connection id is the only identity. Several Connections may have equal endpoints,
direction, name, description and course. Those values never merge or deduplicate a
Connection. Only an exact accepted request retry returns an already-created
Connection automatically; an Agent explicitly reuses another Connection by its
returned id.

An S1 Connection and its course are immutable. No update, delete, retirement,
version, traveller, occupancy, progress, timer or completed trace exists. Character
movement writes only that Character's Position and Activity; it never mutates the
Connection.
