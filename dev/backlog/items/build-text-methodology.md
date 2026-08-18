# Build-facing text methodology and capability-contract pass

> **Role / side:** documentation-and-contract backlog item / development side.
> **Authority:** records the requested outcome, state pointer and completion evidence for the build-text edge.
> **Excludes:** the method itself, the capability contracts and detailed evidence; see `dev/docs/methodology/build-text.md`, `game/docs/capability/` and the plan.

## Outcome

Every document a building Agent reads states what it owns, what is deliberately not
there and where that lives, and every reference is a plain sentence naming the fact,
its relation and its owning path — under one written method that future
documents follow. The fifteen capability contracts are the first complete
application; `game/docs/README.md` states which files to read per kind of change.
No capability semantics change.

The [completed plan](../../plans/20260818-113100-build-text-methodology/plan.md)
owns the task graph, the removal inventory and the evidence claim; the method lives
in [`dev/docs/methodology/build-text.md`](../../docs/methodology/build-text.md).

## Value

Aicadia is built by models. A contract they can read correctly from any single file
— without following every link to find out whether they must — is edited right the
first time and never copied to keep a fact in view. The method keeps the next
capability's contract right from the start, as the public-text method does for the
published Agent texts.

## Completion evidence

The plan's validation result records: method written and bound once in the
constitution, `AGENTS.md` and the vocabulary; fifteen capability contracts in the
fixed shape with wayfinding headers (1,113 → 871 lines), removal inventory
confirmed by an independent parity review (no World-owned fact lost, nine wording
findings resolved); reading-path table in `game/docs/README.md`; tool descriptions
laid out as label blocks with only whitespace changed and one catalog regeneration;
all suites green; builder wayfinding probe `supported` (old 4/6 → new 6/6 owners
found) in [`dev/lab/agent-text/`](../../lab/agent-text/README.md).
