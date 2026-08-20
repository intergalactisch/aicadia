# Place

> **Role / side:** current Place development synthesis / development side.
> **Authority:** owns the current meaning, boundary, decisions, unresolved landscape, components and directional technical model for Place.
> **Excludes:** selected work, exact game behavior, sourced findings, Movement decisions and delivery claims; those remain in `dev/backlog/` and plans, `game/docs/`, `dev/docs/research/`, the [Movement Area](../movement/README.md) and `dev/docs/evidence/`.

## Meaning

Place concerns durable location and spatial structure at rest: how a location keeps
one identity, how Characters and Entities are positioned there, and which explicit
structural facts may connect, contain or bound it. A Place is sparse World state,
not a pre-computed complete map.

## Boundary

### This is

- Stable Place identity and the Position of Characters and Entities.
- Explicit spatial structure such as Area, Relations, Connections and boundaries when earned.
- A lens shared by observation, Movement, Discovery and concurrent World change.
- Sparse, incrementally established World geography.

### This is not

- A database shard, universal transaction lock or universal visibility boundary.
- An identity inferred from a name, description, path, coordinate or geometry.
- A hidden complete geography waiting to be revealed by the server.
- A universal graph in which every fictional relation becomes the same edge type.

## Decisions

### Chosen

- Entity is the broad game identity of one durable, independently addressable World
  subject; a city or other non-object subject may therefore be an Entity.
- Position is the optional exact current point of any Entity in three spatial
  dimensions, expressed as whole-centimetre `x`, `y` and `z` from either the
  permanent World origin or exactly one other Entity.
- Position is separate optional state keyed by the Entity's existing identity. It
  has no independent Position id. A Place uses that same Position rather than
  duplicating coordinates on its Place role.
- A Place is the role of a positioned Entity that World establishes as an independent
  reference for map, discovery, navigation or explicit spatial relationships. It
  uses the Entity's one stable identity.
- Exploration can begin from every Character Position, including a Position with no
  current Place. Place is never required or fabricated merely to admit look-around,
  discovery or ordinary play between named locations.
- The existing Investigation opportunity remains the admission path for a genuine
  natural find and is extended to any current Character Position. Spatial does not
  redesign Investigation and never requires current Place for it.
- If Place discovery succeeds while current Place is absent, the complete confirmed
  result establishes a separately identified named origin Place at the Character's
  unchanged Position, sets it as current and connects it to the discovered Place.
  The Character Entity never receives the Place role merely because it supplied that
  point.
- The Agent dynamically composes exploration from available bounded MCP tools. Place
  prescribes no universal existing-context/discovery order, and World stores no
  exploration workflow or read receipt; shared proposals still require fresh relevant
  grounding and confirmation.
- Any positioned Entity may deliberately receive the Place role when Agent and User
  establish it as an independent spatial reference. World applies no city, forest,
  waterfall, building, object or other semantic kind allowlist and never grants the
  role merely because Position exists.
- Establishing the Place role writes no second subject identity: one role row uses
  `entity_id`, while Entity keeps name, description, Properties and Traits. Area and
  Connections add only Place-specific spatial state, with Activity for the accepted
  state change.
- The player-level shorthand is “a named, discovered Position,” implemented through
  that one subject: Entity owns the name and extensible content, Position owns the
  point and Place owns only the deliberate spatial-reference role. This adds no
  independent Place id or duplicate name, Property, Trait or coordinate storage.
- A bounded coordinate window returns all ordinary established Places selected by
  resolved Position in that exact window, with page limit and continuation instead of
  an unbounded response or count. The Agent may use a window around the Character's
  current Position or an exact proposed discovery point. Base Place geography is not
  Character-Knowledge-gated; a future protected Place must earn explicit access state.
- A Place-window read writes no Knowledge, Observation or Activity and never expands
  matching coordinates into all Characters, Entities or inventory. Sparse
  Character–Place Knowledge remains parked outside S1 for a future remembered map.
- The coordinate-window map read is distinct from Place neighborhood, which remains
  a view of explicit structural relationships around one exact Place. Neither read
  returns every positioned Entity or infers observation and visibility.
- An Area is optional spatial coverage of one Place. A Place need not have an Area,
  and Area overlap does not establish identity, travel, ownership or visibility.
- Area is exact positive coverage: a point inside established coverage is proven to
  fall in that Place, while a point outside all current coverage remains unknown
  rather than proven outside. Missing Area means no coverage has been established.
- Several Place Areas may cover the same point; each positive result remains true
  and overlap never selects one parent, merges Place identities or creates travel.
