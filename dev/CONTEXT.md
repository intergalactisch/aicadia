# Aicadia

> **Role / side:** canonical vocabulary / development side.
> **Authority:** defines current project and domain terminology.
> **Excludes:** behavior contracts, decision history and delivery status; see `game/docs/`, `dev/docs/concept/log/` and `dev/docs/evidence/`.

Aicadia is one persistent shared game world that Users inspect and extend through
their own AI Agents.

## Language

**World**:
The single persistent shared space in which every Entity exists. World is the sole
authority that deterministically accepts or rejects game commands and creates or
changes durable state; it validates structural truth but never interprets
Agent-authored content to infer semantic meaning, causality or preferred outcomes.
_Avoid_: Universe, shard, world instance

**User**:
The durable participant to whom actions in the World can be attributed. A User may
introduce an Entity through an Agent and owns at most one Character.
_Avoid_: Player, account, character

**Agent**:
The User's LLM client, which inspects World state, reasons, proposes actions and
composes bounded commands on the User's behalf, including intended state and claimed
causal or spatial scope. It may contribute semantic judgment to an explicit
collective decision, but has no durable identity or write authority inside the World;
only World may validate and apply its proposals.
_Avoid_: User, Character, narrator

**Character**:
The User-owned role of the Entity through which player behavior enters the World. A
Character has no identity separate from that Entity. It may be unplaced or have one
complete current Place, and does not mean User, Agent, session or account.
_Avoid_: User, Agent, avatar record with a separate identity

**Place**:
The spatial role of an Entity. A Place uses the Entity's stable identity; it is not a
coordinate, geometry, container or second Place id. The current World has at most one
entry Place.
_Avoid_: Location id, scene, node, coordinates

**Place neighborhood**:
A bounded view of explicit spatial relationships around one exact Place, such as
containing and adjacent Places. It is not a metric radius, geometry, prose inference
or automatic visibility.
_Avoid_: Local context object, coordinate radius, everything nearby

**Entity placement**:
The optional current Place at which an ordinary Entity is established. It describes
the Entity, not the acting Character: the target Place may differ from the Character's
current Place and from Activity context Place.
_Avoid_: Actor location, Activity context, ownership, discovery

**Action**:
An accepted Character-led operation whose primary game meaning is a typed World-
state consequence. It is distinct from an Interaction, whose primary meaning is
directed involvement between Entities, and from the Activity that remembers either.
_Avoid_: Interaction, Activity, arbitrary command

**Investigation**:
One explicit Agent request for World to admit and resolve whether the entered
Character can find something at its exact current Place. World derives the context
and rolls before the Agent authors content; the User supplies no mechanical focus,
seed, odds or result count.
_Avoid_: Search query, User-selected roll, Agent inference, discovery commit

**Investigation attempt**:
Durable internal provenance for one admitted investigation, identified by one
World-assigned attempt id and storing its Character, Place, zero/positive outcome
and optional consumed/voided lifecycle. It is not Activity, a session, a listable
opportunity or player inventory.
_Avoid_: Roll token, proposal, quest, pending find, conversation

**Discovery**:
The accepted gameplay outcome in which one positive investigation becomes one
found Entity at the attempt's Place after Agent authorship and User confirmation.
Discovery is not a universal record or Entity kind; its durable state is the
ordinary Entity and initial state, `submit_discovery` Activity and consumed attempt
provenance.
_Avoid_: Entity introduction for something made, generic Discovery object, loot

**Activity**:
Immutable normalized history of one accepted state-changing game operation. Activity
records operation, responsible User internally, optional actor Character, optional
context Place, occurrence time and involved Entity ids with server-owned roles. It
does not replace current state or mean transport log, conversation, private Agent
reasoning or generic event payload.
_Avoid_: Event sourcing, transcript, audit blob, score

**Participation**:
The explicit, immutable way an Entity is involved in one accepted Activity.
Participation is event-specific and directional; it does not by itself mean that the
Entity observed, understood, consented to or formed a durable relationship through
the action.
_Avoid_: Passive interaction, generic link, relationship

**Observation**:
World information a Character acquires through one situated encounter, sensory
situation or other accepted observation path. Observation is distinct from merely
participating in an Activity and from everything the World knows.
_Avoid_: Visibility flag, global state, participation

