# Domain contract

> **Role / side:** Defines current World subjects, state, invariants and normalized Activity meaning / runtime side.
> **Authority:** Current game-domain semantics and validation independent of adapters and storage layout.
> **Excludes:** Delivery status, rollout narrative and evidence results.

This document is the current game authority. Aicadia has one persistent `World`,
durable `User` records, shared `Entity` records, at most one owned `Character` Entity
role per User, and zero or one shared entry `Place`. A Character may remain unplaced
or explicitly enter that Place. Every Entity may carry zero or more compact typed
Properties and stable, developing, non-executable Traits, established at creation or
changed through confirmed local Actions and actor/target Interactions. An entered
Character may submit one Action that introduces one Entity with initial Property and
Trait state, or atomically combines 0–100 exact-local Property changes with 0–100
Trait establishments/developments across exact-local Entities; a state-change Action
requires at least one change. One directed Interaction toward 1–100 existing
co-present Entities may carry optional actor/target Property changes and 0–100 mixed
Trait establishments/developments without authoring their responses. Accepted game mutations append
immutable normalized `activity` in the same PostgreSQL transaction as current state.
An entered Character may also begin one World-resolved investigation and, after a
positive result and User-confirmed Agent authorship, establish exactly one found
Entity at that Place with the same initial Property/Trait rules and attributable
Activity.

## Subjects and current state

### World seam

The concrete `World` type is the only public game-behavior seam. The fifteen player
capabilities ship together through thin HTTP and MCP adapters. Each explicit call
stands alone: there is no durable game session and no server-side Agent invocation
or inference. Agents may reason and propose, but only World assigns identities,
resolves investigation chance, validates commands and writes durable state.

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
split applies to Traits: the Agent authors exact initial or contextual state, the
User accepts or rejects its complete natural preview, and World alone
validates/writes it.
There is no direct Trait editor.

Property keys and values are user-authored in-World content, including keys or text
such as `user_controlled`, `npc` or `owner_user_id`. They may make fictional claims,
but never establish or reveal actual User, Character, NPC, ownership or control
provenance. World applies the ordinary key/value validation rules and has no
control-word denylist; structural provenance comes only from authorized typed World
fields, never Property content.

An ordinary Entity may have zero or one explicit current Place relation. Absence is
valid and is never inferred from prose or Activity.

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
state. An absent `current_place` means the Character exists but has not entered the
World; it is not a missing lookup or unknown coordinate.

### Place and World entry

```rust
struct Place {
    entity: Entity,
    is_entry: bool,
}
```

A Place is an Entity role whose stable identity is `place.entity_id`. Zero entry
Places is valid before genesis; at most one row may have `is_entry = true`.

### Investigation attempt and discovery

An investigation is one explicit request by the Agent for World to test whether the
entered Character can find something at its exact current Place. World derives both
Character and Place, applies per-User admission, reads the bounded recent Place
history signal and performs one authoritative random draw before the Agent authors
content. The User may advise the Agent but supplies no mechanical focus, seed, odds,
result count or retry count.

The signal `n` is the number of `submit_discovery` Activities among the last 48
Activities at that Place. Chance is
`p = 1/10 + (1/2 - 1/10) * 2^(-n/6)`, resolved from operating-system entropy behind
World's private chance component. A fresh admitted attempt is independent. Elapsed
time, prior zero outcomes and consecutive misses never improve odds; there is no
pity, accumulated luck or runtime-configurable chance input.

Every admitted start creates one durable internal attempt with one World-assigned id,
the responsible User, derived Character and Place, stored `zero` or `positive`
outcome, creation time and optional consumed/voided provenance. The attempt is not an
Entity, Activity, pending opportunity, session or player-visible history. It exists
only to make retry, admission, bounded coexistence and one-time consumption exact
across processes and restarts. A start retry returns its stored outcome and immutable
limit without another draw. Zero and unconsumed positive attempts change no current
World state and append no Activity. A voided positive always names a distinct newer
attempt as provenance and can never point to itself.

A positive attempt permits one discovery: an Agent-authored Entity representing
something found rather than made, brought or placed. After re-reading current
exact-Place context, the Agent previews the complete name, description, 0–100
Properties and 0–100 Traits and the User confirms them. World cannot infer or prove
the found-versus-made distinction; the Agent contract owns it. World verifies only
the typed attempt and find rules, then atomically creates and places the Entity,
establishes its state, appends `submit_discovery` Activity, consumes the attempt and
advances the Place pointer. There is no generic Discovery record or World-typed kind.


## Activity

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
    SubmitDiscovery,
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
| `submit_action.change_entity_state` | acting Character | derived current Place | each Property- or Trait-affected Entity as `subject`; current Place as `location` |
| `submit_interaction` | acting Character | derived current Place | 1–100 existing Entities as `target`; current Place as `location` |
| `submit_discovery` | acting Character | attempt's Place, equal to the Character's derived current Place | found Entity as `subject`; current Place as `location` |

