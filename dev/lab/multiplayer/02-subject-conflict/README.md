---
question: Can independent same-Place intents avoid a Place revision while true placement, Property, absence and retry conflicts remain deterministic?
verdict: supported
status: kept
real_seam: [Rust compiler, test runner, dependency sorting, validations, state transitions, accepted-request records, assertions in this crate]
simulated_seam: [World, transaction boundary, lock acquisition, storage, Character authority, request fingerprint construction, Activity persistence]
informs: dev/plans/20260816-153410-multiplayer-lab/plan.md#t3--prove-one-unified-semantic-change-kernel
---

# Subject conflict experiment

> **Role / side:** retained subject-conflict candidate experiment / development side.
> **Authority:** records this fixture, its bounded observations, verdict and artifact status.
> **Excludes:** current game behavior, a selected production transaction contract, PostgreSQL design, concurrency or scale proof.

## Pending decision

Before Aicadia selects a production conflict contract, test whether a Place-wide
revision is technically necessary merely because multiple intents happen at the
same Place.

This experiment informs T3 in the accepted
[`multiplayer-lab` plan](../../../../dev/plans/20260816-153410-multiplayer-lab/plan.md)
and the candidate dependency direction in
[`concurrency-and-world-dynamics.md`](../../../../dev/docs/concept/concurrency-and-world-dynamics.md).
It does not accept that candidate as game or production behavior.

## Question

Can one operation-specific dependency model accept independent intents at the same
Place without a Place revision, while deterministically rejecting stale placement,
same-subject conflicts and changed idempotency payloads?

## Falsifiable hypothesis

For one fixed in-memory World, intents prepared from the same initial snapshot can:

- both create different explicitly identified Entities at one Place;
- both change different Entities or different Properties of one Entity;
- produce exactly one winner when they depend on the same present or expected-absent
  Property;
- both append Activity-only Interactions without a Place conflict;
- reject an intent after its actor has moved;
- replay an equal request without another state change or Activity and reject the
  same request id with a changed fingerprint; and
- leave no accepted-request record or Activity for a rejection, so a corrected
  submission is evaluated normally; and
- check all operation-derived dependencies in one canonical order that contains no
  Place revision.

The hypothesis is false if any independent pair conflicts, both stale same-Property
intents mutate current state, a stale actor still acts, a retry duplicates history,
or an accepted request has anything other than one Activity.

## Fixture and exact run command

Run the standalone dependency-free Rust 2024 crate from the repository root:

```sh
cargo test --manifest-path dev/lab/multiplayer/02-subject-conflict/Cargo.toml
```

[`src/lib.rs`](src/lib.rs) is the one canonical fixture model. It contains:

- Ivo and Nia at Old Quarry, each with a placement version;
- a Stone and Door at the same Place;
- Property current values and per-Property versions;
- explicit `Present { value, version }` and `Absent` expectations;
- caller-supplied request ids and fixture fingerprints;
- explicit Entity ids for create operations, avoiding an allocator in the tested
  conflict path;
- operation-derived dependency keys sorted as Character placement, Entity existence
  and Entity Property keys;
- one in-memory accepted-request record per newly accepted request id; rejected
  attempts leave no such record; and
- one Activity, identified by the stable request id, for each newly accepted request.

`submit` validates the complete candidate dependency set before applying state and
appending Activity. Equal retries return the stored result instead of executing the
operation again. Request-id scope and uniqueness, including reusing it as the
Activity id, are fixture conveniences only and not a production identity contract.
The Activity vector's insertion order is test-process order only; it is not a global
World order or cursor proposal.

## Exact bounds

- Token and model calls: zero.
- Dependencies beyond the Rust standard library: zero.
- Persistence, PostgreSQL, network, HTTP, MCP and external services: none.
- Runtime: one local Rust test process; every test owns a fresh in-memory fixture.
- Tests: ten deterministic scenario tests.
- Actors: Ivo and Nia only.
- Places: Old Quarry and the fixture-only moved destination Quiet Grove.
- Existing Entities: one Stone and one Door; the create scenario adds exactly two.
- Per scenario: at most two competing requests, except retry and correction
  scenarios with three submissions.
