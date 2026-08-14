> **Era:** July scene-claim research; its scene/claim vocabulary predates the 2026-08-07 game reframe.

# Claim support and current state

> **Role / side:** sourced research report / development side.
> **Authority:** records this report's question, sources, findings and implications.
> **Excludes:** product decisions and current implementation contracts; see `docs/game/`.

Date: 2026-07-26

Status: research completed; recommendation confirmed on 2026-07-26

## Question

What does the current `evidence_status = reported | corroborated` mean at an abstract
level? Is `corroborated` the right technical term for a claim that receives
independent attention, and should that status determine whether a claim enters the
current world-state projection?

## Short answer

`corroborated` is not the right name for the complete behavior currently assigned to
it.

The current model uses one status transition for four different events:

1. another person referred to the claim;
2. another source independently supports the claim;
3. the claim becomes eligible for a current-state projection; and
4. the system presents the claim as more reliable.

These events do not imply each other.

A citation proves that another source referenced the claim. It does not prove that
the source independently observed it. Independent support may improve the evidence
available to a reader, but it does not decide which predicates may update current
world state. A mechanically valid action by a character owner may need to update
state immediately from one source, while a rumor repeated by ten characters must
remain a rumor.

Confirmed direction: do not rename `corroborated` with one replacement word. Remove
the universal evidence-status axis and model the separate facts literally:

- the source package was `accepted`;
- the source basis says how the authoring agent knows the claim;
- later accepted records may cite, independently support, repeat or contradict it;
- predicate and rule behavior decides whether it participates in a current
  projection; and
- independent uptake may affect discovery or retention surfaces without becoming a
  truth state.

If a binary derived term is still useful:

- use `independently_supported` only when there is actual independent supporting
  evidence; and
- use `independently_referenced` when another source merely cites or continues it.

Neither should mean “true”.

## What the existing concept currently asks `corroborated` to do

### Evidence

`docs/concept/archive/02-canon-model.md` describes it as support from a qualifying independent
witness or citation.

Problem: a witness and a citation are not the same evidence.

### Current-state eligibility

The same document excludes `reported` claims from binding projections and includes
`corroborated` claims.

Problem: current-state eligibility depends on the predicate and authored action, not
only the number of sources.

### Attention and retention

The concept log says `corroborated` is conferred by independent attention. The
influence document says attention is the scarce resource and cultural reuse creates
depth.

Problem: attention establishes uptake, not factual accuracy.

### Volume control

The canon model uses independent attention as a filter on world volume.

Problem: search, catch-up relevance and current state are different projections.
Content can remain queryable history without entering every catch-up or current-state
read.

### Character authority

The character document permits departed characters to continue through
`corroborated` routine and circumstance claims.

Problem: the permission boundary is ownership and pre-authorized routine, not
evidence quality.

## Five independent axes

### 1. Acceptance

Question:

> Did the server accept the complete public source package?

Values already exist at package level:

```text
accepted
rejected
```

Acceptance means:

- the human explicitly confirmed the package;
- deterministic validation passed;
- the immutable scene and claims entered the public chronicle.

It does not mean every proposition is universally true. The package may intentionally
contain an observation, memory, rumor or disputed classification.

There is no need to repeat `accepted` as an evidence status on every claim.

### 2. Source basis

Question:

> On what basis does this source package assert the claim?

Concrete bases include:

- depicted action by the authorized actor;
- direct observation;
- memory;
- report from another source;
- citation of an earlier claim; and
- interpretation or classification.

The submitting agent already has the intelligence required to state this. The server
can validate the declared basis only where a mechanical boundary exists, such as
character ownership, location and cited-id existence.

The exact storage form and vocabulary remain open. This could be provenance attached
to the claim or a claim-to-claim relationship. It should not be inferred from prose
by the server.

### 3. Later relationship to a claim

Question:

> What did a later accepted source do with this earlier claim?

These are different:

```text
cites
repeats
supports through independent observation
contradicts
supersedes
```

A later citation is not automatically support. Repeating the same underlying source
is not independent evidence. A contradiction is not a lower numeric score.

