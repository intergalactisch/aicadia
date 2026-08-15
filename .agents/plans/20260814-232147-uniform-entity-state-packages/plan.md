---
status: complete
created_at: "2026-08-14T23:21:47+02:00"
updated_at: "2026-08-15T09:02:00+02:00"
accepted_at: "2026-08-15T08:20:42+02:00"
completed_at: "2026-08-15T09:02:00+02:00"
---

# Uniform Entity creation and state-change packages

> **Role / side:** proportional build execution plan / development side.
> **Authority:** owns the accepted uniform Property/Trait creation and Action-change build boundary, completed task order and exact evidence claim.
> **Excludes:** current executable behavior, product rationale and delivery status; see `docs/game/`, `docs/concept/entity-state.md` and `docs/evidence/`.

## Outcome

Let an Agent create any Entity role with zero through 100 initial Properties and
zero through 100 initial Traits in one confirmed atomic call, and let one confirmed
Action change zero through 100 Properties and zero through 100 Traits together in
one atomic state package, with at least one actual change. World assigns every
stable Trait id and stores the creation or Action Activity as its root provenance;
one Activity truthfully owns the complete accepted state change.

The concrete proof is one ordinary Entity—such as a heat-scorched three-legged frog
that has learned to jump unusually high—created with two Properties and one Trait in
one call, then changed through one Action that carries both Property and Trait
changes. The same semantics must hold for Character, entry Place, `create_entity`
and Action introduction, and through World, HTTP and MCP with retry, rollback and
history parity. This closed the selected edge because the former homogeneous Action
and Trait-free creation boundary leaked implementation shape into ordinary play and
would force one fictional event into false sequential Activities.

## Non-goals

- No discovery roll, investigation operation, chance table, movement or new Place.
- No new player capability or generic patch/storage-edit tool; the catalog remains
  thirteen tools and all writes stay inside existing creation, Action and Interaction
  calls.
- No executable Trait mechanics, scores, modifiers, semantic inference or server
  narration; a Trait statement still grants no action or roll behavior.
- No Trait development during creation: initial `trait` items only establish new
  stable lineages; development still names an existing `trait_id` during Action or
  Interaction.
- No target response, consent, thought, relationship, placement or ownership change.
- No paid model playtest, fresh candidate, retry authorization or edit to retained
  paid evidence.
- No compatibility acceptance of the superseded public
  `change_entity_property`/`change_entity_trait` input variants.

## Evidence and authorities

| Evidence or authority | Baseline at acceptance | Consequence for this build |
| --- | --- | --- |
| `AGENTS.md` | One Entity identity owns role-neutral state; every mutation needs Activity history; current-only public contracts replace superseded modes | Apply creation uniformly, retain exact history and remove old public Action variants |
| [Entity-state rationale](../../../docs/concept/entity-state.md) | Property is typed fact; Trait is stable non-executable characterization; User confirmed both belong in general creation and modification | Keep meanings separate while allowing one atomic package |
| [Game domain](../../../docs/game/domain.md) | All four creation routes accept initial Properties but reject Traits; Action is homogeneous; Interaction already combines both | Reuse Interaction's proven combined bounds and writer order rather than inventing another abstraction |
| `src/world/model.rs`, `src/wire/input.rs`, `src/wire/output.rs` | Creation carries only `property`; Action has three tagged variants and separate accepted variants | Add initial `trait`; retain introduction and replace the two change variants with one `change_entity_state` shape |
| `src/world/mutation.rs`, `src/world/activity.rs` | Existing creation transactions already append Activity before Property state; Interaction already writes Property and Trait together; retries use stored fingerprints and discriminator reconstruction | Deepen the existing transaction and preserve historical retry identity without a legacy input path |
| `migration/0008_entity_trait.sql` | Trait roots accept only Trait Actions and Interactions; all Trait identity/version/current tables and indexes already exist | Add one migration that evolves constraints/functions only; no new table or index |
| [Agent contract](../../../docs/game/agent.md) and `src/agent_contract/` | Workshops preview all accepted meaning but teach Trait-free creation and homogeneous Action | Preview initial and combined state completely before one confirmation; keep ids private |
| Current deterministic tests and runners | Creation Property, Trait Action, combined Interaction, parity and both token-free runner suites are delivered | Replace obsolete assertions and prove the new cross-product; no paid run |
| [Discovery draft](../20260814-204007-first-investigation-discovery-loop/plan.md) | Discovery remains unresolved and is distinct from ordinary Entity establishment | Keep it unaccepted and restore it to `Now / Proposed` after this bounded build |

