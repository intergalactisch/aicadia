---
kind: subject
storage_table: [entity, entity_location]
---

# Entity

> **Role / side:** Entity model contract / runtime side.
> **Authority:** the durable World subject, what it may carry and its explicit current Place relation.
> **Excludes:** Property and Trait value rules — defined in [Property](../property/README.md) and [Trait](../trait/README.md); exact spatial state — defined in [Position](../position/README.md); Entity roles — defined in [Character](../character/README.md) and [Place](../place/README.md).

```rust
struct Entity {
    id: EntityId,
    name: String,
    description: String,
    introduced_by_user_id: UserId,
    introduced_at: DateTime<Utc>,
}
```

An Entity is one durable World subject that later participants must be able to refer
to again. Names are display text, not identifiers, and are not unique. Entity has no
type, kind, taxonomy, ownership or discovery claim. `introduced_by_user_id` says who
introduced the record, not who fictionally created, owns or discovered the subject.

Every Entity may carry zero or more Properties without changing or revealing its
role. A Property is one Entity-owned canonical `key = value`; its value is text or a
signed 64-bit integer. Characters, Places, furniture, flora, fauna and ordinary
Entities use the same model. Property is not ownership, control provenance, a score,
a Trait or prose.

Every Entity may also carry zero or more Traits without changing or revealing its
role. A Trait has one World-assigned stable id, belongs to exactly one Entity and
stores one current non-executable statement through an immutable predecessor-linked
version lineage. Its unique root version is the sole establishing Activity
provenance; development retains the same Trait id and advances only its current
pointer. At transaction commit every stable Trait has exactly one root and exactly
one current pointer, and that pointer identifies the lineage tip. Retirement,
reactivation, deletion, merge and transfer are absent.

Trait statements are trimmed, reject U+0000 and contain 1–4,000 Unicode characters.
They may characterize an Entity but never grant a modifier, permission, action, roll
result or other mechanic. World performs no synonym, paraphrase, contradiction or
cross-model inference. Semantic contradictions may coexist; development supersedes
only the prior version in that same Trait lineage and has no automatic precedence
over another Trait, Property or immutable description.

A User never receives a direct profile or Property-storage edit. The User steers
and confirms the complete meaning, the Agent proposes exact initial state or an
Action/Interaction consequence, and World alone validates and writes. No accepted
Property input identifies which Entities are User-controlled. The same authority
split applies to Traits: the Agent authors exact initial or contextual state, the
User accepts or rejects its complete natural preview, and World alone
validates/writes it.
There is no direct Trait editor.

Property keys and values are user-authored in-World content, including keys or text
such as `user_controlled`, `npc` or `owner_user_id`. They may make fictional claims,
but never establish or reveal actual User, Character, NPC, ownership or control
provenance. World applies the ordinary key/value validation rules and has no
control-word denylist; structural provenance comes only from authorized typed World
fields, never Property content.

An Entity may have zero or one Position. Position is separate current state of this
same Entity and never changes Entity identity or role. An ordinary Entity may also
have zero or one explicit current Place relation. Either absence is valid;
coordinate equality never creates the Place relation, and neither state is inferred
from prose or Activity.
