# Aicadia lab

> **Role / side:** retained experimental workbench / development side.
> **Authority:** governs lab-wide experiment boundaries, artifact status and track placement.
> **Excludes:** current game behavior, production implementation, sourced research authority and delivery proof; see `docs/game/`, runtime sources, `docs/research/` and `docs/evidence/`.

`lab/` preserves small experiments that help Aicadia make a concrete product or
technical decision. The code may be rough, narrow and intentionally incomplete. It
only needs to make its own question inspectable and reproducible; it does not need
production architecture, compatibility, polish or long-term support.

Retention never promotes an experiment. Production behavior is designed and built
separately under an accepted plan, against current runtime invariants. Runtime code
must not import a lab artifact or depend on lab-local state.

The lab also never owns a product choice. An experiment links to the active concept
or plan question it tests and reports only its bounded observation and verdict.
Current design rationale and open choices live in `docs/concept/`, chronological
decisions in `docs/concept/log/`, sourced findings in `docs/research/`, and accepted
current behavior in `docs/game/`.

## Tracks

| Track | Status | Purpose | Index |
| --- | --- | --- | --- |
| Multiplayer | Active | Multiplayer semantics, subject-scoped concurrency, observation, delivery and overload | [`multiplayer/README.md`](multiplayer/README.md) |

New tracks require a concrete current question. `lab/` is not limited to
multiplayer, but empty speculative track directories do not earn a place here.

## Experiment contract

Every experiment directory or record names:

- one pending decision and one falsifiable technical question;
- the smallest fixture and the behavior deliberately excluded;
- time, workload, request, connection and any token or cost bounds;
- which database, protocol, host, Agent and client seams are real and which are
  simulated;
- exact observations and what would falsify the result;
- a verdict: `supported`, `refuted` or `inconclusive`;
- an artifact status: `active`, `kept`, `superseded` or `discarded`;
- the downstream question, concept choice or plan change it informs.

Start token-free and in-memory. Use Postgres, MCP or an Agent only when their real
seam is the uncertainty being tested. Agent calls are explicit, bounded and
announced before execution; a lab never creates background token spend.

Compare multiple credible implementations or settings when their difference can
change the pending decision, and record each result against the same fixture and
criteria. This is permission, not ceremony: one falsifiable setup remains preferable
when alternatives cannot alter the choice. Do not turn parameter sweeps, local
microbenchmarks or variant counts into evidence of quality or production scale.

## Implementation medium

Aicadia technical experiments default to small standalone Rust crates. Use
`cargo test --manifest-path lab/<track>/<experiment>/Cargo.toml` so state,
concurrency, ordering and integration lessons are expressed in the runtime language
without making the root package depend on lab code. A lab may depend on a production
interface only when that real seam is the stated question; otherwise keep the
fixture independent and dependency-free.

Use HTML/JavaScript only when human visual or semantic interaction is itself the
evidence, not merely because it is quick to prototype. Maintain one canonical state
machine per experiment; a later viewer may render recorded Rust fixture output but
must not duplicate the logic. Production may adopt the proven invariants, scenarios
and measurements, but experimental implementation is still redesigned and reviewed
under its own accepted build plan rather than copied into runtime.

## Evidence layers

Use two complementary layers and keep their verdicts separate:

1. Deterministic fixtures and authoritative readback test authorization, privacy,
   state transitions, boundedness and failure behavior only for the exact World
   implementation and fixture they execute. An in-memory or simulated World supports
   fixture or model claims, never production World behavior.
2. Direct protocol and Agent smokes test only the exercised transport integration,
   tool comprehension and grounded presentation.

A simulated host or Agent is never reported as MCP or model evidence. Conversely, a
successful Agent answer cannot prove that hidden data was filtered correctly, that a
notification is durable or that World state is race-safe; those require independent
deterministic evidence. When both layers are useful, run the smallest deterministic
test first, then the smallest real-seam smoke—normally one explicit Agent call and
never more than the accepted plan bound. Record model or client identity, call and
token or cost limits, independent authoritative readback and verified cleanup.

## Artifact status

- `active`: currently being changed or measured for one stated question.
- `kept`: the run is finished and remains useful evidence within its stated scope.
- `superseded`: a later experiment or decision replaced its conclusion; retain the
  trail and point to the replacement.
- `discarded`: the artifact no longer helps and may be removed; retain a concise
  record when its rejection materially informed a choice.

Roughness is acceptable. Ambiguous questions, hidden fixtures, unsupported scale
claims, copied credentials and accidental production dependencies are not.
