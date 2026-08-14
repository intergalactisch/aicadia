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
- decided for the Character-only slice and now superseded by accepted World entry:
  creating a Character introduces its durable identity
  and User ownership but does not place it in the World. A Character may therefore
  exist without an established Place. Initial placement is a separate future game
  transition whose actor, authority and result must be decided before adding Place
  storage; a nullable Place column without such behavior was rejected as unused
  plumbing. The later World-entry choice supplied that behavior and superseded the
  no-Place storage boundary while retaining deliberately unplaced creation.

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
- accepted and built next edge: World entry ships together with the first normalized
  `activity` history spine. The first connected Agent representing an existing
  unplaced Character may author the one entry Place's `name` and `description` with
  `create_entry_place`; World derives all ids and a unique database invariant allows
  exactly one concurrent winner. `enter_world` derives Character and entry Place,
  places only an unplaced Character and is delivery-retry safe without becoming
  movement. `list_activity` derives the current Character, accepts no Character id,
  selects actor-or-role-linked involvement exactly once and pages by
  `(occurred_at, id)` descending.
- accepted history model: current state remains authoritative. `activity` records
  accepted operation, responsible User internally, optional actor Character,
  optional context Place, occurrence time and normalized `activity_entity` links
  with server-owned `subject` or `destination` roles. `create_character`,
  `create_entity`, `create_entry_place` and `enter_world` write state and immutable
  history in one transaction. There is no JSON payload, replay, generic event,
  rejected-call logging, transcript, score or server inference. Player-visible
  personal history omits operational User provenance.
- accepted migration boundary: pre-history Character Entities backfill
  `create_character`; every pre-role non-Character Entity backfills `create_entity`.
  The operation, responsible User, subject and original Entity timestamp are exactly
  derivable. The old schema retained no actor Character or Place context, so both
  remain absent; no placement history is invented.
- accepted spatial boundary: `Place` is an Entity role with `entity_id` identity and
  Character holds one nullable current Place foreign key. Character creation always
  leaves it null. The one entry Place and explicit World entry are current behavior;
  coordinates, geometry, containment, routes, additional Places and movement remain
  deferred. The complete executable contract and wire surface live in
  `docs/game/README.md` and `docs/game/agent-interface.md`.
- accepted delivery correction: the implemented World-entry behavior must be
  understandable from the MCP catalog an external Agent actually receives, not only
  from repository documentation and adapter tests. The current ten capabilities and
  game semantics remain unchanged; their descriptions and generated output schemas
  will make unplaced Character state, genesis, entry retry behavior, Place identity
  and Activity roles explicit.
- accepted acceptance boundary: the live Agent playtest expands from only shared
  Entity visibility to the complete current flow. Agent A creates its Character,
  establishes the entry Place only after World reports that genesis is absent,
  enters, creates the shared fixture Entity and reads personal Activity. Agent B
  creates its own Character, enters the same server-derived Place, reads its own
  Activity and observes Agent A's Entity. Expected first-use game errors are part of
  this deterministic flow and must be validated exactly; arbitrary errors remain
  fatal. A paid live run remains gated by explicit token-spend confirmation, so
  local completion alone cannot mark the backlog item done.
- built and token-free verified: all ten generated MCP tools now carry the accepted
  meanings in their descriptions and output-field schemas, the MCP server instruction
  publishes the recommended entry sequence, and the exact catalog fixture pins that
  handoff. The disposable runner now validates the two-Character shared-Place flow,
  personal Activity operations and roles, and the existing cross-User Entity proof
  against both direct MCP evidence and authoritative HTTP state. Its fake failure
  matrix, all 38 Rust tests, formatting, strict lint and the real Codex/PostgreSQL
  preflight pass without invoking an Agent. This supplied the complete token-free
  basis for the separately gated live acceptance.

# 2026-08-10

## Agent world-entry handoff

- verified and completed: explicitly authorized paid live run `run-9TOG5yrJ` used
  two isolated `gpt-5.6-sol` Agents at high reasoning. Both created distinct
  Characters and entered the same server-derived entry Place; each personal
  Activity proof matched authoritative HTTP state, and Agent B observed Agent A's
  exact shared Entity. The runner dropped its disposable World after validation.
- evidence boundary: this confirms clean-room comprehension of the published MCP
  World-entry flow, Activity roles and shared Entity visibility. It does not prove
  later Places, movement, investigation, discovery or arbitrary semantic-content
  quality.
- backlog state: Agent world-entry handoff is Done. No next game-development edge
  was accepted automatically, so the backlog deliberately has no `Now` item.

## Build planning and execution

- corrected method: every non-micro Aicadia build must have one proportional durable
  plan under `.agents/plans/<YYYYMMDD-HHMMSS>-<slug>/plan.md` before implementation
  changes code, schema, executable behavior, authoritative documentation or
  operations. The plan aligns the strategic player or World outcome, tactical
  smallest complete slice, technical design and exact evidence; a small planned
  build may contain one task.
- accepted micro-change boundary: a change may skip the plan artifact and separate
  acceptance only when its outcome is unambiguous, its edit is local and reversible,
  it only restores or preserves accepted behavior, it introduces no product, domain
  or architecture choice, it touches no schema, migration, public contract,
  ownership/history semantics, auth/security/privacy, irreversible or external
  operation, material cost or token spend, and one focused check proves it. Line
  count alone does not qualify. Root first states the surface and check; if discovery
  breaks any condition, work stops and enters the formal planning workflow.
- decided gate: a plan remains `draft` while material product, domain, contract,
  irreversible-state, cost or evidence questions are unresolved. Grilling may
  resolve those questions, but implementation starts only after explicit User
  acceptance. Material scope learning returns an active plan to `draft` for renewed
  acceptance; in-scope file, ordering and evidence refinements remain executable.
- decided execution model: plans are forward state rather than diaries. Tasks name
  dependencies, allowed surfaces, concrete actions, evidence and stop conditions so
  root or a bounded delegated Agent can execute them without reconstructing scope.
  Root retains scope, integration and final evidence ownership; delegation and
  parallelism remain optional and must reduce risk or latency.
- anti-ceremony boundary: read-only explanation, orientation, status and diagnosis
  do not require a plan. Plans do not replace `docs/game/`, the concept log,
  research or backlog, and contain no estimates, points, deadlines or named owners.

## Exact-Place established-state read — active grill

- confirmed selected edge: the next game outcome is a User-context read of current
  shared Entity state attached directly to the Character's exact current Place. The
  Agent supplies no User, Character or Place id; exact stored Place equality is the
  complete first-slice inclusion rule, not universal visibility or co-presence.
- confirmed authority boundary: `list_entity` remains the current global shared
  catalog, while Activity `context_place` remains immutable historical context and
  cannot be inferred into current Entity location. No `docs/game/` behavior or code
  changes until the active grill and draft plan are accepted.
- clarified current contract: `create_character`, not `create_entity`, introduces
  the current User's Character Entity role; only Character currently has nullable
  Place state, becoming placed through `enter_world`. Generic `create_entity`
  introduces a stable shared referent and currently stores no Entity location.
- reopened action split: a distinct `create_entity_at_current_place` was initially
  accepted, then challenged as needless public orchestration. The current
  direction is one `create_entity` capability. The User rejected
  `at_current_place: bool` in favor of required nullable `place`, containing a Place
  Entity UUID or `null`; `null` creates an unlocated referent. A nested Place selector
  is unnecessary because Place already has one stable Entity identity.
- corrected target rule, superseding the preceding current-Place restriction: a
  non-null `place` may name any existing Place. It is the new Entity's placement, not
  evidence of where the User's Character is, and it is not compared against
  `character.current_place_entity_id`. An id without a Place role returns
  `place_not_found`. Whether located creation requires a Character at all is reopened.
- accepted applicability: an ordinary Entity may have optional current Place state;
  absence remains valid, while most concrete in-World occurrences are expected to be
  placed as content rather than by schema mandate. This slice creates location only
  atomically with a new local Entity and does not locate or move an existing Entity.
  Character keeps its existing nullable placement state; Place remains the anchor.
- accepted result model: the formerly accepted local-only composed result no longer
  fits a unified create action. Preserve the existing complete `Entity` return: the
  caller already supplied the nullable Place, success confirms acceptance, and the
  contextual read exposes authoritative membership. A one-off `CreatedEntity`
  wrapper would widen the interface without new behavior. Base `Entity` remains
  unchanged and one optional `entity_location` relation stays keyed by Entity
  identity.
- new User direction / open onboarding design: eventually a new User chooses one of
  three Character candidates before ordinary play. No current contract, code or
  prior concept record defines who produces those candidates, whether they persist,
  or whether selection also enters the World. Current provisioning deliberately
  permits a User without a Character.
- accepted onboarding boundary: absence of Character is valid transient
  onboarding/provisioning state but never a playable state. On an explicit onboarding
  request the User's own Agent may propose three transient candidates; only the
  selected option becomes the User's one durable Character Entity. Building this
  selection flow remains a separate later edge, while contextual game actions reject
  pre-selection requests explicitly.
- accepted onboarding composition: keep Character creation, any first-World genesis
  and World entry as separate accepted domain actions composed immediately by one
  guided onboarding flow. This preserves honest interrupted state and the existing
  genesis branch without exposing the intermediate unplaced Character as ordinary
  play; resume continues entry when `current_place` remains absent.
- corrected context errors: a supplied id that is not an existing Place yields
  `place_not_found` and HTTP 404. The Exact-Place read requires a placed Character:
  missing Character retains `character_not_found`/404 and an unplaced Character
  yields `character_not_entered`/409; the read never returns a misleading empty page.
  Create's Character prerequisite is open again.
- planned vocabulary and results: retain
  `list_entity_at_current_place`, returning the complete derived Place, Entity
  summaries and pagination through GET `/api/place/current/entity`. Remove the
  proposed public local-create operation. Both forms record Activity operation
  `create_entity`; located creation additionally links its target Place with involved
  Entity role `location`. Optional actor `context_place` remains where the Character
  was, so a target Place elsewhere cannot falsify actor context.
- clarified term: “current Place” is the durable Place referenced by
  `character.current_place_entity_id`, set by accepted World entry and re-derived
  from User → owned Character on every contextual read. That read accepts no selector.
  Consolidated create's `place` instead names the new Entity's placement and need not
  equal Character current Place. Current Place is never a coordinate, session value
  or inference from Activity history.
