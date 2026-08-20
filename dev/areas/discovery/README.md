# Discovery

> **Role / side:** current Discovery development synthesis / development side.
> **Authority:** owns the current meaning, boundary, decisions, unresolved landscape, components and directional technical model for Discovery.
> **Excludes:** selected work, exact game behavior, sourced findings, experiment verdicts and delivery claims; those remain in `dev/backlog/` and plans, `game/docs/`, `dev/docs/research/`, `dev/lab/` and `dev/docs/evidence/`.

## Meaning

Discovery is the investigation path through which naturally occurring things that
nobody made or placed can become established shared World state. World resolves an
authoritative opportunity first; after a positive result, the Agent authors one
grounded candidate and the User confirms the complete preview before World commits it.

## Boundary

### This is

- A Character-grounded investigation followed by a World-owned chance result.
- A bounded opportunity for an Agent to author plausible new shared state.
- Explicit User confirmation of the full candidate before mutation.
- A future way to establish natural objects and, when separately designed, new Places.

### This is not

- The way a Character makes, brings or places an object; ordinary Action owns that.
- A User selecting the mechanical focus, odds, seed or result.
- World inferring plausible content, running an LLM or invoking an Agent automatically.
- Guaranteed novelty, success, movement or a new Place on every investigation.

## Decisions

### Chosen

- World resolves the investigation chance before an Agent may author candidate content.
- The Agent selects an investigation from current Character context but cannot control the roll.
- Future spatial Investigation extends that current context to every exact Character
  Position; current Place is not required and World creates no Place merely for
  admission. The existing chance and opportunity behavior otherwise remains intact.
- A positive opportunity is retry-stable and remains bound to its authoritative Character context.
- The Agent re-grounds after a positive result and previews the complete candidate for User confirmation.
- Naturally occurring finds use Discovery; made, brought or placed things use ordinary Action.
- The first slice can commit exactly one found Entity and no new Place or movement.
- For the selected future new-Place scene, independently confirmed discoveries stay
  distinct unless exact existing identity is structurally established; World does
  not infer sameness from names or descriptions.
- When the Character has an explicit current Place A, the selected new-Place package
  can establish one destination and its explicit A→B connectedness while the
  Character remains at its Position; entering B belongs to a later deliberate
  Movement action.
- When current Place is absent, the same confirmed package first establishes a
  separately identified, Agent-authored origin Place A at the Character's exact
  unchanged Position, sets A as current and establishes A→B. It never gives the
  Character Entity itself the Place role or creates an unnamed origin marker.
- Accepted Place discovery makes that exact destination and any accepted Connection
  knowable to the discovering Character even though the Character remains where it
  is. It does not reveal every Place connected beyond that destination or every
  other Place in the same coordinate window. Discovery writes the one sparse current
  Character-owned Knowledge result atomically with its result. One polymorphic shape
  for its two current subject families—Place and Connection—is technically viable;
  World-validated target integrity versus separate native-FK associations remains
  open. See the [storage research](../../docs/research/polymorphic-character-knowledge-storage.md).
  The Knowledge result's immutable server-authored `created_at` records when this
  Character first gained it, while Activity records that Discovery was the cause.
  Later explicit transmission remains open.
- With a valid selected origin Place A, one User confirmation covers the complete
  new-Place candidate: the Place Entity with its name, description and optional
  initial Properties and Traits—including authored landscape state—one exact direct
  Position and the explicit A→B Connection with its allowed direction. World accepts
  all of it with one Activity and consumes the opportunity, or writes none of it.
- A successful new-Place discovery cannot leave an unpositioned Place, and any
  included Connection cannot exist without both endpoint Places. From a Position
  with no current Place, origin A and destination B are both included in the one
  atomic result. Retry returns that same complete accepted result rather than
  repeating any part.
- That established connection explicitly names its allowed direction or directions;
  Discovery does not imply a return path.
- The established Connection also receives its own Agent-authored name and
  description. They let later Agents explain the travel alternative but never cause
  its direction, shape, access or movement behavior.
- For the selected new-Place scene, the Agent reads the Character's exact current
  Position and revision, reasons about the location itself and submits B's exact
  absolute World point as whole-centimetre `x`, `y` and `z` in structured data, with
  `z` as height.
  Increasing `x` is east, `y` north and `z` up. The Agent performs any compass
  conversion; World never derives that point, its unit or axes from prose, a Place
  description or a Property.
- Absolute discovery Positions are measured from the permanent `(0, 0, 0)` World
  origin. That origin is not a discoverable or mutable World subject.
- The first slice applies no gameplay distance limit between A and the proposed
  destination. The Agent may choose any technically valid absolute World point,
  regardless of distance, and the User may confirm it after nearby inspection.
  World validates the structured Position's still-unchosen representable numeric
  bounds, not a discovery radius.
- Before confirmation, the Agent must inspect a bounded set of ordinary established
  Places around B's proposed point and include that returned
  context in the complete User preview. Their proximity informs the Agent and User
  but never makes World block, merge or reuse B automatically. Hidden or out-of-
  scope Places remain unknown rather than being reported absent.
