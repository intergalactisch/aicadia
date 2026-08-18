---
kind: provenance
storage_table: [investigation_attempt]
---

# Investigation attempt

> **Role / side:** Investigation attempt model contract / runtime side.
> **Authority:** the durable internal attempt, the discovery it permits and every investigation chance and admission value.
> **Excludes:** the found Entity's own state, Activity shape and delivery status; see the Entity, Property, Trait and Activity contracts and `dev/docs/evidence/`.

An investigation is one explicit request by the Agent for World to test whether the
entered Character can find something at its exact current Place. World derives both
Character and Place, applies per-User admission, reads the bounded recent Place
history signal and performs one authoritative random draw before the Agent authors
content. The User may advise the Agent but supplies no mechanical focus, seed, odds,
result count or retry count.

Every admitted start creates one durable internal attempt with one World-assigned id,
the responsible User, derived Character and Place, stored `zero` or `positive`
outcome, creation time and optional consumed/voided provenance. The attempt is not an
Entity, Activity, pending opportunity, session or player-visible history. It exists
only to make retry, admission, bounded coexistence and one-time consumption exact
across processes and restarts. A start retry returns its stored outcome and immutable
limit without another draw. Zero and unconsumed positive attempts change no current
World state and append no Activity. A voided positive always names a distinct newer
attempt as provenance and can never point to itself.

A positive attempt permits one discovery: an Agent-authored Entity representing
something found rather than made, brought or placed. After re-reading current
exact-Place context, the Agent previews the complete name, description, 0–100
Properties and 0–100 Traits and the User confirms them. World cannot infer or prove
the found-versus-made distinction; the Agent contract owns it. World verifies only
the typed attempt and find rules, then atomically creates and places the Entity,
establishes its state, appends `submit_discovery` Activity, consumes the attempt and
advances the Place pointer. There is no generic Discovery record or World-typed kind.

## Investigation chance and admission

This section is the single home of every investigation chance and admission value.
They are internal operational constants: no adapter, capability, Agent or User
supplies, reads or negotiates one, and changing any of them is a documented contract
change rather than configuration.

| Symbol | Value | Meaning |
| --- | --- | --- |
| `p_max` | `1/2` | chance at a Place with no recent discovery |
| `p_min` | `1/10` | floor a saturated Place approaches but never reaches |
| `h` | `6` | recent discoveries that halve the remaining distance to `p_min` |
| `W` | `48` | last Activities read at the exact Place to derive the signal `n` |
| `A` | `12` | new attempts admitted per User in one inclusive rolling hour |
| `P` | `3` | live unconsumed positives per User before the oldest is voided |

The signal `n` is the number of `submit_discovery` Activities among the last `W`
Activities at that Place, and chance is `p = p_min + (p_max − p_min) · 2^(−n/h)`,
resolved from operating-system entropy behind World's private chance component. A
fresh admitted attempt is independent. Elapsed time, prior zero outcomes and
consecutive misses never improve odds; there is no pity, accumulated luck or
runtime-configurable chance input.

Admission is decided before the roll: a User who already has `A` attempts inside the
inclusive rolling hour is rejected without an attempt row or draw. Only a newly
inserted positive that takes its User beyond `P` live positives voids the oldest
prior live positive, never itself.
