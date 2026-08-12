---
status: complete
created_at: 2026-08-11T12:45:50+02:00
updated_at: 2026-08-11T20:44:27+02:00
accepted_at: 2026-08-11T13:50:15+02:00
reaccepted_at: 2026-08-11T20:05:29+02:00
completed_at: 2026-08-11T20:44:27+02:00
---

# First Agent-mediated World action

## Outcome

A User can complete the first meaningful post-entry game action through their Agent.
The Agent composes its working context through separate typed reads of the World,
current Character and relevant Place state, presents exactly three grounded private
proposals, incorporates the User's choice and optional steering, previews one final
public package, receives one explicit User confirmation and submits it once. `World`
derives the User's Character and current Place,
deterministically accepts or rejects the whole
package and, only on acceptance, atomically:

- stores the action's readable prose;
- creates one trail-marker Entity;
- establishes that Entity at the Character's exact current Place; and
- writes one immutable Activity footprint linking actor, Place and Entity.

The concrete evidence scenario begins with two Characters at the one entry Place.
The first User chooses a trail-marker direction and commits it. The second User can
then read both the placed Entity and the accepted prose from that same Place without
supplying User, Character or Place ids. Invalid submission changes nothing; delivery
retry returns the first accepted result without duplicating Entity, placement, prose
or Activity.

This is the highest-value current edge because World entry is already proven but has
no actual turn of shared gameplay after arrival. The slice makes one User's chosen
action durable and locally meaningful to another User while keeping intelligence in
the Agent and all state authority in `World`.

The product, persistence and adapter slice is implemented and deterministically
proven. Separately authorized `run-gE8iED5m` also passed the interaction,
authoritative HTTP-state, independent MCP-observation and ownership-verified cleanup
gates in one continuous run, and independent T4R4 review verified its raw evidence.
Two earlier candidates remain truthful rejected/partial evidence and are never
relabelled as successful. No product behavior was reopened to complete the proof.

## Non-goals

- a universal action engine, arbitrary CRUD batch, SQL/JSON patch or generic event
  payload;
- more than one concrete consequence type in the first accepted contract;
- changing or deleting existing Entities, moving a Character, creating additional
  Places, containment, adjacency, routes, literal metric distance, geometry or
  visibility;
- discovery rolls, claims, world events, event sourcing or replay projections;
- storing the three proposals, rejected directions, User steering, conversation or
  private Agent reasoning;
- server-side LLM use, server-triggered Agents or background World simulation;
- an MCP App, required rich UI, preferences or server-authored proposal choices;
- authentication, OAuth, quotas, scores, ranks, currency or administration;
- a global World revision, read-issued request nonce, pending action, preparation
  session or database transaction held open while the Agent or User reasons;
- proving semantic prose quality with deterministic server validation;
- retrofitting prose or placement onto historic Activities or Entities.
- changing World, PostgreSQL, HTTP or MCP behavior merely to satisfy the evidence
  harness;
- adding `get_entity`, Entity description or any fourth read to the observer when
  exact local Entity identity, name, Place and prose already prove its distinct role;
- rewriting either failed live manifest, combining separate candidates into one pass
  or automatically spending tokens on another attempt.

## Evidence and authorities

| Evidence or authority | Current fact | Consequence for this build |
| --- | --- | --- |
| `docs/game/README.md` | `create_entity` creates an unplaced Entity; Activity has actor/context/involved ids but no prose | Deliberately evolve the contract with one compound action while preserving granular Entity introduction |
| `docs/game/README.md` Character and entry | World already derives one Character and nullable exact current Place from User context | Action and local reads accept no User, Character or Place selectors and reject an unplaced Character |
| `docs/game/agent-interface.md` | Ten HTTP/MCP capabilities share one World contract; MCP initialization carries global Agent instructions | Ship action and local reads through World, HTTP and MCP and publish the private-workshop flow in Agent instructions |
| `src/world.rs` | Every mutation locks User context, writes state and Activity in one Postgres transaction | Deepen this existing World seam; Agent and adapters never receive storage authority |
| `migration/0004_world_entry_activity.sql` | Activity and its Entity roles are immutable; current Character placement is explicit state | Extend current state and history without inferring either from prose |
| `docs/research/player-agent-interaction.md` | Research recommends private proposal/steering, exact package preview and one public atomic commit | Use ordinary conversation first; keep pre-commit work outside World state |
| `docs/research/idempotent-action-delivery-and-place-freshness.md` | Client intent identity, payload equality and observed-state freshness are separate; strong revisions must be scoped to one representation | Use an Agent UUID for retry identity and an opaque exact-Place revision for grounded-action freshness; validate both inside World |
| `docs/game/agent-playtest.md`, `tools/agent-playtest` | Existing disposable two-Agent harness proves clean-room MCP understanding, isolates credentials and gates paid execution behind token-free preflight | Extend this earned harness to prove the resumed proposal/selection/preview/confirmation/commit loop and independent observation |
| `.aicadia-playtest/run-G8k1sTRm/manifest.json`, `docs/game/agent-playtest.md#rejected-live-candidate` | The first authorized candidate was rejected before model or tool execution because `uniqueItems` was unsupported; it wrote no World outcome and cleanup dropped its owned database | Keep it as schema/preflight failure evidence; it cannot support the gameplay outcome or be retroactively passed |
| `.aicadia-playtest/run-nvULnvxQ/manifest.json`, retained JSONL and `docs/game/agent-playtest.md#rejected-live-candidate` | Proposals, preview, exactly one commit and three observer MCP reads succeeded; the observer found the canonical Entity/name/Place/prose, but a demanded unobservable description failed final validation before HTTP validation ran | Treat this as strong partial live evidence; remove only the unearned observer field and require one new continuous passing candidate |
| `docs/concept/10-discovery-and-world-context.md` | Agent owns reasoning and candidate authorship; World owns validation and atomic commit | Reuse the authority split without importing the deferred discovery-roll system |
| `.agents/backlog/items/agent-mediated-world-action.md` | This is the proposed current edge | The plan must prove the first concrete action, not speculative future consequence types |
| User choices in this grill | Exactly three Agent proposals; User selects and may steer; final action is usually one bundle; Agent context is composed from typed MCP reads including a semantic Place neighborhood; World alone writes | Separate interaction, transport and transaction while retaining one deterministic authority |

