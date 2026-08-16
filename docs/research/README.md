# Research

> **Role / side:** research standing index / development side.
> **Authority:** indexes each research report's status, era and question.
> **Excludes:** product decisions and current implementation contracts; see `docs/game/`.

Durable research supporting Aicadia concept development.

Research records the question, observations, sources and possible implications. It
does not establish current behavior by itself. Accepted current behavior and
implementation decisions are recorded in `docs/game/`; `docs/concept/` remains
exploration history.

| Research | Status | Era | Question |
|---|---|---|---|
| [D&D rules and durable campaign loops](dnd-rules-and-campaign-loops.md) | pending | August Activity-Property-Trait | Which D&D play loops and rules make campaign play durable, and which underlying heuristics transfer to Aicadia without importing levels, scores or an omnipotent GM? |
| [D&D campaign meta-layers and storytelling](dnd-campaign-meta-and-storytelling.md) | pending | August Activity-Property-Trait | Which D&D campaign practices sustain agency, continuity, tension and callbacks, and how can their functions remain separated across Aicadia's User, Agent, World and Activity? |
| [Tabletop narrative and meta patterns beyond D&D](tabletop-narrative-meta-beyond-dnd.md) | pending | August Activity-Property-Trait | Which first-party tabletop techniques for authority, stakes, failure, oracles, pressure and recollection fit Aicadia, and what should not transfer unchanged? |
| [ChatGPT access over stateless MCP](chatgpt-local-and-hosted-mcp-access.md) | load-bearing | August Activity-Property-Trait | How can local and later hosted Aicadia remain reachable from ChatGPT and other Agents while using only stateless MCP `2026-07-28`? |
| [Current MCP Agent guidance](current-mcp-agent-guidance.md) | load-bearing | August Activity-Property-Trait | How can every current MCP Agent receive the same provider-neutral play methods while World remains deterministic and strict? |
| [Codex agent graphs](archive/ai-agent-graphs.md) | superseded | August Activity-Property-Trait | How could a graph-shaped Codex workflow improve Aicadia development without entering the game runtime? |
| [Agent tool interface](agent-tool-interface.md) | load-bearing | August Activity-Property-Trait | What is the smallest correct MCP tool boundary for the Rust World interface? |
| [Idempotent action delivery and Place freshness](idempotent-action-delivery-and-place-freshness.md) | load-bearing | August Activity-Property-Trait | How should an Agent safely retry one action while World rejects a package grounded in stale relevant Place state? |
| [Massively concurrent dynamic World architecture](massively-concurrent-dynamic-world.md) | pending | August Activity-Property-Trait | How can Aicadia remain correct, bounded and interesting when millions of Users discover one sparse shared World or crowd one Entity or Place? |
| [Multiplayer concurrency and World observation](multiplayer-concurrency-and-world-observation.md) | pending | August Activity-Property-Trait | Which proven multiplayer and shared-world techniques let millions of Characters act and observe one persistent World, including an extreme same-Place hotspot, without per-observer truth or automatic Agent runs? |
| [MCP subscriptions and collective Agent intents](mcp-subscriptions-and-collective-agent-intents.md) | pending | August Activity-Property-Trait | Can current MCP subscriptions notify already-active hosts about a durable local proposal board without becoming Agent invocation, replay, consensus or World truth? |
| [Multi-Agent deliberation and consensus](multi-agent-deliberation-and-consensus.md) | pending | August Activity-Property-Trait | Can bounded nearby Agents independently propose, critique and assemble one semantic World-change package without making conversational agreement authoritative? |
| [Agent-authored bounded World intents](agent-authored-world-intents.md) | pending | August Activity-Property-Trait | How can Agents name exact dependencies, consequences and multi-Place scope while World validates only structural authority, bounds, freshness and atomic settlement? |
| [Realtime Agent subscription transports](realtime-agent-subscription-transports.md) | pending | August Activity-Property-Trait | Which current transport and host capabilities can notify active Codex, ChatGPT, Claude Code and future MCP apps about stale World resources without making push a correctness or Agent-invocation requirement? |
| [PostgreSQL change propagation and fan-out](postgres-change-propagation-and-fanout.md) | pending | August Activity-Property-Trait | What is the smallest bounded path from an accepted PostgreSQL World change to many interested gateways, and when do `LISTEN`/`NOTIFY`, polling, an outbox, CDC or a broker earn their cost? |
| [Entity and Place interest subscriptions at scale](entity-place-interest-subscriptions-at-scale.md) | pending | August Activity-Property-Trait | Should Agents follow a global World board, a region or Place, exact Entity resources, private Character context or a hybrid, and how does each shape behave with millions of listeners? |
| [Character identity and control](character-entity-control-model.md) | load-bearing | August Activity-Property-Trait | How can accounts control one character at a time while abandoned characters and NPCs retain one durable world identity? |
| [Persistent world-state](persistent-world-state.md) | historical | July scene-claim | How do other shared worlds remember, update and expose state? |
| [Player-agent interaction](player-agent-interaction.md) | historical | July scene-claim | How should a person steer an agent without breaking shared canon? |
| [World momentum and player reach](world-momentum.md) | historical | July scene-claim | What may move world-state without granting a player or the server unlimited authorship? |
| [World time and sparse simulation](world-time-and-sparse-simulation.md) | pending | August Activity-Property-Trait | How can a potentially unbounded persistent world use time and change without simulating every region or turning time into a reroll? |
| [Stochastic discovery and bad-luck protection](stochastic-discovery-and-bad-luck-protection.md) | pending | August Activity-Property-Trait | How can a Character-bound investigation yield zero, one or several shared discoveries without turning retries, drought protection or repeated attempts into exploitable progress? |
| [Spatial state](spatial-state.md) | historical | July scene-claim | How can travel, maps and discoveries between places remain deterministic and replayable? |
| [Mutable place geometry](mutable-place-geometry.md) | historical | July scene-claim | How can places grow, shrink, split, disappear and remain historically queryable? |
| [Hierarchical spatial placement](archive/hierarchical-spatial-model.md) | superseded | July scene-claim | When does a physical entity need exact geometry, and when is a containing place sufficient? |
| [Open spatial world systems](archive/open-spatial-world-system.md) | superseded | July scene-claim | How do open-source city builders, world engines and geographic systems represent and scale houses, districts and worlds? |
| [Persistent-game spatial models](persistent-game-spatial-model.md) | pending | August Activity-Property-Trait | How should an immense game world separate Place identity, boundaries, engine partitions and current location? |
| [Locality, co-presence and observation](locality-co-presence-and-observation.md) | pending | August Activity-Property-Trait | How do shared and agentic worlds distinguish direct place, nesting, proximity, observation and technical relevance? |
| [Stable identity and sparse location](stable-identity-and-sparse-location.md) | load-bearing | July scene-claim | How can IDs, changing names and variable-depth places remain simple and scalable? |
| [Spatial occurrence and field](spatial-occurrence-and-field.md) | historical | July scene-claim | When do smoke, water and diffuse phenomena need an entity, location or derived field? |
| [Kind classification](kind-classification.md) | historical | July scene-claim | How should dynamic kinds support classification, multiple parents, changing taxonomy and separate origin lineages? |
| [Claim support and current state](claim-support-and-current-state.md) | historical | July scene-claim | Should independent attention, evidence and current-state eligibility share one claim status? |
