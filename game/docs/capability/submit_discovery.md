# `submit_discovery`

> **Role / side:** one capability contract / runtime side.
> **Authority:** what World accepts, validates, stores and records for `submit_discovery` — establishing one found Entity from a positive attempt.
> **Excludes:** how an Agent words this to a player — published as [its tool description](../../mcp/agent/tool/submit_discovery.md); the investigation and discovery conduct, preview and confirmation — defined in [Required investigation and discovery flow](../agent.md#required-investigation-and-discovery-flow); error codes and their transport mapping — defined in [canonical errors](../protocol.md#canonical-errors).

## Purpose

After a positive investigation and explicit User confirmation, atomically establish
one found Entity with initial Property/Trait state at the attempt's exact Place.

## Input

World call `submit_discovery(context.user_id, input)`; HTTP `POST /api/discovery`; MCP `submit_discovery`. Input is `SubmitDiscovery` below.

## Contract

```rust
struct SubmitDiscovery {
    request_id: Uuid,
    attempt_id: Uuid,
    prose: String,
    find: DiscoveryFind,
}

struct DiscoveryFind {
    name: String,
    description: String,
    property: Vec<PropertyInput>,
    trait: Vec<TraitInput>,
}

struct AcceptedDiscovery {
    activity: Activity,
    entity: Entity,
    place: Place,
}
```

The find is exactly one ordinary Entity with 0–100 initial Properties and 0–100
initial Traits. It accepts no kind, User, Character, Place, revision, effective
time, chance input or additional result. Names need not be unique. World assigns the
Entity, Property-key, Trait and Activity identities and acceptance time.

Before taking a lock, World strictly decodes and normalizes prose and the complete
typed find and derives the canonical discovery fingerprint. Under the User lock it
first resolves an accepted Activity-namespace retry: equal operation and fingerprint
return the canonical stored result; changed content returns
`discovery_request_conflict`. Only then does it resolve the Character and attempt.

The attempt must belong to the contextual User, be positive, unconsumed and
unvoided, and name the Character and its exact current Place. Every well-formed
foreign, zero, consumed, voided, unplaced or moved attempt returns the same neutral
`discovery_attempt_unavailable`. The input binds no `place_revision`; unrelated
Place Activity never invalidates an otherwise available attempt.

After locking the Place, World applies the ordinary Entity, Property and Trait
rules. One acceptance creates and places the Entity, establishes all initial state,
writes one `submit_discovery` Activity with canonical prose and exact roles, consumes
the attempt through that Activity and advances the Place pointer in one transaction.
Partial acceptance is forbidden.

## Input example

```json
{
  "request_id": "15594279-3498-493d-994a-d729b715dd28",
  "attempt_id": "27bb3450-4159-462f-bd9b-ce5617ceef21",
  "prose": "Mara parts the reeds and finds pale cups ringing with trapped rain.",
  "find": {
    "name": "Rainbell Cups",
    "description": "A low cluster of chalk-pale cups whose thin rims ring when rain moves inside them.",
    "property": [
      {"key": "colour", "value": {"type": "text", "text": "chalk-pale"}}
    ],
    "trait": [
      {"statement": "Rings softly when collected rain shifts inside its cups."}
    ]
  }
}
```

## Validation

Invalid discovery prose uses `invalid_discovery`. Find name/description, Property and Trait failures retain `invalid_entity`, `invalid_property`, `invalid_trait` and `property_key_conflict`; `invalid_discovery` is never a generic wrapper for typed find errors. Normalization of prose and the find — constrained by [shared value validation](../domain.md#shared-value-validation); retry and error precedence — defined in [discovery delivery identity](../protocol.md#discovery-delivery-identity); this capability adds only the typed-error rules above.

## Result

The canonical result is exactly `{ activity, entity, place }`. Initial state is not
duplicated beside the Activity; the existing paginated
`get_entity_at_current_place` read returns the Entity's current Properties and
Traits.

## Activity footprint

One immutable `submit_discovery` Activity stores the acting Character, exact context
Place, canonical prose, found Entity as `subject` and Place as `location`. The
attempt's `consumed_by_activity_id` is the internal durable result-provenance link.

## Annotations and retry class

Modifying, irreversible World history; idempotent by Activity request id and normalized discovery content. The discovery request id uses the shared Activity namespace with Action and Interaction: the same id with semantically identical prose, attempt and normalized find returns the stored result; changed content returns `discovery_request_conflict`; reusing an Activity request id accepted for another operation conflicts. The call triggers no other Agent, notification or background process.

## Evidence obligations

World, HTTP and MCP must expose this same strict input, retry precedence, exact
result, neutral attempt error, atomic Activity/state footprint and canonical typed
find errors.