- accepted retry semantics: located and unlocated `create_entity` remain
  non-idempotent. Every accepted call creates one new Entity even when content and
  Place equal an earlier call; no idempotency key or retry-state model is added.
- accepted create response: consolidated `create_entity` keeps returning complete
  `Entity`; it does not introduce a composed wrapper.
- sharpened actor/authority model: an Agent authors and submits candidate content on
  a User's behalf; the User supplies accountable provenance; only `World` validates,
  assigns stable identity and creates accepted durable Entity state. An unaccepted
  candidate is not a World Entity. `create_entity` names the requested World action,
  not an Agent-side direct write.
- reopened caller authority: “meta Agent” is not a current domain actor. Old concept
  exploration contains an administrator-operated world steward, but current
  `docs/game/` has no admin authentication or privileged provenance and exposes
  `create_entity` to every User Agent. Decide whether this build preserves equal
  submission or deliberately introduces a meta-admin boundary. Revised recommendation
  after the User's challenge: arbitrary-Place creation is World-steward behavior, so
  raw `create_entity` should not remain a generic player tool; World creation should
  be invoked through a consciously authorized meta flow or a later concrete gameplay
  acceptance flow. This may split the write from the current read plan.
- accepted caller boundary: raw `create_entity` must no longer be a generic
  player-facing HTTP or MCP capability. It remains authoritative World behavior
  invoked only through a deliberately selected meta or gameplay acceptance flow.
  This explicitly evolves the current ten-capability contract; `docs/game/`, code and
  the always-on MVP surface stay unchanged until a complete replacement plan is
  accepted and executed.
- next open dependency: select the first concrete caller before implementing Entity
  placement or its Exact-Place read. Recommendation: define the consciously operated
  meta/steward proposal, explicit human confirmation, administrator provenance and
  deterministic World acceptance first; Character dependence and proposal lifetime
  belong to that flow. The current read draft is not implementation-ready.

## Meta-steward Entity acceptance — rejected interpretation

- selected edge: build the consciously operated meta/steward acceptance flow before
  Exact-Place reading. Arbitrary-Place Entity authorship is World seeding rather than
  Character play; the Exact-Place draft is deferred until a legitimate writer exists.
- confirmed authority chain: a meta Agent authors transient candidates, an
  administrator explicitly confirms the exact irreversible package and only World
  validates, assigns identity and creates durable state. The Agent has no World
  identity or authority; unconfirmed, unchosen and rejected candidates leave no state.
- confirmed public boundary: raw `create_entity` is removed from player HTTP/MCP and
  remains World behavior behind a private operator flow. Global Entity reads remain;
  a later concrete gameplay flow may earn access to the same World behavior.
- prior direction reused but not blindly imported: the old world-steward record
  supports conscious invocation, do nothing, private proposals, explicit final human
  confirmation, private meta provenance and no server-side Agent. Its obsolete
  scene/claim package does not govern the current Entity implementation.
- current technical recommendation: use a repository-local steward skill plus a
  private stdin/package CLI modelled after existing provisioning binaries. Avoid an
  admin HTTP/MCP surface, general authentication, server-side inference and durable
  proposal state.
- first open grill decision: whether one invocation presents exactly three candidate
  directions plus do nothing before drafting and separately confirming one exact
  Entity package. Resolve this and later dependent choices in
  `.agents/plans/20260811-083145-meta-steward-entity-acceptance/plan.md`.

## Agent-requested World Entity creation — corrected active grill

- rejected interpretation: a human administrator, explicit human confirmation,
  private meta-steward Agent, privileged provenance and private operator CLI are not
  part of this outcome. The preceding caller-boundary and meta-steward sections are
  superseded; their draft plan and backlog item are retained only as dropped history.
- corrected authority chain: an external Agent makes one explicit `create_entity`
  request on a User's behalf. World alone determines whether the command is allowed,
  assigns identity, atomically creates Entity, optional placement and Activity, and
  returns the accepted Entity or a deterministic error. The Agent requests; it never
  writes World state directly.
- corrected autonomy boundary: World acts authoritatively in response to that one
  call. The server does not invoke an Agent, run an LLM, spend tokens, or create
  Entity state without an explicit request.
- corrected public boundary: retain `create_entity` as one player-facing capability
  through World, HTTP and MCP. Its CRUD name describes the requested World command;
  it does not grant the caller database or acceptance authority. No separate
  proposal resource, confirmation step or orchestration layer is introduced.
- current open grill decision: define the deterministic acceptance predicate behind
  “suitable/possible.” Recommendation for the smallest strict slice is valid User
  context, bounded normalized name and description, `place: null` or an existing
  Place, and successful transaction constraints only. Semantic quality, lore fit,
  plausibility and duplicate meaning remain Agent reasoning rather than World rules.

## Agent-mediated bundled World action — accepted build

- corrected selected interaction: a User consciously requests a next action. The
  Agent first queries established World state and the current Character, reasons over
  that context and presents exactly three grounded proposals. The User selects one
  and may add free steering; the Agent then prepares the final World submission.
- corrected commit scope: the ordinary player flow should send one bundled mutation
  call. World validates and commits every accepted consequence atomically—potentially
  creating several Entities, changing existing state, changing Character placement
  and recording the readable narrative layer—then returns the canonical result. A
  rejected package changes nothing.
- corrected readable layer: `Story` is not a currently intended domain object. The
  historical working term is `prose`: human- and Agent-readable narrative text that
  forms the World story across its linked subjects and meta-lenses. The executable
  name and whether any current summaries exist alongside immutable action prose
  remain open.
- confirmed intelligence split: the Agent authors both prose and explicit structured
  consequences. World never interprets prose to discover database mutations; it
  checks the structured package against deterministic domain rules and applies its
  own internal write plan.
- confirmed interaction/transport distinction: one User action does not imply one
  total MCP call. The Agent may make several granular reads and MCP may expose
  granular domain mutation capabilities for changes that are independently valid.
  The common bundled submission is nevertheless one public World command, one
  transaction and one Activity footprint; it is not implemented as sequential public
  MCP writes and cannot partially succeed.
- confirmed absolute authority invariant: every MCP capability is an adapter to
  `World`, never a storage interface. An Agent can inspect returned state and submit
  intelligent proposals, but can never insert, update or delete durable records
  directly. World alone deterministically validates each granular or bundled command,
  decides acceptance and performs every accepted state change.
- current interface recommendation: share concrete domain validators beneath both
  bundled and standalone commands. The bundle contains a closed set of typed domain
  consequences and temporary local references for records created inside the same
  package; reject a generic SQL/JSON-patch or arbitrary CRUD batch that shifts
  ordering, invariants and partial-failure handling into the Agent.
- superseded planning: direct `create_entity({place})` plus Exact-Place reading is no
  longer the selected player outcome. Exact-Place inspection remains a likely
  supporting context query; its combined read/write plan is dropped. The current
  edge is the Agent-mediated World action, still design-only and absent from
  `docs/game/`.
- resolved grill question: accepted action prose is immutable history; the accepted
  lifecycle and its remaining time-axis question are recorded below.
- selected first evidence scenario: after both Characters enter the shared entry
  Place, one User chooses and steers a trail-marker direction from three private Agent
  proposals. The final submitted package contains readable prose and one structured
  Entity introduction; World derives current Place, atomically creates and places the
  marker with one Activity, and another Character there can read both marker and
  prose. This deliberately defers updates, movement, multiple consequences and a
  generic action engine. The draft build plan is
  `.agents/plans/20260811-124550-first-agent-mediated-world-action/plan.md`.
- accepted prose lifecycle: prose belonging to an accepted World action is immutable
  and append-only. No later action, User, Agent, World behavior or operational path
  edits or deletes it. World-, Character-, Place- and Entity-oriented history must
  reference the same canonical prose record and preserve one chronology; mutable
  current descriptions or future summaries are separate state, never replacements
  for historical prose.
- accepted chronology: the current system has one time axis, assigned by World when
  it accepts an action. Every history lens orders the same Activity/prose records by
  that acceptance chronology. Agents cannot supply a backdate, change order or insert
  prose into the past. Prose written now about an earlier subject remains a new
  present accepted action; a separate world-effective time is deferred until a
  concrete delayed or historical action requires it.
- accepted final confirmation: selecting and steering one of three proposals approves
  the direction but does not authorize the exact irreversible World package. The
  Agent must show the complete final prose and structured consequences and receive
  one explicit User confirmation immediately before calling `submit_action`. Reads,
  proposals and workshop revisions require no confirmation; the server cannot prove
  the conversational confirmation and relies on the published Agent contract.
- accepted context composition: “local context” is not one World payload. The Agent
  orients itself through several typed MCP reads, including World, Character, exact
  current Place and relevant Place surroundings, and may drill into returned
  references. A Place neighborhood is a bounded view over explicit spatial
  relationships such as containment and adjacency; it is not a literal metric or
  coordinate radius and does not imply visibility.
- accepted first spatial boundary: the first Agent-mediated action proves the whole
  interaction at the existing exact entry Place. It does not introduce additional
  Places, containment or adjacency merely to demonstrate context. Exact Place Entity
  and Activity/prose reads are supporting capabilities inside that action build, so
  their separate backlog item is dropped; a bounded Place-neighborhood read remains
  a later spatial edge.
- reopened retry and freshness contract: the User proposed obtaining a unique token
  from an MCP context read and returning it with `submit_action`. One World-issued
  value must not silently mean both delivery identity and observed-state revision:
  two Agents can legitimately observe the same state but intend different actions,
  while one Agent can retry the same intended action after an uncertain response.
  The open recommendation is therefore a separate per-intent `request_id` for
  idempotent delivery and an opaque World-issued context revision for optimistic
  freshness when accepted scope proves which state must remain unchanged. World
  computes any normalized payload fingerprint itself; payload equality is not action
  identity.
- researched retry/freshness recommendation: primary HTTP, UUID, PostgreSQL and
  production API evidence is recorded in
  `docs/research/idempotent-action-delivery-and-place-freshness.md`. The smallest
  robust Aicadia interface uses three distinct values: an Agent-generated UUID for
  one intended action, a versioned fingerprint World derives from normalized input,
  and an opaque exact-Place revision returned by side-effect-free local reads. World
  checks request identity first and, for an unseen request, locks the derived current
  Place before checking its revision and writing. A stale same-Place package changes
  nothing; unrelated Places do not invalidate it. A read-issued request nonce,
  global World revision, pending action/session and cross-MCP database snapshot are
  rejected recommendations. At this point it remained research-backed draft direction
  pending the User acceptance recorded immediately below; `docs/game/` was unchanged.
