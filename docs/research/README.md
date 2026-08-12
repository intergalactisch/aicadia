# Research

Durable research supporting Aicadia concept development.

Research records the question, observations, sources and possible implications. It
does not establish current behavior by itself. Accepted current behavior and
implementation decisions are recorded in `docs/game/`; `docs/concept/` remains
exploration history.

| Research | Question |
|---|---|
| [Current MCP Agent guidance](current-mcp-agent-guidance.md) | How can every current MCP Agent receive the same provider-neutral play methods while World remains deterministic and strict? |
| [Codex agent graphs](ai-agent-graphs.md) | How could a graph-shaped Codex workflow improve Aicadia development without entering the game runtime? |
| [Agent tool interface](agent-tool-interface.md) | What is the smallest correct MCP tool boundary for the Rust World interface? |
| [Idempotent action delivery and Place freshness](idempotent-action-delivery-and-place-freshness.md) | How should an Agent safely retry one action while World rejects a package grounded in stale relevant Place state? |
| [Character identity and control](character-entity-control-model.md) | How can accounts control one character at a time while abandoned characters and NPCs retain one durable world identity? |
| [Persistent world-state](persistent-world-state.md) | How do other shared worlds remember, update and expose state? |
| [Player-agent interaction](player-agent-interaction.md) | How should a person steer an agent without breaking shared canon? |
| [World momentum and player reach](world-momentum.md) | What may move world-state without granting a player or the server unlimited authorship? |
| [World time and sparse simulation](world-time-and-sparse-simulation.md) | How can a potentially unbounded persistent world use time and change without simulating every region or turning time into a reroll? |
| [Stochastic discovery and bad-luck protection](stochastic-discovery-and-bad-luck-protection.md) | How can a Character-bound investigation yield zero, one or several shared discoveries without turning retries, drought protection or repeated attempts into exploitable progress? |
| [Spatial state](spatial-state.md) | How can travel, maps and discoveries between places remain deterministic and replayable? |
| [Mutable place geometry](mutable-place-geometry.md) | How can places grow, shrink, split, disappear and remain historically queryable? |
| [Hierarchical spatial placement](hierarchical-spatial-model.md) | When does a physical entity need exact geometry, and when is a containing place sufficient? |
| [Open spatial world systems](open-spatial-world-system.md) | How do open-source city builders, world engines and geographic systems represent and scale houses, districts and worlds? |
| [Persistent-game spatial models](persistent-game-spatial-model.md) | How should an immense game world separate Place identity, boundaries, engine partitions and current location? |
| [Locality, co-presence and observation](locality-co-presence-and-observation.md) | How do shared and agentic worlds distinguish direct place, nesting, proximity, observation and technical relevance? |
| [Stable identity and sparse location](stable-identity-and-sparse-location.md) | How can IDs, changing names and variable-depth places remain simple and scalable? |
| [Spatial occurrence and field](spatial-occurrence-and-field.md) | When do smoke, water and diffuse phenomena need an entity, location or derived field? |
| [Kind classification](kind-classification.md) | How should dynamic kinds support classification, multiple parents, changing taxonomy and separate origin lineages? |
| [Claim support and current state](claim-support-and-current-state.md) | Should independent attention, evidence and current-state eligibility share one claim status? |
