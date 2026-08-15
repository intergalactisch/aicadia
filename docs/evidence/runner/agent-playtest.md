# Agent playtest runner

> **Role / side:** Agent-playtest operation contract / evidence bridge.
> **Authority:** defines clean-room orchestration, isolation, spend gates, validation and recovery.
> **Excludes:** game-contract rules and delivery status; see `docs/game/` and the slice records in `docs/evidence/`.

Delivery history and current status: see [Action evidence](../action.md),
[World-entry evidence](../world-entry.md), [Interaction evidence](../interaction.md),
[Property evidence](../property.md) and [Trait evidence](../trait.md).

The runner defines one bounded clean-room Property claim: a real Agent can
ground itself in separate Aicadia MCP reads, offer exactly three private directions,
incorporate a withheld User selection and steering, retain one exact public package,
wait for a separately withheld explicit confirmation and call `submit_action`
exactly once. A separate Agent must then observe the placed marker and the same
canonical prose at its own Character's exact current Place, including the marker's
complete description and current initial `material = weathered cedar` Property.
This claim does
not prove every model, arbitrary prose quality, later Places, movement, discovery or
future consequence types.

## Trait evidence pointer

Delivery history and current status: see [Trait evidence](../trait.md). The dedicated
[Trait runner contract](trait-playtest.md) owns its token-free scenario and live-gate
machinery.

## Isolation and authority

The controller owns only disposable test setup and evidence orchestration. It:

- creates one uniquely named `aicadia_playtest_*` database and private loopback
  server;
- provisions two distinct disposable Users through the operator-only provisioner;
- establishes two Characters at the one entry Place through the public HTTP
  contract before the Agent workshop begins; and
- independently validates the accepted result through HTTP before dropping that
  complete disposable World.

The controller never inserts game rows directly. Both Agents receive only their
least-privilege MCP subsets from the current thirteen-tool player catalog. They run
in empty isolated working directories, with user configuration and project rules
ignored, no repository or database variables, no shell, web, native application,
delegation or multi-Agent path, and direct-only access to their Aicadia MCP allowlist.
The server performs no model call. The Agents receive no out-of-band or User-supplied
User, Character, Place, Entity or Activity ids; they read authoritative ids from
World responses.

The action Agent's one CLI session is persisted only because phases two and three
must resume its actual grounded context. The observer is a separate ephemeral
session. The runner extracts and resumes one explicit session UUID; it never uses
`--last`, which could select unrelated state.

## Token-free preflight

Use the PostgreSQL administration endpoint selected for this repository:

```sh
DATABASE_URL=postgres://localhost:5433/postgres tools/agent-playtest preflight
```

Preflight starts no Aicadia server or Agent, creates no User, Character, Place,
Entity or Activity, and never invokes `codex exec`. Its database helper performs one
real disposable database create, ownership tag, ownership read-back and verified
drop. It fails closed unless all of these facts agree:

- `codex` is discovered through `PATH`, canonicalized once for the run and is exactly
  `codex-cli 0.147.0`, the installed version inspected during the current delivery
  audit; installation location may differ per machine, while later version or
  capability drift fails before database or model work;
- the locally resolved executable path is retained only in ignored private evidence
  together with its inspected version, model and reasoning effort; public preflight
  and run reject `CODEX_BIN`, fake-mode and test-output overrides before any Codex
  command, so repository configuration cannot redirect the executable; environment
  inspection is Bash-native and every server or Agent child is launched through
  exact `/usr/bin/env`, never a PATH-resolved `env` or `printenv`;
- `gpt-5.6-sol` exists with exactly pinned `high` reasoning and Codex is logged in;
- initial exec and explicit-session `exec resume` expose every required strict
  output, configuration and JSONL flag;
- all four closed output schemas use the supported Structured Outputs subset and the
  compiler-generated catalog contains exactly thirteen complete tools; recursive
  local policy rejects `uniqueItems`, string-length keywords and unsupported
  composition before Codex, while requiring every object schema to close additional
  properties and require exactly its declared properties;
