# Agent-mediated World action

Status: Done

## Outcome

A User consciously starts the next game action. The Agent reads established World
and current Character state, offers exactly three grounded directions, incorporates
the User's selection and optional steering, and prepares one final package. World
accepts or rejects that package as one atomic action and returns the complete accepted
result.

The completed build plan is
`.agents/plans/20260811-124550-first-agent-mediated-world-action/plan.md`. Current
behavior and the Agent contract are authoritative in `docs/game/README.md` and
`docs/game/agent-interface.md`; bounded live evidence is recorded in
`docs/game/agent-playtest.md`.

## Confirmed direction

- proposals, rejected directions and User steering remain private Agent conversation;
- after direction selection and steering, the Agent shows the exact final prose and
  one `introduce_entity` consequence; one explicit User confirmation is required before the
  irreversible submission to World;
- the final submission binds readable prose, one Entity name and description and
  provenance; World never derives mutations by interpreting prose;
- accepted prose is immutable and append-only; every World, Character, Place and
  Entity lens must reference the same chronological source rather than copy or edit
  narrative text;
- World acceptance establishes the only current chronology; Agents cannot backdate,
  reorder or insert prose into earlier history;
- the ordinary flow uses one mutating submission even though the Agent may perform
  several read-only MCP calls while orienting and drafting;
- the Agent composes its working context through separate typed MCP reads for World,
  Character, exact current Place Entity state and exact current Place Activity/prose;
  there is no monolithic `local context` resource;
- spatial surroundings are deferred: a later bounded Place neighborhood will use
  explicit containment and adjacency, not a literal coordinate or metric radius;
- MCP may also expose granular domain capabilities when an individual operation is a
  valid complete action by itself;
- every granular or bundled mutation is a command to World; an Agent never receives
  direct storage access or creates, updates or deletes durable state itself;
- a bundled submission is one World transaction and one Activity footprint, not a
  sequence of public MCP writes with partial success;
- one Agent-created request UUID, a World-derived fingerprint and an opaque
  exact-Place revision separate delivery retry identity from observed-state
  freshness;
- HTTP and MCP remain thin adapters over the same complete World behavior; later
  capabilities must preserve that parity rather than freeze this slice's count.

## Current delivery boundary

No material product, domain, interface or concurrency choice remains. The accepted
retry/freshness contract uses an Agent-created request UUID, a World-derived normalized
fingerprint and an opaque exact-Place revision validated under a Place-row lock; see
`docs/research/idempotent-action-delivery-and-place-freshness.md`. The first slice
deliberately proves the complete interaction at the existing exact entry Place;
bounded containment and adjacency are the next spatial edge, not hidden scope in
this build. The bundle must remain a domain action rather than an unrestricted
database-patch language.

The accepted evidence order keeps the minimal three-read observer, assigns full
Entity-description validation only to the
authoritative HTTP layer, and runs that HTTP validation immediately after commit so
its result is retained before Agent interpretation. Both HTTP and observer must still
pass in one candidate. Candidate `run-gE8iED5m` passed that exact order.

The selected first evidence scenario is one trail-marker action: World derives the
entered Character's current Place, accepts one package containing readable prose and
one Entity consequence, atomically creates and places the marker with one Activity,
and exposes marker and prose to another Character at that Place.

## Completion evidence

The deterministic ladder passes 57 Rust tests. The fail-closed runner passes 27 fake
invocations and 19 failure modes, including duplicate or incomplete submission,
wrong actor, observer disagreement and unsafe cleanup. Public preflight pins the
exact CLI/model/reasoning, schemas, catalog, least-privilege roles and owned database
lifecycle.

Separately authorized `run-gE8iED5m` then proved one complete live loop: four
grounding reads, three private proposals, withheld selection and steering, a no-tool
preview, separately withheld confirmation and exactly one `submit_action`. World
accepted one marker Entity and one immutable Activity/prose at the derived Place.
Authoritative HTTP proved exact actor, Place, roles, description, prose and counts;
only then did a separate Agent find the same Entity id/name, Place and prose through
exactly three MCP reads.

Independent T4R4 review found no P0-P3 issue or evidence drift. All forty retained
artifacts are private, ownership-verified cleanup dropped the disposable database,
and zero database, process, listener or isolated-config leftovers remain. Exact run
ids, bounded claim and the two earlier non-passing candidates are recorded once in
[Agent playtest](../../../docs/game/agent-playtest.md#live-evidence-history).
