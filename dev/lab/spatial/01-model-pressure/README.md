---
question: Which of four temporary spatial model shapes best explains all thirteen fixed scenarios without conflating exact Position, authored meaning, movement, topology, visibility or remote causality?
verdict: supported
status: kept
real_seam: []
simulated_seam: [World authority, PostgreSQL schema and queries, Position resolution, geometry, spatial indexes, Movement, Relation storage, visibility and access, Activity, HTTP, MCP, Agents, LLMs, concurrency, overload and operations]
informs: dev/areas/place/README.md#not-yet-chosen
---

# Spatial model pressure test

> **Role / side:** retained semantic spatial-model experiment / development side.
> **Authority:** records the fixed candidates, paper fixture, observations, bounded verdict and non-claims for this comparison.
> **Excludes:** accepted spatial vocabulary or behavior, production schema, executable evidence and sourced general findings; those remain in `dev/CONTEXT.md`, `game/docs/`, accepted plans, runtime surfaces and `dev/docs/research/`.

## Pending decision

The open Place frontier must explain when “two centimetres above the table” is an
exact Position, an Agent-authored spatial statement or both. It must also survive
travel, hidden carried state, a remote button, derived descriptions, progress,
nested placement, terrain, moving carriers, privacy, topology, co-location and map
roles. Choosing from the cup alone repeatedly blurred those boundaries.

This lab uses the Place Area's
[`spatial scenario catalogue`](../../../areas/place/scenarios.md) under the accepted
[`spatial-model pressure-test plan`](../../../plans/20260818-225124-spatial-model-pressure-test/plan.md).
It cannot accept its own winner.

## Falsifiable question

Does one candidate give each scenario a bounded, retrievable and changeable current
state without:

- forcing an Agent to fabricate an exact point that is not known;
- letting prose grant movement, access, visibility or remote causality;
- duplicating the same canonical truth in several places;
- rewriting every descendant when one carrier moves;
- requiring one generic store whose invariants depend on arbitrary predicates; or
- making a guessed Entity identity reveal hidden state?

A candidate is the leading direction only if its remaining gaps are honest separate
mechanics rather than contradictions inside its spatial foundation.

## Compared candidates

These labels and shapes are experiment-local.

### Candidate A — one absolute exact point

Each positioned Entity has one exact point directly in a World-wide coordinate
space. Spatial descriptions are calculated from exact points and geometry or remain
outside current World state. Moving one Entity changes only its own point unless an
Agent explicitly submits every other moved Entity.

This is the smallest coordinate model and the negative control for relative and
authored meaning.

### Candidate B — one broad Position record

Each Entity has at most one Position record. The same record may contain an exact
point, a reference Entity, free description and open Properties such as distance
and unit. One row and one read try to capture both mechanical location and the
Agent's wording.

This tests the attractive “keep everything spatial together” shape.

### Candidate C — exact Position plus separate meaning and mechanics

Each Entity has at most one exact Position keyed by its Entity identity. For this
test only, that Position may be grounded directly in World or relative to exactly
one Entity through a bounded acyclic chain. A relative Position persists when that
Entity moves. Open Agent-authored spatial meaning is optional separate current
state. Area, direct topology, visibility/access, inventory-like rules and remote
causality stay separate mechanics and are introduced only when a scenario earns
them.

One Agent proposal and one World read may compose several of these facts; separate
storage never requires separate player conversations or network calls. The
candidate does not choose coordinates, geometry, field names or table layout.

### Candidate D — generalized spatial facts

An Entity may have multiple generic spatial facts such as `at`, `inside`, `above`,
`under`, `near`, `connected`, `held`, `distance` or novel Agent-authored predicates.
Facts share one endpoint/value machinery; mechanics branch on predicate or fact
shape when World must enforce behavior.

This tests maximum expression through one extensible substrate.

## Evaluation rules

- **Pass:** the candidate represents the scene without false certainty, duplicated
  canonical truth, prose inference or unbounded work.
- **Qualified:** the foundation remains coherent but the scene honestly requires a
  separate mechanic, geometry or unresolved rule.
- **Fail:** the candidate cannot preserve the scene without changing its own meaning,
  hiding required semantics in prose or paying unbounded correctness work.

The score is explanatory pressure, not arithmetic. A candidate does not win by
pretending that every distinct mechanic is the same spatial fact.

## Scenario matrix

