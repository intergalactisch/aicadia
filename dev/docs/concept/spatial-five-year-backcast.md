---
status: active
---

# Five-year spatial backcast

> **Role / side:** live five-year spatial recommendation / development side.
> **Authority:** owns the future observation, recommended direction, scenario application, technical candidate, rejected alternatives, falsifiers and backward roadmap produced by the 2026-08-18 `5jaar` exercise.
> **Excludes:** accepted Aicadia vocabulary, current Area truth, current game behavior, production schema, API and implementation; those require later User decisions and their own authorities.

Date: 2026-08-18

Status: **complete recommendation, pending User decision**. Nothing in this record is
accepted merely because the recommendation is complete. `Relation`, `coordinate
space`, `Orientation`, `Activation` and every illustrated field or table below are
explicitly proposed or descriptive unless already defined in `dev/CONTEXT.md`.

The primary-source basis is the
[spatial foundation research report](../research/spatial-five-year-foundation.md).
The fixed tests are the thirteen
[spatial scenarios](../../areas/place/scenarios.md). The earlier
[four-candidate paper comparison](../../lab/spatial/01-model-pressure/README.md)
supplied a leading direction; this backcast tried to break it rather than treating
it as decided.

## Executive recommendation

Five years from now, Aicadia should **not** have one universal spatial graph, one
giant map of Places or one flexible Position payload containing coordinates, prose,
ownership and travel rules. It should have a small exact spatial core with other
truths beside it.

The recommended foundation is:

1. **Entity remains the one stable identity of a World subject.** A city, ship,
   Character, tree, bridge and cup can be Entities. A coordinate basis, Position,
   Connection and proposed Relation remain records, not Entities.
2. **Every positioned Entity has exactly one current Position keyed by its Entity
   identity.** Position is an exact point plus the reference in which that point is
   expressed. It has no independent identity.
3. **That reference is either the World or exactly one Entity.** A World-referenced
   cup remains behind when a table moves. A table-referenced cup mechanically moves
   with the table. The Agent chooses this explicitly; prose never chooses it.
4. **World resolves one current World point on every eligible Position read.** The
   resolved point is calculated from a bounded acyclic reference chain and is not a
   second stored Position. Hidden or unpositioned subjects remain unavailable.
5. **Exact Position and current Place participation stay distinct.** Entering Place
   B establishes that Place as the Character's local play context as well as changing
   Position; merely sharing B's coordinate does not. Unnamed ground needs Position
   without a current Place. The existing `current_place` behavior is the first
   concrete example; the generalized storage remains proposed.
6. **Open Agent-authored spatial meaning lives separately.** The proposed Relation
   record can say “under”, “inside”, “near” or “floating two centimetres above” with
   bounded Properties and provenance, without a server enum and without mechanical
   power. It may name exact Position revisions on which the statement depends.
7. **Mechanics keep their own small truths.** Area, Connection, private contents,
   access, remote activation and a later Route do not acquire behavior from a free
   Relation. They are introduced only when actual gameplay requires deterministic
   rules.
8. **Visibility is decided before retrieval, not after spatial matching.** Knowing
   a public Character, coat or Position never reveals a hidden sword or Relation.
   A guessed id, count, cursor, timing difference or error cannot bypass this.
9. **Canonical truth stays small; indexes are disposable.** Operational cells,
   resolved-position projections, carrier envelopes, PostGIS indexes and interest
   lists may accelerate bounded reads but never become Place identity, discovery or
   authority.
10. **Mutations lock and version the smallest exact facts.** One carrier move changes
   one Position rather than every descendant. Local interior Actions do not depend
   on the carrier's external Position unless their outcome actually crosses that
   boundary. Every accepted package and its Activity commit atomically.

In plain game terms: the system remembers exactly where something is when that is
known, remembers what it moves with when that is deliberately chosen, permits Agents
to describe stranger relationships freely, and refuses to let those descriptions
quietly become physics, access or causality.

This is the strongest long-term answer because it spends complexity only where
different behavior has already proved that the truths differ. It is flexible at the
Agent layer, strict at the World layer and bounded at the database layer.

## The future World that this must support

### Ordinary play after five years

An explorer leaves Moss City, crosses unnamed ground, pauses at a forest edge and
later approaches an old hotel. The Character can occupy every intermediate point
without the server minting a Place at each stop. The Agent can ask for straight-line
distance, known direct travel options or a calculated journey and receives three
honestly different answers.

Another player has built a village table with a birdhouse and a floating cup. The
birdhouse is established relative to the table and therefore travels when the table
is moved. The cup may instead be World-referenced and remain floating in place when
the table is dragged away. An Agent may then pick up the cup and explicitly move the
cup and Character into a new bizarre arrangement. World validates the exact submitted
subjects and Positions but never repairs the scene toward ordinary physics.

A ship crosses the ocean while a cabin remains a discovered Place, passengers act,
cargo changes and private items stay hidden. Moving the ship changes one current
Position. Interior Actions that only affect cabin-local subjects do not all conflict
with that external move. A Character looking from shore can receive an exact current
point only if the relevant Position is visible and resolvable.

A Character presses a button while its bomb is miles away. Spatial state says where
both Entities and Activities are. A separate explicit remote mechanic says whether
this button invocation can affect that bomb. A descriptive Relation may explain the
connection to an Agent, but the Relation never grants the effect.

Cartographers publish named journeys such as the Green Route. A durable Route exists
only because named, saved and shared travel has become gameplay. Ordinary paths are
still calculated from direct Connections and current constraints; they are not
stored merely because one Agent asked for directions.

### Culture and fun that emerged

- Players deliberately create moving villages, hidden rooms, travelling markets,
  impossible towers and local customs about how objects are arranged.
- Agents can retain multiple descriptions of one scene without creating multiple
  mechanical Positions. A factual position and a poetic or surprising spatial
  statement can coexist.
- Cartographers, builders and explorers establish useful Places and Areas; the
  World remains sparse rather than revealing a server-authored complete geography.
- Strange results remain playable because World checks structure instead of
  enforcing an ontology of realism. The floating Character is a valid outcome if
  the Agent submitted the bounded affected state and was eligible to change it.
- Moving Places retain identity and history. A cabin does not become a new cabin
  every time its ship moves, and a city does not derive identity from coordinates.