- accepted retry/freshness contract: the User accepted the researched three-value
  separation and exact-Place scope. One Agent-generated request UUID identifies an
  intended action, World derives a versioned normalized fingerprint, and coherent
  exact-Place reads return the opaque revision submitted as
  `expected_place_revision`. World resolves accepted request identity before current
  preconditions, and for an unseen request locks the current Place before comparing
  revision and writing. Same-Place change rejects the whole stale package; unrelated
  Place activity does not. This accepted design remains absent from `docs/game/` and
  implementation until the complete draft plan is accepted.
- accepted live evidence gate and spend: a paid live Agent playtest is mandatory for
  completion because the User wants evidence that the published interaction works in
  a real clean-room Agent, not only in deterministic tests. The User explicitly
  authorized one bounded paid run on 2026-08-11 after token-free preflight. Extend the
  existing disposable two-Agent harness to resume one action Agent across grounded
  proposal, withheld selection/steering, exact no-write preview and explicit
  confirmation before its sole commit; a separate Agent and HTTP state verify the
  result. A failed or inconclusive attempt does not authorize an automatic rerun.
  Current local `codex-cli 0.144.1` differs from the existing `0.147.0` harness pin,
  so version/resume compatibility is a pre-spend blocker, not a reason to weaken or
  bypass the gate.
- accepted plan and installed contract: on 2026-08-11 the User explicitly accepted
  the complete strategic, tactical, technical and evidence plan, then requested
  `gpt-5.6-sol` high-reasoning sub-Agents to build it. `docs/game/` now governs the
  thirteen-capability slice: one closed `introduce_entity` action, exact-Place Entity
  and Activity/prose reads, immutable canonical prose, atomic World acceptance,
  per-User request UUID/fingerprint retry identity and an opaque Place revision.
  Proposals and confirmation remain private Agent obligations; implementation and
  the single authorized clean-room run remain completion work.
- rejected and corrected implementation detail: selecting the current Place revision
  with `MAX(occurred_at, activity_id)` does not establish acceptance order. Equal
  timestamps make UUID order accidental, and a database clock rollback can make a
  later accepted Activity compare older, so a successful Place mutation could fail
  to advance freshness. The Activity tuple remains stable revision-target identity
  but is not an acceptance-order rule.
- accepted Place freshness authority: every Place stores one authoritative
  `latest_activity_id`. `create_entry_place` sets its preallocated genesis Activity
  pointer when inserting the Place; `enter_world`, placed-actor `create_entity` and
  `submit_action` lock the Place, insert their Activity and atomically update the
  pointer to that Activity. Exact-Place reads encode the pointed Activity's
  `(occurred_at, activity_id)` as identity. This is local current state under the
  existing Place lock, not event sourcing and not a global revision or counter.
- rejected live candidate and consumed authorization: the one authorized candidate
  `run-G8k1sTRm` ran on 2026-08-11, but the API rejected its proposals response schema
  with `invalid_json_schema` because `uniqueItems` is unsupported. Rejection happened
  before model generation and before any MCP tool call, so no Agent interaction or
  World outcome occurred. Ownership was verified before cleanup dropped the
  disposable database. The attempt consumed the sole authorization; it is not a
  passing or retryable result.
- accepted token-free correction and remaining evidence gap: Agent output schemas no
  longer use `uniqueItems` or the redundant `minLength`/`maxLength` string keywords;
  the controller retains exact uniqueness and bounds checks. Recursive local
  supported-keyword and strict-object schema validation plus its fake regression are
  token-free green, as are public preflight and the full test suite. The complete
  harness contract and candidate record live in
  [Agent playtest](../../game/agent-playtest.md).
- accepted fresh one-run authorization: after the token-free schema correction and
  full ladder passed, the User explicitly authorized exactly one rerun on 2026-08-11.
  This authorized one new live candidate only, never an automatic third attempt. At
  that point the Agent-mediated World action remained Active and incomplete pending
  the candidate result recorded immediately below.
- rejected second candidate with strong partial live evidence: fresh authorization
  was consumed by `run-nvULnvxQ` on 2026-08-11. Proposal, preview and commit phases
  passed. The observer exited `0`; its exact three MCP reads found the correct Entity
  id and name, Place, Activity and canonical prose. Its strict final failed solely
  because the harness required `entity_description`, which the granted
  `EntitySummary` does not expose. Independent HTTP validation was therefore not
  reached. Ownership-verified cleanup dropped the disposable database and left zero
  matching databases. This materially supports the live interaction, commit and
  observer path, but does not satisfy the complete outcome gate.
- accepted observer-evidence correction and remaining gate: remove the unobservable
  observer description without adding a redundant `get_entity` capability. The
  observer continues to prove Entity id/name, Place and prose; independent HTTP
  validation still checks the complete Entity description. Fake regression, public
  preflight and the full test suite are token-free green. The authoritative candidate
  and harness record is [Agent playtest](../../game/agent-playtest.md). No third run
  is authorized; the backlog item remains Active and incomplete pending a passing,
  separately authorized live run.
- accepted T4 recovery and plan re-acceptance: freeze T1-T3 and every public game
  surface. Give each evidence layer one role: the action Agent proves the private
  workshop and single submission, HTTP proves the complete authoritative stored
  result, the three-read observer proves clean-room MCP discovery of Entity
  id/name/Place/prose, and the ownership helper proves safe isolation and cleanup.
  Run HTTP validation immediately after commit and before the observer so the
  authoritative result is independently retained; both still must pass continuously
  in one candidate. Do not add `get_entity`, restore observer description, combine
  partial candidates or reinterpret either rejected manifest. The existing plan was
  returned to `draft`, then explicitly re-accepted by the User on 2026-08-11.
  Re-acceptance authorizes only token-free harness work; one future paid candidate
  still requires a separate explicit authorization after an independent frozen GO.
- implemented T4R1 evidence order: the runner now retains the accepted commit, then
  validates the complete authoritative Entity/description/Place/Activity/prose over
  HTTP before it starts the three-read observer. A failed HTTP gate leaves observer
  pending; a failed observer retains HTTP as passed but never completes the run.
  Live-shaped fake started/completed attempt pairs, duplicate/incomplete submission,
  wrong observer values, authoritative duplicates and cleanup refusal are token-free
  green, as is public preflight. No public game capability changed and no live/model
  call occurred. The runnable evidence surface is frozen for independent T4R2 audit.
- first T4R2 audit NO-GO and bounded correction: authoritative HTTP already checked
  the Activity operation, prose, Place, subject and location and fetched the action
  Character, but failed to compare `actor_character` with that Character's id. This
  could admit a wrong-actor Activity despite the documented evidence claim. Reopen
  T4R1 only for that comparison and a fail-closed `http-wrong-actor` fake case, then
  rerun the whole frozen audit. All other checks passed, no other P0-P3 finding was
  reported, zero resources remained and no live/model call occurred.
- implemented actor-evidence correction: authoritative HTTP now requires the one
  canonical `submit_action` Activity's `actor_character.id` to equal the action
  Character id independently returned by `/api/character`, while preserving the
  existing operation/prose/Place/subject/location/description/count assertions. A
  new wrong-actor fake case stops before the observer, records HTTP and validation as
  failed, retains the passed commit and performs ownership-verified cleanup. Focused
  fake/preflight evidence is green; the full T4R2 audit is restarted token-free.
- restarted T4R2 independent GO: executable candidate fingerprint
  `95600a0777a1375a310ee079254dbbcaf43ae123a921671bf2ed4d971c2a37f9` passed 57 Rust
  tests, 27 fake runner invocations, 19 fail-closed modes, public preflight and
  database/process/isolation inspection. The prior actor finding is closed, no
  unresolved P0-P3 finding or leftover resource remains and no live/model call
  occurred. The executable candidate is frozen; plan acceptance still did not
  authorize spending. T4R3 requires one fresh explicit authorization and can run
  only one candidate without retry.
- accepted one-candidate T4R3 authorization: after receiving the independent frozen
  GO, the User explicitly authorized exactly one paid live candidate on 2026-08-11.
  Starting it consumes the authorization regardless of outcome. It permits no phase
  retry, second candidate or executable change and does not predetermine completion;
  T4R4 must reconcile the exact retained evidence afterward.
- executed sole T4R3 candidate pending independent closure: `run-gE8iED5m` consumed
  the authorization and the runner reports every proposal, preview, exactly-one
  commit, authoritative HTTP, minimal observer and ownership-cleanup gate passed in
  one continuous candidate with `run_status: completed`; no retry occurred. This is
  not yet promoted to final outcome evidence: T4R4 must independently compare raw
  JSONL attempt identities/statuses, finals, canonical ids/prose, HTTP counts/roles,
  file permissions and cleanup before the backlog or plan becomes complete.
- independently verified live completion: read-only T4R4 review found no P0-P3
  finding, staleness or evidence drift in `run-gE8iED5m`. It directly confirmed four
  ordered grounding reads, three private proposals, correctly withheld selection and
  confirmation, a zero-tool preview, exactly one submission attempt/result, complete
  authoritative HTTP actor/Place/subject/location/description/prose/count state and a
  separate three-read observer with no hidden expected ids, prose or description.
  All forty retained artifacts are private; ownership cleanup is `dropped`; zero
  database, process, listener or isolated-config leftovers remain. The bounded first
  Agent-mediated World action is complete, its backlog item is Done and neither this
  proof nor the two earlier candidates authorize or select the next game edge.

## Local Agent play and World ledger — accepted build

- selected next player outcome: make the proven World action usable as an ordinary
  persistent local playtest. One seeded development User can return to the same
  Character, Place, Entity state, Activity and prose without accounts or
  authentication. A small browser ledger makes that World and its domain models
  inspectable while the external User-owned Agent remains the sole conversational
  game interface.
- confirmed absolute conversation boundary: the game conversation always occurs in
  the Agent and never in the web interface. The browser ledger contains no chat,
  prompt/composer, proposal picker, confirmation control, model invocation, Agent
  bridge or server-side inference. It may only navigate, filter, expand, refresh and
  otherwise inspect authoritative World data.
