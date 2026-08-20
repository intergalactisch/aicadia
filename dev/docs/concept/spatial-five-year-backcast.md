---
status: active
---

# Five-year spatial backcast

> **Role / side:** live five-year spatial recommendation / development side.
> **Authority:** owns the future observation, recommended direction, scenario application, technical candidate, rejected alternatives, falsifiers and backward roadmap produced by the 2026-08-18 `5jaar` exercise.
> **Excludes:** accepted Aicadia vocabulary, current Area truth, current game behavior, production schema, API and implementation; those require later User decisions and their own authorities.

Date: 2026-08-18

Status: **complete exploration and technical synthesis; spatial product-choice
frontier empty; implementation planning pending**. The initial five-year recommendation
is retained as the starting hypothesis; later sections that record what the User
selected, accepted, rejected or corrected supersede it wherever they differ. Every
illustrated field or table remains descriptive unless a later section explicitly
accepts its domain meaning; no production schema or build is authorized here.

The primary-source basis is the
[spatial foundation research report](../research/spatial-five-year-foundation.md).
The initial fixed tests were the first thirteen
[spatial scenarios](../../areas/place/scenarios.md); later User choices may append
new pressure cases without rewriting that research run. The earlier
[four-candidate paper comparison](../../lab/spatial/01-model-pressure/README.md)
supplied a leading direction; this backcast tried to break it rather than treating
it as decided.

## Initial executive recommendation

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
both Entities and Activity are. An eligible descriptive Relation may explain the
causality to the Agent but neither executes nor authorizes it; the Agent submits one
exact confirmed Action under ordinary World authority.

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
| Optional resolved output | Agent always gets the stored basis and gets the World point when bounded resolution succeeds | bounded calculation, explicit absence and no second canonical storage |
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
- **Re-reference:** the Agent submits the complete new World or Entity reference and
  all three new offsets. It may deliberately choose values that preserve a prior
  World point or offsets, but World has no mode that calculates either outcome
  implicitly and never derives it from “on”, “inside”, “held by” or other prose.

One immediate reference is enough. Chains arise compositionally: Birdhouse can be
relative to Table, Table to Cabin and Cabin to Ship. No fixed chain-length maximum
is accepted. Each read instead performs bounded work: it returns the immediate stored
basis and includes a World point only when that work reaches an absolute basis.
Cycles still cannot yield the exact point Position means; their bounded validation
on mutation remains the next open dependency.

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

### 4. One authorized read always returns the basis and may resolve the World point

An Agent should not have to understand storage joins merely to learn the immediate
Position. When the Character is eligible, one World read always returns the stored
absolute values or immediate Entity reference and offsets with their freshness. World
also returns the calculated current World point when bounded work reaches the
absolute basis. Otherwise the point is absent and any Action structurally requiring
it fails closed; World never substitutes a stale cache or rewrites descendants.

That promise has four deliberate limits:

1. an unpositioned Entity has no point and World says so only when the Character is
   eligible to know the Entity and that absence;
2. a hidden Entity or Position is unavailable without confirming its existence; and
3. a deep eligible Position may return its immediate basis without a World point
   when bounded resolution does not reach the absolute basis; and
4. a calculated point is fresh for the named Position/reference revisions, not an
   eternal coordinate that remains valid after an ancestor moves.

How a hidden immediate reference affects the otherwise eligible Position remains an
open privacy choice. Eligibility and technical resolvability are separate: the first
decides which stored facts may be returned, while the second decides whether an exact
World point can be calculated within one bounded read.

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

**Area.** Area is optional exact positive coverage of a Place: established geometry
proves inclusion, while absence outside it remains unknown. Overlap may return
several true Places and creates no Connection, visibility, ownership or movement.
The exact geometry and moving-basis representation remain technical work.

**Connection.** Connection is one stable named and described direct travel
alternative between two Places, and several may share endpoints. It states explicit
direction, may own one optional ordered three-dimensional World-points course and
never follows from geometry or prose. A physical road, door, bridge or ferry remains
a separate Entity.

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
| button/bomb | `connected to` | one exact confirmed Action under ordinary authority; Relation remains non-executable context |
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
- ordinary current subject eligibility and authority must permit every exact named
  change; an Agent-understood Relation never widens them;
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
4. collect proposed subjects and perform bounded work over required
   reference/dependency links;
5. lock existing Entity/current-state rows in one stable id order, then re-read the
   chains so concurrent re-referencing cannot create a cycle;
6. validate expected revisions, coordinate bounds, ordinary authority and every
   claimed structural dependency without interpreting Relation meaning;
7. insert one Activity, immutable versions and exact current-pointer changes;
8. mark only rebuildable index/interest resources dirty or update the minimal
   synchronous candidate index; and
9. commit once and return the authoritative result.

An ordinary move retaining the same immediate reference locks the moved Entity's
Position, not every ancestor and descendant. Re-reference must prove within bounded
work that the proposed reference does not create a cycle; the result when it cannot
prove that remains open. A local cabin Action does not resolve or
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
- A free eligible Relation may describe that the button controls the bomb but remains
  inert and grants no authority.
- The Agent previews and submits one exact bounded Action naming Button, Bomb,
  revisions, intended state and optional current Relation context.
- World validates ordinary eligibility, authority, freshness, idempotency and bounds;
  it performs no blast inference from prose.
- Accepted state, involved roles and Activity commit atomically. A consequence too
  wide or unauthorized is rejected, never silently fanned out.

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

**Input.** The Agent understands an eligible B→X Relation, previews the exact bounded
button-and-bomb Action and submits every intended consequence after confirmation.
Position and Connection do not supply authority.

**Stored state.** Endpoint Positions remain independent. Relation stays explanatory;
the exact accepted current state and one Activity ground the result.

**Read/privacy.** Authorized use need not reveal the bomb's Position or even full
identity to unauthorized observers. Guessed ids fail neutrally.

**Contention/cost.** Validate or lock the exact Button, optional claimed Relation,
Bomb and affected-subject versions. The transaction never locks every subject at the
remote Place.

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
reads attempt bounded resolution and filter private cargo before output.

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
- concrete Inventory, access or collective-authority mechanics;
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

**What had to be true earlier:** relative Position had one mechanical meaning, every
read had bounded work, cycles could not become a valid Position and open Relation
prose never granted access.

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
   eligible read returns its stored basis and adds the World point when bounded
   resolution succeeds; free proposed Relation meaning stays separate and inert.
2. **One real lab:** after acceptance, test direct/relative resolution, explicit
   re-reference values that intentionally preserve a World point or local offsets,
   cycles/deep reads, fixed-point precision,
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
- one eligible World read returns the stored basis and includes the current World
  point only when bounded resolution reaches the absolute basis;
- current Place remains an optional explicit context distinct from Position and
  Area;
- Place uses that same Position and may eventually be relative;
- proposed open Relation stores authored meaning separately and has no mechanical
  execution;
- Area, Connection, access/private contents, remote causality and later Route remain
  distinct earned mechanics;
- per-fact versions, bounded reads, stable locks, atomic Activity and disposable
  indexes provide the scale/security foundation; and
- whole-centimetre coordinates are accepted, while no fixed chain depth is.

### State when the backcast was delivered

At delivery, no item above had entered `dev/CONTEXT.md`, an Area's `Chosen` section,
`game/docs`, the backlog, schema, API, code or public Agent text. Repository history,
not this report alone, records later acceptance.

## Subsequent User choice — first-slice focus

On 2026-08-19 the User accepted the smallest first spatial scene without accepting
the complete backcast model:

- Place A and Place B may differ through their Entity descriptions or Agent-authored
  Properties—for example grassy ground versus dunes—without a server Terrain model.
- Discovery establishes B and its explicit A→B Connection while the Character stays
  at A; the player may inspect B and decline to enter.
- That discovery is one complete confirmed package: B's Entity and Place role,
  direct Position, name, description, optional initial Properties and Traits
  (including authored landscape state) and directed A→B Connection commit with one
  Activity, or none of them do. An accepted retry returns the same complete result.
- The Agent reads A's fresh exact Position, reasons about the spatial meaning and
  submits B's exact three-dimensional absolute World point as structured state.
  World never parses prose into Position. Before confirmation the Agent must inspect a bounded eligible
  set of established Places around the proposed point and include the returned
  context in the complete preview. Hidden or out-of-scope Places remain unknown;
  their omission never establishes global absence.
- The first slice applies no gameplay distance limit from A. The Agent may propose
  any technically valid absolute World point and the User may confirm it after the
  required nearby inspection; World validates representation and structural truth,
  not a discovery radius.
- Every exact World point has three spatial dimensions. This keeps height available
  for caves, cliffs, bridges and object arrangements, but does not introduce
  geometry, surfaces or Orientation and does not make incomplete prose calculable.
- Each dimension is submitted as a whole number of centimetres. Agents may present
  other units to Users but convert them before submission; World never parses unit
  prose. Millimetre precision is rejected for this foundation unless later concrete
  gameplay earns it. Integer width and range remain open.
- The three values are named `x`, `y` and `z`; `x` and `y` form the horizontal plane
  and `z` is vertical height. Increasing `x` is east, increasing `y` north and
  increasing `z` up. Agents translate natural direction language before submission;
  World receives structured values and performs no compass parsing.
- `(0, 0, 0)` is the permanent World origin. It has no Entity or Place identity,
  frame object, database row, lifecycle or lock. The first entry Place is initially
  positioned there, but later movement of that Place would never move the origin.
- If a returned Place plausibly fits the intended discovery, the Agent must discuss
  reuse with the User and explain why a new Place may still be distinct. Only an
  explicit User choice continues the new-Place proposal; World never makes or
  validates that semantic judgment. If the User chooses existing C, the alternative
  atomic result consumes the opportunity and establishes only the explicit allowed
  A→C Connection plus Activity. Proximity informed the choice but did not establish
  direct travel by itself.
- If the required A→C direction already exists or a concurrent request establishes
  it first, a distinct later request creates no duplicate Connection or Activity and
  retains its positive opportunity. The Agent re-grounds; only retrying the exact
  winning request returns the prior accepted result.
- Equal or nearby Place Positions remain independent under concurrent discovery.
  Proximity informs the Agent and User but is not a uniqueness, blocking or merge
  rule; the future may earn explicit Agent- and User-driven organization only after
  actual proliferation proves the need.
- A later deliberate action walks from A to B over that Connection. Walking is the
  only current movement method in this proposed slice, so no travel-mode field or
  enum is earned.
- Geographic adjacency is not stored and is not inferred from the Connection,
  Position, Area or prose. It remains later behavior only if a concrete game scene
  needs to distinguish adjacent-but-impassable from connected-but-distant Places.
- The resulting Connection may therefore span any technically valid distance.
  Distance does not yet decide Movement duration, intermediate Position or another
  travel mechanic.
- Grass and dunes gain mechanical effects only when accepted gameplay requires
  movement, material, visibility or other deterministic terrain behavior. An
  independently discoverable plant may already be an Entity.

This choice updates the Place and Movement development direction and backlog only.
It introduces no Terrain or adjacency vocabulary, `game/docs`, schema, capability,
code or public Agent text. The remaining Relation, Area, geometry, chain-bound and
privacy recommendations remain pending User negotiation.

## Subsequent User choice — Entity-relative Position and description

On 2026-08-19 the User accepted the reference behavior that remained open after the
first-slice focus:

- Position is either absolute from permanent World origin or relative to exactly one
  Entity, using the already accepted whole-centimetre `x`, `y` and `z` World axes.
- Entity-relative Position mechanically follows changes to its reference Entity's
  resolved Position. The Agent chooses that structure explicitly; a Relation or
  authored sentence never creates following behavior.
- Place uses the same Position as every Entity role and may therefore be relative;
  a cabin can remain a stable Place while moving with a ship.
- Until Orientation earns its own accepted behavior, relative Position inherits
  translation only and does not rotate local axes with its reference.