Only accepted `submit_action`, `submit_interaction` and `submit_discovery` Activity
has non-null prose, request id and request fingerprint. Existing and other new
Activity keeps those fields null. Activity rows, relations and accepted prose reject
update and delete. The consumed investigation attempt points to its accepted
discovery Activity; Activity does not duplicate the attempt id in its public shape.
Reads, rejected requests, transport traffic, conversation text and private Agent
reasoning are not activity. There is no JSON event payload, universal event
abstraction or event sourcing.

`property_change` is empty when an Activity established no Properties. Otherwise it
contains that Activity's exact typed values, sorted by Entity id then key, after the
existing personal or Place authorization has selected the Activity. Initial
Properties from all five creation routes are changes of their creation Activity.
Activity never infers a Property from prose, and internal Property-key ids are not
exposed.

`trait_change` is empty when an Activity established or developed no Trait. Initial
Traits from all five creation routes are establishments of their creation Activity;
that provenance records first accepted shared-World establishment and does not claim
the Trait was fictionally learned then. Otherwise it contains the exact sorted Activity-backed establishment/development
results, including stable Trait id, compact owning Entity summary, current statement
and previous statement for development. These are hydrated in one bounded query
after the personal or Place lens authorizes the Activity. Activity Entity references
remain compact historical references and never recursively carry current Property/
Trait associations.

One Activity owns one canonical prose value. Personal and Place-local history reads
return that same record rather than copied lens-specific text. Every history lens
orders canonical records by `(occurred_at, id)`; World acceptance is the only current
time axis and Agents cannot backdate or reorder history.


## Shared value validation

## Property values and keys

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

## Trait statements and lifecycle

```text
TraitInput { statement }
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

Creation `trait` contains 0–100 establishment statements. Action and Interaction
`trait_change` contain 0–100 items; a `change_entity_state` Action requires its
Property or Trait list to be non-empty. Within one mixed list, duplicate establishment
`(entity_id, normalized statement)`, duplicate development `trait_id`, development
to that Trait's exact current statement or any duplicate exact statement in the
intended post-package active set for one Entity returns `invalid_trait`. The final-set
rule rejects development into another unchanged active statement, two developments
to the same statement and an establishment plus development to the same statement.
A statement vacated by another development in that same unordered package may be
reused because it is unique after the complete package; input order never changes
the result. Every such failure rejects the whole creation, Action or Interaction
atomically, including any Property changes, Activity, role or participation state.
Semantic near-duplicates and contradictions are accepted. A well-formed missing,
remote, departed, stale or otherwise ineligible Entity/Trait uses neutral
`trait_unavailable` and exposes no role/control/existence distinction.

Entity, Character and entry Place input is trimmed, requires 1 through 120 Unicode
characters for `name` and 1 through 4,000 for `description`, and rejects U+0000.
Action and Interaction prose use the same normalization, require 1 through 4,000
Unicode characters and reject U+0000. PostgreSQL repeats the stored text invariants.

World distinguishes malformed request or revision input; invalid Entity, Character,
Place, Action, Interaction, discovery prose, Property or Trait input; invalid Entity or Activity
limit; User, Entity,
Character or entry Place not found; unplaced Character; existing Character,
already-entered Character or existing entry Place; request-id conflict; neutral
Interaction-target, discovery-attempt, Property-Entity, scoped-Entity or Trait
unavailability; Action, Interaction or discovery request-id conflict; investigation
admission; Property-key conflict;
exact-Place revision conflict; and unavailable storage.
Adapters expose the canonical spellings and status mapping in
[Protocol contract](protocol.md#canonical-errors).

## Required evidence

Tests retain all prior evidence and prove:

- the schema, World, HTTP/MCP adapters, exact fifteen-tool catalog, Agent
  contract and token-free fake controller agree on the deterministic Trait contract;
  none of that evidence is a paid or real-model Trait claim;
- every creation route accepts 0 and 100 initial Traits beside 0 and 100 Properties,
  gives each Trait a stable id rooted in that creation Activity and rejects duplicate,
  malformed or 101st state without orphan rows;
- mixed 0–100 Action Trait changes uniformly cover actor, current Place, ordinary
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
  exactly the same fifteen-name catalog, schemas, results and errors; and
- Agent guidance requires complete natural Trait preview and whole-package User
  confirmation, exposes no direct editor and never treats Trait prose as mechanics;
- each of `create_entity`, `create_character`, `create_entry_place`,
  `submit_action.introduce_entity` and `submit_discovery` accepts independent 0–100 initial Property and
  Trait lists and commits
  its whole Entity/role/placement/Activity/history/current bundle atomically;
- duplicate or 101st initial values reject the whole creation with no orphan state;
- one `change_entity_state` Action changes Properties and Traits of the actor, current
  Place, an ordinary Entity and another Character under one role/control-neutral
  exact-local rule, while missing,
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
