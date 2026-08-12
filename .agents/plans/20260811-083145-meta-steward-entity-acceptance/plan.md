---
status: dropped
created_at: 2026-08-11T08:31:45+02:00
updated_at: 2026-08-11T08:56:52+02:00
accepted_at: null
completed_at: null
dropped_at: 2026-08-11T08:56:52+02:00
dropped_reason: The User rejected a human administrator and private meta-steward flow; an Agent requests and World deterministically accepts or rejects.
---

# Meta-steward Entity acceptance

Planning state: dropped. This draft preserves a rejected interpretation for the
decision trail only. It must not be accepted or executed.

## Outcome

An administrator consciously starts a meta Agent, reviews its proposed Entity content
and explicitly confirms one complete package. `World` then deterministically accepts
or rejects the command and, only on acceptance, atomically creates the Entity, its
optional placement and durable Activity with private meta-admin provenance.

The Agent never writes World state, the server never invokes an Agent and unchosen,
unconfirmed or rejected candidates leave no durable record. Raw `create_entity`
ceases to be a generic player-facing HTTP or MCP capability; it remains authoritative
World behavior reached only through this private operator flow until a concrete
player-gameplay acceptance flow earns the same behavior later.

This is the highest-value current edge because Exact-Place reading needs legitimate
established state, while arbitrary-Place authoring is World stewardship rather than
Character play. Final evidence must prove one consciously confirmed candidate becomes
exactly one placed or unlocated Entity with exact provenance/history, while a
do-nothing, unconfirmed, malformed or rejected path creates nothing and player Agents
cannot invoke the raw mutation.

## Non-goals

- background, scheduled or server-triggered Agents or token spend;
- player discovery, investigation, Character actions or Exact-Place reading;
- a web admin application, OAuth or general network authentication;
- movement, relocation, update or deletion of an Entity;
- scenes, claims, rules, world events or a generic proposal framework;
- automatic semantic judging by World;
- retaining conversations, rejected candidates or unchosen options;
- giving the meta Agent durable World identity or authority.

## Evidence and authorities

| Evidence or authority | Current fact | Consequence for this build |
| --- | --- | --- |
| `docs/game/README.md` | `World` owns behavior; player HTTP/MCP currently expose raw `create_entity` | Deliberately remove the player adapter while retaining authoritative World creation |
| `docs/game/agent-interface.md` | The current catalog has ten player capabilities including `create_entity` | Catalog, HTTP, MCP, OpenAPI and parity fixture must change together |
| `src/bin/aicadia-provision-user.rs` | Private operations already use a local binary, database credentials and direct `World` calls | Prefer a private operator CLI over a new admin network surface |
| `src/world.rs` | `create_entity` validates and commits Entity plus Activity atomically | Deepen this World behavior with optional Place and meta provenance, not a second writer |
| `AGENTS.md` | Administration is not a player Agent tool; server-side inference and unconscious token spend are forbidden | Meta flow stays private, consciously invoked and external to the player MCP catalog |
| `CONTEXT.md` | Agent proposes, User supplies provenance and World alone accepts durable state | Preserve these three distinct responsibilities |
| `docs/concept/03-time-and-turns.md` | Prior steward direction uses conscious invocation, private candidates, explicit confirmation and no server Agent | Reuse only the accepted interaction constraints that fit the current Entity model |
| User choices in this task | Raw create becomes World-internal and meta/steward acceptance is the selected next edge | Exact-Place implementation waits behind this accepted writer flow |

## Alignment

### Strategic

Aicadia needs authored shared state without turning every player Agent into a generic
World editor or making the server intelligent. This slice establishes the first
honest World-authoring gate: intelligence remains in a consciously operated Agent,
irreversible public creation requires human confirmation, and World owns deterministic
acceptance. It produces legitimate placed content that the following Exact-Place edge
can expose to Characters. The next concrete risk is whether the same World creation
behavior can later be reached through compelling player discovery without inheriting
meta-admin authority.

### Tactical

The smallest complete slice has one private flow:

1. an administrator explicitly invokes the repository-local meta/steward workflow;
2. the Agent reads only the bounded World state available to that workflow and drafts
   transient Entity candidate content;