| Scenario | Candidate A | Candidate B | Candidate C | Candidate D |
| --- | --- | --- | --- | --- |
| **SP01 · A to B** | **Qualified.** Exact endpoints can be points, but proximity cannot establish direct travel, discovery versus entry or an allowed direction. | **Qualified.** Free Position text adds no deterministic traversal basis and may go stale during travel. | **Pass.** Position records the mover's current point; separate direct topology is used only when required; intermediate unnamed Positions remain possible. | **Fail.** A generic `connected` fact becomes typed topology as soon as direction, access and traversal invariants matter. |
| **SP02 · Entity in coat** | **Fail.** One absolute point cannot express hidden carried state or move with the coat without descendant writes or another mechanic. | **Qualified.** “In coat” fits the row, but its words cannot decide movement, privacy, access or exact inner point; row-wide visibility couples them. | **Qualified.** A relative Position can support movement when an exact local point exists; an open statement can preserve meaning; private contents and removability still require an earned mechanic. | **Fail.** A generic `inside` edge cannot safely imply movement, listing, privacy, access and cycles without predicate-specific rules. |
| **SP03 · Remote bomb** | **Qualified.** Positions locate endpoints but correctly do not authorize causality. | **Qualified.** Co-locating “remote trigger” text with Position is irrelevant and unsafe as executable meaning. | **Qualified.** Spatial Position remains coherent and separate, but the scene still requires a later explicit remote capability naming endpoints, authority and consequences. | **Fail.** A generic `activates` edge either remains inert or becomes a separately typed remote capability. |
| **SP04 · Cup above table** | **Qualified.** It can store one exact cup point if the required coordinate and surface data are already known, but not the authored wording or persistent table-relative movement. | **Qualified.** It stores the phrase and numbers conveniently, but cannot tell which fields are calculable, whose authorship is current, or what must change when the table moves. | **Pass.** If reference, surface and point data resolve one exact point, the two-centimetre offset participates in Position; otherwise the statement remains authored meaning without counterfeit coordinates. Relative movement is explicit in the tested Position form. | **Qualified.** It can state everything, but `above` and `distance` do not supply coordinate, surface, movement or geometry semantics without typed interpretation. |
| **SP05 · Dog under bridge** | **Qualified.** Exact geometry may derive “under,” but absent geometry leaves no current expression. | **Qualified.** The row preserves “under,” yet dog movement makes row text stale and mixes observation provenance with Position. | **Pass.** World may derive the description from exact spatial facts or preserve an authored statement; moving the dog changes Position without silently rewriting historical observation. | **Qualified.** It expresses `under` but cannot determine whether that fact is derived, asserted, expiring or mechanically relevant. |
| **SP06 · 100 m to hotel** | **Qualified.** It can calculate straight-line distance between exact points, not path distance, Area arrival or an authored estimate. | **Qualified.** Storing `100 m` in Position creates immediately stale duplicate state and still does not identify the measurement. | **Pass.** Calculate distance on read when Position, geometry or path basis makes it deterministic; otherwise retain it only as an authored estimate. Route and arrival stay separate. | **Qualified.** A distance fact is easy to add but its endpoints, basis, expiry and recalculation remain predicate-specific. |
| **SP07 · Birdhouse on table** | **Fail.** Moving the table requires rewriting the birdhouse or leaving its absolute point stale; nested village grounding is copied or absent. | **Qualified.** References can encode nesting, but one mixed record couples exact state, authored “on,” visibility and movement lifecycle. | **Pass.** A bounded relative Position grounds the birdhouse through the table without copying every ancestor; optional authored meaning remains separate; concurrent reparenting has exact subjects. | **Qualified.** Generic edges can form the chain but need one-parent, cycle, movement and privacy rules that no arbitrary predicate provides. |
| **SP08 · Forest to heath** | **Qualified.** Character points work, but terrain and incomplete boundaries do not follow from points alone. | **Qualified.** Character Position prose is the wrong lifecycle and index for shared forest or heath coverage. | **Pass.** Character Position, optional Place Area and authored terrain meaning remain distinct; unknown or overlapping boundaries stay honest. | **Qualified.** Generic region facts represent claims but metric coverage, boundary completeness and overlap queries need specialized state. |
| **SP09 · Moving ship** | **Fail.** One carrier move forces unbounded descendant rewrites or makes passenger points stale. | **Qualified.** Entity references can avoid rewrites, but broad rows couple interior description, visibility and exact Position and still need depth/cycle rules. | **Pass.** One carrier Position changes; bounded relative chains resolve cabin, passenger and cargo points; interior mechanics can ignore external movement unless their operation depends on it. | **Qualified.** A generic graph avoids rewrites only after acquiring parent cardinality, cycle, depth, transform and operation-dependency rules. |
| **SP10 · Hidden sword** | **Fail.** Hiding one absolute point does not express current backpack grounding, holder listing, memory or removability. | **Qualified.** Hiding the whole broad Position row may work, but couples exact point, authored meaning and access; one leak exposes every layer. | **Qualified.** Position and authored meaning can be hidden independently; memory remains non-current; a future inventory mechanic must earn bounded listing and removal rules. | **Fail.** Generic endpoint indexes and reads risk disclosing hidden facts, while per-predicate privacy recreates typed mechanics inside one store. |
| **SP11 · A connects to B, not C** | **Fail.** Distance or coordinates cannot prove direct topology. | **Fail.** Position description cannot safely authorize travel and mixes two-Place state into one subject row. | **Pass.** Direct topology remains its own directed fact; physical road, door or bridge Entities and later Route state stay separate. | **Qualified.** A generic `connected` fact works only after direction, endpoint role, duplicate and traversal rules make it a typed topology member. |
| **SP12 · Shared unnamed point** | **Qualified.** Independent exact points permit co-location, but visibility and hot-page admission remain separate and scale is unmeasured. | **Qualified.** Co-location works, but broad Position rows increase unrelated conflicts between wording and movement and complicate visibility filtering. | **Qualified.** Each Entity retains its own Position and equality creates no shared owner, but bounded indexed reads, admission and hot-point behavior still require technical evidence. | **Fail.** Generic fact indexes around one hot point or predicate create mixed traffic and do not supply exact spatial indexing for free. |
| **SP13 · City, forest, waterfall** | **Qualified.** Points put subjects on a map but cannot decide Place role or Area coverage. | **Qualified.** Free Position fields invite role meaning and map visibility into the wrong lifecycle. | **Pass.** Entity identity, Position, Place role and optional Area remain separate; discovery and map visibility can filter the bounded Place read. | **Qualified.** Generic facts can say `is_place` or `covers`, but role qualification and coverage invariants then become typed again. |

