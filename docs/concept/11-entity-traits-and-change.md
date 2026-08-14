# Entity properties, traits and change

> **Delivered deterministic Property/Trait direction** — Property and Trait
> establishment/development are accepted and implemented, with T1–T10 complete under
> the completed
> [Trait plan](../../.agents/plans/20260813-200829-entity-trait-development/plan.md)
> and published in `docs/game/`. This concept record does not govern implementation;
> neither paid candidate reached a post-setup Agent submit or Trait mutation, and
> second independent Terry/ownership review returned GO with no P0–P3 finding. No
> live Trait success, retry, new authorization or other candidate was earned.

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

Trait is the selected active build edge and remains a separate capability. The
direction below is implemented through the current `docs/game/` contract, schema,
runtime, adapters, Agent contract and token-free fake evidence.

## Delivered deterministic Trait capability

The desired player outcome is that an Entity can retain one recognizable
characterizing statement and later develop it through accepted play. Pip might begin
as a rat that first becomes known for “startling at every hard sound” during one
accepted situation and later becomes one that “waits for the second echo before
springing.” A later Agent should be able to ground a callback in that current
characterization and its accountable earlier expression without a score, hidden
narrator or prose interpreter.

### Rounds 1–3 confirmed

- Every Entity can carry Property and Trait state. Whenever an Agent actually fetches
  its own Character or another eligible Entity, that Entity comes with the correct
  current Property and Trait associations. The Agent decides what is relevant to the
  situation; World neither infers relevance nor stores observer-specific Knowledge,
  Observation, receipt state or copies. Property and Trait are Entity-owned state,
  not Relationship domain.
- No creation route accepts initial Traits. Traits arise contextually during play,
  keeping establishment tied to a meaningful situation instead of front-loading a
  profile.
- A confirmed exact-local Action may affect actor, current Place, ordinary Entities
  and another Character uniformly without a role/control branch.
- Establishment creates one stable World Trait id. Development appends an immutable
  predecessor-linked statement and advances one current pointer. Retirement remains
  deferred.
- Both `submit_action` and `submit_interaction` may establish or develop Traits
  through one private World writer. Interaction remains directed outward behavior;
  a Trait consequence never authors the target's response, thought, consent or
  volition.
- Interaction Trait subjects are uniformly its actor and explicit targets. Action
  uses one closed mixed `trait_change[1..100]`; Interaction uses optional
  `trait_change[0..100]`, may mix establishment/development and may coexist atomically
  with its Property consequence.
- Exact normalized active duplicates and unchanged development reject. Semantic
  contradictions are accepted; one Trait lineage supersedes only itself and World
  infers no automatic precedence over another Trait, Property or description.
- Only explicit confirmed Action and Interaction are executable Trait causes. No
  external deterministic cause, timer, background Agent or off-screen writer exists
  in this slice.
- Trait statement prose is non-executable. It never grants a modifier, permission,
  action, roll result or mechanic.
- PostgreSQL stores statements in `text`, not `LONGTEXT`. The proposed fixed
  500-character ceiling is rejected because Traits may support richer stories; the
  confirmed bound is trimmed non-NUL 1–4,000 Unicode characters per immutable
  statement version. Richer causal narrative belongs in Activity prose.
- `list_entity_at_current_place` stays compact. `get_character` returns the selected
  Character with one bounded combined current Property/Trait association page, while
  new `get_entity_at_current_place` does the same for one exact-local Entity. Activity,
  creation and mutation Entity/Place values remain compact references rather than
  recursive fetches. The new scoped read replaces
  `list_entity_property_at_current_place`, retaining exactly thirteen player tools.
- The Agent authors each Trait consequence and naturally previews its exact Entity,
  lifecycle and current/new characterization where applicable plus outward prose,
  preserving stable lineage continuity without showing a UUID.
  The User accepts/rejects the complete package but receives no direct Trait editor;
  no post-confirmation Trait may be hidden.
