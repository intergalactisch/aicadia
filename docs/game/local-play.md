# Local play

Aicadia's supported local loop is one persistent World, one stable hidden
development User, one User-owned Agent conversation and one read-only browser
ledger. The Agent is the only conversational game interface. The ledger only
inspects accepted World, Entity and Activity/prose data.

## Start

From the trusted repository root, with local PostgreSQL available, run:

```sh
./tools/aicadia-local
```

The launcher uses database `aicadia_local`, starts the server on loopback port
`3000`, provisions a User only on the first start, opens the ledger and prints its
URL, the MCP URL and the exact Agent command. Supply another PostgreSQL
administration connection when the local default does not apply:

```sh
DATABASE_URL='postgres://localhost/postgres' ./tools/aicadia-local
```

Use `./tools/aicadia-local --no-open` to leave the browser closed. The launcher
stores only the selected database name and stable User UUID in the ignored private
`.aicadia-local/profile.json`; it stores no credentials or conversation.

## Start the Agent conversation

In a second terminal, from the same trusted repository root, run the exact command
printed by the launcher:

```sh
AICADIA_USER_ID='<stable-uuid>' AICADIA_PORT='3000' ./tools/aicadia-agent
```

The adapter first verifies the local profile and server. It then starts Codex with
an empty workspace and isolated home/configuration outside the development
repository, copies only available authentication into that private transient home,
enables current MCP `2026-07-28`, makes the local Aicadia connection required and
injects the exact published player contract. That keeps repository instructions,
personal skills, extra MCP servers and source code out of the game context and
prevents a failed Aicadia connection from silently becoming a coding task or
direct-API substitute. The entire owned temporary root, including its authentication
copy and conversation state, is removed when Codex exits; the source authentication
is never changed.

The launcher only prints this command. It never runs the adapter, Codex, an OpenAI
API or a model, so starting a token-spending Agent conversation remains an explicit
User action. The UUID is untrusted request context, not a login, account or
authorization token.

When no Character exists, the Agent follows the private workshop in
[Agent interface](agent-interface.md): exactly three candidates, selection and
optional steering, a complete natural preview in the User's language, explicit
confirmation and then one existing creation call. Gameplay continues in permanent
player mode through Aicadia MCP. Only accepted World changes become durable;
proposals, drafts and confirmation remain private.

## Ledger boundary

The page reads only `GET /api/world`, loopback operator
`GET /api/entity`/`GET /api/entity/{entity_id}` and contextual
`GET /api/activity`. The two global Entity reads exist for this trusted local ledger;
they are out-of-world inspection, not player knowledge and not accepted MCP tools.
The Agent may never use them as a fallback authority. The page can refresh,
page and expand records but contains no chat, form, proposal control, confirmation,
game mutation or model surface. Before Character creation, personal Activity is
honestly unavailable or empty; onboarding never moves into the browser. The hidden
User UUID is not rendered.

World connection, Entity and Activity/prose are direct regions on one page. Entity
and Activity pages are newest first and offer `Load older` only when the existing
typed cursor has another page; expanding an Entity reads its existing representation
but replaces `introduced_by_user_id` with an explicit hidden-User notice. Initial
load, explicit Refresh and returning focus re-read current state. There is no
background polling. The launcher passes the User UUID in a URL fragment; the page
copies it to session storage and immediately removes the fragment from the visible
URL before using the value only as the contextual Activity header.

## Stop, restart and reset

Press Ctrl-C in the launcher terminal to stop only the server process it started.
The database and `.aicadia-local/profile.json` remain. Run the same launcher command
again to reuse the database and verify the same User before serving the same durable
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
provider-neutral host requirements in [Agent interface](agent-interface.md).
