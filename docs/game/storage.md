# Storage contract

> **Role / side:** Defines the PostgreSQL relations, migrations, locks, indexes and bounded-query invariants / runtime side.
> **Authority:** Relational realization of the current game contract.
> **Excludes:** Delivery status, rollout narrative and evidence results.

## Migration boundary

Before activity storage, every Entity with a Character role could only have resulted
from `create_character`, and every Entity without that role could only have resulted
from `create_entity`. Migration `0004_world_entry_activity.sql` backfills exactly
those derivable operation, responsible User, subject Entity and original
`introduced_at` facts. The old schema retained no acting Character or Place context,
so both remain null. No entry or placement history is fabricated.

The action migration leaves historical Activity prose, request id and fingerprint
null and assigns no historical Entity location. Current Entity location remains
ordinary authoritative state, not a replayed projection or inference from Activity.


## PostgreSQL model and indexes

```text
user(id PK, created_at)
entity(id PK, name, description, introduced_by_user_id FK user, introduced_at)
character(entity_id PK/FK entity, owner_user_id UNIQUE/FK user,
          current_place_entity_id NULL/FK place)
place(entity_id PK/FK entity, is_entry, latest_activity_id FK activity NOT NULL)
entity_location(entity_id PK/FK entity, place_entity_id FK place)
activity(id PK, operation, requested_by_user_id FK user,
         actor_character_entity_id NULL/FK character,
         context_place_entity_id NULL/FK place, prose NULL,
         request_id UUID NULL, request_fingerprint BYTEA NULL,
         action_consequence NULL, occurred_at)
activity_entity(activity_id FK activity, entity_id FK entity, role,
                PK(activity_id, entity_id, role))
property_key(id BIGINT identity PK, key UNIQUE, value_type,
             first_activity_id FK activity)
entity_property_history(entity_id FK entity, property_key_id,
                        activity_id FK activity, previous_activity_id NULL,
                        value_type, text_value NULL, integer_value NULL,
                        PK(entity_id, property_key_id, activity_id))
entity_property(entity_id FK entity, property_key_id FK property_key,
                current_activity_id,
                PK(entity_id, property_key_id))
entity_trait(id UUID PK, entity_id FK entity,
             UNIQUE(id, entity_id))
entity_trait_version(trait_id, entity_id, activity_id FK activity,
                     previous_activity_id NULL, statement,
                     PK(trait_id, activity_id))
entity_trait_current(trait_id PK, entity_id,
                     current_activity_id,
                     FK same Trait/version lineage)
investigation_attempt(id UUID PK,
                      requested_by_user_id FK user,
                      request_id UUID,
                      character_entity_id FK character,
                      place_entity_id FK place,
                      outcome,
                      consumed_by_activity_id NULL UNIQUE/FK activity,
                      voided_by_attempt_id NULL/FK investigation_attempt,
                      created_at)
```

The Property relations above are the delivered schema. `entity_property_history` is
the sole value store and is
append-only. `entity_property` contains only the current Activity pointer, protected
by a same-lineage composite foreign key. `activity.action_consequence` is null for
non-Action operations and stores `introduce_entity` or `change_entity_state` for new
Actions. Immutable historical `change_entity_property` and `change_entity_trait`
rows remain readable and retry-compatible. One Activity may own
up to 100 history rows, so `activity_id` in history is indexed but not unique.

Indexes exist only for current behavior:

- `entity(introduced_at DESC, id DESC)` serves shared Entity pagination;
- unique `character(owner_user_id)` serves contextual lookup and one-Character
  arbitration;
- partial unique `place(is_entry) WHERE is_entry` arbitrates World genesis;
- `entity_location(place_entity_id, entity_id)` serves exact-Place Entity lookup;
- partial `character(current_place_entity_id, entity_id) WHERE
  current_place_entity_id IS NOT NULL` serves exact-Place Character target lookup;
- partial `activity(actor_character_entity_id, occurred_at DESC, id DESC)` and
  `activity_entity(entity_id, activity_id)` serve personal and Place history;
- partial `activity(context_place_entity_id, occurred_at DESC, id DESC) WHERE
  context_place_entity_id IS NOT NULL` serves the bounded exact-Place chance window
  without scanning a hot Place's complete history;
- partial unique `activity(requested_by_user_id, request_id) WHERE request_id IS NOT
  NULL` serves accepted Action, Interaction and discovery retry lookup; fingerprints are
  exactly 32 bytes;
- primary-key indexes serve role joins and involved-Entity lookup;
- unique `investigation_attempt(requested_by_user_id, request_id)` serves stable
  start retry identity in its namespace;
- `investigation_attempt(requested_by_user_id, created_at DESC)` serves the inclusive
  rolling-hour admission window; and
