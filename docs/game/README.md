# Current game contract

> **Role / side:** Navigation index and reading order for the current build contract / runtime side.
> **Authority:** Points to each current game-contract home; it does not restate their contracts.
> **Excludes:** Delivery state, rollout narrative, evidence results and exploration history.

Aicadia has one current game contract. Read it in this order:

1. [Domain contract](domain.md) — subjects, current state, value invariants and Activity.
2. [Capability catalog](#capability-catalog) and the linked per-capability contracts.
3. [Protocol contract](protocol.md) — request context, wire shapes, freshness, HTTP/MCP and errors.
4. [Agent play contract](agent.md) — host conduct, player communication and private workshops.
5. [Storage contract](storage.md) — PostgreSQL relations, migrations, locks and indexes.
6. [Deferred game scope](deferred.md) — behavior and models intentionally absent.
7. [Local play](local-play.md) — supported local launcher, Agent adapter and read-only ledger.

The current contract is authoritative over exploration history.
Delivery history and current status: see [Trait evidence](../evidence/trait.md).

## Capability catalog

Catalog order is deterministic:

| Capability | World call | HTTP | MCP | User context |
| --- | --- | --- | --- | --- |
| `get_world` | `get_world()` | `GET /api/world` | `get_world` | absent |
| `get_user` | `get_user(context.user_id)` | `GET /api/user` | `get_user` | required |
| `get_character` | `get_character(context.user_id, input)` | `GET /api/character?cursor&limit` | `get_character` | required |
| `create_character` | `create_character(context.user_id, input)` | `POST /api/character` | `create_character` | required |
| `create_entry_place` | `create_entry_place(context.user_id, input)` | `POST /api/place/entry` | `create_entry_place` | required |
| `enter_world` | `enter_world(context.user_id)` | `POST /api/world/entry` | `enter_world` | required |
| `list_activity` | `list_activity(context.user_id, input)` | `GET /api/activity` | `list_activity` | required |
| `create_entity` | `create_entity(context.user_id, input)` | `POST /api/entity` | `create_entity` | required |
| `list_entity_at_current_place` | `list_entity_at_current_place(context.user_id, input)` | `GET /api/place/current/entity` | `list_entity_at_current_place` | required |
| `list_activity_at_current_place` | `list_activity_at_current_place(context.user_id, input)` | `GET /api/place/current/activity` | `list_activity_at_current_place` | required |
| `get_entity_at_current_place` | `get_entity_at_current_place(context.user_id, input)` | `GET /api/place/current/entity/{entity_id}?cursor&limit` | `get_entity_at_current_place` | required |
| `submit_action` | `submit_action(context.user_id, input)` | `POST /api/action` | `submit_action` | required |
| `submit_interaction` | `submit_interaction(context.user_id, input)` | `POST /api/interaction` | `submit_interaction` | required |

`create_user` is deliberately absent. Database creation, migration, diagnostics,
administration and every other operational action are not Agent capabilities.
Loopback `GET /api/entity` and `GET /api/entity/{entity_id}` remain available only
to the supported local operator ledger during this build and are deliberately absent
from MCP and this catalog.

## Capability contracts

- [`get_world`](capability/get_world.md)
- [`get_user`](capability/get_user.md)
- [`get_character`](capability/get_character.md)
- [`create_character`](capability/create_character.md)
- [`create_entry_place`](capability/create_entry_place.md)
- [`enter_world`](capability/enter_world.md)
- [`list_activity`](capability/list_activity.md)
- [`create_entity`](capability/create_entity.md)
- [`list_entity_at_current_place`](capability/list_entity_at_current_place.md)
- [`list_activity_at_current_place`](capability/list_activity_at_current_place.md)
- [`get_entity_at_current_place`](capability/get_entity_at_current_place.md)
- [`submit_action`](capability/submit_action.md)
- [`submit_interaction`](capability/submit_interaction.md)
