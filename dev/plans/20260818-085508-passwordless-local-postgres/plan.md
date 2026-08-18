---
status: complete
created_at: "2026-08-18T08:55:08+02:00"
updated_at: "2026-08-18T09:06:26+02:00"
accepted_at: "2026-08-18T08:57:47+02:00"
completed_at: "2026-08-18T09:06:26+02:00"
---

# Make `cargo dev` select local PostgreSQL without a password prompt

> **Role / side:** proportional build plan / development side.
> **Authority:** execution state for restoring the supported zero-configuration local launcher.
> **Excludes:** PostgreSQL installation, remote database discovery, credential management and World behavior.

## Outcome

A local builder runs plain `cargo dev`; the launcher finds the already-running
passwordless local PostgreSQL service, opens Aicadia Studio and prints the MCP and
Agent handoff without asking for a database password. An explicit `DATABASE_URL`
still wins. Every launcher-owned PostgreSQL client call is non-interactive and a
missing connection fails with one actionable error instead of opening a password
prompt.

On this workstation the same build restores the ignored `dev/.local/profile.json`
for the one existing `aicadia_local` User, so the preserved World is reused rather
than reset or reprovisioned. The exact final evidence claim is that plain
`cargo dev --no-open` selects the running Homebrew PostgreSQL service on its
configured port 5433, reuses User `1324e164-4872-4cd3-adc8-703cbb3d5f89`, serves
Studio and MCP on loopback port 3000, and never prompts for credentials.

## Non-goals

- Do not store, invent, request or manage a PostgreSQL password.
- Do not drop, recreate or mutate the existing `aicadia_local` database during
  profile recovery.
- Do not weaken the general fail-closed rule for an existing database without a
  matching profile; this is an explicit one-time recovery after the repository move.
- Do not change World, schema, HTTP, MCP, Agent, Studio-page or token-spending behavior.
- Do not scan arbitrary ports or discover remote databases.

## Evidence and authorities

| Evidence or authority | Current fact | Consequence for this build |
| --- | --- | --- |
| User direction, 2026-08-18 | Plain `cargo dev` must work without a password prompt. | Non-interactive local connection selection is the accepted intended outcome. |
| `game/docs/local-play.md` | Plain `cargo dev` is the supported entrypoint; an explicit `DATABASE_URL` is the override. | Preserve the command and override while correcting its default behavior. |
| `studio/tools/aicadia-local` | The hard-coded default is `postgres://localhost/postgres`, and `psql`/`createdb` may prompt. | Replace the hard-coded assumption with bounded local resolution and pass the clients' no-password flag. |
| Direct probes, 2026-08-18 | OrbStack owns port 5432 and rejects a no-password connection; the started Homebrew `postgresql@18` service is configured on 5433 and accepts the local User without a password. | Try the conventional endpoint non-interactively, then started installed Homebrew PostgreSQL services at their configured port; never port-scan. |
| Direct database readback, 2026-08-18 | `aicadia_local` exists on port 5433 with exactly User `1324e164-4872-4cd3-adc8-703cbb3d5f89`; `dev/.local/profile.json` is absent. | Restore only the matching ignored profile with mode `0600`; do not write the database. |
| `studio/tests/aicadia-local.sh` | The disposable lifecycle already proves first start, restart, profile safety, Studio, MCP and cleanup through an explicit URL on port 5433. | Extend focused launcher coverage for non-interactive resolution, then retain the full lifecycle evidence. |

## Alignment

### Strategic

This retires the concrete blocker between the developer and the only supported
local World/Studio loop. It restores the accepted local-play contract; it does not
select a new game-development capability. The next game risk remains unchanged.

### Tactical

The actor is the local builder. With no `DATABASE_URL`, the launcher probes only
the conventional local administration endpoint and configured started Homebrew
PostgreSQL services, always without prompting. It uses the first unambiguous
reachable candidate. With an explicit URL, it uses only that URL and fails
non-interactively if credentials are unavailable. Once connected, existing launch,
profile and lifecycle behavior is unchanged.

The one-time workstation repair recreates the profile only after independent
readback proves the selected database and its exact sole User. No game action occurs,
so no Activity footprint applies.

### Technical

Connection selection remains inside the Bash launcher; no new service, package or
configuration file is introduced. Homebrew discovery is bounded to installed
`postgresql`/`postgresql@*` formulae whose service reports running, and reads each
formula's configured `port` from its known local data directory through that
formula's `postgres` binary. `psql` and `createdb` receive their standard
`--no-password` option. Explicit `DATABASE_URL` keeps precedence and is never
persisted.

World, PostgreSQL schema, transactions, locks, identity and contention are not
changed. Millions of Users and a deliberately hot subject therefore behave exactly
as before; this launcher serves one local development process only.

## Decisions, assumptions and open questions

### Confirmed decisions

- Plain `cargo dev` must not ask for a PostgreSQL password — User direction.
- A supplied `DATABASE_URL` remains authoritative — current local-play contract.
- Existing World data is preserved and the known matching profile is recovered;
  reset or reprovisioning is rejected — current persistence contract and direct
  readback.

### Reversible assumptions

