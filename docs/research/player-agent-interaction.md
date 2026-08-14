> **Era:** July scene-claim research; its scene/claim vocabulary predates the 2026-08-07 game reframe.

# Player–agent interaction for an irreversible shared world

> **Role / side:** sourced research report / development side.
> **Authority:** records this report's question, sources, findings and implications.
> **Excludes:** product decisions and current implementation contracts; see `docs/game/`.

Date: 2026-07-25

Status: research and `5jaar` recommendation, not concept direction

## Question

When a person starts an Aicadia session through an MCP-connected agent, should the
agent act without further input, offer choices, or accept free text? How can the
interaction preserve personal agency without letting private steering or arbitrary
text enter shared canon unprocessed?

## The three pure models

### Agent autonomy

The person invokes a skill such as "next scene"; the agent reads the briefing and
writes/submits a scene without another exchange.

Strengths:

- Lowest effort and the strongest feeling that the agent is genuinely playing the
  character.
- The character can surprise their owner.
- Works in any text-capable MCP host.

Failure pressure:

- Invocation becomes an implicit approval of an irreversible public act.
- Weak or poorly briefed models can spend the daily scene on generic or invalid prose.
- Repeated one-click play weakens the owner's emotional authorship and makes the world
  feel like a feed of machine output.
- It leaves no natural moment to distinguish private character interpretation from
  public canon.

### Offered choices

The agent or interface presents several possible responses and the person selects
one.

Strengths:

- Removes blank-page anxiety and makes consequences legible.
- Can focus attention on real nearby tensions rather than arbitrary invention.
- Can work as buttons through MCP elicitation or an MCP App, with plain-text fallback.

Failure pressure:

- Fixed server-authored choices would make the server a hidden narrator and ship
  story branches or institutions.
- Agent-generated choices can converge on familiar dramatic clichés.
- Treating the list as exhaustive gives the generator more authorship than the
  person.

### Free text

The person writes anything and the agent uses it to create the next scene.

Strengths:

- Maximum expressive range and best use of the person's imagination, language and
  emotional intent.
- The agent can translate Dutch steering into English canon and structured claims.
- The possible user interaction does not need to be predicted by the server.

Failure pressure:

- Raw instructions may contain out-of-character direction, future plans, private
  motives or rejected alternatives that must not become world evidence.
- If free text is submitted directly, it bypasses the agent's role as constitution,
  context and claim interpreter.
- Open text cannot be made semantically safe by deterministic server validation
  alone.

## Adjacent systems

### AI Dungeon

AI Dungeon supports several free-text action framings (`Do`, `Say`, `Story`, `See`)
and a continuation path. It uses persistent prompt components and trigger-activated
Story Cards to keep relevant facts inside a finite model context.

Lesson: people value both "continue for me" and precise free steering. The system
also shows the cost of making context coherence an LLM memory problem. Its story can
be retried or edited for one adventure; Aicadia's shared irreversible archive cannot
rely on regeneration to repair inconsistency.

### Storium

Storium combines a structured scaffold—scene challenges and cards—with player-written
free prose. The scaffold gives a writer something concrete to respond to. A narrator
may request revision when a move overreaches or breaks the setting; moves become
harder to edit after another player builds on them.

Lesson: guidance and free expression are complements, not opposites. A revision
boundary before downstream dependence is valuable. Storium's narrator authority,
scores and prebuilt card systems do not fit Aicadia, but its "prompt, write, inspect,
then lock through use" shape is relevant.

### LambdaMOO and parser worlds

Classic text worlds accept apparently free text, but server-side parsers resolve it
to a known verb on a known object. Unrecognized prose does not mutate the world.

Lesson: free expression is safe only when there is a transformation boundary between
what the person types and what the world accepts. Aicadia puts that transformation in
the person's agent rather than in an ever-growing server verb library.

### MCP interaction primitives

MCP elicitation allows a server to request structured user input through a supporting
client, including explicit accept, decline and cancel outcomes. MCP Apps can render
interactive forms and multi-step review interfaces inside supporting hosts. Both are
capability-dependent; the core protocol does not force one universal UI.

Lesson: Aicadia may progressively enhance choices and confirmation with native UI,
but the essential loop must still work as ordinary agent conversation. Tool
annotations and skill instructions are hints, not proof that a human approved a
specific public scene.

## `5jaar`: Aicadia after five years

### What ordinary use looks like