- Place identity, Position, Area, Relations, Connection and visibility are separate concerns.
- The World is semantically unbounded and technically sparse; only established state needs storage.
- Exact Place equality is a useful narrow context boundary, not a universal co-presence or observation rule.
- Spatial facts are explicit typed structural truths validated by World, while Agents author their meaning.
- The first selected spatial scene establishes a second Place and lets a Character
  reach it through explicit connectedness; its exact discovery, connection and
  movement contracts remain open.
- Independently confirmed discoveries that are not structurally proven to be one
  known Place keep distinct stable identities; the first spatial slice performs no
  inferred merge or reconciliation.
- Discovering the selected second Place establishes it and its explicit
  connectedness while the discovering Character remains at the origin.
- The confirmed discovery of B is one all-or-nothing package: B's Entity/Place role,
  name, description, optional initial Properties and Traits—including authored
  landscape state—direct Position and explicit A→B Connection commit with one
  Activity. Failure creates none of those records; retry returns the same complete
  result.
- The first scene may distinguish Place A as grassy and Place B as dunes through
  their Entity descriptions or Agent-authored Properties. Those values remain
  inspectable authored Place state and have no server-inferred travel effect.
- The first scene stores no separate geographic adjacency between A and B. Its
  explicit A→B Connection is sufficient for the selected walking behavior and does
  not prove that their Areas touch or their Positions are near.
- In the first scene the Agent derives and submits B's exact absolute World Position
  as structured data after reading A's fresh Position. Before confirmation it must
  perform an eligible bounded nearby read and include returned established C, G or
  Q in the complete preview; World calculates only numeric proximity and never
  interprets the Agent's prose. An omitted hidden or out-of-scope Place is not proof
  that no other Place exists nearby.
- That proposed Position may be any technically valid absolute World point; the
  first slice has no gameplay distance limit from A. Numeric distance neither grants
  nor denies Place identity or Connection.
- Every exact World point has three spatial dimensions from the start. This supports
  vertical distinction for cliffs, caves, bridges and precise object placement
  without yet adding geometry, surfaces, Orientation or server-inferred meaning.
- Each of those three values is an exact whole number of centimetres. This makes the
  accepted two-centimetre object distinction exact without millimetre precision or
  decimal-metre rounding. Agents convert conversational units before submission;
  World never parses a unit from prose.
- The three values are named `x`, `y` and `z`; `x` and `y` form the horizontal plane
  and `z` is vertical height. Increasing `x` is east, increasing `y` is north and
  increasing `z` is up. Agents convert compass language to those values; World never
  parses the words.
- `(0, 0, 0)` is the permanent World origin. It is a coordinate convention, not an
  Entity, Place, frame, row or movable subject. The first entry Place is initially
  positioned there; if that Place ever moves, the origin does not.
- If that context contains a plausible existing Place, the Agent first discusses
  reuse and the reason for any distinct new Place with the User. Only the User's
  explicit choice may continue the new-Place proposal; World neither evaluates the
  reasoning nor chooses between the Places.
- If the User instead chooses existing C as the discovered destination, one
  confirmed atomic result establishes the explicit allowed A→C Connection and
  Activity without creating or changing C's Place identity or Position. Proximity
  supplied context; the confirmed Connection supplies direct travel.
- The required A→C direction is established at most once in this first slice. If it
  already exists or a concurrent request wins, the later proposal writes no
  duplicate Connection or Activity and retains its positive opportunity; only an
  exact retry returns the original accepted result.
- Equal or nearby Positions neither identify nor conflict two Places. Concurrently
  established Places remain distinct, and proximity alone never blocks or merges
  them.
- An independently discoverable or interactable plant may be an Entity; that does
  not require every grass-covered Place to materialize individual grass Entities or
  introduce a terrain mechanic.
- Every connection explicitly states its allowed direction or directions; a reverse
  connection is never inferred from the existence of forward travel.
- `Connection` is the canonical one-word name for this dedicated Place-to-Place
  primitive. It is not a `Relation` type or generic relationship family, and `Link`
  is not Aicadia vocabulary for it.
- One Connection is one stable direct travel alternative rather than the unique
  endpoint pair. A footpath, ferry and portal may therefore be separate Connections
  between the same Places, each with its own stable identity and allowed direction.
- Every Connection has one Agent-authored name and description so an Agent can
  distinguish and explain parallel alternatives without exposing internal ids. This
  text grants no mechanics, and Connection owns no Properties or Traits.
- A Connection may optionally carry one reusable exact spatial shape for its own
  alternative. Absence is valid for a portal or an alternative whose middle has not
  been established; an actual Movement does not restate or own that reusable shape.
