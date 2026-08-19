---
status: pending
era: August 2026 spatial multiplayer reset
---

# Spatial multiplayer foundation

> **Role / side:** new sourced research into spatial multiplayer foundations / development side.
> **Authority:** records this report's external facts, Aicadia inferences, unaccepted
> recommendations and open product choices.
> **Excludes:** current game behavior, accepted product direction, implementation
> architecture and delivery evidence; those remain owned by `game/docs/`,
> `dev/docs/concept/`, accepted plans and `dev/docs/evidence/`.

Controlled: 2026-08-18

Status: research only; nothing below changes the current Aicadia contract

## Question

What spatial foundation could let Aicadia become a compelling persistent multiplayer
game in which millions of Users can discover, connect, inhabit, manipulate and
settle one sparse World, while one deliberately hot Place or Entity remains bounded
and every spatial, relational and control decision stays deterministic?

The research covers four intertwined player promises:

1. a Place or Entity remains the same referent while names, location, geometry,
   contents, connections, operational partition and control change;
2. Characters can act locally, remotely, through routes or portals, inside moving
   containers and across explicit relations without World inferring meaning from
   prose;
3. multiplayer eligibility, privacy, observation, conflict and history follow the
   exact spatial and authority facts that the concrete action needs; and
4. sparse distribution scales independently while contradictory work on one hot
   fact receives a bounded, truthful result.

This is a fresh investigation. Earlier Aicadia spatial reports were not used as
factual authority. Relevant external claims were checked again against current
primary sources. Repository contracts and current multiplayer records are used only
to identify Aicadia's actual constraints and scenarios.

## Evidence language and non-decision boundary

- **External fact** is a statement directly supported by an official specification,
  first-party manual, project-owned source tree or original paper.
- **Repository fact** is a statement currently owned by an Aicadia authority or a
  bounded lab record.
- **Inference** is this report's analysis of one or more facts.
- **Recommendation** is an unaccepted research conclusion for later grilling. It is
  not current game behavior or permission to build.
- **Open question** requires a product choice, an implementation experiment or both.

No recommendation in this report overrides the [current game contract](../../../game/docs/README.md),
the live [spatial direction](../concept/spatial.md), the active
[multiplayer exploration](../concept/concurrency-and-world-dynamics.md), or the draft
[multiplayer plan](../../plans/20260816-153410-multiplayer-lab/plan.md).

## Repository baseline and scenario boundary

