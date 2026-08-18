---
status: active
---

# Multiplayer lab

> **Role / side:** retained multiplayer experiment track / development side.
> **Authority:** identifies each multiplayer lab artifact, its question and where to read it.
> **Excludes:** each experiment's own verdict, status and seams, which live in its front matter; accepted multiplayer game behavior, production architecture and sourced research conclusions; see `game/docs/`, accepted plans and `dev/docs/research/`.

This track supports the one-question-at-a-time multiplayer grill. Experiments are
created only when a concrete observation can change the next decision about
presence, perception, same-subject conflict, causality, catch-up, subscriptions,
Agent knowledge, overload or fairness.

The current proposed execution contract is the draft
[`multiplayer-lab` plan](../../../dev/plans/20260816-153410-multiplayer-lab/plan.md).
The active product exploration remains
[`concurrency-and-world-dynamics.md`](../../../dev/docs/concept/concurrency-and-world-dynamics.md).

The reusable [`scenario catalogue`](scenarios.md) fixes hard narratives and fixtures
so successive experiments test the same cases. It deliberately leaves product
outcomes open and has no authority over current game behavior.

## Experiments

Each experiment carries its own verdict, artifact status and real/simulated seams in
its front matter.

- [01 — Observation ownership](01-observation-ownership/README.md) — Can Q1–Q4 remain one World occurrence with transient host attention and Agent presentation only after World authorization?
- [02 — Subject conflict](02-subject-conflict/README.md) — Can independent same-Place intents avoid a Place revision while true placement, Property, absence and retry conflicts remain deterministic?
- [03 — PostgreSQL subject conflict](03-postgres-subject-conflict/README.md) — Do affected-Entity locks preserve exact conflicts while every disjoint same-Place path is wholly independent from the Place row?
- [04 — PostgreSQL conflict strategies](04-postgres-conflict-strategies/README.md) — Which bounded coordinator preserves operation-scoped present, absent and mixed dependencies without false Place- or lock-class conflicts?

## Multiplayer-specific boundaries

- One accepted World action and Activity are modeled once, never once per observer.
- Delivery, observation eligibility and actual Agent knowledge remain distinct.
- A subscription or notification is never authoritative World state and never
  triggers an LLM call.
- Same-Place actions are not made conflicting merely because they share a Place.
- A local simulation may test semantics and boundedness but may not claim production
  capacity or support for millions of concurrent Users.
