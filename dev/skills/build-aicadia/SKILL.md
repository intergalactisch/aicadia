---
name: build-aicadia
description: Guide Aicadia development by selecting, classifying, planning, executing and reviewing the highest-leverage concrete game-development edge. Use for open next-work questions, substantial Aicadia research or decisions, implementation and review. Execute only unambiguous local reversible micro-changes directly; require a user-accepted strategic, tactical and technical plan for consequential builds, and iterate or grill when material choices remain unclear.
---

# Build Aicadia

> **Role / side:** reusable Aicadia build workflow / development side.
> **Authority:** governs this skill's routing, planning, execution and recording procedure.
> **Excludes:** project contracts and global build rules; see `game/docs/` and `AGENTS.md`.

Keep the build moving without turning the process into a project of its own. Apply Terry and preserve unrelated user changes.

## Find the current edge

- Use the root `AGENTS.md` instructions already in context; read the file only if they are missing, and read nested instructions only when their scope applies.
- Run `cargo brief` as the first source-backed orientation when it is available; follow its pointers to the owning records rather than treating the generated brief as authority.
- Start with `git status --short`, `rg --files`, and targeted `rg` searches for headings, status lines, contract terms, and relevant symbols.
- Read only narrow ranges from the leading `game/docs/`, code, and test candidates. Never load the full document tree or a long file merely to orient.
- When `dev/backlog/README.md` exists, read its ordered horizon and the one linked current item. Treat it as forward planning context, never as authority over `game/docs/`.
- Open `dev/docs/research/` or `dev/docs/concept/` only when the question depends on their findings or history.
- Identify the highest-leverage concrete player or World outcome using the current contract as evidence, then find its smallest safe slice. KISS sizes the selected work; it does not decide which outcome matters most.
- The contract bounds implementation, not ambition. If the valuable next outcome is absent from `game/docs/`, propose the smallest concrete behavior decision that can evolve the contract before any code is written.
- Do not choose validation, cleanup, plumbing or documentation merely because it is bounded. It may lead only when it unlocks the selected game capability or retires a concrete blocker or risk on that edge.
- Stop gathering once one valuable gap has enough repository evidence. Run broader checks only when they could change the choice.

## Suggest exactly one step for open questions

- **Next** — State the concrete action on the selected game-development edge.
- **Why** — Point to the repository evidence and name the concrete player, World or game value it advances.
- **Done** — Give observable completion and verification conditions.

Do not write files or delegate work yet. Wait for the user to accept, reject, or refine the step.

## Execute only a qualifying micro-change directly

- Classify a specific change before formal planning. It qualifies only when every condition holds: the requested outcome is unambiguous; the edit is local and readily reversible; it only restores or preserves an accepted contract; it introduces no product, domain or architecture choice; it touches no schema, migration, public contract, ownership or history semantics, auth, security, privacy, irreversible or external operation, material cost or token spend; and one focused check can prove it.
- Never use line count alone as evidence. When unsure, treat the work as consequential.
- Before editing, briefly state the intended outcome, owned surface and focused check. This execution note is not a durable plan or separate acceptance gate.
- Make only the qualified edit and run its focused check. If discovery breaks any condition or requires broader work, stop before expanding scope and follow the formal planning workflow.
- Do not manufacture decision documentation for a micro-change. If a material choice crystallizes, the work no longer qualifies: stop and plan it.

## Plan the selected build