## Alignment

### Strategic

The action is Aicadia's first complete multiplayer turn: orientation leads to a
human-steered Character action, that action changes the shared World, and another
Character at the same Place can experience its result and history. It advances
settlement with a modest persistent marker rather than adding chance, travel or a
generic fiction engine. The next concrete risk after this slice is whether the same
package seam can safely admit a second consequence type such as modifying established
state without becoming an unrestricted patch language.

Closing its evidence is still the highest-value edge because moving on would leave
the first actual shared-world turn in an ambiguous state: mechanically complete but
not proven end to end under a real Agent. The recovery does not enlarge the game.
It makes the smallest honest claim that matters: one User-mediated action is accepted
once by World, independently visible through both transport surfaces and safely
isolated operationally.

### Tactical

The intended interaction is:

1. The User consciously asks for the next action.
2. The Agent calls separate published World, Character and Place reads and may drill
   into typed references. Spatial surroundings are a bounded semantic Place
   neighborhood over explicit relationships, never a monolithic context payload or
   metric radius. This first slice reads Entity and Activity/prose state at the exact
   entry Place only; later typed reads may add containment and adjacency. Exact-Place
   Entity and Activity/prose responses carry the opaque `place_revision` they read.
   The Agent only grounds one proposal package in Place reads whose revisions agree;
   the reads themselves change no state.
3. The Agent presents exactly three grounded directions. These are Agent output,
   not server-authored options or durable candidates.
4. The User chooses one and may combine it with free steering. The workshop remains
   private.
5. The Agent writes the exact English prose and structured trail-marker consequence,
   shows the complete final package and waits for one explicit User confirmation.
   It creates one UUID for that intended action and, only after confirmation, makes
   one `submit_action` request with that `request_id` and the observed
   `expected_place_revision`.
6. World derives the responsible User, Character and current Place. It validates
   retry identity, locks that Place and rejects the complete package if its revision
   changed before validating the remaining placement and input rules.
7. One transaction inserts the Entity, its exact Place relation, the Activity and
   prose. World returns the canonical accepted result.
8. A Character at that Place can independently list the placed Entity and Place-local
   Activity/prose. A Character without a current Place receives an explicit error.

The first package has one closed structured consequence, working name
`introduce_entity`, containing only semantic `name` and `description`. It accepts no
Entity id, Character id or Place id. World supplies all identities and always places
this first action's new Entity at the derived current Place. This is intentionally
not the earlier arbitrary `create_entity({place})` design.

`create_entity` remains a granular standalone World command and HTTP/MCP capability
for introducing a stable shared referent without action prose or placement. It never
writes directly from the Agent. The new bundle does not invoke that public capability;
both commands reuse private validation and insertion behavior inside World.

One accepted package produces one Activity, even though several database rows change.
Reads, rejected requests, retries, proposals and private reasoning produce none.
The action is all-or-nothing: World never silently drops one part or partially accepts
the package.

### Technical

Implemented public World interface:

```rust
submit_action(user_id, SubmitAction) -> Result<AcceptedAction, WorldError>
list_entity_at_current_place(user_id, ListEntityAtCurrentPlace)
    -> Result<CurrentPlaceEntityPage, WorldError>
list_activity_at_current_place(user_id, ListActivityAtCurrentPlace)
    -> Result<CurrentPlaceActivityPage, WorldError>
```

Implemented first-slice input and result:

```rust
struct SubmitAction {
    request_id: Uuid,
    expected_place_revision: PlaceRevision,
    prose: String,
    consequence: IntroduceEntity,
}

struct IntroduceEntity {
    name: String,
    description: String,
}

struct AcceptedAction {
    activity: Activity,
    entity: Entity,
    place: Place,
}

struct PlaceRevision {
    place_entity_id: EntityId,
    occurred_at: DateTime<Utc>,
    activity_id: ActivityId,
}
```