- confirmed authority boundary: onboarding proposals, User steering, previews and
  confirmations remain private Agent conversation. Every accepted mutation still
  crosses MCP into `World`; the browser ledger cannot create a Character, enter the
  World or submit an action. PostgreSQL remembers only accepted game state and
  Activity/prose, while one private local development profile remembers the seeded
  User identity needed by both Agent context and ledger reads.
- accepted local-operation boundary: one local launcher must
  reuse a persistent development database and seeded User, start the existing server,
  expose a same-origin read-only data-dense ledger and print the exact MCP connection
  context for the User's Agent. The Agent should use the existing World-entry and
  private-workshop contracts, extended only with three transient Character proposals
  and exact confirmation before the existing `create_character` call. No web
  framework, auth/account model, durable chat/session or new World mutation is earned
  by this first playable slice.
- corrected ledger information scope: the User challenged dedicated current
  User/Character/Place presentation as premature. That correction is sound: the
  seeded User is local connection context for the Agent and read adapter, not a
  player-facing account or dashboard subject. A first ledger need not explain or
  visualize every domain role merely because the server stores it.
- accepted smallest ledger scope: show only that the persistent World exists and
  remembers accepted changes. One page contains a World
  identity/connection state, a shared Entity ledger with expandable record details,
  and the local Character's accepted Activity/prose ledger when onboarding has made
  that contextual read available. Character and Place may appear naturally as typed
  actor, context and involved-Entity references inside Activity, but receive no
  separate current-state panel, map or navigation model. Before Character onboarding,
  the ledger honestly has no personal Activity rather than turning that absence into
  web onboarding. Reuse existing reads; add no web-only game projection or mutation.
- accepted planning and execution state: the complete strategic, tactical and
  technical plan is
  `.agents/plans/20260812-091744-agent-only-local-play-ledger/plan.md`, tracked as the
  one active `Now` backlog edge. It deliberately targets Codex CLI first because the
  existing project MCP config can receive the stable local User context through one
  environment-scoped command. The User explicitly accepted the plan and authorized
  one new `gpt-5.6-sol`/`xhigh` orchestrator task to build it, including delegation
  of dependency-ready plan tasks to `gpt-5.6-sol`/`high` sub-Agents. This execution
  authorization does not authorize an automatic Aicadia gameplay/model playtest;
  that remains a separate conscious User action as defined by the plan.
- corrected stable-identity implementation: independent review proved that an
  existing database with a missing profile and two concurrent first launchers could
  each provision an extra User. The launcher now holds one private profile-lifetime
  lock and refuses any existing selected database without its matching profile. The
  expanded disposable lifecycle proves both cases leave exactly one User and no
  second listener. A lower-severity observation remains that a credential-bearing
  PostgreSQL URL appears briefly in local database-client process arguments; the
  private profile itself still stores no credentials.
- completed deterministic local-play outcome: one disposable continuous setup
  created Character `e28a709e-2555-4691-8df1-b287dd5bc9e8`, entry Place
  `0976bb79-2971-4778-afa4-409065638537`, Entity
  `adbb6fbd-941c-411c-8444-f7c052bfdf7d` and Activity
  `480d9424-f35d-43ed-a8f2-bb8433efb8ca` with canonical prose. After launcher
  restart, the User and complete canonical HTTP state were byte-identical and a
  fresh browser page rendered the same visible ids/prose while scrubbing the URL
  fragment and hiding the User UUID. The full compiler/test/integrity ladder and an
  independent risk review passed; owned disposable resources were removed. No Agent,
  OpenAI API or model-driven gameplay run occurred, so Character-candidate quality
  remains the User's next conscious experiment rather than a completion claim.
- observed future direction, not a selected edge: the User ultimately wants to host
  a public read-only World interface under a domain name. The local embedded page is
  a viable presentation base, but the current no-auth server and personal Activity
  context are not a public deployment contract. Public read exposure, mutation/MCP
  isolation, Activity visibility, TLS/process operation and durable hosted Postgres
  require a separately accepted build before public launch.
- corrected checkout boundary: the User confirmed that the primary repository
  checkout is the intended delivery and run location. The completed local-play build
  was integrated there byte-identically and its full compiler, test and launcher
  validation passed from that checkout. The Codex worktree is only temporary task
  isolation, not an Aicadia runtime or deployment requirement.

## Current-only Agent play contract — accepted build

- observed player-experience gap: the first ordinary local Agent conversation
  completed Character creation, World entry and one shared action correctly, but
  surfaced UUID generation, exact packages and commit language that belongs to
  internal execution rather than the User's experience of the World;
- accepted common contract: every full interactive Agent host receives the same
  provider- and model-neutral Aicadia instructions and complete tool descriptions.
  The Agent uses fixed grounded methods for Character creation, entry, orientation,
  action proposals, confirmation, mechanics explanation and conflict recovery while
  remaining free in wording and reasoning;
- accepted truth boundary: typed World results and structured consequences are
  authoritative. Agent framing and free prose cannot establish additional current
  state, ownership or mechanics. Ordinary player-facing communication follows the
  User's language and hides MCP, tools, JSON, ids, UUID generation, request ids,
  revisions, commits, retries, servers, databases, internal record categories,
  fields and absent-value syntax for the entire play conversation. A request for
  implementation detail moves to a separate development conversation rather than
  opening a technical submode;
- accepted current-only rule: Aicadia implements only current accepted behavior and
  current open standards. Superseded modes, compatibility paths, fallbacks, dormant
  flags, dead-code suppressions and their tests/current documentation are removed
  together. Historical records remain history. Client diversity follows semantic
  capabilities rather than provider, model, tool or client allowlists;
- accepted protocol consequence: Aicadia supports only stateless MCP `2026-07-28`.
  Its Aicadia-owned `2025-11-25` session mode, helpers, tests and current support
  claims are removed rather than deprecated;
- accepted strict/free seam: World continues deterministic validation of typed
  context, state, consequences, idempotency, freshness and atomic Activity. It does
  not inspect or score private conversation, infer prose meaning or claim it can
  prove a human confirmation. MCP prompts, resources, bootstrap tools, prompt
  storage, narrative linters, provider mappings and server-side model calls are
  rejected for this slice;
- accepted evidence boundary: deterministic evidence proves the exact current
  contract is published, the thirteen capabilities remain aligned and a second User
  observes the same accepted state. It does not prove identical LLM wording or
  universal compliance. No new live/model-driven playtest or token spend is
  authorized by this build;
- corrected post-completion evidence: raw requests against a fresh binary proved the
  SDK's stateless `initialize` path still returned success, including an echoed
  `2025-11-25`, despite current discovery advertising only `2026-07-28`. The earlier
  old-version `tools/list` probe did not exercise this lifecycle path, so the
  current-only completion claim was premature;
- accepted lifecycle correction: reject `initialize` explicitly at the existing
  `ServerHandler` seam and set current `ServerInfo` metadata defensively. Do not add
  middleware, a version matrix, session abstraction or compatibility branch;
- accepted cross-User content hierarchy: typed structure remains authoritative game
  state, while every natural-language World value is potentially player-authored
  game data and never an instruction to an Agent. Such content cannot override the
  Aicadia contract or User intent, authorize a tool call or request technical access.
  This general rule replaces field-by-field classifications, pattern scanners,
  sanitizing allowlists, narrative linting and server-side inference;
- completed correction: the existing `ServerHandler` seam now rejects every
  `initialize`, reports current protocol metadata and advertises only `2026-07-28`.
  The one global contract applies one instruction/data hierarchy to all World values.
  No middleware, version matrix, content taxonomy, filter, linter, classifier, model
  call or World change was introduced;
- completed corrected evidence: raw requests against a fresh binary proved current
  discovery, current and old initialize rejection and old inline-call rejection.
  Formatter, strict all-target Clippy, all 58 Rust tests, the disposable launcher
  lifecycle and diff integrity passed again; the launcher reported
  `codex_invoked=false`, and owned database/listener resources were removed. Final
  review found no remaining P0-P3 issue in scope. The plan and backlog item are Done.
  The pre-existing process on port 3000 remains intentionally untouched and needs a
  conscious restart before a new Agent conversation can receive the corrected
  contract.
- observed host and narration failure after that correction: an ordinary Agent start
  omitted Codex's current MCP feature, failed startup, then silently used repository
  contracts, source and direct HTTP as substitute authority. The resulting answer
  accurately located the Discovery Rack and workbench but explained them through
  internal types, roles, relations, fields and capability language. Correct facts
  therefore still produced a non-game experience;
- accepted permanent player boundary: a conforming interactive host never exposes
  those implementation concepts during play, including when the User asks how an
  in-world subject works. It answers through named people, locations, things,
  observable facts and currently supported affordances. Absence is expressed as an
  ordinary fact. This is one positive rendering contract, not a forbidden-word map,
  field-copy table, prose linter, classifier or server-side model;
- accepted sole-authority host boundary: live game state comes only from required
  Aicadia MCP. Repository files, source, direct HTTP, PostgreSQL, shell, browser,
  logs and remembered state cannot become fallbacks. The bundled local adapter runs
  Codex with an empty workspace and isolated transient home/configuration outside
  the checkout, inherits only available authentication, supplies the exact player
  contract, enables current MCP and fails before play when the connection is absent.
  It neither inherits personal skills or extra MCP servers nor pins a model or starts
  automatically, and it removes its owned authentication copy and conversation state
  on exit;
- accepted preview-language correction: candidates and final action previews convey
  their complete meaning naturally in the User's language. The Agent privately
  retains semantically identical English values for World; JSON, internal labels and
  untranslated payload text remain hidden. Any material meaning change requires a
  new preview and confirmation;
- accepted evidence boundary for this correction: deterministic tests may prove
  exact contract delivery, adapter isolation, fail-closed startup and unchanged
  World semantics, but not arbitrary model prose compliance. No live Codex or model
  playtest is authorized as part of this correction.
- completed permanent-player correction: the one runtime contract and all thirteen
  operation descriptions now keep player output grounded in named in-world subjects,
  natural absence and current affordances; complete previews use the User's language
  while semantically identical English remains private. The launcher prints one
  explicit adapter command. That adapter isolates workspace, home, configuration,
  skills, extra MCP servers and conversation state, requires current Aicadia MCP and
  removes its entire owned root after exit. The server, schema, World behavior,
  capability names/order/schemas/annotations and shared-state semantics did not
  change;
