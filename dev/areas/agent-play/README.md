# Agent Play

> **Role / side:** current Agent Play development synthesis / development side.
> **Authority:** owns the current meaning, boundary, decisions, unresolved landscape, components and directional technical model for playing through a User-owned Agent.
> **Excludes:** selected work, exact Agent conduct and capability text, host-specific findings, experiments and delivery claims; those remain in `dev/backlog/` and plans, `game/docs/agent.md`, `dev/docs/research/`, `dev/lab/` and `dev/docs/evidence/`.

## Meaning

Agent Play is the way a User experiences Aicadia through an explicitly invoked AI
Agent connected over MCP. The Agent reads authoritative World facts, conducts a
grounded in-world conversation, uses its own intelligence to compose bounded
proposals and asks the User to confirm every complete shared or consequential
World-changing package. During one explicitly User-invoked in-World turn it may store
its own Character's private Observation batch without a second confirmation and
immediately tells the User what it retained.

## Boundary

### This is

- User-owned intelligence operating through the same public capabilities as every other Agent.
- An in-world conversation grounded only in authoritative Aicadia MCP reads.
- Explicit preview and confirmation before each mutation except the selected narrow
  private Observation write inside one active User invocation.
- Provider-neutral Agent conduct with deterministic World validation.

### This is not

- A browser chat client, server narrator or server-owned Agent runtime.
- Background Agent invocation, durable World session or unconscious token spend.
- A fallback to source code, HTTP, database, logs or remembered state when MCP is unavailable.
- Exposure of identifiers, schema fields, protocol work or development status in player conversation.

## Decisions

### Chosen

- Aicadia MCP is the sole live-game authority available to a conforming player Agent.
- The User supplies the Agent and pays for its intelligence; World remains dumb and strict.
- Player conversation renders named people, places, things, events and affordances rather than internal types.
- Every shared or externally consequential mutation requires a complete preview and
  explicit User confirmation; only the selected private Observation batch inside one
  explicit active User invocation is exempt.
- Capabilities are provider- and model-neutral and have semantic parity across World, HTTP and MCP.
- If MCP authority is unavailable, the Agent fails closed before mutation.
- An Agent must be able to express open, nuanced and quantitative relationships
  between Entities without every possible phrase becoming a World-owned enum.
- The Agent may address one stable Relation between a source and target Entity,
  author its free name and description and later develop that same Relation. Several
  Relations may coexist between the same pair; wording never becomes mechanics.
- A Relation such as “activates remotely” may give an eligible Agent meaningful
  causal context for Button B and Bomb X. It never executes or authorizes the effect;
  the Agent must still preview one exact bounded Action, obtain User confirmation
  and submit every intended state change under ordinary World authority and freshness.
- An Agent treats `Connection` as the dedicated exact direct-travel fact between
  Places. Open Relation wording may describe a road or passage but never substitutes
  for Connection or authorizes movement.
- An Agent may propose the Place role for any positioned Entity when the User wants
  it to become an independent map, discovery or navigation reference. It never
  assumes that Position, prose or a semantic kind already made the Entity a Place.
- For map context, the Agent requests one exact bounded World window. World first
  restricts candidates to Places that the acting Character can deterministically
  know, then filters by resolved Position or known Area intersection and returns
  bounded eligible spatial state plus continuation. Matching coordinates never give
  the Agent an omniscient list of every established Place or positioned Entity. One
  sparse current association remembers each known Character–Place pair; the Agent,
  User and a read itself never own or create that Knowledge.
- For every positioned Entity, the Agent grounds either an absolute Position from
  permanent World origin or a Position relative to exactly one Entity. Choosing the
  latter explicitly means mechanical following; free wording and Relations never
  make that choice implicitly.
- Optional Position `description` helps an Agent tell the current spatial story—for
  example, “this cup strangely remains exactly two centimetres above the table;
  nobody knows why.” It may narrate the exact structure but never replaces it or
  causes World behavior.
- The one current Position description may contain several sentences or paragraphs.
  The Agent uses the Entity's multiple Traits for durable independent characteristics
  rather than creating Traits or separately versioned descriptions on Position.
- Whenever an Agent proposes a Position change, its complete preview explicitly says
  whether the current description is kept, replaced or removed. The Agent makes the
  semantic judgment; World only settles that choice with the Position change.
