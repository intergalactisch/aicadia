---
kind: role
storage_table: [place]
---

# Place

> **Role / side:** Place model contract / runtime side.
> **Authority:** the Place Entity role, its stable identity and the World-entry flag.
> **Excludes:** Entity placement rules, movement, geometry and delivery status; see the Entity contract, `docs/game/deferred.md` and `docs/evidence/`.

```rust
struct Place {
    entity: Entity,
    is_entry: bool,
}
```

A Place is an Entity role whose stable identity is `place.entity_id`. Zero entry
Places is valid before genesis; at most one row may have `is_entry = true`.
