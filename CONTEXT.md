# Aicadia

Aicadia is one persistent shared game world that Users inspect and extend through
their own AI Agents.

## Language

**World**:
The single persistent shared space in which every Entity exists.
_Avoid_: Universe, shard, world instance

**User**:
The durable participant to whom actions in the World can be attributed. A User may
introduce an Entity through an Agent and owns at most one Character.
_Avoid_: Player, account, character

**Agent**:
The User's LLM client, which proposes candidate Entities and acts on the User's
behalf but has no durable identity inside the World.
_Avoid_: User, Character, narrator

**Character**:
The User-owned role of the Entity through which later player behavior enters the
World. A Character has no identity separate from that Entity and does not mean User,
Agent, session or account.
_Avoid_: User, Agent, avatar record with a separate identity

**Entity**:
One thing or concept that needs a stable identity so participants can refer to the
same subject again. A word, substance, amount, property or incidental detail is not
an Entity merely because it appears in a description.
_Avoid_: Object, item, record

**Introduction**:
The act by which a User gives one accepted Entity candidate a stable identity in the
shared World. Introduction does not mean that the User created the thing in the
fiction, owns it or discovered it.
_Avoid_: Spawning, generation, ownership, discovery
