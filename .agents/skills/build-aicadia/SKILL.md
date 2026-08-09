---
name: build-aicadia
description: Guide Aicadia development by selecting the highest-leverage concrete game-development edge and executing its smallest safe evidence-based slice. Use for open next-work intent such as asking what to do now, what should come next, what is missing, or where to continue, and for substantial Aicadia research, product or domain decisions, implementation, or review requests.
---

# Build Aicadia

Keep the build moving without turning the process into a project of its own. Apply Terry and preserve unrelated user changes.

## Find the current edge

- Use the root `AGENTS.md` instructions already in context; read the file only if they are missing, and read nested instructions only when their scope applies.
- Start with `git status --short`, `rg --files`, and targeted `rg` searches for headings, status lines, contract terms, and relevant symbols.
- Read only narrow ranges from the leading `docs/game/`, code, and test candidates. Never load the full document tree or a long file merely to orient.
- When `.agents/backlog/README.md` exists, read its ordered horizon and the one linked current item. Treat it as forward planning context, never as authority over `docs/game/`.
- Open `docs/research/` or `docs/concept/` only when the question depends on their findings or history.
- Identify the highest-leverage concrete player or World outcome using the current contract as evidence, then find its smallest safe slice. KISS sizes the selected work; it does not decide which outcome matters most.
- The contract bounds implementation, not ambition. If the valuable next outcome is absent from `docs/game/`, propose the smallest concrete behavior decision that can evolve the contract before any code is written.
- Do not choose validation, cleanup, plumbing or documentation merely because it is bounded. It may lead only when it unlocks the selected game capability or retires a concrete blocker or risk on that edge.
- Stop gathering once one valuable gap has enough repository evidence. Run broader checks only when they could change the choice.

## Suggest exactly one step for open questions

- **Next** — State the concrete action on the selected game-development edge.
- **Why** — Point to the repository evidence and name the concrete player, World or game value it advances.
- **Done** — Give observable completion and verification conditions.

Do not write files or delegate work yet. Wait for the user to accept, reject, or refine the step.

## Build the agreed step

- Treat a specific build request as agreement on that scope.
- Keep the root agent responsible for scope, integration, and the final report.
- Work directly by default. Delegate only when the user asks or bounded independent work materially helps; delegation is never ceremony.
- Implement the smallest complete slice of the selected valuable behavior through every currently required surface.
- For every accepted state-changing game action, implement and verify its durable history footprint in the same transaction; do not infer it later from current state or transport logs.
- Run the relevant tests and report verification honestly.
- Use specialist methods only when the user asks or a concrete blocker requires them.
- For a substantial change, add a fresh read-only review only when its risk merits the extra pass.

## Record every choice once

Recording is part of making a choice, not end-of-task cleanup. As soon as a product,
domain, behavior, architecture, implementation, evidence or operational choice
crystallizes, record its accepted, rejected, deferred, corrected or superseded
status, material reason and affected scope in `docs/concept/log/log.md`. During an
unfinished grill or design session, update one active concept record after each
answer and keep confirmed direction separate from open decisions. Record development
history, not shell commands. In the same change, update, correct or remove every
affected authority so no known-stale documentation remains:

- Put accepted behavior and implementation in `docs/game/`; put sourced research in `docs/research/` and update its index.
- Put canonical vocabulary in `CONTEXT.md`; keep meaningful exploration history in the concept log.
- Change `AGENTS.md` only for an explicitly accepted, compact rule that should govern work across tasks.
- Write the full choice once in its authority and refer to it elsewhere instead of duplicating it.
- Keep `.agents/backlog/README.md` and the current item aligned with material scope, status, dependencies and completion evidence. The backlog points to authoritative choices; it does not duplicate them.

Stop when the agreed step, earned documentation, and relevant verification are complete. Report the result and any concrete blocker; do not automatically start another step.
