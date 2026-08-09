# Concept log

One running log, one line per development, grouped by date. This is the record of
concept development — not the source of truth, but the trail. Tags: **decided**
(user call), **direction** (current thinking), **adopted/rejected** (sparring debate
outcome), **principle** (meta-rule), **method** (how we work).

---

# 2026-07-25

## The idea

- explored: an app where people hook their own AI agents (via MCP, from any tool) into one shared persistent world; the app keeps world state; users spend their own tokens.
- principle: BYO inference — the app is a chronicle-keeper with zero intelligence; all creativity comes from the connected agents.
- direction: the magic moment is returning and reading what genuinely happened around your character while you were away.

## Foundational calls

- decided: one single world to start.
- decided: pure fiction, positive and friendly — wonder over combat; challenging without darkness.
- decided: your agent plays one character living in the world, not a co-author above it.
- decided: hard cap — one canon scene per player per day, equal canonical weight for everyone; reading free; small gestures (rumor layer) plentiful.
- principle: token spend is always a conscious user action; no background burn, ever.
- decided: no standard vices as content (no sex, murder, crime, drugs); interesting must come from elsewhere.
- decided: strict originality — nothing from existing IP, not even elves or dwarves; doubles as the anti-slop rule.
- resolved: the advantage question — frequent play may earn richness (positive-sum: relationships, renown, being woven in) but never power (zero-sum: canon steering, scarce claims); no points/XP/levels anywhere; the archive is the only scoreboard.

## World rules

- direction: claim evidence uses operational statuses `reported` and `corroborated`;
  competing `reported` claims may coexist as folklore. Supersession permission is a
  separate field.
- direction: leefregels ch. 1 social (7 rules) + ch. 2 world physics: world grows at world pace; nothing from nothing; distance is time; most days are ordinary (the anti-escalation rule); everything leaves traces.
- principle: the leefregels are the meta-layer everything must fit within; agents are trusted on taste; the server enforces only what is mechanically checkable.
- rejected → replaced: consent-by-approval; instead three lines — nobody writes your voice; only you end your character; the world may happen to anyone (right of response, never a veto).
- decided: stepping away = giving your character to the valley; it continues as an NPC; the former player is notified, never consulted.
- decided: genesis ships zero content — the first player's first scene founds the settlement (genesis heuristic; rules complete, content zero).

## Debate outcomes (sparring session with a second Opus agent)

- adopted: forks expire, never resolve — nobody ever authors an absent character's choice; missed chances are the better fiction.
- adopted: standing orders — pre-authored routine the server may state as fact, never invent.
- adopted: no visible hesitation labels; guilt mechanics banned.
- adopted: places have momentum, people don't — the world's clockwork lives in places and institutions.
- adopted: naming economy — citing existing is free; new named entities cost rationed naming rights; phantom mentions create hooks for others.
- adopted: citations surfaced as content, never totals — no renown numbers, no ranks.
- rejected: craft mastery by accumulation (XP in disguise); instead distinction over rank, mastery only conferred by others.
- adopted: letters answerable with one gesture; social debt expires visibly.
- adopted: style enforced by exemplars — recent canon excerpts injected every turn; hand-picked first cohort becomes the de facto style guide.
- adopted: irreversible events rationed globally per world-year — cap the world, not just the player.
- rejected: vulnerability-token economy (a currency violates no-score) → replaced by "bad news travels faster" ripple weighting.
- adopted with fix: `corroborated` status is conferred by independent attention; solo discoveries remain `unverified` until a second visitor verifies them.
- rejected: departed characters continuing only through `reported` claims (contradicts "the world goes on"); kept NPC continuation with the interiority guardrail — routine and circumstance may be `corroborated`, voice only as a `reported` quotation.
- adopted: mischief boundary = constrained verb set + every harm has a repair path that costs the harmer (replaces "forgivable by the next festival").
- adopted: safe tension ranking — mortality, incompatible goods, being misremembered, scarcity of presence, impersonal adversity, promises hard to keep, secrets, shame & repair, change itself, rivalry over distinction.

## Method & framing

- method: docs/concept/ is a concept log, not a specification — everything exploratory, definitive-sounding terms are working names.
- decided: no planned ending — the world meanders and develops; if a telling is ever needed, the world develops it collectively, never one player alone, never the system.
- principle: institutions are discovered, not shipped — the app ships flowing time, rules and tools; calendars, seasons, festivals, epochs all emerge; an institution is a habit the world noticed.
- decided: mortality is discovered, not shipped — the server has no biology; age and illness exist only when written; illness may visit anyone (rule 3), endings stay owned (rule 4).
- principle: hierarchy may exist as story, never as mechanics — the server knows no masters.
- discussed: the WoW contrast — frozen world + cheap death + scoreboards is our exact inversion; Hardcore servers prove irreversibility creates meaning.
- decided: language — English canon; every agent speaks its own user's language (the agent is the i18n layer; proper nouns travel untranslated).

## Server & data

