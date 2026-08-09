# Agent playtest

Status: expanded token-free contract complete; live acceptance awaits explicit
token-spend confirmation

This runner verifies one bounded clean-room claim: two different real Codex Agents
can understand the published Aicadia MCP contract, create their own Characters,
enter the same server-derived Place and correctly read the resulting personal
Activity. It retains the earlier proof that Agent B can observe the exact shared
Entity introduced by Agent A.

The Agents have no repository, project rules, game database or operator access.
They receive only their least-privilege MCP tool subset, generated schemas, one
scenario prompt and a strict final-output schema. The prompt supplies exact
disposable semantic fixtures but no ids. World derives User, Character and Place.

The test does **not** prove later Places, movement, investigation, discovery or
arbitrary semantic-content quality. Its fixtures are technical evidence in one
disposable World and are never written to the persistent development World.

## Token-free preflight

Set `DATABASE_URL` to a PostgreSQL administration database whose role may create and
drop databases, then run:

```sh
export DATABASE_URL=postgres://localhost/postgres
tools/agent-playtest preflight
```

Preflight makes no domain mutation, starts no Aicadia server and never invokes
`codex exec`. It builds the server, provisioner and disposable-database helper,
checks both strict final schemas, probes PostgreSQL access and asks Codex's
token-free configuration parser to parse the exact role allowlists. It fails closed
unless all of these facts are true:

- CLI version is exactly `codex-cli 0.147.0` and is logged in;
- `gpt-5.6-sol` is available with `high` reasoning;
- the pinned exec flags and disabled native features still exist;
- Agent A receives only `create_character`, `create_entry_place`, `enter_world`,
  `list_activity` and `create_entity`;
- Agent B receives only `get_user`, `create_character`, `enter_world`,
  `list_activity`, `list_entity` and `get_entity`;
- Code Mode routing is enabled only as local execution plumbing while the complete
  `mcp__aicadia` namespace remains direct-only;
- both final schemas are closed objects with exact required fields and typed
  newest-first `activity_operation` arrays.

The exact version pin makes Agent-host drift visible before tokens or game data can
be spent. The confirmed run verifies the complete ten-tool HTTP and MCP catalogs
before either Agent starts; role allowlists deliberately expose fewer tools.

## Confirmed run

Only this exact command crosses the token-spend gate:

```sh
tools/agent-playtest run --confirm-token-spend
```

After repeating preflight, the runner performs exactly two sequential Agent
attempts:

1. Create a mode-`700` run directory and mode-`600` recovery manifest before any
   database mutation.
2. Create one generated `aicadia_playtest_*` PostgreSQL database, start a private
   loopback Aicadia server, verify the exact ten-capability catalogs and provision
   two distinct Users through operator-only `World::create_user`.
3. Agent A creates its Character. Its first `enter_world` must return exactly
   `entry_place_not_found`; it then creates the one entry Place with the supplied
   semantic fixture and retries entry successfully. After entry it creates exactly
   one shared fixture Entity and reads one first Activity page.
4. Agent B starts once even if A exits, times out or returns bad evidence. It creates
   its own Character, enters the existing server-derived Place, reads one personal
   Activity page, gets its contextual User and observes Agent A's exact Entity
   through one `list_entity` plus `get_entity` sequence.
5. Validate exact JSONL server, tool, order, arguments and results. Agent A's
   newest-first operations must be `create_entity`, `enter_world`,
   `create_entry_place`, `create_character`; its newest Activity must identify its
   Character actor, shared Place context and fixture Entity subject. Agent B's must
   be `enter_world`, `create_character`; its newest Activity must identify its
   Character actor and the same Place as both context and destination.
6. Independently read both Characters and Activity pages over HTTP. Their Character
   ids must differ, their complete current Place must be equal, the entry Place and
   all semantic fixtures must match, and the shared Entity must retain User A as
   introducer. Strict finals must copy those authoritative ids and Activity roles.
7. Stop the server and drop the complete disposable database. This is test cleanup,
   never a game `delete` capability.

The expected `entry_place_not_found` CallToolResult is the only allowed game-error
result: it proves that `create_entry_place` followed an observed genesis absence.
Any other error item, failed call, native tool item, extra mutation, cursor, invalid
argument, fabricated result, reordered required call or third Agent fails the run.

There are no retries. Each Agent uses `gpt-5.6-sol` at `high` reasoning, a read-only
sandbox, an isolated empty working directory, ignored user configuration and rules,
disabled web/native tool paths and disabled multi-Agent support. The deadline
defaults to 600 seconds per Agent and may be set from 30 through 1,800 seconds with
`AICADIA_AGENT_TIMEOUT_SECONDS`.

The Agent processes inherit normal Codex authentication, home and executable-path
state, but not database, PostgreSQL, runner-control, fake-test or playtest variables.
Only the per-Agent `AICADIA_USER_ID` is added for the MCP header. Agents therefore
cannot access or manage the disposable deployment directly.

## Evidence and recovery

The fake integration suite exercises the happy route, strict role configuration,
expected failure routes, timeout process-group cleanup, environment isolation,
schema rejection, ambiguous database creation and signal cleanup without spending
tokens. A passing fake suite proves runner behavior, not Agent comprehension; only
the explicitly authorized live command can supply that final evidence.

Each run directory remains under `.aicadia-playtest/`. Its manifest correlates the
safe run id, generated database, loopback address, model, reasoning effort,
deadline, User ids, Agent exits, validated Character, Place and Entity ids, run
status and cleanup result. Prompts, schema-constrained finals, JSONL events, stderr
and server logs remain private to that mode-`700` directory. Credentials are never
recorded.

Cleanup ownership is recorded before `CREATE DATABASE`. Every ordinary exit and
`INT`, `TERM` or `HUP` uses one idempotent cleanup route. If dropping fails, the
manifest retains a command template with the exact strictly validated disposable
database name. Never point this runner at a persistent game database; it always
creates and owns a separate deployment.