Every relation should preserve:

- the later source;
- the earlier claim id;
- the relation or basis;
- the actor; and
- the time.

The exact claim-reference representation remains open. The existing research already
requires stable claim ids because witnesses and challenges refer to claims.

### 4. Currentness

Question:

> Is this accepted claim still the applicable direct claim for its subject and
> predicate at the requested world time?

Currentness depends on:

- effective time;
- supersession;
- predicate cardinality;
- character authority;
- location and travel;
- and specific mechanical rules.

It does not depend universally on whether a second person mentioned the claim.

### 5. Independent uptake

Question:

> Did another independent person or source consciously reuse this world material?

This is valuable for:

- surfacing cultural continuation;
- showing an author how their contribution was used;
- selecting catch-up material;
- discovering which kinds or customs became shared;
- and deciding whether a solo place discovery has been independently visited.

Literal terms:

```text
independently_referenced
independently_observed
independently_supported
```

These should be derived from concrete accepted records. They should not form a
counter, rank, reputation score or universal truth status.

## Concrete scenarios

### Authorized direct action

Scene:

> Mara places her cup on the mill table.

Structured result:

```text
cup located-at mill table
source basis: depicted action by cup's authorized actor
```

After deterministic ownership and location validation, this should update the
current location projection immediately. Waiting for another player to corroborate
the cup placement would violate realtime world movement.

### Repeated rumor

First source:

> Mara heard that the ferryman found silver under the bridge.

Second source:

> Iven repeats Mara's story.

The second source establishes independent reference or repetition, not independent
evidence of silver. The silver claim must not become current material state merely
because the story spread.

### Independent observation

Mara and Iven separately visit the same cave entrance.

Their source packages can establish two observations with distinct provenance. A
place-specific rule may derive `verification_status = verified` from independent
physical presence.

That is a concrete place-verification rule. It does not require every world claim to
share a generic `corroborated` status.

### New kind

One accepted scene introduces `vlierberk` and one tree instance. The kind and claims
must be queryable immediately so another agent can cite, investigate or contest
them.

A later classification scene may independently support or contradict the
`subtype-of` claim. This enriches the evidence graph but does not turn the first
source package from non-canon into canon; it was already accepted.

### Character movement

The character owner authors a valid departure. The current journey projection must
change on acceptance. A second witness is neither required nor authorized to decide
whether the owner departed.

### NPC routine

Whether an unowned character continues baking follows from an explicit handover,
standing order or NPC rule. Three characters repeating “the baker still bakes” do
not create authority to move or speak for that character.

## Lessons from established systems

### W3C Verifiable Credentials

The W3C specification explicitly separates verification from truth. Verification
establishes that a credential is an authentic and current statement by its issuer;
it does not establish that the claims encoded in it are true. A relying system
applies its own policy before using those claims.

Implication:

- `verified` would also be a poor replacement for `corroborated`;
- source authenticity, claim truth and fitness for a projection are separate.

### W3C PROV

PROV describes the entities, activities and people involved in producing data.
Provenance can be used by a consumer to assess quality, reliability or trustworthiness;
the provenance record does not itself make that assessment.

Implication:

- Aicadia should return source and relationship history;
- the server should not collapse provenance into one truth label.

### Wikidata

Wikidata treats statements as information according to sources and supports
contradicting sourced perspectives. References, qualifiers and ranks are separate.
Its source guidance warns against adding a second reference based on the same source
as if it were independent support.

Implication:

- two references are not necessarily two independent sources;
- source relationships should remain inspectable;
- a preferred query view is not the same as factual certainty.

### Evidence & Conclusion Ontology

ECO distinguishes the type of evidence from the method by which an assertion was
made. It exists because those concepts were previously mixed together.

Implication:

- `direct observation`, `author statement`, `citation` and `independent support`
  describe different provenance;
- Aicadia does not need ECO's scientific vocabulary, but it should preserve the same
  conceptual separation.

## Terminology assessment

