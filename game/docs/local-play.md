# Local play

> **Role / side:** Supported local launcher, isolated Agent adapter and read-only Studio operation contract / runtime side.
> **Authority:** Local development operation at the game-contract boundary.
> **Excludes:** Remote deployment, browser gameplay, authentication, Studio product rationale and delivery evidence; see `dev/docs/concept/aicadia-studio.md` and `dev/docs/evidence/local-play.md`.

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
alias for the owned `studio/tools/aicadia-local` launcher. Without `DATABASE_URL`,
the launcher checks the conventional loopback PostgreSQL endpoint and each installed,
started Homebrew `postgresql` service at its configured port. It selects exactly one
reachable local administration connection; none or multiple fail visibly. The
bounded selection never scans arbitrary ports, and every launcher-owned PostgreSQL
client call is non-interactive, so it never opens a password prompt.

When PostgreSQL was installed as a keg-only Homebrew formula, the launcher also
discovers its client commands without requiring a shell `PATH` change. Supply
another PostgreSQL administration connection when automatic local selection does
not apply:

```sh
DATABASE_URL='postgres://localhost/postgres' cargo dev
```

An explicit `DATABASE_URL` is authoritative and is never replaced by an automatic
fallback. It must connect non-interactively, using credentials supplied outside the
launcher when credentials are required. The launcher never stores credentials.

Use `cargo dev --no-open` to leave the browser closed. The launcher stores only
the selected database name and stable User UUID in the ignored private
`dev/.local/profile.json`; it stores no credentials or conversation.

## Start the Agent conversation

In a second terminal, from the same trusted repository root, run the exact command
printed by the launcher:

```sh
AICADIA_USER_ID='<stable-uuid>' AICADIA_PORT='3000' ./game/tools/aicadia-agent
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

- **Overview** projects the current edge, live plans, open-section count, latest
  decisions, documentation-lint findings, game-surface counts, experiment and
  evidence status, and a bounded connected-World summary. It is orientation, never
  an authority over those records.
- **Game** projects the runtime side: the current game contracts, one conventionally
  discovered page per model, the exact compiled MCP tools and input schemas, the
  assembled Agent surface, vocabulary, storage and explicit deferrals.
- **Development** projects the development side: current concept direction, the
  append-only decision register, source-owned open sections, research, plan task
  graphs, the backlog horizon, retained experiments, evidence and build rules.
- **Live** browses the one connected local World through bounded pages for Users,
  Characters, Places, Entities, Properties and Property keys, Traits,
  investigation attempts and Activity. It also exposes migration state,
  public-schema structure, planner row estimates, an exact-id resolver and bounded
  rows for each introspected application table.

Every resource has a server-rendered path URL such as `/game/model/entity`,
`/development/work`, `/live/entity/<id>` or `/live/storage/entity`. Reloading,
history and sharing therefore need no browser route state. A copied development
reference combines that path with its owning repository source or durable record
context so it can be used in an AI conversation without copying the underlying
truth into browser assets. Long catalogs, provenance, raw payloads and task detail
use progressive disclosure; the mobile disclosure contains primary navigation,
Jump and the local section tree.

Studio discovers governed repository roots and model folders by the conventions in
the documentation constitution. It joins those records to the compiled Agent
catalog and connected PostgreSQL schema; it owns no content or source allowlist.
The same Rust projection validates home mapping, role headers, front matter, links
and anchors, model-to-table ownership and capability coverage during `cargo test`.
Browser assets remain presentation only: they contain no authored rule, field,
relation, tool, status or route-state copy.

`/brief` returns a generated Markdown orientation from that projection. `cargo
brief` renders the same structure in the terminal and remains useful without a
database: repository orientation stays available while the Live summary is marked
unavailable. Both surfaces contain current fields and pointers only and are not an
authority.

Game HTTP and MCP remain independent of development-document meaning. Studio paths
are absent from OpenAPI and MCP and add no player capability. The former
operator-only `GET /api/entity` and `GET /api/entity/{entity_id}` game reads are
absent; the game API exposes only its current player-capability surface. The Agent
may never use Studio as fallback authority.

Every Studio World list defaults to twenty-four records and accepts at most one
hundred. Entity detail previews at most fifty current Properties and fifty current
Traits; one Activity detail previews at most one hundred rows in each related
collection.
Storage inspection is limited to 256 ordinary or partitioned tables in `public` and
at most 4,096 columns, non-foreign-key constraints, ordered foreign keys and indexes
of each kind across that schema. Crossing a schema bound fails the complete read
instead of silently presenting partial structure. Generic row pages are always
`SELECT`-only, validate and quote a live-introspected table name, bind cursor values,
use the table's primary-key order and return at most one hundred rows. A table
without a primary key fails closed and exposes no row page.

The global newest-first Activity view is explicitly labeled as a local-development
sort, not a game read; Place and Character chronicles use their scoped indexes.
Operation filtering narrows only the Activities already loaded in the browser; it
does not add an unindexed server query or claim an exact World-wide match.
Planner estimates are labeled as estimates rather than exact counts. The page
contains no chat, form, proposal control, confirmation, game mutation, model
invocation, automatic Agent launch or background polling. Initial load, navigation
and explicit Refresh perform fresh server reads.

The Storage view can explicitly download one pretty-printed JSON capture from
`/live/storage/snapshot.json`. It includes capture time, the latest successful
migration found inside the newest 100 migration-version rows, tables, columns,
constraints, ordered foreign keys, indexes and a SHA-256 fingerprint over the
structural payload. If that fixed window contains no success, the field is explicitly
`unknown/partial` instead of scanning older rows. Capture time does not affect the
fingerprint. The capture contains no World rows and Studio does not retain, compare
or write it automatically.

The hidden development User UUID is not injected into browser state or rendered as
launcher context. Operator-only User pages are reached through explicit Live
navigation like every other durable record. Before Character creation, Character-
scoped history is honestly unavailable.

The game currently has one persistent World with no durable World id or `world`
table. Studio labels one connected local World and does not present multiple Worlds
as delivered behavior.

## Stop, restart and reset

Press Ctrl-C in the launcher terminal to stop only the server process it started.
The database and `dev/.local/profile.json` remain. Run `cargo dev` again to
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
