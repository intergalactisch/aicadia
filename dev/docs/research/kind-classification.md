---
status: historical
era: July scene-claim
---

> **Era:** July scene-claim research; its scene/claim vocabulary predates the 2026-08-07 game reframe.

# Kind classification, origin and lineage

> **Role / side:** sourced research report / development side.
> **Authority:** records this report's question, sources, findings and implications.
> **Excludes:** product decisions and current implementation contracts; see `game/docs/`.

Date: 2026-07-26

Status: research; strict `subtype-of`, multiple parent kinds, derived query paths and
source-linked traversal confirmed, remaining recommendations not confirmed

Decision follow-up: the universal claim evidence axis was retired after
`claim-support-and-current-state.md`. The classification decisions remain confirmed;
every traversed edge now retains its source claim id and provenance. Any stricter
source filter must be requested explicitly by the caller or projection.

Decision after research: `subtype-of(A, B)` means every A is also a B. A kind may
have multiple direct parent kinds when that statement holds for each parent. No
primary parent, single tree or mandatory root is required.

Decision after research: classification queries may calculate indirect
`instance-of` and `subtype-of` paths from direct source claims. Each result retains
its depth and ordered source claim ids and is identified as calculated query output.
It never becomes an accepted or synthetic claim.

Decision after research: classification traversal may include every applicable
accepted direct edge so a new kind is immediately queryable. Every returned edge and
calculated path retains its source claim ids and provenance. Traversal never changes
or promotes a source claim.

## Question

Are `instance-of` and `subtype-of` sufficient for kinds that are discovered,
described and revised inside Aicadia? Can the model support:

- millions of agents introducing kinds concurrently;
- one entity belonging to several kinds;
- several valid routes through a classification;
- changing classifications and names;
- biological or cultural descent;
- materials produced from other materials;
- parts and mixtures;
- competing reports; and
- fast, explainable queries without an ontology reasoner?

## Terminology correction

“Family tree” and “lineage” are too broad for the technical model. They combine
relationships that answer different questions:

| Question | Technical relation family | Example |
|---|---|---|
| What kind of thing is this? | classification | this plume `instance-of` blue reed smoke |
| Is every member of this kind also a member of another kind? | classification | blue reed smoke `subtype-of` smoke |
| What is this made of or contained by? | composition | the roof `built-of` reed |
| What produced or transformed into this? | origin | this smoke `emitted-by` this kiln |
| What organism or tradition did this descend from? | descent | dawnreed `descends-from` marsh reed |
| Is this another name or competing concept for the same thing? | identity or classification history | old taxon use refers to a later accepted taxon concept |

Use **classification graph** for `instance-of` and `subtype-of`.

Reserve **lineage** for historical descent or derivation. Do not use it for the
general kind hierarchy.

## Short answer

`instance-of` and `subtype-of` are the correct minimal classification pair. They
match RDF Schema, OWL and Wikidata and cover the two questions a classification must
answer:

```text
particular entity -> kind
narrower kind     -> broader kind
```

They become powerful enough for Aicadia when:

1. both ends are immutable entity ids;
2. only direct authored claims are source truth;
3. indirect ancestors and descendants are calculated by a query;
4. multiple direct kinds and multiple parent kinds are allowed;
5. every returned path includes its source claim ids and evidence;
6. definitions remain descriptive, not automatically enforced;
7. classification stays separate from part, origin and descent relations; and
8. cycles are detected deterministically.

The result is not normally a tree. It is a directed graph with possible multiple
parents and multiple paths. A strict mechanical classification projection may need a
directed acyclic graph, but accepted competing source claims can contain a cycle.
Exact cycle acceptance and strict-projection behavior remain a separate decision.

Adding OWL class expressions, cardinality restrictions or a general inference engine
would make the model formally stronger but operationally worse for Aicadia. It would
introduce hidden inferred facts, reasoning complexity and a second semantic system
beside scenes and claims.

## What established systems show

### RDF Schema: the minimal pair is sound

RDF Schema defines:

- `rdf:type`: a resource is an instance of a class; and
- `rdfs:subClassOf`: every instance of the narrower class is also an instance of the
  broader class.

`rdfs:subClassOf` is transitive. If `vlierberk subtype-of tree` and
`tree subtype-of plant`, a query can reach `plant` from `vlierberk`.

Implication for Aicadia:

- keep `instance-of` and `subtype-of`;
- define `subtype-of` strictly as “every current instance of A is also an instance
  of B”;
- do not use it for loose similarity, typical habitat, material source or
  composition.