## Alignment

### Strategic

A shared World is more believable when one event is stored as one event. Creating a
three-legged frog and only afterward inventing a second Action to establish what was
already true creates false history; changing a fact and characterization through two
Actions does the same. Uniform atomic state packages make authored World subjects
richer without adding a generic mechanics engine. Discovery remains the next
gameplay-design edge once this existing capability seam is coherent.

### Tactical

All four creation routes accept `property[0..100]` plus
`trait[0..100]`, where every Trait item contains only one statement. Creation assigns
the new Entity and Trait ids inside one transaction, and the route's existing
Activity becomes each initial Trait's root provenance. “Established at creation”
means first accepted shared-World recording, not that the fictional subject learned
or acquired the characterization at that instant.

`submit_action.introduce_entity` gains the same two initial lists. The two existing
change alternatives are replaced publicly by
`change_entity_state { property_change[0..100], trait_change[0..100] }`; at least one
list must be non-empty. Trait items may mix establish and develop exactly as now.
Actor, Place, co-present Character and ordinary exact-local Entity eligibility stays
uniform. `submit_interaction` keeps its current external shape and semantics.

The whole package is previewed and confirmed once, then either Activity, roles,
Property history/current pointers and Trait roots/versions/current pointers all
commit, or none do. Property validation precedes Trait validation just as in the
current combined Interaction path; neutral eligibility and canonical errors remain
meaning-specific.

### Technical

Add one `TraitInput { statement }` creation shape and one internal normalized Entity
state package. Creation supplies the newly assigned Entity id to initial Trait roots.
Action uses the existing combined Property/Trait normalization, stable locks and
writers already exercised by Interaction, unions every affected Entity into one
`subject` role set and retains one `location` role.

Migration `0009` extends the closed Activity discriminator with
`change_entity_state` while retaining old stored discriminator values as immutable
history. It replaces the Trait-root Activity trigger so new roots may cite
`create_character`, `create_entry_place`, `create_entity`,
`submit_action.introduce_entity`, `submit_action.change_entity_state` or
`submit_interaction`. New code never writes the old change discriminators.

The public old change variants disappear. Current-format Property-only and
Trait-only Action fingerprints deliberately reuse their historical semantic hash
forms; an introduction with empty `trait` retains the historical introduction hash.
Only genuinely combined state or non-empty initial Trait input adds a new
length-prefixed fingerprint component. Stored old change tags decode into the new
current `change_entity_state` result. This is a bounded current response-loss retry
consumer, not an accepted legacy input mode, and no Activity row is rewritten.

HTTP remains thin over World; MCP schemas and descriptions are regenerated from the
same wire types. Existing ignored paid evidence stays immutable. The Trait runner's
current source/catalog digest may change token-free, but its consumed sentinel grants
no candidate or preflight authorization.

## Decisions and verified assumptions

### Confirmed decisions

- General creation and modification, not discovery alone, must support multiple
  Properties and Traits together — User correction and confirmation on 2026-08-14.
- Every Entity role uses the same creation capability: Character, Place and ordinary
  Entity are not special state models — User accepted the uniform four-route
  recommendation.
- One Action state package may combine Property and Trait change, while Interaction
  retains the combination it already supports — User accepted the proposed direction.
- Initial Trait history names the creation Activity as first shared establishment;
  it does not fabricate a separate causal Action — concrete frog counterexample and
  root Activity-history rule.
