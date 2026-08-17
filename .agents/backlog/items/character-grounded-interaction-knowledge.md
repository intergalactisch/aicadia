# Character-grounded interaction and knowledge

> **Role / side:** forward-planning item / development side.
> **Authority:** records this outcome's backlog state, dependencies and completion pointers.
> **Excludes:** current product contracts, decision rationale and detailed delivery evidence; see `docs/game/`, `docs/concept/log/` and `docs/evidence/`.

## Outcome

At one exact current Place, a User can have their Character perform one freely
worded, explicitly confirmed Interaction involving one or more existing co-present
Entities.
World stores directional participation in immutable Activity without authoring a
directed Entity's response. Each Character later receives only the observation and
history justified by its own situation and information paths; User-control
provenance does not accidentally become in-world knowledge.

The accepted build plan is
`.agents/plans/20260813-114241-character-grounded-interaction-history/plan.md`.
The executable contract is in `docs/game/README.md` and
`docs/game/agent.md`. Confirmed design rationale is recorded in
`docs/concept/interaction.md` and `docs/concept/knowledge.md`.

The plan contains the complete research alignment for D&D rules/campaign loops,
D&D meta-storytelling and relevant non-D&D tabletop patterns. Each result is marked
as a current requirement, retained future contract, editorial lens or explicit
non-import and mapped to tasks/evidence.

## Player and World value

The earlier trail marker let Users leave shared things but not act toward another
present Character or build a remembered social history. This delivered edge makes the shared
World feel inhabited: one Character can notice, address, follow or otherwise affect
the situation around another while every participant retains independent agency and
knowledge.

The rat scenario is the acceptance stress test, not a special-case feature. One User
may play a tiny creature that repeatedly appears near another Character. The first
User can author only the creature's actions; the other User encounters observable
behavior, may initially infer an ordinary local creature and owns every response and
later recognition.

## Confirmed facts

- interaction history is many-to-many across Activity: one actor may involve several
  Entities, one Entity may occur in many actions and many actors may separately act
  toward the same Entity;
- the first Interaction is already one-to-many: one derived acting Character names
  one or more explicit, distinct, co-present directed Entities of any Entity role;
  it adds no joint authors, implicit bystanders, movement or state mutation;
- first Interaction roles are `actor`, `target` and `location`; a target Character
  can know the outward behavior but no understanding, consent or response is
  inferred, while non-target bystanders receive nothing automatically;
- later Interactions may atomically carry typed World consequences, but the first
  slice proves participation only;
- one Activity retains one accountable actor for the first slice; multi-author joint
  actions require a later proposal/confirmation contract;
- action participation, observation, Character knowledge, durable relationship and
  recap are distinct;
- `active` and `passive interaction` are not adequate domain roles;
- shared World state is not universally player-queryable;
- player mode does not disclose whether an Entity is controlled by another User;
  Users interact with Entities rather than control categories;
- global Entity inspection is removed from player mode but may remain through a
  separately authorized administrative/operator surface;
- private block/ignore state is deferred from the first slice; targets own every
  response and no Interaction triggers background Agent work or notifications, but
  repeated accepted targeting remains a known safety boundary before reach expands;
- Character-grounded interaction precedes investigation and discovery as the
  selected game-development edge;
- World remains the one deep game-behavior seam; HTTP and MCP remain thin adapters;
- no server-side Agent, background narration, score or relationship meter is added.

## Closed first-slice boundary and retained later choices

- `actor`, `target` and `location` are the conceptual Interaction roles; actor and
  location use Activity's direct foreign keys and target uses the many-Entity link;
- a target Character's access to outward behavior derives from Activity target
  participation; non-target bystanders receive nothing and no Observation table is
  added in the first slice;
- a future administrative meta-Agent is a separately authorized out-of-world
  operator and never expands player reads; and
- distant knowledge requires a later accepted causal carrier/sign reaching the
  Character, whose exact ripple mechanics remain a future plan.

The User has additionally selected structured, historical Entity Properties and
Traits as separate required living-plan capabilities. Their retained rationale is
`docs/concept/entity-state.md`. They may use separate tables and
interfaces while sharing Activity provenance. Terry treats both as follow-ons rather
than silently expanding this social-interaction build. The accepted sequence is
Interaction/knowledge, Property, Trait, then investigation/discovery.

## Dependencies

- current exact-Place Entity and Activity reads;
- current one-actor immutable Activity model and atomic `submit_action` evidence;
- accepted Character-grounded knowledge and identity direction;
- accepted target contract and completed dependency-ordered implementation tasks.

## Non-goals

- generic conversations, combat or arbitrary action grammar;
- multi-actor co-authored commits;
- autonomous NPCs, factions or server narration;
- universal observation, knowledge, relationship or interaction tables;
- global social graphs, reputation, affinity or relationship scores;
- movement, disguise mechanics, private messages or distant ripples in the first
  slice unless later accepted as necessary dependencies.
- private block, ignore, notification or attention-pressure mechanics in the first
  slice; these must be revisited before movement or broader Interaction reach;

## Observable completion

- a same-Place actor performs the selected concrete interaction toward its allowed
  directed-Entity set and World atomically stores actor, Place, prose and exact
  directional Entity participation;
- one-to-many in one action, if accepted, and many-to-one across separate actions
  return complete stable history without duplicates;
- every directed played Character's response remains absent until that User separately acts;
- actor, directed Entity, co-present witness and distant Character receive exactly their
  authorized observations and no global records, aggregates or unauthorized control
  provenance;
- retry, stale context, invalid target, changed Place and storage failure produce no
  partial Activity or participation;
- World, HTTP and MCP semantics, tool guidance, current docs, capability map and
  backlog agree.

## Completion evidence

Delivery history and current status: see [Interaction evidence](../../../docs/evidence/interaction.md).
