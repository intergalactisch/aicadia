---
status: load-bearing
era: August Activity-Property-Trait
---

# Idempotent action delivery and Place freshness

> **Role / side:** sourced research report / development side.
> **Authority:** records this report's question, sources, findings and implications.
> **Excludes:** product decisions and current implementation contracts; see `game/docs/`.

Date: 2026-08-11

Status: research recommendation accepted in the active design grill; not current
executable Aicadia behavior

## Question

What is the smallest robust contract for an Agent that reads several pieces of
World state, privately drafts one action with its User and submits one irreversible
package?

In particular:

- should a read issue the identifier later used as the action request id;
- how should an uncertain delivery be retried without duplicating World state;
- how can World reject an action grounded in stale relevant state;
- can several independent MCP reads truthfully represent one database snapshot; and
- which guarantees belong in the public World interface rather than Agent
  orchestration or Postgres implementation?

## Evidence boundary

This report uses primary specifications and official implementation guidance. The
IETF Idempotency-Key document is an expired Internet-Draft, not an RFC; it is useful
evidence of an emerging API pattern but is not treated as a standard. RFC 9110 is the
normative HTTP reference. PostgreSQL documentation establishes database behavior,
not an Aicadia product choice.

Current Aicadia code supplies additional local evidence:

- each MCP or HTTP read is an independent `World` call;
- current reads use the pool directly and do not share a durable transaction;
- every current mutation opens one Postgres transaction and locks the responsible
  User row;
- accepted mutations append immutable Activity in the same transaction;
- Character placement is re-read inside contextual mutations; and
- there is no current Character movement, Entity update, Place revision or generic
  World revision.

## Distinct guarantees

Three values that may look like “a unique hash” solve different problems and must
not be conflated.

| Value | Question it answers | Correct owner |
|---|---|---|
| request id | “Is this another delivery attempt for the same intended action?” | Agent/client creates it once per intent |
| payload fingerprint | “Was this request id reused with different normalized input?” | World derives it from accepted input |
| state revision | “Is the relevant state still the version the Agent observed?” | World derives and returns it for one defined representation |

Payload equality is not action identity. Two Users may legitimately introduce equal
trail-marker text as two actions, while one action may be delivered several times
after a lost response. A payload hash therefore cannot replace the request id.

## Finding 1 — the request id represents client intent

The IETF Idempotency-Key draft defines the key as a client-generated unique value
used to recognize retries of the same request. It recommends a UUID-like identifier,
allows the server to derive a fingerprint from request fields and distinguishes a
completed retry from concurrent use or reuse with different payload. AWS similarly
models an explicit client request identifier and has its SDK or CLI generate one
when the caller does not, then reuses it for automated retries. Stripe requires a
unique key for a new operation and rejects reuse with different parameters.

Implications:

- an Agent-generated UUID is not weaker merely because World did not allocate it;
- the key must remain stable across an uncertain retry and change for a new intent;
- World owns enforcement, fingerprinting and canonical retry results; and
- the public contract must define scope and retention.

RFC 9562 defines UUIDv4 as 122 random bits plus version and variant. A UUIDv4 is
already supported by Aicadia and is sufficient here; request ordering comes from
Activity, so UUIDv7 would add no current behavior.

## Finding 2 — a read-issued nonce does not prove freshness

A random nonce issued with a read has only three possible implementations:

1. Persist it immediately. Then a read becomes a write, needs expiry and cleanup and
   creates durable preparation/session state before the User has chosen an action.
2. Sign or otherwise encode it without persistence. This adds key or token machinery
   but remains only a unique nonce; it does not prove which state the Agent used.
3. Derive it from response state. Then equal observations share a value, so two
   independent intended actions collide if the value is also used as request id.

None improves idempotent delivery over a client UUID. The first two add machinery;
the third gives the value state-revision semantics instead of request-identity
semantics.