| Term | Literal meaning | Fit for Aicadia |
|---|---|---|
| `reported` | somebody reported the content | useful as a source basis, not a universal claim state |
| `corroborated` | independent evidence supports the assertion | valid only when genuine independent evidence exists |
| `verified` | authenticity or a defined check was completed | does not mean the claim is true |
| `confirmed` | someone established the claim as true | too strong for a dumb server |
| `accepted` | the server accepted the package | correct package lifecycle term |
| `current` | applicable at the requested effective time | correct projection term |
| `independently_referenced` | another independent source cited or reused it | correct attention or uptake term |
| `independently_observed` | another source directly observed it | correct evidence-basis term |
| `independently_supported` | another independent source supplied supporting evidence | correct support topology term |
| `binding` | a rule or projection relies on it | a consequence of a specific consumer, not an evidence status |

## Recommended model

### Source truth

The accepted scene package remains immutable:

```text
scene
claim
claim provenance
```

Do not mutate the source claim from `reported` to `corroborated`.

### Later support

A later accepted source adds a new record or relationship that cites the earlier
claim and states what it does:

```text
cites
repeats
independently observes
supports
contradicts
supersedes
```

No exact new table or schema is recommended yet. The representation must fit the
existing scene-and-claim truth model and stay append-only.

### Current projections

Each mechanical projection declares the exact accepted input it uses.

Examples:

- current character location uses the owner's valid movement claim;
- current object location uses a valid placement or movement claim;
- place verification uses the independent-presence rule;
- kind browsing returns all current sourced classification claims with provenance;
- a strict classification view may apply an explicit source filter;
- rumors remain queryable reports and never become physical state through repetition.

### Attention and retention

If independent uptake remains a product requirement, calculate
`independently_referenced` from accepted later sources and use it only for purposeful
surfaces. Return the concrete later scenes rather than a total.

This preserves the no-score rule and the original insight that cultural reuse matters.

## Confirmed consequences for the current concept

The confirmed direction requires:

- remove `evidence_status = reported | corroborated` as a universal mutable claim
  status;
- remove the universal rule that claims enter current state only after corroboration;
- stop treating a citation as independent evidence;
- keep `accepted` for package lifecycle;
- keep `verification_status` only for explicit verification mechanisms such as an
  independently visited place;
- define how source basis and claim-to-claim relationships enter the accepted
  package;
- make each current projection name its deterministic input rule;
- revisit NPC continuation, gestures, classification cycles and catch-up filtering
  using the separated axes.

The concept choice is confirmed. The exact source-basis vocabulary and reference
storage shape remain later decisions.

## Second research pass: comparable systems

The first pass established that `corroborated` combines several meanings. This
second pass asks a more practical question:

> Which parts of mature shared-state, knowledge and agent-memory systems can Aicadia
> reuse without copying their complexity?

### Comparison

| System | Durable source | Current or preferred view | How support is represented | Direct lesson |
|---|---|---|---|---|
| OpenStreetMap | versioned element edits grouped in a changeset | latest visible element version | edit source and changeset history; uncertain reports are separate notes | a report must not silently edit current state |
| Wikibase | individually identified statements with qualifiers and references | query-selected rank and time qualifiers | references remain provenance; rank remains a separate query preference | a cited statement is not automatically current or true |
| Matrix | immutable room events | latest authorized event for a `(type, state_key)` pair | event authorization and deterministic state resolution | current state needs an explicit competition key and authority rule |
| Datomic | immutable atomic facts in serialized transactions | database value at a selected transaction time | transaction metadata and fact history | current, as-of and history are views over one accepted record |
| KurrentDB/EventStoreDB | ordered events | projection state rebuilt from those events | event metadata and stream linkage | application writes and projection writes must not be mixed |
| Nanopublication | small assertion package | consumer query | assertion provenance and package publication metadata are separate | assertion content and its creation history need different fields |
| GEDCOM X | extracted source data and authored conclusions | application-selected conclusion | source reference, evidence reference, attribution, analysis and confidence are separate | source, evidence and conclusion are not one status |
| CIDOC CRMinf | proposition sets plus recorded argumentation activity | an actor's recorded belief at a time | observation, inference, belief adoption and provenance assessment are distinct | citing a belief is a different act from observing or inferring |
| Graphiti/Zep | verbatim episode | temporal entity and fact graph | facts link back to episodes; valid time and ingestion time are distinct | the source episode must survive extraction and current facts need time |
| Generative Agents / Letta | private experience stream or mutable agent memory | agent-selected working context | LLM retrieval, reflection and agent-managed summaries | private agent memory is useful context, not shared world authority |

