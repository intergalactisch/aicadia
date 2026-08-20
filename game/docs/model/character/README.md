---
kind: role
storage_table: [character]
---

# Character

> **Role / side:** Character model contract / runtime side.
> **Authority:** the User-owned Entity role, its identity, current Position and optional current Place.
> **Excludes:** Entity subject rules — defined in [Entity](../entity/README.md); Position and Place identity — defined in [Position](../position/README.md) and [Place](../place/README.md); movement admission — defined in [`move_character`](../../capability/move_character.md).

```rust
struct Character {
    entity: Entity,
    owner_user_id: UserId,
    position: Option<Position>,
    current_place: Option<Place>,
}
```

A Character is a User-owned Entity role. `character.entity_id` is both its primary
key and Entity foreign key; there is no Character surrogate id or copied Entity
state. Before World entry both Position and current Place are absent. Once entered,
Position is always present. Current Place is present only when the Character stands
at an explicitly associated Place and becomes absent during partial travel between
Places. Current Place is never inferred from coordinate equality.