## Cross-cutting observations

### Agent input

**A** forces exact coordinates even when the Agent knows only “inside the coat” or
“under the bridge.” It either invents false precision or loses the observation.

**B** is initially pleasant: one flexible Position payload accepts coordinates,
reference, phrase and Properties. The ambiguity transfers to every later Agent,
which must guess which pieces are calculable, mechanical, current, private or merely
authored.

**C** asks an Agent to distinguish intent, not technical tables: establish or change
one exact point when known; optionally preserve authored meaning; and explicitly
name the separate mechanic an Action depends on. These can be one confirmed package.
It costs more explicit input only when the player actually intends more than one truth.

**D** accepts nearly any statement but makes each Agent understand an open graph and
which predicates secretly have server behavior. The apparent freedom becomes the
largest public semantic surface.

### Stored current truth

**A** is clean but incomplete. **B** gives facts with different authorship,
visibility, invalidation and indexing one shared revision. **D** gives heterogeneous
facts one storage family while moving their differences into conditional rules.

**C** keeps one home per truth: one current exact Position, optional authored
meaning, and only those separate mechanical facts that current gameplay earns. It
does not require every Entity to carry every layer.

### Reads and visibility

**A** offers the cheapest addressed Position read but cannot answer most relational
scenes. **B** offers one row but cannot safely expose only its non-private parts or
separate an author's statement from authoritative exact state without internal
subdocuments and policies.

**C** may join several storage records behind one bounded World operation; an Agent
does not pay one network call per table. Type- and subject-bounded reads can apply
visibility before hydration. **D** needs predicate-aware filtering over shared
endpoint indexes and risks disclosing that hidden facts exist.

### Movement and invalidation

**A** fails the ship and nested-placement pressure because correctness work grows
with descendants. **B** can introduce a reference but still must decide which free
fields move, expire or conflict with the exact point.

**C** changes one carrier Position and resolves a small acyclic chain. Authored
meaning is not silently rewritten: an Agent may update or supersede it when its
meaning changes. A derived statement is recalculated rather than stored as duplicate
current truth. **D** needs predicate-specific propagation and cycle behavior and
therefore stops being generic where correctness matters.

### Concurrency, history and hot subjects

**A** has small individual rows but unbounded multi-subject carrier mutations. **B**
makes an exact move conflict with unrelated description edits on the same broad row.
**D** mixes hot endpoint and predicate traffic and requires conditional lock,
uniqueness, privacy and history rules.

