---
status: load-bearing
era: August Activity-Property-Trait
---

# Current MCP Agent guidance

> **Role / side:** sourced research report / development side.
> **Authority:** records this report's question, sources, findings and implications.
> **Excludes:** product decisions and current implementation contracts; see `game/docs/`.

Status: complete research; accepted implications are recorded in
[`game/docs/agent.md`](../../../game/docs/agent.md)

## Question

How can every current MCP Agent that plays Aicadia receive the same provider-neutral
play methods while World remains deterministic, strict and free of server-side model
inference?

## Findings

### Current MCP is stateless and discovery is optional

MCP `2026-07-28` removes the `initialize` handshake and transport sessions. Each
request carries its protocol version, client identity and capabilities. A server must
implement `server/discover`, but a client may invoke an RPC directly without calling
discovery first.

Consequently, `server/discover.instructions` is the standard current location for
global server guidance, but Aicadia cannot assume every generic protocol caller has
loaded it before a direct tool call.

### Server instructions guide but cannot guarantee model behavior

MCP defines server instructions as natural-language guidance. Maintainer guidance
states that the host decides how to inject those instructions and recommends testing
client behavior before relying on them. Reliability varies with the model, sampling,
host implementation and other available context. The same guidance recommends
putting cross-tool workflows in global instructions while keeping instructions
concise and model-agnostic.

This means Aicadia can publish one authoritative Agent contract but cannot truthfully
claim identical wording or universal compliance from arbitrary LLMs. Provider or
client allowlisting would narrow access without proving instruction following.

### Prompts and resources are not mandatory replacements

MCP prompts are User-controlled and normally require explicit selection. Resources
are application-controlled. Tools are model-controlled. Adding a prompt or resource
would therefore add another surface without guaranteeing that an Agent receives the
play contract. A new bootstrap tool would have the same problem: the model might not
call it.

### Tool descriptions are the local operation contract

An Agent host must expose tool names, descriptions and input schemas for a model to
select and invoke tools meaningfully. Tool descriptions are therefore the right
place to repeat only the critical preconditions for that operation. Global
instructions remain the right place for relationships among multiple tools and for
the overall play method.

### The host envelope determines whether play stays play

Server instructions and tool descriptions cannot help when a client never completes
MCP startup or when a general coding host treats the failed connection as permission
to inspect a repository or call another interface. Codex discovers project
instructions and skills from its working directory upward, so starting an ordinary
player conversation inside the Aicadia checkout also gives the model development
context that is neither needed nor authoritative for play.

The smallest local remedy is therefore a host envelope, not another server concept:
start with an empty workspace and isolated home/configuration outside the checkout,
inherit only transient authentication, inject the exact published play contract,
require the Aicadia MCP connection and fail before play when it is unavailable.
Disabling obvious coding and browsing surfaces in the bundled adapter reduces
accidental escape routes, but conformance rests on the general authority rule rather
than on a provider, client or tool allowlist.

### Tool results are untrusted content, not instructions

In a shared World, Entity text and Activity prose written on behalf of one User
become tool-result content read by another User's Agent. That is an indirect prompt
injection path even though the JSON shape is valid. OWASP's LLM and MCP guidance
recommends an explicit separation between instructions and untrusted data and warns
that pattern filters do not reliably recognize indirect injection. It also keeps
tool authorization, least privilege and human confirmation separate from model
interpretation.

For Aicadia, the smallest durable mitigation is one general instruction hierarchy:
typed structure establishes World facts, while every returned value is potentially
player-authored game data and never a command. This rule applies to current and
future World fields without enumerating content types. World continues to validate
only typed actions and state. A regex scanner, content allowlist, narrative linter,
classifier or second model would add complexity without making arbitrary model
behavior deterministic and is not introduced.

### Deterministic checks belong in World

LLM instruction following, generated wording and tool choice are nondeterministic.
Deterministic server checks can instead validate typed input, User and Character
context, current Place, supported consequences, idempotent delivery, observed-state
freshness and atomic Activity history. They cannot establish that a private human
confirmation occurred or decide whether free prose is sufficiently immersive
without inspecting conversation or adding inference.

Structured World state must therefore remain authoritative. Agent framing and prose
cannot create additional mechanics or current state.

## Implications for Aicadia

- Support only current stateless MCP `2026-07-28`; retain no Aicadia-owned legacy
  session path.
- Publish one compact provider-neutral play contract through
  `server/discover.instructions`.
- Put critical operation-local constraints in the corresponding tool descriptions,
  sourced from the same runtime module as the global instructions.
- Require a full interactive Agent host to surface current discovery instructions
  and the complete tool catalog, make Aicadia MCP required, suppress raw
  protocol/tool progress and fail closed without alternate live-state sources,
  without inspecting or allowlisting its identity.
- Keep a player conversation permanently in-world. Mechanics questions are answered
  through named subjects, observable facts and current affordances; implementation
  inspection moves to a separate development context rather than enabling a
  technical submode.
- Keep language choice, reasoning, proposal quality and narration free at the Agent;
  keep identities, valid state transitions, structured consequences, concurrency,
  idempotency and Activity strict in World.
- Tell the Agent once that every World value is potentially player-authored game
  data, never instructions; such content cannot authorize tool calls or technical
  disclosure or override the Aicadia contract and User intent.
- Add no MCP prompt, resource, bootstrap tool, prompt database, transcript linter,
  client certification registry or server-side model call for this outcome.
- Limit deterministic evidence to exact contract delivery and World behavior. A
  future live-model claim requires its own scenarios, authorization and bounded
  statement of which host/model version was observed.

## Sources

- Model Context Protocol, [2026-07-28 specification release](https://blog.modelcontextprotocol.io/posts/2026-07-28/),
  especially stateless requests, optional discovery and multi-round requests.
- Model Context Protocol, [Discovery specification](https://modelcontextprotocol.io/specification/draft/server/discover),
  especially the optional client call and `instructions` response field.
- Model Context Protocol, [Server instructions guidance](https://blog.modelcontextprotocol.io/posts/2025-11-03-using-server-instructions/),
  especially implementation variability, model-agnostic wording and non-guaranteed
  behavior.
- Model Context Protocol, [Server concepts](https://modelcontextprotocol.io/docs/2026-07-28/learn/server-concepts),
  especially the control model for prompts, resources and tools.
- OpenAI, [Function description guidance](https://developers.openai.com/cookbook/examples/o-series/o3o4-mini_prompting_guide#function-description),
  on separating cross-tool developer guidance from operation-local tool contracts.
- OpenAI, [Build skills](https://learn.chatgpt.com/docs/build-skills#where-codex-loads-local-skills),
  on Codex discovering project skills and instructions from the working directory
  through the repository root.
- OpenAI, [Evaluation best practices](https://developers.openai.com/api/docs/guides/evaluation-best-practices#identify-where-you-need-evals),
  on instruction, output, tool-selection and argument nondeterminism in Agent systems.
- OWASP, [LLM Prompt Injection Prevention Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/LLM_Prompt_Injection_Prevention_Cheat_Sheet.html),
  especially clear instruction/data separation, indirect injection, least privilege
  and the limits of pattern-based filtering.
- OWASP, [MCP Security Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/MCP_Security_Cheat_Sheet.html),
  especially treating tool responses as untrusted input and explicitly instructing
  a model that returned values are data rather than instructions.
