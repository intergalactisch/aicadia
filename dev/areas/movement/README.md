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

- `enter_world` remains the Character's first spatial introduction after creation
  and establishes its first Position plus Activity from one User-selected option:
  three explained existing Places or one choice for Position with no current Place.
  It is not a participation flag and the loose result creates no Place. World chooses
  and stores that exact loose Position once, broadly around one of those Places;
  retry never rerolls it and World neither checks nor guarantees that it is unoccupied.
- Arrival performs no automatic nearby read, Observation or discovery. The User must
  explicitly invoke the Agent for the next look-around or exploration step.
- That explicit exploration step is grounded in the Character's exact Position even
  when it has no current Place. Movement may therefore stop anywhere and leave the
  Character fully able to continue discovery without establishing a dummy Place.
- The Agent chooses dynamically among the MCP tools available after Movement; World
  imposes no look-around/discovery sequence and stores no exploration workflow.
- When Discovery establishes an origin Place at a loose Character Position, the
  Character remains at the same coordinates and that Place becomes current explicitly.
  The origin Place stays behind when later Movement changes the Character Position.
- Movement must preserve the stable identity of every moved Character, Entity and Place.
- A move is explicit and deterministically validated by World; prose never implies a route or destination.
- Accepted movement changes current Position and writes attributable Activity atomically.
- Retry and reconnect behavior reconstruct accepted truth from World rather than connection-local state.
- Movement correctness is scoped to exact affected subjects, never a global World revision.
- The first selected movement scene moves a Character from its current Place to a
  newly discovered or established second Place through explicit connectedness.
- Discovering that destination never moves the Character; entering it is a later,
  deliberate and separately settled player action.
- The preceding discovery has already committed B, its direct Position, authored
  initial Entity state and A→B Connection atomically. Movement consumes none of
  that discovery package and changes only the separately confirmed walking state
  and Activity.
- For this scene B's committed Position is an absolute World point authored as
  structured Agent input, not a value inferred from its prose or a persistent
  offset from A.
- The discovery may alternatively have reused an already established destination
  C and committed only the explicit allowed A→C Connection and Activity. Later
  Movement uses that Connection in exactly the same way; C's proximity did not
  create it automatically.
- The first slice has at most one required A→C travel direction. A concurrent or
  pre-existing direction is reused for later Movement rather than duplicated by
  Discovery.
- Movement follows only a direction explicitly allowed by the connection; World
  never assumes that a traversable return direction also exists.
- The first movement capability is walking from Place A to Place B over one explicit
  A→B Connection. Because no second movement method exists in that slice, the
  Connection receives no travel-mode field or enum.
- That Connection proves direct travel for the accepted Movement action; it does
  not assert that A and B are geographically adjacent. Adjacency is not required by
  the first movement scene.
- Discovery may establish that Connection between Places at any technically valid
  distance. Distance alone neither rejects the Connection nor selects duration,
  intermediate Positions or another Movement rule; those remain separate choices.
- The first spatial slice stores no Route concept; an Agent may choose one or more
  Connections as a temporary travel plan without creating further World state.
- A Character must eventually be able to retain an unnamed Position between
  established Places, stop there and participate in World play without that Position
  first becoming another Place.
- Position is the canonical name for an Entity's optional exact current point;
  `Transform` and `spatial placement` are not domain terms.
- Every exact Position point has three spatial dimensions. Movement may therefore
  change height as well as the other two spatial values without adding geometry,
  Orientation or a separate vertical-location concept.
- Movement changes those values in exact whole centimetres. Conversational metres
  or kilometres are Agent presentation; World receives no unit text and supports no
  sub-centimetre Position in this direction.
- Those values are `x`, `y` and `z`; Movement changes horizontal `x`/`y` and vertical
  height `z` explicitly. It never derives an axis from movement prose.
- Increasing `x` moves east, increasing `y` north and increasing `z` up. The Agent
  performs that translation before submitting an exact Movement proposal.