**C** admits exact per-subject dependencies: changing a Position conflicts with that
Position and the bounded references needed to resolve it, not every fact on the
Entity or Place. Position and authored meaning can leave distinct attributable
history in one transaction when submitted together. This is a logical advantage,
not measured PostgreSQL evidence.

### Massive-concurrency pressure

Only **C** simultaneously avoids descendant rewrites and a universal heterogeneous
fact index while keeping quiet subjects independent. It still has real hot-root and
read problems: relative-chain depth, cycle prevention, external spatial indexing,
carrier revision dependencies, pagination and admission are unresolved. The paper
test cannot prove their throughput, latency or production feasibility.

## The cup statement, resolved more precisely

The phrase “two centimetres above the table” is not assigned permanently to one
model concept by its grammar.

- It contributes to exact Position **when** World has a named reference, a known
  surface/direction and enough remaining coordinates or geometry to resolve exactly
  one current point.
- It remains authored spatial meaning **when** those structural inputs are absent or
  the Agent is preserving an observation rather than establishing mechanics.
- Both may be submitted together when the Agent intends one exact mechanical point
  and one human-meaningful current description.

The distinction is deterministic completeness, not whether the sentence sounds
more like “position” or “relation.” World never fills the missing structure by
interpreting the phrase.

## Verdict

**Verdict: `supported`.** Candidate C is the strongest direction within this paper
fixture. It passes nine scenarios and is honestly qualified on SP02, SP03, SP10 and
SP12 where inventory/access, remote causality, privacy or measured hot-point behavior
must remain separate. No failure requires changing its Position meaning.

Candidate A is a useful exact-coordinate kernel but not a complete foundation for
moving and authored spatial play. Candidate B minimizes visible rows by combining
facts whose authorship, visibility, invalidation, concurrency and indexes diverge.
Candidate D maximizes expression but recreates typed mechanics through predicate
branches and conditional invariants, making the generic substrate the cleverest and
least bounded option.

The supported direction is therefore **one exact Position per positioned Entity,
with relative grounding as a candidate representation, optional authored spatial
meaning kept separately, and distinct mechanics for topology, access/private
contents and remote causality**. This sentence is a lab verdict, not accepted
Aicadia vocabulary or behavior.

## Remaining falsifiers and open choices

Candidate C should lose its leading status if a later bounded model or experiment
shows any of these:

- an exact Position cannot support both direct and relative grounding without two
  competing current truths;
- bounded resolution and cycle prevention require a global lock, unbounded query or
  descendant rewrite;
- a useful addressed or nearby read cannot hide authored/private layers without
  leaking their existence;
- separate authored meaning necessarily creates stale duplicate mechanical truth;
- a Place cannot retain coherent map identity when its Position is relative; or
- the Agent-facing package is materially harder to understand than Candidate B
  after both expose the same behavior.

The experiment does not choose:

- coordinate, cell, units, precision, orientation, surface or geometry representation;
- whether every relative Position moves with its reference, or whether another explicit rule is needed;
- which Entity or World reference may ground a Position;
- maximum reference depth, cycle algorithm, locking, revision or index strategy;
- Position, authored-meaning or Relation visibility and public read shapes;
- inventory, wearing, holding, ownership or access mechanics;
- whether a Place may have relative Position;
- the final name and scope of Connection or any later Route; or
- the first production capability that earns this model.

## Real and simulated seams

No implementation seam is real. The fixture is a reproducible paper comparison over
fixed repository scenarios and existing research constraints. World authority,
PostgreSQL schema and queries, Position resolution, geometry, spatial indexes,
Movement, Relation storage, visibility/access, Activity, HTTP, MCP, Agents, LLMs,
concurrency, overload and operations are simulated or absent.

No token, model, external service, database, network or runtime call was used.

## Non-claims

This lab does not prove production correctness, database integrity, bounded query
plans, privacy enforcement, Agent comprehension, movement semantics, deadlock
freedom, throughput, latency, overload behavior or million-User scale. The pass and
qualified labels mean only that the candidate remains conceptually coherent under
the fixed narratives and stated constraints.

**Artifact status: `kept`.** Its candidate labels and model shapes remain
experimental and may not be imported, copied or promoted directly into production.

## Downstream implication

The next grill should test Candidate C's first unresolved product boundary rather
than choose a schema: whether an exact Position may be grounded directly in World or
relative to exactly one Entity, and whether relative grounding itself means that the
positioned Entity moves with its reference. That answer precedes coordinates,
chains, indexing and public read design.
