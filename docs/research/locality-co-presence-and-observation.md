# Locality, co-presence and observation in shared worlds

> **Role / side:** sourced research report / development side.
> **Authority:** records this report's question, sources, findings and implications.
> **Excludes:** product decisions and current implementation contracts; see `docs/game/`.

Date: 2026-08-08

Status: generic research; no Aicadia game behavior or implementation decision

Related Aicadia research:

- [Persistent-game spatial models](persistent-game-spatial-model.md) compares
  Place identity, geometry, engine partitions and current location.
- [Hierarchical spatial placement](archive/hierarchical-spatial-model.md) explores
  containment and optional exact geometry.
- [Stable identity and sparse location](stable-identity-and-sparse-location.md)
  explores identity and deliberately imprecise location.

## Question

How do comparable persistent multiplayer, simulation, virtual-world and agentic
world systems decide:

- whether two actors are co-present;
- which entities and events are local to an actor;
- whether a containing or contained location also counts as local;
- when coordinates, distance, occlusion or a route graph matter;
- what an agent or client may observe;
- how unknown or incomplete boundaries affect those answers; and
- how semantic locality stays separate from loading, indexing and network
  interest management?

The purpose is to identify reusable patterns and their tradeoffs. It does not
select a product model for Aicadia.

## Method and evidence boundary

This report uses primary sources: official documentation and specifications,
project-owned source repositories pinned to inspected commits, and original
papers or project pages. A system's implementation is reported as evidence of
what that system does, not proof that its choice is universally correct.

The systems solve materially different problems. Evennia is a text-world
framework; Smallville and AI Town are small agent simulations; Concordia is a
configurable generative-simulation framework; Second Life is a mature virtual
world; Luanti and Unreal are spatial engines. Their differences are part of the
finding.

## Terminology

| Term | Meaning in this report |
|---|---|
| semantic place | A world location actors can name or refer to, such as a room, house or village |
| direct place | The one immediate place or container assigned to an actor or entity |
| containment | A declared parent/child relation between places or located things |
| co-presence | A gameplay or simulation rule saying actors share sufficient locality to interact |
| proximity | A coordinate-based distance relation |
| visibility | A rendering, line-of-sight or presentation result |
| observability | Information the simulation is allowed to expose to one actor or agent |
| relevance | A candidate set that may affect a client or actor; not necessarily something they can perceive |
| interest management | Selecting state to load, simulate or replicate for performance |
| partition | A technical cell, chunk, region, process or shard boundary |
| unknown space | Space whose state has not been established; it is neither known-free nor known-occupied |

These terms form a pipeline rather than synonyms:

```text
authoritative presence
        ↓
candidate locality or relevance
        ↓
gameplay observability and access rules
        ↓
agent observation or client replication
```

A server may use the same predicate at two stages, but that is an implementation
choice. It should not be assumed from the words “nearby,” “visible” or “local.”

## Comparative result

| System | Authoritative spatial model | Co-presence or local-observation rule | Nesting | Technical relevance or partition |
|---|---|---|---|---|
| Evennia | Objects have one direct `location`; Rooms and Exits are Objects | Default room contents, messaging and search use direct location membership; access locks filter what can be found or viewed | Containers are possible because any Object can be a location, but direct contents are not automatically the contents of every ancestor | Database search scope; no required coordinate grid |
| Generative Agents / Smallville | A 2D tile map plus `world:sector:arena:object` addresses | Nearby tiles are candidates, but events must also have the exact current arena path; closest events are limited by attention bandwidth | Explicit world → sector → arena → object hierarchy | Small finite matrix and radius scan |
| AI Town | Continuous 2D player positions over a finite tile map | Conversation starts only after participants are closer than a fixed distance; accepted participants then have explicit Conversation membership and state | No semantic place hierarchy in the inspected engine | One live world state plus pathfinding and collision tiles |
| Concordia | Scenario-defined state mediated by a Game Master | Core observation delivery is configurable; a contributed partial-observability component uses exact string-location equality and treats missing location as unobservable | Scenario policy, not an intrinsic spatial hierarchy | Per-agent observation queues and optional participant filters |
| Second Life | Region-relative coordinates, named simulator Regions and separately identified parcels | APIs offer different scopes: whole Region, parcel, parcel-owner scope, or a metric sensor radius and arc | Parcels sit within one Region, but parcel membership and metric sensing remain different queries | Region/simulator boundary plus viewer/server relevance mechanisms |
| Luanti | 3D object positions and voxel nodes | Games can query objects within a Euclidean radius; the engine itself does not supply named-room co-presence | Object attachment supplies a transform parent, not a semantic place hierarchy | 16×16×16 MapBlocks plus static and active object states |
| Unreal Engine | Actors in continuous world coordinates | Network relevancy composes ownership, attachment, hidden state and optional distance rules | Attached Actors may inherit owner/base relevancy | World Partition streams grid cells by distance from streaming sources; rendering culling is another separate layer |

