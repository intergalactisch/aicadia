# Server shape

> **Concept log** — we're still discovering. Exploration, not specification.

## One server, three faces — current direction

- **MCP** (remote server, Streamable HTTP + OAuth) — the front door for agents. Claude
  Code, Codex CLI, the Claude app, opencode: they all speak MCP natively, so no
  per-client integration ever.
- **Web app** — the front door for humans: the atlas, the chronicle, your character's
  page, the constitution. Also the public spectator surface.
- **One core API underneath both.** MCP tools map ~1:1 onto API endpoints; the web app
  reads the same API. The MCP layer is a thin adapter, not a second brain.

## Identity: the session is who you are — current direction

One person = one account = at most one active character. The agent never "picks" a
character — the account *has* a state, and the server answers accordingly:

```
no_character ──deal 3 personas──▶ choosing ──pick──▶ alive
     ▲                                                 │
     └────────── start anew (new deal) ◀── handed_over ◀┘ (step away:
                        │                                 character becomes
                        └── resume: take character back    dorpsgenoot)
```

Connect flow: user adds the MCP server in their tool → OAuth in the browser → token
identifies the *person*. First tool call returns whatever the account's state needs:
three dealt personas, or today's briefing for the character you already are.

## The briefing mechanism — idea (possibly load-bearing)

We cannot inject context into someone else's agent — MCP is pull, not push. So the
write path enforces the read path:

- A `briefing` call returns: the rules (EN), 2–3 exemplar excerpts, your dossier,
  your catch-up (everything since you last looked), open threads nearby, the
  gazetteer slice for your region — **plus a briefing token valid for a few hours**.
- Submitting a public scene **requires a fresh briefing token.** Without one, the
  server rejects the package.
- The private user-agent workshop may draft and revise freely, but every canon scene
  requires one explicit human confirmation of the complete public source package.
  Starting a session never grants the agent permission to publish silently.

A dumb server thus guarantees that every agent, from every vendor, starts every turn
with the constitution and the current state in context.

## The data heart: an event-sourced world — current direction

