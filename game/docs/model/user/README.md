---
kind: participant
storage_table: [user]
---

# User

> **Role / side:** User model contract / runtime side.
> **Authority:** the durable participant and request-provenance subject and its ownership bound.
> **Excludes:** authentication, accounts, Character behavior and delivery status; see `game/docs/protocol.md`, the Character contract and `dev/docs/evidence/`.

```rust
struct User {
    id: UserId,
    created_at: DateTime<Utc>,
}
```

A User is a durable participant and request-provenance subject. It is not an Entity,
Character, Place, account model or authenticated identity. Each User owns at most one
Character. `create_user` creates only a User and never writes game activity.