- One mixed package rejects a duplicate establishment `(entity_id, normalized
  statement)`, duplicate development `trait_id`, development to the exact current
  statement and establishment of an exact statement already active on the Entity.
  Semantic near-duplicates and contradictions remain accepted.

### Accepted complete slice

The complete plan is accepted. Its full bounded page/cursor shapes, thirteen-tool
catalog, Action/Interaction packages, schema, atomicity, retry and performance are
delivered and proved through T3–T8. Original candidate `candidate-MmwRmcBv`, digest
`f38ed39b7a100ee09cca46743b3b9398f46ccb3d4999f4591f478113fb2b4fa3`,
consumed its authorization and invoked Codex for exactly one model process call. It
failed before any Agent MCP read or post-setup Agent submit/Trait mutation because Codex had not explicitly
enabled the sole current MCP `2026-07-28` wire protocol. Ownership-verified cleanup
dropped its database; evidence and sentinel remain preserved.

On 2026-08-14 the User accepted exactly one replacement candidate using only MCP
`2026-07-28`. This accepts token-free replacement readiness/audit, not spend. The
corrected candidate is bound to digest
`a564ddedd49094dd70b02aad3eefdb39e6334b482d94859d6f61dea720fb1302`.
Archive `.aicadia-trait-playtest/archive-original-f38-MmwRmcBv` now preserves all 68
original files with private permissions and unchanged content/permission
fingerprints. Token-free `preflight-cEeVelIZ` returns GO for a564 with no candidate
start, authorization consumption or Codex invocation, zero model calls, thirteen
equal runtime tools, six passed schemas and ownership-verified dropped database.
Independent audit found only the stale-status P3 corrected by this authority update;
focused independent re-review returned GO with no P0–P3 finding, completing T9R.
The User then explicitly authorized exactly one paid a564 replacement.
`candidate-ydttdFfc` consumed that authorization and one model process call (`68117`
input, `50176` cached input, `798` output and `419` reasoning tokens). Exactly four
current-MCP reads and a strict valid three-proposal final completed before the local
validator falsely rejected valid fractional RFC3339 timestamps from
`get_character` and `list_activity_at_current_place`. No preview, post-setup Agent
submit, Trait mutation or candidate HTTP gate ran; ownership-verified cleanup dropped the database.
Independent audit returned GO for this exact failed claim, with one P1 at the
date-time validator and no other finding. T10 accepts valid UTC fractional seconds
while requiring exact normalized parse/format roundtrip. Its post-failure runner
digest is `6649959c7f230f2240f8d1b7e67081c20c473c00654ef36409fa439a8d85a824`;
the later T4 candidate is the exact two-call failure recorded below. Initial final
review returned NO-GO only
for missing calendar roundtrip P1 and setup-obscuring wording P2; both were corrected
and second independent Terry/ownership review returned GO with no P0–P3 finding. No
Trait live gameplay outcome or retry was earned by that correction; no older
initialize/initialized path, protocol revision, downgrade, fallback, compatibility
shim, retry or other candidate is accepted. The accepted live-validation plan's T2
now preserves the 64 a564 historical files unchanged in a private archive plus one
private algorithm index. Token-free `preflight-HMxwGPCF` returned GO for this digest
with `candidate_started=false`, `authorization_consumed=false`,
`codex_invoked=false`, `model_calls=0`, current-only MCP `2026-07-28`, 13
live-equal tools, 6 schemas and cleanup `ownership_verified_and_dropped`. The initial
T3 review's exact two P3 findings are corrected; independent re-review returned GO
with no P0–P3 finding, completing T3 readiness. Exact-digest
`candidate-63hjH4HW` then consumed its authorization and two model calls. The Action
proposal passed after the four current-MCP reads `get_world`, `get_character`,
`list_entity_at_current_place` and `list_activity_at_current_place`. Its zero-tool
Action preview exited 0 with strict valid JSON and exact `entity_name: "Pip"`, then
failed solely because live `validate_action_preview` required `startswith("Pip ")`
while the prompt and permanent fake contract require exact `Pip`. No Action commit,
candidate HTTP gate, Interaction or Mara phase ran; submits, post-setup Trait
mutation and live success are zero. Total usage was 148068 input, 100352 cached
input, 1722 output and 867 reasoning-output tokens. Cleanup was
`ownership_verified_and_dropped`, evidence is private, no process remains and no
retry or new authorization exists.

