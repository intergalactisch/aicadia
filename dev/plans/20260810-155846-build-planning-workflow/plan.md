---
status: complete
created_at: 2026-08-10T15:58:46+02:00
updated_at: 2026-08-10T16:23:34+02:00
accepted_at: 2026-08-10T16:17:45+02:00
completed_at: 2026-08-10T16:23:34+02:00
---

# Mandatory Aicadia build-plan workflow

## Outcome

Every Aicadia build starts from one user-reviewable, durable plan before code,
schema, executable behavior, authoritative documentation or operational behavior is
changed. The plan aligns game value, delivery scope and technical design; exposes
material uncertainty early; and decomposes accepted work into dependency-ordered
tasks that root and delegated Agents can execute until the stated evidence passes.

The workflow must remain proportional. A small build may have one compact task, but
it may not skip the reasoning or acceptance gate. Read-only explanation, status,
orientation and diagnosis are not builds and do not earn a plan artifact merely for
ceremony.

## Non-goals

- replace `game/docs/` as the executable game authority;
- replace the concept log, research records or backlog;
- add estimates, points, deadlines, named owners or project-management ceremony;
- require delegation or parallelism;
- turn a plan into a chronological work diary;
- select the next Aicadia gameplay edge as part of this workflow change.

## Evidence and constraints

- Root `AGENTS.md` already requires Game Progress First, KISS, complete HTTP/MCP
  parity, same-transaction Activity and immediate decision recording.
- `build-aicadia` currently selects one valuable edge and proceeds from user
  agreement directly to implementation; it has no durable planning gate.
- The backlog permits at most one active edge and remains forward planning context,
  never executable authority.
- The user explicitly requires technical, tactical and strategic alignment, plan
  iteration, optional grilling and a plan detailed enough for repeated Agent
  execution.
- Existing unrelated worktree changes from the completed live Agent playtest must be
  preserved.

## Alignment

### Strategic

The plan must name the concrete player or World outcome, explain why it is the
highest-value current edge and show how it fulfils or deliberately evolves the
current build contract. It must state the evidence claim and the next concrete game
risk. Strategy decides *why this build exists*; it may not be replaced by a list of
technical chores.

### Tactical

The plan must translate the selected outcome into the smallest complete slice:
actor, input, action, accepted state, ownership, history footprint, surfaces,
boundary cases and explicit non-goals. It must identify dependencies and ordering,
including which tasks are genuinely parallel-safe. Tactics decide *what complete
slice is being delivered now*.

### Technical

The plan must describe the concrete implementation seams that the slice touches:
domain and `World` behavior, PostgreSQL state and migrations, transactions and
concurrency, idempotency, errors, HTTP and MCP parity, tool descriptions, tests,
documentation and operations. It must map intended changes to files or surfaces and
name exact validation commands and expected evidence. Technology serves the chosen
slice and may not introduce speculative infrastructure.

## Proposed workflow contract

### 1. Create one proportional plan

Before implementation, create:

```text
dev/plans/<YYYYMMDD-HHMMSS>-<short-kebab-slug>/plan.md
```

Use local time and a stable descriptive slug. At most one plan may be `active` for
the current build edge. Completed and superseded plans remain as compact development
history. A plan is forward execution state: update what remains true and let Git
retain chronology.

Every plan contains only sections that drive execution, with these required
meanings:

1. outcome and explicit non-goals;
2. repository evidence and governing authorities;
3. strategic, tactical and technical alignment;
4. confirmed decisions, assumptions and open questions;
5. implementation map and affected seams;
6. dependency-ordered tasks with evidence and stop conditions;
7. validation ladder and completion conditions.

### 2. Resolve material uncertainty before acceptance

An open question is material when its answer can change the player outcome, domain
meaning, actor, action, state, ownership, nomenclature, public contract, irreversible
data shape, external side effect, token spend or evidence claim. Keep the plan
`draft` and grill the user on those questions before implementation. Record
confirmed choices immediately in the correct authority and keep unresolved branches
explicit in the plan.

Agents may make only reversible, local assumptions that cannot change the accepted
outcome or contract. Every such assumption names its validation or removal point.

