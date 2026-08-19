---
status: load-bearing
era: August 2026 spatial multiplayer reset
---

# Entity and Place through a game-framework lens

> **Role / side:** sourced comparison of broad game-framework identity with
> object, Place, settlement, terrain and spatial facts / development side.
> **Authority:** records this report's external facts, Aicadia inferences,
> recommendations and remaining product choices.
> **Excludes:** accepted Entity or Place meaning, current game behavior, schema,
> implementation architecture and delivery evidence; those remain in
> `dev/CONTEXT.md`, `dev/areas/`, `game/docs/`, accepted plans and
> `dev/docs/evidence/`.

Controlled: 2026-08-18

Status: research informed the accepted broad Entity foundation and the accepted
Position/Place/Area meanings; the current Relation question now separates open
Agent-authored meaning from exact typed World structure, while storage, public
identity and the canonical one-word topology name remain open

## Question

Can Aicadia use the familiar game-framework meaning of `Entity`—a broad identity
to which capabilities and state are attached—without pretending that a city is the
same kind of thing as a tree, or turning Connections, coordinates and every other
database record into Entities?

The immediate design test is concrete:

- a Character can interact with a tree, bridge, waterfall or remote button;
- a city can be discovered, named, governed, attacked and connected to other
  cities;
- a named forest may be entered and referred to as one enduring part of the World;
- ordinary woodland terrain may cover many unnamed positions;
- a Connection states that travel is possible but is not itself a thing in the
  World; and
- every Character and placed Entity must still have one exact position, including
  between named Places.

Which of these are Entities, which are Entity roles, and which are spatial facts or
values?

## Evidence language

- **External fact** is directly supported by official engine documentation,
  first-party game material or project-owned documentation and source.
- **Inference** translates those facts into a consequence for Aicadia.
- **Recommendation** is an unaccepted research conclusion for the continuing
  grill.
- **Open choice** remains for the User; the sources cannot make a product decision.

Engine words do not automatically become Aicadia domain words. A runtime ECS
`Entity` can be a short-lived integer, while an Aicadia Entity is durable shared
World identity. The research compares the conceptual boundary, not the lifetime or
storage implementation.

## Short answer

Yes. A broad Entity foundation is conventional and fits the User's proposed test,
provided Aicadia defines two layers explicitly:

1. **Entity supplies identity, not kind.** An Entity is a durable, independently
   addressable subject in the fictional World. It can own current state and history
   and participate in Actions, Interactions or typed relationships. A city can
   satisfy that definition without being called a physical object.
2. **Typed roles and facts say what the subject is and can do.** `Character`,
   `Place` and a possible later `Settlement` are roles of an Entity. Position,
   Connection, terrain coverage and coordinate interpretation remain separate typed
   state. Having an id or referring to Entities does not make such a record an
   Entity.

This is close to ECS terminology, but it must not be mistaken for a proposal to
build a generic ECS database or universal component table. Conventional typed
Postgres tables can keep one `entity_id` while enforcing each role's own rules.

The research also supports rejecting the earlier shorthand “Place means where.”
That phrase is too broad: an exact position is also a where, yet the accepted
exploration promise says it need not be a Place. The User later accepted this more
precise boundary:

> A Place is the role of an Entity that makes one durable spatial site or extent
> independently addressable for spatial gameplay such as entry, occupancy,
> discovery, containment or connectedness.

Canonical wording now lives in [`dev/CONTEXT.md`](../../CONTEXT.md). It deliberately
says what the role enables rather than claiming that every coordinate, terrain patch
or technical cell is a Place. Which concrete subjects World establishes as Places
remains a product choice.

## 1. ECS engines use Entity as identity, not as “physical object”

### Bevy

