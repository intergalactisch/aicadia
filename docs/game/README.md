# Aicadia current build contract

This document is the current game authority. Aicadia has one persistent `World`,
durable `User` records, shared `Entity` records, at most one owned `Character` Entity
role per User, and zero or one shared entry `Place`. A Character may remain unplaced
or explicitly enter that Place. Every Entity may carry zero or more compact typed
Properties, established at introduction or changed through confirmed local Actions
and actor/target Interactions. Every Entity may also carry zero or more stable,
developing, non-executable Traits established only through confirmed play. An
entered Character may submit one homogeneous Action that introduces one Entity,
changes 1–100 exact-local Entity/key pairs or atomically establishes/develops 1–100
Traits across exact-local Entities. One directed Interaction toward 1–100 existing
co-present Entities may carry optional actor/target Property changes and 0–100 mixed
Trait establishments/developments without authoring their responses. Accepted game mutations append
immutable normalized `activity` in the same PostgreSQL transaction as current state.

Delivery history and current status: see [Trait evidence](../evidence/trait.md).

## World interface

The concrete `World` type is the only public game-behavior seam. The current
surface has these thirteen operations, with thin HTTP and MCP adapters:

```rust
get_world() -> WorldView
create_user() -> Result<User, WorldError>                 // provisioning only
get_user(user_id) -> Result<User, WorldError>
get_character(user_id, input) -> Result<CharacterEntityStatePage, WorldError>
create_character(user_id, input) -> Result<Character, WorldError>
create_entry_place(user_id, input) -> Result<Place, WorldError>
enter_world(user_id) -> Result<Character, WorldError>
list_activity(user_id, input) -> Result<ActivityPage, WorldError>
create_entity(user_id, input) -> Result<Entity, WorldError>
list_entity_at_current_place(user_id, input)
    -> Result<CurrentPlaceEntityPage, WorldError>
list_activity_at_current_place(user_id, input)
    -> Result<CurrentPlaceActivityPage, WorldError>
get_entity_at_current_place(user_id, input)
    -> Result<CurrentPlaceEntityStatePage, WorldError>
submit_action(user_id, input) -> Result<AcceptedAction, WorldError>
submit_interaction(user_id, input) -> Result<AcceptedInteraction, WorldError>
```

`create_user` is internal provisioning, not player-facing. `list_entity` and
`get_entity` remain out-of-world loopback reads for the local operator ledger; they
are not Character knowledge and do not ship in the accepted MCP player catalog.
These thirteen capabilities ship together through HTTP and MCP as specified in
[Agent interface](agent-interface.md). Authentication remains deferred;
adapters obtain an untrusted development `UserId` from `Aicadia-User-Id` for
contextual operations.

Each explicit call stands alone. There is no durable game session and no server-side
Agent invocation or inference. Agents may reason and propose, but only World assigns
identities, validates commands and writes durable state. MCP exposes one current,
provider-neutral Agent play contract and complete tool descriptions as specified in
[Agent interface](agent-interface.md); a host supplies those instructions to its own
model, while typed World results remain authoritative.

## Supported local play

Local development supports one persistent World and one stable hidden development
User through the launcher described in [Local play](local-play.md). The User plays
and completes Character onboarding only through their own Agent. A same-origin
browser page is a read-only ledger of the World identity, shared Entity records and,
once a Character exists, that Character's accepted Activity/prose. It uses only the
existing `get_world`, `list_entity`, `get_entity` and `list_activity` HTTP reads.
The global Entity reads are loopback operator/ledger access, not player capabilities
or a source the Agent may use.

The launcher prints but never invokes one explicit local player-adapter command. The
adapter starts Codex with an empty external workspace and isolated transient home,
requires current Aicadia MCP and injects the exact permanent player contract. It
inherits no development repository, personal skill, extra MCP or durable
conversation context and removes its owned temporary root after exit. Other Agent
hosts are conforming only when they provide the equivalent provider-neutral
contract, required-authority and fail-closed boundaries in
[Agent interface](agent-interface.md).

The ledger has no chat, proposals, confirmation, mutation, model invocation or
dedicated User, Character or Place view. Character and Place may appear only as the
typed references already present in records. Its contextual
`Aicadia-User-Id` value is hidden transport context, not authentication. The page is
not a player-facing capability and adds no `World` operation, game-data endpoint or
MCP tool. Accepted World calls remain the only durable state and Activity; private
Agent conversation is never stored.

## Current state

### User

```rust
struct User {
    id: UserId,
    created_at: DateTime<Utc>,
}
```

A User is a durable participant and request-provenance subject. It is not an Entity,
Character, Place, account model or authenticated identity. Each User owns at most one
Character. `create_user` creates only a User and never writes game activity.

### Entity