- That optional shape is a bounded ordered sequence of exact whole-centimetre
  `x`/`y`/`z` World points, with a straight segment between each consecutive pair.
  It represents the Connection's course, not its width or one Character's footsteps.
- The optional shape may include at most one Agent-authored description for extra
  narrative context. That text is stored and returned with the points, has no
  separate identity or mechanics and is never parsed as geometry or crossings.
- A tangible door, bridge or road is an Entity, while the direct Connection it may
  support is a separate non-Entity spatial fact.
- A Character may persistently stop, meet another Character, discover or place an
  Entity at an unnamed Position between established Places without first
  establishing another Place.
- One Movement may traverse a whole Connection or stop at one exact point on an
  explicitly named segment of its ordered course. The intermediate result is the
  Character's ordinary Position and Activity, not another Place or durable journey.
- Spatial state must let an Agent express distinctions as precise as a cup currently
  floating two centimetres above a table without requiring every spatial predicate
  such as `above` or `under` to be a server-owned enum.
- Position is either absolute from the permanent World origin or relative to exactly
  one other Entity. A relative Position uses the same World axes and mechanically
  follows changes to its reference Entity's resolved Position; Agent-authored
  Relation wording never chooses or causes that behavior.
- Because Place is an Entity role, a Place may also use an Entity-relative Position;
  a cabin can therefore remain a Place while moving with a ship. Orientation is not
  part of Position, so relative coordinates currently inherit translation only and
  do not rotate with their reference.
- Entity-relative offsets start at the reference Entity's one Position point. The
  spatial foundation has no exact surface, part or internal-point target; an Agent
  may understand such wording and choose offsets, but World does not validate it.
- A successfully resolved Position read returns both the current World point and
  the Position basis actually stored. An absolute Position returns its origin-based
  values; an Entity-relative Position returns its immediate reference Entity and
  stored offsets. The resolved point is calculated output, never a second stored
  spatial truth.
- Every Position read stays bounded. When it cannot reach an absolute basis
  within that work, it still returns the immediate stored reference and offsets but
  no current World point. World never substitutes a stale cached point, and an
  operation requiring an exact World point fails closed until resolution succeeds or
  the Position is simplified.
- A bounded current Place read that already returns an Entity also includes that
  Entity's complete Position when one exists. It does not require one follow-up
  Position call per Entity and performs no Position-specific redaction. A later
  privacy rule may decide whether the Entity is selected at all; it does not turn one
  returned Entity into a partially hidden spatial record.
- Relative Position references never form a cycle because such a structure cannot
  establish one exact point. Artistic impossibility belongs in explicit World
  experience instead: Connections may deliberately form loops or join Places whose
  coordinates are not geographically consistent, while authored text narrates why.
- World accepts a new or changed relative Position only when bounded validation
  reaches an absolute basis without returning to the changed Entity. Detecting a
  cycle or exhausting that work both reject the mutation. The revisions examined by
  validation must still be current at commit, so concurrent A→B and B→A proposals
  cannot both win.
- Changing Position reference always supplies one complete new Position: the new
  World or Entity reference and all three new offsets. World has no implicit
  preserve-world or preserve-offset mode. The Agent chooses the intended result and
  must account for eligible authored characteristics such as a cup that always
  floats two centimetres above a surface; World never interprets or enforces them.
- That durable cup characterization is a Trait. It stays with the cup across Position
  changes and guides the Agent's proposal, while Position description narrates only
  the current placement and neither text surface becomes executable World physics.
- A User instruction states the attempted act, not a guaranteed literal Position.
  The Agent may creatively propose that forcing the cup onto the table fails and
  instead leaves it ten centimetres above it. The proposed result still names the
  complete exact Position and what happens to Trait and description; World invents
  none of those consequences.
- Position may carry an optional Agent-authored `description` that helps another
  Agent narrate the current spatial situation, such as a cup strangely remaining
  two centimetres above a table. Position description has no mechanical meaning and
  World never parses it into coordinates, following behavior, geometry or a Relation.
- Every accepted Position change explicitly keeps, replaces or removes its current
  description. That choice settles with the same Position revision and transaction;
  description has no independent revision, write conflict or lock, and World never
  judges whether retained text remains semantically accurate.
- One Position has at most one current description. That text may contain several
  sentences or paragraphs within ordinary text bounds, so narrative richness does
  not require a list of independently versioned Position texts.
- Position owns no Traits. Multiple durable characterizations belong to the Entity's
  existing Traits and survive Position changes; making Position another Trait owner
  would give current placement a separate identity and lifecycle it does not have.