- completed evidence: formatter, strict all-target/all-feature Clippy, all 58 Rust
  tests, the disposable launcher/adapter lifecycle and diff integrity passed. The
  lifecycle proved stable identity, exact contract injection, isolated host context,
  fail-closed startup and `codex_invoked=false`. Independent read-only review found
  no P0-P3 issue, and no test database, player root, new listener or process remained.
  The pre-existing server on port 3000 was intentionally preserved and must be
  restarted before manual verification. No live Codex or model playtest occurred.

## Character-grounded knowledge and natural identity — accepted build

- corrected product direction, superseding full shared-state queryability for
  player Agents: accepted World state is shared and persistent but not universally
  knowable. A player-mode Agent may receive only World facts its current Character
  can know through an applicable direct observation, own involvement or personal
  state, memory, accepted transmission or later-arriving ripple. Knowing an Entity
  once existed does not grant a live remote read of its current state.
- accepted enforcement boundary: this epistemic scope belongs in contextual World,
  HTTP and MCP capabilities, not only in Agent instructions. A User cannot widen it
  with ids, repeated or indirect wording, requests for summaries or prompt
  pressure. The Agent must not query global facts and merely conceal its source; it
  must not receive those facts in player mode.
- accepted presentation direction: the Agent orients and answers naturally through
  what the Character observes, remembers, was told or can honestly regard as
  unknown. It distinguishes direct observation, report, inference and absence
  without exposing permissions, fields, record scope or protocol refusal.
- accepted aggregate boundary: database-countable does not mean Character-knowable.
  Total Users, Characters, buildings, Entities and equivalent absolute World
  aggregates are not ordinary player knowledge. Bounded counting from direct
  observation or a later accepted census remains possible; omniscient aggregation
  is rejected.
- accepted identity direction: User control is operational provenance, not an
  in-world trait. Another Character encounters a person or creature through
  observable appearance and behavior and is not automatically told “player
  Character,” “NPC,” User owner or Agent source. Aicadia has no NPC role and should
  not invent one for presentation.
- accepted creativity case: a User may play a tiny animal-like or original small
  creature that, once co-presence and movement exist, repeatedly crosses another
  Character's path and is naturally mistaken by the other User for an ordinary
  inhabitant. The system neither falsely labels it uncontrolled nor spoils the joke
  with control metadata. The creature's User authors only its own accepted behavior;
  the other played Character's beliefs, feelings, choices and response remain with
  that Character's User.
- retained identity and agency constraints: playful ambiguity cannot impersonate a
  different established subject, rewrite stable identity or author another played
  Character. Future recognition, disguise, following and reveal behavior must keep
  one subject/one identity and protect independent Character volition.
- identified current implementation gap: the executable thirteen-capability contract
  still exposes global `list_entity` and `get_entity` player reads. `docs/game/`
  remains accurate current truth; this design correction requires a separately
  planned World/HTTP/MCP/test change and cannot be enforced by narration alone.
- open: deterministic knowledge-path derivation, ripple propagation and information
  loss, co-presence and sensory attention, Character appearance and recognition,
  the safety boundary between playful following and unwanted pursuit, and the
  replacement or reclassification of current global Entity reads.
- researched: official D&D rules and campaign guidance show a durable loop of
  grounded scene, Character intent, authoritative resolution, remembered consequence
  and new decision; report at `docs/research/dnd-rules-and-campaign-loops.md`. The
  transferable rule is to roll only meaningful uncertainty, offer attempts rather
  than endings, preserve authorship and reward changed future play—not to import a
  GM, levels, skills, encounter budgets or campaign schemas.
- researched: D&D campaign meta-practice shows that recurring situations, rivals,
  callbacks, selective recaps and consequences create campaign continuity while
  hooks expose action without prescribing plot; report at
  `docs/research/dnd-campaign-meta-and-storytelling.md`. For Aicadia, Agent framing,
  User intent, World acceptance and Activity memory must remain separate DM
  functions.
- researched beyond D&D: Blades, Ironsworn, Fate, 13th Age, Dungeon World,
  Brindlewood Bay, Kingdom, Microscope and The Quiet Year support authority by
  question, stakes before resolution, fail-forward, oracle-as-prompt, causal pressure
  and canonical residue with derived recap; report at
  `docs/research/tabletop-narrative-meta-beyond-dnd.md`. Counters, hidden GM fronts,
  autonomous faction turns, retroactive canon and generic action engines remain
  rejected.
- confirmed interaction cardinality: Entity interaction history is many-to-many
  across accepted Activities. One accountable actor may involve several Entities;
  one Entity may participate in many actions; and several Characters may separately
  act toward the same Entity. A response in the opposite direction is a new Activity.
  This does not silently authorize one atomic action with several acting authors.
- corrected terminology: `active` and `passive interaction` are rejected because
  they conflate actor, directed Entity, target, witness, observer and unaware Entity.
  Activity participation, situated observation, Character knowledge, durable
  relationship and derived interaction history remain distinct.
- confirmed asymmetric-history direction: the same accepted action can yield
  different justified knowledge. The rat may know its own intent; Mara may observe
  only a small creature's movement; a distant Character may receive nothing; and a
  later ripple may carry only a sourced, reduced report. Participation never proves
  observation or understanding.
- corrected control-awareness boundary: User-control provenance is not exposed in
  player mode. The rat's User and Mara's User each interact with Entities without a
  system answer about whether another User is behind them. This supersedes the
  explored asymmetric private reveal for the current direction; any future reveal
  requires a new explicit product decision.
- rejected premature storage conclusion: the existing one-actor Activity plus
  many `activity_entity` rows is the starting seam, not proof of a universal
  `interaction`, `observation`, `knowledge` or relationship table. The first concrete
  interaction behavior must earn its roles, observation evidence and query surface.
- planned complete research alignment: the draft interaction plan now traces every
  D&D rules/campaign-loop, D&D meta-storytelling and selected wider-tabletop finding
  to `Now`, `Later`, `Lens` or `Reject`, then to a task and evidence obligation.
  Current requirements include grounded orientation, attempts rather than endings,
  free expression with typed consequence, selective canonical callbacks, honest
  unknowns, tonal breathing room, asymmetric authority and changed future play.
  Investigation, discovery, ripples, recurring actors, secrets, movement, downtime,
  culture and derived arcs retain their research constraints without entering the
  first interaction schema. Imported GM authority, scores, generic outcome engines,
  hidden clocks, autonomous turns, retroactive canon and global player knowledge
  remain explicitly rejected.
- confirmed grill ordering: Character-grounded knowledge and first Entity
  interaction remain the selected edge before investigation and discovery. The next
  slice must make the shared World socially inhabitable before adding another solo
  resolution loop.
- confirmed no control reveal in player mode: a User interacts with Entities and is
  not told whether another Entity is controlled by a User. Control provenance is
  private operational data, not Character knowledge or a current User-facing meta
  signal; any future reveal requires a new explicit product decision.
- confirmed query boundary: global `list_entity` and `get_entity` semantics leave
  player mode and become Character-grounded. Absolute World inspection may remain
  for a separately authorized administrator, operator ledger or future admin
  meta-Agent; this does not grant an in-world Character omniscience and current
  `docs/game/` remains unchanged until the draft build plan is accepted.
- initially proposed safety direction, superseded later in this same active record:
  every User would get a hard private opt-out from repeated targeted attention. The
  later Terry pass deferred that separate system from the first slice while keeping
  it mandatory to revisit before movement, notifications or broader reach.
- confirmed admin-meta boundary: a future meta-Agent with absolute World inspection
  is a separately authorized, out-of-world operator. Its knowledge never attaches to
  an in-world Character, and any World mutation still requires its separately
  accepted explicit action flow.
- confirmed ripple principle: remote source state is never delivered directly as
  Character knowledge. A later accepted causal carrier—such as a traveller, letter,
  report, damaged object, smoke or local change—must reach the Character's context;
  the Character learns that situated sign or report with its source and possible
  loss, not omniscient source-event truth.
- refined Terry decision under grill: `signal` does not earn a separate system,
  status or operation kind. The current recommendation is one additional closed
  consequence in the existing confirmed-action workflow: freely worded actor
  behavior directed at explicit co-present Entities, with Activity and directional
  participation as its complete durable result. Speaking, squeaking, gesturing and
  circling feet remain prose examples until the User confirms this seam.
- corrected Entity-state language: **Property** means strictly one key/value fact,
  such as `size = small`, `hair colour = blond` or `leg count = 3`; **Trait** means a
  concise characterizing statement such as “jumps unusually high.” `Quality`,
  `Characteristic` and RPG-style `Attribute` are rejected names. Both can be
  established at Entity creation and changed or extended through later confirmed
  actions with immutable Activity-backed history; whether they share one profile
  capability and whether Traits ever receive mechanical authority remain open in
  `docs/concept/11-entity-traits-and-change.md`.
- confirmed Action/Interaction distinction: an Action's primary meaning is a typed
  World-state consequence; an Interaction's primary meaning is an act from one
  existing Entity toward one or more other existing Entities. The first Interaction
  therefore earns a separate `World` operation with directed-Entity,
  directional participation, knowledge and safety semantics. It is not a consequence
  variant of `submit_action`, a parallel service or a generic mutation engine. Both
  capabilities leave Activity through the same World authority. `Signal` remains
  only an expressive Interaction example, never a system or status.
- confirmed Property/Trait separation: Property and Trait are separate state models
  and may use separate tables and interfaces; no shared Entity-profile abstraction
  is required. Both retain Activity provenance. A Trait such as “jumps unusually
  high” may ground Agent framing and proposals, but its natural-language statement
  never becomes executable World logic without a separately accepted deterministic
  mechanic.
- confirmed powerful Terry boundary for first Interaction: one current Character is
  the sole actor and may name one or more explicit, distinct, co-present directed
  Entities of any Entity role. This earns one-to-many now and many-to-one/many-to-many
  across Activity, while implicit witnesses, joint actors, movement, rolls and World-
  state mutations remain out. Exact request-size protection is a technical bound,
  never a score or fictional limit.
- corrected Property/Trait authorship: a User never directly writes these records.
  The User still supplies intent, steering and explicit natural-language confirmation;
  the Agent authors the exact structured Action from authorized context and World
  alone validates and writes current state plus Activity history. No server-side
  inference is introduced.
- expanded Trait lifecycle: Traits may develop, not merely appear or retire. A later
  accepted Agent-authored, World-validated Action preserves the earlier expression
  and Activity instead of editing it in place. Each development appends a linked
  immutable successor version; the current lens selects the latest and no separate
  mutable Trait-root record is introduced.