### OpenStreetMap: a note is not a map edit

OpenStreetMap has two relevant write paths:

1. an edit creates a new version of a map element and belongs to a changeset;
2. a note reports that something may be wrong or missing.

The note documentation explicitly warns that edits should not be made solely from a
note because a note can be misleading or false. Resolved notes are retained rather
than treated as if they never existed.

OpenStreetMap elements have stable ids, version numbers, a current visibility value,
a changeset id and full history. The server rejects an edit based on a stale element
version. Its database maintains current and historical tables. OpenStreetMap's
tagging vocabulary is open-ended, but the mechanics of identity, versioning and
changesets are fixed.

Direct Aicadia reuse:

- an accepted report remains queryable source material;
- a report does not update a physical current-state row merely because it was
  accepted;
- a direct, authorized state change uses a different deterministic projection rule;
- stable ids and source-package ids survive later edits and corrections;
- concurrent writes to an exclusive current key must be revalidated against the
  latest state.

Do not copy:

- a second top-level `note` subsystem. Aicadia already has one source package and can
  express the source basis of its claims inside that package;
- OpenStreetMap's policy that only present physical reality belongs in the database.
  Aicadia intentionally preserves memories, myths, predictions and disputed stories.

### Wikibase: references, qualifiers and rank are separate

A Wikibase statement has a stable statement id, a value, qualifiers, references and
a rank. Multiple statements for the same property can coexist. Qualifiers add
context such as effective time or determination method. References record where the
statement came from.

Wikidata's rank documentation is unusually explicit:

- references indicate the source of a value;
- ranks decide which values are normally used in a query or display;
- a referenced value may still be wrong;
- an outdated but historically correct value is not the same as an erroneous value;
- more than one value can be preferred.

Wikidata also recommends not adding a second reference that ultimately comes from
the same source. The number of reference records is therefore not an independence
count.

Direct Aicadia reuse:

- give every claim a stable id;
- keep effective context and source provenance inspectable;
- allow competing claims to coexist;
- let a query expose both the selected current result and all source claims;
- treat “what this query should normally use” as a consumer decision, not evidence
  stored inside the source assertion.

Do not copy:

- a manually edited universal rank. Aicadia's current physical state must follow
  deterministic predicate rules, not community preference;
- Wikibase's complete statement serialization. The useful separation can remain
  ordinary PostgreSQL columns and references.

### Matrix: current state is keyed, authorized event replacement

Matrix separates message events from state events. A room state event competes for
one `(event type, state_key)` slot. The current state contains the selected event for
each key. Sending a later authorized state event with the same key changes current
state while the older event remains in history.

The important mechanism is not “newest always wins”. A state event is first subject
to explicit authorization rules. Matrix federation then needs complex deterministic
state resolution when independent servers produce competing histories.

Direct Aicadia reuse:

- define the exact current-state key for every mechanical projection;
- define who is allowed to replace that key;
- keep the event that produced the current value;
- return historical state from the historical event sequence.

Concrete examples:

```text
projection key: current character location / character_id
authority: the character owner or an explicit applicable rule

projection key: current object location / object_id
authority: an accepted valid action with control over the object

projection key: current name / entity_id + name role
authority: the predicate's accepted replacement rule
```

Do not copy Matrix's federated event DAG or state-resolution algorithm. Aicadia has
one authoritative chronicle server. PostgreSQL transaction order and deterministic
validation are the simpler boundary.

### Datomic and KurrentDB: history owns truth, projections own their output

Datomic stores immutable atomic facts in serialized transactions and exposes current,
`as-of`, `since` and full-history database views. Transaction metadata is a natural
place for provenance. A current entity is a point-in-time view, not a historical
record.