- An Agent receives Position description only as part of the complete Position. It
  can never query, count or discover that description independently; one current
  Place-read path is accepted while other Entity-selection paths remain open.
- When a bounded current Place read already returns an Entity, the Agent receives its
  complete Position in that same response when one exists. It never performs a
  Position lookup per returned Entity, and the spatial foundation performs no
  Position-specific redaction.
- Knowing an Entity or remembering its earlier Position never gives an Agent its
  exact current Position. The Agent may present the remembered point explicitly as
  stale Knowledge, but it cannot use that memory as fresh mutation grounding or
  probe Position by guessing the Entity identity.
- An Agent normally receives its own Character's resolved Position or knows that it
  has none. The spatial foundation contains no Position-only exception that changes
  this structured response.
- The Agent may freely author the Trait or Position description that narrates an
  unusual effect within ordinary text bounds. It may use that meaning to withhold a
  detail in its own player conversation, but this is Agent conduct rather than World-
  enforced confidentiality; World still returned the structured Position.
- The Agent enforces the creative boundary that a User writes only for their own
  Character. If asked to author perception, knowledge or another consequence for a
  different User's Character, it refuses and explains that boundary. World never
  parses the request or resulting prose to decide whose experience it narrates.
- An Agent acts through its Character and may in principle propose changes to
  ordinary World state first authored by any other Agent. It may author surprising
  multi-Entity consequences, but must name them explicitly rather than relying on
  World to understand the story.
- The Agent receives only Relations and endpoints its current Character can know.
  A hidden inventory Relation belonging to another Character is absent from its
  grounding, and guessing an identifier grants neither knowledge nor mutation access.
- The Agent may reason from its Character's remembered prior observation, clearly as
  memory rather than current truth. It may propose a grounded investigation of what
  is currently knowable, but cannot directly mutate a merely remembered hidden Entity.
- An explicit accepted in-World Observation may preserve one bounded free-text note
  authored for the observing Character, such as “this time he wore a red hat.” The
  note never creates the hat, equips it or changes the observed Entity merely because
  the Agent wrote it. It is private attributed Character memory, may be mistaken and
  is never returned as shared World fact through the observed Entity.
- Observing something at a distance never automatically means recognizing or knowing
  its identity. The Agent may record “a dark figure on the ridge” without receiving
  a hidden Character's name or current Entity state. World still retains the exact
  existing model reference on the Observation; Knowledge design is parked.
- The Agent selects an Observation subject through the explicit stable `entity`,
  `place` or `connection` model alias. It never invents a table/class name or treats
  Position, Property or Trait as an independent observed subject.
- World tools return bounded structurally authorized context; the Agent uses its own
  judgment to decide what its Character could see or notice and explicitly shapes
  the resulting Observations. World creates no visibility candidate list or proof.
- One Observation action may include several occurrences. Each has one exact model
  and its own optional private account; the bounded batch and its own Activity settle
  atomically and idempotently. It may follow Movement, but recording failure leaves
  the Character moved and Movement is not a required server receipt.
- Hidden or otherwise unauthorized content remains absent from the Agent's tool
  context. Guessing an identifier grants no current read or mutation, while World
  never interprets Observation prose to decide semantic sight or attention.
- World validates only observing-Character control, admitted target alias, target
  existence, request and text bounds, idempotency and atomic Activity. It checks no
  spatial plausibility or truth and returns no current target fields or authority
  through the Observation write.
- During an explicitly User-invoked in-World turn, the Agent may store one bounded
  private Observation batch for its own Character without separate confirmation and
  immediately tells the User what it retained. This never authorizes shared mutation,
  another Character's memory or delayed/background work.
- A subscription may notify an active client of World change but, under the BYO Agent
  model, can never invoke the Agent, call Observation or spend User tokens.
- The Agent composes Relation meaning, optional exact Position, movement behavior,
  visibility and intended action explicitly; it never receives one generic
  Containment operation that silently decides those concerns.
- For new-Place discovery, the Agent reads fresh structured Position context and
  calculates and submits the exact three-dimensional absolute point itself as whole
  centimetre `x`, `y` and `z`, where `z` is height. Before asking for confirmation it
  must inspect a bounded eligible set of established Places around that point and
  include the returned nearby
  context in the complete preview. World returns structural facts and numeric
  proximity but never interprets the Agent's spatial wording, and omitted hidden or
  out-of-scope state is never presented as proof of global absence.