- Absolute Movement uses permanent World origin `(0, 0, 0)`. The origin never moves
  and is not an affected subject, row or lock in a Movement transaction.
- Position is separate optional state keyed by the Entity's existing identity and
  has no independent id. Moving an Entity changes that state rather than duplicating
  coordinates across Entity roles such as Place.
- Position is either absolute from the permanent World origin or relative to exactly
  one other Entity. An Entity-relative Position uses the same World axes and its
  resolved current point mechanically follows its reference Entity's resolved
  Position; free wording and Relations never cause that movement.
- Relative offsets start at the reference Entity's one Position point. Movement does
  not infer or maintain a distance from a named part or surface; the Agent must submit
  the exact offsets it intends from the structured state it received.
- Moving one reference Entity changes one canonical Position. Relative Entities do
  not each require a Position rewrite merely to follow it, although bounded
  resolution, conflicts and indexing for moving references remain to be proved.
- Optional Position `description` may help an Agent narrate movement or a strange
  arrangement, but Movement is never inferred from it. Every accepted Position
  change explicitly keeps, replaces or removes the description under the same
  Position revision and transaction.
- Position description is returned only as part of the complete Position; a
  Movement capability never reads or changes it as an independent fact.
- Knowing or remembering an Entity does not create a direct current-Position lookup.
  A Movement proposal may rely on Position only when a concrete bounded read returns
  that Entity and its Position; remembered coordinates remain Knowledge rather than
  fresh movement state.
- A bounded read of the acting Character returns its complete Position when one
  exists. The spatial foundation has no own-Position exception, denial or recovery
  path and never reconstructs a current point from memory or prose.
- One confirmed action may explicitly move several affected Entities, including an
  interacted-with object and the acting Character, when the Agent proposes that
  complete consequence; World does not invent the additional movement from prose.
- Moving with another Entity is an explicit behavior, never an automatic consequence
  of a generic Containment concept or of Relation wording such as `inside`.
- Connection topology may deliberately contain loops or join geographically
  inconsistent Places for impossible architecture and other playful spatial
  experiences. Movement still follows one explicit allowed direction per step;
  Position references themselves never cycle.
- `Connection` is the canonical dedicated Place-to-Place topology primitive. It is
  neither `Link` nor a type of open Agent-authored `Relation`.
- One Connection identifies one persistent direct travel alternative. Several may
  join the same endpoint Places, so Movement selects the exact Connection rather
  than only naming a destination.
- A Connection's own Agent-authored name and description let the Agent present and
  choose among those alternatives. World never converts that text into direction,
  access, travel method, cost or spatial shape.
- A Connection may have one optional reusable exact spatial shape for that
  alternative; a portal may have none. Movement uses but does not restate that
  shape, and no completed-Movement Position trace is stored before travel-over-time
  gameplay requires one.
- The optional shape is a bounded ordered sequence of exact whole-centimetre
  `x`/`y`/`z` World points connected by straight segments. It may carry one optional
  Agent-authored description, but neither the points nor text claim travel width or
  one actual Character trace.
- Area is exact positive coverage. Intersecting the Connection points with current
  Areas can therefore prove ordered covered portions, while every remaining portion
  stays unknown rather than being reported as definitely outside all Places.
- One Movement may traverse all or part of one selected Connection. For an ordinary
  shaped Connection, the Agent names its expected revision, the Character's expected
  Position revision, one segment and one exact whole-centimetre target point on it;
  World validates membership and allowed direction rather than interpreting prose.
- A partial Movement writes the Character's ordinary exact Position and Activity in
  one transaction. It creates no journey, progress, timer or background state; a
  later step revalidates that current Position against its explicitly named segment.
- A Character may therefore stop, meet, discover or act at an unnamed intermediate
  Position. Leaving the Connection is a separate explicit Movement, while a portal
  or another Connection without spatial shape permits only direct endpoint travel.
