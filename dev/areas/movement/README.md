# Movement

> **Role / side:** current Movement development synthesis / development side.
> **Authority:** owns the current meaning, boundary, decisions, unresolved landscape, components and directional technical model for Movement.
> **Excludes:** selected work, exact game behavior, Place structure, sourced findings and delivery claims; those remain in `dev/backlog/` and plans, `game/docs/`, the [Place Area](../place/README.md), `dev/docs/research/` and `dev/docs/evidence/`.

## Meaning

Movement concerns an explicit World transition that changes a Character or Entity's
Position. It composes stable Place identity, an allowed Connection when required,
authority, concurrent freshness and atomic history so a retry or observer can
reconstruct one accepted move.

## Boundary

### This is

- A deliberate transition between explicit spatial states.
- Validation of actor, origin, destination, authority, freshness and bounded consequences.
- Atomic current Position and historical footprint for the subjects involved.
- A shared concern with Place, Multiplayer, Discovery and World Change.

### This is not

- Discovery itself, although movement can create a new opportunity to discover.
- Teleportation inferred from prose, reconnect, client location or server guesswork.
- A background simulation that advances Characters while their Agents are offline.
- A universal geometry or pathfinding engine introduced before a scene requires it.

## Decisions

### Chosen

- Movement must preserve the stable identity of every moved Character, Entity and Place.
- A move is explicit and deterministically validated by World; prose never implies a route or destination.
- Accepted movement changes current Position and writes attributable Activity atomically.
- Retry and reconnect behavior reconstruct accepted truth from World rather than connection-local state.
- Movement correctness is scoped to exact affected subjects, never a global World revision.
- The first selected movement scene moves a Character from its current Place to a
  newly discovered or established second Place through explicit connectedness.
- Discovering that destination never moves the Character; entering it is a later,
  deliberate and separately settled player action.
- Movement follows only a direction explicitly allowed by the connection; World
  never assumes that a traversable return direction also exists.
- The first spatial slice stores no Route concept; an Agent may choose one or more
  Connections as a temporary travel plan without creating further World state.
- A Character must eventually be able to retain an unnamed Position between
  established Places, stop there and participate in World play without that Position
  first becoming another Place.
- Position is the canonical name for an Entity's optional exact current point;
  `Transform` and `spatial placement` are not domain terms.
- Position is separate optional state keyed by the Entity's existing identity and
  has no independent id. Moving an Entity changes that state rather than duplicating
  coordinates across Entity roles such as Place.
- A precise arrangement relative to another Entity may either describe only the
  current state or persist when that other Entity moves. Both are required, and the
  Agent must explicitly choose; World never infers movement from free wording.
- One confirmed action may explicitly move several affected Entities, including an
  interacted-with object and the acting Character, when the Agent proposes that
  complete consequence; World does not invent the additional movement from prose.
- Moving with another Entity is an explicit behavior, never an automatic consequence
  of a generic Containment concept or of Relation wording such as `inside`.

### Rejected

- Treating a disconnect, Agent restart or missed notification as movement.
- Inferring reachability from names, descriptions, Position, Area overlap or model judgment.
- Making a Connection or coordinate frame into an Entity.
- Locking an entire Place graph or the whole World for one move.
- Combining movement, observation and discovery into one opaque operation.
- Treating every persistent stopping point as a new Place.

### Not yet chosen

- The single-word canonical name and exact scope of the direct-topology primitive;
  `Connection` remains the working term and two-word names are rejected.
- Whether an open-terrain Connection may exist without a physical Entity and who may establish it.
- Whether Position uses discrete neighboring cells, continuous coordinates or
  another deterministic spatial form.
- Whether Position itself records an independent-versus-relative choice, whether a
  relative Position also causes movement with another Entity, and how that state is
  versioned and settled under concurrent movement.
- How ordinary movement over that spatial ground differs from an explicit Connection.
- The exact later lifecycle of named, saved or shared Routes such as a Character choosing the Green Route.
- Whether a first move is immediate or has an explicit duration or interval.
- Rules for carried or jointly moved Entities.
- How deeply any relative spatial state may be chained, how cycles are rejected and
  how an exact current point is read without updating every related Entity.
- Conflict behavior when origin, destination or a moving subject changes concurrently.

## Research needed

- Design the smallest move transaction and retry contract for one hot origin or destination.
- Test nested relative Position without unbounded traversal or duplicated truth.
- Compare explicit directed connections with other minimal topology against a concrete scene.
- Compare discrete and continuous Position under sparse exploration,
  concurrent co-location and arbitrary Entity placement.
- Establish bounded history and observation semantics during concurrent departure and arrival.

## Components

| Component | Current meaning |
| --- | --- |
| Mover | The Character or Entity whose Position changes. |
| Origin | The fresh explicit spatial state from which movement is attempted. |
| Destination | The established Place or Position the move would enter. |
| Traversal basis | The typed structural fact that can authorize reachability when required; no Route concept is stored. |
| Authority | The permission and control facts World can validate deterministically. |
| Transition | One atomic accepted change plus its attributable Activity. |
| Consequence scope | Exact other subjects whose state must participate in the same transaction. |
| Scenarios | The movement and progress cases in the [spatial scenario catalogue](../place/scenarios.md). |

## Technical model

### Delivered

`enter_world` can place an unplaced Character into the one entry Place and writes
the corresponding Activity. The current contract provides no subsequent movement,
additional Place or route capability. Exact behavior remains in
[`game/docs/`](../../../game/docs/README.md).

### Directional

A future move names one mover, fresh origin and explicit destination. It uses an
explicit Connection where structural travel requires one, while ordinary movement
must also support Position over the still-unchosen exact spatial ground between Places. World
validates authority and exact affected subjects, changes Position and writes
history in one bounded transaction.

### Absent

General movement, general Position, Connections, routes, additional Places, travel
intervals, pathfinding, carrying, moving containers, background travel and movement
subscriptions are absent from the current game contract.

## Sources

- Prepared pressure — [Spatial scenario catalogue](../place/scenarios.md).
- Retained rationale — [spatial direction](../../docs/concept/spatial.md).
- Sourced findings — [spatial multiplayer foundation](../../docs/research/spatial-multiplayer-foundation.md).
- Related synthesis — [Place](../place/README.md), [Multiplayer](../multiplayer/README.md) and [World Change](../world-change/README.md).
- Exact behavior and delivery — [`game/docs/`](../../../game/docs/README.md) and [`dev/docs/evidence/`](../../docs/evidence/README.md).