```rust
struct Entity {
    id: EntityId,
    name: String,
    description: String,
    introduced_by_user_id: UserId,
    introduced_at: DateTime<Utc>,
}
```

An Entity is one durable World subject that later participants must be able to refer
to again. Names are display text, not identifiers, and are not unique. Entity has no
type, kind, taxonomy, ownership or discovery claim. `introduced_by_user_id` says who
introduced the record, not who fictionally created, owns or discovered the subject.

Every Entity may carry zero or more Properties without changing or revealing its
role. A Property is one Entity-owned canonical `key = value`; its value is text or a
signed 64-bit integer. Characters, Places, furniture, flora, fauna and ordinary
Entities use the same model. Property is not ownership, control provenance, a score,
a Trait or prose.

Every Entity may also carry zero or more Traits without changing or revealing its
role. A Trait has one World-assigned stable id, belongs to exactly one Entity and
stores one current non-executable statement through an immutable predecessor-linked
version lineage. Its unique root version is the sole establishing Activity
provenance; development retains the same Trait id and advances only its current
pointer. At transaction commit every stable Trait has exactly one root and exactly
one current pointer, and that pointer identifies the lineage tip. Retirement,
reactivation, deletion, merge and transfer are absent.

Trait statements are trimmed, reject U+0000 and contain 1–4,000 Unicode characters.
They may characterize an Entity but never grant a modifier, permission, action, roll
result or other mechanic. World performs no synonym, paraphrase, contradiction or
cross-model inference. Semantic contradictions may coexist; development supersedes
only the prior version in that same Trait lineage and has no automatic precedence
over another Trait, Property or immutable description.

A User never receives a direct profile or Property-storage edit. The User steers
and confirms the complete meaning, the Agent proposes exact initial state or an
Action/Interaction consequence, and World alone validates and writes. No accepted
Property input identifies which Entities are User-controlled. The same authority
split applies to Traits: the Agent authors an exact contextual consequence, the User
accepts or rejects its complete natural preview, and World alone validates/writes it.
There is no direct Trait editor.

Property keys and values are user-authored in-World content, including keys or text
such as `user_controlled`, `npc` or `owner_user_id`. They may make fictional claims,
but never establish or reveal actual User, Character, NPC, ownership or control
provenance. World applies the ordinary key/value validation rules and has no
control-word denylist; structural provenance comes only from authorized typed World
fields, never Property content.

An ordinary Entity may have zero or one explicit current Place relation. Absence is
valid and is never inferred from prose or Activity. `create_entity` accepts `name`,
`description` and an optional `property` list of 0–100 initial values, derives the
introducing User, always creates a new unplaced Entity and returns the complete
Entity. Equal input remains two Entities. When the
User already owns a Character, the activity records that Character as actor and its
current Place as context when present; otherwise actor and context are absent.
Entity, Activity, initial Property history and current pointers commit as one bundle.

The first `submit_action` consequence also creates an Entity, but World places that
Entity at the acting Character's derived current Place in the same transaction. The
bundle and `create_entity` reuse private validation and insertion behavior; one
public capability never invokes the other.

### Character

```rust
struct Character {
    entity: Entity,
    owner_user_id: UserId,
    current_place: Option<Place>,
}
```

A Character is a User-owned Entity role. `character.entity_id` is both its primary
key and Entity foreign key; there is no Character surrogate id or copied Entity
state. `create_character` accepts `name`, `description` and optional
`property[0..100]`, derives the owner, and atomically creates Entity, Character,
Activity and initial Property state. Concurrent creates for one User yield exactly
one Character without an orphan Entity or Property state.

Character creation always leaves `current_place` absent. Absence means the Character
exists but has not entered the World; it is not a missing lookup or unknown
coordinate. No creation route accepts a Trait. `get_character` returns the
Character, complete current Place when present, nullable `place_revision` and one
bounded combined page of that Character Entity's current Property/Trait associations.

### Place and World entry

```rust
struct Place {
    entity: Entity,
    is_entry: bool,
}
```

A Place is an Entity role whose stable identity is `place.entity_id`. Zero entry
Places is valid before genesis; at most one row may have `is_entry = true`.

`create_entry_place(user_id, {name, description, property})`:

- derives an existing Character from User context and accepts no ids;
- requires that Character to be unplaced;
- uses the same trimming and semantic bounds as Entity creation;
- lets the first connected Agent author the entry Place name and description;
- atomically creates Entity, Place, Activity and 0–100 initial Properties;
- permits exactly one winner under concurrent requests and leaves no orphan Entity
  or Property state.

This is World genesis, not discovery. A second entry Place is rejected.

