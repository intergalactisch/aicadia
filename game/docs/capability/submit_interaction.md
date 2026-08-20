# `submit_interaction`

> **Role / side:** one capability contract / runtime side.
> **Authority:** what World accepts, validates, stores and records for `submit_interaction` — one confirmed outward behavior toward co-present Entities with optional state changes.
> **Excludes:** how an Agent words this to a player — published as [its tool description](../../mcp/agent/tool/submit_interaction.md); the private Interaction workshop, preview and confirmation — defined in [Required private-workshop Interaction flow](../agent.md#required-private-workshop-interaction-flow); error codes and their transport mapping — defined in [canonical errors](../protocol.md#canonical-errors).

## Purpose

Record one outward behavior toward 1–100 explicit co-present Entities with optional actor/target Property and Trait changes, without authoring a response.

## Input

World call `submit_interaction(context.user_id, input)`; HTTP `POST /api/interaction`; MCP `submit_interaction`. Input is `SubmitInteraction` below.

## Contract

`submit_interaction` records one Character's outward behavior toward existing
Entities. It may carry typed Property and Trait changes without authoring a target's
response:

```rust
struct SubmitInteraction {
    request_id: Uuid,
    expected_place_revision: PlaceRevision,
    prose: String,
    target_entity_id: Vec<EntityId>,
    property_change: Vec<EntityPropertyChangeInput>,
    trait_change: Vec<EntityTraitChangeInput>,
}

struct AcceptedInteraction {
    activity: Activity,
    place: Place,
}
```

The target list is an unordered set containing 1 through 100 distinct Entity ids.
Input accepts no User, actor or Place id. World derives the current User's entered
Character and exact current Place, excludes the acting Character and accepts only:

- other Characters currently entered at that exact Place;
- ordinary Entities explicitly located there; and
- the derived current Place Entity itself.

Every target is validated in one atomic operation. An absent id, duplicate id, the
actor's own id, an Entity elsewhere, or an Entity that ceased to be co-present all
return the same neutral `interaction_target_unavailable` result and write nothing.
The result never distinguishes nonexistence from existence outside the Character's
present context. An empty list or a list over 100 is invalid Interaction input.

`property_change` defaults to empty and contains at most 100 unique exact
`(entity_id, key)` pairs. Each subject must be the actor or an explicit target; a
local non-target, absent, remote or otherwise ineligible Entity returns neutral
`property_entity_unavailable` and rolls back the whole Interaction. Empty retains
the delivered outward-only meaning. Non-empty stores outward participation and exact
typed changes under the same Activity. Changing a target Property establishes
neither that target's response nor consent, perception, thought or volition.

`trait_change` likewise defaults to empty and contains at most 100 mixed typed
establish/develop items. Each affected Trait Entity must be the actor or an explicit
target, uniformly across Entity roles. It may coexist atomically with
`property_change`. Any invalid, duplicate, no-op, stale or unavailable Trait item
rejects the complete Interaction, including its outward Activity and Property
changes. A target Trait consequence is World state from the encounter, never the
target's authored response, thought, consent or volition.

One accepted call stores one Activity with one accountable actor, exact context
Place, canonical prose, the complete target set and zero or more exact Property and
Trait changes. The Place is also linked as
`location`. A target Character may later read that outward behavior and the same
canonical co-target participation, but gains no actor intent, thought, response,
consent or User-control provenance. A non-Character target gains no fictional
knowledge. A co-present Character that is neither actor nor target receives no
`submit_interaction` Activity automatically. Any response is a separately authored
later Activity with its own actor and confirmation.

Repeated accepted targeting remains possible; it triggers no target Agent,
background work or notification. Private attention controls must be designed before
notifications or broader Interaction reach.

## Input example

The accepted `submit_interaction` target accepts exactly:

```json
{
  "request_id": "a80e2bb4-07bd-40c9-894c-99b1e60fc48a",
  "expected_place_revision": "opaque-versioned-token",
  "prose": "Pip darts in three quick circles around Mara's feet.",
  "target_entity_id": [
    "9ef31b14-77e9-4ef1-b458-89726154065a"
  ],
  "property_change": [
    {
      "entity_id": "9ef31b14-77e9-4ef1-b458-89726154065a",
      "key": "dusty",
      "value": {"type": "text", "text": "yes"}
    }
  ],
  "trait_change": [
    {
      "type": "develop",
      "trait_id": "0889a741-3212-4a91-8a04-87f78ff11b44",
      "statement": "Waits for Mara's second footfall before darting closer."
    }
  ]
}
```

## Validation

Validation is specified in the contract above. Prose, Property and Trait values — constrained by [shared value validation](../domain.md#shared-value-validation), [Property](../model/property/README.md) and [Trait](../model/trait/README.md); request identity, expected Place revision and retry precedence — constrained by [delivery identity and exact-Place freshness](../protocol.md#delivery-identity-and-exact-place-freshness); this capability adds only the target and change rules stated in the contract.

## Result

The canonical result is the `AcceptedInteraction` described above.

## Activity footprint

One Activity stores actor, exact context, canonical prose, complete target set and exact submitted changes; the Place is `location`.

## Annotations and retry class

Modifying; idempotent only by request id and the normalized unordered target, Property and Trait sets — a retry with the same id and semantically identical input returns the stored result; delivery identity — constrained by [delivery identity and exact-Place freshness](../protocol.md#delivery-identity-and-exact-place-freshness).

## Evidence obligations

World, HTTP and MCP must expose this same semantic contract, strict schema, result and canonical errors.
