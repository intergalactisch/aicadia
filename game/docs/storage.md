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

Migration `0011_spatial_exploration.sql` adds direct Position and immutable
Connection only after a fail-closed preflight proves every required old spatial fact
has one establishing Activity. It backfills in this order:

1. the sole old entry Place receives `(0, 0, 0)` under its original
   `create_entry_place` Activity;
2. each Character with a current Place receives that Place point under its own one
   `enter_world` Activity;
3. each old `entity_location` subject receives its Place point under the one
   `submit_action.introduce_entity` or `submit_discovery` Activity that established
   that exact Entity/location pair; and
4. an unentered Character or Entity without exact spatial establishment remains
   without Position.

The migration fails before schema acceptance when a required subject has zero or
several qualifying Activities, when an old current Place lacks Position or when an
old location disagrees with its Activity context. It creates no synthetic Activity,
guesses no coordinate from prose, rewrites no timestamp and imports no lab row.
After backfill it validates the required Place-to-Position foreign key, builds the
Place map projection from canonical current Position and leaves every old Activity
immutable. Each backfilled Position version receives one `activity_position` result
association to its already-existing establishing Activity; no historical origin
association is fabricated.


## PostgreSQL model and indexes

```text
user(id PK, created_at)
entity(id PK, name, description, introduced_by_user_id FK user, introduced_at)
character(entity_id PK/FK entity, owner_user_id UNIQUE/FK user,
          current_place_entity_id NULL/FK place)
position_version(entity_id FK entity, activity_id FK activity,
                 previous_activity_id NULL,
                 x_cm, y_cm, z_cm, description NULL,
                 PK(entity_id, activity_id), FK previous same Entity)
position(entity_id PK/FK entity, current_activity_id,
         FK current same Entity/version)
place(entity_id PK/FK entity/FK position, is_entry,
      latest_activity_id FK activity NOT NULL)
entity_location(entity_id PK/FK entity, place_entity_id FK place)
activity(id PK, operation, requested_by_user_id FK user,
         actor_character_entity_id NULL/FK character,
         context_place_entity_id NULL/FK place, prose NULL,
         request_id UUID NULL, request_fingerprint BYTEA NULL,
         action_consequence NULL, occurred_at)
activity_entity(activity_id FK activity, entity_id FK entity, role,
                PK(activity_id, entity_id, role))
activity_position(activity_id FK activity, role,
                  position_entity_id, position_activity_id,
                  PK(activity_id, role, position_entity_id,
                     position_activity_id),
                  FK Position version)
connection(id PK, source_place_entity_id FK place,
           destination_place_entity_id FK place,
           source_position_activity_id,
           destination_position_activity_id,
           allows_reverse, has_course, name, description, shape_description NULL,
           created_by_activity_id UNIQUE/FK activity,
           FK both endpoint Position versions)
connection_point(connection_id FK connection, ordinal,
                 x_cm, y_cm, z_cm,
                 PK(connection_id, ordinal))
activity_connection(activity_id FK activity, connection_id FK connection,
                    PK(activity_id))
place_map_index(place_entity_id PK/FK place,
                position_activity_id, x_cm, y_cm, z_cm,
                FK Place Position version)
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
                      kind,
                      position_activity_id,
                      place_entity_id NULL/FK place,
                      outcome,
                      consumed_by_activity_id NULL UNIQUE/FK activity,
                      voided_by_attempt_id NULL/FK investigation_attempt,
                      created_at,
                      FK Character Position version)
```

`position_version` is append-only. Partial unique indexes enforce one root and one
successor per prior version. A non-root insertion must name that Entity's indexed
current pointer as predecessor. One deferred local check on each new version requires
its exact typed result and the current pointer to name that new version at commit;
it never scans the Entity's lineage. Updates may only advance `position` from its
current tip to the one direct successor. Deleting Position state, incomplete roots,
backtracking, branching, cycles, updating a version or advancing a Place Position is
rejected.

Connection and point rows reject update and delete. Endpoint Places must differ and
the stored endpoint Position revisions must be current at creation. Immutable
`has_course` fixes whether the Connection was admitted with a course. Deferred
per-Connection checks require exactly zero points when false, or contiguous ordinals
`0..n-1` with `2 <= n <= 128` when true, exact first/last endpoint points, distinct
consecutive points, adjacent segments meeting only at their shared endpoint and no
non-adjacent segment intersection. No endpoint, direction, text or geometry unique
index exists. `created_by_activity_id` is unique because one S1 discovery Activity
creates exactly one Connection.

