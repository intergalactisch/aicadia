---
status: load-bearing
era: August Activity-Property-Trait
---

# Character identity and control in persistent worlds

> **Role / side:** sourced research report / development side.
> **Authority:** records this report's question, sources, findings and implications.
> **Excludes:** product decisions and current implementation contracts; see `game/docs/`.

Date: 2026-08-07

Status: research complete; evidence-backed recommendation, not confirmed concept
direction

## Question

How should Aicadia model users, characters and world entities when:

- an account exists outside the world;
- an account may have at most one active character;
- disconnecting is different from explicitly abandoning a character;
- an abandoned character must remain the same persistent part of the world;
- NPCs must exist without requiring a separate world-identity system;
- a character may have discovered classifications such as species, people or
  lineage; and
- the Rust MVP must stay small without closing off legitimate later growth?

The primary case studied here is Evennia because it explicitly separates Account,
Session, Object and Character. The comparison also covers LambdaMOO's persistent
object model, Bevy and Flecs ECS identity, Space Station 14's detachable player
control, TrinityCore's conventional account/character/creature schema, and
Veloren's separation of a Rust ECS runtime from persistence. PostgreSQL and the
original Ports and Adapters article are used for the concrete MVP boundary.

## Research conclusion

Evennia's strongest lesson is not its Python framework. It is the separation of four
different facts:

```text
authentication identity != network session != permission to play != world identity
```

