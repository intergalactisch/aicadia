# Agent-mediated World action

> **Role / side:** forward-planning item / development side.
> **Authority:** records this outcome's backlog state, dependencies and completion pointers.
> **Excludes:** current product contracts, decision rationale and detailed delivery evidence; see `docs/game/`, `docs/concept/log/` and `docs/evidence/`.

## Outcome

A User consciously starts the next game action. The Agent reads established World
and current Character state, offers exactly three grounded directions, incorporates
the User's selection and optional steering, and prepares one final package. World
accepts or rejects that package as one atomic action and returns the complete accepted
result.

The completed build plan is
`.agents/plans/20260811-124550-first-agent-mediated-world-action/plan.md`. Current
behavior and the Agent contract are authoritative in `docs/game/README.md` and
`docs/game/agent.md`; the static evidence pointer is under Completion evidence.

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

The selected first evidence scenario is one trail-marker action: World derives the
entered Character's current Place, accepts one package containing readable prose and
one Entity consequence, atomically creates and places the marker with one Activity,
and exposes marker and prose to another Character at that Place.

## Completion evidence

Delivery history and current status: see [Action evidence](../../../docs/evidence/action.md).
