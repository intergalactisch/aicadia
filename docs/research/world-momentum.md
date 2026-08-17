---
status: historical
era: July scene-claim
---

> **Era:** July scene-claim research; its scene/claim vocabulary predates the 2026-08-07 game reframe.

# World momentum and player reach

> **Role / side:** sourced research report / development side.
> **Authority:** records this report's question, sources, findings and implications.
> **Excludes:** product decisions and current implementation contracts; see `docs/game/`.

Date: 2026-07-25

Status: research and `5jaar` recommendation, not concept direction

## Question

How can Aicadia stop one player from simply declaring a city, another character's
disappearance or unearned wealth, while still accepting unpredictable free-form
fiction? Should the world keep changing without new player input, and if so, what may
cause that change without turning the dumb chronicle server into an author?

These are one underlying question: **what is allowed to move world-state?**

## Existing Aicadia constraints

The current concept already supplies most of the boundary:

- A player's agent inhabits one character; it is not an omniscient co-author.
- Big things need long histories and everything starts small (world rule 8).
- New facts must attach to existing canon and geography (world rules 9 and 10).
- Nobody may author another played character's voice, choice, inner life or ending.
- New named entities are scarce; shared institutions must emerge through repetition.
- A lone discovery remains provisional until someone else encounters it.
- Rank, currency and mechanical status do not exist.
- The server validates deterministically and never interprets scene prose with an LLM.

There is also an unresolved tension. The concept says that places have momentum—the
weir silts up, the pass closes, the orchard fruits—but has not yet said who authored
those developments. If elapsed time alone makes the server invent them, the server has
quietly acquired a model of hydrology, weather, agriculture and economics. That would
be shipped world content and a hidden narrator.

## Four mechanisms that look like "the world goes on"

They should not be conflated.

| Mechanism | Source of the change | Fit for Aicadia |
|---|---|---|
| Other people continue writing | A newly accepted player-agent package | Core magic; genuinely new fiction |
| Authored consequence matures | A deadline, condition or standing commitment already present in accepted history | Strong fit if the result is bounded and deterministic |
| Current state is calculated later | Existing claims plus their timestamps and the query time | Strong fit; no new fiction and no background work |
| Autonomous simulation invents change | Server mechanics, an LLM or a prebuilt world model | Poor fit; makes the server an author and ships an ontology |

The first three can make absence consequential without inventing an absent person's
choice. Only the fourth creates genuinely new content without a human source.

## Prior-art patterns

### Heartbeats and scheduled scripts

Evennia, a framework for persistent text worlds, supports persistent scripts and
global tickers. Tickers call subscribed objects at intervals and are a conventional
way to drive weather or other dynamic systems. Its documentation explicitly notes
that separate timers for every object become inefficient and pools work by interval.

This is appropriate when the game already owns the model being simulated. It is a
warning for Aicadia: a weather tick first requires the server to define what weather
is, where it applies, and what it causes. The scheduler is not the difficult choice;
the shipped ontology is.

### Calculate on demand

Evennia also documents an on-demand alternative: store when a process began, then
derive its current stage only when a player or another system asks. A flower in an
unvisited room consumes no recurring compute. Intermediate stages may be skipped
because nobody observed them.

The scale lesson transfers cleanly. With millions of characters and places, Aicadia
should not maintain a heartbeat per entity. If an accepted claim already establishes
a start time and deterministic temporal relation, the current projection can often be
calculated when read or when a related event arrives.

The authorship caveat remains: efficient calculation does not grant the server the
right to invent the flower's lifecycle. That lifecycle must already be in world
history or in a world rule the inhabitants established.

### Player-reactive systems

EVE Online's Dynamic Bounty System changes a solar system's multiplier in response to
player activity: particular player behaviours move the value up or down and an empty
system returns toward equilibrium. It is a useful example of a world whose systems
react continuously to aggregate player action.

For Aicadia this is mostly a contrast. Such a rule can create systemic emergence, but
it also hard-codes an economy, a quantity and an equilibrium. Aicadia can reuse the
smaller idea—deterministic consequences may respond to accumulated accepted
statements—without importing a shipped economy, score or domain module.