The singular `consequence` is deliberate. A tagged list, local references and
cross-consequence ordering add no behavior to this slice and are introduced only
when a selected second consequence type needs them. `SubmitAction` is not a durable
proposal or session; only its accepted Activity and concrete results persist.

`CurrentPlaceEntityPage` and `CurrentPlaceActivityPage` both return the exact derived
Place and one opaque `place_revision`. The token is a strong validator for that exact
Place representation: it identifies the Place and the Activity referenced by the
Place's authoritative `latest_activity_id` pointer, carrying the target Activity's
`(occurred_at, activity_id)` as identity. Those values never infer acceptance order.
Every current change exposed by either exact-Place read updates the pointer under the
same Place lock. The token is never an authorization token, global World version or
Agent-built hash. Pages used together must report the same revision.

Each exact-Place read uses one short read-only Repeatable Read transaction for
Character/Place derivation, revision, page rows and related Activity roles. This makes
the returned page and revision one consistent per-call snapshot without holding a
transaction across MCP calls. Wire code encodes `PlaceRevision` as an opaque,
versioned URL-safe base64 token using the existing cursor convention; malformed
tokens are invalid input, while a well-formed token for another or older Place
representation is a freshness conflict.

Migration `0005_agent_action.sql` adds the smallest current-state and history storage:

- `entity_location(entity_id PK/FK entity, place_entity_id FK place)` plus the exact
  `(place_entity_id, entity_id)` lookup needed by the local Entity read;
- nullable `activity.prose`, non-null only for accepted `submit_action` Activities;
- nullable `activity.request_id` with uniqueness per responsible User for accepted
  action delivery identity;
- nullable `activity.request_fingerprint`, containing a versioned World-derived
  32-byte SHA-256 fingerprint of normalized accepted action input for stable conflict
  detection;
- non-null `place.latest_activity_id`, backfilled to one deterministic historic
  Place-relevant Activity and thereafter set only by genesis or an accepted writer
  under the Place lock;
- Activity operation `submit_action`; and
- Activity Entity role `location` for the Place at which the new Entity was
  established, while `context_place` independently records where the actor acted.

Historic Activities keep null prose/request id/fingerprint. No historical Entity
receives an invented location. The existing immutability triggers protect newly
stored prose and request identity.
Current Entity location remains ordinary authoritative state, not a replayed
projection and not an Activity inference.

One accepted Activity owns one canonical prose value. World-, Character-, Place- and
Entity-oriented history reads always reference that same record and use one stable
chronological ordering; prose is never copied into mutable per-lens histories. World
acceptance time is the only current time axis. Agent input cannot backdate, reorder or
insert an action into earlier history; every lens orders the same canonical records
by `(occurred_at, activity_id)`.

World normalizes prose and Entity text and derives SHA-256 over a versioned,
length-prefixed encoding of the normalized expected Place revision, prose,
consequence tag, Entity name and Entity description. It never fingerprints raw JSON
or field order. In one transaction it locks the User and first resolves an existing
`(requested_by_user_id, request_id)`: an equal fingerprint returns the stored
Activity, Entity and Place, while a different fingerprint returns a deterministic
conflict. Only for an unseen request id does World require an existing placed
Character, re-derive and lock that Character's current Place and compare
`expected_place_revision` with the current exact-Place revision. A mismatch returns a
typed precondition failure and writes nothing. This ordering makes a delayed retry
return its canonical accepted result even after later Place state changed.

Every operation that changes this exact-Place representation must use the same Place
lock before appending its Place-relevant Activity. Current affected writers are
`enter_world`, whose accepted Character arrival appears in Place Activity;
`create_entity` when its acting Character has a current Place, even though the Entity
itself remains unplaced; and `submit_action`. `create_entry_place` establishes a new
Place and its initial revision in its creation transaction. Mutations at other Places
remain concurrent. Revision tokens are derived from durable state by side-effect-free
reads; no preparation row, token expiry or durable Agent session is introduced.

HTTP and MCP are thin adapters over these same types. Working routes are POST
`/api/action`, GET `/api/place/current/entity` and GET
`/api/place/current/activity`, with MCP names equal to the World operations. Local
pages reuse typed `(time, id)` keyset ordering and opaque transport cursors. MCP
initialization instructions define the private-workshop sequence; tool descriptions
make `submit_action` the sole irreversible commit and say that Agent proposals are
not World state. The catalog fixture, OpenAPI and parity tests change together.

The server validates only deterministic structure: request id, text bounds, User,
Character, current Place, accepted consequence kind, Entity/Place constraints,
retry identity and transaction constraints. It cannot prove English, literary
quality, consistency between prose and description or that exactly three proposals
were shown. Those remain Agent-interaction obligations and require separate observed
Agent evidence if claimed.

Current error contract:

