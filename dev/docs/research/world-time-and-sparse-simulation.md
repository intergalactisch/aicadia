---
status: pending
era: August Activity-Property-Trait
---

# World time and sparse simulation in persistent worlds

> **Role / side:** sourced research report / development side.
> **Authority:** records this report's question, sources, findings and implications.
> **Excludes:** product decisions and current implementation contracts; see `game/docs/`.

Date: 2026-08-08

Status: research, not an Aicadia product or domain decision

Related research: [World momentum](world-momentum.md) separates authored
consequences from autonomous invention. [Spatial state](spatial-state.md),
[mutable place geometry](mutable-place-geometry.md) and
[spatial occurrence and field](spatial-occurrence-and-field.md) examine spatial
projections. This report narrows the question to when temporal and regional work is
actually performed.

## Question

Can an immense, potentially unbounded persistent world change meaningfully without
simulating every second in every region, without making elapsed time a reroll button,
and without server-side language-model inference or automatic agent activation?

The research compares mature multiplayer and simulation systems with agentic-world
prototypes. It asks specifically how they treat clocks, ticks, periodic systems,
inactive regions, deferred work, deterministic generation, restart recovery and
agent observations. Sources are project-owner documentation, papers and source code;
implementation links are pinned to the inspected commit where possible.

## Terms that must remain separate

| Term | Meaning in this report | What it does not imply |
|---|---|---|
| Wall clock | Real-world UTC or monotonic elapsed time observed by the server | Fictional date, simulation progress or a game tick |
| World/calendar time | The authoritative fictional instant and calendar | That code ran for every elapsed second |
| Simulation tick | One opportunity to execute a bounded set of simulation work | One wall-clock second or one fictional second |
| Event timestamp | The accepted or effective time attached to one fact or transition | A recurring process |
| Scheduled transition | A known future deadline or condition plus an operation to apply | Continuous simulation until the deadline |
| Observation context | The `as_of` instant and known state used to answer one observer | A new world fact merely because it was viewed |
| Materialization | Turning a derivable or not-yet-generated result into stored shared state | Permission to resample it on every read |

The central distinction is that a **clock is state**, while a **tick is work**. A
world can advance its calendar without visiting every object, and it can execute
several ticks without advancing wall time. Event timestamps and deadlines can be
compared with the current clock only when a request, activation or scheduler makes
that comparison necessary.

## Real-world monitoring: revisit is observation, not reroll

The OGC Observations, Measurements and Samples (OMS) model separates the subject
from the times attached to knowledge about it:

| OMS field | Meaning |
|---|---|
| `phenomenonTime` | Time for which the result applies to the observed characteristic of the FeatureOfInterest |
| `resultTime` | Instant at which the observation result became available |
| `validTime` | Optional interval during which the result is assumed usable |
| `FeatureOfInterest` | Subject of the observation |