### Abuse that also emerged

- Attackers try to create cycles and hundred-thousand-deep relative Position chains.
- Bots request every Entity near a festival containing a million Characters, or use
  exact counts and pagination gaps to infer hidden attendees.
- Griefers repeatedly move a hot carrier or public object, race another Agent's
  move, submit enormous Areas and create wide topology meant to exhaust traversal.
- An Agent guesses a remembered sword id and probes Position, Relation, timing and
  error differences after the sword becomes private.
- A remote Action tries to turn free prose, a Property value or mere spatial
  proximity into authority to change thousands of distant subjects.
- Very large coordinates, mixed units, stale projections and 2D/3D disagreements
  try to turn an apparently exact “two centimetres” into contradictory results.

### Failures the mature system survives

- PostgreSQL fails over between a read and retry. Request identity returns the same
  accepted result or safely revalidates; it never performs the move twice.
- A resolved-position projection is missing or stale. It can reduce performance or
  make a bounded read fail closed, but it cannot authorize a mutation or overwrite
  canonical Position.
- One Place or carrier is saturated. Its exact conflicting changes are admitted,
  delayed or rejected within explicit bounds while quiet subjects continue on other
  rows and indexes.
- A subscription or interest hint is lost. The next authoritative bounded read
  recovers current state; the hint was never World truth and never invoked an Agent.
- An Area or route-planning query reaches its vertex, depth, candidate, time or
  result bound. World returns the defined bounded outcome rather than continuing an
  unbounded search.

## What survived the backcast

The future did not prove that every subsystem below must exist now. It revealed the
boundaries whose removal creates irreversible confusion.

| Surviving truth | Player consequence | Technical consequence |
| --- | --- | --- |
| Stable Entity identity | Moving or redrawing something never renames it | ids are independent of coordinates, cells, parents and partitions |
| One exact Position | one subject never has competing mechanical locations | zero or one current Position keyed by `entity_id` |
| Explicit reference | “stays here” and “moves with that” are deliberate choices | World- or one-Entity-relative variant; no prose inference |
| Derived resolved point | Agent gets one current answer without descendant writes | bounded chain calculation, not second canonical storage |
| Explicit current Place | entering is not guessed from equal coordinates or Area overlap | optional typed Place participation, separate from exact Position |
| Open authored meaning | Agents can express surprising spatial facts | free text/Properties with provenance, dependencies and no execution |
| Typed mechanics | travel, privacy and remote effects remain predictable | separate invariants, indexes, access and transactions when earned |
| Bounded visibility | private arrangements remain unknowable | eligibility filters precede selection, count, hydration and errors |
| Replaceable query acceleration | maps and nearby play remain fast without becoming lore | derived cells, GiST, caches and interest can be rebuilt |
| Exact mutation dependencies | unrelated play continues under load | per-subject/fact versions, stable locks, idempotency and Activity |

## Recommended spatial model

### 1. Entity owns identity; spatial records own no fictional identity

Entity remains broad: a tree, Character, city, forest, waterfall, table, road, door,
bridge, ship or cabin may qualify when it is an independently addressable World
subject with state or history. Place remains an Entity role. A city is normally an
Entity with Place and perhaps Area; a named forest may be the same. A waterfall is
an Entity and receives Place only when World establishes it as a map, discovery,
navigation or spatial-reference subject.

Position, Area and Connection are structure about those subjects. They do not become
Entities merely because they have database rows or history. The recommended proposed
Relation and a later Route likewise have their own record identity when their
lifecycle needs one, but are not fictional World subjects and never borrow an Entity
identity.

This preserves the user's object intuition without making “Entity” literally mean
physical object: the test is durable independent subjecthood, not materiality.

### 2. Position means an exact point and the reference that makes it exact

The key clarification is that Position is not “three numbers somewhere.” Its full
mechanical meaning is:

> **one exact current point of one Entity, expressed either in the World's
> coordinate basis or in exactly one other Entity's local coordinate basis.**

The reference is part of the Position because identical numbers mean different
points in different coordinate bases. It is not a Relation and it does not need a
second Position identity.

- **World-referenced:** moving another Entity changes nothing. This is the cup that
  should remain floating when the table moves.
- **Entity-referenced:** moving the referenced Entity changes the resolved point.
  This is the birdhouse that should travel with the table or the cabin that should
  travel with the ship.
- **Re-reference preserving the World point:** picking an Entity up can change its
  reference while keeping its current resolved point.
- **Re-reference preserving the local point:** attaching an Entity to a new carrier
  can retain the submitted offset and therefore change its resolved World point.

The Agent must choose the re-reference behavior explicitly. World never derives it
from “on”, “inside”, “held by” or another free description.

One immediate reference is enough. Chains arise compositionally: Birdhouse can be
relative to Table, Table to Cabin and Cabin to Ship. The recommendation uses a hard
small maximum and rejects cycles. **Eight references is the proposed initial bound**:
large enough for the fixed scenarios and small enough to inspect, lock and resolve.
That number remains a candidate until a real fixture tests it.

### 3. Relative Position deliberately means movement inheritance

The earlier exploration separated a relative coordinate from the rule that it moves
with its reference. The source backcast resolves that ambiguity in favor of one
meaning: if Entity-relative Position did not inherit movement, it would be only a
temporary alternate notation and would go stale after every reference move. A
separate generic follow rule would duplicate the same decision and force additional
reads and writes.

Therefore the recommendation is explicit:

- Entity-relative Position mechanically follows its reference.
- World-relative Position does not.
- Neither form implies `inside`, ownership, visibility, access, inventory membership
  or physical support.
- A persistent distance or surface constraint that must react to geometry changes
  would be a later earned mechanic, not ordinary Position.

Moving one reference changes the resolved points of its descendants without writing
new descendant Position versions. History records that the reference moved; it does
not fabricate a separate movement Activity for every descendant.

### 4. One authorized read returns one resolved current point

An Agent should not have to understand storage joins or calculate a carrier chain.
When the Character is eligible to know a positioned Entity's current Position, one
World read returns its current resolved World point and a freshness token. Internally
World resolves at most the accepted depth from one request snapshot.

That promise has three deliberate limits:

1. an unpositioned Entity has no point and World says so only when the Character is
   eligible to know the Entity and that absence;