- Property and Trait remain different domain meanings and persistence models; the
  combined package is transaction composition, not a universal state table.

### Verified assumptions

- The existing `entity_trait*` tables and indexes are sufficient; focused 100+100
  creation/Action tests required no storage relation or index.
- Interaction's Property-first normalization/error order remains the consistent
  Action order and is pinned in deterministic invalid-combination tests.
- The generated tool count remains thirteen; runtime and checked-in catalogs are
  exactly equal across HTTP and MCP.

### Accepted details

- **Acceptance detail A:** creation and Action reuse Interaction's independent
  bounds: each Property and Trait list is 0–100, with at least one non-empty list for
  `change_entity_state`. This permits at most 200 typed state changes in one call and
  avoids a second shared-total rule. Plan acceptance accepts this recommendation.
- **Acceptance detail B:** preserve immutable historical discriminator rows and
  cross-upgrade response-loss retries through semantic fingerprint compatibility and
  old-row decoding, while rejecting old public input variants. Plan acceptance
  explicitly accepts this narrow current consumer under `Current Means Current`.

Both details were accepted with this plan before implementation began.

## Implementation map

| Surface | Baseline at acceptance | Delivered change | Invariants |
| --- | --- | --- | --- |
| `docs/game/`, `CONTEXT.md` | Trait-free creation; homogeneous Action | Publish initial Trait and combined Action contract before implementation | Trait stays non-executable; Interaction meaning unchanged |
| `src/world/model.rs`, `src/wire/input.rs`, `src/wire/output.rs`, exports | Separate creation/Action shapes | Add `trait`, `TraitInput` and one state-change variant/result | Strict schemas; singular names; old public variants absent |
| `src/world/mutation.rs`, `property.rs`, `entity_trait.rs`, `activity.rs` | Creation writes only Properties; Interaction alone composes both | Reuse normalized/writer seams for creation and Action; union roles; current retry reconstruction | One transaction; stable locks; no direct adapter logic |
| `migration/0009_*` | Trait root trigger admits only Trait Action/Interaction; discriminator lacks combined tag | Extend constraints/functions without data rewrite or new relation/index | Historical rows immutable; new roots cite exact accepted Activity |
| HTTP/MCP and generated catalog | Thirteen tools with current closed schemas | Regenerate equal schemas/descriptions for existing calls | Capability count and error semantics unchanged |
| `src/agent_contract/`, `docs/game/agent.md` | Creation and Action guidance is split by state kind | Preview complete initial/combined state and confirm once | No ids/fields in player language; no direct editor |
| World/server/contract tests | Separate Property/Trait creation/Action assertions; combined Interaction proof | Replace obsolete boundary tests and add creation/Action cross-product, migration, retry, rollback, concurrency and parity | Existing Interaction, reads and Activity authorization remain green |
| `tools/*-playtest`, `tests/*-playtest.sh`, schema fixtures/digest | Current catalog/action shapes are bound into token-free evidence machinery | Align current schemas/fakes/digest and run token-free only | Retained manifests/sentinels immutable; no paid candidate |
| Concept, evidence, backlog and this plan | Uniform direction accepted; discovery temporarily next | Record delivery and restore discovery to `Now / Proposed` | One current planning edge; no duplicated delivery truth |

## Execution contract

Root owned outcome, scope, plan state, integration and the final evidence claim. At
the User's later request, one Sol-high Agent received this plan and the bounded T3
state/Action surface, re-read live files, changed only its assigned surfaces, ran
focused evidence and returned raw results. Root retained integration and the other
dependency-ordered tasks.

## Task graph

Allowed states are `pending`, `in_progress`, `completed` and `blocked`.