**External fact.** Bevy describes Entities as unique “things” assigned groups of
Components. Its concrete `Entity` is a simple type containing a unique integer;
`Person`, `Name`, `Position` and `Velocity` are examples of components selected by
systems.
[Bevy, *ECS*](https://bevy.org/learn/quick-start/getting-started/ecs/)

**External fact.** Bevy's migration guidance warns applications not to attach
durable external meaning to a runtime Entity id and recommends their own stable
identifier when stability is needed across mapping or reconstruction.
[Bevy, *0.16 to 0.17 migration guide*](https://bevy.org/learn/migration-guides/0-16-to-0-17/)

**Inference.** Bevy supports the terminology “Entity is the identity to which
capabilities are attached,” but not copying its runtime identity lifetime into a
persistent MMO. Aicadia's stable UUID and database lifecycle remain its own design.

### Unity Entities

**External fact.** Unity's Entities documentation defines an Entity as an identifier
associated with components. Components contain the data; an Entity groups them and
has neither behavior nor data of its own in the ECS model.
[Unity, *Entity concepts*](https://docs.unity.cn/Packages/com.unity.entities%400.0/manual/ecs_entities.html),
[Unity, *Components overview*](https://docs.unity.cn/Packages/com.unity.entities%401.3/manual/components-intro.html)

**Inference.** Neither Unity nor Bevy requires the word Entity to mean only a
tangible item. A city can be one Entity if city-specific data and spatial behavior
are represented by explicit components or roles.

### Unreal and Godot

**External fact.** Unreal calls anything placeable in a Level an Actor. Actor types
include physical props and Characters, but also volumes, triggers, cameras, sound
sources and gameplay areas. A Level is a collection of Actors making up all or part
of a game world.
[Epic, *Actors reference*](https://dev.epicgames.com/documentation/en-us/unreal-engine/unreal-engine-actors-reference),
[Epic, *Levels*](https://dev.epicgames.com/documentation/en-us/unreal-engine/levels-in-unreal-engine)

**External fact.** Godot uses Nodes as its fundamental building blocks and `Node3D`
as the base representation for a node in 3D space. Scenes are reusable trees of
Nodes, and Node types include spatial regions and paths as well as visible objects.
[Godot, *Nodes and Scenes*](https://docs.godotengine.org/en/stable/getting_started/step_by_step/nodes_and_scenes.html),
[Godot, *Node3D*](https://docs.godotengine.org/en/stable/classes/class_node3d.html)

**Inference.** Familiar game-framework roots routinely cover more than physical
objects. They gain clarity from explicit Actor, Node or Component types, not from
the root word itself proving what something is.

## 2. Persistent multiplayer frameworks show both the benefit and the danger

**External fact.** Evennia's persistent game objects include Rooms, Characters,
Exits, weapons, flower pots and castles under a common `DefaultObject` base. Rooms
are in-game locations; Exits are one-way links between them.
[Evennia, *Creating things*](https://www.evennia.com/docs/latest/Howtos/Beginner-Tutorial/Part1/Beginner-Tutorial-Creating-Things.html)

**External fact.** Evennia's optional component system can attach reusable
capabilities and persisted fields to an object, while its own documentation names
the additional complexity and required host object as costs.
[Evennia, *Components*](https://www.evennia.com/docs/latest/Contribs/Contrib-Components.html)

**Inference.** A common persistent identity makes generic lookup, containment and
history simple. Evennia also demonstrates the failure boundary for Aicadia: a
framework may make an Exit an Object for convenience, but Aicadia has already
chosen that a Connection is a structural travel fact and not an Entity. “One base”
must therefore be constrained to fictional World subjects, not every useful record.

## 3. World-building tools keep spatial layers separate

### OpenMW

**External fact.** OpenMW separates its object library (items, triggers,
containers, NPCs and similar objects) from instances. An instance says which Cell
contains it and stores position, rotation, scale and possible ownership. Cells are
world-building containers; exterior Cells form a coordinate grid over continuous
terrain, while interior Cells represent enclosed spaces. Regions, terrain Lands and
NPC path grids are separate again.
[OpenMW, *World Tables*](https://openmw.readthedocs.io/en/stable/manuals/openmw-cs/tables-world.html)

**Inference.** Uniform identity for interactive subjects does not eliminate a
spatial substrate. A tree's identity, its placement, the ground beneath it, a named
region and the operational Cell used to process it answer different questions.

### Civilization VII

**External fact.** Civilization VII treats Settlements as Cities or Towns while
population, buildings, Wonders and improvements occupy tiles. The generated world
first establishes geography and later overlays the hex grid used for terrain,
resources and exploration.
[Firaxis, *Managing Your Empire*](https://civilization.2k.com/civ-vii/archive/dev-diary/managing-your-empire/),
[Firaxis, *Improved Map Generation*](https://civilization.2k.com/civ-vii/game-guide/gameplay/map-generation/)

**Inference.** A city is a durable aggregate with gameplay identity, while its
spatial coverage and contents live on finer ground. Calling the city an Entity does
not imply storing every occupied point in the Entity row or treating its buildings
as parts of one object record.

## 4. Geographic standards split social subject, extent and ground cover

**External fact.** Overture represents a `division`, such as a city or
neighborhood, as an organization of people with an approximate point. A separate
`division_area` captures the polygonal land or maritime area belonging to that
division, and a `division_boundary` represents a shared border.
[Overture Maps, *Divisions guide*](https://docs.overturemaps.org/guides/divisions/)

**External fact.** Overture separately represents `land_cover` as the natural or
artificial material covering an area, including forest, crops, wetland and urban
cover. `land_use` records the human use of land. Its `place` dataset is instead a
large collection of point representations for businesses, schools, hospitals,
landmarks and other points of interest.
[Overture Maps, *LandCover*](https://docs.overturemaps.org/schema/reference/base/land_cover/),
[Overture Maps, *Places guide*](https://docs.overturemaps.org/guides/places/)

**Inference.** “City,” “city extent,” “urban ground cover” and “interesting site in
the city” are not interchangeable. Aicadia does not need Overture's schema, but it
does need to avoid asking one Place row to mean all four.

## 5. A precise Entity admission test

The User's proposed rule—an Entity can have a relationship to or Interaction with
another Entity—fits the frameworks, but by itself is too permissive. A Connection
also refers to other subjects, and an Activity has participants, yet neither thereby
becomes a World Entity.

**Recommendation.** Admit something as an Entity only when all of these hold:

1. **World subject:** it exists in the fiction as a subject, not merely as a
   database row, spatial measurement, relationship, event or processing unit.
2. **Independent identity:** participants need to recognize the same subject again
   independently of its current name, position, owner or relationship.
3. **Own state or history:** the subject can meaningfully change or accumulate
   history without that change being only an edit to another subject's field.
4. **Participation:** the subject may participate in an Action, Interaction or
   explicit typed relationship in its own role.

The fourth condition captures the User's insight. The first three stop the test from
turning every relationship and coordinate into an Entity.

### Classification against that test

| Example | Candidate classification | Why |
| --- | --- | --- |
| Character, animal, tree, birdhouse | Entity | Independently recognized World subjects with state, placement and history. |
| Door, bridge, constructed road | Entity | Tangible subjects that can change, be acted on and affect traversal. |
| Remote button and bomb | Entity | Each is independently targetable; a typed remote link relates them. |
| City | Entity with Place role; possible later Settlement role | The city can retain identity and history independently of its extent and contents. |
| Named forest | Entity with Place role only when established as one enduring subject | “The Ashen Wood” can be discovered, changed and related; generic forest cover cannot. |
| Ordinary woodland or heath | Terrain over spatial ground | It describes what covers positions; no independent subject is required. |
| Waterfall | Entity; Place role only if the game establishes the waterfall itself as an occupiable spatial site | It is a durable natural feature, but nearby position does not automatically make every landmark a Place. |
| Exact position | Spatial value or record | It locates subjects but is not itself a fictional subject. |
| Connection | Typed spatial fact | It relates Place endpoints; its existence and direction do not make it a World subject. |
| Coordinate frame | Spatial structure | It interprets positions or transforms; it is not present in the fiction as a subject. |
| Activity | Historical record | It remembers an occurrence rather than persisting as a current World subject. |

These classifications are recommendations. The city, forest, waterfall and exact
Place boundary remain User choices.

## 6. What Place would mean in this model

“Place is where something is” fails three tests:

- a coordinate is also where something is but should not become a Place;
- a Character can occupy an unnamed point between Places; and
- a Place may be a city or named forest whose contents and extent change without
  changing its identity.

**Recommendation.** Treat `Place` as a spatial gameplay role, not a synonym for
position and not a competing root identity:

> A Place is the role of an Entity that makes one durable spatial site or extent
> independently addressable for entry, occupancy, discovery, containment or
> connectedness.

Concrete consequences:

- Moss Village can be one Entity with a Place role. Buildings, inhabitants and
  objects within it keep their own Entity identities and placements.
- A city may later receive a Settlement role if population, governance,
  construction or civic membership becomes gameplay. That role should be added
  only when one accepted behavior requires it.
- A named forest can be a Place while forest cover remains terrain underneath it.
  Not every woodland position becomes another Entity or Place.
- A waterfall remains an Entity landmark without automatically being a Place. “At
  the waterfall” can initially mean an exact position near that Entity. A separate
  Place role is earned only if the waterfall site itself needs occupancy,
  containment, Connections or Place-scoped behavior.
- A building can be an Entity while one or more interiors later have Place roles or
  separate Place Entities. That identity question remains open rather than being
  hidden inside containment.

## 7. Technical shape without a generic ECS database

This is an explanatory shape, not an accepted schema:

```text
entity
  id                -- stable identity of a fictional World subject

character
  entity_id          -- Entity role

place
  entity_id          -- Entity role; spatial behavior, not another identity

settlement           -- absent until gameplay earns it
  entity_id          -- possible civic/social role of the same city Entity

placement
  entity_id          -- where an Entity currently is on the exact spatial ground
  ...                -- representation remains unchosen

connection
  from_place_entity_id
  to_place_entity_id
  ...                -- explicit direction and later traversal facts
```

Each role gets a typed table and its own constraints. This does **not** recommend:

- one generic `component(name, json)` table;
- one `entity.type` enum that permits exactly one kind forever;
- one universal `relation(source, predicate, target)` graph;
- putting coordinates, ownership, visibility and all revision state on `entity`;
  or
- updating a city's Entity row whenever anything inside the city changes.

The same Entity may bear more than one compatible role, but each fact remains in the
table that owns its invariant. A Character is an Entity; a city can be an Entity
with Place and later Settlement roles; a Connection is still not an Entity.

### What an ECS would actually do with a city

**External fact.** Unity's ECS Entities have no type or data of their own; attached
component types categorize the Entity. Position, rotation and scale live in a
`LocalTransform` component. A `Parent` component creates a transform hierarchy, and
Unity explicitly advises against very large hierarchies under one root because
transform work is divided at roots.
[Unity, *Entities*](https://docs.unity.cn/Packages/com.unity.entities%400.0/manual/ecs_entities.html),
[Unity, *Using transforms*](https://docs.unity.cn/Packages/com.unity.entities%401.2/manual/transforms-using.html)

**External fact.** Bevy likewise stores spatial position on `Transform`; its
`GlobalTransform` is computed from the Entity's transform and ancestor transforms.
[Bevy, *GlobalTransform*](https://docs.rs/bevy/latest/bevy/transform/components/struct.GlobalTransform.html)

**External fact.** Unreal uses three-dimensional Volume Actors to detect entry into
an area and apply area behavior. Its large-world grid cells instead load and unload
content based on streaming sources. Those cells are operational partitions, not a
requirement that fictional cities or regions share their identity.
[Epic, *Volume Actors*](https://dev.epicgames.com/documentation/en-us/unreal-engine/volume-actors-in-unreal-engine),
[Epic, *World Partition*](https://dev.epicgames.com/documentation/en-us/unreal-engine/world-partition-in-unreal-engine)

**Inference.** An ECS does not answer “what is a Place?” It permits the game to add
the exact component that answers it. A conceptual ECS-shaped city could be:

```text
Entity Moss Village
  Name
  Description
  Properties
  Traits
  Position
  Place
  Area { optional coverage }
  Settlement { ... }       -- only after civic gameplay earns it
```

A tree in Moss Village would have `Name`, `Description` and its own Position,
but no Place role merely because it is spatially located. Buildings and residents
would likewise remain independent Entities. They should not all become transform
children of the city or force a city revision whenever one of them moves.

In PostgreSQL the same idea stays typed rather than becoming an ECS component store:

```text
entity(id, name, description, ...)
position(entity_id, ...coordinate representation still to choose...)
place(entity_id, ...place-specific state...)
area(place_entity_id, ...optional geometry...)
connection(from_place_entity_id, to_place_entity_id, ...)
```

The Place row says that the Entity itself represents a spatial area or site. The
Position row says where an ordinary Entity is. The Connection row says that two
Place roles have an explicit direct travel relationship. These are three distinct
truths even though an ECS could express all three with components or relationship
data.

### Coordinates do not distinguish Place from ordinary Entity

**External fact.** Unity and Bevy attach transform components to ordinary Entities,
not only to locations. OpenMW likewise stores cell-relative coordinates on each
placed object instance. Evennia's default room graph needs no coordinates, while
its optional XYZGrid adds queryable coordinates to Rooms specifically to support a
logical map and pathfinding.
[Unity, *Using transforms*](https://docs.unity.cn/Packages/com.unity.entities%401.2/manual/transforms-using.html),
[Bevy, *GlobalTransform*](https://docs.rs/bevy/latest/bevy/transform/components/struct.GlobalTransform.html),
[OpenMW, *World Tables*](https://openmw.readthedocs.io/en/stable/manuals/openmw-cs/tables-world.html),
[Evennia, *XYZGrid*](https://www.evennia.com/docs/latest/Contribs/Contrib-XYZGrid.html)

**Inference.** A Character, tree, birdhouse, bomb and city all need spatial
position in Aicadia. Therefore “has a coordinate” cannot be the complete Place
test. The useful database distinction is narrower:

- the base `entity` row does not own coordinate columns;
- a separate Position may spatially locate any Entity; and
- the Place role requires spatial grounding and declares that the Entity is an
  established spatial reference for map, discovery, navigation or topology.

Conceptually, without choosing the coordinate representation:

```text
entity(id, name, description, ...)

position(entity_id, ...coordinates...)
  -- optional for an Entity in general
  -- required for an Entity with the Place role

place(entity_id, ...place-specific state...)
  -- role: this subject is an established spatial reference

area(place_entity_id, ...geometry...)
  -- optional and absent until area behavior earns it
```

This keeps one home for the current position. A tree and city may both have spatial
Position. Only the city bears the Place role unless the World also establishes the
tree as a mapped destination or landmark. A Place may have an optional area; a
point-like waterfall, monument or crossroads can still qualify without pretending
that every positioned Entity is a Place.

The requirement is therefore “spatial grounding is necessary but not sufficient.”
The exact establishment authority remains a product choice: neither coordinates nor
an Agent-authored name may silently grant the Place role.

### Map retrieval is a Place capability, not an unbounded table scan

A Place role is useful as the semantic map layer. A bounded map read can return
established Places in or around one known spatial window together with their Entity
name, description, Position, optional Area and explicit nearby Connections. It
should not return every Place in an unbounded World in one response.

Places alone also do not constitute the complete map. Exact spatial ground and
terrain explain the unnamed heath between them; physical roads, bridges, buildings
and landmarks remain Entities; Connections expose selected direct topology; and
operational cells or indices only accelerate bounded reads. A map projection may
compose those sources without turning them into one model.

### Term comparison

**External fact.** OGC defines `position` as a set of coordinates of one point in a
coordinate system and associated reference frame. Godot exposes `position` and
`global_position`; Unity exposes `Position` inside its transform data; Unreal's
player-facing API says Actor Location while explaining that the Actor itself does
not store that data and obtains it from a scene component.
[OGC, *GeoPose spatial concepts*](https://docs.ogc.org/is/21-056r11/21-056r11.html#toc16),
[Godot, *Node3D*](https://docs.godotengine.org/en/stable/classes/class_node3d.html),
[Unity, *Using transforms*](https://docs.unity.cn/Packages/com.unity.entities%401.2/manual/transforms-using.html),
[Epic, *Actors*](https://dev.epicgames.com/documentation/en-us/unreal-engine/actors-in-unreal-engine)

**Inference.** Aicadia needs the point only. Rotation, scale, velocity, physical
shape, support and area have different lifecycles and rules. Bundling them under
`Transform` would import an engine implementation concept into a persistent game
domain. Calling the fact `Placement` would describe the act of putting something
down rather than its current truth, and becomes especially awkward for moving
Characters.

| Term | Recommended use | Why not use it for exact coordinates? |
| --- | --- | --- |
| `Position` | The optional exact current point of any Entity. | Recommended; conventional, minimal and independent of the eventual coordinate encoding. |
| `Place` | An Entity role for an established map, discovery, navigation or topology reference. | It is semantic identity, not the exact point itself. |
| `Area` | The optional spatial coverage of a Place. | It is not required for point-like Places and is not an Entity's current point. |
| `Location` | Natural-language wording when talking to a player. | Ambiguous between exact Position and semantic Place in code and schema. |
| `Anchor` | Explanatory word for how a Place's Position appears on a map. | A separate stored Anchor would duplicate the same Position truth. |
| `Transform` | Engine rendering and physics composition. | Bundles Position with rotation, scale and a reference-space operation. |
| `Placement` | The action or result of placing an Entity. | Sounds static and editor-oriented; poor wording for movement. |
| `Presence` | Observation or occupancy semantics. | Does not identify an exact point. |
| `Pose` | Position plus orientation. | Adds orientation before gameplay requires it. |
| `Landmark` | A narrower possible Place role or classification later. | Too narrow for cities, forests and rooms. |
| `SpatialFeature` | A geospatial umbrella. | Too technical and broad; naturally includes roads, terrain and boundaries. |

**Historical accepted implication, now partly reopened.** The User selected this
small spatial vocabulary:

- `Position` — where an Entity is, expressed as one exact point;
- `Place` — which positioned Entities are established spatial references;
- `Area` — what a Place optionally covers; and
- `Connection` — which direct travel relationship exists between Places.

Do not store a second Place Anchor. The Place Entity's Position is the map point; a
separate Area is optional. The resulting definition is:

> A Place is the role of an Entity with a Position that World has established as an
> independent reference for map, discovery, navigation or explicit spatial
> relationships. It may, but need not, have an Area.

The Place definition and the `Position`/`Place`/`Area` meanings remain accepted.
The User subsequently rejected every two-word replacement such as
`PlaceConnection` and reopened both the fourth primitive's one-word name and its
scope. Geometry, coordinate representation, Area semantics and the concrete Place
admission rule also remain open.

### Area and Connection answer different questions

**External fact.** Overture Maps keeps an identified settlement or administrative
division separate from its point representation, polygonal `division_area` and
linear boundary. Its own documented uses include point-in-polygon containment, and
it allows some lower-level areas to overlap.
[Overture, *Divisions Guide*](https://docs.overturemaps.org/guides/divisions/)

**External fact.** Overture's transportation model does not infer connectivity from
geometry. Two path segments are not physically connected without an explicit shared
connector even when their shapes overlap or share a coordinate. Access restrictions
can then constrain travel over an otherwise connected network.
[Overture, *Segments and Connectors*](https://docs.overturemaps.org/guides/transportation/segments-and-connectors/)

**Inference for Aicadia.** `Area` answers “which spatial ground does this Place
cover?” `Connection` answers “which direct travel relationship has World explicitly
established between these Places?” They must stay independent:

- Moss City can have one Position used for its map marker and an Area covering its
  established urban ground;
- Bramble Forest can have one Position and an Area even when the forest overlaps a
  larger valley Place;
- Glass Waterfall can be a Place with a Position and no Area;
- Moss City and Bramble Forest may touch or overlap without a Connection; and
- a one-way ferry Connection can relate two Places whose Areas do not touch, while
  the ferry or landing may be a separately involved Entity.

Area overlap therefore proves neither Place identity, travel, ownership, visibility
nor authority. Conversely, a Connection need not describe every intermediate point
or become a stored Route. Ordinary continuous or cell-based movement can later
change Position over terrain; a Connection earns use where gameplay needs one
explicit direct transition or topological statement.

**Open choice.** The sources cannot decide whether an Area is authoritative positive
World coverage, a necessarily complete inside/outside boundary or only descriptive
map geometry. That semantic choice must precede its exact geometry, cardinality and
database representation.

### Direct Place relationship terminology audit

This audit names only the durable non-Entity fact that two Places have one explicit
direct spatial/travel relationship. It does not name a road, door, bridge or ferry;
the permission or current ability of one Character to travel; the act of moving; or
an ordered multi-step Route. It records terminology evidence and does not decide
the reopened vocabulary by itself.

**External fact.** Unity, Godot and Unreal all use `Link` for an auxiliary
navigation connection. Their links combine endpoints with traversal-facing state:
Unity exposes activation, agent type, direction and cost; Godot exposes enabled
state, direction, navigation layers and cost; Unreal distinguishes static simple
links from dynamically enabled smart links.
[Unity `NavMeshLink`](https://docs.unity.cn/Packages/com.unity.ai.navigation%402.0/api/Unity.AI.Navigation.NavMeshLink.html),
[Godot `NavigationLink3D`](https://docs.godotengine.org/en/stable/classes/class_navigationlink3d.html),
[Unreal `ANavLinkProxy`](https://dev.epicgames.com/documentation/en-us/unreal-engine/API/Runtime/AIModule/ANavLinkProxy)

**External fact.** Recast calls the equivalent primitive an off-mesh
`Connection`: one user-defined traversable connection with two endpoints. Overture
uses `connectivity` for explicit physical network topology, then models access and
turn restrictions separately. Its `connector` is specifically a junction point
shared by segments, while Routes are memberships over segments rather than one
direct pair.
[Recast `dtOffMeshConnection`](https://recastnav.com/structdtOffMeshConnection.html),
[Overture segments and connectors](https://docs.overturemaps.org/guides/transportation/segments-and-connectors/),
[Overture roads and restrictions](https://docs.overturemaps.org/guides/transportation/roads/)

**External fact.** Evennia's `Exit` is a one-way in-game Object with a destination,
transit command and `traverse` access lock. Unreal uses `Transition` for an evaluated
change from one state to another whose rule determines whether it may occur. Godot's
generic A* API speaks of points being directly connected by a `segment`, rather
than elevating graph `edge` to game-domain vocabulary.
[Evennia exits](https://www.evennia.com/docs/latest/Components/Exits.html),
[Unreal transition rules](https://dev.epicgames.com/documentation/en-us/unreal-engine/transition-rules-in-unreal-engine),
[Godot `AStar3D`](https://docs.godotengine.org/en/stable/classes/class_astar3d.html)

| Candidate | Player or World reading | Domain and implementation consequence | Evidence-led fit |
| --- | --- | --- | --- |
| `Connection` | World has explicitly related these two Places directly; whether this Character can use it now remains a separate answer. | Neutral about physical carrier, access, enabled state and path cost; needs qualification where other relationship families also use “connection.” | Strongest fit for the stable primitive; supported by Recast and Overture's topology/access split. |
| `Link` | These Places are joined by a navigable shortcut or special crossing. | Very conventional in engines, but commonly bundles live pathfinding eligibility, direction, agent class and cost. | Strong runner-up for a runtime navigation projection; less neutral for the durable World fact. |
| `Passage` | There is a corridor, opening or traversable way in the fiction. | Suggests a physical spatial subject and current passability, colliding with a door, tunnel or bridge Entity. | Useful player wording or Entity kind; weak primitive name. |
| `Transition` | A traveller changes from one Place/state to another. | Naturally names the evaluated movement step or rule, not the topology that remains before and after an attempt. | Better reserved for an Action/result if ever needed. |
| `Edge` | No grounded game meaning without explaining a graph. | Precise storage/algorithm vocabulary but invites one generic relation graph and exposes implementation language. | Fine inside a bounded graph algorithm; poor domain name. |
| `Exit` | A named, usually directed affordance such as “north” or “through the door.” | Evennia makes it an Object, command and access-bearing traversal mechanism, contrary to this non-Entity boundary. | Useful derived player affordance, not the neutral primitive. |
| `Connector` | A junction or attachment point where several paths meet. | Overture gives it point identity and geometry; it is not the pairwise relationship itself. | Wrong shape for a direct Place pair. |
| `PlaceConnection` | The same meaning as Connection, with both endpoint type and scope explicit. | Avoids collision with remote causal or social links; violates the later one-word requirement. | Historically strongest qualified candidate in this audit; subsequently rejected by the User. |

**Inference.** In this bounded Place-only audit, `Connection` best preserved the
stable concept while `Link` most often named its active navigation-engine
realization. `PlaceConnection` added scope rather than another mechanic, but is no
longer a candidate. `Passage`, `Exit` and physical path nouns pull a
road/door/bridge Entity into the relation; `Transition` names travel happening;
`Edge` names the graph implementation; and `Route` remains an ordered composition
of direct relationships.

**Historical recommendation, superseded for the continuing grill.** This audit
recommended a Place-only primitive and compared bare `Connection` with
`PlaceConnection`. The User rejected the qualified name and reopened the primitive
scope; the broader audit below supersedes that comparison without deciding it.

**Later disposition.** The User rejected `PlaceConnection` because a foundational
term must be one word and reopened whether the primitive should be spatial-only or
more broadly relational. Bare `Connection` remains a working term, not a renewed
choice. Directionality, identity, access and availability remain separate product
choices regardless of the eventual name.

### Earlier typed-only question: is Relation reusable alongside Entity?

This earlier audit assumed that every stored Relation would be a server-understood
structural type. The later two-layer audit supersedes that assumption for open
Agent-authored meaning while preserving the comparison for typed operational facts.

**External fact.** Bevy provides generic `Relationship` machinery, but concrete
relationships remain specialized Components. Its built-in hierarchy is
`ChildOf`/`Children`; its official example defines a separate
`Targeting`/`TargetedBy` pair. Each concrete relationship selects its own source of
truth, inverse collection, cardinality and lifecycle behavior rather than storing a
free-form relationship kind in one record.
[Bevy `Relationship`](https://docs.rs/bevy/latest/bevy/ecs/relationship/trait.Relationship.html),
[Bevy relationship example](https://docs.rs/bevy/latest/src/relationships/relationships.rs.html)

**External fact.** Flecs deliberately offers generic relationship pairs of
`(relationship, target)` and uses them for hierarchies, inventory and trade. It
still attaches behavior per relationship: traits such as `Exclusive` and `Acyclic`
change cardinality and cycle rules. Its performance guide also warns that extensive
relationship use can increase archetype fragmentation and wildcard-index work.
[Flecs relationships](https://www.flecs.dev/flecs/md_docs_2Relationships.html),
[Flecs component traits](https://www.flecs.dev/flecs/md_docs_2ComponentTraits.html)

**Inference.** `Relation` is a conventional umbrella beside `Entity`; that does not
make one universal persistent Relation row the conventional or correct Aicadia
model. The candidate Aicadia meanings share endpoints but immediately diverge:

| Meaning | Endpoint and invariant pressure |
| --- | --- |
| Direct Place topology | Place-role endpoints, explicit direction, no inferred reverse, access separate. |
| Spatial navigation link | Spatial endpoints, possibly moving reference frames, entry/exit Position and current availability. |
| Containment or attachment | Entity endpoints, at most one direct parent when selected, cycle and movement-propagation rules. |
| Fictional ownership | Owner and subject roles, transfer authority, privacy and history distinct from physical presence. |
| Button-to-bomb causality | Exact trigger and target Entities, operation-specific authority, freshness and bounded consequences. |

At million-User scale, a single generic table is not automatically one hot row, but
it does make common endpoint indices carry unrelated traffic and forces every broad
read to branch by type-specific authorization, privacy and bounds. Separate typed
tables can still share implementation helpers without sharing truth, locks or an
unbounded `list_relation` surface.

**Historical inference under the typed-only assumption.** The Entity analogy is
incomplete. Every Entity has accepted common identity, name, description, state and
history semantics. The proposed relations do
not yet share an independent lifecycle, addressable identity, common authority or
common visibility contract; two endpoints alone do not earn a root table. If a
common term is useful, `Relation` can name the family while each concrete relation
keeps its own type, schema and validator, matching Bevy's specialized-relationship
pattern. A universal Relation identity or table should wait until concrete gameplay
requires shared behavior that cannot be expressed cleanly by typed facts.

**Historical open choice, now narrowed.** The next product decision was scope, not
spelling: keep the primitive as direct Place topology; broaden it only to
spatial/navigation endpoints; or adopt
`Relation` as a conceptual family of separately typed facts. Choosing a universal
stored Relation primitive is a further architectural decision and is not implied by
choosing the family vocabulary.

### Earlier persistent Relation base-table proposal

The earlier hard-discriminator proposal under review was one durable non-Entity
record with `relation_id`, a server-owned `relation_type`, `source_entity_id` and
`target_entity_id`. Because Place is an Entity role, Place A and B fit those endpoint
columns without turning the relationship itself into an Entity. A stable
`relation_id` would let an Action address, revise or retire one exact fact and let
Activity identify that fact even when parallel facts share endpoints. It earns its
cost only if that independent identity or lifecycle is real; endpoints plus type
already identify a fact when the type forbids duplicates.

#### What PostgreSQL can and cannot enforce

**External fact.** A PostgreSQL enum is a static ordered set. Values can be added or
renamed, but cannot be removed or reordered without recreating the type. An enum is
therefore a strong closed server vocabulary, not a place for Agent-authored relation
kinds.
[PostgreSQL, *Enumerated types*](https://www.postgresql.org/docs/current/datatype-enum.html)

**External fact.** A foreign key can prove that both endpoint ids exist. A row-local
`CHECK` can reject `source_entity_id = target_entity_id` where every type forbids
self-relation, but PostgreSQL explicitly does not support `CHECK` constraints that
depend on other table rows. A Place-only endpoint or “source has button role” rule
therefore needs a foreign key to an appropriate role key, a type-specific table, or
transactional World validation; a base foreign key to Entity proves only Entity
existence. PostgreSQL also does not automatically index the referencing side of a
foreign key.
[PostgreSQL, *Constraints*](https://www.postgresql.org/docs/current/ddl-constraints.html)

**External fact.** Partial unique indexes can enforce a known type-specific rule on
one table—for example at most one outgoing inventory parent for each source Entity.
The query predicate must match the index predicate closely, and PostgreSQL warns
against replacing partitioning with a large set of category-specific partial
indexes. Declarative partitions must share the parent columns; a unique constraint
on a table partitioned by type must include that partition key.
[PostgreSQL, *Partial indexes*](https://www.postgresql.org/docs/current/indexes-partial.html),
[PostgreSQL, *Table partitioning*](https://www.postgresql.org/docs/current/ddl-partitioning.html)

**Inference.** An enum can close admission, but it does not attach the different
authorization, privacy, cardinality, cycle, lifecycle or payload rules to its
values. Flecs makes those rules traits of each relationship, and Bevy makes each
relationship a specialized Component with its own reverse collection and lifecycle.
The official ECS evidence supports reusable machinery with typed semantics, not an
enum-only row as the whole domain contract.

#### Three storage shapes for server-typed structural facts

| Shape | Player or World consequence | State, authority, transaction, contention and cost consequence |
| --- | --- | --- |
| **1. One Relation table with enum only** | Every accepted direct fact has one uniform identity: “cup supported by table,” “button activates bomb” and “A linked to B” look alike at rest while their type gives the verb. | Smallest schema and one atomic insert. Two Entity foreign keys and common endpoint indexes are easy; per-type payloads, role FKs, authorization and privacy remain World branches. Partial unique indexes can express a few fixed cardinalities, but many types accumulate conditional constraints and indexes. A generic reader can accidentally disclose unrelated private facts. |
| **2. Base Relation plus type-specific extension tables** | Every fact retains a common address, while `held`, `inventory`, `activates` and Place topology can carry only their own structural state and rules. | Base insert plus exactly one extension must be one transaction. An extension can reference `(relation_id, relation_type)` to prove its base type, but an ordinary foreign key in that direction does not prove that every base row has its required extension; World code, a deferred trigger or a more elaborate constraint is needed to prevent incomplete/orphaned states. Reads add joins and base endpoint indexes still mix traffic. This shape pays when common identity/history is genuinely shared and typed fields are current. |
| **3. Separate typed tables plus a `UNION ALL` view** | Each mechanic remains a first-class fact; an optional read surface can present a common relation family without granting a common mutation. | Each table gets native role FKs, uniqueness, privacy and indexes with no enum branching. PostgreSQL views are not physically materialized—the defining query runs when referenced—and a union view is not a simple automatically updatable view, so typed commands remain the write boundary. A globally addressable id needs a namespaced `(type, id)` or a separately justified shared allocator/base; cross-table uniqueness and one foreign key to “any relation” do not arise from the view. Adding a type adds DDL and a union arm. [PostgreSQL, `CREATE VIEW`](https://www.postgresql.org/docs/current/sql-createview.html) |

Partitioning the enum-only base by `relation_type` can separate physical indexes
later, but it still requires identical parent columns and makes `(relation_type,
relation_id)` the natural declaratively unique key. It is an operational scale
choice, not evidence that the domain types share semantics. Conversely, separate
typed tables may share Rust query, revision and history helpers without pretending
their facts have one authority contract.

#### The six concrete structural-invariant tests

| Proposed fact | Stored truth versus derived predicate | Type-specific rule the four base columns do not supply |
| --- | --- | --- |
| cup `supported_by` table | Store only if World accepts “support” as an explicit structural fact. If it merely means the cup's current geometry intersects the tabletop, derive it from current spatial state; a stored duplicate becomes stale when either Entity moves. | Directed; source may have one immediate support while a table supports many; detachment and movement-frame effects need their own rules. World can validate declared structure, not infer physical support from names or prose. |
| dog `under` bridge | Normally a geometry-derived, time-specific predicate. Persisting it while dog or bridge moves creates false truth unless the accepted mechanic defines a durable spatial attachment instead. | Coordinate frame, geometry version, observability and expiry; none follows from a generic relation type. |
| glass `held_by` hand | A useful explicit attachment/possession fact, not just proximity. If a hand is not itself an independently addressable Entity, the proposed endpoint columns can name only the Character; a hand/body slot needs typed extension data rather than a counterfeit hand Entity. | At most one current holder, holder authority, local Position/moving frame, release lifecycle and possibly private inventory visibility. |
| sword `contained_by` inventory | Explicit containment; it does not by itself mean owned, equipped, visible or at the holder's exact Position. | One immediate container, cycle rejection, capacity/access if accepted, and no accidental cascade delete. |
| button `activates` bomb | Explicit directed causal eligibility; distance and co-presence are irrelevant unless the action contract says otherwise. | Potentially many-to-many, non-transitive, endpoint control, privacy, current revisions and a bounded atomic press consequence. |
| Place A `linked_to` B | Explicit direct positive topology, distinct from a road/door Entity, access and a multi-step Route. | Place-role endpoints under the spatial-only scope, direction/symmetry, parallel-link identity, discoverability and current availability remain separate choices. |

**Inference: absence is not a negative fact.** If A→B exists and A→C does not,
World knows only that no accepted positive A→C fact currently exists. It does not
know that travel to C is impossible, forbidden or permanently absent. An explicit
prohibition, if gameplay needs one, requires its own bounded typed rule and
authority/history. Storing the complement of every possible Entity pair would be
unbounded in a sparse World.

#### Million-User and one-hot-subject boundary

Every shape can avoid a global Relation lock. Mutation should lock or compare the
exact relation/relation slot plus only endpoint revisions required by that type; it
must not update a relation count on the Entity row. Reads must name an Entity,
allowed relation types, direction, cursor and limit. The useful index shape is
therefore type plus source or target plus a stable pagination key, not an unbounded
`list_relation` or recursive graph walk.

A deliberately hot bridge, inventory or Place still creates real fan-in/fan-out and
hot index ranges. Pagination bounds the read result but does not make millions of
same-target inserts free. Type-leading indexes or separate typed indexes isolate
unrelated mechanics; no design should serialize them through the hot target Entity.
Authorization must select the type-specific policy before returning rows: knowing a
button targets a hidden bomb or that a glass is in a private hand can itself be the
secret. The common base is never permission to expose common endpoint queries.

**Historical recommendation, superseded for the continuing grill.** Treat stable
Relation identity, shared storage and family vocabulary as three independent
choices. If accepted kinds share
only two Entity endpoints, separate typed tables remain the lower-coupling baseline.
If current gameplay needs one addressable lifecycle/history contract across several
kinds, a base plus typed extensions preserves that identity without forcing enum-only
semantics. Choose enum-only only if the accepted kinds truly have no type-specific
stored state or declarative invariant beyond a small, auditable set. In all cases,
keep typed World commands and bounded typed reads.

**Later disposition.** The User rejected both a hard enum as the complete Relation
vocabulary and a structural-only Relation boundary. The comparison above remains
useful for the exact typed operational facts in the second layer and for any later
shared identity substrate; it no longer describes the full meaning an Agent may
author.

### Two layers: open meaning and exact World structure

The clarified requirement is that an explicitly invoked Agent can express an
unanticipated nuanced relationship between two Entities without waiting for a new
server enum—for example, “this cup floats exactly two centimetres above that
table.” That expressive Relation must coexist with, and never impersonate, the
typed facts World uses to validate movement, containment, traversal, access and
bounded causal effects.

#### Primary-source boundary

**External fact.** OpenUSD separates a generic scene-description core from schemas
that give particular data operational meaning. A Prim can contain two kinds of
Property: an Attribute with a typed value that may vary over time, and a
Relationship that points to one or more other scene objects. Both can carry
metadata. A custom Property is explicitly ad hoc data outside a Prim schema and
carries no expectation of specific processing by consuming applications.
[OpenUSD, *Introduction*](https://openusd.org/release/intro.html),
[OpenUSD, `UsdProperty::IsCustom`](https://openusd.org/release/api/class_usd_property.html)

**External fact.** An OpenUSD Relationship is named on its owning Prim and can
target Prims, Attributes or other Relationships. It is always uniform rather than a
time-sampled value, while its inherited Property interface supplies authored state,
display name, documentation and metadata. OpenUSD's shape is not an Aicadia schema:
the Relationship is cheap to refetch from its owning Prim rather than a separately
durable World subject, and one Relationship may have several targets.
[OpenUSD, `UsdRelationship`](https://openusd.org/release/api/class_usd_relationship.html),
[OpenUSD, `UsdPrim::CreateRelationship`](https://openusd.org/release/api/class_usd_prim.html)

**Inference.** The transferable pattern is the semantic boundary, not the storage
layout: an open Relation may carry authored meaning without any consumer being
entitled to treat its name or properties as executable mechanics. Exact mechanics
remain typed schemas. Aicadia's stable ids, multiplayer transactions and privacy
requirements still need their own design.

**External fact.** OpenUSD's `UsdGeomXformable` computes a local-to-parent transform
separately from its local-to-world transform. Godot makes the same distinction:
`Node3D.position` is relative to the parent while `global_position` is relative to
the World.
[OpenUSD, `UsdGeomXformable`](https://openusd.org/release/api/class_usd_geom_xformable.html),
[Godot, `Node3D`](https://docs.godotengine.org/en/stable/classes/class_node3d.html)

**Inference.** “Two centimetres above the table” can therefore have two independent
representations for two questions. The semantic Relation records what the Agent
means. A typed Position can record an exact local coordinate in the table's frame;
an Attachment or frame rule, if accepted, decides whether moving the table also
moves the cup. The prose verb `floating_above` cannot decide movement inheritance,
collision, support or current world coordinates.

**External fact, informative draft only.** The April 2026 RDF 1.2 Concepts document
is a W3C Candidate Recommendation, not a final Recommendation. Its graph model uses
subject–predicate–object triples with a predicate IRI. Its reification model
distinguishes mentioning a proposition from asserting it and permits several
distinct reifiers—for example, claims from different sources—about the same
proposition.
[W3C, *RDF 1.2 Concepts and Abstract Data Model*](https://www.w3.org/TR/rdf12-concepts/)

**Inference.** This is useful conceptual evidence for giving each Aicadia Relation
its own provenance and qualifications while distinguishing “an Agent authored this
claim” from “World structurally guarantees this mechanic.” It does not recommend
RDF, a graph database, IRIs or server-side entailment for Aicadia.

#### Candidate responsibilities, not a decided schema

| Layer | Player or World consequence | Candidate state and validation boundary |
| --- | --- | --- |
| **Open semantic Relation** | Agents can say that a cup floats above a table, a dog rests under a bridge, a sword belongs in an inventory or a button is meant to activate a bomb. Several Agents may describe the same pair differently. | A stable Relation candidate can hold exact source and target Entity ids, a bounded Agent-authored predicate/name/description and bounded relation Properties, plus authenticated author, Activity, revision and visibility/provenance. World validates identity, input bounds, authorization, idempotency and history; it does not understand the predicate, units, truth, causality or spatial consequences. No server enum gates the authored meaning. |
| **Exact typed spatial/operational fact** | Movement, carrying, inventory, travel and button activation behave consistently even when prose is novel, misleading, stale or hostile. | Position/frame, geometry, Attachment, Containment, Place topology, access/control and bounded capability/target facts each keep typed endpoints, invariants, authority, versions and indexes. World may derive an exact current predicate from these facts where the operation is deterministic, but never derives mechanics from the semantic Relation. |

`predicate`, `name`, `description` and Properties are not interchangeable. A
predicate can provide a short Agent-authored verb such as `floating_above`; name and
description can render its particular meaning; Properties can qualify this
particular Relation with values such as `distance = 2` and `unit = centimetre`.
World may validate their encoding and bounds while treating all four as untrusted
meaning. Whether these are four stored surfaces or a smaller package is still an
open KISS choice.

An invoked Agent may interpret this content, compare it with visible typed facts
and use it to compose a proposal. That interpretation is neither stored authority
nor proof: World still admits or rejects the proposal solely against the concrete
typed capability and current structural state.

A stable Relation id becomes materially more useful in this layer than in a unique
server enum pair. It can distinguish two authors' claims about the same endpoints,
receive relation-level qualifications, preserve revision/history and be referenced
as the claim behind later deliberation. This does not make Relation an Entity or
give it Entity gameplay capabilities.

Authorship must resolve to Aicadia's authenticated World actors and accepted
Activity—not to an unverified model name. Author, controller and subject are
different roles: the Character whose Agent authored a Relation need not control
either endpoint, and controlling an endpoint need not grant silent ownership of
someone else's claim. Exactly who may author, revise, hide, challenge or moderate a
Relation about another party's Entity is an unresolved product and abuse-policy
choice.

#### All prior examples through both layers

| Example | Open Agent-authored meaning | Exact typed truth, if gameplay needs it |
| --- | --- | --- |
| cup floating exactly 2 cm above table | Relation `floating_above`, with the precise phrase and optional opaque distance/unit qualifiers. | Position expressed in the table's frame supplies the canonical numeric offset. Attachment/frame inheritance separately decides whether it follows table movement; geometry supplies collision or clearance. |
| cup on table | Relation `on` or `supported_by` can express the fiction even when World has no support mechanic. | A typed Support or Attachment fact is justified only if gameplay must enforce support, detachment or shared movement. Mere geometric contact is derived from current geometry. |
| dog under bridge | Relation `under` remains a legitimate authored observation or belief. | Current “under” is normally derived from both Entities' Position and geometry at an observation revision. It should not be duplicated as authoritative current structure merely because the Relation exists. |
| glass in hand | Relation `held_by` describes the situation. | Attachment to a Character frame/hand slot, local Position, containment and access/control are distinct typed facts. A hand need not become an Entity merely to serve as a slot. |
| sword in inventory | Relation `in` or `carried_by` supplies free meaning. | Containment supplies one immediate container and cycle rules. Ownership, equipped state, visibility and exact Position remain separate. |
| button activates bomb | Relation `activates` can state intention, lore or an Agent's belief. | Only a typed bounded capability/target fact plus endpoint authority and fresh revisions may authorize a press to mutate the bomb. The semantic Relation alone is inert. |
| Place A linked to B but not C | Relations may describe A–B in arbitrary terms. | Typed Place topology establishes the exact direct transition used by traversal. No A–C topology fact means no accepted direct transition; it does not prove C unreachable, forbidden or semantically unrelated. |

#### Duplication and staleness boundary

The two layers may deliberately discuss the same Entities, but only one can own each
structural truth. Position owns the numeric coordinate; Attachment owns movement
inheritance; Containment owns inventory membership; Place topology owns direct
traversal structure; access/capability owns permission to act. World must not create
a shadow semantic Relation for every typed fact or parse Relations to keep typed
state synchronized.

An Agent may submit a bounded package that proposes both an authored Relation and
exact typed state in one transaction. World can validate and atomically store each
part under its own rules, but cannot prove that `floating_above` semantically means
the proposed offset. A Relation may optionally record exact basis ids or endpoint
revisions. World can later report mechanically that those bases are no longer
current; it still cannot decide whether the claim became false. Without such a
basis, a Relation is durable authored meaning and may become stale or be
contradicted without corrupting structural state.

Multiple Relations with the same predicate and endpoints are not automatically
duplicates when authorship, context or qualification differs. Conversely, repeating
one identical Relation can become spam. Deduplication, supersession, expiry,
contradiction presentation and whether a Relation is a claim, observation, belief or
accepted description all need explicit product semantics; endpoint equality alone
cannot decide them.

#### Privacy, abuse and million-scale reads

Agent-authored names, descriptions, predicates and Properties are untrusted World
content, never server instructions. Bounds must cover every string, property count,
total package, affected endpoints and mutation count. A Relation may leak a hidden
bomb, private inventory, social allegation or undiscovered Place merely through its
existence, so authorization must run before returning the row, count, predicate or
endpoint. Endpoint control alone is not a complete visibility policy.

At millions of Users, a bounded read names one Entity, direction, visibility scope,
cursor and limit; an exact predicate or author filter can be optional. It must not
offer an unbounded property search, recursive relation traversal or “all Relations
in World.” Writes conflict on the exact Relation/revision or an explicitly unique
authorship slot, never on a global Relation revision or the endpoint Entity row.
A hot bridge can still accumulate a huge relation index range, so pagination,
type/layer-specific indexes and per-actor admission/abuse limits are required; no
stored counter on the bridge may serialize unrelated authors.

Typed operational facts need separate reads and authorization even when one higher
level response composes both layers. Otherwise an open semantic query becomes a
side channel into private inventory, access or capability state. RDF 1.2 itself
warns that arbitrary graphs need domain-appropriate security/privacy and that
malicious graph queries can be computationally expensive; Aicadia should take the
warning without importing the RDF execution model.
[W3C, *RDF 1.2 security and performance considerations*](https://www.w3.org/TR/rdf12-concepts/#security)

**Recommendation, not product decision.** Carry the two-layer boundary into the
grill as the leading candidate: freely authored, inert Relation meaning beside the
smallest current set of typed spatial and operational facts. Test each proposed
field by asking whether World must understand it to settle gameplay. If yes, it
belongs in a typed fact; if no, it may remain bounded authored Relation content.
This recommendation does not select tables, endpoints or a public capability.

**Open product choices.** Decide whether every semantic Relation is explicitly a
claim or may become a ratified current description; who may author one about
uncontrolled or private endpoints; the minimum predicate/name/description/Property
package; Relation control, visibility, moderation, supersession and deletion; how
conflicting claims render; whether typed-basis references and atomic semantic-plus-
structural proposals are needed; whether relative Position implies movement
inheritance or only names a coordinate frame; and which typed facts are required by
the first accepted gameplay scenario. This research makes none of those choices.

### Why Position survives scale and later spatial models

The name `Position` does not require one global floating-point XYZ tuple. Its later
representation may be a discrete cell, integer coordinate, cell plus local offset,
or a bounded position relative to a moving subject. That implementation choice is
still open. The domain truth remains the same: it identifies one Entity's exact
current point.

At massive concurrency, one moving Entity changes only its own Position row and
Activity. It does not update the containing Place, every observer or one global
World row. A bounded map read joins nearby Positions to the Place role through a
spatial index and page limit. An optional Area can be indexed and versioned
separately because changing a city's boundary is not the same operation as moving a
Character.

### A city population is not automatically a live Property

The accepted Entity base lets Moss Village carry a Property such as
`population_description = about one thousand`. That is an Agent-authored
descriptive fact, like any other Property. It does not make World count residents or
update the value when a Character crosses the city boundary.

If later gameplay requires the exact set of current inhabitants, residence,
citizenship or civic membership must earn its own typed structural facts. A current
count may then be derived from those facts for a bounded read. Treating one mutable
`population = 1000` Property as both prose meaning and authoritative membership
would create drift, concurrent lost updates and a hot city row.

## 8. Multiplayer and the deliberately hot city

Broad Entity identity helps multiplayer because every durable subject can be named
consistently in authorization, typed relationships and Activity participation. It
does not make the Entity row a universal lock.

For a remote button and bomb:

- the button and bomb have stable Entity identities;
- a concrete typed link records that this button may target this bomb;
- pressing validates the acting Character's authority, the exact link, both current
  endpoint versions and the bounded declared consequences;
- World changes the affected subjects and writes one Activity atomically; and
- distance and Place equality are irrelevant unless the specific action requires
  them.

For a city containing a million active Characters:

- movement of one Character must not update or lock the city Entity merely because
  it has the Place role;
- occupancy and interest reads must be bounded and indexed over placement state;
- unrelated Entity actions in the city must settle independently;
- only an action that truly changes the shared city subject—such as accepting a new
  city name—may contend on that city's own state; and
- Area, cell or partition indices may accelerate queries but never replace city or
  Entity identity.

**Inference.** One broad identity layer improves reference consistency. Correct
massive concurrency still comes from fact-specific tables, versions, locks and
bounded reads—not from the root identity model.

## 9. Options for Aicadia

### A. Broad Entity with typed roles and facts

**Player and World consequence.** A tree, Character, city and named forest can all
be durable subjects. Their roles make their differences explicit. A Connection,
position and terrain patch remain non-Entity spatial state.

**Technical consequence.** Shared identity, Activity participation and typed
relationships can consistently use `entity_id`. Place and later Settlement use
one-to-one role tables. Every operational fact still has its own table and conflict
boundary.

**Assessment.** Recommended. This matches common ECS vocabulary, the User's
relationship/Interaction intuition and Aicadia's one-subject/one-identity rule.

### B. Restrict Entity to tangible objects and beings

**Player and World consequence.** City and named forest become separate top-level
spatial or aggregate subjects rather than Entities. Their Actions and relationships
need parallel concepts.

**Technical consequence.** Activity participation, ownership, relationships,
lookup and authorization must accept several identity families or introduce a
polymorphic common reference later.

**Assessment.** Semantically intuitive at first, but it reproduces a broader common
subject abstraction under another name once cities must participate in the same
World mechanics.

### C. Add a neutral World-subject root above Entity and Place

**Player and World consequence.** “Entity” can retain the narrow physical meaning;
Entity, Place and Settlement all share a neutral parent identity.

**Technical consequence.** This makes the two meanings explicit but adds another
canonical concept and table before current gameplay requires it.

**Assessment.** Coherent, but not KISS. The broad game-framework meaning of Entity
already provides this root if the project defines it clearly.

## 10. Accepted implication and remaining choices

Following this research, the User accepted option A and its broad game-engine
meaning. The canonical wording is owned by [`dev/CONTEXT.md`](../../CONTEXT.md); the
acceptance and reason are recorded in the current
[concept log](../concept/log/2026-08.md#spatial-multiplayer-foundation--broad-entity-identity-accepted).

The accepted definition is:

> An Entity is one durable, independently addressable subject in the World that can
> own state and history and participate in Actions, Interactions or typed
> relationships.

Then define `Place` by its concrete spatial capability, not by the vague word
“where.” Keep positions, terrain, geometry, spatial frames, Connections, ownership
and remote causal links as separate typed truths. Add `Settlement` only when a city
needs civic behavior beyond being a spatial subject.

The research still cannot decide these open product choices:

1. which exact spatial capabilities are sufficient or required for the Place role;
2. whether a city always begins as Place or is first established as a different
   kind of Entity;
3. when a named forest becomes one Entity rather than terrain plus a name;
4. whether a landmark such as a waterfall ever needs the Place role; and
5. whether a building and its interior share one Entity identity or use linked
   spatial subjects.

The first remaining choice precedes the other Place examples. The acceptance
requires no runtime or schema change because the delivered Entity already has name,
description, optional Properties and optional Traits, and the delivered Place
already reuses its Entity identity.

## Sources

### Game frameworks and persistent-world engines

- [Bevy, *ECS*](https://bevy.org/learn/quick-start/getting-started/ecs/)
- [Bevy, *0.16 to 0.17 migration guide*](https://bevy.org/learn/migration-guides/0-16-to-0-17/)
- [Unity, *Entity concepts*](https://docs.unity.cn/Packages/com.unity.entities%400.0/manual/ecs_entities.html)
- [Unity, *Components overview*](https://docs.unity.cn/Packages/com.unity.entities%401.3/manual/components-intro.html)
- [Unity, *Using transforms*](https://docs.unity.cn/Packages/com.unity.entities%401.2/manual/transforms-using.html)
- [Epic, *Actors reference*](https://dev.epicgames.com/documentation/en-us/unreal-engine/unreal-engine-actors-reference)
- [Epic, *Levels*](https://dev.epicgames.com/documentation/en-us/unreal-engine/levels-in-unreal-engine)
- [Epic, *Volume Actors*](https://dev.epicgames.com/documentation/en-us/unreal-engine/volume-actors-in-unreal-engine)
- [Epic, *World Partition*](https://dev.epicgames.com/documentation/en-us/unreal-engine/world-partition-in-unreal-engine)
- [Godot, *Nodes and Scenes*](https://docs.godotengine.org/en/stable/getting_started/step_by_step/nodes_and_scenes.html)
- [Godot, *Node3D*](https://docs.godotengine.org/en/stable/classes/class_node3d.html)
- [Bevy, *GlobalTransform*](https://docs.rs/bevy/latest/bevy/transform/components/struct.GlobalTransform.html)
- [Bevy, `Relationship`](https://docs.rs/bevy/latest/bevy/ecs/relationship/trait.Relationship.html)
- [Bevy, relationship example](https://docs.rs/bevy/latest/src/relationships/relationships.rs.html)
- [Epic, *Actors*](https://dev.epicgames.com/documentation/en-us/unreal-engine/actors-in-unreal-engine)
- [Evennia, *Creating things*](https://www.evennia.com/docs/latest/Howtos/Beginner-Tutorial/Part1/Beginner-Tutorial-Creating-Things.html)
- [Evennia, *Components*](https://www.evennia.com/docs/latest/Contribs/Contrib-Components.html)
- [Evennia, *XYZGrid*](https://www.evennia.com/docs/latest/Contribs/Contrib-XYZGrid.html)
- [Evennia, *Exits*](https://www.evennia.com/docs/latest/Components/Exits.html)
- [Unity, `NavMeshLink`](https://docs.unity.cn/Packages/com.unity.ai.navigation%402.0/api/Unity.AI.Navigation.NavMeshLink.html)
- [Godot, `NavigationLink3D`](https://docs.godotengine.org/en/stable/classes/class_navigationlink3d.html)
- [Godot, `AStar3D`](https://docs.godotengine.org/en/stable/classes/class_astar3d.html)
- [Epic, `ANavLinkProxy`](https://dev.epicgames.com/documentation/en-us/unreal-engine/API/Runtime/AIModule/ANavLinkProxy)
- [Epic, *Transition Rules*](https://dev.epicgames.com/documentation/en-us/unreal-engine/transition-rules-in-unreal-engine)
- [Recast, `dtOffMeshConnection`](https://recastnav.com/structdtOffMeshConnection.html)
- [OpenMW, *World Tables*](https://openmw.readthedocs.io/en/stable/manuals/openmw-cs/tables-world.html)
- [Flecs, *Relationships*](https://www.flecs.dev/flecs/md_docs_2Relationships.html)
- [Flecs, *Component Traits*](https://www.flecs.dev/flecs/md_docs_2ComponentTraits.html)
- [OpenUSD, *Introduction*](https://openusd.org/release/intro.html)
- [OpenUSD, `UsdPrim`](https://openusd.org/release/api/class_usd_prim.html)
- [OpenUSD, `UsdProperty`](https://openusd.org/release/api/class_usd_property.html)
- [OpenUSD, `UsdRelationship`](https://openusd.org/release/api/class_usd_relationship.html)
- [OpenUSD, `UsdGeomXformable`](https://openusd.org/release/api/class_usd_geom_xformable.html)

### Database primitives

- [PostgreSQL, *Enumerated Types*](https://www.postgresql.org/docs/current/datatype-enum.html)
- [PostgreSQL, *Constraints*](https://www.postgresql.org/docs/current/ddl-constraints.html)
- [PostgreSQL, *Partial Indexes*](https://www.postgresql.org/docs/current/indexes-partial.html)
- [PostgreSQL, *Table Partitioning*](https://www.postgresql.org/docs/current/ddl-partitioning.html)
- [PostgreSQL, `CREATE VIEW`](https://www.postgresql.org/docs/current/sql-createview.html)

### Semantic relation model

- [W3C Candidate Recommendation, *RDF 1.2 Concepts and Abstract Data Model*](https://www.w3.org/TR/rdf12-concepts/)

### Game and geographic world models

- [Firaxis, *Managing Your Empire*](https://civilization.2k.com/civ-vii/archive/dev-diary/managing-your-empire/)
- [Firaxis, *Improved Map Generation*](https://civilization.2k.com/civ-vii/game-guide/gameplay/map-generation/)
- [Overture Maps, *Divisions guide*](https://docs.overturemaps.org/guides/divisions/)
- [Overture Maps, *LandCover*](https://docs.overturemaps.org/schema/reference/base/land_cover/)
- [Overture Maps, *Places guide*](https://docs.overturemaps.org/guides/places/)
- [Overture Maps, *Segments and Connectors*](https://docs.overturemaps.org/guides/transportation/segments-and-connectors/)
- [Overture Maps, *Roads and restrictions*](https://docs.overturemaps.org/guides/transportation/roads/)
- [OGC, *GeoPose spatial concepts*](https://docs.ogc.org/is/21-056r11/21-056r11.html#toc16)