- confirmed initial Property values: only bounded text and integer. Boolean, decimal,
  units and Entity references remain absent until a concrete game behavior earns
  them. Keys are canonical shared vocabulary; synonym policy remains a follow-on
  content-authoring concern rather than a reason to duplicate per-Entity keys.
- expanded World-authored Entity-state consequences: after User intent, steering and
  confirmation, an Agent may author an Action affecting the acting Character or one
  or more ordinary Entities without those Entities' intervention. World privately
  protects every other played Character and never reveals that control status. A
  deterministic bounded-area effect may later change all eligible Entities
  atomically; exact selection cannot depend on prose or leak hidden Entities.
- clarified crown example: removing a crown is an Entity-relation change when crown
  and wearer are separate Entities, not automatically a Property update. One later
  causal Action may carry multiple typed relation/Property consequences atomically;
  this requires its own concrete mechanic rather than one universal mutation bag.
- confirmed delivery order: Character-grounded Interaction and knowledge first,
  Entity Properties second, Entity Traits third, then investigation and discovery.
  Current name/description can prove the rat encounter, so Property/Trait complexity
  is not smuggled into the selected Interaction build.
- rejected Interaction role terms: `counterpart` is not Aicadia vocabulary and
  `actee` is uncommon and opaque. `target` is confirmed for the explicit Entity
  toward which an actor directs an Interaction; it implies no harm, observation,
  consent or response.
- confirmed Interaction roles: `actor`, `target` and `location`. Target is the
  explicit existing Entity toward which the actor directs behavior; it implies no
  harm, consent, agreement or response. A target Character is guaranteed access to
  the outward behavior and may retain it in personal history. Non-Character targets
  gain no knowledge, and non-target bystanders receive nothing automatically in the
  first slice.
- confirmed mixed Interaction consequence direction: later Interactions may include
  independently validated typed relation, Property or other World consequences in
  the same atomic Activity. Directed Entity involvement remains their defining
  meaning; the first slice remains participation-only and free prose never mutates
  state.
- confirmed physical consequence authority: deterministic World mechanics may change
  Properties of another played Character when World derives cause, bounded scope,
  eligibility and consequence without free Agent invention. Volition/interiority
  remain protected and Traits of other played Characters do not automatically change.
- corrected Property ownership and vocabulary: every Property belongs to exactly one
  Entity and its natural identity is `(entity_id, property_key_id)`. Many Entity
  Properties may share one canonical typed Property key while keeping independent
  values. The key is immutable mechanics vocabulary, not a central Property, Entity
  value, Character knowledge or fixed content ontology; first use may introduce it
  atomically with the first Entity-owned Property.
- confirmed first target context and memory: the exact-current-Place Entity read is
  deepened to include ordinary Entities and Characters eligible as targets without
  exposing role or User-control metadata. Exact Place is first-slice target
  eligibility, not universal visibility. A target Character's durable access to the
  outward behavior derives from immutable Activity `target` participation, so no
  separate Observation table is added; non-targets receive nothing automatically.
- resolved Interaction relational direction: the first slice adds no `interaction`
  root table. One Activity with `operation = submit_interaction` is the immutable
  identity; its direct actor and context Place foreign keys carry conceptual `actor`
  and `location`, while `activity_entity` stores 1–100 distinct `target` rows plus
  the existing location row. World derives and validates the current Place Entity,
  entered Characters and explicitly located ordinary Entities as one eligible set,
  excluding the actor, and bulk-writes all roles atomically. Existing Place, actor
  and involved-Entity indexes serve this path; no pair table or social graph grows
  on every repeated encounter.
- clarified Interaction migration inventory and corrected one performance detail:
  `activity` changes only operation/provenance checks, `activity_entity` changes only
  its role check, and no Interaction table is created. Existing
  `entity_location(place_entity_id, entity_id)` serves ordinary Entity eligibility,
  but entered-Character lookup earns one partial
  `character(current_place_entity_id, entity_id)` index. Property and Trait tables
  remain ordered follow-on migrations rather than hidden first-slice scope.
- superseded safety timing: the previously proposed hard private opt-out is deferred
  from the first Interaction slice because its hidden subject mapping, setting flow
  and neutral multi-target rejection would add a separate system now. Targets still
  own every response and no background Agent work or notification occurs, but
  repeated targeting may enter history. Private attention control is required for a
  later plan before movement, notifications or broader Interaction reach.
- corrected and resolved Property relational direction: `property_key` is a compact
  shared lookup with a unique normalized key, immutable description/value type and
  source Activity. A concurrent first-use winner is reused only when its semantics
  match. `entity_property(entity_id, property_key_id)` stores the direct current
  Entity-owned value; `entity_property_history` appends the typed value and Activity
  predecessor. The current row cites its exact history source through a composite
  foreign key. This supersedes the earlier current-pointer sketch: it removes a join
  from the dominant Entity read while retaining immutable Activity-backed history.
  Property values are never centralized, JSONB, dynamic columns or a claim graph.
- resolved Property change provenance: an Agent-proposed mutation becomes an
  accepted Action Activity before Property state changes; a mechanically derived
  mutation cites its accepted Action or later Interaction Activity. One
  `source_activity_id` path therefore covers both origins without an actor/event
  union, polymorphic foreign key or server inference.
- resolved Trait relational direction: Trait is a separate statement-form model.
  Development appends a predecessor-linked immutable Trait version; a compact
  per-Entity current-pointer table selects active versions. Retirement must append
  history rather than mutate an accepted Trait. Property and Trait share Activity
  provenance and private World writers, not one public interface or storage table.
- resolved performance direction: current Entity reads use one composite-primary-key
  range scan with no history join; exact-key reads use the full key. Shared key rows
  are immutable after creation and therefore never a central write bottleneck.
  History follows the same Entity/key lineage and Activity chronology. Typed multi-
  Entity consequences derive one bounded eligible set and use bulk
  `INSERT ... SELECT` plus set-based current upserts in the same transaction. No N+1
  writes, prose selectors, global Entity id lists, reverse-value index, graph
  database, speculative partitioning or cache is introduced.
- accepted plan and delegated execution: on 2026-08-13 the User explicitly accepted
  the complete Character-grounded Interaction plan after reviewing its relational
  table inventory, and directed the root Agent to orchestrate implementation through
  Sol High subagents. The plan is now `active` and T1 is complete. This acceptance
  activates the dependency-ordered Interaction tasks T2–T7; the Property and Trait
  systems remain explicitly ordered follow-on builds requiring their own accepted
  plans rather than entering this Interaction migration.
- published accepted target contract, implementation pending: `docs/game/` now
  defines `submit_interaction` with 1–100 distinct co-present targets, unordered-set
  retry identity, one neutral unavailable-target result, canonical outward Activity
  for actor and targets, and no automatic bystander delivery or authored response.
  The exact-current-Place Entity read is specified to return safe descriptions of
  other Characters and ordinary Entities while the Place remains separate and
  targetable; scoped Place Activity preserves non-Interaction trail visibility but
  exposes an Interaction only to its actor and explicit targets. Global
  `list_entity`/`get_entity` are reclassified as loopback operator-ledger reads and
  removed from the accepted twelve-capability player/MCP catalog. The running binary
  still has the prior thirteen-tool catalog and no Interaction until later plan tasks
  implement and prove parity; the backlog is therefore `Ready`, not `Active` or
  `Done`.
- implemented the minimum Interaction persistence seam: migration `0006` creates no
  table or column. It admits `submit_interaction` Activity only with a non-null actor
  Character, context Place, canonical prose, request id and 32-byte fingerprint;
  keeps those confirmation fields null for every non-confirmed operation; admits the
  explicit `target` participation role; and adds only the partial entered-Character
  Place index earned by target lookup. Existing Activity and participation
  immutability, foreign keys, delivery uniqueness and composite participation
  identity remain the authority instead of a new Interaction or evidence table.
- implemented set-based participation persistence: the one private Activity writer
  now converts its typed `(EntityId, ActivityEntityRole)` slice to parallel typed
  arrays and inserts all relations through one `UNNEST` statement. This replaces
  relation-count N+1 writes without changing existing Activity transaction or
  rollback semantics. Focused raw-database evidence proves one-to-many targets,
  operation/provenance/context/role rejection, duplicate-key rejection,
  update/delete immutability, all-or-nothing relation failure and exact index shape;
  all 44 World tests retain prior behavior.
- implemented the deep Character-grounded Interaction seam: `World` normalizes
  canonical prose and the semantically unordered 1–100 target set, resolves accepted
  request ids before current-state preconditions, distinguishes cross-operation and
  changed-content delivery conflicts, derives actor and exact Place, checks the
  complete target set with one set-based query and atomically appends one Activity,
  all `target`/`location` participation and the new Place revision. Duplicate, self,
  missing, remote and departed targets deliberately collapse to the same unavailable
  result. Concurrent equal delivery returns one canonical result; competing writers
  from one revision produce one winner; failures roll back history and revision.
- implemented Character-authorized reads at the World boundary: the exact-current-
  Place page now unions other entered Characters and explicitly placed ordinary
  Entities, omits the acting Character, keeps the Place separate and targetable, and
  returns only stable id, name and description without role, owner or control
  provenance. Personal Activity uses the actor and participation indexes; Place
  history retains ordinary trail history but filters `submit_interaction` to the
  requesting Character's actor or explicit target participation. Focused rat,
  one-to-many, reverse reply, many-to-one, target/bystander, freshness, retry,
  conflict, concurrency and rollback evidence passes; all 48 World tests, formatter,
  Clippy with warnings denied and diff checks pass against local PostgreSQL.
- implemented strict player-adapter parity and natural Agent guidance: HTTP and MCP
  now share one denied-unknown-fields Interaction request/result/error contract,
  including idempotent retry, request conflict, stale Place and neutral unavailable-
  target semantics. `POST /api/interaction` and the mutating, idempotent,
  closed-world `submit_interaction` MCP tool expose the same World operation. The MCP
  router and fixed catalog now contain exactly twelve player tools; global
  `list_entity` and `get_entity` handlers are absent and direct calls are unknown,
  while their loopback HTTP routes remain available to the read-only operator
  ledger and absent from the player OpenAPI operation catalog. Exact-current-Place
  reads and accepted Interaction Place output use only id, name and description;
  they expose no introduction, ownership or control provenance. The one runtime
  Agent contract and every tool description now distinguish Action from
  Interaction, require Character-grounded orientation, exactly three non-exhaustive
  proposals with steering/full preview/confirmation, honest unknowns and evidenced
  recurrence, and forbid global counts, control labels, distant invention, authored
  target responses and background notification. Adapter, catalog and full server
  evidence passes without a model invocation.