- Position may include optional Agent-authored `description` that helps another
  Agent narrate the current spatial situation. The cup example may say, “this cup
  strangely remains exactly two centimetres above the table; nobody knows why,”
  while the exact reference and offsets remain the sole mechanical truth.
- World never parses Position description into coordinates, movement, geometry,
  Relation meaning, access or authority. Every accepted Position change explicitly
  keeps, replaces or removes the current description in the same transaction and
  under the same Position revision. Description shares exactly the Position's read
  eligibility and is never independently queryable.
- Knowing an Entity or remembering an earlier point does not grant its exact current
  Position. Position has an independent Character-specific eligibility boundary;
  denied reads reveal neither Position existence nor reference, offsets or
  description. One positive path is now accepted: a bounded current Place read that
  already returns an Entity includes that Entity's readable Position in the same
  response. It never issues or requires one follow-up Position lookup per Entity.
  Other positive eligibility paths remain open.
- A normal eligible Position read returns the resolved current World point together
  with the basis actually stored. For an absolute Position that is its origin-based
  values; for an Entity-relative Position it is the immediate reference Entity and
  stored offsets. The resolved point is calculated for the read and is never stored
  as a second spatial truth. Behavior when the immediate reference itself is hidden
  remains open.

The existing canonical term Prose still means immutable narrative of one accepted
World action; `description` is the accepted name for this optional current Position
text and does not give Prose a second meaning. Bounded reference-chain work, cycles,
hidden-reference reads, re-referencing, privacy, indexing and hot-reference concurrency
remain future evidence questions. No `game/docs`, schema, capability, runtime or
public Agent text changes.

The User subsequently rejected a proposed fixed maximum of 32 relative Position
references. Reads must avoid retrieving unbounded state, but the number of Relations
and the length of a finite relative Position chain should not gain that product
limit merely for operational convenience. These are distinct structures: Relation
count is never traversed to resolve Position, while an exact absolute World point
does mathematically depend on reaching the Position chain's absolute basis. The
At that point the result returned when one bounded read could not reach that basis remained open, as did
the implementation that proves bounded work without a fixed chain-length rule. A
cycle or genuinely non-terminating chain cannot yield the exact point promised by
Position and is not accepted by this rejection.

The User then accepted the bounded incomplete-read result. Every eligible read
returns the immediate stored Position basis. If bounded work reaches an absolute
basis it additionally returns the exact current World point; otherwise that point is
absent. World never falls back to a stale cached point or descendant-wide coordinate
rewrites, and an Action whose mechanics require the absent point fails closed until
resolution succeeds or an eligible Action simplifies or re-establishes the Position.

The User then accepted a deliberate art-and-game distinction. Relative Position
references never cycle: a cycle cannot establish the one exact point Position means.
The player's spatial experience may nevertheless be intentionally impossible.
Explicit Connections may form loops or join Places whose coordinates are
geographically inconsistent; concrete Actions may later change that topology, and
Agent-authored meaning may narrate it without becoming executable inference. An
infinite staircase, looping doors or a space larger inside than outside therefore
does not require corrupt Position state. Each movement settles one explicit
Connection direction, so one loop never requires a global graph traversal or lock.
SP15 preserves this artistic pressure case. Bounded cycle validation on Position
mutation is fail closed: World accepts the new relative Position only when bounded
work reaches an absolute basis without returning to the changed Entity. A detected
cycle or exhausted proof rejects the whole mutation. The examined Position revisions
must remain current at commit, so concurrent A→B and B→A proposals cannot both win.
The exact PostgreSQL lock and validation strategy remains an evidence question; it
may not introduce a global spatial lock or revision.

The User then accepted explicit complete re-referencing. Changing an Entity from
Table-relative to Tray-relative supplies Tray and all new `x`, `y` and `z` values;
World never guesses whether to preserve the prior World point or prior offsets. The
Agent must also account for eligible authored meaning such as “this cup always
floats two centimetres above a surface.” If available World information does not
ground the tray's surface, the Agent explains the uncertainty and asks the User
rather than inventing geometry. World validates only the complete Position,
revisions and acyclic basis; it neither interprets nor enforces the two-centimetre
promise. Whether that durable meaning is canonically a Trait, Property or another
already accepted authored surface remained the next vocabulary choice. The User
then selected Trait: “this cup always floats two centimetres above a surface” is a
durable, open and non-executable characterization of the cup. It remains with the
cup across Position changes and guides the Agent's complete next proposal. Position
description may separately narrate the current tray arrangement. World interprets
neither text surface as geometry or physics.

The User then clarified that User control does not make the Agent a literal command
executor. The User chooses what their Character attempts and the Agent must
understand and communicate the likely impact, but it may also invent a bounded,
playful result. “Put the cup on the table now” may fail; the Agent may propose that
the cup instead rebounds and floats ten centimetres above it. The creative result
must still name the complete exact Position and what is retained, developed or
replaced in Trait and Position description. World never authors the surprise from
prose and cross-User, privacy, subject and boundedness rules do not loosen. Whether
the initial request itself confirms the surprising result or a changed result needs
a new explicit preview and confirmation remains open inside the wider routing choice.

## Subsequent User clarification — cursed visibility exception

The unconditional recommendation that a User's Agent can always read its own
Character's current Position is not yet accepted. The User immediately qualified it
with deliberately strange World behavior such as glasses that are hidden from their
possessor, possibly extending to loss of the Character's own spatial awareness.

Trait remains the accepted non-executable characterization and may narrate “hidden
from its possessor,” but its wording cannot change a read. The User accepted that
normal own-Character Position eligibility may be overridden by an explicitly
proposed current structural consequence for exact Characters and Entities. World
validates and stores that structure with Activity without interpreting the Trait.
The exception may therefore hide cursed glasses from their possessor or even hide
the affected Character's own Position.

The User then accepted the boundary between free meaning and executable structure.
Trait statements and Position description may carry arbitrary Agent-authored meaning
within ordinary bounded-text rules, but an executable visibility exception separately
names the exact protected World information it hides. World never infers that scope
from text and no universal hidden flag silently hides all Entity state.

The User also accepted that `world_change` is not a game resource, model, table or
generic operation. `World Change` remains only the name of a development Area. A
visibility consequence can exist only inside the confirmed Action that establishes,
changes or ends it, or inside the Introduction that first establishes its cause or
basis. That operation names the exact affected subjects, protected information and
expected current revisions; the resulting current state and Activity commit together
or not at all. There is no standalone visibility editor and no orphaned consequence
whose cause changed without it.

No canonical name or storage form exists yet. The available protected information
scopes and how an affected Character recovers remain open. The User corrected the
cross-User direction: the User-owned Agent must refuse a request to author
perception, knowledge or another current experience for a different User's
Character and explain that a User writes only for their own Character. World cannot
interpret prose to enforce that creative boundary. The User accepted the separate
structural security boundary: when the authored meaning has an executable
consequence, World requires the explicitly affected Character to equal the acting
Character. World compares only those stable identities and makes no semantic
judgment. The User then limited the first spatial scope to current Position: the
Agent names each Entity whose Position becomes unreadable to its own Character, but
the same mechanism cannot hide Entity existence, Relations, inventory, Trait,
Property or any other World state. Those need separate later gameplay and privacy
choices. SP14 preserves this pressure case. No `game/docs`, schema, capability,
runtime or public Agent text changes.

## Working direction — non-default unforeseen outcomes

The User wants playful, genuinely unforeseen outcomes to become an intentional part
of Aicadia rather than an accidental Agent failure. A dungeon-master-like Agent may
not always produce the literal requested result: the floating cup can rebound higher,
and later situations may develop in other surprising but grounded ways. This must
never become the ordinary default for every Action. `Chaos` is the User's working
label only; no canonical term, universal mode, operation or server mechanic is chosen.

The current spatial model already preserves the necessary narrow case. The Agent can
reason from eligible Position, Trait, description, Relations and history, then submit
the exact resulting Position and other bounded state. World remains dumb and strict,
does not roll or invent the twist, and grants no extra subjects, hidden knowledge or
cross-User authority. The wider idea belongs primarily to Agent Play and World Change;
SP04 remains its spatial pressure case rather than expanding the spatial model into a
general surprise system.

This direction is now retained in the Areas and backlog so it cannot disappear. The
User chose to finish the spatial foundation first and run a separate grill afterward.
Spatial retains SP04 as a pressure case but gains no temporary general surprise mode.
The later grill owns invocation, canonical name, preview and confirmation, affected-
subject bounds and relation to concrete Actions. This choice does not amend
`game/docs` or current confirmation behavior.

## Correction — World never understands “hidden”

The User correctly challenged the next lantern-and-boat question because it silently
assumed World could understand that Boat B was hidden. It cannot. A Trait, Relation,
description or Agent explanation never changes a read by meaning. World can withhold
state only when a concrete read rule or explicitly stored structural consequence
names the acting Character, exact subject and exact protected information in data it
can compare deterministically.

The earlier accepted cursed-glasses direction is such a structural possibility: an
Agent may narrate the reason freely, but its confirmed Action separately names that
Character W cannot read Entity G's current Position. World validates identities,
self-only authority, current revisions and bounds, then stores and applies that exact
consequence without knowing what a curse or hidden object means. Its canonical name
and storage are still unchosen. The withdrawn boat question becomes meaningful only
after stating which concrete current rule makes B ineligible; it cannot use “hidden”
as an unexplained premise.

The phrase “stored structural consequence” was itself too abstract. A small
illustrative—not accepted—technical shape makes the candidate concrete. A confirmed
Action could submit the acting Character W, affected Character W, subject Entity G,
the exact protected information `Position` and expected current revisions. In the
same transaction World would write Activity plus one current row whose existence
means only “W cannot receive G's current Position.” A later Place read for W would
join or test that exact `(W, G)` pair before selecting Position; finding the row omits
Position, while not finding it follows the normal Place-read rule. World never reads
the cup, glasses, Trait or prose to make that decision. The row name, columns,
lifecycle and whether row existence is ultimately the best model remain open.

The User added a cross-task Terry rule: every grill answer and option must show the
concrete actor, input, stored or read result and important failure boundary, with a
small technical example whenever state or scale could otherwise remain abstract.
Illustrative technical names and shapes must be identified as non-canonical rather
than silently becoming Aicadia vocabulary or schema.

## Position denial uses sparse Position-specific current state

The User selected the Position-specific option. For the first spatial behavior,
World does not store a generic protected-information kind and does not create a
positive permission row for every readable Character–Entity pair. It stores current
state only when one exact affected Character cannot receive one exact Entity's
Position. Record presence denies Position; record absence follows the normal
current Place-read rule. Entity existence, Relations, inventory, Trait, Property and
Activity are unaffected.

Concretely, a confirmed Action for Character W and Entity G creates or changes the
one current `(W, G)` Position-denial record and its Activity atomically. A paginated
Place read for W tests all returned Entity ids against that sparse set in the same
query and sets only a matching Entity's Position result absent. It performs no
per-Entity protocol calls and writes no rows for millions of ordinary readable
pairs. The table name, exact columns, conflict revision, cause lifecycle and removal
operation remain unchosen; the example shape does not canonize them.

## Position denial reopened as premature spatial complexity

Before choosing how denial propagates through relative Position, the User challenged
the whole branch. A Character-specific Position-denial record, derived-output privacy
and dependency-aware redaction are already an information-access system rather than
the minimum model required to establish where Entities and Places are. Q3 is paused
and the sparse Position-specific direction is no longer treated as current.

The leading simplification is to keep the foundation mechanical and direct: one
Entity has one optional Position; Place requires it; a bounded current Place read
returns every selected Entity with its Position; and relative resolution concerns
coordinates, cycles and bounded work only. Cursed glasses and private inventory
remain recorded pressure cases, but privacy, visibility and selective information
release would receive their own later grill rather than hiding inside Position. This
rollback is not yet accepted; it is the next explicit choice.

