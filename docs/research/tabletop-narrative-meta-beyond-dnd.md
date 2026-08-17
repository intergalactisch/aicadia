---
status: pending
era: August Activity-Property-Trait
---

# Tabletop narrative and meta patterns beyond D&D

> **Role / side:** sourced research report / development side.
> **Authority:** records this report's question, sources, findings and implications.
> **Excludes:** product decisions and current implementation contracts; see `docs/game/`.

Date: 2026-08-13

Status: research; no Aicadia product, game-behaviour or implementation decision

## Question

Which first-party tabletop RPG techniques outside D&D solve problems that matter
to Aicadia—distributed narrative authority, scene framing, fail-forward,
stakes-before-commit, discoveries and oracles, faction or world pressure,
history/recap and asynchronous shared play—and what would be gained or lost by
transferring them?

This note deliberately asks for heuristics, not a new ruleset. It does not select
the next Aicadia edge or authorize changes to `docs/game/`.

## Aicadia boundary used for the comparison

The [current build contract](../game/README.md) has one persistent World, zero or
one shared entry Place, nullable Character placement, shared Entity records and
append-only normalized Activity. One current action can add canonical prose and
one placed Entity at the Character's exact current Place. World is dumb and
deterministic: it interprets no prose, calls no model and performs no background
Agent work.

The [Agent play contract](../game/agent.md) adds a separate private layer.
The User chooses and steers; the Agent reads live typed state, offers exactly three
grounded directions, previews the complete meaning, waits for explicit
confirmation, and only then submits the unchanged package. Shared World facts and
private conversational authorship are already distinct authorities.

The [backlog](../../.agents/backlog/README.md) lists a first investigation roll and
first discovery commit as queued later edges. They are not current behaviour. The
comparison therefore tests techniques against both the existing trail-marker
slice and that possible `roll first -> author candidate -> commit` seam without
assuming its eventual contract.

## Evidence boundary

Only publisher-, author- or project-owned public rules, SRDs, play aids and design
notes are used for tabletop claims. Community explanations were excluded. The
systems solve different problems and normally assume synchronous humans plus a GM
or shared table; transfer to persistent asynchronous Agent play is analysis, not a
claim made by those sources.

## Findings at a glance

| Pattern | What it solves | Best Aicadia-scale reading | Main trade-off |
| --- | --- | --- | --- |
| Authority by question | Distributed authorship without argument | Keep User intent, Agent composition, World truth and shared observation separately final | More authority types become ceremony and conflict surfaces |
| Stakes before resolution | Surprise and semantic drift | Ground goal, approach, possible exposure and result boundary before chance or commit | Un-grounded Agent judgment would become invented mechanics |
| Fail forward | Investigation stalls and retry spam | A failed attempt should change what can be understood or chosen, not merely invite the same call | Any durable cost or consequence expands current state and history semantics |
| Oracle as prompt, not fact | Creativity without a hidden author | Let deterministic random output constrain private interpretation; commit separately | Prompts can still produce incoherent or contradictory candidates |
| Roll first, author second | Expensive prose for outcomes that never qualify | Resolve eligibility/shape before Agent drafting, then preview one concrete candidate | The hand-off must not let prose silently acquire mechanical force |
| Clear scene frame | Vague, meandering narration | Establish who, where and what is happening before offering directions | A universal scene object would be a needless new domain model |
| Event-driven pressure | A world that feels consequential | Describe actor, impulse, next observable sign and end condition; advance only through explicit accepted causes | Classic clocks/fronts assume counters, hidden GM work or background advancement |
| Canonical residue, derived recap | Shared continuity across players and time | Activity/Entity are evidence; recaps and thematic lenses are disposable reads over it | Tabletop turn/map solutions do not by themselves solve asynchronous concurrency |

## 1. Assign final authority per question

