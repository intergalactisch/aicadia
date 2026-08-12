# Agent playtest

Status: resumed-action live acceptance complete in independently verified candidate
`run-gE8iED5m`

The runner now verifies one bounded clean-room gameplay claim: a real Agent can
ground itself in separate Aicadia MCP reads, offer exactly three private directions,
incorporate a withheld User selection and steering, retain one exact public package,
wait for a separately withheld explicit confirmation and call `submit_action`
exactly once. A separate Agent must then observe the placed marker and the same
canonical prose at its own Character's exact current Place.

World mechanics, wire parity and failure behavior remain deterministic test claims.
The paid run proves only this one Agent interaction under the pinned setup. It does
not prove every model, arbitrary prose quality, later Places, movement, discovery or
future consequence types.

## Isolation and authority

The controller owns only disposable test setup and evidence orchestration. It:

- creates one uniquely named `aicadia_playtest_*` database and private loopback
  server;
- provisions two distinct disposable Users through the operator-only provisioner;
- establishes two Characters at the one entry Place through the public HTTP
  contract before the Agent workshop begins; and
- independently validates the accepted result through HTTP before dropping that
  complete disposable World.

The controller never inserts game rows directly. Both Agents receive only MCP tools
from the published thirteen-tool catalog. They run in empty isolated working
directories, with user configuration and project rules ignored, no repository or
database variables, no shell, web, native application, delegation or multi-Agent
path, and direct-only access to their Aicadia MCP allowlist. The server performs no
model call. The Agents receive no out-of-band or User-supplied User, Character,
Place, Entity or Activity ids; they read authoritative ids from World responses.

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

- the executable is exactly `codex-cli 0.144.1`, the installed version inspected
  during this build; the earlier `0.147.0` pin described a binary not present here
  and was corrected before any token spend;
- the exact resolved system executable path
  `/opt/homebrew/lib/node_modules/@openai/codex/bin/codex.js` is pinned and retained
  together with its inspected version, model and reasoning effort; public preflight
  and run reject `CODEX_BIN`, PATH substitution, fake-mode and test-output overrides
  before any Codex command; environment inspection is Bash-native and every server
  or Agent child is launched through exact `/usr/bin/env`, never a PATH-resolved
  `env` or `printenv`;
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
  `list_entity_at_current_place` and `list_activity_at_current_place`;
- Code Mode exists only as local direct MCP routing for `mcp__aicadia`, while shell,
  web, application, plugin, skill, elicitation and delegation features are disabled;
- operator binaries build and PostgreSQL completes the same name-plus-token
  ownership route required by live cleanup.

The exact version and feature checks intentionally make local Agent-host drift a
pre-spend blocker.

## Paid run protocol

The completed candidate crossed the separately authorized token-spend gate through
this exact command:

```sh
DATABASE_URL=postgres://localhost:5433/postgres tools/agent-playtest run --confirm-token-spend
```

The first 2026-08-11 authorization was consumed by the schema-rejected candidate.
After token-free correction, the User explicitly granted one rerun; that second
candidate exposed the unobservable observer-description check and consumed its
authorization. After the minimal evidence correction, independent token-free GO and
one final explicit authorization, `run-gE8iED5m` completed. No additional run is
authorized or needed for this outcome. Any future run would require a new concrete
claim and authorization. The completed protocol was:

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
   in the required order, use two equal opaque Place revisions, return exactly three
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
   Characters, the placed Entity and the Place-local Entity and Activity pages over
   HTTP. It requires exactly one placed
   action Entity and exactly one `submit_action` Activity, with the same Place,
   Entity, complete description, prose, actor, `subject` and `location` roles. This
   authoritative gate finishes before another Agent can start.
7. Only after HTTP passes, a separate least-privilege observer Agent reads its
   Character, placed Entities and Place Activity exactly once each. It must copy the
   exact marker Entity name, canonical prose, Entity id and Place id after reading
   those authoritative ids from World; none is supplied out of band. The Place-local
   Entity summary does not expose description, so the observer is not asked to
   fabricate it.
8. It stops the server and asks the helper to drop only after it re-verifies both the
   generated database name and stored ownership token. A prefix alone never
   authorizes deletion. Token mismatch refuses termination and drop. Cleanup failure
   keeps the exact private token-bearing recovery command in the mode-`600` manifest
   and fails the run. Ambiguous creation without ownership proof is never dropped
   automatically and instead records a manual-inspection requirement.

Selection and confirmation are absent from every earlier prompt. A tool call in the
preview phase, a call other than `submit_action` in the commit phase, zero or two
submissions, an incomplete or second call attempt, malformed output, inconsistent
ids or prose, duplicate authoritative Entity/Activity state, observer failure,
authoritative-state disagreement or cleanup failure all fail the run. Failure never
starts a second Agent attempt. Another paid run always requires fresh User
authorization.

## Evidence and recovery