## Position privacy removed from the foundation

The User accepted that simplification. The spatial foundation has no Position-
specific denial record, permission, redaction hook, own-Character exception or
recovery path. A concrete bounded read first decides which Entities it returns. For
each selected Entity it returns the complete Position when one exists, including its
optional description and immediate stored basis. Knowing an Entity or remembering a
prior point does not create a separate current-Position lookup.

A Trait or Position description may tell a conforming Agent to withhold a spatial
detail in its player conversation. World nevertheless sent the structured Position
to that Agent. This is therefore creative Agent conduct, not confidentiality,
authorization or protection from a modified Agent, and it may never be presented as
evidence of World-enforced privacy. Cursed glasses and private inventory remain
pressure cases for the dedicated later privacy grill and backlog item.

The next open model boundary comes from the User's suggestion that Position might
itself have Traits or several descriptions. Current vocabulary still gives multiple
stable Traits only to the Entity and gives its one current Position one optional
description with no independent identity or revision. No alternative has been
accepted yet. The next question must compare the concrete lifecycle, read and write
consequences rather than introducing another spatial term by example.

## One Position description; Traits remain Entity-owned

The User selected option A. One current Position has at most one optional description,
and that text may contain multiple sentences or paragraphs within ordinary bounds.
It remains one part of Position: a Position change explicitly keeps, replaces or
removes it under the same revision and transaction. It has no independent identity,
authorship lifecycle, ordering, pagination or conflict surface.

An Entity may separately have multiple Traits. Durable meanings such as “always
floats two centimetres above a surface” remain Traits of the cup and survive every
Position change. Current narration such as “after resisting the attempt, it now
floats ten centimetres above the oak table” belongs in the one Position description.
Position itself owns no Traits and does not become a second World subject.

This leaves one immediately adjacent spatial boundary open: an Entity-relative
Position currently names only the reference Entity and offsets from its one Position
point. The phrase “above the table surface” may therefore be Agent-understood meaning,
or a future exact sub-Entity reference may eventually earn structural state. No such
reference concept or name is accepted by this observation.

## Relative Position uses only the reference Entity's Position point

The User selected option A. Entity-relative offsets start at the reference Entity's
one Position point. The foundation contains no targetable surface, part, internal
point or geometry, and World never converts “above the tabletop” into one. An Agent
may understand Entity Properties, Traits and descriptions and submit the numerical
offset it intends, but World stores and validates only that explicit Position.

Concretely, if Table T's Position point is at its base and an Agent understands its
top to be 75 centimetres higher, the Agent may propose Cup C relative to T with
`z = 77 cm`. World does not derive either 75 or 2 from text. If the table's understood
shape later changes, no automatic Position change occurs; an Agent must explicitly
propose the new offset. This keeps one Position row and one reference-chain read per
positioned Entity at scale. Exact surfaces, internal points, Orientation and geometry
remain absent until concrete gameplay earns them, without a dormant placeholder.

Under the now-confirmed foundation, six material User choices remain before this
spatial grill can crystallize one coherent technical and database candidate:

1. the exact concept and canonical one-word name for direct Place topology;
2. the identity and storage boundary between open authored Relation meaning and
   exact mechanical spatial facts;
3. whether Area belongs in the first foundation and, if so, its minimum meaning;
4. which Entity earns the Place role and what a bounded map read may reveal;
5. the first exact movement result over direct Place topology; and
6. the spatial boundary of remote consequences such as a button and distant bomb.

Accepting the leading recommendation on each keeps that count at six. A choice that
introduces a new mechanical concept can open a necessary dependent question; numeric
width, indexes, PostgreSQL concurrency and evidence are technical design and proof
work rather than User questions unless they expose a new game trade-off.

## Connection is the dedicated direct Place-topology primitive

After clarifying the other candidate Relation meanings, the User selected option A.
`Connection` is the canonical one-word name for a separately owned mechanical fact
between two Places. It explicitly records which direct travel direction exists; it is
not an Entity, Route, `Link`, open authored Relation or server-owned Relation type.
Position, proximity, Area overlap and prose never create it implicitly.

Concretely, A→B and B→A are distinct allowed directions. One bounded read of A's
outgoing Connections can offer B as a direct option without scanning Relations or
the World. A road, bridge or door may remain an Entity and an Agent-authored Relation
may describe how it leads to B, but neither substitutes for A→B. Concurrent attempts
to establish the same direction settle on that exact Place pair; unrelated Places
share no counter, lock or global graph revision.

This choice fixes conceptual ownership, not a production schema or operation. It
removes direct Place topology from the next Relation decision. Five material User
choices remain on the leading path: the open Relation record itself, Area scope,
Place qualification/map reads, the first Movement result and remote-effect boundary.

## Open Relation has one stable non-Entity identity

The User selected option A. Relation is now the canonical name for one stable,
directed, non-Entity World record between a source Entity and target Entity. Its name
and description are free Agent-authored English rather than a server-owned semantic
kind. World validates identity, endpoints, ordinary bounds, authority, freshness and
the exact Relation being changed, but never understands or executes the wording.

Several Relations may coexist between the same Entity pair. Renaming or developing
“sleeps under” keeps the same Relation identity rather than deleting one natural-key
tuple and creating another. Activity can name that exact Relation as involved history;
original authorship stays attributable but grants no permanent exclusive edit right.
Connection, Position and every later Inventory or remote-effect mechanic remain
separate exact facts and never gain authority from Relation wording.

At scale, one Relation change conflicts only on that Relation and updates no endpoint
counter, revision or lock. Reads must name one endpoint, direction, cursor and limit;
a hot bridge can have many Relation rows and a hot endpoint index range without
serializing unrelated Entity or Connection changes. This accepts the domain record,
not a production table or public operation. Exact text bounds, dependencies, privacy
and duplicate presentation remain later design work.

Four material User choices remain on the leading path: Area scope, Place
qualification/map reads, the first Movement result and the remote-effect boundary.

## Area choice withdrawn; traversal context expands the question

The User rejected all three Area options as premature. Deferring Area, immediately
building complete geometry and canonizing simple circles or rectangles each failed
to address the more important desired experience: the World should eventually be
able to describe where Places fall, what their spatial coverage looks like and what
landscape a Character passes through while traversing a Connection.

That clarification separates at least three candidate truths. A Place may have an
extent or shape. Several Place extents may include or overlap another Place or point.
A Connection may have enough ordered or spatial state to say that its traversal
crosses forest edge, woodland and heath. The Connection's A and B endpoints alone
cannot derive that middle, and an Area attached only to A or B does not supply it.
None of those observations chooses geometry, hierarchy, a path record, traversal
segments or a new canonical term.

Q3 is withdrawn rather than counted as answered. Place qualification, Movement and
the remaining remote-effect boundary now depend on resolving which of these facts
are stored and which are calculated. Existing primary-source research is being
extended specifically across Place extent, overlap and Connection traversal before
a replacement one-question frontier is presented. The number of remaining User
choices will be recomputed from that evidence rather than pretending it remains four.

## Focused extent and Connection-traversal research

The completed [primary-source report](../research/place-area-connection-traversal.md)
confirms that Place extent, inclusion or overlap and Connection traversal are three
independent truths. OGC and PostGIS distinguish coverage, containment, touching,
crossing and overlap; geometry therefore yields many candidates rather than one
Place parent. Overture likewise separates identity, representative point, extent,
boundary, hierarchy, oriented transportation shape and partial linear rules.

Endpoint topology cannot answer what lies between A and B. Two credible sources of
ordered traversal context remain. An ordinary Connection can carry an exact oriented
course, after which current Place extents are intersected and ordered along it. Or an
Agent can author structured ordered intervals along the Connection even without
complete geometry and across impossible topology. The first follows boundary changes
but needs valid compatible geometry and bounded spatial work. The second supports
incomplete Worlds and portals but can become inconsistent with later extents.

Storing both as equally current truth is not a free hybrid: World would need a
deterministic authority rule. Materializing every course/Area intersection creates
large boundary-change fan-out, while deriving on read costs a bounded spatial join.
Indexes bound candidate work, never result cardinality; repeated and overlapping
intervals must be permitted and paginated by course position plus a stable tie-breaker.

The replacement frontier first chooses the source of ordinary Connection traversal
context. Under the leading derived-course direction, one subsequent question can
decide Area extent and inclusion semantics before returning to Place map reads,
Movement and remote effects. The leading path therefore currently has five material
User choices rather than the withdrawn estimate of four.

## Connection-course ownership reopened by travel choice and ceremony

The User challenged the leading course candidate before selecting it. One exact
course attached to A→B can be false because walking a mountain path, sailing a river,
flying directly and entering a portal may connect the same Places through different
or nonexistent intervening space. Making that course mandatory on every Connection
is establishment ceremony; requiring every repeated journey to resubmit a stable
road is execution ceremony.

The question therefore begins with ownership, not geometry. One candidate makes a
Connection one persistent direct travel alternative rather than the unique endpoint
pair. A footpath, ferry and portal may then be separate stable Connections from A to
B; the first two may have reusable spatial shape and the portal may honestly have
none. This reopens the earlier inference that the directed Place pair alone is always
Connection identity. The accepted first discovery's duplicate A→B rule remains its
narrow result and does not yet prove a universal ban on parallel Connections.

Other possible owners remain a physical Entity such as a road, river or tunnel, or
the exact Movement result taken by one Character. Useful shape families also answer
different questions: ordered points or a line describe one centre course; a corridor
describes possible width or exposure rather than exact footsteps; ordered intervals
describe stages without geometry; an actual Position trace describes one completed
journey; and a 3D tube or volume would be later geometry. None is chosen or canonized.

The replacement Q3 must now ask which subject owns reusable travel shape. Only after
that can Area intersections and ceremony be judged honestly. Under the leading
one-Connection-per-alternative candidate the remaining choice count can stay five;
another owner may expose a dependent identity or lifecycle question.

## Connection owns one optional reusable shape

The User selected the one-Connection-per-alternative direction. A Connection is now
one stable direct travel alternative rather than the unique relationship between an
endpoint pair. A walking path, ferry and portal may be three Connections between the
same Places. Each states its own allowed direction and may optionally own one exact
reusable spatial shape; a portal or an incompletely mapped alternative may honestly
have none. The physical road, bridge or vessel remains an Entity, an actual Movement
chooses a Connection without restating its reusable shape and a later Route may
compose several Connection identities.

This selection removes pair uniqueness from general Connection identity. The first
discovery slice still establishes at most one required A→B result because that
capability currently offers only one undifferentiated walking alternative; it does
not justify a database uniqueness rule over every endpoint pair or prohibit a later
capability from establishing another deliberate alternative.

One minimal technical candidate follows, but its names and fields are illustrative
rather than accepted Aicadia schema. A `connection` record has a stable id, two Place
endpoint ids, explicit allowed direction and its own revision. Its optional bounded
ordered exact shape is stored under that same Connection identity, either in the row
or in one keyed optional record; the physical layout is still open. It also needs a
freshness rule for the endpoint Positions against which the shape was established.
A later Movement proposal can submit the chosen Connection id so World can validate
the Character's origin, allowed direction and exact current revisions. Its exact
resulting Position and duration remain a later choice; whatever state it eventually
changes must settle with Activity in one transaction. No per-journey trace is earned
before travel-over-time gameplay needs one.

When Places later have exact Areas, World can intersect the selected Connection shape
with eligible current Areas and return ordered crossings on demand. Those crossings
are calculated output, not duplicated current rows, so an Area edit causes no global
fan-out. A missing shape means “not established or not applicable,” never “the
straight endpoint line” and never “known to cross nothing.” Reads remain bounded by
one Connection, cursor and limit. Concurrent changes conflict on that exact
Connection, its shape or the Position revisions actually examined; millions of
Movements may read one stable alternative without locking its endpoint Places, while
genuine concurrent edits to that one hot alternative may reject and retry.