| ID | State | Depends | Parallel-safe | Objective | Owned surfaces | Evidence |
| --- | --- | --- | --- | --- | --- | --- |
| T1 | completed | — | no | Accept and publish the exact uniform state contract | plan, concept/log, backlog, `CONTEXT.md`, `docs/game/` | One current contract, no stale Trait-free/homogeneous rule |
| T2 | completed | T1 | no | Make every creation route atomically establish initial Traits | migration, World model/writers, wire creation types, creation tests | 0/1/100+100, rollback, root provenance and route parity |
| T3 | completed | T2 | no | Replace homogeneous Action changes with one combined state package | Action model/wire/mutation/activity/fingerprint, tests | property-only, trait-only and combined accept; empty/invalid/retry/concurrency fail exactly |
| T4 | completed | T3 | no | Align HTTP/MCP, Agent guidance and complete deterministic parity | server, generated contract, Agent text, contract/server tests | exact thirteen-tool equality and natural one-confirmation flow |
| T5 | completed | T4 | no | Align token-free evidence machinery, authorities and close integration | runners/fakes/digest, evidence, concept/log, backlog, plan | full ladder green, no model spend, discovery restored |

## Task details

### T1 — Contract and current truth

**Objective:** Make the accepted actor/action/state/history contract unambiguous
before code changes.

**Actions:**

1. Record plan acceptance and both acceptance details.
2. Update canonical vocabulary and the exact creation, Action, Interaction, Agent,
   domain, protocol and storage contracts; remove superseded current rules together.
3. Activate the backlog item while discovery stays unchanged at `Next / Proposed`.

**Invariants:**

- No runtime/schema edit until the complete contract agrees.
- No discovery, generic mutation operation, score or Trait mechanics enter scope.

**Evidence:**

- Targeted `rg` and link checks — all current Trait-free creation and homogeneous
  Action statements are removed or deliberately historical, with one owning truth.

**Stop conditions:**

- Return the plan to draft if bounds, fingerprint compatibility, initial Trait
  meaning or affected creation routes change.

### T2 — Initial Traits on every creation route

**Objective:** One creation Activity atomically owns the new Entity, its initial
Properties and every initial Trait root.

**Actions:**

1. Add the strict initial Trait input and shared normalization.
2. Add migration constraints that admit exact creation-root Activities without
   rewriting existing history.
3. Extend Character, entry Place, Entity and Action-introduction transactions using
   the existing Trait writer and exact subject roles.
4. Add schema/World/storage tests for empty, one, 100+100, duplicate, malformed,
   rollback, concurrency and all four Activity operations.

**Invariants:**

- Initial Traits establish only; stable ids remain World-assigned.
- A failed Property or Trait rejects Entity, role, Activity and all state.
- No new table, index or role-specific state model.

**Evidence:**

- Focused creation World/schema suites plus migration replay — four routes share one
  atomic contract and every root cites the correct creation Activity.

**Stop conditions:**

- Stop on required data rewrite, new persistence relation, ambiguous root provenance
  or a route that cannot preserve atomic rollback.

### T3 — Combined Action state package

**Objective:** One confirmed Action can change Properties and Traits together with
one retry identity and Activity.

**Actions:**

1. Replace both public change variants with `change_entity_state` and one accepted
   result carrying both sorted lists.
2. Reuse Property-first normalization and combined writers; validate exact-local
   eligibility, stable locks and unioned subject roles before writes.
3. Preserve historical semantic fingerprints for single-kind and empty-Trait
   introduction retries; add the new length-prefixed combined forms and old-row
   decoder.
4. Add focused behavior/storage tests for 0/1/100 lists, 100+100 combined, mixed
   establish/develop, invalid/no-op/duplicate/unavailable/stale, rollback,
   idempotency, input reordering, response-loss upgrade reconstruction and
   concurrent reverse-order packages.

**Invariants:**

- Both empty is `invalid_action`; one non-empty list is sufficient.
- Old public variants are absent; old stored rows remain immutable and queryable.
- Property-only and Trait-only current calls stay first-class, not compatibility
  modes, through the one new package.

**Evidence:**

- Focused Action/Activity/storage tests — exact accepted result and history contain
  both lists; retries reconstruct; every failure writes nothing.