### Wikidata: an entity can be both an instance and a kind

Wikidata uses `instance of` and `subclass of`. Its guidance explicitly permits one
item to be both an instance and a class. It determines the role from relationships,
not from a mandatory class flag. Wikidata also notes that characteristic properties
are not enforced on every instance.

Implication for Aicadia:

- the confirmed choice to omit `is_kind` is sound;
- `smoke` can be an instance of `phenomenon` and also be the kind referenced by
  particular plumes;
- a kind definition can guide an agent without becoming an instance validator.

### SKOS: store direct edges; derive paths

SKOS distinguishes direct `broader` links from a transitive broader relation used for
querying all ancestors. It permits multiple paths and calls this a polyhierarchical
knowledge organization system. It also warns that cycles cause problems for
transitive traversal.

Implication for Aicadia:

- an authored `subtype-of` claim represents one direct classification edge;
- do not append inferred ancestor claims to the scene log;
- return indirect classification as a derived path with depth and source claim ids;
- multiple parent kinds are valid;
- every recursive query must be cycle-safe.

### OWL 2: useful boundary, wrong runtime

OWL adds:

- equivalent and disjoint classes;
- property restrictions;
- cardinality restrictions;
- property chains; and
- automated entailment.

OWL also uses an open-world assumption: a missing fact is not false. Its primer
explicitly warns that an ontology is not a syntax-conformance schema.

Useful lesson for Aicadia:

- “missing instance property” remains unknown;
- a kind definition is not a validation contract;
- advanced restrictions could be stored as ordinary descriptive claims if the world
  invents them.

Rejected implication:

- do not add an OWL reasoner, ontology service or hidden entailment pass to the
  server;
- do not turn an authored kind claim into a mechanical rule.

### Darwin Core: classification is a sourced opinion

Darwin Core separates:

- the taxon id;
- the scientific-name id;
- the accepted name usage;
- the direct parent in one classification;
- the source according to which the concept is defined; and
- the taxonomic status.

This matters because a biological name may be moved, split, treated as a synonym or
used with a different scope without erasing its history.

Implication for Aicadia:

- a kind id is not its name;
- every direct parent and classification claim keeps scene provenance;
- reclassification appends and supersedes claims rather than moving an entity row;
- two observers may temporarily report different parents;
- splitting one kind creates new kind ids while the old id remains historically
  queryable.

Aicadia does not need Darwin Core’s taxon-specific columns. The existing generic
claim provenance already expresses the useful distinction.

### Relation Ontology and PROV-O: do not overload classification

The OBO Relation Ontology separately defines relations such as `part of` and
`develops from`. W3C PROV-O separately models derivation, revision, specialization
and alternate representations.

Implication for Aicadia:

- “made from”, “developed from”, “descended from”, “part of” and “subtype of” are not
  synonyms;
- a broad free predicate vocabulary is more powerful than adding those meanings to
  `subtype-of`;
- only `instance-of` and `subtype-of` need special classification-query semantics;
- other relation predicates can emerge when a concrete scene needs them.

### PostgreSQL: graph traversal is already available

PostgreSQL recursive CTEs can return depth-first or breadth-first graph paths. Its
`CYCLE` clause detects repeated nodes and retains the traversed path.

Implication for Aicadia:

- the current indexed `claim` table is enough initially;
- no graph database, ontology database or closure table is required for v1;
- a cached classification closure earns a table only after measured query pressure.

## Recommended classification contract

### Direct source claims

Store only accepted direct claims:

```text
claim
  subject_id: <old mill tree id>
  predicate_id: <instance-of predicate id>
  object_id: <vlierberk id>
  scene_id: <discovery scene id>

claim
  subject_id: <vlierberk id>
  predicate_id: <subtype-of predicate id>
  object_id: <tree id>
  scene_id: <classification scene id>
```

The accepted scene package remains the source. No derived ancestor claim is added to
the log.

### Strict meaning

Use these sentence tests:

```text
X instance-of A
=> X is one particular member of kind A.

A subtype-of B
=> every A is also a B.
```

Allowed:

```text
old mill tree instance-of vlierberk
vlierberk subtype-of tree
blue reed smoke subtype-of smoke
```

Rejected:

```text
smoke subtype-of fire
```

Smoke can be produced by fire, but every smoke is not a fire.

Rejected:

```text
reed roof subtype-of reed
```

The roof is built from reed; it is not a narrower kind of reed.

Rejected:

```text
vlierberk subtype-of marsh organism
```

