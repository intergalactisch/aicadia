---
status: complete
created_at: "2026-08-13T17:12:01+02:00"
updated_at: "2026-08-13T19:44:24+02:00"
accepted_at: "2026-08-13T18:00:14+02:00"
completed_at: "2026-08-13T19:44:24+02:00"
---

# Local Entity Property state

## Outcome

Every existing Entity-creation route can establish 0–100 initial bounded text or
integer Properties atomically. One confirmed World Action can change 1–100 unique
Entity/key pairs across 1–100 exact-current-Place Entities, while one Interaction
can optionally change 0–100 Properties of its actor and explicit targets. World
stores immutable Activity-backed values plus one current pointer, hydrates exact
typed changes into authorized Activity, and exposes one flat outward/local current
Property read through equivalent `World`, HTTP and MCP interfaces.

This is the highest-value edge after Interaction because durable subjects still
cannot change independently queryable state: red hair or three legs exists only as
incidental prose. Final evidence must prove uniform Entity creation, local
multi-Entity consequence, Interaction consequence, current/history reads, retry,
race, rollback and bounded set-based performance without a generic patch language,
global knowledge or control-provenance oracle.

## Non-goals

- Property unset/deletion, key rename, aliases or synonym inference;
- Traits or any Trait lineage/current/retirement storage;
- possession, wearing or other Entity relations;
- volition, thought, response, consent or relationship state;
- placement/movement changes;
- remote/cross-Place subjects or dynamic, prose-derived or area selectors;
- mixed Action consequence kinds in one Action: `introduce_entity` and
  `change_entity_property` remain homogeneous alternatives;
- global/reverse Property search, admin ontology tools, JSONB bags, per-key tables,
  event sourcing or server inference.

## Evidence and authorities

| Evidence or authority | Current fact | Consequence |
| --- | --- | --- |
| `game/docs/README.md` | Four current operations create the same Entity identity: `create_entity`, `create_character`, `create_entry_place`, `submit_action.introduce_entity` | Give all four the same optional initial-Property input and atomicity |
| `game/docs/README.md`, `game/src/world.rs` | Action and Interaction are separate `World` operations with shared Activity/freshness/retry infrastructure | Keep distinct public semantics; share private typed Property writer |
| Earlier User direction | A Property change is Agent-proposed or arises through an Action or Interaction; Action is not Interaction | Include bounded consequences in both existing operations; deferring Interaction would conflict |
| Earlier User direction | Explosion may change every eligible local Entity, including a Character; control provenance is never revealed | Use uniform exact-local Entity eligibility, neutral failure and atomic multi-write |
| `dev/CONTEXT.md` | Property is compact key/value; Trait is the explanatory statement | Store key and type only; no manufactured `property_key.description` |
| `activity`, `activity_entity` | Activity is immutable history with actor, Place, operation, prose, roles and accepted request identity | Property history references the same Activity; hydrate changes rather than infer from prose |
| Exact-current-Place reads | World can already derive actor, Place, co-present Characters and ordinary Entities without control labels | Reuse this bounded set for Action eligibility and outward current reads |
| `dev/docs/concept/11-entity-traits-and-change.md` | Entity-owned `(Entity, key)` state, text/integer and pointer-only current storage are aligned | Create normalized shared keys, immutable value history and current pointer |
| Capability map/backlog | Property follows completed Interaction and precedes separate Trait work | Keep one Proposed Now edge and no Trait schema |

## Alignment

### Strategic

Properties make accepted physical/descriptive changes constrain future situated play
without scores or an omniscient narrator. One shared Entity model means Character,
Place and ordinary Entity state behaves consistently; outward local reads let those
changes matter socially. The accepted uniform eligibility deliberately permits one
confirmed Action to change another User's Character and the Place without revealing
their role or control provenance.

### Tactical

Initial `property` lists are optional, semantically unordered and bounded 0–100.
Each existing creation route commits Entity, role/placement, its existing Activity,
keys, history and pointers together. Initial duplicate canonical keys reject the
whole creation.

`submit_action` gains homogeneous `change_entity_property` with 1–100 writes across
1–100 unique exact-local Entity ids. Eligible subjects are actor, current Place,
co-present Characters and placed ordinary Entities. Every `(entity_id,key)` is
unique; role/control never affects validation or error detail.

