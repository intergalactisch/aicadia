# Multiplayer lab

> **Role / side:** retained multiplayer experiment track / development side.
> **Authority:** indexes multiplayer lab artifacts, their bounded verdicts and status.
> **Excludes:** accepted multiplayer game behavior, production architecture and sourced research conclusions; see `docs/game/`, accepted plans and `docs/research/`.

This track supports the one-question-at-a-time multiplayer grill. Experiments are
created only when a concrete observation can change the next decision about
presence, perception, same-subject conflict, causality, catch-up, subscriptions,
Agent knowledge, overload or fairness.

The current execution contract is the accepted
[`multiplayer-lab` plan](../../.agents/plans/20260816-153410-multiplayer-lab/plan.md).
The active product exploration remains
[`concurrency-and-world-dynamics.md`](../../docs/concept/concurrency-and-world-dynamics.md).

## Experiments

| Experiment | Question | Verdict | Status |
| --- | --- | --- | --- |
| [01 — Observation ownership](01-observation-ownership/README.md) | Can Q1–Q4 remain one World occurrence with transient host attention and Agent presentation only after World authorization? | `supported` for the fixed in-memory scenarios; no MCP, LLM, database, ordering or scale claim | `kept` |
| [02 — Subject conflict](02-subject-conflict/README.md) | Can independent same-Place intents avoid a Place revision while true placement, Property, absence and retry conflicts remain deterministic? | `supported` for ten Rust in-memory interleavings; PostgreSQL concurrency and the production contract remain unproved | `kept` |
| [03 — PostgreSQL subject conflict](03-postgres-subject-conflict/README.md) | Do affected-Entity locks preserve exact conflicts while every disjoint same-Place path is wholly independent from the Place row? | `refuted` for total Place-row independence because placement foreign keys take `KEY SHARE`; narrower subject isolation and current/absence correctness are supported in five scratch-schema tests | `kept` |
| [04 — PostgreSQL conflict strategies](04-postgres-conflict-strategies/README.md) | Which bounded coordinator preserves operation-scoped present, absent and mixed dependencies without false Place- or lock-class conflicts? | `refuted` for the initial hybrid candidate; an exact Property slot is strongest within ten focused scratch-schema tests, while `SERIALIZABLE` remains partial | `kept` |
| [05 — PostgreSQL/MCP interest strategies](05-postgres-mcp-interest-strategies/README.md) | Which global, Place, exact or structural-hybrid interest form gives the strongest required live coverage with bounded database, gateway, MCP and refetch cost? | `inconclusive` overall; structural topology remains strongest, while a focused follow-up separately supports 64-stripe quiet isolation, its exercised one-client rmcp success path and listen-then-baseline recovery after a forced-fatal PgListener-pool loss | `kept` |

## Multiplayer-specific boundaries

- One accepted World action and Activity are modeled once, never once per observer.
- Delivery, observation eligibility and actual Agent knowledge remain distinct.
- A subscription or notification is never authoritative World state and never
  triggers an LLM call.
- Same-Place actions are not made conflicting merely because they share a Place.
- A local simulation may test semantics and boundedness but may not claim production
  capacity or support for millions of concurrent Users.