Recommendation: do not add `prepare_action`, pending submissions, one-time tokens or
read-side writes. If a read returns a World-issued value, it should be a derived
state revision with explicitly bounded meaning.

## Finding 3 — strong revisions are representation-scoped

RFC 9110 defines an ETag as an opaque validator for versions of one selected
representation. `If-Match` uses strong comparison and prevents a state-changing
method from being applied after that representation changed. The specification does
not make an ETag globally unique across different resources.

That scope is essential for Aicadia. A global World revision would invalidate an
action at one Place whenever an unrelated action occurred anywhere. As the World
grows, almost every prepared action would be stale before submission. A revision
must therefore cover only state whose change should invalidate this action.

For the first trail-marker action, the narrow coherent representation is the exact
current Place action context:

- the Character's exact current Place identity;
- the Entity state established directly at that Place; and
- the immutable Activity/prose history whose context or involved roles make it part
  of that exact Place view.

A strong `place_revision` may be returned as an opaque transport value derived from
the Place identity and latest accepted Place-relevant Activity ordering key. It is a
revision, not a secret, authorization capability, request id or payload hash.

## Finding 4 — separate MCP reads are not one database snapshot

PostgreSQL Repeatable Read gives a stable snapshot only to statements inside one
transaction. Exported snapshots also require a live exporting transaction and must
be imported at the start of another Repeatable Read or Serializable transaction.
Independent MCP calls therefore cannot honestly share one database snapshot without
introducing a durable connection/session protocol.

Aicadia should not hold a database transaction open while an Agent reasons or waits
for User confirmation. That would couple token latency to database locks and
connections, violate the stand-alone call model and create operational failure
modes.

Composable reads can still be coherent:

- every exact-Place response returns the `place_revision` of the representation it
  actually read;
- an Agent only combines Place data carrying the same revision;
- if revisions differ, the Agent refreshes the older Place reads; and
- final submission carries that revision as `expected_place_revision`.

This is optimistic concurrency, not a cross-call snapshot.

## Finding 5 — World must validate under a Place lock

Checking a revision before starting the write transaction has a time-of-check to
time-of-use race. PostgreSQL row locks are held through the transaction and block
other writers or lockers of the same row without blocking ordinary reads.

For a Place-scoped action, World should:

1. normalize the package and derive its payload fingerprint;
2. begin one transaction and lock the responsible User;
3. resolve an already accepted `(user_id, request_id)` and return its canonical
   result when the fingerprint agrees;
4. only for an unseen request, re-derive and lock the Character's current Place row;
5. compare `expected_place_revision` with the current revision while holding that
   Place lock;
6. validate and write Entity placement, Activity and prose; and
7. commit and return the canonical result.

Every operation that changes the defined exact-Place representation must follow the
same Place-lock protocol before appending its Place-relevant Activity. This serializes
accepted mutations at one Place, not reads or mutations at unrelated Places.

Resolving an accepted request before current Character and Place preconditions is
intentional. A delayed retry must return the first canonical acceptance even if later
World actions changed that Place; it must not be reinterpreted as a new action against
current state.

If the revision changed, World rejects the entire action with a typed stale-context
error and writes no Activity. The Agent must reread and let the User confirm a newly
grounded package; World must not silently rebase prose or consequences.

## Alternatives

| Alternative | Result | Aicadia assessment |
|---|---|---|
| response hash used as request id | equal observations collide; state and intent are conflated | reject |
| persisted one-time token from a read | read-side writes, expiry, cleanup and pending workflow state | reject |
| stateless server nonce from a read | equivalent uniqueness to client UUID with extra machinery | reject |
| global World revision | unrelated activity invalidates local actions | reject |
| client request id only | safe delivery retries, but no guarantee about observed Place state | insufficient for the desired grounded action loop |
| client request id plus exact-Place revision | separates intent, payload equality and relevant-state freshness | recommend |
| arbitrary list of resource revisions | flexible but pushes dependency selection and orchestration into every Agent | defer until a concrete multi-resource update requires it |

