---
kind: role
storage_table: [place, place_map_index]
---

# Place

> **Role / side:** Place model contract / runtime side.
> **Authority:** the Place Entity role, its stable identity, required direct Position and World-entry flag.
> **Excludes:** Entity subject rules — defined in [Entity](../entity/README.md); coordinate and revision rules — defined in [Position](../position/README.md); travel alternatives — defined in [Connection](../connection/README.md).

```rust
struct Place {
    entity: Entity,
    position: Position,
    is_entry: bool,
}
```

A Place is an Entity role whose stable identity is `place.entity_id`. Assigning the
role never creates a second Entity or Place id. Every Place has one required direct
Position; the `place` row stores no coordinates, name or description. Place Position
is immutable in S1.

Zero entry Places is valid before genesis; at most one row may have
`is_entry = true`. Any deliberately established positioned Entity may receive the
Place role through a capability that explicitly creates that role. Coordinate
equality never creates, merges or selects a Place.
