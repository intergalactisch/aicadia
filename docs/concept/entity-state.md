# Entity state rationale

> **Role / side:** live concept rationale for Entity-owned Property and Trait state / development side.
> **Authority:** the Property/Trait domain distinction and uniform local authority rationale.
> **Excludes:** Current behavior and storage, which belong in `docs/game/`; delivery detail, which belongs in `docs/evidence/`.

> Delivery history and current status: see [Trait evidence](../evidence/trait.md).

## Domain distinction

A **Property** is one compact structured fact owned by one Entity: `key = value`,
such as `size = small`, `hair_colour = blond` or `leg_count = 3`. A **Trait** is an
explanatory characterizing statement such as “jumps unusually high.” Property keys
therefore do not carry explanatory prose merely to manufacture meaning. The
canonical lower-snake-case English key and its immutable value type are their shared
World meaning.

Every Property belongs to exactly one Entity and has natural identity
`(entity_id, property_key_id)`. Many Entity-owned Properties can reuse one key while
retaining independent values. Every kind of Entity can carry zero or more
Properties—furniture, flora, fauna, Characters and Places alike. Entity roles use the
same model; there is no role-specific profile table.

The first value types are bounded text and integer. A User steers and confirms
natural meaning, an Agent submits typed input, and World alone validates and writes
it without inference. Synonyms remain different keys; World does not infer aliases.
A User never receives a direct own-profile, other-profile or storage-edit command.

Trait remains a separate capability. This record owns its design rationale;
`docs/game/` owns current behavior.

## Trait capability rationale

The desired player outcome is that an Entity can retain one recognizable
characterizing statement and later develop it through accepted play. Pip might begin
as a rat that first becomes known for “startling at every hard sound” during one
accepted situation and later becomes one that “waits for the second echo before
springing.” A later Agent should be able to ground a callback in that current
characterization and its accountable earlier expression without a score, hidden
narrator or prose interpreter.


## Current behavior boundary

Current Property/Trait behavior, validation and persistence are defined by the [game domain](../game/domain.md), [capability contracts](../game/README.md#capability-contracts) and [storage contract](../game/storage.md). This concept record does not duplicate their delivery detail.

## Accepted uniform authority boundary

The accepted contract lets a local World Action change actor, current Place,
co-present ordinary Entities and other Characters uniformly. This avoids a control
oracle and enables causal multi-Entity events, but it means one User-confirmed Action
can physically change another User's Character. World validates typed, local,
outward state only; it does not infer consent or narrative plausibility.

The User explicitly accepted this uniform inclusion, including other Characters and
the Place. It remains limited to outward typed Property state and does not authorize
volition, response, consent, relationship or placement changes.

External factors use the same conceptual Activity-backed consequence pipeline, but
the executable slice supports only a User-steered, confirmed Agent-authored Action or
Interaction cause. The shared private validator/writer is a future reuse seam for a
later explicitly accepted deterministic mechanic, not a delivered external-factor
writer. It does not create an autonomous or background Agent, `world_event` table,
timer, scheduler or ungrounded simulation.