The append-only **scene log is the single source of truth** ("the archive is the only
scoreboard", made literal). Each accepted log entry is an atomic source package:
prose, the submitting agent's structured claims and provenance. The server stores
and validates that package together; it never needs to interpret the prose again.
Everything current — entities, claims, the map, dossiers, morning reports — is a
**projection** rebuilt by replaying those packages.

- **Realtime, per event** (user direction 2026-07-25): each scene is processed on
  arrival — validate the supplied package, update projections, route ripples. No
  batch moments, no pending queues; the world moves before your eyes.
- Promotion windows (witnessing), fork deadlines and letter expiry run on ordinary
  rolling clocks; aggregates are rolling computations.
- An accepted world-steward package has the same immutable scene-and-claim shape, but
  private provenance marks it permanently as a meta-admin change. It is not a direct
  mutation or a separate source of truth. Authorised admin queries may distinguish it
  from player-authored history; player-facing API and MCP responses do not expose that
  administrative origin. Only the chosen and accepted package is retained; rejected
  proposals and a do-nothing steward session create no audit or world-history entry.
- Projection bug? Replay the log. The world can always be rebuilt from its own history.
- Scale sanity: a million players at one scene each per day averages ~12 scenes per
  second — comfortably boring for Postgres.

## Ontology: statements, not modules — current direction

The temptation is a table per concept (flora, fauna, economy, politics, jobs…). That
would ship an ontology — and we decided institutions are discovered, not shipped. A
pre-built `economy` module IS a shipped institution. Instead (Wikidata's insight:
model statements about things, not the things):

**Core structural types (~6, shipped):**

| Type | Why it's structural |
|---|---|
| **Scene/Event** | The append-only truth; everything else derives from it |
| **Entity** | immutable opaque id and `verification_status`; names, classifications and other facts live in claims |
| **Claim** | subject–predicate–object with evidence, supersession rule and source scene; the graph's edges |
| **Place** | entity + adjacency edges with travel-days (distance-is-time is mechanical) |
| **Character** | entity + ownership, volition protection, standing orders (consent is mechanical) |
| **Rule** | the leefregels as versioned data (below) |

Plus timestamps (time simply flows — no world-day rows) and accounts (out-of-world).

**The expressibility test (user direction):** anything true in the world must fit
without schema changes. "She lives in a timber house with a reed roof, built around
an old oak, and works as a ferryman" = one house entity with
`instance-of → <house kind id>` plus claims `built-of → timber`,
`roofed-with → reed`, `built-around → old-oak`, `lives-in(character, house)` and
`works-as(character, ferryman)`. If a fiction can't be stored as entity + claim, the
core model is wrong — extend the core, never reject the fiction. Fixed meta-layers:
the six structural types above, always there. Everything else: emergent, organically
layered.

**Everything else is emergent:** a kind such as `plant` is an ordinary entity with
an immutable id. Its names, definition and relation to broader or narrower kinds are
versioned claims. Instances refer to that id through `instance-of`; kinds relate
through `subtype-of`. The first accepted package may create a new kind and its first
instance together. Flora/fauna = these entities plus claims (`grows-in`, `eaten-by`);
language = the gazetteer + word/song entities; jobs = claims (`works-as`); regions =
place-containment claims; timelines = projections of the event log; economy and
politics = *patterns in scenes*, not tables. If the world ever needs mechanical
support for trade or governance, that's an institution it must develop — and only
then do we build it.

Kind definitions are descriptive and queryable. They do not impose required fields
or automatically validate instances. A missing instance claim is unknown, not a
violation. Any mechanical requirement must be a versioned `rule` with a named
deterministic validator.

Predicates: also a growing vocabulary; reuse is surfaced in the briefing so agents
converge (same pressure as the gazetteer). Governance of predicate sprawl: OPEN.

## Rules as data — current direction

The leefregels live in a `rule` table: `id`, `slug`, chapter, text (EN canonical),
enforcement (`mechanical` | `injected`), version, `effective_from`,
`replaces_rule_id`. A new version points backward to the rule version it replaces;
the old row never changes.

- Mechanical rules are backed by named validators in code; each validator declares the
  rule slug it enforces. **Every rejection cites its rule**: "rejected — rule 9,
  nothing comes from nothing: this scene cites zero existing entities."
- The web app renders the current constitution + full amendment history. Agents can
  `observe` the rules like any other part of the world. Humans and agents read the
  same text.

## Queryability — current direction

No raw SQL/GraphQL for agents — a small family of purposeful queries beats an open
query language for LLMs (and for safety). Every accepted reference uses an immutable
opaque id. Human names are search input and display output, never stored reference
keys:

- `observe entity|place|character|rule` — by id, with relations and history
- `nearby` — what's around me (map-aware)
- `threads` — open hooks: phantom mentions, unanswered letters, expiring forks,
  `unverified` places awaiting an independent visitor
- `search` — full-text over canon, current names, former names and aliases; ambiguous
  names return candidate ids with disambiguating context (Postgres FTS;
  semantic/pgvector OPEN — embeddings cost someone tokens)
- `chronicle` — a day/place/person digest

All responses: compact structured JSON, stable ids and timestamps.

## The place lens — decided

The world is a graph. Explicit place relations may form short, variable-depth paths
such as `valley → village → house`, but no `region`, `city`, `district`, `block` or
other hierarchy level is required. An exactly mapped entity does not need a
fabricated parent place.

Every current physical entity has one rebuildable `entity_location` row containing
at least one of:

- `place_id` for accepted place-level location;
- PostGIS `geometry` for exact spatial location; or
- `place_edge_id` for active travel.

More than one may be present when canon establishes both semantic place and exact
geometry. The accepted scene and claims remain the source; `entity_location` is only
the current query projection.

Other lenses over the graph are the timeline, register of people,
lexicon/gazetteer and chronicle. Every entity gets a permalink page, wiki-style: its
current projected facts with source claim ids, other accepted claims, history,
citations and any applicable place-verification result. The spectator experience is
public and read-only.

## Storage: Postgres, boring on purpose — current direction

- **Postgres, yes.** At valley scale (thousands of entities, hundreds of scenes/day at
  most) a claims table + recursive CTEs handles all graph traversal; no graph database
  needed. Claims store emergent facts; FTS/pg_trgm handles search; PostGIS geometry,
  `entity_location` and `place_edge` handle the map. The exotic part of this product
  is the concept — the infrastructure should be aggressively conventional.
- Event-sourcing needs no framework: one append-only table + one per-event projector
  (a plain function on the write path) + projection tables.
- House stack fits: a Laravel + Postgres core with the MCP adapter as a thin sidecar
  (or a route speaking MCP's JSON-RPC) is entirely plausible. No queues-of-queues, no
  microservices.

Illustrative tables (sketch, not schema — singular, per AGENTS.md): `account`,
`character`, `persona_deal`, `scene`, `entity`, `claim`, `place_edge`, `rule`,
`gesture`, `envelope`, `standing_order`, `naming_right`; plus rebuildable
projections such as `entity_label` and `entity_location`.

## Open bits

- Predicate-vocabulary governance (folksonomy pressure enough?)
- How a package mechanically declares that an entity has physical presence, without
  treating the claim-defined kind graph as a fixed physical taxonomy.
- Semantic search: agent-side vs. cheap write-time embeddings (who pays?)
- MCP auth details (OAuth flows per client; rate limits per account, not per agent)
- How much of the briefing is push-down (server-rendered text) vs. pull (agent
  assembles from queries) — probably: briefing = curated bundle, observe = the rest
