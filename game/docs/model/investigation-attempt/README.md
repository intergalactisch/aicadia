---
kind: provenance
storage_table: [investigation_attempt]
---

# Investigation attempt

> **Role / side:** Investigation attempt model contract / runtime side.
> **Authority:** the durable internal attempt, its Agent-selected result kind, exact Position grounding and every investigation chance and admission value.
> **Excludes:** discovered Entity, Place and Connection state — defined in their model contracts; accepted history — defined in [Activity](../activity/README.md).

An investigation is one explicit request by the Agent for World to test whether the
entered Character can find something from its exact current Position. The Agent
selects exactly one mechanical result kind from authoritative reads:
`entity_at_position` or `connected_place`. World derives the Character, current
Position and optional current Place, applies per-User admission and performs one
authoritative random draw before the Agent authors content. The User may advise the
Agent, but World never infers the kind from prose and the User supplies no seed,
odds, result count or retry count.

Every admitted start creates one durable internal attempt with one World-assigned id,
the responsible User, derived Character, selected kind, exact Position revision,
nullable current Place, stored `zero` or `positive` outcome, creation time and
optional consumed/voided provenance. The attempt is not an Entity, Activity, pending
opportunity, session or player-visible history. It exists only to make retry,
admission, grounding, bounded coexistence and one-time consumption exact across
processes and restarts. A same-kind start retry returns its stored outcome and
immutable limit without another draw; reusing the request id for the other kind
conflicts. Zero and unconsumed positive attempts change no current World state and
append no Activity. A voided positive always names a distinct newer attempt as
provenance and can never point to itself.

A positive `entity_at_position` attempt permits one Agent-authored Entity at the
bound point. A positive `connected_place` attempt permits one exact origin,
destination and newly identified Connection package without moving the Character.
After re-reading current Position and relevant spatial context, the Agent previews
the complete package and the User confirms it. World verifies only the typed attempt,
fresh grounding and submitted structure. There is no generic Discovery record,
Agent session, semantic parser or automatic read receipt.

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

When the Character has a current Place, signal `n` is the number of
`submit_discovery` Activities among the last `W` Activities at that Place. A loose
Position has no truthful aggregation owner, so `n = 0`. Chance is
`p = p_min + (p_max − p_min) · 2^(−n/h)`, resolved from operating-system entropy
behind World's private chance component. A fresh admitted attempt is independent.
Elapsed time, prior zero outcomes and consecutive misses never improve odds; there
is no pity, accumulated luck, region counter or runtime-configurable chance input.

Admission is decided before the roll: a User who already has `A` attempts inside the
inclusive rolling hour is rejected without an attempt row or draw. Only a newly
inserted positive that takes its User beyond `P` live positives voids the oldest
prior live positive, never itself.
