---
name: build-aicadia
description: Guide Aicadia development by selecting and executing the smallest evidence-based step that advances the current build contract. Use for open next-work intent such as asking what to do now, what should come next, what is missing, or where to continue, and for substantial Aicadia research, product or domain decisions, implementation, or review requests.
---

# Build Aicadia

Keep the build moving without turning the process into a project of its own. Apply Terry and preserve unrelated user changes.

## Find the current edge

- Use the root `AGENTS.md` instructions already in context; read the file only if they are missing, and read nested instructions only when their scope applies.
- Start with `git status --short`, `rg --files`, and targeted `rg` searches for headings, status lines, contract terms, and relevant symbols.
- Read only narrow ranges from the leading `docs/game/`, code, and test candidates. Never load the full document tree or a long file merely to orient.
- Open `docs/research/` or `docs/concept/` only when the question depends on their findings or history.
- Stop gathering once one gap has enough repository evidence. Run broader checks only when they could change the answer.

## Suggest exactly one step for open questions

- **Next** — State the concrete action.
- **Why** — Point to the repository evidence that makes it useful now.
- **Done** — Give observable completion and verification conditions.

Do not write files or delegate work yet. Wait for the user to accept, reject, or refine the step.

## Build the agreed step

- Treat a specific build request as agreement on that scope.
- Keep the root agent responsible for scope, integration, and the final report.
- Give every repository write to one bounded `gpt-5.6-sol` high-reasoning agent; keep one writer active at a time.
- Implement the smallest complete slice of the accepted behavior through every currently required surface.
- Run the relevant tests and report verification honestly.
- Use specialist methods only when the user asks or a concrete blocker requires them.
- For a substantial change, add a fresh read-only review only when its risk merits the extra pass.

## Record once

Write new knowledge once in its authority and refer to it elsewhere instead of duplicating it:

- Put accepted behavior and implementation in `docs/game/`; put sourced research in `docs/research/` and update its index.
- Put canonical vocabulary in `CONTEXT.md`; put meaningful exploration history in `docs/concept/log/log.md`.
- Change `AGENTS.md` only for an explicitly accepted, compact rule that should govern work across tasks.

Stop when the agreed step, earned documentation, and relevant verification are complete. Report the result and any concrete blocker; do not automatically start another step.