Still open are the smallest exact shape representation, whether it is two- or
three-dimensional, its endpoint alignment and change rules, Area extent and overlap
semantics, how the first discovery's narrow duplicate rule is enforced without a
general endpoint-pair uniqueness constraint, authorship and visibility, and the
concrete World interface. Parallel alternatives also expose one prior question: an
Agent needs minimal readable state that distinguishes “footpath,” “ferry” and
“portal” without showing internal ids or turning Connection into an Entity. That
presentation state is the next frontier before exact shape representation. No
subordinate domain term, production table, capability or current game contract is
selected by this technical sketch.

## Connection carries its own readable meaning

The User selected the minimal self-describing alternative. Every Connection has one
Agent-authored name and description, allowing several alternatives between the same
Places to be presented as “Old Forest Path,” “Moon Ferry” or “Cracked Portal” rather
than opaque ids. Connection still is not an Entity and therefore owns no Properties
or Traits. A physical road, boat or portal device may independently be an Entity.

World stores and returns the text but never interprets it as travel method,
direction, access, cost, timing, geometry or causality. Those mechanical facts must
be separately explicit when gameplay requires them. Exact text bounds, later change
operations, authorship and visibility remain open rather than being smuggled in by
the two fields.

Technically, a bounded Connection read can now return stable id, endpoint Places,
allowed direction, name, description and whether reusable spatial shape exists. An
Agent can explain the alternatives to the User, retain the selected stable id only
inside its subsequent structured command and avoid exposing protocol identity in
player conversation. The database still permits several rows with the same endpoint
Places; names are presentation and never uniqueness keys.

The next frontier is the smallest exact representation of the optional reusable
shape. Ordered three-dimensional World points are the leading candidate because they
can express a curved or elevated representative course with little ceremony and let
later Area crossings be calculated. A corridor adds width or volume, while authored
crossing intervals duplicate changing Area truth; none is selected yet.

## Connection shape uses ordered World points

The User selected the ordered-points candidate. When a Connection has reusable
spatial shape, it contains a bounded ordered sequence of exact whole-centimetre
`x`/`y`/`z` World points. World treats consecutive points as straight segments. The
result is one exact representative course for that travel alternative: it can curve,
rise and descend, but it does not claim the road's width, every reachable location or
one Character's exact footsteps. A portal may continue to have no shape.

The User additionally accepted at most one optional Agent-authored description on
that shape. It can tell another Agent, for example, that the trail clings to a cliff
or that nobody hears birds along its middle. It remains part of the Connection shape
state, has no independent identity, revision, read or lifecycle and is changed with
the points. World stores and returns it but never derives coordinates, Area
crossings, access, danger or movement behavior from the text.

This rejects a corridor or full 3D volume as the foundation and rejects stored
authored Area intervals as co-equal crossing truth. When exact Areas exist, the
ordinary result is derived by intersecting them with the current ordered segments.
Missing shape remains unknown or inapplicable rather than an inferred straight line.
Exact point count and integer range, endpoint alignment, behavior after an endpoint
Place moves, physical PostgreSQL representation and operations remain technical or
dependent design work rather than accepted schema.

Four leading material User choices remain: Area coverage and intersection meaning,
Place qualification and bounded map reads, the first Movement result, and the remote
effect boundary for examples such as a distant bomb button. The count can grow only
when a selected answer genuinely opens a new product concept.

## Area proves positive coverage only

The User selected Area as exact positive coverage rather than a complete boundary or
descriptive-only map. A point inside established Area is proven to fall in that
Place. A point outside all established coverage remains unknown: the Place may end
there, the boundary may still be undiscovered or another Place may also cover it.
Missing Area means that no spatial coverage is established, never that the Place is
empty or point-sized.

Overlapping positive Areas may all be true. A village can lie within forest coverage,
a marsh can overlap both and an incompletely mapped heath can begin before the forest
is known to end. World does not choose a universal parent, merge Places or infer
movement, visibility or ownership from overlap. An Agent authors which Place the
coverage means; World validates only exact bounded geometry and current revisions.

For the selected Connection points, derived traversal output can prove each ordered
portion intersecting eligible current Areas and mark every uncovered portion as
unknown. Those intersections are not current rows. An Area extension or correction
therefore changes later calculated results without rewriting or locking all affected
Connections. At scale, candidate Areas come from a spatial index, results are bounded
and paginated, and the read identifies exact Connection and Area revisions rather
than borrowing a global map revision.

The geometry representation, disconnected coverage, vertical behavior, concrete
operations and visibility remain technical or dependent work. Three leading material
User choices remain: which positioned Entity earns the Place role and what a bounded
map read returns, the first Movement result and the remote-effect boundary.

## Place is a deliberate role with bounded map reads

The User selected deliberate Place establishment rather than a server kind list or
automatic promotion of every positioned Entity. Any Entity with Position may become
a Place when Agent and User establish it as an independent map, discovery, navigation
or explicit spatial reference. A city, forest, cave, waterfall, ship or exceptional
small object can qualify; none does so merely because of its name, description,
Position or inferred category.

Place continues to use `entity_id` as the stable subject identity. Entity owns its
name, description, Properties and Traits; the Place role adds only its requirement
for Position and eligibility for Area and Connections. World validates exact
identity, Position, authority, current revision, bounds and confirmation and commits
the role with Activity. It never judges whether the subject is sufficiently large,
important or location-like.

The selected map consequence is an exact bounded coordinate-window read, not an
unbounded `list all Places`. A Place is a spatial candidate when its resolved current
Position falls within the requested World window or its known positive Area
intersects that window. The result can carry bounded Entity name and description,
complete eligible Position, relevant Area coverage and Connections touching selected
Places, with stable continuation for independently limited collections. Ordinary
positioned cups, grass, fauna and Characters without the Place role do not enter this
map index.

This coordinate-window read remains distinct from Place neighborhood, which begins
at one exact Place and follows explicit structural relationships rather than metric
geometry. At scale, Place-role writes touch only the exact Entity; spatial indexes
bound candidates, cursors bound results and one hot city produces real paginated load
without a global map row, count, revision or lock.

Exact window shape, limits, cursor contract, visibility and public operation names
remain technical or dependent choices. Two leading material User choices remain:
the first Movement result and the remote-effect boundary.

## Movement may stop on a Connection without journey state

The User selected partial-or-complete Connection Movement. For a shaped Connection,
the Agent may propose the destination endpoint or one exact whole-centimetre point on
a named segment of its ordered course. The proposal names the exact Connection and
Character Position revisions. World proves that the Character begins at the allowed
endpoint or current segment, that the target lies on the submitted segment and that
progress follows an allowed direction. It derives none of those facts from name,
description or narration.

Accepted Movement replaces the Character's one Position and stores Activity in the
same transaction. An intermediate stop is an ordinary unnamed Position: the
Character can meet others, discover, place or act there without establishing a Place
or a durable journey. A later Movement names and revalidates the Connection again;
there is no current Connection membership, percentage, departure time, timer,
background process or completed trace. Leaving the course is a separate explicit
Movement. A shapeless portal or impossible alternative has no valid middle and can
only move directly between endpoints.

At arrival, the Agent supplies the complete new Position and may explicitly make it
relative to the destination Place so a moving cabin or ship can carry the Character.
World never silently rebases or invents offsets. Connection Areas and description
give the Agent grounded journey context, while World validates only the exact points,
direction, revisions and authority.

At scale, thousands of travellers read one Connection revision and update only their
own Position rows and Activities. They share no traveller count, progress row or
course lock. A real concurrent edit to the hot Connection makes dependent proposals
stale; it does not serialize quiet Characters or unrelated geography.

Exact point-membership arithmetic, segment and endpoint rules, no-op behavior,
operation names, limits, errors and observation remain technical or dependent work.
One material User choice remains: the remote-effect boundary for the distant button
and bomb.

## Remote causality remains Agent-understood Relation meaning

The User rejected a new typed remote-control basis as unnecessary ceremony and
refined the Relation option: Relation may explain to an eligible Agent that Button B
activates distant Bomb X, but it is not technically executable and grants no World
authority. This is now also the preferred lightweight foundation. It keeps the
accepted open Relation honest instead of quietly adding a rule engine inside it.

Concretely, the Agent reads the current B→X Relation and understands its name and
description. When the User presses B, the Agent previews one bounded Action naming B,
X, their expected revisions and every exact intended mutation. It may name the
Relation identity and revision as claimed causal context; World can then prove that
the same eligible directed record still exists, but never that its words mean
“detonate.” After explicit User confirmation, World applies ordinary Entity
eligibility, authority, freshness, idempotency and bounds. Relation never widens
those permissions, so an otherwise forbidden Bomb change is still rejected.

If accepted, all named state and one attributable Activity settle atomically. The
Activity can involve Button, Bomb and their current Places so later reads establish
where the local act and remote result occurred. World infers no radius, physics,
extra victim, chain reaction or delayed work. Distance, Position, Area, Connection
and Route neither grant nor block the causal result; they only ground the involved
subjects and occurrence context.

This choice has an explicit limitation: a modified or mistaken Agent may assign the
wrong meaning to an eligible Relation, and World cannot catch that semantic error.
User preview plus ordinary authority is sufficient for the open, freely authored
foundation; it is not evidence for secure high-impact control over another User's
protected state. A future concrete mechanic may earn typed authorization only when
that actual gameplay invariant exists. No remote table, universal operation enum,
rule engine, background Agent or spatial-causality concept is added now.

The spatial product-choice frontier is now empty. The technical synthesis below
turns the choices into one recommended implementation shape and separates its proof
gates from accepted truth. None of it silently authorizes a build.

## Technical synthesis after the completed grill

### Standing

This section is the current technical recommendation. It is deliberately more
specific than the accepted domain direction so a productionward lab and later plan
can falsify it. It is **not** a production schema, public contract or implementation
authorization. Field names and table shapes below are candidates; the accepted
meaning remains in the later choice sections above and in the owning Areas.

My preference is one small relational model in PostgreSQL, not a universal spatial
document, graph database, spatial microservice or general Relation engine:

```text
Entity ── zero or one Position ── World or one reference Entity
   │             │
   │             └─ resolved World point is calculated, never a second Position
   │
   ├─ optional Place role ── optional Area
   │                         └─ zero or more Connections to other Places
   │
   └─ zero or more open Relations to other Entities

Character Movement changes Position and optional current Place context.
Every accepted mutation writes Activity and exact typed dependencies atomically.
```

This is layered storage, not layered ceremony for the Agent. One World read or one
confirmed Action may compose several rows and expose one coherent game operation.

### One technical home for each truth

| Truth | Durable identity | Current state | History and conflict boundary |
| --- | --- | --- | --- |
| Entity | `entity.id` | name, description and its established Entity state | Existing Entity and Activity rules |
| Position | the Entity identity; no Position id | zero or one exact point, reference and optional description | one Position revision for that Entity |
| Place | the same Entity identity | deliberate role only | role establishment Activity; Position remains separately revisioned |
| Area | preferably the Place identity; no extra Area id until independent lifecycle is proved | optional positive coverage | one Area revision for that Place |
| Connection | its own stable non-Entity id | immutable Place endpoints plus versioned direction, text and optional course | one Connection revision; parallel endpoint pairs remain possible |
| Relation | its own stable non-Entity id | immutable Entity endpoints plus versioned free name and description | one Relation revision; no semantic kind |
| current Place | the Character or Entity association that concrete local play establishes | optional exact Place context | changes only with the concrete operation that establishes or leaves it |
| Activity | its own immutable id | never current state | exact accepted operation, actor, subjects, typed dependencies and prose |

No generic `spatial`, `edge`, `fact`, `rule`, `frame`, `containment` or
`world_change` table sits above these records. Shared implementation helpers may
normalize text, revisions and history, but the database keeps each invariant in its
own table family.

