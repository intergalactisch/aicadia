---
status: complete
created_at: "2026-08-18T21:06:12+02:00"
updated_at: "2026-08-18T21:14:50+02:00"
accepted_at: "2026-08-18T21:14:11+02:00"
completed_at: "2026-08-18T21:14:50+02:00"
---

# Require negotiation before Aicadia vocabulary becomes current

> **Role / side:** execution plan for one compact Terry vocabulary rule / development side.
> **Authority:** owns the accepted scope, task state and evidence for adding the cross-task rule.
> **Excludes:** canonical vocabulary, product behavior and spatial decisions; those remain in `dev/CONTEXT.md`, `game/docs/` and the current Area and concept authorities.

## Outcome

A building Agent must keep every newly invented Aicadia-specific term visibly
provisional until the User accepts its concrete meaning. One compact Terry rule in
`AGENTS.md` prevents a convenient research or implementation label from silently
becoming current domain, product, architecture or data truth. Final evidence must
prove the rule is present once, routes accepted vocabulary to `dev/CONTEXT.md`, keeps
ordinary English and ecosystem terminology usable, and leaves current game behavior unchanged.

## Non-goals

- Define new spatial, Relation, Position, Inventory or Containment behavior.
- Require negotiation for ordinary prose or established ecosystem vocabulary that
  carries no Aicadia-specific meaning.
- Change a skill, methodology, schema, runtime, `game/docs/` or public Agent surface.

## Evidence and authorities

| Evidence or authority | Current fact | Consequence for this build |
| --- | --- | --- |
| User choice recorded in `dev/docs/concept/log/2026-08.md` | New Aicadia terms must be negotiated instead of smuggled into current truth. | The rule direction is accepted; exact wording remains the plan-acceptance gate. |
| `AGENTS.md` — Terry, Concrete Before Abstract, Game And Server Vocabulary | Terry governs concept and build decisions, but no rule explicitly keeps new terms provisional pending User agreement. | Add one adjacent compact rule; do not duplicate existing vocabulary or documentation-placement rules. |
| `dev/CONTEXT.md` | Owns canonical project and domain terminology. | Accepted Aicadia vocabulary routes here; candidates never do. |
| `dev/docs/README.md` | `AGENTS.md` changes only for an explicitly accepted compact cross-task rule. | Wait for plan acceptance, then change the constitution and decision trail together. |

## Alignment

### Strategic

Shared World freedom depends on stable language that Agents and Users mean in the
same way. Preventing accidental primitives such as generic Containment protects the
game from hidden rules and keeps later spatial and multiplayer decisions genuinely negotiable.

### Tactical

Add exactly one cross-task rule and record its adoption. The rule covers
Aicadia-owned product, domain, behavior, architecture and data vocabulary; requires
concrete meaning and examples before acceptance; labels candidates visibly; blocks
unaccepted terms from constraining current authorities or implementation; and
defines the correction path when leakage is found.

### Technical

Runtime, database, transactions, HTTP, MCP, public text, idempotency and operations
are not applicable. The only implementation seam is build-facing Markdown. The
documentation lint and a focused diff prove structure and scope, not future Agent compliance.

## Decisions, assumptions and open questions

### Confirmed decisions

- Add a compact Terry rule governing terminology across tasks — explicitly requested by the User and recorded in the current concept log.
- Preserve ordinary English and ecosystem vocabulary unless it begins carrying Aicadia-specific meaning — avoids unusable ceremony while closing the observed leak.

### Reversible assumptions

- Place the rule between Concrete Before Abstract and Game And Server Vocabulary — these adjacent heuristics own concept explanation and accepted operational language; focused review confirms fit.

### Open questions

- None. The User accepted the exact proposed wording and single-rule scope as option A on 2026-08-18.

## Proposed rule text

### Vocabulary Is Negotiated