**Knowledge**:
World information a Character can justifiably use because it was observed,
remembered, personally experienced or received through an accepted transmission or
ripple. Knowledge can be partial, reported or stale and is never identical to all
authoritative World state.
_Avoid_: Agent memory, global query access, omniscience

**Control provenance**:
The private operational association between a User and their Character Entity. It
is not an in-world trait, Character knowledge or an ordinary player-facing Entity
fact.
_Avoid_: Player aura, NPC label, control identity

**Interaction**:
A distinct accepted act from one Entity toward one or more existing Entities. It
creates directional participation and a response opportunity, while Activity retains
its history. It may carry independently validated typed World consequences, but
directed involvement—not state mutation—is its defining meaning.
_Avoid_: Action, passive interaction, conversation transcript

**Interaction target**:
An existing Entity toward which an actor explicitly directs an Interaction. Target
does not imply harm, consent or response; for a Character it does guarantee access
to the Interaction's outward behavior.
_Avoid_: Counterpart, actee, recipient, observer

**Entity interaction history**:
A Character-knowable lens over immutable Activity showing how one Entity has acted
toward, participated with or encountered other Entities over time. It is directional
and may differ by Character; it is not a mutable social graph or omniscient dossier.
_Avoid_: Relationship score, complete World graph, transcript

**Property**:
A structured descriptive fact about an Entity expressed as one key and one value,
such as `size = small`, `hair colour = blond` or `leg count = 3`.
_Avoid_: Characteristic, quality, Trait, RPG attribute

**Property key**:
The reusable World vocabulary identity and allowed value type shared by many
Entity-owned Properties. It is not itself a Property or value; one Entity's Property
has the natural identity `(Entity, Property key)`.
_Avoid_: Property definition, Property value, Trait, per-Entity field

**Trait**:
A non-executable Entity-owned statement that characterizes the Entity, such as
“jumps unusually high.” Establishment gives it one stable Trait identity and one
immutable Activity-backed statement version; development appends a predecessor-
linked version and advances its current pointer. It is not reducible to one Property
key/value, Relationship state, observer-specific Knowledge or a mechanic. An Agent
authors its first statement in a confirmed Entity creation, Action or Interaction
and may develop it through a confirmed Action or Interaction; a User has no direct
Trait editor.
_Avoid_: Property, status, score

**Prose**:
The immutable human- and Agent-readable narrative of one accepted World action. Later
actions append new prose; they never edit or delete earlier prose. Every World,
Character, Place or Entity history lens refers to the same accepted prose record and
orders it by World acceptance. An Agent cannot backdate or insert prose into earlier
history. Prose is not current state or private workshop text.
_Avoid_: Story record, mutable summary, conversation, structured consequence

**Entity**:
One thing or concept that needs a stable identity so participants can refer to the
same subject again. A word, substance, amount, property or incidental detail is not
an Entity merely because it appears in a description.
_Avoid_: Object, item, record

**Public-facing text**:
Text the repository publishes verbatim to a party outside it: the served play
contract, every tool description and every schema description. It is product surface
that every Agent pays for in context and comprehension, written once per rule at one
layer under `dev/docs/methodology/public-text.md`. Builder documentation, Studio pages
and repository records are not public-facing text.
_Avoid_: Prompt, system prompt, docs, copy

**Build-facing text**:
Every Markdown document a building Agent reads before it changes code, schema,
contract or records: `AGENTS.md`, `CLAUDE.md`, this vocabulary, `game/docs/`,
`dev/docs/`, the backlog, plans, lab and skills. Each file states what it owns and
routes the reader to what it does not; every reference is a sentence naming the
fact, its relation and its owner, under `dev/docs/methodology/build-text.md`. It is
never published to a playing Agent.
_Avoid_: Internal docs, wiki, comments

**Introduction**:
The act in which an Agent submits an Entity candidate on a User's behalf and World
accepts it, gives it stable identity and stores it in the shared World. A rejected or
unsubmitted candidate is not an Entity. Introduction does not mean that the User or
Agent created the thing in the fiction, owns it or discovered it.
_Avoid_: Spawning, generation, ownership, discovery