- At arrival the Agent supplies one complete resulting Position and may deliberately
  make it relative to the destination Place. World never silently rebases the
  Character or invents offsets, and no completed travel trace is stored.
- Concurrent travellers read the same Connection revision but update their own
  Position rows. They do not lock one another, either endpoint Place or the whole
  course; a concurrent Connection edit makes only proposals depending on its old
  revision stale.

### Rejected

- Treating a disconnect, Agent restart or missed notification as movement.
- Inferring reachability from names, descriptions, Position, Area overlap or model judgment.
- Treating a Connection as proof that its Places are geographically adjacent.
- Interpreting a Place description or Property such as grass or dunes as movement
  cost, access or another mechanical travel rule.
- Making a Connection or coordinate frame into an Entity.
- Locking an entire Place graph or the whole World for one move.
- Combining movement, observation and discovery into one opaque operation.
- Treating every persistent stopping point as a new Place.
- Requiring a durable journey or server timer merely because a Movement stops before
  the destination, or forcing every Connection Movement to arrive immediately.

### Not yet chosen

- How three existing Places become eligible without one hot global entry row and what
  happens before three eligible Places exist.
- Whether an open-terrain Connection may exist without a physical Entity and who may establish it.
- Which exact state lets an Agent say what terrain, Places or Areas a Character
  crosses while traversing one Connection. Endpoint topology alone supplies no
  path, crossing order, intermediate Position or landscape context.
- How current Place Areas are intersected and ordered against the Connection's
  points with bounded pagination and exact input revisions, and how endpoint Position
  changes affect the stored course.
- Which later gameplay, if any, earns multiple travel methods or an explicit
  adjacency mechanic; neither is part of the first walking slice.
- Which later gameplay makes terrain mechanically affect movement rather than remain
  ordinary authored Place state.
- How ordinary movement over that spatial ground differs from an explicit Connection.
- The exact later lifecycle of named, saved or shared Routes such as a Character choosing the Green Route.
- Rules for carried or jointly moved Entities.
- How Position-reference cycles are rejected with bounded work and how an exact
  current point is read without updating every related Entity.
- How changing a Position's reference settles under concurrent movement.
- Conflict behavior when origin, destination or a moving subject changes concurrently.
- Which grounded actions remain possible while a Character cannot read its own
  Position, and how it can recover without a privileged bypass.

## Research needed

- Design the smallest move transaction and retry contract for one hot origin or destination.
- Test nested relative Position without unbounded traversal or duplicated truth.
- Compare explicit directed connections with other minimal topology against a concrete scene.
- Test whole-centimetre Position range and indexing under sparse exploration,
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
must also support exact whole-centimetre Position between Places and an explicit
one-Entity reference when the mover must follow another Entity. World validates
authority and exact affected subjects, changes Position and writes history in one
bounded transaction.

### Absent

General movement, general Position, Connections, routes, additional Places, travel
intervals, pathfinding, carrying, moving containers, background travel and movement
subscriptions are absent from the current game contract.

## Sources

- Prepared pressure — [Spatial scenario catalogue](../place/scenarios.md).
- Retained rationale — [spatial direction](../../docs/concept/spatial.md).
- Sourced findings — [spatial multiplayer foundation](../../docs/research/spatial-multiplayer-foundation.md).
- Sourced extent/traversal comparison — [Place extent, spatial inclusion and Connection traversal](../../docs/research/place-area-connection-traversal.md).
- Current technical candidate — the [completed spatial technical synthesis](../../docs/concept/spatial-five-year-backcast.md#technical-synthesis-after-the-completed-grill) translates the chosen Movement direction into a falsifiable PostgreSQL and World shape without authorizing implementation.
- Related synthesis — [Place](../place/README.md), [Multiplayer](../multiplayer/README.md) and [World Change](../world-change/README.md).
- Exact behavior and delivery — [`game/docs/`](../../../game/docs/README.md) and [`dev/docs/evidence/`](../../docs/evidence/README.md).