**Repository fact.** A Place is currently an Entity role with the same stable
`entity_id`; Character and ordinary Entity placement is zero-or-one exact current
Place; exact-Place reads are bounded pages; accepted mutations write current state
and one immutable Activity atomically. Movement, further Places, containment,
adjacency, routes, coordinates, geometry, possession, relations and access controls
are explicitly absent. Current contextual writers also serialize through one Place
row and `place.latest_activity_id`. These facts are owned by the
[Entity](../../../game/docs/model/entity/README.md),
[Character](../../../game/docs/model/character/README.md),
[Place](../../../game/docs/model/place/README.md),
[Activity](../../../game/docs/model/activity/README.md),
[protocol](../../../game/docs/protocol.md#delivery-identity-and-exact-place-freshness),
[storage](../../../game/docs/storage.md) and
[deferred-scope](../../../game/docs/deferred.md) contracts.

**Repository fact.** The active multiplayer catalogue fixes fourteen broader
scenarios, S01–S14, covering ordinary and hot Entity change, remote causality,
multi-Place effects, observation, atomic linked creation, stale placement,
World-scoped context, reconnect, communal settlement and causal loops. The catalogue
does not select outcomes or authorize capabilities. Its current authority is
[the Multiplayer Area scenario catalogue](../../areas/multiplayer/scenarios.md).

**Repository fact.** Retained labs support exact-fact conflict semantics in a bounded
model, refute complete Place-row independence for insertion through a real foreign
key, and refute a mixed current-row/Entity-fallback lock strategy. They do not prove
production transactions, movement, geometry, privacy or scale. The exact findings
and non-claims are recorded in
[01 observation ownership](../../lab/multiplayer/01-observation-ownership/README.md),
[02 subject conflict](../../lab/multiplayer/02-subject-conflict/README.md),
[03 PostgreSQL subject conflict](../../lab/multiplayer/03-postgres-subject-conflict/README.md)
and [04 conflict strategies](../../lab/multiplayer/04-postgres-conflict-strategies/README.md).

**Inference.** The current exact-Place slice is a useful layer, not a sufficient
spatial foundation. The new design problem is not “add coordinates.” It is to keep
identity, placement, topology, metric shape, traversal, authority, observation,
history and physical distribution independently correct while concrete operations
compose only the layers they need.

## Core result

The strongest research conclusion is a separation, not a universal spatial model:

| Structural truth | Player or World meaning | Why it must remain separate |
| --- | --- | --- |
| Stable subject identity | This is still the same House, Ship or Table | Moving, renaming, reshaping or repartitioning must not create a new referent |
| Immediate placement | The Lantern is directly in this Cabin or attached to this mast now | One current parent prevents simultaneous contradictory placement |
| Semantic Place membership | The Character is aboard the Ship and therefore in its Cabin context | May be derived through a bounded parent chain; it is not necessarily a coordinate predicate |
| Exact geometry and frame | The wall occupies this shape in this coordinate frame at this revision | Optional, versioned metric fact; not every subject needs it |
| Topological relation | The Yard is contained by the House grounds; two rooms are adjacent | May be asserted or geometry-derived, but the authority and boundary rule must be explicit |
| Traversal relation | The door, route or portal permits a directed transition under stated conditions | Reachability is not the same as adjacency or geometric contact |
| Control and access | This User may issue this operation through this Character or grant | Authorization is not fictional ownership, possession or proximity |
| Possession or attachment | The key is carried by Mara; the boat carries its passengers | Physical relation may affect locality but does not automatically grant control |
| Occurrence footprint | The accepted Action happened here and involved these stable subjects | Historical context must survive later movement and geometry change |
| Observation eligibility | This active Character may retrieve this public occurrence now | Operation-specific, privacy-filtered and distinct from Agent narration |
| Interest and delivery | This host wants a hint when these resources become stale | Disposable routing state, never World truth or personal memory |
| Operational cell or partition | These rows or computations are handled together today | Replaceable optimization; never a Place identity or player-visible boundary |

**Recommendation.** Treat this table as a future design checklist: every concrete
capability states which rows it reads, which it may change, which versions make it
stale, which history it writes and which authorized resources become dirty. Do not
require every capability to use every spatial layer.

**Open question.** Which smallest player behavior first needs more than exact Place
equality: additional connected Places, movement, a moving container, or one bounded
cross-Place occurrence?

## 1. Stable Place and Entity identity

### External evidence

**External fact.** Kubernetes distinguishes reusable human-facing names from
system-generated UIDs intended to distinguish historical occurrences of similar
objects. The UID is cluster-lifetime identity even if a name is later reused.
[Kubernetes object names and IDs](https://kubernetes.io/docs/concepts/overview/working-with-objects/names/)

**External fact.** RFC 9562 defines 128-bit UUIDs and multiple generation schemes,
including random UUIDv4 and time-ordered UUIDv7. It separately discusses collision
resistance, unguessability and distributed generation; one property does not imply
the others. [RFC 9562](https://www.rfc-editor.org/rfc/rfc9562.html)

### Alternatives

#### A. Identity derived from name, coordinates, parent path or cell

**Player or World consequence.** Renaming North Gate, moving a ship, redrawing a
boundary or repartitioning the database would make the “same” thing appear to be a
different thing. Historical references and other players' links would break.

**Technical consequence.** Every mutable component becomes part of a cascading key;
foreign keys, history, caches and relation endpoints require rewriting. A hot parent
path or cell can turn independent changes into shared coordination. Operational
repartitioning becomes a domain migration.

#### B. One opaque durable identity per Entity, with Place as an Entity role

**Player or World consequence.** The House, Ship or Valley stays the same referent
through movement, renaming, geometry revision and operational relocation. Two
same-named tables may remain distinct unless a deliberate game process establishes
identity.

**Technical consequence.** Relations, geometry versions, placement, history and
authorization all point to the stable Entity id. Names and spatial locators are
indexed attributes, not identity. UUIDs help decentralized allocation, but every API
still authorizes dereferenced ids; unguessability is not access control.

**Recommendation.** Preserve Aicadia's current One Subject, One Identity rule as the
root invariant. Never derive Place or Entity identity from geometry, containment,
route, owner, shard, H3 cell, process or current parent.

**Open questions.** Does a discovered disconnected fragment receive its own stable
Place identity until players later establish sameness? If a later merge is allowed,
does it add an explicit identity relation while preserving both historical ids, or
does one subject actually retire? That lifecycle remains a product decision.

## 2. Discrete presence and exact geometry

### External evidence

**External fact.** OGC GeoSPARQL deliberately supports both qualitative spatial
systems that assert binary topological relations without geometry and quantitative
systems that calculate relations from explicit geometries. Its standard separates
core spatial objects, topology vocabulary, geometry, and geometry-topology query
functions. [OGC GeoSPARQL 1.1, sections 7.3 and 9](https://docs.ogc.org/is/22-047r1/22-047r1.html)

**External fact.** PostGIS implements separate `geometry` and `geography` types.
`geometry` uses a chosen planar spatial reference system; `geography` performs a
smaller function set over a sphere or spheroid and is generally more expensive.
PostGIS also permits a local Cartesian SRS not tied to Earth.
[PostGIS spatial data management](https://postgis.net/docs/using_postgis_dbmanagement.html),
[geometry or geography](https://postgis.net/documentation/faq/geometry-or-geography/)

**External fact.** Metric predicates encode real boundary choices. `ST_Contains`
does not count objects lying only on a boundary, whereas `ST_Covers` does. Both warn
that invalid geometries produce unexpected results. Ordinary `ST_Intersects` can
report a 2D intersection where `ST_3DIntersects` reports none.
[ST_Contains](https://postgis.net/docs/ST_Contains.html),
[ST_Covers](https://postgis.net/docs/ST_Covers.html),
[ST_3DIntersects](https://postgis.net/docs/ST_3DIntersects.html)

### Alternatives

#### A. Discrete Place presence only

**Player or World consequence.** A Character is “in the Cabin” and can share local
context without the game pretending to know an exact point, wall or distance. This
supports rooms, settlements and portal-linked spaces, but not line-of-sight, precise
blast intersection or continuous travel.

**Technical consequence.** One indexed placement relation and Place equality keep
reads and writes simple. Boundary cases are explicit game transitions. Geometry
queries and coordinate frames are absent, so they cannot answer a later metric
mechanic without an added layer.

#### B. Geometry as the only location truth

**Player or World consequence.** Being “inside” becomes a point-in-polygon result.
Characters on a doorway or boundary may flip membership under predicate, precision
or geometry changes; a portal or magical connection cannot exist without fabricated
geometry.

**Technical consequence.** Every locality check pays coordinate-system, validity,
dimension, index and exact-recheck costs. Geometry edits can unexpectedly change
eligibility for many subjects. Moving containers require transform hierarchies
anyway. Geometry still does not encode traversal permission or access.

#### C. Discrete presence plus optional versioned geometry

**Player or World consequence.** Ordinary local play uses clear Place membership.
Only a mechanic that needs shape or distance asks for geometry. A doorway can be an
explicit traversal even before either room has a complete polygon.

**Technical consequence.** Immediate placement remains the cheap canonical locality
fact. Geometry is attached to stable subjects or frames as optional versioned state.
An operation chooses equality, topology, distance or intersection explicitly and
records the geometry versions it depended on. Two consistency systems must be kept
aligned only for operations that declare both.

**Recommendation.** Prefer layered option C. “Exact geometry” should mean an
authoritative version in an explicitly named coordinate frame and dimension, not
mathematical omniscience. Absence of geometry stays valid and never means origin
`(0,0,0)` or empty space.

**Open questions.** Which first mechanic truly needs 2D or 3D geometry? What does a
point exactly on a Place boundary mean for that mechanic? Is Aicadia's World planar,
planetary, locally framed, or a collection of disconnected spaces? No coordinate
type should be selected before those player consequences are chosen.

## 3. Coordinate frames, geometry validity and precision

**External fact.** `ST_SetSRID` only labels coordinates; it does not transform them.
`ST_Transform` changes coordinates between known spatial reference systems. Spatial
functions comparing geometries generally require a shared SRS.
[PostGIS `ST_SetSRID`](https://postgis.net/docs/ST_SetSRID.html),
[PostGIS `ST_Transform`](https://postgis.net/docs/ST_Transform.html)

**External fact.** PostGIS validity checks are 2D even for 3D/4D geometries, while
3D distance has separate functions and no vertical-datum transform. A 3D game cannot
assume that adding Z makes all 2D topology rules three-dimensional.
[ST_IsValid](https://postgis.net/docs/ST_IsValid.html),
[ST_3DDWithin](https://postgis.net/docs/ST_3DDWithin.html)

### Material frame choices

| Alternative | Player or World consequence first | Technical state, transaction, contention and cost consequence |
| --- | --- | --- |
| One World-wide Cartesian frame | Continuous distance and direction are simple everywhere; remote spaces cannot be truly disconnected | One coordinate domain is easy to query, but a semantically unbounded World eventually faces scale-dependent floating precision and huge global extents |
| One planetary/geodetic frame | Travel follows a globe and distances have geographic meaning | `geography` has fewer operations and higher cost; vertical and interior spaces still need another frame |
| Stable local frames connected by explicit transforms or portals | A Cabin, Ship, cave or pocket world can have its own coherent coordinates and move as a unit | Cross-frame queries require bounded transform chains and frame-version dependencies; cycles and depth must be controlled |
| No coordinates until a mechanic earns one | Early Place play remains clear and cheap | Future geometry remains possible because identity and relations are not coordinate-derived, but the first metric mechanic must choose and migrate its exact frame contract |

**Recommendation.** Keep the frame decision open, but require any future geometry
row to name its frame, SRID/units, dimension, geometry kind and version. Reject NaN,
infinity, unknown-unit comparison and silent SRID relabeling. Geometry validation,
byte size, component count and vertex count must be bounded before a spatial
predicate runs.

**External fact.** PostGIS exposes `ST_NPoints` for vertex count, and `ST_Subdivide`
documents that smaller indexed pieces reduce bounding-box false hits and exact
recheck cost. These are concrete signs that one stored geometry can impose work far
beyond its row count.
[ST_NPoints](https://postgis.net/docs/ST_NPoints.html),
[ST_Subdivide](https://postgis.net/docs/ST_Subdivide.html)

**Inference.** Geometry subdivision is an index representation, not permission to
turn one Place into many game Places. A derived piece must retain its owning stable
Place and geometry-version identity.

## 4. Containment is not one relation

The word “inside” hides at least four different mechanics:

| Relation | Concrete example | Required invariant | What it does not imply |
| --- | --- | --- | --- |
| Immediate placement | Key is directly in Chest | one current spatial parent for the Key | control, visibility or metric containment |
| Semantic Place containment | Cabin is part of Ship | chosen tree/DAG rule and bounded ancestors | physical transform, route or inherited access |
| Metric containment | Point or shape is covered by Room polygon | shared frame, valid geometry, explicit boundary predicate | a door, travel permission or observation |
| Lifecycle dependency | A generated component ceases with its owner | deletion/retirement semantics | spatial locality or User authority |

**External fact.** Kubernetes `ownerReferences` exists primarily to tell garbage
collection which objects depend on others, while RBAC separately governs who may
perform operations. This is a mature example where “owner” is lifecycle structure,
not access control.
[Kubernetes owners and dependents](https://kubernetes.io/docs/concepts/architecture/garbage-collection/#owners-and-dependents),
[Kubernetes RBAC](https://kubernetes.io/docs/reference/access-authn-authz/rbac/)

### Alternatives

#### A. One generic `contains` edge for every meaning

**Player or World consequence.** Putting a key in a chest could accidentally make
the chest's controller own the key, make every passenger observe it, or delete it
with the chest. “Inside” becomes unpredictable between mechanics.

**Technical consequence.** Every query needs hidden interpretation by type or prose.
Cycles, multiplicity, access inheritance and geometry disagreement cannot be
constrained uniformly. A popular container becomes a universal graph and lock hub.

#### B. Named relation families with independent invariants

**Player or World consequence.** Players can learn whether something is physically
carried, part of a Place, metrically inside a boundary or merely access-inherited.
Each action has explainable eligibility.

**Technical consequence.** More relation names exist, but each has a small schema,
cardinality, cycle, authorization, version and history rule. Operations touch only
the relevant relation coordinates.

**Recommendation.** Use conventional, typed relation families in the domain model.
Do not introduce a universal `contains` fact or infer one family from another unless
a concrete capability owns that derivation and its stale dependencies.

**Open questions.** May a semantic Place have more than one parent? Is physical
containment always a one-parent forest? Does access ever inherit through Place
containment, or only through explicit grants? Those choices have different hot-path
and abuse consequences.

## 5. Adjacency, routes and portals

**External fact.** OGC topology distinguishes `touches`, `within`, `contains`,
`overlaps` and other spatial relations. `ST_Touches` means boundaries intersect while
interiors do not; it does not mean traversable.
[PostGIS `ST_Touches`](https://postgis.net/docs/ST_Touches.html)

**External fact.** Recast/Detour represents ordinary neighboring navigation polygons
and separately represents a user-defined off-mesh connection with two endpoints and
a bidirectionality flag. Unreal's `NavLinkProxy` likewise connects navmesh areas that
have no direct navigation path and can dynamically enable or disable a smart link.
[Detour project-owned source](https://github.com/recastnavigation/recastnavigation/blob/main/Detour/Include/DetourNavMesh.h#L2089-L2124),
[Unreal `ANavLinkProxy`](https://dev.epicgames.com/documentation/en-us/unreal-engine/API/Runtime/AIModule/ANavLinkProxy)

**External fact.** OpenStreetMap represents routes as ordered member relations and
turn restrictions with explicit `from`, `via` and `to` roles. Geometric connection
alone is insufficient for directed or mode-specific traversal.
[OpenStreetMap data model](https://wiki.openstreetmap.org/wiki/Elements#Relation),
[turn restriction relation](https://wiki.openstreetmap.org/wiki/Relation:restriction)

### Relation alternatives

| Alternative | Player or World consequence first | Technical state, authority, transaction, contention and cost consequence |
| --- | --- | --- |
| Derive adjacency only from touching geometry | Redrawing a wall may open or close travel even when no door changed | Spatial re-evaluation can alter many edges; boundary precision becomes traversal authority |
| Store symmetric adjacency | “These Places border each other” is stable even with incomplete geometry | One unordered Place pair with one revision; mutations lock the pair, but adjacency still grants no traversal |
| Store directed traversal edge | A one-way stair, locked door or current can be explained directly | Edge has source, destination, operation-specific conditions/version and history; reverse travel needs another edge or bidirectional flag |
| Store a route as an ordered path of edges | Players can follow a named road or service that survives local geometry revisions | Route membership/order is separate state; changing one segment need not rename the route, but route queries and updates must be bounded |
| Store a portal/off-mesh connection | Distant or disconnected Places can be directly reachable | Explicit endpoints and direction avoid fake geometry; access, activation and freshness belong to the portal operation |

**Recommendation.** Separate qualitative adjacency from permitted traversal. Start
with explicit directed connections only when movement or remote reach needs them;
derive geometry contact only as a candidate or validation when a concrete rule says
so. Portal identity and endpoint roles should be stable and history-backed.

**Open questions.** Is the first connection a bare Place-to-Place exit, an Entity
door with endpoints, or a route segment? May Agents create connections, and what
structural standing prevents a User from attaching a private Place to any remote
Place? Does traversal consume one atomic movement Action or a later process?

## 6. Moving frames, vehicles and nested containers

**External fact.** Unreal attaches one Actor root to a parent Actor with explicit
translation, rotation and scale rules; attached components carry transforms relative
to their parent. Attachment itself is replicated separately from the child's local
transform.
[Unreal actor attachment](https://dev.epicgames.com/documentation/en-us/unreal-engine/BlueprintAPI/Transformation/AttachActorToActor),
[Unreal Actor/component hierarchy](https://dev.epicgames.com/documentation/en-us/unreal-engine/actors)

### Alternatives

#### A. Rewrite every contained subject when a carrier moves

**Player or World consequence.** Moving a Ship with ten thousand passengers and
items becomes one huge pause or a partially visible migration. Independent actions
aboard conflict with the Ship's global movement merely because every child's Place
or coordinate is rewritten.

**Technical consequence.** Work and lock count are proportional to contents;
transactions exceed bounded package size, Activity explodes and one hot carrier can
starve quiet subjects. Failure recovery must repair partial descendant movement.

#### B. Store a parent frame and local child placement

**Player or World consequence.** The Ship moves as one subject while its Cabin,
passengers and cargo remain stably aboard. A lamp stays attached to the mast without
being individually teleported each step.

**Technical consequence.** Carrier pose/frame changes touch the carrier coordinate;
children keep local placement. World position is derived through a bounded acyclic
chain. Attach/detach changes one child's parent atomically. External actions may
depend on carrier pose; actions wholly inside the Cabin need not.

#### C. Treat every carrier interior as an ordinary static Place and ignore movement

**Player or World consequence.** Interior play works, but players cannot reason about
where the vehicle is relative to external Places and cross-boundary effects become
arbitrary.

**Technical consequence.** It postpones frame composition but eventually needs an
explicit relation between interior Place and carrier/exterior. Retrofitting is safe
only if Place identity was never derived from a fixed coordinate or parent path.

**Recommendation.** Preserve a path to option B without building it early. A future
moving-frame model should enforce one immediate parent, no attachment/physical-
containment cycle, a small maximum chain depth and stable parent-lock order. It
should not copy derived ancestor Place rows as independent truth unless a measured
read requires a rebuildable projection.

**Hot-carrier implication.** One carrier pose can still be a hot fact, but it is one
honest serial fact rather than ten thousand forced child writes. Eligibility fan-out
to active observers remains real and must be coalesced; storage multiplicity and
delivery multiplicity are separate problems.

**Open questions.** Is a vehicle interior a Place role, a frame owned by an ordinary
Entity, or both? Can a Character act in the interior while the carrier is moving?
Which actions depend on local Cabin state only, and which must also validate the
carrier's external pose? What happens when a nested container crosses a portal?

## 7. Typed relations versus a generic graph

**External fact.** PostgreSQL recursive queries require explicit termination and
cycle handling; its `CYCLE` clause tracks visited keys because arbitrary recursive
graphs can otherwise loop indefinitely.
[PostgreSQL recursive queries and cycle detection](https://www.postgresql.org/docs/current/queries-with.html#QUERIES-WITH-CYCLE)

**External fact.** Google's Zanzibar authorization system uses relation tuples, but
relation semantics are defined by per-namespace configuration. Deep or wide pointer
chasing became expensive enough to require a specialized flattened indexing system.
This is evidence for typed, bounded semantics—not evidence that all World relations
belong in one authorization graph.
[Zanzibar paper, sections 2.1, 2.3 and 3.2.4](https://www.usenix.org/system/files/atc19-pang.pdf)

### Alternatives

#### A. User-authored generic edge `{subject, predicate, object}` drives mechanics

**Player or World consequence.** An Agent can invent `near`, `owns`, `inside` or
`triggers` and accidentally grant travel, observation or control. Different Agents
produce incompatible mechanics from synonymous text.

**Technical consequence.** World needs an ontology, inference engine or unsafe
string switch. Cardinality, direction, cycles, privacy and authorization vary by
predicate but cannot be enforced generically. Unbounded traversal becomes a denial
of service and a hot-node problem.

#### B. One generic storage table with a closed World-owned relation-kind union

**Player or World consequence.** Mixed relation packages can be represented
uniformly while only known kinds have mechanics.

**Technical consequence.** Shared storage may simplify history and package-local
references, but constraints become conditional on kind; hot indexes and wide tables
serve unrelated access patterns. It earns its cost only if several current kinds
need the same lifecycle and queries.

#### C. Relation-specific tables or roles behind one bounded operation vocabulary

**Player or World consequence.** `adjacent`, `traverses`, `attached`, `possesses` and
`may_control` each have one explainable rule. Agent-authored prose can describe them
but cannot create mechanics outside the typed operation.

**Technical consequence.** Each current relation gets exact uniqueness, direction,
cycle, access, version and indexes. A change package may carry a common internal
reference shape without requiring one universal table. Cross-kind queries are
explicit unions, not open traversal.

**Recommendation.** Start relation-specific (option C). Reconsider a closed generic
store only after multiple accepted relation kinds demonstrably share the same
identity, lifecycle, transaction and lookup shapes. Reject open predicates and any
recursive selector without strict depth, edge, result and time bounds.

## 8. Ownership, control, possession and access

The game term “ownership” is dangerously overloaded. A later grill should decide
each row separately:

| Structural fact | Concrete meaning | Candidate owner of truth | Mutation pressure |
| --- | --- | --- | --- |
| Introduction provenance | User U caused Entity E to enter shared history | immutable Activity/Entity provenance | append once, never transfer |
| Character control | User U may currently act through Character C | typed User–Character control state | hot on account/Character switch, not Place |
| Entity operation control | U or C may invoke one operation on E | operation-specific grant/capability | checked at commit with grant version |
| Possession | E is physically carried by C or inside container K | spatial placement/possession relation | changes with pickup, drop, theft, trade |
| Custody or stewardship | C is responsible for maintaining Place P | typed game relation if a mechanic needs it | may be shared or transferred |
| Access | C may enter, observe, use or modify a target | operation-specific authorization | read on every protected operation |
| Fictional claim | A deed says “Mara owns the mill” | Agent-authored Property/Trait/prose | no mechanical authority by itself |
| Lifecycle dependency | Child should retire with parent | explicit lifecycle relation | relevant only if retirement exists |

**External fact.** Zanzibar's model permits relations such as owner, editor and
viewer and composes them with union, intersection and exclusion. It also preserves
causal ordering so a newly protected content version is not checked against an older
ACL. [Zanzibar paper](https://www.usenix.org/system/files/atc19-pang.pdf)

**External fact.** OWASP classifies object-level authorization failure as a leading
API risk and states that every endpoint receiving an object id must check permission
for that object. UUIDs do not prevent object-id manipulation.
[OWASP API1:2023 Broken Object Level Authorization](https://owasp.org/API-Security/editions/2023/en/0xa1-broken-object-level-authorization/)

### Alternatives

#### A. One nullable `owner_user_id` on every Entity

**Player or World consequence.** Carrying, creating, controlling, stewarding and
fictionally owning collapse into one exclusive relation. Shared settlements, theft,
loan, abandonment and public access become exceptions to the field.

**Technical consequence.** One hot row determines unrelated permissions; transfer
changes every operation at once. It cannot express per-operation access or multiple
principals without sentinel values and flags.

#### B. Typed control/access relations plus separate possession

**Player or World consequence.** A borrowed key may be possessed without being
controlled; a public door may be used without owning it; a steward may edit a Place
without owning every item inside. Denials can be explained in game terms without
revealing private grant structure.

**Technical consequence.** Authorization evaluates a bounded relation set owned by
the concrete capability. Grant revisions join the operation's stale dependencies.
Possession changes stay spatial. Transfer or revocation writes one durable Activity
and current relation state atomically.

**Recommendation.** Do not add a universal ownership field. Retain current Character
control as its own structural relation. Future possession and access should be
typed, operation-specific and versioned; Agent-authored Properties, Traits and prose
remain non-authoritative claims.

**Open questions.** Which first mechanic requires control beyond the current User–
Character bound? Can control be shared, delegated, leased or abandoned? Does Place
access inherit to contents or occupants? Who may grant access, and can the grantor
grant only permissions it already holds? What neutral error hides existence without
making legitimate recovery impossible?

## 9. Action-specific locality, co-presence and reach

One universal “nearby” rule cannot serve all game actions. The spatial selector is
part of the operation contract:

| Action family | Player-visible locality rule | World-owned structural checks | Explicit exclusions |
| --- | --- | --- | --- |
| Orient at current Place | list a bounded page of eligible current subjects | current Character placement, exact Place, access lens | geometry, remote subjects, unbounded occupancy scan |
| Direct local Interaction | actor and explicit targets are co-present under the chosen presence rule | actor control, current placements/parents, target ids and versions | bystanders, inferred responses, names as identity |
| Move through connection | source is current; one directed edge/portal admits the destination | placement, connection state, access and expected versions | geometric touching alone, prose-derived exits |
| Pick up or put down | subject and destination/container satisfy one possession operation | placement parent, control/access, capacity only if explicit | “owner” Property, implicit theft permission |
| Remote trigger | explicit non-spatial link or capability connects actor action to target | link identity/version, control and declared consequence | distance or adjacency as fabricated causality |
| Bounded cross-Place occurrence | exact Place set or bounded structural reach is admissible | relation/geometry versions, maximum depth/results, operation standing | World-inferred blast/hearing meaning, mass Entity mutation |
| Metric effect | submitted geometry intersects or lies within allowed scope | same frame/SRID, geometry validity/version, exact predicate | bounding box alone as final truth |
| Public local observation | active Character is structurally eligible for public occurrence | audience class, event-time/current presence choice, access/privacy | private Interaction, personal-memory claim |
| Private or targeted observation | only named participants or authorized group can retrieve | object-level access before any payload or hint | same-Place bystanders and unauthorized subscriptions |
| Later Place history | arriving Character may read bounded public local history | current access lens, stable Place id, immutable occurrence footprint | claim of having personally witnessed it |

**Recommendation.** Every action schema should name exact ids or a bounded, World-
evaluated structural selector. Every selector declares maximum depth, examined
edges, matched subjects/Places and result size. Geometry, relationship traversal and
access are deterministic filters only after a concrete capability chooses them.

**Open question.** For co-presence inside nested or moving frames, is equality of the
immediate Place sufficient, or do selected ancestors count? The answer affects
privacy and hotspot size and must be operation-specific.

## 10. Observability, interest and Agent-authored meaning

**External fact.** Unreal's authoritative server decides gameplay state while
clients render approximations. Its relevancy system chooses a per-connection set;
ownership, attachment, visibility and distance are different relevancy inputs.
Replication Graph shares persistent candidate lists because per-Actor/per-connection
evaluation becomes a CPU bottleneck.
[Unreal networking overview](https://dev.epicgames.com/documentation/en-us/unreal-engine/networking-overview-for-unreal-engine),
[Actor relevancy](https://dev.epicgames.com/documentation/en-us/unreal-engine/actor-relevancy-in-unreal-engine),
[Replication Graph](https://dev.epicgames.com/documentation/en-us/unreal-engine/replication-graph-in-unreal-engine)

**External fact.** Roblox warns that every additional streaming focus increases
server work; one player with nine moving foci may cost approximately ten players'
streaming work. Interest is therefore a bounded workload input, not a free list.
[Roblox instance streaming](https://create.roblox.com/docs/workspace/streaming#replication-focus)

### Ownership split

| Layer | Owns | Must never own |
| --- | --- | --- |
| Concrete operation | public/private audience class and structural scope kind | prose-derived hearing, ownership or physics |
| World | identity, placement, explicit relations, access, versions, bounds and eligibility | semantic interpretation of arbitrary text |
| Activity | one accepted occurrence and stable involved roles | per-recipient copies or delivery status |
| Host | bounded interests, subscription connection, coalescible hints and temporary context | World truth, personal memory or authorization |
| Agent | meaning, natural presentation and any explicitly requested proposal | eligibility to receive secrets or mechanical truth |
| User | explicit Agent invocation and confirmation | hidden server-triggered token spend |

### Alternatives

#### A. Send everything local and let each Agent decide relevance

**Player or World consequence.** A private whisper or hidden object is already leaked
before the Agent “decides” not to mention it. Different Agents create inconsistent
access and observation rules.

**Technical consequence.** Fan-out follows full occupancy; privacy cannot be repaired
after delivery. Malicious prompts can exfiltrate content. World loses deterministic
eligibility.

#### B. World decides eligibility; Agent decides presentation after authorization

**Player or World consequence.** Eligible Characters receive grounded context;
Agents may describe sight, sound or inference naturally without gaining permission
to see hidden content. Later public history remains distinct from personal witness.

**Technical consequence.** World filters by operation-specific audience, spatial
facts and access before content or a resource-specific hint leaves authority. Host
interests remain bounded and disposable. No sensory ontology is required for the
first exact-Place case.

**Recommendation.** Retain option B. An Agent may author meaning such as “this sound
is noticeable to hearing Characters,” but World must first bound the spatial and
privacy scope. Any Character-state interpretation by a receiving Agent creates no
mechanic, stored perception or authority.

**Open questions.** Is observation evaluated at occurrence time, read time or both?
Which public occurrences require lossless active catch-up versus current state plus
bounded recent history? Which spatial resources may one host follow simultaneously,
and what happens when that bound is exceeded?

## 11. Multiplayer conflict, atomicity and hot subjects

### External evidence

**External fact.** PostgreSQL row locks block writers/lockers on the same row but not
ordinary readers. Deadlocks can arise even from row locks; the manual recommends
acquiring multiple locks in a consistent order. Without a timeout, a lock waiter can
wait indefinitely.
[PostgreSQL explicit locking](https://www.postgresql.org/docs/current/explicit-locking.html)

**External fact.** PostgreSQL Serializable may abort a transaction with a
serialization failure; the application must retry the whole transaction. This can
avoid some explicit locks but does not remove contention or retry cost.
[PostgreSQL transaction isolation](https://www.postgresql.org/docs/current/transaction-iso.html)

**External fact.** FoundationDB tracks exact read and write conflict ranges and
rejects intersecting transactions. It warns that frequently modified hot keys remain
inefficient and that a watch says only that a value changed, not which value a later
read will see. [FoundationDB developer guide](https://apple.github.io/foundationdb/developer-guide.html#conflict-ranges)

### Candidate dependency coordinates

A future spatial action may depend on a bounded subset of:

- acting Character control and current placement;
- one subject's immediate parent placement and placement version;
- expected presence or absence of a typed relation edge;
- one portal/route edge and its enabled/access version;
- one geometry/frame version and a named predicate;
- exact affected Property or Trait facts;
- one control/access grant version;
- an operation-specific capacity or admission slot, only if the game mechanic owns
  one; and
- exact ids of every state-changing subject in the atomic package.

It should not automatically depend on all Activity, occupants, geometry or relations
at the containing Place.

### Spatial transaction alternatives

| Alternative | Player or World consequence first | Technical state, transaction, contention and cost consequence |
| --- | --- | --- |
| Whole-Place revision/lock | Any unrelated local change can stale or delay the player's action | simple snapshot semantics, but one hot Place becomes one mutation lane and waiting consumes shared capacity |
| Whole-Entity revision/lock | Different facts on one hot Entity conflict even when gameplay says they compose | smaller than Place but still false contention; easy stable ordering for bounded multi-Entity packages |
| Exact typed fact/edge/placement coordinates | Unrelated same-Place work and independent facts may succeed; true shared facts conflict clearly | more dependency coordinates and absence handling; requires one total lock/conflict order and precise errors |
| Serializable without explicit coordinates | Players see a serial result but may receive retries from wider read dependencies | simpler application locking in some cases, but abort storms and predicate conflicts need real skewed tests |
| Last-write-wins | Players' accepted actions silently erase each other | cheap apparent throughput but invalid durable causality; incompatible with explainable shared history |

**Recommendation.** Continue the current research direction toward operation-scoped
typed dependencies. Movement should atomically replace one immediate parent and
write one Activity; lock old parent, new parent, subject and required connection/
access coordinates in one stable total order. Geometry changes should conflict on
the affected geometry version, not every subject currently in the Place.

**Recommendation.** One compact occurrence reach may authorize observation over many
Places, but it must not disguise millions of literal Entity mutations as one atomic
write. A bounded state-changing package names every changed subject; a wide
contextual effect remains a separate current scoped fact whose read semantics are
explicit.

**Hot-subject truth.** Contradictory writes to one placement slot, portal state,
geometry version, access grant or Entity fact have one honest serialization point.
“Scale” means bounded admission, wait, retry and failure without starving quiet
subjects—not simultaneous acceptance of incompatible outcomes.

**Open questions.** Which dependency granularity earns the first production slice?
How is expected absence represented for placement and relation edges? What is the
maximum atomic subject/edge count? Which busy outcome is fair per User, Character,
Entity, Place or operation without adding a score?

## 12. Idempotency and retry identity

**External fact.** HTTP defines idempotency as repeated identical requests having the
same intended effect and warns that non-idempotent requests should not be retried
automatically without another way to know they are safe.
[RFC 9110 section 9.2.2](https://www.rfc-editor.org/rfc/rfc9110.html#name-idempotent-methods)

**Recommendation.** Every spatial mutation should carry one stable request identity
and a fingerprint of the complete normalized semantic input: actor context,
expected placement/relation/geometry/access versions, targets, transitions and
prose where prose is stored. Equal replay returns the original canonical result;
changed content under the same id conflicts. A rejected/busy attempt creates no
accepted Activity and must not permanently consume the id unless the contract
explicitly records rejected attempts—which current Aicadia does not.

**Movement edge cases requiring proof:** lost response after commit; duplicate enter
through one portal; move A→B racing B→C; carrier movement racing detach; access
revocation racing traversal; retry after geometry or relation revision; and a forced
failure after history insertion but before placement update.

## 13. Durable spatial and authority history

### Required historical questions

Every accepted state-changing spatial action must make it possible to establish:

- who acted and through which controlled Character;
- which stable Entities and Places participated and in which explicit roles;
- the direct placement before and after a movement, pickup, drop, attach or detach;
- which connection, portal, containment, geometry and access versions authorized
  the action when those facts mattered;
- where the occurrence happened in semantic Place terms;
- which exact geometry/frame version grounded a metric claim, if any;
- which control, access or possession relation changed;
- what was accepted at World time; and
- which causal Activity was explicitly named, without inferring causality later.

### Alternatives

#### A. Infer past location and authority from current rows

**Player or World consequence.** After a Ship moves or a door grant changes, old
stories appear to have happened in the new location or under the new permission.
Disputes cannot be resolved from shared history.

**Technical consequence.** History queries join mutable current state and silently
rewrite the past. No transaction can prove which versions were checked.

#### B. Store operation-specific historical footprints with current state

**Player or World consequence.** A past voyage, theft, construction or public event
continues to name the correct people, Places, objects and structural basis after the
World changes.

**Technical consequence.** Each operation writes its minimal normalized roles and
before/after/version references in the same transaction as current state. This is
not event sourcing: current state remains authoritative and history is not replayed
to rebuild it.

#### C. Store full continuous trajectories for every subject

**Player or World consequence.** Arbitrary event-time position can be queried, but
the game claims continuous motion even for dormant or discretely placed subjects.

**Technical consequence.** Temporal geometry, interpolation, indexing and retention
become a large subsystem. MobilityDB demonstrates that moving-point trajectories are
specialized spatiotemporal types built over PostgreSQL/PostGIS, not a free extension
of one location column.
[MobilityDB introduction](https://docs.mobilitydb.com/MobilityDB/develop/ch01.html)

**Recommendation.** Choose B per concrete action. Add non-overlapping placement
intervals only if event-time co-presence becomes a current mechanic. PostgreSQL range
types plus exclusion constraints can enforce non-overlap for one subject, but such a
table is not earned merely for possible future replay.
[PostgreSQL range exclusion constraints](https://www.postgresql.org/docs/current/rangetypes.html#RANGETYPES-CONSTRAINT)

**Open questions.** Does public occurrence eligibility use placement at acceptance
time? Must active-attention history prove continuous presence, or is host attention
deliberately non-durable? When Place geometry changes, should history expose the old
shape, only its version id, or a stable rendered snapshot?

## 14. Sparse, potentially unbounded World

### External evidence

**External fact.** H3 is a hierarchical global grid but explicitly distinguishes
exact logical hierarchy from only approximate geometric containment between
resolutions. Its own guidance recommends a precise point-in-polygon recheck when
exact boundaries matter.
[H3 introduction](https://h3geo.org/docs/),
[H3 indexing](https://h3geo.org/docs/highlights/indexing/)

**External fact.** PostGIS spatial predicates use a bounding-box prefilter through a
GiST spatial index and then apply the exact predicate. The index is a candidate
accelerator, not the final semantic answer.
[PostGIS spatial indexes](https://postgis.net/documentation/faq/spatial-indexes/),
[PostGIS spatial queries](https://postgis.net/docs/using_postgis_query.html#using-postgis-query-indexes)

### Alternatives

| Alternative | Player or World consequence first | Technical state, transaction, contention and cost consequence |
| --- | --- | --- |
| Precreate a global cell lattice as World Places | Empty space appears canonically known and every position belongs to a server-made Place | unbounded rows or implicit hidden geography; cells leak operational precision into gameplay |
| Create only established subjects and relations | Unknown space remains genuinely absent until play establishes it | storage and simulation follow touched state; queries need explicit frontier/connection rules rather than scanning emptiness |
| Use H3/S2/grid cell as Place identity | Reindexing or resolution change renames Places and approximate hierarchy becomes lore | fast bucketing, but cell hierarchy/boundaries become irreversible public contract |
| Use replaceable cells/bounding boxes as derived index | Players keep stable Place identity while implementations change | derived mapping can be rebuilt; exact World predicate rechecks stable geometry/relation state |

**Recommendation.** Keep the World sparse and establish only played state. Any grid,
geohash, bounding box, shard or routing cell must be derived, many-to-many capable,
replaceable and absent from player-facing identity and Activity. A geometry crossing
several cells stays one Place; a hot Place may use several operational pieces
without becoming several semantic Places.

## 15. Indexing and partitioning without making cells canon

**External fact.** PostgreSQL partitioning can improve pruning and maintenance only
for suitable access patterns. Updating a partition key moves a row as delete+insert;
unique/primary constraints on a partitioned table must include the partition key,
and too many partitions increase planning cost.
[PostgreSQL partitioning](https://www.postgresql.org/docs/current/ddl-partitioning.html)

### Candidate index ownership

| Query | Canonical fact | Replaceable access path | Bound that must be proved |
| --- | --- | --- | --- |
| list current occupants | direct/derived placement | `(place_id, subject_id)` B-tree or materialized projection | work near page size, not total occupancy |
| find geometry candidates | current geometry version | GiST/SP-GiST bounding boxes or cell cover | exact predicate recheck; bounded candidate amplification |
| traverse connections | typed directed edge | `(source_place_id, kind, target_id)` index | depth, fan-out, results and time |
| find contents | immediate parent placement | `(parent_id, child_id)` index | page bound independent of total descendants |
| authorize operation | typed grant/control edge | subject/object/relation lookup | one consistent snapshot and bounded traversal |
| read Place history | immutable Activity footprint | `(place_id, occurred_at, activity_id)` keyset index | no full hot-Place scan or unbounded hydration |
| route dirty interests | stable Entity/Place resource id | transient subject map or broker route | per-host interest cap and coalescing |

**Recommendation.** Partition only after a measured query, vacuum, retention or
write-distribution problem earns it. Prefer stable hash or time dimensions whose
uniqueness/history implications are explicit. Movement across operational partitions
must preserve subject id, authorization, one Activity and atomic current placement;
the player never observes the move as a semantic Place transition.

**Open question.** Can one physical database transaction still cover the first
accepted cross-partition movement/relationship package? If not, the game contract
must choose a different atomic boundary before sharding; infrastructure cannot
silently weaken it.

## 16. Privacy, abuse and adversarial spatial cases

| Attack or failure | Player harm | Required structural defense | Scale/hot-subject concern |
| --- | --- | --- | --- |
| Guess a UUID for a remote/private Entity | secret existence or state leaks | object-level authorization on every id dereference; neutral errors | bulk probing must be rate/admission bounded |
| Subscribe to a private Place or moving Character | location tracking without content reads | authorize interest registration and every refetch; no unauthorized hint | subscriptions themselves can become a presence oracle |
| Infer a whisper from a content-free Place hint | bystander learns private activity occurred | private change dirties only authorized personal/target resources | global or Place-wide invalidation leaks traffic shape |
| Use prose/Property `owner_user_id` or `near=true` | Agent-authored text grants control or reach | typed World fields only; text remains non-executable | no denylist or ontology scan on hot content |
| Add containment cycle or million-edge chain | nontermination and resource exhaustion | cycle rule per relation kind; depth/edge/result/time caps | hot hub traversal cannot scan all descendants |
| Submit invalid or million-vertex geometry | CPU/memory exhaustion or incorrect predicates | type, byte, component, vertex, validity, frame and dimension bounds before query | bounding-box false hits and exact recheck must be measured |
| Draw huge geometry covering private Places | unauthorized observation or mass invalidation | operation standing plus access-aware bounded scope; coverage is not authority | one geometry cannot dirty every occupant row |
| Race access revocation and content mutation | old rights apply to new state | grant version in transaction dependency or causally consistent check | hot ACL/grant remains a real serial fact |
| Move a Character while local Action commits | remote or double-place action succeeds | actor placement version and atomic move/action ordering | Place-wide lock is unnecessary if exact slot is coordinated |
| Move carrier with active interior play | every cabin action falsely conflicts or observes wrong exterior | separate local frame revision from external carrier pose dependency | one hot carrier fact, not descendant rewrites |
| Spam one portal or Place entry | quiet World work starves | subject-scoped admission, lock/statement/pool bounds, clear busy outcome | waiting requests may exhaust DB connections before locks resolve |
| Sybil Users claim communal control | traffic or count manufactures authority | explicit eligibility and bounded admitted participants | no score, listener count or global tally hot row |
| Repartition operational cells | Place appears to move or history splits | cells absent from identity/history; rebuildable mapping | migration must retain quiet-subject availability |

**External fact.** PostgreSQL `lock_timeout` can bound each lock acquisition, while
`statement_timeout` bounds statement execution. Connection and per-query memory
settings are finite resources; increasing connections or `work_mem` multiplies
resource use across sessions/workers.
[PostgreSQL client timeouts](https://www.postgresql.org/docs/current/runtime-config-client.html),
[connection settings](https://www.postgresql.org/docs/current/runtime-config-connection.html),
[resource consumption](https://www.postgresql.org/docs/current/runtime-config-resource.html)

**Recommendation.** Reject unauthorized spatial requests before expensive geometry
or graph work wherever the ordering does not create an existence oracle. For
authorized operations, validate cheap syntax/cardinality first, then resolve ids and
access neutrally, then acquire exact dependencies in stable order, then perform the
bounded predicate and commit state plus history.

## 17. Current multiplayer scenario matrix

This matrix does not choose outcomes. It states what a spatial foundation must make
decidable for every fixed scenario in the
[current catalogue](../../areas/multiplayer/scenarios.md).

| Scenario | Spatial/relational facts required | Critical authority and transaction question | Extreme failure to test |
| --- | --- | --- | --- |
| S01 one Agent changes one Entity | stable T and H ids; explicit placement; optional initial relations | who may place T in H, and which exact facts make later changes stale? | no partial Entity/placement/state/history bundle |
| S02 thousands change one table | same Entity, same/different fact coordinates, current Place as context only | which writes compose; which control/access grants are required? | one hot fact bounded while quiet Q commits |
| S03 remote button and bomb | explicit non-spatial trigger link; B and X placements | what grants remote authority, and is the remote consequence one atomic package? | concurrent link retarget or bomb move cannot misfire |
| S04 bomb inside house | containment, inside/outside Places, adjacency, optional geometry/reach | who may declare scope, which subjects change atomically, who may observe? | no Place enumeration or occupant fan-out in mutation transaction |
| S05 music bomb | occurrence Place/reach, Character presence and privacy | World bounds scope; Agent interprets hearing only after authorization | no token wakeup, no secret leak, no per-listener durable row |
| S06 explosion and table/window | stable T/W/H/Y ids; placement, window connection and exact dependencies | which disjoint changes survive movement; which causal facts stale the package? | no double placement; stable lock order over all subjects/edges |
| S07 “same” table | stable identity independent of name and location | who may establish shared materialization identity? | no semantic dedupe, global allocator or Place lock |
| S08 linked Entity graph | package-local refs, placement and closed typed edges | bounded atomic graph size, relation cycles and authority per edge | no orphan/half graph after one invalid link |
| S09 absent/stale/moved target | expected identity/absence and placement version | which neutral error supports refetch without leaking remote existence? | invalid mixed package leaves no current or Activity rows |
| S10 everything blue | stable scoped context distinct from literal per-Entity Property | what structural scope is admissible and how does it compose with local state? | no global revision, fake instant million-row rewrite or mass hint |
| S11 hot Place | occupancy index, per-subject dependencies, bounded interests | where does admission happen and how are quiet subjects isolated? | lock/pool/router pressure on H cannot starve quiet Places |
| S12 disconnect and catch up | stable Place/history footprint and current placement | what bounded history remains available, and what gap is honest? | lost/duplicate hints never change truth or personal memory |
| S13 communal result | explicit eligible subjects, target relation and settlement scope | what typed authority admits a bounded participant set and final package? | no listener-count right, Sybil tally or alternate mutation path |
| S14 causal loop | explicit typed links and bounded declared package/actions | are cycles invalid for this relation or inert until another explicit Action? | no recursive server cascade, duplicate mutation or token spend |

## 18. Additional spatial stress scenarios

These scenarios supplement S01–S14 for later labs; they do not enter the authoritative
catalogue through this report.

### X01 — One moving carrier with a hot interior

Ship S contains Cabin P; 100,000 conceptual occupants/items are attached or placed
inside. S changes external pose while two interior Actions and one detach race.

Required observation: a carrier pose change performs a bounded number of canonical
current/history writes independent of descendant count; interior-only Actions do not
conflict unless their declared dependency needs external pose; detach has exactly one
serial result; no child is ever in two immediate parents.

### X02 — Exact boundary and 2D/3D disagreement

Character C lies on a Room polygon boundary; another is above the Room in Z but has
the same XY. Exercise `contains` versus `covers`, 2D versus 3D, geometry-version
change and invalid self-intersection.

Required observation: the chosen operation names one predicate and dimension;
invalid or mismatched-frame input fails before mutation; no fallback silently changes
the answer.

### X03 — Portal access revocation race

C prepares traversal through Portal D from A to B. The access grant or D's target is
changed before commit, then the original response is lost and retried.

Required observation: stale traversal cannot reach the old or unauthorized target;
equal accepted replay returns one movement/Activity; neutral rejection leaks no
additional Place data.

### X04 — Deep or cyclic containment attack

An Agent submits the maximum legal nested container chain, then one over-limit chain
and one cycle. A separate legal portal graph contains a cycle.

Required observation: physical containment rejects cycle/over-depth atomically;
route traversal permits a legal cycle but never revisits indefinitely and respects
edge/result/time bounds.

### X05 — Private Place inside a public geometry

A public effect geometry overlaps a private interior Place and a public Yard. An
unauthorized host watches the public parent.

Required observation: spatial intersection does not override access; hints and
history contain only authorized resource identities/content; aggregate counts do not
reveal the private Place.

### X06 — Operational repartition while active

One semantic Place and its geometry/index pieces move between operational partitions
while movement, local reads and history pagination continue.

Required observation: stable ids, cursor contract, authorization and accepted
history are unchanged; no partial double ownership or player-visible cell appears.

## 19. Extreme hot-Place and hot-Entity evidence programme

“Millions” needs separate semantic, data-shape and production-capacity evidence.
The following ordered programme prevents a small lab from borrowing a scale claim.

### Gate 1 — deterministic semantic model

- one hot Place, one hot Entity, one quiet Entity and one quiet Place;
- exact accepted/conflict/busy/rejected outcomes for every relevant interleaving;
- movement, parent change, portal/access revocation and retry state machines;
- one Activity per newly accepted mutation and no write on rejection;
- relation cycles and bounds; and
- every query/operation declares the maximum subjects, edges and geometry complexity.

Claim allowed: only the fixed semantics are representable.

### Gate 2 — million-row sparse and dense database shapes

- `P_hot` with at least 1,000,000 current occupancy/placement rows;
- at least 1,000,000 sparse quiet Places or a storage-equivalent distribution;
- bounded pages of 1, 10 and 100 from hot and quiet controls;
- exact-local Activity history with a long hot tail and privacy filtering;
- geometry candidates whose bounding boxes create deliberately high false-hit rates;
- one large Place geometry subdivided only as an index representation; and
- one moving carrier with at least 100,000 descendants but O(1)-sized carrier move
  state transition.

Measure query plans, rows examined, buffers, CPU, memory, I/O, WAL and result size.
Claim allowed: only those data shapes and queries on the tested database build.

### Gate 3 — controlled concurrency and hotspot isolation

- many distinct Entities at one Place, many independent facts on one Entity and many
  contradictory writes to one exact fact;
- concurrent moves into/out of one Place and through one portal;
- carrier pose versus interior work;
- access/grant changes racing protected reads and mutations;
- fixed pool, transaction, statement and lock bounds;
- admission before scarce DB resources are occupied;
- quiet-control traffic throughout the hot burst; and
- retries capped with jitter/backoff outside the authoritative transaction.

Required result: quiet controls remain within a chosen service objective; every hot
request terminates as accepted, conflicted, busy or rejected; no indefinite wait,
duplicate Activity, partial spatial state or retry amplification is hidden.

Claim allowed: only the tested rate, skew, burst, hardware, deployment and percentile.

### Gate 4 — interest, privacy and recovery

- 1,000,000 conceptual residents but separately specified active hosts;
- bounded interests per host over exact Entity, immediate Place and permitted reach;
- duplicate, delayed, reordered and completely lost hints;
- slow consumer coalescing, disconnect and baseline/refetch recovery;
- public occurrence, participant-only Interaction and private Place controls; and
- no Agent call in the delivery fixture unless a separately bounded direct smoke is
  explicitly run.

Required result: mutation work is not proportional to all residents; delivery cost
is honest about active recipients; unauthorized hosts receive neither content nor
existence-revealing hints; recovery returns authoritative bounded state/history.

### Gate 5 — operational failure and repartition

- database restart/failover, gateway loss and stale router mapping;
- index/projection rebuild from canonical state;
- operational cell split/merge with the semantic Place kept intact;
- hot-subject admission during partial outage; and
- verified cleanup and restoration objectives.

Claim allowed: only the exercised recovery objective. A million-User product claim
still requires an explicit connected-User count, read/write mix, geographic
distribution, active-interest ratio, latency percentiles, allowed busy rate, burst
duration, history horizon and recovery objective.

## 20. KISS evolution path that does not block a complete future

Each step is contingent on an accepted player behavior and its own plan.

### Step 0 — preserve the current identity seam

Keep Place as an Entity role, one stable id per subject, optional exact current Place
and immutable Activity. Remove no future option by deriving neither identity nor
history from coordinates, parent paths or cells.

### Step 1 — add one explicit connected-Place movement slice

Candidate only: two or more stable Places, one directed connection and one atomic
Character movement. Prove current placement uniqueness, source/destination/access
freshness, retry, Activity, bounded reads and hot-entry isolation. Do not add
geometry, generic relations or routes if the slice does not need them.

### Step 2 — add typed immediate placement/containment for one mechanic

Candidate only: pickup/drop or one container/interior. Establish one-parent and
cycle/depth rules, distinct possession/control semantics and exact history. Reuse
the same placement transition contract rather than create a second location truth.

### Step 3 — add operation-specific locality and interest over established structure

Candidate only: exact Place plus one bounded adjacency/connection reach. World
authorizes; host interests are capped and disposable; Agent presentation remains
semantic. Prove privacy, loss, coalescing and refetch.

### Step 4 — add geometry only for a mechanic that cannot be expressed structurally

Candidate only: one 2D or 3D predicate with explicit frame, units, dimension,
boundary semantics, geometry type/size/vertex bounds, version and GiST exact recheck.
Geometry does not replace Place presence, traversal or access.

### Step 5 — add moving frames when a playable carrier earns them

Candidate only: one carrier, one interior Place/frame and attach/detach. Derive
descendant world position through bounded acyclic transforms; do not rewrite every
child on carrier movement.

### Step 6 — broaden typed relations and authority one mechanic at a time

Candidate only: stewardship, delegated control, shared access or a route. Each kind
earns its own cardinality, inheritance, cycle, transfer, privacy, history and hot-node
rules. Consider shared storage only after repeated concrete shapes justify it.

### Step 7 — add operational indexing, partitioning or routing only from evidence

Derived spatial covers, partitions, projections and transient routers remain
replaceable. Repartitioning never changes stable identity, player knowledge or
Activity. A broker, specialized graph index or trajectory extension must retire a
measured risk that simpler PostgreSQL/PostGIS cannot.

**Recommendation.** This path is complete-future-safe because every step adds one
orthogonal fact keyed by stable Entity/Place identity. It is KISS because no step
introduces a universal ontology, geometry, graph, ownership model, event system or
cell lattice before a current mechanic needs it.

## 21. Dependency-ordered design tree for later grilling

The tree orders decisions; it does not answer them. A branch may be pruned only by
an explicit User choice recorded in the owning concept/log and later accepted in
`game/docs/` where it changes current behavior.

```text
R0  Choose the first valuable spatial multiplayer behavior
│   Facts: current exact-Place contract; S01–S14; hot-Place lane; deferred scope
│
├─ R1  Identity and lifecycle vocabulary [prerequisite for every branch]
│  ├─ R1a fragment/sameness: distinct ids forever or explicit later identity link?
│  └─ R1b retirement/split/merge: absent, or which stable historical identity remains?
│     Facts: current One Subject, One Identity; Kubernetes name/UID separation; S07
│
├─ R2  Immediate spatial parent [depends on R1]
│  ├─ R2a current direct Place only
│  └─ R2b one immediate Place/container/frame parent with derived ancestors
│     ├─ R2b1 semantic containment tree
│     └─ R2b2 containment DAG or multiple simultaneous contexts
│     Facts: current placement; S04/S06/S09; X01/X04; moving-frame attachment evidence
│
├─ R3  First connectedness/traversal mechanic [depends on R1; movement also on R2]
│  ├─ R3a directed Place connection
│  ├─ R3b Entity door/portal with source and destination roles
│  └─ R3c ordered route of typed edges
│     For each: direction, access, activation, version, discovery, creator authority
│     Facts: Recast off-mesh links; OSM routes/restrictions; S03/S06/S08/S14; X03
│
├─ R4  Metric geometry need [depends on chosen mechanic, not prerequisite for R3]
│  ├─ R4a no geometry for this slice
│  ├─ R4b one World Cartesian frame
│  ├─ R4c planetary/geodetic frame
│  └─ R4d stable local/moving frames with explicit transforms
│     Then choose: 2D/3D, units/SRID, type, boundary predicate, validity/size bounds
│     Facts: OGC qualitative vs quantitative split; PostGIS SRS/predicate limits;
│            S04/S05/S10; X02/X05
│
├─ R5  Relation storage seam [depends on first accepted relation kinds]
│  ├─ R5a relation-specific roles/tables
│  └─ R5b closed generic relation-kind store
│     Never branch: open Agent predicate as mechanic
│     Facts: PostgreSQL recursion/cycles; Zanzibar deep/wide traversal cost; S08/S14
│
├─ R6  Authority vocabulary [depends on R1 and exact operation]
│  ├─ R6a control remains only User→Character
│  ├─ R6b operation-specific Entity/Place grants
│  ├─ R6c possession separate from control
│  └─ R6d shared/delegated/communal authority
│     For each: grantor, scope, transfer/revocation, inheritance, history, neutral error
│     Facts: current Character control; Zanzibar consistency; OWASP BOLA; S03/S04/S13
│
├─ R7  Action-locality grammar [depends on R2/R3; on R4 only for metric actions;
│  │                            on R6 for protected actions]
│  ├─ R7a exact immediate Place
│  ├─ R7b selected bounded ancestor/containment context
│  ├─ R7c explicit edge/route/portal
│  ├─ R7d exact Place set
│  └─ R7e bounded structural or geometry reach
│     Facts: S03–S06/S09/S10; OGC/PostGIS predicates; User's compact-reach direction
│
├─ R8  Observation and interest [depends on R7 and R6 privacy]
│  ├─ R8a occurrence-time eligibility
│  ├─ R8b read-time eligibility
│  └─ R8c both, with their distinction exposed safely
│     Then choose: lossless active catch-up or current state + bounded recent history;
│                  per-host interest cap; exact Entity/Place/reach resources
│     Facts: S04/S05/S11/S12; observation lab; Unreal/Roblox interest evidence;
│            Kubernetes watch recovery
│
├─ R9  Concurrency coordinates and atomic boundary [depends on every state choice above]
│  ├─ R9a conservative Entity coordination
│  ├─ R9b exact typed fact/edge/placement coordination
│  └─ R9c broader Serializable experiment as challenger
│     Then choose: expected absence, stable lock order, package size, busy/admission,
│                  cross-partition rule, idempotency fingerprint
│     Facts: labs 02–04; PostgreSQL locks/isolation; FoundationDB conflict ranges;
│            S02/S06/S09/S11; X01/X03
│
├─ R10 Durable history footprint [depends on R2–R9 transitions]
│  ├─ R10a operation-specific before/after + version references
│  ├─ R10b non-overlapping placement intervals when event-time presence is required
│  └─ R10c continuous trajectory only when continuous movement is gameplay
│     Facts: current Activity rule; PostgreSQL range constraints; MobilityDB scope;
│            S04/S06/S12; X01/X02
│
├─ R11 Query/index design [depends on accepted queries and data shape]
│  ├─ R11a B-tree current placement/edge/history paths
│  ├─ R11b GiST geometry candidate + exact recheck
│  ├─ R11c rebuildable cell cover/projection
│  └─ R11d partition/shard only after measured threshold
│     Facts: PostGIS indexing/subdivision; H3 approximation; PostgreSQL partitioning;
│            S11; Gate 2–5 evidence
│
└─ R12 Public capability and Agent wording [last, depends on chosen semantics]
   ├─ exact operation names and bounded schemas
   ├─ World-owned structural errors without privacy leakage
   ├─ complete HTTP/MCP parity
   └─ Agent narration that hides implementation structure but preserves game truth
      Facts: Aicadia parity/public-text rules; every accepted branch above
```

### Suggested grill order

1. Pick R0 as one concrete scene, not “the spatial system.”
2. Confirm only the R1 identity assumptions that scene needs.
3. Decide R2/R3 player consequences before schema terms.
4. Ask R4 only if the scene genuinely needs metric shape or distance.
5. Decide R6 authority before R7 locality, because reachable does not mean allowed.
6. Decide R7 before R8; World cannot authorize observation without a chosen scope.
7. Derive R9 and R10 from the exact accepted transition, never in isolation.
8. Measure before R11; choose R12 only after semantics are stable.

## 22. Cross-cutting candidate invariants

These are research recommendations for later acceptance or rejection:

1. One durable World subject has one stable Entity id; Place, Character or future
   frame roles do not add another identity for that subject.
2. Names, geometry, parent, location, relation, control and operational cell are
   mutable facts, never identity.
3. A spatially placeable subject has at most one immediate current parent under the
   chosen placement family; absence is explicit and valid.
4. Physical containment/attachment is acyclic and depth-bounded. Route/portal graphs
   may cycle only with bounded traversal and visited-edge protection.
5. Semantic containment, metric containment, traversal, possession, control, access
   and lifecycle dependency are different relation kinds.
6. Geometry is optional, frame-bound, dimension-explicit, valid, versioned and
   complexity-bounded. Bounding boxes/cells produce candidates, never final truth.
7. Every action owns one explicit locality rule and one bounded selector grammar.
   No prose, Property key/value, Trait or name grants reach or authority.
8. World determines structural eligibility and privacy before content or hints leave
   authority; Agent interpretation cannot reveal an unauthorized fact.
9. Control, possession, stewardship and access are independent; introduction
   provenance never silently becomes transferable ownership.
10. Every accepted spatial/control mutation updates current state and one durable,
    queryable Activity footprint atomically; current state is not replay-derived.
11. Every mutation declares exact relevant versions/expected absence and one stable
    request identity; equal replay cannot duplicate state or history.
12. Multi-subject locks/conflict coordinates use a stable total order and a bounded
    package. One wide occurrence scope cannot hide unbounded literal mutations.
13. One hot fact has bounded serial admission; a hot Place or Entity cannot consume
    every connection, queue or lock needed by quiet subjects.
14. Interest and delivery state are bounded, disposable and recoverable through
    authorized reads. They are not World truth, Activity or Agent knowledge.
15. Operational partitions, cells, indexes and owners are invisible and replaceable;
    repartitioning never changes game identity, locality or history.
16. Reads bound both output and examined work under adversarial occupancy, history,
    graph width/depth and geometry complexity.

## 23. Rejected traps

- coordinates, cell ids, parent paths, names, owners or shard keys as Place identity;
- one universal `nearby`, `contains`, `relation` or `owner` fact driving mechanics;
- geometry as mandatory state for every Place or Entity;
- geometry contact as automatic traversal or access;
- open Agent-authored relation predicates with mechanical effect;
- a server ontology that infers physics, blast radius, hearing or ownership from
  prose, Properties or Traits;
- recursive relation traversal without depth, edge, result and time bounds;
- rewriting every descendant when a moving carrier changes pose;
- duplicating derived ancestor placement as ungoverned current truth;
- using a spatial index/cell match without the accepted exact structural predicate;
- letting H3, geohash, region server or database partition become a game Place;
- whole-Place freshness for every mutation merely because actions share context;
- last-write-wins for conflicting shared facts;
- `SKIP LOCKED` or silent omission for canonical movement/placement truth;
- storing one occurrence, delivery or perception row per observer;
- delivering private content to an Agent and asking it to self-filter;
- treating possession as User control or introduction provenance as fictional
  ownership;
- mass literal Entity rewrites hidden behind one “World effect” row;
- continuous trajectory/history for dormant subjects before a mechanic needs it;
- a million-User claim from uniform load, conceptual simulation, one local database
  or one model call; and
- partitioning, a graph database, broker, event sourcing or generic rule engine
  before a measured current risk earns it.

## 24. Exact open product choices

### First game edge

1. Which concrete player outcome is first: connected-Place movement, a container,
   a moving carrier, one cross-Place occurrence or settlement access?
2. What should a second player be able to do or learn that is impossible today?

### Identity and Place structure

3. Can two discovered Place fragments later be related as the same referent without
   deleting either historical identity?
4. Is semantic Place containment a tree, a DAG, or absent from the first slice?
5. Does an immediate spatial parent point only to a Place, or also to containers/
   frames that derive an ultimate Place?

### Traversal

6. Is the first connection a directed Place edge, an Entity door/portal or a route
   segment?
7. Who may establish, modify, disable or discover that connection?
8. Which access fact admits traversal, and what neutral failure is player-legible?

### Geometry

9. Which accepted mechanic cannot be expressed by exact Place and typed relations?
10. Does it need 2D or 3D; Cartesian, geodetic or local moving frames?
11. Which geometry kinds, units, boundary predicate and precision are gameplay truth?
12. May a Place have no geometry, incomplete geometry or multiple geometry versions?
13. Who authors geometry, and what maximum bytes/components/vertices are acceptable?

### Moving frames and containers

14. Is a vehicle interior a Place role, a frame owned by an Entity, or both?
15. Which interior Actions ignore carrier pose, and which depend on it?
16. What maximum nesting depth is meaningful to play?
17. How does attach/detach interact with movement through a portal?

### Ownership, control, possession and access

18. Which first operation needs control beyond current User→Character ownership?
19. Are possession, stewardship and access visible public World facts or partly
    private authorization facts?
20. Can control/access be shared, delegated, leased, inherited, revoked or abandoned?
21. Does a container or Place grant any rights over contents, and if so which exact
    operation—not “ownership” generally?
22. What prevents privilege escalation when a controller creates another grant?

### Locality and observation

23. Does co-presence compare immediate Place, selected ancestor, metric geometry or
    an operation-specific combination?
24. Can one compact reach traverse containment/adjacency edges, and what maximum
    depth/results make it complete enough for the mechanic?
25. Is public occurrence eligibility fixed at acceptance time, evaluated at read
    time, or split into live and later-history rules?
26. Which active occurrences require lossless catch-up, and which allow coalesced
    hints plus current state/bounded history?
27. How many Place/Entity/reach interests may one active host hold?

### Transactions and overload

28. What exact placement, relation, geometry and grant coordinates make the first
    Action stale?
29. Does expected absence require a stable slot, Entity lock, Serializable predicate
    or another bounded coordinator?
30. What is the maximum atomic subject/edge count and maximum reach evaluation work?
31. What bounded busy/conflict response is fair on a hot portal, Place or Entity?
32. Which quiet-control latency and availability must survive the extreme hotspot?

### History and operation

33. Which spatial versions must an Activity retain versus only stable ids and roles?
34. Does any accepted behavior require non-overlapping placement intervals or a
    continuous trajectory?
35. Which indexes/partitions are earned by the chosen queries, and how is their
    rebuild/repartition invisibility proved?
36. What exact connected-User, request-rate, skew, burst, latency, busy-rate,
    history and recovery target will “supports millions” mean?

## 25. Primary-source audit

All external sources below were checked on 2026-08-18. “Transfer” names the fact used
here; it does not endorse the source system as Aicadia architecture.

| Primary source | Owner/type | Transfer used in this report | Limitation |
| --- | --- | --- | --- |
| [RFC 9562](https://www.rfc-editor.org/rfc/rfc9562.html) | IETF standard | UUID generation properties are distinct from access control | does not choose Aicadia's UUID version |
| [RFC 9110](https://www.rfc-editor.org/rfc/rfc9110.html#name-idempotent-methods) | IETF standard | retry safety requires idempotent semantics or another detection mechanism | HTTP method semantics do not implement domain idempotency |
| [Kubernetes names and UIDs](https://kubernetes.io/docs/concepts/overview/working-with-objects/names/) | project-owned docs | human names and stable historical identity are separate | cluster object lifecycle is not game identity |
| [Kubernetes owner references](https://kubernetes.io/docs/concepts/architecture/garbage-collection/#owners-and-dependents) | project-owned docs | lifecycle dependency is not the same as authorization | no recommendation to copy garbage collection |
| [Kubernetes RBAC](https://kubernetes.io/docs/reference/access-authn-authz/rbac/) | project-owned docs | access is a separate explicit policy concern | RBAC alone is too coarse for all game relations |
| [Kubernetes API watch](https://kubernetes.io/docs/reference/using-api/api-concepts/#efficient-detection-of-changes) | project-owned docs | bounded history can expire; clients must re-list/refetch after `410 Gone` | Kubernetes resourceVersion must not become a global World revision |
| [OGC GeoSPARQL 1.1](https://docs.ogc.org/is/22-047r1/22-047r1.html) | OGC standard | qualitative topology and explicit geometry are separable conformance components | RDF/OWL storage is not recommended |
| [PostGIS spatial management](https://postgis.net/docs/using_postgis_dbmanagement.html) | project-owned manual | geometry/geography/SRS distinctions and local SRS support | Earth-oriented examples do not choose Aicadia's World shape |
| [PostGIS `ST_Contains`](https://postgis.net/docs/ST_Contains.html) and [`ST_Covers`](https://postgis.net/docs/ST_Covers.html) | project-owned manual | boundary semantics differ and invalid geometry is unsafe | one predicate must be chosen per mechanic |
| [PostGIS `ST_3DIntersects`](https://postgis.net/docs/ST_3DIntersects.html) and [`ST_3DDWithin`](https://postgis.net/docs/ST_3DDWithin.html) | project-owned manual | 2D and 3D answers/transform assumptions differ | full 3D world modeling remains broader than these predicates |
| [PostGIS validity](https://postgis.net/docs/ST_IsValid.html), [vertex count](https://postgis.net/docs/ST_NPoints.html) and [subdivision](https://postgis.net/docs/ST_Subdivide.html) | project-owned manual | geometry validity and complexity must be bounded; subdivision is an optimization | no acceptable Aicadia bounds are selected |
| [PostGIS spatial indexes](https://postgis.net/documentation/faq/spatial-indexes/) | project-owned docs | bounding-box candidates plus exact predicate | GiST does not solve semantic locality or authorization |
| [H3 indexing](https://h3geo.org/docs/highlights/indexing/) | project-owned docs | cell hierarchy may be logically exact while geometric containment is approximate | global Earth grid may be irrelevant to fictional space |
| [Recast Detour source](https://github.com/recastnavigation/recastnavigation/blob/main/Detour/Include/DetourNavMesh.h#L2089-L2124) | project-owned source | off-mesh traversal is explicit and can be directed | navmesh runtime is not a persistent game-domain model |
| [Unreal Nav Link Proxy](https://dev.epicgames.com/documentation/en-us/unreal-engine/API/Runtime/AIModule/ANavLinkProxy) | first-party engine docs | disconnected navigation areas use explicit, dynamically enabled links | no recommendation to adopt Unreal |
| [OpenStreetMap elements](https://wiki.openstreetmap.org/wiki/Elements#Relation) and [turn restrictions](https://wiki.openstreetmap.org/wiki/Relation:restriction) | project-owned model docs | route membership and directed traversal restrictions need explicit roles | OSM tags are open-world map content, not deterministic game authority |
| [Unreal attachment](https://dev.epicgames.com/documentation/en-us/unreal-engine/BlueprintAPI/Transformation/AttachActorToActor) | first-party engine docs | child transform can remain relative to moving parent | frame persistence/concurrency are Aicadia's own problem |
| [Unreal networking](https://dev.epicgames.com/documentation/en-us/unreal-engine/networking-overview-for-unreal-engine), [relevancy](https://dev.epicgames.com/documentation/en-us/unreal-engine/actor-relevancy-in-unreal-engine) and [Replication Graph](https://dev.epicgames.com/documentation/en-us/unreal-engine/replication-graph-in-unreal-engine) | first-party engine docs | authority, relevancy and client presentation are different; candidate work can be shared | published scale examples are not million-player proof |
| [Roblox streaming](https://create.roblox.com/docs/workspace/streaming#replication-focus) | first-party platform docs | every additional interest focus has server/client cost | Roblox instance architecture is not Aicadia's World model |
| [Zanzibar paper](https://www.usenix.org/system/files/atc19-pang.pdf) | original Google/USENIX paper | typed relation tuples, causal authorization snapshots and deep/wide traversal cost | access-control graph is not a universal World relation graph |
| [OWASP API1:2023](https://owasp.org/API-Security/editions/2023/en/0xa1-broken-object-level-authorization/) | OWASP primary guidance | every object-id operation needs object-level authorization | threat guidance does not choose game-visible errors |
| [PostgreSQL locking](https://www.postgresql.org/docs/current/explicit-locking.html) and [isolation](https://www.postgresql.org/docs/current/transaction-iso.html) | official database manual | row contention, stable lock order, deadlocks and full Serializable retry | docs do not select one Aicadia coordinator |
| [PostgreSQL recursive queries](https://www.postgresql.org/docs/current/queries-with.html#QUERIES-WITH-CYCLE) | official database manual | recursive graph traversal needs termination and cycle tracking | application bounds remain required |
| [PostgreSQL ranges](https://www.postgresql.org/docs/current/rangetypes.html#RANGETYPES-CONSTRAINT) | official database manual | exclusion constraints can enforce non-overlapping intervals | intervals are unearned until event-time presence is gameplay |
| [PostgreSQL partitioning](https://www.postgresql.org/docs/current/ddl-partitioning.html) | official database manual | partitions affect uniqueness, movement and planning cost | partitioning threshold is workload-specific |
| [PostgreSQL timeouts/resources](https://www.postgresql.org/docs/current/runtime-config-client.html) | official database manual | lock/statement waits and connection/memory use need explicit bounds | settings alone do not provide fair admission |
| [FoundationDB developer guide](https://apple.github.io/foundationdb/developer-guide.html) | project-owned manual | exact conflict ranges, hot-key limit and watch/refetch semantics | FoundationDB is pattern evidence, not an infrastructure recommendation |
| [MobilityDB introduction](https://docs.mobilitydb.com/MobilityDB/develop/ch01.html) | project-owned manual | continuous trajectories are a specialized temporal-spatial subsystem | no need is established for this dependency |

## Final synthesis

**Inference.** Aicadia can support a rich spatial multiplayer future without making
every Entity geometrical, every relation generic or every Place a shard. The key is
to make each structural truth independently explicit and composable by concrete
operations: stable identity; one immediate placement; optional geometry in a named
frame; typed containment, connection, possession and access relations; operation-
specific locality; exact conflict dependencies; atomic Activity; authorized bounded
interest; and replaceable indexes.

**Recommendation.** The next grill should not ask “which full spatial architecture
do we choose?” It should start at R0 with one game scene whose player outcome needs
the foundation—most plausibly a Character moving through one explicit connection
between two stable Places, or a carrier interior if moving multiplayer is the higher
value edge. The grill should then walk the dependency tree and accept only the
identity, placement, authority, locality, transaction and history facts that scene
requires, while preserving the separations above.

**Remaining evidence gap.** Primary sources establish transferable patterns and
failure modes, while Aicadia's labs establish only small exact-fact fixtures. No
current evidence proves a production spatial schema, geometry model, access model,
movement transaction, moving-frame implementation, privacy boundary, hot-Place
capacity or million-User operation. Those claims require accepted product choices
followed by the ordered semantic, PostgreSQL, privacy, delivery and failure gates in
this report.
