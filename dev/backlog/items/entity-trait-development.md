# Local Entity Trait development

> **Role / side:** forward-planning item / development side.
> **Authority:** records this outcome's backlog state, dependencies and completion pointers.
> **Excludes:** current product contracts, decision rationale and detailed delivery evidence; see `game/docs/`, `dev/docs/concept/log/` and `dev/docs/evidence/`.

## Outcome

Contextual confirmed Actions and Interactions can establish and develop concise
Entity-owned Traits with stable identity and immutable Activity-backed earlier
statements. Whenever an Agent actually fetches its Character or one exact-local
Entity, one bounded combined page carries current Property/Trait associations; the
Agent decides their relevance and World stores no observer-specific knowledge.

Delivery history and current status: see [Trait evidence](../../../dev/docs/evidence/trait.md).

## Confirmed direction

- Trait is a concise explanatory Entity characterization, not a Property, status,
  score or executable rule.
- A Trait can develop; User intent/confirmation, Agent exact authorship and World
  validation/write remain separate.
- Traits are never initial creation input; they arise contextually through accepted
  play.
- Action may uniformly affect actor, Place, ordinary Entity and other Character.
- Establishment creates a stable id; development appends immutable versions and
  advances one current pointer; retirement is deferred.
- The unique root version is the sole establishing Activity provenance; development
  names stable Trait id/new statement while expected Place revision plus the locked
  current pointer select its predecessor atomically.
- Both Action and Interaction may establish/develop through one private writer;
  Interaction actor and explicit targets are uniformly eligible and it never authors
  a target response.
- Every Entity owns its Property/Trait state. `get_character` and every actual fetch
  of another eligible Entity return correct current associations; the Agent decides
  relevance. No observer-specific Knowledge/Observation or Relationship state exists.
- `list_entity_at_current_place` remains compact; `get_character` and new scoped
  `get_entity_at_current_place` use one combined typed association page with one
  cursor, default 25 and maximum 100. Activity/mutation references stay compact. The
  new read replaces the standalone Property list for exactly thirteen player tools.
- One Action `trait_change[1..100]` and optional Interaction
  `trait_change[0..100]` may mix establishment/development; Interaction Trait and
  Property consequences may coexist atomically.
- Exact normalized active duplicates and no-op development reject; semantic
  contradictions remain accepted and one lineage supersedes only itself, with no
  inferred Property/description precedence.
- Only explicit confirmed Action/Interaction are executable causes; no external
  writer is delivered.
- Trait prose is non-executable.
- Every immutable statement is trimmed non-NUL PostgreSQL `text` of 1–4,000 Unicode
  characters; richer causal story remains Activity prose.
- Agent authors and fully previews the exact natural Trait consequence; User accepts
  or rejects the whole package and receives no direct Trait editor.
- Every accepted mutation must retain immutable Activity provenance.
- Trait is the selected edge after delivered Property state and before queued
  investigation/discovery.

## Delivery boundary

The static evidence pointer above owns this record.

## Dependencies

- delivered Character-grounded Interaction and knowledge;
- delivered Local Entity Property state and thirteen-capability player contract;

## Non-goals

Initial creation Traits, retirement/reactivation, semantic inference, mechanics,
direct storage editing, automatic all-local visibility, remote/global Trait access,
relations, movement, unaccepted autonomous factors, background Agents and changes to
historical live-evidence byte content.

## Completion evidence

The static evidence pointer above owns this record.