- Per mutation intent: one Entity and at most two Property dependencies.
- Scheduling: explicit sequential interleavings prepared from the same snapshot; no
  threads, lock waits, deadlocks, throughput or duration load.

## Real and simulated seams

The Rust compiler, test runner, dependency sorting, validations, state transitions,
accepted-request records and assertions in this crate are real. They execute the
exact in-memory fixture committed here.

The World, transaction boundary, lock acquisition, storage, Character authority,
request fingerprint construction and Activity persistence are simulations.
PostgreSQL, production Aicadia code, actual concurrent requests, authentication,
HTTP, MCP, Agents and LLMs are absent. Sequential interleavings test the candidate
conflict semantics only; they do not prove that a concurrent implementation is
race-safe or atomic.

## Observations

All ten tests passed and produced these bounded observations:

1. Two distinct new Entity ids at Old Quarry were accepted with two Activities.
2. Stone state and Door state changed independently with two Activities.
3. Stone state and Stone color changed independently because each Property carried
   its own current value and version; two Activities were stored.
4. Two intents prepared from the same Stone-state version yielded one accepted
   result and one `PropertyChanged` conflict in either submission order; exactly one
   Activity remained.
5. Two intents expecting the same absent Stone `mark` Property yielded one version-1
   Property, one conflict and one Activity.
6. Ivo and Nia each appended an Activity-only Interaction at Old Quarry without
   changing Entity state or conflicting; two Activities remained.
7. Moving Ivo after intent preparation changed only his placement version; the stale
   intent was rejected without state change or Activity.
8. An equal request replay returned the original accepted result, while the same
   request id with another fingerprint conflicted; state version and Activity count
   remained unchanged after the first acceptance.
9. A two-Property mutation checked Character placement followed by lexically ordered
   Property keys. No Place dependency exists in the model, and the accepted request
   stored exactly one Activity.
10. A stale Property request left no accepted-request record and no Activity. A
    corrected submission using that still-unaccepted request id was then evaluated
    and accepted normally.

## Falsifier

Change the verdict to `refuted` if a reproducible case inside these stated bounds
shows that:

- distinct Entity or Property dependencies require a shared Place revision;
- two intents with the same stale Property expectation both mutate current state;
- `Absent` fails to act as an explicit conflicting expectation;
- movement after preparation does not invalidate the actor dependency;
- an equal retry changes state or Activity, or a changed fingerprint reuses the
  earlier request result;
- a rejected request creates an accepted-request record or prevents a later
  corrected submission solely through that rejected attempt;
- dependency order varies with input order; or
- accepted and rejected outcomes do not have the asserted exact Activity counts.

Change the verdict to `inconclusive` when the remaining uncertainty is actual
PostgreSQL locking, transaction isolation, unique constraints, durable idempotency,
parallel scheduling or crash recovery. Those seams are absent here.

## Verdict and artifact status

**Verdict: `supported`.** For the fixed simulated World, operation-specific
placement, Entity-existence and Property dependencies are sufficient to represent
all ten outcomes without a Place revision. Independent same-Place work proceeds,
while same-Property, expected-absence, stale-placement and fingerprint conflicts
remain deterministic with exact Activity counts.

**Artifact status: `kept`.** The crate remains non-authoritative experimental
evidence. Its code must not be imported, copied or promoted into production.

## Limitations and non-claims

This does not prove that Aicadia should adopt this gameplay contract. It does not
prove PostgreSQL transaction or lock correctness, simultaneous execution,
serializability, crash atomicity, durable retry reconstruction, canonical production
fingerprinting, identity generation, Activity ordering, fairness, latency,
throughput or million-User scale. Per-Property versions, request-id Activity
identity and dependency-key order are candidate fixture choices, not accepted schema
or API decisions.

## Downstream implication

The Place-wide conflict unit is not required to continue T3 reasoning. The next
grill can compare this bounded candidate against concrete mechanics that genuinely
need multi-subject atomicity, decide who declares each expected value or absence,
and only then choose a production transaction and idempotency contract.
