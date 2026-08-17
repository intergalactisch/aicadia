---
status: Deterministic creation and combined Action delivery complete across World, HTTP and MCP; no new paid/model call
---

# Uniform Entity-state package evidence

> **Role / side:** uniform Property/Trait creation and Action delivery record / evidence bridge.
> **Authority:** owns the deterministic delivery status and exact proof for uniform Entity-state packages.
> **Excludes:** current game semantics, historical paid Property/Trait candidates and runner operation; see `docs/game/`, `property.md`, `trait.md` and `runner/`.

## Delivered outcome

Every Entity creation route now accepts independent `property[0..100]` and
`trait[0..100]` lists. One `change_entity_state` Action accepts independent
`property_change[0..100]` and `trait_change[0..100]` lists with at least one item.
Each accepted call writes one Activity and either commits its complete Entity, role,
Property and Trait state or rolls everything back. Properties and Traits retain
their separate validation, history and current-state models.

Migration `0009_uniform_entity_state.sql` adds no table or index. It admits Trait
roots from the exact creation and current mutation Activities, retains immutable old
Action discriminator rows and rejects new Trait writes through superseded tags. The
public old Action variants are absent. Current Property-only and Trait-only retry
fingerprints remain byte-identical to their historical semantic forms; stored old
rows decode to the current combined output, while genuinely combined state and
non-empty initial Traits use the new length-prefixed components.

## Deterministic proof

The PostgreSQL-backed World suite passes 82/82. Its focused proof includes:

- all four creation routes with initial Trait roots, one 100-Property plus 100-Trait
  creation Activity, normalized duplicate rejection and forced-storage rollback;
- Property-only, Trait-only and mixed `change_entity_state`, one subject-role union,
  one location, Property-first invalidation, both-empty rejection and no partial
  writes;
- 100 Property plus 100 Trait Action changes, reordered semantic retry, raw
  historical Property/Trait retry reconstruction and reverse-order concurrency
  without deadlock; and
- unchanged Interaction, lineage/current pointer, local eligibility, pagination,
  revision and Activity-history behavior.

The HTTP/MCP suite passes 14/14, including all four creation routes across both
adapters, combined state creation/readback, strict rejection of the old public tags
and exact current errors. The lib/Agent-contract suite passes 21/21. The runtime MCP
catalog and checked-in fixture are equal and contain exactly the same thirteen
tools; no fourteenth capability was added. The complete all-target Rust run passes
119/119, and Clippy across all targets/features passes with warnings denied.

Both permanent playtest fake suites pass token-free. The current Trait runner digest
is `1720e1052cf2a2d4823395f7c667132a61f97b5aab5031fa347be6c55d344f34` and binds the
current `change_entity_state` catalog and fake commit shape. The consumed sentinel,
private historical candidates and their manifests were not edited. This build made
zero paid/model calls and claims no new live-model evidence.

## Sol-medium smoke result

An initial controller-heavy `gpt-5.6-sol` medium attempt made one model process call
and correctly performed all four grounding reads, but its proposal prompt asked only
for a shared non-null revision while the hidden validator required a byte-exact copy.
The model returned `shared-non-null-confirmed`; that prompt/validator mismatch made
the attempt inconclusive, not negative model evidence. It stopped before mutation
and its owned database was dropped.

The User then narrowed the question to the combined call itself. One direct
Sol-medium process called `get_world`, `get_character`,
`list_entity_at_current_place` and `list_activity_at_current_place`, copied the exact
Place revision, and called `submit_action` exactly once. Its
`change_entity_state` contained both `leg_count = 3` and one Trait establishment.
World accepted Activity `afb71711-eecd-404e-98ac-70c9e3c726bb` and assigned Trait
`3409bebb-5263-442d-ab48-d7370f01aded`. Independent HTTP reads proved the Property,
Trait, prose, subject and location in that same Activity.

The successful process used 89,172 input tokens, 54,272 cached input tokens, 590
output tokens and 157 reasoning-output tokens. Ownership-verified cleanup dropped
the disposable database; a direct database check returned zero matching databases.
This proves the narrow claim that one pinned Sol-medium Agent understood and executed
the current combined Property/Trait MCP call after explicit confirmation.

## Boundary

No discovery, movement, new Place, Trait mechanic, score, generic patch operation,
new relation, background writer or server-side inference is delivered.