### Position candidate

The candidate follows the repository's current immutable-version plus current-pointer
pattern. `reference_entity_id = NULL` means the permanent World origin; a value means
the three coordinates are offsets from that Entity's Position point. No enum, World
row or coordinate-frame Entity is needed.

```sql
position_version(
    entity_id               uuid,
    activity_id             uuid,
    previous_activity_id    uuid null,
    reference_entity_id     uuid null,
    x_cm                    bigint,
    y_cm                    bigint,
    z_cm                    bigint,
    description             text null,
    primary key (entity_id, activity_id)
)

position(
    entity_id               uuid primary key,
    current_activity_id     uuid,
    foreign key (entity_id, current_activity_id)
        references position_version(entity_id, activity_id)
)
```

Required database invariants are conventional and exact:

- both tables reference the same existing Entity and every version references its
  Activity;
- `reference_entity_id` references an Entity and may not equal `entity_id`;
- one partial unique index permits one root version per Entity and another permits
  at most one successor for each previous version;
- the current pointer names an immutable version from the same Entity;
- `x_cm`, `y_cm` and `z_cm` are signed whole centimetres; and
- the optional description uses ordinary bounded, trimmed text rules and changes
  under the same Position revision.

An Entity without rows has no Position. The first slice should not invent an
`is_positioned` flag, tombstone or removal operation. If actual gameplay later needs
an Entity to become unpositioned again, that lifecycle must first define its history
and read meaning.

I prefer `BIGINT` storage plus one symmetric server constant below the arithmetic
limits over decimal metres, floating-point canonical coordinates or cell identities.
That constant is a technical representability bound, not a gameplay travel limit.
The proof must select it so reference addition, subtraction and exact segment
products can use checked `i128` arithmetic without overflow. A Connection may still
span any distance inside that representable World.

The common addressed read joins the one current pointer to its immutable version.
It returns the stored reference and offsets and, only when bounded resolution reaches
the World origin, the calculated current World point. It does not persist that point
back into Position.

### Resolving a relative Position

There is no accepted semantic maximum chain depth. There must nevertheless be a
bounded request. The first productionward candidate is a terminating recursive read
under a database work and time budget, followed by stable locks and an exact recheck:

1. Starting at the selected Entity, read the current Position chain until a
   World-referenced version, a missing Position or a repeated Entity is found.
2. Stop and fail the resolution if the request's resource budget is exhausted. This
   is operational admission, not a claim that the deeper World state is invalid.
3. Collect the changed Entity and every Position dependency, sort their Entity ids,
   and acquire compatible row locks in that order.
4. Re-read every current Position revision after locking. Reject stale input rather
   than retrying an unbounded number of times inside the request.
5. Reject a repeated Entity. Otherwise add every offset with checked integer
   arithmetic and reject overflow or a result outside the technical coordinate bound.

For a Position write, the changed Entity needs a writer lock while ancestors need
reader locks that block their Position writers. The kept first lab compared
`FOR NO KEY UPDATE` for changed Entity coordinator rows with `FOR SHARE` for
dependencies, acquired in one Entity-id order. That exact scratch fixture survived
its foreign-key and race falsifiers. A production plan must still re-establish the
result against the real Entity foreign keys, Position operation and Activity
dependencies rather than importing the lab implementation.

This solves the concurrent A→B and B→A case without a global graph lock. Both
transactions first discover their candidate chains, but the common ordered lock set
forces one to recheck after the other. The second then sees the cycle and writes
nothing. Moving Ship conflicts with an operation only when that operation actually
requires Ship's resolved external Position. Cabin-local work that does not use that
point never traverses or locks Ship's external chain.

### Place and current Place remain separate from Position

The existing `place.entity_id` is the correct identity shape. When Position becomes
current behavior, Place establishment must require a current Position for the same
Entity. The simplest schema constraint is a foreign key from `place.entity_id` to
the current Position row, provided the migration first positions every existing
Place. Place stores no `x`, `y`, `z`, duplicate name or duplicate description.

Position does not replace current Place. A Character halfway across heath has a
Position and no newly established Place context. Entering destination B changes the
Character's Position and its optional current Place context in the same Movement
transaction. Merely resolving to B's coordinates or falling inside B's future Area
does not perform that change.

The current `place.latest_activity_id` is not a suitable spatial revision or
long-term coordinator. It makes unrelated activity at one hot Place share one row.
Position, Connection, Area and Relation each need their own current revision; a
later build must not expand the existing Place-wide revision into a universal
spatial lock.

### Connection candidate

Connection needs a stable identity because parallel alternatives can share the same
Places and later development must address exactly one of them. Its endpoints define
the identity and should be immutable; changing an endpoint creates another
Connection. Direction, text and course can develop through immutable versions.

```sql
connection(
    id                       uuid primary key,
    start_place_entity_id    uuid,
    end_place_entity_id      uuid,
    current_activity_id      uuid,
    check (start_place_entity_id <> end_place_entity_id)
)

connection_version(
    connection_id            uuid,
    activity_id              uuid,
    previous_activity_id     uuid null,
    allows_start_to_end      boolean,
    allows_end_to_start      boolean,
    name                     text,
    description              text,
    shape_description        text null,
    primary key (connection_id, activity_id),
    check (allows_start_to_end or allows_end_to_start)
)

connection_point(
    connection_id            uuid,
    connection_activity_id   uuid,
    ordinal                  integer,
    x_cm                     bigint,
    y_cm                     bigint,
    z_cm                     bigint,
    primary key (connection_id, connection_activity_id, ordinal)
)
```

Zero points means no spatial course and therefore only direct endpoint Movement.
A shaped version has a bounded sequence of at least two points with contiguous
ordinals. Points are ordered from `start_place_entity_id` toward
`end_place_entity_id`; reverse travel reads them backward. One deferred validation
can prove zero-or-at-least-two points and contiguous ordinals for only the affected
Connection version. Endpoint alignment is validated by World against the exact
endpoint Position revisions when the course is established or used; a Place move
does not silently rewrite Connection history.

The endpoint indexes are separate and directional so one bounded Place read does
not scan every Connection. There is no unique endpoint-pair constraint because a
footpath, ferry and portal may coexist. A concrete discovery operation that must
reuse rather than duplicate an already suitable Connection needs its own
opportunity- or endpoint-scoped conflict rule; global Connection uniqueness would
contradict the accepted parallel-alternative behavior.

### Exact Movement over a Connection

One Movement proposal needs only the exact Connection id and revision, Character
Position revision, allowed direction, target segment ordinal and target World point.
World performs this deterministic sequence:

1. select the named current Connection and hold a shared dependency lock;
2. resolve and lock the Character's current Position dependencies;
3. prove the current World point is the allowed endpoint or lies on the current
   course;
4. prove the submitted target point lies exactly on the named segment;
5. compare the current and target progress along the ordered points in the selected
   direction; and
6. write one new Character Position version, optional destination current Place and
   one Activity atomically.

Exact segment membership does not require floating point. With bounded integer
coordinates, checked `i128` cross products prove collinearity and dot-product bounds
prove the point falls between the segment endpoints. If the current point occurs at
several different positions on a self-crossing course and direction is therefore
ambiguous, the small first contract should reject the proposal. A second
`current_segment` input earns its place only when an accepted self-crossing scene
requires it.

Thousands of travellers may hold compatible shared locks on one Connection revision
and update different Character Position rows. They share no traveller row, count,
progress or endpoint Place lock. A Connection editor waits for those short
transactions; after its commit, proposals carrying the older revision fail stale.
The lock-manager cost and editor fairness on a deliberately hot Connection remain a
real PostgreSQL proof gate.

### Relation candidate

Relation uses the same identity/version pattern without becoming a common base table
for Connection, Position or Area:

```sql
relation(
    id                       uuid primary key,
    source_entity_id         uuid,
    target_entity_id         uuid,
    current_activity_id      uuid
)

relation_version(
    relation_id              uuid,
    activity_id              uuid,
    previous_activity_id     uuid null,
    name                     text,
    description              text,
    primary key (relation_id, activity_id)
)
```

The endpoints are immutable; free name and description may develop. A Relation may
refer from an Entity to itself because open authored meaning supplies no mechanical
reason to forbid that shape. There is no
`type`, `kind`, predicate enum, Properties container or executable payload. Multiple
rows between the same endpoints are valid. Bounded indexes support `(source, id)`
and `(target, id)` pages; no operation returns every Relation of a hot Entity.

When an Agent uses Button B's Relation to explain a Bomb Action, Activity may cite
the exact Relation id and revision. World verifies that eligible directed record is
still current, but the Bomb mutation independently passes ordinary subject,
authority and revision rules. The Relation row is never a permission row and its
text is never evaluated.

### Area candidate boundary

Area has accepted meaning but not yet an exact geometry representation. My
preference is therefore **not to put placeholder JSON, prose, circles or rectangles
into the first schema**. That would make the lightweight choice look complete while
quietly weakening “exact positive coverage.”

When an Area-requiring scene becomes current, the smallest storage boundary is one
current Area per Place, keyed by `place_entity_id`, plus immutable Activity-backed
versions. One coverage value may contain bounded disconnected components; an extra
stable Area id is earned only if components later need independent authorship,
visibility or lifecycle.

The first geometry comparison should include an exact three-dimensional point
predicate. A two-dimensional footprint alone cannot distinguish a cave, bridge and
surface sharing `x` and `y`. My leading practical candidate is bounded horizontal
polygon components with explicit inclusive vertical intervals, using a spatial
index only to find candidates and integer/accepted-geometry checks for the final
positive result. A full arbitrary 3D solid is more expressive but should lose unless
a current scene proves that the simpler volumes cannot represent it.

Connection-to-Area traversal context remains derived output. World intersects one
exact Connection revision with bounded current Area revisions and returns ordered
covered intervals; overlapping Areas may both appear and every uncovered interval
is reported as unknown. No crossing table becomes canonical merely to make narration
cheaper.

### Activity and typed dependencies

Every accepted mutation keeps the existing one-Activity transaction. The spatial
extension should add exact typed dependency rows rather than one polymorphic JSON
payload or generic foreign key:

```text
Activity
  ├─ activity_entity roles for actor, moved subject, source, target and destination
  ├─ exact Position versions used to resolve an occurrence point
  ├─ exact Connection revision used by Movement
  └─ exact Relation revision cited as Agent-understood causal context
```

An operation that used a resolved World point stores the ordered Position-version
chain it actually validated. That freezes where the occurrence was grounded without
copying a second current Position or letting a later ship move relocate history. An
interior operation that never used Ship's external point stores no counterfeit Ship
dependency.

The candidate tables are narrow and typed, for example
`activity_connection_revision`, `activity_relation_revision` and an ordered
`activity_position_revision`. Their exact names belong to a plan, but their foreign
keys must reach the concrete immutable version rows. The Activity and all current
pointers commit together or nothing does.

Existing `(requested_by_user_id, request_id)` idempotency remains the correct retry
scope. The canonical fingerprint covers every exact subject, expected revision,
coordinate, direction, segment, Relation dependency and intended mutation. Reusing
one request id with different input fails; an exact retry returns the original
accepted result without another Activity.

### World reads and operations

Database separation must not turn into Agent ceremony. My preferred World surface
has these properties:

- every bounded Entity or Place read that selects an eligible Entity includes its
  complete Position when present; there is no separate `get_position` capability;
- Position output contains stored `x`, `y`, `z`, optional reference Entity,
  description, revision and the resolved World point when available;
- one bounded coordinate-window Place read returns only deliberately established
  Places, their eligible Position and bounded independently paged Area and Connection
  context;
- one bounded Connection read returns exact alternatives from one Place, not a
  calculated multi-Connection Route;
- one bounded Relation page filters by exact endpoint, direction, cursor and limit;
  unlimited durable Relations never mean an unlimited response; and
- mutations remain concrete game operations such as discovering a Place or moving a
  Character. There is no public universal spatial patch.

