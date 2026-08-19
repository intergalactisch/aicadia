---
status: pending
era: August 2026 spatial multiplayer reset
---

# Spatial granularity in exploration worlds

> **Role / side:** sourced comparison of spatial substrate, terrain, meaningful
> locations and traversal in exploration games / development side.
> **Authority:** records this report's external facts, Aicadia inferences,
> unaccepted recommendations and open product choice.
> **Excludes:** accepted Place or Movement direction, current game behavior,
> implementation architecture and delivery evidence; those remain owned by
> `dev/areas/`, `game/docs/`, accepted plans and `dev/docs/evidence/`.

Controlled: 2026-08-18

Status: research only; nothing below changes the Aicadia contract or current Area
decisions

## Question

How do Civilization-style tile worlds, continuous open worlds and room/graph worlds
represent the space between important locations? In particular:

- is every piece of traversable world a meaningful Place;
- where do terrain, biome, roads, occupancy and ownership live;
- how do Characters meet or stop between named destinations; and
- what does each proven model imply for Aicadia's open exploration, settlement and
  multiplayer requirements?

The immediate design doubt is concrete. Suppose Moss Village and the Old City are
known. Between them lies a forest edge followed by open heath. Two Characters can
walk there at the same time, meet one another, stop, discover flora or perhaps put
down a birdhouse. Must the forest edge and every part of the heath be a `Place`, or
does Aicadia need a different kind of spatial ground beneath Places?

## Evidence language and boundary

- **External fact** is directly supported by a first-party game document, official
  engine manual or project-owned source tree.
- **Inference** compares those facts and translates them into a consequence for
  Aicadia.
- **Recommendation** is an unaccepted research conclusion for the continuing grill.
- **Open choice** is a product decision the external evidence cannot settle.

The game examples solve different problems. Civilization is a turn-based 4X game;
Minecraft is a block world; OpenMW implements Morrowind's continuous RPG world;
Evennia is a framework for room-based multiplayer games; and Smallville is a finite
agent simulation. None is a blueprint for Aicadia. They are useful because their
contrasting spatial foundations make the cost of each choice observable.

## Short answer

No researched family makes every piece of world space a player-meaningful Place.
They do one of two things:

1. **They give all traversable space a finer spatial address.** Civilization uses
   hex tiles; Minecraft uses block positions; OpenMW uses local 3D positions inside
   exterior or interior cells. Settlements, regions, structures and named locations
   are then separate structures over that ground.
2. **They deliberately omit most in-between space.** Evennia's default model moves
   an object directly from one Room to another through an Exit. A stretch of heath
   exists as playable space only if the designer makes it a Room or adds a
   coordinate/grid extension.

Therefore “everything is a Place” hides two radically different designs:

- **every map unit is addressable**, as with Civilization tiles; or
- **only every persistent gameplay stop is addressable**, as with Rooms.

Calling both `Place` would blur the difference. The first needs a spatial substrate;
the second is a sparse Place graph.

## 1. Civilization: tiles are the gameplay ground, not just named places

### What the official design exposes

