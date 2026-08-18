---
status: draft
created_at: "<YYYY-MM-DDTHH:MM:SS+HH:MM>"
updated_at: "<YYYY-MM-DDTHH:MM:SS+HH:MM>"
accepted_at: null
completed_at: null
---

# <Concrete build outcome>

> **Role / side:** proportional build-plan template / development side.
> **Authority:** defines the required structure for a consequential Aicadia build plan.
> **Excludes:** actual execution state, product contracts and global build rules; those belong to the created plan, `game/docs/` and `AGENTS.md`.

Replace every angle-bracket instruction. Keep required meanings, but scale detail
with risk: a small build may use one task and short sections. Record current forward
state, not a diary.

## Outcome

Name the concrete actor, action, accepted state and player or World value. State why
this is the highest-value current edge and the exact claim the final evidence must
prove.

## Non-goals

- <Explicitly excluded behavior or surface.>

## Evidence and authorities

| Evidence or authority | Current fact | Consequence for this build |
| --- | --- | --- |
| `<path, symbol, command result or user choice>` | <Observed fact> | <Constraint or decision> |

Name the governing `game/docs/` contract, relevant backlog edge and any concept or
research record. Do not duplicate their full content.

## Alignment

### Strategic

Explain why the outcome advances Aicadia as a compelling shared-world discovery and
settlement game, how it fulfils or deliberately evolves the current build contract,
and which concrete game risk follows it.

### Tactical

Define the smallest complete slice: actor, input, action, accepted state, ownership,
Activity footprint, public surfaces, boundary cases and explicit exclusions.

### Technical

Map the slice to its real seams: `World` behavior, PostgreSQL and migrations,
transactions and concurrency, idempotency, errors, HTTP and MCP parity, tool
descriptions, tests, documentation and operations. Mark a seam not applicable
instead of inventing work.

## Decisions, assumptions and open questions

### Confirmed decisions

- <Decision — material reason — authority where it is recorded.>

### Reversible assumptions

- <Assumption — why it cannot change the accepted contract — when/how it is checked.>

### Open questions

- <Question — why it is or is not material — who or what can resolve it.>

Keep `status: draft` while any material question can change the outcome, domain
meaning, actor, action, state, ownership, nomenclature, public contract, irreversible
data, external side effect, token spend, material cost or evidence claim. Grill
before acceptance when needed.

## Implementation map

| Surface | Current state | Intended change | Invariants |
| --- | --- | --- | --- |
| `<file, module, table, endpoint, tool, doc or operation>` | <What exists> | <Exact bounded change> | <What must remain true> |

## Execution contract

Root owns outcome, scope, plan state, integration and the final evidence claim. A
delegated Agent receives this plan path and one dependency-ready task id, re-reads
the live repository, changes only its owned surfaces, runs focused evidence and
returns raw results. Delegation is optional. Run tasks in parallel only when the
table marks them safe, write surfaces do not overlap and results verify independently.

## Task graph

Allowed states are `pending`, `in_progress`, `completed` and `blocked`.

| ID | State | Depends | Parallel-safe | Objective | Owned surfaces | Evidence |
| --- | --- | --- | --- | --- | --- | --- |
| T1 | pending | — | no | <One observable result> | `<exact paths or surfaces>` | <Focused proof> |

## Task details

### T1 — <Task name>

**Objective:** <Observable result.>

**Actions:**

1. <Concrete bounded action.>

**Invariants:**

- <Behavior, ownership, vocabulary or user work that must remain intact.>

**Evidence:**

- `<exact command or observation>` — <expected proof, scoped to this task>.

**Stop conditions:**

- Stop and return to root or the user when <specific boundary, ambiguity or failure>
  occurs.

Repeat this task-detail block only for tasks needed by the dependency graph. Do not
create tasks for shell commands, formatting, estimates or status narration.

## Validation ladder

1. **Focused:** <Fast evidence for each changed seam.>
2. **Contract:** <Cross-surface, persistence, parity or boundary evidence.>
3. **Outcome:** <Concrete demonstration of the promised player or World result.>
4. **Integrity:** `git diff --check`, focused diff review and confirmation that
   unrelated user changes and all governing authorities remain intact.

## Change control

Refine paths, task order and stronger evidence in place while the accepted outcome
and contract remain unchanged. Stop implementation, set `status: draft`, revise and
request explicit re-acceptance when new evidence changes the outcome, public
behavior, domain meaning, non-goals, irreversible state, external authority,
material cost or evidence claim.

## Completion conditions

- every required task is `completed` and the validation ladder passes;
- the exact strategic outcome and evidence claim are demonstrated;
- current behavior, concept choices, vocabulary and backlog are aligned;
- no known-stale authority, material open question or accidental unrelated change
  remains;
- `status: complete` and `completed_at` are recorded only after these conditions.
