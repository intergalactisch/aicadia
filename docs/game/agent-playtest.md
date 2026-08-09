# Agent playtest

Status: operator acceptance proof for the current Agent interface

This runner verifies one narrow fact: two different real Codex Agents can use
Aicadia MCP against the same disposable World, with Agent B observing the exact
Entity introduced by Agent A. It proves MCP wiring, request User context, shared
World state and cross-User visibility through one current list page. Pagination is
covered by the ordinary World and adapter tests, not by this live acceptance proof.

It does **not** prove that an Agent can decide whether arbitrary game content meets
the semantic Entity boundary. The runner supplies a technical fixture and exact
arguments. That fixture is valid only as disposable test data and is never written
to the persistent development World.

## Token-free preflight

Set `DATABASE_URL` to a PostgreSQL administration database whose role may create and
drop databases, then run:

```sh
export DATABASE_URL=postgres://localhost/postgres
tools/agent-playtest preflight
```

Preflight makes no domain mutation, starts no Aicadia server and never invokes
`codex exec`. It builds the server, provisioner and disposable-database helper,
checks the local schemas, probes PostgreSQL access and asks Codex's token-free
configuration parser to parse both exact MCP role allowlists. Its runner-owned
schema check also validates the strict structured-output contract, not only JSON
syntax: each schema is a closed root object, `properties` and `required` contain the
same exact fields, every field has an explicit string type, and `status` has its
role-specific exact value. It fails closed unless all of these Codex facts are true:

- CLI version is exactly `codex-cli 0.147.0`;
- the CLI is logged in;
- the model catalog contains exactly `gpt-5.6-sol` with `high` reasoning support;
- the exact pinned CLI release that owns the run flags is installed;
- the pinned exec help still contains every run flag used by the runner;
- `codex mcp get --json` parses Agent A as `create_entity` only and Agent B as
  `get_user`, `list_entity`, `get_entity` only;
- Codex parses Code Mode routing as enabled with the local host available and the
  `mcp__aicadia` namespace restricted to direct calls;
- every native tool-bearing feature disabled by the run is still a recognized,
  non-removed feature and parses as disabled.

The exact version pin makes feature drift visible before tokens or game data can be
spent. The parsed role configuration also suppresses Codex's unstable-feature
startup warning; JSONL validation remains fail-closed for every emitted error item.
Updating Codex requires updating and re-verifying the runner contract first.

## Confirmed run

Only this exact command crosses the token-spend gate:

```sh
tools/agent-playtest run --confirm-token-spend
```

After repeating preflight, the runner performs this sequence once:

1. Create a mode-`700` run directory and mode-`600` initial manifest before any
   database mutation. The manifest already owns the generated database name and
   contains the safe `DROP IF EXISTS` recovery template.
2. Create a generated `aicadia_playtest_*` PostgreSQL database and start a private
   Aicadia server on a generated loopback port.
3. Verify the exact seven HTTP operations and MCP tools, then provision two distinct
   Users through the operator-only `World::create_user` binary.
4. Start Agent A once with only `create_entity`; it must make exactly one call with
   the supplied fixture arguments.
5. Start Agent B once, even if A exits, times out or returns a bad artifact. B has
   only `get_user`, `list_entity` and `get_entity` and must observe the same Entity.
6. Validate both JSONL streams, final schemas, exact tool servers, names and
   arguments, stored content and introducer User. Agent B's one first-page
   `list_entity` call may omit `limit` or set only an integer `limit` from 1 through
   100; cursors and other arguments are rejected. Its completed MCP results must
   prove, in order, User B, a first-page summary for the exact stored Entity and
   `get_entity` returning that same complete Entity; its final must match those
   results and server state.
7. Stop the private server and drop the entire disposable database. This is
   test-infrastructure cleanup, not a game `delete` capability.

There are exactly two Agent attempts, no retry and no third Agent. Each attempt uses
`gpt-5.6-sol` at `high` reasoning, a read-only sandbox, an isolated empty working
directory, ignored user configuration and rules, disabled web/native tool paths and
disabled multi-Agent support. The local Code Mode host remains available only as
Codex execution plumbing. Code Mode routing marks the entire `mcp__aicadia`
namespace direct-only, while the role allowlists above are the only game calls each
Agent receives. The prompts also require direct MCP calls. Every error and every
program, program-output, Code Mode or other native-tool item fails the run; only
direct `mcp_tool_call` evidence counts. The deadline defaults to 600 seconds per
Agent and may be set from 30 through 1,800 seconds with
`AICADIA_AGENT_TIMEOUT_SECONDS`.

The Agent processes inherit normal Codex authentication, home and executable-path
state, but not `DATABASE_URL`, the generated database name, PostgreSQL administration
variables, runner-control variables or playtest variables. Only the per-Agent
`AICADIA_USER_ID` is added for the MCP header. The Agents therefore cannot access or
manage the disposable deployment directly.

## Evidence and recovery

The run directory remains under `.aicadia-playtest/`. Its manifest correlates the
safe run id, generated database name, loopback address, model, reasoning effort,
deadline, provisioned User ids, Agent exit statuses, validation result and cleanup
result. Prompts, schema-constrained finals, JSONL events, stderr and server logs are
private to that mode-`700` directory. The manifest and logs never record
`DATABASE_URL` or credentials.

The runner marks cleanup as required before it attempts `CREATE DATABASE`. A
non-zero or ambiguous create result therefore still attempts the strictly
name-validated `DROP DATABASE IF EXISTS`. `INT`, `TERM`, `HUP` and ordinary exit all
use one idempotent cleanup route that stops an active Agent and private server,
retains artifacts and updates the manifest. Provisioning is recorded as soon as each
User exists, so a setup or validation failure leaves evidence even when no Agent
starts. Cleanup is best effort. If dropping the disposable database fails, the
manifest keeps the command template using
`DATABASE_URL=<PostgreSQL administration URL>` and the exact safe database name. Do
not point this runner at a persistent game database; it always creates and owns a
separate deployment.