No single interaction style won. Three durable player habits emerged:

1. **Delegators** enjoy seeing what their agent thinks their character would do. They
   often type nothing beyond starting the session, but they still inspect the
   proposed public scene.
2. **Directors** react to two or three live possibilities surfaced from the world.
   They choose one, combine them or add a sentence of intent.
3. **Writers** give detailed free-form steering and sometimes rewrite the proposed
   prose, while letting the agent ground it in canon and construct claims.

The successful product does not label or score these styles. People drift between
them depending on time, confidence and how much the scene matters.

### What failed during those five years

- **Immediate autoplay:** produced volume but little attachment. A bad model could
  irreversibly spend the scene before its owner saw what it intended.
- **Choice menus as the game:** made the world feel pre-authored and caused agents to
  herd toward the same visible hooks.
- **Raw free text as canon:** leaked planning language and private motives, and let
  unconstrained prose bypass the world's grounding step.
- **Approval after every tool call:** created confirmation fatigue. People approved
  mechanically without reading.
- **A rich UI as a requirement:** fragmented the experience across MCP hosts with
  different capabilities.

### The interaction that survived

The durable shape is a private workshop followed by one public commit:

1. **Start** — the person consciously invokes Aicadia.
2. **Orient** — the agent fetches the briefing and renders what changed, what is near
   and what matters to this character.
3. **Propose** — by default the agent says what it believes the character is drawn to
   do. It may show a small number of alternative doors, always with free steering
   available.
4. **Steer** — the person may approve the direction, select/combine an alternative,
   type freely or ask to be surprised. This conversation stays private.
5. **Draft** — the agent queries more world detail as needed and produces the exact
   public scene package: English prose, structured claims and provenance.
6. **Inspect** — the person sees a compact human-facing preview of what will enter the
   chronicle. Claim JSON need not dominate the experience.
7. **Commit** — one explicit act places the package in the world. The server validates
   mechanical rules and accepts the package atomically.

Everything before commit is reversible conversation. Everything after commit is
world history.

## Backcast to now

### Product recommendation

Use a hybrid interaction, with **agent proposal as the default**:

- A person may start with no creative text; the agent proposes rather than submits.
- Choices are temporary agent-side affordances derived from current world-state,
  never exhaustive server-authored branches.
- Free text is always accepted as private steering, never directly as canon.
- The agent remains the character's player and grounded writer; the human controls
  how much to delegate.
- Require one explicit confirmation for the complete public source package, not for
  every read or draft step.
- The baseline is ordinary conversation. MCP elicitation or an MCP App may later
  render the same proposal/review/commit loop more elegantly.

### Smallest experiment

Prototype the loop as a skill before building a custom interface:

`briefing → agent proposal → optional steering → package preview → explicit submit`

Test it with three instructions:

- only `next scene`;
- selection of one offered direction;
- a detailed free-form Dutch intention.

Judge whether all three create grounded scenes while leaving the user aware of the
single irreversible boundary. Do not yet build preference settings, an MCP App or
multiple interaction modes in the server.

## Still to decide

- Whether an explicit user confirmation is mandatory for every canon scene or merely
  the default skill behavior.
- Whether the agent presents one recommended direction or several equally weighted
  options.
- What the preview must reveal: full prose, a short consequence summary, affected
  entities, claims, or some combination.
- Whether any private steering or rejected draft is ever saved by Aicadia.
- How a non-compliant MCP client can prove that a human approved the final package.

## Sources

- [AI Dungeon 101](https://help.aidungeon.com/ai-dungeon-101/ai-dungeon-101)
- [AI Dungeon Plot Essentials](https://help.aidungeon.com/faq/plot-essentials)
- [AI Dungeon Story Cards](https://help.aidungeon.com/faq/story-cards)
- [Storium player guide](https://storium.com/help/how)
- [Storium making moves](https://storium.com/help/making-moves)
- [Storium managing play](https://storium.com/help/managing-play)
- [LambdaMOO programmer's manual](https://lambda.moo.mud.org/pub/MOO/ProgrammersManual.html)
- [MCP elicitation](https://modelcontextprotocol.io/specification/2025-06-18/client/elicitation)
- [MCP Apps overview](https://modelcontextprotocol.io/extensions/apps/overview)
- [Structure, Agency, and Improvisation in Human-Led Digital Interactive Narrative Exercises](https://ojs.aaai.org/index.php/AIIDE/article/view/36827)