KurrentDB stores state changes as ordered events. Its projection documentation adds
an important operational guard: application code must not append to streams owned by
a projection, because the projection must be able to rebuild and verify its own
output. Resetting a projection starts it from the beginning. The documentation also
warns that projections which emit more events cause write amplification.

Direct Aicadia reuse:

- accept the complete scene package atomically;
- serialize accepted packages in one server order;
- rebuild every current row from accepted packages;
- store the source claim id on every projected row;
- only the projector writes projection tables;
- keep projections small and consumer-specific instead of creating derived events
  for every possible relationship.

Do not copy Datomic or KurrentDB as infrastructure. PostgreSQL can implement the
required semantics. Do not append calculated classification paths or “support
changed” events to the source log; calculate or materialize them as rebuildable
views.

### Nanopublications: assertion and publication are different provenance levels

A nanopublication consists of:

1. an assertion;
2. provenance describing how the assertion was produced; and
3. publication information describing who published the package and when.

This maps closely to Aicadia:

```text
nanopublication assertion        -> claim content
assertion provenance             -> claim source basis and source references
nanopublication publication info -> accepted scene package and submitting actor
```

The useful lesson is the boundary, not RDF. Aicadia does not need named graphs,
SPARQL, content-addressed publication networks or one extra object for every
provenance layer.

### GEDCOM X and CRMinf: useful distinctions, excessive implementation size

Genealogy repeatedly encounters multiple documents, faulty memories, alternative
identifications and revised conclusions. GEDCOM X consequently separates:

- a source description;
- a reference to a specific source fragment;
- an evidence reference used to derive a conclusion;
- the conclusion itself;
- the contributor's attribution and optional confidence.

CRMinf goes further. It represents a proposition separately from the activity that
led an actor to believe it. Observation, inference, adoption of somebody else's
belief and assessment of source provenance are distinct activities. Its own examples
show why this matters: an early classification can be based on contextual inference
and later be revised after direct scientific analysis.

Direct Aicadia reuse:

- distinguish direct action, direct observation, report/citation and interpretation;
- preserve which earlier claim or source a later claim used;
- store later revisions without rewriting the earlier source.

Do not copy:

- confidence levels or numerical evidence strength;
- a universal scholarly argumentation ontology;
- separate database entities for every mental act.

Those systems serve researchers who must document the complete reasoning process.
Aicadia first needs enough provenance for agents to query and judge the source. The
connected agent can explain richer reasoning in the immutable scene prose.

### Graphiti/Zep: the closest agent-memory architecture, with the wrong intelligence boundary

Graphiti ingests an immutable episode, extracts entity and fact edges, retains links
from those edges to the source episode and records two timelines:

- ingestion time: when the system learned the information;
- valid time: when the information applied in the represented world.

New temporal facts can invalidate older overlapping facts without deleting them.
Queries can retrieve current or historical edges and trace a fact back to the
verbatim episode. This is structurally close to Aicadia.

The decisive difference is where intelligence runs. Graphiti uses LLM calls during
ingestion for entity extraction, entity resolution, fact extraction, temporal
extraction and contradiction detection. It then gives newer information priority
when the LLM identifies an overlap.

Aicadia should invert that boundary:

```text
Graphiti                         Aicadia
server-side LLM extraction      submitting player's agent supplies claims
server-side entity resolution   agent selects stable ids before submission
LLM contradiction detection     deterministic predicate and authority rule
graph database                  PostgreSQL projections
raw episode retained            accepted scene package retained
valid + ingestion time          world-effective + acceptance time
```

Direct reuse:

- retain the exact episode/source package;
- keep source-to-fact links in both query directions;
- distinguish world-effective time from acceptance time;
- perform incremental projection updates;
- return compact, source-linked world context to an agent.

Do not copy:

- Neo4j or another graph database merely because the data is graph-shaped;
- embeddings or LLM-generated summaries as world truth;
- LLM-selected contradiction invalidation;
- deletion-oriented generic graph CRUD for accepted history.

Graphiti is evidence that the episode–fact–time separation works for AI agents. It is
not evidence that the semantic extractor belongs inside the shared server.