- Prefer the conventional local endpoint when it accepts a non-interactive
  connection; otherwise try configured started Homebrew PostgreSQL services. This
  preserves normal port-5432 setups while resolving this workstation's 5432/5433
  collision and can be changed without touching World data.
- If more than one fallback service is reachable, fail explicitly rather than make
  database ownership ambiguous; the current workstation has one started Homebrew
  PostgreSQL service.

### Open questions

None. The desired operation, precedence, safety boundary and exact local state were
confirmed and the accepted implementation completed without expanding them.

## Implementation map

| Surface | Current state | Intended change | Invariants |
| --- | --- | --- | --- |
| `studio/tools/aicadia-local` | Defaults blindly to localhost:5432 and permits client password prompts. | Resolve a bounded local no-password URL when none is supplied; make every client invocation non-interactive. | Explicit URL wins; no credentials persist; existing profile/database safety stays fail-closed. |
| `studio/tests/aicadia-local.sh` | Covers the complete lifecycle only with an explicit URL. | Add focused proof for resolution precedence, no-prompt flags and failure; retain the disposable real-PostgreSQL lifecycle. | Test-created databases/state are identified and cleaned; existing state is untouched. |
| `game/docs/local-play.md` | Describes a local default but not its bounded selection or no-prompt behavior. | State the exact default resolution, override and failure behavior. | Plain `cargo dev` remains the sole supported entrypoint. |
| `dev/docs/evidence/local-play.md` | Owns completed launcher evidence. | Append only the completed bounded proof after it passes. | Do not overclaim portability or credential support. |
| `dev/.local/profile.json` | Missing beside an existing preserved database with one known User. | Restore `{version, database_name, user_id}` with mode `0600`. | Ignored private state only; no database mutation or credential. |

## Execution contract

Root owns outcome, scope, plan state, integration and the final evidence claim. No
delegation is needed for this one-task repair. Any need to change database contents,
credential storage, schema or the missing-profile safety rule stops execution and
returns the plan to draft.

## Task graph

| ID | State | Depends | Parallel-safe | Objective | Owned surfaces | Evidence |
| --- | --- | --- | --- | --- | --- | --- |
| T1 | completed | — | no | Plain `cargo dev` starts the preserved local World and Studio without a password prompt. | launcher, launcher tests, local-play contract/evidence, ignored local profile | focused shell checks, disposable lifecycle and direct local smoke/readback |

## Task details

### T1 — Restore zero-configuration local startup

**Objective:** A plain local start selects the existing Homebrew PostgreSQL service,
reuses the existing User and serves Studio/MCP without credential interaction.

**Actions:**

1. Implement bounded, explicit-override-first, non-interactive local connection
   resolution in the launcher.
2. Add focused regression coverage and update the current local-play contract.
3. Restore the ignored profile only after revalidating the exact database and sole
   User; run the real local smoke, then record its bounded evidence.

**Invariants:**

- No World/database mutation during profile recovery; normal server migrations and
  the already-accepted launcher lifecycle remain unchanged.
- No password prompt, password persistence, remote discovery, port scan or silent
  fallback from an explicit URL.
- No game/API/MCP/Agent/Studio capability change.

**Evidence:**

- `bash -n studio/tools/aicadia-local studio/tests/aicadia-local.sh` — shell syntax.
- Focused launcher test — explicit precedence and automatic candidates are probed
  with `--no-password`; zero or multiple candidates fail visibly.
- `DATABASE_URL='postgres://localhost:5433/postgres' studio/tests/aicadia-local.sh`
  — complete disposable real-PostgreSQL lifecycle still passes and cleans its own state.
- Plain `cargo dev --no-open` plus loopback HTTP/MCP readback — exact real local
  outcome; stop only the process started for this smoke.
- Independent SQL/profile readback — the original database and sole User match the
  restored profile; no reset or second User occurred.

**Stop conditions:**

- Stop before mutation if the existing database has anything other than the exact
  independently observed sole User, if connection selection is ambiguous, or if
  implementation would need credentials, database repair or a contract expansion.

## Validation ladder

1. **Focused:** Bash syntax and deterministic resolution/no-prompt coverage.
2. **Contract:** existing disposable launcher/Agent lifecycle with explicit URL.
3. **Outcome:** plain no-environment local launch serves Studio and MCP through the
   Homebrew port and reuses the preserved User.
4. **Integrity:** `cargo fmt --all --check`, targeted Clippy if Rust changes,
   `git diff --check`, focused diff review and confirmation that unrelated work,
   World data and governing authorities remain intact.

## Change control

Refine paths, test mechanics and stronger evidence in place while this accepted
outcome and contract remain unchanged. Stop implementation, keep or return
`status: draft`, revise and request explicit re-acceptance when new evidence changes
connection precedence, database/profile safety, credential behavior, external
authority, cost or the evidence claim.

## Completion conditions

- T1 and the complete validation ladder pass;
- plain `cargo dev` no longer prompts and serves the preserved World and Studio;
- the original sole User is reused and no database or User reset occurred;
- current contract, decision trail and evidence agree;
- no known-stale authority, material open question or unrelated change remains;
- `status: complete` and `completed_at` are recorded only then.