`enter_world(user_id)` accepts no payload, Character id or Place id. World derives
the current Character and the entry Place, then atomically sets
`character.current_place_entity_id` only when it is absent and writes one activity.
A Character may remain unplaced indefinitely. Retrying or racing a successful entry
returns the same Character and does not append another activity. This operation
cannot select a destination and is not movement.

Coordinates, geometry, boundaries, containment, routes, distance, Place
neighborhoods and additional Places are absent.

### Character-grounded Action

`submit_action` accepts one of three homogeneous typed consequence kinds. It is not
a generic patch language and one Action never mixes Entity introduction, Property
changes and Trait changes with each other. A Trait-change Action contains one mixed
Trait lifecycle list:

```rust
struct SubmitAction {
    request_id: Uuid,
    expected_place_revision: PlaceRevision,
    prose: String,
    consequence: ActionConsequence,
}

struct IntroduceEntity {
    name: String,
    description: String,
    property: Vec<PropertyInput>,
}

struct AcceptedAction {
    activity: Activity,
    consequence: AcceptedActionConsequence,
    place: Place,
}

struct PlaceRevision {
    place_entity_id: EntityId,
    occurred_at: DateTime<Utc>,
    activity_id: ActivityId,
}
```

`ActionConsequence` is a strict tagged union:

- `introduce_entity { name, description, property[0..100] }` creates and places one
  Entity with optional initial Property state; or
- `change_entity_property { property_change[1..100] }` changes 1–100 unique exact
  `(entity_id, key)` pairs in one atomic operation; or
- `change_entity_trait { trait_change[1..100] }` atomically mixes typed
  `establish { entity_id, statement }` and `develop { trait_id, statement }` items.

The change subjects may be the actor, current Place, other co-present Characters or
placed ordinary Entities at that exact current Place. The submitted Entity ids are
exact selectors, never a dynamic, prose-derived or area selector. Missing, remote,
departed and otherwise ineligible subjects all return the same neutral
`property_entity_unavailable` or `trait_unavailable` error for the corresponding
consequence. World does not branch on Entity role or reveal which Entity is
controlled by a User.

Input accepts no User, Character or Place selector and no effective time. World
derives the User's Character and exact current Place; a missing Character is not
found and an unplaced Character cannot act. World assigns new Entity, Activity,
Property-key and Trait identities and the acceptance time. Establishment names an
eligible Entity; development names one stable current Trait id and World derives its
Entity. Expected Place revision plus the locked current pointer selects the
predecessor atomically; development accepts no predecessor selector.

One accepted introduction atomically creates and places the Entity, writes its
initial Properties, inserts one Activity with one canonical prose value and records
explicit Entity roles. One accepted Property Action atomically writes every new
immutable value and current pointer under that same Activity. One accepted Trait
Action atomically writes every root/version/current pointer under the Activity; each
affected Entity has `subject` participation and the Place remains `location`.
Partial acceptance is forbidden. Rejected calls, stale calls and retries add no
Activity, Property or Trait state. The returned `AcceptedAction` tags the introduced
Entity, exact sorted Property changes or exact sorted established/developed Traits
and is the canonical stored result.

The required Agent interaction happens before `submit_action`: compose context from
published reads, show exactly three private grounded proposals, incorporate the
User's selection and optional steering, then show the exact final prose and
structured consequence, including every Property key/type/value/subject or every
Trait lifecycle, affected Entity and exact current/new characterization where
applicable. Stable lineage continuity is rendered naturally without showing its UUID.
The User may accept or reject the whole natural package but never
edits Trait storage directly. The Agent may call `submit_action` only after one explicit
User confirmation of that complete package. Proposals, steering, drafts and
confirmation are private conversation and never World state. World enforces the
submitted package deterministically; it does not and cannot verify the conversational
workflow or semantic prose quality.

### Character-grounded Interaction

`submit_interaction` records one Character's outward behavior toward existing
Entities. It may carry typed Property and Trait changes without authoring a target's
response:

```rust
struct SubmitInteraction {
    request_id: Uuid,
    expected_place_revision: PlaceRevision,
    prose: String,
    target_entity_id: Vec<EntityId>,
    property_change: Vec<EntityPropertyChangeInput>,
    trait_change: Vec<EntityTraitChangeInput>,
}

struct AcceptedInteraction {
    activity: Activity,
    place: Place,
}
```

The target list is an unordered set containing 1 through 100 distinct Entity ids.
Input accepts no User, actor or Place id. World derives the current User's entered
Character and exact current Place, excludes the acting Character and accepts only:

- other Characters currently entered at that exact Place;
- ordinary Entities explicitly located there; and
- the derived current Place Entity itself.

Every target is validated in one atomic operation. An absent id, duplicate id, the
actor's own id, an Entity elsewhere, or an Entity that ceased to be co-present all
return the same neutral `interaction_target_unavailable` result and write nothing.
The result never distinguishes nonexistence from existence outside the Character's
present context. An empty list or a list over 100 is invalid Interaction input.