- After ruling out a micro-change, treat a specific build request as agreement on the intended outcome, never as acceptance of an unwritten plan. Read-only explanation, status, orientation, research and diagnosis do not need a plan unless they turn into a build mutation.
- Look for an existing `active` plan before creating one. Keep at most one active plan for the current edge; resume it when the request remains inside its accepted outcome.
- Before making a consequential change to code, schema, executable behavior, authoritative documentation or operations, read `assets/plan-template.md` completely and create `dev/plans/<YYYYMMDD-HHMMSS>-<short-kebab-slug>/plan.md` from it. Use local time. Keep the plan proportional: one task is valid for a small planned build.
- Ground the plan in current repository evidence. Align the strategic player or World outcome, the tactical smallest complete slice and the technical implementation seams. State the exact evidence claim, non-goals, affected authorities and next concrete risk.
- For every experiment or smoke, name which seams are real and which are simulated. Keep deterministic authorization, privacy and state-correctness evidence separate from direct protocol, model-comprehension and presentation evidence; each claim is limited to the exact implementation and fixture exercised, so a simulated World never proves production World behavior and one layer never proves an unexercised property of the other.
- When a project designates an implementation language for technical labs, use it for state, concurrency, ordering and integration experiments. Reserve a visual prototype medium for questions whose evidence is human interaction; carry productionward the proven invariants, scenarios and measurements, never copied experimental code.
- Make every task independently executable: give it a stable id, dependencies, state, objective, owned surfaces, concrete actions, invariants, focused evidence and stop conditions. Mark parallel work safe only when write surfaces do not overlap and evidence is independent.
- Present every material Aicadia option in two layers: an understandable player/World outcome first, then its exact technical state, ownership, transaction, contention and cost implications. Explicitly apply the million-User and deliberately hot-subject Terry gate; never make the User decode implementation jargon to choose.
- Keep semantic intelligence and command composition in explicitly invoked User-owned Agents. An Agent may name exact intended state, claimed causal dependencies and affected spatial scope; World may validate only structural facts it owns and must reject anything unbounded or structurally unjustified rather than infer meaning from prose, Entity names or Property content. A validated capability or collective ratification can authorize Agent-authored meaning without making World understand it. If a desired outcome requires semantic judgment, plan a bounded Agent-facing proposal or collective-deliberation capability with deterministic World admission and settlement instead of adding a server ontology, heuristic or LLM. Subscriptions may carry hints but never Agent invocation or authority.
- Keep the plan `draft` while a material question could change the player outcome, domain meaning, actor, action, state, ownership, nomenclature, public contract, irreversible data, external side effect, token spend, material cost or evidence claim. Grill the user until the required choices are explicit. Follow any explicit User cadence, including exactly one question per turn, and use the recording procedure below between questions.
- Present the complete plan and wait for explicit user acceptance. On acceptance, set `status: active` and `accepted_at`. Do not implement while the plan is `draft`.

## Execute the accepted plan

- Keep root responsible for scope, plan state, integration and the final evidence claim. Delegation remains optional and must comply with current coordination instructions.
- Select one dependency-ready task, mark it `in_progress`, implement its smallest complete change, run its focused evidence, then mark it `completed` or record one concrete blocker. Update the plan to current truth and repeat until its completion gate passes.
- When delegating, give an Agent the plan path and exactly one eligible task id. Require it to re-read live files, stay within owned surfaces, preserve invariants, run focused evidence and return raw results. Do not let delegated Agents edit shared plan state unless plan maintenance is their assigned task.
- Implement the smallest complete slice through every required `World`, PostgreSQL, HTTP, MCP, test, history, documentation and operational surface named by the plan. Every accepted state-changing game action leaves its durable Activity footprint in the same transaction.
- Refine file maps, ordering and evidence inside the accepted outcome without new approval. If evidence changes the outcome, public behavior, domain meaning, non-goals, irreversible state, external authority, material cost or evidence claim, stop; return the plan to `draft`, revise it, grill if needed and obtain explicit re-acceptance.
- Use specialist methods only when requested or when a named plan risk requires them. Add a fresh read-only review only when the build risk merits it.
- Set `status: complete` and `completed_at` only after every required task and validation passes, the exact outcome is demonstrated, all authorities agree and no material question remains. Do not automatically start another edge.

## Record every choice once

Recording is part of making a choice, not end-of-task cleanup. Repository state, not
conversation memory, carries work across turns. On starting or resuming a grill,
design or research-led decision, reread the active plan, active concept record and
applicable log, research and game authorities. After each material answer and before
the next question, update the active concept record's confirmed direction and open
decisions, then record the accepted, rejected, deferred, corrected or superseded
choice, material reason and affected scope in the current period file of
`dev/docs/concept/log/` according to its README. Record development history, not shell
commands. In the same change, update, correct or remove every affected authority so
no known-stale documentation remains:

- Put current exploration state in one active concept record and full decision history in the current period concept log; link rather than duplicating rationale.
- Put sourced facts and Aicadia implications in `dev/docs/research/` and update its index; research informs a choice but never makes one.
- Put only accepted current behavior and implementation in `game/docs/`; an unresolved or exploratory direction does not change the build contract.
- Put experiment questions, fixtures, observations and bounded verdicts in the project-designated `dev/lab/`. Lab artifacts never own choices or become production dependencies; retained artifacts remain experimental and production logic is redesigned separately.
- Record the real-versus-simulated seam boundary beside every lab verdict. Use a real database, protocol or Agent only when that seam is the remaining uncertainty, and limit the claim to what that direct run observed.
- Put canonical vocabulary in `dev/CONTEXT.md`.
- Change `AGENTS.md` only for an explicitly accepted, compact rule that should govern work across tasks.
- Write the full choice once in its authority and refer to it elsewhere instead of duplicating it.
- Keep `dev/backlog/README.md` and the current item aligned with material scope, status, dependencies and completion evidence. The backlog points to authoritative choices; it does not duplicate them.

Stop when the qualifying micro-change or accepted plan, earned documentation and exact verification are complete. Report the result and any concrete blocker; do not automatically start another step.