- the action-read role contains only `get_world`, `get_character`,
  `list_entity_at_current_place` and `list_activity_at_current_place`;
- the commit role contains only `submit_action`;
- the observer role contains only `get_character`,
  `list_entity_at_current_place`, `get_entity_at_current_place` and
  `list_activity_at_current_place`;
- Code Mode exists only as local direct MCP routing for `mcp__aicadia`; Codex
  explicitly enables the sole supported MCP wire protocol revision `2026-07-28`
  and fails closed without it—there is no older initialize flow, downgrade,
  fallback or compatibility shim—while shell, web, application, plugin, skill,
  elicitation and delegation features are disabled;
- operator binaries build and PostgreSQL completes the same name-plus-token
  ownership route required by live cleanup.

The exact version and feature checks intentionally make semantic Agent-host drift a
pre-spend blocker without making one developer's installation directory part of
the repository contract.

## Paid run protocol

Any candidate crosses a separately authorized token-spend gate through:

```sh
DATABASE_URL=postgres://localhost:5433/postgres tools/agent-playtest run --confirm-token-spend
```

Delivery history and current status: see [Action evidence](../action.md). Every run
requires its own concrete evidence claim and explicit authorization. The protocol is:

The exact spend boundary is four `gpt-5.6-sol` high process calls and zero retries:
three calls in one resumed action session and one separate ephemeral observer call.
Codex CLI 0.147.0 exposes no enforceable per-run token ceiling, so the controller
records that honest process-call boundary and any usage events Codex emits instead
of claiming a token maximum.

1. Before database mutation it creates a mode-`700` run directory and mode-`600`
   recovery manifest, records an unguessable 64-hex ownership token and the exact
   inspected Codex path/version/model/reasoning, and writes all evidence files with
   private permissions. The manifest is a `live_candidate`; it becomes completed
   evidence only after every phase and verified cleanup pass.
2. It creates the database with collision-failing `CREATE DATABASE`, stores the
   ownership token in the database comment and reads that tag back. Automatic
   cleanup is armed only after exact name-and-token proof. It then starts the
   disposable deployment, verifies the exact OpenAPI operation ids
   and byte-meaning-equivalent complete MCP tool catalog, provisions two Users and
   establishes their distinct Characters at the same entry Place.
3. Action phase one exposes only four reads. The Agent must call each exactly once
   in the required order, use three equal opaque Place revisions, return exactly three
   distinct grounded proposals with ids `one`, `two`, `three`, and make no mutation.
4. Only after phase-one JSONL and strict final validation does the controller reveal
   selection `two` plus the retained steering. The same explicit session resumes.
   Phase two may call no MCP tool and must return one exact English prose plus one
   `introduce_entity` consequence. Prose, name and description each contain the
   unique evidence marker.
5. Only after that exact preview is retained and validated does the controller send
   explicit confirmation containing the unchanged package. The same session resumes
   with only `submit_action`; it must create one UUID and make exactly one call using
   the unchanged Place revision and exact preview. Validation counts every observed
   MCP call id and status, including incomplete attempts: exactly one unique
   `submit_action` attempt and exactly one completed result are permitted.
6. Immediately after validating the commit, the controller independently reads both
   Characters, the Place-local Entity summary, that Entity's scoped complete current
   state and Place Activity over HTTP. It requires exactly one matching marker among
   the possibly other co-located Entity summaries and exactly one `submit_action`
   Activity, with the same Place,
   Entity, complete description, exact initial Property, matching Property history,
   prose, actor, `subject` and `location` roles. This authoritative gate finishes
   before another Agent can start.
7. Only after HTTP passes, a separate least-privilege observer Agent reads its
   Character, placed Entity summaries, the marker Entity's scoped complete current
   state and Place Activity exactly once each, in that order. It must copy the exact
   Entity name and description, canonical prose, Entity id, Place id and
   `material = weathered cedar` Property after deriving the Entity id from compact
   orientation; none is supplied out of band. All four reads must share one opaque
   Place revision. The three pageable reads accept an omitted limit or any explicit
   schema-valid limit from 1 through 100, but never a cursor in this one-page
   scenario.