### Generative Agents and Letta: private memory is not public canon

Generative Agents stores an agent's experiences, retrieves relevant memories and uses
LLM reflection to create higher-level memories that guide planning. Letta similarly
offers mutable, agent-managed memory blocks plus searchable archival memory. Shared
Letta blocks can coordinate agents, but they remain mutable context containers.

These are valuable models for a player's personal agent:

- remember the human's preferences;
- retain private drafts and intentions;
- select a small relevant context window;
- form a private interpretation of public events.

They are unsafe as Aicadia's public state:

- a reflection is generated interpretation, not an accepted public source;
- an agent can rewrite a memory block;
- retrieval relevance is contextual and model-dependent;
- a private summary may omit or distort contested source material.

Direct Aicadia boundary:

```text
private agent memory   mutable, selective, intelligent, owned by the player
public world chronicle immutable accepted packages, dumb deterministic storage
world briefing         rebuildable query result with source ids
```

The server supplies reliable source-linked world slices. Each connected agent decides
what to remember privately and how to narrate it to its human.

## Findings shared by the compared systems

### 1. Acceptance is not truth

An accepted write means the server admitted a package under its mechanical rules. It
does not certify every proposition. W3C credentials, Wikibase references, OSM notes,
GEDCOM conclusions and CRMinf beliefs all preserve this distinction.

### 2. Provenance is a graph, not a promotion ladder

One source can cite another source, derive from several sources, repeat a report,
contradict it or replace a current value. Collapsing that topology into
`reported -> corroborated` destroys information that later agents need.

### 3. Currentness is consumer-specific

The current map, character location, current label and kind browser do not need the
same selection rule. Matrix uses a key per state family; Wikibase uses query ranks;
OSM uses the latest valid element edit; event-sourced systems use projection code.

### 4. Uncertain input remains useful without mutating current state

OSM notes are queryable and actionable without being map data. Wikibase keeps
deprecated and competing statements. Aicadia can retain a rumor, failed observation
or disputed classification without allowing it to relocate a character or create
material silver.

### 5. World-effective time and acceptance time are both necessary

Graphiti, XTDB, Wikibase qualifiers and event-sourced databases distinguish when a
fact applied from when the database learned it. Aicadia needs this for:

- a scene accepted today that recounts something from last winter;
- a correction accepted later about an earlier event;
- historical map and relationship queries;
- deterministic replay in acceptance order.

### 6. Independent support cannot be counted from references

Several references may share one underlying source. A citation can repeat a rumor.
An observation can be independent even when it reaches the same conclusion.
Independence is derived from actual source lineage, actors and observation context,
not the length of a reference list.

### 7. Public state and agent memory have different owners

Agent-memory systems optimize relevance and behavioral continuity. Shared-state
systems optimize auditability, authorization and deterministic convergence.
Aicadia needs both, but on opposite sides of MCP.

## KISS translation for Aicadia

### Keep now

The smallest design supported by all comparisons is:

1. one immutable accepted `scene` source package;
2. stable ids for every included `claim`;
3. explicit provenance supplied by the submitting agent;
4. both acceptance time and world-effective time;
5. a deterministic selection contract per current projection;
6. the source claim id on every projected row; and
7. direct query access to accepted claims that a current projection does not select.

A current projection contract must answer four concrete questions:

```text
key        Which claims compete for this current row?
authority  Which actor or rule may change it?
time       At what world time does the claim apply?
replace    How does an eligible later claim replace or coexist with the earlier one?
```

Example:

```text
entity_location
  key: entity_id
  authority: predicate-specific movement or control rule
  time: claim effective time
  replace: latest eligible accepted claim in server order
  provenance: source claim id
```

This is a projector contract. It is not evidence quality.

### Do not add now

- no universal `evidence_status`;
- no stored support count;
- no confidence score;
- no community-edited truth rank;
- no RDF or graph database;
- no server-side LLM extraction, entity resolution or contradiction detection;
- no federated DAG or CRDT;
- no complete GEDCOM/CRMinf argumentation model;
- no derived event for every projection update;
- no separate “corroboration service”.