`activity_position` accepts only `origin` and `result`; all its rows and
`activity_connection` rows are immutable. Each result Position must name the same
Activity as its version. Each Activity can name at most one Connection. Only a
Discovery may name exactly the Connection it created, and only Movement may otherwise
name one traversed Connection.

`place_map_index` is synchronous rebuildable candidate state, not a second Position
truth. Place insertion writes its one row in the same transaction. Reads exact-check
the indexed `(place_entity_id, position_activity_id, coordinates)` against `place`,
`position` and `position_version` before hydration. Rebuild truncates and repopulates
only from that canonical join and creates no Activity.

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
- unique Position root/successor indexes and the Position primary/current-pointer
  indexes serve one-Entity lineage validation and current read;
- three covering Place-map indexes lead respectively with X, Y and Z:
  `(x_cm, y_cm, z_cm, place_entity_id)`,
  `(y_cm, z_cm, x_cm, place_entity_id)` and
  `(z_cm, x_cm, y_cm, place_entity_id)`, each including
  `position_activity_id`. PostgreSQL may choose the useful leading axis while the
  query preserves canonical `(x_cm, y_cm, z_cm, place_entity_id)` ordering and
  continuation;
- `connection(source_place_entity_id, id)` and
  `connection(destination_place_entity_id, id)` serve incident paging without course
  hydration; `connection_point` primary key serves one selected ordered course;
- `activity_position(position_entity_id, position_activity_id, activity_id)` and
  `activity_connection(connection_id, activity_id)` serve typed history hydration;
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
  start retry identity and stored-kind comparison in its namespace;
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
history and bulk-upserts current pointers. Capability-specific Entity, role and placement
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
Each bound is owned by [Investigation attempt](model/investigation-attempt/README.md#investigation-chance-and-admission).
These are per-User and per-Place access paths; no global row, lock or counter exists.

The migration extends the closed `activity.operation` check and prose/request
provenance check with `submit_discovery`. It also replaces the existing
`validate_entity_trait_version_activity()` function so a Trait root may be owned by
`submit_discovery`; without that explicit validator correction a discovery with an
initial Trait would fail at commit. Accepted submit locks User then Place and, in one
transaction, inserts the Entity, placement, initial Property/Trait state, Activity
and `subject`/`location` roles, points `consumed_by_activity_id` to that Activity and
advances `place.latest_activity_id`. Rejection changes none of them.

## S1 transaction and query admission

New spatial reads use one short read-only Repeatable Read transaction with a local
three-second `statement_timeout`. New spatial mutations normalize and bound all
input, resolve accepted request identity, set the same statement timeout and a local
500-millisecond `lock_timeout`, then lock in this order:

1. responsible User;
2. controlled Character;
3. existing affected Entity rows in ascending UUID-byte order; and
4. the exact attempt and current pointers needed by that capability.

Connection creation never locks or arbitrates an endpoint pair. Movement reads the
immutable Connection and locks no traveller, course or Place-wide coordinator. A
timeout maps to retryable `temporarily_unavailable` and the complete transaction
writes nothing. An unrelated User, Character, Place or Connection shares no new S1
lock.

For an unseen mutation request, World rechecks Position revision, nullable current
Place, attempt and selected Place/Connection eligibility after locks; inserts
prerequisite Entity rows; appends Activity; inserts Position, Place, Connection and
typed history; consumes the attempt when applicable; and commits once. Discovery
does not advance the old broad `place.latest_activity_id` for Connection
establishment. `entity_at_position` retains its existing exact-Place pointer advance
when it has a current Place. Movement neither reads nor advances that pointer.

The shared partial unique `(requested_by_user_id, request_id)` Activity index covers
Action, Interaction, discovery and Movement. A versioned normalized SHA-256
fingerprint includes the complete tagged input, exact ordered course, selected ids,
Position revision and Movement target. Same id and fingerprint reconstructs the
accepted canonical result; different operation or content conflicts. Independently
confirmed Connections never deduplicate by meaning. Investigation start remains in
its separate attempt namespace and compares stored kind directly instead of adding a
fingerprint.

## S1 numeric storage constraints

Every stored coordinate is a signed `bigint` constrained inclusively to
`-1_000_000_000_000_000..=+1_000_000_000_000_000`. Course validation and Movement
use checked `i128` cross and dot products; overflow is invalid input and writes
nothing. Name, description and optional Position/shape description checks repeat the
shared normalized text invariants. The Place map projection repeats the coordinate
checks but never becomes canonical.