An Evennia Account has no in-world presence. A Character is a persistent in-game
Object. An account can have a durable list of playable characters, while puppeting
is the live Account/Session-to-Object connection. Unpuppeting removes the live
connection; it does not delete the Object. These distinctions are explicit in the
[Account documentation](https://www.evennia.com/docs/latest/Components/Accounts.html),
[Object documentation](https://www.evennia.com/docs/latest/Components/Objects.html),
[Character documentation](https://www.evennia.com/docs/latest/Components/Characters.html)
and
[connection-style documentation](https://www.evennia.com/docs/latest/Concepts/Connection-Styles.html).

The smallest Aicadia model suggested by these findings separates the three control
relations explicitly:

```text
session -- authenticates as --> account
session -- transiently acts as --> character
account <-- durable character_control --> character -- shared primary key --> entity
```

`character` is a sparse mechanical role attached one-to-one to an `entity`, not a
second independently generated identity. `character_control` is a rebuildable
current relation sourced from a `world_event`: its presence is the durable account
assignment, across every login and agent session. Session puppeting is transient.
Explicit abandonment appends the event that removes the current control projection;
the `character` and `entity` remain unchanged.

In the MVP, an NPC need not be a different structural type. It is a character with
no active account assignment. A character can therefore begin as an NPC, be
abandoned into that state, or potentially receive a player later without changing
its world identity.

The evidence-backed MVP recommendation is therefore:

- persist one universal `entity` identity and attach a shared-primary-key
  `character` role only where character mechanics apply;
- represent account entitlement as a separate, current `character_control`
  projection whose history remains in `world_event`;
- derive NPC as "character without current account control", not as a permanent
  entity kind;
- keep species, flora, fauna, material and every other discovered classification
  in the open entity-and-claim model rather than Rust types or schema enums;
- use PostgreSQL from the first multiplayer slice, and do not persist runtime ECS
  identifiers; and
- use one Rust modular monolith with a clear application/storage seam, without a
  framework-shaped hexagonal directory or trait hierarchy.

This recommendation is deliberately small. It is not yet confirmed Aicadia concept
direction.

## Evennia findings

### 1. Account and world object are deliberately separate

Evennia describes an Account as the actual user and an out-of-character entity with
no presence in the game world. The Account connects to a Character Object to enter
the game. The account stores authentication and out-of-character state; the
Character carries the in-world representation. This is a hard separation in the
[Account API](https://www.evennia.com/docs/latest/api/evennia.accounts.accounts.html),
not merely different names for the same record.

All things with in-world presence are Objects. Evennia applies that common model to
characters, chairs, monsters, rooms and other interactable things. `DefaultCharacter`
is a subclass of `DefaultObject`; rooms and exits are other subclasses. The
[Object documentation](https://www.evennia.com/docs/latest/Components/Objects.html)
shows the full client-to-world path as `Client -> Session -> Account -> Object`.

This gives a useful boundary for Aicadia:

- authentication data does not belong on a world entity;
- deleting or disabling an account must not imply deletion of a character; and
- world references should point to the character's world identity, never to the
  account.

### 2. Existence, play permission and live control are separate state

Evennia keeps an Account-side `characters` handler containing the account's
playable characters. Adding or removing a character from this collection is
separate from puppeting it. The implementation wraps a persistent
`_playable_characters` collection and exposes explicit `add`, `remove` and `all`
operations in the
[DefaultAccount source](https://www.evennia.com/docs/latest/_modules/evennia/accounts/accounts.html).

Puppeting is then a separate live operation. `puppet_object(session, obj)` checks the
account's `puppet` permission on the target Object, checks whether another account
already controls it, applies the simultaneous-puppet limit, and only then links the
Session, Account and Object. `unpuppet_object` removes those live links and clears
the session's puppet reference. The Object record itself is not deleted. These steps
are visible in the same
[account source](https://www.evennia.com/docs/latest/_modules/evennia/accounts/accounts.html)
and summarized in the
[Account API](https://www.evennia.com/docs/latest/api/evennia.accounts.accounts.html).

Evennia also configures maximum created characters separately from maximum
simultaneously puppeted characters. Its documented defaults are one character slot
and one simultaneous puppet, but it can allow a larger stable of characters while
still permitting only one to be played at a time. This is direct evidence that
"may play" and "is playing now" are different cardinalities, not one `owner_id`
field with overloaded meaning
([connection styles](https://www.evennia.com/docs/latest/Concepts/Connection-Styles.html)).

For Aicadia, this becomes three relations with different lifetimes:

```text
authenticated account  session-to-account; transient transport/authentication fact
character assignment   account-to-character; durable across every logout
session puppeting       session-to-character; transient authority for this connection
```

Only explicit abandonment ends the durable assignment. A network or MCP disconnect
ends the session relations and must never mean abandonment.

### 3. Unpuppeting is not the same operation as abandonment

Evennia's ordinary unpuppet operation disengages live control. The account's
playable-character collection is a different piece of state, so the same Character
can be selected again later. Aicadia needs the same conceptual distinction:

- disconnect or end session: keep the `character_control` row;
- explicit abandon: append a `world_event` whose projection removes that row;
- account deletion: never cascade into the character or entity.

There is one Evennia default that Aicadia should specifically not copy.
`DefaultCharacter.at_post_unpuppet` moves a disconnected character into a `None`
storage location so that it does not remain "headless" in a room; it restores the
character on the next puppet. This is an overrideable game policy in
[DefaultCharacter's API](https://www.evennia.com/docs/latest/api/evennia.objects.objects.html),
not a requirement of the Account/Object separation.

In Aicadia, connectivity is outside world history. The durable assignment and its
explicit abandonment are world-event-sourced control history, but they must not
silently move, rename, destroy or otherwise rewrite the character's fictional state.
If leaving the world also has a fictional action, that must be explicitly included
in the complete confirmed player action rather than inferred from transport state.

### 4. NPC is not required to be a different base identity

Evennia intentionally leaves NPC representation open. Its Object documentation says
that NPCs and monsters may be ordinary Objects, while another game may represent an
NPC as an unpuppeted Character. Either way, they share the same persistent Object
identity system
([Objects](https://www.evennia.com/docs/latest/Components/Objects.html)).

The official EvAdventure tutorial uses `DefaultCharacter` for both player
characters and NPCs. It gives PCs and NPCs different classes because that particular
game uses different combat rules, then shares living behavior through a mixin. The
[character tutorial](https://www.evennia.com/docs/latest/Howtos/Beginner-Tutorial/Part3/Beginner-Tutorial-Characters.html)
and
[NPC API](https://www.evennia.com/docs/latest/api/evennia.contrib.tutorials.evadventure.npcs.html)
show that `EvAdventureNPC` still derives from `DefaultCharacter`.

Two different ideas are therefore easy to confuse:

- character capability: this entity can occupy the character role in the game;
- current controller: a human account is presently entitled to act as it.

The first is durable world structure. The second can change. Encoding both as an
immutable `PlayerCharacter` versus `NpcCharacter` type would make abandonment,
adoption or temporary control require identity or type migration.

### 5. Typeclasses provide a stable object table with extensible behavior

Evennia's typeclasses let many Python types share a small set of real database
models. `ObjectDB` is the real in-game object table; Object, Character, Room and Exit
are proxy types that add Python behavior without adding one database table per
subclass. Typeclass instances also have stable database ids, persistent Attributes,
Tags and locks. The official
[typeclass documentation](https://www.evennia.com/docs/latest/Components/Typeclasses.html)
explains this database/proxy split and notes that object database references are not
reused.

This supports two useful conclusions for Aicadia:

1. A small universal identity core can support many in-world forms.
2. A new discovered kind does not need its own table or Rust enum.

Evennia's exact mechanism is less suitable for Aicadia. A typeclass couples a
persisted object's semantic type to server code. That works when the game developer
predefines `Rose`, `ShopKeeper` or `Mob`, but Aicadia's players and agents must be
able to discover species, materials, peoples and institutions without installing a
new server class.

The EvAdventure tutorial demonstrates the same tradeoff for species-like data: it
suggests storing character race and class as persistent Attributes
([race and class example](https://www.evennia.com/docs/latest/Howtos/Beginner-Tutorial/Part3/Beginner-Tutorial-Characters.html#about-races-and-classes)).
That is convenient mutable state, but a free attribute does not by itself provide a
shared identity for the species, source provenance, competing classifications or
historical replacement. Aicadia should not treat schema-free attribute storage as
equivalent to an open, provenance-carrying world model.

### 6. Composition helps reuse mechanics, but is not the world ontology

Evennia also offers a component system on top of typeclasses. Components can be
attached to a whole typeclass or one runtime instance, can carry persistent fields,
and allow features to be reused without inheritance. Its own documentation lists
the cost: extra complexity and a required host typeclass
([Evennia Components](https://www.evennia.com/docs/latest/Contribs/Contrib-Components.html)).

This is useful when a mechanically enforced feature such as health is shared across
several programmed actor types. It is not a reason for Aicadia to build a generic ECS
or component framework for all discovered content. A discovered species, material
or culture is world data, not a Rust behavior component. Components should only be
introduced when two current mechanical roles demonstrably need the same coded
behavior.

## LambdaMOO comparison

LambdaMOO uses one persistent object model for people, rooms, exits and other
concrete things. An object receives a unique number, continues to exist until it is
explicitly recycled, and never has its number reused. A character is an object with
the player flag set. The server can remove that flag, immediately preventing login
and disconnecting the current connection, without recycling the underlying object
([LambdaMOO Programmer's Manual](https://lambda.moo.mud.org/pub/MOO/ProgrammersManual.html)).

That is an early example of a valuable property:

> Removing player capability does not require destroying world identity.

LambdaMOO also keeps network connection state distinct enough to list all player
objects separately from currently connected player objects. A new connection starts
unassociated and becomes associated with a player object after login. Only one
connection may control one player object at a time
([connection association](https://lambda.moo.mud.org/pub/MOO/ProgrammersManual.html)).

The limitation is that LambdaMOO does not give the human account a separate durable
identity. The player object's inherited properties include its password, while the
same object is also its world embodiment. This makes the object/player flag elegant
but couples authentication, control rights and fictional identity. Evennia's
Account/Character separation is the safer source for Aicadia's boundary.

## Cross-system comparison

| System | How it models the problem | Useful lesson for Aicadia | What not to copy |
|---|---|---|---|
| Evennia | An out-of-character Account controls a persistent Character Object through a transient Session/puppet relation ([Accounts](https://www.evennia.com/docs/latest/Components/Accounts.html), [Objects](https://www.evennia.com/docs/latest/Components/Objects.html)) | Separate authentication, durable entitlement, live control and world identity | Typeclasses and arbitrary attributes as the player-authored world ontology |
| LambdaMOO | One persistent object model; a detachable player flag grants login capability ([Programmer's Manual](https://lambda.moo.mud.org/pub/MOO/ProgrammersManual.html)) | Removing player capability need not destroy the object | Password/login identity coupled to the world object |
| Bevy and Flecs | Entities are runtime identifiers composed from components; Flecs also represents relations as `(relation, target)` pairs ([Bevy ECS](https://docs.rs/bevy/latest/bevy/ecs/), [Flecs entity and component](https://www.flecs.dev/flecs/md_docs_2EntitiesComponents.html), [Flecs relation](https://www.flecs.dev/flecs/md_docs_2Relationships.html)) | Mechanical roles and relations compose better than a deep inheritance tree | Treating discovered semantic kinds as compiled component types, or an ECS world as durable history |
| Space Station 14 | A mob may be player- or NPC-controlled; control is attached using player-control/Mind components ([ECS guide](https://docs.spacestation14.com/en/robust-toolbox/ecs.html), [body guide](https://docs.spacestation14.com/en/space-station-14/core-tech/body.html)) | Player control is a capability/relation, not the entity's permanent class | Its high-frequency round simulation architecture for a low-frequency authored persistent world |
| TrinityCore | Authentication accounts and persistent characters are separate; players and template-backed creatures use distinct fixed models ([account](https://trinitycore.info/database/master/auth/account), [character](https://trinitycore.info/database/master/characters/characters), [creature](https://trinitycore.info/database/master/world/creature)) | Account/character separation and database-enforced identity are proven MMO conventions | Fixed race/class columns and separate closed Player/Creature taxonomies for an open discovery game |
| Veloren | A Rust MMO uses ECS for runtime simulation and a separate database for durable character state ([ECS](https://book.veloren.net/contributors/developers/ecs.html), [server configuration](https://book.veloren.net/players/server-hosting/configuration.html)) | Rust does not make ECS the persistence model; runtime and durable identity are different concerns | Building a frame-oriented ECS before Aicadia has frame-oriented mechanics |

### Runtime composition does not provide durable world identity

Bevy says an Entity identifies a runtime entity and explicitly warns that direct
entity serialization is not a stable long-term wire format; identifiers may refer
to despawned entities and may be reused with a different generation
([Bevy Entity](https://docs.rs/bevy/latest/bevy/ecs/prelude/struct.Entity.html)).
Flecs likewise describes an entity as a versioned 64-bit id whose underlying id can
be recycled after deletion
([Flecs entities and components](https://www.flecs.dev/flecs/md_docs_2EntitiesComponents.html)).

That makes both libraries useful implementation tools for a future hot simulation,
but unsuitable as the source of Aicadia's durable public ids. If an ECS is later
earned, its entity id should map to an Aicadia `EntityId`; it must never replace it.
The current authored-action loop does not perform large component queries every
frame, so adding ECS now solves a workload the MVP does not have.

### A classic MMO schema is strong on mechanics and closed on meaning

TrinityCore's schema stores an account id on each character and gives a character
fixed race and class fields. Creatures are spawn rows referring to predefined
creature templates; every spawned creature requires such a template
([character table](https://trinitycore.info/database/master/characters/characters),
[creature template](https://trinitycore.atlassian.net/wiki/spaces/tc/pages/1203568708/creature%2Btemplate%2Bmaster)).
That is appropriate for a game whose ontology is authored in advance. It is the
wrong extension axis for Aicadia: a newly discovered fungus or people should append
entities and claims, not require a template category, Rust variant or migration.

The transferable part is narrower: accounts and characters are different durable
records, and mechanically enforced character state may have typed storage. The
closed `Player` versus `Creature` hierarchy and fixed classification columns should
not cross that boundary.

## Model tradeoffs exposed by the research

The following comparison is an inference from the source systems, not a claim that
Evennia itself prescribes Aicadia's schema.

| Model | Immediate benefit | Cost for Aicadia |
|---|---|---|
| `character` has an identity unrelated to `entity` | Explicit separation | Every world relation must accept two identity domains or add a permanent join; an NPC-to-PC transition risks identity confusion |
| `entity.kind = character | flora | fauna | material | ...` | One table and simple branching | Kinds become closed and mutually exclusive; discovered content requires migrations and code changes |
| Class/typeclass hierarchy for world kinds | Behavior reuse and no table per subclass | Discovered semantics become server code; multiple classifications and changing world taxonomy fit poorly |
| Arbitrary attribute or JSON bag | Fast schema-free addition | Weak constraints, poor shared identity, difficult indexing, and no inherent source history |
| `entity` plus sparse mechanical role rows | One world identity with typed mechanical capabilities | Adds a join and requires role invariants, but does not prescribe the world's semantic kinds |

The last model is the best fit from this research slice. It means neither "copy all
fields into one Entity struct" nor "give Character a second identity." It means:

```text
entity.id == character.entity_id
```

where the `character` row exists only for entities that need character mechanics.

## Recommended Aicadia MVP candidate

### Minimal durable records

For one world, the evidence supports this candidate:

```text
account
  id

entity
  id

character
  entity_id primary key -> entity.id

character_control                         rebuildable current projection
  character_id primary key -> character.entity_id
  account_id unique -> account.id
  source_world_event_id -> world_event.id
```

There is no `world_id` yet because there is one world, no `npc` table, no
`entity_kind` enum, no `character.owner_account_id`, and no generic component
framework.

The shared primary key and current-control relation are deliberate:

- `character.entity_id` gives the role the entity's existing world identity rather
  than allocating a second id;
- `character_control.character_id` as the primary key allows each character zero or
  one assigned account;
- `character_control.account_id` as unique allows each account zero or one assigned
  character;
- `source_world_event_id` retains the provenance of the current assignment;
- removing an account cannot cascade into character deletion; and
- the character record contains only world mechanics, not authentication identity.

`active` here means an existing `character_control` assignment, not online. The row
survives login, logout, MCP reconnects and server restarts. A session or connected
agent is transient transport state and is not stored in this projection.

`character_control` stores current state only. Assignment and abandonment history
stays in immutable `world_event`; replay selects the current row or its absence. It
does not need `started_at`, `ended_at`, a status enum or soft-deleted rows. If
temporary delegation or multiple simultaneous controller types becomes a confirmed
scenario, that is a later contract rather than dormant columns in this one.

### Exact transitions

```text
create player character
  require no character_control exists for the account
  append the confirmed world_event
  project its entity and character using the same id
  insert character_control with that source world_event id

disconnect
  change no durable domain state
  remove only transient session authentication and puppeting

abandon character
  require character_control matches the account and character
  append the explicitly confirmed abandonment world_event
  remove character_control while projecting that event
  leave entity and character unchanged

create NPC
  append the accepted source world_event
  project its entity and character using the same id
  insert no character_control
```

Bootstrap data may use the same event and projection path. This research does not
decide the final public payload for character creation or abandonment, only that the
current relation retains a source event and its history is not stored by mutating the
character.

### Mechanical meaning of NPC

For this MVP, `NPC` is a derived game term:

```text
NPC = character with no current character_control row
```

An offline but not abandoned player character is not an NPC because its durable
control row remains. An abandoned character is an NPC without losing any name,
location, relationship, history or entity id. A native NPC has the same structure
but no prior assignment. If that distinction matters in a query, its source
world-event history can answer it; it does not require two character base types or a
stored permanent `is_npc` flag.

NPC existence also does not authorize autonomous server behavior. Under Aicadia's
no-unconscious-token-burn boundary, an unassigned character can remain visible and
referable without the server invoking an agent for it.

### Species, people and lineage

Species or the user's colloquial "race" does not change the identity model. A
character's entity can be related to another entity representing a species, people,
lineage or other discovered classification. Those are world assertions, not reasons
to add `race_id` to the mechanical `character` role or variants to a Rust enum.

This keeps two extension directions separate:

- mechanical expansion: add a narrow role or field only when deterministic game
  behavior requires it;
- semantic expansion: add entities and sourced world relations without changing
  Rust code.

### Small Rust shape

The domain relationship can remain explicit in Rust without adopting an ECS or a
hexagonal framework:

```rust
pub struct EntityId(/* opaque id */);
pub struct AccountId(/* opaque id */);
pub struct WorldEventId(/* opaque id */);

pub struct Entity {
    id: EntityId,
}

pub struct Character {
    entity_id: EntityId,
}

pub struct CharacterControl {
    character_id: EntityId,
    account_id: AccountId,
    source_world_event_id: WorldEventId,
}
```

An API may expose a validated `CharacterId` newtype for type safety, but it should
wrap the same persistent entity id rather than allocate another id.

The production MVP should use PostgreSQL immediately. Persistence, concurrent
character creation and restart survival are part of the smallest multiplayer loop,
so an in-memory `World` would answer the wrong question. `World` remains the game
concept and application boundary; it should not become one process-wide struct that
loads every entity or serializes every action behind one lock. PostgreSQL stores the
event log and projections, while each read fetches only the local state needed by
the current action.

The two cardinality rules belong in the database as the primary key on
`character_control.character_id` and a unique constraint on
`character_control.account_id`. PostgreSQL documents primary-key, unique and
foreign-key enforcement in
[CREATE TABLE](https://www.postgresql.org/docs/current/sql-createtable.html).
Application validation can return the intended `rule` slug, but the transaction and
constraint must still decide concurrent attempts. A check followed by an
unprotected insert would allow two simultaneous sessions to create two active
characters for one account.

There is no need for a temporal control table in the MVP. An interval table with an
open-ended row plus partial unique indexes could enforce one open assignment per
account and character; PostgreSQL explicitly supports uniqueness over only the rows
matching a predicate
([partial indexes](https://www.postgresql.org/docs/current/indexes-partial.html)).
That becomes useful only if control-period queries become a current product need.
Today it would duplicate history already retained by `world_event`. The one-row
current projection is smaller and fully rebuildable.

### Architecture boundary

Cockburn's original Ports and Adapters description is about keeping the application
drivable without a particular UI or database, with ports representing purposeful
conversations and adapters translating technologies
([Hexagonal Architecture](https://alistair.cockburn.us/hexagonal-architecture)).
That boundary is useful here, but it does not require folders named `port`,
`adapter`, `repository` and `service`, or a trait for every database call.

The smallest shape is one Rust package and one deployable server:

```text
src/main.rs       process wiring
src/lib.rs        public application entry points
game/src/world/        ids, commands, validation and transitions
src/postgres.rs   SQL transactions and projection writes
```

The MCP adapter and later web endpoint call the same application entry point. A
confirmed action is handled in one transaction: authorize the account against
`character_control`, validate declared rules, append one `world_event`, then update
the projections idempotently for that event. SQL remains concrete until a second
storage implementation or a hard testing seam actually exists. Pure transition
functions can be unit-tested; database cardinality and transaction behavior need
PostgreSQL integration tests.

This is the useful intent of hexagonal architecture with none of its speculative
ceremony. If a second real adapter appears, the existing application entry point is
already the port; the code can be extracted then.

### Where this model can scale

"Infinitely scalable" should mean open-ended in the right dimension, not literally
unbounded infrastructure. The semantic dimension is open: a new animal, plant,
material, settlement or species adds ordinary `entity` and `claim` data without a
schema or binary change. The mechanical dimension stays intentionally closed:
`character` exists because the server must deterministically enforce who may act.

Operationally, one PostgreSQL database, indexed foreign keys, bounded local reads
and rebuildable projections are enough for the MVP. Stateless server replicas,
partitioning or a separate hot simulation can be introduced only after measured
pressure identifies which path needs it. None requires changing the durable
account/entity/character/control identities proposed here.

### Smallest end-to-end proof

The first executable slice needs only two accounts and this sequence: account A
creates character Elian; a concurrent second creation is rejected; A disconnects
and reconnects to the same character; A explicitly confirms abandonment; Elian
remains observable as an uncontrolled character; A creates Mara; account B reads
Mara, Elian and both source events. Restarting the server must preserve the same
result. The next discovery can introduce a flora kind and one specimen through
entities and claims without changing Rust or SQL structure.

### First invariants to test

1. A character cannot exist without its entity.
2. An entity may exist without being a character.
3. An account appears in at most one current `character_control` row.
4. A character appears in at most one current `character_control` row.
5. Every current control row names the world event that established it.
6. Disconnecting does not change current control or world state.
7. Abandoning appends history and removes current control while preserving the
   character and entity.
8. Replaying world events reconstructs the same current control relation.
9. Absence of current control is sufficient to identify an NPC; there is no stored
   permanent NPC kind.
10. An unassigned character and an assigned character use the same entity-reference
    path in world data.
11. Account deletion never deletes or moves a character.

These tests prove the current product requirement while preserving the seam needed
for NPCs and open-ended discovered entities. They do not require typeclasses, ECS,
ports, adapters or a generalized role framework.

## Open questions not answered by this source slice

1. Can any abandoned character later be adopted, or does control only ever move from
   assigned to permanently unassigned?
2. What exact public payload establishes or ends `character_control` in a
   `world_event`?
3. Is initial character creation an ordinary confirmed player action or a distinct
   account setup flow that still appends the same source event shape?
4. Which properties are genuinely mechanical character state in the first playable
   loop?
5. Should `CharacterId` be a public newtype over `EntityId`, or should APIs use
   `EntityId` plus explicit character-role validation?

## Sources

- [Evennia Accounts](https://www.evennia.com/docs/latest/Components/Accounts.html)
- [Evennia Account API](https://www.evennia.com/docs/latest/api/evennia.accounts.accounts.html)
- [Evennia DefaultAccount source](https://www.evennia.com/docs/latest/_modules/evennia/accounts/accounts.html)
- [Evennia Objects](https://www.evennia.com/docs/latest/Components/Objects.html)
- [Evennia Characters](https://www.evennia.com/docs/latest/Components/Characters.html)
- [Evennia DefaultCharacter API](https://www.evennia.com/docs/latest/api/evennia.objects.objects.html)
- [Evennia connection styles](https://www.evennia.com/docs/latest/Concepts/Connection-Styles.html)
- [Evennia typeclasses](https://www.evennia.com/docs/latest/Components/Typeclasses.html)
- [Evennia components](https://www.evennia.com/docs/latest/Contribs/Contrib-Components.html)
- [Evennia player-character tutorial](https://www.evennia.com/docs/latest/Howtos/Beginner-Tutorial/Part3/Beginner-Tutorial-Characters.html)
- [Evennia EvAdventure NPC API](https://www.evennia.com/docs/latest/api/evennia.contrib.tutorials.evadventure.npcs.html)
- [LambdaMOO Programmer's Manual](https://lambda.moo.mud.org/pub/MOO/ProgrammersManual.html)
- [Bevy ECS](https://docs.rs/bevy/latest/bevy/ecs/)
- [Bevy Entity](https://docs.rs/bevy/latest/bevy/ecs/prelude/struct.Entity.html)
- [Flecs entities and components](https://www.flecs.dev/flecs/md_docs_2EntitiesComponents.html)
- [Flecs relationships](https://www.flecs.dev/flecs/md_docs_2Relationships.html)
- [Space Station 14 ECS guide](https://docs.spacestation14.com/en/robust-toolbox/ecs.html)
- [Space Station 14 body guide](https://docs.spacestation14.com/en/space-station-14/core-tech/body.html)
- [TrinityCore account table](https://trinitycore.info/database/master/auth/account)
- [TrinityCore character table](https://trinitycore.info/database/master/characters/characters)
- [TrinityCore creature table](https://trinitycore.info/database/master/world/creature)
- [TrinityCore creature template](https://trinitycore.atlassian.net/wiki/spaces/tc/pages/1203568708/creature%2Btemplate%2Bmaster)
- [Veloren ECS](https://book.veloren.net/contributors/developers/ecs.html)
- [Veloren server configuration](https://book.veloren.net/players/server-hosting/configuration.html)
- [PostgreSQL `CREATE TABLE`](https://www.postgresql.org/docs/current/sql-createtable.html)
- [PostgreSQL partial indexes](https://www.postgresql.org/docs/current/indexes-partial.html)
- [Alistair Cockburn, Hexagonal Architecture](https://alistair.cockburn.us/hexagonal-architecture)