`submit_interaction` keeps its existing outward behavior and gains optional
`property_change` 0–100. Every changed Entity must be actor or explicit target.
Empty is today's Interaction. Non-empty commits outward Activity/participation and
typed changes together without authoring a target response.

All changes are outward/local current facts. One separate paged Property read
returns flat `(Entity summary,key,typed value)` rows for the exact local set.
Authorized Activity returns its exact sorted typed Property changes. Structured
current Property overrides conflicting introduction for that key; introduction stays
immutable history.

### Technical

The `World` seam owns normalization, eligibility, key arbitration, locks, set-based
writes, result hydration, retry and reads. PostgreSQL adds `property_key`, immutable
`entity_property_history`, pointer-only `entity_property`, one Action discriminator
and only earned indexes. HTTP/MCP remain thin. Agent guidance teaches initial values,
Action versus Interaction consequences, complete preview and current-fact priority.

## Decisions and assumptions

### Confirmed decisions

- Property is one compact Entity-owned `key = value`; shared key stores canonical
  English lower-snake key, immutable type and first Activity provenance only.
- Text and signed 64-bit integer are the only value types.
- All four Entity creation routes accept the same 0–100 initial Property shape.
- `submit_action.change_entity_property` is a homogeneous 1–100 write consequence;
  `submit_interaction` optionally carries 0–100 actor/target changes.
- Property values live once in immutable history; current table is only
  `(entity_id,property_key_id,current_activity_id)`.
- Keys are Agent-created at first accepted use. Same key/type reuses; same key with a
  different type yields `property_key_conflict`; aliases/inference are absent.
- Current Properties are outward/local and override conflicting introductory prose
  for the same exact meaning.
- Activity output hydrates exact typed changes; one separate flat current-local read
  avoids nested arrays and N+1 calls.
- Trait remains separate later work with no storage decision here.
- Every Entity can carry zero or more Properties, including furniture, flora, fauna,
  Characters and Places; Entity roles do not fork the Property model.
- A User never directly replaces their own or another Entity's Property and receives
  no profile-edit/storage command. The User steers and confirms; the Agent proposes
  an exact Action or Interaction; World alone validates and writes.
- Uniform local World Action eligibility includes other co-present Characters and
  the current Place. Typed outward physical state does not require role/control
  branching or imply consent, volition or response.
- A later explicitly accepted deterministic mechanic may reuse the private Property
  writer for an external factor. The current slice admits only explicitly submitted,
  confirmed Agent-authored Action/Interaction causes: no autonomous/background
  Agent, `world_event` table, timer or ungrounded simulation exists.

### Reversible assumptions

- Key is 1–64 ASCII lower-snake-case characters starting with a letter; text value
  is trimmed non-NUL 1–4,000 Unicode characters; list bounds are 100.
- Current read paginates by `(entity_id,property_key_id)` with opaque cursor,
  default 25/max 100; Activity changes sort by Entity id/key.
- Property input/value is one strict tagged union such as
  `{key, value:{type:"text",text}}` or `{key,value:{type:"integer",integer}}`.
- Neutral `property_entity_unavailable` covers every missing, remote or ineligible
  Action/Interaction change subject; structural duplicate/bound errors remain typed
  invalid input and key/type mismatch is `property_key_conflict`.

### Resolved acceptance boundary

On 2026-08-13 the User explicitly confirmed the uniform rule: every Entity can carry
Properties, players never directly edit Property storage, and World may apply an
Agent-proposed confirmed local consequence uniformly to actor, current Place,
ordinary Entities and other Characters. This resolves the final material question
and accepts the complete plan. No material open question remains.

## Proposed database contract

Migration `0007_entity_property.sql`:

```text
activity
  + action_consequence text null
  check: submit_action => introduce_entity | change_entity_property
         all other operations => null
  backfill existing submit_action rows as introduce_entity

property_key
  id bigint generated always as identity primary key
  key text not null unique + canonical format/length check
  value_type text not null check (text | integer)
  first_activity_id uuid not null FK activity ON DELETE RESTRICT
  unique (id,value_type)

entity_property_history
  entity_id uuid not null FK entity ON DELETE RESTRICT
  property_key_id bigint not null
  activity_id uuid not null FK activity ON DELETE RESTRICT
  previous_activity_id uuid null
  value_type text not null
  text_value text null
  integer_value bigint null
  PK (entity_id,property_key_id,activity_id)
  FK (property_key_id,value_type) -> property_key(id,value_type)
  FK (entity_id,property_key_id,previous_activity_id)
    -> entity_property_history(entity_id,property_key_id,activity_id)
  check exactly one matching bounded value
  append-only UPDATE/DELETE trigger
  index (activity_id,entity_id,property_key_id)

entity_property
  entity_id uuid not null FK entity ON DELETE RESTRICT
  property_key_id bigint not null FK property_key ON DELETE RESTRICT
  current_activity_id uuid not null
  PK (entity_id,property_key_id)
  FK (entity_id,property_key_id,current_activity_id)
    -> entity_property_history(entity_id,property_key_id,activity_id)
```

