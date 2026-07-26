# Persistent world-state in other systems

Date: 2026-07-25

Status: research, not concept direction

## Question

How do existing shared worlds and adjacent state systems turn participant input into
durable, queryable world-state, and what can Aicadia learn from them while keeping a
dumb, strict server and agent-supplied intelligence?

## Aicadia constraints used in the comparison

- One persistent shared world, eventually used concurrently by millions of agents.
- The application performs no LLM inference.
- An accepted scene is one immutable source package: prose, structured claims
  authored by the submitting agent and provenance.
- Current entity, claim, map, dossier and catch-up views are rebuildable projections.
- The server understands a small mechanical constitution, not the meaning of every
  institution the world may invent.

## Systems

### Matrix: authored events plus resolved current state

Matrix keeps room history as events. It distinguishes message events, which represent
one-off activity, from state events, which compete for a current `(event type,
state key)` slot. Clients can page through history or request current room state.
Client-supplied transaction ids make message submission idempotent. Federation adds a
deterministic state-resolution problem when concurrent histories merge.

Useful for Aicadia:

- History and current state are different products over the same accepted events.
- A stable event id and client transaction id belong in the ingress contract.
- The key that a state assertion competes for must be explicit.
- Aicadia has one authoritative server and should not inherit Matrix's federated DAG
  and state-resolution complexity without a real need.

### Wikidata/Wikibase: statements are richer than triples

Wikidata represents knowledge as item–property–value statements. A statement may
carry qualifiers, references and a rank; properties determine allowed value types.
Qualifiers commonly express context such as the period during which a statement
applies. Multiple statements for the same property can coexist.

Useful for Aicadia:

- Provenance and temporal/context qualifiers must not be squeezed into prose alone.
- A claim needs its own stable identity if witnesses, challenges or other claims can
  refer to it.
- Multiple values and explicit context are safer defaults than assuming every
  predicate has one value.
- Wikidata's editorial rank is evidence that "which assertion is current/preferred"
  is a separate concern from preserving the assertion.

Difference: Wikidata separates structured statements from encyclopedia prose.
Aicadia has decided to bind prose and claims atomically in one source package.

### Datomic: an accepted transaction creates a new database value

Datomic treats a transaction as one complete information set accepted at an atomic
point. Facts are immutable information; database values can be queried as-of a past
point, since a point, or through full history. Validation sees the database before
and proposed database after the transaction.

Useful for Aicadia:

- The scene package maps naturally to a transaction boundary: all accepted or none.
- Validation should use the current relevant world-state and the proposed complete
  package, not apply claims one by one in an order-dependent loop.
- Current, `as_of`, `since` and history are fundamental query modes, not later
  reporting features.
- A plain PostgreSQL implementation can borrow these semantics without adopting
  Datomic itself.

### LambdaMOO: a mutable object world persisted by checkpoints

LambdaMOO stores rooms, characters, objects, properties and executable verbs in one
shared object database. The server executes user commands and in-world programs. The
classic server keeps the database in memory and periodically writes a full
checkpoint so the world survives restart.

Useful for Aicadia:

- A tiny set of universal objects can support a surprisingly open-ended social world.
- Putting executable world behavior beside persisted objects makes the server
  intelligent and institution-aware; that is powerful, but opposite to Aicadia's
  boundary.
- A checkpoint answers "what exists now" but is a weak sole record for "who said what,
  from which scene, and what did the world know then?" Aicadia needs authored history
  as the base, even if it later adds checkpoints for faster recovery.

### Evennia: persistent objects with extensible attributes and server typeclasses

Evennia persists a small family of base objects—objects, characters, rooms, exits and
accounts—and lets builders attach persistent attributes, tags and Python typeclass
behavior. Tags provide fast shared classification; arbitrary attributes provide
flexible per-object state.

Useful for Aicadia:

- The small structural core closely resembles Aicadia's entity/place/character
  direction.
- Tags demonstrate why a normalized shared vocabulary is much easier to search than
  arbitrary attribute bags.
- Typeclasses and feature components also demonstrate the pressure to add a server
  module for every newly invented institution. Aicadia should resist that pressure:
  semantic variety belongs in claims until a mechanical rule genuinely earns code.
- Mutable arbitrary attributes are convenient current state, but do not by
  themselves preserve narrative provenance.

### EVE Online: one logical universe, location-based simulation processes

EVE's Tranquility architecture has historically kept the universe in one central
database while proxy processes handle sessions and SOL processes simulate solar
systems. Solar systems can be reassigned across machines, but a very crowded system
still becomes one hot simulation node.

Useful for Aicadia:

- One shared world does not require one application process.
- Geography is a natural future placement and routing boundary.
- Spatial partitioning does not solve mass gathering. Any design whose cost is
  proportional to everyone present at a scene still has an unbounded hotspot.
- EVE has gradually added gateways, search and message-bus consumers around its core.
  That is an earned evolution, not a starting architecture for Aicadia.

### Second Life: region simulators plus central data services

Second Life divides land into fixed regions. A simulator owns the live state and
visibility work of a region; routing locates neighboring simulators and hands a viewer
from one region to another. Central data services support database, log, inventory
and search access.

Useful for Aicadia:

- A place can be both a fictional fact and an operational scope for reads, routing
  and eventual physical placement.
- Agent briefings should receive a bounded world slice, analogous to spatial
  streaming, rather than a copy of global state.
