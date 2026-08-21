# Research

> **Role / side:** research navigation index / development side.
> **Authority:** identifies each research report's question and where to read it.
> **Excludes:** Each report's own status and era, which live in its front matter; product decisions and current implementation contracts, which belong in `game/docs/`.

Durable research supporting Aicadia concept development.

Research records the question, observations, sources and possible implications. It
does not establish current behavior by itself. Accepted current behavior and
implementation decisions are recorded in `game/docs/`; `dev/docs/concept/` remains
exploration history. Each report carries its own standing and era in its front
matter.

## August Activity-Property-Trait era

- [Entity and Place through a game-framework lens](entity-place-framework-boundary.md) — Can one broad game-framework Entity identity support objects, cities and named spatial subjects while typed roles and Relations keep Place, Position, Area, terrain, topology and other structural facts distinct?
- [Spatial granularity in exploration worlds](exploration-world-spatial-granularity.md) — How do Civilization-style tiles, continuous open worlds, room graphs and hybrids represent terrain and the space between meaningful locations, and does Aicadia need spatial ground beneath Place?
- [Place extent, spatial inclusion and connection traversal](place-area-connection-traversal.md) — Which separate spatial facts describe a Place's shape, inclusion or overlap, and the ordered landscape followed by a direct Connection, including mutable boundaries and impossible topology?
- [Spatial multiplayer foundation](spatial-multiplayer-foundation.md) — Which typed spatial, connectedness, relationship and authority truths let one sparse persistent World support open-ended exploration, creation and the complete multiplayer scenario catalogue without global contention or server inference?
- [Spatial foundation through a five-year backcast](spatial-five-year-foundation.md) — Which spatial separations and technical pressures survive five years of million-player World growth, and what is the smallest present foundation that preserves that future without prebuilding it?
- [Laravel-style polymorphic Character knowledge storage](polymorphic-character-knowledge-storage.md) — Can a Laravel-like polymorphic table reliably and scalably remember that one Character knows a Place or Connection, and which guarantees remain outside its indexes?
- [D&D rules and durable campaign loops](dnd-rules-and-campaign-loops.md) — Which D&D play loops and rules make campaign play durable, and which underlying heuristics transfer to Aicadia without importing levels, scores or an omnipotent GM?
- [D&D campaign meta-layers and storytelling](dnd-campaign-meta-and-storytelling.md) — Which D&D campaign practices sustain agency, continuity, tension and callbacks, and how can their functions remain separated across Aicadia's User, Agent, World and Activity?
- [Tabletop narrative and meta patterns beyond D&D](tabletop-narrative-meta-beyond-dnd.md) — Which first-party tabletop techniques for authority, stakes, failure, oracles, pressure and recollection fit Aicadia, and what should not transfer unchanged?
- [ChatGPT access over stateless MCP](chatgpt-local-and-hosted-mcp-access.md) — How can local and later hosted Aicadia remain reachable from ChatGPT and other Agents while using only stateless MCP `2026-07-28`?
- [Current MCP Agent guidance](current-mcp-agent-guidance.md) — How can every current MCP Agent receive the same provider-neutral play methods while World remains deterministic and strict?
- [Agent tool interface](agent-tool-interface.md) — What is the smallest correct MCP tool boundary for the Rust World interface?
- [Idempotent action delivery and Place freshness](idempotent-action-delivery-and-place-freshness.md) — How should an Agent safely retry one action while World rejects a package grounded in stale relevant Place state?
- [Massively concurrent dynamic World architecture](massively-concurrent-dynamic-world.md) — How can Aicadia remain correct, bounded and interesting when millions of Users discover one sparse shared World or crowd one Entity or Place?
- [Multiplayer concurrency and World observation](multiplayer-concurrency-and-world-observation.md) — Which proven multiplayer and shared-world techniques let millions of Characters act and observe one persistent World, including an extreme same-Place hotspot, without per-observer truth or automatic Agent runs?
- [Persistent multiplayer GX and concurrency patterns](persistent-multiplayer-gx-concurrency-patterns.md) — Which game-development and virtual-authority patterns can keep one canonical World fun when thousands of Agents act at once, and does an Entity-sized micro-instance solve or merely move the hotspot?
- [Multiplayer resolution from first principles](multiplayer-first-principles-resolution.md) — If every Aicadia action is multiplayer, can each mechanic resolve a short subject-local set of concurrent Agent intents so fairness and composition are game rules rather than database side effects?
- [Recursive Agent synthesis of one World state](recursive-agent-world-state-synthesis.md) — Can one universal recursive Agent operation reduce one or thousands of complete Agent-authored outcomes to one exact final state while World understands no semantics?
- [BYO-Agent coordination without server inference](byo-agent-coordination-without-server-inference.md) — Who can know that changes belong together when World has no semantics, no additional BYO Agent can be launched and thousands cannot deliberate all-to-all?
- [Three blank-slate Multiplayer mechanics](blank-slate-multiplayer-mechanics.md) — What universal mechanic emerges when three isolated sol/high designers see only S01–S14 plus the BYO-Agent and dumb-World premises?
- [MCP subscriptions and collective Agent intents](mcp-subscriptions-and-collective-agent-intents.md) — Can current MCP subscriptions notify already-active hosts about a durable local proposal board without becoming Agent invocation, replay, consensus or World truth?
- [Multi-Agent deliberation and consensus](multi-agent-deliberation-and-consensus.md) — Can bounded nearby Agents independently propose, critique and assemble one semantic World-change package without making conversational agreement authoritative?
- [Agent-authored bounded World intents](agent-authored-world-intents.md) — How can Agents name exact dependencies, consequences and multi-Place scope while World validates only structural authority, bounds, freshness and atomic settlement?
- [Realtime Agent subscription transports](realtime-agent-subscription-transports.md) — Which current transport and host capabilities can notify active Codex, ChatGPT, Claude Code and future MCP apps about stale World resources without making push a correctness or Agent-invocation requirement?
- [PostgreSQL change propagation and fan-out](postgres-change-propagation-and-fanout.md) — What is the smallest bounded path from an accepted PostgreSQL World change to many interested gateways, and when do `LISTEN`/`NOTIFY`, polling, an outbox, CDC or a broker earn their cost?
- [Entity and Place interest subscriptions at scale](entity-place-interest-subscriptions-at-scale.md) — Should Agents follow a global World board, a region or Place, exact Entity resources, private Character context or a hybrid, and how does each shape behave with millions of listeners?
- [Unified World change system](unified-world-change-system.md) — Can one bounded Agent-authored change package unify Entity creation and change, deterministic concurrency, Activity, resource invalidation, scoped World effects and optional collective assembly at massive scale?
- [Character identity and control](character-entity-control-model.md) — How can accounts control one character at a time while abandoned characters and NPCs retain one durable world identity?
- [World time and sparse simulation](world-time-and-sparse-simulation.md) — How can a potentially unbounded persistent world use time and change without simulating every region or turning time into a reroll?
- [Stochastic discovery and bad-luck protection](stochastic-discovery-and-bad-luck-protection.md) — How can a Character-bound investigation yield zero, one or several shared discoveries without turning retries, drought protection or repeated attempts into exploitable progress?
- [Persistent-game spatial models](persistent-game-spatial-model.md) — How should an immense game world separate Place identity, boundaries, engine partitions and current location?
- [Locality, co-presence and observation](locality-co-presence-and-observation.md) — How do shared and agentic worlds distinguish direct place, nesting, proximity, observation and technical relevance?

