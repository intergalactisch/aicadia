---
status: complete
created_at: "2026-08-18T23:03:30+02:00"
updated_at: "2026-08-18T23:19:08+02:00"
accepted_at: "2026-08-18T23:03:30+02:00"
completed_at: "2026-08-18T23:19:08+02:00"
---

# Backcast Aicadia's five-year spatial foundation

> **Role / side:** execution plan for one source-backed five-year spatial backcast / development side.
> **Authority:** owns the bounded research, synthesis, review and evidence state for recommending Aicadia's long-term spatial foundation.
> **Excludes:** accepted product choices, current game behavior, canonical vocabulary, schema and runtime implementation; those remain in User decisions, `game/docs/`, `dev/CONTEXT.md` and runtime surfaces.

## Outcome

The User wakes to one source-backed, scenario-complete recommendation for a spatial
foundation that can plausibly remain fun, powerful, resilient, flexible, secure,
lightweight, clear and Terry-aligned after five years and millions of players. The
record first observes ordinary future play, culture, abuse, failures and operational
pressure, then walks backward to the smallest present decisions and experiments.
Future observation, recommendation and still-required User acceptance remain visibly
separate. Final evidence must prove primary-source coverage, SP01–SP13 coverage,
database/API/concurrency/privacy concreteness and honest non-claims.

## Non-goals

- Accepting the recommendation into Areas, `dev/CONTEXT.md`, `game/docs`, backlog, schema, API or code while the User is away.
- Pretending predicted scale, latency or throughput is measured evidence.
- Designing a universal physics engine, ontology, graph database, microservice fleet, global partition or background Agent system.
- Resolving detailed coordinates, geometry kernels, route planning or inventory mechanics beyond the minimum long-term seams the backcast proves necessary.

## Evidence and authorities

| Evidence or authority | Current fact | Consequence for this build |
| --- | --- | --- |
| User request, 2026-08-18 | Apply a five-year future-to-present methodology and return the best complete spatial system while the User sleeps. | The backcast is autonomous and recommendation-complete but does not silently accept choices. |
| `AGENTS.md` Five-Year Backcast | Describe ordinary future use, culture, abuse, failures and surviving pressures, then walk backward to the smallest present decisions and experiments. | The report separates observation, recommendation and User decision. |
| `dev/areas/place/scenarios.md` | Thirteen fixed scenes own the spatial pressure. | The recommendation must explain all thirteen without rewriting fixtures. |
| `dev/lab/spatial/01-model-pressure/README.md` | A paper matrix supports exact Position plus separate authored meaning/mechanics, with relative grounding unresolved. | The backcast must try to falsify this direction rather than merely repeat it. |
| Existing spatial research | Identity, coordinates, moving carriers, topology, privacy, indexing and massive concurrency already have primary-source groundwork. | Reuse and update; do not duplicate claims or ignore current-source drift. |
| `dev/docs/README.md` | Research, concept exploration, current contract and evidence have distinct homes. | Primary facts go to research; future recommendation goes to a live concept record; no contract change. |

## Alignment

### Strategic

Spatial state is the common foundation beneath exploration, travel, settlement,
creation, carrying, moving Worlds, observation and remote Actions. A weak early
choice would either make emergent play impossible or force an identity-breaking
migration after players have created cultural history. The highest-value current
edge is therefore not implementation but identifying which invariants must survive
five years and which mechanisms should remain deliberately absent today.

### Tactical

Produce one primary-source audit and one live backcast. The backcast includes: the
five-year World; abuse and failure stories; surviving invariants; recommended domain
layers; concrete candidate PostgreSQL shape; World read/write and visibility
contracts; concurrency, history and indexing; all thirteen scenario walkthroughs;
rejected alternatives; falsifiers; and a year-five-to-now roadmap ending in the
smallest present slice and earned experiments.

### Technical