## Recommended first contract

Research recommends the following draft direction:

```text
exact-Place reads
  -> data
  -> place_revision

submit_action
  -> request_id                 # Agent-created UUID, stable for retries
  -> expected_place_revision    # copied unchanged from coherent Place reads
  -> prose
  -> one typed consequence
```

World derives a versioned fingerprint from the normalized prose, typed consequence
and expected Place revision. For an already accepted request id:

- equal fingerprint returns the canonical accepted result;
- different fingerprint returns `action_request_conflict`; and
- World never creates another Activity.

For an unseen request id:

- stale Place revision returns a typed precondition failure with no mutation;
- valid current revision proceeds through ordinary deterministic validation; and
- success stores request id and fingerprint with the immutable accepted Activity.

Because Activity is permanent, accepted idempotency identity can also remain
permanent at one small fixed storage cost per action. No cleanup job or expiry rule
is then required. Definitively rejected or malformed attempts are not World history
and do not reserve an accepted request id; Agent instructions should generate a new
id after a known rejection or any redraft, and reuse an id only when the delivery
outcome is unknown.

The exact-Place revision is justified in this first action despite the additive
consequence: the public prose and proposal are explicitly grounded in shared local
state that may change while the User deliberates. The scoped precondition prevents a
stale narrative commit without blocking unrelated Places. This is stricter than
ordinary additive-POST handling and is an Aicadia game choice, not a conclusion of
the HTTP standard.

## Boundaries

- The revision never authorizes a mutation; World still derives User, Character and
  Place and applies all deterministic rules.
- A revision is not a durable Agent session, reservation or lock. Reads remain
  side-effect free.
- World never hashes “the whole World” or requires every queried value to remain
  unchanged.
- The Agent does not interpret or construct revision tokens.
- A stale rejection does not automatically spend tokens or invoke the Agent.
- Future updates to an existing Entity should earn that Entity's own explicit
  version/precondition rather than enlarging `place_revision` into a universal
  dependency token.
- HTTP may map stale revision to `412 Precondition Failed`; MCP exposes the same
  semantic World error without depending on HTTP headers.

## Implications for Aicadia

This direction deepens the existing `World` interface: callers supply one intent id,
one opaque relevant-state precondition and semantic action content; `World` hides
normalization, fingerprinting, locking, duplicate resolution, revision comparison,
atomic writes and canonical retry results. HTTP and MCP remain thin adapters.

It adds no generic action session, token table, global counter, event sourcing,
server inference or database authority for Agents. The concrete costs are one
Place-scoped revision in exact local reads, one expected revision in the action, one
immutable accepted request fingerprint and a shared Place-lock discipline for every
operation that changes that representation.

The User accepted this research recommendation in the active design grill. It does
not become current executable behavior until the complete draft plan is accepted and
`game/docs/` is updated before implementation.

## Sources

- [RFC 9110: HTTP Semantics — validators and conditional requests](https://www.rfc-editor.org/rfc/rfc9110.html#section-13)
- [IETF HTTPAPI Idempotency-Key draft 07 — expired work in progress](https://datatracker.ietf.org/doc/html/draft-ietf-httpapi-idempotency-key-header-07)
- [RFC 9562: Universally Unique IDentifiers](https://www.rfc-editor.org/rfc/rfc9562.html)
- [AWS Builders' Library: Making retries safe with idempotent APIs](https://aws.amazon.com/builders-library/making-retries-safe-with-idempotent-APIs/)
- [Stripe: Idempotent requests](https://docs.stripe.com/api/idempotent_requests)
- [PostgreSQL: Transaction isolation](https://www.postgresql.org/docs/current/transaction-iso.html)
- [PostgreSQL: SET TRANSACTION and exported snapshots](https://www.postgresql.org/docs/current/sql-set-transaction.html)
- [PostgreSQL: Explicit locking](https://www.postgresql.org/docs/current/explicit-locking.html)