These times can differ. OMS gives laboratory and post-processing examples where the
result becomes available after the interaction with the world, and a simulation
example where a result produced now applies to a phenomenon in the past or future.
It also uses `resultTime` to distinguish repeat measurements of the same property,
feature and procedure.
[OGC OMS observation semantics](https://docs.ogc.org/as/20-082r4/20-082r4.html#_attribute_phenomenontime)

Consequently, a later `resultTime` is not evidence of a later phenomenon. A second
acquisition normally is a new observation with its own `phenomenonTime`, but it can
still concern the same FeatureOfInterest. Whether the subject changed is a conclusion
from comparable results, procedures and quality—not a consequence of reading it
again.

The appropriate revisit rate is also not universal. USGS guidance says measurement
frequency depends on the monitoring objective, intended use and required level of
analysis. It should be fine enough to detect the short-term and seasonal variation of
interest and distinguish short- from long-term hydrologic stresses; periodic sampling
can miss extremes and bias apparent trends when its frequency is poorly chosen.
[USGS Circular 1217](https://pubs.usgs.gov/circ/circ1217/pdf/circular1217reprint.pdf)

Earth observation shows that *how* a place is observed can matter as much as *when*.
NASA specifies Landsat 8 at a 16-day temporal resolution, with 15, 30 and 100 metre
spatial resolutions for different instruments and 11 spectral bands. Joining Landsat
7 doubled temporal coverage to eight days. The added coastal/aerosol, cirrus and
thermal bands respectively expose water and aerosol properties, high thin clouds,
and distinctions between surface and atmospheric temperature that a different band
set would not show.
[NASA Landsat 8 mission specifications](https://science.nasa.gov/mission/landsat-8/)

The neutral context lesson is that a later, finer, differently located or differently
instrumented observation can add valid information without the observed place being
randomized anew. A context system can therefore distinguish the persistent subject,
the time to which a result applies, when the result became known, the observation
method and its resolution. This permits knowledge to become richer through revisit
while leaving open—rather than assuming—whether the world itself changed. It does not
choose an Aicadia observation or discovery model.

## Findings

1. No inspected system uses one universal mechanism for all temporal behaviour.
   Mature systems combine detailed active simulation, deadline-like timers,
   elapsed-time catch-up and functions of location/time/seed.
2. Sparse worlds make *activation* explicit. Luanti simulates only mapblocks around
   players and Cataclysm: DDA loads and saves submaps at the edge of a finite
   “reality bubble.” Inactive state carries enough temporal metadata to be
   actualized later.
3. Lazy catch-up is semantically safe only when skipped intermediate states are not
   themselves required events. Evennia explicitly warns that unobserved stages may
   be skipped; Cataclysm: DDA implements separate catch-up rules for rain collection,
   plant growth, seasonal fruit, sap, radiation and field decay.
4. Randomness is an independent state problem. Stable procedural inputs or a saved
   PRNG state make replay possible. Re-running mutable random draws during activation
   can make reload or observation order affect outcomes unless the result is stored
   or the draw is keyed to a stable transition identity.
5. A deterministic tick loop can preserve multiplayer consistency, but it does not
   make an unbounded world cheap. OpenTTD distributes a finite map's tile work across
   256 ticks and persists the tick cursor and gameplay PRNG state.
6. Agentic prototypes prove useful clock/context patterns, not large-world
   scalability. Smallville advances a fixed amount of fictional time per simulation
   step and invokes every persona on every step. Concordia exposes both a fixed
   increment clock and an LLM-updated “generative clock”; the latter is a deliberate
   example of why world-time authority and inference should not be conflated when a
   strict deterministic server is required.
7. Overload handling is also a clock-policy question. EVE Online slows local combat
   simulation while deliberately keeping some long deadlines outside dilation. A
   single server node may host multiple solar systems, so technical co-location can
   make a quiet system inherit dilation caused elsewhere.

## Comparative evidence

| System | Clock and work model | Inactive or deferred work | Determinism and restart boundary |
|---|---|---|---|
| Luanti | Server steps, game time and time-of-day coexist; only active mapblocks run ABMs and node timers | ABM catch-up adjusts probability; LBMs and node timers receive elapsed time on activation | Chunk seed derives from world seed and position; LBM introduction timestamps and persistent timers bound repeated work |
| Cataclysm: DDA | Turn-based game calendar inside a player-centred reality bubble | A persisted `last_touched` drives rule-specific actualization when a submap loads | Calendar and touch times persist; much weather is keyed by location/time/seed, but some catch-up and wind logic still draw mutable RNG |
| OpenTTD | Pausable game ticks plus separate calendar and economy dates | Finite-map work is staggered rather than put to sleep | Gameplay PRNG, tick, dates and tile-loop cursor are saved; ordered tick processing supports deterministic multiplayer |
| Evennia | Optional real-time/game-time scaling, scheduled callbacks and pooled tickers | On-demand tasks derive current stage only when read and may skip unobserved stages | Persistent ticker state can survive reloads; task identity is a unique key/category rather than a new timer per read |
| EVE Online | Load-sensitive simulation clock for active combat; selected long timers remain wall-clock-like | Not a sparse-world catch-up model; it demonstrates distinct clock classes under load | Slowing the shared node preserves command order, but unrelated hosted systems can experience the same dilation |
| Smallville / Generative Agents | Explicit simulation step, fictional `curr_time`, fixed seconds per step | No inactive-region strategy in the reference loop; all personas move each step | Step and current time are stored, but inference is in the per-step loop and therefore scales with simulated agents |
| Concordia | Engine steps and observation/action order are separate from pluggable clock components | Scenario framework, not a persistent unbounded-world implementation | A fixed clock stores start/step/increment; an alternative LLM clock updates after resolved events and is not deterministic authority |

### Luanti: active blocks, catch-up and seeded concretization

Luanti makes regional activity a first-class engine state. A mapblock is the unit
stored in the world database and sent to clients. Blocks near players may be loaded;
a smaller set is *active*. Entities, active block modifiers (ABMs) and node timers run
only in active blocks, while loading block modifiers (LBMs) run when a block is
activated. An absent block is loaded from storage or generated when it is emerged.
[Luanti map terminology](https://api.luanti.org/map-terminology-and-coordinates/)

The APIs expose several different missed-time policies rather than pretending they
are equivalent:

- An ABM runs randomly at an interval in active blocks. With `catch_up=true`, its
  chance is temporarily increased after a block was unattended. This approximates
  missed opportunities; it does not replay every historical tick.
- An LBM receives `dtime_s`, the in-game seconds since the block was last active.
  LBMs also carry an introduction timestamp, so only LBMs newer than the block's
  last activation normally run. This is useful for once-per-version migrations or
  deterministic elapsed-time repair.
- A node timer is high-resolution and persistent; its callback receives the total
  elapsed time. A global-step callback, by contrast, runs each server step.

These behaviours are defined separately in the official
[definition tables](https://api.luanti.org/definition-tables/) and
[class reference](https://api.luanti.org/class-reference/). The engine also exposes
game time, day count and time-of-day independently through its
[core namespace](https://api.luanti.org/core-namespace-reference/).

Procedural concretization is regional and stable-input-friendly: the map generator
receives a chunk `blockseed` derived from block position and the world's 64-bit seed.
The generation callback can access only the current chunk, and the mapgen environment
has no global step or timer. This supports generating a region when needed without
running a background simulation there. It does not by itself solve generator-version
changes: a system must either retain already generated results or version the
generation rules and seed contract.
[Luanti mapgen API](https://api.luanti.org/core-namespace-reference/#mapgen-environment)

### Cataclysm: DDA: reality bubble and rule-specific actualization

Cataclysm: DDA distinguishes global and local coordinates. Twelve-by-twelve-tile
submaps are loaded or saved as they enter or leave the player-centred reality bubble;
larger scales drive broader map generation.
[Coordinate documentation](https://github.com/CleverRaven/Cataclysm-DDA/blob/c92176491b0494a2520cacb62de4c9938fe63681/doc/c%2B%2B/POINTS_COORDINATES.md#L35-L56)

Each persisted submap carries `turn_last_touched`. On loading, `map::actualize`
computes `calendar::turn - last_touched`, processes contained items and vehicles,
then applies different temporal rules per tile: retrospective funnel filling, plant
growth, seasonal fruit restocking, sap production, radiation scorching and cosmetic
field decay. It finally sets `last_touched` to the current turn.
[Actualization source](https://github.com/CleverRaven/Cataclysm-DDA/blob/c92176491b0494a2520cacb62de4c9938fe63681/src/map.cpp#L9583-L9647)
[Submap serialization](https://github.com/CleverRaven/Cataclysm-DDA/blob/c92176491b0494a2520cacb62de4c9938fe63681/src/savegame_json.cpp#L5261-L5268)

This is not “simulate all missed turns quickly.” Fruit can restock when a season
changed or enough time elapsed; sap calculates how much of the elapsed interval
overlapped its producing seasons; other effects use probability based on elapsed
duration. The rules intentionally summarize history at the semantic resolution each
feature needs.
[Catch-up rules](https://github.com/CleverRaven/Cataclysm-DDA/blob/c92176491b0494a2520cacb62de4c9938fe63681/src/map.cpp#L9356-L9581)

The save records the current calendar turn and restores it before loading the map.
[Game-time save/load](https://github.com/CleverRaven/Cataclysm-DDA/blob/c92176491b0494a2520cacb62de4c9938fe63681/src/savegame.cpp#L96-L112)
For procedural environmental context, the weather generator's temperature, humidity
and pressure noise takes absolute `x/y`, world time and a seed. Wind strength and
direction still use mutable random calls and state, however, so “weather is a pure
function” would be too broad a claim.
[Weather source](https://github.com/CleverRaven/Cataclysm-DDA/blob/c92176491b0494a2520cacb62de4c9938fe63681/src/weather_gen.cpp#L94-L226)

The failure mode is visible in the same source: load-time catch-up for some effects
uses random draws. Unless the surrounding PRNG state and load order are controlled,
reloading or causing regions to actualize in a different order can change results.
Elapsed time is therefore not an anti-reroll mechanism on its own.

### OpenTTD: deterministic global ticks with staggered work

OpenTTD is the finite-map counterexample to region sleeping. Its game-tick counter is
monotonic since game start and pauses with the game. Fixed tick periods exist for
station rating, cargo ageing, industry production and town growth.
[Game-tick definitions](https://github.com/OpenTTD/OpenTTD/blob/8ef6fa58a83f197c2dca78d032eb0f4e19a45f32/src/timer/timer_game_tick.h#L20-L87)

It nevertheless avoids updating every tile on every tick. `RunTileLoop` gradually
visits all map tiles, with each tile updated once every 256 ticks, retaining a cursor
between calls.
[Tile-loop source](https://github.com/OpenTTD/OpenTTD/blob/8ef6fa58a83f197c2dca78d032eb0f4e19a45f32/src/landscape.cpp#L95-L96)
[Iteration source](https://github.com/OpenTTD/OpenTTD/blob/8ef6fa58a83f197c2dca78d032eb0f4e19a45f32/src/landscape.cpp#L800-L843)

The save format persists calendar date, calendar fraction, tick counter, economy
date, tile-loop cursor and the gameplay randomizer's state. The code explicitly
distinguishes `_random`, used for game-state calculations, from
`_interactive_random`, used where state is not directly affected.
[Persisted temporal and random state](https://github.com/OpenTTD/OpenTTD/blob/8ef6fa58a83f197c2dca78d032eb0f4e19a45f32/src/saveload/misc_sl.cpp#L86-L112)
[Randomizer distinction](https://github.com/OpenTTD/OpenTTD/blob/8ef6fa58a83f197c2dca78d032eb0f4e19a45f32/src/core/random_func.hpp#L27-L44)

This provides strong deterministic ordering and restart continuity, but its cost
still grows with the finite materialized map. Staggering is load smoothing, not a
solution for an unbounded set of inactive regions.

### Evennia: tick only when effects must arrive unprompted

Evennia documents both recurring work and lazy state. Its TickerHandler pools
subscribers with the same interval rather than creating a clock for each object, and
persistent subscriptions can be restored after a restart.
[TickerHandler](https://www.evennia.com/docs/latest/Components/TickerHandler.html)

Its OnDemandHandler makes the stronger sparse-world move: store the task start and
stage boundaries, calculate the current stage only when requested, and do no work in
between. The documentation warns that intermediate stages may never fire. If a flower
is first revisited while wilting, earlier stages may be skipped, so those stages must
not contain required side effects. A ticker is still necessary when an idle player
must receive an unsolicited notification.
[OnDemandHandler](https://www.evennia.com/docs/latest/Components/OnDemandHandler.html)

Evennia also explicitly separates real time from accelerated or slowed game time and
can schedule callbacks against game-calendar fields.
[Game-time documentation](https://www.evennia.com/docs/latest/Howtos/Howto-Game-Time.html)
This combination demonstrates a useful boundary: deriving “what stage is visible
now” is cheap and lazy; promising an externally visible event at an exact moment
requires a durable scheduler or a well-defined late-delivery policy.

### EVE Online: not every timer follows the same clock

EVE Online's Time Dilation slows simulation under node load so that commands and
stateful tasks can remain responsive and ordered. CCP reported the feature fully
active in January 2012 and described 1,300-player fights where module processing no
longer accumulated large delays.
[Deployment report](https://www.eveonline.com/news/view/time-dilation-hows-that-going)

The original design explicitly separated combat-time processes from long timers:
shield recharge should dilate with combat, but reinforcement deadlines should not,
because players could otherwise manipulate their completion time by creating load.
[Time Dilation design](https://www.eveonline.com/news/view/introducing-time-dilation-tidi)
This is direct evidence that “world time” is too coarse a category for every temporal
contract. A duration whose fairness depends on the wall clock can require a different
clock from moment-to-moment local simulation.

There is also a partition failure mode. One EVE server node can host many solar
systems, and official reports note that a quiet system may be dilated because another
system on the same node is busy. Technical partition membership is therefore not the
same thing as fictional locality.
[EVE node-locality discussion](https://forums-archive.eveonline.com/message/778251/)

### Agentic worlds: time as explicit context, inference as optional policy

The Generative Agents reference implementation stores `start_time`, `curr_time`,
`sec_per_step` and `step`. Once the frontend supplies the next environment snapshot,
the server invokes `move` for every persona with the same current fictional time,
then increments both step and time. That makes observation context coherent inside a
small synchronous simulation, but it couples each time step to inference for every
agent.
[Smallville loop](https://github.com/joonspk-research/generative_agents/blob/fe05a71d3e4ed7d10bf68aa4eda6dd995ec070f4/reverie/backend_server/reverie.py#L68-L94)
[Per-step persona execution](https://github.com/joonspk-research/generative_agents/blob/fe05a71d3e4ed7d10bf68aa4eda6dd995ec070f4/reverie/backend_server/reverie.py#L350-L408)
The original paper describes the sandbox as 25 agents and treats perception,
planning, reflection and action as the experimental architecture; it does not claim
an inactive-region or unbounded-world solution.
[Generative Agents paper](https://arxiv.org/abs/2304.03442)

Concordia makes the policy choice even clearer. Its engine owns observation,
scheduling, resolution and termination, while clock behaviour is supplied by
components. A fixed increment clock stores a start instant, integer step and increment.
The library also offers a `GenerativeClock` that asks a language model to infer new
time after each resolved event.
[Engine interface](https://github.com/google-deepmind/concordia/blob/513c3d622d19cf99f1c2f63991b648ffd3d5fcb5/concordia/environment/engine.py)
[Fixed clock](https://github.com/google-deepmind/concordia/blob/513c3d622d19cf99f1c2f63991b648ffd3d5fcb5/concordia/contrib/prefabs/game_master/simultaneous_resolution_gm.py#L46-L230)
[Generative clock](https://github.com/google-deepmind/concordia/blob/513c3d622d19cf99f1c2f63991b648ffd3d5fcb5/concordia/components/game_master/world_state.py#L405-L562)

These are useful contrasting components. An agent may *receive* authoritative time
as observation context without being woken by it. Letting an LLM decide what time it
is is a separate scenario-design choice and cannot provide deterministic clock
authority, replay or zero-background-token guarantees.

## Reusable design patterns and trade-offs

### 1. Full deterministic tick

Process the entire materialized world in a stable order and advance a saved PRNG.
This gives clear replay and multiplayer order. Its cost is proportional to stored
active state, even where nothing interesting happens. Batching or staggering smooths
load but does not remove that asymptotic cost.

### 2. Active-detail simulation

Only regions near current interaction run detailed physics, entity logic or frequent
timers. This bounds work by concurrent activity rather than total world extent. The
hard boundary is activation: entry must produce the same shared state for all players,
and cross-boundary effects need explicit summary rules.

### 3. Deadline or stage calculation

Store `started_at`, `effective_at` or ordered stage thresholds. At observation or a
scheduler wake-up, compare the relevant authoritative clock and derive the result.
This is ideal when intermediate states have no independent side effects. It is wrong
when every intermediate transition must notify someone, transfer ownership or create
another event.

### 4. Elapsed-time actualization

Store `last_updated_at` per sparse aggregate and apply a feature-specific catch-up
function over `[last_updated_at, now]`. A closed-form quantity or a bounded list of
season intersections is cheap. Replaying every missed tick merely postpones the
scaling problem until activation and can create a latency spike after long absence.

### 5. Stable procedural field

Calculate context from stable inputs such as `(world_seed, rule_version, position,
time_bucket)`. This is suitable for unobserved terrain candidates or environmental
conditions that need no independent history. Once actors can change or rely on a
result, materializing it prevents a later rule deployment from silently rewriting
their past.

### 6. Generated-once shared concretization

When observation itself must turn an unknown possibility into a durable fact, give
the attempt a stable identity such as `(world, region, rule_version, time_bucket)`.
Derive or generate one candidate, commit it atomically, and have concurrent observers
read the winner. This is distinct from generating a fresh answer per observer.

## Preventing time from becoming a reroll button

The evidence suggests four independent safeguards; any one alone is incomplete:

- **Stable clock source:** define whether a rule reads wall time, world time or tick
  count. Never let a request supply an arbitrary `now` for state-changing work.
- **Stable transition identity:** identify the subject, rule/version and interval.
  Enforce one committed outcome for that identity so retries and concurrent workers
  are idempotent.
- **Stable randomness:** either derive randomness from the transition identity and a
  world seed, or save the outcome/PRNG position before exposing it. Advancing time and
  asking again must not create a new draw for the same transition.
- **Monotonic watermark:** move `last_updated_at` only in the same transaction as the
  effects. Never process an already closed interval again; never move the watermark
  backward because of a wall-clock correction.

The trade-off between derived and stored outcomes is historical mutability. Pure
functions are compact, but a rule or seed change can alter every recomputed answer.
Stored results cost space, but preserve what participants actually encountered.

## Concrete edge cases

| Case | Required question | Typical failure if unspecified |
|---|---|---|
| Server restarts before a deadline | Is the deadline durable, and which clock resumes it? | Transition is lost or fires early |
| Server restarts after a deadline | Is late delivery allowed, and is it exactly once? | Missed outcome or duplicate outcome |
| Region is inactive for six months | Is catch-up closed-form, bounded, skipped or replayed? | Activation stall or fabricated dense history |
| Time jump crosses several seasons | Are seasonal intersections calculated, or is only the final season relevant? | Impossible yields or repeated seasonal rewards |
| Administrative clock moves backward | Is state-changing time monotonic despite display correction? | Reopening old windows and farming them again |
| Two players reveal the same unknown area concurrently | What stable key and transaction select one shared result? | Divergent maps or duplicate discoveries |
| A storm passes where nobody observes it | Does it need durable event history, or only a field value when queried? | Storage of meaningless events or inconsistent later claims |
| Rules change while a region sleeps | Which rule version owns the missed interval? | Deployment rewrites the region's past |
| Catch-up contains random draws | Is each draw keyed/stored, and is iteration order stable? | Reload and visit-order rerolls |
| Active region crosses a partition boundary | Which process owns effects spanning both sides? | Double processing or missing interactions |
| Two agents observe at different instants | Is each answer labelled `as_of`, with one committed state revision? | One agent reasons from a future or mixed snapshot |
| No player is connected | Which deterministic deadlines truly require a wake-up? | Unconscious agent/token burn disguised as world time |

## Neutral implications for Aicadia

The sources do not select an Aicadia design. They constrain the questions a future
decision must answer:

- Whether Aicadia has a world calendar at all is separate from whether it has a
  background tick.
- Any temporal rule needs to name its authoritative clock and the stored timestamps
  or deadlines it reads.
- A potentially unbounded world is compatible with bounded server work when
  unmaterialized space is procedural, inactive state is sparse, and catch-up is
  feature-specific rather than universal tick replay.
- “Current weather,” “an orchard has matured” and “a scheduled opening expired” are
  different temporal semantics; one generic `update_world(elapsed)` contract would
  hide important differences.
- Observation can carry an explicit `as_of` world time without triggering an agent.
  Server time progression and deterministic materialization need no language model.
- If an Agent supplies creative content after an explicit user call, the accepted
  result can still use stable event timestamps and materialization keys. That is
  different from waking agents because a clock advanced.
- A rule that uses randomness must specify whether the result is a stable procedural
  function or a once-committed event. Otherwise time, retries, restarts and visit order
  become player-controlled reroll inputs.
- Quiet regions need not accumulate invisible detailed history. They may retain only
  last-known state, deadlines, seeds and watermarks until a concrete observation or
  action requires more.

The smallest future decision can therefore remain narrow: choose one concrete
time-dependent world behaviour, classify its clock, side effects, inactive-region
semantics and reroll boundary, then test that behaviour across restart, long absence
and concurrent observation. This report intentionally does not choose that behaviour
or introduce a schema.