- The first slice lets the Agent propose any technically valid absolute World point,
  regardless of distance from A. It must make that distance intelligible in the
  User conversation and still complete nearby inspection; World supplies no hidden
  gameplay radius for the Agent to satisfy.
- The Agent may converse in centimetres, metres or kilometres but converts the
  confirmed point to whole centimetres before submission. World receives no free-
  text unit to interpret, and sub-centimetre precision is not promised.
- The Agent also converts compass wording using increasing `x` for east, `y` for
  north and `z` for up. It presents natural directions to the User while submitting
  only the resulting structured Position.
- Absolute Position reads and proposals share permanent World origin `(0, 0, 0)`.
  The Agent never presents that convention as an Entity, Place or discoverable
  object in player conversation.
- A successfully resolved Position read gives the Agent both the current World
  point and the stored basis. For an Entity-relative Position that includes the
  immediate reference Entity and stored offsets, so the Agent knows both where the
  subject is now and why it mechanically follows another Entity.
- If bounded resolution cannot reach an absolute basis, the Agent receives the
  immediate reference and offsets without a World point. It must present that limit
  honestly and cannot propose an Action whose mechanics require the missing point;
  it may instead simplify or re-establish the Position through an eligible Action.
- The Agent may propose and narrate impossible architecture through explicit
  Connections, concrete Actions and authored meaning, including travel loops whose
  geography makes no ordinary sense. It never creates that experience by proposing
  a cyclic Position reference or asking World to infer mechanics from the story.
- A proposed relative Position succeeds only when World can prove within bounded
  work that it reaches an absolute basis and stays acyclic. On an unproven or stale
  chain the Agent receives a rejection and may offer an absolute Position or a
  simpler eligible reference; it never presents the rejected placement as World truth.
- On every reference change the Agent proposes the complete new reference and all
  three offsets; it never asks World to preserve a prior point implicitly. Before
  preview it considers eligible current Traits, including a cup's “always floats two
  centimetres above a surface.” If the new surface point is not grounded, the Agent
  explains the uncertainty and asks rather than inventing geometry. Position
  description may then narrate the resulting current placement.
- The User controls what their Character attempts, but an instruction is not a
  guaranteed literal outcome. The Agent reasons about current World meaning and may
  warn, comply or invent a bounded surprising consequence: forcing the two-centimetre
  cup down might fail and leave it floating ten centimetres above the table. It must
  expose the exact resulting Position, Trait and description choices rather than
  hiding invention inside prose.
- Deliberately unforeseen Agent-authored outcomes are desired future game value, but
  never ordinary default behavior. `Chaos` is only the User's current working label;
  no canonical game term or capability has been chosen.
- The spatial foundation is finished before a separate grill designs that wider
  unforeseen-result direction. Spatial keeps SP04 as pressure but gains no interim
  surprise mode or extra mechanic.
- When nearby context contains a plausible existing Place, the Agent must use its
  conversation with the User to offer that Place first and explain why a new Place
  may still be distinct. It asks for final new-Place confirmation only after the
  User has explicitly chosen to continue; World never evaluates that semantic case.
- If the User chooses the existing Place, the Agent previews an explicit directed
  Connection from the origin to that Place instead of a new Place package. The
  Agent must present this as a deliberate travel fact, never as an automatic result
  of numerical proximity.
- If that travel direction is already established or another request wins it, the
  Agent receives no new mutation and retains the opportunity. It must re-ground and
  discuss another candidate rather than presenting a duplicate Connection as a new
  discovery; an exact accepted-request retry simply resumes the prior result.

### Rejected

- Browser gameplay or an Aicadia-hosted narrator in the current product boundary.
- Provider, model, client or tool allowlists as a substitute for semantic capability contracts.
- Server-side inference, localization or LLM use to interpret Agent-authored content.
- Quiet fallback to repository, HTTP, database, logs or memory during player interaction.
- Durable conversational session state inside World.
- Requiring a closed server vocabulary to contain every relationship an Agent may describe.
- Expecting Trait wording such as “hidden from its possessor” to execute a privacy
  consequence without an explicit structural change in the confirmed proposal.
- Giving an Agent a standalone visibility editor or generic World-change operation.
- Presenting Relation meaning as a World-enforced remote control, rule or permission.
- Letting an Agent author perception, knowledge or another current experience for a
  different User's Character merely because its User requested a convincing story.