2. a hidden Entity or Position is unavailable without confirming its existence; and
3. a calculated point is fresh for the named Position/reference revisions, not an
   eternal coordinate that remains valid after an ancestor moves.

The response may also expose the immediate relative reference when that fact is
visible and useful. The resolved point can be returned without exposing a private
ancestor's identity when the access design eventually permits that combination.
Eligibility, not technical resolvability, decides the output.

### 5. Exact Position and current Place are separate

Position answers “what exact point does this Entity occupy?” It does not by itself
answer “which established Place currently owns the local play context?” Those facts
already differ in the first chosen scene: discovering Place B does not enter it, and
standing at B's exact coordinate need not imply entry.

The recommendation therefore retains an optional explicit current-Place association
beside Position. The existing `character.current_place` and exact-Place Entity
placement are concrete predecessors; **Place presence** is only a proposed
descriptive label for a future uniform model.

- Entering B updates Character Position and current Place atomically.
- Walking onto unnamed heath updates Position and can clear current Place.
- A cabin passenger can have Cabin as current Place while exact Position resolves
  through Cabin and Ship.
- A table can carry its local contents through Position references without copying
  Village current Place to every descendant. A bounded reference/root-local read can
  resolve their effective local context.
- Area overlap never silently establishes current Place; a movement capability must
  explicitly decide when entry or exit occurs.

This extra fact is not generic Containment. It has the narrow game meaning needed by
Place-scoped reading, discovery, Interaction and history. It does not mean ownership,
visibility or geometric inside. Long-term work may converge the current Character
and ordinary-Entity placement rows behind one model, but this backcast does not
authorize deleting or migrating either current table.

### 6. Position remains a point; proposed Orientation and geometry remain separate

The current Position definition deliberately excluded engine-style Transform. That
is still right. Rotation and size should not silently arrive with every point.

The five-year system will eventually need rotating ships, tables and doors. The
recommended extension is a separately negotiated optional **Orientation** for the
few Entities whose local axes matter. A relative point then composes translation
and Orientation. Scale is not inherited by default; changing an Entity's geometry
does not silently rescale every child Position.

Until Orientation is earned, relative coordinates use the World's axes and inherit
translation only. This limitation must be explicit rather than hidden behind a
premature Transform abstraction.

Geometry likewise remains optional versioned state. It can give a table surface, a
bridge volume or a Place Area an exact shape. Every accepted geometry declares its
coordinate basis, dimension and bounds. World uses only named predicates and exact
versions; it never interprets Entity names or Relation prose as geometry.

### 7. Proposed coordinate representation

The long-term representation must make “two centimetres” reproducible far from the
origin. The recommended starting candidate is signed 64-bit fixed-point integers in
millimetres for each axis:

```text
x_mm: signed 64-bit integer
y_mm: signed 64-bit integer
z_mm: signed 64-bit integer
```

This gives exact centimetre and millimetre offsets, deterministic JSON/database
round trips and vastly more extent than a planetary game requires. It also avoids
making floating-point equality part of World correctness. Before acceptance it must
be compared in a real precision fixture against double precision and cell-plus-local
coordinates at the largest accepted extent.

The World-referenced variant also names one proposed non-Entity **coordinate space**
whose axes, units and dimension are fixed. The first implementation should have one
implicit World space rather than a table full of unused spaces. A second disconnected
space is introduced only when actual gameplay requires a cave, interior or world
whose coordinates are not comparable. Coordinate spaces never become Entities or
Places.

Operational grid cells remain derived. They may use a different resolution or be
replaced without moving, renaming or rediscovering any Entity.

### 8. Place, Area, Connection and later Route

**Place.** A Place uses its Entity's Position. It receives no `x`, `y` or `z`
columns of its own. A Place can be Entity-relative: a cabin remains a stable Place
while its ship moves. Place answers whether a subject is a durable independent map,
discovery, navigation or spatial reference—not whether it is large or immobile.

**Area.** Area remains optional coverage of a Place. The recommendation defaults to
positive established coverage: geometry says where the Place is established to
cover, while absence outside the geometry does not reveal a complete hidden World.
A complete boundary must be asserted explicitly and versioned. Overlap does not
create Connection, visibility, ownership or movement. Moving Places may keep Area
geometry in their local basis so the geometry follows without rewriting every point.

**Connection.** Keep the existing one-word term for direct Place topology unless a
later negotiation finds a materially clearer word. Connection records only that one
or both explicit travel directions exist between two Places. A physical road, door,
bridge or ferry remains an Entity and may be a dependency of the Connection. Its
name, geometry or Property never silently creates topology or current access.

**Route.** Keep Route absent now. If named, saved or shared journeys become current
gameplay, a Route is an ordered sequence of Connection identities with its own
name, description, visibility and history. It is not an Entity and does not replace
the graph. An ordinary calculated path is output, not durable Route state.

### 9. Proposed Relation: free meaning without secret mechanics

The best answer to the user's Relation concern is neither “no Relations” nor “put
all mechanics in a generic Relation table.” It is one open authored layer beside
the exact mechanics.

The proposed Relation record can contain:

- its own stable record id, not an Entity id;
- one source Entity and one target Entity for the first bounded form;
- a short Agent-authored name and description in English;
- optional bounded typed Properties such as distance `2` and unit `centimetre`;
- the acting Character and Activity that established or changed it;
- its visibility rule; and
- optional exact dependency revisions, such as the dog and bridge Positions from
  which the statement was authored.

It deliberately has **no server-owned semantic enum** for `above`, `under`, `inside`
or future words. World validates lengths, endpoint eligibility, Properties,
dependencies, visibility, revisions and bounds. It does not understand the name or
description and never executes them.

Dependency revisions solve an important freshness problem without semantic
inference. An Agent can establish “Dog sleeps under Bridge” as current only while
the named dog and bridge Position revisions remain current. If either moves, the
statement ceases to qualify as fresh without World pretending to understand
`under`, rewriting every dependent Relation or deleting its history. An Agent may
omit those dependencies when the statement is intentionally independent or
historical.

Relations are shared World state, not permanently owned by their first author. An
eligible Agent may change one. Eligibility can still be private: the existence,
endpoints and Properties of a Relation concerning a hidden sword remain unavailable
to another Character. Authorship remains history, not exclusive edit control.

### 10. Mechanical facts stay typed and earn their place

