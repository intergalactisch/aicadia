---
status: retained
---

# Interaction participation rationale

> **Role / side:** live concept rationale for asymmetric Entity participation / development side.
> **Authority:** why Activity, observation, knowledge, relationship and recap remain distinct
> **Excludes:** Delivered Action/Interaction behavior and storage, which live in the current game contract; delivery evidence.

## Entity interaction history and asymmetric participation

A rich shared World needs to remember how Entities have affected one another, not
only that they exist at a Place. That history is naturally many-to-many across time:

- one Character action may involve one or several other Entities;
- one Entity may participate in many accepted actions with many different Entities;
- many Characters may separately act toward the same Entity;
- two Entities may accumulate actions in both directions; and
- different Characters may observe different subsets or aspects of the same accepted
  action.

Action and Interaction are now distinct capabilities. An Action's primary game
meaning is a typed World-state consequence; an Interaction's primary meaning is an
act from one existing Entity toward one or more other existing Entities. Interaction
therefore earns its own `World` operation, directed-Entity validation, direction and
safety contract. Both still leave immutable Activity and remain under the same
`World` authority; this is not a second interaction service. A later Interaction may
also carry independently validated typed state consequences in that same atomic
Activity, but free prose never mutates state by itself.
`Signal` is only a prose example—speaking, squeaking, gesturing and circling feet are
possible Interaction expressions, not separate systems or flags.

Current delivered Action and Interaction semantics, cardinality, target roles, contextual reads and Activity storage live in [the capability contracts](../game/README.md#capability-contracts). This record does not restate them.

`Active` and `passive interaction` are rejected as canonical roles because they
collapse different facts. For one accepted action an Entity may instead be:

- the accountable actor who intentionally performed it;
- an explicit Interaction target toward which the actor directed the behavior;
- a co-present potential observer who was not part of the action;
- an actual observer who acquired only the facts available from its situation; or
- entirely unaware despite being mentioned, affected later or technically related.

The exact participation vocabulary must be earned by concrete actions. A universal
bag of roles would merely relocate prose ambiguity into enums. Observation likewise
must not generally be inferred from participation. The first Interaction makes one
narrow exception deliberately: a target Character can know the outward behavior.
Understanding and response remain unproven, while a future witness can observe
without being a target.

History, knowledge and relationship remain separate:

1. **Activity** records what World accepted, with actor, Place, time, canonical prose
   and explicit Entity participation.
2. **Observation** states what a particular Character could acquire from a situated
   event or state.
3. **Knowledge** is the Character-grounded information later available through
   observation, memory, own involvement, transmission or ripple.
4. **Relationship** would be durable current state between Entities only when a
   future behavior needs it; repeated interaction does not automatically create a
   friendship, rivalry, trust level or score.
5. **Recap and interaction history** are derived, Character-scoped lenses over those
   authorities, never a second canon or global dossier.

Applied to the rat case, one action may store the rat Character as actor and Mara as
an addressed or affected Entity at their shared Place. The rat remembers its
own intent. Mara may observe only a small creature darting around her feet. A third
Character behind a closed door may learn nothing. If Mara later speaks, steps aside
or leaves food, that is Mara's separately authored Activity. Neither direction
automatically establishes what the other thought, and neither User can demand the
other Character's private knowledge.

## Open decisions

A genuine joint action with multiple authors would require its own proposal, confirmation, concurrency and partial-decline contract and remains unearned. A later witness/sensory capability may earn explicit Observation evidence. Relationship becomes durable current state only when a future behavior requires it.
