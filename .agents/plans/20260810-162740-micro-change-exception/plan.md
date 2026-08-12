---
status: complete
created_at: 2026-08-10T16:27:40+02:00
updated_at: 2026-08-10T16:31:49+02:00
accepted_at: 2026-08-10T16:29:20+02:00
completed_at: 2026-08-10T16:31:49+02:00
---

# Permit self-evident micro-changes without a plan artifact

## Outcome

Aicadia Agents may execute an unambiguous, local, reversible micro-change after a
brief execution note, without creating or awaiting acceptance of a durable plan.
Anything with product, contract, domain, persistence, security, external-effect,
cost or evidence uncertainty still requires the full accepted planning workflow.

The evidence must prove that root instructions, `build-aicadia` and its visible
metadata express the same boundary without weakening the normal plan gate.

## Non-goals

- make formal planning optional for ordinary features or bug fixes;
- use line count alone to classify work as safe;
- change gameplay behavior, `docs/game/`, the backlog or the plan template;
- create a second planning tier or micro-plan file.

## Evidence and authorities

| Evidence or authority | Current fact | Consequence |
| --- | --- | --- |
| User clarification in this task | Very small changes and fixes should be exempt | Add a narrow conjunctive exception |
| `AGENTS.md` | Every build mutation currently requires a plan | Amend the always-on gate |
| `build-aicadia/SKILL.md` | Skill says every build needs a user-accepted plan | Add classification before planning |
| `build-aicadia/agents/openai.yaml` | Default prompt always asks for a plan | Reflect conditional planning |
| `docs/concept/log/log.md` | Recorded method currently has no exception | Record the corrected boundary |

## Alignment

### Strategic

Remove process friction where planning cannot improve the result, while preserving
deliberate alignment for changes that can affect Aicadia's game direction or durable
World. The next risk is agents stretching “small” to bypass consequential decisions;
the eligibility test therefore requires every condition, not intuition or line count.

### Tactical

A micro-change is eligible only when the requested outcome is unambiguous, the
change is local and readily reversible, it restores or preserves an already accepted
contract, it contains no product/domain/architecture choice, it touches no schema,
migration, ownership/history semantics, public contract, auth/security/privacy,
irreversible or external operation, material cost or token spend, and one focused
check can prove it. Before editing, state the intended surface and check. If discovery
breaks any condition, stop and create a formal plan before expanding scope.

### Technical

Change only the workflow rule, skill, generated metadata and concept log. Do not
change game code, schemas, tests, current game authority, backlog or the reusable
formal-plan template. Validate the skill package, terminology and complete diff.

## Decisions, assumptions and open questions

### Confirmed decisions

- The user accepted the micro-change exception in this task.
- Eligibility is conjunctive and risk-based; line count is only a signal.
- A brief execution note is conversational coordination, not a durable plan or a
  second acceptance gate.

### Reversible assumptions

- No new skill asset is needed; validate this by ensuring the formal template remains
  correct once the eligibility decision happens before it.

### Open questions

- None material. Borderline work deliberately falls back to the formal plan.

## Implementation map

| Surface | Intended change | Invariants |
| --- | --- | --- |
| `AGENTS.md` | Add the exact micro-change test and escalation rule | Compact, always-on, formal planning remains default |
| `.agents/skills/build-aicadia/SKILL.md` | Classify micro-changes before formal planning | Direct requests are not implicit formal-plan acceptance |
| `.agents/skills/build-aicadia/agents/openai.yaml` | Make the prompt conditional | `$build-aicadia` named; implicit invocation preserved |
| `docs/concept/log/log.md` | Record the corrected boundary | No duplicated implementation detail |

## Execution contract

Root executes this single task sequentially and preserves every unrelated worktree
change. Delegation would add ceremony and is not part of this plan.

## Task graph

| ID | State | Depends | Parallel-safe | Objective | Owned surfaces | Evidence |
| --- | --- | --- | --- | --- | --- | --- |
| T1 | completed | — | no | Install and validate the narrow micro-change exception | four implementation-map surfaces | skill validator, targeted text checks, `git diff --check`, focused diff review |

## Task details

### T1 — Install the exception

**Actions:** Amend the four mapped surfaces so classification is explicit, execute
eligible micro-changes after a brief note, and require immediate escalation to a
formal plan when any eligibility condition fails during discovery.

**Invariants:** Preserve Terry, plan acceptance for consequential work, the authority
hierarchy, implicit skill invocation, live-playtest changes and all game behavior.

**Evidence:** The skill validator passes; root and skill share the same exclusion
categories; metadata stays valid; `git diff --check` is clean; review finds no route
for schema, contract, security, irreversible, external or costly work to skip a plan.

**Stop conditions:** Return to `draft` if the exception needs subjective approval,
touches game behavior or requires changing the formal-plan schema.

## Validation ladder

1. Run the skill creator validator on `.agents/skills/build-aicadia`.
2. Check root, skill, metadata and concept log express the same conditional gate.
3. Run `git diff --check` and review the focused and complete worktree diff.

## Change control

Wording and validation refinements inside this eligibility boundary need no renewed
acceptance. Any broader exemption returns this plan to `draft` for re-acceptance.

## Completion evidence

- the skill creator validator reports `Skill is valid!`;
- targeted checks find no stale current rule requiring a plan before every build;
- root, skill and concept log exclude schema, migration, public contract,
  ownership/history, auth/security/privacy, irreversible/external operations,
  material cost and token spend from the micro-change path;
- metadata is valid, names `$build-aicadia`, retains implicit invocation and has a
  37-character short description;
- `git diff --check` passes and focused review confirms no game, backlog, formal
  template or earlier live-playtest change was disturbed.

## Completion conditions

- T1 and the validation ladder pass;
- consequential and borderline work still requires an accepted durable plan;
- eligible micro-changes require only a brief execution note and focused evidence;
- all authorities agree and no unrelated change is disturbed.