8. It stops the server and asks the helper to drop only after it re-verifies both the
   generated database name and stored ownership token. A prefix alone never
   authorizes deletion. Token mismatch refuses termination and drop. Cleanup failure
   keeps the exact private token-bearing recovery command in the mode-`600` manifest
   and fails the run. Ambiguous creation without ownership proof is never dropped
   automatically and instead records a manual-inspection requirement.

Selection and confirmation are absent from every earlier prompt. A tool call in the
preview phase, a call other than `submit_action` in the commit phase, zero or two
submissions, an incomplete or second call attempt, malformed output, inconsistent
ids, prose, description or Property, duplicate authoritative Entity/Activity state,
observer failure,
authoritative-state disagreement or cleanup failure all fail the run. Failure never
starts a second Agent attempt. Another paid run always requires fresh User
authorization.

## Evidence and recovery

### Property evidence pointer

Delivery history and current status: see [Property evidence](../property.md).

### Interaction evidence pointer

Delivery history and current status: see [Interaction evidence](../interaction.md).

Every public run remains under `.aicadia-playtest/`. Its private manifest correlates
the run id, generated database and ownership token, loopback address, exact Codex
path/version/model/reasoning, deadline,
disposable User ids, separate proposal, preview, commit, HTTP and observer passed or
failed statuses, accepted Entity and Place ids, authoritative validation and cleanup.
Usage events are aggregated into the manifest after every completed model process,
including terminal failures before the observer.
Overall status becomes complete only after every phase and ownership-verified
cleanup pass. Prompts, schema-constrained finals, JSONL events, stderr and server
logs remain private inside the mode-`700` directory. Credentials are never recorded.

The fake integration suite uses an explicit internal-only command path, spends no
tokens and writes `evidence_kind: fake_controller_test` plus
`run_status: fake_completed`; it can never produce `completed` live evidence. It
proves public fake/executable override rejection, portable PATH discovery and
semantic CLI drift rejection before Codex execution, the controller's happy path,
withheld selection and confirmation, exact explicit-session resume, environment and
allowlist isolation, premature phase-one and phase-two mutation rejection, absent
and double commit rejection, incomplete extra commit-attempt rejection, malformed
proposal/preview/commit rejection, exact authoritative count checks, observer
failure, server failure, ownership mismatch, ambiguous create without auto-drop,
verified cleanup, cleanup failure, CLI drift and no-token preflight.

The fake success events use the same paired `item.started`/`item.completed` MCP
attempt shape and per-attempt identity observed in the second live candidate. They
also prove that authoritative HTTP runs after commit and before the observer. An
HTTP failure leaves the observer unstarted; an observer failure retains the already
passed HTTP phase. Fake evidence remains structurally distinct from live evidence.

The current observer fixtures require the combined scoped Entity read, exercise its
valid explicit `limit: 100`, and reject a wrong Entity id, name, description, prose
or Property. The fake suite pins the
compiler-generated current thirteen-tool catalog. A separate immutable fixture
still proves the exact historical Property-era catalog as history; it is never used
by public preflight or live execution.

The suite also injects a forbidden `uniqueItems` keyword into an isolated schema
copy; preflight proves rejection before operator build, any Codex command or evidence
creation. Array `minItems` and `maxItems` and UUID `format` remain in the published
schemas. String non-emptiness, semantic bounds, exactly three proposals, proposal ids
and distinct directions/grounding are proved by the controller and World instead of
unsupported or redundant schema keywords.

Cleanup intent and the unguessable token are recorded before `CREATE DATABASE`, but
cleanup is armed only after the database proves that exact token. Ordinary exit and
`INT`, `TERM` or `HUP` use the same guarded cleanup route. Never point this runner at
a persistent game database; it always creates and proves ownership of a separate
deployment.

## Delivery records

Delivery history and current status: see [Action evidence](../action.md) and
[World-entry evidence](../world-entry.md).