### 3. Present and accept the plan

Present the plan to the user before implementation. The user may accept, reject or
iterate it. On explicit acceptance, set `status: active`, record `accepted_at`, and
start only tasks within the accepted outcome, boundaries and evidence claim.

No implementation task may start while the plan is `draft`. Creating or refining
the plan, focused research and grilling are planning work, not implementation.

### 4. Execute the task graph

Each task must be independently understandable and contain:

- a stable task id and state (`pending`, `in_progress`, `completed`, `blocked`);
- dependencies and any explicitly safe parallel peers;
- one observable objective;
- exact files, modules or operational surfaces it may change;
- concrete actions and invariants it must preserve;
- focused evidence that proves that task, not a broader claim;
- stop conditions that return control to root or the user.

Root owns scope, plan state, integration and the final evidence claim. A delegated
Agent receives the plan path and one eligible task id, re-reads the live repository,
changes only its assigned surface, runs its focused evidence and returns raw results.
Root integrates the result and updates the plan. Delegated Agents do not silently
expand scope or edit the shared plan unless that is their explicit task.

Execution repeats:

1. select the first dependency-ready task with material game value;
2. mark it `in_progress` before mutation;
3. implement its smallest complete change;
4. run the task's focused evidence;
5. integrate and mark it `completed`, or record one concrete blocker;
6. update affected authorities and plan truth;
7. continue until the completion gate passes.

Parallel tasks are allowed only when the plan marks them parallel-safe, their write
surfaces do not overlap and their results can be verified independently. Otherwise
execute sequentially. Delegation is optional and must reduce risk or latency rather
than add ceremony.

### 5. Control change without freezing learning

Revise an active plan freely when implementation reveals a smaller file map, better
task ordering or stronger evidence inside the accepted outcome and contract. Record
the current truth, not a diary entry.

Stop implementation, return the plan to `draft` and request re-acceptance when new
evidence changes the outcome, public behavior, domain meaning, non-goals, irreversible
storage, external authority, material cost or evidence claim. Use grilling before
re-acceptance when the new branch is unclear.

### 6. Complete on evidence

Set `status: complete` and `completed_at` only when:

- every required task is completed;
- the stated validation ladder passes;
- the exact strategic player or World outcome is demonstrated;
- current behavior, concept choices, vocabulary and backlog are aligned;
- no known-stale authority or unresolved material question remains;
- unrelated user changes remain intact.

Completion does not automatically select or begin another edge.

## Anti-ceremony rules

- Scale detail with risk; a one-file build can have one task.
- Write one full fact in its authority and link to it from the plan.
- Do not copy requirements between plan sections merely to fill a template.
- Do not add subtasks for shell commands, formatting or status narration.
- Do not use estimates, points, percentages or named owners.
- Do not keep a diary; replace stale plan state with current state.
- Do not call research, review or planning “progress” unless it retires a named risk
  or unlocks the selected game outcome.
- Do not delegate work that root can complete more clearly in one pass.

## Planned repository changes

1. Add one compact `Plan Before Build` rule to root `AGENTS.md`.
2. Update `build-aicadia/SKILL.md` with the mandatory planning gate, plan lifecycle,
   execution loop, re-planning threshold and completion gate.
3. Add `build-aicadia/assets/plan-template.md` as the reusable proportional plan
   skeleton; do not add a README or planning script.
4. Regenerate `build-aicadia/agents/openai.yaml` so its prompt reflects selection,
   planning and evidence-based execution.
5. Record the accepted development-method choice once in
   `dev/docs/concept/log/log.md`; do not change `game/docs/` or select a backlog item.
6. Validate the skill package, template, documentation diff and this live plan.

## Task graph

