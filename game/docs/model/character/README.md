---
kind: role
storage_table: [character]
---

# Character

> **Role / side:** Character model contract / runtime side.
> **Authority:** the User-owned Entity role, its identity and its optional current Place.
> **Excludes:** Entity subject rules, Place identity and delivery status; see the Entity and Place contracts and `dev/docs/evidence/`.

```rust
struct Character {
    entity: Entity,
    owner_user_id: UserId,
    current_place: Option<Place>,
}
```

A Character is a User-owned Entity role. `character.entity_id` is both its primary
key and Entity foreign key; there is no Character surrogate id or copied Entity
state. An absent `current_place` means the Character exists but has not entered the
World; it is not a missing lookup or unknown coordinate.