## July scene-claim era

- [Persistent world-state](persistent-world-state.md) — How do other shared worlds remember, update and expose state?
- [Player-agent interaction](player-agent-interaction.md) — How should a person steer an agent without breaking shared canon?
- [World momentum and player reach](world-momentum.md) — What may move world-state without granting a player or the server unlimited authorship?
- [Spatial state](spatial-state.md) — How can travel, maps and discoveries between places remain deterministic and replayable?
- [Mutable place geometry](mutable-place-geometry.md) — How can places grow, shrink, split, disappear and remain historically queryable?
- [Stable identity and sparse location](stable-identity-and-sparse-location.md) — How can IDs, changing names and variable-depth places remain simple and scalable?
- [Spatial occurrence and field](spatial-occurrence-and-field.md) — When do smoke, water and diffuse phenomena need an entity, location or derived field?
- [Kind classification](kind-classification.md) — How should dynamic kinds support classification, multiple parents, changing taxonomy and separate origin lineages?
- [Claim support and current state](claim-support-and-current-state.md) — Should independent attention, evidence and current-state eligibility share one claim status?

## Archive

Superseded reports keep their archive banners and their original citations.

- [Codex agent graphs](archive/ai-agent-graphs.md) — How could a graph-shaped Codex workflow improve Aicadia development without entering the game runtime?
- [Hierarchical spatial placement](archive/hierarchical-spatial-model.md) — When does a physical entity need exact geometry, and when is a containing place sufficient?
- [Open spatial world systems](archive/open-spatial-world-system.md) — How do open-source city builders, world engines and geographic systems represent and scale houses, districts and worlds?