| ID | State | Depends | Parallel-safe | Objective | Owned surfaces | Evidence |
| --- | --- | --- | --- | --- | --- | --- |
| P0 | completed | — | no | User accepts or iterates this plan | this `plan.md` only | explicit user acceptance; `status: active` and `accepted_at` recorded |
| P1 | completed | P0 | with P2 | Install the compact always-on rule and decision trail | `AGENTS.md`, `dev/docs/concept/log/log.md` | rule and log agree; no game contract or backlog change |
| P2 | completed | P0 | with P1 | Implement the executable planning and task-loop method | `build-aicadia/SKILL.md`, `build-aicadia/assets/plan-template.md` | skill and template cover every workflow clause without duplication |
| P3 | completed | P2 | no | Align UI metadata with the revised skill | `build-aicadia/agents/openai.yaml` | generated metadata passes documented constraints and names `$build-aicadia` |
| P4 | completed | P1, P2, P3 | no | Validate and integrate the complete workflow | all changed workflow files and this plan | skill validator, template checks, `git diff --check`, focused diff review |
| P5 | completed | P4 | no | Close the plan on exact evidence | this `plan.md` only | all tasks complete, no material open question, `status: complete` |

## Task details

### P1 — Always-on rule and decision trail

Add a short root rule that requires a proportional accepted plan before build
mutation, permits grilling while draft, distinguishes material re-planning from
in-scope refinement and points to `dev/plans/`. Record the accepted method and
reason in the August 2026 concept log. Preserve the compact nature of `AGENTS.md`.

Stop if the rule would conflict with Terry, the backlog's single-active-edge model
or the documentation authority hierarchy.

### P2 — Skill and reusable template

Keep edge selection intact, then insert a hard plan gate between user agreement and
implementation. Make the template proportional but explicit enough that a fresh
Agent can execute one task without reconstructing scope. Define root/sub-Agent
responsibilities, task state, dependencies, safe parallelism, evidence, stop
conditions, material change control and completion.

Do not add scripts: timestamped directory creation and template adaptation are
simple, variable operations that do not justify another executable abstraction.

### P3 — Skill metadata

Regenerate only the existing interface fields. Keep `display_name` unless the
revised skill makes it misleading. Make `short_description` 25–64 characters and
make `default_prompt` one sentence that explicitly uses `$build-aicadia` and asks
for an aligned plan before execution. Preserve implicit invocation.

### P4 — Validation and integration

Run the skill creator's `quick_validate.py` on the skill directory. Check the
template contains the plan state, three alignment layers, task evidence and change
control. Verify Markdown and whitespace with `git diff --check`. Review the complete
diff for duplicated authority, ceremonial requirements, stale workflow language and
accidental changes to the live-playtest evidence already in the worktree.

No sub-Agent forward test is part of this plan: the current coordination policy does
not authorize spawning one, and this plan itself supplies a real first artifact for
structural validation.

## Validation ladder

1. **Structure:** skill validator passes; YAML metadata and template frontmatter are
   valid.
2. **Semantic consistency:** root rule, skill, template and concept log describe the
   same gate, lifecycle and authority hierarchy.
3. **Executability:** every template task requires objective, dependencies, owned
   surface, actions, evidence and stop conditions; this plan conforms to it.
4. **Proportionality:** a one-task plan remains valid; optional delegation and
   parallelism are never mandatory.
5. **Repository integrity:** `git diff --check` passes and pre-existing live-playtest
   documentation changes remain intact.

## Completion evidence

- the skill creator validator reports `Skill is valid!`;
- the template exposes draft/acceptance/completion state, all three alignment layers,
  dependency tasks, focused evidence, stop conditions and material change control;
- the generated short description is 39 characters, the default prompt names
  `$build-aicadia`, and implicit invocation remains enabled;
- `git diff --check` passes; the same check reports no whitespace errors for both
  new untracked Markdown files;
- focused diff review confirms the root rule, skill, template, metadata and concept
  log agree, and the earlier live two-Agent acceptance changes remain intact.

## Completion conditions

- The user has explicitly accepted the method encoded in this plan.
- Every task P1–P5 is completed with its stated evidence.
- Aicadia cannot enter implementation through `build-aicadia` without an accepted
  plan, and material scope drift forces re-planning.
- Plans are detailed enough for bounded Agent execution but remain proportional and
  subordinate to the project authorities.
- No gameplay behavior or next backlog edge has been selected by this method change.