- malformed wire input or unknown fields: `invalid_request` / HTTP 400;
- malformed opaque Place revision: `invalid_request` with field
  `expected_place_revision` / HTTP 400;
- invalid action prose or consequence text: `invalid_action` with field and reason /
  HTTP 400;
- missing Character: `character_not_found` / HTTP 404;
- unplaced Character: `character_not_entered` / HTTP 409;
- reused request id with different content: `action_request_conflict` / HTTP 409;
- changed exact Place representation: `place_revision_conflict` / HTTP 412;
- unavailable storage: existing `unavailable` / HTTP 503.

## Decisions, assumptions and open questions

### Confirmed decisions

- This trail-marker action is the selected first vertical outcome; it replaces direct
  Entity placement as the current edge.
- The Agent performs grounded reasoning and three private proposals; the User selects
  and may steer; only the final package is submitted.
- Selection and steering approve a direction, not the exact irreversible package.
  The Agent must preview the complete final prose and structured consequences and
  receive one explicit User confirmation immediately before submission.
- Agent, HTTP and MCP never write storage. Every granular and bundled command crosses
  the World interface and only World deterministically accepts and mutates state.
- The usual player action is one bundled mutating submission and one Activity; reads
  may use several granular MCP calls and standalone granular mutations remain valid
  separate actions.
- The Agent composes context from separate typed World, Character, Place and spatial
  reads. `Place neighborhood` means bounded explicit containment and adjacency
  relationships, not coordinates, metric distance, inferred prose or visibility.
- The first slice uses the existing exact entry Place only. Additional Places,
  containment and adjacency remain a later spatial edge; the exact Place Entity and
  Activity/prose reads ship as supporting capabilities in this action build.
- Prose and explicit structured consequences are submitted together. World never
  interprets prose to invent consequences.
- Accepted prose is immutable and append-only. Every history lens references the same
  canonical Activity/prose record; no later action edits or deletes it.
- World acceptance establishes the sole current chronology. The action accepts no
  effective-time field, cannot be backdated and is ordered identically through every
  history lens by `(occurred_at, activity_id)`.
- First-slice action placement is the server-derived exact current Place. The action
  accepts no Place selector and a Character must already be entered.
- Acceptance is atomic; partial success is forbidden.
- The first structured consequence introduces one Entity only. Updates, movement,
  multiple consequences and local references remain absent.
- Proposals, steering, rejected packages and private reasoning are not stored and do
  not create Activity.
- Another Character at the same Place must be able to query the Entity and readable
  action history through World, HTTP and MCP.
- One intended action uses one Agent-generated UUID that remains stable only across
  uncertain delivery retries. World derives and stores a versioned fingerprint of
  normalized accepted input; equal retry content returns the canonical result and
  different content under the same id conflicts.
- Exact-Place Entity and Activity/prose reads return one opaque strong Place revision.
  A new action carries that value as `expected_place_revision`; World locks the
  derived current Place and rejects an unseen request when same-Place state changed,
  while activity elsewhere does not invalidate it.
- Accepted request identity is resolved before current Character/Place preconditions,
  so a late retry still returns the original result. Reads issue no nonce, reserve no
  action and create no session; there is no global revision or cross-call database
  snapshot.
- One continuous passing paid live Agent playtest is a mandatory completion gate.
  `run-G8k1sTRm` and `run-nvULnvxQ` each consumed their separate authorization and
  remain rejected/partial candidates. A future candidate requires a new explicit
  authorization after the revised token-free gate reports GO; plan acceptance alone
  never authorizes token spend, a model call or an automatic retry.
- Each live evidence layer has one earned role. The action Agent proves the private
  proposal/selection/preview/confirmation interaction and exactly one submission;
  HTTP proves the complete authoritative stored result independently of Agent
  interpretation; the observer proves that a clean-room Agent can find and understand
  the local Entity and canonical prose through only its three granted MCP reads;
  ownership verification proves isolation and safe cleanup.
- Authoritative HTTP validation runs immediately after the accepted commit and before
  the observer. The final candidate still passes only when both validations and
  cleanup pass; ordering merely preserves the authoritative evidence if later Agent
  interpretation fails.
- Entity description remains required in the preview, committed Entity and HTTP
  validation. It is absent from the observer result because the granted local Entity
  summary cannot expose it. Adding `get_entity` solely to duplicate the HTTP assertion
  would not earn another tool call.
- After T4R2 returned independent GO on the frozen executable candidate, the User
  explicitly authorized exactly one T4R3 paid live candidate on 2026-08-11. This
  authorization is consumed when that candidate starts and never permits a retry.
- That authorization was consumed by `run-gE8iED5m`. Its retained manifest reports
  proposal, preview, commit, authoritative HTTP, observer and cleanup passed with
  `run_status: completed`; T4R4 independently verifies the raw evidence before any
  authority or backlog item may call the outcome complete.

### Reversible assumptions

- Use the existing text normalization for the introduced Entity; choose the prose
  length bound during contract installation without changing its semantic role.
