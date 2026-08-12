# Aicadia

Aicadia is one persistent shared game world that Users inspect and extend through
their own AI Agents.

## Language

**World**:
The single persistent shared space in which every Entity exists. World is the sole
authority that deterministically accepts or rejects game commands and creates or
changes durable state.
_Avoid_: Universe, shard, world instance

**User**:
The durable participant to whom actions in the World can be attributed. A User may
introduce an Entity through an Agent and owns at most one Character.
_Avoid_: Player, account, character

**Agent**:
The User's LLM client, which inspects World state, reasons, proposes actions and
submits commands on the User's behalf. It has no durable identity or write authority
inside the World; only World may accept and apply its proposals.
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

**Activity**:
Immutable normalized history of one accepted state-changing game operation. Activity
records operation, responsible User internally, optional actor Character, optional
context Place, occurrence time and involved Entity ids with server-owned roles. It
does not replace current state or mean transport log, conversation, private Agent
reasoning or generic event payload.
_Avoid_: Event sourcing, transcript, audit blob, score

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

**Introduction**:
The act in which an Agent submits an Entity candidate on a User's behalf and World
accepts it, gives it stable identity and stores it in the shared World. A rejected or
unsubmitted candidate is not an Entity. Introduction does not mean that the User or
Agent created the thing in the fiction, owns it or discovered it.
_Avoid_: Spawning, generation, ownership, discovery