Eligibility is decided before hydration. If a Character is not eligible to know a
hidden Entity or Relation, its Position, endpoint, count, cursor gap and detailed
error remain unavailable. The current foundation has no World-enforced private
Position subset: once an Entity is legitimately selected, its Position is returned
whole. Agent narration may be selective, but it is not a security boundary.

### The candidate against all fifteen spatial scenarios

| Scenario | Exact technical composition | Honest remaining boundary |
| --- | --- | --- |
| SP01 · A to B | Place Positions, one selected Connection revision, Character Position change and optional destination current Place | timing and travel methods remain absent |
| SP02 · Entity in coat | Entity-relative Position can make it follow the coat; Relation can describe “inside” | private contents, listing and removal need a later mechanic |
| SP03 · distant bomb | Button and Bomb Positions plus an optional current Relation dependency and one exact Action | Relation neither authorizes nor executes |
| SP04 · floating cup | table-relative exact Position and Position description; the durable anomaly remains a cup Trait | no surface geometry or automatic physics |
| SP05 · dog under bridge | independent Positions and an open Relation or Agent-derived narration | World can derive “under” only after exact geometry earns a predicate |
| SP06 · one hundred metres to hotel | resolved World points calculate straight-line distance; a Connection course can calculate course distance | estimate, path choice and arrival meaning stay explicit |
| SP07 · birdhouse on table | birdhouse Position references table; table may reference another Entity; Activity records exact changed lineage | deep-chain latency and map projection remain proof gates |
| SP08 · forest edge to heath | Character Position plus overlapping positive Areas and derived course intersections | exact Area geometry is deliberately not faked now |
| SP09 · moving ship | ship root Position changes once; cabin and passengers resolve through references | private cargo and moving-Place index repair remain separate |
| SP10 · hidden sword | remembered Knowledge stays non-current; no eligible Entity selection means no Position or Relation hydration | World-enforced inventory privacy needs its dedicated model |
| SP11 · A connects to B, not C | exact Connection rows answer direct travel; absence of A→C returns no known alternative | absence never proves geographic distance or impossibility outside the eligible view |
| SP12 · hot unnamed point | each Character owns one Position row; bounded indexed pages return candidates | no count or promise to enumerate the million matches |
| SP13 · city, forest and waterfall | each qualifying Entity deliberately gains Place, uses its Position and may later gain Area | ordinary positioned Entities do not enter the Place map |
| SP14 · cursed glasses | Trait and Agent narration preserve the curse without altering Position | conforming-Agent withholding is not World-enforced secrecy |
| SP15 · impossible staircase | Connection topology may cycle or form impossible loops; each Movement still writes one exact Position | Position-reference cycles remain invalid and no background infinite journey runs |

The unresolved boundaries are not failures hidden inside Relation or prose. They are
the exact later mechanics and evidence gates the lightweight foundation deliberately
leaves separate.

### Map indexing without canonical cells

Canonical Position alone is sufficient for addressed reads but not automatically
for a bounded World-window query. The index must remain disposable and may never
decide Place identity, discovery, visibility or authority.

The smallest first map slice can index only World-referenced Places. A rebuildable
`place_world_index` keyed by Place Entity stores the current Position revision and
World point, updates in the same transaction as an absolute Place Position and is
always rechecked against canonical Position before output. A missing or stale index
fails closed or is repaired; World never performs an unbounded Place scan as a
fallback.

Relative Places require a later root/local design rather than descendant canonical
writes. The leading candidate keeps one indexed World point for each World-referenced
root and one root-relative point for each map-indexed Place. A root move changes one
root index row; the map query translates its World window into that root's local
coordinates and pages matching Places. Moving or re-referencing an intermediate
carrier can invalidate a bounded or large subtree of derived index rows, so dirty
root handling, rebuild admission and false-negative prevention must be proved before
relative Places are promised in a production window read. No prose claim closes
that gap.

A million Characters or Places at one point still produces a million matches. The
index only finds candidates; a stable limit, opaque continuation and query-admission
budget bound every request. There is no exact count, shared point row, region lock or
global map revision.

### Transaction template

Every concrete spatial mutation should instantiate the same small sequence without
introducing a generic change resource:

1. normalize and bound all structured input before opening the transaction;
2. start PostgreSQL and resolve an exact idempotent retry;
3. establish the User, Character and current eligibility without semantic prose
   inference;
4. read enough immutable endpoint data to enumerate exact Position, Connection,
   Relation and current-state dependencies;
5. acquire all coordinator rows in one documented type-and-id order before any
   write, then re-read every expected current revision;
6. validate coordinates, cycles, direction, segment membership, bounds and ordinary
   operation authority;
7. append Activity and its typed involved-subject and dependency rows;
8. append immutable versions and advance only the exact current pointers changed;
9. update or invalidate only earned disposable indexes; and
10. commit once.

The lock order itself needs real PostgreSQL evidence because foreign-key locks and
mixed present/absent state can defeat an order that looks correct on paper. The kept
multiplayer lab already refuted a hybrid lock-class strategy; spatial should compare
the conservative Entity coordinator above with any narrower slot only after the
concrete first operation identifies false contention worth paying to remove.

### Million-User and deliberately hot-subject result

- **Quiet Entity:** its Position write touches its Entity coordinator, Position
  lineage, Activity and any exact index entry; no Place, region or World row.
- **Hot unnamed point:** writers remain independent. Reads are admitted and paged;
  there is no co-location owner or count to lock.
- **Hot Connection:** travellers share a read dependency and update only themselves.
  Editing the Connection is the honest conflicting act; shared-lock overhead and
  fairness are measured rather than hand-waved.
- **Hot carrier:** one root move changes one canonical Position. Only operations
  whose outcome needs its external resolved point conflict; interior actions remain
  independent. Derived map-index repair is operational work, never descendant World
  history.
- **Hot Place:** Position, Area, each Connection and each Relation have distinct
  revisions. The existing Place-wide activity pointer must not become their common
  lock.
- **Remote Action:** work is proportional to Button, optional Relation, Bomb and the
  explicitly named affected subjects. Distance causes no spatial scan and Relation
  causes no fan-out.

This model does not make a genuinely shared fact conflict-free. It confines the
conflict to the fact and applies bounded admission when one subject is hotter than
PostgreSQL can fairly serve.

### Dependency-ordered implementation path

The five-year model should not arrive in one migration. My preferred Terry order is:

1. **Completed productionward Position lab:** real PostgreSQL lineage, cycle race,
   checked centimetre arithmetic, idempotency and hot-carrier dependency isolation
   survived the fixed scratch fixture. No production imports from the lab.
2. **First complete spatial game slice:** direct World Position, deliberate second
   Place discovery, one named shaped Connection, bounded direct-Place map context and
   one complete or partial Character Movement with Activity. No Area, Relation,
   relative Position, Route or generic spatial endpoint.
3. **Relative Position slice:** table/birdhouse and ship/cabin behavior, addressed
   resolved reads and exact re-reference concurrency. Moving-Place map indexing is a
   separate proof gate, not a hidden promise in this slice.
4. **Open Relation slice:** stable directed Relation and bounded endpoint reads;
   remote causal citation uses ordinary exact Action authority and gains no execution
   mechanism.
5. **Area slice:** only after one current exploration scene chooses and proves the
   minimum exact 3D coverage representation and Connection intersection result.
6. **Later earned mechanics:** private contents, protected remote control, terrain
   mechanics, Orientation, physical surfaces and named Route each require their own
   concrete game behavior. None belongs in the foundation migration.

The first production slice is the highest-leverage next game edge because it turns
the already completed discovery loop into actual large-world exploration while
testing the durable Position and Connection seams. The kept lab now precedes it
because a reference-cycle or locking mistake would corrupt a core shared-world
invariant.

### Exact proof gates before implementation claims

| Gate | Must prove | Does not prove |
| --- | --- | --- |
| Position lineage | direct and relative resolution, cycle race rejection, checked overflow and atomic Activity in real PostgreSQL | production throughput or moving-Place map lookup |
| Connection Movement | exact endpoint/segment/direction rules, stale revision rejection, independent travellers and editor behavior | timed journeys, collision or terrain cost |
| Direct Place map | bounded indexed window, authorization before hydration, stable continuation and hot-point admission | relative-Place indexing or privacy not in the fixture |
| Hot carrier | one canonical move, no descendant World writes and quiet interior action independence | free projection rebuilds or arbitrary chain depth latency |
| Relative Place map | no stale false negatives across root and intermediate moves, bounded dirty handling and rebuild | Area geometry |
| Area | exact positive 3D predicate, overlaps, disconnected coverage, bounded course intersections and revision freshness | ownership, visibility or travel permission |
| Relation privacy | guessed ids, endpoints, counts, cursors, errors and timing do not reveal an ineligible Relation | semantic correctness of Agent interpretation |

If any gate requires a global revision, unbounded fallback scan, descendant canonical
rewrite or server interpretation of prose, this candidate must be revised rather
than patched with more ceremony.

### Bounded PostgreSQL Position verdict

The retained [PostgreSQL Position-lineage lab](../../lab/spatial/02-postgres-position-lineage/README.md)
supports the first Position-lineage and hot-carrier candidates inside its exact
scratch boundary. Eight focused cases on local PostgreSQL 17 under `READ COMMITTED`
resolved a 64-level chain, rejected symmetric coordinate overflow without partial
history, permitted only one side of a synchronized A→B/B→A race, moved one Ship
without rewriting any of exactly 1,000 descendants, kept Cabin-local work
independent and rolled back an injected post-Activity failure. Its separate cleanup
audit also passed.

The experiment narrows rather than completes the production proof. Both deliberate
lock waits returned `57014` because the equal 150 ms statement timeout won before a
distinct lock-timeout code; this proves bounded conflicting work in the fixture but
chooses no public retry contract. Production authorization, actual migrations,
typed Activity dependency rows, concurrent first-Position behavior, map indexes,
hosted operation and load remain untested. The implication is to carry this small
candidate into a separately accepted first spatial slice, not to promote lab code
or collapse Position, Place, Area, Connection and Relation into one system table.

### Result

The recommended technical foundation is therefore one stable Entity identity with
separate sparse Position, Place, Area, Connection and Relation truths; immutable
per-truth versions; exact operation-scoped dependencies; atomic Activity; and
replaceable indexes. It is powerful because those truths compose, lightweight
because an Entity pays only for the state it actually has, and secure at the World
boundary because prose never becomes coordinates, access or execution.

Technical synthesis is complete and its first PostgreSQL Position falsifier is kept.
Exact production fields, operation names, limits, privacy, geometry, integration
lock behavior and indexes remain evidence and plan work. No `game/docs`, migration,
runtime, HTTP, MCP or public Agent surface is authorized by this record.

## Staged delivery plan drafted

The draft [spatial exploration system plan](../../plans/20260820-071639-spatial-exploration-system/plan.md)
turns the dependency-ordered recommendation into one implementable first slice and a
non-authorizing preservation map for everything after it. S1 covers direct Position,
deliberate Place expansion, bounded eligible map context, one stable Connection and
complete or partial Movement. Later independent slices retain relative carriers,
open Relation, Area, private arrangement and separately earned Route, terrain,
protected remote-control and unforeseen-result mechanics.

This planning structure is proposed, not accepted implementation. Seven S1 choices
remain material: map eligibility, discovery admission, first-Connection
deduplication, public capability boundaries, observation away from a Place, Activity
vocabulary and representability/request bounds. `game/docs`, schema, World, HTTP,
MCP and public Agent text remain unchanged until those choices are resolved and the
complete plan is explicitly accepted.

## Character-scoped map direction accepted; current knowledge state open

The User accepted the first map direction: one coordinate window must never reveal
every established Place merely because its Position matches. World first needs a
deterministic reason that the acting Character can know the Place; only those
candidates enter the bounded spatial filter. Accepted entry and personal Place
Discovery are concrete positive examples. A guessed window, nearby coordinate or
arbitrary Connection traversal grants no recursive World map.