- Reuse existing keyset pagination conventions for both local reads.
- Keep `create_entity` unchanged as a granular unplaced introduction; reconsider only
  when a concrete gameplay or operational caller proves it redundant.
- Use `submit_action` as the working operation name and `prose` as the working field
  name until the grill closes nomenclature.

### Open questions

No product, domain or authorization question is open. The User explicitly
re-accepted the revised T4 recovery on 2026-08-11 and, after independent T4R2 GO,
separately authorized exactly one paid T4R3 candidate. No second candidate or phase
retry is authorized.

## Implementation map

| Surface | Current state | Intended change | Invariants |
| --- | --- | --- | --- |
| `docs/game/README.md` | Governs the implemented first action, prose, placement, local reads, retry and errors | Freeze; change only if evidence disproves current behavior | Current state remains authoritative; no event sourcing or server intelligence |
| `docs/game/agent-interface.md` | Publishes all thirteen capabilities and the private action workflow | Freeze | User/Character/Place ids remain derived where contextual |
| `migration/0005_agent_action.sql` | Stores Entity location, action prose/request identity and the Place's authoritative Activity pointer | Freeze | No historic fabrication; one Entity identity; immutable history |
| `Cargo.toml`, `Cargo.lock` | Contain the conventional SHA-256 dependency used for fingerprints | Freeze | Fingerprint format is versioned and independent of JSON serialization |
| `src/world.rs`, `src/lib.rs` | Implement and export the accepted action, local reads, validation and transactions | Freeze | World alone writes; all-or-nothing; exact roles and context |
| `src/wire.rs` | Implements strict shared wire types, typed cursors, Place revision and error mappings | Freeze | Unknown fields rejected; adapters share semantics |
| `src/server.rs` | Implements thirteen thin HTTP/MCP capabilities and action guidance | Freeze | Complete catalog parity; no adapter orchestration |
| `tests/world.rs` | Forty-one passing World tests cover persistence, concurrency, rollback and Activity evidence | Re-run unchanged in T4R2 | Evidence matches only the first consequence type |
| `tests/server.rs`, `tests/agent-tool-catalog.json` | Nine passing server tests and exact thirteen-tool fixture prove adapter parity | Re-run unchanged in T4R2 | Published Agent contract is complete and deterministic |
| `tools/agent-playtest`, `tools/agent-playtest-schema/*`, `tests/agent-playtest.sh` | Minimal observer, HTTP-before-observer order, separate phase results and exact Activity-actor comparison passed token-free and live evidence | Freeze until a new concrete claim earns a change | No product/API change; no fourth observer tool; fake evidence can never become live evidence; no automatic run |
| `docs/game/agent-playtest.md` | Records both earlier candidates, the accepted protocol and independently verified `run-gE8iED5m` | Freeze | Past manifests and evidence claims are immutable history |
| `docs/concept/log/log.md`, `.agents/backlog/` | Record accepted build choices, evidence corrections and completed outcome | Freeze; a next edge requires separate selection | Planning does not override `docs/game/` |

## Execution contract

Root owns outcome, scope, plan state, integration and the final evidence claim. No
sub-agent delegation is authorized unless the User explicitly requests it. Execute
the remaining tasks sequentially: the harness contract constrains the audit, the
audit constrains whether token authorization may be requested, and only an observed
candidate can close the authorities.

T1-T3 are frozen and remain completed. A delegated builder for T4R1 receives only
the playtest harness, schema, fake-test and playtest-doc surfaces; it must stop if the
change would touch World, migration or public adapter behavior. T4R2 is read-only and
must be independent of the T4R1 implementation. After its GO result, no file changes
are allowed before T4R3; any finding returns the plan to T4R1 and invalidates the GO.
T4R3 cannot begin on plan acceptance: root first presents the exact token-free result
and receives a fresh explicit one-candidate authorization. T4R4 records the outcome
truthfully whether the candidate passes or fails.

The first T4R2 audit returned NO-GO with one P1: authoritative HTTP validated the
Activity operation, prose, Place, subject and location but did not compare its
`actor_character` to the action Character returned by HTTP. T4R1 is reopened only to
add that existing claimed comparison and one `http-wrong-actor` fake regression;
there were no other P0-P3 findings. That bounded correction is now focused green:
the wrong-actor case retains commit passed, marks HTTP/validation failed, never starts
the observer and drops only the owned database. The full audit now restarts.

The restarted T4R2 audit returned GO on executable candidate fingerprint
`95600a0777a1375a310ee079254dbbcaf43ae123a921671bf2ed4d971c2a37f9`: all 57 Rust
tests, 27 fake runner invocations, 19 fail-closed modes, public preflight, full
operational review and cleanup passed with no unresolved P0-P3 finding or leftover
resource. No live/model call occurred. Only plan/backlog/concept status is recorded
after this audit; any executable runner, schema, test, game-contract or source change
invalidates GO and returns to T4R1/T4R2.

T4R3 then ran exactly one authorized candidate, `run-gE8iED5m`; the runner reports
all five phases and authoritative validation passed, `run_status: completed`,
mode-`700` evidence, mode-`600` manifest and ownership cleanup `dropped`.