The open Relation layer never authorizes an Action. Deterministic mechanics receive
their own exact models only when accepted gameplay requires them:

| Scene | Open meaning may say | Mechanical truth, only when earned |
| --- | --- | --- |
| cup/table | `floating above` | Position; optional later surface constraint |
| dog/bridge | `sleeping under` | Positions and optional exact geometry predicate |
| sword/backpack | `inside` | private Position; later Inventory membership/access if listing and removal need it |
| Character/glass | `holding` | relative Position; later hand/equipment rule if slots and release need enforcement |
| button/bomb | `connected to` | proposed typed Activation record and bounded remote Action |
| A/B Places | `a road leads there` | Connection with explicit direction; access remains separate |

There is no generic Containment mechanic. Relative Position supplies movement only.
A future Inventory table would exist because it needs bounded listing, membership,
privacy, removal and perhaps capacity—not because `inside` was renamed.

### 11. Freedom, knowledge and secure mutation

Aicadia's freedom does not require permanent author ownership of ordinary public
state. If a Character can currently know and affect a cup, its Agent can propose
moving it, including a surprising bounded consequence. World does not ask who first
created the cup merely to preserve conventional physics.

Freedom also does not mean omniscience:

- the Character must be eligible to know every exact subject and current fact it
  proposes to change;
- current locality, explicit access or an operation-specific remote basis must make
  the attempted Action eligible;
- expected revisions prove that the proposal was grounded in fresh authorized state;
- guessed Entity ids and stale memories do not satisfy current eligibility;
- private facts are filtered before counts, pagination, hydration and errors; and
- high-impact shared mechanics may require explicit access or collective
  ratification when that gameplay is introduced.

The World remains dumb and strict. A Property saying `locked`, a Relation saying
`belongs to Mara` or a description saying `too heavy` changes no permission unless
an accepted typed mechanic explicitly uses structural state submitted for that
purpose.

## Candidate PostgreSQL shape

This section is an implementation sketch to make the recommendation falsifiable. It
does not accept table or field names and does not authorize a migration.

### Exact current Position and history

The shape fits Aicadia's existing current-pointer plus immutable-version convention:

```text
position(
    entity_id                 PK/FK entity,
    current_activity_id,
    FK (entity_id, current_activity_id) -> position_version
)

position_version(
    entity_id                 FK entity,
    activity_id               FK activity,
    previous_activity_id      NULL,
    reference_kind            CHECK world | entity,
    coordinate_space_id       NULL,
    reference_entity_id       NULL/FK entity,
    x_mm                      BIGINT,
    y_mm                      BIGINT,
    z_mm                      BIGINT,
    PRIMARY KEY (entity_id, activity_id),
    exact XOR check:
      world  => coordinate_space_id present, reference_entity_id absent
      entity => coordinate_space_id absent,  reference_entity_id present,
    CHECK (reference_entity_id IS NULL OR reference_entity_id <> entity_id)
)
```

`position.entity_id` is the Position identity. `activity_id` identifies a version,
not another durable subject. A move of several Entities appends one version per
affected Entity under one Activity and advances their current pointers atomically.
Removing Position needs an explicit versioned absence/tombstone design before
implementation; silently deleting the current row would not by itself preserve what
the accepted Action changed.

The core indexes are small and earned:

```text
position_version(reference_entity_id, entity_id)
  WHERE reference_kind = entity

position(entity_id)                         -- supplied by PK
position_version(entity_id, activity_id)    -- supplied by PK
```

The reference index serves bounded immediate-child pages and cycle/re-reference
checks. It is not an unbounded descendant listing.

### Proposed current-Place association

If later behavior proves that Character and ordinary-Entity exact-Place context can
share one model without changing their rules, its narrow shape can remain:

```text
place_presence(                -- proposed label, not accepted vocabulary
    entity_id                  PK/FK entity,
    place_entity_id            FK place,
    current_activity_id        FK activity
)

INDEX place_presence(place_entity_id, entity_id)
```

This row says only that the Entity directly participates in that Place's current
local context. Descendants whose local context is inherited through relative
Position are not copied into it. The current game uses separate
`character.current_place_entity_id` and `entity_location`; a real migration must
first prove identical lifecycle, visibility and conflict behavior rather than
renaming them by architectural preference.

### Proposed Relation and dependencies

One possible open authored shape is:

```text
relation(
    id,
    source_entity_id          FK entity,
    target_entity_id          FK entity,
    current_activity_id,
    visibility_policy_id,
    UNIQUE as required only after duplicate semantics are chosen
)

relation_version(
    relation_id               FK relation,
    activity_id               FK activity,
    previous_activity_id      NULL,
    name,
    description,
    bounded_properties,
    PRIMARY KEY (relation_id, activity_id)
)

relation_dependency(
    relation_id,
    relation_activity_id,
    entity_id,
    position_activity_id,
    PRIMARY KEY (relation_id, relation_activity_id, entity_id)
)
```

The exact visibility and Property realization remains open. It may reuse an earned
policy seam, but a polymorphic ACL table with weak foreign keys is not automatically
better than explicit resource-specific rows. Public reads always start from one
eligible Character and one bounded subject/type/cursor scope; there is no global
Relation scan or arbitrary predicate query.

### Area, Connection and later Route

Illustrative structures remain separate:

```text
area_version(
    place_entity_id,
    activity_id,
    previous_activity_id,
    local_geometry,
    completeness,
    coordinate_basis_revision,
    bounded vertex/component metadata
)

connection(
    id,
    first_place_entity_id,
    second_place_entity_id,
    allows_first_to_second,
    allows_second_to_first,
    current_activity_id,
    CHECK at least one direction,
    CHECK endpoints differ
)

route(                       -- absent until named journeys are accepted
    id, name, description, visibility_policy_id, current_activity_id
)

route_step(
    route_id,
    ordinal,
    connection_id,
    PRIMARY KEY (route_id, ordinal)
)
```

Area geometry can be PostGIS geometry in a named basis when current gameplay earns
exact predicates. PostGIS bounding boxes produce candidates; World still applies
the exact accepted predicate, visibility and output bound. Connection endpoint
indexes serve each travel direction. A Route step does not copy endpoint geometry.

### Activity footprint