No executable seam changes. The research uses real current primary web sources and
repository sources. PostgreSQL schema, World behavior, coordinate resolution,
spatial indexes, concurrency, privacy, HTTP, MCP, Agents, operations and production
scale remain proposed or simulated. No paid model call, external mutation, database
or runtime operation is authorized.

## Decisions, assumptions and open questions

### Confirmed decisions

- Use the five-year backcast exactly as requested and remain recommendation-complete without waking the User for intermediate choices.
- Test the existing leading candidate against the future rather than accepting it by momentum.
- Keep every new name visibly proposed; the Vocabulary Is Negotiated rule remains binding.
- Record findings and recommendation in their correct non-authoritative homes only.

### Reversible assumptions

- Five years means mature intended-scale operation, not a literal traffic forecast; all numeric capacity claims remain requirements or hypotheses.
- One source report plus one concept backcast is enough because implementation and measurement are explicitly excluded.
- Existing PostgreSQL/World/MCP boundaries remain constitutional unless the future exposes a direct contradiction, which the report must surface rather than silently change.

### Open questions

- Which exact recommended model survives the source audit and future backcast — resolved by T1 and T2 as a recommendation, still awaiting User choice.
- Which technical risks require immediate experiments — selected by backcasting from the surviving operational pressures, not by speculative completeness.

## Implementation map

| Surface | Current state | Intended change | Invariants |
| --- | --- | --- | --- |
| `dev/docs/research/spatial-five-year-foundation.md` | Absent | Add one current primary-source report covering long-lived spatial and scale mechanisms. | Research informs and never chooses product direction. |
| `dev/docs/research/README.md` | No link to the new report. | Add one question-focused index link. | Status remains in report front matter only. |
| `dev/docs/concept/spatial-five-year-backcast.md` | Absent | Add the complete future observation, recommendation, scenario validation and backward roadmap. | Every proposed term is labeled; no current Area truth is overwritten. |
| `dev/docs/concept/README.md` | No link to the backcast. | Add one themed-record link. | Concept remains non-authoritative over `game/docs`. |
| `dev/docs/concept/log/2026-08.md` | Ends at the four-candidate paper verdict. | Append the requested methodology, bounded recommendation and explicit pending User choice. | History stays append-only. |
| `dev/plans/20260818-230330-five-year-spatial-backcast/plan.md` | Active plan. | Maintain exact task and completion evidence. | No unrelated plan or worktree state changes. |

## Execution contract

Root owns scope, five-year judgment, synthesis, plan state and final evidence. The
research subagent owns only T1's source report and research index link, must reread
live files and may not edit concept, Area, plan, contract or runtime surfaces. T2
starts from the live completed report. Overlap is prohibited.

## Task graph

Allowed states are `pending`, `in_progress`, `completed` and `blocked`.

| ID | State | Depends | Parallel-safe | Objective | Owned surfaces | Evidence |
| --- | --- | --- | --- | --- | --- | --- |
| T1 | completed | — | yes | Produce the verified primary-source spatial foundation audit. | `dev/docs/research/spatial-five-year-foundation.md`, `dev/docs/research/README.md` | 23/23 primary URLs returned HTTP 200; Studio documentation lint passed 1/1; scoped diff check was clean. |
| T2 | completed | T1 | no | Produce and adversarially review the complete five-year backcast and recommendation. | `dev/docs/concept/spatial-five-year-backcast.md`, `dev/docs/concept/README.md`, `dev/docs/concept/log/2026-08.md`, this plan | All 13 scenario headings and all 13 required sections were present exactly as scoped; Studio documentation lint passed 5/5; `git diff --check` was clean. |

## Task details

### T1 — Audit current primary sources

**Objective:** Every load-bearing technical or comparative claim used by the
backcast has a current official specification, documentation or source-code owner.

**Actions:**

1. Verify and synthesize primary sources for local/World positions, transform
   hierarchies, moving carriers, coordinate precision, topology, spatial indexing,
   recursive bounds, interest, privacy/capabilities and PostgreSQL concurrency.
