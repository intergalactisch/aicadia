# Wayfinding tasks

> **Role / side:** probe fixture / development side.
> **Authority:** the six change tasks, the file each task's owner is, and the files a builder is expected to open.
> **Excludes:** runs, scores and verdict; see [`README.md`](README.md).

Each task is one sentence a User might say. "Owner" is the file the change is
edited in first (any listed owner counts); "expected reads" are the files on the
reading path in `game/docs/README.md#reading-paths-by-change` that a builder
should open for that task (recall is measured against them). Paths are
repository-relative.

| Id | Task | Owner (any) | Expected reads |
| --- | --- | --- | --- |
| W1 | Raise the maximum length of an Entity `name` from 120 to 160 characters. | `game/docs/domain.md` | `game/docs/README.md`, `game/docs/domain.md`, `game/docs/protocol.md` |
| W2 | Add an optional `pronouns` field to the `create_character` input. | `game/docs/capability/create_character.md` | `game/docs/README.md`, `game/docs/capability/create_character.md`, `game/docs/protocol.md`, `game/mcp/agent/tool/create_character.md` |
| W3 | Make `enter_world` also append an Activity when a successful entry is retried. | `game/docs/capability/enter_world.md` | `game/docs/README.md`, `game/docs/capability/enter_world.md`, `game/docs/model/activity/README.md` |
| W4 | Change how an Agent tells the player that `enter_world` failed. | `game/mcp/agent/tool/enter_world.md`, `game/mcp/agent/instruction/14-recovery.md` | `game/docs/agent.md`, `game/mcp/agent/tool/enter_world.md` |
| W5 | Add a new canonical error code `character_already_entered` with its HTTP status. | `game/docs/protocol.md` | `game/docs/README.md`, `game/docs/protocol.md`, `game/docs/adapter-parity.md` |
| W6 | Change which local PostgreSQL service the launcher prefers when two are reachable. | `game/docs/local-play.md`, `studio/tools/aicadia-local` | `game/docs/local-play.md`, `studio/tools/aicadia-local` |