`property_change` defaults to empty and contains at most 100 unique exact
`(entity_id, key)` pairs. Each subject must be the actor or an explicit target; a
local non-target, absent, remote or otherwise ineligible Entity returns neutral
`property_entity_unavailable` and rolls back the whole Interaction. Empty retains
the delivered outward-only meaning. Non-empty stores outward participation and exact
typed changes under the same Activity. Changing a target Property establishes
neither that target's response nor consent, perception, thought or volition.

`trait_change` likewise defaults to empty and contains at most 100 mixed typed
establish/develop items. Each affected Trait Entity must be the actor or an explicit
target, uniformly across Entity roles. It may coexist atomically with
`property_change`. Any invalid, duplicate, no-op, stale or unavailable Trait item
rejects the complete Interaction, including its outward Activity and Property
changes. A target Trait consequence is World state from the encounter, never the
target's authored response, thought, consent or volition.

One accepted call stores one Activity with one accountable actor, exact context
Place, canonical prose, the complete target set and zero or more exact Property and
Trait changes. The Place is also linked as
`location`. A target Character may later read that outward behavior and the same
canonical co-target participation, but gains no actor intent, thought, response,
consent or User-control provenance. A non-Character target gains no fictional
knowledge. A co-present Character that is neither actor nor target receives no
`submit_interaction` Activity automatically. Any response is a separately authored
later Activity with its own actor and confirmation.

Before submission, the Agent uses the same private three-direction, steering, exact
preview and explicit-confirmation boundary as `submit_action`. The preview includes
every Property subject/key/type/value and every Trait lifecycle, affected Entity and
exact current/new characterization where applicable. Stable lineage continuity is
rendered naturally without showing its UUID. Free prose
expresses the behavior but has no state consequence beyond typed participation and
submitted Property/Trait changes.
Repeated accepted targeting remains possible in this slice; it triggers no target
Agent, background work or notification. Private attention controls must be designed
before movement, notifications or broader Interaction reach.

### Delivery identity and exact-Place freshness

`request_id` is an Agent-generated UUID for one intended Action or Interaction. It
remains stable only across uncertain delivery retries and must not be reused for a
different mutation.
World derives a versioned SHA-256 `request_fingerprint` from a length-prefixed
encoding of the normalized request. Action fingerprints include the expected Place
revision, prose, consequence tag and Entity introduction fields, Property changes or
typed Trait changes. Interaction fingerprints include the expected Place revision,
prose, target Entity ids sorted by UUID bytes, Property changes and typed Trait
changes. Initial Property lists
sort by canonical key; change lists sort by normalized `(entity_id,key,type,value)`.
Trait change lists sort typed establish items by `(entity_id,statement)` and develop
items by `(trait_id,statement)`.
List order therefore cannot change retry identity. Fingerprints never hash raw JSON
or depend on field order.

All exact-Place reads return the same opaque `place_revision` when they observed the
same Place representation. Each Place stores one authoritative
`latest_activity_id`, pointing to the Activity most recently accepted as relevant to
that Place under its serialization lock. Internally the revision identifies the
Place and that target Activity as `(occurred_at, activity_id)`. Those Activity fields
identify the pointed-to record; timestamp comparison, UUID order and
`MAX(occurred_at, activity_id)` never determine which Activity was accepted latest.
The revision is a strong freshness validator for this representation, not an
authorization token, request id, global World version or Agent-built hash. Clients
copy the versioned URL-safe token unchanged. Each read uses one short read-only
Repeatable Read transaction so its derived Character, Place, pointer target, page
and revision form one per-call snapshot; no database transaction or durable session
spans Agent calls.

In one mutation transaction World locks the User and first looks up an accepted
`(requested_by_user_id, request_id)`. Equal operation and fingerprint return the canonical stored
result even if Character or Place state later changed. Different fingerprint returns
the operation's request-conflict result. For an unseen id, World derives and locks the Character's
current Place and compares `expected_place_revision` with the current revision before
the remaining validation and writes. A changed representation returns
`place_revision_conflict` and writes nothing. Activity at another Place does not
invalidate the token.

Every writer that changes this exact-Place representation takes the same Place lock
before acceptance. `create_entry_place` assigns its preallocated genesis Activity as
`latest_activity_id` when it inserts the new Place. `enter_world`, `create_entity`
when its acting Character is currently placed, although the new Entity remains
unplaced; `submit_action`; and `submit_interaction` lock the existing Place, insert
their Activity, then
atomically point that Place to the inserted Activity in the same transaction. A
failure rolls back both Activity and pointer change. Mutations at different Places
remain concurrent. Reads issue no nonce, reservation or preparation record, and no
global revision or counter exists.