Every accepted spatial mutation writes one Activity in the same transaction as its
current pointers and immutable versions. Its normalized involved-Entity roles must
answer:

- which Character acted and which User requested it;
- which Entities were moved, referenced, related, targeted or used as location;
- which Place or exact resolved basis contextualized the Action;
- which Position, Relation, Area, Connection or operation-specific versions changed;
- which request id makes a retry return the same result; and
- when the World accepted it.

This is operation-specific structured history, not event sourcing and not a universal
JSON event payload. Descendants whose resolved point changed only because an ancestor
moved do not receive counterfeit movement Activities.

When an Activity's external historic point matters, its footprint names the exact
Position versions in the resolved ancestor chain, so later carrier movement cannot
move history. A purely cabin-local Action can record Cabin as location without
depending on Ship's external Position; it should not manufacture an external
coordinate that the Action never used.

## World read contracts

One database table never needs to mean one Agent call. World can compose a small
number of exact records behind one bounded operation.

### Addressed Entity or Place read

When eligible, one read returns:

- the safe Entity fields and roles;
- Position as the resolved current point plus freshness;
- explicit current Place when established and visible;
- immediate reference only when visible and useful;
- optional Area summary for a Place;
- a bounded page of currently visible authored Relations when requested; and
- a bounded page of direct Connections when requested for a Place.

The operation never returns every descendant, every nearby subject or every Relation
by default. An unpositioned, hidden or stale selection uses neutral behavior that
does not reveal which condition applied.

### Nearby and map reads

Every spatial list requires an actor, an exact bounded scope, a maximum result count,
stable continuation and an overload contract. Useful reads remain separate:

- Entities at the same established Place;
- immediate Entities relative to one known reference;
- visible Entities within a metric radius;
- established Places within a map window;
- direct Connections from one Place; and
- a calculated path with depth, candidate and time bounds.

There is no `list_all_place` or unbounded `nearby`. At five-year scale an Agent sees
a bounded, Character-grounded map slice rather than downloading the World.

### Relation read

A Relation read is bounded by one visible endpoint or exact Relation id, direction,
visibility, cursor and limit. Filtering occurs before a row contributes to a count,
page boundary or error. Dependencies are checked against current exact revisions;
stale authored meaning can remain in history without appearing as fresh current fact.

## World mutation contract

One Agent proposal can compose several exact changes without one network call per
table. A move such as “pick up the cup and make the Character float too” can submit
two Position changes and an optional Relation change in one bounded Action.

The strict settlement order is:

1. derive User and Character; resolve request-id retry before fresh work;
2. parse and bound subjects, geometry, relation text, properties and output scope;
3. retrieve only currently eligible exact facts without revealing missing/private
   distinctions;
4. collect proposed subjects and the bounded reference/dependency chains;
5. lock existing Entity/current-state rows in one stable id order, then re-read the
   chains so concurrent re-referencing cannot create a cycle;
6. validate expected revisions, depth, coordinate bounds, access, locality or the
   explicit remote mechanic and every claimed dependency;
7. insert one Activity, immutable versions and exact current-pointer changes;
8. mark only rebuildable index/interest resources dirty or update the minimal
   synchronous candidate index; and
9. commit once and return the authoritative result.

An ordinary move retaining the same immediate reference locks the moved Entity's
Position, not every ancestor and descendant. Re-reference must inspect the new
bounded ancestor chain to reject cycles. A local cabin Action does not resolve or
lock the ship's external Position unless its validity actually depends on the
external World point. A cross-boundary shot or shore observation does name that
dependency and may retry if the ship moves concurrently.

Entering or leaving a Place changes Position and the exact current-Place association
in that same transaction. Movement between unnamed points changes only Position.
Neither case advances one shared Place row merely to serialize every occupant's
movement; Place metadata changes remain separate conflicts.

## Indexing and massive concurrency

### Canonical state versus query acceleration

Canonical truth is the current Position tuple and its immutable versions. The
resolved World point is calculated. Five-year performance may earn three disposable
accelerators:

1. a root-position spatial index for World-referenced Entities and carrier
   envelopes;
2. a local spatial index for Entities relative to a carrier or other reference; and
3. a rebuildable resolved-position projection for measured hot queries that cannot
   be served by the two-stage form.

An external nearby query first finds a bounded set of root Entities or carrier
envelopes, transforms its search volume into each candidate local basis, obtains a
bounded local candidate set and exact-rechecks current Position chains and
visibility. A ship move updates its root Position; the local passenger/cargo index
does not move.

No stale projection can authorize a mutation. To avoid false-negative observation,
the system either maintains a conservative synchronous envelope, proves projection
freshness for the query or fails closed/retries; it never treats silent projection
lag as exact absence.

### The million-Character point

A million independent Characters at one coordinate do not share one Position row
or lock. Each retains its own Position and revision. The pressure moves to the index
range and response cardinality:

- no exact count is promised;
- every page is limited and authorization-filtered;
- cursors use stable ordering and cannot expose private gaps;
- per-Character and per-query admission bounds CPU, rows and bytes;
- presence/interest hints may be coalesced; and
- saturation of the hot range returns an explicit bounded overload outcome rather
  than starving unrelated coordinates.

### The hot carrier

The carrier's one true external Position is necessarily a serial conflict point for
simultaneous moves of that same carrier. No architecture can make contradictory
truthful carrier moves conflict-free. The correct target is isolation:

- carrier moves serialize/admit on that Position only;
- interior local Actions avoid that row unless they need its resolved point;
- descendants are not rewritten or individually locked;
- subscriptions are hints and can coalesce many carrier changes; and
- repeated hostile moves are bounded per actor and subject.

### Partitioning

Operational partitions may follow hash(Entity id), coordinate buckets, activity
time or measured table pressure. None becomes Entity identity or a public Place.
Any future repartition must preserve stable ids and semantic queries. PostgreSQL
remains the authority; this recommendation does not require microservices, a graph
database or server affinity.

## Privacy and security model

### Required guarantees

1. **Default unavailable:** a Character receives no fact merely because an id is
   syntactically valid.
2. **Filter before shape:** existence, counts, order, cursors, timing and error text
   are subject to eligibility, not only the final hydrated payload.
3. **Fresh authority:** a stale observation or remembered id may inform an Agent's
   reasoning but cannot directly mutate hidden current state.