This is valid only if every vlierberk is a marsh organism. If marsh growth is merely
usual, use a descriptive habitat claim instead.

### Multiple classifications

An entity may have more than one direct kind:

```text
Merel's ferry instance-of boat
Merel's ferry instance-of home
```

A kind may have more than one direct parent when both “every A is B” statements are
true:

```text
dawnreed subtype-of reed
dawnreed subtype-of luminous organism
```

This is a polyhierarchy, not a single-parent family tree. A mandatory parent would
force agents to choose one arbitrary facet as primary.

No universal root kind is required.

### Derived query paths — confirmed behavior

A classification query can return:

```text
entity_id: <old mill tree id>

direct_kind:
  - kind_id: <vlierberk id>
    source_claim_id: <claim id>

ancestor_path:
  - kind_id: <tree id>
    depth: 1
    path_claim_id: [<vlierberk-to-tree claim id>]
  - kind_id: <plant id>
    depth: 2
    path_claim_id:
      - <vlierberk-to-tree claim id>
      - <tree-to-plant claim id>
```

This is illustrative API output; exact response field names remain open.

The confirmed query behavior distinguishes:

- a direct accepted claim;
- an indirect path calculated from accepted claims;
- the source claim id and provenance of every edge; and
- competing current paths.

An indirect path is never presented as if an agent authored it.

### Definition context without property inheritance

When an agent observes the old mill tree, the response may include:

```text
direct entity claims
direct kind ids
kind ancestor paths
definition claims grouped under the kind that owns each claim
```

It must not copy:

```text
vlierberk has winter leaf colour silver
```

into:

```text
old mill tree has winter leaf colour silver
```

The first is kind context. The second requires its own source claim.

### Classification changes

#### Rename

Rename the kind through name claims. Its id and classification edges stay unchanged.

#### Reclassify one instance

Append a new `instance-of` claim and supersede the earlier current classification
when the predicate’s supersession policy permits it. Preserve both in history.

#### Split one kind

Example:

```text
old kind: vlierberk
new kind: northern vlierberk
new kind: fen vlierberk
```

The new kinds get new ids. Accepted claims may make both `subtype-of vlierberk`.
Individual trees can be reclassified over time. The original kind id remains
available to interpret older scenes.

#### Merge duplicate concepts

Do not rewrite either id or silently choose one. A sourced equivalence, synonym or
accepted-concept relationship may connect them. The exact predicate and projection
behavior remain a separate decision.

## Classification is not origin

### Material transformation

```text
blue glass subtype-of glass
this blue glass shard instance-of blue glass
this blue glass shard produced-from this sand batch
```

The last claim is origin, not classification.

### Smoke

```text
this plume instance-of blue reed smoke
blue reed smoke subtype-of smoke
this plume emitted-by this kiln
blue reed smoke produced-from blue reed
```

`produced-from` on a kind describes a general process. `emitted-by` on this
plume identifies its particular source. Neither becomes `subtype-of`.

### Biological descent

```text
dawnreed subtype-of reed
dawnreed descends-from marsh reed
```

The first claim says every dawnreed is a reed. The second records historical or
evolutionary descent. A future reclassification can change the first without
rewriting the descent claim.

The names `produced-from` and `descends-from` are examples for predicate-vocabulary
discussion, not confirmed core predicates.

## Cycle behavior

Example:

```text
reed subtype-of plant
plant subtype-of reed
```

This makes each kind its own indirect ancestor and makes descendant queries
unstable.

Minimum safe behavior:

1. every recursive query carries visited ids and stops a repeated node;
2. the result reports the cycle and its source claim ids;
3. no derived path beyond the cycle is emitted; and
4. no inferred claim is written.

Recommended strict-projection behavior:

- the direct `subtype-of` edges selected by that projection form an acyclic graph;
- multiple parents and multiple paths remain allowed;
- competing accepted edges remain visible with provenance;
- the exact action when an accepted edge would create a cycle in the strict
  projection is still a concept decision.

Do not solve cycles by requiring one parent. That removes legitimate
polyclassification but does not address bad semantics.

## Query surface

Purposeful operations are more useful than one unrestricted graph language:

### Observe one entity

Return:

- direct current claims;
- direct kind ids and labels;
- bounded ancestor paths;
- definition claims grouped by owning kind;
- provenance and evidence on every direct edge.

### Browse one kind

Return:

- direct parent kinds;
- direct child kinds;
- definition claims;
- a bounded, paginated sample of direct instances;
- classification history; and
- competing accepted edges.

