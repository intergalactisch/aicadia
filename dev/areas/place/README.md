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
- Position is the optional exact current point of any Entity; its coordinate or cell
  representation remains open.
- Position is separate optional state keyed by the Entity's existing identity. It
  has no independent Position id. A Place uses that same Position rather than
  duplicating coordinates on its Place role.
- A Place is the role of a positioned Entity that World establishes as an independent
  reference for map, discovery, navigation or explicit spatial relationships. It
  uses the Entity's one stable identity.
- An Area is optional spatial coverage of one Place. A Place need not have an Area,
  and Area overlap does not establish identity, travel, ownership or visibility.
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
- Every connection explicitly states its allowed direction or directions; a reverse
  connection is never inferred from the existence of forward travel.
- A tangible door, bridge or road is an Entity, while the direct Connection it may
  support is a separate non-Entity spatial fact.
- A Character may persistently stop, meet another Character, discover or place an
  Entity at an unnamed Position between established Places without first
  establishing another Place.
- Spatial state must let an Agent express distinctions as precise as a cup currently
  floating two centimetres above a table without requiring every spatial predicate
  such as `above` or `under` to be a server-owned enum.
- The same precise arrangement may be independent or persist relative to another
  Entity. The Agent explicitly chooses which is established; Relation wording does
  not decide whether one Entity follows another.
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
- Giving the first author permanent exclusive edit authority over ordinary spatial state.
- Treating `containment` as one generic primitive for physical `inside`, inventory,
  holding, part-of, Area inclusion and moving with another Entity.

### Not yet chosen

- Whether `Connection`, `Link` or another single canonical name identifies the
  primitive, and whether its scope is Place-only, navigation-wide or one member of
  a broader typed relation family. Two-word names are rejected.
- Whether Place topology lives in a universal Relation base with a server-owned type,
  a typed Relation extension or its own table behind a common bounded read.
- How exact Position, geometry and open Agent-authored Relation meaning divide one
  nuanced statement without duplicated truth.
- Whether Position itself has independent and relative forms, whether relative
  Position also causes movement with another Entity, or whether those are separate
  structural truths. This is reopened after Q15's tentative preference for the
  combined form.
- Whether that Position stores a direct World coordinate, a coordinate relative to
  one Entity or another deterministic representation.
- How an Agent may additionally establish a free quantitative spatial statement such
  as “two centimetres from the ground” when that statement alone does not identify
  one exact point.
- Whether a Place may use a relative Position or must remain independently grounded.
- Which Characters may learn an Entity's Position and whether a readable resolved
  point exposes any other spatial structure used to determine it.
- How any relative spatial chains are bounded, kept cycle-free and read consistently
  while one of their Entities moves concurrently.
- Which accepted inventory gameplay, if any, earns a concrete Inventory mechanic
  with its own current invariants.
- Whether an open-terrain Connection may exist without a physical Entity and who may establish it.
- Whether Position uses discrete neighboring cells, continuous coordinates or
  another deterministic representation.
- Which durable spatial subjects qualify for the Place role; examples still to
  settle include a city, forest and waterfall.
- Whether Area is authoritative positive coverage, a complete inside/outside boundary
  or descriptive map geometry, and how overlapping Areas behave.
- How Area geometry and boundaries are represented when a concrete scene requires them.
- Which explicit later evidence can relate or reconcile independently established Places.
- Which exact Discovery authority confirms a new Place and its connectedness.
- Which minimum placement and traversal facts are sufficient for its first Movement capability.

## Research needed

- Test Place identity and later merge or relation semantics without server omniscience.
- Find the smallest topology that supports a concrete movement and exploration scene.
- Compare discrete cells with continuous positions only after the Place-role
  boundary is settled.
- Measure bounded occupancy and history reads for one extremely busy Place.
- Compare optional geometry and reference-frame models only against earned game behavior.
- Compare open semantic Relations with relative Position and moving-reference models
  against the cup-above-table, held-glass and Entity-on-Entity scenarios.

## Components

| Component | Current meaning |
| --- | --- |
| Identity | The stable identity of the broad Entity subject that bears the Place role. |
| Position | An Entity's optional exact current point; its deterministic representation remains open. |
| Area | Optional spatial coverage of one Place; its exact boundary meaning remains open. |
| Connection | Explicit directed travel topology between two Places, never inferred from geometry. |
| Boundary | A known Area limit, which may remain incomplete or absent. |
| Occupancy | Bounded current membership used by concrete reads, not a permanent audience list. |
| Scenarios | The [thirteen spatial hard cases](scenarios.md) used to pressure candidate models before vocabulary or schema is chosen. |

## Technical model

### Delivered

The current World has at most one entry Place represented by an Entity-role row.
Characters and Entities can be established at that exact Place, contextual writes
advance its latest Activity, and bounded reads expose its current contents. Exact
fields and rules remain in [`game/docs/`](../../../game/docs/README.md).

### Directional

One broad Entity identity represents each independently addressable World subject
and Place is a typed spatial role of that Entity. Separate optional Position state
uses the same Entity id and stores that Entity's exact current point; a Place uses
that Position as its map point and may have a separate Area. Connection stores only
explicit directed Place topology. The Position representation, Area semantics and
concrete capability that establishes a Place remain open.
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
- Related synthesis — [Movement](../movement/README.md), [Discovery](../discovery/README.md) and [Multiplayer](../multiplayer/README.md).
- Exact behavior and delivery — [`game/docs/`](../../../game/docs/README.md) and [`dev/docs/evidence/`](../../docs/evidence/README.md).