4. **No prose authority:** Relation names, Entity descriptions and Properties never
   grant access, topology, remote effects or ownership.
5. **Exact dependencies:** remote and multi-subject Actions name the exact current
   structural facts on which they rely.
6. **Bound every input:** subject count, chain depth, coordinates, radius, geometry
   bytes/vertices/components, topology fan-out, path depth, text and Properties all
   have explicit limits.
7. **Neutral rejection:** missing, private, departed, stale and otherwise
   ineligible facts do not become an oracle through different public errors.
8. **One transaction:** authorization basis, current changes and Activity cannot
   settle in contradictory commits.

### Hidden sword example

Mara once saw Noor put Sword S in Backpack P. Later Mara sees only P.

- Mara's Agent may remember the old observation but cannot read current Position or
  Relation for S by id.
- The Relation and any private Position reference are filtered before existence,
  counts and pagination.
- Mara can propose an eligible investigation or interaction with the visible
  backpack. World does not reveal whether S remains inside until that mechanic
  explicitly permits discovery.
- Noor's eligible read can return a bounded private-contents page and resolved
  Position without making the same result public.
- If inventory gameplay later needs removal, capacity or listing invariants, it
  earns a typed Inventory mechanic. Relative Position alone never grants removal.

### Distant bomb example

- Button and bomb retain independent Position; distance is irrelevant to authority.
- A free Relation may describe that the button controls the bomb but is inert.
- A proposed typed Activation record names the exact endpoints, permitted operation,
  visibility and current revisions.
- Pressing validates current access to the button and that Activation basis. The
  Agent supplies a bounded affected set and claimed Places; World performs no blast
  inference from prose.
- If button press and bomb state change are one accepted Action, both current facts,
  involved roles and Activity commit atomically. A consequence too wide for the
  accepted bound is rejected or routed through a separately accepted collective
  mechanism, never silently fanned out.

## Scenario walkthrough

### SP01 — Character moves from Place A to Place B

**Input.** The Agent names the Character, fresh source Position, destination Place
and either a permitted direct Connection or an exact World point under the accepted
movement capability. Discovery and entry remain separate.

**Stored state.** One Character Position version changes and deliberate entry updates
current Place to B in the same transaction. A and B retain identity; Connection
remains unchanged. Intermediate stops use Position and can have no current Place.

**Read/privacy.** Eligible reads return the current resolved point and bounded known
Place/Connection context. An undisclosed Place does not appear because it is nearby
in the technical index.

**Contention/cost.** Lock the Character Position and exact travel dependencies, not
all Entities at A or B. One Activity commits. Movement duration remains a separate
game choice.

### SP02 — Entity in a coat

**Input.** The Agent can choose a coat-relative exact Position and optionally author
a private proposed Relation saying `inside`. Neither choice is inferred.

**Stored state.** Moving the coat changes only its Position; the child's resolved
point follows. A later Inventory mechanic is absent until listing/removal earns it.

**Read/privacy.** Seeing the coat does not reveal the child, its Position, the
Relation, count or cursor gap. The eligible holder may receive a bounded composed
read.

**Contention/cost.** Coat-local changes use immediate subject/reference indexes. An
ordinary holder move does not rewrite the hidden child.

### SP03 — Distant button and bomb

**Input.** The Agent presses the visible/usable button and names the exact accepted
remote basis and bounded consequences. Position and Connection do not supply it.

**Stored state.** Endpoint Positions remain independent. A typed remote mechanic and
one Activity ground the accepted effect; an optional Relation is explanatory only.

**Read/privacy.** Authorized use need not reveal the bomb's Position or even full
identity to unauthorized observers. Guessed ids fail neutrally.

**Contention/cost.** Lock exact button, remote-basis and affected-subject versions.
The transaction never locks every subject at the remote Place.

### SP04 — Cup two centimetres above a table

**Input.** To establish exact Position, the Agent supplies a table reference and
enough accepted local structure to identify one point: horizontal point, surface or
local height, direction and two-centimetre offset. Otherwise it submits only authored
meaning.

**Stored state.** A table-relative Position follows the table; a World-relative
Position stays behind. The proposed Relation can preserve “floating two centimetres
above” with Position/geometry revision dependencies. Both may change in one Action.

**Read/privacy.** One composed read can return the exact resolved point and the
visible authored statement without mixing their authority.

**Contention/cost.** Moving the table writes one Position. Picking up the cup
re-references it explicitly. Moving cup and Character together locks exactly those
current Positions and writes one Activity.

### SP05 — Dog under a bridge

**Input.** The Agent authors the statement with optional current dog/bridge Position
and geometry dependencies, or invokes a named exact geometry predicate if such a
capability has been accepted.

**Stored state.** Dog and bridge Positions remain canonical. `under` is either a
fresh authored Relation or a calculated answer, never a second mechanical Position.

**Read/privacy.** Moving either subject makes the dependency-bound statement stale;
history remains. World does not understand the prose or invent geometry.

**Contention/cost.** The move touches the moved Entity only. No reverse scan rewrites
all Relations; freshness is checked on a bounded endpoint query.

### SP06 — One hundred metres to a hotel

**Input.** The request chooses straight-line distance, named-path distance or an
Agent-authored estimate and identifies what hotel point or Area counts as arrival.

**Stored state.** Character and Hotel Positions stay canonical. A changing distance
is calculated; it is stored only as authored meaning when preserving the estimate is
itself gameplay.

**Read/privacy.** The hotel must be currently knowable. A path query returns a
bounded calculation, not a hidden map.

**Contention/cost.** Metric distance reads two resolved chains. Path distance has
explicit topology depth, candidate and time limits.

### SP07 — Birdhouse on table in a village

**Input.** Creation establishes Birdhouse B, a table-relative Position and optional
Relation; Village Place and Table remain independent Entities.

**Stored state.** The chain B→T→possibly V/root resolves one point. Village identity
or Area is not copied into Birdhouse. Moving T moves B mechanically.

**Read/privacy.** A bounded local query can discover B when visibility rules and
exact candidate state permit it. The table relationship need not expose a private
ancestor.

**Contention/cost.** Moving T writes one Position. Concurrently lifting B and moving
T validates exact Position revisions; one wins or the other retries without a
descendant rewrite.

### SP08 — Forest edge and open heath