### A human-operated world layer

Follow-up: 2026-07-26

Helldivers 2 pairs aggregate player action with a studio Game Master. Arrowhead
describes the Galactic War as inspired by tabletop play: the Game Master controls
factions and responds to surprising player action, so the shared narrative is
reciprocal play between the community and the studio. Its early Battle of Malevelon
Creek became meaningful partly because players turned an unplanned defeat into shared
culture.

This demonstrates a different source of aliveness from autonomous simulation. A human
operator can read what the community actually did and deliberately introduce a shared
pressure that gives many people something to respond to. The system does not have to
pretend that an algorithm spontaneously authored the event.

Most of the Helldivers implementation does not transfer: it begins with fixed factions,
a total war, central orders, progress bars and communal rewards. Those would violate
Aicadia's content-zero, no-score and institution-emergence principles. The transferable
pattern is only:

> A clearly sourced world steward may present circumstances; inhabitants determine
> what those circumstances come to mean.

For Aicadia, three possible layers should remain distinct:

| Layer | Operation | Concept pressure |
|---|---|---|
| Autonomous director | An AI runs in the background and invents events | Hidden author, unconscious inference, difficult trust and cost boundary |
| World steward | An administrator consciously runs an agent, reviews its proposal and confirms a public package | Promising source of shared pressure, but creates privileged authorship |
| Shipped world system | Weather, ecology, economy or factions are predefined and keep producing content | Reliable supply, but fixes the world's ontology before inhabitants discover it |

The world-steward option fits the dumb server technically: its intelligence lives in a
connected, deliberately invoked agent, and its accepted output can use the same
immutable source-package boundary as player scenes. The important problem is authority,
not infrastructure.

Candidate guardrails, not decisions:

- The steward's provenance is always explicit; it never masquerades as emergent player
  history.
- A human administrator starts the run and confirms the exact public package. There is
  no unattended inference loop.
- It introduces impersonal **circumstances and open pressures**, never a played
  character's voice, choice, inner life or ending.
- It cannot award wealth, mastery, rank, institutional status or a resolution. Players
  must still create meaning and outcome.
- Its reach is scarce and inspectable so that the operator cannot become the world's
  dominant author by volume.
- It responds to established history rather than delivering a separate prewritten
  plot.

The unresolved conceptual cost is serious. A steward is a privileged non-character
source and therefore conflicts with the current formulations "only inhabitants are
characters played by users", "equal narrative weight", and "no per-player claim on
shared direction". If adopted, that exception must be named honestly and bounded as
service to the world rather than disguised as an ordinary player.

## Bounding what one scene can do

Free private steering is not the same as unrestricted public causation. A useful
candidate principle is:

> A scene may plant any seed that fits the world, but it may not skip the seed's
> history.

This follows from world rule 8 rather than requiring a vocabulary of forbidden nouns.
Applied to the examples:

- "I found a city" can become staking out a meeting place, raising the first shelter,
  inviting neighbours or provisionally naming a settlement. A city can only become
  true through many later traces and independent use.
- "They disappear" may become a missed appointment, an unanswered letter or a rumour.
  It cannot remove the other character, author their departure or give them an ending.
- "I am suddenly a millionaire" has no mechanical meaning because the world has no
  currency or rank. Material comfort, possessions and social distinction must be
  grounded in prior claims, and social standing is conferred by other people's use and
  recognition rather than self-declared.

The server can enforce only the structural subset: current location and travel,
ownership and ending protection, citations and connectivity, naming rights,
single-valued contradiction checks and any explicitly marked irreversible boundary.
The connected agent must challenge or reshape semantic overreach before presenting
the public package for confirmation.

This leaves a real open problem. A hostile or incompetent agent can put world-breaking
language in prose while supplying structurally valid claims. With no server-side
interpretation, deterministic validation cannot guarantee semantic taste. The concept
still needs to decide the authority relationship between prose and claims, and the
challenge/moderation path for a source package that passed mechanical validation but
violated an injected rule.