- Position description is returned together with Position and has no separate read,
  audience or query. The spatial foundation performs no partial Position redaction.
- Knowing an Entity exists does not create a separate current-Position lookup. A
  concrete bounded read decides which Entities it returns, and returns each selected
  Entity's complete Position when one exists. A prior observed point may remain
  Knowledge but is not fresh current state for mutation.
- Trait statements and Position description remain semantically open Agent-authored
  text. A conforming Agent may use that meaning to withhold details in its own player
  conversation, but World returns the ordinary structured Position and provides no
  Position-specific confidentiality or visibility mechanic in this foundation.
- Relation is a stable non-Entity record with one source Entity, one target Entity
  and free Agent-authored name and description. It has no server-owned semantic kind
  or mechanical authority, and several Relations may coexist between the same pair.
- Relation wording may be developed while its stable identity remains. Activity can
  identify the exact Relation involved; first authorship remains history rather than
  permanent exclusive control.
- Ordinary current Position and Relation meaning are not permanently editable only
  by their first author or an Entity controller. Any Agent acting through a Character
  may in principle propose their change; exact locality and action requirements remain open.
- Spatial presence does not expose every Relation involving a present Entity. A
  Character may encounter a person without learning which Entities are related to
  that person's inventory; even the hidden Relation's existence is not disclosed.
- A previously observed spatial Relation may remain Character memory after becoming
  hidden, but that memory is not authoritative current placement and grants no live read.
- No generic Containment primitive combines physical `inside`, inventory, holding,
  part-of, Area inclusion or moving together. A scene composes its open Relation,
  optional exact Position, explicit movement behavior, visibility and action access.

### Rejected

- Giving a Place a second surrogate identity for the same durable subject.
- Treating names, prose similarity or coordinates as authoritative Place identity.
- Making a Connection or coordinate frame into an Entity.
- Using Place as a universal infrastructure partition, visibility scope or mutation lane.
- Treating co-location or Place equality as automatic visibility of every spatial Relation.
- Requiring every Position to receive a Place identity.
- Storing a second Anchor that duplicates a Place Entity's Position.
- Treating a closed enum of spatial words as the complete expressive World model.
- Treating a Connection as geographic adjacency or inferring Connection from it.
- Using a cyclic Position reference as the implementation of an impossible staircase,
  looping doors or another artistic travel experience.
- Allowing the selected discovery to leave B without its Position or confirmed A→B
  Connection, or to leave a Connection whose B endpoint was not established.
- Giving grass, dunes or another authored Place description automatic movement,
  access or visibility mechanics.
- Making one coordinate or proximity radius a Place-identity, uniqueness or
  automatic-merge rule.
- Automatically giving every positioned Entity the Place role, or restricting Place
  to a server-owned list of semantic kinds.
- Giving the World origin an Entity, Place, frame identity, lifecycle or lock, or
  making it follow the entry Place after initial placement.
- Giving the first author permanent exclusive edit authority over ordinary spatial state.
- Treating `containment` as one generic primitive for physical `inside`, inventory,
  holding, part-of, Area inclusion and moving with another Entity.
- Treating Trait wording such as “hidden from its possessor” as an executable
  Position-visibility rule that World must interpret.
- Introducing a generic `world_change` model, table or operation, or a standalone
  editor that mutates visibility without one concrete Action or Introduction.
- Giving Position its own Traits or an independently growing description collection;
  neither current gameplay nor the spatial scenarios require those extra owners,
  identities, ordering rules or unbounded reads.

### Not yet chosen

- Which rule makes three existing Places eligible for `enter_world` and what happens
  before three eligible Places exist.
- The exact bounded establish/develop operations, ordinary text bounds, dependency
  revisions and duplicate presentation for the accepted Relation record.
- Which exact geometry representation Area uses, whether one Place may have
  disconnected coverage and how bounded corrections or extensions change it.
- Whether one Place falling spatially within or across another is calculated from
  Position and Area, recorded as an explicit structural fact, or both for distinct
  purposes. Overlap may return several candidates and never implies one parent.
- The exact point-count and coordinate bounds, endpoint alignment and freshness
  behavior when an endpoint Place Position changes.
- Whether Area intersections are calculated from the ordered points on read and how
  incomplete or impossible geography reports unknown portions without separately
  authored crossing truth.
- Which concrete later gameplay, if any, earns exact Entity surfaces, geometry or
  internal reference points beyond the foundation's one Position point.
- The integer width, signed range and maximum extent of Position's three centimetre
  values.
- How an Agent may additionally establish a free quantitative spatial statement such
  as “two centimetres from the ground” when that statement alone does not identify
  one exact point.