3. the administrator selects or rejects the candidate direction and explicitly
   confirms the final English `{ name, description, place }` package;
4. only after confirmation, a private operator adapter submits the package plus
   administrator provenance to `World`;
5. World validates User provenance, Entity text and nullable Place, then atomically
   stores Entity, optional `entity_location`, Activity and private meta provenance;
6. success returns the complete Entity; rejection or no confirmation stores nothing.

The raw write has no player HTTP endpoint or MCP tool. A supplied Place may name any
existing Place and need not equal a Character's Place. The meta Agent has no Character
precondition, durable identity or direct database access. Every accepted command is
non-idempotent unless the grill deliberately adds a delivery key.

### Technical

`World` remains the deep module. The provisional accepted command is
`CreateEntity { name, description, place: Option<EntityId> } -> Entity`; the private
adapter supplies responsible administrator provenance separately. `null` stores no
location. A non-null id must resolve to a Place, then Entity, location, Activity and
meta provenance commit or roll back together.

The provisional storage design adds `entity_location(entity_id PK/FK entity,
place_entity_id FK place)` and the earned lookup index on `place_entity_id`. Activity
keeps operation `create_entity`, links the new Entity as `subject`, and links a target
Place as `location`; `actor_character` and `context_place` remain absent for a pure
meta action. Private meta provenance must identify the responsible administrator
without presenting the meta Agent as a World actor. Its exact representation remains
open.

The recommended delivery seam is a repository-local steward skill plus a private
binary modelled after `aicadia-provision-user`. The binary reads a confirmed package
without putting content or credentials in command-line arguments, obtains database
configuration from the established environment and calls `World` directly. It is not
served over HTTP or MCP. The workflow—not a meaningless `--confirmed` flag—owns the
human confirmation pause. Candidate generation remains outside server tests; World,
CLI and workflow boundary behavior receive deterministic evidence.

In the same change, remove `POST /api/entity` and MCP `create_entity`, update the
catalog/OpenAPI/parity fixture and amend the always-on MVP surface. Global
`list_entity` and `get_entity` remain player reads. A live paid Agent demonstration is
not required for token-free correctness and needs separate explicit authorization if
later selected as acceptance evidence.

## Decisions, assumptions and open questions

### Confirmed decisions

- Agent authorship, administrator/User provenance and World acceptance are distinct;
  only World creates a durable Entity.
- Meta Agent invocation is conscious and external; server-side Agent calls, automatic
  token spend and a background steward are forbidden.
- Raw `create_entity` is removed from the generic player-facing HTTP/MCP catalog and
  remains World behavior behind a private accepted flow.
- A candidate is not an Entity until World accepts it.
- The create input uses required nullable `place: EntityId | null`; any existing Place
  may be targeted independently of Character placement.
- Accepted creation returns base `Entity`, and equal accepted commands create distinct
  Entities unless retry semantics are deliberately changed.
- Exact-Place reading is the next dependent edge, not part of this slice.

### Reversible assumptions

- Start from the existing local-binary operational seam rather than adding an admin
  network interface; revisit only if the confirmed operator workflow cannot invoke it
  safely.
- Reuse existing Entity normalization and complete Entity output; focused tests check
  that meta delivery does not fork those semantics.

### Open questions

1. Does one invocation offer exactly three candidate directions plus “do nothing”,
   followed by a separately confirmed final package? **Recommendation:** yes, carrying
   forward the prior steward interaction: alternatives improve administrator choice,
   do-nothing is explicit, unchosen candidates remain transient, and only the exact
   final package crosses the irreversible boundary.
2. What identifies and authorizes the administrator? **Recommendation:** for this
   local first slice, database/process access authorizes the private CLI while an
   existing durable User id supplies accountable provenance; record a private
   `meta_admin` origin without introducing accounts, OAuth or a second person model.
3. How is private meta provenance stored? **Recommendation:** add the narrowest
   normalized Activity-owned marker that distinguishes `meta_admin` origin and its
   responsible User without exposing it in player history or inventing a universal
   payload.
4. Is confirmed submission delivery-idempotent? **Recommendation:** yes for this
   operator flow, using one explicit confirmation/package id so an uncertain CLI retry
   cannot create duplicate irreversible World state; semantic re-approval creates a
   new id and may create another Entity.