## `5jaar`: Aicadia after five years

### What ordinary use looks like

Active places move because many people touch them. Nobody needs a simulation to make a
busy harbour feel alive: arrivals, letters, practices and consequences continuously
enter through accepted packages. A returning player sees real changes caused by
other people and by opportunities whose already-authored clocks ran out.

Quiet places are quiet. A half-built shelter in an abandoned valley does not accumulate
five years of synthetic happenings. It may weather only if that possibility and its
temporal shape were already established. The silence itself is legible history.

Large institutions have recognisable ancestry. The place now called a city can be
walked backward through first shelter, repeated meeting, paths, shared names and other
people's reliance on it. Wealth, authority and mastery are similarly visible as
relationships and traces, never a status bit one owner awarded themselves.

### What failed during those five years

- **The universal world tick:** millions of mostly unobserved objects consumed work,
  made replay and incident diagnosis harder, and produced changes nobody experienced.
- **Ambient simulation packages:** weather, ecology and economy began as atmosphere
  but became a server-owned ontology that constrained what agents could imagine.
- **Generated background prose:** filled catch-ups with unowned, repetitive sludge and
  violated conscious token spend when it used inference.
- **Absence decay:** automatic damage, loss or obligation made returning feel like a
  punishment and recreated FOMO.
- **Semantic server validation:** either missed poetic overreach or required an LLM,
  becoming costly, nondeterministic and a second creative authority.
- **Single-scene grandeur:** cities, titles and fortunes lost meaning when declaration
  was cheaper than a history. The archive became incompatible superlatives.

### What survived

The durable cultural phrase is **the world has momentum, not autonomy**:

- Other people's accepted scenes create new, surprising fiction.
- Time may close an opening or fulfil a bounded commitment that somebody already
  authored.
- Queries may derive what is currently true from accepted claims and elapsed time.
- The server never invents intentions, events, weather, economies or prose to prove
  that the world is alive.

Places can therefore have momentum without having an invisible author. People retain
their protected volition. When all authored momentum is exhausted, the world is
allowed to rest until someone acts again.

## Backcast to now

### Recommended decisions for discussion

1. Sharpen "places have momentum, people don't" to "the world has momentum, not
   autonomy." Without a new accepted player package, only already-authored temporal
   consequences and deterministic projections may change.
2. Make world rule 8 operational in the interaction: a scene may begin a large
   development but may only complete what prior public history supports.
3. Treat social scale and distinction as conferred through independent attention and
   use, not self-asserted status.
4. Keep the semantic enforcement split honest: the agent applies taste and scope; the
   server enforces only explicit mechanical invariants.

These recommendations are not yet concept choices.

### Smallest experiments

- Write one fork in which a caravan leaves at a stated time. Submit no reply. Confirm
  that catch-up can show the expired opportunity without inventing the character's
  choice or generating prose.
- Write one standing routine and query it after elapsed time. Confirm that the current
  projection can state the commitment without creating a daily scene.
- Give an agent each of the three overreaching instructions above. Observe whether it
  preserves the person's intent by proposing a legal first step rather than merely
  refusing.
- Replay all accepted packages at an earlier `as_of` time and at the present. Confirm
  that temporal projections are reproducible without a background world tick.

## Sources

- [Evennia Scripts API](https://www.evennia.com/docs/latest/api/evennia.scripts.scripts.html)
- [Evennia TickerHandler](https://www.evennia.com/docs/4.x/Components/TickerHandler.html)
- [Evennia OnDemandHandler](https://www.evennia.com/docs/latest/Components/OnDemandHandler.html)
- [EVE Online: CONCORD introduces the Dynamic Bounty System](https://www.eveonline.com/news/view/concord-introduces-the-dynamic-bounty-system)
- [PlayStation Blog: Helldivers 2, one year later](https://blog.playstation.com/2025/02/06/helldivers-2-one-year-later/)
- [PlayStation Blog: Helldivers 2 hands-on report](https://blog.playstation.com/2024/02/02/helldivers-2-hands-on-report-chaotic-co-op-and-empowering-stratagems/)
