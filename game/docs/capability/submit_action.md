# `submit_action`

> **Role / side:** one capability contract / runtime side.
> **Authority:** what World accepts, validates, stores and records for `submit_action` — one confirmed Action that introduces an Entity or changes exact-local state.
> **Excludes:** how an Agent words this to a player — published as [its tool description](../../mcp/agent/tool/submit_action.md); the private action workshop, preview and confirmation — defined in [Required private-workshop action flow](../agent.md#required-private-workshop-action-flow); error codes and their transport mapping — defined in [canonical errors](../protocol.md#canonical-errors).

## Purpose

Atomically introduce one Entity with initial Property/Trait state or change exact-local
Properties and Traits together after confirmation.

## Input

World call `submit_action(context.user_id, input)`; HTTP `POST /api/action`; MCP `submit_action`. Input is `SubmitAction` below.

## Contract

`submit_action` accepts one of two typed consequence kinds. It is not a generic patch
language: one Action either introduces one Entity or changes existing Entity state.
The state package keeps Property and Trait meanings distinct while committing them
together:

```rust
struct SubmitAction {
    request_id: Uuid,
    expected_place_revision: PlaceRevision,
    prose: String,
    consequence: ActionConsequence,
}

struct IntroduceEntity {
    name: String,
    description: String,
    position_description: Option<String>,
    property: Vec<PropertyInput>,
    trait: Vec<TraitInput>,
}

struct AcceptedAction {
    activity: Activity,
    consequence: AcceptedActionConsequence,
    place: Place,
}

struct PlaceRevision {
    place_entity_id: EntityId,
    occurred_at: DateTime<Utc>,
    activity_id: ActivityId,
}
```

`ActionConsequence` is a strict tagged union:

- `introduce_entity { name, description, position_description?, property[0..100],
  trait[0..100] }` creates and places one Entity at the actor's exact Position with
  optional Position description and initial state; or
- `change_entity_state { property_change[0..100], trait_change[0..100] }` atomically
  combines unique exact `(entity_id, key)` Property changes with typed
  `establish { entity_id, statement }` and `develop { trait_id, statement }` Trait
  changes. At least one list must be non-empty.

The change subjects may be the actor, current Place, other co-present Characters or
placed ordinary Entities at that exact current Place. The submitted Entity ids are
exact selectors, never a dynamic, prose-derived or area selector. Missing, remote,
departed and otherwise ineligible subjects all return the same neutral
`property_entity_unavailable` or `trait_unavailable` error for the corresponding
consequence. World does not branch on Entity role or reveal which Entity is
controlled by a User.

Input accepts no User, Character or Place selector and no effective time. World
derives the User's Character and exact current Place; a missing Character is not
found and a Character without current Place cannot act. World assigns new Entity, Activity,
Property-key and Trait identities and the acceptance time. Establishment names an
eligible Entity; development names one stable current Trait id and World derives its
Entity. Expected Place revision plus the locked current pointer selects the
predecessor atomically; development accepts no predecessor selector.

One accepted introduction atomically creates and places the Entity at the acting
Character's exact Position, writes the Entity's root Position with the submitted
optional Position description, its initial Properties and Trait roots, inserts one Activity with one
canonical prose value and records explicit Entity and Position roles. One accepted state-change Action atomically
writes every Property value/current pointer and every Trait root/version/current
pointer under that same Activity; each affected Entity has `subject` participation
and the Place remains `location`.
Partial acceptance is forbidden. Rejected calls, stale calls and retries add no
Activity, Property or Trait state. The returned `AcceptedAction` tags the introduced
Entity or both exact sorted Property and Trait changes and is the canonical stored
result.

## Input examples

The introduction form is:

```json
{
  "request_id": "20b7e11a-82de-4e1b-b667-34953f398324",
  "expected_place_revision": "opaque-versioned-token",
  "prose": "Mara braces a carved cedar marker beside the crossing.",
  "consequence": {
    "type": "introduce_entity",
    "name": "Cedar Crossing Marker",
    "description": "A waist-high cedar marker carved with three crossing lines.",
    "property": [
      {"key": "material", "value": {"type": "text", "text": "cedar"}}
    ],
    "trait": [
      {"statement": "Answers a hard strike with a low cedar note."}
    ]
  }
}
```

The combined state-change Action is:

```json
{
  "request_id": "fcd45b43-b7d4-45df-a5ee-22b1bd76036b",
  "expected_place_revision": "opaque-versioned-token",
  "prose": "The blast blackens the gate while its echo teaches Pip to wait.",
  "consequence": {
    "type": "change_entity_state",
    "property_change": [
      {
        "entity_id": "8ec3cf2f-7484-4230-ad63-16b9e84e4545",
        "key": "surface",
        "value": {"type": "text", "text": "blackened"}
      }
    ],
    "trait_change": [
      {
        "type": "establish",
        "entity_id": "8ec3cf2f-7484-4230-ad63-16b9e84e4545",
        "statement": "Waits for the second echo before springing."
      }
    ]
  }
}
```

## Validation

Validation is specified in the contract above. Prose, name, description, Property and Trait values — constrained by [shared value validation](../domain.md#shared-value-validation), [Property](../model/property/README.md) and [Trait](../model/trait/README.md); request identity, expected Place revision and retry precedence — constrained by [delivery identity and exact-Place freshness](../protocol.md#delivery-identity-and-exact-place-freshness); this capability adds only the local rules stated in the contract.

## Result

The canonical result is the `AcceptedAction` described above. The introduction bundle and `create_entity` reuse private validation and insertion behavior; one public capability never invokes the other.

## Activity footprint

One Activity per accepted Action, with each affected Entity as `subject` and the
Place as `location` as stated in the contract. An introduction also records the
acting Character Position as `origin` and new Entity Position as `result`; the
general Activity semantics and roles — defined in [Activity](../model/activity/README.md);
this capability narrows them only as the contract states.

## Annotations and retry class

Modifying; idempotent only by request id and the normalized unordered typed set — a retry with the same id and semantically identical input returns the stored result; delivery identity — constrained by [delivery identity and exact-Place freshness](../protocol.md#delivery-identity-and-exact-place-freshness).

## Evidence obligations

World, HTTP and MCP must expose this same semantic contract, strict schema, result and canonical errors.
