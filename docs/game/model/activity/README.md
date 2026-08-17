---
kind: history
storage_table: [activity, activity_entity]
---

# Activity

> **Role / side:** Activity model contract / runtime side.
> **Authority:** the immutable normalized history record, its operations, roles, changes, prose and ordering.
> **Excludes:** current subject state, value validation and delivery status; see the Entity, Property and Trait contracts and `docs/evidence/`.

Current state remains authoritative and is never rebuilt by replay. Each accepted
covered mutation appends one immutable activity and its normalized involved-Entity
relations in the same transaction:

```rust
struct Activity {
    id: ActivityId,
    operation: ActivityOperation,
    actor_character: Option<EntitySummary>,
    context_place: Option<PlaceSummary>,
    involved_entity: Vec<ActivityEntityReference>,
    property_change: Vec<EntityPropertyChange>,
    trait_change: Vec<ActivityTraitChange>,
    prose: Option<String>,
    occurred_at: DateTime<Utc>,
}

enum ActivityOperation {
    CreateCharacter,
    CreateEntity,
    CreateEntryPlace,
    EnterWorld,
    SubmitAction,
    SubmitInteraction,
    SubmitDiscovery,
}

enum ActivityEntityRole {
    Subject,
    Destination,
    Location,
    Target,
}
```

The stored `activity.requested_by_user_id` retains accountable internal request
provenance but is intentionally absent from player-visible history. Stable ids and
server-owned roles have these exact meanings:

| Operation | Actor Character | Context Place | Involved Entity |
| --- | --- | --- | --- |
| `create_character` | absent | absent | new Character Entity as `subject` |
| `create_entity` | current Character when one exists | its current Place when present | new Entity as `subject` |
| `create_entry_place` | proposing unplaced Character | absent | new Place Entity as `subject` |
| `enter_world` | entering Character | entry Place | entry Place Entity as `destination` |
| `submit_action.introduce_entity` | acting Character | derived current Place | new Entity as `subject`; current Place as `location` |
| `submit_action.change_entity_state` | acting Character | derived current Place | each Property- or Trait-affected Entity as `subject`; current Place as `location` |
| `submit_interaction` | acting Character | derived current Place | 1–100 existing Entities as `target`; current Place as `location` |
| `submit_discovery` | acting Character | attempt's Place, equal to the Character's derived current Place | found Entity as `subject`; current Place as `location` |

Only accepted `submit_action`, `submit_interaction` and `submit_discovery` Activity
has non-null prose, request id and request fingerprint. Existing and other new
Activity keeps those fields null. Activity rows, relations and accepted prose reject
update and delete. The consumed investigation attempt points to its accepted
discovery Activity; Activity does not duplicate the attempt id in its public shape.
Reads, rejected requests, transport traffic, conversation text and private Agent
reasoning are not activity. There is no JSON event payload, universal event
abstraction or event sourcing.

`property_change` is empty when an Activity established no Properties. Otherwise it
contains that Activity's exact typed values, sorted by Entity id then key, after the
existing personal or Place authorization has selected the Activity. Initial
Properties from all five creation routes are changes of their creation Activity.
Activity never infers a Property from prose, and internal Property-key ids are not
exposed.

`trait_change` is empty when an Activity established or developed no Trait. Initial
Traits from all five creation routes are establishments of their creation Activity;
that provenance records first accepted shared-World establishment and does not claim
the Trait was fictionally learned then. Otherwise it contains the exact sorted Activity-backed establishment/development
results, including stable Trait id, compact owning Entity summary, current statement
and previous statement for development. These are hydrated in one bounded query
after the personal or Place lens authorizes the Activity. Activity Entity references
remain compact historical references and never recursively carry current Property/
Trait associations.

One Activity owns one canonical prose value. Personal and Place-local history reads
return that same record rather than copied lens-specific text. Every history lens
orders canonical records by `(occurred_at, id)`; World acceptance is the only current
time axis and Agents cannot backdate or reorder history.