Independent T4R4 review then verified every raw interaction, state and cleanup claim:
exact calls and withholding, one submission, matching canonical ids/prose and HTTP
roles/counts, minimal observer inputs, private artifacts and zero leftovers. It found
no P0-P3 issue, staleness or drift. The current evidence authority, concept log,
backlog and this plan now record that bounded completion.

## Task graph

| ID | State | Depends | Parallel-safe | Objective | Owned surfaces | Evidence |
| --- | --- | --- | --- | --- | --- | --- |
| T1 | completed | — | no | Install the accepted behavior and Agent contract | `docs/game/*`, concept log, backlog, plan | Every confirmed actor/action/state/history/error boundary is explicit |
| T2 | completed | T1 | no | Implement atomic World state, history and revisioned local reads | migration, `src/world.rs`, `src/lib.rs`, `tests/world.rs` | Focused World tests prove acceptance, rollback, retry, Place freshness and cross-User local state |
| T3 | completed | T2 | no | Ship HTTP/MCP parity and Agent guidance | `src/wire.rs`, `src/server.rs`, `tests/server.rs`, catalog fixture | Both adapters expose identical action and read semantics |
| T4R1 | completed | T3 | no | Make every live evidence layer minimal, observable and independently retained | `tools/agent-playtest`, `tools/agent-playtest-schema/*`, `tests/agent-playtest.sh`, `docs/game/agent-playtest.md` | Live-shaped fake paths, public preflight and wrong-actor regression pass token-free; zero model calls and leftover databases |
| T4R2 | completed | T4R1 | no | Independently audit the frozen pre-spend candidate | read-only review of all changed surfaces and local operations | GO on fingerprint `95600a…2a37f9`: 57 Rust tests, 27 fake invocations, 19 fail-closed modes, preflight and zero-resource checks pass; no P0-P3 findings |
| T4R3 | completed | T4R2 + fresh explicit authorization | no | Run exactly one continuous live candidate | private `.aicadia-playtest/run-gE8iED5m/` evidence and its owned disposable database | Runner reports proposal, preview, commit, HTTP, observer and cleanup passed continuously; no retry occurred |
| T4R4 | completed | T4R3 | no | Reconcile the exact outcome and close or retain the edge | plan, `docs/game/agent-playtest.md`, concept log, backlog | Independent raw review GO; authorities aligned, backlog Done and zero leftovers |

## Task details

### T1 — Accepted contract

**Objective:** Turn the accepted grill answers into one current executable contract
before code or schema changes.

**Actions:**

1. Resolve every open question and receive explicit plan acceptance.
2. Amend `docs/game/README.md` and `docs/game/agent-interface.md` with exact domain,
   interaction, World, HTTP, MCP, error, history and evidence contracts.
3. Align the active concept record, backlog item and plan state without rewriting
   exploration history.

**Invariants:**

- `docs/game/` is the only current executable truth.
- The contract contains one concrete consequence and no generic mutation language.
- Agent proposal generation remains private and World remains deterministic.

**Evidence:**

- Focused contract review against this plan — no unresolved actor, selector,
  authority, mutation, retry, history or visibility behavior.

**Stop conditions:**

- Stop if prose lifecycle, final User approval, read shape, retry identity or evidence
  scope remains materially open.

### T2 — Persistence and World

**Objective:** Make one accepted trail-marker action and both local reads authoritative
through World with exact atomic, retry and Place-freshness behavior.

**Actions:**

1. Add only the accepted Entity-location and Activity prose/request/fingerprint
   storage, SHA-256 support, constraints, immutability and earned indexes.
2. Add normalized domain types and private transaction helpers reused by granular and
   bundled behavior without allowing nested public transactions.
3. Implement side-effect-free exact current-Place Entity and Activity reads carrying
   one strong revision in short Repeatable Read transactions, then
   submit/retry/request-conflict/stale-revision behavior under the shared Place-lock
   discipline for every current Place-relevant writer.
4. Prove same-request concurrency, same-Place stale concurrency, rollback,
   late retry after Place change, persistence, unrelated-Place isolation, pagination
   and two-User visibility in focused World tests.

**Invariants:**

- Agent and adapters never touch storage.
- One accepted User action creates one Activity; a retry creates nothing.
- Only one of two distinct same-Place actions based on the same revision can commit;
  activity at another Place does not invalidate this action.
- Every returned exact-Place page and its revision come from one database snapshot;
  no transaction survives the World call.
- Activity prose is never used to infer current state.
- Existing Entity, Character, Place and Activity behavior remains intact.

**Evidence:**

- `cargo test --test world action` — focused action acceptance, concurrent and late
  retry, request conflict, stale revision, Place locking and rollback.
- `cargo test --test world current_place` — focused exact-local read and isolation.
- focused concurrency tests prove `enter_world`, placed-actor `create_entity` and
  `submit_action` all advance and serialize the same exact-Place revision.

**Stop conditions:**

