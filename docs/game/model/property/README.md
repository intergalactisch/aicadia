---
kind: state
storage_table: [property_key, entity_property, entity_property_history]
---

# Property

> **Role / side:** Property model contract / runtime side.
> **Authority:** the tagged Property value shapes, canonical key rules, bounds and their rejection behavior.
> **Excludes:** Entity subject rules, Trait statements, Activity shape and delivery status; see the Entity, Trait and Activity contracts and `docs/evidence/`.

Every initial or changed Property uses one strict tagged value:

```text
PropertyInput { key, value: PropertyValue }
EntityPropertyChangeInput { entity_id, key, value: PropertyValue }
PropertyValue = { type: "text", text } | { type: "integer", integer }
EntityPropertyChange { entity: EntitySummary, key, value: PropertyValue }
```

A canonical key contains 1–64 ASCII lower-snake-case characters, starts with a
letter and matches `^[a-z][a-z0-9]*(?:_[a-z0-9]+)*$`. The Agent creates a key at its
first accepted use. World stores only its canonical English key, immutable value
type and first Activity provenance. Reuse with the same type is valid; reuse with a
different type returns `property_key_conflict`. There is no finite catalog,
description, alias, synonym inference or control provenance on a key.

Control-like keys and values are not denylisted, but they remain ordinary in-World
content and never become authorization, ownership or actual control metadata.

Text values are trimmed, reject U+0000 and contain 1–4,000 Unicode characters.
Integer values are signed 64-bit integers. Initial lists are semantically unordered
and require unique keys. Action and Interaction change lists are semantically
unordered and require unique exact `(entity_id,key)` pairs. A duplicate, invalid
tag/value, invalid key or list outside its route's 0–100 or 1–100 bound returns
`invalid_property`; the complete enclosing operation writes nothing.
