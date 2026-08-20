# `submit_discovery`

> **Role / side:** one capability contract / runtime side.
> **Authority:** the World-side contract of `submit_discovery`: one typed confirmed result, validation, atomic state, Activity footprint and retry class.
> **Excludes:** Agent wording — published as [its tool description](../../mcp/agent/tool/submit_discovery.md); discovery workshop conduct — defined in [Required investigation and discovery flow](../agent.md#required-investigation-and-discovery-flow); canonical errors — defined in [canonical errors](../protocol.md#canonical-errors).

## Purpose

After a positive investigation and explicit User confirmation, atomically establish
one Entity at the bound Position or one complete origin, destination and new
Connection package without moving the Character.

## Input

World call `submit_discovery(context.user_id, input)`; HTTP `POST /api/discovery`;
MCP `submit_discovery`.

## Contract

```rust
struct SubmitDiscovery {
    request_id: Uuid,
    attempt_id: Uuid,
    prose: String,
    result: DiscoveryResultInput,
}

enum DiscoveryResultInput {
    EntityAtPosition {
        name: String,
        description: String,
        position_description: Option<String>,
        property: Vec<PropertyInput>,
        trait_: Vec<TraitInput>,
    },
    ConnectedPlace {
        origin: DiscoveryOriginInput,
        destination: DiscoveryDestinationInput,
        connection: ConnectionInput,
    },
}

enum DiscoveryOriginInput {
    AttemptPlace,
    New { entity: PlaceEntityInput, position_description: Option<String> },
    Existing { place_id: EntityId },
}

enum DiscoveryDestinationInput {
    New { entity: PlaceEntityInput, position: DirectPositionInput },
    Existing { place_id: EntityId },
}

struct ConnectionInput {
    name: String,
    description: String,
    shape_description: Option<String>,
    allows_reverse: bool,
    course: Vec<ConnectionPointInput>,
}
```

`PlaceEntityInput` contains name, description and independent 0–100 initial
Properties and 0–100 initial Traits. `DirectPositionInput` contains exact `x_cm`,
`y_cm`, `z_cm` and optional Position description. Course is empty or contains 2–128
exact points in source-to-destination order.

`entity_at_position` creates one Entity at the attempt's exact Character Position.
When the attempt has a current Place, the Entity also receives that explicit current
Place relation. `connected_place` uses `attempt_place` only when the attempt stored a
current Place. At a loose Position it must explicitly create one origin Place at the
bound point or select one existing Place whose current Position equals that point;
coordinate equality never selects it automatically. Destination is one new Place at
the submitted point or one explicitly selected existing Place. Origin and
destination must differ.

Every accepted connected-Place result creates exactly one new immutable Connection
from origin to destination. A pre-existing Connection is reused through reads and a
later Movement; there is no no-op discovery input. Equal-looking independently
submitted Connections remain distinct.

## Input example

```json
{
  "request_id": "15594279-3498-493d-994a-d729b715dd28",
  "attempt_id": "27bb3450-4159-462f-bd9b-ce5617ceef21",
  "prose": "Beyond the dunes, Mara finds a bell tower above a salt meadow.",
  "result": {
    "type": "connected_place",
    "origin": {"type": "attempt_place"},
    "destination": {
      "type": "new",
      "entity": {
        "name": "Salt Bell Meadow",
        "description": "A wind-cut meadow below a solitary bell tower.",
        "property": [],
        "trait": []
      },
      "position": {"x_cm": 120000, "y_cm": 0, "z_cm": 0}
    },
    "connection": {
      "name": "Dune Bell Path",
      "description": "A direct path over pale dunes.",
      "allows_reverse": true,
      "course": []
    }
  }
}
```

## Validation

Strict decoding and complete normalization happen before locking. Under the User
lock, World resolves accepted Activity retry before current eligibility. The attempt
must be own, positive, unconsumed, unvoided, same kind and still match the exact
Character Position revision and nullable current Place stored at start. Every
well-formed foreign, zero, consumed, voided, wrong-kind or stale attempt shares
`discovery_attempt_unavailable`.

Entity, Place, Property, Trait, Position and Connection input retains its typed
canonical error. Existing Place selection is neutral when unavailable. World never
parses prose or content to choose coordinates, merge Places, infer a course or
deduplicate a Connection.

## Result

`entity_at_position` returns `{ type, activity, entity, position, place }`, where
Place is nullable. `connected_place` returns `{ type, activity, origin, destination,
connection, character }`. New Entity/Place state, Position versions, explicit local
relations, Connection, Activity, typed history and attempt consumption commit once.
If a loose origin is created or selected, Character Position is unchanged and that
origin becomes current. Discovery never moves the Character.

## Activity footprint

One immutable `submit_discovery` Activity stores the acting Character, canonical
prose, exact Entity roles, bound Character Position as `origin`, every newly created
Position as `result` and the new Connection when present. Newly created subjects use
`subject`; origin uses `location`; destination uses `destination`. When a new origin
would make immediate context cyclic, Activity context Place remains absent while the
typed Position and Entity roles retain exact history.

## Annotations and retry class

Modifying and irreversible World history; idempotent by Activity request id and
complete normalized input. Same id and content returns the canonical accepted result
before later eligibility checks. Changed content or reuse across another Activity
operation returns `discovery_request_conflict`. The call triggers no Agent,
notification or background process.

## Evidence obligations

World, HTTP and MCP must prove both result kinds, every origin/destination variant,
fresh grounding, strict errors, retry reconstruction, rollback and two concurrent
equal-looking Connections that both commit. Independent readback must show Discovery
never changed Character Position.