5. Is the private delivery seam a repository skill plus local CLI? **Recommendation:**
   yes. It provides the human pause and Agent reasoning without adding an admin MCP
   tool, server Agent or web-auth system.

Keep this plan `draft` until the grill resolves these questions one at a time and the
complete plan is explicitly accepted.

## Implementation map

| Surface | Current state | Intended change | Invariants |
| --- | --- | --- | --- |
| `AGENTS.md`, `CONTEXT.md` | Generic player create is current; meta Agent only partially defined | Install accepted player/meta/World ownership rules and vocabulary | Compact always-on rules; no stale current surface |
| `docs/game/*` | Ten player capabilities; no meta flow or Entity placement | Define private acceptance, remove player raw create and document exact history/errors | Current executable truth only; concept history cannot govern |
| `migration/0005_*.sql` | No ordinary Entity location or meta provenance | Add location, Activity location role, accepted meta provenance and retry state only if confirmed | One Entity identity; normalized, queryable, atomic history |
| `src/world.rs`, `src/lib.rs` | User-facing create writes Entity plus Activity | Deepen World creation with placement, meta provenance and accepted retry semantics | Dumb strict World; no Agent calls; transaction owns all state |
| private operator binary | Provisioning binary is the existing operational pattern | Add one stdin/package-based meta acceptance adapter | No HTTP/MCP exposure; no credentials/content in argv |
| `.agents/skills/world-steward/` | No executable current steward workflow | Add consciously invoked proposal/selection/confirmation workflow | No write before exact human confirmation; unchosen state remains private |
| `src/server.rs`, `src/wire.rs` | Player POST/MCP raw create exists | Remove raw player adapters while preserving shared Entity reads | Catalog/OpenAPI/HTTP/MCP parity changes together |
| `tests/world.rs`, binary tests | Global create and Activity behavior tested | Prove atomic placement, provenance, retry and rejection boundaries | Evidence covers exactly accepted behavior |
| `tests/server.rs`, catalog fixture | Ten-capability create parity pinned | Prove raw write absent and remaining catalog complete | No hidden player mutation route |
| backlog, plan, concept log | Exact-Place was selected before writer authority crystallized | Make meta acceptance Now and Exact-Place its dependent Next | At most one Now item; plans reflect live dependencies |

## Execution contract

Root owns outcome, scope, plan state, integration and the final evidence claim. A
delegated Agent receives this plan path and one dependency-ready task id, re-reads the
live repository, changes only its owned surfaces, runs focused evidence and returns
raw results. Delegation is optional. Tasks remain sequential because contract,
storage, operator workflow and adapter removal share public meanings.

## Task graph

| ID | State | Depends | Parallel-safe | Objective | Owned surfaces | Evidence |
| --- | --- | --- | --- | --- | --- | --- |
| T1 | pending | — | no | Install the accepted meta acceptance contract | `AGENTS.md`, `CONTEXT.md`, `docs/game/*`, backlog, concept log | authorities name exact actor, confirmation, provenance, World state and exclusions |
| T2 | pending | T1 | no | Implement atomic World Entity acceptance | migration, `src/world.rs`, `src/lib.rs`, `tests/world.rs` | World tests prove placement, provenance, rejection, concurrency and retry contract |
| T3 | pending | T2 | no | Deliver private steward operation and remove player raw create | operator binary, steward skill, `src/server.rs`, `src/wire.rs`, server/catalog tests | confirmed private flow succeeds; HTTP/MCP raw write is absent |
| T4 | pending | T3 | no | Prove outcome and close forward state | all changed surfaces, plan, backlog, concept log | validation ladder and focused operator scenario pass |

## Task details

### T1 — Accepted contract

**Objective:** Make the accepted meta-steward flow the sole current authority for raw
Entity creation delivery.

**Actions:**

1. Incorporate every grill answer into `docs/game/`, player catalog, operations,
   Activity semantics, `AGENTS.md` and canonical vocabulary.
2. Mark the Exact-Place plan/item dependent and remove every stale statement that raw
   create is a generic player capability.

**Invariants:**