- When that read returns a plausible existing Place, the Agent must discuss it with
  the User before final confirmation: offer use of the existing Place and explain
  why a proposed B may nevertheless be a distinct Place. The User explicitly
  chooses whether the Agent may continue with new B; World does not judge the
  explanation or infer sameness.
- If the User chooses existing Place C as the discovered destination, the confirmed
  alternative result creates no new Place or Position. It explicitly establishes
  the allowed A→C Connection, consumes the opportunity and writes one Activity, or
  writes none of them. The Agent and User's confirmed proposal establishes that
  direct travel fact; C's proximity alone never does.
- If the required A→C travel direction already exists or another request establishes
  it first, this proposal writes no duplicate Connection or Activity and does not
  consume its opportunity. The Agent re-grounds and may author another candidate.
  A retry of the exact request that originally established A→C instead returns that
  accepted result and consumed-opportunity state.
- Concurrent discoveries at equal or nearby points remain independent Places unless
  later explicit World state establishes otherwise. A newly committed nearby Place
  does not invalidate an otherwise fresh proposal merely by entering its map window.
- Future spatial Discovery must be able to occur at an unnamed persistent Position
  between Places without first turning it into a new Place.

### Rejected

- Letting the User or Agent provide odds, seed, roll result or number of opportunities.
- Requiring every positive opportunity to carry a maximum distance from A before a
  concrete discovery behavior proves that restriction valuable.
- Rejecting an otherwise valid first-slice destination merely because it is far
  from A.
- Asking the Agent to invent content before World establishes a positive opportunity.
- Using Agent, connection or conversation restarts to reset discovery context.
- Adding a generic Discovery domain object when attempt provenance is sufficient.
- Treating every investigation as successful or every found thing as geographically new.
- Splitting the first confirmed new-Place candidate into independently accepted
  Place, Position, landscape and Connection mutations.
- Parsing Agent prose to choose a Position, or treating coordinate equality or
  proximity as automatic Place identity, uniqueness or reconciliation.
- Letting a conforming Agent ask for confirmation of a new Place without first
  inspecting and presenting the bounded nearby Place context available to its
  Character.
- Letting the Agent silently ignore a plausible returned Place, automatically reuse
  it or decide by itself that the proposed Place is distinct.

### Not yet chosen

- Whether later Discovery can establish several connected Entities or Places at once.
- How repeated investigation balances scarcity, novelty and hoarding without scores.
- Which future Character or Place facts may shape bounded chance tables.
- Which later concrete discovery behavior, if any, earns its own distance limit.
- How discoveries expose information differently to present, distant or returning Characters.
- How later evidence relates two independently discovered subjects that may be the same.
- Which later concrete capability may deliberately establish another Connection
  between the same Places; the first walking slice still has only one required A→C
  alternative even though the general model permits parallel Connections.
- Which later Agent- and User-driven behavior can curb, organize or reconcile a
  proven proliferation of overlapping Places without making World infer semantic
  sameness or invalidating their stable identity and history.
- Which later terrain context, beyond the accepted ordinary bounded Place window, is
  available to an Investigation grounded at a non-Place Position.

## Research needed

- Exercise Agent comprehension of the natural-find versus made-or-placed boundary.
- Test repeated investigation behavior, retry stability and abuse pressure at multiplayer scale.
- Design new-Place discovery only together with the Place and Movement Areas.
- Study bounded novelty and opportunity rules without server semantic inference or score mechanics.

## Components

| Component | Current meaning |
| --- | --- |
| Investigation | The Agent-selected action grounded in current Character context. |
| Attempt | World-owned technical provenance and retry-stable chance resolution. |
| Opportunity | A positive authoritative result that permits one bounded candidate. |
| Re-grounding | A fresh authoritative read before candidate authoring and confirmation. |
| Candidate | Agent-authored concrete state constrained by the opportunity. |
| Confirmation | User approval of the complete preview before World mutation. |
| Find | The accepted Entity and Activity committed to the shared World. |

## Technical model

### Delivered

The current capability exposes an investigation attempt, retry-stable World result,
one positive-opportunity token, complete Entity preview and confirmed commit with
Activity. It can establish exactly one Entity in the existing Place. Exact inputs,
states and errors remain in [`game/docs/`](../../../game/docs/README.md).

### Directional

Future discovery keeps the same responsibility split: World owns bounded mechanical
opportunity and structural validation; an explicitly invoked Agent authors meaning;
the User confirms; one transaction commits exact state and history. The selected
new-Place direction grounds that opportunity at the Character's exact Position. With
a current Place it expands the package with destination B and A→B. Without one it
also establishes separately identified origin A at the Character's unchanged point,
sets A as current and connects A→B. Broader multi-Entity expansion requires a
separate accepted contract.

### Absent

New-Place discovery, several results per attempt, server-authored content, Agent
background invocation, caller-controlled odds, a generic Discovery table and
universal novelty mechanics are absent.

## Sources

- Retained rationale and corrections — [Discovery and investigation](../../docs/concept/discovery.md).
- Exact behavior — [current game contract](../../../game/docs/README.md).
- Related synthesis — [Place](../place/README.md), [Agent Play](../agent-play/README.md) and [World Change](../world-change/README.md).
- Experiments and delivery — [`dev/lab/`](../../lab/README.md) and [`dev/docs/evidence/`](../../docs/evidence/README.md).