The table exposes three families rather than one industry standard:

1. **Identity locality:** direct room or scene identity is authoritative.
2. **Geometric locality:** distance, angle, collision or occlusion is authoritative.
3. **Hybrid locality:** a semantic scope first limits candidates, then geometry,
   permissions, attention or action-specific rules refine the result.

Agentic systems add a fourth concern: the world must turn authoritative state
into a bounded observation payload for an inference process. That payload is a
view of the world, not world-state itself.

## Primary evidence

### Evennia: exact direct location is the default locality boundary

Evennia gives every in-game Object a direct `location`. Its `contents` property
returns Objects whose `location` points to that Object; `msg_contents()` sends to
those direct contents. Characters, Rooms, Exits and ordinary Objects share the
same Object base. Default object search is scoped to the caller's current
location and inventory unless a different or global scope is requested.
[Evennia Objects](https://www.evennia.com/docs/3.x/Components/Objects.html),
[Evennia Object API](https://www.evennia.com/docs/2.x/api/evennia.objects.objects.html)

Visibility and reachability are separate. `look` applies a `view` access lock,
while an Exit stores a destination, checks a `traverse` lock and moves an Object
by replacing its source `location` with the destination. A one-way Exit is enough
to connect two Rooms in one direction; geometry is not required.
[Evennia Objects](https://www.evennia.com/docs/3.x/Components/Objects.html),
[Evennia Exits](https://www.evennia.com/docs/latest/Components/Exits.html)

**Inference:** exact room identity is a complete and scalable semantic rule for
room-like interaction. Nesting does not imply recursive observability by itself:
an item directly inside a closed chest and a Character directly inside the Room
have different immediate locations. A game must deliberately add recursive
container, openness or sound rules if it wants the Character to perceive the
item.

### Generative Agents: hierarchy, radius and exact arena identity work together

Smallville's environment is a 2D tile matrix. Each tile carries `world`, `sector`,
`arena`, `game_object`, collision and event data. The engine also builds reverse
indexes from hierarchical string addresses to the tiles belonging to each
address. This gives semantic areas and objects a many-tile footprint without
making the address equal to one coordinate.
[Smallville `maze.py`, inspected commit](https://github.com/joonspk-research/generative_agents/blob/fe05a71d3e4ed7d10bf68aa4eda6dd995ec070f4/reverie/backend_server/maze.py)

Perception then combines several gates:

1. `vision_r` selects a square of nearby tiles;
2. the agent learns world, sector, arena and object addresses found there;
3. an event is eligible only when its tile's arena path exactly equals the
   observer's current arena path;
4. eligible events are sorted by Euclidean tile distance; and
5. only the closest `att_bandwidth` events become perceptions.

The default scratch state uses a four-tile vision radius and an attention
bandwidth of three. Perceived events are written into associative memory rather
than treated as the entire current world.
[Smallville `perceive.py`, inspected commit](https://github.com/joonspk-research/generative_agents/blob/fe05a71d3e4ed7d10bf68aa4eda6dd995ec070f4/reverie/backend_server/persona/cognitive_modules/perceive.py),
[Smallville `scratch.py`, inspected commit](https://github.com/joonspk-research/generative_agents/blob/fe05a71d3e4ed7d10bf68aa4eda6dd995ec070f4/reverie/backend_server/persona/memory_structures/scratch.py)

The original paper describes the agents as observing their environment, storing
experiences and retrieving them for planning; the published sandbox contains 25
agents rather than evidence of internet-scale operation.
[Generative Agents publication](https://research.google/pubs/generative-agents-interactive-simulacra-of-human-behavior/)

**Inference:** Smallville is not evidence for “same Place only” or “distance
only.” It is evidence for a hybrid: hierarchy provides meaningful location,
exact arena identity prevents seeing through semantic room boundaries, distance
orders candidates, and attention bounds agent context.

### AI Town: geometric encounter, then explicit interaction membership

AI Town stores a finite map with width, height and tile layers, while Players
carry continuous 2D positions. Its inspected engine sets
`CONVERSATION_DISTANCE = 1.3`. Invited Players first enter a `walkingOver` state;
only when their positions fall within that distance does the simulation stop
them and change both memberships to `participating`. Conversation identity,
participants, typing state and message state then persist independently of the
raw distance test.
[AI Town map model, inspected commit](https://github.com/a16z-infra/ai-town/blob/7b242334bfbfef02f7718bded120d431e8f307df/convex/aiTown/worldMap.ts),
[AI Town constants, inspected commit](https://github.com/a16z-infra/ai-town/blob/7b242334bfbfef02f7718bded120d431e8f307df/convex/constants.ts),
[AI Town Conversation, inspected commit](https://github.com/a16z-infra/ai-town/blob/7b242334bfbfef02f7718bded120d431e8f307df/convex/aiTown/conversation.ts)

**Inference:** a proximity predicate is useful for starting an encounter, but
it is a poor substitute for the encounter's durable state. If distance were the
only truth, small position changes during a conversation would repeatedly create
and destroy the interaction.

### Concordia: observation scope is explicit policy, not a built-in world law

Concordia uses a Game Master to receive natural-language action attempts, resolve
their effects and generate observations. Its core `MakeObservation` component
queues observations per named entity and can fall back to asking an LLM what the
active entity observes. Event resolution can ask which entities are aware of an
event, and other components can supply a deterministic participant filter before
delivery.
[Concordia publication](https://deepmind.google/research/publications/64717/),
[Concordia `make_observation.py`, inspected commit](https://github.com/google-deepmind/concordia/blob/513c3d622d19cf99f1c2f63991b648ffd3d5fcb5/concordia/components/game_master/make_observation.py),
[Concordia `event_resolution.py`, inspected commit](https://github.com/google-deepmind/concordia/blob/513c3d622d19cf99f1c2f63991b648ffd3d5fcb5/concordia/components/game_master/event_resolution.py)

A 2026 contributed `LocationBasedFilter` makes one concrete policy explicit. It
extracts named locations from a scene narrative, maintains
`entity → location` and `location → entities` maps, and allows observation only
when observer and target have equal location strings. If either location is
missing, `can_observe` returns false. Its narrative filter first tries explicit
scene participation and may use an LLM fallback for prose filtering.
[Concordia `location_based_filter.py`, inspected commit](https://github.com/google-deepmind/concordia/blob/513c3d622d19cf99f1c2f63991b648ffd3d5fcb5/concordia/contrib/components/game_master/location_based_filter.py)

**Inference:** generative-agent infrastructure does not remove the need for a
locality contract. Concordia deliberately makes observation a configurable Game
Master responsibility. Exact location equality is one supplied policy, while
LLM-generated awareness is another. The latter is appropriate for exploratory
simulation but does not by itself provide reproducible authoritative behavior.

### Second Life: scope depends on the operation

Second Life distinguishes a named simulator Region, Region-relative positions
and parcels within a Region. Its agent-list API can return agents for an entire
Region, one parcel, or parcels with the same owner. Its sensor API instead scans
for avatars or objects within a metric range and angular arc; the documented
range is at most 96 metres, and sensors do not detect across Region boundaries.
[Second Life land model](https://wiki.secondlife.com/wiki/Land),
[`llGetAgentList`](https://create.secondlife.com/script/lsl-reference/functions/llgetagentlist/),
[`llSensor`](https://create.secondlife.com/script/lsl-reference/functions/llsensor/),
[`llSensor` caveats](https://wiki.secondlife.com/wiki/LlSensor)

**Inference:** Region membership, parcel membership and physical proximity are
all valid notions of locality, but for different operations. Sharing a Region
does not mean being within sensor range; sharing a parcel is not required for a
nearby sensor result. “Local” needs an action-specific definition.

### Luanti: positions and operational blocks do not create semantic places

Luanti represents moving things as Players or entities with 3D positions. Its
mod API exposes `get_objects_inside_radius(center, radius)` using a Euclidean
metric. Static objects are saved in MapBlocks; active objects are loaded and
updating. A MapBlock contains 16×16×16 nodes and is a storage, mesh and transfer
unit. A MapChunk groups 5×5×5 MapBlocks for generation.
[Luanti core API](https://api.luanti.org/core-namespace-reference/),
[Luanti Objects](https://docs.luanti.org/for-engine-devs/objects/),
[Luanti basic data structures](https://docs.luanti.org/for-engine-devs/basic-data-structures/)

Objects can also attach to a parent Object and use relative position and
rotation. The documented attachment relation is a transform relation; Luanti
does not define that an attached or nearby Object belongs to a named room,
village or territory.
[Luanti ObjectRef](https://api.luanti.org/class-reference/)

**Inference:** the same engine can need exact coordinates, a radius query,
parent-relative transforms and technical blocks without any of those becoming
the world's semantic Place model.

### Unreal Engine: three different visibility-like systems coexist

Unreal's World Partition stores one persistent level and assigns spatially
loaded Actors to grid cells. Streaming sources such as Player Controllers load
cells within range. This is a large-world loading mechanism.
[Unreal World Partition](https://dev.epicgames.com/documentation/en-us/unreal-engine/world-partition-in-unreal-engine)

Network relevancy is separately calculated per connection. The documented rules
compose “always relevant,” ownership, owner-inherited relevancy, owner-only
relevancy, attachment, hidden/collision state and optional net-cull distance.
Only the resulting relevant Actor set is replicated to that client.
[Unreal Actor relevancy](https://dev.epicgames.com/documentation/en-us/unreal-engine/actor-relevancy-and-priority-in-unreal-engine)

Rendering visibility is separate again: distance, view-frustum, precomputed and
dynamic occlusion culling decide whether loaded content is drawn.
[Unreal visibility and occlusion](https://dev.epicgames.com/documentation/en-us/unreal-engine/visibility-and-occlusion-culling-in-unreal-engine)

**Inference:** loading, replication and rendering may all use location, yet
remain different contracts. A network relevance set is allowed to over-include
things the player cannot literally see because it approximates what could affect
the client.

## Evidence from mapping partially known space

Agent prototypes usually start with a fully authored small map or a prose scene.
They provide little direct evidence for a world whose boundaries are discovered
gradually. Mapping systems supply two useful, narrower precedents.

### Identity, representative point, area and boundary can be separate

Overture Maps represents a `division` with feature identity, hierarchy and an
approximate representative Point. A separate `division_area` references the
division and supplies Polygon or MultiPolygon geometry. A separate
`division_boundary` links the two divisions on either side of a line. Boundary
records may mark a disputed or “best guess” boundary and may carry alternate
political perspectives.
[Overture Divisions guide](https://docs.overturemaps.org/guides/divisions/),
[Division schema](https://docs.overturemaps.org/schema/reference/divisions/division/),
[Division area schema](https://docs.overturemaps.org/schema/reference/divisions/division_area/),
[Division boundary schema](https://docs.overturemaps.org/schema/reference/divisions/division_boundary/)

This does not directly model exploration. It does prove that a place-like
identity, an anchor, a containing hierarchy, an area and boundary segments need
not be one field or share one certainty.

### Unknown is a first-class map state

ROS's standard 2D `OccupancyGrid` distinguishes occupancy probabilities
`0..100` from `-1`, which means unknown. OctoMap's project explicitly states that
unknown areas matter for autonomous exploration and distinguishes them from free
and occupied 3D space.
[ROS `OccupancyGrid`](https://docs.ros.org/en/kinetic/api/nav_msgs/html/msg/OccupancyGrid.html),
[OctoMap project and paper](https://octomap.github.io/)

**Inference:** absent boundary evidence should not silently mean “outside,”
“inside,” “empty” or “free.” A query over incomplete geometry needs an explicit
unknown result or a separate authoritative fallback such as direct Place
membership.

## Patterns and tradeoffs

### Pattern 1: exact direct-place identity

```text
local(A, B) := A.place_id = B.place_id
```

Strengths:

- deterministic without coordinates or complete boundaries;
- one indexed equality lookup;
- easy to explain in an observation payload; and
- well matched to rooms, scenes and graph worlds.

Weaknesses:

- a Character in a house and one in its containing village are not local unless
  another rule says so;
- large Places have uniform locality regardless of physical distance; and
- doors, walls, sound and line of sight need separate behavior.

### Pattern 2: containment expansion

```text
local(A, B) := direct place equal
            OR one direct place is an allowed ancestor of the other
```

Strengths:

- useful for nested narrative context such as room → house → village; and
- works without exact geometry.

Weaknesses:

- “ancestor” does not imply sensory access: a cellar is within a village but not
  necessarily visible from its square;
- recursion can expose private or closed interiors accidentally;
- multiple parents or overlapping hierarchies make the candidate set larger;
  and
- the rule needs an allowed relation type, direction and maximum scope.

### Pattern 3: geometry or distance

```text
local(A, B) := distance(A.position, B.position) <= action_radius
```

Strengths:

- natural for continuous movement, hearing, combat and encounters;
- supports graded distance and direction; and
- spatial indexes can scale candidate search.

Weaknesses:

- requires compatible coordinate frames and sufficiently precise positions;
- distance alone sees through walls and across disconnected floors;
- a threshold produces churn at its edge unless the interaction gains durable
  state or hysteresis; and
- incomplete geometry cannot prove containment or occlusion.

Polygon containment adds another edge choice. PostGIS `ST_Covers` includes a
boundary while `ST_Contains` does not contain every point of its boundary. Two
adjacent or overlapping polygons can therefore return multiple candidates; a
geometric predicate alone does not choose one canonical gameplay Place.
[`ST_Covers`](https://postgis.net/docs/ST_Covers.html),
[`ST_Contains`](https://postgis.net/docs/ST_Contains.html)

### Pattern 4: graph or portal locality

```text
local(A, B) := reachable through an allowed edge within N steps
```

Strengths:

- models rooms, doors, portals, caves and disconnected spaces;
- requires no Euclidean geometry; and
- access and sound rules can attach to edges.

Weaknesses:

- graph distance is not physical distance;
- a traversal edge does not automatically mean visual or acoustic permeability;
  and
- changing doors or permissions changes locality dynamically.

### Pattern 5: Game-Master or narrative observation

```text
observation(A) := resolver(world_state, recent_events, A)
```

Strengths:

- can express social knowledge, attention, disguise and narrative relevance;
- produces compact natural-language context for an LLM agent; and
- works in worlds without formal geometry.

Weaknesses:

- LLM resolution can vary across runs;
- hidden facts can leak through prompts or summaries;
- it is difficult to audit why one agent received an event; and
- it moves authoritative filtering into inference unless deterministic candidate
  selection happens first.

### Pattern 6: staged hybrid

The researched systems which expose multiple stages compose rules:

```text
candidates = same_scene_or_spatial_query(actor)
allowed    = apply_access_occlusion_and_event_rules(candidates)
payload    = rank_summarize_or_replicate(allowed, actor_budget)
```

Smallville combines arena equality, radius and attention. Unreal combines
ownership, state and distance for replication. Second Life exposes different
Region, parcel and radius scopes. The stages are useful because each can answer
a different question without pretending to be the source of every other truth.

## Concrete edge cases

### House inside a village

Ada is directly in `Lantern House`; Bo is directly in `Moss End` square. The
house is declared inside the village.

- Direct-place equality: they are not co-present.
- Containment expansion: they may share village context.
- Sensory behavior: still needs doors, range or a named village-wide action.

The containment fact alone does not settle whether they can see, hear, speak to
or discover each other.

### A doorway on a shared boundary

Ada stands exactly on the line between a house and street. A boundary-inclusive
query can cover both. Possible deterministic policies include one primary direct
Place, an explicit transition state, or returning both as spatial context. A
polygon predicate cannot choose among them without such a policy.

### A known anchor with an undiscovered boundary

An expedition knows a marsh's representative point and several surveyed edge
segments, but not a closed extent. Direct membership may still be established by
a world event; point-in-polygon cannot yet answer. “No polygon match” must not be
reported as proof that the expedition is outside the marsh.

### Overlapping places

A cottage is in both a forest and a jurisdiction. These relations can both be
true while only one direct Place is used for room-like co-presence. A visibility
query must say whether it wants physical enclosure, jurisdiction, social context
or all covering extents.

### A moving container

Two Characters aboard the same ship may be co-present even as the ship changes
global position. A direct ship or cabin identity remains stable; child transforms
or absolute coordinates answer a different question. Ancestor expansion must not
make everyone in the surrounding ocean automatically present inside the cabin.

### Crossing a proximity threshold mid-conversation

Two agents start talking inside a permitted range, then drift slightly apart.
AI Town demonstrates one solution: proximity starts the encounter, after which
explicit Conversation membership carries interaction state. Otherwise a noisy
coordinate can repeatedly end and restart the conversation.

### Remote but observable events

A bell, broadcast, map update or public announcement may be observable outside
direct locality. This is not evidence that all Places are one locality; it is an
event-specific delivery rule with its own range, channel or audience.

## Sparse and potentially unbounded worlds

The researched agentic prototypes do not establish an industry-proven model for
an unbounded persistent world. Smallville and AI Town use finite authored maps;
Concordia scenarios are configured simulations. Mature engines demonstrate
operational scale mechanisms, not a universal semantic ontology.

Three independent growth axes should therefore be kept distinct in generic
design analysis:

- **semantic growth:** new place identities and relations are added without
  pre-allocating every possible place;
- **spatial growth:** coordinate or occupancy records exist only where explored
  or generated; and
- **operational growth:** cells, indexes, caches or shards restrict the working
  set without becoming world identity.

A room graph can grow sparsely with exact ID locality and no coordinates. A
coordinate world can grow sparsely by persisting only generated or discovered
blocks. Either can later use technical partitions. “Unbounded” does not by itself
select direct-place, hierarchy or distance semantics.

## Implications for an open-ended agentic shared world

These are design implications from the evidence, not Aicadia decisions.

### Define locality per action

“Can talk,” “can inspect,” “appears in a place briefing,” “is affected by an
explosion,” and “must be replicated to a client” may need different scopes. One
global `is_local` function tends to hide this distinction.

### Keep current presence separate from observation history

An agent may remember a person who has left. Current co-presence should come from
authoritative current state; memory retrieval may enrich the prompt but must not
reintroduce a stale entity as currently visible.

### Treat observation as a first-class, bounded read model

An agent observation can state:

- the observer and authoritative direct location;
- the rule or scope used for included entities and events;
- current observable results;
- known surrounding or ancestor context when requested; and
- which spatial facts are unknown rather than silently absent.

This makes context limits and information barriers auditable. Natural-language
summarization can follow deterministic selection rather than decide secret access
implicitly.

### Make hierarchy expansion explicit

If nested Places count for an action, define:

- which relation qualifies;
- whether expansion goes upward, downward or both;
- whether closed/private containers block it;
- the maximum depth or named stopping scope; and
- what happens with multiple parents.

“Same broader Place” is not implementable until these are answered.

### Preserve unknown boundary state

Incomplete geometry should yield `unknown` where geometry is required. A direct
place assertion, anchor, discovered boundary fragments and a closed extent can
coexist without pretending each carries the certainty of the others.

### Derive technical partitions

Chunk, cell, Region-server or spatial-index membership can accelerate candidate
selection. Repartitioning should not rename a semantic Place, change remembered
history or determine game access unless the game explicitly exposes that
partition as world-state.

### Test the rule, not just the query

Useful contract tests for any selected locality model include:

- same direct Place and different direct Place;
- nested open and nested closed Places;
- incomplete and missing geometry;
- an exact shared boundary and overlapping extents;
- moving containers and coordinate-frame changes;
- stale agent memory versus current presence;
- a remote event with explicit audience; and
- a technical repartition that leaves semantic results unchanged.

## What the evidence does not settle

The sources do not determine:

- whether a Character should have one primary Place or several simultaneous
  gameplay Places;
- whether parent and child Places should share default visibility;
- which actions need metric range;
- whether unknown geometry should fall back to direct membership for every
  action;
- whether observations should return raw structured state, prose, or both; or
- which locality behavior is the next Aicadia build contract.

Those are product decisions. The evidence narrows the choice: exact identity,
containment, geometry, observability and interest management are independent
dimensions, and systems that need several of them compose them deliberately.

## Primary source audit

Checked on 2026-08-08:

- Evennia official 3.x/latest component documentation and project API for Object
  location, contents, search, locks and Exit traversal.
- Generative Agents' official publication page and author-owned repository at
  commit `fe05a71d3e4ed7d10bf68aa4eda6dd995ec070f4` for tile hierarchy,
  radius, arena equality and attention limits.
- AI Town's project-owned repository at commit
  `7b242334bfbfef02f7718bded120d431e8f307df` for positions, finite map,
  conversation distance and durable Conversation state.
- Google DeepMind's Concordia publication and repository at commit
  `513c3d622d19cf99f1c2f63991b648ffd3d5fcb5` for Game Master observation
  delivery and exact-location partial observability.
- Linden Lab's Second Life creation reference and official wiki for Region,
  parcel and sensor scopes.
- Luanti's official engine and API documentation for positions, radius queries,
  attachments, active/static objects and MapBlocks.
- Epic's official Unreal Engine documentation for World Partition, network
  relevancy and rendering culling.
- Overture's official 2026 Divisions guide and schema reference for Division,
  DivisionArea and DivisionBoundary.
- ROS official message documentation and the OctoMap project/original-paper page
  for explicit unknown map state.
- PostGIS official function documentation for boundary-inclusive and
  boundary-exclusive containment predicates.

No secondary comparison article or unsourced game wiki is used as evidence.