- partial `investigation_attempt(requested_by_user_id, created_at) WHERE outcome
  = 'positive' AND consumed_by_activity_id IS NULL AND voided_by_attempt_id IS NULL`
  serves live-positive count and deterministic FIFO voiding.

The delivered Property migration adds only the unique canonical-key lookup and
`entity_property_history(activity_id, entity_id, property_key_id)` hydration index;
the composite primary keys serve current Entity/key and predecessor access. No key,
value or reverse/global search index is accepted.

Migration `0008_entity_trait.sql` delivers three Trait relations.
`entity_trait` owns only stable identity and Entity; the unique null-predecessor
`entity_trait_version` root is the sole establishing Activity provenance and every
later version is append-only; `entity_trait_current` holds only one same-lineage
pointer. Set-based reads/writes and stable Entity/Trait lock order serve current
association pages, Activity hydration, 100-item packages, branch prevention and
deadlock resistance. No knowledge, observation, relationship, external-factor or
generic event relation is introduced.
Deferred per-Trait commit checks make the three relations total without constraining
their valid insertion order inside the transaction: every stable Trait must finish
with exactly one root, exactly one current pointer and no successor after the current
version. An incomplete identity or root, current-row deletion, pointer backtracking
or successor insertion without the matching pointer advance therefore cannot
commit. Each check is bounded to the affected Trait id through the primary, partial
unique and predecessor indexes; it never scans every Trait.
Migration `0009_uniform_entity_state.sql` extends the closed Action discriminator
with `change_entity_state` while retaining immutable historical tags, and admits
Trait roots from all Entity creation routes plus typed state changes on Action and
Interaction. It adds no relation, index, universal payload or separate mutation
operation.

Short contextual mutations lock their responsible User row. Place-relevant writers
also lock the affected Place as specified above, serializing state changes at one
Place and making `place.latest_activity_id`, rather than Activity timestamp or UUID
ordering, authoritative for its latest accepted representation. This imposes no
global World lock, revision or counter. Existing Activity immutability also protects
accepted prose, request identity and fingerprint.

Migration `0006_entity_interaction.sql` creates zero tables and adds no columns. It extends the
`activity.operation` check with `submit_interaction`, generalizes the existing
prose/request provenance check to both confirmed mutation operations, extends the
`activity_entity.role` check with `target`, and adds only the partial Character-at-
Place index above. `activity` remains the Interaction identity; there is no
`interaction`, `observation`, `knowledge`, `relationship` or `signal` table.

The delivered `0007_entity_property.sql` migration adds the Action
discriminator and three Property relations above. Activity is inserted before a
first-use key and its history, so every provenance foreign key names a real accepted
Activity. One shared private writer normalizes and sorts keys and Entity/key pairs,
locks existing pointers in stable order, arbitrates first-use keys, bulk-inserts
history and bulk-upserts current pointers. Route-specific Entity, role and placement
writes remain in the same transaction. No public generic Property-write capability
or deterministic external-factor writer is delivered by this slice.

Migration `0010_investigation.sql` adds the attempt relation and three attempt
indexes above, plus the earned partial Place-Activity ordering index. Attempt outcome
is exactly `zero` or `positive`. Zero may never be consumed or voided; a positive may
be consumed or voided, never both. Attempt rows are otherwise immutable, and each
lifecycle pointer is set at most once. A check rejects
`voided_by_attempt_id = id`, so an attempt cannot be its own void provenance. There
is no response snapshot, fingerprint, counter, session, secret or generic discovery
payload.

Start locks only the responsible User. It resolves retry first, then uses one
PostgreSQL `statement_timestamp()` for both the inclusive one-hour boundary and the
new row's `created_at`. Admission caps new attempts inside that window, and chance
reads only a bounded tail of Activities at the derived Place. After insertion, only a
new positive beyond the live-positive bound voids the oldest prior live positive with
`id <> new_attempt_id`, ordered by `(created_at ASC, id ASC)`, and records the
now-existing new attempt in `voided_by_attempt_id`. Zero never voids another attempt.
Each bound is owned by [Domain contract](domain.md#investigation-chance-and-admission).
These are per-User and per-Place access paths; no global row, lock or counter exists.

The migration extends the closed `activity.operation` check and prose/request
provenance check with `submit_discovery`. It also replaces the existing
`validate_entity_trait_version_activity()` function so a Trait root may be owned by
`submit_discovery`; without that explicit validator correction a discovery with an
initial Trait would fail at commit. Accepted submit locks User then Place and, in one
transaction, inserts the Entity, placement, initial Property/Trait state, Activity
and `subject`/`location` roles, points `consumed_by_activity_id` to that Activity and
advances `place.latest_activity_id`. Rejection changes none of them.