**Stop conditions:**

- Stop on history rewrite, lost idempotency, nondeterministic error order or branch/
  deadlock risk not covered by the existing set-based writers.

### T4 — Adapter and Agent parity

**Objective:** HTTP, MCP and the Agent expose exactly the same complete state package
and one-confirmation experience.

**Actions:**

1. Align wire outputs, OpenAPI/MCP schemas and generated thirteen-tool catalog.
2. Teach creation workshops and Action guidance to preview every Property/Trait
   meaning naturally, including lifecycle/current/new Trait characterization.
3. Replace obsolete Agent-contract and server fixtures; prove creation and combined
   Action parity, strict unknown old variants and canonical errors.

**Invariants:**

- Thin adapters; World alone validates and writes.
- Player conversation exposes no fields, ids, protocol or control provenance.
- `submit_interaction` external behavior remains byte-meaning-equivalent.

**Evidence:**

- Agent-contract tests plus focused HTTP/MCP parity and schema-invalid tests — exact
  catalog count/equality and one confirmed complete package.

**Stop conditions:**

- Stop if adapter-specific behavior, a fourteenth tool, legacy public input or an
  Agent-visible storage concept is required.

### T5 — Evidence machinery and integration

**Objective:** Leave current deterministic proof and planning truth aligned without
claiming new paid evidence.

**Actions:**

1. Align both runner catalogs/schemas/fakes and recompute only current token-free
   digest material; never alter retained candidates or consumed sentinels.
2. Run focused, full, formatting/lint, runner and documentation integrity ladders.
3. Update evidence status, concept history, completed backlog/plan state and restore
   discovery to `Now / Proposed` only after all checks pass.

**Invariants:**

- `codex exec`, paid candidates and public Trait preflight remain absent.
- Existing user discovery/local-play work is preserved except exact planning order.

**Evidence:**

- Both playtest fake suites with `model_calls:0`; calculated/file Trait digest
  equality; all Rust tests; Clippy; formatting; link and `git diff --check`; exactly
  zero active plan/item and one restored `Now / Proposed` edge.

**Stop conditions:**

- Keep the plan active on schema/catalog drift, failed migration replay, stale docs,
  ambiguous evidence or any need for paid validation.

## Validation ladder

1. **Focused:** model normalization, creation/Action writer, fingerprint, migration
   and strict-schema tests for positive and fail-closed cases.
2. **Contract:** all four creation routes, combined Action, unchanged Interaction,
   Activity hydration and exact HTTP/MCP/catalog parity.
3. **Outcome:** one frog-style Entity is created with two Properties plus one Trait,
   then one Action changes a Property and develops/establishes Traits together; a
   second Character reads the same current state and one Activity per event.
4. **Integrity:** full PostgreSQL-backed Rust suite, both token-free runner suites,
   Clippy, formatting, link checks, `git diff --check`, focused diff review and
   confirmation that unrelated user changes and historical paid evidence remain
   intact.

## Change control

Refine paths, task order, test names and stronger token-free proof in place while all
four creation routes, the one combined Action package, independent 0–100 bounds,
historical fingerprint compatibility and no-paid boundary remain unchanged. Stop,
return to `draft`, revise and regain acceptance for a different public shape, shared
total bound, discovery behavior, history rewrite, legacy input mode, new relation,
new tool, Trait mechanic, paid run or weaker atomicity/evidence claim.

## Completion conditions

- T1–T5 are completed and the entire validation ladder passes;
- every creation route and combined Action prove exact current state plus one
  attributable Activity without partial writes;
- current-only schemas replace superseded public inputs while historical rows and
  cross-upgrade response-loss retry semantics remain intact;
- current game, concept, evidence, vocabulary, runner and backlog authorities agree;
- no known-stale authority, material open question or accidental unrelated change
  remains;
- discovery is restored unchanged to `Now / Proposed`; and
- `status: complete` and `completed_at` are recorded only after these conditions.