Never introduce an Aicadia-owned product, domain, behavior, architecture or data
term as current merely because it is convenient. First compare it with
`dev/CONTEXT.md`, explain the concrete actor, action or state it distinguishes and
give included and excluded examples, then negotiate it with the User. Until explicit
acceptance, label it `working`, `candidate` or `unaccepted` in its owning exploration
and never let it constrain an Area's `Chosen` section, a plan, `game/docs/`, schema,
API or code. On acceptance, update `dev/CONTEXT.md` and every affected current
authority together. If an unaccepted term is found carrying current invariants, stop
dependent work, reopen it with the User and correct the affected current authorities;
retained history remains history. Ordinary English and ecosystem vocabulary need no
negotiation unless they begin carrying Aicadia-specific meaning.

## Implementation map

| Surface | Current state | Intended change | Invariants |
| --- | --- | --- | --- |
| `AGENTS.md` | Concrete explanation and conventional vocabulary rules exist, but candidate-term negotiation is implicit. | Add the accepted rule text once beside those heuristics. | Keep the constitution compact; do not duplicate `dev/CONTEXT.md` content or change game behavior. |
| `dev/docs/concept/log/2026-08.md` | Records the requested direction and draft plan. | Record exact acceptance and completed constitutional adoption. | History remains append-only and distinguishes request, acceptance and delivery. |

## Execution contract

Root owns outcome, scope, plan state, integration and final evidence. This one-task
documentation change is not delegated. Any material wording or scope change returns
the plan to draft for User re-acceptance.

## Task graph

| ID | State | Depends | Parallel-safe | Objective | Owned surfaces | Evidence |
| --- | --- | --- | --- | --- | --- | --- |
| T1 | completed | — | no | Adopt the accepted compact terminology rule and record it. | `AGENTS.md`, `dev/docs/concept/log/2026-08.md`, this plan | Focused text audit, documentation lint and diff integrity passed. |

## Task details

### T1 — Adopt the negotiated-vocabulary rule

**Objective:** Every building Agent receives one always-loaded rule that keeps new
Aicadia vocabulary provisional until User acceptance.

**Actions:**

1. Add the accepted text once to `AGENTS.md` beside the related Terry heuristics.
2. Record adoption in the current concept log and complete this plan only after evidence passes.

**Invariants:**

- No product, schema, runtime, public text, skill or methodology behavior changes.
- Candidate vocabulary remains usable in research and grills only when visibly provisional.
- Existing user changes outside the owned surfaces remain untouched.

**Evidence:**

- `rg -n "Vocabulary Is Negotiated|working.*candidate.*unaccepted" AGENTS.md` — the rule and provisional labels appear in the owning constitution.
- `cargo test -p aicadia-studio --test studio the_documentation_lint_is_clean -- --nocapture` — governed documentation remains valid.
- `git diff --check` — changed text has no whitespace errors.

**Stop conditions:**

- Stop before editing `AGENTS.md` if the User changes the wording, scope or exception boundary without accepting the revised plan.

## Validation ladder

1. **Focused:** inspect the single `AGENTS.md` insertion and verify each required clause is present once.
2. **Contract:** documentation lint passes; no runtime or product contract file changes for this task.
3. **Outcome:** a fresh reader can distinguish accepted vocabulary, explicitly provisional candidates and ordinary non-domain language from the rule alone.
4. **Integrity:** `git diff --check`, focused diff review and confirmation that unrelated user changes and all governing authorities remain intact.

## Change control

Refine evidence in place while the accepted rule meaning and exception remain
unchanged. Stop implementation, keep `status: draft`, revise and request explicit
re-acceptance if wording changes which terms require negotiation or where candidates may be used.

## Completion conditions

- T1 is `completed` and the validation ladder passes;
- the exact accepted rule exists once in `AGENTS.md` and its adoption is recorded;
- no current product, schema, runtime, public text, skill or methodology changed;
- no known-stale affected authority remains;
- `status: complete` and `completed_at` are recorded only after these conditions.
