# Concurrent Entity requests

> **Role / side:** retained multiplayer experiment / development side.
> **Authority:** owns only experiment 06's fixture, observations and verdict.
> **Excludes:** current game behavior, production schema/API, semantic importance,
> voting, proposal rounds, cooldown and scale claims.

## Pending decision

Can one small exact-fact request kernel let independent changes to one Entity compose
while conflicting causal changes settle deterministically, without teaching World
what `felled`, `blue` or “destructive” means?

## Current slice — T3A

The fixture uses a concrete Tree with actual `color` and `condition` Properties. An
Agent-authored package contains its writes and every exact current or absent fact on
which those writes depend. World validates only bounds, placement, identities,
versions and atomicity. It never infers that felling matters more than painting.

Each accepted package commits, in one scratch-PostgreSQL transaction:

- current Property state and per-Property history;
- one Activity with actor, Place and operation;
- the bounded exact dependency snapshot;
- one accepted request identity and replay result.

A conflict, busy result or injected failure commits none of those records. A stable
transaction-scoped PostgreSQL advisory lock coordinates only equal
`(Character, request_id)` attempts; exact Property-slot rows coordinate facts.

Hard fixture bounds are 16 dependencies, 16 writes, 64 bytes per lower-snake-case
Property key, 4,096 stored bytes per value, 128 operation bytes and 64 KiB serialized
request input. Every written fact must have its own exact dependency. Accepted
dependencies on an absent fact intentionally retain one empty slot row; this storage
cost is measured rather than hidden.

## Run

```sh
DATABASE_URL=postgres://localhost/postgres \
  cargo test --locked \
  --manifest-path lab/multiplayer/06-concurrent-entity-requests/Cargo.toml \
  --test semantic_matrix -- --test-threads=1
```

## Evidence boundary

Real in T3A: SQLx, PostgreSQL constraints/locks/transactions, concurrent independent
pools, rollback, exact Activity/history storage and `EXPLAIN (ANALYZE, BUFFERS)`.

Simulated or absent: Agents, MCP, HTTP, authentication, authorization, network loss,
admission fairness, hosted clients and production/million-User load. The previously
planned T3B load, T3C rmcp and T3D Agent tiers are paused; they do not remain an
accepted continuation after the foundation choice changed.

## Observations

Thirteen real scratch-PostgreSQL tests passed. They showed exact one-accept/one-
conflict settlement on the same fact, concurrent composition of different facts on
one Tree, canonical multi-fact locking, expected-absence coordination, atomic
multi-write history, idempotent replay, stale placement rejection and complete
rollback after an injected post-Activity failure. A retained 100,000-unrelated-slot
`EXPLAIN (ANALYZE, BUFFERS)` used `property_slot_pkey`, touched six shared buffers and
reported 0.036 ms local execution. One accepted empty absence slot occupied 54 bytes
in this fixture. These are local bounded-mechanics observations, not capacity claims.

## Verdict

`inconclusive` / `active`. The exact-fact T3A matrix passed, but the User reopened its
foundational premise: this variant cannot detect an unmentioned concurrent fact that
changes the semantic validity of an Agent's action. It is retained as the narrow,
high-concurrency comparison variant. The User selected a whole coherent Entity basis
and one global registration/trigger/settlement system as the direction instead.
Further load, MCP and Agent tiers are paused until that plan is decision-complete and
re-accepted; no production behavior is selected.
