# Local play

> **Role / side:** Supported local launcher, isolated Agent adapter and read-only Studio operation contract / runtime side.
> **Authority:** Local development operation at the game-contract boundary.
> **Excludes:** Remote deployment, browser gameplay, authentication, Studio product rationale and delivery evidence; see `docs/concept/aicadia-studio.md` and `docs/evidence/local-play.md`.

Aicadia's supported local loop is one persistent World, one stable hidden
development User, one User-owned Agent conversation and one read-only **Aicadia
Studio**. The Agent is the only conversational game interface. Studio helps the
developer inspect accepted game sources and the local World but never supplies
player knowledge, proposes gameplay or mutates the World.

## Start

From the trusted repository root, with local PostgreSQL available, run:

```sh
cargo dev
```

The launcher uses database `aicadia_local`, starts one Rust process on loopback
port `3000`, provisions a User only on the first start, opens Aicadia Studio and
prints its URL, the MCP URL and the exact Agent command. `cargo dev` is the Cargo
alias for the owned `tools/aicadia-local` launcher. When PostgreSQL was installed
as a keg-only Homebrew formula, the launcher discovers its client commands without
requiring a shell `PATH` change. Supply another PostgreSQL administration
connection when the local default does not apply:

```sh
DATABASE_URL='postgres://localhost/postgres' cargo dev
```

Use `cargo dev --no-open` to leave the browser closed. The launcher stores only
the selected database name and stable User UUID in the ignored private
`.aicadia-local/profile.json`; it stores no credentials or conversation.

## Start the Agent conversation

In a second terminal, from the same trusted repository root, run the exact command
printed by the launcher:

```sh
AICADIA_USER_ID='<stable-uuid>' AICADIA_PORT='3000' ./tools/aicadia-agent
```

The adapter first verifies the local profile and server, then fetches the published
player contract from that running server through one stateless `server/discover`
call and fails closed when it cannot. It starts Codex with an empty workspace and
isolated home/configuration outside the development repository, copies only
available authentication into that private transient home, enables current MCP
`2026-07-28`, makes the local Aicadia connection required and injects exactly the
served player contract. Repository instructions, personal skills, extra MCP servers
and source code stay out of the game context. The entire owned temporary root,
including its authentication copy and conversation state, is removed when Codex
exits; the source authentication is never changed.

The launcher only prints this command. It never runs the adapter, Codex, an OpenAI
API or a model, so starting a token-spending Agent conversation remains an explicit
User action. The UUID is untrusted request context, not a login, account or
authorization token.

When no Character exists, the Agent follows the private workshop in
[Agent play contract](agent.md): exactly three candidates, selection and optional
steering, a complete natural preview in the User's language, explicit confirmation
and then one existing creation call. Gameplay continues in permanent player mode
through Aicadia MCP. Only accepted World changes become durable; proposals, drafts
and confirmation remain private.

## Studio boundary

Studio is served at `/` by the same loopback Rust process. Its top-level sections
are:

- **Game**, a read-only projection of allowlisted owning repository sources,
  including current game contracts, exact compiled MCP tools and their input
  schemas, Entity/Property/Trait model sections, exploration, experiments, planning,
  evidence and decision history. Rust reads and renders those sources; browser code
  contains no authored copy of their rules, field lists or catalog.
- **Live**, a read-only browser for the one actual local World. It shows bounded
  Entity, Character-role, Place-role and personal Activity pages, bounded current
  Property/Trait previews for one selected Entity, and the connected PostgreSQL
  application's current public-schema structure. It does not expose unrestricted
  files, arbitrary SQL or table rows.

Studio has one interface. `Game` and `Live` share one primary navigation, while
their source, model, tool, development-status, Entity, Activity and storage-table
views leave their current selection in query parameters. A copied development
reference combines that reload-safe URL with its owning source or durable record
context so it can be used in an AI conversation without copying the underlying
truth into browser assets.

Game HTTP and MCP remain independent of development-document meaning. Studio-only
`GET /studio/api/**` routes are absent from OpenAPI and MCP; they add no player
capability. Existing loopback `GET /api/entity` and
`GET /api/entity/{entity_id}` remain operator-only reads and are absent from MCP.
The Agent may never use Studio or those reads as fallback authority.

Every Studio list defaults to twenty-four records and accepts at most one hundred.
Entity detail previews at most fifty current Properties and fifty current Traits;
one Activity detail previews at most 256 explicitly involved Entities. Storage
inspection is limited to 256 ordinary or partitioned tables in `public` and at most
4,096 columns, non-foreign-key constraints, ordered foreign keys and indexes of each
kind across that schema. Crossing a bound fails the complete schema read instead of
silently presenting partial structure. The page contains no chat, form, proposal
control, confirmation, game mutation, model invocation, automatic Agent launch or
background polling. Initial load and explicit Refresh re-read current state.

The Storage view can download one pretty-printed JSON capture from
`GET /studio/api/live/storage/snapshot`. It includes capture time, latest successful
migration, tables, columns, constraints, ordered foreign keys, indexes and a SHA-256
fingerprint over the structural payload. Capture time does not affect the
fingerprint. The capture contains no World rows and Studio does not retain, compare
or write it automatically.

Before Character creation, personal Activity is honestly unavailable. The hidden
User UUID is not rendered. The launcher passes it in a URL fragment; the page copies
it to session storage and immediately removes the fragment from the visible URL
before using it only as the contextual Activity header.

The game currently has one persistent World with no durable World id or `world`
table. Studio labels one connected local World and does not present multiple Worlds
as delivered behavior.

## Stop, restart and reset

Press Ctrl-C in the launcher terminal to stop only the server process it started.
The database and `.aicadia-local/profile.json` remain. Run `cargo dev` again to
reuse the database and verify the same User before serving the same durable
Character, placement, Entities, Activity and prose. A concurrent launcher, corrupt
profile, missing profile beside an existing selected database, missing profiled User
or database mismatch fails visibly instead of silently provisioning a replacement.

The shipped launcher has no reset or database-drop path. Reset is deliberately an
external destructive operator action: it deletes accepted World history and must
remove both the explicitly selected local database and its matching local profile.
Disposable automated tests may clean up only databases they created and identified
as test-owned. Normal stop and restart never reset Aicadia.

Authentication, multiple profiles, User switching, browser gameplay, durable Agent
sessions and automatic token spend remain outside this local-play contract. A host
other than the bundled local adapter is conforming only when it satisfies the
provider-neutral host requirements in [Agent play contract](agent.md).
