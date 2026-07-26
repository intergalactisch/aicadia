# Open questions

> **Concept log** — we're still discovering. Exploration, not specification.

Decisions still needing a call, roughly ordered by how much they shape everything else.

## 1. ~~Does the first epoch have a planned ending?~~ — answered: no

User ruling (2026-07-25): nothing ends by design; the world meanders and develops.
If the need for a telling or ending-rite ever arises, the world may develop it —
collectively, never one player alone, never the system. See "Emergent time" in doc 03.
(The debate's retention argument for planned endings — A Tale in the Desert's Tellings —
is recorded here and set aside.)

## 2. ~~Mortality tone~~ — direction chosen: discovered, not shipped

See doc 04. Nothing to build: the server has no biology, so mortality exists only when
written. Remaining watchpoint: observe how (and whether) the world actually discovers
age and illness, and whether leefregels 3 + 4 hold up in practice around them.

## 3. ~~World language~~ — answered: English canon, native experience

User ruling (2026-07-25): the server/canon language is English; toward the user, the
agent always speaks the user's own language. The LLM is the i18n layer — a Dutch
player's agent reads English canon and delivers a Dutch morning report; steering in
Dutch becomes an English scene. No translation engineering needed, and the world stays
open to any language community. Proper nouns (places, people, invented words) travel
untranslated; the gazetteer keeps their sound coherent. Caveat: agent-less spectators
read English canon directly — the fully native-language experience is an agent perk.

## 4. Numbers to tune (all deterministic knobs)

- Briefing-token TTL (hours); scene-credit accrual (every 24h?) and max held (3?)
- Witness window (days) and newcomer grace length
- Naming rights cadence (1/week?)
- Letter expiry (N days)
- Protected-event budget per world-year (one flood?)

## 5. The human-only participant

Can someone read without an agent (spectator web UI: atlas, timeline, chronicle)?
Almost certainly yes — readers are the audience and the funnel. Can someone *play*
without an agent (manual scene writing)? Probably yes-but-unassisted; decide whether
that's a supported path or a side effect.

## 6. Moderation beyond the challenge mechanism

Kind Words' lesson: asynchronous letters need moderation. The verb set and leefregels
filter most abuse structurally, but free-text letters between strangers still need a
human-report path. Scope for v1?

## 7. Identity and auth over MCP

One person = one character requires one person = one account. What's the identity story
for MCP connections from arbitrary clients (Claude, Codex, local models)? Also: rate
limiting per account, not per agent.

## 8. The shareable morning artifact

Wordle's grid is completable + shareable + spoiler-free. What is the morning report's
equivalent — a tiny weather-and-ripples glyph card for your character's day? Nice-to-
have, not v1-blocking.

## 9. Governance of the leefregels

Rule stewardship is a high-depth influence type (doc 05) and rule claims use
`ordinary_scene_can_supersede = false`. Who may propose amendments, and through which slow deliberate mechanism
(assembly aggregates? elder council of long-standing characters?)? Defer past v1, but
don't accidentally make the rules unamendable.