- Giving the original Agent permanent exclusive edit authority over ordinary World content.
- Supplying hidden Relations to an Agent and relying on its prompt to conceal them.
- Asking World to turn prose such as “one kilometre east” into Position or to decide
  that nearby Places are semantically identical.
- Asking the User to confirm a new Place before the Agent has inspected and
  presented the bounded nearby Place context available to its Character.
- Silently ignoring a plausible nearby Place or deciding on the User's behalf
  whether it should be reused.

### Not yet chosen

- Hosted authentication and OAuth for remote Agent hosts.
- The minimum supported host capabilities for subscriptions, reconnect and confirmation UX.
- How long-running Agent work presents changing World context without stale proposals.
- Which context-compaction guidance belongs in the public Agent contract as capabilities grow.
- How shared or delegated User control could safely invoke one Character through several Agents.
- How one confirmed proposal presents exact Position, its optional description
  and open Relation meaning without presenting authored wording as mechanics.
- Whether the User's initial request authorizes the Agent to commit its bounded
  creative result, or whether every changed result still requires a fresh explicit
  preview and confirmation.
- After the spatial foundation, how one User explicitly invokes unforeseen-result
  play for one bounded attempt without turning it into a global mode, background
  process or default Agent habit.
- Which concrete bounded World reads may select an Entity beyond the current Place;
  each such read returns the selected Entity's complete Position when one exists.
- How a later privacy design decides whether an Entity or private Relation is
  returned at all, without reintroducing Position-specific denial or redaction.
- How a later privacy grill replaces best-effort Agent withholding with actual World-
  enforced information boundaries when concrete gameplay requires them.

## Research needed

- Exercise the public contract with diverse current MCP hosts without provider-specific branches.
- Measure Agent comprehension and token cost as capability and context surfaces grow.
- Test confirmation, stale-context recovery and reconnect behavior in real host conversations.
- Verify which transports can support optional multiplayer hints without becoming required truth.

## Components

| Component | Current meaning |
| --- | --- |
| User | The human who chooses and explicitly invokes an Agent and confirms mutation. |
| Agent | User-owned intelligence that reads, reasons, presents and proposes. |
| Character | The durable player subject through which the Agent acts in World. |
| MCP contract | The sole live-game capability and authority surface for Agent play. |
| Grounding | Fresh bounded World reads used before explanation or proposal. |
| Preview | The complete player-readable package shown before mutation. |
| Confirmation | Explicit User authorization for that exact package. |
| Authored relation | Open Agent-supplied meaning between exact World subjects; it grants no mechanic merely by its wording. |
| Scenarios | The Agent expression, grounding and privacy cases in the [spatial scenario catalogue](../place/scenarios.md). |

## Technical model

### Delivered

One compiled Agent contract and MCP capability catalogue expose the current World
reads and confirmed mutations through the same `World` semantics as HTTP. The
current local Studio is read-only development context, not player conversation.
Exact conduct and capabilities remain in [`game/docs/agent.md`](../../../game/docs/agent.md).

### Directional

The capability surface stays compact, semantic and provider-neutral. An Agent reads
fresh authoritative context, composes one bounded proposal, obtains explicit User
confirmation and submits it; World validates without retaining conversational state
or invoking the Agent later. Future relationship authorship may remain semantically
open while the proposal separately names any exact Position, dependency, authority
or persistent constraint that World can validate structurally.

### Absent

Hosted auth, OAuth, browser chat, server Agents, background invocation, durable
domain sessions, provider branches, automatic token spending and an open Relation
capability are absent from the current contract.

## Sources

- Prepared pressure — [Spatial scenario catalogue](../place/scenarios.md).
- Exact Agent conduct — [`game/docs/agent.md`](../../../game/docs/agent.md).
- Exact capabilities — [`game/docs/`](../../../game/docs/README.md) and the generated public catalogue it governs.
- Related synthesis — [Multiplayer](../multiplayer/README.md), [Discovery](../discovery/README.md) and [World Change](../world-change/README.md).
- Research, experiments and delivery — [`dev/docs/research/`](../../docs/research/README.md), [`dev/lab/`](../../lab/README.md) and [`dev/docs/evidence/`](../../docs/evidence/README.md).