- Fixed-size simulation regions are specific to a geometric realtime world. Aicadia
  should derive scopes from its emergent containment/map graph rather than ship a
  square grid.

### Roblox and Unity: server-authoritative mutable game state

Roblox treats its server runtime data model as authoritative and streams nearby
content to clients; persistent data stores keep selected state across sessions.
Unity's recommended asynchronous game pattern sends player commands to server code,
which validates fixed game rules, mutates authoritative state and saves the result.

Useful for Aicadia:

- The client cannot be authoritative for mechanical invariants such as identity,
  ownership, credit or travel.
- These systems assume the server understands the complete game rules. Aicadia
  cannot use that model for narrative meaning: the submitting agent supplies the
  semantic claims and the server validates only its known mechanical envelope.
- Mutable save objects are useful projections or caches, but insufficient as the
  chronicle.

## Cross-system findings

### 1. Preserve input history separately from current state

Matrix, Datomic and Wikidata all provide stronger historical reasoning than systems
whose primary persistence is a current object snapshot. Aicadia's archive and its
current graph should be separate query surfaces over the same accepted sources.

### 2. Put semantic work before the dumb boundary

MOO, Evennia, Roblox and Unity put interpretation or behavior in server code. That
works because their possible actions and rules are programmed in advance. Aicadia's
unpredictable semantic growth requires the opposite boundary: the consciously invoked
agent produces both narrative and structure before submission; the server only
checks mechanics and stores the result.

### 3. Treat the accepted package as a transaction

Datomic supplies the clearest model: validate the whole proposal against the
pre-existing state, then accept it at one atomic point. Matrix adds the practical
lesson of client transaction ids for safe retries.

### 4. Give claims context and identity

Wikidata's qualifiers, references and coexistence of competing statements show that
a bare subject–predicate–object triple is likely not enough for history. The exact
Aicadia claim shape remains a concept choice; research does not decide whether time,
place, witness and confidence are columns, related claims or source-package metadata.

### 5. Locality is both product context and a scale boundary

EVE, Second Life and Roblox all scope simulation or replication by location. Aicadia
can make the same move for reads and future data placement without starting with a
distributed system. A global gathering must remain a deliberate stress case because
locality alone does not bound audience size.

### 6. Extensible server behavior quietly ships an ontology

MOO verbs and Evennia typeclasses make new institutions mechanically powerful, but
they also place their meaning in server code. Aicadia's claims-first model is not
merely a flexible schema choice; it protects the principle that institutions are
discovered rather than shipped.

## Implications already decided

- An accepted scene is an atomic, immutable source package.
- The package retains both prose and agent-authored structured claims.
- Current state is rebuilt/queryable from accepted packages; it is not a replacement
  for the source.
- No server-side model later reinterprets the prose.

## Candidate implications, not decided

- Give every source package a server id and require a client idempotency key.
- Record author character, acceptance time, place and applicable rule version as
  provenance.
- Give each claim a stable id and explicit contextual qualifiers.
- Expose `current`, `as_of`, `since` and `history` as first-class query modes.
- Build briefing/context reads as bounded place/entity slices.
- Keep a mutable current projection for speed, but never mistake it for the archive.

## Decision questions exposed by the research

1. Is the private human instruction part of the durable source, or does the durable
   source begin with the agent's submitted world package?
2. Which fields make a claim expressive enough: time, place, polarity, source,
   qualifiers, replacement and references to other claims?
3. How does a later package correct or supersede an earlier claim without erasing it?
4. Which current-state conflicts are mechanically exclusive, and which remain
   competing fiction?
5. Which query modes must the first vertical slice prove?
6. How is a world slice bounded when a place contains a million characters?

## Sources

- [Matrix Client-Server API](https://spec.matrix.org/latest/client-server-api/)
- [Matrix room state resolution](https://spec.matrix.org/latest/rooms/)
- [Wikidata data model](https://www.wikidata.org/wiki/Help:Data_model)
- [Wikibase data model](https://www.mediawiki.org/wiki/Wikibase/DataModel)
- [Datomic transaction model](https://docs.datomic.com/transactions/model.html)
- [Datomic historic data](https://docs.datomic.com/peer-tutorial/see-historic-data.html)
- [LambdaMOO programmer's manual](https://lambda.moo.mud.org/pub/MOO/ProgrammersManual.html)
- [LambdaMOO checkpointing](https://brn227.brown.wmich.edu/Barn/files/docs/lambdamoo/pm1.8.1/ProgrammersManual_67.html)
- [Evennia core components](https://www.evennia.com/docs/latest/Components/Components-Overview.html)
- [Evennia persistent objects](https://www.evennia.com/docs/latest/Howtos/Beginner-Tutorial/Part1/Beginner-Tutorial-Learning-Typeclasses.html)
- [Evennia tags](https://www.evennia.com/docs/latest/Components/Tags.html)
- [EVE Online Tranquility Tech IV](https://www.eveonline.com/news/view/tranquility-tech-iv)
- [EVE Online cluster architecture](https://www.eveonline.com/news/view/my-node-was-equipped-with-the-following...)
- [Second Life server architecture](https://wiki.secondlife.com/wiki/Server_architecture)
- [Roblox client-server runtime](https://create.roblox.com/docs/projects/client-server)
- [Roblox persistent data stores](https://create.roblox.com/docs/cloud-services/data-stores)
- [Unity game-state management](https://docs.unity.com/en-us/cloud-code/game-state-management)