- No product code changes while a material flow question remains open.
- Old scene/claim steward exploration is not imported into the current Entity model.

**Evidence:**

- Targeted text review proves actor, candidate, confirmation, accepted state,
  provenance, errors and exclusions agree across current authorities.

**Stop conditions:**

- Stop if the contract requires general authentication, a web admin app, server-side
  inference or a durable generic proposal model.

### T2 — World acceptance

**Objective:** World atomically accepts one confirmed meta Entity command.

**Actions:**

1. Add only accepted location, Activity-role, private provenance and retry storage.
2. Implement strict Entity/Place/provenance validation and one transaction for all
   accepted state.
3. Prove malformed, missing Place, duplicate delivery, concurrent and rollback paths.

**Invariants:**

- Agent and administrator never become Entity or Activity actor Character by
  implication.
- Current state is not reconstructed from Activity and no event-sourcing payload is
  introduced.

**Evidence:**

- Focused World tests prove exactly one accepted state/history footprint and zero
  state for every rejection.

**Stop conditions:**

- Stop if implementation needs Entity update/delete, Place creation, semantic LLM
  validation or a generalized origin/authorization framework.

### T3 — Private operator delivery

**Objective:** A consciously confirmed meta package can reach World privately while
player Agents lose the raw mutation.

**Actions:**

1. Create the private stdin/package-based operator binary and deterministic tests.
2. Use the skill-creation workflow to add the minimal steward skill with proposal,
   choice, final-package review and hard confirmation pause.
3. Remove POST/MCP create adapters and align OpenAPI, tool catalog and parity tests.

**Invariants:**

- The skill never calls the write path before explicit confirmation.
- No player-visible interface reveals private meta provenance or admin controls.
- No paid Agent run occurs without separate authorization.

**Evidence:**

- Binary tests and a token-free scripted scenario prove accepted/no-op/rejected
  boundaries; adapter tests prove the raw mutation is absent everywhere public.

**Stop conditions:**

- Stop if human confirmation cannot be represented honestly, operator credentials
  would leak, or the workflow requires a public admin endpoint.

### T4 — Outcome evidence

**Objective:** Demonstrate the accepted World-authoring outcome and align all forward
state.

**Actions:**

1. Run focused and full Rust evidence, formatting, strict lint and diff review.
2. Demonstrate exactly one confirmed Entity and Activity/provenance footprint, plus
   zero state for do-nothing/unconfirmed/rejected paths.
3. Close this plan and make Exact-Place reading the next current edge.

**Invariants:**

- Existing Character entry, Activity reads and shared Entity reads remain intact.
- Unrelated user changes remain untouched.

**Evidence:**

- Full validation ladder passes and current authorities match the demonstrated
  behavior exactly.

**Stop conditions:**

- Stop if the exact outcome requires a paid live Agent run not already authorized.

## Validation ladder

1. **Focused:** World and operator tests prove accepted, no-op, rejected, retry,
   concurrency and rollback semantics.
2. **Contract:** Player HTTP/MCP/OpenAPI/catalog parity proves raw create absent;
   storage and docs prove private meta provenance and Activity exactness.
3. **Outcome:** One explicitly confirmed package becomes exactly one Entity with
   optional Place and history; all non-confirmed paths create nothing.
4. **Integrity:** `cargo fmt --check`, `cargo clippy --all-targets --all-features --
   -D warnings`, `cargo test`, `git diff --check`, focused diff review and confirmation
   that unrelated user changes and all governing authorities remain intact.

## Change control

Refine paths, task order and stronger evidence in place while the accepted actor,
confirmation, private delivery, World authority and evidence claim remain unchanged.
Stop implementation, return to `draft`, revise and request explicit re-acceptance if
caller authority, proposal lifetime, provenance, idempotency, public exposure,
external side effects, material cost or the evidence claim changes.

## Completion conditions

- every required task is `completed` and the validation ladder passes;
- one conscious meta-steward acceptance is demonstrated without a player raw write;
- current behavior, concept choices, vocabulary and backlog are aligned;
- Exact-Place reading is queued next with this writer as its explicit dependency;
- no known-stale authority, material open question or accidental unrelated change
  remains;
- `status: complete` and `completed_at` are recorded only after these conditions.
