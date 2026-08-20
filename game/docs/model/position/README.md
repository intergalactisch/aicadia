---
kind: state
storage_table: []
---

# Position

> **Role / side:** Position model contract / runtime side.
> **Authority:** one Entity's exact direct World point, optional current description, immutable versions and current revision.
> **Excludes:** Place identity — defined in [Place](../place/README.md); travel topology — defined in [Connection](../connection/README.md); relative placement, geometry and privacy — absent in [deferred scope](../../deferred.md#absent).

Position is the exact spatial state of one Entity. Its subject identity is always
that Entity's `entity_id`; Position has no separate id, name, kind, Property or
Trait. An Entity may have no Position. Every Place and every Character that has
entered the World has exactly one current Position.

```rust
struct Position {
    x_cm: i64,
    y_cm: i64,
    z_cm: i64,
    description: Option<String>,
    position_revision: PositionRevision,
}
```

Each coordinate is an exact whole-centimetre offset from one permanent World origin
and lies inclusively between `-1_000_000_000_000_000` and
`+1_000_000_000_000_000`. Coordinates are mechanical truth. World never parses a
name, description, Property or Trait into coordinates and never infers adjacency,
distance, visibility, collision or access from them.

The optional description is trimmed non-NUL English text of 1–4,000 Unicode
characters. It can preserve useful narration such as an unusual height or
orientation that World does not interpret. It does not change the point, establish a
Relation, grant authority or become a Trait.

Every accepted Position change appends one immutable version and atomically advances
that Entity's current pointer. A version is identified internally by
`(entity_id, activity_id)`, names at most one prior version of the same Entity and is
the result of that Activity. Each Entity has at most one root and at most one
successor per version. The current pointer must name the same Entity and the unique
lineage tip at commit.

`PositionRevision` is the opaque public encoding of `(entity_id, activity_id)`. It
is a freshness value, not an authorization token, request id, timestamp ordering,
global World version or coordinate hash. Clients copy it unchanged.

S1 creates or changes Position only through entry Place creation, World entry,
situated Entity introduction, discovery and Character movement. It has no generic
Position create, update, delete or removal capability. Place Position and Connection
endpoint Position are immutable in S1.