Final independent T5 review returned GO for the exact completed-as-failed alignment
with P0/P2/P3=0. The known deferred P1 is live Action `startswith("Pip ")` and
analogous unreached Interaction `startswith("Pip ")`/`startswith("Mara ")` drift
against prompt/fake exact names. No fix, retry, live success or new authorization
was added. Trait validation is complete with no active edge. The User chose unchanged
draft `.agents/plans/20260814-130554-documentation-architecture/plan.md` next; this
closure does not edit, start or activate that plan.

The delivered persistence uses three relations—immutable `entity_trait`
identity/owner, append-only `entity_trait_version` statement/history whose unique
root is the sole establishing Activity provenance, and pointer-only
`entity_trait_current`—with stable-order set-based writes. Development input names
stable Trait id plus new statement; expected Place revision and the locked current
pointer choose the predecessor atomically rather than accepting a predecessor id.
No Entity creation touches them and no observer relation exists. The replacement
design retains confirmed Action establishment, later Interaction development of the
same stable id and correct enriched Entity retrieval. Its token-free readiness audit
is complete; its separate paid authorization was consumed by the exact failed
replacement and never authorizes a retry. The two-Agent, two-session, seven-call,
zero-retry boundary remains exact; the superseded initial-Trait/four-call candidate
and any legacy MCP path are not reused.

## Delivered Property slice

### Initial Properties on every Entity creation route

All four existing Entity-creation routes accept an unordered `property` list of
0–100 unique canonical keys: `create_entity`, `create_character`,
`create_entry_place` and `submit_action.introduce_entity`. The first creates an
unplaced ordinary Entity, the last a placed ordinary Entity, and the middle routes
create Character and Place Entity roles.

The route's existing Activity is provenance for every initial value. Entity, role,
placement where applicable, Activity, first-use keys, immutable history and current
pointers commit or roll back together. This is one Entity/Property model, not four
features: omitting a route would create role-dependent Property semantics.

### World Action Property change

The existing closed `submit_action` consequence union gains
`change_entity_property`. It contains 1–100 semantically unordered unique
`(entity_id, canonical key)` writes over 1–100 exact-current-Place Entities. Eligible
Entities are uniformly the actor, current Place, other co-present Characters and
explicitly placed ordinary Entities. Character role and User control never change
eligibility or errors.

This supports one Action such as an explosion changing the legs of the actor, an
ordinary creature and another Character together. World changes only the listed
Properties; prose cannot select Entities or derive values. An unavailable, remote,
duplicate or otherwise ineligible subject yields one neutral whole-request rejection
and zero writes.

### Interaction Property consequence

`submit_interaction` remains directed outward behavior and accepts an optional
unordered list of 0–100 Property changes. Empty retains today's pure outward
Interaction. With changes, each affected Entity must be either the actor or one of
the explicit Interaction targets; actor and every target remain exact-co-present and
uniformly eligible regardless of Entity role or control provenance. One Interaction
Activity, outward prose, participation, Property history/current state and Place
revision commit atomically.

Changing a target's Property is not an authored response, thought, consent,
volition, relationship or placement. It is an explicit typed consequence of the
encounter. Action and Interaction remain different public semantics and share only
private Property validation/writing machinery. This preserves the earlier confirmed
rule that a Property change may arise through an Action or Interaction; deferring it
would contradict that direction.

### Reads and presentation

Current Properties are Entity-owned state. The delivered scoped
`get_entity_at_current_place` read returns one exact-local Entity with a bounded
combined current Property/Trait page, while `get_character` returns the same
association shape for the actor and compact local orientation remains compact. The
superseded flat Property read is absent; neither current read exposes role/control
labels or remote/global values.