This makes exploration meaningful but earned one technical question rather than
solving it with prose. Activity participation cannot generically prove current
Knowledge, and current state may not be reconstructed from arbitrary history. The
User selected one sparse current Character–Place knowledge association below;
Knowledge transmission, revocation and private Places remain later unless that
selected S1 basis requires their concrete behavior.

## Sparse Character–Place knowledge selected; generic scope challenged

The User selected the sparse current association. Each exact Character–Place pair
that becomes knowable occupies one row; map reads never write a “seen” receipt and
accepted Activity already owns establishment provenance and time. Knowledge belongs
to the Character. Storing `user_id` would instead attach in-World understanding to
private control provenance merely because the current product allows one Character
per User.

The suggested generic `Knowledge(user_id, subject_type, subject_id, seen_at)` remains
under negotiation and exposes two different meanings. “Seen at” is Observation
history, while the selected row is current eligibility. The owner remains the
Character and Knowledge of a subject would mean only that its identity is eligible
for a typed, authorized current read—not that every fact about it is known.

[Primary-source storage research](../research/polymorphic-character-knowledge-storage.md)
corrected one overly broad objection. A Laravel-style polymorphic row is a normal,
indexable design: `(character_entity_id, subject_type, subject_id)` can be its unique
Character-leading key. S1 also already has two concrete candidates because accepted
Discovery makes both its Place and exact Connection knowable. Indexes do not,
however, provide one native PostgreSQL foreign key whose target table changes with
`subject_type`; that integrity must come from deterministic typed World validation
or separate typed associations. The next decision is precisely which strictness
boundary S1 accepts. Entity and Relation are not current target candidates merely
because the shape could later accommodate them.

## Knowledge creation time selected; update meaning remains open

The User wants a directly available `created_at` on each Knowledge association so
an Agent can tell when the Character first gained that Knowledge. It is immutable,
server-authored and written atomically with the establishing Activity. “Discovered
at” is not the universal definition: entry, arrival or a later accepted introduction
may establish the same Knowledge without a Discovery. The Activity retains which
accepted act established it; `created_at` supplies its direct time.

No `updated_at` meaning has yet been accepted. A change to the known Place or
Connection itself must not update every Character's Knowledge row: the subject owns
its current version, and fan-out across all knowing Characters would create false
meaning, write amplification and a hot-subject failure. The remaining question is
whether Knowledge has any current mutable state of its own; absent such a state,
`updated_at` would always equal `created_at` and earn no place.

## Per-view Observation proposed; read receipts remain rejected

The User sharpened the recurrence need: meeting an Entity again or returning to a
Place may deserve its own immutable occurrence, allowing the Character and Agent to
remember repeated encounters instead of keeping only first and last timestamps.
The existing canonical term for this candidate is Observation: information acquired
by one Character through one situated encounter or other accepted observation path.
Knowledge remains the current information that Character may use; Observation would
be its occurrence history rather than another copy of current World state.

At this point in the grill, an Observation table or capability was not yet accepted;
the following section records the subsequent choice. “Character views” had two
materially different readings. An ordinary authorized World/HTTP/MCP read is
transport and must not write history: repeated calls or retries could otherwise
manufacture encounters, grow storage without game action and turn API behavior into
a score. A genuine in-World observation instead needs one explicit, idempotent
accepted act with exact bounded observer, subject and structural basis. World may
validate those typed facts but cannot infer seeing from prose, nearby coordinates or
Agent narration.

If an occurrence model is accepted, it remains private Character-grounded history.
It must not expose a global reverse list of who watched another Character, create a
stored view counter, invoke Agents in the background or update every Character when
the subject changes. Whether current Movement/Interaction Activities already carry
enough occurrence identity or a separate Observation record earns its own state is
downstream of deciding which concrete acts count.

## Explicit Observation selected; per-occurrence free text selected

The User selected explicit in-World Observation rather than writes caused by
ordinary reads or prose alone. This clarifies the technical split. Knowledge is one
current sparse Character–subject association used to determine whether the subject
identity is eligible for its typed authorized read. Observation is append-only
occurrence history: several records may refer to the same Character and subject,
each through its exact accepted Activity and occurrence time. A retry of the same
request returns the same result; a later intentional observation may append another.

Each Observation may carry bounded free text authored by the observing Character's
Agent. For example, Mara's second Observation of Ivo may say “this time he wore a
red hat.” World can validate observer control, exact subject eligibility,
idempotency, bounds and atomic Activity, but it cannot understand whether “red hat”
is true. The note does not by itself create a Hat Entity, equip it, change Ivo or
override current structured World state.

The useful artistic consequence is a private, potentially imperfect trail of
recurrence: Agents can notice familiar people, places and motifs without turning
every recollection into universal canon. The User selected that boundary. The note
is definitively attributed only to the observing Character's memory and may be
selective, interpretive or wrong. It is available only through that Character's
authorized personal context; no target-facing or global reverse observer read is
created. Shared truth still requires the exact ordinary World structures it claims,
such as an Entity and Relation for a worn hat.

An illustrative non-canonical storage split is one unique Knowledge row per
`(character_entity_id, subject_type, subject_id)` and multiple immutable Observation
occurrences tied to exact Activity. Exact Observation identity, text field name,
subject limits, eligible acts, pagination and retention remain plan choices rather
than accepted schema.

## Private Observation account selected; Knowledge consequence parked

Option A is accepted for per-occurrence text. Mara's “this time he wore a red hat”
is Mara-attributed memory, not a Property of Ivo, not proof that a Hat Entity exists
and not text another Character receives by reading Ivo. World does not judge its
truth; it only preserves bounded authorship, observer control, exact eligible
subjects, request identity, occurrence time and Activity atomically. A conforming
Agent can later narrate the note explicitly as memory and compare it creatively with
fresh current World information.

This keeps free authorship safe without making memory mechanically useless. The User
then parked Knowledge and rejected any automatic consequence from Observation to
Knowledge. Seeing something at a distance does not mean recognizing, identifying or
knowing it. A Character may therefore possess private Observation history which
grants no current read of the possible subject. The separate Knowledge storage,
establishment and hydration branch remains unresolved and may not be decided by
Observation implementation.

## Distant Observation may lack Character knowledge; exact model reference selected

Concrete case: Mara sees a dark figure on a far ridge. Her Agent may preserve “a
dark figure stood motionless against the sunset” as private Observation text. World
must not thereby reveal Ivo's name, grant Ivo's current Entity state or pretend Mara
recognized him, even if the authoritative World internally contains Ivo at that
location.

The User selected an exact model reference for every Observation. Even the distant
figure occurrence therefore stores one required `subject_type` and `subject_id`
pointing at the exact existing model record World accepted. There is no subjectless
Observation. This lets World match repeated occurrences deterministically and reject
a missing target without interpreting the text.

That internal reference still proves neither Character recognition nor Knowledge.
The authorized Observation view may preserve “unknown figure” without hydrating the
subject's name, current fields or other Character-private data. Exact subject
hydration remains part of the parked Knowledge/eligibility design. The technical
cost is polymorphic target integrity: World must dispatch and validate the selected
model type deterministically, because one ordinary PostgreSQL foreign key cannot
change its table based on `subject_type`. The next Observation-only choice is the
current admitted model-type set, not whether a target exists.

## Explicit expandable Observation model list selected

The User selected an explicit World-owned list rather than arbitrary model, table or
runtime class names. The current stable aliases are `entity`, `place` and
`connection`. An observed Character, animal, plant or object uses `entity`. A city,
forest or other positioned Entity observed specifically in its spatial reference
role uses `place`. A direct travel alternative uses `connection`.

Position, Property and Trait are not separate current Observation targets: an Agent
may describe their eligible details in the private account of the owning Entity or
Place. Relation, Area and every future model remain absent until one concrete
accepted Observation behavior earns each type. This keeps dispatch, authorization,
querying and target validation exhaustive without preventing an additive extension.
Stored aliases are stable game/server vocabulary and never Rust type paths or table
names.

The chosen list makes the polymorphic row technically bounded: World switches over
three current types, validates that exact record and uses a Character-leading and
subject-aware index shape. It still does not decide Knowledge or expose the target's
current fields. The next Observation-only choice is creation: a dedicated operation
for every look, or an explicit optional Observation inside another structurally
grounding action such as Movement or Interaction.

## Operation-embedded Observation selected provisionally

**Superseded by the later timing choice below.** This was deliberately selected as a
reversible first direction and remains here only as the explored alternative.

The User selected option A as the first shape to try. A structurally grounding
accepted action may explicitly carry its Observation result. For example, Movement
to Place B may explicitly record the acting Character's private account of B, and an
Interaction with Ivo may explicitly record one Entity Observation. World does not
infer that either was seen from movement, distance, Position or prose: the Agent must
name the exact admitted model reference and bounded account in the proposal.

The grounding state change, its Activity and its Observation occurrence commit in
one transaction. If the Movement or Interaction fails, no Observation survives. A
retry of the same request returns the same accepted result without appending another
occurrence. An ordinary map or model read still writes nothing. A future act whose
only purpose is looking may earn a dedicated Observation operation rather than a
dummy Movement or Interaction.

This is a reversible first design to test, not an irrevocable generic action-effects
framework. Observation is added only to each concrete operation that earns it. If a
real payload or confirmation becomes confusing, a dedicated Observation operation
remains the smaller fallback without changing Observation identity, privacy or
history. The next dependent choice is cardinality: whether one grounding action may
explicitly produce several independently targeted Observation occurrences or exactly
one.

## Several Observations per one recording action selected

The User selected option A for cardinality. Under the later timing correction, one
follow-up Observation action may produce several occurrences grounded by the same
Movement. Mara's arrival through a sandy Connection into Duindorp may therefore be
followed by one recording action that preserves one private account about that
Connection and another about the destination Place. These are two Observation
records, each with exactly one model reference and its own optional text, not one
ambiguous record pointing at several models.

All occurrences share the follow-up action's Activity, refer to the grounding
Movement Activity and settle atomically with each other, not with Movement. A failed
recording stores none but leaves Mara moved; retrying the same recording request
appends none. World still interprets no account and infers no extra observer or
subject. Requiring one separate recording action per occurrence remains rejected.

The submitted collection must receive a finite technical admission limit with the
other S1 request bounds so one action cannot create an unbounded transaction. That
number is not yet selected and will not limit a Character's retained history. At
million-Character scale, one action writes only its own bounded observer-owned rows;
it updates no subject counter and fans out to nobody. The next dependent choice is
structural eligibility: which exact Entity, Place or Connection references each
kind of grounding action is allowed to include.

## Movement Observation subjects selected from its exact result

The User selected option A. A Movement may admit three sources of Observation
subjects: the exact Connection it traverses, its destination Place and any Entities
included in World's bounded authorized arrival result for that Character. The Agent
chooses which of those exact references receive private Observation accounts. The
result is not a semantic claim that everything present was visible, nor a read of
every Entity at the Place.

For example, if the allowed result contains Connection 7, Duindorp and a dog on the
square, the Agent may submit separate Observations about Duindorp and the dog. If a
sword inside another Character's hidden inventory is absent, guessing its identifier
does not make it eligible. Including that sword rejects the whole follow-up
Observation action; World never silently drops an invalid occurrence, but the earlier
Movement remains accepted.

Technically, World checks exact typed membership and the current structural versions
that justify the bounded result. It interprets no prose and creates no subject
counter, global visibility list or fan-out. At a Place with millions of associated
records, only the already bounded authorized result can participate. This choice
exposed the timing conflict resolved below: the Agent cannot author an informed note
about a new dog until World has returned it.

## Post-Movement Observation selected

**Partly superseded by the Agent-owned selection correction below.** Movement may
still precede Observation in ordinary play, but it is no longer required as a
grounding receipt and its result is not a server-owned visibility candidate set.