- Which other exact current facts let a Character read an Entity's Position, and
  whether an immediate reference that is structurally ineligible for that Character
  makes the whole Position unavailable or permits a safely reduced result. Authored
  text alone never creates that ineligibility.
- How a later privacy and visibility design protects information from a modified or
  non-conforming Agent. The spatial foundation deliberately contains no Position-
  specific denial, redaction hook or placeholder permission seam.
- How concurrent movement or re-referencing locks and validates the exact examined
  Position revisions without an ambiguous current point or unnecessarily locking a
  whole deep chain.
- Which accepted inventory gameplay, if any, earns a concrete Inventory mechanic
  with its own current invariants.
- Whether an open-terrain Connection may exist without a physical Entity and who may establish it.
- Which concrete later behavior earns geographic adjacency as stored structure or
  terrain as a mechanical model; both are deliberately absent from the first scene.
- How Area geometry and boundaries are represented when a concrete scene requires them.
- Which explicit later evidence can relate or reconcile independently established Places.
- Which later player behavior can prevent or organize a proven wild growth of
  overlapping Places while preserving stable identity, history and a dumb World.
- Which exact Discovery authority confirms a new Place and its connectedness.
- Which minimum placement and traversal facts are sufficient for its first Movement capability.

## Research needed

- Test Place identity and later merge or relation semantics without server omniscience.
- Find the smallest topology that supports a concrete movement and exploration scene.
- Test whole-centimetre Position at world scale only when a concrete movement scene
  makes numeric range, indexing or precision the remaining risk.
- Measure bounded occupancy and history reads for one extremely busy Place.
- Compare optional geometry and reference-frame models only against earned game behavior.
- Compare open semantic Relations with relative Position and moving-reference models
  against the cup-above-table, held-glass and Entity-on-Entity scenarios.

## Components

| Component | Current meaning |
| --- | --- |
| Identity | The stable identity of the broad Entity subject that bears the Place role. |
| Position | An Entity's optional exact whole-centimetre point from permanent World origin or exactly one reference Entity. |
| Area | Optional exact positive coverage of one Place; uncovered space remains unknown. |
| Connection | One stable named direct travel alternative between Places, never inferred from geometry. |
| Boundary | A known Area limit, which may remain incomplete or absent. |
| Occupancy | Bounded current membership used by concrete reads, not a permanent audience list. |
| Scenarios | The [fifteen spatial hard cases](scenarios.md) used to pressure candidate models before vocabulary or schema is chosen. |

## Technical model

### Delivered

The current World has at most one entry Place represented by an Entity-role row.
Characters and Entities can be established at that exact Place, contextual writes
advance its latest Activity, and bounded reads expose its current contents. Exact
fields and rules remain in [`game/docs/`](../../../game/docs/README.md).

### Directional

One broad Entity identity represents each independently addressable World subject
and Place is a typed spatial role of that Entity. Separate optional Position state
uses the same Entity id and stores that Entity's exact absolute or one-Entity-
relative current point; a Place uses that Position as its map point and may have a
separate positive Area. One stable named Connection alternative may own ordered World
points, and Movement may stop at an exact point on them without journey state.
Position integer range, relative-reference bounds, exact Area geometry and concrete
capabilities remain open.
Transactions lock the smallest affected subjects; no Place becomes a global World
partition.

### Absent

Additional Places, Connections, Area, general Position, inventory, routes,
geometry, coordinate frames, Place establishment and identity-reconciliation
behavior are absent from the current game contract.

## Sources

- Prepared pressure — [Spatial scenario catalogue](scenarios.md).
- Retained rationale — [spatial direction](../../docs/concept/spatial.md).
- Sourced findings — [spatial multiplayer foundation](../../docs/research/spatial-multiplayer-foundation.md).
- Sourced Entity/Place comparison — [game-framework boundary](../../docs/research/entity-place-framework-boundary.md).
- Sourced extent/traversal comparison — [Place extent, spatial inclusion and Connection traversal](../../docs/research/place-area-connection-traversal.md).
- Current technical candidate — the [completed spatial technical synthesis](../../docs/concept/spatial-five-year-backcast.md#technical-synthesis-after-the-completed-grill) translates the chosen Place direction into a falsifiable PostgreSQL and World shape without authorizing implementation.
- Related synthesis — [Movement](../movement/README.md), [Discovery](../discovery/README.md) and [Multiplayer](../multiplayer/README.md).
- Exact behavior and delivery — [`game/docs/`](../../../game/docs/README.md) and [`dev/docs/evidence/`](../../docs/evidence/README.md).