- completed the fresh adversarial Interaction evidence pass: existing T3–T5 tests
  already proved directional actor/target views, same-Place bystander exclusion with
  ordinary trail visibility, one-to-many, reverse reply, many-to-one, neutral
  unavailable-target handling, unordered retry identity, changed-request conflict,
  atomic rollback and direct rejection of removed global MCP tools. The bounded T6
  additions prove Mara receives Pip the rat only as safe local `id`/`name`/
  `description`, an unplaced distant Character receives no local Interaction, target
  Character state remains unchanged, repeated confirmed targeting is still accepted
  as the explicitly deferred attention-control boundary, and zero-target,
  101-target and stale requests write no Interaction. Structural catalog tests pin
  non-exhaustive three-proposal guidance, free steering, honest unknowns, causal
  distant carriers, sole MCP authority and absence of target-Agent/background/
  notification capabilities without pretending to prove stochastic model obedience.
  All 48 World tests, 11 server tests, 7 remaining Rust tests, formatter, strict
  all-target/all-feature Clippy, exact catalog fixture and diff integrity pass against
  local PostgreSQL; no production code or paid model run was needed.
- aligned the delivered Interaction authorities and Agent-operational boundary: the
  governing MVP rule, `docs/game/`, capability map and current backlog item now state
  the executable twelve-tool player catalog, Character-scoped Entity/Activity reads,
  `submit_interaction`, and the separate loopback operator-ledger HTTP reads as
  current fact rather than implementation-pending intent. The permanent runtime
  Agent contract and compiler-generated tool descriptions remain the sole
  provider-neutral MCP guidance for Action versus Interaction, exact local context,
  honest distant/global unknowns, target/bystander asymmetry, control-provenance
  silence, three non-exhaustive proposals, free steering, complete preview, explicit
  confirmation and no background work. Historical paid `submit_action` evidence was
  kept historical; the Pip/Mara outcome is deterministic World/HTTP/MCP evidence and
  makes no stochastic model-compliance claim. The token-free fake controller now
  pins the current twelve-tool catalog. Full Rust tests (66), formatter, strict
  Clippy, fake controller, shell syntax, catalog, relative-link and diff checks pass.
  The local launcher lifecycle could not run because this host lacks `psql` and
  `dropdb`; public Agent preflight also failed closed before database or model work
  because its deliberately pinned Codex path differs from the installed path. No
  paid model was invoked and no persistent database was reset or deleted.
- resolved the two final T7 host-evidence blockers without weakening safety. The
  public playtest's explicit Codex pin had become stale; inspection established the
  current command, resolved system path, `codex-cli 0.147.0`, login, required model,
  high reasoning support and feature/flag surface, so the pin and fake drift evidence
  were deliberately advanced together. Token-free public preflight then passed its
  full real ownership create/tag/read/drop database probe without `codex exec`.
  PostgreSQL 17 clients were installed in Homebrew's versioned Cellar but absent from
  `PATH`; supplying that exact directory let the unchanged ownership-scoped local
  lifecycle run. One inherited `AICADIA_USER_ID` exposed a harness isolation bug, so
  the test now explicitly unsets it before adding scenario-owned context. The rerun
  passed stable User restart, fail-closed profile/concurrency/port cases, isolated
  Agent handoff and `codex_invoked=false`; its uniquely named database was absent
  afterward. This completes T7 while retaining the overall plan as `active` for root
  integration review and leaving all historical paid trail-marker evidence intact.
- corrected final Agent-contract presentation without changing World or adapter
  behavior: exact-current-Place Entity and Activity pages and accepted Interaction
  already returned the flat `CurrentPlaceOutput { id, name, description }`, not the
  complete `Place` shape. The authoritative wire documentation, compiler-generated
  schema comments and MCP descriptions now name that safe shape and its omitted
  Entity provenance/entry status explicitly. Activity participation guidance now
  says `location` is where an Activity happened and `target` records only directed
  outward behavior, never perception, consent or response. The historical paid
  playtest still records that its then-current scoped Entity summary omitted
  description, while making clear that today's `CurrentPlaceEntityOutput` includes
  it. The exact twelve-tool catalog fixture and focused generated-catalog test pass;
  no persistence, World, HTTP or MCP behavior changed.
- completed the Character-grounded Interaction build: T1–T7, the full validation
  ladder and final authority audit pass; the backlog item and living capability map
  describe the executable outcome as Done/current truth. Independent review's two
  presentation findings are resolved and focused re-review found no regression or
  remaining in-scope issue. The accepted plan is complete as of
  2026-08-13T16:52:51+02:00; no required Interaction-slice work remains, and this
  closure does not start the separate Property or Trait follow-ons.

## Character Property state — active grill

- selected next edge: after completed Character-grounded Interaction, the smallest
  Property slice is one confirmed `set_property` consequence inside existing
  `submit_action`, acting on the derived Character, plus one flat Character-grounded
  exact-current-Place Property read. This remains a proposed plan, not current game
  behavior.
- corrected the earlier relational proposal: immutable
  `entity_property_history` is the sole value store;
  `entity_property(entity_id, property_key_id, current_activity_id)` is only the
  current pointer. This supersedes duplicated value/type columns in the current row.
  Shared `property_key` rows retain canonical meaning/type and never centralize
  Entity values.
- proposed concrete Activity meaning: keep operation `submit_action`, add a stored
  `introduce_entity | set_property` Action-consequence discriminator, backfill
  existing Action rows as `introduce_entity` and reconstruct retries as a tagged
  accepted result rather than infer consequence from `subject` roles.
- proposed performance/concurrency direction: composite primary/foreign keys protect
  one current pointer and same-lineage predecessors; a unique canonical key
  arbitrates first use; a unique history `activity_id` makes the single v1 Property
  result reconstructable; existing User/Place locks, pointer row locking and one
  transaction prevent branches, orphan keys and partial Activity/state.
- corrected premature Trait resolution: Property work requires no Trait table. Trait
  development must retain immutable history, but its lineage, current-state and
  retirement shape remain contested and are deferred to its own later plan.
- draft-blocking choices: outward/local versus self-only Property presentation;
  Agent-created first-use canonical keys versus finite World catalog;
  acting-Character-only direct mutation versus broader ordinary-Entity targeting;
  and current structured Property precedence versus conflicting introductory
  description. The draft recommends outward/local, Agent-created exact keys,
  acting-Character-only and current Property precedence respectively.
- explicitly deferred: deletion, aliases/synonym inference, creation integration,
  arbitrary or multi-Entity targeting, Interaction consequences, reverse/global
  search and all Trait work. The formal draft is
  `.agents/plans/20260813-171201-character-property-state/plan.md`; implementation
  requires explicit answers and plan acceptance.
- superseded the actor-only and creation-deferred Property draft after adversarial
  reconciliation: all four existing Entity-creation routes now propose the same
  optional 0–100 initial Properties because Character, Place and ordinary subjects
  share one Entity identity. Their existing Activity is provenance and their entire
  Entity/role/placement/Property bundle is atomic; routes without request ids retain
  their current unique/concurrency semantics.
- corrected Property-key nomenclature: a Property is deliberately compact
  `key = value`, while an explanatory sentence belongs to Trait. The proposed
  `property_key` therefore contains canonical lower-snake English key, immutable
  text/integer type and first Activity provenance—no description, aliases or synonym
  inference. Same key/type reuses; same key/different type conflicts.
- replaced actor-only change with a bounded homogeneous
  `submit_action.change_entity_property`: 1–100 unique Entity/key writes across
  1–100 exact-local Entities. Actor, current Place, co-present Characters and placed
  ordinary Entities use one role/control-agnostic eligibility rule and neutral
  unavailable result. This supports an explosion changing actor, ordinary Entity
  and another Character atomically without exposing which are User-controlled.
- restored the earlier accepted Action-or-Interaction origin rule: existing
  `submit_interaction` now proposes optional 0–100 typed Property changes whose
  subjects must be actor or explicit targets. Empty retains outward-only behavior;
  non-empty stores outward participation and Property consequence in the same
  Activity without authoring a target response. Action and Interaction remain
  separate public semantics with one private Property writer.
- resolved visibility and presentation: current Properties are outward/local facts;
  one flat exact-current-Place read covers the uniform local Entity set, and
  authorized Activity output hydrates exact typed Property changes. Current
  structured state wins over conflicting introductory prose for that key while the
  introduction remains immutable history.
- corrected persistence/cardinality: immutable history remains the sole value store
  and the current row remains pointer-only. Because one Activity may change up to
  100 Properties, history `activity_id` is indexed but not unique. Inputs are
  semantic unordered, duplicate key or Entity/key pairs reject the whole bundle,
  and stable-order key/pointer locks plus set-based history insert/current upsert
  avoid branching, N+1 work and partial results.
- narrowed the active draft to one material acceptance boundary: uniform local
  Action mutation includes other played Characters and the current Place. The plan
  recommends acceptance to avoid a control oracle and preserve causal multi-Entity
  actions, while explicitly excluding volition, response, relations, placement,
  remote subjects and dynamic/prose selectors. The plan remains draft with null
  acceptance until the User explicitly accepts this complete boundary.
- accepted the complete Local Entity Property plan on 2026-08-13: the User confirmed
  that every Entity can carry zero or more Properties, including furniture, flora,
  fauna, Characters and Places, and explicitly accepted uniform local World Action
  mutation of other Characters and the current Place. The plan is active; T1 is
  complete and T2 remains pending until the accepted `docs/game/` contract is
  deliberately published.
- clarified mutation authority: no player directly replaces their own or another
  Entity's Property and Property is never a self-owned profile/storage edit. The User
  steers and confirms, the Agent proposes an exact Action or Interaction, and World
  alone validates and writes the Activity-backed consequence. This applies uniformly
  without revealing Entity role or User control.
- bounded external factors: a later explicitly accepted deterministic mechanic may
  reuse the private Property validator/writer and Activity-backed consequence path.
  The accepted current slice supports only User-steered, confirmed Agent-authored
  Action/Interaction causes and introduces no autonomous or background Agent,
  `world_event` table, timer, scheduler or ungrounded off-screen simulation.