The User selected option B and thereby superseded the provisional embedded design.
Movement first commits the Character Position and its own Activity, then returns its
bounded authorized arrival result. Only after seeing that result may the Agent issue
a separate explicit Observation action containing its chosen model references and
private accounts. No ordinary read writes history and World still invokes no Agent.

That follow-up writes its bounded Observation occurrences and its own Activity in
one idempotent transaction while referring to the grounding Movement Activity. If it
fails, the Character remains at the destination and no partial Observation batch is
stored. This reflects what happened: failing to preserve memory cannot teleport Mara
back, repeat her Movement or erase its durable history.

The split costs one additional bounded write but makes information flow honest and
keeps transactions short at scale. Any World instance must still be able to verify
that each submitted subject belonged to the exact earlier arrival result. How that
proof survives the gap—without permanently copying every returned candidate into
World history—is now the next open technical and player-timing choice.

## Agent chooses Observations; server visibility proof rejected

The User rejected all three server-proof alternatives and corrected the abstraction.
World does not decide that Mara could see the dog, persist a candidate list, issue an
arrival proof or reconstruct past visibility. Its tools return bounded information
the Character is structurally allowed to receive. Mara's invoked Agent understands
that context, decides what Mara could notice and uses an explicit Observation action
to shape the exact private accounts it wants to retain.

Movement followed by Observation remains a natural conversation flow, not a required
database dependency. The Observation action may contain several exact one-model
occurrences and writes those plus its own Activity atomically and idempotently. A
failed recording never affects Movement. It need not cite Movement merely to satisfy
World, and World adds no visibility state, arrival-result rows, cryptographic receipt
or current-scene recheck pretending to prove what the Agent understood.

This keeps the intended split honest. World is still strict about what its read tools
return: a hidden sword absent from the Agent's authorized context is not available
for the Agent to reason from, and guessing an id grants no current subject read or
mutation authority. But Observation text may be selective, imaginative or wrong, so
World cannot validate semantic sight without becoming the intelligence layer. The
remaining boundary is deliberately smaller: which structural checks—observer
control, admitted target alias, target existence, bounds and perhaps no more—apply
when the Agent submits its chosen references. That is the next grill choice.

## Minimal deterministic Observation validation selected

The User selected option A. World checks that the authenticated User controls the
observing Character, every occurrence names one admitted target alias and existing
model id, the batch and each account fit technical bounds, the request is idempotent
and the Observation rows plus their Activity commit atomically. Missing or invalid
targets fail the bounded action; exact error disclosure remains a later public
contract detail.

World does not check the target's Position, distance, Connection, co-presence,
visibility, attention, semantic truth or current revision. It does not return the
target's current fields through the write and grants no Knowledge, read access or
mutation authority. Thus a private account may be mistaken without becoming a path
to inspect or alter its subject.

At scale, World performs bounded typed primary-key existence checks followed by
observer-owned inserts. An Observation does not update or lock the target, increment
a subject counter or fan out to anyone; millions of Characters can privately
observe one famous Entity without contending on one shared Entity row. The next
player-flow choice is confirmation: whether saving this private memory needs a fresh
preview and approval or may occur within the User's already explicit active Agent
turn.

## Active-turn Observation needs no separate confirmation

The User selected option B. During one explicitly User-invoked in-World Agent turn,
the Agent may decide and store a bounded private Observation batch for its own
Character without pausing for another preview and approval. In the same response it
tells the User what the Character noticed and retained. This is a narrow exception
for private Character memory; Movement and every shared or externally consequential
World change keep their complete preview and confirmation boundary.

The permission belongs to Agent conduct, not World session state. World sees an
authenticated bounded Observation request and enforces the already selected hard
checks, but cannot prove what was said in the surrounding conversation. The explicit
User invocation stands alone; no durable session, delayed continuation or server-side
Agent exists.

The User also made option C categorically impossible under the BYO AI-Agent
subscription model. A subscription may notify an already connected client that World
state changed, but can never invoke a User's Agent, make it reason, call Observation
or spend tokens. At scale this introduces no queues of Agent work or recipient writes.
The correction rule for an Agent-authored memory that proves wrong is resolved below.

## Prior Observation context required before encounter narration

The User made recurrence context a firm requirement. Before an Agent treats an exact
Entity, Place or Connection as newly encountered, familiar or narratively important,
it first asks World for that Character's prior Observations about the same model. An
empty result means there is no stored prior occurrence; an ordinary history read
never creates one.

The result must contain a bounded newest-first slice of the private occurrence
history, including each Agent-authored account, its occurrence time and the location
available from its Activity. That is enough for the Agent—not World—to create a line
such as “Vorige keer zag ik je in Duindorp, en nu zie ik je mijlenver hier,” notice
patterns, revive running jokes or reinterpret an earlier mistake. Further history is
cursor-paginated; there is no stored view count or omniscient recognition flag.

This also clarifies Q12. An explicit link saying “Observation B corrects Observation
A” is not what enables callbacks across Places; the subject-addressed history query
does that. The correction choice is resolved below without adding such a link.

Technically, the natural lookup begins with observing Character, target alias and
target id, ordered newest-first. It touches no target row and can use an index shaped
for exactly that access. The ceremony choice between one multi-model call, automatic
payloads and one call per model is resolved below.

## One rich batched Observation-history read selected

The User selected option A. One explicit bounded read accepts several exact model
references chosen from the Agent's current authorized context. It groups the private
history by model and returns several newest-first Observation occurrences for each,
including each stored account, occurrence time and available Activity location. A
per-model continuation says that older occurrences remain without calculating or
storing an exact total.

That response lets the Agent distinguish no stored encounter, one earlier encounter
and several returned encounters. It can therefore truthfully say “we have met quite
often” when the returned evidence supports that interpretation, while World never
stores a `view_count` or labels a relationship `often`. The Agent may page further
when older detail matters; it does not need one call per model and ordinary Entity,
Place or Connection reads do not automatically absorb private history and context
cost.

For example, one response about Ivo can contain three recent occurrences in
Duindorp, the dunes and the harbour, plus a continuation. The exact public response
schema and bounds remain technical design work, but the contract must preserve the
model grouping, multiple recent occurrences and independent continuation.

The database access remains Character-first and subject-addressed. One bounded
request performs bounded newest-first indexed scans for a bounded number of exact
subjects; it reads no subject-wide counter, locks no observed model and cannot turn a
famous Character into one shared write row.

## `list_observation` selected

The User selected option A. The proposed public capability is named
`list_observation`: `list` communicates a bounded collection and the singular domain
noun follows the same convention as `list_activity`. The name does not imply one
subject; its explicit input may still contain several exact model references and its
response remains grouped by model. `get_observations` and a one-off naming exception
are rejected. No runtime capability exists until the draft S1 plan is accepted and
implemented.

## Corrections remain ordinary chronological Observations

The User selected option B. When Mara later learns that Ivo's apparent red hat was a
lamp behind him, her Agent appends a new Observation account explaining that mistake.
The earlier Observation remains unchanged. `list_observation` returns both in
newest-first order, and the Agent—not World—understands the newer account as a
correction, joke, doubt or reinterpretation.

Observation therefore receives no correction reference, corrected status, active
version or update operation. World validates and stores the new occurrence exactly
like any other private Observation and never decides which account is true. This is
the smallest model that preserves fallible memory and its story value; a structural
correction link may be reconsidered only if later Agent evidence shows that the rich
chronological response is insufficient.

This closes the Observation-correction branch and exposes the next spatial
prerequisite. The User rejected “participation” as an extra state or ceremony: a
Character already exists in World, while being abandoned by its User would be a
separate later control or lifecycle question. `current_place` only says that a Place
is the Character's current local context; its absence may validly mean that a
positioned Character stands between Places.

## `enter_world` retained as the Character's introduction

The User initially selected immediate Position during `create_character`, then
reopened and superseded that answer after recalling the game value of
`enter_world`. Character creation may leave the Character unpositioned. This is not
“no participation”; it is the short, intentional state before the Character is
spatially introduced into the shared World. `enter_world` establishes its first
Position and attributable Activity.

The introduction should offer a bounded set of several World locations from which
the User can choose instead of forcing every new Character through one global entry
Place. That keeps first arrival a meaningful scene and allows a mature World to offer
geographically different beginnings. It does not require a Character participation
field, per-location arrival counter or update to one hot global row.

“World locations” was deliberately temporary wording rather than a new domain term.
The User selected a small hybrid presentation: three existing Places with a short
explanation of each, plus one explicit “do not begin in an existing Place” choice.
Selecting a Place establishes the Character at that Place's Position and makes it
the current Place. Selecting the other choice establishes a Position with no current
Place and creates no Place merely for the arrival.

The latter wording is intentional. World cannot generally prove that an exact point
lies geometrically outside every Place because Area coverage may be absent or
incomplete. It can prove that the Character selected no existing Place as its local
context. The User selected World to choose the exact loose Position. Choosing the
open-space option authorizes that unknown result; the accepted `enter_world`
transaction chooses and stores one exact `x`/`y`/`z` Position, no current Place and
its Activity. Concurrent or lost-response retry returns that same stored result and
never rerolls.

World chooses only coordinates under an explicit numeric distribution. It does not
infer suitable terrain, safety, beauty, accessibility or Place exclusion, and the
Agent narrates only from structurally available context after arrival. No persistent
pool of anonymous entry points, coordinate uniqueness or shared random counter is
needed; several Characters may validly receive the same Position.

The User selected a random point broadly around one of the three offered Places. The
anchor contributes only its exact Position; World draws an offset under a bounded
numeric distribution and still stores no current Place. “Good point” means valid
under that entry distribution, not empty: World neither scans for occupants nor
avoids, prefers or reserves another Character's Position. A new Character may arrive
alone, beside one other traveller or amid many, and exact Position equality remains
valid rather than a uniqueness conflict.

This distribution keeps the open-space beginning related to established World
geography without forcing social isolation or one central origin. Exact distance and
random-generation bounds remain technical choices, never later Movement limits. The
arrival itself returns only the accepted Character placement and stops. It performs
no automatic surroundings read, Observation, Investigation or discovery merely
because other Characters might be nearby.

The User starts the next gameplay step explicitly by asking the Agent to look around
or explore. Only that invoked turn may obtain bounded current surroundings, interpret
them, preserve Observations or pursue a new discovery. This separates a possible
encounter from an automatically consumed encounter. The next prerequisite is whether
the first explicit exploration step reads already existing nearby World subjects,
attempts to discover something new or deliberately sequences those two different
behaviors. How the three Places become eligible and what happens before three exist
also remain downstream choices.

## Exploration begins at every Character Position

The User clarified the core game loop: after entry, an explicitly invoked Agent may
explore from the Character's exact current Position whether or not `current_place`
is set. A loose Position is therefore playable spatial ground, not a waiting room
until somebody creates a Place. World must never require or fabricate a dummy Place
merely to admit exploration.

The Agent reads bounded structurally eligible context around the Character's fresh
Position, uses its own intelligence to imagine and discuss a discovery, and submits
the exact structured result through the later confirmed discovery boundary. World
validates identity, coordinates, dependencies, authority, bounds, idempotency and
atomic Activity; it does not infer the discovery from prose or from the coordinate.
Millions of Characters exploring unrelated points touch no shared current-Place or
region row, while genuinely conflicting proposals coordinate only on their exact
subjects and dependencies.

The User described Place at the player level as a Position that has been discovered,
named and can later gain properties. That confirms Place is deliberately established
through play rather than a prerequisite or automatic classification of every point.
Its literal storage meaning remains the one active vocabulary question: either the
existing one-subject design is the technical implementation—a named Entity with one
Position and a Place role—or Place becomes a separate coordinate-and-content owner.
No canonical vocabulary or runtime contract changes until that distinction is
settled.