- Stop if implementation needs movement, an existing-Entity update, generic payload,
  second public behavior seam or server-side semantic interpretation.

### T3 — Adapters and catalog

**Objective:** Publish the accepted World behavior identically through HTTP and MCP
with sufficient instructions for an Agent to perform the private workshop.

**Actions:**

1. Add strict wire input/output, errors, opaque cursors and opaque Place revisions.
2. Add three thin HTTP routes and MCP tools over the same World methods.
3. Update initialization instructions, tool descriptions, OpenAPI, exact catalog
   fixture and cross-adapter tests in the same change.

**Invariants:**

- HTTP/MCP never sequence granular writes to emulate the action.
- MCP descriptions distinguish private proposals from the one public commit.
- User, Character and current Place ids are absent from contextual input.

**Evidence:**

- `cargo test --test server action` — HTTP/MCP semantic and error parity.
- `cargo test --test server catalog` — exact instructions, schemas and tool list.

**Stop conditions:**

- Stop if one adapter needs state, authority or behavior the World interface does not
  own.

### T4R1 — Earned evidence contract

**Objective:** Make the live runner ask only questions its granted surfaces can
answer and retain each independent evidence layer as soon as it exists.

**Actions:**

1. Freeze `World`, migration, HTTP and MCP public behavior. Confirm the observer
   schema, prompt and controller require only the exact Entity id/name, Place id and
   canonical prose available through `get_character`,
   `list_entity_at_current_place` and `list_activity_at_current_place`; do not grant
   `get_entity` or pass expected ids/prose out of band.
2. After the action Agent's exactly-one `submit_action` result is validated, run the
   complete authoritative HTTP checks before starting the observer. Persist commit,
   HTTP and observer statuses separately. Keep overall `run_status` non-complete
   until proposal, preview, commit, HTTP, observer and verified cleanup all pass.
3. Keep Entity description in preview, World result and HTTP equality/count checks.
   Remove no deterministic assertion simply because it is absent from the observer's
   narrower local summary.
4. Add or sharpen one fake success case that mirrors the actual Codex JSONL attempt
   shapes observed in `run-nvULnvxQ`, including started/completed tool-call records,
   and failure cases for wrong observer id/name/prose, extra or incomplete submit
   attempt, authoritative duplicate state and cleanup refusal. Do not commit private
   live prompts, tokens or raw evidence as a fixture.
5. Align `docs/game/agent-playtest.md` to the accepted order and keep both earlier run
   records unchanged as rejected/partial history.

**Invariants:**

- The action Agent, World/HTTP, observer and cleanup each prove a different claim.
- Fake mode can never emit `live_candidate` or `completed` live evidence.
- The controller never reveals selection before proposals, confirmation before the
  preview, or accepted ids/prose to the observer.
- A failed phase never triggers another Agent/model attempt.

**Evidence:**

- `/bin/bash tests/agent-playtest.sh` — live-shaped success and fail-closed paths pass
  without a model call or persistent database.
- `DATABASE_URL=postgres://localhost:5433/postgres tools/agent-playtest preflight` —
  exact CLI/model/reasoning, schemas, catalogs, allowlists and ownership-safe
  create/verify/drop pass without model generation.
- Focused source/fixture review — HTTP precedes observer and observer has exactly its
  three earned reads and four observable result values.

**Stop conditions:**

- Stop if satisfying a check requires a new public capability, World change, fourth
  observer read, hidden expected value or weaker authoritative assertion.

### T4R2 — Frozen pre-spend audit

**Objective:** Establish an independent token-free GO on the exact code that would
run, with no mutation between review and authorization.

**Actions:**

1. Review the public runner path, environment isolation, schema subset, exact tool
   allowlists, Codex resume phases, all MCP attempt accounting, HTTP uniqueness,
   name-plus-token database ownership, cleanup signals, file permissions and failure
   recovery against the evidence claim.
2. Run the complete deterministic ladder and inspect the focused diff for unearned
   concepts, stale authorities and unrelated user changes.
3. Verify no `aicadia_playtest_%` database, server or Agent process remains. Publish
   the exact GO/NO-GO result and freeze the candidate if GO.

**Invariants:**

- This task is read-only. Any required edit returns control to T4R1 and requires the
  audit and preflight to run again from the start.
- A green fake suite proves the controller, not real Agent behavior; it cannot replace
  T4R3.

**Evidence:**

- `cargo fmt --check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `DATABASE_URL=postgres://localhost:5433/postgres cargo test`
- `/bin/bash -n tools/agent-playtest tests/agent-playtest.sh`
- `/bin/bash tests/agent-playtest.sh`
- `git diff --check`
- read-only PostgreSQL/process inspection — zero disposable leftovers.

**Stop conditions:**

- Return NO-GO for any unresolved finding, drift, unowned cleanup target, test
  failure, stale authority or post-audit file change. Do not request token approval.

### T4R3 — One separately authorized live candidate

**Objective:** Produce one continuous, private and independently checkable outcome
record under the frozen GO-reviewed runner.

**Actions:**