`activity_id` in history is intentionally non-unique because one creation, Action or
Interaction may change up to 100 Properties. Its index hydrates all changes for one
or many Activity ids. Primary keys serve Entity/key current and predecessor access;
the unique key serves lookup/race arbitration. No reverse-value index is added.

Activity is inserted before first-use key/history so provenance FKs are real. The
shared writer sorts keys and Entity/key pairs, obtains existing pointer locks in
stable order, arbitrates first-use keys in stable order, bulk-inserts history and
bulk-upserts pointers. Route-specific Entity/role/location writes share the same
transaction. Any error rolls back the complete bundle.

## Public interface contract

### Creation

`CreateEntity`, `CreateCharacter`, `CreateEntryPlace` and the `introduce_entity`
Action variant gain `property: []` (default empty, max 100). Each item has canonical
key plus tagged value. Their result shapes remain the relevant Entity/Character/
Place or introduced-Entity Action result; authorized Activity/current reads expose
the stored Properties.

The non-request-id creation routes retain current concurrency behavior: Character
uniqueness, entry-Place uniqueness and ordinary Entity creation semantics arbitrate
the whole bundle. No speculative idempotency layer is added. The introduce Action
retains exact request-id retry.

### Action

`SubmitAction.consequence` becomes a strict tagged union:

- `introduce_entity { name,description,property[0..100] }`;
- `change_entity_property { property_change[1..100] }`.

Each change contains Entity id, key and typed value. The list is semantic unordered;
fingerprinting sorts normalized `(entity_id,key,type,value)`. The accepted Action
result becomes a tagged consequence: introduced Entity or exact sorted Property
changes, plus Activity and Place. Stored `activity.action_consequence` makes retry
reconstruction exact instead of inferring meaning from participation roles. Affected
Action Entities receive `subject` Activity participation; Place also remains
`location`.

### Interaction

`SubmitInteraction` gains `property_change[0..100]`, default empty. Fingerprinting
adds its sorted normalized writes after the existing unordered target set. Equal
retry returns the same Activity hydrated with exact changes; changed writes conflict.
Each changed Entity must equal actor or an explicit target, otherwise the whole call
returns neutral `property_entity_unavailable`. Existing target validation and no-
response meaning remain unchanged. No new operation/tool is added.

### Reads

Add `list_entity_property_at_current_place` through World, HTTP and MCP. It derives
Character/Place and returns Place, matching Place revision, flat current Property
rows and cursor. Its eligible Entity set is exactly actor, current Place, co-present
Characters and placed ordinary Entities. Activity wire output gains sorted
`property_change`; all existing personal/Place authorization continues to decide
which Activity is visible before hydration.

## Acceptance scenarios

| Scenario | Expected result/evidence |
| --- | --- |
| Each of four creation routes with 0 and 100 initial Properties | route semantics unchanged; one atomic Activity-backed bundle |
| Duplicate initial key or 101st value | whole Entity/role/location/Activity/key/history/pointer bundle absent |
| Explosion Action affects actor, ordinary Entity and another Character | three or more sorted writes in one Activity; all current pointers advance atomically |
| Action changes current Place plus local Entities | Place and Entities use identical Property rules; Place remains Activity location |
| Missing, remote or departed Action subject | neutral unavailable, zero writes and no control/existence detail |
| Interaction with no changes | byte-semantic current outward behavior retained |
| Interaction changes actor and explicit target | same Interaction Activity stores participation and exact changes; no target response implied |
| Interaction names local non-target in changes | neutral rejection and zero Activity/participation/Property state |
| One Activity changes same Entity/key twice | duplicate invalid; zero writes |
| Same key/type across Entities | one key, independent history/pointers |
| Concurrent new same key/type | one key; both complete according to route locks |
| Concurrent same key/different type | one winner; loser key conflict and whole transaction rollback |
| Equal Action/Interaction retry after later state | original Activity and exact original changes returned; no new writes |
| Same request id with reordered equivalent list | same fingerprint/result; changed value conflicts |
| Local current read at 100 rows | one set-based query/page using declared indexes; no remote/control data |
| Activity page containing multi-change Activities | one batched hydration query, deterministic changes, no N+1 |
| Intro says blond, current `hair_colour=red` | Agent presents red as current and may retain introduction only as past context |