2. Separate external fact, Aicadia inference, production requirement and open risk.
3. Record scenario implications, falsifiers and non-claims and link the report once.

**Invariants:**

- No secondary-source-only load-bearing claim.
- No product choice, accepted vocabulary or implementation change.
- No source claim extends beyond what the cited primary page supports.

**Evidence:**

- Source URL audit and Studio documentation lint.
- `git diff --check` and exact owned-surface review.

**Stop conditions:**

- Stop and report if a primary source contradicts an accepted constitutional boundary rather than rationalizing the contradiction away.

### T2 — Backcast and recommend

**Objective:** The User can judge one concrete long-term system, its tradeoffs and
the smallest present action without decoding unexplained architecture jargon.

**Actions:**

1. Describe ordinary five-year play, emergent culture, abuse, failures and the
   technical/operational pressures that actually survived.
2. State the recommended layered model in player consequences and concrete
   PostgreSQL/World state, including identity, Position, relative grounding,
   authored meaning, Place/Area/topology/Route, access/privacy, mutation/history,
   derived indexes and admission.
3. Walk SP01–SP13 through actor, input, stored state, reads, mutation, privacy,
   contention and cost.
4. Compare and reject credible alternatives, state falsifiers/non-claims, and
   backcast year by year to the smallest present decisions and experiments.
5. Append only the bounded recommendation and pending User choice to the log.

**Invariants:**

- World stays dumb and strict; Agents author meaning explicitly.
- PostgreSQL remains authority; no graph database, microservices or process-local correctness.
- Every mutation is bounded, attributable and atomic with Activity.
- No global lock/revision, descendant rewrite, universal Relation semantics, identifier-derived privacy or unbounded read.
- No proposed term becomes canonical or current while the User is away.

**Evidence:**

- All thirteen scenario ids and required backcast/model/security/scale/roadmap sections are present.
- Every load-bearing sourced claim points to the T1 research owner.
- Studio documentation lint and `git diff --check` pass.

**Stop conditions:**

- Stop before changing current Areas, vocabulary, `game/docs`, backlog, schema, code or public surfaces; return recommendations and choices to the User instead.

## Validation ladder

1. **Focused:** source report, future pressures, model, schema/API, security, SP01–SP13, falsifiers and backcast-roadmap audits pass.
2. **Contract:** no current contract, vocabulary, Area, backlog, schema, runtime or public surface changes.
3. **Outcome:** one argued recommendation and one smallest present next step are understandable without the prior conversation.
4. **Integrity:** Studio documentation lint, `git diff --check`, link audit and focused owned-surface review pass.

## Change control

Refine sources, candidate detail, document structure and stronger evidence while the
accepted five-year research/recommendation outcome remains unchanged. Stop, return
to draft and request re-acceptance if work would accept product meaning, change a
current authority, add implementation, incur cost or claim measured production evidence.

## Completion conditions

- T1 and T2 are completed and the validation ladder passes;
- future observation, recommendation and pending User decision are visibly separate;
- all scenarios and surviving operational pressures are covered;
- no current authority silently accepts a proposed term or system;
- the plan records exact bounded evidence and completion time.

## Completion evidence

- T1 produced a 530-line research report grounded in 23 unique primary URLs. The
  delegated audit reached HTTP 200 for all 23, Studio documentation lint passed and
  the scoped diff check was clean.
- T2 produced a 1,238-line backcast with every required future, model, schema, read,
  write, privacy, concurrency, scenario, alternative, falsifier and roadmap section.
  The focused audit found SP01 through SP13 exactly once and all thirteen required
  top-level sections.
- `cargo test -p aicadia-studio --test studio lint` passed all five tests after the
  new concept status was corrected to the governed `active` value.
- `git diff --check` returned no error.
- Scoped status review showed changes only in the plan, the new research and concept
  records and their exact index/log links. No Area, vocabulary, current contract,
  backlog, schema, runtime or public surface changed.
