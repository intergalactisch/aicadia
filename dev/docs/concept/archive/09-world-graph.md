> **Archived:** July-2026 generation whose scene/claim vocabulary predates the 2026-08-07 game reframe; the concept log records which decisions were superseded; individual ideas may still inform future direction.

# The world-graph

> **Concept log** — we're still discovering. Exploration, not specification.

How everything connects: the nodes, the edges, the lenses over them, and why this
stays fast, powerful, scalable and dynamic without exotic infrastructure.

## The shape in one paragraph

The world is a **provenance-carrying, time-versioned graph**: entities are nodes,
claims are edges, and every claim points back to the scene that established it.
Accepted source packages are never updated in place. Claims accrete; a later claim
may replace or coexist with an earlier one under the applicable projection contract.
"Current state" is the deterministic result of those contracts, not a status stored
on source claims. The tree, map, timeline, social graph and lexicon are all
rebuildable projections of the one accepted source log.

## Nodes: `entity`

Anything that exists: a person, a place, a house, an oak, a plant species, a dish, a
song, a word, a profession, a material, a myth. Deliberately thin:

| column | meaning |
|---|---|
| `id` | immutable opaque id; every persisted world reference uses it |
| `created_in_scene_id` | provenance: the scene that brought it into the world |

There is no source `kind` string or fixed kind enum on `entity`. A kind is itself an
ordinary entity with a stable id. `instance-of` and `subtype-of` claims define how
other entities use it. Its names, gloss, parent kinds and properties are claims too.
Everything else about an entity is also claims. Human names are not identity and are
not stored on `entity`. (Refinement of doc 08: no JSONB attribute bag — claims-first
is one mechanism instead of two, and Terry says flat.)

`verification_status` is not a universal source field on every entity. A discovered
place may have an `unverified | verified` result in its rebuildable
place-verification projection, produced by that projection's specific presence rule.

## Names: claims, never reference keys

A current name, former name or alias is a provenance-carrying claim about an entity.
A rename appends a new name claim and supersedes the previous current-name claim. It
never changes the entity id or any relationship referring to it.

Every accepted claim references existing entities and predicates by immutable id:

```text
subject_id: <entity id>
predicate_id: <predicate id>
object_id: <entity id>
```

A name string may be used to search. Search returns candidate ids with current name,
classification ids and labels, location and gloss so the agent can select the
intended entity. The accepted write uses that id; unresolved name strings are not
accepted as references.

Duplicate current names are structurally safe because identity is the id. Whether a
narrative naming rule should discourage confusing duplicates remains a separate
world-design question.

A rebuildable `entity_label` projection supplies the current display name, current
one-line gloss and their source claims for lists, briefings and search. Former-name
and alias claims remain searchable without becoming current labels.

## Edges: `claim`

A claim is an immutable, individually addressable triple inside an accepted scene
package:

| column | meaning |
|---|---|
| `id` | immutable claim id |
| `subject_id` | entity |
| `predicate_id` | from the predicate vocabulary (below) |
| `object_id` / `object_value` | another entity, or a literal (a quantity, a date, a phrase) |
| `ordinary_scene_can_supersede` | whether an ordinary later scene may replace this claim |
| `scene_id` | the scene that established this claim — every fact is traceable |
| `effective_at` | when it applies in world time |

Acceptance time belongs to the containing scene. Every accepted claim is immediately
queryable. Its immutable package also contains agent-authored source basis and
references to accepted source claims; the exact provenance fields or relation remain
open. A later package may cite, repeat, observe, contradict or propose a replacement,
but it never changes an earlier claim row. Time-versioning comes from accepted
claims, world-effective time and projection-specific replacement rules. "The baker
used to live at the mill" remains source history after an authorized move causes
`entity_location` to select a later claim. The archive really is the only scoreboard.

## Three structural relation families

All predicates are equal, but three families carry the world's skeleton:

1. **`instance-of` / `subtype-of` — the type layer.** A kind is not a string stored
   on its instances. It is an entity used as the object of `instance-of` or as either
   side of `subtype-of`; no `is_kind` flag is needed. "Vlierberk" (an invented tree
   species) is an entity; *the old vlierberk at the mill* is another entity with
   `instance-of → vlierberk`. Same for `longhouse` (a house type someone coined) vs
   *Merel's longhouse*. Species, house types, professions and dish types all emerge
   as entities, and the type layer itself can deepen
   (`vlierberk subtype-of tree`) without any schema change.

   `subtype-of(A, B)` has one strict meaning: every instance of A is also an
   instance of B. A kind may have several direct parent kinds when that sentence is
   true for each parent. There is no primary-parent field, required single tree or
   mandatory root kind.

   A classification query may traverse these direct claims to calculate indirect
   kinds, ancestors and descendants. The result distinguishes direct claims from
   calculated paths and returns the path depth plus every source claim id. No
   indirect `instance-of` or `subtype-of` claim is appended to the scene log.

   The traversal may include every applicable accepted direct classification claim
   so a newly discovered kind is immediately queryable. Every returned edge retains
   its source claim id and provenance. A caller may apply an explicit provenance
   filter once that vocabulary is fixed; traversal itself never promotes a source
   claim or turns a calculated path into accepted source truth.

   Nature, material and physical phenomena use this same type layer. The server does
   not ship a catalogue of materials, species or phenomena. An accepted scene
   introduces an ordinary entity and authored claims establish its name,
   classification and observed properties. Its id remains stable; its definition is
   the time-versioned, sourced set of claims and can therefore grow, be contested or
   be superseded. Queries retrieve kinds through their ids and current labels from
   these claims, rather than through a free-text field on each instance.

   The first accepted package that introduces such an entity allocates its permanent
   id immediately. The same package may introduce the kind and its first observed
   instance. The package's name, classification and property claims are separate
   accepted claims with their own provenance; creating the entity does not endorse a
   complete definition. Later packages can cite the id immediately and support,
   contest or propose replacements for individual claims. A kind does not require a
   pre-existing parent kind.

   Both the reusable category and a concrete occurrence are normal entities and may
   reference each other by id. For example:

   ```text
   <blue reed smoke id> subtype-of <smoke id>
   <plume id> instance-of <blue reed smoke id>
   <plume id> emitted-by <kiln id>
   ```

   The plume may have changing location and extent claims. Later scenes may revise
   its cause or add properties to blue reed smoke without replacing either id. There
   is no `material` table, fixed material enum or immutable definition record.

   A kind definition is descriptive and queryable, not a validation schema. If
   `vlierberk` has a claim saying its leaves are silver in winter, the server can
   return that claim when an agent queries the kind. It does not reject a particular
   vlierberk merely because that instance has no leaf-colour claim: missing
   information is not a contradiction. A kind claim never creates a validator. A
   mechanical requirement exists only as a versioned `rule` backed by a named,
   deterministic validator.
2. **Composition, place and current location.** Accepted predicates may establish
   that one entity is structurally part of another or spatially related to a place.
   Only relationships the world actually establishes are stored; there is no
   required `region → city → district → village → block` ladder. Explicit place
   relations may be materialized as a Postgres `ltree` path for fast subtree queries.
   They do not need to contain every physical entity.

   Every current physical entity has one rebuildable `entity_location` row:

   ```text
   entity_location
     entity_id
     place_id nullable
     geometry nullable
     place_edge_id nullable
     source_location_claim_id nullable
     source_geometry_claim_id nullable
     source_journey_scene_id nullable
   ```

   At least one of `place_id`, `geometry` or `place_edge_id` is present. A house known
   only to be in a village uses `place_id`; an exactly mapped house may use geometry
   without a parent place; a traveller uses `place_edge_id`. `place_id` and geometry
   may coexist when both are established. The exact predicate vocabulary and
   validation of other field combinations remain open. The server also needs a
   deterministic way to know when the physical-location invariant applies without
   treating the claim-defined kind graph as a fixed physical taxonomy; that
   declaration remains open.
3. **Free domain predicates — everything else.** `works-as`, `built-of`,
   `roofed-with`, `grows-in`, `married-to`, `taught-by`, `named-after`, `eaten-by`…
   Unbounded, born in play.

The expressibility test from doc 08 runs entirely on these three families.

## The write contract: prose + claims

The server has no LLM, so who extracts facts from a scene? **The agent that wrote
it.** A scene submission is two-layered:

- **Prose** — the story, for humans and other agents to read.
- **Claims** — the structured assertions the scene makes, authored by the same agent
  that wrote the prose (it has the intelligence; the server doesn't need any).

These layers plus provenance are accepted atomically as one immutable source package.
Claims are not later re-extracted from prose and prose is not discarded after claims
exist. Other agents may read the source; the server queries and projects the
structured half.

The server validates the claims deterministically: briefing token fresh, scene
credit available, location plausible (map + travel time), cited entities exist
(connectivity), new names covered by naming rights, consent fields untouched
(volition/voice of others' characters), and every attempted current-state update
satisfies its projection's key, authority, effective-time and replacement rules
(below). Accept → insert scene + claims in one transaction → update projections →
fan out ripples. Milliseconds, index-bound, no queue required.

Every continued factual claim names the accepted scene or claim that supports it in
the submitted provenance. A direct action uses the current scene as its source. The
server checks that referenced ids exist and that mechanical ownership, location and
connectivity rules hold; it does not judge whether a citation is narratively
sufficient. That judgement belongs to the submitting agent before human confirmation
and remains challengeable afterward.

Concrete boundary: a package may store `built(character, marker)` and
`located-at(marker, clearing)` when the scene depicts that work. It may not store
`instance-of(clearing, city)` merely because the character announces a city. A package
may store that one character did not observe another at an appointment; it may not
change the other character's location or existence.

## Contradiction, honestly scoped

The server can only referee *structured* attempts to change a mechanically exclusive
current projection. That projection declares:

```text
key        the row that claims compete for
authority  the actor or rule allowed to change it
time       the world-effective time used for selection
replace    whether an eligible later claim replaces or coexists
```

For example, `entity_location` has one current row per located entity. A claim that
tries to move somebody else's character without authority is rejected with the
applicable rule slug. A sourced report saying that character was seen elsewhere may
still be accepted and queried, but it is not eligible to update `entity_location`.

Predicates and projections that permit several perspectives retain their competing
accepted claims. Contradictions in prose are handled by the challenge mechanism.
Don't pretend the server reads stories.

## The predicate vocabulary

Free-form predicates would fragment (`lives-in` vs `resides-at`). Direction:

- `predicate` table: slug, description and inverse name
  (`contains` ↔ `part-of`). A predicate that feeds a mechanical current projection
  names the applicable versioned projection rule; the exact storage field remains
  open.
- Reuse is surfaced: the briefing includes the vocabulary slice relevant to your
  region and current classifications, so agents converge on existing predicates
  (gazetteer pressure).
- Creating one is free but nudged: a deterministic `pg_trgm` similarity check flags
  near-synonyms and answers "did you mean `lives-in`?" — no LLM needed.

## The lenses (all projections, all rebuildable)

| Lens | What it is | How |
|---|---|---|
| Place hierarchy | explicit variable-depth place relations | optional `ltree` path over accepted place relations |
| Map | current and travel geography | PostGIS geometry + `entity_location` + `place_edge` |
| Timeline | history of anything | scenes + claims by time |
| Social graph | who relates to whom | character↔character claims |
| Lexicon | names, words, songs | name claims + entities of language kinds + gazetteer |
| Search | find anything | Postgres FTS over scene prose + label/gloss projections + former-name and alias claims |

## The inbox (this is all the "morning report" ever was)

When a scene lands, the server computes who it plausibly touches — characters
located in or adjacent to the scene's place, plus characters claim-linked to any
cited entity (owner, spouse, teacher…) — and writes a row per recipient:

`ripple(character_id, scene_id, weight, reason, created_at)`

That's fanout-on-write, the boring proven feed pattern. "Bad news travels faster"
is just the `weight` column. The **catch-up** is nothing more than
`ripples since last_seen` — a query, not a ceremony. Your agent reads the rows and
narrates them in your language; the server composes nothing. Locality bounds the
fanout: a village scene touches a village, never the world.

## Performance envelope

- **Reads** are local by construction: dossier, neighborhood, inbox, subtree — all
  single index scans or bounded recursive CTEs. Nothing ever scans the world.
- **Writes** are one transaction: scene + claims + ripple fanout (fanout size ≈
  local population).
- **Scale math**: 1M players × 1 scene/day ≈ 12 scenes/s; at ~8 claims/scene ≈ 3B
  claims/year — time-partition the claim table when it hurts, keep live (non-
  superseded) claims as the hot set. Append-only data is cache- and replica-friendly
  (immutable scenes, ETags). None of this is needed for a valley of 50; all of it
  has a known boring path at a million.
- **Dynamic without migrations**: new kinds, new predicates, new type layers are
  *rows*, not schema. The world grows richer daily; the schema stays still. That is
  the technical meaning of "organic meta-layers".
- **Why not a graph database**: our traversals are local and depth-bounded (a
  neighborhood, a subtree, a dossier). Two composite indexes on `claim`
  (`subject,predicate` and `object,predicate`) cover them. A graph DB earns its spot
  only for unbounded global traversals we don't have. Terry: boring wins.

## Open bits

- Narrative duplicate-name policy: storage permits two entities called "Molenpad"
  because references use ids; whether the gazetteer should discourage that within
  one locality remains open.
- Literal typing on `object_value` (quantity vs date vs phrase) — how strict?
- Exact fixed vocabulary and storage for direct action, direct observation, memory,
  report/citation and interpretation provenance.
- Exact representation of later claim references such as citation, contradiction
  and replacement.
- Which first-version projections, beyond place verification, need qualifying
  independent observation.
- Ripple recipients beyond locality and claim-links: subscriptions ("follow the
  harbor")? Probably later; locality first.