## Activity history

Current state remains authoritative and is never rebuilt by replay. Each accepted
covered mutation appends one immutable activity and its normalized involved-Entity
relations in the same transaction:

```rust
struct Activity {
    id: ActivityId,
    operation: ActivityOperation,
    actor_character: Option<EntitySummary>,
    context_place: Option<PlaceSummary>,
    involved_entity: Vec<ActivityEntityReference>,
    property_change: Vec<EntityPropertyChange>,
    trait_change: Vec<ActivityTraitChange>,
    prose: Option<String>,
    occurred_at: DateTime<Utc>,
}

enum ActivityOperation {
    CreateCharacter,
    CreateEntity,
    CreateEntryPlace,
    EnterWorld,
    SubmitAction,
    SubmitInteraction,
}

enum ActivityEntityRole {
    Subject,
    Destination,
    Location,
    Target,
}
```

The stored `activity.requested_by_user_id` retains accountable internal request
provenance but is intentionally absent from player-visible history. Stable ids and
server-owned roles have these exact meanings:

| Operation | Actor Character | Context Place | Involved Entity |
| --- | --- | --- | --- |
| `create_character` | absent | absent | new Character Entity as `subject` |
| `create_entity` | current Character when one exists | its current Place when present | new Entity as `subject` |
| `create_entry_place` | proposing unplaced Character | absent | new Place Entity as `subject` |
| `enter_world` | entering Character | entry Place | entry Place Entity as `destination` |
| `submit_action.introduce_entity` | acting Character | derived current Place | new Entity as `subject`; current Place as `location` |
| `submit_action.change_entity_property` | acting Character | derived current Place | each changed Entity as `subject`; current Place as `location` |
| `submit_action.change_entity_trait` | acting Character | derived current Place | each affected Trait-owning Entity as `subject`; current Place as `location` |
| `submit_interaction` | acting Character | derived current Place | 1–100 existing Entities as `target`; current Place as `location` |

Only accepted `submit_action` and `submit_interaction` Activity has non-null prose,
request id and request fingerprint. Existing and other new Activity keeps those
fields null. Activity rows, relations and accepted prose reject update and delete.
Reads, rejected requests, transport traffic, conversation text and private Agent
reasoning are not activity. There is no JSON event payload, universal event
abstraction or event sourcing.

`property_change` is empty when an Activity established no Properties. Otherwise it
contains that Activity's exact typed values, sorted by Entity id then key, after the
existing personal or Place authorization has selected the Activity. Initial
Properties from all four creation routes are changes of their creation Activity.
Activity never infers a Property from prose, and internal Property-key ids are not
exposed.

`trait_change` is empty when an Activity established or developed no Trait.
Otherwise it contains the exact sorted Activity-backed establishment/development
results, including stable Trait id, compact owning Entity summary, current statement
and previous statement for development. These are hydrated in one bounded query
after the personal or Place lens authorizes the Activity. Activity Entity references
remain compact historical references and never recursively carry current Property/
Trait associations.

One Activity owns one canonical prose value. Personal and Place-local history reads
return that same record rather than copied lens-specific text. Every history lens
orders canonical records by `(occurred_at, id)`; World acceptance is the only current
time axis and Agents cannot backdate or reorder history.

### Migration boundary

Before activity storage, every Entity with a Character role could only have resulted
from `create_character`, and every Entity without that role could only have resulted
from `create_entity`. Migration `0004_world_entry_activity.sql` backfills exactly
those derivable operation, responsible User, subject Entity and original
`introduced_at` facts. The old schema retained no acting Character or Place context,
so both remain null. No entry or placement history is fabricated.

The action migration leaves historical Activity prose, request id and fingerprint
null and assigns no historical Entity location. Current Entity location remains
ordinary authoritative state, not a replayed projection or inference from Activity.

### Personal history read

`list_activity` derives the current Character from User context and accepts no
Character id. It returns an activity exactly once when that Character is either the
stored actor or a role-linked involved Entity. It orders by `(occurred_at, id)`
descending and uses a typed cursor internally:

```rust
struct ListActivity {
    cursor: Option<ActivityCursor>,
    limit: u16,
}

struct ActivityCursor {
    occurred_at: DateTime<Utc>,
    activity_id: ActivityId,
}
```

The default limit is 25; accepted limits are 1 through 100. `next` is absent when no
further row exists. Actor Character, context Place and involved Entity references are
typed summaries.

### Exact current Place reads

`list_entity_at_current_place` and `list_activity_at_current_place` derive the
Character and its exact current Place from User context and accept no Character or
Place selector. They reject an unplaced Character instead of returning an empty page.
Each response contains the complete derived Place, its opaque `place_revision`, one
typed page and `next`.

