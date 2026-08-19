# Agent Play

> **Role / side:** current Agent Play development synthesis / development side.
> **Authority:** owns the current meaning, boundary, decisions, unresolved landscape, components and directional technical model for playing through a User-owned Agent.
> **Excludes:** selected work, exact Agent conduct and capability text, host-specific findings, experiments and delivery claims; those remain in `dev/backlog/` and plans, `game/docs/agent.md`, `dev/docs/research/`, `dev/lab/` and `dev/docs/evidence/`.

## Meaning

Agent Play is the way a User experiences Aicadia through an explicitly invoked AI
Agent connected over MCP. The Agent reads authoritative World facts, conducts a
grounded in-world conversation, uses its own intelligence to compose bounded
proposals and asks the User to confirm every complete World-changing package.

## Boundary

### This is

- User-owned intelligence operating through the same public capabilities as every other Agent.
- An in-world conversation grounded only in authoritative Aicadia MCP reads.
- Explicit preview and confirmation before each mutation.
- Provider-neutral Agent conduct with deterministic World validation.

### This is not

- A browser chat client, server narrator or server-owned Agent runtime.
- Background Agent invocation, durable World session or unconscious token spend.
- A fallback to source code, HTTP, database, logs or remembered state when MCP is unavailable.
- Exposure of identifiers, schema fields, protocol work or development status in player conversation.

## Decisions

### Chosen

- Aicadia MCP is the sole live-game authority available to a conforming player Agent.
- The User supplies the Agent and pays for its intelligence; World remains dumb and strict.
- Player conversation renders named people, places, things, events and affordances rather than internal types.
- Every mutation requires a complete preview and explicit User confirmation.
- Capabilities are provider- and model-neutral and have semantic parity across World, HTTP and MCP.
- If MCP authority is unavailable, the Agent fails closed before mutation.
- An Agent must be able to express open, nuanced and quantitative relationships
  between Entities without every possible phrase becoming a World-owned enum.
- When an arrangement may either be momentary or follow another Entity, the Agent's
  confirmed proposal explicitly chooses the intended behavior; free wording never
  makes that choice implicitly.
- An Agent acts through its Character and may in principle propose changes to
  ordinary World state first authored by any other Agent. It may author surprising
  multi-Entity consequences, but must name them explicitly rather than relying on
  World to understand the story.
- The Agent receives only Relations and endpoints its current Character can know.
  A hidden inventory Relation belonging to another Character is absent from its
  grounding, and guessing an identifier grants neither knowledge nor mutation access.
- The Agent may reason from its Character's remembered prior observation, clearly as
  memory rather than current truth. It may propose a grounded investigation of what
  is currently knowable, but cannot directly mutate a merely remembered hidden Entity.
- The Agent composes Relation meaning, optional exact Position, movement behavior,
  visibility and intended action explicitly; it never receives one generic
  Containment operation that silently decides those concerns.

### Rejected

- Browser gameplay or an Aicadia-hosted narrator in the current product boundary.
- Provider, model, client or tool allowlists as a substitute for semantic capability contracts.
- Server-side inference, localization or LLM use to interpret Agent-authored content.
- Quiet fallback to repository, HTTP, database, logs or memory during player interaction.
- Durable conversational session state inside World.
- Requiring a closed server vocabulary to contain every relationship an Agent may describe.
- Giving the original Agent permanent exclusive edit authority over ordinary World content.
- Supplying hidden Relations to an Agent and relying on its prompt to conceal them.

### Not yet chosen

- Hosted authentication and OAuth for remote Agent hosts.
- The minimum supported host capabilities for subscriptions, reconnect and confirmation UX.
- How long-running Agent work presents changing World context without stale proposals.
- Which context-compaction guidance belongs in the public Agent contract as capabilities grow.
- How shared or delegated User control could safely invoke one Character through several Agents.
- How one confirmed proposal presents exact current Position, open Relation meaning
  and optional persistent movement with another Entity without duplicating truth.
- Whether a Character that may know an Entity can always read its exact resolved
  Position, or Position has an independent visibility boundary.

## Research needed

- Exercise the public contract with diverse current MCP hosts without provider-specific branches.
- Measure Agent comprehension and token cost as capability and context surfaces grow.
- Test confirmation, stale-context recovery and reconnect behavior in real host conversations.
- Verify which transports can support optional multiplayer hints without becoming required truth.

## Components

| Component | Current meaning |
| --- | --- |
| User | The human who chooses and explicitly invokes an Agent and confirms mutation. |
| Agent | User-owned intelligence that reads, reasons, presents and proposes. |
| Character | The durable player subject through which the Agent acts in World. |
| MCP contract | The sole live-game capability and authority surface for Agent play. |
| Grounding | Fresh bounded World reads used before explanation or proposal. |
| Preview | The complete player-readable package shown before mutation. |
| Confirmation | Explicit User authorization for that exact package. |
| Authored relation | Open Agent-supplied meaning between exact World subjects; it grants no mechanic merely by its wording. |
| Scenarios | The Agent expression, grounding and privacy cases in the [spatial scenario catalogue](../place/scenarios.md). |

## Technical model

### Delivered

One compiled Agent contract and MCP capability catalogue expose the current World
reads and confirmed mutations through the same `World` semantics as HTTP. The
current local Studio is read-only development context, not player conversation.
Exact conduct and capabilities remain in [`game/docs/agent.md`](../../../game/docs/agent.md).

### Directional

The capability surface stays compact, semantic and provider-neutral. An Agent reads
fresh authoritative context, composes one bounded proposal, obtains explicit User
confirmation and submits it; World validates without retaining conversational state
or invoking the Agent later. Future relationship authorship may remain semantically
open while the proposal separately names any exact Position, dependency, authority
or persistent constraint that World can validate structurally.

### Absent

Hosted auth, OAuth, browser chat, server Agents, background invocation, durable
domain sessions, provider branches, automatic token spending and an open Relation
capability are absent from the current contract.

## Sources

- Prepared pressure — [Spatial scenario catalogue](../place/scenarios.md).
- Exact Agent conduct — [`game/docs/agent.md`](../../../game/docs/agent.md).
- Exact capabilities — [`game/docs/`](../../../game/docs/README.md) and the generated public catalogue it governs.
- Related synthesis — [Multiplayer](../multiplayer/README.md), [Discovery](../discovery/README.md) and [World Change](../world-change/README.md).
- Research, experiments and delivery — [`dev/docs/research/`](../../docs/research/README.md), [`dev/lab/`](../../lab/README.md) and [`dev/docs/evidence/`](../../docs/evidence/README.md).
