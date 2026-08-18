# Public-facing text methodology and Agent-text rewrite

> **Role / side:** documentation-and-contract backlog item / development side.
> **Authority:** records the requested outcome, state pointer and completion evidence for the public-text edge.
> **Excludes:** the method itself, the published texts and detailed evidence; see `dev/docs/methodology/public-text.md`, `game/mcp/agent/` and the plan.

## Outcome

Every text Aicadia publishes verbatim to Agents — the play contract, the fifteen
tool descriptions and the schema descriptions — states each rule once, in plain
imperative English with the play loop first and schema-owned bounds out of the
prose, under one written method that future texts follow. No rule is added, dropped
or weakened.

The [completed plan](../../plans/20260818-093700-public-text-methodology/plan.md)
owns the task graph, the rule inventory and the evidence claim; the method lives in
[`dev/docs/methodology/public-text.md`](../../docs/methodology/public-text.md) and
the layering contract in [`game/docs/agent.md`](../../../game/docs/agent.md#instruction-layering).

## Value

The published texts are the player interface every conforming host feeds to its
model before the first player word. Fewer, clearer words per rule lower context cost
and raise comprehension for every present and future Agent, and the method keeps the
next capability's text right from the start.

## Completion evidence

The plan's validation result records: inventory parity confirmed by an independent
review (four low findings resolved), all suites green against the regenerated
catalog fixture, live discover and catalog byte-equality, before/after sizes
(contract 2,833 → 2,387 words; descriptions 2,245 → 1,756; schema descriptions
24.2k → 10.0k chars) and the `supported` comprehension-probe verdict in
[`dev/lab/agent-text/`](../../lab/agent-text/README.md).