The accepted Entity read lists other Characters currently entered at that Place and
ordinary Entities with an explicit current relation to it, excluding the requesting
Character. Each entry exposes only stable id, name and description: no Character,
Place or ordinary-Entity role and no owner, User or control provenance. The complete
Place is returned separately and its Entity id remains an eligible Interaction
target. The page orders Entities by `(introduced_at, id)` descending.

The Activity read first scopes every record to the derived current Place: that Place
must be its stored context or linked through any involved-Entity role. Within that
Place page it retains every non-Interaction Activity, and lists a
`submit_interaction` Activity only when the requesting Character is its actor or an
explicit target.
This preserves existing same-Place trail-marker visibility without turning every
co-present Character into an Interaction witness. It orders each canonical Activity
once by `(occurred_at, id)` descending and includes canonical optional prose and the
complete outward participation set. The Entity and Activity reads reuse their typed cursor
families, default limit 25 and accepted limit 1 through 100.

Exact stored Place equality is target eligibility for this first Interaction slice,
not a general claim about visibility, sensory access, distance, containment,
adjacency or a metric radius.

### Bounded current Entity state reads

`list_entity_at_current_place` remains compact orientation and never inlines current
Property or Trait collections. `get_entity_at_current_place` accepts exactly
one `entity_id` selected from grounded local context plus optional current-state
`cursor` and `limit`. Its eligible Entity set is uniformly the acting Character,
current Place, other co-present Characters and ordinary Entities explicitly located
there. It returns that one safe Entity, the safe current Place, matching opaque
`place_revision` and one combined tagged page of its current Property/Trait
associations. Missing, remote, departed and otherwise ineligible Entity selection
uses neutral `entity_at_current_place_unavailable` without role/control distinction.

`get_character` accepts only optional current-state `cursor` and `limit`. It
returns the derived Character and one combined tagged current-state page for the
Character Entity. When entered, its `place_revision` identifies the same exact-Place
representation used by scoped reads and mutations; when unplaced it is null.

Both pages default to 25 and accept 1–100 associations. The single typed sequence
orders Properties first by internal Property-key id and Traits second by stable Trait
id; internal Property-key ids are never exposed. The opaque operation-specific
cursor binds the returned Entity, nullable Place identity/revision and last typed
sort key. Continuations repeat the same Entity and Place revision. An intervening
exact-Place state change returns `place_revision_conflict`; a selected Entity that is
no longer eligible returns neutral `entity_at_current_place_unavailable`. Callers
start a fresh first page rather than combine snapshots.

Creation, entry, Action and Interaction results and Activity references are compact
acknowledgements/references, not full Entity fetches, and do not recursively contain
current association pages. The delivered surface removes the standalone flat
`list_entity_property_at_current_place` player capability and exposes scoped
`get_entity_at_current_place`, retaining exactly thirteen capabilities.

Current structured Property remains authoritative for the fictional current meaning
of its exact key. Trait lineage supersedes only itself. Neither model automatically
overrides the other or immutable description prose; World infers no alias, synonym,
semantic contradiction or cross-model precedence. A `user_controlled`, `npc` or
`owner_user_id` Property/Trait remains only user-authored in-World content and never
infrastructure provenance.

## Operator Entity listing

`list_entity` and `get_entity` are out-of-world loopback operator/ledger reads with no
User context. They retain their existing HTTP handlers for the supported local
ledger while implementation removes them from the MCP player catalog. They are not
Character knowledge, gameplay capabilities or an authority a player Agent may
consult. A future remotely accessible administrator or meta-Agent requires separate
authentication and authorization; this reclassification does not provide it.

## Validation and errors

### Property values and keys

Every initial or changed Property uses one strict tagged value:

```text
PropertyInput { key, value: PropertyValue }
EntityPropertyChangeInput { entity_id, key, value: PropertyValue }
PropertyValue = { type: "text", text } | { type: "integer", integer }
EntityPropertyChange { entity: EntitySummary, key, value: PropertyValue }
```

A canonical key contains 1–64 ASCII lower-snake-case characters, starts with a
letter and matches `^[a-z][a-z0-9]*(?:_[a-z0-9]+)*$`. The Agent creates a key at its
first accepted use. World stores only its canonical English key, immutable value
type and first Activity provenance. Reuse with the same type is valid; reuse with a
different type returns `property_key_conflict`. There is no finite catalog,
description, alias, synonym inference or control provenance on a key.

Control-like keys and values are not denylisted, but they remain ordinary in-World
content and never become authorization, ownership or actual control metadata.