## Implementation map

| Surface | Current state | Intended change | Invariants |
| --- | --- | --- | --- |
| concept/log/backlog/map | Complete Property direction accepted; executable contract not published | retain uniform creation/Action/Interaction direction and delivery state | `game/docs/` remains current until T2 |
| `game/docs/`, `AGENTS.md` | no Property; twelve tools | specify accepted contract and thirteen-tool surface in T2 | no premature executable claim |
| `migration/0007_entity_property.sql` | no Property/discriminator | column/backfill/check, three tables, minimal indexes/triggers | one value store; singular names; restrictive FKs |
| `game/src/world.rs`, `src/lib.rs` | four creation paths; introduce Action; outward Interaction | shared normalizer/key resolver/set writer, closed result unions, exact eligibility/read/hydration | one World seam; atomic; role/control agnostic |
| `game/src/wire.rs` | strict existing inputs/results | initial lists, tagged Action union/result, optional Interaction changes, flat read, Activity hydration/errors | unknown fields denied; internal key ids absent |
| `game/src/server.rs` | HTTP/MCP parity, twelve tools | one read endpoint/tool; mutation inputs deepen existing calls | adapters own no game rules |
| Agent contract files | no Property use guidance | teach key/value, initial state, Action/Interaction consequences, previews and precedence | player conversation in World; MCP sole authority |
| World/database tests | current atomicity/retry/privacy | all scenarios, locks, rollback, query counts/EXPLAIN/index shape | retain prior evidence |
| adapter/catalog/playtest | exact current catalog and rat/marker scenarios | strict parity, thirteen tools, deterministic Property scenario | no paid model required |

## Execution contract

Root owns outcome, scope, plan state, integration and final evidence. Each delegated
Sol High Agent receives this plan path and one dependency-ready task, re-reads live
files, owns only assigned surfaces, runs focused evidence and reports raw results.
Root alone maintains shared plan state. Parallel work is allowed only where marked
and write/evidence surfaces are independent.

## Task graph

| ID | State | Depends | Parallel-safe | Objective | Owned surfaces | Evidence |
| --- | --- | --- | --- | --- | --- | --- |
| T1 | completed | — | no | Obtain explicit acceptance of uniform local mutation and complete plan | plan/concept/log/backlog/map | User confirmed the uniform Entity rule; the accepted plan had no material open question |
| T2 | completed | T1 | no | Publish accepted game/Agent capability contract before code | `game/docs/`, `AGENTS.md`, planning status | authorities published exact target shapes/errors with an explicit pre-implementation boundary; links and diff check passed |
| T3 | completed | T2 | no | Implement schema and shared set-based persistence | migration, storage helpers, DB tests | 5 focused Property DB tests and all 53 World tests pass; constraints, races, rollback, indexes and forced-index query plans proved |
| T4 | completed | T3 | no | Deepen World creation, Action, Interaction, reads and hydration | world/lib and World tests | 62/62 World and 5/5 lib tests pass; all creation, Action, Interaction, hydration and scoped-read scenarios proven |
| T5 | completed | T4 | yes | Ship strict HTTP/MCP/wire/catalog parity | wire/server/adapter fixtures/tests | 12/12 server tests and exact runtime-generated thirteen-tool catalog pass |
| T6 | completed | T4 | yes | Teach Agents and prove bounded play semantics | Agent contract/playtest surfaces | 4/4 contract tests and token-free fake Agent controller pass; deterministic World/server evidence cited without paid-model claim |
| T7 | completed | T5,T6 | no | Integrate, independently review, align authorities and complete | plan/backlog/map/log/dev/docs/evidence | DB-backed 88/88 Rust tests, Agent contract 4/4, fake harness, exact 13-tool catalog, integrity checks and independent re-review pass with no P0–P3 finding |

