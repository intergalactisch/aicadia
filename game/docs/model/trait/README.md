---
kind: state
storage_table: [entity_trait, entity_trait_version, entity_trait_current]
---

# Trait

> **Role / side:** Trait model contract / runtime side.
> **Authority:** Trait statement shapes, normalization, establishment/development packages and their rejection behavior.
> **Excludes:** Entity subject rules, Property values, Activity shape and delivery status; see the Entity, Property and Activity contracts and `dev/docs/evidence/`.

```text
TraitInput { statement }
EntityTrait { id, statement }
EntityTraitChangeInput =
  { type: "establish", entity_id, statement } |
  { type: "develop", trait_id, statement }
ActivityTraitChange =
  { type: "establish", entity: EntitySummary, trait: EntityTrait } |
  { type: "develop", entity: EntitySummary, trait: EntityTrait,
    previous_statement }
```

Statement normalization trims outer Unicode whitespace, rejects U+0000 and validates
1–4,000 Unicode characters. It preserves internal whitespace, case, punctuation and
code points. Exact duplicate/no-op comparison uses only that stored trimmed value;
World performs no Unicode folding or semantic comparison.

Creation `trait` contains 0–100 establishment statements. Action and Interaction
`trait_change` contain 0–100 items; a `change_entity_state` Action requires its
Property or Trait list to be non-empty. Within one mixed list, duplicate establishment
`(entity_id, normalized statement)`, duplicate development `trait_id`, development
to that Trait's exact current statement or any duplicate exact statement in the
intended post-package active set for one Entity returns `invalid_trait`. The final-set
rule rejects development into another unchanged active statement, two developments
to the same statement and an establishment plus development to the same statement.
A statement vacated by another development in that same unordered package may be
reused because it is unique after the complete package; input order never changes
the result. Every such failure rejects the whole creation, Action or Interaction
atomically, including any Property changes, Activity, role or participation state.
Semantic near-duplicates and contradictions are accepted. A well-formed missing,
remote, departed, stale or otherwise ineligible Entity/Trait uses neutral
`trait_unavailable` and exposes no role/control/existence distinction.