Every public run remains under `.aicadia-playtest/`. Its private manifest correlates
the run id, generated database and ownership token, loopback address, exact Codex
path/version/model/reasoning, deadline,
disposable User ids, separate proposal, preview, commit, HTTP and observer passed or
failed statuses, accepted Entity and Place ids, authoritative validation and cleanup.
Overall status becomes complete only after every phase and ownership-verified
cleanup pass. Prompts, schema-constrained finals, JSONL events, stderr and server
logs remain private inside the mode-`700` directory. Credentials are never recorded.

The fake integration suite uses an explicit internal-only command path, spends no
tokens and writes `evidence_kind: fake_controller_test` plus
`run_status: fake_completed`; it can never produce `completed` live evidence. It
proves public fake/executable override rejection before Codex invocation, the
controller's happy path,
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

Observer fixtures prove that the observable Entity id and name plus canonical prose
are sufficient for a successful observer result, and independently reject a wrong
Entity id, name or prose. Full Entity description remains independently checked
against the retained preview by authoritative HTTP validation.

The fake suite also injects a forbidden `uniqueItems` keyword into an isolated schema
copy and proves preflight rejects it before operator build, any Codex command or
evidence creation. Array `minItems` and `maxItems` and UUID `format` remain in the
published schemas. String non-emptiness, semantic bounds, exactly three proposals,
proposal ids and distinct directions/grounding are proved by the controller and
World instead of unsupported or redundant schema keywords.

Cleanup intent and the unguessable token are recorded before `CREATE DATABASE`, but
cleanup is armed only after the database proves that exact token. Ordinary exit and
`INT`, `TERM` or `HUP` use the same guarded cleanup route. Never point this runner at
a persistent game database; it always creates and proves ownership of a separate
deployment.

## Live evidence history

Authorized candidate `run-G8k1sTRm` ran on 2026-08-11 and was rejected by the API
with `invalid_json_schema` because `proposals.json` contained unsupported
`uniqueItems`. Rejection occurred before model execution and before any MCP tool
call. The disposable database ownership was verified, cleanup finished with
`dropped`, and no `aicadia_playtest_%` database remained. The candidate supplies no
Agent-interaction or game-outcome evidence and exhausted the sole authorization; no
automatic or currently authorized rerun exists.

Token-free correction removed `uniqueItems`, `minLength` and `maxLength` from Agent
output schemas, retained supported array cardinality and UUID format, and added the
recursive fail-closed schema-policy gate described above.

Authorized rerun `run-nvULnvxQ` ran on 2026-08-11. Its proposal, preview and commit
phases passed. The observer's three MCP reads found the correct placed Entity and
canonical prose, but its final validation failed because the harness required an
`entity_description` field that the observer's Place-local Entity summary tool does
not expose. The authoritative HTTP stage was therefore not reached. Cleanup
finished with `dropped`, no `aicadia_playtest_%` database remained, and the candidate
is not complete outcome evidence. The observer schema, prompt and validation were
corrected token-free to require only observable Entity id, name, Place and prose;
the HTTP stage still checks the complete Entity description. It remains strong
partial evidence and is not relabelled by the later successful candidate.

### Completed resumed-action candidate

Separately authorized candidate `run-gE8iED5m` ran once on 2026-08-11 after the
frozen token-free audit returned GO. One resumed `gpt-5.6-sol` action Agent at high
reasoning:

- made the four required grounded reads once each and returned exactly three
  distinct proposals `one`, `two`, `three`;
- received selection `two` plus steering only in its first resume, produced one exact
  preview without a tool call, and received explicit confirmation only in its second
  resume; and
- made exactly one paired `submit_action` attempt/result with request UUID
  `7b2dd549-dcdf-4821-9443-24a308916611`, with no retry or other commit tool call.

World accepted Activity `81c6b9fe-ab8c-402f-ad91-af5dfffae49c`, Entity
`46c5b7d0-116a-46e2-afb2-c35866989969` and Place
`e19e9c85-c68b-4dff-a715-aebcdee749c6`. Authoritative HTTP found exactly one placed
action Entity and one `submit_action` Activity. Its actor is the independently read
action Character, and its Place, `subject` and `location` roles, full description and
canonical prose all match the confirmed preview.

Only after HTTP passed, a separate ephemeral observer made exactly the three granted
reads and copied the same Entity id/name, Place id and prose without receiving those
values out of band. Its stderr contains one non-blocking Codex cache-TTL warning;
the process exited `0`, all calls completed and its canonical output matches.

The private mode-`700` evidence directory contains forty mode-`600` artifacts. Its
manifest is a `live_candidate` with every phase and validation `passed`,
`run_status: completed` and ownership-verified cleanup `dropped`. Independent T4R4
review found no P0-P3 issue, no evidence drift, no second candidate and zero database,
process, listener or isolated-config leftovers. This proves the bounded interaction
under the pinned setup; it does not broaden the product or claim universal Agent or
prose quality.

## Earlier entry proof

The previous runner supplied the narrower entry proof on 2026-08-10 in authorized
run `run-9TOG5yrJ`: two real Agents created distinct Characters, entered the same
entry Place, matched personal Activity through authoritative HTTP state and let the
second Agent observe the first Agent's unplaced shared Entity. That retained result
remains valid historical evidence. It is separate from the completed resumed-action
evidence above.