**Input.** The Character changes Position through unnamed ground. An Agent may
author terrain descriptions and, when known, establish or refine Area geometry.

**Stored state.** No Place is created for every stopping point. Current Place may be
cleared while exact Position remains. A named forest may be an Entity with Place and
incomplete Area; open heath can remain description until it earns independent Place
identity.

**Read/privacy.** A boundary crossing is calculated only from accepted geometry and
an explicit predicate or remains Agent-authored meaning. Unknown outside coverage
stays unknown.

**Contention/cost.** Position movement is per Character. Area changes are bounded by
geometry limits and their own versions rather than locking every Character nearby.

### SP09 — Cabin, passengers and cargo on a ship

**Input.** Ship is World-referenced; cabin can be ship-relative; passengers and
cargo use the nearest meaningful reference. Cross-boundary Actions explicitly name
the ship Position dependency.

**Stored state.** One ship move changes one Position. Cabin remains a Place with the
same Entity id, and a passenger's current Place can remain Cabin. Descendant
Positions, current-Place rows and local indexes remain unchanged.

**Read/privacy.** Interior reads operate in the cabin/ship local basis. Exterior
reads resolve bounded chains and filter private cargo before output.

**Contention/cost.** Interior local Actions do not lock ship external Position.
Carrier moves serialize honestly on the ship row. Candidate/root-local indexes keep
nearby work bounded; real latency remains an experiment.

### SP10 — Remembered sword hidden in backpack

**Input.** The old observation is memory, not fresh mutation basis. The Agent can
attempt investigation against the visible backpack but cannot name current hidden
sword state as if observed.

**Stored state.** Sword may have a private backpack-relative Position and Relation;
neither is exposed. Earlier Activity remains historical.

**Read/privacy.** Exact identity, endpoint, Position, Relation, count and failure
reason remain unavailable until current eligibility changes.

**Contention/cost.** Holder/private-content queries remain reference- and cursor-
bounded. No global reverse Relation scan occurs.

### SP11 — A connects to B but not C

**Input.** The Agent establishes one Connection with explicit direction(s) between
Place A and B. Absence of A–C remains absence, not an explicit negative edge.

**Stored state.** Connection is direct topology. Road/bridge/door Entities and
current access are separate dependencies. A calculated A→…→C path is not stored.

**Read/privacy.** A bounded Connection page shows only topology eligible for the
Character. Hidden endpoints do not leak through graph degree or path failure.

**Contention/cost.** Mutations lock that Connection/endpoints. Traversal is bounded
by depth, fan-out, candidates, time and results. Later Green Route state orders
Connection ids without copying them.

### SP12 — Million Characters at one unnamed Position

**Input.** Every Character independently establishes the same exact coordinate;
none owns a shared Position object.

**Stored state.** One Position row per Character. No Place or cell is minted merely
because the point is popular.

**Read/privacy.** Nearby returns a limited authorized page, never the full set or an
exact hidden count. Interest hints may be coalesced.

**Contention/cost.** Writes conflict per Character; index-page and query admission
are the hot resources. Overload at that point does not require a World lock or starve
quiet ranges.

### SP13 — City, forest and waterfall on a map

**Input.** World establishes independent subject identity first, Position second and
Place role only where map/discovery/navigation reference behavior is intended.

**Stored state.** City is Entity+Place+likely Area; named forest can be the same;
waterfall is Entity and may be Place without Area. Ordinary trees remain positioned
Entities. All use the same Position model.

**Read/privacy.** Map queries are bounded by known/visible window and role; they do
not list the whole Place table. Area overlap does not grant travel or reveal hidden
subjects.

**Contention/cost.** Role/Area edits use their own rows and versions. Operational
map cells remain rebuildable indexes.

## Alternatives rejected by the backcast

### Absolute World coordinates only

This is attractive for the first Place-to-Place move. It fails the ship, cabin,
birdhouse and carried-item future: either descendants are rewritten, follow behavior
is hidden elsewhere or resolved state becomes stale. Keep it as the first direct
Position implementation, not as the permanent model limit.

### Place owns `x`, `y`, `z`

It removes one join for Places but duplicates the same truth once Characters,
ordinary Entities and moving Places need Position. It also makes Place look like the
universal spatial address. One Position keyed by Entity keeps the model and Agent
read simpler even if PostgreSQL joins internally.

### One broad Position row with coordinates, reference, prose and Properties

It looks lightweight because there is one row. Exact movement, authored meaning,
visibility, freshness, indexing and concurrent edits then share one revision and
lifecycle. A description edit conflicts with movement; a private sentence hides or
exposes coordinates; every reader must decide which fields are mechanical. It is
ceremony deferred into every future Action.

### Relative coordinate plus separate generic follow Relation

It creates two truths that must agree for the ordinary case and requires extra
reads/writes merely to know where an Entity is. If relative does not follow, it goes
stale; if follow exists, it duplicates reference semantics. Explicit World-relative
versus Entity-relative Position is smaller and clearer.

### Universal Relation table drives mechanics

It maximizes writable vocabulary but forces Position, inventory, Connection,
activation, access and ownership to branch on arbitrary predicates. Conditional
cardinality, cycles, privacy, indexes and authority recreate typed mechanics inside
one clever table. Keep Relation open and inert; let deterministic mechanics earn
small typed homes.

### Everything meaningful is a Place

It makes every forest step, table and unnamed encounter a map node. At millions of
subjects, Place loses its independent-reference meaning and map reads become entity
scans. Position covers exact presence; Place remains a role earned by spatial
reference behavior.

### Derive current Place from coordinate equality or Area overlap

It would make discovery equal entry, turn a redrawn Area into mass movement and
force every boundary predicate to decide local game context. Explicit current Place
is one narrow separate fact; Position and Area remain usable without it.

### Canonical cells or tiles

They simplify indexing but turn resolution, sharding and engine choices into World
lore. Repartitioning can then rename Places and reveal a complete geography.
Cells remain replaceable projections over stable Entities and Positions.

### Persist every resolved World point

It makes nearby indexing easy but creates a competing current truth for descendants.
A carrier move either rewrites them or leaves them stale. A derived projection is
allowed only with explicit freshness and exact canonical recheck.

### Graph database or microservice spatial service

