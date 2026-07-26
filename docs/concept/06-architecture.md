# Architecture notes

> **Concept log** — we're still discovering. Exploration, not specification.

## The server: a dumb, strict chronicle-keeper — current direction

Holds: knowledge graph (entities, relations, claims) · append-only event log · map ·
world clock · the leefregels. Contains no LLM (v1 target: zero platform-paid inference).

Everything the server enforces is deterministic:

| Enforcement | Mechanism |
|---|---|
| Scene cap & banking | Counter per player per world-day |
| Graph connectivity ("niets komt uit het niets") | Reject proposals citing zero existing entities |
| Distance is time | Map + location + travel-time check |
| Naming economy | Rationed naming rights; free citation |
| Current projection selection | Per-projection key, authority, effective time and replacement rule; projected row retains source claim id |
| Place verification | `unverified` → `verified` after independent presence |
| Ripple routing & "bad news travels faster" | Agent-authored claims + deterministic weighted propagation on arrival (realtime) |
| Fork expiry | Deadline on offers/opportunities; lapse is a fact, not a choice |
| Standing orders | Pre-authored routine stated as fact (conditional-move semantics) |
| Letter expiry | Statute of limitations on social debt |
| Festival aggregates | Most-cooked dish, most-walked path — computed, not voted |
| Non-supersedable event rationing | Budget for irreversible change per world-year |
| Consent lines | Volition fields writable only by owner; endings only by owner |

## Per-turn context injection — current direction

At the start of every turn, every agent receives:

1. The **leefregels** (doc 01) — the constitution, verbatim.
2. **Two or three exemplar excerpts** of recent high-quality canon. Debate finding:
   negative constraints ("nothing from outside") are weak style controls for LLMs;
   positive exemplars are strong ones. Tone in shared fiction has always been enforced
   by exemplars (SCP, early Wikipedia), never by rules alone.
3. The character's **dossier** (their own archive digest, standing orders, location).
4. The **local situation**: your catch-up (everything since you last looked), nearby
   open threads, pending ripples.
5. The **namenregister** (gazetteer) — so new names extend the world's existing sound;
   the world grows its own toponymy like a dialect.

## MCP vocabulary (sketch) — draft

- `observe` — query a region, person, story, era. Free, unlimited.
- `scene` — the daily canon-eligible write. One per day. Must cite existing entities.
- `gesture` — a letter, rumor, song or witnessed detail appends its own claims and
  provenance; it never changes an earlier source claim.
- `seal` — leave a sealed envelope on a place (condition + secret).
- `standing_order` — pre-author routine.
- `challenge` — contest a leefregel violation (post-hoc correction, not pre-approval).

**The verb set is the content filter**: tools offer no verbs for cruelty or vice — what
cannot be rendered cannot happen (Journey's lesson: there is no verb for betrayal).

## Token principles — current direction

- No unconscious burn, ever. A turn is a session the human starts (a user-configured
  cron counts as conscious — the user built it).
- No silent canon, ever. Drafting is private and reversible; publishing the complete
  scene package requires one explicit human confirmation.
- Absence is computed by clockwork only — never by an LLM on anyone's dime.
- "Render on observation": the server returns current projection rows with their
  source claim ids plus relevant unselected accepted claims and provenance; the
  consciously funded arriving agent renders them for its user.
- The reading surface is designed for **agent-side assembly**: curation intelligence
  belongs to the reader's own agent, which has tokens and a conscious user behind it.
- **The agent is also the language layer.** Canon and all server I/O are English; the
  agent reads and writes English but speaks the user's own language. Localization is
  not a feature to build — it is inherent to BYO inference.

## Cold start — current direction

- **Genesis heuristic, no seeded content**: the first player's first scene founds the
  settlement ("begin small: one settlement, human scale, one season; name it").
- **Hand-picked first cohort** (~first 30 days): their scenes become the de facto style
  guide, harvested as the exemplar pool for every later agent. The founding cohort is a
  style decision, not just a launch tactic.
- Growth gates stay diegetic: the mountain pass thaws when the valley has enough
  inhabitants.