Authorized Activity output hydrates the exact typed Property changes stored by that
Activity, including initial Properties. Current structured Property wins over
conflicting introductory prose for that exact fictional key meaning; the immutable
introduction remains historical context. This precedence never grants infrastructure
meaning: a Property key or value such as `user_controlled`, `npc` or `owner_user_id`
is only user-authored in-World content and cannot establish or reveal actual User,
Character, NPC, ownership or control provenance. World has no control-word denylist;
ordinary Property shape and type validation still applies.

## Shared key and Property relations

```text
property_key
  id bigint generated always as identity primary key
  key text not null unique
  value_type text not null check ('text' | 'integer')
  first_activity_id uuid not null references activity(id)
  unique (id, value_type)

entity_property_history
  entity_id uuid not null references entity(id)
  property_key_id bigint not null
  activity_id uuid not null references activity(id)
  previous_activity_id uuid null
  value_type text not null
  text_value text null
  integer_value bigint null
  primary key (entity_id, property_key_id, activity_id)
  foreign key (property_key_id, value_type)
    references property_key(id, value_type)
  foreign key (entity_id, property_key_id, previous_activity_id)
    references entity_property_history(entity_id, property_key_id, activity_id)
  constraint exactly_one_typed_value

entity_property
  entity_id uuid not null references entity(id)
  property_key_id bigint not null references property_key(id)
  current_activity_id uuid not null
  primary key (entity_id, property_key_id)
  foreign key (entity_id, property_key_id, current_activity_id)
    references entity_property_history(entity_id, property_key_id, activity_id)
```

The canonical key is 1–64 ASCII lower-snake-case characters beginning with a
letter. First accepted use creates the global immutable key/type pair. Same key and
type reuses it; same key with another type yields `property_key_conflict`. No key
description, alias registry or synonym inference exists.

History is the sole value store. Text is trimmed non-NUL content of 1–4,000 Unicode
characters; integer is signed 64-bit. The typed check permits exactly the matching
column. `previous_activity_id` is null on first establishment and otherwise names
the same Entity/key lineage. History is append-only.

The current table holds only a pointer. Its composite identity and foreign key
prevent two current values or cross-lineage pointers. One Activity may change many
Properties, so `activity_id` is indexed but deliberately not unique. Activity
hydration orders rows by Entity id and canonical key.

## Transaction, retry and performance

Every request normalizes and sorts its semantic Property set before fingerprinting.
Initial lists reject duplicate keys; change lists reject duplicate
`(entity_id, key)` pairs. Bounds are checked before mutation and every list is
all-or-nothing.

The shared writer resolves the full eligible Entity set without role/control
branches; sorts and locks affected Entity/current rows; resolves or inserts keys in
key order; bulk-inserts history; bulk-upserts pointers; and commits them with the
route's Activity and other role/placement state. A concurrent first-key loser reuses
the winner only for the same type; a type conflict rolls back everything.

Action and Interaction fingerprints include sorted normalized Property writes.
Initial Properties enter the relevant creation/action semantic input.
`submit_action.introduce_entity` retains request-id retries; other creation routes
retain existing unique/concurrency semantics. An accepted equal retry hydrates its
original Property rows from Activity before current preconditions.

Local current reads join the bounded local Entity set to pointer, history and key in
one set-based query. Activity hydration fetches changes for all page Activity ids in
one query. Required evidence includes query-count assertions and `EXPLAIN`/index
shape at 100 changes/results; no N+1, reverse-value index, cache or partitioning.

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

## Explicit Property deferrals and Trait evidence boundary

Property unset/deletion; key rename/aliases/inference; possession/relations;
volition/thought/response/consent; placement/movement; remote/cross-Place subjects;
dynamic, prose or area selectors; and global/reverse Property search remain
deferred. The deterministic Trait slice above now exists in schema, runtime,
adapter/catalog behavior and token-free Agent evidence. T7/T8 are complete; the
original and replacement paid candidates are completed-as-failed, and completed T10
retains the exact incomplete live-gameplay claim after final review.