The required truths and reads are bounded relational facts, exact versions and
spatial candidates. PostgreSQL and optional PostGIS can own them. Another authority
adds distributed consistency and authorization seams without removing hierarchy or
hot-subject pressure.

### Server physics or semantic inference

It would decide what `above`, `inside`, `locked`, `blast` and `heavy` mean from
content, normalize deliberate weirdness and spend compute without an explicit User
Agent call. Agents propose exact consequences; World validates structure.

## Falsifiers and honest unknowns

The recommendation must be reopened if a real bounded fixture shows any of these:

- Entity-relative Position cannot resolve one unambiguous point through accepted
  orientation/coordinate rules;
- correct cycle prevention needs a global lock or unbounded traversal;
- one carrier move still needs descendant current-state writes;
- local interior Actions cannot avoid irrelevant carrier conflicts;
- a private relative Position cannot be indexed and read without an existence leak;
- bounded nearby search requires a second authoritative Position;
- fixed-point millimetres cannot satisfy accepted extent, precision or HTTP/MCP
  round trips;
- one composed World operation over separate truths is materially harder for Agents
  than a broad payload after both support the same scenarios; or
- direct Connection and calculated paths duplicate travel truth in practice.

Still unknown and not smuggled in as decisions:

- exact coordinate axes, maximum extent and final numeric representation;
- whether proposed Orientation is needed in the first relative scene;
- complete versus positive-only Area operations and 2D versus 3D geometry;
- the final Relation fields, visibility model, duplicate semantics and dependencies;
- concrete Inventory, access, Activation or collective-authority mechanics;
- movement time, collision, reach, terrain costs and arrival rules;
- exact nearby index/projection design and overload thresholds;
- production query plans, capacity, privacy timing and failure behavior; and
- final API names and public Agent text.

## Backcast: five years to now

### Year five — mature, measured spatial operation

The World runs canonical Position and typed mechanics in PostgreSQL, with measured
root/local spatial indexes and rebuildable projections. Multiple coordinate spaces,
moving Places, Area geometry and named Routes exist only where actual play earned
them. Hot ranges have explicit admission and fairness. Privacy probes, failover,
projection rebuild and repartition are routine evidence gates.

**What had to be true earlier:** stable Entity identity and Position could not be
derived from partitions; local and resolved state were already separate; all reads
and mutations already carried exact versions and bounded visibility.

### Year four — moving communities and hard privacy

Ships, vehicles and moving settlements use relative Position and optional
Orientation. Carrier/root-local indexing and privacy around cargo are tested with
real PostgreSQL data. Interior play is independent of external carrier motion. Remote
Actions name exact capability and scope.

**What had to be true earlier:** relative Position had one mechanical meaning, chain
depth and cycles were bounded, and open Relation prose never granted access.

### Year three — terrain, topology and player journeys

Areas support only the geometry predicates gameplay uses. Direct Connection has
access and physical dependencies where earned. Calculated paths remain output; named
shared journeys introduce Route only now. Bounded map and nearby queries use measured
indexes rather than canonical cells.

**What had to be true earlier:** Place, Area, Connection and Route were separate,
and Characters could occupy unnamed Position between Places.

### Year two — continuous local exploration

Characters and ordinary Entities can hold direct World-referenced Positions. Agents
can ask for exact addressed Position, bounded local subjects and metric distance.
The system tests precision, privacy and the million-Character point before promising
broad nearby queries. Current Place remains an explicit optional game context and is
not inferred from a coordinate or Area.

**What had to be true earlier:** one Position belonged to Entity identity, history
was structured and every current read was Character-grounded.

### Year one — first spatial game-complete slice

The game establishes a second Place, allows discovery without entry and then accepts
one deliberate move. Place uses the same Position model as Character. Direct
Connection records explicit direction when the chosen movement rule needs it. The
move updates exact Position and explicit current Place together. The slice proves
idempotency, exact conflict, Activity and bounded reads; it does not prebuild Area,
Relation, Route, relative Position or geometry.

**What had to be decided first:** the long-term Position identity and layer
boundaries, so the direct-only first schema does not become a false universal model.

### Now — smallest Terry step

Do not implement the full future schema. The smallest high-leverage sequence is:

1. **User decision:** accept, revise or reject the core recommendation that Position
   is one exact `{reference, point}` per Entity; Entity-relative means following; an
   eligible read returns one resolved World point; free proposed Relation meaning
   stays separate and inert.
2. **One real lab:** after acceptance, test direct/relative resolution, preserve-
   world versus preserve-local re-reference, cycle/depth, fixed-point precision,
   private child reads and a ship with 1/1,000/100,000 descendants against real
   PostgreSQL. Name every simulated seam and do not claim million-player proof.
3. **First production behavior:** if the lab does not falsify the foundation, return
   to the already selected second-Place discovery-and-movement scene. Implement only
   direct Position required by that slice. Do not add dormant relative, Relation,
   Area, Route, geometry or projection fields.
4. **Earn the next layer:** use the birdhouse/table or ship fixture to introduce
   relative Position only when that behavior becomes the highest-value game edge;
   use the floating-cup scene to decide proposed Relation storage only when durable
   authored spatial meaning becomes current gameplay.

This ending is intentionally asymmetric: the architecture decision is broad enough
to prevent a dead end, while each build remains the smallest complete player-facing
behavior that currently earns code.

## Recommendation versus decision

### Recommended now

- one Position keyed by Entity;
- Position contains exact point plus World-or-one-Entity reference;
- Entity-relative Position means mechanical movement inheritance;
- one eligible World read resolves the current World point;
- current Place remains an optional explicit context distinct from Position and
  Area;
- Place uses that same Position and may eventually be relative;
- proposed open Relation stores authored meaning separately and has no mechanical
  execution;
- Area, Connection, access/private contents, remote causality and later Route remain
  distinct earned mechanics;
- per-fact versions, bounded chains, stable locks, atomic Activity and disposable
  indexes provide the scale/security foundation; and
- fixed-point millimetres and depth eight are concrete starting candidates to test.

### Not decided while the User sleeps

No item above has entered `dev/CONTEXT.md`, an Area's `Chosen` section, `game/docs`,
the backlog, schema, API, code or public Agent text. The next conversation can accept
the recommendation as a package, challenge one premise or choose the first proposed
decision to grill. Repository history, not this report alone, will record any later
acceptance.