### Search within a kind

Input:

```text
kind_id
include_subtype: true
place_id optional
effective_at optional
```

The server traverses `subtype-of` for ids and then applies ordinary indexed filters.
The exact MCP/API name is not decided.

### Explain a classification

Input:

```text
entity_id
kind_id
```

Return every bounded path connecting the entity to the kind, including the direct
source claim ids. This matters when several agents or classification routes disagree.

## Scale

### Initial implementation

Use the existing `claim` table with indexes that support:

```text
(subject_id, predicate_id)
(object_id, predicate_id)
```

A recursive CTE follows only `subtype-of` claims. Classification depth should be
bounded in API requests and descendants paginated.

### What not to add

Do not add now:

- a graph database;
- an OWL reasoner;
- one materialized row for every implied ancestor;
- a single-parent column on `entity`;
- a fixed rank enum such as kingdom/class/family;
- a mandatory root kind; or
- automatic instance-property inheritance.

### Later optimization if measured

If ancestor queries become a demonstrated bottleneck, add a rebuildable
classification-path projection containing descendant id, ancestor id, depth and
source path. It is query state, never canon. One changed direct edge invalidates only
the affected descendant/ancestor region.

This projection is not earned for v1.

## Failure modes after long use

### One parent per kind

A kind such as `luminous reed` must arbitrarily choose between `reed` and `luminous
organism`. Search misses valid routes and agents create duplicate kinds.

### `subtype-of` used for every relation

Fire, smoke, ash, kiln and fuel become one nonsensical classification chain. Queries
cannot distinguish what something is from how it was produced.

### Inferred claims written as history

One new parent edge generates thousands of synthetic claims with no authoring scene.
Removing the edge later requires rewriting history.

### Definition properties inherited as instance facts

A damaged silver-leaf tree is described as having silver leaves even when no scene
observed them. Exceptions become contradictions.

### One global accepted taxonomy

Classification changes overwrite past meaning. Old scenes become unreadable and
minority reports disappear.

### Full OWL semantics

Agents create class restrictions and property chains that interact in ways neither
the human nor server operator can explain. Write latency and debugging depend on a
reasoner rather than the source scene log.

### Unbounded recursive query

A cycle or very broad ancestor causes expensive global traversal. Bound depth and
result size, track paths, and detect cycles.

## Recommendation for Aicadia

Keep `instance-of` and `subtype-of` as the only structural classification
predicates.

Make them more useful through a concrete query contract:

- direct source edges only;
- immutable ids;
- multiple direct kinds and multiple parent kinds;
- transitive path traversal as rebuildable query output;
- path provenance;
- descriptive definitions;
- no automatic property inheritance; and
- deterministic cycle detection.

Use ordinary emergent predicates for composition, origin, descent, similarity and
classification history. The general claim model supplies extensibility; the two
classification predicates supply predictable traversal.

## Decision status

1. Confirmed: `subtype-of` strictly means “every A is also a B” and a kind may
   have multiple direct parent kinds.
2. Confirmed: queries may return indirect `instance-of` and `subtype-of` paths as
   derived results, never as source claims.
3. Superseded: there is no universal evidence status. Traversal uses applicable
   accepted direct source claims and returns their source claim ids and provenance.
4. Open: what should traversal return when an accepted `subtype-of` claim closes a
   cycle?
5. Open: how are duplicate or equivalent kind ids connected without erasing either
   history?

The next unresolved decision is cycle handling. The smallest option is to keep the
source claim accepted and queryable, stop traversal when it encounters the same kind
id again, and return the detected cycle with its source claim ids. This needs a
concept decision before it becomes binding.

## Sources

- [W3C RDF Schema 1.1](https://www.w3.org/TR/rdf-schema/)
- [W3C OWL 2 Primer](https://www.w3.org/TR/owl2-primer/)
- [W3C SKOS Reference](https://www.w3.org/TR/skos-reference/)
- [Wikidata basic membership properties](https://www.wikidata.org/wiki/Help:Basic_membership_properties)
- [TDWG Darwin Core Quick Reference](https://dwc.tdwg.org/terms/)
- [OBO Relation Ontology](https://oborel.github.io/obo-relations/)
- [OBO direct and indirect relations](https://oborel.github.io/obo-relations/direct-and-indirect-relations/)
- [W3C PROV-O](https://www.w3.org/TR/prov-o/)
- [PostgreSQL recursive query and cycle detection](https://www.postgresql.org/docs/16/queries-with.html)