- direction: one core API; MCP as thin front door for agents; web app for humans and spectators.
- direction: session-is-who-you-are — an account state machine deals personas and resumes characters; the agent never picks.
- direction: the briefing mechanism — the write path enforces the read path: without a fresh briefing token, no public scene package is accepted.
- direction: rules as data — versioned rule table; every mechanical rejection cites its rule slug.
- principle: statements, not modules — six structural types (scene, entity, claim, place, character, rule); everything else is emergent kinds + claims; economy and politics are patterns in scenes, not tables.
- decided: realtime, not turns — the synchronized dawn was a shipped institution and is dropped; scenes process on arrival; the cap becomes a rolling 24h scene credit, hold max 3.
- clarified: the "morning report" was never a product — it is the ripple-inbox diff ("what touched me since I last looked"); fanout-on-write; "bad news travels faster" is a weight column.
- decided: Terry — AGENTS.md carries the Aicadia build standard (singular names always; flat over clever; boring infrastructure; the scene log is the truth; realtime per event; statements not modules; everything must be expressible; rules are data; dumb & strict server; no unconscious token burn; English everywhere; no score anywhere; earn your spot); CLAUDE.md points to it.
- principle: everything must be expressible — "she lives in a timber house with a reed roof and works as a ferryman" must fit without a schema change; extend the core, never reject the fiction.
- direction: the world-graph — a time-versioned provenance graph (facts accrete, never mutate; Datomic's model in plain Postgres); thin entities, claims-first (JSONB dropped); three structural relation families (instance-of type layer; part-of/located-at composition — the world-tree is this lens; free domain predicates); write contract = prose + claims authored by the submitting agent, validated deterministically; contradiction checks only on single-valued predicates; predicate vocabulary with trigram near-synonym nudge; six lenses (tree, map, timeline, social graph, lexicon, search) all projections; performance is local reads + one-transaction writes, ~12 scenes/s at 1M players, no graph database needed.
- decided: the accepted scene is one immutable source package — prose + structured claims authored by the submitting agent + provenance; the server validates and places it atomically, never reinterprets the prose, and rebuilds every current view by replaying these packages.
- method: log-first from now on — running development lands here, one line each; numbered docs are created or updated only when something is genuinely pinned (Terry: documents earn their place).
- decided method / Terry: research always leaves a durable, sourced record under `docs/research/`; research informs choices but never silently becomes concept direction.
- researched: persistent state in Matrix, Wikidata, Datomic, LambdaMOO, Evennia, EVE Online, Second Life, Roblox and Unity; report at `docs/research/persistent-world-state.md` — strongest shared lesson is to keep authored history separate from current query state, while Aicadia's client-supplied semantic package + dumb server remains unusual.
- decided method / Terry: `5jaar` means inhabit Aicadia after five real years at intended scale, study ordinary use, emergent culture, abuse/failure and operational pressure, then backcast to the smallest correct decisions and experiments now; future observation never silently becomes a concept decision.
- researched / `5jaar`: free input, offered choices and agent-led play across AI Dungeon, Storium, classic text worlds and current MCP interaction primitives; report at `docs/research/player-agent-interaction.md` — recommendation for discussion is a private hybrid workshop (agent proposes, user may approve/choose/write) with one explicit confirmation only at the irreversible public boundary.
- decided: every canon scene requires one explicit human confirmation of the complete public source package; the private user-agent workshop may propose, choose, freely steer and revise, but an agent never silently publishes or spends the daily scene.
- researched / `5jaar`: bounded player causation and world change without fresh input, including persistent tickers, on-demand calculation and player-reactive systems; report at `docs/research/world-momentum.md` — recommendation for discussion is that a scene may plant but not skip a history, and that the world has momentum from authored causes but no autonomous server authorship.

## Where we stand

The skeleton feels coherent: one friendly, strictly original world; realtime; BYO
agents; one character per person; rolling scene cap; canon by attention; everything
emergent above six structural types; Terry as build standard. Explored but not yet
pinned: the ring model, witnessing thresholds, naming economy, ripple weighting.
Parked deliberately: MCP tool vocabulary (until the graph feels right).

## Open

- From doc 09: naming uniqueness scope; literal typing in claims; whether `reported`
  claims become `corroborated` when an `unverified` entity becomes `verified`; ripple
  subscriptions beyond locality.
- From doc 07: tuning knobs, moderation of free-text letters, MCP auth details,
  human-only participation.
- Next exploration queued: the life of one claim end-to-end (scene → witness →
  `corroborated` → superseded years later).

# 2026-07-26

## Product name

- decided: the product and shared world are named **Aicadia**, replacing the former
  working title `aiworld`; the repository is `Intergalactisch/aicadia`.

## World agency

- researched: a consciously operated world-steward layer, prompted by the concern that authored momentum alone may not make the most interesting MMO; follow-up added to `docs/research/world-momentum.md`, using Helldivers 2's human Game Master as prior art — a steward could create shared circumstances without autonomous server intelligence, but would be an explicit exception to equal narrative weight and therefore needs a user decision and strict authority boundary.
- decided: v1 has one minimal administrator-only world-steward interaction — on conscious invocation the agent queries current canon and offers exactly three local, open `world move` directions plus doing nothing; choosing nothing changes nothing, while choosing a direction leads to an exact public package that still requires explicit admin confirmation and enters through the ordinary append-only scene path; the steward never rewrites history, authors character volition, grants status or determines the outcome.
- clarified: the steward's three proposals have no prescribed categories; they are the three interventions its meta-analysis currently finds most interesting. Every accepted `world move` remains an ordinary immutable source package but carries permanent, queryable provenance as a meta-admin change.
- decided: meta-admin provenance is private operational history for authorised administrators; players and their agents experience the resulting fiction without seeing that it originated in a world-steward intervention.
- decided: only a chosen package that the admin explicitly confirms and the server
  accepts is retained as meta-admin history; unchosen proposals and a do-nothing
  steward session remain ephemeral private workshop material and leave no audit or
  canon entry.
- decided method / Terry: concrete before abstract — every proposal must name the actor, input, action, stored result and boundary cases, with specific allowed and rejected examples; slogans and broad principles never stand alone as design decisions.

## Player scene

- decided: every new factual claim in a player package must either be the direct depicted action of that player's own character or name accepted canon that supports it. A result requiring another character's choice, unrecorded intermediate work or elapsed time, or recognition by other people cannot be stored as complete; the agent must submit the concrete attempt, invitation, first work or observation instead. The server checks only ownership, location/travel, citation existence, connectivity and claims for which `ordinary_scene_can_supersede = false`; semantic sufficiency remains the agent's responsibility and can be challenged afterward.

## Spatial model

- explored: stable coordinates would make the world directly plottable, but duplicating longitude/latitude on every scene, entity and claim would create conflicting location state. Recommendation for discussion: only a `place` owns or inherits one deterministic map point; a scene stores its `place_id`, a movable physical entity uses a time-versioned `located-at` claim, and non-spatial entities carry no location. Coordinate form and assignment remain undecided.
- explored: travel between two places needs a first-class current transit state rather than pretending the character is still at the origin or already at the destination. Recommendation for discussion: an accepted departure records edge, origin, destination, `departed_at` and `arrives_at`; the server derives map position from elapsed time and places the character at the destination when the authored arrival time is reached. Interruption and route-change rules remain undecided.
- researched / `5jaar`: spatial state, travel and discovering a new place between endpoints, using PostGIS linear referencing as prior art; report at `docs/research/spatial-state.md` — recommendation for discussion is one location contract with `at_place(place_id)` or `on_edge(place_edge_id, position)`, server-derived travel position, edge-anchored `unverified` discoveries, an explicit player choice to stop or continue, and route splitting only as a rebuildable projection.
- challenged / researched: a universal `(x, y)` position with bounded places is more
  scalable for map, nearby and point-in-boundary reads than exposing only `at_place`
  or `on_edge`. Revised recommendation for discussion: return every current physical
  position as a PostGIS `Point`; give `place` a point or boundary geometry; retain
  place/route, time and source scene as accepted source context for validation and
  replay. Static points can be spatially indexed. A traveller's changing point is
  calculated after selecting candidate routes and active journeys, avoiding millions
  of recurring movement writes. No concept choice yet.
- researched / `5jaar`: mutable place geometry across OpenStreetMap, GeoGig, CityGML,
  PostGIS, MobilityDB, Unreal/Roblox spatial streaming and SLEUTH urban growth;
  report at `docs/research/mutable-place-geometry.md`. The previous one-geometry
  model is insufficient. Recommendation for discussion: keep permanent `place`
  identity separate from versioned geometry-valued claims, keep calculated physical
  `place_coverage` separate from authored `place_extent`, and preserve both
  world-effective and server-accepted time. Population alone never determines a
  shape or direction, current disappearance never deletes history, and no concept
  choice has yet been made.
- challenged / researched / `5jaar`: the concrete need for `place_coverage` and exact
  geometry on every house, tree or room; report at
  `docs/research/hierarchical-spatial-model.md`, using OpenStreetMap, CityGML,
  IndoorGML, OpenUSD/glTF, PostGIS and PostgreSQL `ltree`. Revised recommendation for
  discussion: every current physical entity has one containing place, exact geometry
  is optional and may become more precise through later accepted claims, composition
  differs from current spatial location, and a hull or union of child geometry is
  only a labelled map query/cache if a concrete consumer later earns it. Do not add
  a core `place_coverage` table. No concept choice yet.
- researched: open-source spatial models in OpenTTD, OpenRCT2, Luanti,
  Cataclysm: DDA, OpenStreetMap, Overture, CityGML, GRASS, PostGIS, H3 and 3D
  Tiles; report at `docs/research/open-spatial-world-system.md`. City builders use
  cells as simulation truth, geographic systems keep stable feature identity
  separate from geometry, and large viewers use tiles only for delivery. Revised
  recommendation for discussion: accepted entity/place identity and claims remain
  world truth; one containing place supplies a default physical query path; exact
  geometry stays optional and versioned; overlapping relationships remain claims;
  and every chunk, spatial cell and map tile stays a disposable projection. No
  concept choice yet.
- challenged / researched: names and mandatory containing-place hierarchy create
  rename coupling and artificial districts; report at
  `docs/research/stable-identity-and-sparse-location.md`, using UUIDv7,
  Wikidata identifiers and labels, OpenStreetMap/JSON:API local references,
  Overture hierarchy and PostGIS. Revised recommendation for discussion: every
  persisted reference uses an immutable opaque ID; names and aliases are versioned
  claims with a rebuildable label projection; new entities may use package-local
  references during drafting which are resolved before human confirmation; no
  region, district or other level is required; and every current
  physical entity has one location projection row containing at least a place ID,
  geometry or active route. This explicitly retracts the earlier recommendation that
  exact geometry must also have a containing place. No concept choice yet.
- decided: every persisted world reference uses an immutable opaque id; an accepted
  claim references its subject, predicate and entity object by id, never by a human
  name. Current names, former names and aliases are time-versioned,
  provenance-carrying claims. A rename therefore changes no id or existing
  relationship. Name search may return several candidates with ids and
  disambiguating context, but the accepted write must use the selected id. A
  rebuildable `entity_label` projection supplies current display names. The concrete
  id format, package-local draft references and revised spatial-location contract
  remain undecided.
- decided: every current physical entity has exactly one rebuildable
  `entity_location` projection row containing at least one of `place_id`, PostGIS
  `geometry` or active `place_edge_id`; more than one may be present when canon
  establishes both place-level and exact location. No containing place and no
  `region`, `city`, `district`, `village`, `block` or other hierarchy level is
  mandatory. Explicit place relations form only the sparse, variable-depth paths the
  world has actually established. The accepted scene and claims remain source truth;
  how a package mechanically declares physical presence, exact field-combination
  validation and derived geometric coverage remain open.
- challenged / researched: smoke and water show that “physical entity” is not a
  deterministic applicability boundary; report at
  `docs/research/spatial-occurrence-and-field.md`, using OGC feature, observation,
  coverage and CityGML water-body models. Revised recommendation for discussion:
  apply `entity_location` to a discrete current spatial occurrence, not to every
  physical type, material or condition. Reusable `water` and `smoke` entities have no
  location; a time-bounded condition may use its already located subject and source
  scene; an independently cited plume, lake or flood earns an entity and location;
  dense concentration or depth remains accepted sample/zone claims and only later a
  derived coverage if a concrete consumer earns it. No concept correction yet.
- user direction: nature, materials, species and physical phenomena are never a
  fixed server catalogue. Each is an ordinary world entity introduced by an accepted
  scene, with an immutable id but a definition made entirely from time-versioned,
  sourced claims. Reusable categories and concrete occurrences may reference each
  other by id through `instance-of`, `subtype-of`, composition and free predicates;
  the occurrence may carry changing location and extent. An emergent `kind` supports
  broad retrieval but is not the semantic definition. No material, nature or
  phenomenon table or fixed enum is added.
- decided: the first accepted scene that introduces a material, species or natural
  phenomenon gives it a permanent entity id immediately. Its name, classification
  and properties remain separate `reported` claims; entity creation does not
  corroborate a complete definition. Later agents can cite the stable id while
  individual claims are corroborated, contested or superseded.
- discussed: whether nature and material need a specific entity type. Recommendation
  for discussion: no structural subtype and no fixed `category | occurrence`
  discriminator. Both are ordinary entities; emergent `kind` helps broad retrieval
  while id-based `instance-of`, `subtype-of`, composition and free claims carry
  semantic meaning. Whether a package must explicitly declare current spatial
  presence remains a separate location-contract question. No concept choice yet.
- decided, refining the preceding recommendation: no structural entity type is added
  for nature or material, and `entity.kind` is not a source string or enum. Every
  kind is itself an ordinary entity with a permanent id, defined by versioned,
  sourced claims. `instance-of` and `subtype-of` give it its role in the type graph;
  no `is_kind` flag or mandatory root kind is required. One accepted package may
  introduce a new kind and its first instance together. Search returns kind ids and
  current labels derived from claims. Whether kind definitions are descriptive or
  mechanically enforceable remains open.
- decided: kind definitions are descriptive and queryable, never implicit validation
  schemas. The server may return a kind's sourced definition claims but does not
  reject an instance because it lacks a property described on its kind; missing
  information remains unknown. A kind claim cannot create mechanics. Any enforced
  requirement must exist separately as a versioned `rule` backed by a named,
  deterministic validator.

## Kind model

- researched: whether `instance-of` and `subtype-of` are sufficient for a dynamic
  kind family tree; report at `docs/research/kind-classification.md`, using RDF
  Schema, OWL, SKOS, Wikidata, Darwin Core, OBO Relation Ontology, PROV-O and
  PostgreSQL recursive graph queries. Research conclusion: the pair is the correct
  minimal classification core, but the result is a polyhierarchical classification
  graph rather than one family tree. Direct authored edges remain source claims;
  multiple parent kinds are valid; indirect paths are derived with provenance; and
  composition, production origin, biological descent and classification history use
  separate emergent predicates. An OWL reasoner, fixed ranks and automatic property
  inheritance do not fit the dumb-server model. First recommendation for discussion:
  define `subtype-of` strictly as “every A is also a B” while allowing multiple
  direct parent kinds. No new concept choice yet.
- decided: `subtype-of(A, B)` strictly means every A is also a B. A kind may have
  multiple direct parent kinds when the statement holds for each one. The source
  model has no primary-parent field, required single classification tree or
  mandatory root kind.
- decided: classification queries may traverse direct `instance-of` and
  `subtype-of` claims and return indirect kind, ancestor and descendant paths. Every
  calculated result is distinguished from a direct claim and retains path depth and
  the ordered source claim ids. An indirect result remains rebuildable query output
  and is never appended to the scene log as an accepted or synthetic claim.
- decided: classification traversal may include current `reported` and
  `corroborated` source edges so a new kind is immediately queryable. Every edge
  retains its evidence status, and callers may filter for paths made entirely from
  `corroborated` edges. A path containing any `reported` edge remains non-binding;
  queryability never promotes its evidence.
- challenged / researched: `corroborated` currently conflates independent reference,
  independent evidence, current-state eligibility and reliability; report at
  `docs/research/claim-support-and-current-state.md`, using W3C Verifiable
  Credentials, W3C PROV, Wikidata and the Evidence & Conclusion Ontology. A citation
  proves reuse, not independent observation; a valid owner-authored action may need
  to update current state immediately, while a repeatedly cited rumor must remain a
  report. Recommendation for discussion: retire universal
  `evidence_status = reported | corroborated`; keep package acceptance, source basis,
  later claim relationships, currentness and independent uptake separate. The
  classification evidence filter and the pending cycle decision are paused. No
  concept correction yet.
- further researched: expanded the same report with OpenStreetMap, Matrix, Datomic,
  KurrentDB, Nanopublications, GEDCOM X, CIDOC CRMinf, Graphiti/Zep, Generative
  Agents and Letta. The repeated proven shape is immutable source plus explicit
  provenance plus consumer-specific current projection. OSM keeps uncertain notes
  outside map edits; Matrix selects current state by an authorized competition key;
  Wikibase separates references from query rank; Graphiti separates verbatim
  episodes, valid time and derived facts but uses server-side LLM inference that
  Aicadia must leave with the submitting agent. KISS recommendation: keep scene,
  stable claim id, provenance, acceptance and world-effective time, and one
  deterministic contract per projection; add no universal evidence state, support
  counter, confidence score, graph database or server-side LLM. No concept
  correction yet.

## Technical vocabulary

- decided method / Terry: technical names must state their operational meaning in
  conventional English; metaphorical presentation terms never appear in schema, API,
  status or job identifiers. Current mechanics keep three separate axes: a package
  explicitly confirmed by its human becomes `accepted` after server validation; a
  place uses `verification_status = unverified | verified`; a claim projection uses
  `evidence_status = reported | corroborated`; and the source claim stores
  `ordinary_scene_can_supersede = true | false`. Any player-facing names are a
  separate later decision.
- decided method / Terry: KISS (“Keep It Simple, Stupid”) is an always-on build
  rule. Implement the smallest design that satisfies confirmed current behavior. If
  two designs work, prefer fewer concepts, tables, states, branches and moving parts.
  Complexity is added only when a concrete current scenario demonstrates that the
  smaller design cannot work; expected future scale by itself is not sufficient.
- decided, superseding every earlier universal evidence-status direction:
  `evidence_status = reported | corroborated` is retired. An accepted claim is
  immediately queryable and may enter a current projection only when that
  projection's deterministic contract permits it. Each contract defines its current
  key, authority, world-effective time and replacement behavior; every projected row
  retains its source claim id. A later citation, repetition, observation,
  contradiction or replacement appends its own claim and provenance and never
  promotes or mutates the earlier source claim. The previous corroborated-only kind
  traversal filter is also superseded: classification paths retain every source
  claim id and provenance, and any stricter source filter must be explicit.
  `verification_status = unverified | verified` remains only for the specific
  place-verification projection. Exact source-basis vocabulary and claim-reference
  storage remain open.

# 2026-08-07

## Game and MVP direction

- decided: Aicadia is developed as an MMO-like shared-world discovery and settlement
  game, not as a literary platform or collaborative-fiction database. Human players
  use their own MCP-connected AI agents to control characters and author changes to
  one persistent multiplayer world.
- decided method / Terry: all current design and implementation terminology uses
  conventional game-development, server and event-sourcing English. Technical terms
  name an actor, action, state or stored record; presentation metaphors do not enter
  schema, API or architecture. The initial canonical vocabulary is pinned in the
  root `CONTEXT.md`.
- decided method / Terry: the MVP is the filter for all subsequent work. The target
  loop is player connects agent → reads local world state → prepares and explicitly
  confirms a player action → server validates and appends one immutable world event
  → another player can read the resulting state and event. Work that does not decide,
  implement or verify this loop is deferred.
- decided: `world_event` replaces `scene` as the technical source-record term.
  `player_action` is the confirmed request; acceptance creates one immutable
  `world_event` containing narrative text, claims and provenance. `scene` may still
  describe narrative content but is not an implementation type.
- decided: `event_feed` replaces the technical use of ripple, catch-up, morning
  report and inbox; `action_context` replaces briefing, dossier and catch-up as the
  input required before action submission. Existing exploration documents retain
  legacy wording as historical material until their current direction is repinned.
- direction: player actions should create durable, queryable world structure through
  discovery, movement, construction, use and continuation. Self-contained narrative
  detail may remain in event text, but it does not by itself earn a world-state claim
  or a feed entry. These semantic categories guide agents and are not fixed action
  enums or domain tables.

# 2026-08-08

## Development method

- challenged and rejected: recommending another Agent playtest or validation pass as
  the next step was locally tidy but did not materially advance Aicadia as a game.
- decided method / Terry: Game Progress First. Select the highest-leverage concrete
  player or World outcome first, then use KISS to size its smallest safe slice.
  Validation, cleanup, plumbing and documentation lead only when they unlock that
  outcome or retire a concrete blocker or risk on the selected edge. The current
  contract bounds implementation but not ambition: once a slice is sufficiently
  proved, decide the next missing game behavior and update `docs/game/` before code.
- decided method / Terry: every completed unit of Aicadia work records its meaningful
  outcome or decision in this log and updates, corrects or removes every affected
  authoritative document in the same change. This is project memory, not a diary of
  shell commands.
- corrected method: the root builder works directly by default. Delegation is used
  only when requested or when bounded independent work materially helps; a mandatory
  writer subagent is ceremony and has been removed from the build skill.

## Spatial model

- researched: comparable persistent-game spatial models across EVE Online, Second
  Life, Luanti, Cataclysm: DDA, OpenTTD, Overture, PostGIS, H3 and OGC; report at
  `docs/research/persistent-game-spatial-model.md`. Recommendation for the current
  grill: a Place is an Entity role with the same stable id; the first discovery slice
  uses exact `place_id` equality for one Place-scoped discovery read, not as a
  universal visibility rule; boundaries, coordinates, containment, travel and engine
  cells remain separate later layers. No game-contract decision yet.
- researched: locality, co-presence and observation across Evennia, Smallville, AI
  Town, Concordia, Second Life, Luanti, Unreal, Overture and partial-map systems;
  report at `docs/research/locality-co-presence-and-observation.md`. There is no
  universal locality rule: direct Place, nesting, distance, access, observation and
  technical relevance are action-specific layers. The open grill question is
  corrected from universal visibility to exact inclusion in one Place-scoped
  discovery read. No game-contract decision yet.
- researched: World clocks, ticks, scheduled transitions, lazy catch-up, sparse
  regional simulation and stable procedural context across persistent games,
  engines, agentic worlds and real-world observation systems; report at
  `docs/research/world-time-and-sparse-simulation.md`. A clock supplies shared time
  but performs no work by itself; scalable worlds combine action-specific temporal
  rules with active regions, bounded catch-up or on-demand derivation. Stable
  transition identity and randomness are required to prevent time, retries, restarts
  or visit order from becoming rerolls. Whether and how Aicadia adopts World time or
  temporal processes remains an open grill decision.

# 2026-08-09

## Development method

- decided method / Terry, strengthening the earlier living-documentation rule:
  every Aicadia product, domain, behavior, architecture, implementation, evidence or
  operational choice is recorded when it crystallizes, including rejection,
  deferral, correction and supersession. Each record keeps the material reason,
  current status and affected scope; an unfinished grill maintains one active design
  record with confirmed and open branches rather than waiting for implementation.
  `AGENTS.md` and the build skill now enforce this across tasks.

## Stochastic discovery rolls

- researched: independent weighted rolls, rising and hard guarantees, entropy-like
  sequences, shuffle bags and variable result counts across Cataclysm: DDA,
  Warframe, Hearthstone, Apple GameplayKit and Path of Exile; report at
  `docs/research/stochastic-discovery-and-bad-luck-protection.md`. No compared model
  simultaneously bounds droughts, needs no cross-attempt state, removes the
  incentive to repeat and keeps future results unpredictable. Recurring hard or
  rising-odds protection is functionally a streak counter even when hidden or
  derived, while entropy and shuffle bags replace it with other durable sequence
  state. Idempotent delivery retries, operational rate limits and gameplay
  bad-luck protection solve separate problems. The open grill question is what
  concrete Character, Place or investigation change makes a request a new eligible
  attempt; no investigation-roll contract has been accepted yet.
- decided direction: prior empty investigations and elapsed time never improve a
  Character's later discovery odds. Every eligible new investigation resolves the
  same context-dependent independent chance table, including its possibility of
  zero, one or several results. There is no hard pity, soft pity, entropy sequence,
  shuffle bag or hidden accumulated luck. Delivery retries remain idempotent and
  operational rate limits remain separate safeguards. The exact rule that makes an
  investigation a new eligible attempt is still open, so this does not yet amend the
  executable game contract.
- decided direction: the World resolves the authoritative chance table before the
  Agent authors any discovery content. The World derives Character and Place,
  validates context, grants zero, one or several bounded opportunities and makes a
  delivery retry stable; the Agent then supplies the intelligent content inside the
  granted envelope. This deliberately introduces an unresolved completion boundary
  rather than letting Agent-proposed candidates influence the roll.
- recorded: the confirmed spatial, context, time, responsibility and stochastic
  discovery directions from the ongoing grill are consolidated in
  `docs/concept/10-discovery-and-world-context.md`. The record names every remaining
  decision and is explicitly non-executable; `docs/game/` remains unchanged until a
  smallest complete behavior is confirmed.
- corrected direction: the User does not choose a mechanical investigation focus,
  influence its chance or declare or confirm a discovery. The connected Agent—the
  only LLM in the flow—selects intelligently from current World state and authors
  within the envelope; the server-owned World derives context and owns validation
  and chance resolution. This supersedes the earlier statement that the User steers
  or chooses the investigation direction. Whether ordinary conversation may advise
  but never bind the Agent remains open.
- decided direction: ordinary User conversation may advise the Agent's investigation
  choice, preserving meaningful human participation without creating server
  authority. The Agent may follow, reinterpret or ignore expressed curiosity; User
  wording never maps directly to a mechanical focus, creates no eligible attempt,
  changes no probability and guarantees no investigation or result. This closes the
  advisory-influence branch while retaining the User's prohibition on discovery
  power.
- decided direction: a successful World roll grants zero, one or several bounded
  structural World-change opportunities, each with an allowed operation,
  authoritative scope and limits. The Agent supplies semantic content only after the
  grant. Blank counts are rejected because they delegate mechanical authority to the
  Agent; semantic recipes are rejected because they put content intelligence in the
  server. `World-change opportunity` remains a working design name until the full
  behavior and canonical nomenclature are accepted.
- reframed / open: persistence of a granted opportunity was asked too early. The
  result shape may combine the authoritative roll, structural envelope and relevance
  metadata derived from explicit World-state heuristics, allowing the Agent to see
  that the World currently lacks or would benefit from a kind of element. “Fun” may
  not become free-form server judgment or a universal score; it must decompose into
  inspectable rules such as missing local variety, repeated outcome shapes or an
  unresolved structural gap. The next decision is whether those heuristics select a
  binding direction or only advise the Agent.
- corrected direction, superseding the preceding open proposal: the World does not
  select or recommend a semantic direction such as `local_flora`. It returns the
  authoritative roll, structural envelope, applicable current meta-state and
  changes. The Agent alone evaluates heuristics, coherence, interest and fun value
  and decides what discovery fits. The server may derive mechanical data
  deterministically but owns no `fun_score`, semantic need or content intelligence;
  the exact raw-versus-derived meta boundary remains open.
- decided direction: the World returns authoritative facts plus rich, qualitative,
  neutral signals derived deterministically from them. Each signal may retain its
  affected scope, relationships, before/after state, time, provenance and inclusion
  rule so the Agent has enough connected evidence for an insightful judgment. A
  bare change code is rejected as too poor; ranking, scoring, value assignment and
  semantic recommendation are rejected as server intelligence. Packaging this rich
  context in one bounded response or progressive detail remains open.
- decided direction, closing that packaging choice: the roll response contains a
  rich qualitative overview plus typed stable references for deeper read-only World
  inspection. Every reference must be resolvable through a published Agent
  capability; dead ids are invalid. Required reads ship with semantic parity through
  `World`, HTTP and MCP and remain separate from investigations: following them
  consumes no roll, grants no opportunity and changes no state. The exact traversal
  and query surface remains open.
- decided direction: Agent inspection may query broadly across exposed World state,
  but the query contract stays flat, typed and deterministic. It uses allow-listed
  targets, filters, comparisons, stable ordering and cursor pagination, with
  explainable predicate matches. SQL, Cypher, GraphQL, recursive patterns, free-text
  interpretation and semantic or vector search are rejected; an Agent deepens its
  research through further bounded calls. Whether one call may combine several
  kinds of World state remains open.
- decided direction: every inspection query explicitly returns exactly one result
  kind. Filters may reference exposed fields and relationships, but mixed or joined
  result shapes are rejected; the Agent combines results through further calls.
  Every Agent-visible kind of established shared World state must have an explicit
  read and query path in the MCP catalog with the same semantic capability through
  `World` and HTTP. “Fully queryable” means complete composition across these typed
  tools, not an omnipotent query or enormous response; the visibility boundary
  remains open.
- decided direction, closing the shared-state boundary: all established shared World
  state, history, relationships, provenance and neutral meta-signals are Agent-
  queryable. Random seeds, hidden chance mechanics, authentication and rate-limit
  state, uncommitted Agent candidates and operational internals are not shared World
  state and remain outside game queries.
- decided direction: full Agent queryability covers two separately authorized
  scopes: all established shared World state and the current Character's complete
  personal state. Personal reads derive the Character from trusted User request
  context and never accept a caller-selected Entity id. Personal state need not
  be published into shared projections merely to be queryable; visibility of other
  Characters' personal state remains open.
- decided direction, closing the personal visibility boundary: an Agent may query
  its current Character's complete personal state but no personal state belonging to
  another Character. Other Characters are visible only through their established
  shared World state. This provides a fixed authorization rule without per-field or
  player-managed visibility controls.
- corrected direction: a positive roll is volatile because its relevance depends on
  changing World-state. It is not stored as a durable or stockpilable Character
  opportunity. The prior `introduce_entity`-style helper envelope is rejected as
  persistence structure masquerading as gameplay and as an overly prescriptive
  server decision. The roll now yields a state-bound roll result with
  generic limits and rich meta-context; the Agent inspects the World and submits one
  structured candidate World change, after which the World revalidates current state
  and commits only concrete accepted domain results. The exact two-call interface
  and staleness rule remain open.
- challenged / open: choosing a separate deep Investigation interface was paused
  because its ownership of concrete results was unclear. Applying the existing
  architecture and deep-module seam discipline yields a proposed map: `World`
  remains the sole public owner of game behavior, the Agent owns reasoning and
  candidate authorship, concrete domain behavior owns its invariants inside World,
  World owns roll/freshness/acceptance and the atomic commit, shared results belong
  to the World, and HTTP/MCP remain thin transport adapters. No new public module or
  internal seam is accepted yet.
- decided direction: the proposed ownership map is accepted. `World` remains the
  sole public game-behavior seam and owns investigation flow, roll, meta-view,
  freshness, acceptance and atomic commit. The Agent owns reasoning and candidate
  authorship; concrete Entity, Place, relationship and future domain behavior owns
  its invariants inside World; accepted state belongs to the shared World; HTTP and
  MCP adapters own transport only. Investigation gains no competing public interface
  and an internal module must still earn its seam from implementation evidence.
- corrected nomenclature: `resolution` was ambiguous with an Agent or transport
  session. The working domain term is now `roll result`: the output of exactly one
  investigation roll. A positive result may carry an opaque technical `roll_token`;
  neither term denotes stored Character state or a session.
- provisional direction: only changes to the bounded state and signals on which a
  roll depended make its result stale. Unrelated changes elsewhere in the immense
  World do not. The proposed high-velocity implementation is a server-authenticated,
  client-carried roll token with opaque dependency fingerprints, stateless
  verification by any World instance and one idempotent Postgres transaction only
  when a candidate is completed. Choosing this over pending database rows or
  process-local state remains open.
- decided direction: a volatile positive roll result is carried by an opaque,
  server-authenticated `roll_token`, not by an Agent session, process-local state or
  a durable pending-roll record. The token binds the derived Character, attempt,
  result limits, contract version and opaque fingerprints of its bounded relevant
  dependencies. Any World implementation instance can verify it; unrelated World
  changes do not stale it, and an accepted candidate commits idempotently in one
  Postgres transaction. Caches may accelerate reads but never determine correctness.
  The remaining eligibility decision must prevent an Agent from retaining one valid
  token while farming additional rolls without turning the token into stored
  gameplay state.
- decided direction: exactly one roll can be current per Character. When World
  admits a genuinely new roll it atomically replaces a small server-generated
  current-roll marker; the new `roll_token` binds that marker, so every older token
  becomes stale. A delivery retry with the same idempotency key remains the same
  roll. The marker stores no outcome, meta-state or candidate, needs no cleanup and
  creates contention only within one Character. This deliberately adds one small
  per-roll write to the earlier client-carried-token direction; no pending-roll row,
  Agent session, process affinity or global World lock is introduced. What World
  admits as a genuinely new roll remains open and must stay distinct from
  operational throttling.
- challenged and reopened: the one-current-roll-per-Character direction couples
  every concurrent conversation for the same Character. A roll in one conversation
  would silently invalidate unrelated investigation work in another, even though a
  conversation is transport context and must not become a durable World identity or
  authority source. The client-carried authenticated token remains accepted, but
  its concurrency rule is open again. Current recommendation: allow attempt-specific
  tokens in parallel, aggregate operational limiting across the Character rather
  than per conversation, and use bounded dependency overlap for optimistic conflict
  handling—an accepted change stales overlapping tokens while unrelated tokens stay
  usable. This recommendation is not accepted yet.
- decided and superseded: one Character may carry several attempt-specific,
  scope-bound `roll_token`s concurrently across any number of conversations.
  Conversation identity is not stored in or trusted by World, and operational
  admission limits aggregate across the Character, so opening another conversation
  never increases roll throughput. Accepted changes stale only tokens whose bounded
  dependencies overlap; unrelated investigation work remains usable. This
  supersedes the Character-global `current_roll_id` and its one-roll-at-a-time
  behavior, which created false conflicts between unrelated work. The authenticated
  client-carried token and optimistic transactional revalidation remain accepted.
- decided direction: an explicit Agent investigation request becomes a fresh
  eligible attempt only when World admits it before rolling. World derives the
  Character, applies one operational admission policy across all of its
  conversations, creates the attempt identity and performs the independent roll.
  Reusing an idempotency key is a delivery retry; a rejected request creates no
  attempt, roll or World change. User conversation, free text and asserted effort
  confer no eligibility. Operational limiting protects throughput but never changes
  odds or creates gameplay progress; its concrete mechanism and thresholds remain
  deferred until implementation requires them.
- process decision: the discovery grill pauses with five explicitly open design
  areas rather than pursuing speculative completeness. Chance-table derivation,
  multi-result composition, duplicate or contradictory candidates, exact committed
  records and the first vertical discovery behavior remain deferred. They are
  reopened only when a selected smallest end-to-end slice needs them; until then
  `docs/game/` and the executable MVP remain unchanged.
- built for evaluation: `docs/concept/discovery-roll-prototype.html` is one
  self-contained, in-memory logic lab for the question whether scope-bound
  optimistic rolls remain understandable across multiple conversations for one
  Character. Free play and four guided walkthroughs expose full World, Character,
  client-held token and accepted-package state after every action: unrelated scopes
  both commit, overlapping scopes stale, delivery retry creates one attempt, and
  closed Character-wide admission creates no attempt for either conversation. The
  forced zero/one/two branches and all fixture content are prototype controls, not
  accepted odds, schema or Agent input. DOM-free execution reached every expected
  terminal state, JavaScript syntax and DOM references were checked, and an offline
  render confirmed the visual hierarchy. No verdict has been inferred from building
  it; `docs/game/` and production code remain unchanged pending hands-on evaluation.

## Character production slice

- confirmed behavior: the current User may create and retrieve exactly one owned
  Character through User-context-derived World, HTTP and MCP operations. Ownership
  and one-per-User cardinality are server-enforced, creation is atomic under
  concurrent requests, and the Agent supplies no owner or Character selector.
  Place, movement, investigation, rolls, discovery, sessions and authentication
  remain absent.
- decided after Terry comparison: Character is a concrete Entity role, not a second
  World subject. `character.entity_id` is the role table's primary/foreign key and
  `character.owner_user_id` is the unique ownership relation. Domain and Agent
  results compose the complete Entity instead of copying its fields or relabeling
  `introduced_at` as Character creation time. Separate Character identity was
  rejected until accepted behavior requires Character continuity across different
  Entities; independent Character and Entity namespaces were rejected because they
  would make every future location, observation and relationship choose a subject
  kind. The complete contract and relationship/index meanings live in
  `docs/game/README.md` and `docs/game/agent-interface.md`.
- corrected spatial lifecycle: creating a Character introduces its durable identity
  and User ownership but does not place it in the World. A Character may therefore
  exist without an established Place. Initial placement is a separate future game
  transition whose actor, authority and result must be decided before adding Place
  storage; a nullable Place column without such behavior is rejected as unused
  plumbing. `docs/game/README.md` now states the current no-Place boundary, and the
  discovery design no longer assumes every Character already has a current Place.

## World history and development backlog

- confirmed product direction: every accepted state-changing game action must leave
  durable, queryable history sufficient to establish who acted, what the World
  accepted, when and where it happened, and which Characters, Places and other
  Entities were involved. Stable ids and explicit roles preserve this context;
  conversation text, rejected calls, reads and private Agent reasoning are not World
  history. The exact schema and public nomenclature remain open.
- decided build method: `.agents/backlog/README.md` is the compact ordered forward
  plan, with at most one active item and a detailed document only for current or
  near work. It does not compete with `docs/game/`, research or this log. Builders
  update current scope, state, dependencies and completion evidence in place while
  material choices continue to be recorded in their proper authority.
- proposed next edge, not yet executable contract: build World entry together with
  the first normalized activity-history spine. Character remains unplaced after
  creation; a later explicit action establishes spatial presence. Current state and
  append-only activity are written atomically but remain separate, avoiding both an
  unused Place column and premature event sourcing. The concrete proposal, open
  names and migration risk are maintained in
  `.agents/backlog/items/world-entry-history.md` pending acceptance.