Text values are trimmed, reject U+0000 and contain 1–4,000 Unicode characters.
Integer values are signed 64-bit integers. Initial lists are semantically unordered
and require unique keys. Action and Interaction change lists are semantically
unordered and require unique exact `(entity_id,key)` pairs. A duplicate, invalid
tag/value, invalid key or list outside its route's 0–100 or 1–100 bound returns
`invalid_property`; the complete enclosing operation writes nothing.

### Trait statements and lifecycle

```text
EntityTrait { id, statement }
EntityTraitChangeInput =
  { type: "establish", entity_id, statement } |
  { type: "develop", trait_id, statement }
ActivityTraitChange =
  { type: "establish", entity: EntitySummary, trait: EntityTrait } |
  { type: "develop", entity: EntitySummary, trait: EntityTrait,
    previous_statement }
```

Statement normalization trims outer Unicode whitespace, rejects U+0000 and validates
1–4,000 Unicode characters. It preserves internal whitespace, case, punctuation and
code points. Exact duplicate/no-op comparison uses only that stored trimmed value;
World performs no Unicode folding or semantic comparison.

Action `trait_change` contains 1–100 items; Interaction `trait_change` contains
0–100. Within one mixed list, duplicate establishment
`(entity_id, normalized statement)`, duplicate development `trait_id`, development
to that Trait's exact current statement or any duplicate exact statement in the
intended post-package active set for one Entity returns `invalid_trait`. The final-set
rule rejects development into another unchanged active statement, two developments
to the same statement and an establishment plus development to the same statement.
A statement vacated by another development in that same unordered package may be
reused because it is unique after the complete package; input order never changes
the result. Every such failure rejects the whole Action or Interaction atomically,
including any Interaction Property changes, Activity and participation.
Semantic near-duplicates and contradictions are accepted. A well-formed missing,
remote, departed, stale or otherwise ineligible Entity/Trait uses neutral
`trait_unavailable` and exposes no role/control/existence distinction.

Entity, Character and entry Place input is trimmed, requires 1 through 120 Unicode
characters for `name` and 1 through 4,000 for `description`, and rejects U+0000.
Action and Interaction prose use the same normalization, require 1 through 4,000
Unicode characters and reject U+0000. PostgreSQL repeats the stored text invariants.