## Task details

### T1 — Accept the uniform local boundary

The User accepted Action mutation of other local Characters and current Place under
the same Entity rule, clarified that players never directly edit Property storage,
and accepted the complete plan. Planning/concept/backlog surfaces now agree; T1 is
complete. No runtime, schema or current-game authority changed in this task.

### T2 — Publish accepted contract

Update current game docs, Agent interface and compact MVP/capability rule with all
creation, Action, Interaction, read, Activity, error and evidence semantics. Mark
backlog Ready, not executable. Link/diff audit must map each claim to plan.

Completed on 2026-08-13. `game/docs/README.md` and `agent-interface.md` publish the
exact thirteen-capability target while preserving the delivered twelve-capability
Interaction baseline; `AGENTS.md`, backlog, capability map and one concept-log entry
agree. All local links and anchors in the changed contract/planning documents
resolve, the required contract-term audit passes and `git diff --check` is clean.
T3 is dependency-ready but remains pending and has not started.

### T3 — Persist Property state

Add migration and shared normalization/resolution/write/hydration primitives. Prove
format/type/value checks, same-lineage pointers, history immutability, discriminator
backfill, non-unique Activity cardinality, first-key races, stable lock order, full
rollback, indexes, query count and `EXPLAIN` at bounds. Stop if N+1, deadlock or
partial state needs a wider model.

### T4 — Deepen World

Integrate initial Properties into all four creation routes, the homogeneous Action
variant, optional Interaction changes, tagged accepted results, exact retry
reconstruction, uniform local eligibility, flat current read and Activity hydration.
Prove every acceptance scenario and retain prior behavior. Stop if authorization
depends on role/control disclosure or prose inference.

### T5 — Ship adapter parity

Implement strict tagged wire types, error mappings, one HTTP/MCP read, deeper existing
mutation calls and exact catalog/OpenAPI fixtures. Prove HTTP/MCP semantic equality,
unknown-field rejection and absence of internal key ids/control data.

### T6 — Teach Agent use

Update global/tool descriptions for initial Properties, canonical keys, local
orientation, complete multi-change preview/confirmation, Action versus Interaction,
no target response and structured-current precedence. Deterministic evidence proves
correct MCP-only flow without claiming stochastic model obedience.

### T7 — Integrate and review

Run full validation, independent Standards/Spec review, resolve findings and align
game docs, concept log, capability map, backlog and plan to delivered truth. Do not
start Trait or another edge.

Completed on 2026-08-13. Final DB-backed evidence passes 88/88 Rust tests: 11 library,
2 playtest-database binary, 12 server and 63 World tests. Agent-contract tests pass
4/4, the token-free fake Agent controller passes, and the runtime-generated catalog
contains the exact 13 tools. Formatter, strict all-target/all-feature Clippy, shell
syntax, relative-link/anchor, stale-authority and `git diff --check` gates pass. The
independent final re-review found no P0–P3 issue after the evidence count correction.
Current game docs, Agent guidance and planning agree; Property is Done with no Active
backlog item. Trait and every next edge remain unstarted.

## Validation ladder

1. **Focused:** migration constraints, all creation routes, Action/Interaction
   changes, retry/fingerprint, concurrency, history/current and scoped reads.
2. **Performance:** bounded 100-write/read query counts, stable lock order,
   `EXPLAIN`/index shape and explicit absence of N+1.
3. **Contract:** strict World/HTTP/MCP parity, Activity hydration, errors, catalog and
   Agent guidance with no global/control leak.
4. **Outcome:** one continuous deterministic scenario creates role-diverse Entities
   with initial Properties, changes actor/Place/ordinary/other Character through one
   Action, changes actor/target through Interaction, and reads exact current/history.
5. **Integrity:** full Rust tests, formatter, strict Clippy, fixtures, shell/link
   checks, `git diff --check`, focused review and preservation of unrelated work.

## Change control

Refine paths/order/evidence while accepted meaning is unchanged. Stop, return to
`draft`, revise and regain acceptance if actor/scope, Property meaning, creation
uniformity, Action/Interaction semantics, visibility, irreversible state, public
contract or evidence claim changes.

## Completion conditions

All tasks and validation pass; exact outcome is demonstrated; current behavior,
vocabulary, Agent contract and planning agree; no stale authority/material question
or unrelated damage remains; only then set `complete` and `completed_at`.