### Leave open until a concrete write contract requires it

- the exact fixed vocabulary for claim source basis;
- whether a source reference is a nullable claim field or a small normalized
  relation;
- whether explicit `contradicts` and `supersedes` relations are needed in the first
  vertical slice;
- which projections need independent observation;
- whether retention/catch-up needs an `independently_referenced` derived query.

## Four scenarios after applying the research

### Mara moves her cup

The accepted package contains the depicted action and its location claim. The
`entity_location` projector checks actor/control, travel and rule validity. If valid,
it selects the claim immediately and stores its id. No witness is needed.

### Iven repeats a silver rumor

The accepted package records that Iven received or repeated the report and references
its source. The material-state projector has no rule accepting a repeated report as
physical silver. Search and narrative briefing can still return the report.

### Iven independently sees silver

The accepted package records a direct observation rather than a citation. A later
agent can retrieve both observations and their source scenes. Whether this creates a
current material entity is decided by a concrete discovery or materialization rule,
not by a generic promotion.

### A second traveller reaches the cave

The place-verification projector can require two distinct qualifying presence claims
whose actors actually reached the place. It then derives `verified` for that place.
This is one explicit rule over source claims, not a new evidence status on every
claim in the world.

## Decision

Confirmed on 2026-07-26: an accepted claim is immediately queryable and may be used
by a current projection when that projection's deterministic rules allow it. Later
citations, repetitions, observations, support, contradictions and replacements
remain separate append-only provenance. The universal
`reported | corroborated` status is retired.

## Sources

- [W3C Verifiable Credentials Data Model 2.0](https://www.w3.org/TR/vc-data-model/)
- [W3C PROV Overview](https://www.w3.org/TR/prov-overview/)
- [Wikidata statements](https://www.wikidata.org/wiki/Help:Statements/en)
- [Wikidata sources](https://www.wikidata.org/wiki/Help:Sources)
- [Wikidata ranking](https://www.wikidata.org/wiki/Help:Ranking)
- [Evidence & Conclusion Ontology](https://evidenceontology.org/)
- [ECO conceptual background](https://evidenceontology.org/about_eco/)
- [OpenStreetMap elements](https://wiki.openstreetmap.org/wiki/Element)
- [OpenStreetMap changesets](https://wiki.openstreetmap.org/wiki/Changeset)
- [OpenStreetMap notes](https://wiki.openstreetmap.org/wiki/Notes)
- [OpenStreetMap verifiability](https://wiki.openstreetmap.org/wiki/Verifiability)
- [Matrix room events and current state](https://spec.matrix.org/latest/client-server-api/#types-of-room-events)
- [Matrix room versions](https://spec.matrix.org/latest/rooms/)
- [Datomic information model](https://docs.datomic.com/datomic-overview.html)
- [Datomic historical database views](https://docs.datomic.com/reference/filters.html)
- [KurrentDB event streams](https://docs.kurrent.io/server/v24.10/features/streams)
- [KurrentDB projections](https://docs.kurrent.io/server/latest/features/projections/)
- [Nanopublication Guidelines](https://nanopub.net/guidelines/working_draft/)
- [GEDCOM X](https://developers.familysearch.org/main/docs/gedcom-x)
- [GEDCOM X Conceptual Model](https://github.com/FamilySearch/gedcomx/blob/master/specifications/conceptual-model-specification.md)
- [CIDOC CRMinf 1.2.1](https://cidoc-crm.org/sites/default/files/CRMinf_v1.2.1%28stable%29.pdf)
- [Graphiti](https://github.com/getzep/graphiti)
- [Graphiti episodes](https://help.getzep.com/graphiti/core-concepts/adding-episodes)
- [Zep temporal knowledge graph paper](https://blog.getzep.com/content/files/2025/01/ZEP__USING_KNOWLEDGE_GRAPHS_TO_POWER_LLM_AGENT_MEMORY_2025011700.pdf)
- [Generative Agents](https://arxiv.org/abs/2304.03442)
- [Letta memory blocks](https://docs.letta.com/guides/core-concepts/memory/memory-blocks)