**External fact.** Civilization VII describes territory as hexes on the map. A
Settlement gains population that can be allocated to a tile for improvement or to
an urban tile as a specialist. Urban tiles hold buildings, Wonders occupy a full
tile, and adjacency is computed from surrounding natural features and constructed
structures. The Settlement remains a distinct game concept over those tiles.
[Firaxis, *Managing Your Empire*](https://civilization.2k.com/civ-vii/archive/dev-diary/managing-your-empire/)

**External fact.** Firaxis's current map-generation explanation first builds
landmasses and natural features, then overlays the hex grid and hands the result to
the gameplay layer to determine tile types, yields, resources and starting
locations. Exploration reveals the next hex, while Homelands and Distant Lands are
larger concepts formed by the generated map rather than replacements for its
individual tiles.
[Firaxis, *Improved Map Generation*](https://civilization.2k.com/civ-vii/game-guide/gameplay/map-generation/)

**External fact.** Official patch notes show several truths composing on this one
tile ground: units move onto bridges; fallout changes tile yields; the minimap shows
tile ownership; roads connect visually; railroads appear on rural and improved
tiles; features such as atolls can occur on ocean tiles and stop unit movement; and
resources appear only on eligible terrain tiles.
[Civilization VII patch notes, 5 February 2025](https://support.civilization.com/hc/en-us/articles/38337649895187-Civilization-VII-Patch-Notes-February-5-2025),
[Civilization VII patch notes, 4 November 2025](https://support.civilization.com/hc/en-us/articles/46192509687443-Civilization-VII-Patch-Notes-November-4-2025)

### What the open Civ-family implementation corroborates

**External fact.** Unciv, the project-owned open-source Civilization V remake,
stores a tile position and neighboring tiles together with base terrain, terrain
features, rivers, an improvement, resource and amount, road status, units, owning
city and continent. This is evidence for a common Civ-family implementation shape,
not evidence for Civilization VII's private schema.
[Unciv `Tile` at inspected commit](https://github.com/yairm210/Unciv/blob/5b9ddb6cc415c0bb3f0673304104ff9eec4f327e/core/src/com/unciv/logic/map/tile/Tile.kt)

### Meaning for the Place question

**Inference.** Civilization does not model “Moss Village,” “open heath,” a road and
an arbitrary hex as four variants of one semantic Place. The tile is the uniform
address at which movement, terrain, occupancy and local effects settle. Settlements
and larger regions organize or control tiles; roads and features modify them.

**Inference.** Civilization can remember that two units are both on the forest edge
because both have exact tile positions. It does not need to mint a named Place for
that encounter. A route can be calculated over neighboring tiles, while a trade
route or player-named course can remain a higher-level behavior.

**Transferable lesson.** If Aicadia wants meaningful arbitrary in-between position,
then Place alone is not the Civ-like answer. A finer spatial address is.

**Non-transferable assumption.** Civilization begins with a finite generated map in
which every playable hex already exists. Aicadia is semantically unbounded, sparse
and Agent-authored. Precreating or simulating a dense world grid merely because
Civilization does so would contradict Aicadia's current premise and add enormous
state before gameplay earns it.

## 2. Minecraft: terrain, biome, feature, structure and active chunk are different

**External fact.** Minecraft Bedrock generates the world in separate passes. Base
terrain establishes valleys, plains, mountains and oceans; a biome pass establishes
ecosystems; structure and feature passes add further content. Its documentation
defines a biome as a geographic and ecological region and distinguishes terrain,
biomes, features, structures and entities.
[Microsoft, *World Generation Overview*](https://learn.microsoft.com/en-us/minecraft/creator/documents/world-generation?view=minecraft-bedrock-stable)

**External fact.** A Minecraft chunk is a 16-by-16 horizontal portion of the world
used for generation, rendering and simulation distance. Render distance and
simulation distance are separate: the server can process a smaller chunk radius
than the client draws.
[Microsoft, *Simulation Distance, Render Distance, and Ticking Areas*](https://learn.microsoft.com/en-us/minecraft/creator/documents/simulationrenderdistanceguide?view=minecraft-bedrock-stable)

**Inference.** Minecraft separates at least three questions:

1. what exists at an exact block position;
2. what environmental region influences that position; and
3. which technical chunks are currently generated, rendered or simulated.

A biome is not the block address, and a chunk is not automatically a meaningful
location. The same forest can cross chunk boundaries, while one chunk can contain
several locally meaningful structures.

**Transferable lesson.** “Forest edge” may be terrain or environmental coverage
without being the Character's only canonical spatial identity. Operational chunks
must remain disposable implementation structure rather than Place identity.

**Non-transferable assumption.** Minecraft's blocks are its core building mechanic.
Aicadia's current birdhouse is a durable Entity, not a construction assembled from
thousands of simulated material cells. A voxel substrate would change the game, not
merely improve its database.

## 3. OpenMW: continuous position, cells, terrain, regions and doors coexist

**External fact.** OpenMW describes cells as the basic world-building containers
needed to render and process a large world. Exterior cells form a coordinate grid,
contain terrain and join into a seamless landmass crossed by walking over their
boundaries. Interior cells model houses, dungeons and mines and are entered through
door objects or teleportation. Object instances carry 3D coordinates relative to
their parent cell.
[OpenMW, *World Tables — Cells and Instances*](https://openmw.readthedocs.io/en/stable/manuals/openmw-cs/tables-world.html#cells)

**External fact.** Terrain is stored in land records paired with exterior cells.
Regions are separately assigned to cells and define rules such as weather, ambient
sound and random encounters. A cell-level pathgrid is yet another structure: points
and edges which NPCs use to navigate complicated surroundings.
[OpenMW, *World Tables — Lands, Pathgrids and Regions*](https://openmw.readthedocs.io/en/stable/manuals/openmw-cs/tables-world.html#lands)

**Inference.** OpenMW gives an especially useful counterexample to “everything is a
Place.” A Character can stand at an exact point in wilderness terrain. That point
belongs to an engine cell, may inherit a named region, and may be near a door or a
pathgrid edge. These records answer different questions and do not need one shared
identity.

**Inference.** The forest edge and heath can be remembered as terrain or region
coverage while each Character keeps an exact position. Two Characters may meet in
the same cell without the cell being a named settlement, and may also be far apart
inside that cell. A door is a physical object enabling a discontinuous transition;
ordinary movement across exterior cell boundaries needs no door or explicit Place
Connection.

**Transferable lesson.** If Aicadia needs continuous in-between play, the clean
conceptual separation is position → optional environmental coverage → meaningful
Place, with explicit Connections reserved for discontinuities or structural travel
rules. It is not “make every cell a Place.”

**Non-transferable assumption.** OpenMW runs a 3D realtime world whose renderer,
physics and NPC navigation already require exact coordinates. Aicadia has no such
accepted engine or geometry. Introducing coordinates would create new rules for
frames, distance, overlap, placement, movement, observation and concurrency.

## 4. Evennia: a Place graph is coherent precisely because it omits the in-between

**External fact.** Evennia Rooms are root containers of other game objects. Exits
connect Rooms, are one-way by default, hold a destination and move a Character by
changing its location from the source Room to the destination Room. Evennia has no
coordinate system by default; coordinates are an optional extension.
[Evennia Rooms](https://www.evennia.com/docs/latest/Components/Rooms.html),
[Evennia Exits](https://www.evennia.com/docs/latest/Components/Exits.html),
[Evennia coordinate extension](https://www.evennia.com/docs/0.x/Coordinates.html)

**Inference.** This is the closest researched analogue to Aicadia's current
Place-plus-Connection direction. It is internally consistent. A room need not be a
literal room: it can represent a square, forest clearing, road section or heath.
But the price is exact: there is no persistent “halfway between two Rooms” unless
the designer adds another Room, a travel state or a coordinate/grid system.

**Inference.** A graph world therefore does not claim that every physical patch is
a Place. It claims that only established gameplay stops matter as durable spatial
state. Unrepresented distance is compressed into the transition.

**Transferable lesson.** Variable-size Places plus directional Connections are a
valid sparse game model, not a database mistake. They fit text-mediated play well
when discovery establishes meaningful stops and travel between them is summarized.

**Boundary.** They fit poorly if “walk anywhere, meet anywhere, put something down
anywhere” is a core promise. Repeatedly minting tiny Places only to simulate a hidden
grid would reproduce a grid with worse nomenclature and less predictable geometry.

## 5. Smallville: a finite hybrid uses both tiles and semantic addresses

**External fact.** The Generative Agents Smallville code stores a finite 2D tile
matrix. Each tile can carry `world`, `sector`, `arena`, `game_object`, collision and
current events. Reverse indexes map a semantic `world:sector:arena:object` address
to all tiles bearing that address, while nearby-tile queries use tile coordinates.
[Smallville `maze.py`, inspected commit](https://github.com/joonspk-research/generative_agents/blob/fe05a71d3e4ed7d10bf68aa4eda6dd995ec070f4/reverie/backend_server/maze.py)

**Inference.** Smallville demonstrates the hybrid directly: one arena can cover
many movement tiles, and a tile can answer both exact proximity and broader
semantic context. It does not force “bedroom” and “tile 58,9” to be the same kind of
thing.

**Transferable lesson.** If Aicadia later needs both nearby encounters and named
Place context, one canonical exact location plus separately indexed Place coverage
is a proven composition.

**Non-transferable assumption.** Smallville's map is small, finite and authored in
advance. Its hierarchy is fixed and its indexes are in-process structures. It does
not prove a persistent, partially known, million-Character World architecture.

## Comparative result

| Family | Canonical Character location | What remembers terrain | Meaningful location | What happens between named locations |
|---|---|---|---|---|
| Civilization | one hex tile | tile terrain and features | Settlement and larger map concepts over tiles | every hex remains playable and occupiable |
| Minecraft | exact block position | blocks plus biome and generated features | structures or player meanings over the block world | every valid position remains playable |
| OpenMW | 3D position relative to a cell | exterior land and region assignment | named regions, interiors and authored locations | seamless coordinate movement across exterior cells |
| Evennia | one Room | whatever the Room description or custom model records | the Room itself | omitted unless another Room or coordinate extension is added |
| Smallville | one 2D tile | authored tile layers | sector/arena/object address spanning tiles | tile-by-tile movement beneath semantic areas |

The consistent finding is not that one representation wins. It is that successful
systems do not make one record answer all of these questions:

- exact current location;
- terrain or environment;
- meaningful Place identity;
- direct traversability;
- a reusable journey;
- observation distance; and
- render, simulation or storage partition.

## What “everything is Place” could mean in Aicadia

### Meaning A — every metre, hex or cell is a Place

This would copy the *coverage* of Civilization while using the wrong name. It would
make a Place both a durable Entity role and a mechanical map cell. A forest, city,
house and anonymous cell would then compete for the same identity and placement
meaning.

**Recommendation:** reject this meaning. If Aicadia earns a cell or exact position
substrate, it should be separate from Place and it should not be an Entity.

### Meaning B — every persistent gameplay stop is a Place

This is the Evennia-like interpretation. The forest edge may be a Place because
Characters can deliberately stop, meet, discover and build there. The uninteresting
stretch beyond it need not exist as separate state. A Place may be a room, bridge
landing, clearing, district, heath or city; its size and name do not define the role.

**Inference:** this is not literally “everything.” It is “every established spatial
subject at which World actions settle.” It preserves sparse storage and simple
exact-placement queries.

**Cost:** a Character cannot have persistent location between Places. If travel
takes time, the model needs a travel state. If Characters can diverge from a
Connection, encounter each other halfway or place an Entity at an arbitrary point,
that point must become a Place or the model fails.

### Meaning C — Place overlays a finer spatial ground

This is the Civilization/OpenMW/Smallville family. A Character has an exact
position, tile or patch. A Place is a separate durable subject which may cover one
or many such positions. Forest edge and heath may be terrain coverage; Moss Village
may be a Place over part of it. A Character could be in the village, on grassland
and inside one operational cell simultaneously because those statements have
different meanings.

**Benefit:** arbitrary stopping, meeting, building, local distance, gradual travel
and landscape continuity become expressible without inventing a Place per step.

**Cost:** Aicadia must choose a world frame or patch identity, boundary semantics,
how Place coverage is established while geography remains partially known, which
location is authoritative, how exact positions move atomically and how all reads
remain bounded. This is a new foundation, not a small field added to `place`.

## Candidate Aicadia shapes

These are research options, not accepted models.

### Option 1 — sparse Place graph

Concrete game consequence:

- a Character is at one established Place;
- a forest edge or open heath becomes a Place only when the World establishes it as
  a persistent stop;
- a Connection permits a later deliberate move between two Places; and
- unrepresented terrain is part of the journey description, not an occupiable
  address.

Technical consequence:

- `character` keeps exact direct Place placement;
- `connection` stores two Place endpoints plus explicit allowed directions;
- terrain description can initially belong to Place or Connection content without
  authorizing movement rules; and
- presence and hot-subject contention remain scoped to established Places.

This is the smallest continuation of the currently selected first scene.

### Option 2 — dense tile or cell substrate

Concrete game consequence:

- every traversable map unit exists and can hold Characters, terrain and local
  effects;
- Places such as villages cover or refer to one or more units; and
- ordinary paths follow adjacency while doors, portals and other discontinuities
  use explicit structural transitions.

Technical consequence:

- the tile/cell becomes canonical placement identity without becoming an Entity;
- Place membership or coverage becomes a separate indexed fact;
- generation, persistence and queries must remain bounded by explored or requested
  range; and
- millions of Characters distribute naturally across cells, but one truly hot cell
  or one shared Place still needs bounded admission rather than a global lock.

This is closest to Civilization but least compatible with “store only established
meaning” unless cells are generated lazily and have deterministic identity.

### Option 3 — continuous position with derived operational cells

Concrete game consequence:

- Characters may stop, meet and build at arbitrary coordinates;
- terrain and Places can cover areas of that coordinate space; and
- Connections remain useful for doors, portals, one-way drops or other transitions
  not explained by ordinary proximity.

Technical consequence:

- every placement needs a declared spatial frame and coordinates;
- Place extents, distance, overlap and boundary rules become necessary before they
  can authorize gameplay;
- operational cells can index or stream positions but never become canon; and
- concurrent movement usually updates one Character's placement, while actions on
  a shared exact Entity or boundary still conflict on that smaller fact.

This is closest to OpenMW and supplies the strongest open-world promise at the
highest immediate domain cost.

### Option 4 — sparse discovered patches beneath Places

Concrete game consequence:

- only explored terrain patches exist; a Character can occupy one without every
  patch being a Place; and
- named Places may later cover or relate several patches.

Technical consequence:

- a patch needs stable identity, adjacency, creation authority, merge or relation
  rules, terrain, placement and history;
- patch size and boundary become correctness decisions; and
- two Agents discovering apparently identical ground recreate the same identity
  reconciliation problem already recognized for Place.

**Inference:** this sounds like a compromise but can easily become the most complex
option: it introduces a second sparse spatial identity without receiving the simple
mathematics of a fixed grid or continuous coordinates. It should not be selected
merely to avoid the word Place.

## Multiplayer and the remote-button scenario

The choice above governs presence and ordinary movement. It does not decide every
spatial Action.

A Character and another Character walking through the same forest need a common
current address or an explicit encounter rule. A dense position substrate makes
that address finer; a Place graph makes it the same Place; a journey model could
make it the same active Connection plus progress, but that is another current-state
concept.

A button that triggers a remote bomb needs none of those Characters to be local to
the bomb. Its correctness comes from an explicit operation-specific link, User or
Character authority, fresh endpoint state and an explicit bounded affected scope.
Changing from Places to tiles does not make remote causality safe automatically.
This agrees with the broader finding in the
[spatial multiplayer foundation](spatial-multiplayer-foundation.md): locality,
control, causal linkage and affected scope remain separate typed truths.

At million-Character scale, none of the four models makes one genuinely shared hot
subject conflict-free. Tiles can distribute unrelated Characters; Places can keep
the sparse world compact; operational cells can bound candidate reads. But one bomb,
one bridge state, one exact terrain patch or one communal Place fact still needs a
bounded transaction and an overload outcome scoped to that subject.

## Research conclusion for Aicadia

The User's discomfort is justified: **Aicadia should not call every piece of world
coverage a Place.** Civilization, Minecraft, OpenMW and Smallville all separate fine
spatial address from larger semantic location. Evennia shows that a Place-only
graph remains coherent only by deliberately compressing or omitting the
in-between.

The evidence does not yet prove that Aicadia needs a coordinate, tile or patch
substrate. Its Agent-mediated, sparse World may intentionally make discovery about
establishing meaningful locations rather than traversing every metre. The selected
second-Place scene can therefore still be the smallest first slice, provided its
schema does not declare that arbitrary in-between position will never exist.

**Recommendation for the continuing grill:** decide the player promise before the
data structure. The decisive distinction is not whether forest edge sounds like a
Place. It is whether a Character can persistently stop, meet, discover or place an
Entity at an unnamed point between established Places without first establishing
that point as a Place.

- If **yes**, Place plus Connection is not a complete long-term spatial foundation.
  A separate non-Entity exact-location substrate is required; tile, continuous
  position and sparse patch then deserve their own comparison.
- If **no**, variable-granularity Places plus Connections are not strange. They are
  the intended compression rule, and terrain along a journey need not become
  separately occupiable World state.

No table, field, coordinate frame, grid, terrain record or new Area decision is
accepted by this report.

## Sources

### First-party and project-owned game sources

- [Firaxis — Civilization VII: Managing Your Empire](https://civilization.2k.com/civ-vii/archive/dev-diary/managing-your-empire/)
- [Firaxis — Civilization VII: Improved Map Generation](https://civilization.2k.com/civ-vii/game-guide/gameplay/map-generation/)
- [Civilization VII patch notes — 5 February 2025](https://support.civilization.com/hc/en-us/articles/38337649895187-Civilization-VII-Patch-Notes-February-5-2025)
- [Civilization VII patch notes — 4 November 2025](https://support.civilization.com/hc/en-us/articles/46192509687443-Civilization-VII-Patch-Notes-November-4-2025)
- [Unciv `Tile` at inspected commit](https://github.com/yairm210/Unciv/blob/5b9ddb6cc415c0bb3f0673304104ff9eec4f327e/core/src/com/unciv/logic/map/tile/Tile.kt)
- [Minecraft Bedrock — World Generation Overview](https://learn.microsoft.com/en-us/minecraft/creator/documents/world-generation?view=minecraft-bedrock-stable)
- [Minecraft Bedrock — Simulation and Render Distance](https://learn.microsoft.com/en-us/minecraft/creator/documents/simulationrenderdistanceguide?view=minecraft-bedrock-stable)
- [OpenMW — World Tables](https://openmw.readthedocs.io/en/stable/manuals/openmw-cs/tables-world.html)
- [Evennia — Rooms](https://www.evennia.com/docs/latest/Components/Rooms.html)
- [Evennia — Exits](https://www.evennia.com/docs/latest/Components/Exits.html)
- [Evennia — Coordinate extension](https://www.evennia.com/docs/0.x/Coordinates.html)
- [Generative Agents — Smallville `maze.py` at inspected commit](https://github.com/joonspk-research/generative_agents/blob/fe05a71d3e4ed7d10bf68aa4eda6dd995ec070f4/reverie/backend_server/maze.py)

### Related Aicadia research

- [Spatial multiplayer foundation](spatial-multiplayer-foundation.md)
- [Persistent-game spatial models](persistent-game-spatial-model.md)
- [Locality, co-presence and observation](locality-co-presence-and-observation.md)