1. Present T4R2's exact GO result to the User and obtain explicit authorization for
   one candidate. Plan re-acceptance alone is insufficient.
2. Run exactly
   `DATABASE_URL=postgres://localhost:5433/postgres tools/agent-playtest run --confirm-token-spend`
   once. Never automatically retry a phase or start a second candidate.
3. Require, in order: three grounded proposals; withheld selection plus steering;
   an unchanged preview; explicit confirmation; exactly one submission attempt and
   result; complete authoritative HTTP state and exact counts; minimal independent
   observer result; server stop; exact ownership re-verification and database drop.
4. Retain the mode-`700` run directory and mode-`600` manifest. Independently inspect
   its phase statuses, canonical ids/prose, raw MCP attempt ids/statuses, authoritative
   counts, cleanup result and absence of a leftover database.

**Invariants:**

- The run proves one bounded Agent/model/setup, not universal Agent quality.
- World is the only state authority; HTTP and MCP merely expose the same result.
- A partial run remains partial even if its core action succeeded.

**Evidence:**

- One new manifest has `evidence_kind: live_candidate`, passed proposal, preview,
  commit, authoritative HTTP and observer phases, `cleanup_status: dropped` and
  `run_status: completed`.
- Raw private JSONL contains exactly the allowed reads and one unique completed
  `submit_action` attempt; HTTP reports exactly one action Entity and Activity with
  matching actor, Place, roles, description and prose.

**Stop conditions:**

- Stop after this single candidate regardless of outcome. On failure, preserve its
  exact evidence, clean up only when ownership verifies and do not spend again.

### T4R4 — Truthful closure

**Objective:** Make plan, game evidence authority, concept history and backlog state
say exactly what the candidate proved.

**Actions:**

1. If T4R3 passed every gate, record the retained run id and bounded claim in
   `docs/game/agent-playtest.md`, record the accepted evidence/operational choices in
   the concept log, mark the backlog item Done and complete this plan.
2. If any gate failed, record the exact passed and failed evidence, keep the backlog
   Active and return this plan to `draft` before proposing another correction or run.
   Never rewrite earlier candidates or aggregate partial runs into a pass.
3. Recheck authority links, timestamps, file permissions, cleanup state and the final
   diff.

**Invariants:**

- Documentation follows evidence; it cannot promote it.
- The next game edge is not started automatically after closure.

**Evidence:**

- Direct manifest/JSONL/HTTP comparison supports every sentence in the final record.
- `git diff --check` and focused authority review are clean.

**Stop conditions:**

- Do not mark T4, the backlog item or this plan complete unless one individual live
  candidate passed every required gate.

## Validation ladder

1. **Focused:** The live-shaped fake suite proves the corrected observable observer,
   HTTP-before-observer order, exact attempt accounting, phase isolation, ownership
   refusal and successful cleanup without a model call.
2. **Contract:** Existing World, HTTP, MCP, OpenAPI and catalog tests remain green;
   no public behavior changed to serve the harness.
3. **Deterministic outcome:** Existing World/server suites prove exactly one atomic
   marker/prose/Activity, rollback, retry, stale Place revision, local visibility and
   cross-adapter parity for two Users.
4. **Pre-spend integrity:** Public preflight, formatting, strict lint, full tests,
   syntax, `git diff --check`, independent source review and zero leftover resources
   all pass on one frozen candidate.
5. **Live outcome:** One separately authorized resumed action Agent passes proposals,
   preview, confirmation and exactly one commit; HTTP proves complete authoritative
   state; a separate least-privilege Agent finds the local Entity and prose; exact
   ownership cleanup passes in the same retained candidate.
6. **Authority integrity:** Raw evidence, game docs, concept log, backlog and this plan
   agree without modifying or combining earlier failed candidates.

## Change control

After re-acceptance, refine only private helper placement and stronger token-free
evidence inside T4R1. Stop implementation, keep or return `status: draft`, revise and
request explicit re-acceptance if actor roles, public behavior, observer capabilities,
gate meaning/order, cleanup authority, external token cost or the final evidence
claim changes. T4R2 GO freezes the runnable files; any later edit invalidates GO.
Starting T4R3 always requires a separate, fresh one-candidate authorization.

## Completion conditions

- every open material question is resolved and the complete plan explicitly accepted;
- T1-T3 and T4R1-T4R4 are `completed` and the validation ladder passes;
- one World-controlled action creates exactly one placed marker, one immutable prose
  record and one Activity, all visible locally to another Character;
- invalid submission and delivery retry prove zero partial or duplicate state;
- World, HTTP and MCP agree and Agent/database separation is absolute;
- one fresh, individually authorized paid clean-room candidate passes proposals,
  preview, commit, authoritative HTTP, minimal observer and ownership-cleanup
  assertions continuously, with `run_status: completed`;
- current docs, vocabulary, backlog, concept record and plan agree;
- no unrelated change is overwritten, no prior candidate is relabelled and no live
  Agent spend repeats without another explicit plan revision and approval;
- `status: complete` and `completed_at` are recorded only after these conditions.