Blades in the Dark does not make “shared authorship” vague. Its official SRD says
everyone contributes, then assigns final say: players decide which actions their
characters attempt; the GM decides danger, effect, consequences and whether a roll
is required. Narration after the roll is collaborative, but consequence authority
does not become ambiguous.
[Blades core system](https://bladesinthedark.com/core-system) and
[action roll](https://bladesinthedark.com/action-roll).

Kingdom demonstrates a more radical split without a GM. `Power` decides what the
community does, `Perspective` predicts consequences, and `Touchstone` establishes
what the people want. The split makes otherwise-confusable statements—decision,
prediction and sentiment—different authorities.
[Kingdom official overview](https://www.lamemage.com/kingdom/) and
[official role cards](https://www.lamemage.com/kingdom/K2/Kingdom_2E_roles.pdf).

**Aicadia implication.** The transferable heuristic is an authority ledger, not
Blades' GM role or Kingdom's three roles. For every statement, ask who has final
say:

- the User owns intent, selection, steering and confirmation;
- the Agent owns private wording and proposals, but no World fact;
- World owns identities, current typed state, random resolution and acceptance;
- committed Activity/prose is shared evidence other Agents may render but not
  reinterpret into extra state.

This directly fits the current seam. It also warns against letting one prose field
simultaneously establish an action, a consequence, public sentiment and a future
threat. The trade-off is proportionality: importing formal authority roles into the
game would add governance before any concrete conflict requires it.

## 2. State goal, approach, exposure and effect before resolution

Blades resolves an action only after the player states a concrete goal and chooses
an approach, while the GM states `position` (how dangerous) and `effect` (how much
can be achieved). Position and effect are independent, so a bold approach may be
dangerous but potent, safe but limited, or another explicit combination. Only then
are dice rolled.
[Blades action roll](https://bladesinthedark.com/action-roll) and
[setting position and effect](https://bladesinthedark.com/setting-position-effect).

Brindlewood Bay makes the same fairness principle even more explicit for a risky
move: the player names what they fear will happen, the Keeper must be honest about
the vulnerability or worse outcome, and the player's right to proceed or back down
depends on that information. Designer Jason Cordova describes this as a precise,
structured negotiation protecting player agency.
[Official design note on Day and Night moves](https://www.gauntlet-rpg.com/blog/the-day-move-and-the-night-move-in-brindlewood-bay).

**Aicadia implication.** The useful pre-roll frame is qualitative and concrete:
what is the Character trying to learn, how are they investigating, what is exposed
if it goes badly, and what class of result can this attempt establish? This is close
to the current exact-preview discipline, but it belongs before a future roll rather
than only before mutation.

The critical constraint is grounding. A Blades GM is authorized to judge position
and effect; an Aicadia Agent is not a hidden rules authority. Any risk or result
boundary it tells the User must come from published rules and typed World context,
not improvisation presented as mechanics. Copying Blades' full categorical matrix
would also drift toward a generic action engine, which Aicadia explicitly excludes.

## 3. Fail forward where absence would stall play

Fate Condensed tells the GM to remove bare failure from an investigation when
missing a detail would stall the story and to focus on the cost instead. Its
investigation example still reveals a clue; outcome quality or cost carries the
uncertainty.
[Fate Condensed: taking action](https://fate-srd.com/fate-condensed/taking-action-rolling-dice).

The 13th Age SRD gives the broader rule: outside battle, if failure would slow the
action, interpret it as a near-success or an event with unwanted consequences or
side effects so something happens and play continues.
[13th Age Archmage Engine SRD, p. 16](https://pelgranepress.com/media/SRD/13thAgeArchmageEngineSRD.pdf#page=16).

**Aicadia implication.** A queued investigation outcome of “zero discoveries” is
not automatically bad, but a repeatable zero with no changed understanding makes
retrying the same action rational. The transferable test is:

> After this outcome, can the User make a meaningfully different grounded choice?

If not, it is a stall rather than fail-forward. A zero might still be meaningful if
it narrows what is plausible, exposes a concrete risk, closes this approach, or
changes the next available question. Which of those, if any, becomes shared state
is a product choice not made here.

The trade-off is real. “Success with a cost” needs a defined cost owner, accepted
state change and historical footprint. Adding ad-hoc Agent-authored complications
would violate deterministic World validation; adding a universal consequence
system would exceed the smallest investigation slice.

## 4. Treat an oracle result as constrained input, not established truth

Ironsworn supports guided, co-op without a GM, and solo play. Its official rules
overview describes oracles as random tools for outcomes, world details or narrative
events, with players creatively interpreting the result in light of the story and
world already established. Its core outcomes are strong hit, weak hit and miss,
with the move text telling players how to interpret them.
[Official Ironsworn overview](https://tomkinpress.com/pages/ironsworn) and
[official digital edition description](https://tomkinpress.com/products/ironsworn-digital-edition).

**Aicadia implication.** This is the cleanest non-GM pattern for a dumb server:

1. World emits a deterministic, retry-stable neutral result or prompt.
2. The Agent and User interpret it privately against live typed facts.
3. One complete candidate is previewed and confirmed.
4. World validates and commits only its explicit structured meaning.

The oracle does not itself establish a ruin, faction, motive or history. It creates
a creative constraint. This preserves surprise without hiding an LLM or author in
the server.

The cost is interpretive variance. Broad prompts can yield incompatible tone,
duplicate subjects or contradictions; narrow tables can become repetitive and
secretly encode much of the setting. The boundary must therefore be inspectable:
random seed/result, Agent interpretation and accepted World fact are three
different things.

## 5. Roll first, then spend authorship on an eligible discovery

Brindlewood Bay separates clue gathering from theory creation. Its Meddling move
finds a clue on a hit and may add a complication; after enough clues, players openly
construct a theory and roll using the clues they incorporated. A hit makes that
theory the correct solution, possibly with a complication. The official designer
notes explain that clues are intentionally not tied to a fixed person or location,
so they can be revealed in varied scenes and later made coherent by the players.
[Official play sheets](https://www.gauntlet-rpg.com/uploads/7/7/8/1/77811662/brindlewood_bay_play_sheets.pdf),
[revealing a clue](https://www.gauntlet-rpg.com/blog/revealing-a-clue-in-brindlewood-bay),
and [mystery-system design notebook](https://www.gauntlet-rpg.com/blog/design-notebook-from-the-between-to-brindlewood-bay-and-back-again).

**Aicadia implication.** The useful structural echo is not “players may declare any
solution true.” It is that scarce creative work happens after the mechanic reveals
what kind of contribution is eligible. That aligns unusually well with the queued
split between a first investigation roll and later discovery commit: chance first,
then a bounded Agent-authored candidate, then exact User confirmation.

Brindlewood's complete theory procedure is a poor direct fit. It depends on clue
counts and mystery Complexity, Keeper-authored reactions and retroactively
establishing a solution. Aicadia currently has neither a clue collection, a hidden
culprit, nor a semantic judge. Its valuable lesson is sequencing and separation of
authority, not the mystery math.

## 6. Frame the scene clearly, then cut to the decision

Ben Robbins' first-party Kingdom scene guidance says a useful scene first needs a
clear picture, not a startling idea. It reduces framing to three questions: who is
there, where are they, and what is happening. Once those are concrete, players can
act in the moment.
[Lame Mage: A Beginner's Guide to Making Scenes](https://arsludi.lamemage.com/index.php/552/beginner-making-scenes/).

Blades then cuts aggressively to the first obstacle. Its engagement procedure
avoids exhaustive hypothetical planning; details that actually matter can be
addressed once the situation is known. Flashbacks are its mechanism for past
preparation, but they cannot undo an already established present fact.
[Blades planning, engagement and flashbacks](https://bladesinthedark.com/planning-engagement).

**Aicadia implication.** A private Agent turn can begin with a compact natural
frame: the named Character, the exact current Place, the one changed or salient
fact, and the immediate question. That is a narration heuristic over existing
reads—not a new `Scene` record. It can make the existing three-direction workshop
less generic and help a returning User understand why these three directions matter
now.

Blades-style flashbacks should not transfer. Aicadia has immutable Activity and
shared concurrent observers; retroactively inserting preparation would need a
real, validated World action and history. The lighter principle is simply to avoid
speculative planning and re-read current Place state before proposing a commit.

## 7. Model pressure as causal signs, not autonomous clock ticks

Dungeon World's author-owned source defines a `front` as GM preparation: linked
dangers, each with an impulse, grim portents, an impending doom and open stakes
questions. Grim portents may be marked descriptively because play already made them
happen, or prescriptively as a GM move after failure or an exposed opportunity.
[Dungeon World `Fronts.xml`, author repository, pinned commit](https://github.com/Sagelt/Dungeon-World/blob/05a2b6733a66675fee796feae744dc4784fa40a1/text/Fronts.xml).

Blades clocks make progress or danger visible in segments. Its faction clocks,
however, advance during downtime so the city changes around the characters, and
the GM may “catch up” factions later with several downtime phases.
[Blades progress and faction clocks](https://bladesinthedark.com/progress-clocks) and
[downtime faction catch-up](https://bladesinthedark.com/downtime-activities-play).

**Aicadia implication.** The useful design diagnostic is prose, not a clock:

- who or what exerts pressure;
- what impulse makes its next act coherent;
- what observable sign would prove that the pressure changed;
- which explicit accepted cause advances it;
- what concrete World condition would result if it completes.

That can make future faction or environmental behaviour legible. It does not
authorize it now. Aicadia cannot import downtime ticks, GM fiat or catch-up
simulation: no server process may advance a World or trigger an Agent between
explicit calls. Segment clocks also function as counters/progress meters, directly
colliding with Aicadia's no-score rule. If pressure ever exists, every transition
would need deterministic input, stored current state and Activity in the same
accepted action—not an invisible metagame countdown.

## 8. Preserve canonical residue; make recaps disposable views

Microscope's first-party material separates large Periods, contained Events and
particular Scenes; later contributions can focus on and add detail to another
player's earlier contribution. A Focus temporarily directs attention without
making one author own all of history.
[Microscope Chronicle first-party playtest](https://www.lamemage.com/microscope-chronicle/Microscope_Chronicle_%28playtest_2025_05%29.pdf) and
[Microscope Explorer first-party sample](https://www.lamemage.com/microscope-explorer/Microscope_Explorer_sample.pdf).

The Quiet Year uses a shared map plus one card per fictional week; the map and card
sequence externalize what the group has established and what changed. Its finite
calendar and projects deliberately create dwindling time and rising concern.
[Official Quiet Year overview](https://buriedwithoutceremony.com/the-quiet-year) and
[official product rules summary](https://store.buriedwithoutceremony.com/products/the-quiet-year-pdf).

**Aicadia implication.** These games show the value of an artifact that outlives
one speaker, but their authorship model should not be copied. Aicadia already has a
stronger asynchronous primitive: each accepted action leaves Entity and Activity
residue with actor, Place, involved subjects, prose and time. A later recap should
therefore be a read-time lens over canonical history—“what changed here,” “what did
this Character witness,” or “which subject recurs”—not a second stored history that
can disagree.

Microscope's out-of-order insertion is specifically incompatible with Activity's
append-only occurrence history. The Quiet Year's turn order, finite year, project
dice and automatic weekly cadence also do not fit a perpetual concurrent World.
The transferable principle is visible shared residue plus re-incorporation. Exact
Place revisions, not a global turn token, handle current Aicadia concurrency.

## Meta-layer synthesis

The sources converge on a useful separation of layers. This is a descriptive lens
for reviewing future proposals, not a selected architecture:

1. **World facts** — typed, authoritative and deterministic.
2. **Resolution signal** — a roll, outcome band or neutral oracle result; surprising
   but not self-interpreting.
3. **Private interpretation** — Agent proposals grounded in facts and signal.
4. **User authority** — selection, steering and exact confirmation.
5. **Committed consequence** — the smallest structured World change plus canonical
   prose and Activity.
6. **Shared recollection** — recaps, summaries and thematic lenses derived from
   committed history, never new truth.
7. **Pressure** — only if later accepted: explicit current state advanced by a
   concrete accepted cause, never autonomous narration.

Most tabletop failure modes appear when one layer silently impersonates another:
an oracle prompt is narrated as fact, private prose mutates unmodeled state, a recap
rewrites history, or a hidden clock advances without an accountable action.

## Patterns Aicadia should not import unchanged

- **Fate points, Blades stress/coin/rep, relationship points, progress segments and
  similar economies.** They are counters or currencies; Aicadia prohibits scores,
  points and currencies. Fate explicitly uses fate points as narrative influence
  that can declare story details.
  [Fate points](https://fate-srd.com/fate-core/fate-points).
- **A universal strong/weak/miss or position/effect engine.** These systems work
  because a human adjudicator supplies genre judgment. Generalizing them would
  exceed one concrete investigation behaviour and make the Agent a rules oracle.
- **Hidden GM fronts, secret solutions or an off-screen server author.** Dungeon
  World calls fronts secret GM knowledge; Aicadia has no privileged live narrator
  besides deterministic World facts.
- **Autonomous faction turns, downtime ticks or calendar simulation.** Blades and
  The Quiet Year use between-scene/world cadence; Aicadia forbids background Agent
  work and unconscious token spend.
- **Flashbacks that establish past preparation after current facts are known.** They
  conflict with immutable shared Activity unless implemented as a new explicit
  validated historical action, which is outside the current model.
- **Brindlewood's whole “the successful theory becomes true” rule.** It is elegant
  for a GM-led mystery with flexible clues, but Aicadia has no semantic theory
  validator and should not let eloquent prose manufacture arbitrary state.
- **Microscope-style insertion into earlier history.** Layered history is useful as
  presentation; retroactive canonical authorship would break append-only Activity.
- **Global turns, Lens ownership or one shared synchronous scene.** Table order
  allocates attention among people who are present together. It is not a concurrency
  protocol for independent Users and Agents.
- **A generic faction/reputation matrix.** Kingdom and 13th Age show why relations
  generate stories, but numerical relation points and globally authored sentiment
  add identity, ownership and score semantics not required by the current Place or
  first discovery.

## Source uncertainties and exclusions

- Ironsworn's publisher page is a first-party rules overview, not the complete
  browser-readable rulebook. Claims here are limited to the three play modes,
  outcome bands and the stated purpose of oracles; detailed move interactions were
  not inferred from community SRDs.
- The full base Microscope rules are commercial. The history hierarchy and Focus
  claims use Lame Mage's own `Microscope Explorer` sample, Chronicle playtest and
  author material; this is enough for the narrow comparison but not a complete
  Microscope rules audit.
- Dungeon World's source is pinned to the last commit touching the official
  `Fronts.xml` in Sage LaTorra's author repository (2012). It establishes the
  published front procedure, not a claim about a future edition.
- The Quiet Year and Kingdom are synchronous table games. No inspected first-party
  source demonstrates conflict-free asynchronous persistent-world play. Their
  artifacts and authority splits are analogies only; Aicadia's revision and
  Activity semantics must carry that burden.
- Apocalypse World threat sheets were inspected but not used for a substantive
  claim because the freely available first-party sheets expose fields more clearly
  than their operating procedure. Dungeon World's author source and Blades' open
  SRD provide stronger public evidence for fronts and clocks.
- D&D rules, campaign formats and West Marches are intentionally outside this note;
  this is the requested “and more” comparison line.

## Raw first-party sources inspected

- Blades in the Dark official SRD:
  [core](https://bladesinthedark.com/core-system),
  [action roll](https://bladesinthedark.com/action-roll),
  [position/effect](https://bladesinthedark.com/setting-position-effect),
  [clocks](https://bladesinthedark.com/progress-clocks), and
  [planning/flashbacks](https://bladesinthedark.com/planning-engagement).
- Ironsworn publisher pages:
  [game and oracle overview](https://tomkinpress.com/pages/ironsworn) and
  [digital edition](https://tomkinpress.com/products/ironsworn-digital-edition).
- Evil Hat's official Fate SRD:
  [taking action](https://fate-srd.com/fate-condensed/taking-action-rolling-dice)
  and [fate points](https://fate-srd.com/fate-core/fate-points).
- Pelgrane/Fire Opal:
  [13th Age Archmage Engine SRD](https://pelgranepress.com/media/SRD/13thAgeArchmageEngineSRD.pdf).
- Sage LaTorra's Dungeon World source:
  [`Fronts.xml` at inspected commit](https://github.com/Sagelt/Dungeon-World/blob/05a2b6733a66675fee796feae744dc4784fa40a1/text/Fronts.xml).
- The Gauntlet/Jason Cordova:
  [Brindlewood Bay play sheets](https://www.gauntlet-rpg.com/uploads/7/7/8/1/77811662/brindlewood_bay_play_sheets.pdf),
  [clue guidance](https://www.gauntlet-rpg.com/blog/revealing-a-clue-in-brindlewood-bay),
  and [Day/Night negotiation](https://www.gauntlet-rpg.com/blog/the-day-move-and-the-night-move-in-brindlewood-bay).
- Lame Mage/Ben Robbins:
  [Kingdom overview](https://www.lamemage.com/kingdom/),
  [Kingdom 2e role cards](https://www.lamemage.com/kingdom/K2/Kingdom_2E_roles.pdf),
  [scene framing](https://arsludi.lamemage.com/index.php/552/beginner-making-scenes/),
  [Microscope Explorer sample](https://www.lamemage.com/microscope-explorer/Microscope_Explorer_sample.pdf),
  and [Microscope Chronicle playtest](https://www.lamemage.com/microscope-chronicle/Microscope_Chronicle_%28playtest_2025_05%29.pdf).
- Buried Without Ceremony/Avery Alder:
  [The Quiet Year](https://buriedwithoutceremony.com/the-quiet-year) and
  [official product summary](https://store.buriedwithoutceremony.com/products/the-quiet-year-pdf).