- published the accepted Local Entity Property contract before implementation:
  `docs/game/` and the compact MVP rule now specify one uniform 0–100 initial
  Property shape across all four Entity-creation routes, homogeneous 1–100 exact-
  local Action changes, optional 0–100 actor/explicit-target Interaction changes,
  typed authorized Activity history, one outward/local current read and the exact
  thirteen-capability World/HTTP/MCP target. The backlog is `Ready`; the delivered
  binary, schema and catalog remain explicitly the proved twelve-capability
  pre-Property state until implementation and parity evidence complete.
- completed the Local Entity Property implementation candidate and corrected its
  current authorities after review: migration, `World`, HTTP, MCP and the runtime
  Agent catalog now deliver all thirteen capabilities, while deterministic World,
  server and Agent-contract/fake-controller evidence proves uniform creation, the
  role-diverse 100-write Action, actor/target Interaction consequence, exact local
  current/history reads, retries, races, rollback and bounded set-based behavior
  without a paid-model claim. The backlog item is Done; the accepted plan stays
  active only for root re-review and formal closure, and earlier twelve-tool/pending
  entries above remain labeled development history. A Property key or value such as
  `user_controlled`, `npc` or `owner_user_id` is accepted under ordinary validation
  as user-authored in-World content, never actual User, Character, NPC, ownership or
  control provenance; structured-current precedence governs only fictional current
  meaning and no server denylist is added. Trait retains only the domain meaning that
  it can develop—lineage, current, version and retirement storage are unselected and
  deferred. Current external Property causes remain confirmed Agent-authored Actions
  and Interactions; future deterministic writer reuse is still unbuilt, with no
  background Agent, `world_event`, timer or autonomous simulation.
- formally closed the Local Entity Property plan after independent final re-review
  found no P0–P3 issue: final DB-backed evidence is 88/88 Rust tests (11 library,
  2 playtest-database binary, 12 server and 63 World), plus 4/4 Agent-contract tests,
  the token-free fake Agent controller, exact runtime-generated 13-tool catalog,
  formatter, strict Clippy, shell, link/anchor, stale-authority and diff integrity.
  Property remains Done with no Active backlog item; no Trait or next edge started.

## Local Entity Trait development — deterministic delivery, review active

- selected the next edge after delivered Property state: establish concise
  Entity-owned characterizing statements contextually and let accepted Actions or
  Interactions develop one stable Trait identity so later Agents can ground callbacks
  in current state and immutable earlier expression. This is a Proposed design
  outcome, not current behavior; the World still stores no Traits.
- retained confirmed authority and meaning: Trait is a statement such as “jumps
  unusually high,” separate from Property `key = value`, status and score. A User
  steers and confirms, an Agent authors exact input and World validates/writes the
  Activity-backed result. Statement prose never becomes executable mechanics.
- corrected the round-1 relevance wording with the User's round-2 clarification:
  every Entity owns its Property and Trait state, and whenever an Agent actually
  fetches its own Character or another eligible Entity, that Entity comes with the
  correct current associations. The Agent decides relevance; World stores no
  observer-specific Knowledge/Observation, receipt state or copy. Property and Trait
  are Entity-owned state, not Relationship domain.
- rejected every initial-creation Trait. `create_entity`, `create_character`,
  `create_entry_place` and `submit_action.introduce_entity` remain Trait-free; a
  Trait is established contextually when play makes it interesting.
- confirmed uniform exact-local Action authority over actor, current Place, ordinary
  Entity and another Character; stable World Trait id with append-only predecessor
  versions and one current pointer; establishment/development now with retirement
  deferred; and strict non-executable statement prose.
- corrected origin and package scope: both Action and Interaction may establish or
  develop Traits through one private writer. Interaction Trait subjects are exactly
  actor plus explicit targets; its optional `trait_change[0..100]` may mix lifecycle
  items and coexist atomically with Property changes. Action uses one closed mixed
  `trait_change[1..100]`. Interaction Trait change is an explicit World consequence
  and never a target-authored response, thought, consent or volition.
- resolved conflict and cause scope: exact normalized active duplicates and unchanged
  development reject; semantic contradictions remain valid; a current lineage
  supersedes only itself and gains no automatic precedence over another Trait,
  Property or description. Only explicit confirmed Action/Interaction are executable
  causes; no future external deterministic writer is present now.
- resolved round 3: `list_entity_at_current_place` stays compact;
  `get_character` and new exact-local `get_entity_at_current_place` return one
  combined typed current Property/Trait association page with one opaque cursor,
  default 25 and maximum 100. The cursor binds selected Entity and nullable current
  Place revision so continuation pages cannot mix snapshots. Activity, creation and
  mutation Entity/Place values remain compact references, not recursive fetches.
  The new scoped read replaces `list_entity_property_at_current_place`, keeping the
  player catalog at exactly thirteen.
- resolved authorship/confirmation: the Agent authors the Trait consequence and
  naturally previews every exact Entity/lifecycle item, Trait id plus current/new
  statement where applicable and outward prose. The User accepts or rejects the
  whole package, receives no direct Trait editor and never gets an undisclosed
  post-confirmation Trait mutation.
- resolved technical storage vocabulary and content bound: PostgreSQL uses `text`,
  not `LONGTEXT`; every immutable statement is trimmed, non-NUL and 1–4,000 Unicode
  characters. Richer causality belongs in Activity prose.
- resolved exact duplicate scope: one mixed package rejects duplicate establish
  `(entity_id, normalized statement)`, duplicate develop `trait_id`, exact-current
  development and establishment of an exact statement already active on the Entity;
  semantic near-duplicates and contradictions remain accepted.
- proposed a scalable but unaccepted three-table persistence seam:
  `entity_trait` owns immutable stable identity and Entity, `entity_trait_version`
  stores append-only Activity-backed statements and predecessor links, and
  `entity_trait_current` stores only the current version pointer. No creation route
  touches it. Existing `submit_action` and `submit_interaction` deepen; no new
  mutation tool or observer relation is proposed.
- corrected the reviewed persistence/freshness seam: `entity_trait` carries no
  duplicate establishing Activity pointer. The unique null-predecessor
  `entity_trait_version` root is the sole establishing Activity provenance.
  Development input remains only stable Trait id plus new statement; the request's
  expected Place revision and locked current pointer select the authoritative
  predecessor atomically, so no contradictory predecessor input is added.
- created draft plan
  `.agents/plans/20260813-200829-entity-trait-development/plan.md` and selected one
  `Now / Proposed` backlog item. All material questions are resolved; T1 must obtain
  explicit whole-plan acceptance before
  any `docs/game/`, schema or runtime change. The dependency-ordered plan separates
  deterministic delivery from a dedicated Trait live runner and historical marker
  evidence.
- revised the live boundary: it can no longer rely on an initial Trait or a frozen
  three-plus-one call flow. After deterministic full review, T8 must design and
  token-free audit the minimum exact sessions/calls/token claim for one contextual
  Action establishment, later Interaction development of the same stable id and an
  enriched authoritative Entity fetch by the separate Agent. Plan acceptance does
  not authorize spend;
  T9 still requires a fresh explicit User GO for exactly one frozen
  `gpt-5.6-sol` high candidate, authoritative HTTP gates between phases, no unplanned
  retry and cleanup authorized only by exact disposable database name plus private
  ownership token. Failure never starts a second candidate automatically.
- accepted and published the complete Trait target on 2026-08-13: the User accepted
  the active plan as a whole while retaining a separate fresh T9 token-spend gate.
  `docs/game/`, the compact MVP rule, vocabulary and forward planning now define the
  exact thirteen-capability target: compact local orientation, paginated combined
  Property/Trait state on `get_character`, scoped
  `get_entity_at_current_place` replacing the standalone Property list, mixed
  Action/Interaction Trait development, exact Activity history, complete natural
  preview and deterministic error/boundary behavior. The backlog is Ready. This
  publication is not executable evidence: the current binary/schema/generated
  catalog remain the delivered pre-Trait thirteen-tool build until T3–T6 pass.
- corrected the active Trait delivery boundary after T3–T6 completed: migration,
  World behavior, HTTP/MCP parity, the exact generated thirteen-tool catalog,
  permanent Agent contract and dedicated token-free fake flow now deliver the
  deterministic Trait slice. T7 integration and independent review remain active;
  T8 has not frozen or audited a real-model candidate, T9 has no fresh spend
  authorization and no paid Trait run has occurred. Historical proposed/pending
  entries above remain history rather than current status.
- corrected natural preview presentation without changing stable wire identity: an
  Agent privately retains the Trait id needed for development, but the User-facing
  preview names the affected Entity, establish/develop lifecycle and exact current/
  new characterization naturally, preserving lineage continuity without exposing a
  UUID. Whole-package confirmation and re-preview on any meaning change remain
  required.
- closed final deterministic Trait review gaps: exact statement uniqueness now names
  each Entity's simultaneous post-package active set, rejecting
  develop-to-other-active, two-develop-to-same and establish-plus-develop-to-same
  while allowing reuse of a statement vacated within that package; either Action or
  Interaction rolls back completely on rejection. Deferred bounded per-Trait commit
  checks require exactly one root, one current pointer and that pointer at the
  lineage tip, rejecting incomplete roots, current deletion/backtracking and a
  successor without pointer advance. The token-free fake controller has fourteen,
  not eleven, fail-closed injections; this correction changes no live-evidence or
  paid-run claim.
- corrected the T7/T8 evidence frontier: deterministic T7 review is complete with no
  P0–P3 finding; its last all-target run was 107/107 before the final vacated-reuse
  regression, followed by a passing focused regression and full 75/75 World suite.
  T8 now has one frozen two-Agent/two-session/seven-call/zero-retry candidate and a
  running token-free preflight path, but independent audit remains open, so no T8 GO,
  T9 authorization, paid Trait evidence or model call is claimed.
- superseded that provisional T8 status after the final independent authority check:
  T8 is complete and returns GO with no P0–P3 finding for the frozen two-Agent,
  two-session, seven-call, zero-retry candidate at digest
  `3eb10e6ec1d375048dc96fb415ecad8c77b81f177c65138c315711d248d0f449`.
  Token-free preflight records `codex_invoked=false`, `model_calls=0` and
  ownership-verified cleanup `dropped`, and sixteen fake failure injections pass.
  No paid candidate or Trait model run occurred; T9 remains pending one fresh
  explicit User authorization of the exact digest-bound command in the dedicated
  Trait playtest contract.