World distinguishes malformed request or revision input; invalid Entity, Character,
Place, Action, Interaction, Property or Trait input; invalid Entity or Activity
limit; User, Entity,
Character or entry Place not found; unplaced Character; existing Character,
already-entered Character or existing entry Place; request-id conflict; neutral
Interaction-target, Property-Entity, scoped-Entity or Trait unavailability;
Property-key conflict;
exact-Place revision conflict; and unavailable storage.
Adapters expose the canonical spellings and status mapping in
[Agent interface](agent-interface.md).

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
```

The Property relations above are the delivered schema. `entity_property_history` is
the sole value store and is
append-only. `entity_property` contains only the current Activity pointer, protected
by a same-lineage composite foreign key. `activity.action_consequence` is null for
non-Action operations, stores `introduce_entity` or `change_entity_property` for
Actions and backfills existing Actions as `introduce_entity`. One Activity may own
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
- partial unique `activity(requested_by_user_id, request_id) WHERE request_id IS NOT
  NULL` serves accepted Action and Interaction retry lookup; fingerprints are
  exactly 32 bytes;
- primary-key indexes serve role joins and involved-Entity lookup.

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
The migration extends the closed Action discriminator with `change_entity_trait` and
admits typed Trait changes on Interaction without adding a universal consequence
payload or separate mutation operation.

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

## Required evidence

Tests retain all prior evidence and prove:

- the delivered schema, World, HTTP/MCP adapters, exact thirteen-tool catalog, Agent
  contract and token-free fake controller agree on the deterministic Trait contract;
  none of that evidence is a paid or real-model Trait claim;
- every creation route remains Trait-free and strictly rejects a `trait` field;
- mixed 1–100 Action Trait changes uniformly cover actor, current Place, ordinary
  Entity and other Character; optional 0–100 Interaction Trait changes cover actor
  and explicit targets and coexist atomically with Property changes without a target
  response;
- establishment produces one stable id/root Activity version, development preserves
  the id and advances one predecessor/current pointer, retirement/external causes are
  absent, and 1/4,000/4,001 statement bounds behave exactly;
- exact no-ops and duplicate intended active state reject the complete Action or
  Interaction atomically—including develop-to-other-active, two-develop-to-same and
  establish-plus-develop-to-same cases—while same-package reuse of a statement
  vacated by another development succeeds; semantic contradictions remain valid,
  concurrent development cannot branch, reverse Action/Interaction lock order
  cannot deadlock and retries reconstruct original versions;
- deferred bounded per-Trait commit checks require exactly one root, exactly one
  current pointer and that pointer at the lineage tip; incomplete roots, current
  deletion/backtracking and successor-without-advance all reject;
- combined 1/100-row current-state pages for `get_character` and scoped
  `get_entity_at_current_place` preserve one Entity/Place revision, reject stale
  continuations, expose no role/control state and keep orientation, mutation results
  and Activity Entity references compact;
- HTTP/MCP replace the standalone Property list with scoped Entity fetch and publish
  exactly the same thirteen-name catalog, schemas, results and errors; and
- Agent guidance requires complete natural Trait preview and whole-package User
  confirmation, exposes no direct editor and never treats Trait prose as mechanics;
- each of `create_entity`, `create_character`, `create_entry_place` and
  `submit_action.introduce_entity` accepts 0 and 100 initial Properties and commits
  its whole Entity/role/placement/Activity/history/current bundle atomically;
- duplicate or 101st initial values reject the whole creation with no orphan state;
- one homogeneous Action changes actor, current Place, an ordinary Entity and
  another Character under one role/control-neutral exact-local rule, while missing,
  remote and departed subjects share one neutral error and leave zero writes;
- an Interaction with no changes preserves its outward-only result; one with changes
  atomically updates actor and explicit targets without authoring a response; and a
  local non-target subject rejects the whole Interaction neutrally;
- duplicate Entity/key changes reject atomically; same key/type reuses one key;
  concurrent first use with equal type succeeds without duplicates while different
  types yield one winner and one full `property_key_conflict` rollback;
- equal Action and Interaction retries return exact original sorted changes even
  after later state, reordered equivalent lists retain identity and changed values
  conflict;
- one set-based local current read page returns actor, Place, co-present Characters
  and ordinary Entities without role/control or remote leakage, and authorized
  Activity pages hydrate changes in one batched query without N+1 work;
- declared indexes and query plans support 100-write and 100-read bounds, stable lock
  order prevents deadlocks, history is immutable and current pointers stay on one
  lineage; and
- current structured Property wins presentation of its exact key over conflicting
  introductory prose while both Activities remain immutable history;
- one accepted trail-marker package atomically writes one placed Entity, one Activity,
  canonical prose, exact actor, context, subject and location roles;
- every validation, storage and stale-revision failure rolls back every package row;
- equal request retries return the canonical result, changed content under one id
  conflicts, and accepted identity resolves before later Character/Place preconditions;
- exact-Place pages, pointer targets and revisions are consistent snapshots;
  same-Place writers serialize and advance the pointer even across equal timestamps
  or clock rollback, unrelated Places do not conflict, and malformed tokens are
  rejected;
- a second Character at the Place can read the marker and same Activity/prose;
- one actor can submit 1–100 distinct co-present targets as one Activity, a target
  can recall the canonical outward behavior and co-target set, a reverse response is
  a new Activity, and a non-target bystander receives no Interaction automatically;
- missing, fabricated, duplicate, self, remote and no-longer-present targets return
  the same neutral error and leave no Activity or partial target rows;
- equal Interaction retries ignore target order and return the canonical result,
  while changed content under the same request id conflicts;
- non-Interaction Place history, including trail markers, remains available to
  Characters currently there under the existing scoped-Place rule;
- the thirteen player capabilities have one semantic World/HTTP/MCP contract,
  strict schemas, complete catalog/OpenAPI publication and matching errors;
- the local launcher preserves one database and User across restart, refuses
  concurrent or unprofiled reuse that could create a second User, and never starts
  Codex itself; its printed adapter isolates workspace, home/configuration and
  transient conversation state while requiring current Aicadia MCP;
- the browser ledger uses only the four accepted GET reads, hides User UUIDs, remains
  responsive and keyboard-operable, and renders identical accepted ids and prose
  before and after restart; and

## Explicitly deferred

Authentication, OAuth, browser gameplay, general web UI beyond the supported local
read-only ledger, movement, additional Places, coordinates, routes, Place containment
and adjacency, metric neighborhoods, investigation, rolls, discovery, claims,
generic action or consequence engines, multiple consequences, Entity update or
movement, generic events, event sourcing, global World revisions, durable proposal
or Agent sessions, replay, as-of state, scores, currencies, clocks, background
simulation and server-side intelligence are absent. Interaction witness observation,
private block/ignore state, notifications, background target-Agent activation,
remote Interaction, relationship scores and multi-actor commits are also absent.
Property deletion/unset, key rename, aliases, Trait retirement/reactivation/delete/
merge/transfer or direct editing, possession/relations,
placement changes, remote or prose-derived selectors, reverse/global Property
search and mixed Action consequence kinds are absent. The accepted Property/Trait
target admits only explicitly submitted and confirmed Agent-authored Actions and
Interactions. No deterministic external Trait writer, background Agent, timer,
scheduler, `world_event` or ungrounded simulation exists.
